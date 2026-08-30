#!/usr/bin/env python3
"""Is normalisation a decision about the costs, or a reparameterisation of the
weighting?

`100` section 8 measures that min-max normalisation whose range is read off the
arm set breaks independence of irrelevant alternatives, that raw coordinates do
not, and that freezing the range as declared constants restores it. It then says
a design shipping normalised costs "has to state the normalisation range as
declared constants, because it is part of the semantics rather than a
presentation detail", and calls that a cost somebody has to pay.

The algebra says something stronger, and this probe tests it three ways.

  THEOREM. Let the objective be the weighted sum `sum_i w_i c_i` over cost
  coordinates c, and let each coordinate be mapped by a FIXED affine transform
  `c_i -> (c_i - b_i) / a_i` with `a_i > 0`. Then

      sum_i w'_i (c_i - b_i)/a_i  =  sum_i (w'_i / a_i) c_i  -  sum_i w'_i b_i / a_i

  and the second term is constant across arms. So the argmin over arms is
  identical to the raw argmin under the weighting `w_i = w'_i / a_i`. A fixed
  per-coordinate affine map is therefore a BIJECTION on weightings that
  preserves every section it can produce: normalising with declared constants
  and not normalising at all are the same model in two coordinate systems, and
  neither can express a section the other cannot.

  COROLLARY 1. The weights carry the units. A weight on a time coordinate is per
  nanosecond and a weight on a size coordinate is per byte; "0.5 speed, 0.5
  size" is not a statement until the ranges are named, because the ranges are
  the units.

  COROLLARY 2. Only a DATA-DEPENDENT transform is a different model, because
  then `a_i` is a function of the arm set and the map is no longer fixed.
  That is the whole of the independence failure, and it is a theorem rather
  than a measurement: an argmin is unaffected by an alternative that is never
  the minimum, unless that alternative changes the objective.

  COROLLARY 3, which bites something in this unit. Any quantity stated as a
  fraction of the objective's achievable range is data-dependent in exactly the
  same way. `100`'s tolerance band is stated as a percentage of
  `(worst - best)` at the region, so adding an arm that no weighting can select
  can widen or narrow the band and change whether the differential fires.

Three blocks:

  A. The equivalence, on real cost tables from every committed family that has
     a control arm, over random weightings. Frozen-normalised and raw must
     agree on every section, every time.
  B. The independence controls, extended past `100`'s single family: drop a
     dominated arm, and add an arm that is strictly worse everywhere, under
     each of the three transforms.
  C. The band corollary: does an unselectable arm change whether a
     range-relative band accepts a table, and does a coordinate-relative band
     stay put?

This reads committed artifacts. It is NOT a bench, no measurement was taken,
and no number here prices anything.

Run:  python3 p2_normalisation_is_a_change_of_basis.py
"""

import csv
import glob
import os
import random
import re
import statistics
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))

SEED = 20260814
DRAWS = 2000

# The four committed families that declare a byte-identical noise-floor control
# arm, per `p1_the_coordinate_census.out` section 3.
FAMILIES = [
    "bitpack-carrier-width",
    "bitpack-contend-decode",
    "bitpack-contention",
    "bitpack-wide",
]


def samples(family):
    """Raw algo_ns per arm per region from the committed CSVs."""
    out = {}
    for path in sorted(glob.glob(os.path.join(BENCH, f"{family}_n*.csv"))):
        n = int(re.search(r"n(\d+)\.csv$", path).group(1))
        per = defaultdict(list)
        with open(path) as fh:
            for row in csv.DictReader(fh):
                per[row["variant"]].append(float(row["algo_ns"]))
        out[n] = dict(per)
    return out


def cost_table(per_region, arms):
    """Two measured coordinates per arm per region: median and interquartile
    range of `algo_ns`. Two suffice for the algebra and both are real."""
    table = {}
    for n, per in per_region.items():
        row = {}
        for arm in arms:
            xs = sorted(per[arm])
            q = statistics.quantiles(xs, n=4)
            row[arm] = (statistics.median(xs), q[2] - q[0])
        table[n] = row
    return table


def section(table, arms, w, transform=None):
    """argmin per region under weighting w, optionally after transforming."""
    t = transform(table, arms) if transform else table
    out = []
    for n in sorted(t):
        best, bestv = None, None
        for arm in arms:
            v = sum(wi * ci for wi, ci in zip(w, t[n][arm]))
            if bestv is None or v < bestv:
                best, bestv = arm, v
        out.append(best)
    return tuple(out)


def minmax(table, arms):
    """Min-max per coordinate over the whole table. The range is read off the
    arm set, which is the data dependence."""
    d = len(next(iter(next(iter(table.values())).values())))
    lo = [min(table[n][a][i] for n in table for a in arms) for i in range(d)]
    hi = [max(table[n][a][i] for n in table for a in arms) for i in range(d)]
    out = {}
    for n in table:
        out[n] = {
            a: tuple(
                (table[n][a][i] - lo[i]) / (hi[i] - lo[i]) if hi[i] > lo[i] else 0.0
                for i in range(d)
            )
            for a in arms
        }
    return out


def frozen(consts):
    """Min-max with the range FROZEN as declared constants, so the transform
    does not depend on which arms are present."""

    def f(table, arms):
        lo, hi = consts
        d = len(lo)
        return {
            n: {
                a: tuple((table[n][a][i] - lo[i]) / (hi[i] - lo[i]) for i in range(d))
                for a in arms
            }
            for n in table
        }

    return f


def dominated(table, arms):
    """Arms no weighting with non-negative weights can ever select: strictly
    worse than some other arm on every coordinate, at every region."""
    out = []
    for a in arms:
        for b in arms:
            if a == b:
                continue
            if all(
                all(table[n][b][i] < table[n][a][i] for i in range(len(table[n][a])))
                for n in table
            ):
                out.append(a)
                break
    return out


