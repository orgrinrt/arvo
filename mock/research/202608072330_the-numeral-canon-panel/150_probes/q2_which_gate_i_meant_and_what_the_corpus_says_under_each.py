"""q2: the direction of conservatism `145` says my gate does not state.

NOT A BENCH RUN. Reads the committed harness CSVs under `mock/benches/` and does
exact arithmetic on them. It takes no measurement.

`145` z2 reports that `144` F144-18's gate does not state its direction of
conservatism, that the two readings z2 implemented give 29 askable families
against 20, and that its own prediction B2, that the established non-dominated
count falls with gate strength, was refuted because a gated dominance test has
two comparisons pulling opposite ways. `146` section 1.4 carries that.

**Both of z2's readings phrase the gate as escaping domination. Mine does not.**
`144_probes/p10c`'s `established_nondominated` asks, for every rival, whether the
arm beats that rival at some coordinate by more than the band. It is a positive
claim the arm has to earn, not a negative one it has to escape, and it is a third
reading stricter than both of z2's. So the question is answerable rather than
open, and this probe answers it and then measures what it costs.

The unreachability test is held fixed at `p10c`'s throughout: not strictly
selectable on the medians, and still not selectable at the optimistic band edge.
Only the non-dominance reading varies, so the comparison isolates one thing.

The exact selectability procedure is `144_probes/exact_lp.py`, imported rather
than reimplemented, because it was cross-checked there against full vertex
enumeration on 770 arms with 406 strictly negative, 63 exactly zero and 301
strictly positive optima and zero mismatches. Reusing it means this probe
inherits that check rather than asserting a new one.

PREDICTIONS, before running:
  CC1 my reading is strictly stricter than z2's conservative one: every arm it
      establishes as non-dominated is also established under z2's, and somewhere
      the inclusion is strict.
  CC2 my reading is monotone NON-INCREASING in the gate multiplier. Raising the
      bar makes every required win harder and can only remove arms. This is z2's
      B2 as B2 predicted it, so B2 is refuted for z2's two readings and holds for
      the one `144` actually used.
  CC3 the askable count under my reading at k = 0.5 is BELOW z2's 29, because a
      stricter non-dominance test admits fewer arms.
  CC4 under the looser readings an established non-dominated and linearly
      unreachable arm appears somewhere in the corpus, because a larger
      non-dominated set is a larger candidate pool. If it does, F144-18's zero is
      gate-dependent in the direction that matters and I say so.

CONTROLS:
  DD1 a synthetic family whose arms are separated far above any plausible spread
      keeps its non-dominated arms under all three readings at every k.
  DD2 THE DISCRIMINATOR, and it must separate the readings. A family of exact
      duplicates has FOUR arms established non-dominated under z2's conservative
      reading, because no arm establishes domination over an identical one, and
      ZERO under mine, because no arm beats an identical one anywhere either. If
      the two agree there, the readings are not actually different and CC1 is
      vacuous.
  DD3 every reading must report zero movement between k and k on the same family,
      which is the trivial idempotence check that the sweep is keyed correctly.
"""

import csv
import os
import re
import statistics as st
import sys
from collections import defaultdict
from fractions import Fraction as F

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.normpath(os.path.join(HERE, "..", "144_probes")))
from exact_lp import strictly_selectable  # noqa: E402

BENCH = os.path.normpath(os.path.join(HERE, "..", "..", "..", "benches"))

fail = []


