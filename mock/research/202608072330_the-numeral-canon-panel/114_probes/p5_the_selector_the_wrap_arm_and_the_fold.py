#!/usr/bin/env python3
"""p5. Three things the composition needs and nobody has measured.

ONE. The wrap arm, finished.
`p2` establishes that under a wrapping realisation map over ring operations the
arms agree exactly when the ROOT's exact value is in range, so the discharge
check is a root property and every per-node rule is answering a question nobody
asked. `p4` then measures that a root-only check using the plain corner
propagation licenses FEWER cells than a per-node hybrid, which looks like a
contradiction and is not: the root-only check was using the loosest interval
available at the root. The rule that follows from `p2` is "check the root, using
the tightest interval you have", and nothing has measured it.

TWO. The selector's step counts.
`111` section 21 proposes two arms under one predicate. `p1` measures the
predicate is heavily incomplete, and that the disjunction `corner licenses OR
(a and b)` is sound and closes part of the gap. What a design actually needs is
how often each step of the selector is reached, and, at the last step, whether
the expensive form pays. That is the number that decides whether carrying the
expensive form is worth it, and it does not exist.

THREE. The fold, and the state claim at lengths other than 64.
`111` F111-18 states the corner rule carries two numbers on every term and an
affine form one coefficient per distinct leaf plus one per non-constant
multiplication, giving "2 against 64 on a 64-element fold". That is a count read
off a model rather than a measurement, and the load-bearing half is not the
count but whether the corner rule stays EXACT as the fold grows. Nobody has
checked that at any length, and it is the claim the whole cheap arm rests on.

PREDICTIONS
-----------
P1. Under wrap, checking the root with the hybrid interval is sound and licenses
    at least as many cells as any per-node rule, on every row.
P2. Under sat, the same rule is UNSOUND, which is what makes it an arm rather
    than a replacement.
P3. The selector reaches its expensive step on a minority of cells, and when it
    does the expensive form licenses something the cheap one refused on a
    minority of those. Both numbers are what price the arm.
P4. The corner rule is exact on a fold of adds at every length checked, and the
    structural predicate fires on every cell of every such fold, because a fold
    has distinct leaves and no multiplication. If either fails the cheap arm has
    no region.
P5. The state counts are 2 for the corner rule and n for a fold of n leaves,
    at every n, since neither depends on the width or the declaration.

NEGATIVE CONTROLS
-----------------
C1. The wrap root rule applied to a saturating primitive must go unsound.
C2. A fold whose leaves are deliberately identified (the same leaf n times) must
    make the structural predicate STOP firing, or P4's "fires on every cell" is
    a property of folds in general rather than of distinct leaves.
C3. The exactness check on the fold must be able to report a failure: the same
    check is run on a fold with a multiplication under it, where `p1`'s condition
    (b) predicts conservatism is possible.
"""

from fractions import Fraction
from itertools import product
import importlib.util
import random
import sys
from pathlib import Path

sys.setrecursionlimit(100000)

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
p4 = load("p4_local", HERE / "p4_the_per_node_intersection_beats_both_rules.py")

to_p7, plabel, intersect = p4.to_p7, p4.plabel, p4.intersect
hybrid_walk, hybrid_ok = p4.hybrid_walk, p4.hybrid_ok


def root_with(P, t, gi, go, which):
    """Check ONLY the root, using the named interval rule. `p2` says this is the
    right shape under wrap; C1 runs it under sat where it must fail."""
    for lo, hi in gi.values():
        if not (P.lo <= lo and hi <= P.hi):
            return False
    if which == "corner":
        iv = p7.corner(t, gi)
    elif which == "one":
        iv = p9.one_eval(t, go).interval()
    else:
        iv, _ = hybrid_walk(t, gi, go, intersect)
    return P.lo <= iv[0] and iv[1] <= P.hi


def arms_agree7(P, t7, names, doms):
    for tup in product(*doms):
        env = dict(zip(names, tup))
        if p7.eval_exact(t7, env) != p7.eval_general(P, t7, env):
            return False
    return True


# ------------------------------------------------------------------- ONE


