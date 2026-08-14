#!/usr/bin/env python3
"""q4. Diagnosing what q3 refuted three times, with witnesses rather than a
fourth hypothesis.

WHAT q3 REFUTED
---------------
At `F > 0` under wrap the map stops being a homomorphism for multiplication
(`116` F116-7, reproduced). q3 asked what the cut rule has to do about that and
made three predictions, all wrong:

  P4  reduce the multiplication's OPERANDS and the root          25, 25, 21 differ
  P5  reduce its RESULT instead                                  26, 26, 12 differ
  P7  requantise at it, deferring only the range part            41, 36, 15 differ
  P8  the same under FLOOR rather than truncation toward zero    41, 36, 15 differ

and one arrangement that works: reduce **both** its operands and its result,
0 differing on every wrapping row.

Three wrong hypotheses in a row is the point at which guessing a fourth is worse
than instrumenting, so this probe stops proposing and finds the smallest cell of
each kind, prints every intermediate, and reads the mechanism off it.

WHAT IS BEING ASKED
-------------------
`cut@operands` and `cut@result` each fail on cells the other may or may not fail
on. If the failing sets are **different**, there are two distinct mechanisms and
"reduce both" is not one repair but two. If they are the **same** set, one
mechanism is being fixed twice and the honest rule is smaller than "both".

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. The two failing sets are different, and neither contains the other. That is
    what "both are needed" means and it has not been checked.
P2. A cell failing only `cut@operands` has a multiplication whose result is out
    of range, so deferring the range reduction past a later operation changes it.
P3. A cell failing only `cut@result` has a multiplication whose operand is out of
    range, so the product itself is computed from the wrong value.
P4. Neither mechanism needs subtraction or a negative value, so both witnesses
    exist on an unsigned base with only addition and multiplication.

NEGATIVE CONTROLS
-----------------
C1. At `F = 0` both failing sets must be empty, because the map is a
    homomorphism there and no cut is needed at all.
C2. The `cut@both` arm must agree on every cell reported, or the witnesses are
    not witnesses for the claim that both halves fix it.
C3. A cell where all three arms agree must exist and be printed, so the sweep is
    not one where everything fails.
"""

from fractions import Fraction
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
q3 = load("q3_local", HERE / "q3_the_fraction_width_splits_my_arms_too.py")
Fx, ex, ev_every_node, ev_cut, ev_exact = q3.Fx, q3.ex, q3.ev_every_node, q3.ev_cut, q3.ev_exact


SYM = {"add": "+", "sub": "-", "mul": "*"}


def show(t):
    if t[0] == "leaf":
        return "xyzw"[t[1]]
    return f"({show(t[1])} {SYM[t[0]]} {show(t[2])})"


def trace(P, t, env, cutset, where):
    """Every node's value under one arm, bottom up, as (label, value, in range)."""
    out = []

    def go(node):
        if node[0] == "leaf":
            return env[node[1]]
        x, y = go(node[1]), go(node[2])
        if node[0] in cutset and where in ("operands", "both"):
            x, y = P.R(x), P.R(y)
        v = ex(node[0], x, y)
        if node[0] in cutset and where in ("result", "both"):
            v = P.R(v)
        out.append((show(node), x, y, v, P.lo <= v <= P.hi))
        return v

    r = P.R(go(t))
    return r, out


def trace_general(P, t, env):
    out = []

    def go(node):
        if node[0] == "leaf":
            return env[node[1]]
        x, y = go(node[1]), go(node[2])
        v = P.R(ex(node[0], x, y))
        out.append((show(node), x, y, v, True))
        return v

    return go(t), out


def find(P, terms, want):
    """want(op_ok, res_ok, both_ok) -> bool. Returns the first cell matching."""
    for t in terms:
        k = max(p1.leaves(t)) + 1
        for bs in product(range(0, min(6, P.khi) + 1), repeat=k):
            doms = [[v * P.step for v in range(0, b + 1)] for b in bs]
            if any(len(d) == 0 for d in doms):
                continue
            op_ok = res_ok = both_ok = True
            bad_env = None
            for tup in product(*doms):
                env = dict(enumerate(tup))
                base = ev_every_node(P, t, env)
                a = ev_cut(P, t, env, frozenset({"mul"}), "operands") == base
                b = ev_cut(P, t, env, frozenset({"mul"}), "result") == base
                c = ev_cut(P, t, env, frozenset({"mul"}), "both") == base
                if not a:
                    op_ok = False
                if not b:
                    res_ok = False
                if not c:
                    both_ok = False
                if bad_env is None and not (a and b):
                    bad_env = env
            if want(op_ok, res_ok, both_ok):
                return t, bs, bad_env
    return None


def report(P, t, bs, env, title):
    print()
    print(f"  {title}")
    print(f"    term {show(t)}   declared " +
          ", ".join(f"{'xyzw'[i]} in [0, {b * P.step}]" for i, b in enumerate(bs)))
    if env is None:
        print("    (no disagreeing tuple recorded)")
        return
    print(f"    values " + ", ".join(f"{'xyzw'[i]} = {v}" for i, v in sorted(env.items())))
    print(f"    container [{P.lo}, {P.hi}], grid step {P.step}")
    base, gtr = trace_general(P, t, env)
    print(f"    reduce at every node   -> {base}")
    for lbl, x, y, v, _ in gtr:
        print(f"        {lbl:<16} {x} , {y}  ->  {v}")
    for where in ("operands", "result", "both"):
        r, tr = trace(P, t, env, frozenset({"mul"}), where)
        mark = "  AGREES" if r == base else "  DIFFERS"
        print(f"    cut at the mul's {where:<9} -> {r}{mark}")
        for lbl, x, y, v, inr in tr:
            flag = "" if inr else "   <- out of range"
            print(f"        {lbl:<16} {x} , {y}  ->  {v}{flag}")


