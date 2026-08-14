"""
Cold-derivation probe, persona 126 (wronski). Follow-up to p1's prediction 8, which was
refuted by hand-picked examples (both attempts agreed). This does an exhaustive search
over small denominators instead of hand construction, checking two distinct questions:

(a) NUMERIC divergence: does the final stored value differ between round-then-clamp and
    clamp-then-round, when the clamp boundary IS grid-aligned at the rounding target width?
(b) VERDICT divergence: does whether-overflow-fires differ between the two orders, even
    when the final stored value coincides?
"""
from fractions import Fraction as Fr
import math

def round_mode(x: Fr, F: int, mode: str) -> Fr:
    scale = 1 << F
    scaled = x * scale
    if mode == 'floor':
        k = math.floor(scaled)
    elif mode == 'ceil':
        k = math.ceil(scaled)
    elif mode == 'trunc':
        k = math.trunc(scaled)
    elif mode == 'round_half_up':
        k = math.floor(scaled + Fr(1, 2))
    elif mode == 'round_half_even':
        lo = math.floor(scaled); hi = math.ceil(scaled)
        if lo == hi:
            k = lo
        else:
            d_lo = scaled - lo; d_hi = hi - scaled
            k = lo if d_lo < d_hi else (hi if d_hi < d_lo else (lo if lo % 2 == 0 else hi))
    return Fr(k, scale)

def sat(v: Fr, lo: Fr, hi: Fr):
    return lo if v < lo else (hi if v > hi else v)

MODES = ['floor', 'ceil', 'trunc', 'round_half_up', 'round_half_even']

print("SEARCH A: single target width F_final, grid-aligned boundary hi_final.")
print("round-then-clamp vs clamp-then-round, numeric value and verdict, over exhaustive")
print("sweep of exact values with denominator 24 (finer than any tested F<=3 grid).")
found_numeric = []
found_verdict = []
for F_final in (1, 2, 3):
    hi = Fr(3, 1)  # grid-aligned boundary at any F (integer)
    lo_b = Fr(0, 1)
    for mode in MODES:
        for n in range(-6, 6 * 24 + 6):
            exact = Fr(n, 24)
            # path A: round first, then clamp
            r_first = round_mode(exact, F_final, mode)
            a_overflow = (r_first > hi) or (r_first < lo_b)
            a_final = sat(r_first, lo_b, hi)
            # path B: clamp first (on exact value), then round
            b_overflow = (exact > hi) or (exact < lo_b)
            clamped = sat(exact, lo_b, hi)
            b_final = round_mode(clamped, F_final, mode)
            if a_final != b_final:
                found_numeric.append((F_final, mode, exact, a_final, b_final))
            elif a_overflow != b_overflow:
                found_verdict.append((F_final, mode, exact, a_overflow, b_overflow, a_final))
print(f"  numeric divergences found: {len(found_numeric)}")
for row in found_numeric[:5]:
    print("   ", row)
print(f"  verdict-only divergences found: {len(found_verdict)}")
for row in found_verdict[:5]:
    print("   ", row)

print()
print("SEARCH B: two widths, F_acc (accumulator) > F_final (target), boundary hi_final")
print("grid-aligned at F_final. round-at-acc-then-narrow-with-clamp vs clamp-exact-then-round.")
found_numeric2 = []
found_verdict2 = []
for F_acc, F_final in [(3, 1), (4, 1), (4, 2), (5, 1)]:
    hi = Fr(2, 1)
    lo_b = Fr(0, 1)
    for mode in MODES:
        for n in range(-4 * 24, 4 * 24):
            exact = Fr(n, 24)
            # path A: round to accumulator grid, clamp there (checked against hi/lo directly,
            # boundary is exact-comparable regardless of grid), then narrow to F_final.
            acc = round_mode(exact, F_acc, mode)
            a_overflow = (acc > hi) or (acc < lo_b)
            acc_clamped = sat(acc, lo_b, hi)
            a_final = round_mode(acc_clamped, F_final, mode)
            # path B: clamp the exact value directly, then round straight to F_final.
            b_overflow = (exact > hi) or (exact < lo_b)
            b_clamped = sat(exact, lo_b, hi)
            b_final = round_mode(b_clamped, F_final, mode)
            if a_final != b_final:
                found_numeric2.append((F_acc, F_final, mode, exact, a_final, b_final))
            elif a_overflow != b_overflow:
                found_verdict2.append((F_acc, F_final, mode, exact, a_overflow, b_overflow))
print(f"  numeric divergences found: {len(found_numeric2)}")
for row in found_numeric2[:8]:
    print("   ", row)
print(f"  verdict-only divergences found: {len(found_verdict2)}")
for row in found_verdict2[:8]:
    print("   ", row)
