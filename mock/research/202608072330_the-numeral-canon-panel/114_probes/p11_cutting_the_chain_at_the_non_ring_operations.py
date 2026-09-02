#!/usr/bin/env python3
"""p11. Extending arm W0 past the first clamp, which is where the shipped kernel
needs it.

WHY THIS IS NOT AN AFTERTHOUGHT
--------------------------------
`p2`'s C2 measures that adding `min` to the signature breaks the ring
homomorphism, so arm W0 stops there, and section 6.4 of `114` finds that the
shipped `warm-clamp` kernel is exactly that shape: a run of wrapping additions
terminated by a `min_with`. Read literally, arm W0 licenses the interior of one
chunk and nothing else, which is a licence with a boundary drawn in the wrong
place for the one consumer it has.

The question this asks is whether the licence extends by CUTTING rather than by
weakening. A term over a mixed signature decomposes into maximal segments of ring
operations separated by non-ring nodes. Inside a segment the homomorphism holds.
At a non-ring node it does not, and the operands have to be reduced before that
node can be applied at all, because `min` compares representatives rather than
residues.

THE CANDIDATE RULE
------------------
    Reduce the operands of every non-ring operation, and reduce at the root.
    Reduce nowhere else.

If that equals reducing at every node, arm W0 becomes a statement about ring
SEGMENTS rather than about ring terms, and the shipped kernel's shape is licensed
rather than excluded: the additions inside a chunk are one segment, the
`min_with` is the cut, and the next chunk is the next segment.

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. The cut rule equals the reduce-everywhere arm on every term and every
    declaration, under a wrapping map, with no declaration read. If it does, arm
    W0 generalises and the boundary moves from "the first non-ring operation" to
    "each non-ring operation".
P2. It does NOT equal it under saturation, which is the control that makes P1 a
    fact about the map.
P3. Reducing at the root ALONE, ignoring the non-ring nodes, differs, because a
    `min` applied to unreduced operands compares the wrong things. If it does not
    differ, the cut is unnecessary and the rule is simply "reduce at the root",
    which would be a stronger and more surprising result.
P4. The number of reductions the cut rule performs is the number of non-ring
    nodes plus one, against one per node for the general arm, so on a chunked
    fold of arity n with one clamp it is 2 against n + 1.

NEGATIVE CONTROLS
-----------------
C1. P2 is the primary control: the same measurement at `sat` must differ.
C2. A mutation that cuts at the WRONG nodes, reducing the operands of additions
    instead of the operands of `min`, must differ under wrap. If it does not, the
    instrument is insensitive to where the cut is placed and P1 is a dead branch.
C3. A term with no non-ring operation at all must give the cut rule and arm W0
    the same answer, since with no cuts they are the same rule.
"""

from itertools import product
import importlib.util
import random
import sys
from pathlib import Path

sys.setrecursionlimit(10000)
HERE = Path(__file__).parent

_spec = importlib.util.spec_from_file_location(
    "p1_local", HERE / "p1_the_structural_predicate_on_a_systematic_term_enumeration.py"
)
p1 = importlib.util.module_from_spec(_spec)
sys.modules["p1_local"] = p1
_spec.loader.exec_module(p1)

RING = ("add", "sub", "mul")
ALL_OPS = ("add", "sub", "mul", "min")


def apply_op(op, x, y):
    if op == "add":
        return x + y
    if op == "sub":
        return x - y
    if op == "mul":
        return x * y
    return min(x, y)


def red_wrap(v, n):
    return v % (1 << n)


def red_sat(v, n):
    hi = (1 << n) - 1
    return 0 if v < 0 else (hi if v > hi else v)


def terms_mixed(nleaves):
    """Every term at `nleaves` leaf slots over {add, sub, mul, min}, with every
    leaf identification, reusing p1's enumerator with the signature widened."""
    old = p1.OPS
    p1.OPS = ALL_OPS
    try:
        return p1.all_terms(nleaves)
    finally:
        p1.OPS = old


def is_ring(op):
    return op in RING


# --------------------------------------------------------------------- arms


def ev_every_node(t, env, n, red):
    if t[0] == "leaf":
        return env[t[1]]
    x = ev_every_node(t[1], env, n, red)
    y = ev_every_node(t[2], env, n, red)
    return red(apply_op(t[0], x, y), n)


def ev_cut(t, env, n, red, cut_at_ring=False):
    """Reduce the operands of every non-ring node, and reduce at the root.
    `cut_at_ring` is C2's mutation: cut at the ring nodes instead."""

    def go(node):
        if node[0] == "leaf":
            return env[node[1]]
        x = go(node[1])
        y = go(node[2])
        want_cut = (not is_ring(node[0])) if not cut_at_ring else is_ring(node[0])
        if want_cut:
            x, y = red(x, n), red(y, n)
        return apply_op(node[0], x, y)

    return red(go(t), n)


