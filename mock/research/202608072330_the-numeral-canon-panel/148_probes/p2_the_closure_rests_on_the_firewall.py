"""p2 (148): does 146 section 6.3's closure depend on section 6.1's unpredicated proposition?

`146` presents two things as independent.

Section 6.3 closes the count's second argument: visibility is monotone in the
observation set and saturates, so "the axis-only property is visibility under the
maximal observation set", and an axis visible there "is component one and cutting
it is forbidden".

Section 6.1 records that the firewall carries no predicate in any file that
endorses it, and that the candidate cannot supply one.

My claim, to be tested rather than argued: **6.3's closure is a consequence of
the firewall, so it inherits 6.1's status.** The visibility test says an
answer-visible axis is component one. That is only forced if nothing outside the
declared policy may move an answer. Drop the firewall, and an answer-visible
choice may sit in component two, which is exactly what `139`'s retired slack
repair proposed and what `146` section 1.8 records as retired on measured
grounds rather than on principle.

Method. Take one arm set where fusing a multiply-add changes the answer (signed
saturating). Classify the axis that distinguishes the arms under two rules:

  FIREWALL:  two arms computing different answers realise different denotations,
             so they are two assignments and the axis is COMPONENT ONE.
  SLACK(k):  two arms whose answers never differ by more than k conform to one
             declaration, so they are two realisations and the axis is
             COMPONENT TWO.

If the component assignment is a function of the rule rather than of the
measurement, the closure depends on which rule is adopted, and the rule is the
unpredicated proposition.

THE CASE THAT MUST FAIL, and there are three:
  (i)   SLACK(0) must agree with FIREWALL on every axis. Slack zero IS the
        firewall, so a disagreement means the two rules are not comparable and
        nothing below can be concluded.
  (ii)  A DEAD axis, whose two positions are extensionally identical, must come
        out COMPONENT TWO under both rules at every slack. If a dead axis is ever
        classified component one, the classifier is broken.
  (iii) A LIVE axis must come out COMPONENT ONE under the firewall. If it does
        not, the sweep contains no case where the two rules could differ and the
        result is vacuous.
Any failing control voids the verdict and exits non-zero.
"""

import sys
from itertools import product

W = 5
F = 1


def lo(signed):
    return -(1 << (W - 1)) if signed else 0


def hi(signed):
    return (1 << (W - 1)) - 1 if signed else (1 << W) - 1


def sat(v, signed):
    l, h = lo(signed), hi(signed)
    return l if v < l else (h if v > h else v)


def shift_floor(v, f):
    return v >> f if f else v


def arm_stepwise(a, b, c, signed):
    """Reduce the product, then add, then reduce."""
    p = sat(shift_floor(a * b, F), signed)
    return sat(p + c, signed)


def arm_fused(a, b, c, signed):
    """Hold the product, add, reduce once."""
    return sat(shift_floor(a * b, F) + c, signed)


def arm_stepwise_copy(a, b, c, signed):
    """A dead axis: a second spelling of the same function. Control (ii)."""
    p = shift_floor(a * b, F)
    p = sat(p, signed)
    t = p + c
    return sat(t, signed)


def domain(signed):
    r = range(lo(signed), hi(signed) + 1)
    return product(r, r, r)


def max_gap(f, g, signed):
    """The largest absolute difference between two arms over the whole domain."""
    m = 0
    diffs = 0
    for a, b, c in domain(signed):
        d = abs(f(a, b, c, signed) - g(a, b, c, signed))
        if d:
            diffs += 1
            if d > m:
                m = d
    return m, diffs


def classify_firewall(gap):
    """Different answers anywhere means two denotations."""
    return "component one" if gap > 0 else "component two"


def classify_slack(gap, k):
    """Conforming within k means one declaration with two realisations."""
    return "component two" if gap <= k else "component one"


