"""Exact decision procedures over a finite arm set, shared by the 144 probes.

Two questions, both answered exactly over Fractions so that a boundary case
cannot be rounded across. Boundaries are not rare here: collinear cost points
put the optimum exactly at zero, and a float solver lands on whichever side its
tolerance happens to favour.

`min_regret(arms, i)` returns

    min over w in the simplex of   max over j != i of   w . (c_i - c_j)

so t < 0 means some weight makes arm i the unique argmin of the linear
objective, t = 0 means it can only tie, and t > 0 means it is never even weakly
optimal. This is exact vertex enumeration over the whole constraint set and it
costs C(n - 1 + d, d) systems per arm, which is fine for small sets and is what
the other procedure is checked against.

`strictly_selectable(arms, i)` answers the sign question alone, by the dual
characterisation rather than by enumeration:

    arm i is strictly selectable  <=>  c_i is NOT in conv({c_j : j != i}) + R^d_+

Separation gives it directly: c_i lies outside that closed convex set exactly
when some direction w separates it, and the recession cone R^d_+ forces w >= 0,
which is the simplex after normalising. Deciding membership is a phase-one
feasibility LP with d + 1 rows and n - 1 + d columns, so it costs a handful of
pivots rather than a combinatorial sweep, and it is what makes d = 5 reachable.

Two earlier attempts are recorded because they closed:

  * full vertex enumeration over every constraint, which is `min_regret` and
    which did not finish in ten minutes at d = 5, n = 24;
  * an active-set loop over the same enumeration, which did not finish either,
    because nothing dropped a constraint and the working set grew until the
    inner enumeration was the outer one.

THE CASE THAT MUST FAIL. `_selftest` runs on import and compares the two
procedures on a set of hand-built cases that include a strictly positive
optimum, an exactly zero one and a dominated arm. Agreement only on the strictly
negative cases would leave the boundary untested, which is the only place the
two procedures could plausibly differ.
"""

from fractions import Fraction as F
from itertools import combinations


def _solve(rows, rhs):
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
    """Exact optimum by vertex enumeration. Slow, and the reference."""
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
        sol = _solve(rows, rhs)
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


def _phase1_feasible(A, b):
    """Exact phase-one simplex. Is there x >= 0 with A x = b?  b must be >= 0."""
    m = len(A)
    n = len(A[0])
    # tableau: [A | I_artificial | b], objective = sum of artificials
    T = [list(A[r]) + [F(1) if c == r else F(0) for c in range(m)] + [b[r]]
         for r in range(m)]
    basis = [n + r for r in range(m)]
    # reduced cost row for minimising the artificial sum
    cost = [F(0)] * (n + m + 1)
    for r in range(m):
        for c in range(n + m + 1):
            cost[c] -= T[r][c]
    for c in range(n, n + m):
        cost[c] = F(0)
    guard = 0
    while True:
        guard += 1
        if guard > 4000:
            raise RuntimeError("phase-one simplex did not terminate")
        # Bland's rule: lowest index with negative reduced cost
        enter = next((c for c in range(n + m) if cost[c] < 0), None)
        if enter is None:
            break
        ratio, leave = None, None
        for r in range(m):
            if T[r][enter] > 0:
                q = T[r][n + m] / T[r][enter]
                if ratio is None or q < ratio or (q == ratio and basis[r] < basis[leave]):
                    ratio, leave = q, r
        if leave is None:
            break  # unbounded below, cannot happen with artificials bounded
        pv = T[leave][enter]
        T[leave] = [x / pv for x in T[leave]]
        for r in range(m):
            if r != leave and T[r][enter] != 0:
                f = T[r][enter]
                T[r] = [x - f * y for x, y in zip(T[r], T[leave])]
        if cost[enter] != 0:
            f = cost[enter]
            cost = [x - f * y for x, y in zip(cost, T[leave])]
        basis[leave] = enter
    obj = -cost[n + m]
    return obj == 0


def strictly_selectable(arms, i):
    """c_i not in conv(others) + R^d_+, decided by phase-one feasibility."""
    d = len(arms[0])
    others = [arms[j] for j in range(len(arms)) if j != i]
    if not others:
        return True
    m = len(others)
    # variables: lambda_0..lambda_{m-1}, s_0..s_{d-1}
    A, b = [], []
    for k in range(d):
        A.append([F(others[j][k]) for j in range(m)] + [F(1) if t == k else F(0)
                                                        for t in range(d)])
        b.append(F(arms[i][k]))
    A.append([F(1)] * m + [F(0)] * d)
    b.append(F(1))
    if any(x < 0 for x in b):
        raise ValueError("costs must be non-negative for this formulation")
    return not _phase1_feasible(A, b)


def dominated(arms, i):
    d = len(arms[0])
    return any(
        j != i and all(arms[j][k] <= arms[i][k] for k in range(d))
        and any(arms[j][k] < arms[i][k] for k in range(d))
        for j in range(len(arms))
    )


def _selftest():
    cases = [
        ([(F(0), F(10)), (F(10), F(0)), (F(6), F(6))], 2, F(1)),
        ([(F(0), F(10)), (F(10), F(0)), (F(6), F(6))], 0, F(-6)),
        ([(F(1), F(9)), (F(9), F(1)), (F(5), F(5)), (F(9), F(9))], 2, F(0)),
        ([(F(1), F(9)), (F(9), F(1)), (F(5), F(5)), (F(9), F(9))], 3, F(4)),
        ([(F(1), F(9)), (F(9), F(1)), (F(3), F(4)), (F(4), F(3))], 2, None),
    ]
    signs = set()
    for arms, i, want in cases:
        got = min_regret(arms, i)
        if want is not None:
            assert got == want, f"min_regret arm {i}: {got} != {want}"
        sel = strictly_selectable(arms, i)
        assert sel == (got < 0), (
            f"the two procedures disagree on arm {i}: enumeration t={got}, "
            f"separation says {sel}")
        signs.add(-1 if got < 0 else (0 if got == 0 else 1))
    assert signs == {-1, 0, 1}, f"boundary not exercised, signs seen: {signs}"
    return True


_selftest()
