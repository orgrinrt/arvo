#!/usr/bin/env python3
"""p4. Attacking p3's result rather than reporting it.

WHAT p3 FOUND
-------------
`112` F112-24 reports the one-sided affine form "beaten on none" over thirteen
rows. Over the systematic enumeration it is beaten, and only in one place:

    uW3/sat, one-sided declarations       beaten on   0 cells
    iW3/sat, one-sided declarations       beaten on   0 cells
    uW3/sat, TWO-ENDPOINT declarations    beaten on  43 cells
    iW3/sat, TWO-ENDPOINT declarations    beaten on 593 cells   (arity 3)
    iW3/sat, TWO-ENDPOINT, arity 2        beaten on  92 cells, all on `x * y`

`112` F112-23 already records that every extent in that file is one-sided from
zero. It does not connect that limitation to F112-24, and the connection is the
whole finding: **F112-24's domination is a fact about one-sided declarations.**

THE MECHANISM, NAMED RATHER THAN LEFT AS A COUNT
-------------------------------------------------
A leaf declared `[lo, hi]` becomes `lo + (hi - lo) * e` with `e` in `[0, 1]`, so
the constant is the lower bound and the coefficient is non-negative. Multiplying
two such forms scales each side's coefficients by the OTHER side's constant. If
that constant is negative, the coefficient flips sign, and the form's interval
`[k + sum of negative coefficients, k + sum of positive]` spreads both ways.

Worked, at iW3 (container [-4, 3]) with `x` and `y` both declared `[-4, -1]`:

    x = -4 + 3e1,  y = -4 + 3e2
    x*y = 16 + (3 * -4) e1 + (3 * -4) e2 + cross,  cross in [0, 9]
        interval [16 - 12 - 12, 16 + 9] = [-8, 25]
    corner: [(-1)(-1), (-4)(-4)] = [1, 16]        exact

So the one-sided basis is exact when the constants are non-negative and loses
when a declared lower bound is negative and a multiplication is present. That is
a predicate, not a defect, and it is why `112`'s three straddling rows did not
catch it: a leaf declared `[-4, b]` with `b` positive has a large positive
coefficient and the loss is small, while a leaf declared entirely below zero has
no positive part at all.

THE ATTACK
----------
Both rules are sound over-approximations of the same reachable set at every
node, so their INTERSECTION at every node is also a sound over-approximation,
and it is at least as tight as either. That is strictly stronger than disjoining
the two verdicts, which `112` section 9 offers, because an intersection
propagates the tighter bound into the parent while a disjunction of verdicts
does not.

    hybrid(node) = corner(children's hybrid intervals) INTERSECT affine(node)

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. The hybrid is sound everywhere: zero unsound.
P2. The hybrid is beaten by neither the corner rule nor the one-sided form on
    any row, including the two-endpoint rows where the one-sided form loses.
P3. The hybrid strictly beats the disjunction of the two verdicts somewhere,
    because propagating the tighter bound is what the disjunction cannot do.
    If it never does, the intersection buys nothing over disjoining and the
    honest answer is that `112` section 9's clause was already right.
P4. Under wrap the hybrid buys nothing over the root-only check `p2`
    establishes, because there the ground truth is a root property.

NEGATIVE CONTROLS
-----------------
C1. The halved rule from p3, carried through, must be reported unsound.
C2. An "intersection" taken the wrong way round, the UNION of the two intervals,
    must be reported as licensing fewer cells, or the intersect/union direction
    is not doing anything and P2 is measuring a coincidence.
C3. On the one-sided-declaration rows the hybrid must EQUAL the one-sided form,
    since that is the region where p3 measures the one-sided form as unbeaten. A
    hybrid that beats it there would mean p3's row is wrong.
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


def plabel(P):
    return f"{'i' if P.signed else 'u'}W{P.W}/{P.policy}"


def to_p7(t):
    if t[0] == "leaf":
        return p7.L("xyzw"[t[1]])
    return p7.O(t[0], to_p7(t[1]), to_p7(t[2]))


# ------------------------------------------------------------------- the hybrid


def hybrid_walk(t, gi, go, combine):
    """Returns (interval, one-sided form) at this node. The interval is the
    corner rule applied to the CHILDREN'S combined intervals, then combined with
    the one-sided form's own interval by `combine`."""
    if t[0] == "leaf":
        return gi[t[1]], go[t[1]]
    (ia, oa) = hybrid_walk(t[2], gi, go, combine)
    (ib, ob) = hybrid_walk(t[3], gi, go, combine)
    cs = [p7.exact(t[1], x, y) for x in ia for y in ib]
    corner_iv = (min(cs), max(cs))
    if t[1] == "add":
        o = p9.one_add(oa, ob, 1)
    elif t[1] == "sub":
        o = p9.one_add(oa, ob, -1)
    else:
        o = p9.one_mul(oa, ob)
    return combine(corner_iv, o.interval()), o


