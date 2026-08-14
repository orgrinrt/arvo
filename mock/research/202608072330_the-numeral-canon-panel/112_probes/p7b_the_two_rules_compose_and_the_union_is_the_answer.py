#!/usr/bin/env python3
"""
p7b. Neither grade rule dominates, so measure the composition rather than
     picking a winner.

WHAT p7 FOUND
-------------
The affine grade recovers everything the corner grade loses to correlation and
LOSES on multiplication of non-centred quantities:

  (x + y) - y      corner  16/256    affine 136/256   oracle 136/256
  x * (y - y)      corner  31/64     affine  64/64    oracle  64/64
  x * y            corner  76/256    affine  31/256   oracle  76/256
  (x + y) * z      corner 385/4096   affine 151/4096  oracle 385/4096
  (x+y) * (z+w)    corner 212/256    affine  31/256   oracle 212/256

The mechanism for the losses is standard and is worth naming rather than
leaving as a number: an affine form centres a non-negative interval [0, b] at
b/2 with radius b/2, and the product of two symmetric forms carries a negative
lower bound the interval rule never had.  Affine arithmetic trades sign
information for correlation information, and on a non-negative domain that is
a bad trade whenever a multiply is present and a good one whenever a leaf
repeats.

So this is not a rule to choose.  It is two arms with two predicates, which is
I13's shape, and the composition is the deliverable.

THE COMPOSITION
---------------
Both rules are sound: p7 reports unsound 0 for both on every row.  Two sound
predicates disjoin into a sound predicate, so the composed arm licenses when
EITHER licenses, and it costs both evaluations at compile time and nothing at
runtime.

PREDICTIONS, RECORDED BEFORE THE RUN
------------------------------------
P1. The union is sound: 0 unsound on every row.
P2. The union equals the elementwise maximum of the two counts on every row,
    which is the claim that neither rule licenses a cell the other refuses
    while being the smaller count.  If this FAILS the union is strictly better
    than the max and the two rules are genuinely complementary rather than
    nested per row.
P3. The union reaches the reachable-set oracle on every row swept.  If it
    does, two const-computable rules together match an enumerating oracle,
    which is the result worth carrying.
P4. There exists a term where the union is still short of the oracle, because
    the annihilation case in p3b is about the term's result not depending on a
    node and neither rule reasons about that.  I expect `(x + y) * z` with z
    declared zero to be the witness, and I expect P3 to fail on it.
"""

from fractions import Fraction
from itertools import product
import importlib.util
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location(
    "p7", Path(__file__).with_name("p7_an_affine_grade_recovers_the_lost_licences.py")
)
p7 = importlib.util.module_from_spec(spec)
sys.modules["p7"] = p7
spec.loader.exec_module(p7)

Prim, L, O = p7.Prim, p7.L, p7.O


def sweep_union(P, t, bound_range, label):
    names = sorted(p7.leaves_of(t))
    values = P.values()
    counts = dict(corner=0, affine=0, union=0, oracle=0, extents=0)
    unsound = dict(corner=0, affine=0, union=0)
    strictly_complementary = 0
    for bs in product(bound_range, repeat=len(names)):
        gi = {n: (Fraction(0), Fraction(b)) for n, b in zip(names, bs)}
        ga = {
            n: p7.aff_from_interval(Fraction(0), Fraction(b), n)
            for n, b in zip(names, bs)
        }
        doms = [[v for v in values if 0 <= v <= b] for b in bs]
        if any(len(d) == 0 for d in doms):
            continue
        counts["extents"] += 1
        c = p7.corner_ok(P, t, gi)
        a = p7.affine_ok(P, t, ga)
        u = c or a
        oracle = all(
            p7.node_ok_exact(P, t, dict(zip(names, tup))) for tup in product(*doms)
        )
        agrees = all(
            p7.eval_exact(t, dict(zip(names, tup)))
            == p7.eval_general(P, t, dict(zip(names, tup)))
            for tup in product(*doms)
        )
        counts["corner"] += int(c)
        counts["affine"] += int(a)
        counts["union"] += int(u)
        counts["oracle"] += int(oracle)
        if c and not agrees:
            unsound["corner"] += 1
        if a and not agrees:
            unsound["affine"] += 1
        if u and not agrees:
            unsound["union"] += 1
        if a and not c:
            strictly_complementary += 1
    mx = max(counts["corner"], counts["affine"])
    print(
        f"  {label:<32} c {counts['corner']:>5} a {counts['affine']:>5} "
        f"UNION {counts['union']:>5} oracle {counts['oracle']:>5} "
        f"of {counts['extents']:>5}   unsound(u)={unsound['union']}  "
        f"union>max: {counts['union'] > mx}  reaches oracle: "
        f"{counts['union'] == counts['oracle']}"
    )
    return counts, unsound, strictly_complementary


