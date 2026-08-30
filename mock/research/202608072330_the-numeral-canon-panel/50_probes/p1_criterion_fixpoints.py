#!/usr/bin/env python3
# p1: the criterion at 16:100-101, written as a formal condition on a candidate output set,
# and solved exhaustively.
#
#   "A component is an output of the derivation when the consumer did not write it, the machine
#    needs it, and a downstream site that holds THE OTHER COMPONENTS cannot recover it."
#
# The third clause is a condition on O (the output set) in which O appears on both sides, because
# "the other components" are the other members of O. So the criterion is not a definition, it is a
# fixpoint equation:
#
#     O = { f in NEEDED \ DECL : not derivable(f, (O \ {f}) union HELD union PRIM) }
#
# and it says nothing about which fixpoint is meant. The operator is NON-MONOTONE (adding a fact to
# O can make another fact derivable and therefore REMOVE it from O), so there is no least or
# greatest fixpoint to appeal to. This file enumerates every fixpoint, in every cell of a 2x2x2
# grid of modelling choices the criterion leaves unstated.
#
# The three unstated parameters:
#
#   HELD      Reading A: the site holds only the derivation's outputs.  HELD = {}
#             Reading B: the site holds the numeral type, hence the declaration. HELD = DECL
#             48 sections 5 reports 16 using A at 16:187-189 and B at 16:572-577.
#
#   STRATSET  S4: exactly the four strategies of the prior attempt, where the only packing
#                 discipline is Cold's stride == W.
#             S5: the strategy set is open (INTENTS I1), so some strategy may pack to a grid,
#                 making stride < 8*size_of(carrier) without stride == W.
#
#   KIND      blind:  a rule may produce a type from consts.
#             aware:  it may not. 16_probes/p5b (4 refusals), 47_probes/p2 (6), 47_probes/p3 (3),
#                     all naming the forbidden generic_const_exprs.
#
# Nothing here is a measurement of arvo. It is a model of one sentence, and its whole purpose is to
# show how far the sentence's answer moves when parameters it never names are varied.
#
#   python3 p1_criterion_fixpoints.py

from itertools import chain, combinations

DECL = frozenset({"W", "SIGN", "STRAT"})

# every fact the unit has named as possibly-an-output, plus the declaration
ALL = ["CARRIER", "STRIDE", "ALIGN", "ACCESS", "COMPUTE"]

# (antecedents, consequent, consequent_is_a_type, provenance)
RULES_BASE = [
    (frozenset({"CARRIER"}), "ALIGN", False,
     "16:605-613  align_of is a property of a type"),
    (frozenset({"W", "STRIDE"}), "ACCESS", True,
     "16:186-189  floor((W+6)/8)+1, corrected by p4 to need the stride as well"),
    (frozenset({"W", "STRAT", "CARRIER"}), "STRIDE", False,
     "16:153-157  Cold: W. otherwise 8*size_of(carrier)"),
    (frozenset({"W", "STRAT", "SIGN"}), "CARRIER", True,
     "the width-to-container ladder, 16 section 2"),
    (frozenset({"W", "STRAT", "CARRIER"}), "COMPUTE", True,
     "the Precise-widens rule, 16 section 6"),
]

# available only when the strategy set is closed at four: the sole packing discipline is
# stride == W, so a site seeing stride < 8*size_of(carrier) may conclude W == stride.
RULE_S4_ONLY = (frozenset({"STRIDE", "CARRIER"}), "W", False,
                "S4 only: the only packer is Cold, whose stride IS the width")


def closure(held, rules):
    """everything a site holding `held` can reach, using `rules` to exhaustion."""
    have = set(held)
    changed = True
    while changed:
        changed = False
        for ante, cons, _is_type, _prov in rules:
            if cons not in have and ante <= have:
                have.add(cons)
                changed = True
    return have


def rules_for(stratset, kind):
    rs = list(RULES_BASE)
    if stratset == "S4":
        rs.append(RULE_S4_ONLY)
    if kind == "aware":
        # a rule whose consequent is a TYPE and whose antecedents are not all types is refused.
        # the only type in play on the antecedent side is CARRIER.
        rs = [r for r in rs if (not r[2]) or (r[0] <= {"CARRIER"})]
    return rs