def ev_root_only(t, env, n, red):
    def go(node):
        if node[0] == "leaf":
            return env[node[1]]
        return apply_op(node[0], go(node[1]), go(node[2]))

    return red(go(t), n)


def count_reductions(t):
    if t[0] == "leaf":
        return 0, 0
    lc, ln = count_reductions(t[1])
    rc, rn = count_reductions(t[2])
    nodes = 1 + ln + rn
    cuts = lc + rc + (0 if is_ring(t[0]) else 2)
    return cuts, nodes


# -------------------------------------------------------------------- sweep


def sweep(terms, n, red, label, decls_per_term, seed):
    rng = random.Random(seed)
    cells = d_cut = d_root = d_mut = 0
    mixed = 0
    for t in terms:
        k = max(p1.leaves(t)) + 1
        has_nonring = any(not is_ring(nd[0]) for nd in p1.internal(t))
        mixed += 1 if has_nonring else 0
        hi = (1 << n) - 1
        for _ in range(decls_per_term):
            bs = [rng.randint(0, hi) for _ in range(k)]
            doms = [range(0, b + 1) for b in bs]
            cells += 1
            bad_cut = bad_root = bad_mut = False
            for env in product(*doms):
                base = ev_every_node(t, env, n, red)
                if ev_cut(t, env, n, red) != base:
                    bad_cut = True
                if ev_root_only(t, env, n, red) != base:
                    bad_root = True
                if ev_cut(t, env, n, red, cut_at_ring=True) != base:
                    bad_mut = True
                if bad_cut and bad_root and bad_mut:
                    break
            d_cut += bad_cut
            d_root += bad_root
            d_mut += bad_mut
    print(
        f"  {label:<34} terms {len(terms):>4} ({mixed} mixed)  cells {cells:>6}  "
        f"CUT differs {d_cut:>6}  root-only differs {d_root:>6}  "
        f"[C2 wrong-cut differs {d_mut:>6}]",
        flush=True,
    )
    return d_cut, d_root, d_mut


def main():
    print("=" * 120)
    print("p11. Cutting the chain at the non-ring operations")
    print("=" * 120)
    print()

    t2 = terms_mixed(2)
    t3 = terms_mixed(3)
    mixed_only = [t for t in t2 + t3 if any(not is_ring(nd[0]) for nd in p1.internal(t))]
    ring_only = [t for t in t2 + t3 if all(is_ring(nd[0]) for nd in p1.internal(t))]

    print("P1 and P3: WRAPPING map, mixed signature {add, sub, mul, min}")
    print()
    for n in (3, 4):
        sweep(t2 + t3, n, red_wrap, f"all terms, logical width {n}", 20, 7)
        sweep(mixed_only, n, red_wrap, f"terms WITH a min, width {n}", 20, 7)

    print()
    print("C3: terms with no non-ring operation, where the cut rule and arm W0")
    print("are the same rule and must agree")
    print()
    for n in (3, 4):
        sweep(ring_only, n, red_wrap, f"ring-only terms, width {n}", 20, 7)

    print()
    print("C1 / P2: the same measurement under a SATURATING map")
    print()
    for n in (3, 4):
        sweep(t2 + t3, n, red_sat, f"all terms, logical width {n}", 20, 7)
        sweep(mixed_only, n, red_sat, f"terms WITH a min, width {n}", 20, 7)

    print()
    print("P4: how many reductions each arm performs")
    print()
    print(f"  {'term':<34} {'nodes':>7} {'general':>9} {'cut rule':>9}")
    fold = ("leaf", 0)
    for i in range(1, 6):
        fold = ("add", fold, ("leaf", i))
    chunked = ("min", fold, ("leaf", 6))
    for name, t in (
        ("x + y", ("add", ("leaf", 0), ("leaf", 1))),
        ("min(x + y, z)", ("min", ("add", ("leaf", 0), ("leaf", 1)), ("leaf", 2))),
        ("fold of 6 adds", fold),
        ("min(fold of 6 adds, z)", chunked),
    ):
        cuts, nodes = count_reductions(t)
        print(f"  {name:<34} {nodes:>7} {nodes:>9} {cuts + 1:>9}")
    print()
    print("  The last row is the shipped kernel's shape: one chunk of wrapping")
    print("  additions terminated by a clamp. The general arm reduces at every")
    print("  node; the cut rule reduces the clamp's two operands and the root.")

    print()
    print("=" * 120)
    print(
        """
  P1 holds when 'CUT differs' is zero on every wrapping row INCLUDING the
  mixed-only rows, which is where the cut is doing work.

  P3 holds when 'root-only differs' is nonzero on the mixed rows, which is what
  says the cut is necessary rather than decorative.

  C1 holds when the saturating rows show the cut rule differing.
  C2 holds when the wrong-cut mutation differs under wrap.
  C3 holds when the ring-only rows show the cut rule agreeing, since with no
  non-ring node the cut rule reduces only at the root and is arm W0 exactly.
"""
    )


if __name__ == "__main__":
    main()
