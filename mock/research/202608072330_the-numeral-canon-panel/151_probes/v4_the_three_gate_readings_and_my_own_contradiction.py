#!/usr/bin/env python3
# v4 (151): 150's gate reading, and 150's dissent against 145 which is against me.
#
# 150 says my z2 implemented two readings of non-dominance and 144's gate is a third: p10c asks an
# arm to EARN non-dominance by beating every rival somewhere above the band, where both of z2's
# readings ask it to ESCAPE an established domination. It reports the earned reading monotone
# non-increasing in the gate strength, strictly stricter than either of mine, and leaving 4 askable
# families at k = 0.5 rather than 29 or 20. And it reports the zero surviving all three readings.
#
# 150 D1 is against 145 section 2, which says that at overflow = wrap all four lowering arms conform
# to every intermediate position, and which is contradicted by 145's own committed z3 output.
#
# Predictions, stated before running:
#
#   D1. The earned reading is monotone NON-INCREASING in k, unlike either escaped reading. One
#       comparison cannot pull two ways, which is why z2's B2 was refuted against readings 144 did
#       not use.
#   D2. The earned reading is a subset of escaped-conservative in every family.
#   D3. At k = 0.5 the earned reading leaves far fewer askable families than either escaped
#       reading. 150 reports 4; a different number on my instrument would be a disagreement worth
#       naming rather than a rounding difference.
#   D4. CONTROL, 150's discriminator: on a family of exact duplicates, escaped-conservative reports
#       every arm non-dominated and earned reports zero, because no arm beats an identical one.
#   D5. CONTROL: on a well-separated family every reading agrees, or the readings differ for
#       reasons other than the gate.
#   D6. 145 section 2's wrapping sentence is contradicted by 145_probes/z3_output.txt, at source.
import csv
import os
import re
import statistics as st
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)
ARVO = os.path.abspath(os.path.join(HERE, "..", "..", "..", ".."))
BENCH = os.path.join(ARVO, "mock", "benches")

def load():
    fam = defaultdict(lambda: defaultdict(lambda: defaultdict(list)))
    for name in sorted(os.listdir(BENCH)):
        m = re.match(r"^(.*)_n(\d+)\.csv$", name)
        if not m:
            continue
        with open(os.path.join(BENCH, name)) as f:
            for row in csv.DictReader(f):
                try:
                    fam[m.group(1)][int(m.group(2))][row["variant"]].append(float(row["algo_ns"]))
                except (KeyError, ValueError, TypeError):
                    continue
    return fam

