#!/usr/bin/env python3
"""p3. `112` F112-24's domination, taken off its thirteen hand-picked rows and
onto the systematic term enumeration.

WHAT IS ESTABLISHED AND WHAT IS NOT
------------------------------------
`112` F112-24: a form over one-signed symbols with a corner cross-term bound
"dominates both the interval rule and the standard affine rule, and reaches the
enumerating oracle on every term shape swept. Thirteen rows, zero unsound,
thirteen of thirteen reaching the oracle, beaten on none."

Thirteen rows chosen by their author. `112` section 6 is itself the argument
that a hand-picked row set hides a boundary, and `111` section 19.1 found three
counterexamples to `112` F112-6 inside `112`'s own probe output. So the claim
worth testing is not whether the one-sided form is good, which it plainly is,
but whether "beaten on none" survives an enumeration nobody chose.

This matters to the design and not only to the record. `111` section 21 proposes
two arms under one predicate: the corner rule where the structural predicate
fires, the one-sided form elsewhere. That composition is only correct if the
one-sided form is at least as good as the corner rule everywhere the predicate
does NOT fire. If the corner rule beats it somewhere, the composition needs a
third case and the predicate is not the selector.

INSTRUMENTS REUSED RATHER THAN REBUILT
---------------------------------------
`112_probes/p7` supplies the corner rule, the symmetric affine rule and the
reachable-set oracle. `112_probes/p9` supplies the one-sided form. Both are
committed and both were mutation-tested by their author. Nothing here
reimplements them; the terms and the declarations are what is new.

Note on the ground truth. `112` reports against `node_ok_exact`, which asks
whether every reachable intermediate is inside the container. That is STRICTLY
STRONGER than the arms agreeing, and the two come apart under wrapping, where
`p2` establishes that an intermediate may leave the container and the root still
be right. Both are reported below and the gap between them is named.

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. Zero unsound for all three rules on every row. The rules over-approximate.
P2. The one-sided form is not beaten by the corner rule on any saturating row.
P3. It IS beaten somewhere under wrapping, because `p2` shows the ground truth
    there is a root property and every one of these rules is a per-node rule, so
    a rule that propagates tighter intermediate bounds has no advantage and the
    comparison is between two rules answering the wrong question.
P4. The symmetric affine rule is beaten often, which is `112` p9 reproduced.

NEGATIVE CONTROLS
-----------------
C1. The correlation-breaking mutation: rename the second occurrence of a
    repeated leaf. The one-sided form must fall back to the corner rule's count.
    If it does not, the cancellation is not what is producing its advantage.
C2. A deliberately unsound rule (halve every propagated radius) must be reported
    unsound, or the unsound counter is a dead branch.
"""

from fractions import Fraction
from itertools import product
import importlib.util
import random
import sys
from pathlib import Path

sys.setrecursionlimit(10000)

HERE = Path(__file__).parent
P112 = HERE.parent / "112_probes"


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    sys.modules[name] = mod
    spec.loader.exec_module(mod)
    return mod


p7 = load("p7", P112 / "p7_an_affine_grade_recovers_the_lost_licences.py")
p9 = load("p9", P112 / "p9_a_one_sided_form_attacks_the_affine_rules_weakness.py")
p1 = load(
    "p1_local",
    HERE / "p1_the_structural_predicate_on_a_systematic_term_enumeration.py",
)


# --------------------------------------------------------- term shape bridging
# p1 spells a node (op, a, b); p7 spells it ("op", op, a, b). Same tree.


def plabel(P):
    return f"{'i' if P.signed else 'u'}W{P.W}/{P.policy}"


def to_p7(t):
    if t[0] == "leaf":
        return p7.L("xyzw"[t[1]])
    return p7.O(t[0], to_p7(t[1]), to_p7(t[2]))


# ------------------------------------------------------------- C2's bad rule


