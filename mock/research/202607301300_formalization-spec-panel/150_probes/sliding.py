"""Does 148's sliding mechanism hold, and is it the only thing that breaks joins?

148's mechanism, restated: a covering numeral's window may take only the sizes
its radix and precision permit, so where no permitted size matches the convex
hull of two value sets, the smallest cover can slide, and no placement is least.

Four things are checked separately.

  (a) Is the mechanism valid as mathematics? Built directly, from windows with a
      power-of-two size ladder and a free integer placement.
  (b) Does anchoring kill it? A numeral that always contains zero cannot slide.
  (c) Does anything slide in the anchored fixed-point family? Joins are computed
      in a box strictly larger than the one the pairs are drawn from, so that
      enumeration boundary is not mistaken for a real failure.
  (d) The fixed-versus-float join failure is between two covers that both
      contain zero. Nothing slides there, so it is a second mechanism that 148
      does not name.
"""

from fractions import Fraction as Q
from itertools import combinations
from poset import fixed, flt


def window(lo, size, step=1):
    """A grid of `size` points, step apart, with the lowest point at lo."""
    return frozenset(Q(lo) + Q(k) * Q(step) for k in range(size))


def mubs(a, b, universe):
    un = a | b
    ups = [c for c in universe if un <= c]
    return [c for c in ups if not any(d < c for d in ups)]


def part_a():
    print("=== (a) the mechanism built directly, free placement, sizes 2^n ===")
    uni = list({window(lo, 2 ** n) for n in range(0, 5) for lo in range(-6, 7)})
    a, b = window(0, 1), window(2, 1)
    m = mubs(a, b, uni)
    print(f"  cover of {{0}} and {{2}}: {len(m)} minimal upper bounds")
    for w in sorted(m, key=lambda s: sorted(s)):
        print(f"    {[str(x) for x in sorted(w)]}")
    print("  -> VALID. The hull spans 3 points, the ladder offers only 4, "
          "and the size-4 window slides.")


def part_b():
    print("\n=== (b) the same construction with placement anchored to zero ===")
    anchored = [window(0, 2 ** n) for n in range(0, 6)]
    m = mubs(window(0, 1), window(2, 1), anchored)
    print(f"  {len(m)} minimal upper bound(s): "
          f"{[[str(x) for x in sorted(w)] for w in m]}")
    print("  -> anchoring removes the freedom the mechanism needs.")


def part_c():
    print("\n=== (c) does anything slide in the anchored fixed-point family? ===")
    small = list({fixed(i, f, s) for i in range(0, 5) for f in range(0, 5)
                  for s in (False, True) if i + f > 0} | {frozenset({Q(0)})})
    big = list({fixed(i, f, s) for i in range(0, 8) for f in range(0, 8)
                for s in (False, True) if i + f > 0} | {frozenset({Q(0)})})
    bad = 0
    for a, b in combinations(small, 2):
        if len(mubs(a, b, big)) != 1:
            bad += 1
    print(f"  pairs drawn from the I,F<=4 box, joins computed in the I,F<=7 box")
    print(f"  pairs: {len(small)*(len(small)-1)//2}   non-unique join: {bad}")
    print("  -> nothing slides. Every anchored pair has one least cover.")


def part_d():
    print("\n=== (d) the fixed-versus-float failure is not sliding ===")
    f_lo, f_hi = 0, 3
    F = flt(2, -1, 1, signed=False)          # p=2, exponents -1..1, subnormals
    A = fixed(1, 0, False)                   # {0, 1}
    B = fixed(0, 2, False)                   # {0, 1/4, 1/2, 3/4}
    uni = list({fixed(i, f, False) for i in range(0, 5) for f in range(0, 5)
                if i + f > 0} | {F, frozenset({Q(0)})})
    m = mubs(A, B, uni)
    print(f"  float set: {[str(x) for x in sorted(F)]}")
    print(f"  A = {[str(x) for x in sorted(A)]}")
    print(f"  B = {[str(x) for x in sorted(B)]}")
    print(f"  A join B: {len(m)} minimal upper bounds")
    for w in m:
        contains_zero = Q(0) in w
        print(f"    contains 0: {contains_zero}   {[str(x) for x in sorted(w)]}")
    print("  -> both covers contain zero, so no window moved. They are "
          "incomparable in SHAPE, uniform against tapered.")


if __name__ == "__main__":
    part_a()
    part_b()
    part_c()
    part_d()
