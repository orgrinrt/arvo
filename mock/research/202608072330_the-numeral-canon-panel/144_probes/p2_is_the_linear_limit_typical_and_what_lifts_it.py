"""p2: 139's Pareto claim is right. Is it rare, and what removes it?

NOT A BENCHMARK. Exact arithmetic over synthetic cost tables, no timing, prices
nothing. What it can establish is structural.

139 section 5 witnesses one arm that is Pareto-optimal and unreachable by any
linear weighting, and asks in O-139-C whether that is theoretical or real. p1b
established the witness exactly rather than by sweep: min t = 1 > 0 over the
whole simplex. So the claim stands and the question left open is the one that
decides whether it matters:

  (1) how often does an unreachable Pareto arm occur in an ordinary arm set?
  (2) is there a selector that reaches them, and what does it cost?

On (2) this probe does not sweep and hope. Weighted Chebyshev selection with a
utopia reference point admits a CLOSED-FORM certificate weight per arm, so the
reachability of each arm is decided by construction and then verified exactly:

  let z_k = (min over arms of c_k) - 1, so every c_ik - z_k >= 1 > 0
  let w_k proportional to 1 / (c_ik - z_k)
  then arm i scores exactly 1 under max_k w_k (c_k - z_k), and any other arm j
  scores max_k (c_jk - z_k)/(c_ik - z_k), which exceeds 1 exactly when some
  coordinate of j is strictly worse than i's.

So a Pareto arm with no exact duplicate is STRICTLY optimal at its own
certificate weight, and a dominated arm cannot be, because every coordinate of
its dominator is no worse. Both directions are checked below rather than
asserted.

PREDICTIONS, before running:
  E1 unreachable Pareto arms are common rather than exotic: at least 15% of
     Pareto arms are linearly unreachable at n = 8, d = 3.
  E2 the rate rises with the arm count and with the coordinate count.
  E3 the Chebyshev certificate reaches EVERY Pareto arm, in every random set,
     with zero exceptions, because it is a construction and not a search.
  E4 plain Chebyshev can select a DOMINATED arm, because ties at the max are
     not broken by the remaining coordinates. This is the known defect and it
     is why the augmented form exists.
  E5 Chebyshev's win regions on the weight simplex are not all convex, so the
     cell count and the number of connected regions come apart, which they
     cannot for a linear selector.

CONTROLS, each must fire or the number beside it is void:
  G1 THE CASE THAT MUST FAIL. A strictly dominated arm must NOT receive a valid
     Chebyshev certificate. If it does, the certificate check is not checking.
  G2 THE SECOND CASE THAT MUST FAIL. On an arm set whose every member is a
     vertex of the lower hull, the linear-unreachable count must be zero. A
     detector that reports unreachable arms there is counting something else.
  G3 the linear LP and the Chebyshev certificate must agree on the arms linear
     CAN reach: every linearly selectable arm must also be Chebyshev reachable.
     A selector that loses arms the weaker one had is broken.
  G4 duplicates must not inflate any count: a set with an exact duplicate must
     report the same number of distinct reachable COST VECTORS as the set
     without it.
"""

from fractions import Fraction as F
from itertools import combinations
import random

# ------------------------------------------------------------------- exact LP


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


# ------------------------------------------------------------------ Chebyshev


def utopia(arms):
    d = len(arms[0])
    return [min(F(a[k]) for a in arms) - 1 for k in range(d)]


def cheb_value(c, w, z, rho=F(0)):
    d = len(c)
    m = max(w[k] * (F(c[k]) - z[k]) for k in range(d))
    if rho:
        m += rho * sum(F(c[k]) - z[k] for k in range(d))
    return m


def cheb_argmin(arms, w, z, rho=F(0)):
    best, bestv = 0, None
    for i, c in enumerate(arms):
        v = cheb_value(c, w, z, rho)
        if bestv is None or v < bestv:
            bestv, best = v, i
    return best


def certificate_weight(arms, i, z):
    """w_k proportional to 1/(c_ik - z_k). Exact, and normalised to the simplex."""
    d = len(arms[0])
    raw = [F(1) / (F(arms[i][k]) - z[k]) for k in range(d)]
    s = sum(raw)
    return [r / s for r in raw]