def halved_ok(P, t, g):
    """A deliberately unsound rule: the corner rule with every interval shrunk
    toward its midpoint. It must be reported unsound."""

    def shrink(iv):
        lo, hi = iv
        mid = (lo + hi) / 2
        return (mid + (lo - mid) / 2, mid + (hi - mid) / 2)

    def walk(node, g):
        if node[0] == "leaf":
            return g[node[1]]
        a, b = walk(node[2], g), walk(node[3], g)
        cs = [p7.exact(node[1], x, y) for x in a for y in b]
        return shrink((min(cs), max(cs)))

    def ok(node):
        if node[0] == "leaf":
            lo, hi = g[node[1]]
            return P.lo <= lo and hi <= P.hi
        if not ok(node[2]) or not ok(node[3]):
            return False
        lo, hi = walk(node, g)
        return P.lo <= lo and hi <= P.hi

    return ok(t)


# --------------------------------------------------------------------- sweep


def sweep_term(P, t7, names, decls):
    out = dict(
        n=0,
        corner=0,
        symaff=0,
        one=0,
        oracle=0,
        agree=0,
        halved=0,
        c_uns=0,
        a_uns=0,
        o_uns=0,
        h_uns=0,
        one_lt_corner=0,
        one_lt_symaff=0,
    )
    for ext in decls:
        gi = {k: (Fraction(lo), Fraction(hi)) for k, (lo, hi) in zip(names, ext)}
        ga = {
            k: p7.aff_from_interval(Fraction(lo), Fraction(hi), k)
            for k, (lo, hi) in zip(names, ext)
        }
        go = {
            k: p9.one_from(Fraction(lo), Fraction(hi), k)
            for k, (lo, hi) in zip(names, ext)
        }
        doms = [[v for v in P.values() if lo <= v <= hi] for lo, hi in ext]
        if any(len(d) == 0 for d in doms):
            continue
        out["n"] += 1
        c = p7.corner_ok(P, t7, gi)
        a = p7.affine_ok(P, t7, ga)
        o = p9.one_ok(P, t7, go)
        h = halved_ok(P, t7, gi)
        oracle = True
        agree = True
        for tup in product(*doms):
            env = dict(zip(names, tup))
            if oracle and not p7.node_ok_exact(P, t7, env):
                oracle = False
            if agree and p7.eval_exact(t7, env) != p7.eval_general(P, t7, env):
                agree = False
            if not oracle and not agree:
                break
        out["corner"] += c
        out["symaff"] += a
        out["one"] += o
        out["halved"] += h
        out["oracle"] += oracle
        out["agree"] += agree
        if c and not agree:
            out["c_uns"] += 1
        if a and not agree:
            out["a_uns"] += 1
        if o and not agree:
            out["o_uns"] += 1
        if h and not agree:
            out["h_uns"] += 1
        if c and not o:
            out["one_lt_corner"] += 1
        if a and not o:
            out["one_lt_symaff"] += 1
    return out


def run(label, P, terms, decls_for, show_losers=True):
    tot = dict(
        n=0, corner=0, symaff=0, one=0, oracle=0, agree=0, halved=0,
        c_uns=0, a_uns=0, o_uns=0, h_uns=0, one_lt_corner=0, one_lt_symaff=0,
    )
    losers = []
    for t in terms:
        k = max(p1.leaves(t)) + 1
        names = sorted("xyzw"[i] for i in range(k))
        t7 = to_p7(t)
        r = sweep_term(P, t7, names, decls_for(k))
        for key in tot:
            tot[key] += r[key]
        if r["one_lt_corner"]:
            losers.append((p1.show(t), r["one_lt_corner"], r["corner"], r["one"]))
    print()
    print(f"  {label}   primitive {plabel(P)}   terms {len(terms)}")
    print(
        f"    cells {tot['n']}   arms agree {tot['agree']}   "
        f"reachable-set oracle {tot['oracle']}   (gap {tot['agree'] - tot['oracle']})"
    )
    print(
        f"    corner {tot['corner']}   symmetric affine {tot['symaff']}   "
        f"ONE-SIDED {tot['one']}"
    )
    print(
        f"    unsound: corner {tot['c_uns']}  symAff {tot['a_uns']}  "
        f"one-sided {tot['o_uns']}   [C2 halved rule: {tot['h_uns']}]"
    )
    print(
        f"    cells where the corner rule licenses and the one-sided form does "
        f"NOT: {tot['one_lt_corner']}"
    )
    print(
        f"    cells where the symmetric affine licenses and the one-sided form "
        f"does NOT: {tot['one_lt_symaff']}"
    )
    if show_losers and losers:
        print("    terms on which the one-sided form is beaten by the corner rule:")
        for name, cnt, c, o in sorted(losers, key=lambda r: -r[1])[:12]:
            print(f"      {name:<24} beaten on {cnt} cells   corner {c}  one-sided {o}")
    return tot


