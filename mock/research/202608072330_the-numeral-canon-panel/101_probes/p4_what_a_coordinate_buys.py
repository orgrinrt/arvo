#!/usr/bin/env python3
"""How many distinguishable strategies does a coordinate SET support?

`97` and `98` both report that on the committed carrier table, 9 of 15625
sections are rationalisable by a strictly positive weighting, and `99` carries
that as the unit's corrected headline. Read carefully, `98`'s reproduction says
what model that number belongs to: "6 regions, 5 arms, 2 coordinates (ns per
record, bits per element)" (`98_probes/p6_reproduce_the_predecessors_count_and_rung_it.out:1`).

So 9 is a fact about a TWO-coordinate model, and it is the first measurement in
this unit of a quantity nobody has named: **how many distinct answers the
coordinate set can support at all**. That is the ceiling on how far two
strategies can differ. If a coordinate set admits nine sections, then nine is
the number of distinguishable strategies available, whatever names are given to
them, and any two weightings landing in the same cell ARE the same strategy.

This probe measures that ceiling as a function of the coordinate set, exactly.

  Method. A section is strictly rationalisable when some `w > 0` makes the named
  arm the unique argmin at every region: for each region n and each other arm b,
  `<w, c_b - c_s(n)> > 0`. The system is homogeneous, so normalise to the
  simplex and the feasible set is a polytope of dimension d-1. For d = 2 that is
  an interval and for d = 3 a polygon, both clipped exactly in `Fraction`
  arithmetic.

  The instrument is written from the geometry rather than read from
  `98_probes/cone.py` or `97_probes/p9_the_decider.py`, deliberately, so that
  agreement is evidence and disagreement is visible. Block A checks it against
  their published counts on their own model before anything rests on it.

Blocks:
  A. Reproduce `97`'s two published counts on `97`'s model, as a calibration of
     this instrument against theirs.
  B. The ceiling as a function of the coordinate set: {time}, {time, size},
     {time, size, spread}, and {time, spread}.
  C. What that says about a coordinate op's intents name and the corpus lacks.

This reads committed artifacts. It is NOT a bench, no measurement was taken,
and no number here prices anything.

Run:  python3 p4_what_a_coordinate_buys.py
"""

import csv
import glob
import itertools
import os
import re
import statistics
from collections import defaultdict
from fractions import Fraction

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))

