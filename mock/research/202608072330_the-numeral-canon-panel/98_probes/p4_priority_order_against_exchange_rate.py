#!/usr/bin/env python3
"""p4. Naming a priority is not the same as naming an exchange rate.

If a strategy is "a weighting over measurements", there are two very different
things that could mean, and op's own statements of the four strategies pick one
of them explicitly and repeatedly.

  PRIORITY. The strategy names which measurement outranks which, and the ranking
  is absolute: a gain on a higher-ranked coordinate justifies any loss on a
  lower-ranked one. This is a lexicographic order.

  EXCHANGE RATE. The strategy names how much of one measurement a unit of
  another is worth. A loss on the primary is accepted when the gain elsewhere is
  large enough, and refused when it is not. This is a weighting with finite
  ratios.

This probe establishes four things about the difference.

  1. A priority is always realisable as an exchange rate on a finite model, so
     the two are not rival formalisms and a canon stating priorities is
     implementable as weights. Tested over random models and every coordinate
     permutation.

  2. The converse fails by a wide margin. On the real carrier table there are at
     most d! = 6 priority orders and p2 measured 58 sections realisable by some
     strictly positive weighting. So a priority carries strictly less information
     than a weighting, and the gap is most of the space.

  3. The sections a priority CANNOT reach are exhibited, so the difference is
     concrete rather than a cardinality argument.

  4. A THRESHOLD rule, "minimise A subject to B at most t", is not realisable as
     any weighting at all, with a compiled witness. This bounds the weighting
     model from the other side: if a strategy is a weighting, then no strategy
     may be stated as a hard bound on a measurement, because that shape has no
     weighting that expresses it.

Why this matters for what a strategy is. Op states each strategy as a primary
concern AND THEN explicitly refuses to make it absolute. Warm "does not make it
absolutely required, if mimicking is consistently just worse choice" (I4). Hot
"can sacrifice soundness ... but should not lose it for nothing, instead,
provable meaningful gains" (I5). Cold "has more leeway to do things
non-efficient" and "does not have to drop efficiency wins elsewhere" (I6).
Precise "sacrifices as much performance and efficiency as makes sense" (I7).
Four intents, four refusals of the absolute reading. Under 1 and 2 that is not a
stylistic hedge: it is the difference between a lexicographic order and a
weighting with finite ratios, and it is measurable.
"""

import itertools
import json
import os
import random
import sys
from fractions import Fraction

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import cone  # noqa: E402

COORDS = ("time", "bytes", "spread")


def load_real():
    with open(os.path.join(HERE, "p1_cost_table.json")) as f:
        raw = json.load(f)
    regions = sorted(int(k) for k in raw)
    arms = sorted(raw[str(regions[0])])
    table = {r: {a: tuple(Fraction(raw[str(r)][a][c]) for c in COORDS) for a in arms}
             for r in regions}
    return table, regions, arms


def gs_for(table, r, a):
    ca = table[r][a]
    return [(tuple(y - x for x, y in zip(ca, table[r][b])), table[r][b] != ca)
            for b in table[r] if b != a]


def classify(table, regions, combo):
    gs = []
    for r, a in zip(regions, combo):
        gs.extend(gs_for(table, r, a))
    poly = cone.region([g for g, _ in gs])
    if not poly:
        return None
    if not cone.has_strictly_positive_weights(poly):
        return "nonneg"
    if cone.has_point_strict_on(poly, [g for g, d in gs if d]):
        return "strict"
    return "positive"


def lex_section(table, regions, arms, perm):
    """The lexicographically minimal arm at each region, under a coordinate order."""
    out = []
    for r in regions:
        best = min(arms, key=lambda a: tuple(table[r][a][k] for k in perm))
        out.append(best)
    return tuple(out)


def weighted_section(table, regions, arms, w):
    out = []
    for r in regions:
        best = min(arms, key=lambda a: sum(wi * ci for wi, ci in zip(w, table[r][a])))
        out.append(best)
    return tuple(out)


# --------------------------------------------------------------------------
# 1. A priority is realisable as an exchange rate, on any finite model.
# --------------------------------------------------------------------------

def check_lex_is_linear(trials, seed):
    rng = random.Random(seed)
    R, A, D = 4, 5, 3
    failures = []
    checked = 0
    for _ in range(trials):
        table = {r: {a: tuple(Fraction(rng.randint(1, 60)) for _ in range(D))
                     for a in range(A)} for r in range(R)}
        regions = list(range(R))
        arms = list(range(A))
        for perm in itertools.permutations(range(D)):
            combo = lex_section(table, regions, arms, perm)
            verdict = classify(table, regions, combo)
            checked += 1
            if verdict not in ("positive", "strict"):
                failures.append((table, perm, combo, verdict))
    return checked, failures


# --------------------------------------------------------------------------
# 4. A threshold rule has no weighting.
# --------------------------------------------------------------------------

def threshold_witness():
    """Minimise coordinate 0 subject to coordinate 1 at most 3.

    A = (0, 4) is excluded by the bound however good its first coordinate is.
    Among the rest, C beats B on the first coordinate. The rule chooses C.

    No weighting chooses C: preferring C to A needs 5 w0 + 3 w1 <= 4 w1, so
    w1 >= 5 w0, and preferring C to B needs 5 w0 + 3 w1 <= 10 w0, so
    3 w1 <= 5 w0. Together 15 w0 <= 3 w1 <= 5 w0, impossible for w0 > 0.
    """
    table = {0: {"A": (Fraction(0), Fraction(4)),
                 "B": (Fraction(10), Fraction(0)),
                 "C": (Fraction(5), Fraction(3))}}
    return table, [0], ["A", "B", "C"]


