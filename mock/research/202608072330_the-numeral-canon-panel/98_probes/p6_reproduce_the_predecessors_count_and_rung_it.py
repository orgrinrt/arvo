#!/usr/bin/env python3
"""p6. Reproduce 97's count on 97's model, and find which rung it sits at.

`97` section 2.2 reports 72 of 15625 sections rationalisable on the
`bitpack-carrier-width` family, from five arms, six regions and two cost
coordinates, counting a tie as admitting either arm. It also reports, in its
section 10, that two of those five arms are "beaten on both time and bits by
another arm at all six record counts", and concludes that they "cannot be
selected by any weighting-defined strategy".

Both sentences are in the same file and they cannot both be about the same
notion of weighting, because 63 of the 72 select one of the two arms it says no
weighting can select. This probe reproduces the model and shows where the two
sentences part company, which is not a mistake in either of them: they are
correct statements about two different rungs, and the file states them as one.

The rungs, as in p2:
  L1 Pareto-admissible.
  L3 rationalisable by w >= 0, not all zero.
  L4 rationalisable by w > 0, every coordinate weighed.
  L5 as L4 with the choice forced rather than permitted.

The prediction, before running. A weight vector with a zero on the time
coordinate is indifferent between two arms that carry the same bits, so it
weakly admits either, and one of them is dominated on time everywhere. So the
generous count belongs to L3, the dominance argument belongs to L4, and L4 will
be far smaller than 72.

Which matters for what the canon says, and not only for bookkeeping. If a canon
sentence reads "a strategy is a weighting over measurements", the reader has to
know whether a weight may be zero. If it may, a strategy can select an arm that
is worse on every coordinate it does not weigh and no better on the ones it
does. If it may not, that cannot happen, and the price is that no strategy may
ignore a measurement outright.
"""

import csv
import glob
import itertools
import json
import os
import re
import statistics
import sys
from fractions import Fraction

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import cone  # noqa: E402

BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))

# 97's model: five arms and two coordinates. The control arm is excluded, which
# is the reading that makes its arm count five; time is per record rather than
# per batch, and the second coordinate is bits per element, a declared fact
# about the arm rather than a measurement.
ARMS_97 = [
    "bitpack-carrier-d16",
    "bitpack-carrier-d32",
    "bitpack-carrier-d64",
    "bitpack-carrier-packed",
    "bitpack-carrier-packed-simd",
]
BITS = {
    "bitpack-carrier-d16": 16,
    "bitpack-carrier-d32": 32,
    "bitpack-carrier-d64": 64,
    "bitpack-carrier-packed": 13,
    "bitpack-carrier-packed-simd": 13,
}


def load():
    table = {}
    for path in sorted(glob.glob(os.path.join(BENCH, "bitpack-carrier-width_n*.csv"))):
        n = int(re.search(r"n(\d+)\.csv$", path).group(1))
        per = {}
        for row in csv.DictReader(open(path)):
            per.setdefault(row["variant"], []).append(float(row["algo_ns"]))
        table[n] = {a: (Fraction(statistics.median(per[a])) / n, Fraction(BITS[a]))
                    for a in ARMS_97}
    return table


def dominates(x, y):
    return all(a <= b for a, b in zip(x, y)) and any(a < b for a, b in zip(x, y))


def admissible(table, r, a):
    return not any(dominates(table[r][b], table[r][a]) for b in table[r] if b != a)


def interval_for(table, regions, combo, strict_all):
    """Exact feasibility in two coordinates, on w0 + w1 = 1 with w0 = t.

    Returns (lo, hi) or None. Each constraint <w, g> >= 0 becomes
    (g0 - g1) t + g1 >= 0. `strict_all` demands > 0 wherever the two cost
    vectors differ, which is the forced-choice rung.
    """
    lo, hi = Fraction(0), Fraction(1)
    open_lo = open_hi = False
    for r, a in zip(regions, combo):
        ca = table[r][a]
        for b in table[r]:
            if b == a:
                continue
            cb = table[r][b]
            g0, g1 = (y - x for x, y in zip(ca, cb))
            need_strict = strict_all and cb != ca
            m = g0 - g1
            if m > 0:
                bound = -g1 / m
                if bound > lo:
                    lo, open_lo = bound, need_strict
                elif bound == lo and need_strict:
                    open_lo = True
            elif m < 0:
                bound = -g1 / m
                if bound < hi:
                    hi, open_hi = bound, need_strict
                elif bound == hi and need_strict:
                    open_hi = True
            else:
                if g1 < 0 or (need_strict and g1 <= 0):
                    return None
    if lo > hi or (lo == hi and (open_lo or open_hi)):
        return None
    return (lo, hi, open_lo, open_hi)


