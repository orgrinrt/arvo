#!/usr/bin/env python3
"""S1. Reproduce `114` F114-3 on my own implementation, then re-aim the
predicate at the check the overflow policy selects.

`114` F114-3 reports that `111` F111-15's structural predicate, whose predicate
lists `overflow policy in {sat, wrap}`, is unsound at `wrap`: 28 violations of
13032 cells at `uW3/wrap` and 16 of 2148 at `iW3/wrap`, with the smallest
witness `(x - (y - z))` declared `[(0,0), (0,0), (0,1)]`.

A concession on someone else's numbers is not a concession, so this reproduces it
on the implementation `111_probes/r2` actually shipped, imported rather than
rewritten, and then asks the question the refutation opens.

**The question.** My predicate certifies that a refusal by the **per-node
interval check** is honest. `114` F114-1 establishes that a wrapping map is a
ring homomorphism, and its arm W1 establishes that under a homomorphism the
right check is the **root** rather than every node. So under `wrap` the predicate
was certifying refusals by a check that is not the check that policy selects.
That is a different defect from the predicate being wrong about term structure,
and it predicts a specific repair: aim the predicate at the policy-selected
check and the violations should go to zero without the predicate changing at all.

Predicted before running, recorded so it can be wrong:

  P1. the witness reproduces exactly: predicate fires, per-node refuses, arms
      agree, so the cell is conservative and the predicate certified it.
  P2. violations against the PER-NODE check are nonzero at `wrap` and zero at
      `sat`, reproducing F114-3's shape.
  P3. violations against the POLICY-SELECTED check (root under wrap, per-node
      under sat) are zero at both policies.
  P4. condition (b) is not needed under wrap, because (b) exists to forbid
      downstream masking of an intermediate node's overflow and under a
      homomorphism no intermediate node's overflow matters.

Controls, because an arm with no failing case proves nothing:

  C1. the violation counter must be able to fire. P2's wrap rows are that.
  C2. a predicate that always fires must produce violations equal to the
      conservative count, on every row.
  C3. the root check must be UNSOUND at `sat`, or "select the check by policy"
      would be a preference rather than a requirement.
"""

import importlib.util
import os
import sys
from itertools import product

HERE = os.path.dirname(os.path.abspath(__file__))
R2 = os.path.join(os.path.dirname(HERE), "111_probes",
                  "r2_a_structural_predicate_for_where_the_corner_rule_is_exact.py")

spec = importlib.util.spec_from_file_location("r2", R2)
r2 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r2)

Prim = r2.Prim
ev = r2.ev
iv = r2.iv
internal = r2.internal
leaves = r2.leaves
leaves_are_linear = r2.leaves_are_linear
no_annihilating_ancestor = r2.no_annihilating_ancestor

OPS = ("add", "sub", "mul")


# ------------------------------------------------------------------ checks

def per_node_licenses(P, t, ext):
    """The check r2 uses, and the one `111` F111-15's predicate certifies."""
    for n in internal(t):
        lo, hi = iv(n, ext)
        if lo < P.lo or hi > P.hi:
            return False
    return True


def root_licenses(P, t, ext):
    """`114` arm W1's check: the root's propagated interval and nothing else."""
    lo, hi = iv(t, ext)
    return P.lo <= lo and hi <= P.hi


def policy_check(P):
    """The check the overflow policy selects, per `114` F114-1 and arm W1."""
    return root_licenses if P.policy == "wrap" else per_node_licenses


def arms_agree(P, t, ext):
    for env in product(*[range(lo, hi + 1) for lo, hi in ext]):
        if ev(P, t, env, False) != ev(P, t, env, True):
            return False
    return True


# ------------------------------------------------------------- term space

def terms_at(slots):
    """Every term over OPS at `slots` leaf slots, with every leaf
    identification, which is `114` p1's enumeration."""
    out = []
    if slots == 2:
        for op in OPS:
            for a, b in ((0, 0), (0, 1)):
                out.append((None, op, ("leaf", a), ("leaf", b)))
        return out
    # three slots: left-nested and right-nested
    parts = [(0, 0, 0), (0, 0, 1), (0, 1, 0), (0, 1, 1), (0, 1, 2)]
    for o1 in OPS:
        for o2 in OPS:
            for p in parts:
                l = (None, o1, (None, o2, ("leaf", p[0]), ("leaf", p[1])),
                     ("leaf", p[2]))
                r = (None, o1, ("leaf", p[0]),
                     (None, o2, ("leaf", p[1]), ("leaf", p[2])))
                out.append(l)
                out.append(r)
    return out


ALL_TERMS = terms_at(2) + terms_at(3)


def nvars(t):
    return len(set(leaves(t)))


def decls(P, k):
    return [[(0, b) for b in bs] for bs in product(range(P.hi + 1), repeat=k)]


# -------------------------------------------------------------- the sweep

