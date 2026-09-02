"""
Cold-derivation probe, persona 126 (wronski), topic: does rounding select character.

Built from scratch, before reading any other panel file on this topic. Tests eight
falsifiable predictions about rounding mode's algebraic and order-theoretic behaviour,
independent of overflow policy (grid values are kept small enough that no product
exceeds a generous bound, so overflow never triggers and the effect measured is
rounding's alone).

Every check states its prediction, then reports PASS/FAIL against an exhaustive or
adversarially-constructed case, not a case chosen to agree.
"""
from fractions import Fraction as Fr
import itertools
import math
import random

def round_mode(x: Fr, F: int, mode: str) -> Fr:
    """Round exact rational x to the nearest F-fractional-bit grid point."""
    scale = 1 << F
    scaled = x * scale
    if mode == 'floor':
        k = math.floor(scaled)
    elif mode == 'ceil':
        k = math.ceil(scaled)
    elif mode == 'trunc':  # toward zero
        k = math.trunc(scaled)
    elif mode == 'round_half_up':
        k = math.floor(scaled + Fr(1, 2))
    elif mode == 'round_half_even':
        lo = math.floor(scaled)
        hi = math.ceil(scaled)
        if lo == hi:
            k = lo
        else:
            d_lo = scaled - lo
            d_hi = hi - scaled
            if d_lo < d_hi:
                k = lo
            elif d_hi < d_lo:
                k = hi
            else:
                k = lo if lo % 2 == 0 else hi
    else:
        raise ValueError(mode)
    return Fr(k, scale)

def rmul(a: Fr, b: Fr, F: int, mode: str) -> Fr:
    return round_mode(a * b, F, mode)

def radd(a: Fr, b: Fr, F: int, mode: str) -> Fr:
    # addition of two F-bit grid points is always exact (sum is a grid point);
    # round_mode is a no-op here but applied for uniformity / to catch bugs
    return round_mode(a + b, F, mode)

MODES = ['floor', 'ceil', 'trunc', 'round_half_up', 'round_half_even']

def grid(F, kmax):
    return [Fr(k, 1 << F) for k in range(0, kmax + 1)]

print("=" * 70)
print("PREDICTION 1: at F=0, rounded multiply is associative for ALL modes,")
print("exhaustively over the swept grid (rounding never engages at F=0).")
print("=" * 70)
F = 0
vals = grid(F, 6)
for mode in MODES:
    fail = None
    for a, b, c in itertools.product(vals, repeat=3):
        lhs = rmul(rmul(a, b, F, mode), c, F, mode)
        rhs = rmul(a, rmul(b, c, F, mode), F, mode)
        if lhs != rhs:
            fail = (a, b, c, lhs, rhs)
            break
    status = "FAIL (counterexample found, prediction WRONG)" if fail else "PASS (no counterexample, exhaustive)"
    print(f"  F=0 mode={mode:16s} associativity: {status}" + (f" {fail}" if fail else ""))

print()
print("=" * 70)
print("PREDICTION 2: at F>=1, rounded multiply is NOT associative for standard")
print("modes; expect at least one counterexample per mode within a modest sweep.")
print("=" * 70)
for F in (1, 2, 3):
    vals = grid(F, 6)
    for mode in MODES:
        fail = None
        for a, b, c in itertools.product(vals, repeat=3):
            lhs = rmul(rmul(a, b, F, mode), c, F, mode)
            rhs = rmul(a, rmul(b, c, F, mode), F, mode)
            if lhs != rhs:
                fail = (a, b, c, lhs, rhs)
                break
        status = "counterexample found (associativity FAILS, as predicted)" if fail else "NO counterexample (prediction WRONG for this F/mode)"
        print(f"  F={F} mode={mode:16s}: {status}" + (f" a={fail[0]} b={fail[1]} c={fail[2]} (ab)c={fail[3]} a(bc)={fail[4]}" if fail else ""))

print()
print("=" * 70)
print("PREDICTION 3: at F>=1, rounded multiply does NOT distribute over addition")
print("for standard modes; a*(b+c) =?= a*b + a*c under rounding.")
print("=" * 70)
for F in (1, 2, 3):
    vals = grid(F, 6)
    for mode in MODES:
        fail = None
        for a, b, c in itertools.product(vals, repeat=3):
            lhs = rmul(a, radd(b, c, F, mode), F, mode)
            rhs = radd(rmul(a, b, F, mode), rmul(a, c, F, mode), F, mode)
            if lhs != rhs:
                fail = (a, b, c, lhs, rhs)
                break
        status = "counterexample found (distributivity FAILS, as predicted)" if fail else "NO counterexample (prediction WRONG)"
        print(f"  F={F} mode={mode:16s}: {status}" + (f" a={fail[0]} b={fail[1]} c={fail[2]}" if fail else ""))