def main():
    failures = []
    signed = True

    print("p2 (148): is the component assignment a function of the rule or of the measurement?")
    print(f"W={W}, F={F}, signed, overflow=saturating, exhaustive over the declared range\n")

    live_gap, live_diffs = max_gap(arm_stepwise, arm_fused, signed)
    dead_gap, dead_diffs = max_gap(arm_stepwise, arm_stepwise_copy, signed)

    print("=== the two axes ===")
    print(f"LIVE axis (stepwise vs fused): differ at {live_diffs} triples, max gap {live_gap}")
    print(f"DEAD axis (stepwise vs a second spelling): differ at {dead_diffs} triples, "
          f"max gap {dead_gap}")

    # control (iii)
    print("\n=== control (iii): the live axis must be component one under the firewall ===")
    if classify_firewall(live_gap) != "component one":
        print("  !! CONTROL FAIL: the live axis is not component one under the firewall,")
        print("  so the two rules can never differ here and the result is vacuous.")
        failures.append("iii")
    else:
        print("  fires: the live axis is component one under the firewall.")

    # control (ii)
    print("\n=== control (ii): the dead axis must be component two under every rule ===")
    dead_ok = classify_firewall(dead_gap) == "component two"
    for k in (0, 1, 2, 4, 8, 16, 32, 64):
        if classify_slack(dead_gap, k) != "component two":
            dead_ok = False
    if not dead_ok:
        print("  !! CONTROL FAIL: a dead axis was classified component one somewhere.")
        print("  the classifier is broken and every verdict below is void.")
        failures.append("ii")
    else:
        print("  fires: the dead axis is component two under the firewall and at every")
        print("  slack tried, so the classifier does not manufacture policy content.")

    # control (i)
    print("\n=== control (i): SLACK(0) must agree with FIREWALL ===")
    agree = True
    for gap in (live_gap, dead_gap):
        if classify_slack(gap, 0) != classify_firewall(gap):
            agree = False
    if not agree:
        print("  !! CONTROL FAIL: slack zero disagrees with the firewall, so the two")
        print("  rules are not comparable and nothing below follows.")
        failures.append("i")
    else:
        print("  fires: slack zero is the firewall, so the two rules are one family")
        print("  and the slack parameter is what separates them.")

    # the actual question
    print("\n=== the classification, as a function of the rule ===")
    print(f"{'rule':<16} {'live axis':<16} {'dead axis'}")
    print("-" * 48)
    print(f"{'FIREWALL':<16} {classify_firewall(live_gap):<16} {classify_firewall(dead_gap)}")
    flip_at = None
    for k in (0, 1, 2, 4, 8, 16, 32, 64):
        cl = classify_slack(live_gap, k)
        cd = classify_slack(dead_gap, k)
        print(f"{'SLACK(' + str(k) + ')':<16} {cl:<16} {cd}")
        if flip_at is None and cl == "component two":
            flip_at = k

    print("\n=== verdict ===")
    if flip_at is None:
        print("the live axis stayed component one at every slack tried, so within this")
        print("sweep the classification did NOT move with the rule. my claim is REFUTED")
        print("here and the closure does not depend on the firewall in this cell.")
    else:
        print(f"the live axis is COMPONENT ONE under the firewall and COMPONENT TWO once")
        print(f"the slack reaches {flip_at}, with the measurement held fixed throughout.")
        print("So which component an answer-visible axis belongs to is a function of the")
        print("classification rule, not of the measurement.")
        print()
        print("146 section 6.3 closes the count's second argument by declaring that an")
        print("axis visible under the maximal observation set IS component one. That")
        print("declaration is the firewall. Section 6.1 records the firewall as carrying")
        print("no predicate in any file that endorses it. So the closure inherits that")
        print("status: it is as settled as the firewall is and no more.")

    print("\n=== controls ===")
    if failures:
        print(f"CONTROL FAILURES: {failures}. every verdict above is void.")
        sys.exit(1)
    print("all three controls fired.")


if __name__ == "__main__":
    main()