def main():
    print("=" * 78)
    print("p3. Does the one-sided form dominate off its thirteen chosen rows?")
    print("=" * 78)

    rng = random.Random(20260814)
    P3u = p7.Prim(3, 0, False, "sat")
    P3s = p7.Prim(3, 0, True, "sat")
    P3uw = p7.Prim(3, 0, False, "wrap")
    P3sw = p7.Prim(3, 0, True, "wrap")

    def one_sided(P):
        def f(k):
            hi = int(P.hi)
            return [tuple((0, b) for b in bs) for bs in product(range(0, hi + 1), repeat=k)]
        return f

    def two_endpoint(P, cap=None):
        def f(k):
            lo_, hi_ = int(P.lo), int(P.hi)
            per = [(a, b) for a in range(lo_, hi_ + 1) for b in range(a, hi_ + 1)]
            allc = list(product(per, repeat=k))
            if cap is not None and len(allc) > cap:
                return [tuple(c) for c in rng.sample(allc, cap)]
            return [tuple(c) for c in allc]
        return f

    terms2 = p1.all_terms(2)
    terms3 = p1.all_terms(3)

    print()
    print("-" * 78)
    print("SATURATING, one-sided declarations, every term at 2 and 3 leaf slots")
    run("exhaustive", P3u, terms2 + terms3, one_sided(P3u))
    run("exhaustive", P3s, terms2 + terms3, one_sided(P3s))

    print()
    print("-" * 78)
    print("SATURATING, TWO-ENDPOINT declarations")
    run("exhaustive, arity 2", P3u, terms2, two_endpoint(P3u))
    run("exhaustive, arity 2", P3s, terms2, two_endpoint(P3s))
    run("sampled 500 decls, arity 3", P3u, terms3, two_endpoint(P3u, 500))
    run("sampled 500 decls, arity 3", P3s, terms3, two_endpoint(P3s, 500))

    print()
    print("-" * 78)
    print("WRAPPING, where P3 predicts the comparison is between two rules")
    print("answering a question p2 shows the root already answers")
    run("exhaustive", P3uw, terms2 + terms3, one_sided(P3uw))
    run("exhaustive", P3sw, terms2 + terms3, one_sided(P3sw))

    print()
    print("-" * 78)
    print("SATURATING, arity 4, sampled terms and declarations")
    t4 = rng.sample(p1.all_terms(4), 80)
    P2u = p7.Prim(2, 0, False, "sat")
    run("sampled", P2u, t4, one_sided(P2u))

    # ----------------------------------------------------------------- C1
    print()
    print("-" * 78)
    print("C1. The correlation-breaking mutation, which must collapse the")
    print("one-sided form's advantage back onto the corner rule.")
    print()
    t_corr = p7.O("sub", p7.O("add", p7.L("x"), p7.L("y")), p7.L("y"))
    t_brok = p7.O("sub", p7.O("add", p7.L("x"), p7.L("y")), p7.L("y2"))
    for label, t7, names in (
        ("(x + y) - y   correlated", t_corr, ["x", "y"]),
        ("(x + y) - y2  broken", t_brok, ["x", "y", "y2"]),
    ):
        hi = int(P3u.hi)
        if len(names) == 2:
            decls = [((0, a), (0, b)) for a in range(hi + 1) for b in range(hi + 1)]
        else:
            decls = [
                ((0, a), (0, b), (0, b))
                for a in range(hi + 1)
                for b in range(hi + 1)
            ]
        r = sweep_term(P3u, t7, names, decls)
        print(
            f"  {label:<28} corner {r['corner']:>4}  one-sided {r['one']:>4}  "
            f"of {r['n']:>4} cells"
        )
    print()
    print("  If the broken row's one-sided count drops to the corner count, the")
    print("  cancellation is what produces the advantage rather than a looser check.")


if __name__ == "__main__":
    main()
