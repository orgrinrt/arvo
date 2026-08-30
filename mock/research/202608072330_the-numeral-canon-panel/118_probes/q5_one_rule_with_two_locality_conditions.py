#!/usr/bin/env python3
"""q5. The rule q4's witnesses hand over, and whether it covers both settings.

WHAT q4 ESTABLISHED
-------------------
At `F > 0` under wrap, `cut@operands` and `cut@result` fail on **disjoint** sets
of cells, 660 and 648 with an overlap of 0, so "reduce both" is not one repair
applied twice. It is two repairs, and each witness names its own mechanism.

  Only `cut@operands` fails, on `x - (x * x)` at `x = 1/2`, grid step `1/2`:
  the product is `1/4`, which is OFF THE GRID. Reducing at every node quantises
  it to `0` and the subtraction gives `1/2`. Deferring the quantisation carries
  `1/4` into the subtraction, and `trunc(1/2 - 1/4) = 0` while
  `1/2 - trunc(1/4) = 1/2`. **Quantisation does not commute with the operations
  above it.**

  Only `cut@result` fails, on `x * (x - y)` at `x = 1/2, y = 1`: the inner
  difference is `-1/2`, OUT OF RANGE. Reducing at every node wraps it to `15/2`
  and the product is `15/4 -> 7/2`. Deferring the range reduction multiplies by
  `-1/2` instead and gets `0`. **The range reduction cannot be deferred through
  an operation that is not a homomorphism for it.**

THE RULE THOSE TWO HAND OVER
-----------------------------
A reduction is a composition of a **grid part** and a **range part**, which is
`112` F112-4's decomposition, and each part has its own locality condition:

  the GRID part must be applied at the RESULT of every node whose exact result
  can leave the grid;

  the RANGE part must be applied at the OPERANDS of every node for which the map
  is not a homomorphism;

  everywhere else, both may be deferred to the root.

If that is right it unifies two results that currently look unrelated. `114`
p11's cut rule at `F = 0` over `{add, sub, mul, min}` cuts at the non-ring
operations and cuts at their **operands** only, which this rule predicts exactly:
at `F = 0` nothing leaves the grid, so the grid condition is vacuous, and `min`
is the only non-homomorphism, so its operands take the range part. And q3's
`F > 0` case takes both parts at the multiplication.

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. Setting A, `F = 0` over `{add, sub, mul, min}` under wrap: the two-part rule
    equals reducing at every node, and it reduces at strictly fewer places than
    the general arm. This is `114` p11's result arriving as a special case.
P2. Setting B, `F > 0` over `{add, sub, mul}` under wrap: the two-part rule
    equals reducing at every node.
P3. Setting C, `F > 0` over `{add, sub, mul, min}` under wrap, which no probe in
    this sitting has run: the two-part rule equals reducing at every node with
    the range part at both `mul` and `min` and the grid part at `mul` alone.
P4. Under saturation the rule fails in every setting, because the range part has
    no node it may be deferred past.

NEGATIVE CONTROLS
-----------------
C1. Dropping the grid condition must break setting B and C and NOT break setting
    A, because at `F = 0` the grid condition is vacuous. If it breaks A too, the
    condition is not the one named.
C2. Dropping the range condition must break every setting including A.
C3. The saturating rows are P4 and are the control that the rule is a fact about
    the map rather than about where reductions are placed.
C4. Each setting must contain cells where the exact result leaves the range and
    cells where an intermediate leaves the grid, or the conditions are being
    measured where they cannot fire. Both counts are reported.
"""

from fractions import Fraction
from itertools import product
import importlib.util
import random
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
q3 = load("q3_local", HERE / "q3_the_fraction_width_splits_my_arms_too.py")
Fx = q3.Fx

RING = ("add", "sub", "mul")
ALL_OPS = ("add", "sub", "mul", "min")


def ex(op, a, b):
    if op == "add":
        return a + b
    if op == "sub":
        return a - b
    if op == "mul":
        return a * b
    return min(a, b)


def quantise(P, q):
    k = int(q / P.step) if q >= 0 else -int(-q / P.step)
    return k * P.step


def reduce_range(P, q):
    """The range part alone. Assumes its argument is already on the grid."""
    k = int(q / P.step) if q >= 0 else -int(-q / P.step)
    span = P.khi - P.klo + 1
    if P.policy == "sat":
        k = min(max(k, P.klo), P.khi)
    else:
        k = P.klo + (k - P.klo) % span
    return k * P.step


def full_R(P, q):
    return reduce_range(P, quantise(P, q))


# ---------------------------------------------------- what the map is, per op


def leaves_grid(P, op):
    """Can this operation's exact result leave the grid?"""
    return P.F > 0 and op == "mul"


def is_hom_for(P, op):
    """Is the map a homomorphism for this operation, here?"""
    if P.policy != "wrap":
        return False
    if op == "min":
        return False
    return not leaves_grid(P, op)


