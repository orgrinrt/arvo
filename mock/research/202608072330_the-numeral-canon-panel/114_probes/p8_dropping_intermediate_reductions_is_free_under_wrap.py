#!/usr/bin/env python3
"""p8. Separating three arms the panel has been treating as two, and measuring
which of them a declaration is needed for.

WHY THIS EXISTS
---------------
`111` and `112` both compare exactly two arms: an `exact` arm with no reduction
anywhere, and a `general` arm that reduces at every node. `p2` establishes that
under a wrapping map over ring operations those two agree exactly when the root's
value is in range. `p7` then compiles the same thing at a LOGICAL width narrower
than the container, which is arvo's actual case, and the emitted assembly shows
something the two-arm framing cannot express:

    _general_masked = _cheap_reduced_ungated

masking after every node and masking once at the root are ONE BODY. So there are
three arms, not two, and only two of them need a declaration:

  general          reduce at every node.               correct by definition
  cheap_reduced    no intermediate reductions, one     ??? needs a declaration?
                   at the root.
  cheap_unreduced  no reduction at all, the result     needs the root check
                   handed on as a wider value.

THE CLAIM UNDER TEST
--------------------
Under a wrapping map over ring operations, `cheap_reduced` equals `general` for
EVERY tuple and EVERY declaration, so it needs no declaration at all. It is a
free win available on a predicate that reads only the overflow policy and the
operation set, and nothing in this panel has said so.

The reason, if it holds: reduction mod 2^n is a ring homomorphism, and so is
reduction mod 2^m for the intermediate carrier, and a composite of ring
homomorphisms is a ring homomorphism. So reducing early and reducing late land in
the same residue class.

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. Under wrap, `cheap_reduced` never differs from `general`, at any intermediate
    carrier width at least the logical width, on any term, on any declaration.
P2. Under saturation it differs, often, which is what makes P1 a fact about the
    map rather than about the arms.
P3. `cheap_unreduced` differs from `general` exactly when the root's exact value
    leaves the logical range, under wrap. That is `p2`'s result restated at the
    level of the arm a design would actually emit.
P4. If the intermediate carrier is itself SATURATING while the logical reduction
    is wrapping, P1 fails, because the composite is no longer a homomorphism.
    That is the mixed case a real implementation could stumble into.

NEGATIVE CONTROLS
-----------------
C1. P2 is the control for P1: the same measurement under saturation must be
    nonzero, or the instrument cannot tell the two maps apart.
C2. P4 is a second control from the other direction: keep the logical map
    wrapping and break the CARRIER instead.
C3. A mutation that reduces at the wrong width (mod 2^(n-1) instead of 2^n) must
    show differences under wrap too, or the comparison is insensitive to the
    reduction itself.
"""

from itertools import product
import importlib.util
import random
import sys
from pathlib import Path

sys.setrecursionlimit(10000)
HERE = Path(__file__).parent
p1 = importlib.util.module_from_spec(
    importlib.util.spec_from_file_location(
        "p1_local",
        HERE / "p1_the_structural_predicate_on_a_systematic_term_enumeration.py",
    )
)
sys.modules["p1_local"] = p1
importlib.util.spec_from_file_location(
    "p1_local", HERE / "p1_the_structural_predicate_on_a_systematic_term_enumeration.py"
).loader.exec_module(p1)


def red_wrap(v, n):
    return v % (1 << n)


def red_sat(v, n):
    hi = (1 << n) - 1
    return 0 if v < 0 else (hi if v > hi else v)


def carrier_wrap(v, m):
    return v % (1 << m)


def carrier_sat(v, m):
    hi = (1 << m) - 1
    return 0 if v < 0 else (hi if v > hi else v)