def main():
    print("=" * 96)
    print("q4. Two witnesses for why the cut needs both halves")
    print("=" * 96)

    P = Fx(4, 1, False, "wrap")
    terms = [t for t in p1.all_terms(2) + p1.all_terms(3)
             if any(n[0] == "mul" for n in p1.internal(t))]
    nosub = [t for t in terms if not any(n[0] == "sub" for n in p1.internal(t))]

    print()
    print(f"  primitive {P.label()}, {len(terms)} terms containing a multiplication,")
    print(f"  {len(nosub)} of them with no subtraction (P4's space)")

    # -------------------------------------------------------------------- P1
    print()
    print("-" * 96)
    print("P1. Are the two failing sets different?")
    print()
    only_op = only_res = both_fail = neither = 0
    for t in terms:
        k = max(p1.leaves(t)) + 1
        for bs in product(range(0, min(6, P.khi) + 1), repeat=k):
            doms = [[v * P.step for v in range(0, b + 1)] for b in bs]
            op_ok = res_ok = True
            for tup in product(*doms):
                env = dict(enumerate(tup))
                base = ev_every_node(P, t, env)
                if ev_cut(P, t, env, frozenset({"mul"}), "operands") != base:
                    op_ok = False
                if ev_cut(P, t, env, frozenset({"mul"}), "result") != base:
                    res_ok = False
                if not op_ok and not res_ok:
                    break
            if op_ok and res_ok:
                neither += 1
            elif op_ok and not res_ok:
                only_res += 1
            elif res_ok and not op_ok:
                only_op += 1
            else:
                both_fail += 1
    print(f"    cells where both arms agree with the general arm : {neither}")
    print(f"    cells where only cut@operands fails              : {only_op}")
    print(f"    cells where only cut@result   fails              : {only_res}")
    print(f"    cells where both fail                            : {both_fail}")
    print()
    print("    P1 holds when the two 'only' counts are both nonzero, which is what")
    print("    'each half is needed' means and what nothing had checked.")

    # ---------------------------------------------------------------- P2, P3
    print()
    print("-" * 96)
    print("P2 and P3. The witnesses, with every intermediate printed.")

    print()
    print("  P4 asked whether both witnesses exist without a subtraction. Both")
    print("  spaces are searched, the subtraction-free one first, so the answer is")
    print("  visible rather than assumed.")
    for space, sname in ((nosub, "no subtraction"), (terms, "all terms")):
        w = find(P, space, lambda o, r, b: (not o) and r and b)
        if w:
            report(P, *w, f"[{sname}] only cut@operands fails, so the RESULT "
                          f"reduction is needed:")
            break
        print(f"\n  [{sname}] no cell found where only cut@operands fails")

    for space, sname in ((nosub, "no subtraction"), (terms, "all terms")):
        w = find(P, space, lambda o, r, b: o and (not r) and b)
        if w:
            report(P, *w, f"[{sname}] only cut@result fails, so the OPERAND "
                          f"reduction is needed:")
            break
        print(f"\n  [{sname}] no cell found where only cut@result fails")

    # -------------------------------------------------------------------- C3
    w = find(P, nosub, lambda o, r, b: o and r and b)
    if w:
        t, bs, _ = w
        print()
        print(f"  C3. A cell where all three arms agree exists: {show(t)} declared " +
              ", ".join(f"[0, {b * P.step}]" for b in bs))

    # -------------------------------------------------------------------- C1
    print()
    print("-" * 96)
    print("C1. At F = 0 no cut is needed at all.")
    print()
    P0 = Fx(4, 0, False, "wrap")
    bad_op = bad_res = cells = 0
    for t in terms:
        k = max(p1.leaves(t)) + 1
        for bs in product(range(0, 5), repeat=k):
            doms = [[Fraction(v) for v in range(0, b + 1)] for b in bs]
            cells += 1
            for tup in product(*doms):
                env = dict(enumerate(tup))
                base = ev_every_node(P0, t, env)
                if ev_cut(P0, t, env, frozenset({"mul"}), "operands") != base:
                    bad_op += 1
                    break
            for tup in product(*doms):
                env = dict(enumerate(tup))
                base = ev_every_node(P0, t, env)
                if ev_cut(P0, t, env, frozenset({"mul"}), "result") != base:
                    bad_res += 1
                    break
    print(f"    {P0.label()}: {cells} cells, cut@operands fails on {bad_op}, "
          f"cut@result fails on {bad_res}")

    print()
    print("=" * 96)
    print(
        """
  WHAT TO READ OFF THE WITNESSES

  The intermediates are printed for the general arm and for all three cut
  arrangements, with out-of-range values flagged. The mechanism is whatever
  differs between the arm that agrees and the arm that does not, and it is meant
  to be read rather than asserted, because three asserted mechanisms have already
  been wrong on this question.
"""
    )


if __name__ == "__main__":
    main()
