#!/usr/bin/env python3
"""Sharpened follow-up to sign_domain.py: every witness below is non-degenerate.

The first run's witnesses were all at precision zero, where every domain denotes the
zero set and any two things agree.  A conclusion resting on that would be resting on a
degeneracy.  Everything here requires the value sets involved to carry at least two
values, and the bottom question is asked with precision zero excluded outright.

Run: python3 sign_domain2.py
"""

from fractions import Fraction
from itertools import product, combinations
from sign_domain import DOMAINS, endpoints, value_set, partial_orders_on_three


def nondegenerate_collisions(reading, radices, precs, expos):
    print(f"--- A  cross-domain value-set collisions with at least two values   [{reading}]")
    for r in radices:
        hits = []
        for p, e in product(precs, expos):
            sets = {d: value_set(r, p, d, e, reading) for d in DOMAINS}
            for a, b in combinations(DOMAINS, 2):
                if sets[a] == sets[b] and len(sets[a]) >= 2:
                    hits.append((p, e, a, b, len(sets[a])))
        print(f"  radix {r:>2}: {len(hits)} collisions")
        for h in hits[:2]:
            print(f"            p={h[0]} e={h[1]}  {h[2]} == {h[3]}  ({h[4]} values)")


def nondegenerate_product_witness(reading, radices, precs, expos):
    print(f"--- B  a non-degenerate pair no order on the domains can carry   [{reading}]")
    for r in radices:
        fam = {(p, d, e): value_set(r, p, d, e, reading)
               for p, d, e in product(precs, DOMAINS, expos)}
        found = []
        for x, y in product(sorted(fam), repeat=2):
            if len(fam[x]) < 2 or len(fam[y]) < 2:
                continue
            if fam[x] <= fam[y] and x[0] > y[0]:
                found.append((x, y))
        if not found:
            print(f"  radix {r:>2}: none in this box")
            continue
        x, y = found[0]
        lx, hx = endpoints(r, x[0], x[1], reading)
        ly, hy = endpoints(r, y[0], y[1], reading)
        print(f"  radix {r:>2}: {len(found)} such pairs; smallest: "
              f"{x[1]}(p={x[0]},e={x[2]}) spanning [{lx},{hx}] quanta "
              f"is contained in {y[1]}(p={y[0]},e={y[2]}) spanning [{ly},{hy}] quanta, "
              f"with precision {x[0]} > {y[0]}")


def antisymmetry_of_the_three(reading, radices, precs):
    """Restate Q3 as a per-precision relation table, precision one and up only."""
    print(f"--- C  the three domains at equal grid and equal precision, p >= 1   [{reading}]")
    for r in radices:
        rows = []
        for p in range(1, max(precs) + 1):
            sets = {d: value_set(r, p, d, 0, reading) for d in DOMAINS}
            rel = []
            for a, b in combinations(DOMAINS, 2):
                if sets[a] == sets[b]:
                    rel.append(f"{a[:3]}={b[:3]}")
                elif sets[a] < sets[b]:
                    rel.append(f"{a[:3]}<{b[:3]}")
                elif sets[b] < sets[a]:
                    rel.append(f"{b[:3]}<{a[:3]}")
                else:
                    rel.append(f"{a[:3]}|{b[:3]}")
            rows.append((p, " ".join(rel)))
        print(f"  radix {r:>2}: " + " | ".join(f"p={p}: {s}" for p, s in rows[:4]))


def bottom_without_zero_precision(reading, radices):
    print(f"--- D  does the order have a bottom if precision zero is refused?   [{reading}]")
    for r in radices:
        hits = [(p, d) for p in range(1, 7) for d in DOMAINS
                if value_set(r, p, d, 0, reading) == frozenset({Fraction(0)})]
        print(f"  radix {r:>2}: {hits if hits else 'no numeral at p >= 1 denotes exactly {0}'}")


def lattice_without_zero_precision(reading, radix, precs, pool_precs):
    print(f"--- E  meets and joins at fixed grid with precision zero refused   [{reading}]")
    fam = {(p, d): value_set(radix, p, d, 0, reading)
           for p, d in product(precs, DOMAINS)}
    pool = {(p, d): value_set(radix, p, d, 0, reading)
            for p, d in product(pool_precs, DOMAINS)}
    n = miss_m = multi_m = miss_j = multi_j = 0
    ex_m = ex_j = None
    for x, y in combinations(sorted(fam), 2):
        n += 1
        inter, union = fam[x] & fam[y], fam[x] | fam[y]
        lows = [k for k, v in pool.items() if v <= inter]
        ups = [k for k, v in pool.items() if v >= union]
        maxv = {pool[a] for a in lows if not any(pool[a] < pool[b] for b in lows)}
        minv = {pool[a] for a in ups if not any(pool[b] < pool[a] for b in ups)}
        if not maxv:
            miss_m += 1
            ex_m = ex_m or (x, y)
        elif len(maxv) > 1:
            multi_m += 1
        if not minv:
            miss_j += 1
            ex_j = ex_j or (x, y)
        elif len(minv) > 1:
            multi_j += 1
    print(f"  radix {radix} p in {list(precs)}: pairs={n}  meet absent={miss_m} non-unique={multi_m}  "
          f"join absent={miss_j} non-unique={multi_j}")
    if ex_m:
        print(f"    first meet absence: {ex_m[0]} against {ex_m[1]}")


if __name__ == "__main__":
    RADICES = (2, 3, 10)
    PRECS = range(0, 6)
    EXPOS = range(-3, 1)
    for reading in ("P1", "P2"):
        print(f"\n================ reading {reading}")
        nondegenerate_collisions(reading, RADICES, PRECS, EXPOS)
        nondegenerate_product_witness(reading, RADICES, PRECS, EXPOS)
        antisymmetry_of_the_three(reading, RADICES, PRECS)
        bottom_without_zero_precision(reading, RADICES)
        lattice_without_zero_precision(reading, 2, range(1, 5), range(1, 9))
