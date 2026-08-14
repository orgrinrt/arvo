#!/usr/bin/env python3
# r1 (128, replying to 127): reproduce the shared-threshold construction on my own instrument
# before conceding anything. Exact Fractions throughout; the expectation is computed as a measure
# of U-intervals, not asserted from the formula being checked.
#
# Predictions, stated before running:
#   1. Q_U monotone for every fixed U over a negation-closed window; zero violations.
#   2. E_U[Q_U(x)] = x exactly, computed as interval measure over U in [0,1).
#   3. saturate . Q_U monotone per realisation on an asymmetric signed range; zero violations.
#   4. Q_U is q-translation-equivariant per realisation, hence commutes with wrap MOD SPAN with
#      zero quotient mismatches; representative-level mismatches nonzero (it can round up out of
#      the top cell, like ceil/half_up in my p3).
#   5. Tie precision: at U = 1/2 under 127's own `f <= U` convention, Q_U rounds frac = 1/2 DOWN,
#      so it equals half_down at ties and differs from half_up there (correction to F127-2's
#      "exactly half_up"); on tie-free points they agree.
#   6. Control (the instrument distinguishes couplings): under INDEPENDENT per-element draws the
#      outcome pair Q(x)=ceil, Q(y)=floor for same-cell x<y has positive probability f_x*(1-f_y),
#      while under the shared threshold the same event has probability 0, computed exactly as the
#      measure of {U : f_x <= U... } intersections, not by sampling.
from fractions import Fraction

def q_floor(x): return x.numerator // x.denominator
def q_shared(x, U):
    k = q_floor(x); f = x - k
    return k if f <= U else k + 1
def q_half_up(x): return q_floor(x + Fraction(1, 2))
def q_half_down(x): return -q_floor(-x + Fraction(1, 2))  # ties toward -inf

E = 5
pts = [Fraction(u, 1 << E) for u in range(-7 * (1 << E), 7 * (1 << E) + 1)]  # [-7, 7] quanta
Us = [Fraction(1, 5), Fraction(1, 3), Fraction(1, 2), Fraction(17, 32), Fraction(4, 5),
      Fraction(1, 97), Fraction(96, 97)]

print("--- 1. per-realisation monotonicity, window [-7q, 7q] closed under negation ---")
for U in Us:
    v = [q_shared(x, U) for x in pts]
    bad = sum(1 for i in range(len(v) - 1) if v[i] > v[i + 1])
    print(f"U={U}: {bad} violations of {len(v)-1} adjacent pairs (must be 0)")

print("--- 2. exact expectation as U-interval measure ---")
# {U in [0,1): Q_U(x) = floor} = [f, 1), measure 1-f; ceil on [0, f), measure f.
for x in [Fraction(-13, 32), Fraction(7, 32), Fraction(1, 2), Fraction(3), Fraction(-27, 8)]:
    k = q_floor(x); f = x - k
    m_floor = Fraction(1) - f   # measure of [f, 1)
    m_ceil = f                  # measure of [0, f)
    e = k * m_floor + (k + 1) * m_ceil
    print(f"x={x}: E={e}, exact={'YES' if e == x else 'NO'} (must be YES)")

print("--- 3. saturate . Q_U on asymmetric signed range [-8, 7] ---")
m, M = -8, 7
sat_pts = [Fraction(u, 1 << E) for u in range((m - 4) * (1 << E), (M + 4) * (1 << E) + 1)]
for U in Us:
    v = [max(m, min(M, q_shared(x, U))) for x in sat_pts]
    bad = sum(1 for i in range(len(v) - 1) if v[i] > v[i + 1])
    print(f"U={U}: {bad} violations of {len(v)-1} (must be 0)")

print("--- 4. wrap commutation per realisation, mod span and representative level ---")
for W in (3, 4):
    span = 1 << W
    for signed in (True, False):
        lo_r = -(span // 2) if signed else 0
        def wrap_exact(x): return x - span * ((x - lo_r) // span)
        def wrap_grid(k): return (k - lo_r) % span + lo_r
        window = [Fraction(u, 1 << E)
                  for u in range((lo_r - 2 * span) * (1 << E), (lo_r + 3 * span) * (1 << E) + 1)]
        tag = "signed" if signed else "unsigned"
        for U in [Fraction(1, 3), Fraction(96, 97)]:
            qm = sum(1 for x in window
                     if (wrap_grid(q_shared(x, U)) - q_shared(wrap_exact(x), U)) % span != 0)
            rm = sum(1 for x in window if wrap_grid(q_shared(x, U)) != q_shared(wrap_exact(x), U))
            print(f"W={W} {tag} U={U}: {qm} quotient mismatches (must be 0); "
                  f"{rm} representative mismatches (expected > 0 at top cells)")

print("--- 5. the midpoint of the family is half_down at ties, not half_up ---")
ties = [Fraction(2 * n + 1, 2) for n in range(-4, 4)]  # frac exactly 1/2
U = Fraction(1, 2)
eq_hd = sum(1 for x in ties if q_shared(x, U) == q_half_down(x))
eq_hu = sum(1 for x in ties if q_shared(x, U) == q_half_up(x))
nontie = [x for x in pts if (x - q_floor(x)) != Fraction(1, 2)]
agree_hu_nontie = sum(1 for x in nontie if q_shared(x, U) == q_half_up(x))
print(f"at {len(ties)} tie points: Q_(1/2) == half_down on {eq_hd} (must be {len(ties)}); "
      f"== half_up on {eq_hu} (must be 0)")
print(f"on {len(nontie)} non-tie points: Q_(1/2) == half_up on {agree_hu_nontie} "
      f"(must be {len(nontie)})")

print("--- 6. control: the couplings differ, computed exactly ---")
x, y = Fraction(1, 5), Fraction(4, 5)  # same cell, f_x=1/5 < f_y=4/5
fx, fy = x - q_floor(x), y - q_floor(y)
p_inv_indep = fx * (1 - fy)  # product of marginals: presumes independence, as my P5 did
# shared threshold: inversion needs U < fx (so Q(x)=ceil) AND fy <= U (so Q(y)=floor):
# U in [0, fx) intersect [fy, 1) = empty since fx < fy
lo_i, hi_i = max(Fraction(0), fy), min(fx, Fraction(1))
p_inv_shared = max(Fraction(0), hi_i - lo_i)
print(f"P[inversion] independent coupling: {p_inv_indep} (must be > 0)")
print(f"P[inversion] shared threshold:     {p_inv_shared} (must be 0)")
