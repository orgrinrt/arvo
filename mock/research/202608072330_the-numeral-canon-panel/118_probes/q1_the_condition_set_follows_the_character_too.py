#!/usr/bin/env python3
"""q1. `115`'s repair reproduced, and one notch tighter than `115` states it.

THE DISAGREEMENT THIS IS FOR
-----------------------------
`114` F114-3 refuted `111` F111-15 at `overflow policy = wrap`. `114` offered one
repair, restricting the finding to `sat`. `115` reproduced the refutation exactly,
conceded it, and offered a different repair: keep both policies and aim the
predicate at **the check the overflow policy selects**, which takes the violations
to zero at both policies with the conditions untouched (F115-1).

`115` declines `114`'s repair on the ground that deleting the dimension discards a
region where the conditions do hold against the right check. That is a live
disagreement about the shape of the replacement, and this probe is the
measurement that decides it.

WHAT NEITHER FILE MEASURED
---------------------------
`115` F115-2 reports condition (b) unnecessary under wrap with the root check:
condition (a) alone fires on 9408 and 1200 cells with zero violations. It does not
run the mirror image, condition (a) alone at `sat` against the per-node check.
`114`'s p1 ran that mirror image and reports (a) alone producing 234 and 20
violations at `sat`, and never ran the wrap side against the policy-selected
check.

So each file has one half of a two-by-two and neither has the table. Put together,
the table says something neither states: **the condition set is a function of the
map's algebraic character, exactly as the check is.** If so, `115`'s single
conjunction is sound at both policies and is not minimal at either, and the cost
of the non-minimality is countable.

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. `115` F115-1 reproduces: zero violations at all four primitives with
    (a) and (b) against the policy-selected check.
P2. `115` F115-2 reproduces: (a) alone at wrap against the root check fires on
    9408 and 1200 with zero violations.
P3. The mirror image fails: (a) alone at `sat` against the per-node check
    produces violations. If it does not, condition (b) is doing nothing anywhere
    and both files have been defending a condition that carries no weight.
P4. Therefore the conjunction declines cells at wrap that (a) alone certifies,
    and the count is 9408 - 6336 = 3072 at `uW3/wrap` and 1200 - 816 = 384 at
    `iW3/wrap`.
P5. On every one of those declined cells the root check's verdict already equals
    the arms' verdict, so the decline is a lost licence rather than a harmless
    caution. If some declined cell is one where the root check is genuinely
    conservative, the conjunction is buying something there and P4's count is not
    a pure cost.

NEGATIVE CONTROLS
-----------------
C1. A predicate that always fires must produce violations equal to the
    conservative count against whichever check is in play, or the violation
    counter is not counting.
C2. A condition (b) mutated to always hold must reproduce the (a)-only column
    exactly, or the two spellings of "drop (b)" disagree and one of them is
    measuring something else.
C3. The per-node check must be reported unsound nowhere and the root check
    reported unsound at `sat`, or the two checks are not the two checks.
"""

from itertools import product
import importlib.util
import sys
from pathlib import Path

sys.setrecursionlimit(10000)
HERE = Path(__file__).parent
P114 = HERE.parent / "114_probes"


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    sys.modules[name] = mod
    spec.loader.exec_module(mod)
    return mod


p1 = load("p1_local", P114 / "p1_the_structural_predicate_on_a_systematic_term_enumeration.py")

Prim, all_terms, leaves, internal = p1.Prim, p1.all_terms, p1.leaves, p1.internal
iv, ev, tuples, show = p1.iv, p1.ev, p1.tuples, p1.show
cond_a, cond_b = p1.cond_a, p1.cond_b
per_node = p1.corner_licenses


def root_only(P, t, ext):
    for lo, hi in ext:
        if not (P.lo <= lo and hi <= P.hi):
            return False
    lo, hi = iv(t, ext)
    return P.lo <= lo and hi <= P.hi


def selected_check(P):
    """`114` F114-1 and arm W1: the map's character decides which check is
    sound, and the policy fixes the character at F = 0 over the ring."""
    return root_only if P.policy == "wrap" else per_node


def arms_agree(P, t, ext, k):
    for env in tuples(ext, k):
        if ev(P, t, env, False) != ev(P, t, env, True):
            return False
    return True


def one_sided(P, k):
    return [tuple((0, b) for b in bs) for bs in product(range(0, P.hi + 1), repeat=k)]


CERTS = {
    "(a) and (b)": lambda t, ext: cond_a(t) and cond_b(t, ext),
    "(a) alone": lambda t, ext: cond_a(t),
    "(b) alone": lambda t, ext: cond_b(t, ext),
    "always [C1]": lambda t, ext: True,
    "(a) and (b:=true) [C2]": lambda t, ext: cond_a(t) and True,
}


def sweep(P, terms, check, cert):
    cells = consv = fires = viol = unsound = 0
    for t in terms:
        k = max(leaves(t)) + 1
        for ext in one_sided(P, k):
            cells += 1
            lic = check(P, t, ext)
            ag = arms_agree(P, t, ext, k)
            if ag and not lic:
                consv += 1
            if lic and not ag:
                unsound += 1
            if cert(t, ext):
                fires += 1
                if ag and not lic:
                    viol += 1
    return cells, consv, fires, viol, unsound