def intersect(a, b):
    return (max(a[0], b[0]), min(a[1], b[1]))


def union(a, b):
    return (min(a[0], b[0]), max(a[1], b[1]))


def hybrid_ok(P, t, gi, go, combine=intersect):
    def ok(node):
        if node[0] == "leaf":
            lo, hi = gi[node[1]]
            return P.lo <= lo and hi <= P.hi
        if not ok(node[2]) or not ok(node[3]):
            return False
        iv, _ = hybrid_walk(node, gi, go, combine)
        return P.lo <= iv[0] and iv[1] <= P.hi
    return ok(t)


def halved_ok(P, t, g):
    def shrink(iv):
        lo, hi = iv
        mid = (lo + hi) / 2
        return (mid + (lo - mid) / 2, mid + (hi - mid) / 2)

    def walk(node):
        if node[0] == "leaf":
            return g[node[1]]
        a, b = walk(node[2]), walk(node[3])
        cs = [p7.exact(node[1], x, y) for x in a for y in b]
        return shrink((min(cs), max(cs)))

    def ok(node):
        if node[0] == "leaf":
            lo, hi = g[node[1]]
            return P.lo <= lo and hi <= P.hi
        if not ok(node[2]) or not ok(node[3]):
            return False
        lo, hi = walk(node)
        return P.lo <= lo and hi <= P.hi

    return ok(t)


def root_only(P, t, gi):
    for lo, hi in gi.values():
        if not (P.lo <= lo and hi <= P.hi):
            return False

    def walk(node):
        if node[0] == "leaf":
            return gi[node[1]]
        a, b = walk(node[2]), walk(node[3])
        cs = [p7.exact(node[1], x, y) for x in a for y in b]
        return (min(cs), max(cs))

    lo, hi = walk(t)
    return P.lo <= lo and hi <= P.hi


# --------------------------------------------------------------------- sweep

KEYS = (
    "n", "agree", "corner", "one", "hyb", "uni", "disj", "root",
    "c_uns", "o_uns", "h_uns", "u_uns", "halv_uns", "r_uns",
    "hyb_lt_corner", "hyb_lt_one", "hyb_gt_disj", "hyb_ne_one",
)


def sweep_term(P, t7, names, decls):
    out = {k: 0 for k in KEYS}
    for ext in decls:
        gi = {k: (Fraction(lo), Fraction(hi)) for k, (lo, hi) in zip(names, ext)}
        go = {
            k: p9.one_from(Fraction(lo), Fraction(hi), k)
            for k, (lo, hi) in zip(names, ext)
        }
        doms = [[v for v in P.values() if lo <= v <= hi] for lo, hi in ext]
        if any(len(d) == 0 for d in doms):
            continue
        out["n"] += 1
        c = p7.corner_ok(P, t7, gi)
        o = p9.one_ok(P, t7, go)
        h = hybrid_ok(P, t7, gi, go, intersect)
        u = hybrid_ok(P, t7, gi, go, union)
        d = c or o
        r = root_only(P, t7, gi)
        hv = halved_ok(P, t7, gi)
        agree = True
        for tup in product(*doms):
            env = dict(zip(names, tup))
            if p7.eval_exact(t7, env) != p7.eval_general(P, t7, env):
                agree = False
                break
        for key, val in (("corner", c), ("one", o), ("hyb", h), ("uni", u),
                         ("disj", d), ("root", r), ("agree", agree)):
            out[key] += int(val)
        for key, val in (("c_uns", c), ("o_uns", o), ("h_uns", h),
                         ("u_uns", u), ("halv_uns", hv), ("r_uns", r)):
            if val and not agree:
                out[key] += 1
        if c and not h:
            out["hyb_lt_corner"] += 1
        if o and not h:
            out["hyb_lt_one"] += 1
        if h and not d:
            out["hyb_gt_disj"] += 1
        if h != o:
            out["hyb_ne_one"] += 1
    return out


