"""p3: the observable vocabulary of the weighting component, bounded and measured.

NOT A BENCHMARK. Exact arithmetic, no timing, prices nothing.

139 section 5 says "the observable weighting count is bounded by the arm count,
so however continuous the weight space is, the design's vocabulary for it need
never exceed the number of arms." That bound is correct and it needs no
measurement: a selector is a function from weights into a finite set of arms, so
its image is finite and no larger than the set. The quotient claim is a fact
about functions rather than a fact about weightings, and calling it a
measurement overstates what p4 established.

The bound that is NOT free is the interesting one: for a LINEAR selector the
image is not the arm set, nor the Pareto set. It is the set of SUPPORTED
efficient arms, and that is what a design shipping named weightings can ever
distinguish. This probe measures it against the arm count and the coordinate
count, and measures how much a Chebyshev selector widens it.

PREDICTIONS, before running:
  H1 the number of arms a linear selector can ever pick uniquely grows well
     below the arm count. At n = 16, d = 3, fewer than 8 on average.
  H2 the Chebyshev selector's image is exactly the non-dominated set, so its
     vocabulary is strictly wider than the linear one wherever an unsupported
     efficient arm exists.
  H3 a Chebyshev win region on the one-dimensional weight edge can be
     DISCONNECTED, which a linear one cannot be. I expect to find one by search
     over random two-coordinate sets within a few hundred tries.

CONTROLS:
  I1 THE CASE THAT MUST FAIL. On a set of collinear cost points, all on a single
     lower-hull edge, the linear unique-winner count must be 2 (the endpoints)
     rather than n. A counter reporting n there is counting arms rather than
     cells.
  I2 every linear unique winner must also be Chebyshev reachable, on every set.
     A widening that loses members is not a widening.
  I3 the linear region walk must report exactly one run per winning arm on every
     set tested, because linear argmin regions are intersections of half-spaces
     and therefore convex. If a linear region ever splits, the walk is broken
     and H3's evidence from the same walk is void.
"""

from fractions import Fraction as F
from itertools import combinations
import random


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


def utopia(arms):
    d = len(arms[0])
    return [min(F(a[k]) for a in arms) - 1 for k in range(d)]


def cheb_value(c, w, z):
    return max(w[k] * (F(c[k]) - z[k]) for k in range(len(c)))


def cheb_reachable(arms, i, z):
    d = len(arms[0])
    raw = [F(1) / (F(arms[i][k]) - z[k]) for k in range(d)]
    s = sum(raw)
    w = [r / s for r in raw]
    vi = cheb_value(arms[i], w, z)
    return all(cheb_value(arms[j], w, z) > vi for j in range(len(arms)) if j != i)


fail = []
random.seed(20260815 + 1)

print("=" * 78)
print("H1/H2. how wide is the observable weighting vocabulary?")
print("=" * 78)
print(f"{'d':>2} {'n':>3} {'sets':>5} {'mean arms':>10} {'mean pareto':>12} "
      f"{'mean linear image':>18} {'mean cheb image':>16}")
results = {}
for d in (2, 3):
    for n in (4, 8, 12, 16):
        sets_n = 40
        s_par = s_lin = s_ch = 0
        for _ in range(sets_n):
            while True:
                arms = [tuple(random.randint(1, 40) for _ in range(d)) for _ in range(n)]
                if len({a for a in arms}) == n:
                    break
            z = utopia(arms)
            par = [i for i in range(n) if not dominated(arms, i)]
            lin = [i for i in range(n) if min_regret(arms, i) < 0]
            ch = [i for i in par if cheb_reachable(arms, i, z)]
            s_par += len(par)
            s_lin += len(lin)
            s_ch += len(ch)
            bad = [i for i in lin if i not in ch]
            if bad:
                fail.append("I2")
        results[(d, n)] = (s_par / sets_n, s_lin / sets_n, s_ch / sets_n)
        print(f"{d:>2} {n:>3} {sets_n:>5} {n:>10} {s_par / sets_n:>12.2f} "
              f"{s_lin / sets_n:>18.2f} {s_ch / sets_n:>16.2f}")

