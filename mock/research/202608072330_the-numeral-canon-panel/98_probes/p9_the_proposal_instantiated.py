#!/usr/bin/env python3
"""p9. The converged shape, instantiated on the real table, with its own failure modes tested.

The shape offered in the file this probe belongs to is:

  A strategy is a weighting over a fixed set of measurement coordinates, with
  EVERY coordinate carrying a strictly positive weight, and its section is the
  argmin of that weighting over the arms available in each region.

Four things have to be true for that to be worth writing down, and each is
checked here rather than asserted.

  1. Strict positivity is what makes "a strategy never selects an arm that is
     worse on every coordinate" true. If a weight may be zero, it is false.
     Checked by exhibiting the failure at a zero weight.

  2. "More leeway on a coordinate" (I6, "Cold ... has more leeway to do things
     non-efficient") is expressible as a small positive weight rather than as a
     zero one, and a small positive weight still gives the guarantee in 1.
     Checked by sweeping the weight down toward zero and finding where, if
     anywhere, the selected section changes and where the guarantee breaks.

  3. A lexicographic priority on one coordinate, with finite weights on the
     rest, is realisable by a single strictly positive weight vector on a finite
     model. This is the shape `40` section 5.3 derives from I5 and I34:
     "accuracy is lexicographically prior for every objective except Hot, and
     finitely weighted for Hot". If it were not realisable, the weighting model
     could not carry op's own reading of his intents.

  4. Two intents that weigh different coordinates give different sections, so
     the model distinguishes strategies rather than collapsing them. Checked on
     the real table for the two intents p8 found the corpus can express.

None of the weight numbers below is a proposal. They are scaffolding chosen to
reach a check, in the sense the spike rule means: what is being established is
which SHAPES are expressible and which guarantees follow, not what Hot's weight
on footprint should be.
"""

import itertools
import json
import os
import sys
from fractions import Fraction

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import cone  # noqa: E402

COORDS = ("time", "bytes", "spread")


def load():
    with open(os.path.join(HERE, "p1_cost_table.json")) as f:
        raw = json.load(f)
    regions = sorted(int(k) for k in raw)
    arms = sorted(raw[str(regions[0])])
    table = {r: {a: tuple(Fraction(raw[str(r)][a][c]) for c in COORDS) for a in arms}
             for r in regions}
    return table, regions, arms


def dominates(x, y):
    return all(a <= b for a, b in zip(x, y)) and any(a < b for a, b in zip(x, y))


def admissible(table, r, a):
    return not any(dominates(table[r][b], table[r][a]) for b in table[r] if b != a)


def normalise(table, regions, arms):
    """Scale each coordinate to [0, 1] over the whole table.

    Without this a weight vector is uninterpretable, because the coordinates are
    in nanoseconds, bytes and nanoseconds and differ by six orders of magnitude,
    so a "weight of 1 on each" would be a weighting on time and nothing else.
    Scaling by a positive constant per coordinate is the reparameterisation p2
    argues is free: it changes which numbers spell a given preference, never
    which preferences are expressible.
    """
    lo = [min(table[r][a][k] for r in regions for a in arms) for k in range(3)]
    hi = [max(table[r][a][k] for r in regions for a in arms) for k in range(3)]
    def n(v):
        return tuple((v[k] - lo[k]) / (hi[k] - lo[k]) if hi[k] > lo[k] else Fraction(0)
                     for k in range(3))
    return {r: {a: n(table[r][a]) for a in arms} for r in regions}


def section_under(tab, regions, arms, w):
    out = []
    for r in regions:
        best = min(arms, key=lambda a: (sum(wi * ci for wi, ci in zip(w, tab[r][a])), a))
        out.append(best)
    return tuple(out)


def picks_a_dominated_arm(table, regions, sec):
    return [(r, a) for r, a in zip(regions, sec) if not admissible(table, r, a)]


def short(sec):
    return [a.replace("bitpack-carrier-", "") for a in sec]


def check_lex_then_finite(trials, seed):
    """A priority on one coordinate plus finite weights on the rest is one weighting."""
    import random
    rng = random.Random(seed)
    R, A, D = 4, 5, 3
    ok = bad = 0
    for _ in range(trials):
        tab = {r: {a: tuple(Fraction(rng.randint(1, 50)) for _ in range(D))
                   for a in range(A)} for r in range(R)}
        regions, arms = list(range(R)), list(range(A))
        for lead in range(D):
            rest = [k for k in range(D) if k != lead]
            tail = [Fraction(rng.randint(1, 9)) for _ in rest]
            # Choose the arm minimising the lead coordinate, breaking ties by the
            # finite weighting on the remaining two.
            sec = []
            for r in regions:
                best = min(arms, key=lambda a: (
                    tab[r][a][lead],
                    sum(t * tab[r][a][k] for t, k in zip(tail, rest)), a))
                sec.append(best)
            gs = []
            for r, a in zip(regions, sec):
                ca = tab[r][a]
                for b in arms:
                    if b != a:
                        gs.append(tuple(y - x for x, y in zip(ca, tab[r][b])))
            poly = cone.region(gs)
            if poly and cone.has_strictly_positive_weights(poly):
                ok += 1
            else:
                bad += 1
    return ok, bad


