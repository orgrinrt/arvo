#!/usr/bin/env python3
"""p2. Isolating p1's wrap violations, and the mechanism behind them.

WHAT p1 FOUND
-------------
The structural predicate `111` F111-15 proposes, measured over every term at two
and three leaf slots rather than over twelve hand-picked rows:

    uW3/sat    violations   0
    iW3/sat    violations   0
    uW3/wrap   violations  28
    iW3/wrap   violations  16

and, in the same run, the root-only discharge mutation `112` p7c refutes:

    uW3/sat    root-only unsound on 38 cells
    iW3/sat    root-only unsound on 34 cells
    uW3/wrap   root-only unsound on  0 cells
    iW3/wrap   root-only unsound on  0 cells

Two inversions on the same axis, pointing opposite ways. That is a mechanism
rather than a coincidence, and this probe names it.

THE HYPOTHESIS
--------------
A wrapping realisation map is a RING HOMOMORPHISM and a saturating one is not.

  R_wrap(v) = ((v - lo) mod n) + lo  is the canonical representative of v's
  residue class mod n, so R(v) = v (mod n) for every v. For any ring operation
  `op` in {add, sub, mul},

      R(R(a) op R(b)) = R(a op b)

  because both sides are the representative of the same residue class. So the
  arm that clamps at every node computes exactly R applied once to the exact
  result, and every intermediate overflow is invisible at the root.

  R_sat has no such property: clamping destroys the residue and the loss is
  carried forward.

FOUR CONSEQUENCES, EACH PREDICTED BEFORE THE RUN
-------------------------------------------------
P1. Under wrap, `arms agree` is exactly `the root's exact value is in range for
    every tuple`, with no reference to any intermediate node. Predicted: 0
    disagreements between those two properties over the whole sweep.
P2. Therefore under wrap a ROOT-ONLY corner check is SOUND on every cell, and
    it is EXACT wherever condition (a) holds, because under (a) the propagated
    interval's endpoints are the reachable extremes.
P3. Therefore under wrap the PER-NODE corner check is conservative rather than
    load-bearing, and condition (b) is answering a question nobody asked, which
    is why the predicate goes unsound: it certifies the per-node rule as exact
    when the per-node rule is refusing cells the root-only rule licenses.
P4. Under sat the root-only check is unsound, which is `112` F112-21, and the
    per-node check is required. So the two policies want two different checks.

NEGATIVE CONTROLS
-----------------
C1. Apply the wrap-shaped rule (root-only) to a SATURATING primitive. It must
    go unsound. If it does not, "root-only is sound" is not a fact about
    wrapping and P2 is a dead branch.
C2. Break the homomorphism by adding an operation that is not a ring operation.
    `min` is used, because it is monotone and total and is exactly the shape a
    clamping kernel already contains. Under wrap with `min` in the signature,
    P1 must FAIL, or the ring argument is not what is carrying the result.
C3. The homomorphism itself is checked directly and exhaustively rather than
    asserted, and the same check is run against saturation, where it must fail.
"""

from itertools import product
import random
import sys

sys.setrecursionlimit(10000)

import importlib.util
from pathlib import Path

spec = importlib.util.spec_from_file_location(
    "p1", Path(__file__).with_name("p1_the_structural_predicate_on_a_systematic_term_enumeration.py")
)
p1 = importlib.util.module_from_spec(spec)
sys.modules["p1"] = p1
spec.loader.exec_module(p1)

Prim = p1.Prim
all_terms, leaves, internal, show = p1.all_terms, p1.leaves, p1.internal, p1.show
iv, ev, tuples = p1.iv, p1.ev, p1.tuples
cond_a, cond_b = p1.cond_a, p1.cond_b
corner_licenses = p1.corner_licenses


def root_only(P, t, ext):
    for lo, hi in ext:
        if not (P.lo <= lo and hi <= P.hi):
            return False
    lo, hi = iv(t, ext)
    return P.lo <= lo and hi <= P.hi


