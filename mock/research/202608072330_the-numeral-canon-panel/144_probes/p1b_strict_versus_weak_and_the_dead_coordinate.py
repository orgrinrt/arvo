"""p1b: repairing the two controls that fired in p1, and what they were telling me.

NOT A BENCHMARK. Exact arithmetic, no timing, prices nothing.

p1 ran two controls that failed, and both failures were the instrument being
right about something I had not modelled.

  C6 FAILED. `scalar-widened` wins 55720 grid points and my LP said it is not
  strictly selectable. Both are correct. The grid breaks ties by index and the
  LP asks for a STRICT minimum, and the arm has an exact duplicate in the set,
  so no weight makes it the unique argmin. Grid-winner and strictly-selectable
  are two different predicates and I compared across them.

  A5 FAILED. 139's compromise-C witness has min t = 0, not > 0, so it is WEAKLY
  selectable: some weight makes it tie for best. The weight is (0, 0, 1). Every
  arm in that witness carries 0 in the third coordinate, so putting all the
  weight there makes the objective identically zero and everything ties. 139's
  own sweep never saw this because it swept only the edge (t, 1-t, 0).

Both repairs, and the second is a finding rather than a fix.

PREDICTIONS, before running:
  B1 with weak selectability (min t <= 0) as the predicate, every grid winner in
     both of 139's tables qualifies. C6 repaired and the disagreement was mine.
  B2 in its true dimension, which is two, compromise-C has min t STRICTLY
     positive, so the Pareto claim is exact rather than grid-limited once the
     dead coordinate is dropped.
  B3 a coordinate on which every arm carries the same value makes EVERY arm
     weakly selectable, because all the weight can be put on it. So a constant
     coordinate destroys strict selection everywhere, not only for the witness.
  B4 `scalar-widened`'s min t = 0 is caused by its duplicate. Deleting the
     duplicate makes it strictly selectable, min t < 0.

CONTROLS:
  D1 THE CASE THAT MUST FAIL for B3: on a table with no constant coordinate and
     no duplicate, at least one arm must have min t > 0, namely a dominated one.
     If everything is weakly selectable there too, B3 is measuring nothing.
  D2 THE CASE THAT MUST FAIL for B2: an arm genuinely on the lower hull in the
     same two-dimensional set must come out with min t < 0. If the
     two-dimensional LP calls everything unselectable it is broken.
  D3 the exact-arithmetic grid must agree with 139's f64 grid on the CELL COUNT
     even where it disagrees on cell populations, and the population difference
     must be small. A large difference would mean one of the two sweeps is
     wrong rather than that f64 rounds at a boundary.
"""

from fractions import Fraction as F
from itertools import combinations

import importlib.util
import os

_spec = importlib.util.spec_from_file_location(
    "p1", os.path.join(os.path.dirname(os.path.abspath(__file__)),
                       "p1_reproduce_and_decompose.py"))

# p1 exits at import, so the pieces are re-stated here rather than imported.
# Duplicating them is deliberate: an instrument that shares code with the one it
# repairs cannot show the repair is independent of the defect.


def solve_exact(rows, rhs):
    n = len(rows)
    a = [list(r) + [rhs[i]] for i, r in enumerate(rows)]
    for col in range(n):
        piv = next((r for r in range(col, n) if a[r][col] != 0), None)
        if piv is None:
            return None
        a[col], a[piv] = a[piv], a[col]
        pv = a[col][col]
        a[col] = [x / pv for x in a[col]]
        for r in range(n):
            if r != col and a[r][col] != 0:
                f = a[r][col]
                a[r] = [x - f * y for x, y in zip(a[r], a[col])]
    return [a[r][n] for r in range(n)]


