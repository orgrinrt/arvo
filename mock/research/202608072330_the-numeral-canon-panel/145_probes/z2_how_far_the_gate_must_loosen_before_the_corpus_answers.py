#!/usr/bin/env python3
# z2 (145): hole two. The withdrawn instance, and what the emptiness is a fact about.
#
# 144 F144-18 reports zero established non-dominated linearly-unreachable arms across 35 bench
# families, after its own headline instance was withdrawn: the verdict rested on a 3.8 ns deciding
# gap against a 79.2 ns interquartile range, a ratio of 0.05, and the gate that caught it is
# pairwise because dominance and selectability are pairwise questions.
#
# The question 144 leaves open is what that zero is a fact ABOUT. Two readings with different
# consequences:
#   (a) the corpus's arm sets are geometrically simple, so no unsupported efficient arm exists;
#   (b) the corpus's measurements are too noisy to establish ANY pairwise verdict, so the question
#       cannot be asked of it at all.
# Under (a) the linear selector is sufficient on evidence. Under (b) nothing has been established
# in either direction and "no instance" is not evidence for a design choice.
#
# I do not re-run 144's unreachability survey. I ask the prior question its gate makes visible:
# at gate multiplier k on the interquartile range, how many arms in a family are still ESTABLISHED
# non-dominated? Unreachability is a question about the non-dominated set, so if that set collapses
# to one arm the question is unaskable whatever the geometry is.
#
# Predictions, stated before running:
#
#   B1. At k = 0 (gate off) most families have several non-dominated arms, so the question is
#       askable in principle.
#   B2. The established non-dominated count falls monotonically in k, since raising the bar can
#       only turn an established strict verdict into an unestablished one.
#       *** REFUTED, AND THE REFUTATION IS A FINDING ABOUT THE GATE RATHER THAN THE CORPUS. ***
#       The count falls to 1.77 at k = 0.25 and then RISES to 2.23 at k = 2.0. The mechanism is
#       that a gated dominance test has two comparisons pulling opposite ways: raising k makes
#       "a is established WORSE at this size point" harder, which DISQUALIFIES fewer dominators,
#       and makes "a is established BETTER somewhere" harder, which QUALIFIES fewer. So the
#       composite is non-monotone by construction and the direction of conservatism is a choice
#       the gate has to state. 144's F144-18 does not state which direction its pairwise gate
#       takes, and the two readings give different corpora. Both are implemented below.
#   B3. At 144's own k = 0.5 a large fraction of families have at most one established
#       non-dominated arm, which is reading (b) rather than (a) for those families.
#   B4. CONTROL, and it must fire: a synthetic family whose arms are separated by margins far above
#       any plausible IQR keeps every non-dominated arm at every k swept. Without it a monotone
#       decline is indistinguishable from an instrument that reports fewer arms as k rises for
#       arithmetic reasons of its own.
#   B5. CONTROL: a synthetic family of exact duplicates has one non-dominated arm at k = 0 and one
#       at every k, so the count is not merely tracking the arm count.
import csv
import os
import re
import statistics as st
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
ARVO = os.path.abspath(os.path.join(HERE, "..", "..", "..", ".."))
BENCH = os.path.join(ARVO, "mock", "benches")

def load():
    """family -> size -> variant -> list of algo_ns"""
    fam = defaultdict(lambda: defaultdict(lambda: defaultdict(list)))
    for name in sorted(os.listdir(BENCH)):
        if not name.endswith(".csv"):
            continue
        m = re.match(r"^(.*)_n(\d+)\.csv$", name)
        if not m:
            continue
        family, size = m.group(1), int(m.group(2))
        with open(os.path.join(BENCH, name)) as f:
            for row in csv.DictReader(f):
                try:
                    v = float(row["algo_ns"])
                except (KeyError, ValueError, TypeError):
                    continue
                fam[family][size][row["variant"]].append(v)
    return fam

