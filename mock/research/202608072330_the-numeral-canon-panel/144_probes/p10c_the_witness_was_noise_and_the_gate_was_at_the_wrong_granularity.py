"""p10c: my own AA1 withdrawn, and the gate that should have caught it.

NOT A BENCH RUN. Reads committed harness CSVs and does exact arithmetic.

p10 reported one real arm, `warm-clamp-accfit-dyn` in `warm-clamp-arity-w32`,
that is Pareto-optimal and unreachable by any linear weighting, and read that as
closing 139's O-139-C in the affirmative. p10b then found that the verdict rests
entirely on the largest size point: drop it and the arm is dominated.

The margin at that point is 3.8 nanoseconds against a within-arm interquartile
range of 79.2 nanoseconds on the arm it has to beat. The verdict rests on a
difference one twentieth the size of the harness's own scatter on the same arm
in the same run.

SO AA1 IS WITHDRAWN. p10's headline is wrong and its output stays committed with
the wrong headline in it, because the sequence is the finding.

WHY IT GOT THROUGH, which is the part worth carrying. p10's BB3 control compared
the ARM-TO-ARM spread against the WITHIN-ARM spread, per family, and
`warm-clamp-arity-w32` passed it at 9.2. That gate is real and it is at the wrong
granularity: a family can have arms separated by microseconds and still have the
one pairwise comparison that decides a verdict separated by nanoseconds. A
family-level noise gate does not protect a pairwise verdict, and every
dominance and selectability question is pairwise.

PREDICTIONS, before running:
  EE1 the witness fails a pairwise gate: the comparison that makes it
      non-dominated has a gap below the within-arm spread.
  EE2 with a pairwise gate applied, ZERO arms in the whole corpus are
      established as Pareto-optimal and linearly unreachable.
  EE3 the reordering result survives, because a reordering across the size axis
      involves gaps far larger than the scatter. At least two thirds of the
      reordering families should survive.

CONTROLS:
  FF1 THE CASE THAT MUST FAIL. A synthetic table whose arms differ by far more
      than any plausible scatter must PASS the pairwise gate. A gate that
      rejects everything is not a gate.
  FF2 THE SECOND CASE THAT MUST FAIL. Two arms with identical committed
      distributions must be reported indistinguishable by the gate.
"""

import csv
import glob
import os
import re
import statistics
from fractions import Fraction as F

from exact_lp import strictly_selectable, dominated

BENCH = os.path.normpath(os.path.join(
    os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "benches"))

fail = []


