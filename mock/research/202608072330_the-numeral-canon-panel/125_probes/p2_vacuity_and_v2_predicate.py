#!/usr/bin/env python3
# P2: rounding is vacuous for {+,-,x} at F=0 and for {+,-} at any F (F4); division is the residue
# at F=0 (F5, the must-fail control); the v2 predicate decides multiplicative exactness both ways (F6).
# Representation: operands are scaled integers k (value = k * 2^-F). Exact op results computed as
# Fractions of subunits; rounding applied per mode. No overflow policy applied: the claim is about
# the quantisation step alone, range effects are the overflow axis's own.
from fractions import Fraction

def q_floor(x):  # x: Fraction in quantum units -> int grid index
    return x.numerator // x.denominator
def q_ceil(x): return -((-x).numerator // (-x).denominator)
def q_tz(x): return q_floor(x) if x >= 0 else q_ceil(x)
def q_half_up(x): return q_floor(x + Fraction(1, 2))
def q_half_even(x):
    k = q_floor(x); r = x - k
    if r < Fraction(1, 2): return k
    if r > Fraction(1, 2): return k + 1
    return k if k % 2 == 0 else k + 1

MODES = [("floor", q_floor), ("ceil", q_ceil), ("toward_zero", q_tz),
         ("half_up", q_half_up), ("half_even", q_half_even)]

def operand_range(w, signed):
    return range(-(1 << (w - 1)), 1 << (w - 1)) if signed else range(0, 1 << w)

# --- F4: vacuity of rounding for {+,-,x} at F=0, and {+,-} at F=2 ---
W = 5
for signed in (True, False):
    ks = list(operand_range(W, signed))
    # exact result IN QUANTUM UNITS, per op: add/sub of k-scaled operands is a+b quanta
    # exactly (the grid is a group); mul of k-scaled operands is a*b/2^F quanta. The first run
    # of this probe scaled add/sub by 2^F too (a units bug), manufacturing off-grid sums; that
    # defective run is preserved as p2_output_run1_defective_units.txt.
    for opname, op, F in [("add", lambda a, b, F: Fraction(a + b), 0),
                          ("sub", lambda a, b, F: Fraction(a - b), 0),
                          ("mul", lambda a, b, F: Fraction(a * b, 1 << F), 0),
                          ("add", lambda a, b, F: Fraction(a + b), 2),
                          ("sub", lambda a, b, F: Fraction(a - b), 2),
                          ("mul", lambda a, b, F: Fraction(a * b, 1 << F), 2)]:
        differing = 0
        for a in ks:
            for b in ks:
                exact = op(a, b, F)
                results = {f(exact) for _, f in MODES}
                if len(results) > 1: differing += 1
        tag = "signed" if signed else "unsigned"
        expect = "must be 0" if (opname != "mul" or F == 0) else "control: must be > 0"
        print(f"F4 vacuity: op={opname} F={F} W={W} {tag}: {differing} mode-differing cells "
              f"of {len(ks)**2} ({expect})")

# --- F5 control (must fail the all-equal shape): division at F=0, signed ---
ks = list(operand_range(W, True))
diff_div = 0; total_div = 0
for a in ks:
    for b in ks:
        if b == 0: continue
        total_div += 1
        exact = Fraction(a, b)
        if q_floor(exact) != q_tz(exact): diff_div += 1
print(f"F5 control: op=div F=0 W={W} signed: floor vs toward_zero differ on {diff_div} of "
      f"{total_div} pairs (must be > 0)")

# --- second control: multiplication at F=2 differs across modes ---
F = 2
diff_mul = 0
for a in ks:
    for b in ks:
        exact = Fraction(a * b, 1 << F)
        if len({f(exact) for _, f in MODES}) > 1: diff_mul += 1
print(f"control: op=mul F={F} W={W} signed: {diff_mul} mode-differing cells of {len(ks)**2} (must be > 0)")

# --- F6: v2 predicate decides multiplicative exactness, both directions ---
def v2(n):
    if n == 0: return 10**9  # stands for infinity
    c = 0
    while n % 2 == 0: n //= 2; c += 1
    return c

mismatch_a = mismatch_b = cells = 0
for F in (0, 2, 3):
    for signed in (True, False):
        W6 = 6
        ks6 = list(operand_range(W6, signed))
        for a in ks6:
            for b in ks6:
                cells += 1
                exact = Fraction(a * b, 1 << F)
                is_exact = exact.denominator == 1
                predicate = v2(a) + v2(b) >= F
                if predicate and not is_exact: mismatch_a += 1  # predicate claims exact, is not
                if is_exact and not predicate: mismatch_b += 1  # exact but predicate missed it
print(f"F6 v2-predicate: {cells} cells over F in {{0,2,3}}, W=6, both signednesses: "
      f"{mismatch_a} false-exact, {mismatch_b} missed-exact (both must be 0)")
