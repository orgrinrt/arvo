#!/usr/bin/env python3
"""
p7c. The witness p7b's mutation could not reach, built by hand.

p7b's mutation asked whether a root-only range check is as good as a per-node
one, and measured 0 unsound over 4096 extents.  That number is vacuous: every
extent swept has the form [0, b], so over a non-negative domain the root of an
addition chain is the widest node and a root that fits implies every node fits.
The mutation's condition could not occur.

A two-endpoint extent breaks that, and a two-endpoint extent is the shape `111`
section 12 alternative B names as untested and `82` F6 actually uses.

PREDICTION, RECORDED BEFORE THE RUN
-----------------------------------
Over unsigned saturating W = 4, the term (x + y) - z with x in [8, 10],
y in [8, 10] and z pinned at 15 has a root interval of [1, 5], which fits the
container, and an intermediate x + y in [16, 20], which does not.  A root-only
check licenses the cheap arm and the cheap arm is WRONG, because the general
arm saturates the intermediate.  I predict a non-zero disagreement count and I
predict the per-node check refuses.
"""

from fractions import Fraction
from itertools import product


def sat(v, lo, hi):
    return min(max(v, lo), hi)


def main():
    LO, HI = 0, 15
    xs = range(8, 11)
    ys = range(8, 11)
    zs = [15]

    root_lo = min(xs) + min(ys) - max(zs)
    root_hi = max(xs) + max(ys) - min(zs)
    inner_lo, inner_hi = min(xs) + min(ys), max(xs) + max(ys)

    print("=" * 74)
    print("p7c. The per-node check is load-bearing")
    print("=" * 74)
    print()
    print(f"  container            : [{LO}, {HI}]")
    print(f"  declared x           : [{min(xs)}, {max(xs)}]")
    print(f"  declared y           : [{min(ys)}, {max(ys)}]")
    print(f"  declared z           : [{min(zs)}, {max(zs)}]")
    print(f"  propagated x + y     : [{inner_lo}, {inner_hi}]   fits: "
          f"{LO <= inner_lo and inner_hi <= HI}")
    print(f"  propagated (x+y) - z : [{root_lo}, {root_hi}]   fits: "
          f"{LO <= root_lo and root_hi <= HI}")
    print()

    root_only = LO <= root_lo and root_hi <= HI
    per_node = root_only and LO <= inner_lo and inner_hi <= HI
    print(f"  a ROOT-ONLY check licenses the cheap arm : {root_only}")
    print(f"  a PER-NODE check licenses the cheap arm  : {per_node}")
    print()

    bad = n = 0
    witnesses = []
    for x, y, z in product(xs, ys, zs):
        n += 1
        cheap = x + y - z
        general = sat(sat(x + y, LO, HI) - z, LO, HI)
        if cheap != general:
            bad += 1
            if len(witnesses) < 4:
                witnesses.append((x, y, z, cheap, general))
    print(f"  the two arms disagree on {bad}/{n} tuples inside the declaration")
    for x, y, z, c, g in witnesses:
        print(f"    x={x} y={y} z={z}: cheap {c}, general {g}")
    print()
    print(f"  so the root-only check is UNSOUND here: {root_only and bad > 0}")
    print(f"  and the per-node check refuses, correctly: {not per_node}")
    print()
    print("INSTRUMENT CHECK")
    print()
    print("  the same term with one-sided extents [0, b], which is what every")
    print("  sweep in this file uses:")
    for bx, by, bz in [(10, 10, 15), (15, 15, 15), (3, 3, 15)]:
        rl, rh = 0 + 0 - bz, bx + by - 0
        il, ih = 0, bx + by
        ro = LO <= rl and rh <= HI
        pn = ro and LO <= il and ih <= HI
        print(
            f"    x<={bx} y<={by} z<={bz}: root [{rl},{rh}] fits {ro}, "
            f"inner [{il},{ih}] fits {LO <= il and ih <= HI}, "
            f"root-only and per-node agree: {ro == pn}"
        )
    print()
    print("  they agree on every one, which is why p7b's mutation measured 0.")


if __name__ == "__main__":
    main()