def eval_arm(t, env, arm, n, m, red, carrier):
    """arm 0: reduce at every node. arm 1: no intermediate reduction, reduce at
    the root. arm 2: no reduction at all. The intermediate carrier applies to
    arms 1 and 2, because a machine still has a container."""
    def go(node, reduce_here):
        if node[0] == "leaf":
            return env[node[1]]
        x = go(node[1], reduce_here)
        y = go(node[2], reduce_here)
        v = p1.apply_op(node[0], x, y)
        if reduce_here:
            return red(v, n)
        return carrier(v, m)

    if arm == 0:
        return go(t, True)
    v = go(t, False)
    return red(v, n) if arm == 1 else v


def sweep(terms, n, m, red, carrier, label, decls_per_term, rng):
    d1 = d2 = cells = 0
    root_out = 0
    for t in terms:
        k = max(p1.leaves(t)) + 1
        hi = (1 << n) - 1
        for _ in range(decls_per_term):
            bs = [rng.randint(0, hi) for _ in range(k)]
            doms = [range(0, b + 1) for b in bs]
            cells += 1
            diff1 = diff2 = False
            ro = True
            for env in product(*doms):
                a0 = eval_arm(t, env, 0, n, m, red, carrier)
                a1 = eval_arm(t, env, 1, n, m, red, carrier)
                a2 = eval_arm(t, env, 2, n, m, red, carrier)
                ex = eval_arm(t, env, 2, n, 64, red, lambda v, _m: v)
                if a0 != a1:
                    diff1 = True
                if a0 != a2:
                    diff2 = True
                if not (0 <= ex <= (1 << n) - 1):
                    ro = False
            d1 += diff1
            d2 += diff2
            root_out += 0 if ro else 1
    print(
        f"  {label:<46} cells {cells:>6}  cheap_reduced differs {d1:>6}  "
        f"cheap_unreduced differs {d2:>6}  root out of range {root_out:>6}",
        flush=True,
    )
    return d1, d2, root_out


def main():
    print("=" * 110)
    print("p8. Three arms, and which of them a declaration is needed for")
    print("=" * 110)
    rng = random.Random(20260814)
    terms = p1.all_terms(2) + p1.all_terms(3)
    n = 3  # logical width

    print()
    print(f"  logical width {n}, terms {len(terms)}, 6 sampled declarations each")
    print()
    print("P1: wrapping logical map, wrapping carrier, carrier width at or above")
    print("the logical width")
    for m in (3, 4, 5, 8):
        sweep(terms, n, m, red_wrap, carrier_wrap,
              f"wrap logical, wrap carrier m = {m}", 6, random.Random(1))

    print()
    print("C1: the same measurement under a saturating logical map")
    for m in (3, 4, 5, 8):
        sweep(terms, n, m, red_sat, carrier_sat,
              f"sat logical, sat carrier m = {m}", 6, random.Random(1))

    print()
    print("C2 / P4: wrapping logical map, SATURATING carrier, which breaks the")
    print("composite even though the logical map is a homomorphism")
    for m in (4, 5, 8):
        sweep(terms, n, m, red_wrap, carrier_sat,
              f"wrap logical, SAT carrier m = {m}", 6, random.Random(1))

    print()
    print("C3: the mutation, reducing at the wrong width")
    for m in (4, 8):
        sweep(terms, n, m, lambda v, _n: red_wrap(v, n - 1), carrier_wrap,
              f"wrap logical at n-1 (WRONG), wrap carrier m = {m}", 6,
              random.Random(1))

    print()
    print("=" * 110)
    print(
        """
  P1 holds when the 'cheap_reduced differs' column is zero on every wrapping
  row, at every carrier width, INCLUDING the rows where 'root out of range' is
  large. That is the point: the equality does not depend on any declaration.

  P3 is the 'cheap_unreduced differs' column tracking 'root out of range' on the
  wrapping rows. Where they are equal, the root check is exactly the licence for
  the unreduced arm.

  C1, C2 and C3 must all be nonzero somewhere, or the instrument cannot see a
  difference and every zero above is a dead branch.
"""
    )


if __name__ == "__main__":
    main()