# ------------------------------------------------------------------ the arms


def ev_every_node(P, t, env):
    if t[0] == "leaf":
        return env[t[1]]
    return full_R(P, ex(t[0], ev_every_node(P, t[1], env), ev_every_node(P, t[2], env)))


def ev_exact(t, env):
    if t[0] == "leaf":
        return env[t[1]]
    return ex(t[0], ev_exact(t[1], env), ev_exact(t[2], env))


def ev_two_part(P, t, env, use_grid=True, use_range=True, track=True):
    """The rule, with settledness tracked so a value that is already on the grid
    and in range is not reduced again.

    Skipping is exactly equal rather than an approximation, because `full_R` is
    idempotent: a settled value is its own reduction. The first version of this
    probe did not track it and reduced both operands of every non-homomorphic
    node unconditionally, which made the rule look more expensive than the
    general arm at `F > 0`. That was a defect in the instrument rather than a
    property of the rule, and the untracked form is kept behind `track=False` so
    the two counts can be compared."""
    counts = {"grid": 0, "range": 0}

    def go(node):
        """Returns (value, settled)."""
        if node[0] == "leaf":
            return env[node[1]], True
        x, sx = go(node[1])
        y, sy = go(node[2])
        settled = False
        if use_range and not is_hom_for(P, node[0]):
            if not sx or not track:
                x = full_R(P, x)
                counts["range"] += 1
            if not sy or not track:
                y = full_R(P, y)
                counts["range"] += 1
        v = ex(node[0], x, y)
        if use_grid and leaves_grid(P, node[0]):
            v = quantise(P, v)
            counts["grid"] += 1
        return v, settled

    v, settled = go(t)
    if not settled:
        counts["range"] += 1
    return full_R(P, v), counts


def nodes_of(t):
    return len(p1.internal(t))


def terms_over(ops, slots):
    old = p1.OPS
    p1.OPS = ops
    try:
        return p1.all_terms(slots)
    finally:
        p1.OPS = old


def run(P, terms, label, decls_per_term=3, seed=20260814):
    rng = random.Random(seed)
    cells = 0
    diff_full = diff_nogrid = diff_norange = 0
    root_out = off_grid = 0
    red_general = red_rule = 0
    for t in terms:
        k = max(p1.leaves(t)) + 1
        nn = nodes_of(t)
        for _ in range(decls_per_term):
            bs = [rng.choice(list(P.raws())) for _ in range(k)]
            doms = [[v * P.step for v in range(0, b + 1)] for b in bs]
            if any(len(d) == 0 for d in doms):
                continue
            cells += 1
            bad_f = bad_g = bad_r = False
            ro = og = False
            for tup in product(*doms):
                env = dict(enumerate(tup))
                base = ev_every_node(P, t, env)
                vf, cnt = ev_two_part(P, t, env)
                if vf != base:
                    bad_f = True
                if ev_two_part(P, t, env, track=False)[0] != base:
                    bad_f = True
                if ev_two_part(P, t, env, use_grid=False)[0] != base:
                    bad_g = True
                if ev_two_part(P, t, env, use_range=False)[0] != base:
                    bad_r = True
                e = ev_exact(t, env)
                if not (P.lo <= e <= P.hi):
                    ro = True
                for nd in p1.internal(t):
                    if ev_exact(nd, env) % P.step != 0:
                        og = True
                red_general += nn
                red_rule += cnt["grid"] + cnt["range"]
            diff_full += bad_f
            diff_nogrid += bad_g
            diff_norange += bad_r
            root_out += ro
            off_grid += og
    print(
        f"  {label:<34} {P.label():<14} {cells:>6} {root_out:>9} {off_grid:>9} "
        f"{diff_full:>10} {diff_nogrid:>14} {diff_norange:>15}",
        flush=True,
    )
    return red_general, red_rule


