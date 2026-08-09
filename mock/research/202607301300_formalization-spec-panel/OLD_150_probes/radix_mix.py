"""The ingredient that breaks the join is a second radix, not bias and not the
second value map.

Three probes have now held the bias at zero and varied one thing at a time.

  radix two, quantum a power of two          0 failures over 5356 pairs
  radix two, both value maps                 0 failures over 6216 pairs
  several radices                            this probe

The reason to expect the split. Within one radix, refining the grid by one digit
multiplies the count by that radix at the same window shape, so a finer cover
always contains the coarser one and the least cover is unique. Refinement and
extension move together. Across radices that monotonicity has no reason to hold:
a cover on a finer grid supplied by a different radix can be strictly SHORTER
than the coarser cover it was meant to refine, and then neither contains the
other.
"""

from fractions import Fraction as Q
from itertools import combinations


def shapes(radices, pmax, emin, emax, fullrange=True, cap=2048):
    out = set()
    for r in radices:
        for p in range(0, pmax + 1):
            n = r ** p
            if n > cap:
                break
            for e in range(emin, emax + 1):
                qs = [Q(r) ** e]
                if fullrange and n > 1:
                    qs.append(Q(r) ** e / (n - 1))
                for q in qs:
                    out.add((q, 0, n - 1))
                    out.add((q, -(n - 1), n - 1))
                    out.add((q, -n, n - 1))
    return sorted(c for c in out if c[2] > c[1])


def inside(a, b):
    m = a[0] / b[0]
    if m.denominator != 1:
        return False
    m = m.numerator
    return b[1] <= a[1] * m and a[2] * m <= b[2]


def extremes(a, b, pool):
    ups = [c for c in pool if inside(a, c) and inside(b, c)]
    mubs = [c for c in ups if not any(d != c and inside(d, c) for d in ups)]
    lows = [c for c in pool if inside(c, a) and inside(c, b)]
    mlbs = [c for c in lows if not any(d != c and inside(c, d) for d in lows)]
    return mubs, mlbs


def run(radices, label):
    small = shapes(radices, 2, -2, 2)
    pool = shapes(radices, 5, -6, 6)
    jm = mm = 0
    sj = sm = 0
    for a, b in combinations(small, 2):
        mubs, mlbs = extremes(a, b, pool)
        if len(mubs) > 1:
            jm += 1
            if sj < 2:
                sj += 1
                print(f"  JOIN {a} v {b}")
                for x in mubs[:3]:
                    print(f"      q={x[0]} count={x[2]-x[1]+1} "
                          f"reach {x[2]*x[0]}")
        if len(mlbs) > 1:
            mm += 1
            if sm < 2:
                sm += 1
                print(f"  MEET {a} ^ {b}")
                for x in mlbs[:3]:
                    print(f"      q={x[0]} count={x[2]-x[1]+1} "
                          f"reach {x[2]*x[0]}")
    print(f"{label}\n   operands {len(small)}  pool {len(pool)}  "
          f"pairs {len(small)*(len(small)-1)//2}"
          f"   join non-unique {jm}   meet non-unique {mm}\n")


if __name__ == "__main__":
    run([2], "radix two only, bias zero")
    run([2, 3], "radices two and three, bias zero")
    run([2, 3, 5], "radices two, three and five, bias zero")
