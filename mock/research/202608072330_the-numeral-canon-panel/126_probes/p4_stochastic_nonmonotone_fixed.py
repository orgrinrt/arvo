"""
Cold-derivation probe, persona 126 (wronski). Corrects p1's failed adversarial construction
for stochastic-rounding non-monotonicity (that attempt picked x,y in different grid
intervals, which cannot produce a violation). The correct construction needs x,y in the
SAME interval, with independent draws pushing x up and y down.
"""
from fractions import Fraction as Fr

def stoch_round_given_u(x: Fr, F: int, u):
    scale = 1 << F
    scaled = x * scale
    import math
    lo = math.floor(scaled); hi = math.ceil(scaled)
    if lo == hi:
        return Fr(lo, scale)
    frac = float(scaled - lo)
    return Fr(hi, scale) if u < frac else Fr(lo, scale)

F = 0
x, y = Fr(1, 10), Fr(9, 10)   # both in [0,1), x < y
assert x < y
u_x = 0.01   # < frac(x)=0.1  -> rounds x UP to 1
u_y = 0.95   # >= frac(y)=0.9 -> rounds y DOWN to 0
rx = stoch_round_given_u(x, F, u_x)
ry = stoch_round_given_u(y, F, u_y)
print(f"x={x} (frac=0.1, u={u_x}) -> {rx}")
print(f"y={y} (frac=0.9, u={u_y}) -> {ry}")
print(f"x < y: {x < y}")
print(f"round(x) > round(y): {rx > ry}   <-- monotonicity VIOLATED if True")