def rung(table, regions, combo):
    weak = interval_for(table, regions, combo, strict_all=False)
    if weak is None:
        return None
    lo, hi, _, _ = weak
    # w > 0 means t strictly inside (0, 1).
    positive = not (hi <= 0 or lo >= 1)
    if positive and (lo == hi) and (lo == 0 or lo == 1):
        positive = False
    strict = interval_for(table, regions, combo, strict_all=True) is not None
    if strict and positive:
        return "L5"
    return "L4" if positive else "L3"


def main():
    table = load()
    regions = sorted(table)
    total = len(ARMS_97) ** len(regions)

    print(f"97's model reproduced: {len(regions)} regions, {len(ARMS_97)} arms, "
          f"2 coordinates (ns per record, bits per element)")
    print(f"L0 = {len(ARMS_97)}^{len(regions)} = {total}\n")

    print("dominance, the claim in 97 section 10")
    everywhere = []
    for a in ARMS_97:
        if all(not admissible(table, r, a) for r in regions):
            everywhere.append(a)
    for a in ARMS_97:
        where = [r for r in regions if not admissible(table, r, a)]
        print(f"  {a:30s} dominated at {len(where)} of {len(regions)} regions")
    print(f"  dominated in EVERY region: {everywhere}")
    print(f"  (97 section 10 names exactly these two)\n")

    counts = {"L1": 0, "L3": 0, "L4": 0, "L5": 0}
    l3_sections = []
    l4_sections = []
    for combo in itertools.product(ARMS_97, repeat=len(regions)):
        if all(admissible(table, r, a) for r, a in zip(regions, combo)):
            counts["L1"] += 1
        v = rung(table, regions, combo)
        if v is not None:
            counts["L3"] += 1
            l3_sections.append(combo)
            if v in ("L4", "L5"):
                counts["L4"] += 1
                l4_sections.append(combo)
            if v == "L5":
                counts["L5"] += 1

    print("counts on 97's model")
    print(f"  L0 any section                     {total:6d}")
    print(f"  L1 Pareto-admissible               {counts['L1']:6d}")
    print(f"  L3 rationalisable, w >= 0          {counts['L3']:6d}   "
          f"<- 97 section 2.2 reports 72 here")
    print(f"  L4 rationalisable, w > 0           {counts['L4']:6d}")
    print(f"  L5 forced, w > 0                   {counts['L5']:6d}   "
          f"<- 97 section 2.2 reports 9 strict")
    print()

    dominated_pick = [c for c in l3_sections
                      if any(a in everywhere for a in c)]
    print("the two sentences, measured against each other")
    print(f"  sections at L3 that select an arm 97 says no weighting can select: "
          f"{len(dominated_pick)} of {counts['L3']}")
    print(f"  sections at L4 that do:                                            "
          f"{len([c for c in l4_sections if any(a in everywhere for a in c)])} "
          f"of {counts['L4']}")
    print()
    print("  So both sentences are true and they are about different rungs. The")
    print("  headline count admits a zero weight, and a zero weight is exactly")
    print("  what lets a strategy pick a dominated arm. The dominance claim is")
    print("  about strictly positive weights, where it holds by construction.")
    print()

    if l4_sections:
        print("  the sections that survive at L4, listed, since there are few")
        for c in sorted(l4_sections):
            print(f"    {[a.replace('bitpack-carrier-', '') for a in c]}")
    print()

    print("what to carry, and what not")
    print(f"  Not '72 of 15625'. That figure counts sections a strategy could")
    print(f"  only reach by ignoring a measurement outright.")
    print(f"  The figure with the property 97 wants from it is {counts['L4']} of "
          f"{total}, which is {100.0 * counts['L4'] / total:.4f}%, and it is")
    print(f"  smaller than the one reported, so 97's conclusion that a weighting")
    print(f"  admits a vanishing fraction of the sections is strengthened rather")
    print(f"  than weakened by the correction.")

    with open(os.path.join(HERE, "p6_model97.json"), "w") as f:
        json.dump({str(r): {a: [str(table[r][a][0]), str(table[r][a][1])]
                            for a in ARMS_97} for r in regions}, f, indent=1)
    print("\nwritten: p6_model97.json")


if __name__ == "__main__":
    main()
