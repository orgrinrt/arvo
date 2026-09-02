"""Which ingredient makes the join fail: bias, radix, or the second value map?

minimal_ubs.py found zero pairs with more than one minimal upper bound over 5356
unbiased radix-two pairs, where the quantum is a power of two. 148 reports 81
decided join failures in what it calls the unbiased radix-two slice. Both cannot
describe the same family, so the families differ, and this probe isolates where.

148 records two adjustment constructors, `Unit` and `FullRange` (148:144), and a
five-point progression at quantum one quarter realised at radix five (148:351).
A quantum of one quarter is not a power of five, so under at least one of the two
maps the quantum is not a power of the radix. The map that produces exactly that
behaviour spreads r^p points across a unit interval, giving quantum 1/(r^p - 1)
times a power of the radix.

At radix two that yields quanta 2^e, 2^e/3, 2^e/7, 2^e/15 and so on. So a family
described as radix two still carries quanta with odd denominators, and the count
and the quantum stop being independent. This probe adds that map and nothing
else, holding the radix at two and the bias at zero.
"""

from fractions import Fraction as Q
from itertools import combinations


def shapes(pmax, emin, emax, fullrange):
    out = set()
    for p in range(0, pmax + 1):
        n = 2 ** p
        for e in range(emin, emax + 1):
            qs = [Q(2) ** e]
            if fullrange and n > 1:
                qs.append(Q(2) ** e / (n - 1))
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


def minimal_ubs(a, b, pool):
    ups = [c for c in pool if inside(a, c) and inside(b, c)]
    return [c for c in ups if not any(d != c and inside(d, c) for d in ups)]


def run(fullrange, label):
    small = shapes(3, -3, 3, fullrange)
    pool = shapes(7, -7, 7, fullrange)          # strictly wider search region
    multi = 0
    shown = 0
    for a, b in combinations(small, 2):
        m = minimal_ubs(a, b, pool)
        if len(m) > 1:
            multi += 1
            if shown < 3:
                shown += 1
                print(f"  {a} v {b}")
                for x in m:
                    span = f"[{x[1]*x[0]}, {x[2]*x[0]}]"
                    print(f"      q={x[0]}  count={x[2]-x[1]+1}  span {span}")
    print(f"{label}: pairs {len(small)*(len(small)-1)//2}"
          f"   operands {len(small)}  search pool {len(pool)}"
          f"   MORE THAN ONE MINIMAL UPPER BOUND: {multi}\n")


if __name__ == "__main__":
    run(False, "radix two, one value map, bias zero")
    run(True, "radix two, both value maps, bias zero")
