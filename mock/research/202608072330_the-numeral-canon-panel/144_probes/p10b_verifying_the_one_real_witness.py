"""p10b: the single real witness p10 found, verified by a second method.

NOT A BENCH RUN. Reads committed harness CSVs and does exact arithmetic.

p10 surveyed 35 bench families, excluded 21 on two controls, and found exactly
ONE arm in the surviving 14 that is Pareto-optimal and unreachable by any linear
weighting: `warm-clamp-accfit-dyn` in `warm-clamp-arity-w32`. A result with a
count of one is precisely where an instrument defect hides, so it is checked
here by a different decision procedure and stressed along the dimensions that
could be carrying it.

PREDICTIONS, before running:
  CC1 the arm is non-dominated for a reason a reader can see: for every other
      arm there is a size at which it is faster.
  CC2 the slow full-enumeration LP, which shares no code path with the
      separation LP p10 used, also reports it unreachable, with a strictly
      positive optimum.
  CC3 the finding is not carried by one size point: dropping any single
      coordinate leaves it unreachable. I am not confident here and I expect at
      least one drop to change the verdict, which would make the finding a
      statement about the full coordinate set rather than about the arm.
  CC4 the sibling arms p10 calls selectable are confirmed selectable by the same
      second method, so the procedure is not simply calling everything
      unreachable.

CONTROLS:
  DD1 THE CASE THAT MUST FAIL. A deliberately dominated arm, constructed by
      taking the witness and adding one nanosecond to every coordinate, must be
      reported dominated and unreachable by both procedures.
  DD2 the two procedures must agree on every arm of this table, not only on the
      witness.
"""

import csv
import glob
import os
import re
import statistics
from fractions import Fraction as F

from exact_lp import min_regret, strictly_selectable, dominated

BENCH = os.path.normpath(os.path.join(
    os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "benches"))
FAMILY = "warm-clamp-arity-w32"
WITNESS = "warm-clamp-accfit-dyn"

fail = []

sizes = {}
for path in sorted(glob.glob(os.path.join(BENCH, f"{FAMILY}_n*.csv"))):
    m = re.match(rf"{re.escape(FAMILY)}_n(\d+)\.csv$", os.path.basename(path))
    per = {}
    for r in csv.DictReader(open(path)):
        per.setdefault(r["variant"], []).append(float(r["algo_ns"]))
    sizes[int(m.group(1))] = per

keys = sorted(sizes)
vs = sorted(sizes[keys[0]])
mat = [tuple(F(int(round(statistics.median(sizes[k][v]) * 10)), 10) for k in keys)
       for v in vs]
wi = vs.index(WITNESS)

print(f"family {FAMILY}, {len(vs)} arms, {len(keys)} size coordinates")
print(f"sizes: {keys}")
print()
print(f"{'arm':<26} " + " ".join(f"{k:>10}" for k in keys))
for i, v in enumerate(vs):
    print(f"{v:<26} " + " ".join(f"{float(mat[i][j]):>10.0f}" for j in range(len(keys))))

print()
print("=" * 78)
print("CC1. why the witness is non-dominated, arm by arm")
print("=" * 78)
ok = True
for j, v in enumerate(vs):
    if j == wi:
        continue
    better = [k for k in range(len(keys)) if mat[wi][k] < mat[j][k]]
    print(f"  against {v:<26} faster at sizes "
          f"{[keys[k] for k in better] if better else 'NONE'}")
    if not better:
        ok = False
print(f"  CC1 -> {'CONFIRMED' if ok else 'REFUTED'}")
if not ok:
    fail.append("CC1")

print()
print("=" * 78)
print("CC2/CC4/DD2. two decision procedures over every arm")
print("=" * 78)
print(f"  {'arm':<26} {'dominated':>10} {'separation LP':>14} {'enumeration min t':>19}")
disagree = 0
for i, v in enumerate(vs):
    sep = strictly_selectable(mat, i)
    enu = min_regret(mat, i)
    if sep != (enu < 0):
        disagree += 1
    print(f"  {v:<26} {str(dominated(mat, i)):>10} "
          f"{('selectable' if sep else 'unreachable'):>14} {str(enu):>19}")
print(f"  DD2 procedures disagree on {disagree} arms -> "
      f"{'PASS' if disagree == 0 else 'FAIL'}")
if disagree:
    fail.append("DD2")
t = min_regret(mat, wi)
cc2 = t is not None and t > 0
print(f"  CC2 the witness has strictly positive optimum {t}: "
      f"{'CONFIRMED' if cc2 else 'REFUTED'}")
if not cc2:
    fail.append("CC2")
sel = [i for i in range(len(vs)) if strictly_selectable(mat, i)]
print(f"  CC4 selectable siblings confirmed by both: {[vs[i] for i in sel]}")
if not sel:
    fail.append("CC4")

print()
print("=" * 78)
print("DD1. the case that must fail: a deliberately dominated arm")
print("=" * 78)
bad = mat + [tuple(x + 1 for x in mat[wi])]
print(f"  witness plus one nanosecond on every coordinate: "
      f"dominated={dominated(bad, len(bad) - 1)}, "
      f"selectable={strictly_selectable(bad, len(bad) - 1)}")
if not dominated(bad, len(bad) - 1) or strictly_selectable(bad, len(bad) - 1):
    fail.append("DD1")
print(f"  DD1 -> {'PASS' if 'DD1' not in fail else 'FAIL'}")

print()
print("=" * 78)
print("CC3. is the finding carried by one size point?")
print("=" * 78)
flips = []
for drop in range(len(keys)):
    sub = [tuple(c[k] for k in range(len(keys)) if k != drop) for c in mat]
    dom = dominated(sub, wi)
    sel2 = strictly_selectable(sub, wi)
    verdict = "dominated" if dom else ("selectable" if sel2 else "unreachable")
    print(f"  dropping size {keys[drop]:>10}: witness is {verdict}")
    if verdict != "unreachable":
        flips.append((keys[drop], verdict))
cc3 = not flips
print(f"  CC3 the verdict survives dropping any single coordinate: "
      f"{'CONFIRMED' if cc3 else 'REFUTED'}")
if flips:
    print(f"      it flips when dropping {[f'{k} -> {v}' for k, v in flips]}")
    print("      so the finding is a statement about the full coordinate set rather")
    print("      than about the arm alone, and its predicate has to say so.")

print()
print("=" * 78)
print(f"control failures: {len(set(fail))} {sorted(set(fail))}")
print("=" * 78)
raise SystemExit(1 if fail else 0)
