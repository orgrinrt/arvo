"""p1: reproduce 139's p4 on an independent instrument, then decompose the gap.

NOT A BENCHMARK. It times nothing and prices nothing. It is exact arithmetic
over the synthetic cost tables 139 committed in `139_probes/p4_weight_cells.rs`,
plus an exact linear-programming decision procedure that its grid sweep does not
have. Under the workspace rule this is an ad-hoc quick spike as far as any
magnitude is concerned; what it can establish is arithmetic and structural.

WHY REPRODUCING IS POSSIBLE AND CORRECT HERE. 143 deliberately did NOT match
141's count because their axis sets differed and matching would have shown the
instrument was not independent. This case is the opposite: 139's arm tables are
literal committed constants, and the quantity is a deterministic function of
them. So an independent implementation MUST land on the same number, and a
disagreement would be a defect in one of the two instruments rather than a
finding. Independence here is in the code (Python, own argmin, own dominance,
own sweep, exact Fractions where the Rust used f64), not in the input.

THE THREE CLAIMS OF 139 SECTION 5 THIS ATTACKS:
  (1) seven arms quotient to five cells, on both targets;
  (2) the same weight vector picks a different arm at 44.3% of the simplex;
  (3) "a Pareto-optimal arm exists that no linear weighting can select",
      with a control said to prove the zero belongs to the arm.

PREDICTIONS, recorded before the first run:
  A1 the cell counts reproduce exactly: 7 arms, 6 pareto, 5 cells, both targets.
  A2 the mapping difference reproduces exactly: 838 of 1891, 44.3%.
  A3 the single Pareto-but-never-winning arm in EACH 7-arm table is the
     DUPLICATE, so the 6-versus-5 gap is an artifact of index-order tie-breaking
     and NOT an instance of the unsupported-efficient-point phenomenon. Deleting
     the duplicate should give pareto=5, cells=5, gap=0.
  A4 the exact LP agrees with the grid on every arm of both tables, and in
     particular reports the duplicate as NOT strictly selectable while reporting
     every grid winner as selectable.
  A5 on 139's own 3-arm hull witness the LP reports compromise-C as not
     strictly selectable, with min t > 0 strictly (not merely = 0), which is
     stronger than any grid can say because a grid can only miss a cell.

CONTROLS, each of which must fire or the corresponding number is void:
  C1 a strictly dominated arm wins zero grid points AND is rejected by the LP.
  C2 the narrow-band arm wins at least once, so the grid resolves small cells.
  C3 the exact duplicate wins zero grid points under lower-index tie-breaking.
  C4 THE CASE THAT MUST FAIL. On an arm set built so that every arm is a vertex
     of the lower convex hull, the reported gap MUST be zero. If the gap
     detector reports a gap there, it is counting something other than
     unsupported efficiency and every gap number here is void.
  C5 THE SECOND CASE THAT MUST FAIL. On an arm set carrying a GENUINE
     unsupported efficient point and no duplicate, the gap MUST survive
     duplicate removal. If it does not, the decomposition in A3 cannot tell the
     two causes apart.
  C6 the LP must agree with the grid wherever the grid has a witness. A grid
     win is a certificate; the LP saying otherwise means the LP is wrong.
"""

from fractions import Fraction as F
from itertools import combinations, product

# ---------------------------------------------------------------- exact linalg


def solve_exact(rows, rhs):
    """Gaussian elimination over Fractions. Returns None if singular."""
    n = len(rows)
    a = [list(r) + [rhs[i]] for i, r in enumerate(rows)]
    for col in range(n):
        piv = None
        for r in range(col, n):
            if a[r][col] != 0:
                piv = r
                break
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