def part_one(rng):
    print()
    print("=" * 78)
    print("ONE. The wrap arm: check the root, with the tightest interval")
    print("=" * 78)
    terms = p1.all_terms(2) + p1.all_terms(3)

    def two_endpoint(P, cap):
        lo_, hi_ = int(P.lo), int(P.hi)
        per = [(a, b) for a in range(lo_, hi_ + 1) for b in range(a, hi_ + 1)]

        def f(k):
            allc = list(product(per, repeat=k))
            return [tuple(c) for c in (rng.sample(allc, cap) if len(allc) > cap else allc)]

        return f

    def one_sided(P):
        def f(k):
            return [tuple((0, b) for b in bs)
                    for bs in product(range(int(P.hi) + 1), repeat=k)]
        return f

    print()
    print(
        f"  {'primitive':<12} {'decls':<12} {'cells':>7} {'agree':>7} "
        f"{'pernode hyb':>12} {'root+corner':>12} {'root+one':>9} {'ROOT+HYB':>9} "
        f"{'unsound':>8}"
    )
    for P, dl, decls_for in (
        (p7.Prim(3, 0, False, "wrap"), "one-sided", one_sided(p7.Prim(3, 0, False, "wrap"))),
        (p7.Prim(3, 0, True, "wrap"), "one-sided", one_sided(p7.Prim(3, 0, True, "wrap"))),
        (p7.Prim(3, 0, False, "wrap"), "two-endpoint", two_endpoint(p7.Prim(3, 0, False, "wrap"), 500)),
        (p7.Prim(3, 0, True, "wrap"), "two-endpoint", two_endpoint(p7.Prim(3, 0, True, "wrap"), 500)),
        (p7.Prim(3, 0, False, "sat"), "one-sided  [C1]", one_sided(p7.Prim(3, 0, False, "sat"))),
        (p7.Prim(3, 0, True, "sat"), "two-endpoint [C1]", two_endpoint(p7.Prim(3, 0, True, "sat"), 500)),
    ):
        cells = agree = hyb = rc = ro = rh = uns = 0
        for t in terms:
            k = max(p1.leaves(t)) + 1
            names = sorted("xyzw"[i] for i in range(k))
            t7 = to_p7(t)
            for ext in decls_for(k):
                gi = {n: (Fraction(a), Fraction(b)) for n, (a, b) in zip(names, ext)}
                go = {n: p9.one_from(Fraction(a), Fraction(b), n)
                      for n, (a, b) in zip(names, ext)}
                doms = [[v for v in P.values() if a <= v <= b] for a, b in ext]
                if any(len(d) == 0 for d in doms):
                    continue
                cells += 1
                ag = arms_agree7(P, t7, names, doms)
                agree += ag
                hyb += hybrid_ok(P, t7, gi, go, intersect)
                rc += root_with(P, t7, gi, go, "corner")
                ro += root_with(P, t7, gi, go, "one")
                r = root_with(P, t7, gi, go, "hyb")
                rh += r
                if r and not ag:
                    uns += 1
        print(
            f"  {plabel(P):<12} {dl:<12} {cells:>7} {agree:>7} {hyb:>12} "
            f"{rc:>12} {ro:>9} {rh:>9} {uns:>8}"
        )
    print()
    print("  The last column is the point. Zero on the wrap rows and nonzero on")
    print("  the [C1] saturating rows is the arm; a zero on the sat rows would")
    print("  mean the rule is universally sound and there is no arm to name.")


# ------------------------------------------------------------------- TWO