def root_exact_in_range(P, t, ext, k):
    for env in tuples(ext, k):
        v = ev(P, t, env, False)
        if not (P.lo <= v <= P.hi):
            return False
    return True


def arms_agree(P, t, ext, k):
    for env in tuples(ext, k):
        if ev(P, t, env, False) != ev(P, t, env, True):
            return False
    return True


# ------------------------------------------------------------------ C3: the map


def homomorphism_check(P, ops):
    """Exhaustive over the whole ambient product range: is R(R(a) op R(b)) equal
    to R(a op b)?"""
    span = P.hi - P.lo + 1
    dom = range(P.lo - span, P.hi + span + 1)
    bad = tot = 0
    for a in dom:
        for b in dom:
            for op in ops:
                lhs = P.R(p1.apply_op(op, P.R(a), P.R(b)))
                rhs = P.R(p1.apply_op(op, a, b))
                tot += 1
                if lhs != rhs:
                    bad += 1
    return bad, tot


def main():
    print("=" * 78)
    print("p2. The overflow policy decides which discharge check is right")
    print("=" * 78)

    # ---------------------------------------------------------------- C3 first
    print()
    print("C3. Is the realisation map a ring homomorphism? Checked, not asserted.")
    print("    Domain is the whole ambient range the operands can reach.")
    print()
    print(f"  {'primitive':<12} {'ops':<20} {'mismatches':>12} {'of':>10}")
    for P in (
        Prim(3, False, "wrap"),
        Prim(3, True, "wrap"),
        Prim(4, False, "wrap"),
        Prim(3, False, "sat"),
        Prim(3, True, "sat"),
    ):
        bad, tot = homomorphism_check(P, ("add", "sub", "mul"))
        print(f"  {P.label():<12} {'add, sub, mul':<20} {bad:>12} {tot:>10}")
    print()
    print("  The saturating rows are the control. A zero on every row would mean")
    print("  the check cannot distinguish the two policies and proves nothing.")

    # ------------------------------------------------------------------ P1, P2
    print()
    print("-" * 78)
    print("P1 and P2. Under wrap, does the root alone decide everything?")
    print()
    print(
        f"  {'primitive':<12} {'cells':>7} {'agree':>7} {'rootval':>8} {'P1 gap':>7} "
        f"{'pernode':>8} {'rootchk':>8} {'ro unsnd':>9} {'ro consv':>9}"
    )
    for P in (
        Prim(3, False, "wrap"),
        Prim(3, True, "wrap"),
        Prim(3, False, "sat"),
        Prim(3, True, "sat"),
    ):
        terms = all_terms(2) + all_terms(3)
        cells = agree_n = rootval_n = gap = per_n = ro_n = ro_uns = ro_cons = 0
        for t in terms:
            k = max(leaves(t)) + 1
            for bs in product(range(0, P.hi + 1), repeat=k):
                ext = tuple((0, b) for b in bs)
                cells += 1
                ag = arms_agree(P, t, ext, k)
                rv = root_exact_in_range(P, t, ext, k)
                pn = corner_licenses(P, t, ext)
                ro = root_only(P, t, ext)
                agree_n += ag
                rootval_n += rv
                per_n += pn
                ro_n += ro
                if ag != rv:
                    gap += 1
                if ro and not ag:
                    ro_uns += 1
                if ag and not ro:
                    ro_cons += 1
        print(
            f"  {P.label():<12} {cells:>7} {agree_n:>7} {rootval_n:>8} {gap:>7} "
            f"{per_n:>8} {ro_n:>8} {ro_uns:>9} {ro_cons:>9}"
        )
    print()
    print("  P1 gap: cells where 'the arms agree' and 'the root's exact value is")
    print("  always in range' disagree. Zero means the root decides everything.")
    print("  ro unsnd / ro consv: the root-only check's own soundness and loss.")
    print("  C1 is the saturating rows: root-only must go unsound there.")

    # ------------------------------------------------------------- P2 exactness
    print()
    print("-" * 78)
    print("P2 continued. Under wrap, is the root-only check EXACT where (a) holds?")
    print()
    for P in (Prim(3, False, "wrap"), Prim(3, True, "wrap")):
        terms = all_terms(2) + all_terms(3)
        fires = viol = 0
        for t in terms:
            k = max(leaves(t)) + 1
            if not cond_a(t):
                continue
            for bs in product(range(0, P.hi + 1), repeat=k):
                ext = tuple((0, b) for b in bs)
                fires += 1
                ag = arms_agree(P, t, ext, k)
                ro = root_only(P, t, ext)
                if ag != ro:
                    viol += 1
        print(
            f"  {P.label():<12} condition (a) holds on {fires} cells; "
            f"root-only disagrees with the arms on {viol}"
        )
    print()
    print("  The same count run on a saturating primitive, which must not be zero:")
    for P in (Prim(3, False, "sat"), Prim(3, True, "sat")):
        terms = all_terms(2) + all_terms(3)
        fires = viol = 0
        for t in terms:
            k = max(leaves(t)) + 1
            if not cond_a(t):
                continue
            for bs in product(range(0, P.hi + 1), repeat=k):
                ext = tuple((0, b) for b in bs)
                fires += 1
                if arms_agree(P, t, ext, k) != root_only(P, t, ext):
                    viol += 1
        print(
            f"  {P.label():<12} condition (a) holds on {fires} cells; "
            f"root-only disagrees with the arms on {viol}"
        )

    # ------------------------------------------------------------ the witnesses
    print()
    print("-" * 78)
    print("The violations named. Cells where (a) and (b) hold, the per-node corner")
    print("rule refuses, and the arms agree anyway. Under wrap only.")
    print()
    P = Prim(3, False, "wrap")
    shown = 0
    for t in all_terms(2) + all_terms(3):
        k = max(leaves(t)) + 1
        for bs in product(range(0, P.hi + 1), repeat=k):
            ext = tuple((0, b) for b in bs)
            if not (cond_a(t) and cond_b(t, ext)):
                continue
            if corner_licenses(P, t, ext):
                continue
            if not arms_agree(P, t, ext, k):
                continue
            if shown < 8:
                nodes = [
                    f"{show(n)} in {iv(n, ext)}" for n in internal(t)
                ]
                print(f"  {show(t):<20} declared {list(ext)}")
                print(f"      nodes: {'; '.join(nodes)}   container [{P.lo}, {P.hi}]")
                print(f"      root-only says: {root_only(P, t, ext)}")
            shown += 1
    print()
    print(f"  total such cells at {P.label()}: {shown}")

    # -------------------------------------------------------------------- C2
    print()
    print("-" * 78)
    print("C2. Break the ring by adding `min` to the signature. P1 must fail.")
    print()
    old_ops = p1.OPS
    old_apply = p1.apply_op

    def apply_with_min(op, x, y):
        if op == "min":
            return min(x, y)
        return old_apply(op, x, y)

    p1.apply_op = apply_with_min
    p1.OPS = ("add", "sub", "mul", "min")
    try:
        for P in (Prim(3, False, "wrap"), Prim(3, True, "wrap")):
            bad, tot = homomorphism_check(P, ("add", "sub", "mul", "min"))
            terms = all_terms(3)
            cells = gap = 0
            rng = random.Random(20260814)
            for t in rng.sample(terms, 60):
                k = max(leaves(t)) + 1
                for bs in product(range(0, P.hi + 1), repeat=k):
                    ext = tuple((0, b) for b in bs)
                    cells += 1
                    if arms_agree(P, t, ext, k) != root_exact_in_range(P, t, ext, k):
                        gap += 1
            print(
                f"  {P.label():<12} homomorphism mismatches {bad} of {tot};  "
                f"P1 gap {gap} of {cells} cells over 60 sampled terms"
            )
    finally:
        p1.apply_op = old_apply
        p1.OPS = old_ops
    print()
    print("  A nonzero P1 gap here is the control firing: the root stops deciding")
    print("  everything the moment the signature leaves the ring.")


if __name__ == "__main__":
    main()