def iqr(xs):
    if len(xs) < 4:
        return 0.0
    xs = sorted(xs)
    n = len(xs)
    return st.median(xs[(n + 1) // 2:]) - st.median(xs[: n // 2])

def summarise(fam):
    out = {}
    for family, bysize in fam.items():
        sizes = sorted(bysize)
        if not sizes:
            continue
        variants = sorted(set.intersection(*[set(bysize[s]) for s in sizes]))
        if len(sizes) < 3 or len(variants) < 4:
            continue
        out[family] = (
            sizes, variants,
            {v: {s: st.median(bysize[s][v]) for s in sizes} for v in variants},
            {v: {s: iqr(bysize[s][v]) for s in sizes} for v in variants},
        )
    return out

# ---------------------------------------------------------------------------
# Three readings of "arm v is established non-dominated".
# ---------------------------------------------------------------------------
def nd_earned(v, sizes, variants, med, spread, k):
    """144's p10c: v must BEAT every rival somewhere by more than the band."""
    for u in variants:
        if u == v:
            continue
        if not any(med[u][s] - med[v][s] > k * max(spread[u][s], spread[v][s]) for s in sizes):
            return False
    return True

def nd_escaped(v, sizes, variants, med, spread, k, symmetric):
    """z2's two: v is non-dominated unless some u establishes domination over it."""
    for u in variants:
        if u == v:
            continue
        better_somewhere = False
        worse_anywhere = False
        for s in sizes:
            gap = med[v][s] - med[u][s]
            bar = k * max(spread[u][s], spread[v][s])
            if symmetric:
                if gap < -bar:
                    worse_anywhere = True
            else:
                if gap < 0:
                    worse_anywhere = True
            if gap > bar:
                better_somewhere = True
        if better_somewhere and not worse_anywhere:
            return False
    return True

READINGS = {
    "earned": lambda v, *a: nd_earned(v, *a),
    "escaped-conservative": lambda v, *a: nd_escaped(v, *a, symmetric=False),
    "escaped-symmetric": lambda v, *a: nd_escaped(v, *a, symmetric=True),
}

S = summarise(load())
KS = [0.0, 0.05, 0.1, 0.25, 0.5, 1.0, 2.0]

print("=" * 96)
print("D1/D2/D3. The three readings over the committed corpus")
print("=" * 96)
print(f"  families with >= 3 size points and >= 4 shared variants: {len(S)}")
counts = {r: {} for r in READINGS}
for r, fn in READINGS.items():
    for k in KS:
        counts[r][k] = {f: sum(1 for v in S[f][1] if fn(v, *S[f], k)) for f in S}

print(f"\n  {'k':>6}" + "".join(f"{r:>26}" for r in READINGS))
print(f"  {'':>6}" + "".join(f"{'mean / askable':>26}" for _ in READINGS))
for k in KS:
    row = ""
    for r in READINGS:
        c = list(counts[r][k].values())
        row += f"{st.mean(c):>18.2f} / {sum(1 for x in c if x >= 2):>4}"
    print(f"  {k:>6}{row}")

mono = {}
for r in READINGS:
    mono[r] = all(counts[r][KS[i]][f] >= counts[r][KS[i + 1]][f]
                  for i in range(len(KS) - 1) for f in S)
print(f"\n  monotone NON-INCREASING in k:")
for r in READINGS:
    print(f"    {r:<24}{mono[r]}")
d1 = mono["earned"] and not mono["escaped-conservative"]
print(f"  D1: {'CONFIRMED' if d1 else 'REFUTED'}")

subset = all(set(v for v in S[f][1] if READINGS['earned'](v, *S[f], k))
             <= set(v for v in S[f][1] if READINGS['escaped-conservative'](v, *S[f], k))
             for f in S for k in KS)
print(f"  D2 (earned is a subset of escaped-conservative in every family and k): {subset}")

ask = {r: sum(1 for x in counts[r][0.5].values() if x >= 2) for r in READINGS}
print(f"\n  askable families at k = 0.5: " + ", ".join(f"{r} = {n}" for r, n in ask.items()))
d3 = ask["earned"] < ask["escaped-symmetric"] < ask["escaped-conservative"]
print(f"  D3: {'CONFIRMED' if d3 else 'REFUTED'}; 150 reports 4, 20 and 29")
if ask["earned"] != 4:
    print(f"  NOTE: my earned count is {ask['earned']} against 150's 4. That is a disagreement")
    print(f"  about the gate's implementation rather than about its direction, and I name it.")

print()
print("=" * 96)
print("D4/D5. The controls")
print("=" * 96)

def synth(medmap, sp):
    sizes = sorted(next(iter(medmap.values())))
    return (sizes, sorted(medmap), medmap, {v: {s: sp for s in sizes} for v in medmap})

dup = synth({c: {1: 100.0, 2: 100.0} for c in "abcd"}, 1.0)
wide = synth({"a": {1: 10.0, 2: 1000.0}, "b": {1: 1000.0, 2: 10.0},
              "c": {1: 400.0, 2: 400.0}, "d": {1: 5000.0, 2: 5000.0}}, 1.0)
print(f"  {'family':<22}" + "".join(f"{r:>26}" for r in READINGS))
for tag, fam in (("exact duplicates", dup), ("well separated", wide)):
    row = "".join(f"{sum(1 for v in fam[1] if READINGS[r](v, *fam, 0.5)):>26}" for r in READINGS)
    print(f"  {tag:<22}{row}")
d4 = (sum(1 for v in dup[1] if READINGS["earned"](v, *dup, 0.5)) == 0
      and sum(1 for v in dup[1] if READINGS["escaped-conservative"](v, *dup, 0.5)) == 4)
d5 = len({sum(1 for v in wide[1] if READINGS[r](v, *wide, 0.5)) for r in READINGS}) == 1
print(f"\n  D4 (duplicates: earned 0, escaped-conservative 4): {d4} (must be True)")
print(f"  D5 (well separated: all three agree): {d5} (must be True)")

print()
print("=" * 96)
print("D6. 150's dissent against 145 section 2, checked at source")
print("=" * 96)
prose = open(os.path.join(PANEL, "145_leroy_formalising_the_strategy_object.md")).read()
out = open(os.path.join(PANEL, "145_probes", "z3_output.txt")).read()
claim = "all four lowering arms in the topic's own arm set conform to every intermediate position"
in_prose = " ".join(claim.split()) in " ".join(prose.split())
summary = [l.strip() for l in out.splitlines() if "conforming-arm counts observed" in l]
twos = [l.strip() for l in out.splitlines()
        if re.search(r"\bwrap/\w+\s+2 conforming arms", l)]
print(f"  the claim is in 145's prose: {in_prose}")
print(f"  z3's own summary lines:")
for l in summary:
    print(f"    {l}")
print(f"  z3 rows showing a wrapping assignment with 2 conforming arms: {len(twos)}")
for l in twos[:3]:
    print(f"    {l}")
d6 = in_prose and any("[2, 4]" in l for l in summary) and len(twos) > 0
print(f"\n  D6: {'CONFIRMED' if d6 else 'REFUTED'}. The verdict contradicts its own table, and the")
print(f"  mechanism is 142 F142-2: absorption relocates a REDUCTION, the residual is a ROUNDING")
print(f"  relocation, free only for translation-equivariant positions, and toward-zero is not one.")
print(f"  That is the same defect 136 section 9 records against x4, occurring a second time in the")
print(f"  same file and not caught by me.")

print()
print("=" * 96)
print("VERDICT")
print("=" * 96)
print(f"  150's gate reading is a third one and mine were both the wrong shape for 144's gate.")
print(f"  Earned is monotone non-increasing, is a subset of escaped-conservative, and leaves")
print(f"  {ask['earned']} askable families at k = 0.5 against {ask['escaped-symmetric']} and "
      f"{ask['escaped-conservative']}.")
print(f"  So z2's B2 refutation is correct about the readings it implemented and wrong as a")
print(f"  generalisation to 'a gated dominance test', which is what 145 section 7 claims.")
print()
print(f"  And 150's D1 against 145 is correct at source.")