def part_two(rng):
    print()
    print("=" * 78)
    print("TWO. The selector's step counts")
    print("=" * 78)
    print(
        """
  The selector, in the order a design would evaluate it:

    step 1  the cheap rule licenses            -> cheap arm, 2 numbers per node
    step 2  else (a) and (b) hold              -> honest refusal, general arm
    step 3  else                               -> instantiate the expensive form

  step 3 is where the coefficient vector is paid for, and the question is how
  often it is reached and how often it pays when reached."""
    )
    terms = p1.all_terms(2) + p1.all_terms(3)
    print()
    print(
        f"  {'primitive':<12} {'decls':<13} {'cells':>7} {'step1':>7} {'step2':>7} "
        f"{'step3':>7} {'step3 pays':>11} {'lost':>6}"
    )

    def one_sided(P):
        return lambda k: [tuple((0, b) for b in bs)
                          for bs in product(range(int(P.hi) + 1), repeat=k)]

    def two_endpoint(P, cap):
        lo_, hi_ = int(P.lo), int(P.hi)
        per = [(a, b) for a in range(lo_, hi_ + 1) for b in range(a, hi_ + 1)]

        def f(k):
            allc = list(product(per, repeat=k))
            return [tuple(c) for c in (rng.sample(allc, cap) if len(allc) > cap else allc)]
        return f

    for P, dl, decls_for in (
        (p7.Prim(3, 0, False, "sat"), "one-sided", one_sided(p7.Prim(3, 0, False, "sat"))),
        (p7.Prim(3, 0, True, "sat"), "one-sided", one_sided(p7.Prim(3, 0, True, "sat"))),
        (p7.Prim(3, 0, False, "sat"), "two-endpoint", two_endpoint(p7.Prim(3, 0, False, "sat"), 400)),
        (p7.Prim(3, 0, True, "sat"), "two-endpoint", two_endpoint(p7.Prim(3, 0, True, "sat"), 400)),
    ):
        s1 = s2 = s3 = pays = lost = cells = 0
        for t in terms:
            k = max(p1.leaves(t)) + 1
            names = sorted("xyzw"[i] for i in range(k))
            t7 = to_p7(t)
            a_ok = p1.cond_a(t)
            for ext in decls_for(k):
                gi = {n: (Fraction(a), Fraction(b)) for n, (a, b) in zip(names, ext)}
                go = {n: p9.one_from(Fraction(a), Fraction(b), n)
                      for n, (a, b) in zip(names, ext)}
                doms = [[v for v in P.values() if a <= v <= b] for a, b in ext]
                if any(len(d) == 0 for d in doms):
                    continue
                cells += 1
                ext_i = tuple((int(a), int(b)) for a, b in ext)
                if p7.corner_ok(P, t7, gi):
                    s1 += 1
                    continue
                if a_ok and p1.cond_b(t, ext_i):
                    s2 += 1
                    continue
                s3 += 1
                if hybrid_ok(P, t7, gi, go, intersect):
                    pays += 1
                elif arms_agree7(P, t7, names, doms):
                    lost += 1
        print(
            f"  {plabel(P):<12} {dl:<13} {cells:>7} {s1:>7} {s2:>7} {s3:>7} "
            f"{pays:>11} {lost:>6}"
        )
    print()
    print("  step3 pays: cells reaching step 3 where the expensive form licenses.")
    print("  lost: cells reaching step 3 where no rule licenses and the arms agree")
    print("  anyway, which is the ceiling nothing per-node reaches.")


# ----------------------------------------------------------------- THREE


def _unused_fold(k, op="add"):
    t = ("leaf", 0)
    for i in range(1, k):
        t = (op, t, ("leaf", i))
    return t


def fold_same_leaf(k, op="add"):
    t = ("leaf", 0)
    for _ in range(1, k):
        t = (op, t, ("leaf", 0))
    return t


def sample_decls(rng, hi, k, want, lo=0):
    """Sample declarations WITHOUT materialising the product. The exhaustive
    list is (hi - lo + 1) ** k, which reaches four billion at k = 8."""
    space = (hi - lo + 1) ** k
    if space <= want:
        return [tuple((0, b) for b in bs)
                for bs in product(range(lo, hi + 1), repeat=k)]
    seen = set()
    while len(seen) < want:
        seen.add(tuple(rng.randint(lo, hi) for _ in range(k)))
    return [tuple((0, b) for b in bs) for bs in seen]


def fold_ops(k, ops):
    t = ("leaf", 0)
    for i in range(1, k):
        t = (ops[(i - 1) % len(ops)], t, ("leaf", i))
    return t


