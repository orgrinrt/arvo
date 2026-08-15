"""p3b: which argument governs the weighting vocabulary, the arm count or the
coordinate count?

NOT A BENCHMARK. Exact arithmetic, no timing, prices nothing.

p3 measured the linear selector's image at d in {2, 3} and found it small and
slow-growing in n. If that saturation is real, the number of named weightings a
design can ever ship distinguishably is governed by how many cost coordinates it
has and almost not at all by how many arms it has. That is a design number
rather than a curiosity, because the coordinate set is a design choice (139's
O-139-E) and the arm count is not.

Two earlier versions of this probe did not finish in ten minutes at d = 5. The
first used p1's full vertex enumeration, which is C(n - 1 + d, d) systems per
arm. The second wrapped that in an active set that never dropped a constraint,
so the working set grew until the inner enumeration was the outer one. Both dead
routes are recorded in `exact_lp.py`. What runs here is the dual
characterisation, decided by an exact phase-one simplex, and it is checked
against the enumeration below rather than trusted.

PREDICTIONS, before running:
  J1 the image rises monotonically with d at every n, over d in {1..5}.
  J2 the image saturates in n: tripling the arm count from 8 to 24 moves it less
     than adding one coordinate does.

CONTROLS:
  K1 THE CASE THAT MUST FAIL. At d = 1 the image must be exactly 1: one
     coordinate admits one weight up to scale and only the cheapest arm can win.
     Any other answer means the counter is not counting the image.
  K2 no dominated arm may ever be strictly selectable.
  K3 THE SOLVER CONTROL. The active-set solver must agree with the slow full
     enumeration, exactly, on every arm of 150 random small sets, including the
     zero and positive cases. Agreement only on negatives would leave the
     boundary untested, so the run reports how many of each sign it compared.
"""

from fractions import Fraction as F
from itertools import combinations
import random

from exact_lp import min_regret, strictly_selectable, dominated


def slow_solve(rows, rhs):
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


def slow_min_regret(arms, i):
    """p1's method: enumerate every vertex over the whole constraint set."""
    d = len(arms[0])
    diffs = [[F(arms[i][k]) - F(arms[j][k]) for k in range(d)]
             for j in range(len(arms)) if j != i]
    if not diffs:
        return F(0)
    pool = []
    for k in range(d):
        row = [F(0)] * (d + 1)
        row[k] = F(1)
        pool.append((row, F(0)))
    for dv in diffs:
        pool.append(([dv[k] for k in range(d)] + [F(-1)], F(0)))
    eq = ([F(1)] * d + [F(0)], F(1))
    best = None
    for combo in combinations(range(len(pool)), d):
        rows = [eq[0]] + [pool[c][0] for c in combo]
        rhs = [eq[1]] + [pool[c][1] for c in combo]
        sol = slow_solve(rows, rhs)
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


fail = []

print("=" * 78)
print("K3. the separation LP against the full enumeration")
print("=" * 78)
random.seed(20260815 + 11)
neg = zero = pos = 0
mismatch = 0
for _ in range(150):
    d = random.randint(2, 4)
    n = random.randint(3, 7)
    arms = [tuple(F(random.randint(1, 12)) for _ in range(d)) for _ in range(n)]
    if len(set(arms)) != n:
        continue
    for i in range(n):
        a, b = strictly_selectable(arms, i), slow_min_regret(arms, i)
        if a != (b < 0):
            mismatch += 1
            if mismatch == 1:
                print(f"  MISMATCH arms={arms} i={i} separation={a} enumeration t={b}")
        if b < 0:
            neg += 1
        elif b == 0:
            zero += 1
        else:
            pos += 1
print(f"  compared {neg + zero + pos} arms: {neg} strictly negative, {zero} exactly zero, "
      f"{pos} strictly positive")
print(f"  mismatches: {mismatch} -> {'PASS' if mismatch == 0 else 'FAIL'}")
if mismatch:
    fail.append("K3")
if zero == 0 or pos == 0:
    print("  K3 WEAK: the boundary cases were not exercised, so agreement means less")
    fail.append("K3-untested-boundary")

print()
print("=" * 78)
print("J1/J2. the image against the two arguments")
print("=" * 78)
random.seed(20260815 + 7)
SETS = 20
table = {}
print(f"{'d':>2} " + "".join(f"{('n=' + str(n)):>10}" for n in (8, 16, 24)))
for d in (1, 2, 3, 4, 5):
    row = []
    for n in (8, 16, 24):
        tot = 0
        for _ in range(SETS):
            while True:
                arms = [tuple(F(random.randint(1, 60)) for _ in range(d)) for _ in range(n)]
                if len(set(arms)) == n:
                    break
            img = [i for i in range(n) if strictly_selectable(arms, i)]
            if any(dominated(arms, i) for i in img):
                fail.append("K2")
            tot += len(img)
        table[(d, n)] = tot / SETS
        row.append(tot / SETS)
    print(f"{d:>2} " + "".join(f"{v:>10.2f}" for v in row))

print()
k1 = all(table[(1, n)] == 1.0 for n in (8, 16, 24))
print(f"K1 image at d=1 is exactly 1: {[table[(1, n)] for n in (8, 16, 24)]} "
      f"-> {'PASS' if k1 else 'FAIL'}")
if not k1:
    fail.append("K1")
print(f"K2 no dominated arm strictly selectable: {'PASS' if 'K2' not in fail else 'FAIL'}")

j1 = all(table[(d, n)] <= table[(d + 1, n)] for d in (1, 2, 3, 4) for n in (8, 16, 24))
print(f"J1 image rises monotonically with d at every n: {'CONFIRMED' if j1 else 'REFUTED'}")

print()
print("J2 which argument moves it more:")
for d in (2, 3, 4):
    dn = table[(d, 24)] - table[(d, 8)]
    dd = table[(d + 1, 16)] - table[(d, 16)]
    print(f"   d={d}: tripling the arm count 8 -> 24 adds {dn:+.2f}; "
          f"adding one coordinate adds {dd:+.2f}")
j2 = all((table[(d, 24)] - table[(d, 8)]) < (table[(d + 1, 16)] - table[(d, 16)])
         for d in (2, 3))
print(f"   J2 -> {'CONFIRMED' if j2 else 'REFUTED'}")

print()
print("reading, with J2 refuted and the conclusion narrowed to what survives:")
print("  the image grows in BOTH arguments, so my saturation prediction was wrong,")
print("  and above d = 3 the arm count moves it more than a coordinate does. What")
print("  survives is the level rather than the growth. At d = 2 the image is 2.45")
print("  to 2.70 arms whatever the arm count, and at d = 3 it is 3.55 to 5.75. In")
print("  the region 139 actually proposes, three cost coordinates, a linear")
print("  selector can distinguish about four to six arms however many exist, so a")
print("  design naming many more weightings than that has named points nothing can")
print("  tell apart. At d = 5 the image reaches half the arm set and the bound stops")
print("  being interesting, which is itself a reason to keep the coordinate set small.")
print()
print(f"control failures: {len(set(fail))} {sorted(set(fail))}")
raise SystemExit(1 if fail else 0)
