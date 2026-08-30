#!/usr/bin/env python3
"""
p2. Reproducing `114` F114-10 before conceding it, and pinning the mechanism
    with a hand witness rather than only with a count.

THE CLAIM AGAINST ME
--------------------
`112` F112-24 reports the one-sided form "beaten on none" over thirteen rows.
`114` section 5.3 reports that this is a fact about ONE-SIDED declarations,
and that under two-endpoint declarations the form is beaten by the plain
interval rule on 92 cells at arity two and 593 at arity three, with the
mechanism being a negative constant flipping the sign of every coefficient it
scales.

`112` F112-23 already records that every extent that file swept is one-sided,
so the two findings sit in the same list and neither points at the other.

WHAT THIS PROBE DOES
--------------------
The object under test is MY form, so it is imported from my own committed
`112_probes/p9` rather than reimplemented, which is the same code `114`
imported.  A reimplementation would be testing my reading of my own rule.
What is independent here is the sweep: my own two-endpoint declaration
enumeration and my own term set.

PREDICTIONS, RECORDED BEFORE THE RUN
------------------------------------
P1. The hand witness in `114` section 5.3 reproduces exactly: at `iW3` with
    both leaves declared [-4, -1], my form gives [-8, 25] on `x * y` and the
    corner rule gives [1, 16].
P2. Under ONE-SIDED declarations the form is beaten on 0 cells, reproducing
    my own F112-24 in the region it was measured over.
P3. Under TWO-ENDPOINT declarations it is beaten on a nonzero count.
P4. The beaten cells concentrate on multiplication terms, because the sign
    flip needs a product to appear at all.
P5. Restricting the two-endpoint sweep to declarations whose lower bound is
    at least zero returns the count to 0, which would show the mechanism is
    the negative constant rather than the two endpoints as such.  This is the
    discriminator `114` names but does not measure separately.

CONDITION-CAN-FIRE CHECK
------------------------
P2 and P3 are each other's control: if the one-sided rows are also nonzero
the sweep is not isolating what it claims to, and if the two-endpoint rows
are zero the enumeration never reached a negative-constant declaration.
"""

from fractions import Fraction
from itertools import product
import importlib.util
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
P112 = HERE.parent / "112_probes"


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    m = importlib.util.module_from_spec(spec)
    sys.modules[name] = m
    spec.loader.exec_module(m)
    return m


p7 = load("p7", P112 / "p7_an_affine_grade_recovers_the_lost_licences.py")
p9 = load("p9", P112 / "p9_a_one_sided_form_attacks_the_affine_rules_weakness.py")

Prim, L, O = p7.Prim, p7.L, p7.O


def leaves(t):
    return p7.leaves_of(t)


def sweep(P, t, decls, label, quiet=False):
    """decls: list of dicts name -> (lo, hi)."""
    names = sorted(leaves(t))
    vals = P.values()
    beaten = corner_yes = one_yes = oracle = cells = 0
    one_unsound = corner_unsound = 0
    witnesses = []
    for d in decls:
        gi = {n: (Fraction(d[n][0]), Fraction(d[n][1])) for n in names}
        go = {n: p9.one_from(Fraction(d[n][0]), Fraction(d[n][1]), n) for n in names}
        doms = [[v for v in vals if d[n][0] <= v <= d[n][1]] for n in names]
        if any(len(dm) == 0 for dm in doms):
            continue
        cells += 1
        c = p7.corner_ok(P, t, gi)
        o = p9.one_ok(P, t, go)
        agrees = all(
            p7.eval_exact(t, dict(zip(names, tp)))
            == p7.eval_general(P, t, dict(zip(names, tp)))
            for tp in product(*doms)
        )
        corner_yes += int(c)
        one_yes += int(o)
        oracle += int(agrees)
        if c and not agrees:
            corner_unsound += 1
        if o and not agrees:
            one_unsound += 1
        if c and not o:
            beaten += 1
            if len(witnesses) < 2:
                witnesses.append(dict(d))
    if not quiet:
        print(
            f"  {label:<44} cells {cells:>6}  corner {corner_yes:>6}  "
            f"one-sided {one_yes:>6}  BEATEN {beaten:>5}  "
            f"unsound c={corner_unsound} o={one_unsound}"
        )
        for w in witnesses:
            print(f"      witness: {w}")
    return beaten, cells


def onesided_decls(P, names):
    return [
        {n: (0, b) for n, b in zip(names, bs)}
        for bs in product(range(0, int(P.hi) + 1), repeat=len(names))
    ]


