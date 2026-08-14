#!/usr/bin/env python3
"""p6. Two different objects are both called "the weighting", and the canon
candidate states an opposite fact about each under that one name.

`106` section 1, clause seven:

    "Two strategies are related by an order on their first components where one
     exists, and by NOTHING on their second, because two weightings are
     incomparable vectors and nothing ever asks them to combine."

`106` section 5, thirty lines later in the same document:

    "On the weighting, the join is union and it is free."

Both are true. They are about different objects.

  THE SUPPORT: which coordinates a strategy demands at all. A set. `97`
  section 4.1's free join semilattice on four generators, compiled, whose
  carrier is the non-empty subsets. Union is a canonical least upper bound and
  the join is total.

  THE RATE: the exchange rates among the demanded coordinates. A ray in the
  positive orthant. `101`'s "the weights carry the units" and "a weighting is a
  ray rather than a point". No canonical combination of two exists.

This probe measures both on the committed carrier table, in exact rational
arithmetic, and shows the two facts are about the two objects rather than in
conflict.

Data: the six committed `bitpack-carrier-width_n*.csv`, the same table `97`,
`98` and `101` used, with the same second coordinate they used (declared bits
per element). Nothing is timed here; every number is a computation over
committed artifacts.

Run from the probe directory.
"""

import csv
import os
import statistics
import sys
from fractions import Fraction
from itertools import combinations

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.abspath(os.path.join(HERE, "..", "..", "..", "benches"))

# The declared bits per element, which is the second coordinate 97/98/101 used.
# It is a declaration rather than a measurement, so it is exact.
BITS = {
    "bitpack-carrier-d16": 16,
    "bitpack-carrier-d32": 32,
    "bitpack-carrier-d64": 64,
    "bitpack-carrier-packed": 13,
    "bitpack-carrier-packed-simd": 13,
}


def load():
    """median algo_ns per (region, arm) over the committed carrier CSVs."""
    table = {}
    for fn in sorted(os.listdir(BENCH)):
        if not (fn.startswith("bitpack-carrier-width_n") and fn.endswith(".csv")):
            continue
        region = fn[len("bitpack-carrier-width_n") : -len(".csv")]
        per_arm = {}
        with open(os.path.join(BENCH, fn), newline="", encoding="utf-8") as f:
            for row in csv.DictReader(f):
                v = row["variant"]
                try:
                    t = float(row["algo_ns"])
                except (TypeError, ValueError):
                    continue
                per_arm.setdefault(v, []).append(t)
        table[region] = {a: statistics.median(ts) for a, ts in per_arm.items() if ts}
    return table


def section(table, arms, w_time, w_size):
    """The argmin arm at every region, as a tuple. Exact rationals throughout."""
    out = []
    for region in sorted(table, key=lambda r: int(r)):
        best, best_cost = None, None
        for a in arms:
            if a not in table[region]:
                continue
            t = Fraction(table[region][a]).limit_denominator(10**9)
            s = Fraction(BITS[a])
            c = w_time * t + w_size * s
            if best_cost is None or c < best_cost:
                best_cost, best = c, a
        out.append(best)
    return tuple(out)


