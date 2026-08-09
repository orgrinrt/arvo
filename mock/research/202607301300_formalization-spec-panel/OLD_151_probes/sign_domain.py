#!/usr/bin/env python3
"""What the sign domain is: coordinate of the order, or input to the range coordinate.

Builds the three declared sign domains (NonNegative, Symmetric, AsymmetricLow, from
110:915) as value sets, and asks four questions of the resulting order:

  Q1  Does the sign domain survive the quotient by V(a) = V(b)?  A coordinate of an
      order defined by value-set inclusion must; a presentation parameter need not.
  Q2  Does the inclusion order factor as a product order over (grid, precision, domain)
      for ANY partial order on the three domains?  A coordinate must make it factor.
  Q3  Which domain pairs are comparable at equal grid and equal precision?  A two-domain
      reading has no instance of an intra-precision relation; a three-domain one may.
  Q4  Does a numeral denoting exactly {0} exist without admitting precision zero?

The record is ambiguous on whether Precision counts the sign digit.  138:92 gives
IFixed<I,F> precision 1+I+F (sign counted) and FastFloat<P,..> precision P with an IEEE
sign bit outside it (sign not counted).  Both readings are built and every answer is
reported under both, so no conclusion here rests on resolving that ambiguity.

Run: python3 sign_domain.py
"""

from fractions import Fraction
from itertools import product, combinations

DOMAINS = ("NonNegative", "Symmetric", "AsymmetricLow")