def powerset(xs):
    return chain.from_iterable(combinations(xs, k) for k in range(len(xs) + 1))


def solve(held_decl, stratset, kind, needed):
    rules = rules_for(stratset, kind)
    candidates = [f for f in ALL if f in needed and f not in DECL]
    base = set(DECL) if held_decl else set()
    fixpoints = []
    for sub in powerset(candidates):
        O = set(sub)
        ok = True
        for f in candidates:
            others = (O - {f}) | base
            recoverable = f in closure(others, rules)
            should_be_output = not recoverable
            if should_be_output != (f in O):
                ok = False
                break
        if ok:
            fixpoints.append(frozenset(O))
    # soundness: with the criterion's own answer in hand, can the site reach EVERY needed fact?
    sound = []
    for O in fixpoints:
        reach = closure(set(O) | base, rules)
        missing = sorted(n for n in needed if n not in reach)
        sound.append((O, missing))
    return sound


def fmt(O):
    return "{" + ", ".join(sorted(O)) + "}" if O else "{}"


def main():
    for precise_widens in (False, True):
        needed = set(ALL)
        if not precise_widens:
            needed.discard("COMPUTE")
        # the machine also needs the declared width, to scale and to mask. it is not eligible to
        # BE an output (clause one) but it is still needed, which is what the soundness column
        # below measures.
        needed_for_soundness = needed | {"W"}
        print("=" * 92)
        print("Precise widens compute past storage: %s" % precise_widens)
        print("=" * 92)
        print("%-9s %-9s %-7s  %-46s %s"
              % ("reading", "stratset", "kind", "criterion-consistent output sets", "unreachable but needed"))
        for held_decl, rname in ((False, "A"), (True, "B")):
            for stratset in ("S4", "S5"):
                for kind in ("blind", "aware"):
                    res = solve(held_decl, stratset, kind, needed)
                    if not res:
                        print("%-9s %-9s %-7s  %-46s %s"
                              % (rname, stratset, kind, "NONE", "-"))
                        continue
                    for O, _ in res:
                        rules = rules_for(stratset, kind)
                        base = set(DECL) if held_decl else set()
                        reach = closure(set(O) | base, rules)
                        missing = sorted(n for n in needed_for_soundness if n not in reach)
                        print("%-9s %-9s %-7s  %-46s %s"
                              % (rname, stratset, kind, fmt(O),
                                 ",".join(missing) if missing else "none"))
        print()

    # the non-monotonicity, shown rather than asserted
    print("=" * 92)
    print("the operator is non-monotone, which is why there is no canonical fixpoint")
    print("=" * 92)
    rules = rules_for("S4", "blind")
    for O in ({"CARRIER"}, {"CARRIER", "STRIDE"}):
        reach = closure(set(O), rules)
        print("  site holds %-24s -> W recoverable: %-5s  ACCESS recoverable: %s"
              % (fmt(O), "W" in reach, "ACCESS" in reach))
    print("  ACCESS is an output when the site holds {CARRIER} and is NOT one when it holds")
    print("  {CARRIER, STRIDE}. so growing O REMOVES a member of O. a monotone operator cannot")
    print("  do that, and lfp/gfp are therefore unavailable as a tie-break between the fixpoints.")
    print()

    # how many distinct answers does one sentence produce
    answers = set()
    for pw in (False, True):
        needed = set(ALL)
        if not pw:
            needed.discard("COMPUTE")
        for held_decl in (False, True):
            for stratset in ("S4", "S5"):
                for kind in ("blind", "aware"):
                    for O, _ in solve(held_decl, stratset, kind, needed):
                        answers.add(O)
    print("distinct criterion-consistent output sets across all 16 cells: %d" % len(answers))
    for a in sorted(answers, key=lambda s: (len(s), sorted(s))):
        print("   |O| = %d   %s" % (len(a), fmt(a)))
    print()
    print("counts the sentence admits: %s"
          % sorted({len(a) for a in answers}))


if __name__ == "__main__":
    main()