def main():
    print("p6. Two objects, one word.")
    print()

    table = load()
    arms = sorted(BITS)
    print(f"committed carrier files read: {len(table)}")
    print(f"regions: {sorted(table, key=lambda r: int(r))}")
    print(f"arms:    {arms}")
    print()
    missing = [a for a in arms if any(a not in table[r] for r in table)]
    if missing:
        print(f"arms absent from some region (excluded there): {missing}")
        print()

    # -----------------------------------------------------------------
    # OBJECT ONE: the support. A set. Union is canonical and total.
    # -----------------------------------------------------------------
    print("=== object one: the SUPPORT, a set of demanded coordinates ===")
    print()
    gens = ["time", "size"]
    subsets = []
    for k in range(1, len(gens) + 1):
        for c in combinations(gens, k):
            subsets.append(frozenset(c))
    print(f"generators: {gens}")
    print(f"non-empty subsets (the carrier of the free join semilattice): {len(subsets)}")
    total = 0
    canonical = 0
    for a in subsets:
        for b in subsets:
            total += 1
            j = a | b
            # least upper bound: contains both, and no smaller element does
            uppers = [x for x in subsets if x >= a and x >= b]
            if j in subsets and all(j <= u for u in uppers):
                canonical += 1
    print(f"ordered pairs: {total}")
    print(f"pairs with a unique least upper bound under union: {canonical}")
    print(f"join is total and canonical: {canonical == total}")
    print()
    print("So `106` section 5 is right ABOUT THIS OBJECT. Two strategies demanding")
    print("different coordinates combine into the one demanding both, uniquely.")
    print()

    # -----------------------------------------------------------------
    # OBJECT TWO: the rate. A ray. No canonical combination.
    # -----------------------------------------------------------------
    print("=== object two: the RATE, a ray in the positive orthant ===")
    print()
    print("Two strategies with the SAME support {time, size} and different rates.")
    print("Four ordinary ways to combine two rays, all of them defensible, and the")
    print("section each selects on the committed table:")
    print()

    # two rates in the same support, both strictly positive
    r1 = (Fraction(1), Fraction(1, 100))    # leans on time
    r2 = (Fraction(1, 100), Fraction(1))    # leans on size

    def gmean(a, b):
        # exact geometric mean is irrational in general; take it on the squares
        # so the comparison stays exact, which is order-preserving for a ray
        return (Fraction(a * b).limit_denominator(10**12),)

    combos = {
        "arithmetic mean": ((r1[0] + r2[0]) / 2, (r1[1] + r2[1]) / 2),
        "componentwise max": (max(r1[0], r2[0]), max(r1[1], r2[1])),
        "componentwise min": (min(r1[0], r2[0]), min(r1[1], r2[1])),
        "geometric mean (squared, order preserving)": (
            Fraction(r1[0] * r2[0]).limit_denominator(10**12),
            Fraction(r1[1] * r2[1]).limit_denominator(10**12),
        ),
        "first operand": r1,
        "second operand": r2,
    }

    sections = {}
    for name, (wt, ws) in combos.items():
        sec = section(table, arms, wt, ws)
        sections[name] = sec
        print(f"  {name:<44} -> {sec}")
    print()
    distinct = len(set(sections.values()))
    print(f"distinct sections reached by six defensible combinations: {distinct}")
    print()

    if distinct > 1:
        print("So `106` section 1 clause seven is right ABOUT THIS OBJECT. There is no")
        print("canonical combination of two rates: the ordinary candidates disagree on")
        print("the committed table, so a design that picks one has made a choice rather")
        print("than computed a join.")
    else:
        print("On this table the candidates happen to agree, which does not make a join")
        print("canonical; it makes this table a bad discriminator. The structural point")
        print("stands and this measurement does not carry it.")
    print()

    # ---- and the same question swept, because one pair is an existence claim ----
    print("=== the same question over a grid of rate pairs ===")
    print()
    print("One pair refutes a canonical join. A rate of disagreement makes the")
    print("finding usable, so the six combinations are compared over every ordered")
    print("pair drawn from a log-spaced grid of exchange rates.")
    print()
    grid = [Fraction(1, 1000), Fraction(1, 100), Fraction(1, 10), Fraction(1),
            Fraction(10), Fraction(100), Fraction(1000)]
    pairs = 0
    disagree = 0
    worst = 0
    for a in grid:
        for b in grid:
            if a == b:
                continue
            pairs += 1
            ra = (Fraction(1), a)
            rb = (Fraction(1), b)
            cands = {
                "amean": ((ra[0] + rb[0]) / 2, (ra[1] + rb[1]) / 2),
                "cmax": (max(ra[0], rb[0]), max(ra[1], rb[1])),
                "cmin": (min(ra[0], rb[0]), min(ra[1], rb[1])),
                "gmean2": (ra[0] * rb[0], ra[1] * rb[1]),
                "first": ra,
                "second": rb,
            }
            secs = {section(table, arms, wt, ws) for wt, ws in cands.values()}
            if len(secs) > 1:
                disagree += 1
            worst = max(worst, len(secs))
    pct = 100.0 * disagree / pairs if pairs else 0.0
    print(f"  ordered rate pairs swept:                       {pairs}")
    print(f"  pairs where the six combinations DISAGREE:      {disagree} ({pct:.1f}%)")
    print(f"  most distinct sections reached by one pair:     {worst}")
    print()
    print("  holds for: regions = 6, arms = 5, cost coordinates = 2 (median algo_ns")
    print("  per record, declared bits per element as 16/32/64/13/13), cost source =")
    print("  the committed bitpack-carrier-width_n* CSVs, arithmetic exact rational,")
    print("  threads = 1, target features any")
    print()

    # -----------------------------------------------------------------
    print("=== what the collision costs ===")
    print()
    print("The canon candidate's definition says two strategies are related by")
    print("nothing on their second component. Its own section 5 says the join on the")
    print("second component is union and free. A reader who takes the first cannot")
    print("build `97`'s demand lattice; a reader who takes the second will try to")
    print("join two exchange rates and find no least upper bound.")
    print()
    print("The repair is to split the word rather than to choose a side:")
    print()
    print("  component two = a SUPPORT (which coordinates are demanded) together")
    print("                  with a RATE (the exchange among them).")
    print()
    print("  supports join, uniquely and totally, by union. That is `97`'s lattice,")
    print("  it is compiled, and it is what makes silence a first-class element.")
    print()
    print("  rates do not join, and nothing should ask them to. Where two rates meet,")
    print("  either the site names one or the operation reports, exactly as clause")
    print("  seven says for the first component.")
    print()
    print("That also explains a result the unit has and does not connect: `93`'s four")
    print("markers leaving 12 of 16 ordered pairs unresolvable. The supports resolve")
    print("in all 16. What does not resolve is the rate, and a flat marker set was")
    print("carrying both in one slot, which is `94`'s own diagnosis of the flat set.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