def classify2d(table, regions, combo):
    """Exact feasibility in TWO coordinates, done directly.

    An earlier version of this function padded the two-coordinate model with a
    zero third coordinate and handed it to the three-coordinate solver. That is
    wrong and it reported the threshold choice as feasible: every difference
    vector has a zero in the padded slot, so the weight vector (0, 0, 1) meets
    every constraint trivially and rationalises anything. The lesson is worth
    keeping in the file rather than deleting: a degenerate coordinate no
    alternative differs on makes every section weakly rationalisable, which is
    true and useless, and it is exactly the shape a padded model produces by
    accident.

    Done properly: normalise w0 + w1 = 1, write w0 = t, and each constraint
    <w, g> >= 0 becomes (g0 - g1) t + g1 >= 0, a linear condition on t in the
    closed interval [0, 1]. Intersect the intervals exactly.
    """
    lo, hi = Fraction(0), Fraction(1)
    strict_lo, strict_hi = set(), set()
    for r, a in zip(regions, combo):
        ca = table[r][a]
        for b in table[r]:
            if b == a:
                continue
            cb = table[r][b]
            g0, g1 = (y - x for x, y in zip(ca, cb))
            m = g0 - g1
            if m > 0:
                lo = max(lo, -g1 / m)
            elif m < 0:
                hi = min(hi, -g1 / m)
            elif g1 < 0:
                return None
    if lo > hi:
        return None
    return "nonneg" if (lo == 0 and hi == 0) or (lo == 1 and hi == 1) else "positive"


def main():
    table, regions, arms = load_real()

    print("1. a priority is always realisable as an exchange rate")
    checked, failures = check_lex_is_linear(trials=200, seed=20260814)
    print(f"   {checked} lexicographic sections over 200 random 4x5x3 models, "
          f"one per coordinate permutation")
    print(f"   realisable by a strictly positive weighting: "
          f"{checked - len(failures)} of {checked}")
    if failures:
        print(f"   FAILURES: {len(failures)}, first perm {failures[0][1]}, "
              f"verdict {failures[0][3]}")
    else:
        print("   no failures. So a canon that states which measurement outranks")
        print("   which has said something a weighting can implement, and the two")
        print("   formalisms are not rivals at that level.")
    print()

    print("2. the converse fails, on the real carrier table")
    lexes = {}
    for perm in itertools.permutations(range(len(COORDS))):
        lexes[perm] = lex_section(table, regions, arms, perm)
    distinct = set(lexes.values())
    with open(os.path.join(HERE, "p2_rungs.json")) as f:
        rungs = json.load(f)
    l4 = {tuple(x) for x in rungs["L4"]}
    print(f"   coordinate permutations: {len(lexes)}, distinct sections they "
          f"produce: {len(distinct)}")
    print(f"   sections realisable by a strictly positive weighting (p2, L4): {len(l4)}")
    inside = [c for c in distinct if c in l4]
    print(f"   of the priority sections, in L4: {len(inside)} of {len(distinct)}")
    print(f"   sections a weighting reaches and NO priority order reaches: "
          f"{len(l4 - distinct)}")
    print("   So naming which measurement outranks which fixes at most a handful")
    print("   of the available behaviours. The rest need an actual rate.")
    print()

    print("   the priority sections, named")
    for perm, combo in lexes.items():
        order = " > ".join(COORDS[k] for k in perm)
        print(f"     {order:28s} -> {[a.replace('bitpack-carrier-', '') for a in combo]}")
    print()

    print("3. three sections a weighting reaches that no priority does")
    for combo in sorted(l4 - distinct)[:3]:
        print(f"     {[a.replace('bitpack-carrier-', '') for a in combo]}")
    print()

    print("   and one of them, with a weight that realises it, to show the rate")
    target = sorted(l4 - distinct)[0]
    gs = []
    for r, a in zip(regions, target):
        gs.extend(gs_for(table, r, a))
    poly = cone.region([g for g, _ in gs])
    w = cone.weights_at(poly)
    print(f"     section {[a.replace('bitpack-carrier-', '') for a in target]}")
    print(f"     realised by w = ({', '.join(str(x) for x in w)}) on "
          f"(time, bytes, spread)")
    print(f"     check, by recomputing the argmin under that weight: "
          f"{weighted_section(table, regions, arms, w) == target}")
    print()

    print("4. a threshold rule has no weighting at all")
    t2, r2, a2 = threshold_witness()
    print("   alternatives, on (primary, bounded): "
          + ", ".join(f"{k} = {tuple(int(x) for x in v)}" for k, v in t2[0].items()))
    print("   rule: minimise the primary subject to the bounded coordinate <= 3")
    admitted = [k for k, v in t2[0].items() if v[1] <= 3]
    pick = min(admitted, key=lambda k: t2[0][k][0])
    print(f"   admitted by the bound: {admitted}, of which the rule picks {pick}")
    for k in t2[0]:
        v = classify2d(t2, r2, (k,))
        print(f"   choosing {k}: realisable by a weighting? "
              f"{'no' if v is None else v}")
    print(f"   the rule's choice {pick} is not realisable, so a strategy stated as")
    print("   a hard bound on a measurement is outside the weighting model, and a")
    print("   canon saying 'a strategy is a weighting' has thereby said that no")
    print("   strategy is a hard bound. That is a real consequence and it should")
    print("   be said out loud rather than discovered later.")


if __name__ == "__main__":
    main()