def fold_row(Pi, t, k, decls, label):
    cells = consv = fires = unsound = root_uns = 0
    for ext in decls:
        doms = [range(a, b + 1) for a, b in ext]
        if any(len(d) == 0 for d in doms):
            continue
        cells += 1
        lic = p1.corner_licenses(Pi, t, ext)
        ag = p1.arms_agree(Pi, t, ext, k)
        if ag and not lic:
            consv += 1
        if lic and not ag:
            unsound += 1
        lo, hi = p1.iv(t, ext)
        ro = Pi.lo <= lo and hi <= Pi.hi
        if ro and not ag:
            root_uns += 1
        if p1.cond_a(t) and p1.cond_b(t, ext):
            fires += 1
    ls = p1.leaves(t)
    nmul = sum(1 for nd in p1.internal(t) if nd[0] == "mul")
    print(
        f"  {label:<30} {cells:>6} {consv:>7} {unsound:>8} {fires:>7} "
        f"{root_uns:>9} {2:>7} {len(set(ls)) + nmul:>7}",
        flush=True,
    )
    return consv, unsound


def part_three(rng):
    print()
    print("=" * 78, flush=True)
    print("THREE. The fold: exactness and state at every length, not just 64")
    print("=" * 78, flush=True)
    Pi = p1.Prim(2, False, "sat")
    print()
    print(f"  primitive {Pi.label()}, declarations sampled without replacement")
    print(
        f"  {'term':<30} {'cells':>6} {'consv':>7} {'unsound':>8} "
        f"{'fires':>7} {'root uns':>9} {'corner':>7} {'affine':>7}",
        flush=True,
    )
    for n in (2, 3, 4, 5, 6, 7, 8):
        t = fold_ops(n, ["add"])
        fold_row(Pi, t, n, sample_decls(rng, Pi.hi, n, 60), f"fold of {n} adds")

    print()
    print("  Mixed add and subtract, where the spine is not monotone, so the root")
    print("  stops being the widest node and the per-node check has work to do:",
          flush=True)
    print()
    for n in (3, 4, 5, 6, 7, 8):
        t = fold_ops(n, ["add", "sub"])
        fold_row(Pi, t, n, sample_decls(rng, Pi.hi, n, 60),
                 f"fold of {n}, alternating +/-")

    print()
    print("  C2. The same fold with every leaf identified. The predicate must")
    print("  stop firing, or it is not condition (a) doing the work:", flush=True)
    print()
    for n in (2, 3, 4, 5, 6):
        t = ("leaf", 0)
        for _ in range(1, n):
            t = ("add", t, ("leaf", 0))
        decls = [((0, b),) for b in range(Pi.hi + 1)]
        fold_row(Pi, t, 1, decls, f"fold of {n} adds, one leaf")

    print()
    print("  C3. A fold under a multiplication, where condition (b) predicts")
    print("  conservatism is possible and the check must be able to report it:",
          flush=True)
    print()
    for n in (2, 3, 4, 5):
        inner = fold_ops(n, ["add"])
        t = ("mul", inner, ("leaf", n))
        fold_row(Pi, t, n + 1, sample_decls(rng, Pi.hi, n + 1, 60),
                 f"(fold of {n}) * z")

    print()
    print("  A wider container, to check the verdicts are not an artifact of W = 2:",
          flush=True)
    print()
    P3 = p1.Prim(3, False, "sat")
    for n in (2, 3, 4, 5):
        fold_row(P3, fold_ops(n, ["add"]), n,
                 sample_decls(rng, P3.hi, n, 40), f"fold of {n} adds  uW3")
        fold_row(P3, fold_ops(n, ["add", "sub"]), n,
                 sample_decls(rng, P3.hi, n, 40), f"fold of {n} +/-   uW3")

    print()
    print("  The state counts, extended past what any sweep can reach, because")
    print("  they are properties of the term and neither the width nor the")
    print("  declaration appears in them:", flush=True)
    print()
    print(f"  {'fold length':>12} {'corner state':>13} {'affine state':>13} {'ratio':>8}")
    for n in (2, 4, 8, 16, 64, 256, 1024):
        t = fold_ops(n, ["add"])
        ls = p1.leaves(t)
        nmul = sum(1 for nd in p1.internal(t) if nd[0] == "mul")
        aff = len(set(ls)) + nmul
        print(f"  {n:>12} {2:>13} {aff:>13} {aff / 2:>8.1f}", flush=True)


def main():
    rng = random.Random(20260814)
    part_one(rng)
    part_two(rng)
    part_three(rng)


if __name__ == "__main__":
    main()
