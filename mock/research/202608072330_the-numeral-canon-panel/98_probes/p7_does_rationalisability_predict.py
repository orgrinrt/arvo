#!/usr/bin/env python3
"""p7. Does the rationalisability constraint buy what it is argued to buy.

`97` section 2.4 argues for requiring a strategy's table to be rationalisable,
and the argument is not that it is elegant. It is this:

  "If one weighting explains the whole table, then a consumer reading 'this one
  weighs time heavily' can predict what it will do at a region nobody has
  benched, and a later expert adding a region has a rule for filling it in. If
  no weighting explains the table, the name carries no information beyond the
  rows already written, and every new region is a fresh decision with nothing to
  derive it from."

That is a claim about prediction and it is testable, so it should be tested
rather than agreed with. `97` itself says the section is "an argument rather
than a measurement".

The test is leave-one-out. Take a rationalisable table. Hide one region. Fit the
weight cone to the remaining five, exactly. Then ask which arms at the hidden
region are the minimiser for SOME weight still in that cone. Three outcomes:

  determinate   exactly one arm survives, and the constraint predicted the
                hidden row from the others.
  narrowed      more than one survives but fewer than were admissible, so the
                constraint carries partial information.
  vacuous       every admissible arm survives, so the constraint predicted
                nothing at that region and the name really is a lookup key.

The baseline to beat is the count of Pareto-admissible arms at the hidden
region, because that is what a reader knows without any weighting at all. A
constraint that narrows four arms to four has bought nothing.

Run on both models: the three-coordinate table from p1 and 97's own
two-coordinate model from p6.
"""

import json
import os
import sys
from fractions import Fraction

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import cone  # noqa: E402


def load3():
    with open(os.path.join(HERE, "p1_cost_table.json")) as f:
        raw = json.load(f)
    regions = sorted(int(k) for k in raw)
    arms = sorted(raw[str(regions[0])])
    table = {r: {a: tuple(Fraction(raw[str(r)][a][c])
                          for c in ("time", "bytes", "spread")) for a in arms}
             for r in regions}
    return table, regions, arms


def load2():
    with open(os.path.join(HERE, "p6_model97.json")) as f:
        raw = json.load(f)
    regions = sorted(int(k) for k in raw)
    arms = sorted(raw[str(regions[0])])
    # Pad to three coordinates with a constant so the same solver applies. A
    # constant coordinate contributes a zero difference for every pair, so it
    # adds no constraint and cannot change any verdict; it is padding for the
    # solver's arity and nothing else. The degenerate case p4 ran into does not
    # arise here because the strictly-positive test is applied on the other two.
    table = {r: {a: (Fraction(raw[str(r)][a][0]), Fraction(raw[str(r)][a][1]),
                     Fraction(0)) for a in arms} for r in regions}
    return table, regions, arms


def dominates(x, y):
    return all(a <= b for a, b in zip(x, y)) and any(a < b for a, b in zip(x, y))


def admissible(table, r, a):
    return not any(dominates(table[r][b], table[r][a]) for b in table[r] if b != a)


def gs_for(table, r, a):
    ca = table[r][a]
    return [tuple(y - x for x, y in zip(ca, table[r][b])) for b in table[r] if b != a]


def feasible_positive(gs, pad2d):
    poly = cone.region(gs)
    if not poly:
        return False
    if pad2d:
        # Only the first two weights need to be positive; the third multiplies a
        # coordinate every arm shares and is free.
        return any(x > 0 for x, _ in poly) and any(y > 0 for _, y in poly)
    return cone.has_strictly_positive_weights(poly)


def l4_sections(table, regions, arms, pad2d):
    found = []

    def rec(i, gs, chosen):
        if not feasible_positive(gs, pad2d):
            return
        if i == len(regions):
            found.append(tuple(chosen))
            return
        r = regions[i]
        for a in arms:
            rec(i + 1, gs + gs_for(table, r, a), chosen + [a])

    rec(0, [], [])
    return found


def run(label, table, regions, arms, pad2d):
    sections = l4_sections(table, regions, arms, pad2d)
    print(f"{label}")
    print(f"  regions {len(regions)}, arms {len(arms)}, "
          f"sections at the strictly-positive rung: {len(sections)}")
    adm = {r: [a for a in arms if admissible(table, r, a)] for r in regions}
    print(f"  Pareto-admissible arms per region (the baseline a reader already "
          f"has): {[len(adm[r]) for r in regions]}")
    print()

    tally = {"determinate": 0, "narrowed": 0, "vacuous": 0}
    correct = 0
    trials = 0
    widths = []
    for sec in sections:
        for i, r in enumerate(regions):
            gs = []
            for j, rr in enumerate(regions):
                if j != i:
                    gs.extend(gs_for(table, rr, sec[j]))
            survivors = [a for a in arms
                         if feasible_positive(gs + gs_for(table, r, a), pad2d)]
            trials += 1
            widths.append(len(survivors))
            base = len(adm[r])
            if len(survivors) == 1:
                tally["determinate"] += 1
            elif len(survivors) < base:
                tally["narrowed"] += 1
            else:
                tally["vacuous"] += 1
            if sec[i] in survivors and len(survivors) == 1:
                correct += 1

    print(f"  leave-one-out trials: {trials}")
    for k in ("determinate", "narrowed", "vacuous"):
        print(f"    {k:12s} {tally[k]:6d}   {100.0 * tally[k] / trials:6.2f}%")
    print(f"  mean surviving arms at the hidden region: "
          f"{sum(widths) / len(widths):.3f}")
    print(f"  mean admissible arms at the hidden region: "
          f"{sum(len(adm[r]) for r in regions) * len(sections) / trials:.3f}")
    print(f"  hidden row recovered uniquely and correctly: {correct} of {trials} "
          f"({100.0 * correct / trials:.2f}%)")
    print()
    return tally, trials


def main():
    print("Does requiring one weighting to explain the whole table let a reader")
    print("predict a row that weighting was not fitted to?\n")

    t3, r3, a3 = load3()
    run("three-coordinate table (time, bytes, spread), p1", t3, r3, a3, pad2d=False)

    t2, r2, a2 = load2()
    run("97's two-coordinate model (ns per record, bits), p6", t2, r2, a2, pad2d=True)

    print("reading")
    print("  The constraint is not a lookup-key-versus-rule dichotomy. It buys a")
    print("  narrowing, and how much it buys is a property of the table rather")
    print("  than of the constraint. Where a strategy's remaining freedom at the")
    print("  hidden region is more than one arm, the name is predictive about")
    print("  which arms are POSSIBLE and silent about which is taken, which is")
    print("  weaker than the argument for the constraint claims and stronger")
    print("  than nothing.")
    print()
    print("  Two honest bounds on this probe. Leave-one-out over the regions a")
    print("  table was fitted on is not the same as a genuinely new region with")
    print("  costs nobody has seen, and the second is what the argument is about;")
    print("  this measures the first because the second needs a measurement that")
    print("  does not exist. And a determinate rate is a fact about these two")
    print("  tables, which p3 showed varies by a factor of tens across tables of")
    print("  the same shape.")


if __name__ == "__main__":
    main()