def main():
    print("=" * 92)
    print("q1. The condition set follows the character too")
    print("=" * 92)

    prims = [
        Prim(3, False, "sat"),
        Prim(3, True, "sat"),
        Prim(3, False, "wrap"),
        Prim(3, True, "wrap"),
    ]
    terms = all_terms(2) + all_terms(3)
    print(f"\nterms enumerated: {len(terms)}  (2 leaf slots: {len(all_terms(2))}, "
          f"3 leaf slots: {len(all_terms(3))})")

    # ---------------------------------------------------- P1, P2, P3, C1, C2
    print()
    print("P1, P2, P3. Every certificate against the check the policy selects.")
    print()
    header = f"  {'primitive':<10} {'check':<9} {'cells':>6} {'consv':>6} {'unsnd':>6}  "
    header += "  ".join(f"{n:>22}" for n in CERTS)
    print(header)
    print(f"  {'':<10} {'':<9} {'':>6} {'':>6} {'':>6}  " +
          "  ".join(f"{'fires / violations':>22}" for _ in CERTS))
    rows = {}
    for P in prims:
        chk = selected_check(P)
        name = "root" if P.policy == "wrap" else "per-node"
        cols = []
        base = None
        for cn, cf in CERTS.items():
            cells, consv, fires, viol, unsound = sweep(P, terms, chk, cf)
            if base is None:
                base = (cells, consv, unsound)
            cols.append(f"{fires:>10} / {viol:<10}")
            rows[(P.label(), cn)] = (fires, viol)
        print(f"  {P.label():<10} {name:<9} {base[0]:>6} {base[1]:>6} {base[2]:>6}  "
              + "  ".join(cols), flush=True)

    print()
    print("P3's mirror image, stated on its own because it is the asymmetry:")
    print()
    print(f"  {'primitive':<10} {'check':<9} {'(a) alone: fires':>18} {'violations':>12}")
    for P in prims:
        f, v = rows[(P.label(), "(a) alone")]
        name = "root" if P.policy == "wrap" else "per-node"
        flag = "   <- (b) IS load-bearing here" if v else "   <- (b) is not load-bearing here"
        print(f"  {P.label():<10} {name:<9} {f:>18} {v:>12}{flag}")

    # ------------------------------------------------------------- P4 and P5
    print()
    print("-" * 92)
    print("P4 and P5. What `115`'s conjunction declines at wrap, and whether")
    print("the decline costs anything.")
    print()
    print(f"  {'primitive':<10} {'(a)+(b) fires':>14} {'(a) fires':>11} {'declined':>10} "
          f"{'of those, root check exact':>28} {'root check conservative':>25}")
    for P in prims:
        if P.policy != "wrap":
            continue
        declined = lost = kept = 0
        for t in terms:
            k = max(leaves(t)) + 1
            for ext in one_sided(P, k):
                if not (cond_a(t) and not cond_b(t, ext)):
                    continue
                declined += 1
                lic = root_only(P, t, ext)
                ag = arms_agree(P, t, ext, k)
                if lic == ag:
                    lost += 1
                else:
                    kept += 1
        fab, _ = rows[(P.label(), "(a) and (b)")]
        fa, _ = rows[(P.label(), "(a) alone")]
        print(f"  {P.label():<10} {fab:>14} {fa:>11} {declined:>10} "
              f"{lost:>28} {kept:>25}")
    print()
    print("  'root check exact' means the check's verdict already equals the arms'")
    print("  verdict on that cell, so declining to certify it loses a licence and")
    print("  buys nothing. 'root check conservative' is the opposite and would mean")
    print("  the conjunction is paying for something there.")

    # ----------------------------------------------------------------- C3
    print()
    print("-" * 92)
    print("C3. The two checks, each against the other's policy.")
    print()
    print(f"  {'primitive':<10} {'per-node unsound':>18} {'root unsound':>14}")
    for P in prims:
        pn_u = ro_u = 0
        for t in terms:
            k = max(leaves(t)) + 1
            for ext in one_sided(P, k):
                ag = arms_agree(P, t, ext, k)
                if per_node(P, t, ext) and not ag:
                    pn_u += 1
                if root_only(P, t, ext) and not ag:
                    ro_u += 1
        print(f"  {P.label():<10} {pn_u:>18} {ro_u:>14}")

    print()
    print("=" * 92)
    print(
        """
  READING IT

  P1 holds when the '(a) and (b)' violation figure is zero on all four rows.
  That is `115` F115-1 reproduced, and if it fails I have mis-read the repair.

  P2 and P3 together are the finding. `115` measured only the wrap half and
  `114` only the sat half. If (a) alone is clean at wrap and dirty at sat, the
  condition set is a function of the character rather than a constant, and the
  minimal certificate differs per policy.

  P4 and P5 price `115`'s formulation. A declined cell where the check was
  already exact is a licence lost for nothing.

  C1 must equal the conservative count. C2 must reproduce the (a)-only column
  exactly. C3 must show the root check unsound at sat and the per-node check
  unsound nowhere.
"""
    )


if __name__ == "__main__":
    main()