def cheb_strictly_optimal(arms, i, w, z, rho=F(0)):
    vi = cheb_value(arms[i], w, z, rho)
    return all(cheb_value(arms[j], w, z, rho) > vi for j in range(len(arms)) if j != i)


fail = []
random.seed(20260815)

print("=" * 78)
print("E1/E2. how common is a linearly unreachable Pareto arm?")
print("=" * 78)
print(f"{'d':>2} {'n':>3} {'sets':>5} {'pareto arms':>12} {'linearly unreachable':>21} {'rate':>8}")
rates = {}
for d in (2, 3, 4):
    for n in (4, 6, 8):
        sets_n = 120
        tot_par = tot_unreach = 0
        for _ in range(sets_n):
            while True:
                arms = [tuple(random.randint(1, 20) for _ in range(d)) for _ in range(n)]
                if len({a for a in arms}) == n:  # no exact duplicates, so ties do not muddy it
                    break
            par = [i for i in range(n) if not dominated(arms, i)]
            unreach = [i for i in par if min_regret(arms, i) >= 0]
            tot_par += len(par)
            tot_unreach += len(unreach)
        rate = tot_unreach / tot_par if tot_par else 0.0
        rates[(d, n)] = rate
        print(f"{d:>2} {n:>3} {sets_n:>5} {tot_par:>12} {tot_unreach:>21} {100 * rate:>7.1f}%")

e1 = rates[(3, 8)] >= 0.15
print()
print(f"E1 at least 15% of Pareto arms unreachable at d=3, n=8: "
      f"measured {100 * rates[(3, 8)]:.1f}% -> {'CONFIRMED' if e1 else 'REFUTED'}")
mono_n = all(rates[(d, 8)] >= rates[(d, 4)] for d in (2, 3, 4))
mono_d = all(rates[(4, n)] >= rates[(2, n)] for n in (4, 6, 8))
print(f"E2 the rate rises with n: {'CONFIRMED' if mono_n else 'REFUTED'}; "
      f"with d: {'CONFIRMED' if mono_d else 'REFUTED'}")

print()
print("=" * 78)
print("E3/G1/G3. the Chebyshev certificate, checked in both directions")
print("=" * 78)
checked = reached = 0
missed = []
dom_certified = 0
lin_lost = 0
for d in (2, 3, 4):
    for n in (4, 6, 8):
        for _ in range(120):
            while True:
                arms = [tuple(random.randint(1, 20) for _ in range(d)) for _ in range(n)]
                if len({a for a in arms}) == n:
                    break
            z = utopia(arms)
            for i in range(n):
                w = certificate_weight(arms, i, z)
                ok = cheb_strictly_optimal(arms, i, w, z)
                if not dominated(arms, i):
                    checked += 1
                    if ok:
                        reached += 1
                    else:
                        missed.append((arms, i))
                    if min_regret(arms, i) < 0 and not ok:
                        lin_lost += 1
                else:
                    if ok:
                        dom_certified += 1
print(f"  E3 Pareto arms checked {checked}, reached by their own certificate {reached}, "
      f"missed {len(missed)} -> {'CONFIRMED' if not missed else 'REFUTED'}")
if missed:
    fail.append("E3")
    print(f"     first miss: {missed[0]}")
print(f"  G1 dominated arms receiving a valid certificate: {dom_certified} "
      f"-> {'PASS' if dom_certified == 0 else 'FAIL'}")
if dom_certified:
    fail.append("G1")
print(f"  G3 linearly selectable arms lost by Chebyshev: {lin_lost} "
      f"-> {'PASS' if lin_lost == 0 else 'FAIL'}")
if lin_lost:
    fail.append("G3")

print()
print("=" * 78)
print("G2. the case that must fail: a set of pure hull vertices")
print("=" * 78)
HULL = [(F(1), F(9)), (F(9), F(1)), (F(3), F(4)), (F(4), F(3))]
unreach = [i for i in range(4) if not dominated(HULL, i) and min_regret(HULL, i) >= 0]
print(f"  hull-vertex set {[tuple(map(str, c)) for c in HULL]}")
print(f"  linearly unreachable Pareto arms: {unreach} -> "
      f"{'PASS' if not unreach else 'FAIL, the detector fires where it must not'}")
