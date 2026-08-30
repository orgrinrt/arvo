#!/usr/bin/env python3
"""i2d: is it the families that fail to close, or their union?

i2c measured the three named exponent shapes together as one set and found it
closed under the meet and not under the join. That is a statement about the
union. This asks the same of each family alone, because if each family alone is
closed under both, then the failure is a cross-family fact and not a defect in
either family, which is exactly what `03` reports from the other side.

It also checks the claim against the record's own polarity, which runs the other
way: `seed/SETTLED_laws.md:278-288` has meets needing two closure conditions and
joins needing none, WITHIN one family. That is a statement about endpoints, in
the coordinates (F, -L, G). This instrument holds endpoints aside and looks only
at the exponent function. The two are not in conflict and this file says so
rather than letting a reader assume it.
"""

import itertools

from i2c_join_closure import shapes, close_under, describe


def family_sets(window, depth, allshapes):
    es = list(range(window))
    fixed = {tuple(c for _ in es) for c in range(-depth, window)} & allshapes
    flt = {tuple(e - c for e in es) for c in range(0, depth + 1)} & allshapes
    knee = set()
    for c in range(0, depth + 1):
        for k in es:
            v = tuple(max(e - c, k - c) for e in es)
            if v in allshapes:
                knee.add(v)
    return {"fixed alone": fixed,
            "float alone": flt,
            "float with underflow alone": knee,
            "fixed + float": fixed | flt,
            "all three": fixed | flt | knee}


if __name__ == "__main__":
    for window, depth in ((4, 3), (6, 4), (8, 4)):
        A = shapes(window, depth)
        print(f"window {window}, depth {depth}, {len(A)} monotone inhabited shapes")
        for name, S in family_sets(window, depth, A).items():
            M = close_under(max, S, A)
            J = close_under(min, S, A)
            print(f"    {name:28s} n={len(S):>3}  meet-closure={len(M):>3}  "
                  f"join-closure={len(J):>3}"
                  + ("" if len(J) == len(S) else "   <- leaks"))
        print()

    A = shapes(6, 4)
    F = family_sets(6, 4, A)
    print("The first leak from fixed + float, in full:")
    shown = 0
    for a in sorted(F["fixed alone"]):
        for b in sorted(F["float alone"]):
            m = tuple(min(x, y) for x, y in zip(a, b))
            if m in A and m not in F["fixed + float"]:
                print(f"    fixed {a}")
                print(f"  v float {b}")
                print(f"  =       {m}  -> {describe(m)}")
                shown += 1
                if shown == 3:
                    break
        if shown == 3:
            break
