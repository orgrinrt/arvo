#!/usr/bin/env python3
"""Third pass: the factorisation question asked only of non-degenerate numerals.

sign_domain.py reported that no partial order on the three sign domains makes the
inclusion order factor as a product over (grid, precision, domain), and every witness it
printed was at precision zero, where all three domains denote the zero set.  A result
resting on that is resting on a degeneracy, so the whole test is re-run here over
numerals carrying at least two values, and the killing pair is extracted by name rather
than by a count.

The shape of the kill, stated before it is computed, so the computation can refute it:

    Symmetric(p)      is contained in AsymmetricLow(p)      forces  S <= A
    AsymmetricLow(p)  is contained in Symmetric(p + k)      forces  A <= S

for some k, and antisymmetry then forces S = A while the two denote different value
sets.  If both inclusions hold non-degenerately, no partial order survives, and the sign
domain cannot be a coordinate of a product order.

A first version of this file materialised every value set and did not finish inside two
minutes at radix ten.  It is rewritten to decide inclusion from the shape alone, and the
structural predicate is cross-checked against materialised sets on a small box before it
is used, so the speedup is not taken on trust.

Run: python3 sign_domain3.py
"""

from fractions import Fraction
from itertools import product
from sign_domain import DOMAINS, endpoints, value_set, partial_orders_on_three


def shape(radix, prec, domain, expo, reading):
    """(lo, hi) in absolute units as Fractions, plus the quantum."""
    lo, hi = endpoints(radix, prec, domain, reading)
    q = Fraction(radix) ** expo
    return (Fraction(lo) * q, Fraction(hi) * q, q)


def includes(sx, sy):
    """Does the numeral with shape sx have its value set inside sy's?

    Both are anchored at zero, so the phase condition is automatic and the test is the
    remaining three of the four: the target grid is at least as fine, and the target
    reaches at least as far on each side.
    """
    lx, hx, qx = sx
    ly, hy, qy = sy
    return (qx / qy).denominator == 1 and ly <= lx and hx <= hy


def cardinality(radix, prec, domain, reading):
    lo, hi = endpoints(radix, prec, domain, reading)
    return hi - lo + 1


def crosscheck(reading, radix, precs, expos):
    """Confirm the structural predicate agrees with materialised sets."""
    bad = n = 0
    for x in product(precs, DOMAINS, expos):
        for y in product(precs, DOMAINS, expos):
            n += 1
            vx = value_set(radix, x[0], x[1], x[2], reading)
            vy = value_set(radix, y[0], y[1], y[2], reading)
            if (vx <= vy) != includes(shape(radix, *x, reading), shape(radix, *y, reading)):
                bad += 1
    return n, bad


def factorisation(reading, radix, precs, expos, min_values=2):
    keys = [k for k in product(precs, DOMAINS, expos)
            if cardinality(radix, k[0], k[1], reading) >= min_values]
    shp = {k: shape(radix, *k, reading) for k in keys}
    survivors = 0
    for rel in partial_orders_on_three():
        if all(includes(shp[x], shp[y]) ==
               (x[2] >= y[2] and x[0] <= y[0]
                and (DOMAINS.index(x[1]), DOMAINS.index(y[1])) in rel)
               for x in keys for y in keys):
            survivors += 1
    return len(keys), survivors


def forced_relations(reading, radix, precs, expo=0, min_values=2):
    """Every domain relation the inclusion order forces, at one grid."""
    keys = [(p, d) for p, d in product(precs, DOMAINS)
            if cardinality(radix, p, d, reading) >= min_values]
    shp = {k: shape(radix, k[0], k[1], expo, reading) for k in keys}
    forced = set()
    for x in keys:
        for y in keys:
            if x[0] <= y[0] and includes(shp[x], shp[y]):
                forced.add((x[1], y[1]))
    return forced


def witness(reading, radix, precs, expo=0):
    out = []
    for p in precs:
        if cardinality(radix, p, "Symmetric", reading) < 2:
            continue
        s = shape(radix, p, "Symmetric", expo, reading)
        a = shape(radix, p, "AsymmetricLow", expo, reading)
        if includes(s, a) and not includes(a, s):
            out.append(("Symmetric", p, "AsymmetricLow", p))
            break
    for p in precs:
        if cardinality(radix, p, "AsymmetricLow", reading) < 2:
            continue
        a = shape(radix, p, "AsymmetricLow", expo, reading)
        for pp in precs:
            if pp < p:
                continue
            s = shape(radix, pp, "Symmetric", expo, reading)
            if includes(a, s) and not includes(s, a):
                out.append(("AsymmetricLow", p, "Symmetric", pp))
                break
        if len(out) > 1:
            break
    return out


if __name__ == "__main__":
    BOXES = {2: range(0, 8), 3: range(0, 6), 10: range(0, 4)}
    EXPOS = range(-3, 1)
    for reading in ("P1", "P2"):
        print(f"\n================ reading {reading}")
        for r, precs in BOXES.items():
            n, bad = crosscheck(reading, r, range(0, 4), range(-1, 1))
            n_keys, surv = factorisation(reading, r, precs, EXPOS)
            print(f"  radix {r:>2}: structural predicate checked against materialised sets on "
                  f"{n} ordered pairs, disagreements = {bad}")
            print(f"            non-degenerate numerals in the box = {n_keys:>3}, "
                  f"partial orders on the three domains tried = 19, surviving = {surv}")
            print(f"            domain relations the order forces: "
                  f"{sorted('%s<=%s' % (a[:3], b[:3]) for a, b in forced_relations(reading, r, precs))}")
            for w in witness(reading, r, precs):
                lo1, hi1 = endpoints(r, w[1], w[0], reading)
                lo2, hi2 = endpoints(r, w[3], w[2], reading)
                print(f"            witness: {w[0]}(p={w[1]}) spanning [{lo1},{hi1}] quanta "
                      f"strictly inside {w[2]}(p={w[3]}) spanning [{lo2},{hi2}] quanta, "
                      f"precision {w[1]} <= {w[3]}")