def min_regret_lp(arms, i):
    """Exact solution of  min t  s.t.  w.(c_i - c_j) <= t for all j != i,
    w >= 0, sum w = 1.

    Returns the optimal t as a Fraction. t < 0 means arm i is STRICTLY
    selectable by some weight in the simplex: there is a w making it the unique
    argmin. t == 0 means it can only tie. t > 0 means it is never even weakly
    optimal.

    Method: the optimum of a bounded LP is attained at a vertex of the feasible
    region, and a vertex in (w, t) space needs d + 1 linearly independent active
    constraints. The equality is always active, so enumerate every choice of d
    further active constraints from {w_k = 0} union {w.(c_i - c_j) = t}, solve
    exactly, keep the feasible ones, and take the minimum. No floating point
    anywhere, so a boundary case cannot be rounded across.
    """
    d = len(arms[0])
    diffs = [[F(arms[i][k]) - F(arms[j][k]) for k in range(d)] for j in range(len(arms)) if j != i]
    # variables: w_0..w_{d-1}, t
    nv = d + 1
    # constraint pool as (coeff vector over nv, rhs) meaning coeff . x = rhs
    pool = []
    for k in range(d):  # w_k = 0
        row = [F(0)] * nv
        row[k] = F(1)
        pool.append((row, F(0)))
    for dv in diffs:  # w.dv - t = 0
        row = [dv[k] for k in range(d)] + [F(-1)]
        pool.append((row, F(0)))
    eq = ([F(1)] * d + [F(0)], F(1))  # sum w = 1

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


def strictly_selectable(arms, i):
    t = min_regret_lp(arms, i)
    return t is not None and t < 0


# ---------------------------------------------------------------- basic pieces


def argmin_lin(arms, w):
    """Lower index wins ties, matching 139's strict less-than."""
    best, bestv = 0, None
    for i, c in enumerate(arms):
        v = sum(w[k] * c[k] for k in range(len(c)))
        if bestv is None or v < bestv:
            bestv, best = v, i
    return best


def dominated(arms, i):
    return any(
        j != i
        and all(arms[j][k] <= arms[i][k] for k in range(len(arms[i])))
        and any(arms[j][k] < arms[i][k] for k in range(len(arms[i])))
        for j in range(len(arms))
    )


def sweep_wins(arms, n):
    d = len(arms[0])
    assert d == 3
    wins = [0] * len(arms)
    for i in range(n + 1):
        for j in range(n - i + 1):
            k = n - i - j
            w = (F(i, n), F(j, n), F(k, n))
            wins[argmin_lin(arms, w)] += 1
    return wins


# ---------------------------------------------------------------- 139's tables

NAMES = [
    "scalar-widened",
    "packed",
    "packed-simd",
    "table-lookup",
    "narrow-band-compromise",
    "naive-loop",
    "scalar-widened [dup]",
]

T1 = [
    (F(10), F(40), F(32)),
    (F(26), F(90), F(13)),
    (F(14), F(240), F(13)),
    (F(8), F(60), F(512)),
    (F(31, 2), F(55), F(20)),
    (F(40), F(95), F(32)),
    (F(10), F(40), F(32)),
]

T2 = [
    (F(34), F(40), F(32)),
    (F(30), F(90), F(13)),
    (F(11), F(240), F(13)),
    (F(9), F(60), F(512)),
    (F(31, 2), F(55), F(20)),
    (F(40), F(95), F(32)),
    (F(34), F(40), F(32)),
]

failures = []