def sweep(P, check, cond_a_only=False, always_fire=False):
    cells = conservative = fires = violations = unsound = 0
    for t in ALL_TERMS:
        k = nvars(t)
        for full in decls(P, k):
            # `iv` and `ev` index by leaf label, so `full` is per label
            lic = check(P, t, full)
            agree = arms_agree(P, t, full)
            cells += 1
            if lic and not agree:
                unsound += 1
            if agree and not lic:
                conservative += 1
            if always_fire:
                fired = True
            elif cond_a_only:
                fired = leaves_are_linear(t)
            else:
                fired = leaves_are_linear(t) and no_annihilating_ancestor(t, full)
            if fired:
                fires += 1
                if agree and not lic:
                    violations += 1
    return cells, conservative, fires, violations, unsound


def main():
    print("S1. the wrap violation reproduced, and the predicate re-aimed")
    print("=" * 78)
    print(f"terms enumerated: {len(ALL_TERMS)} "
          f"(2 leaf slots: {len(terms_at(2))}, 3 leaf slots: {len(terms_at(3))})")

    # ---- P1: the named witness
    print()
    print("P1. the witness 114 p2 prints, on r2's own implementation")
    P = Prim(3, False, "sat")
    Pw = Prim(3, False, "wrap")
    t = (None, "sub", ("leaf", 0), (None, "sub", ("leaf", 1), ("leaf", 2)))
    ext = [(0, 0), (0, 0), (0, 1)]
    for prim in (Pw, P):
        inner = iv(t[3], ext)
        print(f"  {prim.label():<10} term (x - (y - z)) declared {ext}")
        print(f"    inner (y - z) propagates to {inner}, container "
              f"[{prim.lo}, {prim.hi}]")
        print(f"    per-node licenses : {per_node_licenses(prim, t, ext)}")
        print(f"    root      licenses: {root_licenses(prim, t, ext)}")
        print(f"    arms agree        : {arms_agree(prim, t, ext)}")
        print(f"    condition (a)     : {leaves_are_linear(t)}")
        print(f"    condition (b)     : {no_annihilating_ancestor(t, ext)}")
        cons = arms_agree(prim, t, ext) and not per_node_licenses(prim, t, ext)
        fires = leaves_are_linear(t) and no_annihilating_ancestor(t, ext)
        print(f"    => conservative under the per-node check: {cons}")
        print(f"    => predicate fires: {fires}   VIOLATION: {cons and fires}")

    # ---- P2 and P3
    print()
    print("P2 and P3. violations against the per-node check and against the")
    print("check the overflow policy selects (root under wrap, per-node at sat)")
    print()
    print(f"  {'primitive':<11} {'cells':>7} {'consv':>7} {'fires':>7} "
          f"{'viol/per-node':>14} {'viol/policy':>12} {'unsound/policy':>15}")
    rows = [Prim(3, False, "sat"), Prim(3, True, "sat"),
            Prim(3, False, "wrap"), Prim(3, True, "wrap")]
    for prim in rows:
        c, cons, f, v_pn, _ = sweep(prim, per_node_licenses)
        c2, cons2, f2, v_pol, uns_pol = sweep(prim, policy_check(prim))
        print(f"  {prim.label():<11} {c:>7} {cons:>7} {f:>7} "
              f"{v_pn:>14} {v_pol:>12} {uns_pol:>15}")

    # ---- P4: is condition (b) needed under wrap?
    print()
    print("P4. is condition (b) load-bearing under wrap with the root check?")
    print()
    print(f"  {'primitive':<11} {'fires (a)+(b)':>14} {'viol':>6} "
          f"{'fires (a) only':>15} {'viol':>6}")
    for prim in (Prim(3, False, "wrap"), Prim(3, True, "wrap")):
        _, _, f_ab, v_ab, _ = sweep(prim, policy_check(prim))
        _, _, f_a, v_a, _ = sweep(prim, policy_check(prim), cond_a_only=True)
        print(f"  {prim.label():<11} {f_ab:>14} {v_ab:>6} {f_a:>15} {v_a:>6}")

    # ---- controls
    print()
    print("CONTROLS")
    print()
    print("C2. a predicate that always fires must produce violations equal to")
    print("    the conservative count, or the violation counter is not counting")
    print(f"  {'primitive':<11} {'consv':>7} {'viol (always-fire)':>19}")
    for prim in rows:
        _, cons, _, v, _ = sweep(prim, policy_check(prim), always_fire=True)
        print(f"  {prim.label():<11} {cons:>7} {v:>19}")

    print()
    print("C3. the root check must be UNSOUND at sat, or selecting the check by")
    print("    policy would be a preference rather than a requirement")
    print(f"  {'primitive':<11} {'unsound (root check)':>21}")
    for prim in (Prim(3, False, "sat"), Prim(3, True, "sat")):
        _, _, _, _, uns = sweep(prim, root_licenses)
        print(f"  {prim.label():<11} {uns:>21}")

    print()
    print("-" * 78)
    print("reading: the predicate did not need changing. What was wrong was the")
    print("check it was pointed at, and which check is right is a property of")
    print("the overflow policy, which 114 F114-1 establishes and which is known")
    print("before any value exists.")


if __name__ == "__main__":
    main()
