#!/usr/bin/env python3
"""Is a better-behaved estimator a better estimator of the same coordinate, or
a different coordinate?

`100` section 7.3 measures six estimators of what it calls the tail coordinate
on the committed carrier samples and finds the 95th percentile strictly better
than the interquartile range on both axes it looks at: 3 distinct sections
against 161 under resampling, and 54 of 60 arm pairs separated against 43 of 60.
It concludes that three separate negative findings about that coordinate "was a
fact about the interquartile range and not about tail behaviour".

The conclusion is about resolution and it does not check the one thing that
would distinguish the two readings. The 95th percentile of a batch of timings is
a LEVEL: it contains the median and adds a tail on top of it. The interquartile
range is a SPREAD: the median cancels out of it by construction. So an estimator
that is stable and separating because it is largely re-measuring coordinate one
looks exactly like an estimator that is stable and separating because it
resolves coordinate three, on every statistic `100` reports.

Three measurements distinguish them, and the third is the decisive one.

  A. RESOLUTION. Two arms the bench declares byte-identical differ only by
     measurement error, so the distribution of `E(A) - E(B)` across resamples is
     a pure noise sample for estimator `E`. That gives each estimator a
     resolution in its own units, per family, from the corpus itself. Signal is
     the spread of `E` across the arms that genuinely differ. An estimator whose
     signal does not exceed its own floor cannot decide anything.

  B. COLLINEARITY. Correlation of each candidate against the median across every
     (arm, region) cell. A candidate at correlation near one is not an
     independent axis.

  C. WHAT IT ADDS. The exact count of sections a strictly positive weighting can
     reach on `{median, X}`, by the instrument calibrated in
     `p4_what_a_coordinate_buys.py`. This is the only one of the three that
     answers the design question, because a second coordinate exists to let two
     strategies disagree, and a coordinate that adds no reachable section lets
     nobody disagree about anything. If `{median, p95}` reaches one section and
     `{median, IQR}` reaches six, then the interquartile range is the axis and
     the 95th percentile is a restatement of the first coordinate, however much
     better behaved it is.

Candidates. Three spreads (interquartile range, interdecile range, median
absolute deviation), three levels (95th and 99th percentiles, mean), and two
EXCESSES built by subtracting the median from a level, which is the shape a
tail-weighing intent actually wants: it asks how far the tail runs beyond the
typical case, not where the tail sits.

This reads committed artifacts. It is NOT a bench, no measurement was taken,
and no number here prices anything.

Run:  python3 p5_an_estimator_is_a_coordinate_choice.py
"""

import csv
import glob
import itertools
import math
import os
import random
import re
import statistics
from collections import defaultdict
from fractions import Fraction

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))

SEED = 20260814
RESAMPLES = 2000

# Block C enumerates arms^regions sections exhaustively, so a family with many
# regions is out of reach: bitpack-contention is 5 real arms over 12 regions,
# which is 244 million sections. It is skipped there and said so rather than
# sampled, because a sampled count of REACHABLE sections is a lower bound and
# would read as the same quantity the other rows report exactly.

PAIRS = {
    "bitpack-carrier-width": ("bitpack-carrier-d16", "bitpack-carrier-d16-control"),
    "bitpack-contend-decode": ("bitpack-contend-d16", "bitpack-contend-d16-control"),
    "bitpack-contention": ("bitpack-contend-d16", "bitpack-contend-d16-control"),
    "bitpack-wide": ("bitpack-wide-d16", "bitpack-wide-d16-control"),
}


def q(xs, p):
    xs = sorted(xs)
    if len(xs) == 1:
        return xs[0]
    k = p * (len(xs) - 1)
    lo = int(math.floor(k))
    hi = min(lo + 1, len(xs) - 1)
    return xs[lo] + (xs[hi] - xs[lo]) * (k - lo)


def med(xs):
    return statistics.median(xs)