def report_table(label, arms, names, n=400):
    wins = sweep_wins(arms, n)
    winners = [i for i in range(len(arms)) if wins[i] > 0]
    pareto = [i for i in range(len(arms)) if not dominated(arms, i)]
    sel = [i for i in range(len(arms)) if strictly_selectable(arms, i)]
    print(f"{label}")
    print(
        f"  arms={len(arms)}  pareto-nondominated={len(pareto)}  "
        f"grid winners (cells)={len(winners)}  LP strictly selectable={len(sel)}"
    )
    for i, c in enumerate(arms):
        t = min_regret_lp(arms, i)
        print(
            f"    {names[i]:<24} cost=({c[0]},{c[1]},{c[2]})  wins={wins[i]:<7} "
            f"pareto={'Y' if i in pareto else 'n'} "
            f"LP-min-t={str(t):<10} {'SELECTABLE' if i in sel else 'not selectable'}"
        )
    # C1
    dom_wins = sum(wins[i] for i in range(len(arms)) if dominated(arms, i))
    dom_sel = [i for i in sel if dominated(arms, i)]
    print(f"  C1 dominated arms: grid wins {dom_wins}, LP selectable {len(dom_sel)}"
          f" -> {'PASS' if dom_wins == 0 and not dom_sel else 'FAIL'}")
    if dom_wins or dom_sel:
        failures.append(f"C1 {label}")
    # C2
    nb = names.index("narrow-band-compromise") if "narrow-band-compromise" in names else None
    if nb is not None:
        ok = wins[nb] > 0
        print(f"  C2 narrow-band arm wins {wins[nb]} -> {'PASS' if ok else 'FAIL'}")
        if not ok:
            failures.append(f"C2 {label}")
    # C3
    dup = next((i for i, nm in enumerate(names) if "[dup]" in nm), None)
    if dup is not None:
        ok = wins[dup] == 0
        print(f"  C3 duplicate arm wins {wins[dup]} -> {'PASS' if ok else 'FAIL'}")
        if not ok:
            failures.append(f"C3 {label}")
    # C6
    bad = [i for i in winners if i not in sel]
    print(f"  C6 every grid winner is LP-selectable: "
          f"{'PASS' if not bad else 'FAIL on ' + str(bad)}")
    if bad:
        failures.append(f"C6 {label}")
    return set(pareto), set(winners), set(sel)


print("=" * 78)
print("PART A. reproduce 139's p4 headline numbers on an independent instrument")
print("=" * 78)
p1, w1, s1 = report_table("TARGET 1 (wide multiplier, cheap wide loads)", T1, NAMES)
print()
p2, w2, s2 = report_table("TARGET 2 (no wide multiplier, expensive icache)", T2, NAMES)

print()
print("A1 139 reports 7 arms, 6 pareto, 5 cells on both targets.")
a1 = len(p1) == 6 and len(w1) == 5 and len(p2) == 6 and len(w2) == 5
print(f"   mine: t1 pareto={len(p1)} cells={len(w1)}, t2 pareto={len(p2)} cells={len(w2)}"
      f"  -> {'REPRODUCED' if a1 else 'DISAGREES'}")
if not a1:
    failures.append("A1")

# A2: the mapping difference, at 139's n=60
n2 = 60
moved = same = 0
for i in range(n2 + 1):
    for j in range(n2 - i + 1):
        k = n2 - i - j
        w = (F(i, n2), F(j, n2), F(k, n2))
        if NAMES[argmin_lin(T1, w)] == NAMES[argmin_lin(T2, w)]:
            same += 1
        else:
            moved += 1
tot = moved + same
print()
print(f"A2 139 reports 838 of 1891 (44.3%).")
print(f"   mine: {moved} of {tot} ({100.0 * moved / tot:.1f}%)"
      f"  -> {'REPRODUCED' if (moved, tot) == (838, 1891) else 'DISAGREES'}")
if (moved, tot) != (838, 1891):
    failures.append("A2")

print()
print("=" * 78)
print("PART B. decompose the six-versus-five gap: artifact or genuine?")
print("=" * 78)
for label, arms, pareto, winners in (("target 1", T1, p1, w1), ("target 2", T2, p2, w2)):
    gap = sorted(pareto - winners)
    print(f"  {label}: pareto-but-never-wins = {[NAMES[i] for i in gap]}")
    dupix = next(i for i, nm in enumerate(NAMES) if "[dup]" in nm)
    is_dup_only = gap == [dupix]
    print(f"    is that exactly the duplicate? {'YES' if is_dup_only else 'NO'}")
    # now delete the duplicate and recount
    keep = [i for i in range(len(arms)) if i != dupix]
    sub = [arms[i] for i in keep]
    subnames = [NAMES[i] for i in keep]
    subwins = sweep_wins(sub, 400)
    subwinners = {i for i in range(len(sub)) if subwins[i] > 0}
    subpareto = {i for i in range(len(sub)) if not dominated(sub, i)}
    print(f"    with the duplicate deleted: pareto={len(subpareto)} cells={len(subwinners)} "
          f"gap={len(subpareto - subwinners)}")
    if len(subpareto - subwinners) != 0:
        print(f"    A3 REFUTED for {label}: a gap survives duplicate removal")
        failures.append(f"A3 {label}")
    else:
        print(f"    A3 holds for {label}: the gap was the duplicate, a tie-break artifact")

