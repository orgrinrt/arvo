#!/usr/bin/env python3
"""i2b: how much bigger is the general concept than the two named families,
counted over shapes that could actually be a format.

i2's Q5 counted every function from binades to grid exponents, including ones
no format could have, and reported 96.1 percent unnamed. That number is real
and it is loose, so it is kept in `i2.out` and this instrument tightens it.

Two conditions cut the space to formats:

  monotone      fexp is non-decreasing, so the grid never gets finer as the
                magnitude grows. Every format in i1b satisfies this.
  inhabited     fexp(e) <= e, so binade e holds at least one value. A binade
                whose step exceeds its own width is a hole, not a segment.

Both are read off the measured formats rather than assumed: the check runs
against i1b's own value sets first and reports any that violate them.
"""

import itertools
from fractions import Fraction

from i1_classify import flt, posit, fixed
from i2_lattice import fexp_of


def check_conditions_against_real_formats():
    print("First: do the two conditions hold on the formats i1b measured?")
    cases = [
        ("fixed U<3,3>", [v for v in fixed(3, 3) if v > 0]),
        ("float p=3 e=-2..3", [v for v in flt(3, -2, 3, subnormals=True) if v > 0]),
        ("posit<8,0>", [v for v in posit(8, 0) if v > 0]),
        ("posit<8,1>", [v for v in posit(8, 1) if v > 0]),
        ("posit<10,2>", [v for v in posit(10, 2) if v > 0]),
    ]
    for name, vals in cases:
        f = fexp_of(vals)
        es = sorted(f)
        mono = all(f[es[i + 1]] >= f[es[i]] for i in range(len(es) - 1))
        inhab = all(f[e] <= e for e in es)
        print(f"    {name:22s} monotone={mono}  inhabited={inhab}")
    print()


def count(window, depth):
    """Binades 0..window-1; fexp(e) in [e-depth, e]; monotone."""
    es = list(range(window))
    total = 0
    named = set()
    allshapes = set()
    ranges = [range(e - depth, e + 1) for e in es]
    for combo in itertools.product(*ranges):
        if any(combo[i + 1] < combo[i] for i in range(len(combo) - 1)):
            continue
        allshapes.add(combo)
        total += 1

    # fixed: constant
    for c in range(-depth, window):
        v = tuple(c for _ in es)
        if v in allshapes:
            named.add(v)
    # float: slope one
    for c in range(0, depth + 1):
        v = tuple(e - c for e in es)
        if v in allshapes:
            named.add(v)
    # float with gradual underflow: constant below a knee, slope one above
    for c in range(0, depth + 1):
        for knee in es:
            v = tuple(max(e - c, knee - c) for e in es)
            if v in allshapes:
                named.add(v)

    closure = set(named)
    changed = True
    while changed:
        changed = False
        for a, b in itertools.combinations(list(closure), 2):
            m = tuple(max(x, y) for x, y in zip(a, b))
            if m in allshapes and m not in closure:
                closure.add(m)
                changed = True

    # tapered: slope two or more somewhere
    tapered = sum(1 for v in allshapes
                  if any(v[i + 1] - v[i] >= 2 for i in range(len(v) - 1)))
    return total, len(named), len(closure), tapered


if __name__ == "__main__":
    check_conditions_against_real_formats()
    print("Shapes over a window of binades, counting only monotone inhabited ones:")
    print(f"{'window':>7} {'depth':>6} {'all':>10} {'named':>7} {'+closure':>9} "
          f"{'unnamed':>9} {'pct':>7} {'tapered':>8}")
    for window in (4, 6, 8):
        for depth in (3, 4, 5):
            t, n, c, tp = count(window, depth)
            print(f"{window:>7} {depth:>6} {t:>10} {n:>7} {c:>9} "
                  f"{t - c:>9} {100.0 * (t - c) / t:>6.1f}% {tp:>8}")
    print()
    print("`named` is the three shapes the design's two-instance exponent axis")
    print("reaches: a constant, a slope of one, and a slope of one with a knee.")
    print("`+closure` adds every pointwise maximum of those, which is what")
    print("closing the family under intersection buys. `tapered` counts the")
    print("shapes with a slope of two or more anywhere, none of which the")
    print("closure reaches.")
