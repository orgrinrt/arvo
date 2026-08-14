"""
Cold-derivation probe, persona 126 (wronski). Isolates whether p2's Search B numeric
divergences are actually about ROUNDING-OVERFLOW INTERACTION, or purely about DOUBLE
ROUNDING (chaining round-to-F_acc then round-to-F_final vs rounding directly to
F_final), by setting the clamp bounds so wide that no value can ever trigger them.
If the same divergences persist, they are not about overflow at all.
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

MODES = ['floor', 'ceil', 'trunc', 'round_half_up', 'round_half_even']
count_by_mode = {m: 0 for m in MODES}
first_hit = {}
for F_acc, F_final in [(3, 1), (4, 1), (4, 2), (5, 1), (2, 1)]:
    for mode in MODES:
        for n in range(-4 * 24, 4 * 24):
            exact = Fr(n, 24)
            double_rounded = round_mode(round_mode(exact, F_acc, mode), F_final, mode)
            direct = round_mode(exact, F_final, mode)
            if double_rounded != direct:
                count_by_mode[mode] += 1
                key = (F_acc, F_final, mode)
                if key not in first_hit:
                    first_hit[key] = (exact, double_rounded, direct)

print("Double rounding WITHOUT any clamp/overflow bound present at all:")
print(f"  total divergences per mode across all (F_acc,F_final) pairs swept: {count_by_mode}")
print("  first example per (F_acc,F_final,mode) that diverges:")
for k, v in list(first_hit.items())[:12]:
    print("   ", k, "exact=", v[0], "double_rounded=", v[1], "direct_single_round=", v[2])
