#!/usr/bin/env python3
"""T1. Reproduce F118-1 before accepting or rejecting it, and check the one
thing its statement asserts that arithmetic on my own figures does not give.

F118-1, as the dispatch states it: my F115-1 conjunction declines 3072 and 384
cells at wrap on which the policy-selected check is already exact, so condition
(b) is vacuous at wrap and load-bearing at sat, and the answer is neither my
repair nor `114`'s but two arms with two condition sets.

**Two of those three numbers are already in my own committed output.** `115`
F115-1 reports the conjunction firing on 6336 and 816 cells at the two wrapping
bases; F115-2 reports condition (a) alone firing on 9408 and 1200. The
differences are 3072 and 384 exactly. So the counts are arithmetic on figures I
published side by side in one table rather than a new measurement against me.

**What is not arithmetic on my figures is the word "exact".** My s1 counted
violations, which is the conjunction certifying a cell the check is conservative
on. It did not count, on the cells the conjunction declines and condition (a)
alone accepts, whether the policy-selected check is in fact exact there. If some
of those 3072 are cells where the check is conservative, then condition (a) alone
would be certifying a conservative cell, which is a violation of the weaker
certificate, and F118-1 would be wrong in my favour rather than against me.

So this probe measures the thing my own run did not:

  T1a. the decline counts, recomputed rather than subtracted.
  T1b. on every declined cell, is the policy-selected check EXACT? A single
       cell where it is conservative refutes F118-1's "already exact".
  T1c. the sat half, which is `114`'s p1 control on my instrument: dropping
       condition (b) at saturation must produce violations, or (b) is not
       load-bearing anywhere and the two-arm conclusion collapses to one arm.

The case that must fail is T1c. If dropping (b) produced zero violations at
saturation on this enumeration, the conjunction would be pointless everywhere and
the correct answer would be my F115-2 generalised rather than two arms.
"""

import importlib.util
import os
import sys
from itertools import product

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)
S1 = os.path.join(PANEL, "115_probes",
                  "s1_the_wrap_violation_reproduced_and_the_predicate_reaimed.py")

spec = importlib.util.spec_from_file_location("s1", S1)
s1 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(s1)

Prim = s1.Prim
ALL_TERMS = s1.ALL_TERMS
nvars = s1.nvars
decls = s1.decls
arms_agree = s1.arms_agree
policy_check = s1.policy_check
per_node_licenses = s1.per_node_licenses
root_licenses = s1.root_licenses
leaves_are_linear = s1.leaves_are_linear
no_annihilating_ancestor = s1.no_annihilating_ancestor


def sweep(P):
    """One pass, collecting everything the three questions need."""
    check = policy_check(P)
    n = dict(cells=0, exact=0, conservative=0, unsound=0,
             fires_ab=0, fires_a=0, declined=0,
             declined_and_exact=0, declined_and_conservative=0,
             viol_ab=0, viol_a=0)
    for t in ALL_TERMS:
        k = nvars(t)
        a_ok = leaves_are_linear(t)
        for ext in decls(P, k):
            lic = check(P, t, ext)
            agree = arms_agree(P, t, ext)
            b_ok = no_annihilating_ancestor(t, ext)
            conservative = agree and not lic
            unsound = lic and not agree
            exact = not conservative and not unsound

            n["cells"] += 1
            n["exact"] += exact
            n["conservative"] += conservative
            n["unsound"] += unsound
            if a_ok and b_ok:
                n["fires_ab"] += 1
                n["viol_ab"] += conservative
            if a_ok:
                n["fires_a"] += 1
                n["viol_a"] += conservative
                if not b_ok:
                    # condition (a) accepts, the conjunction declines
                    n["declined"] += 1
                    n["declined_and_exact"] += exact
                    n["declined_and_conservative"] += conservative
    return n


def sat_ablation(P):
    """T1c. At saturation, drop condition (b) and count violations of the
    resulting weaker certificate against the per-node check."""
    check = policy_check(P)
    viol_ab = viol_a = fires_ab = fires_a = 0
    for t in ALL_TERMS:
        k = nvars(t)
        a_ok = leaves_are_linear(t)
        for ext in decls(P, k):
            lic = check(P, t, ext)
            agree = arms_agree(P, t, ext)
            conservative = agree and not lic
            if a_ok:
                fires_a += 1
                viol_a += conservative
                if no_annihilating_ancestor(t, ext):
                    fires_ab += 1
                    viol_ab += conservative
    return fires_ab, viol_ab, fires_a, viol_a


def main():
    print("T1. F118-1 reproduced on 115's own instrument")
    print("=" * 78)
    print("terms:", len(ALL_TERMS), "(the enumeration s1 built)")
    print()

    wraps = [Prim(3, False, "wrap"), Prim(3, True, "wrap")]
    sats = [Prim(3, False, "sat"), Prim(3, True, "sat")]

    print("T1a and T1b. at wrap, with the policy-selected check (root)")
    print()
    print(f"  {'primitive':<11} {'fires (a)+(b)':>13} {'fires (a)':>10} "
          f"{'declined':>9} {'of those EXACT':>15} {'CONSERVATIVE':>13}")
    ok_b = True
    for P in wraps:
        n = sweep(P)
        if n["declined_and_conservative"]:
            ok_b = False
        print(f"  {P.label():<11} {n['fires_ab']:>13} {n['fires_a']:>10} "
              f"{n['declined']:>9} {n['declined_and_exact']:>15} "
              f"{n['declined_and_conservative']:>13}")
    print()
    print(f"  every declined cell is exact: {ok_b}")
    print("  (a single conservative one would refute F118-1's 'already exact',")
    print("   and would mean condition (a) alone certifies a conservative cell)")

    print()
    print("  the same rows' violation counts, so the soundness claim is visible:")
    print(f"  {'primitive':<11} {'viol (a)+(b)':>13} {'viol (a) alone':>15}")
    for P in wraps:
        n = sweep(P)
        print(f"  {P.label():<11} {n['viol_ab']:>13} {n['viol_a']:>15}")

    print()
    print("-" * 78)
    print("T1c. THE CASE THAT MUST FAIL: at sat, is condition (b) load-bearing?")
    print()
    print(f"  {'primitive':<11} {'fires (a)+(b)':>13} {'viol':>6} "
          f"{'fires (a) only':>15} {'viol':>6}")
    b_matters = False
    for P in sats:
        fab, vab, fa, va = sat_ablation(P)
        if va > vab:
            b_matters = True
        print(f"  {P.label():<11} {fab:>13} {vab:>6} {fa:>15} {va:>6}")
    print()
    print(f"  dropping condition (b) at saturation produces violations: {b_matters}")
    print("  so (b) is load-bearing where the map is not a homomorphism, and")
    print("  the counter that reports zero at wrap is the same counter")

    print()
    print("-" * 78)
    print("verdict on F118-1, computed rather than asserted:")
    d = [sweep(P)["declined"] for P in wraps]
    print(f"  decline counts at the two wrapping bases : {d}")
    print(f"  all declined cells already exact          : {ok_b}")
    print(f"  condition (b) load-bearing at saturation  : {b_matters}")
    print(f"  => F118-1 holds on this instrument        : "
          f"{ok_b and b_matters and d == [3072, 384]}")


if __name__ == "__main__":
    sys.exit(main())