def iqr(vals):
    s = sorted(vals)
    n = len(s)
    return s[(3 * n) // 4] - s[n // 4]


def load():
    fam = {}
    for path in sorted(glob.glob(os.path.join(BENCH, "*_n*.csv"))):
        m = re.match(r"(.+)_n(\d+)\.csv$", os.path.basename(path))
        if not m:
            continue
        rows = list(csv.DictReader(open(path)))
        per = {}
        for r in rows:
            per.setdefault(r["variant"], []).append(float(r["algo_ns"]))
        fam.setdefault(m.group(1), {})[int(m.group(2))] = per
    out = {}
    for name, sizes in fam.items():
        keys = sorted(sizes)
        if len(keys) < 3:
            continue
        vs = sorted(sizes[keys[0]])
        if len(vs) < 4 or any(sorted(sizes[k]) != vs for k in keys):
            continue
        out[name] = (keys, vs, sizes)
    return out


def matrices(keys, vs, sizes):
    """Median cost matrix and the half-IQR band around each entry."""
    med, band = [], []
    for v in vs:
        med.append(tuple(F(int(round(statistics.median(sizes[k][v]) * 10)), 10)
                         for k in keys))
        band.append(tuple(F(int(round(iqr(sizes[k][v]) * 10)), 10) / 2
                          for k in keys))
    return med, band


def optimistic_for(med, band, i):
    """Push arm i down by its half-band and everything else up by theirs.
    If arm i is selectable HERE, its unreachability is inside the scatter."""
    out = []
    for j, c in enumerate(med):
        if j == i:
            out.append(tuple(c[k] - band[j][k] for k in range(len(c))))
        else:
            out.append(tuple(c[k] + band[j][k] for k in range(len(c))))
    return out


def established_nondominated(med, band, i):
    """Non-dominated with every deciding gap outside the scatter."""
    for j in range(len(med)):
        if j == i:
            continue
        wins = [k for k in range(len(med[i]))
                if med[j][k] - med[i][k] > max(band[i][k], band[j][k])]
        if not wins:
            return False
    return True


print("=" * 78)
print("EE1. the witness, against a pairwise gate")
print("=" * 78)
fams = load()
keys, vs, sizes = fams["warm-clamp-arity-w32"]
med, band = matrices(keys, vs, sizes)
wi = vs.index("warm-clamp-accfit-dyn")
print(f"  arm {vs[wi]}, deciding coordinate n={keys[-1]}")
for j, v in enumerate(vs):
    if j == wi:
        continue
    gap = med[j][-1] - med[wi][-1]
    thr = max(band[wi][-1], band[j][-1])
    print(f"    vs {v:<24} gap {float(gap):>8.1f} ns, half-IQR threshold "
          f"{float(thr):>7.1f} ns, ratio {float(gap) / float(thr):>5.2f} "
          f"{'clears' if gap > thr else 'INSIDE THE SCATTER'}")
ee1 = not established_nondominated(med, band, wi)
print(f"  EE1 the witness fails the pairwise gate: {'CONFIRMED' if ee1 else 'REFUTED'}")
if not ee1:
    fail.append("EE1")
print(f"  and under the optimistic-band matrix it is "
      f"{'selectable' if strictly_selectable(optimistic_for(med, band, wi), wi) else 'still unreachable'}")

print()
print("=" * 78)
print("EE2. the whole corpus, with the pairwise gate applied")
print("=" * 78)
established = []
surveyed = 0
for name in sorted(fams):
    keys, vs, sizes = fams[name]
    med, band = matrices(keys, vs, sizes)
    surveyed += 1
    for i in range(len(vs)):
        if not established_nondominated(med, band, i):
            continue
        if strictly_selectable(med, i):
            continue
        # unreachable on the medians; is it still unreachable at the band edge?
        if strictly_selectable(optimistic_for(med, band, i), i):
            continue
        established.append((name, vs[i]))
print(f"  surveyed {surveyed} families")
print(f"  arms established as non-dominated AND linearly unreachable, with every")
print(f"  deciding gap outside the harness's own scatter: {len(established)}")
for n, a in established:
    print(f"    {n}: {a}")
ee2 = len(established) == 0
print(f"  EE2 -> {'CONFIRMED, none' if ee2 else 'REFUTED'}")

print()
print("=" * 78)
print("EE3. does the reordering result survive the same gate?")
print("=" * 78)
survived = total = 0
for name in sorted(fams):
    keys, vs, sizes = fams[name]
    med, band = matrices(keys, vs, sizes)
    first = min(range(len(vs)), key=lambda i: (med[i][0], i))
    last = min(range(len(vs)), key=lambda i: (med[i][-1], i))
    if first == last:
        continue
    total += 1
    # the reordering is established only if each winner beats the other winner
    # by more than the scatter at its own coordinate
    g0 = med[last][0] - med[first][0]
    t0 = max(band[first][0], band[last][0])
    g1 = med[first][-1] - med[last][-1]
    t1 = max(band[first][-1], band[last][-1])
    if g0 > t0 and g1 > t1:
        survived += 1
        if survived <= 8:
            print(f"    {name}: {vs[first]} at the smallest size (by "
                  f"{float(g0):.0f} ns, threshold {float(t0):.0f}), "
                  f"{vs[last]} at the largest (by {float(g1):.0f} ns, "
                  f"threshold {float(t1):.0f})")
print(f"  {survived} of {total} reordering families survive the pairwise gate")
ee3 = total > 0 and survived * 3 >= total * 2
print(f"  EE3 -> {'CONFIRMED' if ee3 else 'REFUTED'}")

print()
print("=" * 78)
print("FF1/FF2. the gate's own controls")
print("=" * 78)
big_med = [(F(100), F(900)), (F(900), F(100)), (F(600), F(600))]
big_band = [(F(1), F(1))] * 3
ff1 = all(established_nondominated(big_med, big_band, i) for i in range(3))
print(f"  FF1 a table with huge gaps passes the gate for every arm: "
      f"{'PASS' if ff1 else 'FAIL, the gate rejects everything'}")
if not ff1:
    fail.append("FF1")
same_med = [(F(500), F(500)), (F(500), F(500)), (F(100), F(900))]
same_band = [(F(50), F(50))] * 3
ff2 = not established_nondominated(same_med, same_band, 0)
print(f"  FF2 two identical arms are reported indistinguishable: "
      f"{'PASS' if ff2 else 'FAIL'}")
if not ff2:
    fail.append("FF2")

print()
print("the corrected answer to O-139-C, and the lesson:")
print("  no arm set in arvo's committed bench corpus contains an established")
print("  Pareto-optimal arm that no linear weighting can select. The one candidate")
print("  rests on a gap one twentieth of the scatter around it. So the linear limit")
print("  is real in general, at 11.7% of Pareto arms over random sets in p2, and it")
print("  is NOT established anywhere in the arm sets arvo has today. O-139-C closes")
print("  in the direction 139 named as the alternative.")
print("  The lesson is about the gate rather than the arms: a family-level noise")
print("  check passed at 9.2 while the pairwise comparison that decided the verdict")
print("  sat at 0.05. Dominance and selectability are pairwise questions, so the")
print("  gate has to be pairwise, and mine was not until this probe.")

print()
print("=" * 78)
print(f"control failures: {len(set(fail))} {sorted(set(fail))}")
print("=" * 78)
raise SystemExit(1 if fail else 0)