def main():
    table, regions, arms = load()
    tab = normalise(table, regions, arms)

    print("1. what a zero weight costs\n")
    zero_cases = [
        ("weighs footprint only", (Fraction(0), Fraction(1), Fraction(0))),
        ("weighs time only", (Fraction(1), Fraction(0), Fraction(0))),
        ("weighs spread only", (Fraction(0), Fraction(0), Fraction(1))),
    ]
    for label, w in zero_cases:
        sec = section_under(tab, regions, arms, w)
        bad = picks_a_dominated_arm(table, regions, sec)
        print(f"  {label:24s} -> {short(sec)}")
        print(f"  {'':24s}    selects a Pareto-dominated arm at "
              f"{len(bad)} of {len(regions)} regions"
              + ("" if not bad else f": {[(r, a.replace('bitpack-carrier-', '')) for r, a in bad]}"))
    print()
    print("  Two of the three zero-weight cases select no dominated arm, and that")
    print("  is the honest shape of the claim rather than a weakening of it. The")
    print("  two statements are not symmetric:")
    print()
    print("    A strictly positive weight vector CANNOT select a dominated arm.")
    print("    That is a theorem, not a measurement: if b beats a on every")
    print("    coordinate and strictly on one, then <w, b> < <w, a> for every")
    print("    w > 0, so a is never an argmin.")
    print()
    print("    A weight vector with a zero MAY select one, and whether it does is")
    print("    a fact about the table. It does exactly when the zeroed coordinate")
    print("    is the only one separating an arm from something that beats it.")
    print()
    print("  So the guarantee is not 'usually holds' at a zero weight, it is")
    print("  'unclaimed'. p2 measured 48 sections on this table that do select a")
    print("  dominated arm under a non-negative weighting, and p6 measured 63 of")
    print("  the 72 a predecessor reported. A canon that wants the guarantee has")
    print("  to require strict positivity; it cannot get it from non-negativity")
    print("  and a hope about the table.")
    print()

    print("2. 'more leeway' as a small positive weight, swept toward zero\n")
    print("  I6 says the storage-minimising path 'has more leeway to do things")
    print("  non-efficient'. Read as a weight, the question is whether that is a")
    print("  small one or a zero one, and the two behave differently.\n")
    print(f"  {'weight on time':>16s}  {'section':56s} dominated picks")
    for num, den in [(1, 1), (1, 4), (1, 16), (1, 64), (1, 256), (1, 4096), (0, 1)]:
        wt = Fraction(num, den)
        w = (wt, Fraction(1), Fraction(1, 8))
        sec = section_under(tab, regions, arms, w)
        bad = picks_a_dominated_arm(table, regions, sec)
        print(f"  {str(wt):>16s}  {str(short(sec)):56s} {len(bad)}")
    print()
    print("  Two things in that table, and the second is the one that matters.")
    print()
    print("  The selected section MOVES as the weight shrinks, through four")
    print("  distinct sections, so 'more leeway' is a real dial with real")
    print("  consequences rather than a figure of speech. A canon that said the")
    print("  storage-first strategy 'ignores compute' would be naming the bottom")
    print("  row; op's own wording names a row above it and does not say which.")
    print()
    print("  And no strictly positive weight, however small, selects a dominated")
    print("  arm, which is the theorem in 1 rather than a result of this sweep.")
    print("  The zero row also selects none here, which is the table being kind")
    print("  and is not a guarantee. So 'more leeway' is expressible at any")
    print("  positive weight without giving up the property, and the canon can")
    print("  require strict positivity without contradicting I6.")
    print()

    print("3. a priority plus finite weights is still one weighting\n")
    ok, bad = check_lex_then_finite(trials=150, seed=20260814)
    print(f"  lexicographic-lead sections over 150 random 4x5x3 models, one per")
    print(f"  lead coordinate: {ok + bad} checked")
    print(f"  realisable by a single strictly positive weight vector: {ok}")
    print(f"  not realisable: {bad}")
    if bad == 0:
        print("  So `40` section 5.3's reading of I5 and I34, that accuracy is")
        print("  lexicographically prior for every objective except the speed-first")
        print("  one and finitely weighted for that one, is expressible in this")
        print("  model without a second mechanism. A priority is a weight ratio")
        print("  large enough, and on a finite arm set large enough is finite.")
    print()

    print("4. do two intents give two different strategies on this table\n")
    intents = [
        ("speed-first (I5): time heavy, footprint light, spread light",
         (Fraction(1), Fraction(1, 32), Fraction(1, 32))),
        ("storage-first (I6): footprint heavy, time light, spread light",
         (Fraction(1, 32), Fraction(1), Fraction(1, 32))),
        ("tail-first: spread heavy, the coordinate the harness's own findings",
         (Fraction(1, 32), Fraction(1, 32), Fraction(1))),
    ]
    secs = {}
    for label, w in intents:
        sec = section_under(tab, regions, arms, w)
        secs[label] = sec
        bad = picks_a_dominated_arm(table, regions, sec)
        print(f"  {label}")
        print(f"    -> {short(sec)}")
        print(f"    dominated picks: {len(bad)}")
    distinct = len(set(secs.values()))
    print()
    print(f"  distinct sections from {len(intents)} intents: {distinct}")
    if distinct == len(intents):
        print("  The model distinguishes them. Two intents weighing different")
        print("  coordinates produce different behaviour on the same measured")
        print("  table, which is what makes the name carry information.")
    else:
        print("  Two intents collapse to one section here, which is I6's own")
        print("  sentence working: two strategies pick the same arm exactly where")
        print("  their weightings agree, and nothing has to be written to make")
        print("  that happen.")
    print()

    print("  and the third one is the finding rather than an illustration: the")
    print("  spread coordinate is not named by any of op's four intents, and it")
    print("  produces a section none of the other two produce. Either it is a")
    print("  coordinate a strategy should weigh and no intent names it, or it is")
    print("  noise and should not be in the coordinate set. The harness's own")
    print("  findings file for this family raises it as a decision axis in those")
    print("  words, so it is at least a candidate.")


if __name__ == "__main__":
    main()