print()
print("=" * 78)
print("PART C. controls on the gap detector itself")
print("=" * 78)

# C4: every arm a vertex of the lower hull -> gap must be zero
C4_ARMS = [(F(1), F(9), F(5)), (F(9), F(1), F(5)), (F(5), F(5), F(1))]
C4_NAMES = ["v-a", "v-b", "v-c"]
c4wins = sweep_wins(C4_ARMS, 400)
c4win = {i for i in range(3) if c4wins[i] > 0}
c4par = {i for i in range(3) if not dominated(C4_ARMS, i)}
print(f"  C4 all-vertices set: pareto={len(c4par)} cells={len(c4win)} "
      f"gap={len(c4par - c4win)} -> {'PASS' if c4par == c4win else 'FAIL'}")
if c4par != c4win:
    failures.append("C4")

# C5: a genuine unsupported efficient point, no duplicate anywhere
C5_ARMS = [(F(0), F(10), F(0)), (F(10), F(0), F(0)), (F(6), F(6), F(0))]
C5_NAMES = ["endpoint-A", "endpoint-B", "compromise-C"]
c5wins = sweep_wins(C5_ARMS, 400)
c5win = {i for i in range(3) if c5wins[i] > 0}
c5par = {i for i in range(3) if not dominated(C5_ARMS, i)}
c5gap = c5par - c5win
print(f"  C5 genuine-unsupported set: pareto={len(c5par)} cells={len(c5win)} "
      f"gap={[C5_NAMES[i] for i in sorted(c5gap)]} -> {'PASS' if c5gap else 'FAIL'}")
if not c5gap:
    failures.append("C5")

print()
print("=" * 78)
print("PART D. the Pareto claim decided exactly, not swept")
print("=" * 78)
print("  139's witness, with the exact LP rather than a 2001-point sweep:")
for i, nm in enumerate(C5_NAMES):
    t = min_regret_lp(C5_ARMS, i)
    print(f"    {nm:<14} pareto={'Y' if not dominated(C5_ARMS, i) else 'n'}  "
          f"min over the whole simplex of (its cost - the best other) = {t}  "
          f"{'SELECTABLE' if t < 0 else 'NEVER STRICTLY OPTIMAL'}")
tc = min_regret_lp(C5_ARMS, 2)
print()
print(f"  A5: min t for compromise-C is {tc}.")
print(f"      strictly positive? {'YES' if tc > 0 else 'NO'} "
      f"-> the arm is not merely unselected on a grid, it is beaten by a margin")
print(f"      of {tc} at EVERY point of the simplex, which no sweep can establish.")
if not (tc > 0):
    failures.append("A5")

# and 139's own control, re-run exactly
C5B = [(F(0), F(10), F(0)), (F(10), F(0), F(0)), (F(4), F(4), F(0))]
tb = min_regret_lp(C5B, 2)
print(f"  139's control, the same arm pulled inside the hull to (4,4): min t = {tb}, "
      f"{'SELECTABLE' if tb < 0 else 'still not selectable'}")
if not (tb < 0):
    failures.append("A5-control")

print()
print("=" * 78)
print(f"control failures: {len(failures)} {failures}")
print("=" * 78)
raise SystemExit(1 if failures else 0)
