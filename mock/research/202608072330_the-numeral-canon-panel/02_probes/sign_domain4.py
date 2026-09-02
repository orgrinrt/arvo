#!/usr/bin/env python3
"""Locating the disagreement sign_domain3.py's crosscheck reported.

sign_domain3.py checked its structural inclusion predicate against materialised value
sets and found 48 disagreements out of 576 ordered pairs at radix two.  That is a defect
and it is reported here rather than smoothed over, because the factorisation result of
that file rests on the predicate.

Hypothesis, stated before it is tested: the grid clause of the four-condition order is
VACUOUS on a numeral carrying fewer than two values.  A singleton lies on every grid, so
set inclusion can hold while the predicate's divisibility test fails.  If that is the
whole of it, every disagreement has a source of cardinality one, and the predicate is
sound wherever both sides carry at least two values, which is the region the
factorisation result is computed over.

Run: python3 sign_domain4.py
"""

from itertools import product
from sign_domain import DOMAINS, value_set
from sign_domain3 import shape, includes, cardinality


def classify(reading, radix, precs, expos):
    total = dis = dis_singleton_src = dis_other = 0
    examples = []
    for x in product(precs, DOMAINS, expos):
        for y in product(precs, DOMAINS, expos):
            total += 1
            vx = value_set(radix, x[0], x[1], x[2], reading)
            vy = value_set(radix, y[0], y[1], y[2], reading)
            if (vx <= vy) == includes(shape(radix, *x, reading), shape(radix, *y, reading)):
                continue
            dis += 1
            if len(vx) < 2:
                dis_singleton_src += 1
            else:
                dis_other += 1
                if len(examples) < 3:
                    examples.append((x, y, sorted(vx), sorted(vy)))
    return total, dis, dis_singleton_src, dis_other, examples


def restricted(reading, radix, precs, expos):
    """The same check, over pairs where both sides carry at least two values."""
    total = dis = 0
    for x in product(precs, DOMAINS, expos):
        if cardinality(radix, x[0], x[1], reading) < 2:
            continue
        for y in product(precs, DOMAINS, expos):
            if cardinality(radix, y[0], y[1], reading) < 2:
                continue
            total += 1
            vx = value_set(radix, x[0], x[1], x[2], reading)
            vy = value_set(radix, y[0], y[1], y[2], reading)
            if (vx <= vy) != includes(shape(radix, *x, reading), shape(radix, *y, reading)):
                dis += 1
    return total, dis


if __name__ == "__main__":
    for reading in ("P1", "P2"):
        print(f"\n================ reading {reading}")
        for r in (2, 3, 10):
            t, d, ds, do, ex = classify(reading, r, range(0, 4), range(-1, 1))
            print(f"  radix {r:>2}: ordered pairs = {t}, disagreements = {d}, "
                  f"of which source carries fewer than two values = {ds}, other = {do}")
            for e in ex:
                print(f"            unexplained: {e[0]} vs {e[1]}: {e[2]} vs {e[3]}")
            t2, d2 = restricted(reading, r, range(0, 4), range(-1, 1))
            print(f"            restricted to both sides carrying two or more values: "
                  f"pairs = {t2}, disagreements = {d2}")
