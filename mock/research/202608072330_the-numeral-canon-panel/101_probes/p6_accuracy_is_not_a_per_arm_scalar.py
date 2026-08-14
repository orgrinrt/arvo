#!/usr/bin/env python3
"""Can an accuracy coordinate be a cell in the cost table the unit has settled
on?

The table is indexed by region and arm and holds a vector of numbers. Every
coordinate in it so far is a scalar property of one arm at one region: a time, a
size, a spread. Op's stated intent for the accuracy-first strategy is not that
shape:

    Precise on other hand is the one that sacrifices as much performance and
    efficiency as makes sense, to be the most precise possible answer, throwing
    out all cold or hot axis optimisations to be *accurate* and *precise*,
    especially within chains and ops, not only alone.   (INTENTS.md:125-127)

"Within chains and ops, not only alone" is a claim about compositions. This
probe asks whether a per-operation accuracy number can stand in for it, and the
answer is no in the strongest available form: the per-operation ranking and the
chain ranking of the same two arms **cross**, at a length that can be computed.

The pair, both of them ordinary fixed-point choices:

  A. A finer intermediate grid with truncation. Quantise to 2^-(F+2) by
     truncating toward zero. Per-step error is uniform on [-g_A, 0], so the mean
     absolute error is g_A/2 and it is BIASED.
  B. The declared grid with round-to-nearest. Quantise to 2^-F by rounding half
     to even. Per-step error is on [-g_B/2, +g_B/2] with mean zero, so the mean
     absolute error is g_B/4 and it is UNBIASED.

At one operation A is four times as accurate as B in mean absolute error. Down a
chain A's bias accumulates linearly and B's unbiased error accumulates as a
random walk, so B overtakes A at a length near `(g_B/2) / (g_A/2))^2` steps. A
per-arm scalar recording "A is more accurate" is right at every chain length
below the crossing and wrong at every length above it.

The control is the pair everyone expects: the same grid, truncation against
round-to-nearest. There B wins at every length and nothing crosses, which is
what makes the first result a property of the pair rather than of the harness.

The reference is exact rational arithmetic, so the error reported is the real
error and not a comparison against a third approximation.

This is a computation over a model. It is NOT a bench, it measures nothing about
any machine, and no number here prices anything.

Run:  python3 p6_accuracy_is_not_a_per_arm_scalar.py
"""

import random
from fractions import Fraction

SEED = 20260814
STREAMS = 4000
MAX_K = 96
F = 8  # declared fractional bits


def trunc(x, g):
    """Truncate toward zero onto the grid of step g."""
    n = x / g
    q = int(n) if n >= 0 else -int(-n)
    return Fraction(q) * g


def rne(x, g):
    """Round half to even onto the grid of step g."""
    n = x / g
    fl = n.numerator // n.denominator
    frac = n - fl
    if frac > Fraction(1, 2):
        q = fl + 1
    elif frac < Fraction(1, 2):
        q = fl
    else:
        q = fl if fl % 2 == 0 else fl + 1
    return Fraction(q) * g


def run(pair, rng):
    """Mean absolute error of each arm against exact, per chain length."""
    (ga, fa), (gb, fb) = pair
    err_a = [Fraction(0)] * (MAX_K + 1)
    err_b = [Fraction(0)] * (MAX_K + 1)
    for _ in range(STREAMS):
        exact = Fraction(0)
        acc_a = Fraction(0)
        acc_b = Fraction(0)
        for k in range(1, MAX_K + 1):
            # One term of the chain: a product of two values on the declared
            # grid, so the exact term needs 2F fractional bits and every arm
            # must quantise something.
            u = Fraction(rng.randrange(1, 1 << F), 1 << F)
            v = Fraction(rng.randrange(1, 1 << F), 1 << F)
            term = u * v
            exact += term
            acc_a = fa(acc_a + term, ga)
            acc_b = fb(acc_b + term, gb)
            err_a[k] += abs(acc_a - exact)
            err_b[k] += abs(acc_b - exact)
    return (
        [e / STREAMS for e in err_a],
        [e / STREAMS for e in err_b],
    )