def main():
    print("=" * 118)
    print("q5. One rule with two locality conditions, over both settings")
    print("=" * 118)

    ring2 = terms_over(RING, 2) + terms_over(RING, 3)
    all2 = terms_over(ALL_OPS, 2) + terms_over(ALL_OPS, 3)
    print()
    print(f"  terms over the ring operations: {len(ring2)}")
    print(f"  terms over the ring operations plus min: {len(all2)}")
    print()
    print(f"  {'setting':<34} {'primitive':<14} {'cells':>6} {'root out':>9} "
          f"{'off grid':>9} {'RULE fails':>10} {'no grid part':>14} "
          f"{'no range part':>15}")

    savings = {}
    print()
    print("  P1 and C1. Setting A: F = 0 over {add, sub, mul, min}, where the")
    print("  grid condition is vacuous and only the range condition can fire.")
    savings["A"] = run(Fx(4, 0, False, "wrap"), all2, "A. F=0, ring + min")
    run(Fx(4, 0, False, "wrap"), ring2, "A'. F=0, ring only")

    print()
    print("  P2. Setting B: F > 0 over the ring, where both conditions fire.")
    savings["B1"] = run(Fx(4, 1, False, "wrap"), ring2, "B. F=1, ring only")
    savings["B2"] = run(Fx(4, 2, False, "wrap"), ring2, "B. F=2, ring only")
    run(Fx(3, 2, True, "wrap"), ring2, "B. F=2, ring only, signed")

    print()
    print("  P3. Setting C: F > 0 over the ring plus min, which nothing in this")
    print("  sitting has run.")
    savings["C"] = run(Fx(4, 1, False, "wrap"), all2, "C. F=1, ring + min")
    run(Fx(4, 2, False, "wrap"), all2, "C. F=2, ring + min")

    print()
    print("  P4 and C3. Saturating. P4 predicted the rule FAILS here. It does not,")
    print("  and the reason is worth recording rather than hiding: at `sat` no")
    print("  operation is a homomorphism, so the range condition fires at every")
    print("  node and the rule DEGENERATES to the general arm. It agrees because")
    print("  it is the same arm, which the savings table below is what shows.")
    savings["D0"] = run(Fx(4, 0, False, "sat"), ring2, "D. F=0, ring only, SAT")
    savings["D1"] = run(Fx(4, 1, False, "sat"), all2, "D. F=1, ring + min, SAT")

    print()
    print("-" * 118)
    print("What the rule saves, as a count of reductions rather than a timing.")
    print()
    print(f"  {'setting':<24} {'general arm':>14} {'two-part rule':>15} {'ratio':>8}")
    for name, (g, r) in savings.items():
        ratio = (g / r) if r else float("inf")
        print(f"  {name:<24} {g:>14} {r:>15} {ratio:>8.2f}")
    print()
    print("  A count of reductions performed, not a duration. What a reduction")
    print("  costs is unpriced and no bench ran.")

    # ---- Added after the first count, which showed the rule saving 1.60x at
    # ---- F = 0 and costing 0.94x at F > 0 over a term set that is more than
    # ---- half multiplications. The saving is a function of how many
    # ---- non-homomorphic nodes a term has, so it is a property of the SHAPE
    # ---- and averaging it over an enumeration hides which shapes it is for.
    print()
    print("-" * 118)
    print("Where the saving actually lives: reductions per term shape.")
    print("The enumeration above is more than half multiplication terms, which")
    print("is not what a consumer's hot path looks like. `114` section 6.4's")
    print("shipped kernel is a fold, so the fold is the shape to count.")
    print()

    def fold(n, op="add"):
        t = ("leaf", 0)
        for i in range(1, n):
            t = (op, t, ("leaf", i))
        return t

    def count_only(P, t, env):
        _, c = ev_two_part(P, t, env)
        return c["grid"] + c["range"]

    shapes = [
        ("fold of 4 adds", fold(4)),
        ("fold of 8 adds", fold(8)),
        ("fold of 16 adds", fold(16)),
        ("fold of 8 adds, one mul at the end",
         ("mul", fold(8), ("leaf", 8))),
        ("fold of 8 adds, clamped at the end",
         ("min", fold(8), ("leaf", 8))),
        ("fold of 4 muls", fold(4, "mul")),
    ]
    print(f"  {'shape':<38} {'nodes':>6}  " +
          "  ".join(f"{lbl:>18}" for lbl in ("F=0 wrap", "F=1 wrap", "F=0 sat")))
    print(f"  {'':<38} {'':>6}  " +
          "  ".join(f"{'general / rule':>18}" for _ in range(3)))
    for name, t in shapes:
        k = max(p1.leaves(t)) + 1
        nn = nodes_of(t)
        cols = []
        for P in (Fx(8, 0, False, "wrap"), Fx(8, 1, False, "wrap"),
                  Fx(8, 0, False, "sat")):
            env = {i: Fraction(1) for i in range(k)}
            r = count_only(P, t, env)
            cols.append(f"{nn:>8} / {r:<9}")
        print(f"  {name:<38} {nn:>6}  " + "  ".join(cols))
    print()
    print("  A count of reductions the arm performs on one evaluation of that")
    print("  shape. Not a duration, and what a reduction costs is unpriced.")

    print()
    print("=" * 118)
    print(
        """
  READING IT

  P1, P2 and P3 hold when 'RULE fails' is zero on every wrapping row, with
  'root out' and 'off grid' both nonzero so the conditions had something to fire
  on, which is C4.

  C1 is 'no grid part' against setting A. It must be ZERO on the A rows, because
  at F = 0 nothing leaves the grid and dropping a vacuous condition changes
  nothing, and NONZERO on the B and C rows.

  C2 is 'no range part'. It must be nonzero on every row including A.

  C3 is the saturating rows, where 'RULE fails' must be nonzero.
"""
    )


if __name__ == "__main__":
    main()
