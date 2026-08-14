#!/usr/bin/env python3
"""How much room is there between two strategies on the coordinates the corpus
actually carries?

The unit's object is "a weighting over cost coordinates". A weighting on raw
coordinates in different units is not a set of shares adding to one; it is a
RATE OF EXCHANGE. With two coordinates, `w1 * time + w2 * size` compares arms
exactly as `time + r * size` does with `r = w2/w1`, so the whole weighting space
is one number with units of coordinate-1 per coordinate-2 (nanoseconds per byte,
here), and scaling a weighting changes nothing. That is `p2`'s theorem read the
other way.

Two things follow and both are computable exactly rather than by sampling.

  CELLS. Arm `a` beats arm `b` at a region exactly when
  `(x_a - x_b) + r (y_a - y_b) < 0`. That is linear in `r`, so each ordered pair
  at each region contributes at most one boundary,
  `r* = -(x_a - x_b)/(y_a - y_b)`. Between consecutive boundaries the argmin at
  every region is constant, so the section is a piecewise-constant function of
  `r` and the pieces are intervals. The number of pieces is a property of the
  cost table and nothing else: it is how many distinguishable answers the
  coordinate set can support, and it bounds how far two strategies can differ.

  MARGIN. A declared `r` sits somewhere inside its cell. The distance to the
  nearest boundary, in orders of magnitude, is how wrong the number may be
  before the answer changes. Two weightings inside one cell are the same
  strategy: the design does not need the weights to be right, only to land in
  the right cell.

  This is `97`'s cone, in coordinates. Cone membership of a stated weighting and
  "the stated `r` lies in the cell whose section is the shipped table" are the
  same test, and the cell has a width the cone did not report.

Block A prints, per family, the cells over `r` and their sections. Block B
reports the widest cell and where the boundaries bunch. Block C runs the same
question on three coordinates by sampling, because a three-coordinate weighting
is a two-dimensional projective space and no longer an interval.

Coordinates. Coordinate 1 is the median of `algo_ns`, which the corpus
measures. Coordinate 2 is bytes per element, which the corpus does NOT measure:
it is declared per arm, exactly as `98` p10 and `100` p1 declare it, and it is
copied from `100_probes/p1_what_the_instability_is_made_of.py:78-87` so the two
are comparable. Where a family has no declared bytes, the second coordinate is
the interquartile range of the same samples, which is measured.

This reads committed artifacts. It is NOT a bench, no measurement was taken,
and no number here prices anything.

Run:  python3 p3_the_exchange_rate_and_its_cells.py
"""

import csv
import glob
import math
import os
import random
import re
import statistics
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))

# Declared bytes per element, copied from `100_probes/p1_*.py:78-87`, which
# copies `98` p10. A declared property of the arm, not a measurement.
BYTES = {
    "bitpack-carrier-d16": 2.0,
    "bitpack-carrier-d16-control": 2.0,
    "bitpack-carrier-d32": 4.0,
    "bitpack-carrier-d64": 8.0,
    "bitpack-carrier-packed": 13.0 / 8.0,
    "bitpack-carrier-packed-simd": 13.0 / 8.0,
}


def samples(family):
    out = {}
    for path in sorted(glob.glob(os.path.join(BENCH, f"{family}_n*.csv"))):
        n = int(re.search(r"n(\d+)\.csv$", path).group(1))
        per = defaultdict(list)
        with open(path) as fh:
            for row in csv.DictReader(fh):
                per[row["variant"]].append(float(row["algo_ns"]))
        out[n] = dict(per)
    return out


def table_time_and(second):
    """Cost table builder. `second` maps (arm, samples) -> the y coordinate."""

    def build(per_region, arms):
        t = {}
        for n, per in per_region.items():
            t[n] = {a: (statistics.median(per[a]), second(a, per[a])) for a in arms}
        return t

    return build


def iqr(_arm, xs):
    q = statistics.quantiles(sorted(xs), n=4)
    return q[2] - q[0]


def declared_bytes(arm, _xs):
    return BYTES[arm]


def section_at(t, arms, r):
    """argmin per region of x + r*y."""
    return tuple(min(arms, key=lambda a: t[n][a][0] + r * t[n][a][1]) for n in sorted(t))


def boundaries(t, arms):
    """Every positive r at which some pairwise comparison ties, at any region."""
    bs = set()
    for n in t:
        for i, a in enumerate(arms):
            for b in arms[i + 1 :]:
                dx = t[n][a][0] - t[n][b][0]
                dy = t[n][a][1] - t[n][b][1]
                if dy == 0:
                    continue
                r = -dx / dy
                if r > 0 and math.isfinite(r):
                    bs.add(r)
    return sorted(bs)