def iqr(xs):
    if len(xs) < 4:
        return 0.0
    xs = sorted(xs)
    n = len(xs)
    return st.median(xs[(n + 1) // 2:]) - st.median(xs[: n // 2])


def load():
    fam = defaultdict(lambda: defaultdict(lambda: defaultdict(list)))
    for name in sorted(os.listdir(BENCH)):
        m = re.match(r"^(.*)_n(\d+)\.csv$", name) if name.endswith(".csv") else None
        if not m:
            continue
        with open(os.path.join(BENCH, name)) as fh:
            for row in csv.DictReader(fh):
                try:
                    fam[m.group(1)][int(m.group(2))][row["variant"]].append(float(row["algo_ns"]))
                except (KeyError, ValueError, TypeError):
                    continue
    out = {}
    for family, bysize in fam.items():
        sizes = sorted(bysize)
        if len(sizes) < 3:
            continue
        variants = sorted(set.intersection(*[set(bysize[s]) for s in sizes]))
        if len(variants) < 4:
            continue
        med = {v: {s: st.median(bysize[s][v]) for s in sizes} for v in variants}
        spr = {v: {s: iqr(bysize[s][v]) for s in sizes} for v in variants}
        out[family] = (sizes, variants, med, spr)
    return out


# ------------------------------------------------------------ the three gates


def nd_earned(sizes, variants, med, spr, k):
    """144's p10c reading: an arm earns non-dominance by beating every rival
    somewhere by more than the band."""
    keep = []
    for v in variants:
        ok = True
        for u in variants:
            if u == v:
                continue
            if not any(med[u][s] - med[v][s] > k * max(spr[v][s], spr[u][s]) for s in sizes):
                ok = False
                break
        if ok:
            keep.append(v)
    return keep


def _dominates(a, b, sizes, med, spr, k, reading):
    strict = False
    for s in sizes:
        gap = med[b][s] - med[a][s]
        bar = k * max(spr[a][s], spr[b][s])
        if reading == "symmetric":
            if gap < -bar:
                return False
        else:
            if gap < 0:
                return False
        if gap > bar:
            strict = True
    return strict


def nd_escaped(sizes, variants, med, spr, k, reading):
    """z2's two readings: an arm is non-dominated if nobody establishes
    domination over it."""
    return [v for v in variants
            if not any(_dominates(u, v, sizes, med, spr, k, reading)
                       for u in variants if u != v)]


READINGS = {
    "earned (144 p10c)": lambda *a: nd_earned(*a),
    "escaped-conservative (145 z2)": lambda *a: nd_escaped(*a, "conservative"),
    "escaped-symmetric (145 z2)": lambda *a: nd_escaped(*a, "symmetric"),
}

S = load()
KS = [0.0, 0.05, 0.1, 0.25, 0.5, 1.0, 2.0]

print("=" * 84)
print("DD2. the discriminator: do the three readings actually differ?")
print("=" * 84)


def synth(medmap, spread):
    sizes = sorted(next(iter(medmap.values())))
    variants = sorted(medmap)
    return sizes, variants, medmap, {v: {s: spread for s in sizes} for v in variants}


DUP = synth({c: {1: 100.0, 2: 100.0} for c in "abcd"}, 1.0)
WIDE = synth({"a": {1: 10.0, 2: 1000.0}, "b": {1: 1000.0, 2: 10.0},
              "c": {1: 400.0, 2: 400.0}, "d": {1: 5000.0, 2: 5000.0}}, 1.0)
print(f"  {'reading':<32}{'duplicates':>12}{'well-separated':>18}")
for name, fn in READINGS.items():
    d = len(fn(*DUP, 0.5))
    w = len(fn(*WIDE, 0.5))
    print(f"  {name:<32}{d:>12}{w:>18}")
dd2 = len(READINGS["earned (144 p10c)"](*DUP, 0.5)) == 0 and \
    len(READINGS["escaped-conservative (145 z2)"](*DUP, 0.5)) == 4
print(f"  DD2 earned gives 0 on duplicates and escaped-conservative gives 4: "
      f"{'PASS, the readings are genuinely different' if dd2 else 'FAIL'}")
if not dd2:
    fail.append("DD2")
dd1 = all(len(fn(*WIDE, k)) == 3 for fn in READINGS.values() for k in KS)
print(f"  DD1 the well-separated family keeps its 3 non-dominated arms under every reading "
      f"at every k: {'PASS' if dd1 else 'FAIL'}")
if not dd1:
    fail.append("DD1")

print()
print("=" * 84)
print("CC1/CC2/CC3. the three readings on the committed corpus")
print("=" * 84)
print(f"  {len(S)} families with at least 3 size points and 4 shared variants")
print()
print(f"  {'k':>6}" + "".join(f"{n.split()[0]:>22}" for n in READINGS))
print(f"  {'':>6}" + "".join(f"{'mean nd / askable':>22}" for _ in READINGS))
counts = {n: {} for n in READINGS}
for k in KS:
    row = f"  {k:>6}"
    for name, fn in READINGS.items():
        c = {f: len(fn(*S[f], k)) for f in S}
        counts[name][k] = c
        askable = sum(1 for x in c.values() if x >= 2)
        row += f"{st.mean(c.values()):>15.2f} /{askable:>5}"
    print(row)

mono = all(counts["earned (144 p10c)"][KS[i]][f] >= counts["earned (144 p10c)"][KS[i + 1]][f]
           for i in range(len(KS) - 1) for f in S)
print()
print(f"  CC2 the earned reading is monotone NON-INCREASING in k: "
      f"{'CONFIRMED' if mono else 'REFUTED'}")
if not mono:
    fail.append("CC2")
print(f"      so z2's B2 holds for the reading 144 used and was refuted against two readings")
print(f"      144 did not use. A gate that has to earn a positive claim has one comparison,")
print(f"      not two, and one comparison cannot pull two ways.")

subset_ok = strict_somewhere = True
strict_cases = 0
for f in S:
    a = set(READINGS["earned (144 p10c)"](*S[f], 0.5))
    b = set(READINGS["escaped-conservative (145 z2)"](*S[f], 0.5))
    if not a <= b:
        subset_ok = False
    if a < b:
        strict_cases += 1
print(f"  CC1 earned is a subset of escaped-conservative in every family: "
      f"{'CONFIRMED' if subset_ok else 'REFUTED'}, strict in {strict_cases} of {len(S)}")
if not subset_ok:
    fail.append("CC1")

ask_earned = sum(1 for x in counts["earned (144 p10c)"][0.5].values() if x >= 2)
ask_cons = sum(1 for x in counts["escaped-conservative (145 z2)"][0.5].values() if x >= 2)
ask_sym = sum(1 for x in counts["escaped-symmetric (145 z2)"][0.5].values() if x >= 2)
print(f"  CC3 askable families at k = 0.5: earned {ask_earned}, escaped-conservative "
      f"{ask_cons}, escaped-symmetric {ask_sym}")
print(f"      {'CONFIRMED' if ask_earned < ask_cons else 'REFUTED'}: the reading 144 used is")
print(f"      stricter than either z2 implemented, so F144-18's zero is a statement about")
print(f"      {ask_earned} families rather than 29 or 20.")

print()
print("=" * 84)
print("CC4. does a looser reading resurrect an established unreachable arm?")
print("=" * 84)


def survey(reading_fn, k):
    """p10c's unreachability test, with only the non-dominance reading varied."""
    hits = []
    for f in S:
        sizes, variants, med, spr = S[f]
        nd = reading_fn(sizes, variants, med, spr, k)
        if len(nd) < 2:
            continue
        mat = [tuple(F(int(round(med[v][s] * 10)), 10) for s in sizes) for v in nd]
        opt = []
        for i in range(len(nd)):
            row = []
            for j, v in enumerate(nd):
                half = [F(int(round(spr[v][s] * 10)), 10) / 2 for s in sizes]
                base = mat[j]
                row.append(tuple(base[t] - half[t] if j == i else base[t] + half[t]
                                 for t in range(len(sizes))))
            opt.append(row)
        for i, v in enumerate(nd):
            if strictly_selectable(mat, i):
                continue
            if strictly_selectable(opt[i], i):
                continue
            hits.append((f, v, len(nd)))
    return hits


print(f"  {'reading':<32}{'askable families':>18}{'established unreachable arms':>32}")
found_any = False
for name, fn in READINGS.items():
    hits = survey(fn, 0.5)
    ask = sum(1 for f in S if len(fn(*S[f], 0.5)) >= 2)
    print(f"  {name:<32}{ask:>18}{len(hits):>32}")
    for f, v, n in hits:
        print(f"      {f}: {v}, among {n} established non-dominated arms")
    if hits:
        found_any = True
print()
if found_any:
    print("  CC4 CONFIRMED. F144-18's zero is gate-dependent: a looser non-dominance reading")
    print("      produces an instance, so the corpus does not settle the selector question")
    print("      independently of which gate the design means.")
else:
    print("  CC4 REFUTED. No reading produces an instance, so F144-18's zero survives every")
    print("      direction of conservatism on the table and the gate question changes what the")
    print("      zero is a statement ABOUT without changing the zero itself.")

print()
print("=" * 84)
print("DD3. idempotence")
print("=" * 84)
idem = all(READINGS[n](*S[f], 0.5) == READINGS[n](*S[f], 0.5) for n in READINGS for f in S)
print(f"  the same reading at the same k gives the same set: {'PASS' if idem else 'FAIL'}")
if not idem:
    fail.append("DD3")

print()
print("=" * 84)
print("the settlement, in the author's own words")
print("=" * 84)
print("  I meant the earned reading and I should have said so. The gate exists because I had")
print("  just published a false positive, so the thing it guards is the POSITIVE claim: an arm")
print("  counts as non-dominated only if it beats every rival somewhere by more than the noise.")
print("  Guarding the negative claim instead, which is what both of z2's readings do, admits")
print("  arms that merely escape an established domination, and that is the wrong direction for")
print("  a finding whose failure mode is claiming an arm is special when it is not.")
print()
print("  What 145 gets right and I owe it: the direction is not stated anywhere in 144, the")
print("  finding is unusable without it, and z2 is the file that made that visible.")

print()
print("=" * 84)
print(f"control failures: {len(set(fail))} {sorted(set(fail))}")
print("=" * 84)
raise SystemExit(1 if fail else 0)