def main():
    print("=" * 78)
    print("p7b. The union of the two grade rules")
    print("=" * 78)
    print()

    P4 = Prim(4, 0, False, "sat")
    S4 = Prim(4, 0, True, "sat")
    P4w = Prim(4, 0, False, "wrap")

    cases = [
        (P4, O("add", L("x"), L("y")), range(0, 16), "x + y"),
        (P4, O("mul", L("x"), L("y")), range(0, 16), "x * y"),
        (P4, O("add", O("add", L("x"), L("y")), L("z")), range(0, 16), "(x + y) + z"),
        (
            P4,
            O("sub", O("add", L("x"), L("y")), L("y")),
            range(0, 16),
            "(x + y) - y",
        ),
        (
            S4,
            O("mul", L("x"), O("sub", L("y"), L("y"))),
            range(0, 8),
            "x * (y - y)  SIGNED",
        ),
        (
            P4,
            O("mul", O("add", L("x"), L("y")), L("z")),
            range(0, 16),
            "(x + y) * z",
        ),
        (
            P4,
            O("mul", O("add", L("x"), L("y")), O("add", L("z"), L("w"))),
            range(0, 4),
            "(x+y) * (z+w)",
        ),
        (
            S4,
            O("sub", O("mul", L("x"), L("y")), O("mul", L("x"), L("y"))),
            range(0, 4),
            "x*y - x*y  SIGNED",
        ),
        (P4w, O("sub", O("add", L("x"), L("y")), L("y")), range(0, 16), "(x + y) - y  WRAP"),
        (
            S4,
            O("add", O("sub", L("x"), L("y")), L("y")),
            range(0, 8),
            "(x - y) + y  SIGNED",
        ),
    ]

    rows = []
    for P, t, rng, label in cases:
        rows.append((label,) + sweep_union(P, t, rng, label))

    print()
    print("SUMMARY")
    print()
    tot_unsound = sum(r[2]["union"] for r in rows)
    reaches = sum(1 for r in rows if r[1]["union"] == r[1]["oracle"])
    beats_max = sum(
        1 for r in rows if r[1]["union"] > max(r[1]["corner"], r[1]["affine"])
    )
    compl = sum(1 for r in rows if r[3] > 0)
    print(f"  rows swept                                   : {len(rows)}")
    print(f"  rows where the union is unsound              : {tot_unsound}")
    print(f"  rows where the union reaches the oracle      : {reaches}/{len(rows)}")
    print(f"  rows where the union beats BOTH counts       : {beats_max}/{len(rows)}")
    print(f"  rows where affine licenses what corner refuses: {compl}/{len(rows)}")

    print()
    print("THE ROWS THE UNION DOES NOT REACH, named rather than left in a ratio")
    print()
    for label, counts, unsound, compl in rows:
        if counts["union"] != counts["oracle"]:
            print(
                f"  {label:<32} union {counts['union']}, oracle {counts['oracle']}, "
                f"short by {counts['oracle'] - counts['union']}"
            )
    if reaches == len(rows):
        print("  none: the union reaches the oracle on every row swept")

    print()
    print("THE ANNIHILATION CASE, checked directly since P4 predicted it")
    print()
    P = P4
    t = O("mul", O("add", L("x"), L("y")), L("z"))
    names = sorted(p7.leaves_of(t))
    union_yes = agrees_yes = 0
    for bx, by in product(range(0, 16), repeat=2):
        bs = {"x": bx, "y": by, "z": 0}
        gi = {n: (Fraction(0), Fraction(bs[n])) for n in names}
        ga = {n: p7.aff_from_interval(Fraction(0), Fraction(bs[n]), n) for n in names}
        doms = [[v for v in P.values() if 0 <= v <= bs[n]] for n in names]
        u = p7.corner_ok(P, t, gi) or p7.affine_ok(P, t, ga)
        agrees = all(
            p7.eval_exact(t, dict(zip(names, tup)))
            == p7.eval_general(P, t, dict(zip(names, tup)))
            for tup in product(*doms)
        )
        union_yes += int(u)
        agrees_yes += int(agrees)
    print(
        f"  with z declared 0: the union licenses {union_yes}/256, "
        f"the arms actually agree on {agrees_yes}/256"
    )
    print(
        f"  so the union is short by {agrees_yes - union_yes} on the annihilation "
        f"case, which is a fact about the TERM's dependence rather than about "
        f"any node's range"
    )

    print()
    print("INSTRUMENT CHECK")
    print()
    print("  MUTATION: make the union unsound on purpose by dropping the node check")

    def broken_ok(P, t, g):
        lo, hi = p7.corner(t, g)
        return P.lo <= lo and hi <= P.hi

    P = P4
    t = O("add", O("add", L("x"), L("y")), L("z"))
    names = sorted(p7.leaves_of(t))
    bad = n = 0
    for bs in product(range(0, 16), repeat=3):
        gi = {k: (Fraction(0), Fraction(b)) for k, b in zip(names, bs)}
        doms = [[v for v in P.values() if 0 <= v <= b] for b in bs]
        if any(len(d) == 0 for d in doms):
            continue
        n += 1
        if broken_ok(P, t, gi):
            agrees = all(
                p7.eval_exact(t, dict(zip(names, tup)))
                == p7.eval_general(P, t, dict(zip(names, tup)))
                for tup in product(*doms)
            )
            if not agrees:
                bad += 1
    print(
        f"    a root-only check (no per-node check) is unsound on {bad}/{n} extents"
    )
    print(
        f"    so the per-node discipline is load-bearing and the sound counts "
        f"above are not free: {bad > 0}"
    )


if __name__ == "__main__":
    main()