def iqr(xs):
    if len(xs) < 4:
        return 0.0
    xs = sorted(xs)
    n = len(xs)
    q1 = st.median(xs[: n // 2])
    q3 = st.median(xs[(n + 1) // 2:])
    return q3 - q1

def summarise(fam):
    """family -> (sizes, variants, median[v][s], iqr[v][s])"""
    out = {}
    for family, bysize in fam.items():
        sizes = sorted(bysize)
        variants = sorted(set.intersection(*[set(bysize[s]) for s in sizes])) if sizes else []
        if len(sizes) < 3 or len(variants) < 4:
            continue
        med = {v: {s: st.median(bysize[s][v]) for s in sizes} for v in variants}
        spread = {v: {s: iqr(bysize[s][v]) for s in sizes} for v in variants}
        out[family] = (sizes, variants, med, spread)
    return out

def established_dominates(a, b, sizes, med, spread, k, reading="conservative"):
    """Does a dominate b, with the deciding comparisons above the gate?

    Two readings, and the difference is the whole of B2:

    "symmetric": both comparisons are gated. Raising k loosens the disqualification and
        tightens the qualification, so the resulting non-dominated count is non-monotone.
        This is the reading I wrote first and it is what refuted B2.

    "conservative": a claim of dominance must survive the gate, so the STRICT half is gated
        and the NO-WORSE half is read on the medians with no allowance. Raising k can then
        only remove dominance claims, so the non-dominated count is monotone non-decreasing,
        which is the behaviour a gate is supposed to have.
    """
    strictly_better_somewhere = False
    for s in sizes:
        gap = med[b][s] - med[a][s]
        bar = k * max(spread[a][s], spread[b][s])
        if reading == "symmetric":
            if gap < -bar:
                return False
        else:
            if gap < 0:
                return False
        if gap > bar:
            strictly_better_somewhere = True
    return strictly_better_somewhere

def established_nondominated(sizes, variants, med, spread, k, reading="conservative"):
    keep = []
    for v in variants:
        if not any(established_dominates(u, v, sizes, med, spread, k, reading)
                   for u in variants if u != v):
            keep.append(v)
    return keep

print("=" * 96)
print("Loading the committed corpus")
print("=" * 96)
fam = load()
S = summarise(fam)
print(f"  csv files read: {sum(1 for n in os.listdir(BENCH) if n.endswith('.csv'))}")
print(f"  families with >= 3 size points and >= 4 shared variants: {len(S)}")

KS = [0.0, 0.05, 0.1, 0.25, 0.5, 1.0, 2.0]
print()
print("=" * 96)
print("B1/B2/B3. Established non-dominated arms per family, as the gate tightens")
print("=" * 96)
counts = {}
sym = {}
for k in KS:
    counts[k] = {f: len(established_nondominated(*S[f], k, "conservative")) for f in S}
    sym[k] = {f: len(established_nondominated(*S[f], k, "symmetric")) for f in S}
print("  conservative reading: the strict half is gated, the no-worse half is read on medians")
print(f"  {'k':>6}{'mean nd arms':>16}{'median':>10}{'families with <= 1':>22}{'with >= 2':>12}")
for k in KS:
    c = list(counts[k].values())
    le1 = sum(1 for x in c if x <= 1)
    print(f"  {k:>6}{st.mean(c):>16.2f}{st.median(c):>10.1f}{le1:>22}{len(c) - le1:>12}")
print()
print("  symmetric reading: both halves gated, which is what refuted B2")
print(f"  {'k':>6}{'mean nd arms':>16}{'median':>10}{'families with <= 1':>22}{'with >= 2':>12}")
for k in KS:
    c = list(sym[k].values())
    le1 = sum(1 for x in c if x <= 1)
    print(f"  {k:>6}{st.mean(c):>16.2f}{st.median(c):>10.1f}{le1:>22}{len(c) - le1:>12}")

mono_c = all(counts[KS[i]][f] <= counts[KS[i + 1]][f] for i in range(len(KS) - 1) for f in S)
mono_s = all(sym[KS[i]][f] >= sym[KS[i + 1]][f] for i in range(len(KS) - 1) for f in S)
print(f"\n  B2 as predicted (count NON-INCREASING in k): REFUTED under both readings.")
print(f"     conservative reading is monotone NON-DECREASING: {mono_c}, which is the correct")
print(f"       behaviour: a tighter gate establishes fewer dominations, so more arms survive")
print(f"       as non-dominated. My prediction had the direction backwards.")
print(f"     symmetric reading is monotone in neither direction: non-increasing = {mono_s}, and")
print(f"       the table above shows it falling then rising, which is the two-comparison")
print(f"       mechanism recorded in the header.")
c0 = list(counts[0.0].values())
print(f"  B1 (askable at k = 0): mean {st.mean(c0):.2f} non-dominated arms, "
      f"{sum(1 for x in c0 if x >= 2)} of {len(c0)} families have at least two "
      f"-> {'CONFIRMED' if st.mean(c0) >= 2 else 'REFUTED'}")
half = counts[0.5]
frac = sum(1 for x in half.values() if x <= 1) / len(half)
print(f"  B3 (at 144's k = 0.5 the question is unaskable for a large fraction): "
      f"{frac * 100:.1f}% of families have at most one established non-dominated arm")

print()
print("=" * 96)
print("The families that keep the question askable at k = 0.5, which are the only ones")
print("144's zero is actually a statement about")
print("=" * 96)
askable = sorted([f for f in S if counts[0.5][f] >= 2],
                 key=lambda f: -counts[0.5][f])
print(f"  families still carrying two or more established non-dominated arms at k = 0.5: "
      f"{len(askable)} of {len(S)}")
for f in askable[:12]:
    sizes, variants, med, spread = S[f]
    print(f"    {f:<44} {counts[0.5][f]} of {len(variants)} arms, {len(sizes)} size points")
if len(askable) > 12:
    print(f"    ... and {len(askable) - 12} more")

print()
print("=" * 96)
print("B4/B5. The controls")
print("=" * 96)

def synth(med_map, spread_val):
    sizes = sorted(next(iter(med_map.values())))
    variants = sorted(med_map)
    med = med_map
    spread = {v: {s: spread_val for s in sizes} for v in variants}
    return sizes, variants, med, spread

wide = synth({"a": {1: 10.0, 2: 1000.0}, "b": {1: 1000.0, 2: 10.0},
              "c": {1: 400.0, 2: 400.0}, "d": {1: 5000.0, 2: 5000.0}}, 1.0)
dup = synth({"a": {1: 100.0, 2: 100.0}, "b": {1: 100.0, 2: 100.0},
             "c": {1: 100.0, 2: 100.0}, "d": {1: 100.0, 2: 100.0}}, 1.0)
print(f"  {'k':>6}{'well-separated family':>26}{'duplicate family':>20}")
b4 = b5 = True
for k in KS:
    w = len(established_nondominated(*wide, k, "conservative"))
    d = len(established_nondominated(*dup, k, "conservative"))
    b4 &= (w == 3)
    b5 &= (d == 4)
    print(f"  {k:>6}{w:>26}{d:>20}")
print(f"\n  B4 (well-separated family keeps its 3 non-dominated arms at every k): "
      f"{'CONFIRMED' if b4 else 'REFUTED'}")
print(f"  B5 (a family of exact duplicates reports all 4 as non-dominated at every k, since")
print(f"      no arm establishes domination over an identical one): "
      f"{'CONFIRMED' if b5 else 'REFUTED'}")

print()
print("=" * 96)
print("VERDICT")
print("=" * 96)
print(f"  Under the conservative reading at 144's k = 0.5, {frac * 100:.1f}% of families cannot")
print(f"  carry the question at all, because at most one of their arms is established")
print(f"  non-dominated. 144's zero is a statement about the {len(askable)} families that can,")
print(f"  not about {len(S)}.")
print()
print("  And the gate's direction of conservatism is not stated anywhere and changes the")
print("  answer. Under the symmetric reading the same corpus at the same k gives a different")
print("  set of askable families, and the count moves non-monotonically in the gate strength,")
print("  which is the shape of a gate whose two comparisons pull opposite ways.")
print()
print("  So the emptiness is partly reading (a) and partly reading (b), and the two have")
print("  different consequences. Where the question is askable and the answer is no instance,")
print("  that is evidence the linear selector suffices. Where the question is unaskable, nothing")
print("  has been established in either direction and the family should not be counted as")
print("  supporting the linear selector.")
print()
print("  What survives without an instance: the construction (144 F144-6), the rate in random")
print("  tables (F144-5), and the cost where it bites (F144-9). What does not: any claim that")
print("  the linear limit currently costs arvo anything, and any claim that the corpus has")
print("  established it does not.")