def twoendpoint_decls(P, names, step=1):
    per = [
        (lo, hi)
        for lo in range(int(P.lo), int(P.hi) + 1, step)
        for hi in range(lo, int(P.hi) + 1, step)
    ]
    return [dict(zip(names, combo)) for combo in product(per, repeat=len(names))]


def nonneg_twoendpoint_decls(P, names, step=1):
    per = [
        (lo, hi)
        for lo in range(0, int(P.hi) + 1, step)
        for hi in range(lo, int(P.hi) + 1, step)
    ]
    return [dict(zip(names, combo)) for combo in product(per, repeat=len(names))]


def main():
    print("=" * 92)
    print("p2. Reproducing the two-endpoint beating of my own one-sided form")
    print("=" * 92)

    # ---- P1: the hand witness, computed rather than quoted ----------------
    print()
    print("P1. The hand witness from `114` section 5.3, recomputed")
    print()
    S3 = Prim(3, 0, True, "sat")
    t = O("mul", L("x"), L("y"))
    gi = {"x": (Fraction(-4), Fraction(-1)), "y": (Fraction(-4), Fraction(-1))}
    go = {
        "x": p9.one_from(Fraction(-4), Fraction(-1), "x"),
        "y": p9.one_from(Fraction(-4), Fraction(-1), "y"),
    }
    ci = p7.corner(t, gi)
    oi = p9.one_eval(t, go).interval()
    true_lo = min(a * b for a in range(-4, 0) for b in range(-4, 0))
    true_hi = max(a * b for a in range(-4, 0) for b in range(-4, 0))
    print(f"  leaves both declared [-4, -1], term x * y, container [{S3.lo}, {S3.hi}]")
    print(f"    true reachable range   : [{true_lo}, {true_hi}]")
    print(f"    corner rule            : [{ci[0]}, {ci[1]}]")
    print(f"    my one-sided form      : [{oi[0]}, {oi[1]}]")
    print(
        f"    matches `114`'s worked [-8, 25] against [1, 16]: "
        f"{(oi[0], oi[1]) == (-8, 25) and (ci[0], ci[1]) == (1, 16)}"
    )

    # ---- P2 and P3 --------------------------------------------------------
    print()
    print("P2/P3. One-sided against two-endpoint declarations")
    print()
    tests = [
        (Prim(3, 0, True, "sat"), O("mul", L("x"), L("y")), "iW3/sat  x * y"),
        (Prim(3, 0, True, "sat"), O("mul", L("x"), L("x")), "iW3/sat  x * x"),
        (Prim(3, 0, True, "sat"), O("add", L("x"), L("y")), "iW3/sat  x + y"),
        (Prim(3, 0, False, "sat"), O("mul", L("x"), L("y")), "uW3/sat  x * y"),
        (
            Prim(3, 0, True, "sat"),
            O("mul", O("add", L("x"), L("y")), L("z")),
            "iW3/sat  (x + y) * z",
        ),
        (
            Prim(3, 0, True, "sat"),
            O("sub", O("add", L("x"), L("y")), L("y")),
            "iW3/sat  (x + y) - y",
        ),
    ]
    tot_one = tot_two = 0
    for P, t, label in tests:
        names = sorted(leaves(t))
        b1, _ = sweep(P, t, onesided_decls(P, names), f"{label}  ONE-SIDED")
        b2, _ = sweep(P, t, twoendpoint_decls(P, names), f"{label}  TWO-ENDPOINT")
        tot_one += b1
        tot_two += b2
        print()

    print("  totals across the six terms:")
    print(f"    beaten under one-sided declarations : {tot_one}")
    print(f"    beaten under two-endpoint            : {tot_two}")

    # ---- P5: the discriminator -------------------------------------------
    print()
    print("P5. Two-endpoint but with every lower bound at least zero")
    print()
    tot_nonneg = 0
    for P, t, label in tests:
        names = sorted(leaves(t))
        b, _ = sweep(
            P, t, nonneg_twoendpoint_decls(P, names), f"{label}  TWO-ENDPOINT, lo >= 0"
        )
        tot_nonneg += b
    print()
    print(f"    beaten with two endpoints but no negative lower bound: {tot_nonneg}")
    print(
        "    so the mechanism is the NEGATIVE CONSTANT and not the second endpoint: "
        f"{tot_nonneg == 0 and tot_two > 0}"
    )

    print()
    print("CONDITION-CAN-FIRE CHECK")
    print()
    print(
        f"  the one-sided rows are {tot_one} and the two-endpoint rows are {tot_two}; "
        f"the sweep isolates the declaration shape: {tot_one == 0 and tot_two > 0}"
    )


if __name__ == "__main__":
    main()
