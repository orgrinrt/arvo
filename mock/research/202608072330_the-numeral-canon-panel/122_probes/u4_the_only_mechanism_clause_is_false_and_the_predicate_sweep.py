#!/usr/bin/env python3
"""u4. What u3 does to `119` 4.5, and the sweep for the dimension both dissents
turned out to be about.

THE CLAUSE NEITHER SIGNATORY DISSENTED ON, AND BOTH POINTED AT
----------------------------------------------------------------
`119` 4.5: "A declared restriction is therefore the **only** mechanism that makes
both families available at once, and no choice of behaviour can buy that."

`120` section 8 flagged it without pressing: "'only mechanism' quantifies over
mechanisms rather than over maps", and said it belonged to `116`'s author to
answer. `121` did not dissent on 4.5 and, in its section 3.6, supplied exactly
the thing that decides it: on a non-negative domain saturation carries a semiring
homomorphism, and saturation is order-preserving. So both licence families are
available there, with no declaration at all.

Neither said so about 4.5. u3 measured the deferral half. This measures both
halves at once and states the consequence.

THE SWEEP, WHICH IS THE LARGER JOB
-----------------------------------
Two dissents landed on two clauses and both are the same missing dimension. A
defect found in two places is evidence of a class, so this extracts every
predicate in `119` and checks each for a dimension naming the domain, its sign,
or the ambient span, rather than fixing only the two clauses where it was caught.

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. On a one-signed domain a saturating map has BOTH characters at once: it is
    order-preserving, and it carries the deferral licence u3 measured. So `119`
    4.5's "only mechanism" is false and a second escape exists.
P2. On a domain closed under negation the same map has only the order-preserving
    one, which is the control that makes P1 about the domain.
P3. A wrapping map has the deferral licence on both domains and is
    order-preserving on neither, which is the second control.
P4. Most of `119`'s predicates name no domain dimension. The two the signatories
    caught are not the only two.

NEGATIVE CONTROLS
-----------------
C1. P2 and P3 are the controls for P1.
C2. The extractor must find the predicates that DO name a domain-ish dimension,
    or it is reporting an absence it cannot detect. `119` 4.2 names two domain
    conditions, so at least one predicate must come back positive.
C3. The extractor must report zero domain dimensions on a predicate known to have
    none, and must not match on the word "declarations", which is a different
    dimension and is present nearly everywhere.
"""

import re
import sys
from itertools import product
from pathlib import Path

HERE = Path(__file__).parent
PANEL = HERE.parent


# --------------------------------------------------------------- the two halves


def make_R(policy, lo, hi):
    def R(v):
        if policy == "sat":
            return lo if v < lo else (hi if v > hi else v)
        span = hi - lo + 1
        return ((v - lo) % span) + lo
    return R


def ex(op, a, b):
    return a + b if op == "add" else (a - b if op == "sub" else a * b)


def deferral_holds(R, lo, hi, dom, ops):
    """Does reducing once at the root equal reducing at every node, over every
    two- and three-node term on this operation set? This is arm W0's identity."""
    def every(t, env):
        if t[0] == "leaf":
            return env[t[1]]
        return R(ex(t[0], every(t[1], env), every(t[2], env)))

    def exact(t, env):
        if t[0] == "leaf":
            return env[t[1]]
        return ex(t[0], exact(t[1], env), exact(t[2], env))

    terms = []
    for o1 in ops:
        terms.append((o1, ("leaf", 0), ("leaf", 1)))
        for o2 in ops:
            terms.append((o1, (o2, ("leaf", 0), ("leaf", 1)), ("leaf", 2)))
    bad = tot = 0
    for t in terms:
        k = max(_leaves(t)) + 1
        for env in product(dom, repeat=k):
            e = dict(enumerate(env))
            tot += 1
            if R(exact(t, e)) != every(t, e):
                bad += 1
    return bad, tot


def _leaves(t):
    return [t[1]] if t[0] == "leaf" else _leaves(t[1]) + _leaves(t[2])


def order_preserving(R, dom):
    """Read on the range of EXACT RESULTS the operations reach, not on the
    operand domain.

    The first version of this read R on the operand domain alone, and where that
    domain fits inside the container R is the identity there and every row came
    back monotone, including the wrapping ones. That is the map not being
    exercised rather than the map being order-preserving, and it is the same
    class of defect as measuring a homomorphism on a domain that cannot reach the
    reduction."""
    lo, hi = min(dom), max(dom)
    reach = range(min(lo, lo * lo, lo + lo), max(hi + hi, hi * hi) + 1)
    vals = [R(v) for v in reach]
    return all(vals[i] <= vals[i + 1] for i in range(len(vals) - 1))


