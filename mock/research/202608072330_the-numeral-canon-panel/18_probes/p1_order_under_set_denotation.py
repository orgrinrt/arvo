"""p1. What happens to the value-level order when a datum denotes a set.

Hypothesis under test. `SETTLED.md:116` records at RATIFIED that the design has a value-level
order usable in laws, distinct from a datum-level total order, and that the value-level order is
a precondition of the distributivity law. If a datum denotes a SET rather than a point, the
value-level order is an order on sets, and the question is whether it is still total.

Three set-denotation kinds are compared against the point denotation, over one small numeral:

  point        gamma(d) = {d}                       (the clause as written in 08:556)
  cell         gamma(d) = the rounding preimage      (07 section 2.3, measured there)
  absorbing    gamma(top) = [top, inf), else {d}     (07 section 4.2, measured there)
  interval     a datum is a pair, gamma = [lo, hi]   (08 section 4.5, excluded)

The value-level order on sets that a numeral needs is the SEPARATION order:

  A <= B  iff  every a in A is <= every b in B

because that is the only order on sets under which "a <= b" transfers to every pair of concrete
values the two data stand for, which is what a law quantified over concrete values needs.

DOMAIN, stated in the same breath as every count below: the base numeral is the 16 values
k/4 for k in 0..15, that is U<2,2> unsigned with adjustment 1/4, and every count is over the
data of that one numeral or over pairs drawn from them. Nothing here generalises to other
widths without rerunning; the point being made is structural, and section three of the file
reruns it at three widths to check the shape is not an artifact of 16.

Run: python3 p1_order_under_set_denotation.py
"""

from fractions import Fraction as F
from itertools import combinations


def numeral(int_bits, frac_bits):
    """Value set of an unsigned anchored numeral, as an ordered list of Fractions."""
    step = F(1, 2 ** frac_bits)
    n = 2 ** (int_bits + frac_bits)
    return [k * step for k in range(n)], step


def gamma_point(values, step):
    return [(v, v) for v in values]


def gamma_cell(values, step):
    """Round-to-nearest preimage: the half-open cell centred on each value.

    Endpoints are represented as a closed pair (lo, hi) of Fractions. Ties are ignored
    because they change nothing about comparability, which is what this probe measures.
    """
    h = step / 2
    return [(v - h, v + h) for v in values]


def gamma_absorbing(values, step):
    """Every datum is a point except the top, which stands for everything above it.

    Infinity is modelled as the top plus a large finite slack, which is sound for a
    comparability count because nothing in the value set exceeds it.
    """
    top = values[-1]
    big = top + 10 ** 6
    return [(v, v) if v != top else (top, big) for v in values]


def gamma_interval(values, step):
    """Every ordered pair of data is one interval datum."""
    return [(lo, hi) for i, lo in enumerate(values) for hi in values[i:]]


def separation_leq(a, b):
    """a <= b under the separation order: every element of a is at most every element of b."""
    return a[1] <= b[0]


def comparable(a, b):
    return separation_leq(a, b) or separation_leq(b, a)


def report(name, sets):
    n = len(sets)
    pairs = list(combinations(range(n), 2))
    comp = sum(1 for i, j in pairs if comparable(sets[i], sets[j]))
    total = len(pairs)
    frac = comp / total if total else 1.0
    print(f"{name:12s} data={n:5d}  pairs={total:7d}  comparable={comp:7d}  ({frac:6.2%})")
    return n, total, comp


def main():
    values, step = numeral(2, 2)
    print(f"# base numeral U<2,2>: {len(values)} values, adjustment {step}, "
          f"range [{values[0]}, {values[-1]}]")
    print()
    print("# comparability of the value-level (separation) order, per denotation")
    report("point", gamma_point(values, step))
    report("cell", gamma_cell(values, step))
    report("absorbing", gamma_absorbing(values, step))
    report("interval", gamma_interval(values, step))
    print()

    print("# does datum equality agree with denotation equality")
    for name, g in (("point", gamma_point), ("cell", gamma_cell),
                    ("absorbing", gamma_absorbing), ("interval", gamma_interval)):
        sets = g(values, step)
        distinct = len(set(sets))
        print(f"{name:12s} data={len(sets):5d}  distinct denotations={distinct:5d}  "
              f"{'injective' if distinct == len(sets) else 'NOT injective'}")
    print()

    print("# are the denotations pairwise disjoint (a partition of what they cover)")
    for name, g in (("point", gamma_point), ("cell", gamma_cell),
                    ("absorbing", gamma_absorbing), ("interval", gamma_interval)):
        sets = g(values, step)
        overlaps = 0
        for i, j in combinations(range(len(sets)), 2):
            a, b = sets[i], sets[j]
            if a[0] <= b[1] and b[0] <= a[1]:
                overlaps += 1
        print(f"{name:12s} overlapping pairs={overlaps:7d}")
    print()

    print("# the same comparability question at three widths, to check 16 is not the artifact")
    for ib, fb in ((1, 1), (2, 2), (3, 3)):
        vals, st = numeral(ib, fb)
        iv = gamma_interval(vals, st)
        pairs = list(combinations(range(len(iv)), 2))
        comp = sum(1 for i, j in pairs if comparable(iv[i], iv[j]))
        print(f"U<{ib},{fb}>  base values={len(vals):3d}  interval data={len(iv):5d}  "
              f"comparable={comp / len(pairs):6.2%}")


if __name__ == "__main__":
    main()
