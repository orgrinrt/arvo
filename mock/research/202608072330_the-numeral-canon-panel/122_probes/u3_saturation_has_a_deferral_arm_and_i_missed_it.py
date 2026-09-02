#!/usr/bin/env python3
"""u3. `121` section 3 reproduced, and what it opens that no file has taken.

THE DISSENT
-----------
`119` 4.4 says "a saturating map is a homomorphism for no operation". `121` says
that is false on a non-negative domain, and offers three kinds of evidence: its
own `116_probes/p4_output.txt:13` printing `0/2116` for addition at `uW4F0/sat`,
my `118_probes/q3_output.txt:13` printing `720/2304` for the same cell, and a
shipped test, `warm-clamp-shared/src/lib.rs:1105`, asserting the identity
directly and passing in every count of this sitting.

The reconciliation `121` offers is that the saturating rows move with the ambient
span and the wrapping rows do not, so both files measured correctly and neither
predicate named the span.

**The clause is mine.** It came from reading my own q3's saturating column as a
control rather than as a finding, and my q3's ambient range runs from
`klo - span`, which is negative even for an unsigned primitive. So my column
measured saturation on a domain straddling zero and I wrote the verdict down as
though it were about saturation.

WHAT THIS OPENS, WHICH IS THE PART NO FILE HAS TAKEN
------------------------------------------------------
If saturation is a homomorphism for addition and multiplication on a non-negative
domain, then the deferral licence `119` 4.3 attributes to the homomorphism is
available under saturation too, on the region where the exact intermediates
cannot leave the domain's sign. That region is exactly "no subtraction over
non-negative declarations", and it is exactly the shape the shipped fold has.

Nothing in `114`, `116`, `118`, `119`, `120` or `121` measures whether arm W0 or
the root-only discharge check actually hold there. `121` names the structure as a
semiring homomorphism and stops. This probe takes it.

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. `121`'s reconciliation reproduces on my own instrument: the saturating
    homomorphism verdict flips with the ambient domain's sign and the wrapping
    one does not.
P2. Under saturation with non-negative declarations, arm W0 holds on terms with
    no subtraction and fails on terms with one. That is the same operation split
    the fraction width produced at `wrap`, arriving on a second axis.
P3. Under the same conditions the root-only discharge check is sound on
    subtraction-free terms and unsound on terms with subtraction.
P4. Every cell where the root-only check was unsound at `sat` in `118` q1, which
    is 38 and 34, is a term containing a subtraction. If any is subtraction-free,
    P2 and P3 are wrong.
P5. The shipped test's identity re-derives at the widths it sweeps.

NEGATIVE CONTROLS
-----------------
C1. The wrapping rows must be unaffected by the domain's sign, or the instrument
    is reading something other than the map.
C2. The subtraction-free saturating rows must contain cells where the exact
    result leaves the container, or a zero difference is a region where nothing
    could differ.
C3. A signed container with declarations that reach below zero must break the
    subtraction-free result too, because there the reachable set straddles zero
    without any subtraction being present. If it does not, "no subtraction" is
    the condition rather than "the reachable set keeps its sign", and the weaker
    statement is the one to write.
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


def ex(op, a, b):
    if op == "add":
        return a + b
    if op == "sub":
        return a - b
    return a * b


# ------------------------------------------------------------------- P1 and C1


def hom_failures(P, op, lo, hi):
    bad = tot = 0
    for a in range(lo, hi + 1):
        for b in range(lo, hi + 1):
            tot += 1
            if P.R(ex(op, P.R(a), P.R(b))) != P.R(ex(op, a, b)):
                bad += 1
    return bad, tot


# ------------------------------------------------------------ the arms, again


def ev_every_node(P, t, env):
    if t[0] == "leaf":
        return env[t[1]]
    return P.R(ex(t[0], ev_every_node(P, t[1], env), ev_every_node(P, t[2], env)))


def ev_exact(t, env):
    if t[0] == "leaf":
        return env[t[1]]
    return ex(t[0], ev_exact(t[1], env), ev_exact(t[2], env))


def ev_w0(P, t, env):
    return P.R(ev_exact(t, env))


def per_node(P, t, ext):
    return p1.corner_licenses(P, t, ext)


def root_only(P, t, ext):
    for lo, hi in ext:
        if not (P.lo <= lo and hi <= P.hi):
            return False
    lo, hi = iv(t, ext)
    return P.lo <= lo and hi <= P.hi


def arms_agree(P, t, ext, k):
    for env in tuples(ext, k):
        if ev(P, t, env, False) != ev(P, t, env, True):
            return False
    return True


def has(t, op):
    return any(nd[0] == op for nd in internal(t))


def one_sided(P, k):
    return [tuple((0, b) for b in bs) for bs in product(range(0, P.hi + 1), repeat=k)]


def two_sided(P, k):
    """Declarations that reach below zero, for C3."""
    lo = P.lo
    return [tuple((lo, b) for b in bs) for bs in product(range(0, P.hi + 1), repeat=k)]


def sweep(P, terms, decls_for, label):
    cells = w0_diff = ro_unsound = pn_unsound = out = 0
    for t in terms:
        k = max(leaves(t)) + 1
        for ext in decls_for(P, k):
            doms = [range(lo, hi + 1) for lo, hi in ext]
            if any(len(d) == 0 for d in doms):
                continue
            cells += 1
            ag = arms_agree(P, t, ext, k)
            bad = False
            leaves_range = False
            for env in tuples(ext, k):
                if ev_w0(P, t, env) != ev_every_node(P, t, env):
                    bad = True
                e = ev_exact(t, env)
                if not (P.lo <= e <= P.hi):
                    leaves_range = True
            w0_diff += bad
            out += leaves_range
            if root_only(P, t, ext) and not ag:
                ro_unsound += 1
            if per_node(P, t, ext) and not ag:
                pn_unsound += 1
    print(f"  {label:<44} {cells:>6} {out:>10} {w0_diff:>11} {ro_unsound:>12} "
          f"{pn_unsound:>13}", flush=True)
    return w0_diff, ro_unsound


def main():
    print("=" * 104)
    print("u3. Saturation has a deferral arm on a one-signed domain, and I missed it")
    print("=" * 104)

    # ------------------------------------------------------------- P1 and C1
    print()
    print("P1 and C1. The homomorphism verdict against the ambient domain's sign.")
    print("`118` q3's ambient range starts at `klo - span`, which is negative even")
    print("for an unsigned primitive, so its saturating column measured a")
    print("straddling domain. Both spans are run here.")
    print()
    print(f"  {'primitive':<12} {'ambient domain':<22} {'add':>13} {'sub':>13} {'mul':>13}")
    for P in (Prim(4, False, "sat"), Prim(4, False, "wrap")):
        span = P.hi - P.lo + 1
        for dlabel, lo, hi in (
            ("non-negative 0..45", 0, P.hi + span + 1),
            ("straddling -48..48", P.lo - span, P.hi + span + 1),
        ):
            cols = []
            for op in ("add", "sub", "mul"):
                bad, tot = hom_failures(P, op, lo, hi)
                cols.append(f"{bad:>5}/{tot:<7}")
            print(f"  {P.label():<12} {dlabel:<22} " + " ".join(cols))
    print()
    print("  C1 is the wrapping rows: identical on both domains. P1 is the")
    print("  saturating rows moving, which is `121`'s reconciliation reproduced.")

    # ------------------------------------------------------------- P2 and P3
    print()
    print("-" * 104)
    print("P2 and P3. Does the deferral licence exist under saturation?")
    print()
    terms = all_terms(2) + all_terms(3)
    nosub = [t for t in terms if not has(t, "sub")]
    withsub = [t for t in terms if has(t, "sub")]
    print(f"  terms: {len(terms)}, of which {len(nosub)} contain no subtraction")
    print()
    print(f"  {'setting':<44} {'cells':>6} {'exact out':>10} {'W0 differs':>11} "
          f"{'root unsound':>12} {'per-node uns':>13}")
    for P in (Prim(3, False, "sat"), Prim(4, False, "sat")):
        sweep(P, nosub, one_sided, f"{P.label()}, one-sided, NO subtraction")
        sweep(P, withsub, one_sided, f"{P.label()}, one-sided, with subtraction")
    for P in (Prim(3, False, "wrap"),):
        sweep(P, nosub, one_sided, f"{P.label()}, one-sided, NO subtraction [C1]")
        sweep(P, withsub, one_sided, f"{P.label()}, one-sided, with subtraction [C1]")

    # ------------------------------------------------------------------- C3
    print()
    print("  C3. A signed container with declarations reaching below zero, where the")
    print("  reachable set straddles zero with no subtraction present:")
    print()
    for P in (Prim(3, True, "sat"), Prim(4, True, "sat")):
        sweep(P, nosub, two_sided, f"{P.label()}, two-sided decls, NO subtraction")

    # ------------------------------------------------------------------- P4
    print()
    print("-" * 104)
    print("P4. Are `118` q1's saturating root-only unsound cells all subtraction terms?")
    print()
    for P in (Prim(3, False, "sat"), Prim(3, True, "sat")):
        tot = with_sub = without = 0
        for t in terms:
            k = max(leaves(t)) + 1
            for ext in one_sided(P, k):
                if root_only(P, t, ext) and not arms_agree(P, t, ext, k):
                    tot += 1
                    if has(t, "sub"):
                        with_sub += 1
                    else:
                        without += 1
        print(f"    {P.label():<12} root-only unsound on {tot:>4} cells; "
              f"{with_sub} contain a subtraction, {without} do not")

    # ------------------------------------------------------------------- P5
    print()
    print("-" * 104)
    print("P5. The shipped test's identity, re-derived.")
    print("`warm-clamp-shared/src/lib.rs:1105` asserts that for non-negative")
    print("addition, clamping every step equals clamping once.")
    print()
    for w in (3, 4, 5):
        limit = (1 << w) - 1
        for arity in (2, 3):
            bad = tot = 0
            for terms_ in product(range(0, limit + 1), repeat=arity):
                tot += 1
                eager = 0
                for x in terms_:
                    eager = min(eager + x, limit)
                once = min(sum(terms_), limit)
                if eager != once:
                    bad += 1
            print(f"    W={w} arity={arity}: eager and deferred clamping disagree "
                  f"on {bad}/{tot} folds")

    print()
    print("=" * 104)
    print(
        """
  READING IT

  P2 and P3 are the finding. If W0 differs is zero and root unsound is zero on
  the saturating subtraction-free rows, with the 'exact out' column nonzero so
  C2 is satisfied, then saturation carries the deferral licence on that region
  and `119` 4.3 and 4.4 are both narrower than the evidence.

  C3 decides how the region is stated. If the signed two-sided rows break, the
  condition is that the reachable set keeps its sign rather than that no
  subtraction appears, and the weaker syntactic form is a sufficient condition
  for it rather than the condition itself.
"""
    )


if __name__ == "__main__":
    main()
