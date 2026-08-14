#!/usr/bin/env python3
# P4: the adjunction laws for floor/ceil (T3) and staged-narrowing composition (T4).
# Controls that must fail: half_up and half_even under double rounding.
from fractions import Fraction

def q_floor(x): return x.numerator // x.denominator
def q_ceil(x): return -((-x).numerator // (-x).denominator)

# adjunction: g <= x  <=>  g <= floor(x)   and   x <= g  <=>  ceil(x) <= g
E = 4
pts = [Fraction(u, 1 << E) for u in range(-2000, 2001)]
grid = range(-140, 141)
bad_f = sum(1 for x in pts for g in grid if (g <= x) != (g <= q_floor(x)))
bad_c = sum(1 for x in pts for g in grid if (x <= g) != (q_ceil(x) <= g))
print(f"adjunction floor: {bad_f} failures over {len(pts)}x{len(list(grid))} (must be 0)")
print(f"adjunction ceil:  {bad_c} failures over {len(pts)}x{len(list(grid))} (must be 0)")

# staged narrowing: exact -> grid at F2=2 -> grid at F1=0, versus exact -> F1 directly.
# modes are applied on the respective grids: round x (in F1-quanta) needs scaling bookkeeping,
# done here entirely in Fractions of F1-quanta; the F2 grid is (1/4)Z in those units.
def on_grid(x, cell):  # round x to multiples of `cell` using mode f, returns a Fraction
    return None  # placeholder to keep names visible; specialised below

def make_stager(f):
    def stage(x):
        mid = Fraction(f(x * 4), 4)  # round to the F2 grid (quarters)
        return f(mid)                # then to the F1 grid (integers)
    return stage

def q_tz(x): return q_floor(x) if x >= 0 else q_ceil(x)
def q_half_up(x): return q_floor(x + Fraction(1, 2))
def q_half_even(x):
    k = q_floor(x); r = x - k
    if r < Fraction(1, 2): return k
    if r > Fraction(1, 2): return k + 1
    return k if k % 2 == 0 else k + 1

MODES = [("floor", q_floor), ("ceil", q_ceil), ("toward_zero", q_tz),
         ("half_up", q_half_up), ("half_even", q_half_even)]
fine = [Fraction(u, 1 << E) for u in range(-2000, 2001)]  # E=4 subquanta: finer than F2's grid
for name, f in MODES:
    stage = make_stager(f)
    mism = sum(1 for x in fine if stage(x) != f(x))
    expect = "must be 0" if name in ("floor", "ceil", "toward_zero") else "control: must be > 0"
    print(f"staged narrowing {name}: {mism} mismatches of {len(fine)} ({expect})")