def endpoints(radix, prec, domain, reading):
    """Return (lo, hi) as integer multiples of the quantum, or None if not admitted."""
    if reading == "P1":
        # Precision counts every digit, the sign digit included (138:92, IFixed row).
        c = radix ** prec
        if domain == "NonNegative":
            return (0, c - 1)
        if domain == "Symmetric":
            m = (c - 1) // 2
            return (-m, m)
        if domain == "AsymmetricLow":
            return (-(c // 2), (c - c // 2) - 1)
    else:
        # Precision counts magnitude digits, the sign sits outside it
        # (138:92, FastFloat row, matching IEEE 754's separate sign bit).
        c = radix ** prec
        if domain == "NonNegative":
            return (0, c - 1)
        if domain == "Symmetric":
            return (-(c - 1), c - 1)
        if domain == "AsymmetricLow":
            return (-c, c - 1)
    raise ValueError(domain)


def value_set(radix, prec, domain, expo, reading):
    """Exact value set. Quantum is radix**expo, phase zero throughout."""
    lo, hi = endpoints(radix, prec, domain, reading)
    q = Fraction(radix) ** expo
    return frozenset(Fraction(k) * q for k in range(lo, hi + 1))


def build(radix, precs, expos, reading):
    fam = {}
    for p, d, e in product(precs, DOMAINS, expos):
        fam[(radix, p, d, e)] = value_set(radix, p, d, e, reading)
    return fam


def partial_orders_on_three():
    """Every reflexive, antisymmetric, transitive relation on three labelled elements."""
    idx = range(3)
    offdiag = [(i, j) for i in idx for j in idx if i != j]
    out = []
    for bits in product((0, 1), repeat=len(offdiag)):
        rel = {(i, i) for i in idx}
        rel |= {pair for pair, b in zip(offdiag, bits) if b}
        if any((i, j) in rel and (j, i) in rel for i, j in offdiag):
            continue  # antisymmetry
        if any((i, j) in rel and (j, k) in rel and (i, k) not in rel
               for i in idx for j in idx for k in idx):
            continue  # transitivity
        out.append(rel)
    return out


def q1_quotient(reading, radices, precs, expos):
    print(f"--- Q1  does the sign domain survive the quotient by V(a) = V(b)?   [{reading}]")
    for r in radices:
        fam = build(r, precs, expos, reading)
        collisions = []
        for a, b in combinations(sorted(fam), 2):
            if a[2] != b[2] and fam[a] == fam[b]:
                collisions.append((a, b))
        verdict = "NO, it collapses" if collisions else "yes, distinct"
        print(f"  radix {r:>2}: cross-domain value-set collisions = {len(collisions):>4}   ({verdict})")
        for a, b in collisions[:3]:
            print(f"            {a[1:4]} == {b[1:4]}   both denote a set of size {len(fam[a])}")


def q2_product(reading, radices, precs, expos):
    print(f"--- Q2  does inclusion factor as a product order over (grid, precision, domain)?   [{reading}]")
    pos = partial_orders_on_three()
    for r in radices:
        fam = build(r, precs, expos, reading)
        keys = sorted(fam)
        survivors = []
        witness = None
        for rel in pos:
            ok = True
            for x in keys:
                for y in keys:
                    incl = fam[x] <= fam[y]
                    # componentwise: target grid at least as fine, precision no smaller,
                    # domain no larger under the candidate order
                    grid_ok = (x[3] >= y[3])
                    prod = grid_ok and x[1] <= y[1] and (DOMAINS.index(x[2]), DOMAINS.index(y[2])) in rel
                    if incl != prod:
                        ok = False
                        if witness is None:
                            witness = (x, y, incl, prod)
                        break
                if not ok:
                    break
            if ok:
                survivors.append(rel)
        print(f"  radix {r:>2}: partial orders on the three domains tried = {len(pos)}, "
              f"surviving = {len(survivors)}")
    # one witness, stated in full, at radix two
    fam = build(2, precs, expos, reading)
    for x in sorted(fam):
        for y in sorted(fam):
            if x[3] != y[3] or x[2] == y[2]:
                continue
            if fam[x] <= fam[y] and x[1] > y[1]:
                lo_x, hi_x = endpoints(2, x[1], x[2], reading)
                lo_y, hi_y = endpoints(2, y[1], y[2], reading)
                print(f"  radix  2 witness: {x[2]}(p={x[1]}) [{lo_x},{hi_x}] "
                      f"is contained in {y[2]}(p={y[1]}) [{lo_y},{hi_y}] "
                      f"while its precision is strictly larger, so no order on the domains "
                      f"can carry the relation")
                return


def q3_intra_precision(reading, radices, precs):
    print(f"--- Q3  which domains are comparable at equal grid and equal precision?   [{reading}]")
    for r in radices:
        counts = {}
        for p in precs:
            sets = {d: value_set(r, p, d, 0, reading) for d in DOMAINS}
            for a, b in combinations(DOMAINS, 2):
                if sets[a] <= sets[b]:
                    counts[(a, b)] = counts.get((a, b), 0) + 1
                if sets[b] <= sets[a]:
                    counts[(b, a)] = counts.get((b, a), 0) + 1
        if not counts:
            print(f"  radix {r:>2}: no intra-precision relations (the domains form an antichain)")
        for (a, b), n in sorted(counts.items()):
            print(f"  radix {r:>2}: {a} is contained in {b} at {n} of {len(precs)} precisions")


def q4_bottom(reading, radices, precs):
    print(f"--- Q4  is there a numeral denoting exactly the zero set, at precision one or more?   [{reading}]")
    for r in radices:
        hits = [(p, d) for p in precs for d in DOMAINS
                if p >= 1 and value_set(r, p, d, 0, reading) == frozenset({Fraction(0)})]
        print(f"  radix {r:>2}: {'yes ' + str(hits) if hits else 'no'}")


def q5_codes(reading, radices, precs):
    print(f"--- Q5  code count against value count (which domain leaves a spare code)   [{reading}]")
    for r in radices:
        for p in precs:
            if p != 3:
                continue
            for d in DOMAINS:
                lo, hi = endpoints(r, p, d, reading)
                vals = hi - lo + 1
                codes = r ** p if reading == "P1" else (r ** p if d == "NonNegative" else 2 * r ** p)
                print(f"  radix {r:>2} p={p} {d:<14} values={vals:>5} codes={codes:>5} "
                      f"spare={codes - vals}")


def q6_lattice(reading, precs, expos):
    """Meets and joins in the radix-two three-domain family, at fixed grid, phase zero."""
    print(f"--- Q6  meets and joins in the radix-two three-domain family, fixed grid   [{reading}]")
    fam = {k: v for k, v in build(2, precs, expos, reading).items() if k[3] == 0}
    keys = sorted(fam)
    pool = {k: v for k, v in build(2, range(max(precs) + 4), [0], reading).items()}
    n_pairs = meet_missing = meet_multi = join_missing = join_multi = 0
    for x, y in combinations(keys, 2):
        n_pairs += 1
        inter = fam[x] & fam[y]
        union = fam[x] | fam[y]
        lows = [k for k, v in pool.items() if v <= inter]
        ups = [k for k, v in pool.items() if v >= union]
        maximal = [a for a in lows if not any(pool[a] < pool[b] for b in lows)]
        minimal = [a for a in ups if not any(pool[b] < pool[a] for b in ups)]
        maxv = {pool[a] for a in maximal}
        minv = {pool[a] for a in minimal}
        if not maxv:
            meet_missing += 1
        elif len(maxv) > 1:
            meet_multi += 1
        if not minv:
            join_missing += 1
        elif len(minv) > 1:
            join_multi += 1
    print(f"  pairs={n_pairs}  meet absent={meet_missing}  meet non-unique={meet_multi}  "
          f"join absent={join_missing}  join non-unique={join_multi}")


if __name__ == "__main__":
    RADICES = (2, 3, 10)
    PRECS = range(0, 6)
    EXPOS = range(-3, 1)
    for reading in ("P1", "P2"):
        print(f"\n================ reading {reading} "
              f"({'sign digit inside precision' if reading == 'P1' else 'sign outside precision'})")
        q1_quotient(reading, RADICES, PRECS, EXPOS)
        q2_product(reading, RADICES, PRECS, EXPOS)
        q3_intra_precision(reading, RADICES, PRECS)
        q4_bottom(reading, RADICES, PRECS)
        q5_codes(reading, RADICES, PRECS)
        q6_lattice(reading, range(0, 5), [0])
