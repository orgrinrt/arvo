#!/usr/bin/env python3
# w1: does the "stochastic vs deterministic" fork that 125 (F8) and 126 (Finding 4) both claim,
# independently, as the one place rounding echoes the overflow axis's genuine either/or trade, survive
# a construction that decouples the two properties it says are traded?
#
# Prediction, stated before running: a SHARED-THRESHOLD stochastic rounding, Q_U(x) = floor(x) if
# frac(x) <= U else ceil(x), for a single U drawn ONCE per evaluation pass and held fixed across every
# x evaluated in that pass, is (a) monotone nondecreasing on ALL of Q for every fixed U (not merely
# "per draw" in the weak per-element sense both files tested, but globally, the same way floor and
# ceil are), and (b) exactly unbiased in expectation over U, E_U[Q_U(x)] = x, for every x. If both hold,
# the claimed fork (monotone XOR unbiased) is false: this construction is both at once, so "stochastic"
# is not one thing with one tradeoff, and the independent per-element draw both files tested is one
# member of a family, not the only shape available.
#
# This is NOT a claim that T1 (no deterministic map is exact off-grid) is wrong. Q_U for a FIXED U is
# a deterministic map and is not exact off-grid (checked below as a control: it fails additivity same
# as floor/ceil/etc do). The unbiasedness is a property of the DISTRIBUTION over U, achieved without
# ever needing a single realisation to be exact. This is the same shape 126's own dithering aside
# gestures at (a fixed per-position dither pattern) but sharper: a single global U, not position-keyed,
# gives monotonicity over the WHOLE domain per draw, not merely "as a function of value for one fixed
# position while still breaking it across positions" as 126 states for its weaker construction.

from fractions import Fraction
import random

def q_floor(x):
    return x.numerator // x.denominator

def q_ceil(x):
    return -((-x).numerator // (-x).denominator)

def q_shared(x, U):
    # U is a Fraction in [0, 1). frac(x) is the fractional part in [0, 1) with floor semantics.
    k = q_floor(x)
    f = x - k
    return k if f <= U else k + 1

print("=== Part 1: monotonicity, exhaustive over a fine grid, for many fixed U ===")
# exact rationals at subquantum resolution E=6 (finer than the rounding-mode probes used elsewhere,
# since U itself needs fine resolution to land between grid points at varied offsets)
E = 6
lo, hi = -6, 6
pts = [Fraction(u, 1 << E) for u in range(lo * (1 << E), hi * (1 << E) + 1)]

Us = [Fraction(1, 7), Fraction(1, 3), Fraction(1, 2), Fraction(2, 3), Fraction(6, 7),
      Fraction(1, 1000), Fraction(999, 1000)]
mono_violations_total = 0
for U in Us:
    vals = [q_shared(x, U) for x in pts]
    violations = sum(1 for i in range(len(vals) - 1) if vals[i] > vals[i + 1])
    mono_violations_total += violations
    print(f"  U={U}: {violations} monotonicity violations over {len(pts)} consecutive pairs "
          f"(must be 0)")

print()
print("=== Part 1 control: independent per-element draw (both files' construction) DOES violate ===")
random.seed(12345)
def q_independent(x):
    k = q_floor(x)
    f = x - k
    u = Fraction(random.randrange(0, 1 << 20), 1 << 20)
    return k if f <= u else k + 1

trials = 20000
found = 0
for _ in range(trials):
    x = Fraction(1, 10)
    y = Fraction(9, 10)
    qx, qy = q_independent(x), q_independent(y)
    if x < y and qx > qy:
        found += 1
print(f"  independent draws: {found} of {trials} trials show x<y with Q(x)>Q(y) "
      f"(control: must be > 0, reproduces 125 P5 / 126 Finding 4's construction)")

print()
print("=== Part 2: unbiasedness of the shared-threshold construction, exact and Monte Carlo ===")
# Exact: E_U[Q_U(x)] over U ~ Uniform[0,1) for fixed x with frac(x) = f.
# Q_U(x) = k for U >= f (measure 1-f), k+1 for U < f (measure f). E = k(1-f) + (k+1)f = k+f = x.
# Verify by exact symbolic integration over a partition of U by the threshold f, for several x.
test_xs = [Fraction(1, 4), Fraction(3, 8), Fraction(-5, 3), Fraction(0), Fraction(7, 2)]
for x in test_xs:
    k = q_floor(x)
    f = x - k
    expectation = k * (1 - f) + (k + 1) * f
    print(f"  x={x} (k={k}, f={f}): exact E_U[Q_U(x)] = {expectation}, matches x: {expectation == x}")

# Monte Carlo cross-check, independent instrument (uniform floats, not the exact partition above)
random.seed(999)
N = 500000
for x in test_xs:
    xf = float(x)
    acc = 0.0
    for _ in range(N):
        U = random.random()
        f = xf - (xf // 1)
        k = xf // 1
        acc += k if f <= U else k + 1
    mean = acc / N
    print(f"  x={x}: Monte Carlo mean over {N} draws = {mean:.5f}, exact target = {xf:.5f}, "
          f"diff = {abs(mean - xf):.5f} (must be small, sampling noise only)")

print()
print("=== Part 3 control: Q_U for a fixed U is not additive off-grid (T1 still holds; unbiasedness ===")
print("=== is a property of the distribution over U, never of a single realisation)             ===")
U = Fraction(1, 3)
viol = 0
total = 0
for a in pts[:200]:
    for b in pts[:5]:
        total += 1
        lhs = q_shared(a + b, U)
        rhs = q_shared(a, U) + q_shared(b, U)
        if lhs != rhs:
            viol += 1
print(f"  U={U}: additivity violations {viol} of {total} sampled pairs "
      f"(control: must be > 0, confirms Q_U is an ordinary deterministic rounding mode, "
      f"consistent with T1/Finding 2, not a loophole in it)")

print()
print("=== Part 4: does composing with saturate eject Q_U from the monotone family? ===")
print("=== (125 F8's second half claims stochastic-composed-with-saturate is ejected  ===")
print("=== from the monotone family; check whether that holds for THIS construction) ===")
def sat_grid(k, m, M):
    return max(m, min(M, k))

m, M = -8, 7  # asymmetric two's-complement-style bounds, W=4 signed, matching 125/126's own sweeps
sat_pts = [Fraction(u, 1 << E) for u in range((-12) * (1 << E), 12 * (1 << E) + 1)]
sat_viol_total = 0
for U in Us:
    vals = [sat_grid(q_shared(x, U), m, M) for x in sat_pts]
    viol = sum(1 for i in range(len(vals) - 1) if vals[i] > vals[i + 1])
    sat_viol_total += viol
    print(f"  U={U}: saturate(Q_U(.)) monotonicity violations = {viol} of {len(vals) - 1} (must be 0)")
print(f"  total: {sat_viol_total} (predicted 0; every realisation of saturate(Q_U) stays monotone,")
print("  because Q_U is monotone for every fixed U and composition of monotone maps is monotone,")
print("  the same T6/Finding-6 composition argument both files already use for the other five modes)")

print()
print("=== Summary ===")
print(f"  total monotonicity violations across all tested U: {mono_violations_total} (predicted 0)")
print("  If 0 monotonicity violations and unbiasedness confirmed both exact and by Monte Carlo,")
print("  the shared-threshold construction is BOTH monotone (every realisation) AND unbiased")
print("  (over the draw), which the claimed fork (125 F8, 126 Finding 4) says cannot happen.")