ESTIMATORS = {
    "median": med,
    "mean": statistics.fmean,
    "iqr  (spread)": lambda xs: q(xs, 0.75) - q(xs, 0.25),
    "idr  (spread)": lambda xs: q(xs, 0.90) - q(xs, 0.10),
    "mad  (spread)": lambda xs: statistics.median([abs(x - med(xs)) for x in xs]),
    "p95  (level)": lambda xs: q(xs, 0.95),
    "p99  (level)": lambda xs: q(xs, 0.99),
    "p95-med (excess)": lambda xs: q(xs, 0.95) - med(xs),
    "p99-med (excess)": lambda xs: q(xs, 0.99) - med(xs),
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


# ── exact section counting, reusing p4's geometry ──


def clip(poly, hp):
    a, b, c = hp
    out = []
    for i in range(len(poly)):
        px, py = poly[i]
        qx, qy = poly[(i + 1) % len(poly)]
        vp, vq = a * px + b * py + c, a * qx + b * qy + c
        if vp >= 0:
            out.append((px, py))
        if (vp > 0 and vq < 0) or (vp < 0 and vq > 0):
            t = vp / (vp - vq)
            out.append((px + t * (qx - px), py + t * (qy - py)))
    ded = []
    for p in out:
        if not ded or ded[-1] != p:
            ded.append(p)
    if len(ded) > 1 and ded[0] == ded[-1]:
        ded.pop()
    return ded


def feasible2(constraints, weights_strict):
    lo, hi = Fraction(0), Fraction(1)
    lo_open = hi_open = weights_strict
    for g in constraints:
        a, c = g[0] - g[1], g[1]
        if a == 0:
            if c < 0:
                return False
            continue
        b = -Fraction(c, a)
        if a > 0:
            if b > lo:
                lo, lo_open = b, False
        else:
            if b < hi:
                hi, hi_open = b, False
    if lo > hi:
        return False
    if lo == hi and (lo_open or hi_open):
        return False
    return True


def reachable(table, arms, weights_strict=True):
    regions = sorted(table)
    n = 0
    for sec in itertools.product(arms, repeat=len(regions)):
        cons = []
        for r, a in zip(regions, sec):
            for b in arms:
                if b != a:
                    cons.append(tuple(table[r][b][i] - table[r][a][i] for i in range(2)))
        if feasible2(cons, weights_strict):
            n += 1
    return n


def main():
    rng = random.Random(SEED)

    print("=" * 78)
    print("A. EACH ESTIMATOR'S RESOLUTION, CALIBRATED BY THE BYTE-IDENTICAL PAIR")
    print("=" * 78)
    print("floor = 95% half-width of the resampled relative difference between two")
    print("arms that compile to the same machine code, in percent of the estimate.")
    print("signal = (max - min) / min of the estimate across the arms that differ.")
    print("An estimator whose signal does not clear its own floor decides nothing.")
    print()
    for fam, (a, b) in PAIRS.items():
        per = samples(fam)
        arms = sorted(set.intersection(*[set(p) for p in per.values()]))
        real = [x for x in arms if not x.endswith("-control")]
        print(f"  {fam}   pair = {a} / {b}")
        print(f"    {'estimator':18s} {'floor %':>9s} {'signal %':>10s} {'signal/floor':>13s}")
        for name, E in ESTIMATORS.items():
            floors, signals = [], []
            for n in sorted(per):
                xa, xb = per[n][a], per[n][b]
                diffs = []
                for _ in range(RESAMPLES):
                    ra = [xa[rng.randrange(len(xa))] for _ in range(len(xa))]
                    rb = [xb[rng.randrange(len(xb))] for _ in range(len(xb))]
                    ea, eb = E(ra), E(rb)
                    if ea:
                        diffs.append(abs(ea - eb) / abs(ea))
                floors.append(q(diffs, 0.95) * 100)
                vals = [E(per[n][x]) for x in real]
                lo, hi = min(vals), max(vals)
                signals.append((hi - lo) / abs(lo) * 100 if lo else float("nan"))
            f = statistics.fmean(floors)
            s = statistics.fmean(signals)
            print(f"    {name:18s} {f:9.2f} {s:10.2f} {s / f if f else float('nan'):13.1f}")
        print()

    print("=" * 78)
    print("B. IS THE CANDIDATE AN INDEPENDENT AXIS, OR THE MEDIAN AGAIN")
    print("=" * 78)
    print("Pearson correlation against the median across every (arm, region) cell,")
    print("and the same after the median is subtracted out of the candidate.")
    print()
    for fam in PAIRS:
        per = samples(fam)
        arms = sorted(set.intersection(*[set(p) for p in per.values()]))
        meds = [med(per[n][x]) for n in sorted(per) for x in arms]
        print(f"  {fam}")
        print(f"    {'estimator':18s} {'corr with median':>18s}")
        for name, E in ESTIMATORS.items():
            vals = [E(per[n][x]) for n in sorted(per) for x in arms]
            r = statistics.correlation(meds, vals)
            print(f"    {name:18s} {r:18.4f}")
        print()

    print("=" * 78)
    print("C. WHAT EACH CANDIDATE ADDS AS A SECOND COORDINATE")
    print("=" * 78)
    print("Exact count of sections a strictly positive weighting can reach on")
    print("{median algo_ns, candidate}, by the instrument p4 calibrates against")
    print("`97`'s published 72 and 9. One means the candidate changes nothing: no")
    print("two strategies can disagree on it.")
    print()
    for fam in PAIRS:
        per = samples(fam)
        arms = sorted(set.intersection(*[set(p) for p in per.values()]))
        real = [x for x in arms if not x.endswith("-control")]
        if len(real) ** len(per) > 100000:
            print(f"  {fam}   SKIPPED: {len(real)}^{len(per)} = "
                  f"{len(real) ** len(per)} sections is beyond exhaustive enumeration")
            print()
            continue
        print(f"  {fam}   {len(real)} arms, control dropped, {len(per)} regions")
        print(f"    {'estimator':18s} {'sections reachable':>19s}")
        for name, E in ESTIMATORS.items():
            t = {
                n: {
                    x: (
                        Fraction(med(per[n][x])).limit_denominator(10**12),
                        Fraction(E(per[n][x])).limit_denominator(10**12),
                    )
                    for x in real
                }
                for n in per
            }
            print(f"    {name:18s} {reachable(t, real):19d}")
        print()


if __name__ == "__main__":
    main()
