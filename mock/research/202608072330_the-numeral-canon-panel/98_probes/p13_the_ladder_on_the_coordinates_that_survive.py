#!/usr/bin/env python3
"""p13. The ladder recomputed after p12 removed a coordinate.

p2 measured the ladder over three coordinates, one of which was the
interquartile range of the samples. p12 then found that the specific spread
differences p1b's rescue depended on are not distinguishable from zero, so the
third coordinate's standing is weaker than p2 assumed when it used it.

The honest response is to recompute the ladder over the two coordinates that
survive scrutiny without qualification, so the headline figures rest on the
model that is least open to the objection. This is not a replacement for p2:
p2's counts are correct for the coordinate set they name, and I13's notation
means a predicate carrying `cost coordinates = 3` claims nothing about two. It
is the same measurement over a narrower model, so a reader can see which of p2's
conclusions were about the ladder and which were about the third coordinate.

Six arms and two coordinates, against p6's five arms and two coordinates and
p2's six arms and three.
"""

import csv
import glob
import itertools
import json
import os
import re
import statistics
from fractions import Fraction

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))

BYTES_PER_ELEM = {
    "bitpack-carrier-d16": 2,
    "bitpack-carrier-d16-control": 2,
    "bitpack-carrier-d32": 4,
    "bitpack-carrier-d64": 8,
    "bitpack-carrier-packed": Fraction(13, 8),
    "bitpack-carrier-packed-simd": Fraction(13, 8),
}


def load():
    table = {}
    for path in sorted(glob.glob(os.path.join(BENCH, "bitpack-carrier-width_n*.csv"))):
        n = int(re.search(r"n(\d+)\.csv$", path).group(1))
        per = {}
        for row in csv.DictReader(open(path)):
            per.setdefault(row["variant"], []).append(float(row["algo_ns"]))
        table[n] = {a: (Fraction(statistics.median(per[a])), Fraction(BYTES_PER_ELEM[a]))
                    for a in sorted(BYTES_PER_ELEM)}
    return table


def dominates(x, y):
    return all(a <= b for a, b in zip(x, y)) and any(a < b for a, b in zip(x, y))


def admissible(table, r, a):
    return not any(dominates(table[r][b], table[r][a]) for b in table[r] if b != a)


def interval(table, regions, combo, strict_all):
    """Exact feasibility on w0 + w1 = 1 with w0 = t; returns (lo, hi, open_lo, open_hi)."""
    lo, hi = Fraction(0), Fraction(1)
    open_lo = open_hi = False
    for r, a in zip(regions, combo):
        ca = table[r][a]
        for b in table[r]:
            if b == a:
                continue
            cb = table[r][b]
            g0, g1 = (y - x for x, y in zip(ca, cb))
            need = strict_all and cb != ca
            m = g0 - g1
            if m > 0:
                bound = -g1 / m
                if bound > lo:
                    lo, open_lo = bound, need
                elif bound == lo and need:
                    open_lo = True
            elif m < 0:
                bound = -g1 / m
                if bound < hi:
                    hi, open_hi = bound, need
                elif bound == hi and need:
                    open_hi = True
            else:
                if g1 < 0 or (need and g1 <= 0):
                    return None
    if lo > hi or (lo == hi and (open_lo or open_hi)):
        return None
    return (lo, hi, open_lo, open_hi)


def rung(table, regions, combo):
    weak = interval(table, regions, combo, False)
    if weak is None:
        return None
    lo, hi, _, _ = weak
    positive = not (hi <= 0 or lo >= 1)
    if positive and lo == hi and (lo == 0 or lo == 1):
        positive = False
    strict = interval(table, regions, combo, True) is not None
    if positive and strict:
        return "L5"
    return "L4" if positive else "L3"


def main():
    table = load()
    regions = sorted(table)
    arms = sorted(BYTES_PER_ELEM)
    total = len(arms) ** len(regions)

    print(f"regions {len(regions)}, arms {len(arms)}, coordinates 2 "
          f"(median algo_ns, declared bytes per element)")
    print(f"L0 = {len(arms)}^{len(regions)} = {total}\n")

    counts = {"L1": 0, "L3": 0, "L4": 0, "L5": 0}
    l4 = []
    for combo in itertools.product(arms, repeat=len(regions)):
        if all(admissible(table, r, a) for r, a in zip(regions, combo)):
            counts["L1"] += 1
        v = rung(table, regions, combo)
        if v:
            counts["L3"] += 1
            if v in ("L4", "L5"):
                counts["L4"] += 1
                l4.append(combo)
            if v == "L5":
                counts["L5"] += 1

    print("counts")
    print(f"  L0 any section                     {total:6d}   100.000%")
    for k, label in (("L1", "Pareto-admissible"),
                     ("L3", "linear, w >= 0"),
                     ("L4", "linear, w > 0"),
                     ("L5", "linear, w > 0, unique argmin")):
        print(f"  {k} {label:32s} {counts[k]:6d}   "
              f"{100.0 * counts[k] / total:7.3f}%")
    print()

    print("dominated in every region, under two coordinates")
    for a in arms:
        where = [r for r in regions if not admissible(table, r, a)]
        if len(where) == len(regions):
            print(f"  {a}")
    print()

    print("the sections at the strictly-positive rung")
    for c in sorted(l4):
        print(f"  {[a.replace('bitpack-carrier-', '') for a in c]}")
    print()

    print("comparison across the three models measured in this probe set")
    print(f"  {'model':46s} {'L0':>8s} {'L1':>6s} {'L3':>5s} {'L4':>5s}")
    print(f"  {'p2:  6 arms, 3 coords (time, bytes, spread)':46s} "
          f"{46656:8d} {2048:6d} {117:5d} {58:5d}")
    print(f"  {'p6:  5 arms, 2 coords (time, bytes)':46s} "
          f"{15625:8d} {144:6d} {72:5d} {9:5d}")
    print(f"  {'p13: 6 arms, 2 coords (time, bytes)':46s} "
          f"{total:8d} {counts['L1']:6d} {counts['L3']:5d} {counts['L4']:5d}")
    print()
    print("  What is stable across all three: L4 is a small fraction of L1, which")
    print("  is a small fraction of L0, and the gap between the non-negative and")
    print("  the strictly-positive rungs is large. What is not stable: any")
    print("  particular number, which is p3's finding restated on real data rather")
    print("  than on random models.")


if __name__ == "__main__":
    main()