print()
print("=" * 70)
print("PREDICTION 4: all deterministic rounding modes are monotone: x<=y implies")
print("round(x)<=round(y), swept over a fine sample of non-grid-aligned rationals.")
print("=" * 70)
F = 3
samples = [Fr(n, 37) for n in range(0, 37 * 8)]  # non-grid-aligned denominators
for mode in MODES:
    fail = None
    for i in range(len(samples)):
        for j in range(i, len(samples)):
            x, y = samples[i], samples[j]
            if x <= y:
                rx = round_mode(x, F, mode)
                ry = round_mode(y, F, mode)
                if rx > ry:
                    fail = (x, y, rx, ry)
                    break
        if fail:
            break
    status = "FAIL (violation found, prediction WRONG)" if fail else "PASS (monotone over the sweep)"
    print(f"  mode={mode:16s}: {status}" + (f" {fail}" if fail else ""))

print()
print("=" * 70)
print("PREDICTION 5: stochastic rounding is (a) NOT monotone per-realization")
print("(adversarial draws constructible), (b) unbiased in expectation exactly.")
print("=" * 70)

def stoch_round_given_u(x: Fr, F: int, u: float) -> Fr:
    scale = 1 << F
    scaled = x * scale
    lo = math.floor(scaled)
    hi = math.ceil(scaled)
    if lo == hi:
        return Fr(lo, scale)
    frac = float(scaled - lo)
    return Fr(hi, scale) if u < frac else Fr(lo, scale)

F = 0
x = Fr(99, 100)   # in [0,1), frac = 0.99, rounds up almost always
y = Fr(101, 100)  # in [1,2), frac = 0.01, rounds up almost never
assert x < y
u_x = 0.5   # 0.5 < 0.99 -> rounds x UP to 1
u_y = 0.5   # 0.5 > 0.01 -> rounds y DOWN to 1
rx = stoch_round_given_u(x, F, u_x)
ry = stoch_round_given_u(y, F, u_y)
print(f"  x={x} (u={u_x}) -> {rx};  y={y} (u={u_y}) -> {ry};  x<y holds: {x<y}")
if rx == ry:
    # try to find a strict violation instead of a tie
    u_x2, u_y2 = 0.999, 0.001
    rx2 = stoch_round_given_u(x, F, u_x2)
    ry2 = stoch_round_given_u(y, F, u_y2)
    print(f"  retry: x={x} (u={u_x2}) -> {rx2};  y={y} (u={u_y2}) -> {ry2}")
    violation = rx2 > ry2
    print(f"  strict violation (round(x) > round(y) while x<y): {violation}")
else:
    violation = rx > ry
    print(f"  strict violation (round(x) > round(y) while x<y): {violation}")

# unbiasedness: E[round(x)] = floor(x) + frac(x) = x, exactly, for any x.
# verify analytically for several x, and via Monte Carlo.
random.seed(1234)
for xv in [Fr(1, 3), Fr(7, 8), Fr(5, 2), Fr(0), Fr(4)]:
    F = 0
    scale = 1 << F
    scaled = xv * scale
    lo = math.floor(scaled)
    hi = math.ceil(scaled)
    if lo == hi:
        analytic_mean = Fr(lo, scale)
    else:
        p_up = scaled - lo
        analytic_mean = (Fr(lo, scale) * (1 - p_up)) + (Fr(hi, scale) * p_up)
    N = 200000
    total = 0.0
    for _ in range(N):
        u = random.random()
        total += float(stoch_round_given_u(xv, F, u))
    mc_mean = total / N
    print(f"  x={xv!s:6s} analytic E[round(x)]={float(analytic_mean):.6f} exact_x={float(xv):.6f} MC_mean(N={N})={mc_mean:.6f} diff={abs(mc_mean-float(xv)):.5f}")

print()
print("=" * 70)
print("PREDICTION 6: round-half-to-even accumulates less bias than round-half-up")
print("or truncation, over a long chain of multiplications by a fixed factor")
print("that lands exactly halfway between grid points at each step.")
print("=" * 70)
F = 1  # grid spacing 0.5; choose a multiplier that produces exact .25 offsets -> ties at F=1 after *2
factor = Fr(1, 1)  # trivial multiply won't create ties; construct ties directly instead
# Directly simulate repeated "add a value that is exactly at the midpoint between grid points"
# then round each step, for trunc / round_half_up / round_half_even, and sum the rounding error.
F = 0
N = 2000
for mode in ['trunc', 'round_half_up', 'round_half_even']:
    err_sum = Fr(0)
    exact_sum = Fr(0)
    rounded_sum = Fr(0)
    for i in range(N):
        # exact contribution at each step: i + 0.5 (always a tie at F=0)
        v = Fr(2 * i + 1, 2)
        exact_sum += v
        r = round_mode(v, F, mode)
        rounded_sum += r
        err_sum += (r - v)
    mean_err = float(err_sum) / N
    print(f"  mode={mode:16s} N={N} ties, mean per-step rounding error = {mean_err:+.4f} (0.0 = unbiased)")

