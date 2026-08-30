#!/usr/bin/env python3
# P5: (a) two's complement bit-drop is floor, not toward_zero, on signed values (F9);
#     (b) stochastic rounding is exact in expectation and not pointwise monotone (F8).
# Stochastic is handled analytically with exact Fractions: no RNG, no sampling error.
from fractions import Fraction

# --- (a) bit-drop vs floor vs toward_zero, exhaustive at W=8, F in {1,3,5} ---
W = 8
for F in (1, 3, 5):
    for signed in (True, False):
        vals = range(-(1 << (W - 1)), 1 << (W - 1)) if signed else range(0, 1 << W)
        tag = "signed" if signed else "unsigned"
        bd_ne_floor = bd_ne_tz = 0
        for v in vals:
            bitdrop = v >> F                       # arithmetic shift on Python ints: sign-extending
            fl = v >> F                            # floor division by 2^F, same thing, kept separate
            fl_check = v // (1 << F)               # independent floor computation
            assert fl == fl_check, (v, F)
            tz = v // (1 << F) if v >= 0 else -((-v) // (1 << F))
            if bitdrop != fl: bd_ne_floor += 1
            if bitdrop != tz: bd_ne_tz += 1
        expect_tz = "must be > 0 (control)" if signed else "must be 0"
        print(f"bitdrop W={W} F={F} {tag}: bitdrop!=floor on {bd_ne_floor} of {len(list(vals))} "
              f"(must be 0); bitdrop!=toward_zero on {bd_ne_tz} ({expect_tz})")

# --- (b) stochastic: expectation exact; pointwise monotonicity fails ---
def sr_outcomes(x):  # x in quantum units -> [(value, probability)]
    k = x.numerator // x.denominator
    f = x - k
    if f == 0: return [(k, Fraction(1))]
    return [(k, 1 - f), (k + 1, f)]

E = 4
pts = [Fraction(u, 1 << E) for u in range(-2000, 2001)]
dev = sum(1 for x in pts if sum(v * p for v, p in sr_outcomes(x)) != x)
print(f"stochastic expectation: {dev} deviations from exactness over {len(pts)} points (must be 0)")

x, y = Fraction(1, 5), Fraction(4, 5)  # 0.2q < 0.8q
px = dict(sr_outcomes(x)); py = dict(sr_outcomes(y))
p_inv = px.get(1, Fraction(0)) * py.get(0, Fraction(0))
print(f"stochastic inversion pair: x={x} < y={y}, P[Q(x)=1 and Q(y)=0] = {p_inv} (must be > 0)")

# deterministic modes admit no inversion on the same sweep (their outcome is a function; check
# monotonicity of every mode again on this window as the negative control's counterpart)
def q_floor(z): return z.numerator // z.denominator
def q_ceil(z): return -((-z).numerator // (-z).denominator)
def q_tz(z): return q_floor(z) if z >= 0 else q_ceil(z)
def q_half_up(z): return q_floor(z + Fraction(1, 2))
def q_half_even(z):
    k = q_floor(z); r = z - k
    if r < Fraction(1, 2): return k
    if r > Fraction(1, 2): return k + 1
    return k if k % 2 == 0 else k + 1
for name, f in [("floor", q_floor), ("ceil", q_ceil), ("toward_zero", q_tz),
                ("half_up", q_half_up), ("half_even", q_half_even)]:
    inv = sum(1 for a, b in zip(pts, pts[1:]) if f(a) > f(b))
    print(f"deterministic {name}: {inv} adjacent inversions (must be 0)")