h1 = results[(3, 16)][1] < 8
print()
print(f"H1 linear image below 8 at d=3, n=16: measured {results[(3, 16)][1]:.2f} "
      f"-> {'CONFIRMED' if h1 else 'REFUTED'}")
h2 = all(results[k][2] >= results[k][1] for k in results) and any(
    results[k][2] > results[k][1] for k in results)
print(f"H2 the Chebyshev image is never smaller and somewhere strictly larger: "
      f"{'CONFIRMED' if h2 else 'REFUTED'}")
print(f"I2 every linear unique winner is Chebyshev reachable: "
      f"{'PASS' if 'I2' not in fail else 'FAIL'}")

print()
print("  the ratio that matters for a design shipping named weightings:")
for k in sorted(results):
    par, lin, ch = results[k]
    print(f"    d={k[0]} n={k[1]:>2}: linear can ever distinguish {lin:.2f} of {k[1]} arms "
          f"({100 * lin / k[1]:.0f}%), Chebyshev {ch:.2f} ({100 * ch / k[1]:.0f}%)")

print()
print("=" * 78)
print("I1. the case that must fail: collinear arms")
print("=" * 78)
COLL = [(F(0), F(12)), (F(3), F(9)), (F(6), F(6)), (F(9), F(3)), (F(12), F(0))]
lin = [i for i in range(5) if min_regret(COLL, i) < 0]
par = [i for i in range(5) if not dominated(COLL, i)]
print(f"  five collinear points on one hull edge: pareto={len(par)}, "
      f"linear unique winners={len(lin)} {lin}")
print(f"  I1 -> {'PASS' if len(lin) == 2 else 'FAIL, expected the two endpoints only'}")
if len(lin) != 2:
    fail.append("I1")
zc = utopia(COLL)
chc = [i for i in par if cheb_reachable(COLL, i, zc)]
print(f"  and Chebyshev reaches {len(chc)} of the {len(par)}: {chc}")
print("  so on a linear tradeoff frontier a linear selector can name exactly the two")
print("  extremes and nothing between them, while Chebyshev names every point.")

print()
print("=" * 78)
print("H3/I3. is a Chebyshev win region ever disconnected?")
print("=" * 78)
RES = 2000


def walk(arms, val):
    seq = []
    for i in range(RES + 1):
        w = [F(i, RES), F(1) - F(i, RES)]
        best, bv = 0, None
        for j, c in enumerate(arms):
            v = val(c, w)
            if bv is None or v < bv:
                bv, best = v, j
        seq.append(best)
    runs = {}
    prev = None
    for s in seq:
        if s != prev:
            runs[s] = runs.get(s, 0) + 1
        prev = s
    return runs


found = None
lin_split = 0
tries = 0
random.seed(20260815 + 2)
for _ in range(400):
    n = random.randint(3, 6)
    arms = [tuple(random.randint(1, 30) for _ in range(2)) for _ in range(n)]
    if len({a for a in arms}) != n:
        continue
    tries += 1
    z = utopia(arms)
    lr = walk(arms, lambda c, w: sum(w[k] * F(c[k]) for k in range(2)))
    if any(v > 1 for v in lr.values()):
        lin_split += 1
    cr = walk(arms, lambda c, w: cheb_value(c, w, z))
    if any(v > 1 for v in cr.values()) and found is None:
        found = (arms, cr, lr)

print(f"  searched {tries} random two-coordinate sets at grid resolution 1/{RES}")
print(f"  I3 linear regions that split into more than one run: {lin_split} "
      f"-> {'PASS' if lin_split == 0 else 'FAIL, the walk is broken'}")
if lin_split:
    fail.append("I3")
if found:
    arms, cr, lr = found
    print(f"  H3 CONFIRMED. witness arms {[tuple(map(str, c)) for c in arms]}")
    print(f"     Chebyshev runs per arm {cr}  (an arm with 2 runs wins, loses, wins again)")
    print(f"     linear runs per arm    {lr}")
else:
    print("  H3 NOT WITNESSED in this search. The claim is unestablished and is not used.")
    fail.append("H3-not-witnessed")

print()
print("=" * 78)
print(f"control failures: {len(fail)} {sorted(set(fail))}")
print("=" * 78)
raise SystemExit(1 if [f for f in fail if not f.startswith('H3')] else 0)