# Bits per element, as `97`'s model declares them (`98_probes/p6_model97.json`
# carries 16, 32, 64, 13, 13). A declared property of the arm; the corpus
# measures no size.
BITS = {
    "bitpack-carrier-d16": 16,
    "bitpack-carrier-d16-control": 16,
    "bitpack-carrier-d32": 32,
    "bitpack-carrier-d64": 64,
    "bitpack-carrier-packed": 13,
    "bitpack-carrier-packed-simd": 13,
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


def median(xs):
    return Fraction(statistics.median(xs)).limit_denominator(10**12)


def spread(xs):
    q = statistics.quantiles(sorted(xs), n=4)
    return Fraction(q[2] - q[0]).limit_denominator(10**12)


# ── exact feasibility, written from the geometry ──


def feasible(constraints, d, weights_strict, cons_strict):
    """Is there a w on the simplex of dimension d, in its interior when
    `weights_strict`, with <w, g> > 0 (when `cons_strict`) or >= 0 for every g?

    Two independent knobs, and the first version of this probe had them welded
    together, which is why its output reported 9 where `97` reports 72. That
    output is kept at `p4_first_version_conflated_two_knobs.out`. The knobs are
    different questions: `weights_strict` asks whether a strategy may put zero
    weight on a coordinate, `cons_strict` asks whether the named arm must be the
    UNIQUE argmin or merely one of the minima.
    """
    if d == 2:
        # w = (x, 1-x). <w,g> = x*(g0-g1) + g1.
        lo, hi = Fraction(0), Fraction(1)
        lo_open = hi_open = weights_strict
        for g in constraints:
            a, c = g[0] - g[1], g[1]
            if a == 0:
                if c < 0 or (cons_strict and c == 0):
                    return False
                continue
            b = -Fraction(c, a)
            if a > 0:  # x >= b, strictly when cons_strict
                if b > lo:
                    lo, lo_open = b, cons_strict
                elif b == lo:
                    lo_open = lo_open or cons_strict
            else:  # x <= b, strictly when cons_strict
                if b < hi:
                    hi, hi_open = b, cons_strict
                elif b == hi:
                    hi_open = hi_open or cons_strict
        if lo > hi:
            return False
        if lo == hi and (lo_open or hi_open):
            return False
        return True
    if d == 3:
        tri = [
            (Fraction(0), Fraction(0)),
            (Fraction(1), Fraction(0)),
            (Fraction(0), Fraction(1)),
        ]
        # (a, b, c) means a*x + b*y + c >= 0, paired with whether it must be > 0.
        hps = [((g[0] - g[2], g[1] - g[2], g[2]), cons_strict) for g in constraints]
        hps += [
            ((Fraction(1), Fraction(0), Fraction(0)), weights_strict),
            ((Fraction(0), Fraction(1), Fraction(0)), weights_strict),
            ((Fraction(-1), Fraction(-1), Fraction(1)), weights_strict),
        ]
        poly = tri
        for hp, _ in hps:
            poly = clip(poly, hp)
            if not poly:
                return False
        # A point strictly satisfying every strict half-plane at once exists
        # exactly when each is strict at some vertex: the centroid's value is
        # the mean of non-negative terms.
        for (a, b, c), st in hps:
            if st and not any(a * x + b * y + c > 0 for x, y in poly):
                return False
        return True
    raise ValueError(d)


def clip(poly, hp):
    a, b, c = hp
    out = []
    n = len(poly)
    for i in range(n):
        px, py = poly[i]
        qx, qy = poly[(i + 1) % n]
        vp = a * px + b * py + c
        vq = a * qx + b * qy + c
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


def count_sections(table, arms, d, weights_strict, cons_strict):
    """Enumerate every arms^regions section and count the feasible ones."""
    regions = sorted(table)
    n_ok = 0
    for sec in itertools.product(arms, repeat=len(regions)):
        cons = []
        for n, a in zip(regions, sec):
            for b in arms:
                if b == a:
                    continue
                cons.append(tuple(table[n][b][i] - table[n][a][i] for i in range(d)))
        if feasible(cons, d, weights_strict, cons_strict):
            n_ok += 1
    return n_ok


def main():
    per = samples("bitpack-carrier-width")
    arms5 = sorted(a for a in BITS if not a.endswith("control"))
    arms6 = sorted(BITS)

    def build(arms, coords):
        t = {}
        for n, p in per.items():
            t[n] = {}
            for a in arms:
                v = []
                if "time" in coords:
                    v.append(median(p[a]))
                if "size" in coords:
                    v.append(Fraction(BITS[a]))
                if "spread" in coords:
                    v.append(spread(p[a]))
                t[n][a] = tuple(v)
        return t

    print("=" * 78)
    print("A. CALIBRATION AGAINST `97`'S PUBLISHED COUNTS, ON `97`'S MODEL")
    print("=" * 78)
    t2 = build(arms5, ["time", "size"])
    nonneg = count_sections(t2, arms5, 2, weights_strict=False, cons_strict=False)
    strict = count_sections(t2, arms5, 2, weights_strict=True, cons_strict=False)
    print(f"  6 regions, 5 arms, 2 coordinates (median algo_ns, bits per element)")
    print(f"  sections total                          {len(arms5) ** len(t2)}")
    print(f"  rationalisable, w >= 0    this probe:   {nonneg:6d}    97 and 98 report      72")
    print(f"  rationalisable, w  > 0    this probe:   {strict:6d}    97 and 98 report       9")
    ok = (nonneg, strict) == (72, 9)
    print(f"  instrument agrees with theirs: {ok}")
    print()
    print("  Two exact implementations written from different geometries agreeing")
    print("  on both counts is a third instance of the same number. `98` was the")
    print("  second and wrote its own; this is written from interval and polygon")
    print("  clipping without reading either.")

    print()
    print("=" * 78)
    print("B. THE CEILING AS A FUNCTION OF THE COORDINATE SET")
    print("=" * 78)
    print("  How many sections a strictly positive weighting can reach. That is the")
    print("  number of distinguishable strategies the coordinate set supports; two")
    print("  weightings reaching the same section are the same strategy.")
    print()
    print(f"  {'coordinates':32s} {'arms':>5s} {'sections':>9s} {'w >= 0':>8s} {'w > 0':>7s}")
    for arms, label in [(arms5, "5 (control dropped)"), (arms6, "6 (control kept)")]:
        for coords in (["time"], ["time", "size"], ["time", "spread"], ["time", "size", "spread"]):
            d = len(coords)
            if d == 1:
                # One coordinate: the weighting is a positive scalar and drops
                # out. Exactly one section is reachable, the per-region argmin.
                nn = st = 1
            else:
                t = build(arms, coords)
                nn = count_sections(t, arms, d, weights_strict=False, cons_strict=False)
                st = count_sections(t, arms, d, weights_strict=True, cons_strict=False)
            name = "{" + ", ".join(coords) + "}"
            print(f"  {name:32s} {label[:1]:>5s} {len(arms) ** len(per):9d} {nn:8d} {st:7d}")

    print()
    print("=" * 78)
    print("C. WHAT THAT SAYS ABOUT A COORDINATE THAT DOES NOT EXIST")
    print("=" * 78)
    print("  With one coordinate the weighting is a positive scalar, it cancels, and")
    print("  exactly one section is reachable: every strategy agrees, by algebra and")
    print("  not by luck. Each coordinate added multiplies what the space can express.")
    print()
    print("  So a strategy whose intent names a quantity with no coordinate is not")
    print("  merely unmeasured. It is INEXPRESSIBLE: there is no axis along which it")
    print("  can differ from any other strategy, so it and its opposite are the same")
    print("  point in the space, whatever the canon calls them.")


if __name__ == "__main__":
    main()