def run(label, P, terms, decls_for):
    tot = {k: 0 for k in KEYS}
    gains = []
    for t in terms:
        k = max(p1.leaves(t)) + 1
        names = sorted("xyzw"[i] for i in range(k))
        r = sweep_term(P, to_p7(t), names, decls_for(k))
        for key in KEYS:
            tot[key] += r[key]
        if r["hyb_gt_disj"]:
            gains.append((p1.show(t), r["hyb_gt_disj"], r["disj"], r["hyb"]))
    print()
    print(f"  {label}   primitive {plabel(P)}   terms {len(terms)}")
    print(
        f"    cells {tot['n']}   arms agree {tot['agree']}   "
        f"corner {tot['corner']}   one-sided {tot['one']}"
    )
    print(
        f"    disjunction of the two verdicts {tot['disj']}   "
        f"HYBRID (per-node intersection) {tot['hyb']}   root-only {tot['root']}"
    )
    print(
        f"    unsound: corner {tot['c_uns']}  one-sided {tot['o_uns']}  "
        f"hybrid {tot['h_uns']}  root-only {tot['r_uns']}"
    )
    print(
        f"    C1 halved rule unsound on {tot['halv_uns']};  "
        f"C2 union-instead-of-intersection licenses {tot['uni']} "
        f"and is unsound on {tot['u_uns']}"
    )
    print(
        f"    hybrid beaten by corner on {tot['hyb_lt_corner']}, "
        f"by one-sided on {tot['hyb_lt_one']}; "
        f"beats the disjunction on {tot['hyb_gt_disj']}"
    )
    print(f"    C3: hybrid differs from the one-sided form on {tot['hyb_ne_one']} cells")
    if gains:
        print("    terms where the per-node intersection beats disjoining verdicts:")
        for name, cnt, d, h in sorted(gains, key=lambda r: -r[1])[:8]:
            print(f"      {name:<24} +{cnt} cells   disjunction {d}  hybrid {h}")
    return tot


def main():
    print("=" * 78)
    print("p4. The per-node intersection against both rules and their disjunction")
    print("=" * 78)

    rng = random.Random(20260814)
    P3u = p7.Prim(3, 0, False, "sat")
    P3s = p7.Prim(3, 0, True, "sat")
    P3uw = p7.Prim(3, 0, False, "wrap")

    def one_sided(P):
        def f(k):
            hi = int(P.hi)
            return [tuple((0, b) for b in bs) for bs in product(range(hi + 1), repeat=k)]
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

    terms2, terms3 = p1.all_terms(2), p1.all_terms(3)

    print()
    print("-" * 78)
    print("C3's region: ONE-SIDED declarations, where p3 measures the one-sided")
    print("form as unbeaten. The hybrid must equal it here.")
    run("exhaustive", P3u, terms2 + terms3, one_sided(P3u))
    run("exhaustive", P3s, terms2 + terms3, one_sided(P3s))

    print()
    print("-" * 78)
    print("TWO-ENDPOINT declarations, where the one-sided form loses")
    run("exhaustive, arity 2", P3u, terms2, two_endpoint(P3u))
    run("exhaustive, arity 2", P3s, terms2, two_endpoint(P3s))
    run("sampled 500, arity 3", P3u, terms3, two_endpoint(P3u, 500))
    run("sampled 500, arity 3", P3s, terms3, two_endpoint(P3s, 500))

    print()
    print("-" * 78)
    print("P4: under wrap, does any of this buy anything over the root check?")
    run("exhaustive", P3uw, terms2 + terms3, one_sided(P3uw))
    run("sampled 500, arity 3, two-endpoint", P3uw, terms3, two_endpoint(P3uw, 500))

    print()
    print("=" * 78)
    print("READING IT")
    print("=" * 78)
    print(
        """
  P1 holds when every 'hybrid' unsound count is zero AND C1's halved rule is
  nonzero, so the counter is live.

  P2 holds when 'hybrid beaten by corner' and 'beaten by one-sided' are both
  zero on every row.

  P3 holds when 'beats the disjunction' is nonzero somewhere. If it is zero
  everywhere the intersection buys nothing a disjunction does not, and `112`
  section 9's clause needs no repair.

  C2 fires when the union variant licenses more cells than the intersection AND
  is reported unsound. A union of two sound over-approximations is not sound,
  so a zero there would mean the soundness counter is not reading this rule.

  C3 holds when 'hybrid differs from one-sided' is zero on the one-sided rows.
"""
    )


if __name__ == "__main__":
    main()