# ------------------------------------------------------------ the predicate sweep

DOMAIN_WORDS = (
    "domain", "ambient", "span", "closed under negation", "straddl",
    "non-negative", "window",
)


def extract_predicates(text):
    """Every `holds for:` block in `119`, as (label, body)."""
    out = []
    for m in re.finditer(r"\*[^*]*?holds for:(.+?)\*", text, re.S):
        body = re.sub(r"\s+", " ", m.group(1)).strip()
        start = text.rfind("\n### ", 0, m.start())
        head = text[start:text.find("\n", start + 1)].strip("# \n") if start >= 0 else "?"
        out.append((head, body))
    return out


def names_domain(body):
    low = body.lower()
    return [w for w in DOMAIN_WORDS if w in low]


def main():
    print("=" * 100)
    print("u4. `119` 4.5's 'only mechanism' is false, and the sweep for the class")
    print("=" * 100)

    # ------------------------------------------------------- P1, P2, P3, C1
    print()
    print("P1, P2, P3. Which characters each map has, on each domain.")
    print("'deferral' is arm W0's identity over the operation set named.")
    print()
    lo, hi = 0, 7
    print(f"  container [{lo}, {hi}]")
    print()
    print(f"  {'policy':<8} {'domain':<26} {'ops':<16} {'order-preserving':>17} "
          f"{'deferral holds':>16} {'failures':>10}")
    for policy in ("sat", "wrap"):
        R = make_R(policy, lo, hi)
        for dlabel, dom in (
            ("one-signed 0..7", list(range(0, 8))),
            ("closed under negation -7..7", list(range(-7, 8))),
        ):
            for ops, olabel in ((("add", "mul"), "{add, mul}"),
                                (("add", "sub", "mul"), "{add, sub, mul}")):
                bad, tot = deferral_holds(R, lo, hi, dom, ops)
                op_ok = order_preserving(R, dom)
                print(f"  {policy:<8} {dlabel:<26} {olabel:<16} {str(op_ok):>17} "
                      f"{str(bad == 0):>16} {bad:>4}/{tot:<5}")

    print()
    print("  P1 holds when the saturating one-signed `{add, mul}` row is True in")
    print("  BOTH character columns. That is both licence families at once with no")
    print("  declaration, which `119` 4.5 says only a declaration can buy.")
    print()
    print("  C1 is the two controls: the closed-under-negation row must lose the")
    print("  deferral column, and the wrapping rows must lose the order column on")
    print("  both domains.")

    # ------------------------------------------------------------------- P4
    print()
    print("-" * 100)
    print("P4. Every predicate in `119`, checked for a dimension naming the domain.")
    print()
    text = (PANEL / "119_leroy_the_canon_candidate_for_the_realisation_map.md").read_text()
    preds = extract_predicates(text)
    print(f"  predicates found: {len(preds)}")
    print()
    named = missing = 0
    for head, body in preds:
        hits = names_domain(body)
        short = head[:52]
        if hits:
            named += 1
            print(f"  NAMES  {short:<54} {', '.join(hits)}")
        else:
            missing += 1
            print(f"  none   {short:<54}")
    print()
    print(f"  predicates naming a domain-ish dimension : {named}")
    print(f"  predicates naming none                   : {missing}")
    print()
    print("  C2 is the nonzero 'NAMES' count: 4.2 states two domain conditions, so")
    print("  an extractor reporting zero everywhere would be blind rather than")
    print("  reporting an absence.")

    # ------------------------------------------------------------------- C3
    print()
    print("C3. The extractor on a predicate with no domain dimension, and the check")
    print("    that it does not fire on the word 'declarations'.")
    print()
    for probe, label in (
        ("W = 3, F = 0, signedness any, threads = 1", "no domain dimension"),
        ("declarations = one-sided exhaustive, threads = 1", "'declarations' only"),
        ("ambient domain in {non-negative, closed under negation}", "a real one"),
    ):
        print(f"    {label:<24} -> {names_domain(probe) or 'none'}")

    print()
    print("=" * 100)
    print(
        """
  READING IT

  P1 is the correction to 4.5. If a one-signed domain buys both families with no
  declaration, then "only mechanism" is false and there are two escapes: a
  declared restriction, and a domain the map cannot carry a value out of the
  sign of.

  P4 is the sweep. The two clauses the signatories caught are the two where the
  omission produced a visible contradiction; the count above is how many others
  carry the same silence.
"""
    )


if __name__ == "__main__":
    sys.setrecursionlimit(10000)
    main()