def cells(t, arms):
    """(r_lo, r_hi, section) for each interval between consecutive boundaries,
    merging neighbours whose section is the same."""
    bs = boundaries(t, arms)
    edges = [0.0] + bs + [float("inf")]
    out = []
    for lo, hi in zip(edges, edges[1:]):
        mid = (lo + hi) / 2 if math.isfinite(hi) else lo * 10 + 1
        s = section_at(t, arms, mid)
        if out and out[-1][2] == s:
            out[-1] = (out[-1][0], hi, s)
        else:
            out.append((lo, hi, s))
    return out


def main():
    print("=" * 78)
    print("A. THE EXCHANGE-RATE AXIS, EXACTLY")
    print("=" * 78)
    print("Coordinate 1 is median algo_ns. r is how many nanoseconds one unit of")
    print("coordinate 2 is worth. Every section the coordinate pair can produce")
    print("appears below, with the interval of r that produces it.")
    print()

    setups = [
        ("bitpack-carrier-width", "declared bytes/elem", declared_bytes),
        ("bitpack-carrier-width", "IQR of algo_ns", iqr),
        ("bitpack-contend-decode", "IQR of algo_ns", iqr),
        ("bitpack-contention", "IQR of algo_ns", iqr),
        ("bitpack-wide", "IQR of algo_ns", iqr),
    ]

    summary = []
    for fam, label, second in setups:
        per = samples(fam)
        arms = sorted(set.intersection(*[set(p) for p in per.values()]))
        t = table_time_and(second)(per, arms)
        cs = cells(t, arms)
        print(f"  {fam}   coordinate 2 = {label}   arms={len(arms)} regions={len(t)}")
        print(f"    {'r from':>14s} {'r to':>14s}  decades  section")
        for lo, hi, s in cs:
            dec = (
                math.log10(hi / lo)
                if lo > 0 and math.isfinite(hi)
                else float("inf")
            )
            short = ",".join(a.split("-")[-1] for a in s)
            print(f"    {lo:14.6g} {hi:14.6g}  {dec:7.2f}  {short}")
        widest = max(
            (
                (math.log10(hi / lo) if lo > 0 and math.isfinite(hi) else float("inf"), s)
                for lo, hi, s in cs
            ),
            key=lambda z: z[0],
        )
        summary.append((fam, label, len(cs), widest[0]))
        print()

    print("=" * 78)
    print("B. HOW MANY ANSWERS THE COORDINATE PAIR CAN SUPPORT")
    print("=" * 78)
    print(f"  {'family':24s} {'coordinate 2':22s} {'cells':>6s} {'widest cell (decades)':>22s}")
    for fam, label, n, w in summary:
        print(f"  {fam:24s} {label:22s} {n:6d} {w:22.2f}")
    print()
    print("A cell is a set of weightings that are the same strategy. The count is")
    print("how many distinguishable strategies the coordinate pair supports at all;")
    print("the width is how wrong a declared exchange rate may be and still select")
    print("the shipped table.")

    print()
    print("=" * 78)
    print("C. THREE COORDINATES, BY SAMPLING, SINCE THE SPACE IS NO LONGER AN INTERVAL")
    print("=" * 78)
    print("(median algo_ns, declared bytes, IQR) on the carrier family, log-uniform")
    print("exchange rates over twelve decades each, 20000 draws.")
    rng = random.Random(20260814)
    per = samples("bitpack-carrier-width")
    arms = sorted(set.intersection(*[set(p) for p in per.values()]))
    t3 = {}
    for n, p in per.items():
        t3[n] = {
            a: (statistics.median(p[a]), BYTES[a], iqr(a, p[a])) for a in arms
        }
    seen = defaultdict(int)
    DRAWS = 20000
    for _ in range(DRAWS):
        r2 = 10 ** rng.uniform(-6, 6)
        r3 = 10 ** rng.uniform(-6, 6)
        s = tuple(
            min(arms, key=lambda a: t3[n][a][0] + r2 * t3[n][a][1] + r3 * t3[n][a][2])
            for n in sorted(t3)
        )
        seen[s] += 1
    print(f"  distinct sections reached: {len(seen)} of {len(arms)}^{len(t3)} possible")
    for s, c in sorted(seen.items(), key=lambda kv: -kv[1]):
        short = ",".join(a.split("-")[-1] for a in s)
        print(f"    {c / DRAWS * 100:6.2f}% of the sampled space   {short}")
    print()
    print("The share of sampled space is NOT scale invariant: it depends on the")
    print("twelve-decade window the rates were drawn from, which is a choice. The")
    print("COUNT is invariant, because scaling a weighting is a bijection on the")
    print("weighting space that maps the reachable set onto itself.")


if __name__ == "__main__":
    main()