def main():
    rng = random.Random(SEED)
    print("=" * 78)
    print("A. FROZEN-RANGE NORMALISATION AND RAW COORDINATES ARE THE SAME MODEL")
    print("=" * 78)
    print("For each family and each of", DRAWS, "random weightings w' drawn on the")
    print("simplex, the section under frozen normalisation with w' must equal the")
    print("section under raw coordinates with w_i = w'_i / (hi_i - lo_i).")
    print()
    tables = {}
    for fam in FAMILIES:
        per = samples(fam)
        arms = sorted(set.intersection(*[set(p) for p in per.values()]))
        t = cost_table(per, arms)
        tables[fam] = (t, arms)
        d = 2
        lo = [min(t[n][a][i] for n in t for a in arms) for i in range(d)]
        hi = [max(t[n][a][i] for n in t for a in arms) for i in range(d)]
        # Declared constants: deliberately NOT the data's own min and max, so
        # the test is of a fixed transform rather than of min-max in disguise.
        cl = [lo[i] * 0.5 for i in range(d)]
        ch = [hi[i] * 3.0 for i in range(d)]
        agree = 0
        for _ in range(DRAWS):
            wp = [rng.random() for _ in range(d)]
            s = sum(wp) or 1.0
            wp = [x / s for x in wp]
            wraw = [wp[i] / (ch[i] - cl[i]) for i in range(d)]
            a = section(t, arms, wp, transform=frozen((cl, ch)))
            b = section(t, arms, wraw)
            agree += a == b
        print(f"  {fam:24s} regions={len(t):3d} arms={len(arms)}  identical sections: {agree}/{DRAWS}")

    print()
    print("=" * 78)
    print("B. INDEPENDENCE OF IRRELEVANT ALTERNATIVES, PER TRANSFORM")
    print("=" * 78)
    print("Two perturbations that no weighting can rationally respond to:")
    print("  A. drop an arm that is dominated at every region (never selectable)")
    print("  B. add an arm strictly worse than every real arm on every coordinate")
    print("A transform under which either changes a section is reading the arm set.")
    print()
    for fam in FAMILIES:
        t, arms = tables[fam]
        d = 2
        lo = [min(t[n][a][i] for n in t for a in arms) for i in range(d)]
        hi = [max(t[n][a][i] for n in t for a in arms) for i in range(d)]
        cl = [lo[i] * 0.5 for i in range(d)]
        ch = [hi[i] * 3.0 for i in range(d)]
        dom = dominated(t, arms)
        # B: an arm strictly worse everywhere by a wide factor.
        FACTOR = 32.0
        tb = {n: dict(t[n]) for n in t}
        for n in tb:
            tb[n]["synthetic-unselectable"] = tuple(
                max(t[n][a][i] for a in arms) * FACTOR for i in range(d)
            )
        armsb = arms + ["synthetic-unselectable"]
        for label, tf in [
            ("raw", None),
            ("min-max (arm-set range)", minmax),
            ("frozen (declared range)", frozen((cl, ch))),
        ]:
            movedA = movedB = 0
            picked_synth = 0
            for _ in range(DRAWS):
                w = [rng.random() for _ in range(d)]
                s = sum(w) or 1.0
                w = [x / s for x in w]
                base = section(t, arms, w, transform=tf)
                if dom:
                    kept = [a for a in arms if a not in dom]
                    movedA += section(t, kept, w, transform=tf) != base
                sb = section(tb, armsb, w, transform=tf)
                picked_synth += "synthetic-unselectable" in sb
                movedB += sb != base
            print(
                f"  {fam:24s} {label:24s} dominated={len(dom)} "
                f"A moved {movedA:5d}/{DRAWS}   B moved {movedB:5d}/{DRAWS}"
                f"   (B ever selected the added arm: {picked_synth})"
            )
        print()

    print("=" * 78)
    print("C. A BAND STATED AS A FRACTION OF THE ACHIEVABLE RANGE IS DATA-DEPENDENT")
    print("=" * 78)
    print("`100` section 7 states its tolerance as a percentage of (worst - best)")
    print("at the region. That denominator is a function of the arm set, so the")
    print("same perturbation B moves the band's width. Reported in the objective's")
    print("own units, per region, with and without the unselectable arm.")
    print()
    for fam in FAMILIES[:2]:
        t, arms = tables[fam]
        tb = {n: dict(t[n]) for n in t}
        for n in tb:
            tb[n]["synthetic-unselectable"] = tuple(
                max(t[n][a][i] for a in arms) * 32.0 for i in range(2)
            )
        armsb = arms + ["synthetic-unselectable"]
        w = [1.0, 0.0]  # speed only, so the objective is a time in nanoseconds
        print(f"  {fam}")
        print(f"    {'region':>10s} {'range without':>15s} {'range with':>13s} {'band 1% grows by':>18s}")
        for n in sorted(t):
            vals = [sum(wi * ci for wi, ci in zip(w, t[n][a])) for a in arms]
            valsb = [sum(wi * ci for wi, ci in zip(w, tb[n][a])) for a in armsb]
            r0 = max(vals) - min(vals)
            r1 = max(valsb) - min(valsb)
            print(f"    {n:10d} {r0:15.1f} {r1:13.1f} {r1 / r0 if r0 else float('nan'):17.1f}x")
        print()
    print("A band stated in the coordinate's own units, or as a fraction of the")
    print("region's own best arm, has no such denominator. `100` section 7.1 already")
    print("measures the floor that way: the control pair's apparent gap as a")
    print("percentage of runtime, which is per region and independent of the arm set.")


if __name__ == "__main__":
    main()