def min_regret(arms, i):
    """min over the simplex of  max_j (w.c_i - w.c_j).  Exact."""
    d = len(arms[0])
    diffs = [[F(arms[i][k]) - F(arms[j][k]) for k in range(d)]
             for j in range(len(arms)) if j != i]
    if not diffs:
        return F(0)
    nv = d + 1
    pool = []
    for k in range(d):
        row = [F(0)] * nv
        row[k] = F(1)
        pool.append((row, F(0)))
    for dv in diffs:
        pool.append(([dv[k] for k in range(d)] + [F(-1)], F(0)))
    eq = ([F(1)] * d + [F(0)], F(1))
    best = None
    for combo in combinations(range(len(pool)), d):
        rows = [eq[0]] + [pool[c][0] for c in combo]
        rhs = [eq[1]] + [pool[c][1] for c in combo]
        sol = solve_exact(rows, rhs)
        if sol is None:
            continue
        w, t = sol[:d], sol[d]
        if any(x < 0 for x in w):
            continue
        if any(sum(dv[k] * w[k] for k in range(d)) > t for dv in diffs):
            continue
        if best is None or t < best:
            best = t
    return best


def dominated(arms, i):
    d = len(arms[0])
    return any(
        j != i and all(arms[j][k] <= arms[i][k] for k in range(d))
        and any(arms[j][k] < arms[i][k] for k in range(d))
        for j in range(len(arms))
    )


def argmin_lin(arms, w):
    best, bestv = 0, None
    for i, c in enumerate(arms):
        v = sum(w[k] * c[k] for k in range(len(c)))
        if bestv is None or v < bestv:
            bestv, best = v, i
    return best


def sweep_wins3(arms, n):
    wins = [0] * len(arms)
    for i in range(n + 1):
        for j in range(n - i + 1):
            k = n - i - j
            wins[argmin_lin(arms, (F(i, n), F(j, n), F(k, n)))] += 1
    return wins


NAMES = ["scalar-widened", "packed", "packed-simd", "table-lookup",
         "narrow-band-compromise", "naive-loop", "scalar-widened [dup]"]
T1 = [(F(10), F(40), F(32)), (F(26), F(90), F(13)), (F(14), F(240), F(13)),
      (F(8), F(60), F(512)), (F(31, 2), F(55), F(20)), (F(40), F(95), F(32)),
      (F(10), F(40), F(32))]
T2 = [(F(34), F(40), F(32)), (F(30), F(90), F(13)), (F(11), F(240), F(13)),
      (F(9), F(60), F(512)), (F(31, 2), F(55), F(20)), (F(40), F(95), F(32)),
      (F(34), F(40), F(32))]

fail = []

print("=" * 78)
print("B1. grid winners against WEAK selectability, which is the right predicate")
print("=" * 78)
for label, arms in (("target 1", T1), ("target 2", T2)):
    wins = sweep_wins3(arms, 400)
    winners = [i for i in range(len(arms)) if wins[i] > 0]
    weak = [i for i in range(len(arms)) if min_regret(arms, i) <= 0]
    strict = [i for i in range(len(arms)) if min_regret(arms, i) < 0]
    print(f"  {label}: grid winners {sorted(winners)}, weakly selectable {sorted(weak)}, "
          f"strictly selectable {sorted(strict)}")
    bad = [i for i in winners if i not in weak]
    print(f"    every grid winner weakly selectable -> {'PASS' if not bad else 'FAIL ' + str(bad)}")
    if bad:
        fail.append(f"B1 {label}")
    # the tie-free cell count: distinct cost vectors among the weakly selectable
    vecs = {tuple(arms[i]) for i in weak}
    print(f"    distinct weakly-selectable COST VECTORS = {len(vecs)}  "
          f"(139 reports 5 cells; the arm count and the vector count differ only by ties)")

print()
print("=" * 78)
print("B4. scalar-widened's zero is its duplicate")
print("=" * 78)
for label, arms in (("target 1", T1), ("target 2", T2)):
    before = min_regret(arms, 0)
    sub = arms[:6]
    after = min_regret(sub, 0)
    print(f"  {label}: with the duplicate present min t = {before}; "
          f"with it deleted min t = {after}  -> "
          f"{'B4 holds' if before == 0 and after < 0 else 'B4 REFUTED'}")
    if not (before == 0 and after < 0):
        fail.append(f"B4 {label}")