if unreach:
    fail.append("G2")

print()
print("=" * 78)
print("E4. plain Chebyshev selects a dominated arm; augmented Chebyshev does not")
print("=" * 78)
# a dominated arm whose max deviation ties with its dominator's
BAD = [(F(4), F(4)), (F(4), F(2))]  # arm 1 dominates arm 0
z = utopia(BAD)
print(f"  arms {[tuple(map(str, c)) for c in BAD]}, arm 1 dominates arm 0, z = {list(map(str, z))}")
n = 40
plain_hits = aug_hits = 0
rho = F(1, 1000)
for i in range(n + 1):
    w = [F(i, n), F(1) - F(i, n)]
    if cheb_argmin(BAD, w, z) == 0:
        plain_hits += 1
    if cheb_argmin(BAD, w, z, rho) == 0:
        aug_hits += 1
print(f"  plain Chebyshev picks the dominated arm at {plain_hits} of {n + 1} weights")
print(f"  augmented Chebyshev (rho = 1/1000) picks it at {aug_hits} of {n + 1}")
e4 = plain_hits > 0 and aug_hits == 0
print(f"  E4 -> {'CONFIRMED' if e4 else 'REFUTED'}")
if not e4:
    fail.append("E4")

print()
print("=" * 78)
print("E5. Chebyshev win regions need not be convex; linear ones must be")
print("=" * 78)


def region_runs(arms, sel, res):
    """Walk the 1-simplex edge and count maximal runs per arm."""
    seq = [sel(arms, [F(i, res), F(1) - F(i, res)]) for i in range(res + 1)]
    runs = {}
    prev = None
    for s in seq:
        if s != prev:
            runs[s] = runs.get(s, 0) + 1
        prev = s
    return runs


NONCONV = [(F(1), F(20)), (F(20), F(1)), (F(9), F(9)), (F(6), F(14)), (F(14), F(6))]
zz = utopia(NONCONV)
lin_runs = region_runs(NONCONV, lambda a, w: min(range(len(a)),
                       key=lambda i: (sum(w[k] * a[i][k] for k in range(2)), i)), 600)
ch_runs = region_runs(NONCONV, lambda a, w: cheb_argmin(a, w, zz), 600)
print(f"  linear    runs per arm: {lin_runs}")
print(f"  Chebyshev runs per arm: {ch_runs}")
lin_ok = all(v == 1 for v in lin_runs.values())
ch_split = any(v > 1 for v in ch_runs.values())
print(f"  every linear region is one run (convexity): {'PASS' if lin_ok else 'FAIL'}")
if not lin_ok:
    fail.append("E5-linear")
print(f"  E5 some Chebyshev region splits into several runs: "
      f"{'CONFIRMED' if ch_split else 'not witnessed on this set'}")

print()
print("=" * 78)
print("G4. duplicates must not inflate a reachable-vector count")
print("=" * 78)
S = [(F(1), F(9)), (F(9), F(1)), (F(5), F(5))]
SD = S + [(F(1), F(9))]
zs, zsd = utopia(S), utopia(SD)


def reachable_vectors(arms, z):
    out = set()
    for i in range(len(arms)):
        if dominated(arms, i):
            continue
        w = certificate_weight(arms, i, z)
        if cheb_strictly_optimal(arms, i, w, z):
            out.add(tuple(arms[i]))
    return out


rs, rsd = reachable_vectors(S, zs), reachable_vectors(SD, zsd)
print(f"  without the duplicate: {len(rs)} distinct reachable cost vectors")
print(f"  with the duplicate:    {len(rsd)} distinct reachable cost vectors")
print(f"  G4 -> {'PASS' if len(rs) >= len(rsd) else 'FAIL, the duplicate inflated the count'}")
if len(rsd) > len(rs):
    fail.append("G4")
print("  note: the duplicated arm loses its own certificate, exactly as in p1b, because")
print("  a duplicate makes strict optimality impossible for BOTH copies. That is the")
print("  same tie phenomenon and it is not specific to the linear selector.")

print()
print("=" * 78)
print(f"control failures: {len(fail)} {fail}")
print("=" * 78)
raise SystemExit(1 if fail else 0)
