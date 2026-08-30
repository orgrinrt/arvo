#!/usr/bin/env python3
"""
Probe 1: does a per-operation rounding law survive a chain, and what changes
the answer.

First cut of this probe used addition, and it was uninformative on
purpose left visible below (see main()): fixed-point addition of two
already-quantized values is exact whenever it does not overflow, because
the sum of two multiples of a quantum is itself an exact multiple of that
quantum. Addition chains introduce no per-step rounding at all, so "does a
naive per-step strategy preserve the chain" was never in question for
addition; only overflow/saturation is. That is itself a finding worth
keeping: which operation is being chained decides whether chain-level
machinery is needed at all, not chaining in the abstract.

Multiplication is where the interesting case lives: the exact product of
two F-fractional-bit fixed-point values needs 2F fractional bits to
represent exactly, so a naive strategy that rounds back down to F bits
after every multiply discards information every step; a strategy that
keeps a widened intermediate and rounds once at the end does not.

Hypothesis: for repeated fixed-point multiplication, the additional
error a strategy contributes (isolated from the one-time input
representation error, which both strategies pay identically) grows with
chain length n under naive round-after-every-step, and stays bounded by
one rounding step under widen-then-round-once. This is arithmetic, not a
claim about the Rust type system, and is checked directly since the
crate tree is empty.
"""
from fractions import Fraction as Fr

def round_to_quantum(x: Fr, f_bits: int) -> Fr:
    """Round x to the nearest multiple of 2^-f_bits, ties to even."""
    q = Fr(1, 2 ** f_bits)
    n = x / q
    floor_n = n.numerator // n.denominator
    rem = n - floor_n
    if rem < Fr(1, 2):
        k = floor_n
    elif rem > Fr(1, 2):
        k = floor_n + 1
    else:
        k = floor_n if floor_n % 2 == 0 else floor_n + 1
    return k * q

def naive_power(v_rounded: Fr, n: int, f_bits: int) -> Fr:
    """v^n via repeated multiply, rounding back to f_bits after every step."""
    acc = v_rounded
    for _ in range(n - 1):
        acc = round_to_quantum(acc * v_rounded, f_bits)
    return acc

def wide_power_then_round(v_rounded: Fr, n: int, f_bits: int) -> Fr:
    """v^n computed exactly (unbounded intermediate width), rounded once."""
    exact = v_rounded ** n
    return round_to_quantum(exact, f_bits)

def main():
    F_BITS = 8
    q = Fr(1, 2 ** F_BITS)
    # 0.6 is not exactly representable in binary at any finite width, so
    # v_rounded carries nonzero input error, identically for both
    # strategies. multiplying a value < 1 by itself repeatedly (decay,
    # compound scaling, EMA-style updates) is a real workload shape, not a
    # contrived one.
    v_exact = Fr(6, 10)
    v_rounded = round_to_quantum(v_exact, F_BITS)
    one_step_bound = q / 2

    print(f"F_BITS={F_BITS}  quantum={float(q):.6f}  "
          f"input v_rounded={float(v_rounded):.6f} "
          f"(exact 0.6, input repr. error={float(v_exact - v_rounded):.6e})")
    print(f"one rounding step bound = quantum/2 = {float(one_step_bound):.6e}")
    print()
    print(f"{'n':>6} | {'naive extra err':>18} | {'naive / one-step':>18} "
          f"| {'wide extra err':>16} | {'wide / one-step':>16}")

    ns = [1, 2, 4, 8, 16, 32, 64, 128, 256]
    naive_points = []
    for n in ns:
        # reference: v_rounded^n computed exactly then rounded ONCE.
        # this isolates what each strategy's rounding SCHEDULE contributes,
        # holding the input value (and its one-time representation error)
        # fixed across both strategies.
        reference = wide_power_then_round(v_rounded, n, F_BITS)
        naive_result = naive_power(v_rounded, n, F_BITS)
        naive_err = abs(naive_result - reference)
        wide_err = abs(reference - reference)  # 0 by construction
        naive_ratio = float(naive_err / one_step_bound)
        naive_points.append((n, naive_ratio))
        print(f"{n:>6} | {float(naive_err):>18.6e} | {naive_ratio:>18.3f} "
              f"| {float(wide_err):>16.6e} | {0.0:>16.3f}")

    print()
    print("growth check: naive_ratio[n] / naive_ratio[n/2] for successive "
          "doublings of chain length (a value that keeps climbing shows "
          "the naive strategy's error is not merely bounded noise; a "
          "value that plateaus shows it saturates at the container's "
          "range instead of growing without bound):")
    for i in range(1, len(naive_points)):
        n_prev, r_prev = naive_points[i - 1]
        n_cur, r_cur = naive_points[i]
        growth = (r_cur / r_prev) if r_prev > 1e-12 else float("nan")
        print(f"  n={n_prev:>4} -> n={n_cur:>4}: naive_ratio {r_prev:>8.3f} "
              f"-> {r_cur:>8.3f}  (factor {growth:.3f})")

    print()
    print("wide strategy: extra error is 0 for every n by construction. "
          "it pays exactly one rounding step, at the end, which is the "
          "same one-time cost the naive strategy pays on its very first "
          "step too; every additional step the naive strategy takes pays "
          "another rounding cost the wide strategy does not.")

if __name__ == "__main__":
    main()