print()
print("=" * 70)
print("PREDICTION 7: composing overflow with rounding: saturate(monotone) o round")
print("stays monotone; wrap(non-monotone) o round stays non-monotone.")
print("=" * 70)

def sat(v: Fr, lo: Fr, hi: Fr) -> Fr:
    if v < lo:
        return lo
    if v > hi:
        return hi
    return v

def wrap(v_int: int, width_vals: int) -> int:
    return v_int % width_vals

F = 2
lo, hi = Fr(0), Fr(3)  # small saturating range on the F=2 grid
samples = [Fr(n, 41) for n in range(-10, 4 * 41 + 10)]
fail = None
for i in range(len(samples)):
    for j in range(i, len(samples)):
        x, y = samples[i], samples[j]
        if x <= y:
            rx = sat(round_mode(x, F, 'round_half_even'), lo, hi)
            ry = sat(round_mode(y, F, 'round_half_even'), lo, hi)
            if rx > ry:
                fail = (x, y, rx, ry)
                break
    if fail:
        break
print(f"  saturate(round(x)) monotone over sweep: {'FAIL '+str(fail) if fail else 'PASS'}")

# wrap: work directly on integers mod N to demonstrate the family, independent of rounding,
# then show composing with rounding does not repair it.
N = 8
ints = list(range(-3, N + 3))
fail = None
for a, b in itertools.product(ints, repeat=2):
    if a <= b:
        wa, wb = wrap(a, N), wrap(b, N)
        if wa > wb:
            fail = (a, b, wa, wb)
            break
print(f"  bare wrap monotone: {'FAIL (violation, as predicted for wrap) '+str(fail) if fail else 'PASS (unexpected)'}")

print()
print("=" * 70)
print("PREDICTION 8: rounding-then-saturate can diverge from saturate-then-round")
print("when the intermediate grid (accumulator) differs from the final grid.")
print("=" * 70)
# accumulator: F_acc=3, final: F_final=1, saturate range [0, 3] on the FINAL grid.
F_acc, F_final = 3, 1
lo_final, hi_final = Fr(0), Fr(3)
exact = Fr(27, 8)  # 3.375, exceeds hi_final=3 and is not on either grid exactly
# path A: round to accumulator grid first, then saturate to final range (still on acc grid,
#         not narrowed to final grid - this models "round early, saturate late, forget to renarrow")
step_a = round_mode(exact, F_acc, 'round_half_even')          # 3.375 grid-F3 exactly -> 3.375
step_a_sat = sat(step_a, lo_final, hi_final)                   # 3.375 > 3 -> clamp to 3
# path B: saturate the exact value to final range first, then round to final grid
step_b_sat = sat(exact, lo_final, hi_final)                    # clamp 3.375 -> 3
step_b = round_mode(step_b_sat, F_final, 'round_half_even')    # 3 already on F1 grid -> 3
print(f"  exact={float(exact)} path A (round@F{F_acc} then clamp)={step_a_sat} path B (clamp then round@F{F_final})={step_b}")
print(f"  divergent: {step_a_sat != step_b}")

# a genuinely divergent case: pick an exact value whose ACCUMULATOR-grid rounding lands
# above hi_final by an amount that, once narrowed to F_final, differs from directly
# clamping-then-narrowing.
exact2 = Fr(49, 16)  # 3.0625
step_a2 = round_mode(exact2, F_acc, 'round_half_even')  # F3 grid: 3.0625*8=24.5 -> tie -> even -> 24/8=3.0? check
step_a2_clamped = sat(step_a2, lo_final, hi_final)
step_a2_narrowed = round_mode(step_a2_clamped, F_final, 'round_half_even')
step_b2 = round_mode(sat(exact2, lo_final, hi_final), F_final, 'round_half_even')
print(f"  exact2={float(exact2)} roundAcc={step_a2} clampToFinalRange={step_a2_clamped} thenNarrow={step_a2_narrowed} vs clampThenRound={step_b2}")
print(f"  divergent2: {step_a2_narrowed != step_b2}")