print()
print("=" * 78)
print("B2. the witness in its true dimension")
print("=" * 78)
W3 = [(F(0), F(10), F(0)), (F(10), F(0), F(0)), (F(6), F(6), F(0))]
W2 = [(F(0), F(10)), (F(10), F(0)), (F(6), F(6))]
WN = ["endpoint-A", "endpoint-B", "compromise-C"]
for lbl, arms in (("three coordinates, the third dead", W3), ("two coordinates, the real problem", W2)):
    print(f"  {lbl}:")
    for i, nm in enumerate(WN):
        t = min_regret(arms, i)
        print(f"    {nm:<14} pareto={'Y' if not dominated(arms, i) else 'n'}  min t = {t}  "
              f"{'strictly selectable' if t < 0 else ('ties only' if t == 0 else 'never even ties')}")
t2 = min_regret(W2, 2)
print(f"  B2: compromise-C in two coordinates has min t = {t2}, strictly positive? "
      f"{'YES' if t2 > 0 else 'NO'}")
if not (t2 > 0):
    fail.append("B2")
print(f"      so it loses by at least {t2} at EVERY point of the simplex. A sweep can only")
print(f"      report that it did not win at the points sampled. This is the whole simplex.")
# D2
d2 = min_regret(W2, 0)
print(f"  D2 control: endpoint-A in the same two-coordinate set has min t = {d2}, "
      f"{'PASS' if d2 < 0 else 'FAIL'}")
if not (d2 < 0):
    fail.append("D2")

print()
print("=" * 78)
print("B3. a constant coordinate makes every arm weakly selectable")
print("=" * 78)
BASE = [(F(1), F(9)), (F(9), F(1)), (F(5), F(5)), (F(9), F(9))]
BN = ["a", "b", "mid", "bad"]
print("  base set, two coordinates, no constant coordinate, no duplicate:")
for i, nm in enumerate(BN):
    print(f"    {nm:<5} min t = {min_regret(BASE, i)}  dominated={dominated(BASE, i)}")
worst = [i for i in range(4) if min_regret(BASE, i) > 0]
print(f"  D1 control: at least one arm has min t > 0 -> "
      f"{'PASS ' + str([BN[i] for i in worst]) if worst else 'FAIL, everything is weakly selectable'}")
if not worst:
    fail.append("D1")

PAD = [c + (F(7),) for c in BASE]
print("  the same set with a third coordinate on which every arm carries 7:")
allweak = True
for i, nm in enumerate(BN):
    t = min_regret(PAD, i)
    print(f"    {nm:<5} min t = {t}")
    if t > 0:
        allweak = False
print(f"  B3: every arm weakly selectable after padding -> "
      f"{'CONFIRMED' if allweak else 'REFUTED'}")
if not allweak:
    fail.append("B3")
print("      mechanism: put all the weight on the constant coordinate and every arm")
print("      scores the same, so the argmin is decided entirely by the tie-break rule.")
print("      A cost coordinate on which the arms do not differ is not a coordinate; it")
print("      is a direction in weight space along which the selector says nothing.")

print()
print("=" * 78)
print("D3. exact arithmetic against 139's f64 sweep")
print("=" * 78)
F64_T1 = [55720, 4514, 2269, 52, 18046, 0, 0]
F64_T2 = [20102, 3514, 4153, 683, 52149, 0, 0]
for label, arms, ref in (("target 1", T1, F64_T1), ("target 2", T2, F64_T2)):
    mine = sweep_wins3(arms, 400)
    diff = sum(abs(a - b) for a, b in zip(mine, ref))
    print(f"  {label}: 139 f64 {ref}")
    print(f"           mine exact {mine}")
    print(f"    total absolute difference {diff} of {sum(mine)} grid points "
          f"({100.0 * diff / sum(mine):.4f}%), cell count "
          f"{'agrees' if sum(1 for x in mine if x) == sum(1 for x in ref if x) else 'DISAGREES'}")
    if sum(1 for x in mine if x) != sum(1 for x in ref if x):
        fail.append(f"D3 {label}")
    if diff > 20:
        fail.append(f"D3-magnitude {label}")

print()
print("=" * 78)
print(f"control failures: {len(fail)} {fail}")
print("=" * 78)
raise SystemExit(1 if fail else 0)