def report(title, pair, names):
    rng = random.Random(SEED)
    a, b = run(pair, rng)
    ulp = Fraction(1, 1 << F)
    print("=" * 78)
    print(title)
    print("=" * 78)
    print(f"  errors in units of the declared ulp = 2^-{F}")
    print(f"    {'chain k':>8s} {names[0]:>14s} {names[1]:>14s}   winner")
    flip = None
    for k in [1, 2, 3, 4, 5, 6, 8, 12, 16, 24, 32, 48, 64, 96]:
        if k > MAX_K:
            continue
        wa, wb = a[k] / ulp, b[k] / ulp
        win = names[0] if wa < wb else names[1]
        print(f"    {k:8d} {float(wa):14.4f} {float(wb):14.4f}   {win}")
    # A CROSSING requires the first arm to lead at k = 1 and lose later. The
    # first version of this probe reported the control as "flips at k = 1",
    # which is the reversed pair being reversed from the start rather than a
    # crossing at all; that output is kept at
    # `p6_first_version_called_a_reversal_a_crossing.out`.
    if a[1] < b[1]:
        for k in range(1, MAX_K + 1):
            if a[k] > b[k]:
                flip = k
                break
        if flip:
            print(f"\n  CROSSING at chain length k = {flip}: "
                  f"{names[0]} is more accurate for k < {flip}, {names[1]} for k >= {flip}")
        else:
            print(f"\n  no crossing: {names[0]} leads at every k up to {MAX_K}")
    else:
        print(f"\n  no crossing: {names[1]} leads from k = 1 onward, so the "
              f"per-operation ranking and the chain ranking agree")
    print()
    return flip


def main():
    ga = Fraction(1, 1 << (F + 2))
    gb = Fraction(1, 1 << F)

    flip = report(
        "A. FINER GRID WITH TRUNCATION AGAINST DECLARED GRID WITH ROUND-TO-NEAREST",
        ((ga, trunc), (gb, rne)),
        ("fine+trunc", "coarse+rne"),
    )

    ctl = report(
        "B. CONTROL: SAME GRID, TRUNCATION AGAINST ROUND-TO-NEAREST",
        ((gb, trunc), (gb, rne)),
        ("coarse+trunc", "coarse+rne"),
    )

    print("=" * 78)
    print("C. WHAT THIS SAYS ABOUT THE TABLE SHAPE")
    print("=" * 78)
    print(f"  In A the per-operation ranking is reversed at k >= {flip}. A cost table")
    print("  cell holding one accuracy number per arm therefore names an arm that is")
    print("  wrong for every chain longer than that, and nothing in the cell says so.")
    print()
    if ctl is None:
        print("  In B nothing crosses, so the crossing in A is a property of the pair")
        print("  rather than of this probe: a bias that accumulates linearly against an")
        print("  unbiased error that accumulates as a random walk.")
    print()
    print("  The constructive reading: chain length is a REGION dimension, not a")
    print("  coordinate. Indexed that way the table holds, each cell is a scalar")
    print("  again, and the strategy that weighs accuracy selects a different arm at")
    print("  a different chain length, which is what op's intent says it should do.")
    print()
    print("  The corpus already half-does this. `warm-clamp-shared`'s key encoding is")
    print("  KEY = W * 10000 + NC * 1000 + LOG2A * 10 + OP, where LOG2A is the fold")
    print("  arity and OP selects between a chunked fold and 'an elementwise clamping")
    print("  chain of four steps' (warm-clamp-shared/src/lib.rs:83-89). Chain shape is")
    print("  in the region key there, and the cost vector stays a scalar per arm.")


if __name__ == "__main__":
    main()
