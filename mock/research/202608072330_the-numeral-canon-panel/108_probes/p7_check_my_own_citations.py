#!/usr/bin/env python3
"""p7. Every remaining quotation in `108`, opened and its content tested.

`p3` covers the three textual findings it was built for. This covers everything
else the file quotes or attributes, which is most of sections 2, 3, 4, 5, 7 and
12. `RULES.md`: "Check your own citations before shipping, by opening them. A
reference that resolves is not a reference that says what you claim."

Mutation-tested at the end, because a checker that has never failed has not been
tested either.

Run from the probe directory.
"""

import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.join(HERE, "..")
WSRULES = os.path.abspath(os.path.join(PANEL, "..", "..", "..", "..", ".claude", "rules"))

FILES = {
    "40": os.path.join(PANEL, "40_leijen_what_the_axes_actually_are.md"),
    "88": os.path.join(PANEL, "88_op_the_intent_is_not_every_clause_and_there_is_no_universal.md"),
    "93": os.path.join(PANEL, "93_orchard_the_strategy_axis_derived_cold.md"),
    "94": os.path.join(PANEL, "94_wingo_the_strategy_axis_derived_cold.md"),
    "95": os.path.join(PANEL, "95_op_the_panel_runs_to_ratification_and_units_must_converge.md"),
    "97": os.path.join(PANEL, "97_dolan_the_strategy_space_attacked.md"),
    "101": os.path.join(PANEL, "101_wronski_the_cost_coordinates.md"),
    "102": os.path.join(PANEL, "102_torvalds_does_the_mechanism_serve_the_intents.md"),
    "103": os.path.join(PANEL, "103_mcsherry_what_the_corpus_can_and_cannot_show.md"),
    "104": os.path.join(PANEL, "104_op_the_imitation_is_ergonomic_and_i9_is_not_his_to_settle.md"),
    "106": os.path.join(PANEL, "106_giesen_consolidation_the_strategy_axis.md"),
    "107": os.path.join(PANEL, "107_arntzen_entailment_check_on_the_strategy_consolidation.md"),
    "INTENTS": os.path.join(PANEL, "INTENTS.md"),
    "RULES": os.path.join(PANEL, "RULES.md"),
}


def norm(s):
    # strip blockquote, heading and table markers, then emphasis. Boldface is
    # presentation rather than content, and 106 re-bolds a sub-phrase of a
    # sentence 103 bolded whole, which is not a difference in what was said.
    s = re.sub(r"(?m)^\s*[>#|]*\s?", " ", s)
    s = s.replace("**", "").replace("__", "")
    s = re.sub(r"\s+", " ", s)
    return s.strip()


_cache = {}


def body(key):
    if key not in _cache:
        path = FILES.get(key)
        if path is None or not os.path.exists(path):
            _cache[key] = None
        else:
            with open(path, encoding="utf-8") as f:
                _cache[key] = norm(f.read())
    return _cache[key]


def check(key, phrase):
    b = body(key)
    if b is None:
        return None
    return norm(phrase) in b


CITES = [
    # --- section 2, the gates ---
    ("106 names the thing it did not do", "106",
     "I did not attack the pair"),
    ("107 says the same about itself", "107",
     "I did not attack the pair**, per the brief"),
    ("95, the convergence steer, verbatim", "95",
     "towards convergence and solution-finding together, instead of just disproving and attacking"),
    ("I16, the policing clause", "INTENTS",
     "We shouldn't police what kind of laws there are or what shapes they take"),
    ("RULES on opening citations", "RULES",
     "Check your own citations before shipping, by opening them"),
    ("103's lesson about an untested checker", "103",
     "a citation checker that has never failed"),

    # --- section 3.1, observable ---
    ("40's definition of observable", "40",
     "An axis is **observable** when moving it changes the value the program computes"),
    ("40's convention, the only-if", "40",
     "Headroom is on the unobservable side **only if** the overflow policy is applied at the logical "
     "width rather than at the container width"),
    ("40 calls it a canon-shaped sentence nobody has written", "40",
     "is a canon-shaped sentence nobody has written"),
    ("40's own conclusion on the shape", "40",
     "name the objective, expose the observable axes"),
    ("102's ring boundary", "102",
     "become visible at the first step that is not a ring operation"),
    ("102's p2 table, headroom in both regimes", "102",
     "1 headroom 0/640 500/640 OBSERVABLE only past a non-ring step"),
    ("102's p2 table, packing", "102",
     "2 packing 0/640 0/640 UNOBSERVABLE everywhere swept"),
    ("102's p2 table, overflow policy", "102",
     "3 overflow policy 511/640 511/640 OBSERVABLE in both regimes"),

    # --- section 3.2, clause three ---
    ("106's fidelity-constant sentence", "106",
     "cost-only is correct and complete; a fidelity column would measure a constant"),
    ("103's per-arm oracle shape", "103",
     "validated **arm by arm, each against its own declared semantics**"),
    ("103's refined predicate on the measured-coordinate hazard", "103",
     "where the cost ordering and the answer ordering disagree"),

    # --- section 3.3, clause four ---
    ("106 prices the cast", "106",
     "Under type-carried cost, folding one column two ways requires a cast that changes no value"),
    ("97 section 5's arity result", "97",
     "warm-clamp-arity-w13"),
    ("94's W9 shape, as 106 carries it", "106",
     "four sites, three sharing one value type, **zero conditional instructions and zero casts**"),

    # --- section 3.4, clause seven ---
    ("106 clause seven", "106",
     "by **nothing** on their second, because two weightings are incomparable vectors and nothing ever "
     "asks them to combine"),
    ("106 section 5, the opposite about the same word", "106",
     "**On the weighting, the join is union and it is free.**"),
    ("97's p4b, unions that are not generators", "97",
     "pairs whose union is NOT itself a generator: 12"),
    ("97's p4b, nothing unresolvable inside the closure", "97",
     "ordered pairs unresolvable INSIDE the closure: 0 of 225"),
    ("97's generators", "97",
     "d = 4 generators: speed, residency, accuracy, familiarity"),
    ("101's units clause, as 106 carries it", "106",
     "**the weights carry the units**"),
    ("101's ray clause, as 106 carries it", "106",
     "a weighting is a\nray rather than a point"),
    ("94's diagnosis of the flat set, as 106 carries it", "106",
     "a flat set forces two roles through one slot"),

    # --- section 3.5, the ceiling ---
    ("101's ceiling, as 106 carries it", "106",
     "`{time}` reaches **1** section, `{time, size}` reaches **9**, `{time, size, spread}` reaches "
     "**42**"),
    ("40's p2 product size", "40",
     "product size with these values: 16"),
    ("98's 144 rung, restored in 106 section 16", "106",
     "**144 are Pareto-admissible**"),

    # --- section 4 ---
    ("104's ergonomics answer", "104",
     "Neither, it's ergonomics"),
    ("104's test for what is not op's", "104",
     "if both answers leave the intent intact and differ only in what the panel calls things, it is "
     "not his"),
    ("I5 verbatim", "INTENTS",
     "Hot *can* sacrifice soundness, that is its explicit purpose, but it should not lose it for "
     "nothing, instead, provable meaningful gains"),
    ("40 section 5.3's lexicographic-against-finite reading", "40",
     "accuracy is lexicographically prior for every objective except `Hot`, and finitely weighted for "
     "`Hot`"),
    ("40 records the rate is unset", "40",
     "the finite one has exactly one, and it is unset"),
    ("88 on the pull-apart being real", "88",
     "a later expert finding the two readings pull apart somewhere has found something real"),

    # --- section 5 ---
    ("40's two-space relation", "40",
     "resolve : objective × evidence -> mechanism"),
    ("93's phase-one credit paragraph", "93",
     "it keeps both levels and names which one the strategy lives in rather than collapsing to the "
     "generator"),

    # --- section 6 ---
    ("97's dependent function", "97",
     "Arms : Policy -> Set"),

    # --- section 12 ---
    ("100's detector counts, as 106 carries them", "106",
     "Rationalisability catches 0 of 190 unit errors, 0 of 147 column swaps and 0 of 152 dropped "
     "coordinates"),
    ("cone membership catches all 489", "106",
     "**Cone membership of the stated weighting catches all 489**"),

    # --- section 14 ---
    ("101's failed citations, as 106 carries them", "106",
     "fourteen of thirty-seven citations fail on its first run, eight\n  of them because `100` grew by "
     "46 lines underneath it while it read"),
]


MINE = os.path.join(PANEL, "108_lamport_the_pair_attacked.md")

# Entries `108` cites by number, by figure or by heading rather than quoting
# verbatim. These are checked against the source only. Listing them explicitly
# is the point: an entry silently exempted from the reverse check is the hole
# the reverse check exists to close.
ATTRIBUTION_ONLY = ["100's detector counts, as 106 carries them", "101's failed citations, as 106 carries them", "102's p2 table, headroom in both regimes", "102's p2 table, overflow policy", "102's p2 table, packing", "102's ring boundary", '107 says the same about itself', '40 records the rate is unset', "40's p2 product size", "97's generators", "98's 144 rung, restored in 106 section 16", 'RULES on opening citations', 'cone membership catches all 489']


def in_my_file(phrase):
    """The gap every checker in this panel leaves open.

    A checker that tests (source, phrase) proves the phrase exists in the
    source. It does NOT prove the file being checked actually quotes it: the
    two lists can drift, and then the checker is green about a sentence the
    deliverable never contained or, worse, contained in a mutated form.

    So each phrase is also required to appear in `108` itself. A quotation
    present in the source and absent from my file is a stale checker entry; a
    quotation present in my file and mutated from the source is caught by the
    other direction. Both are needed and one alone is theatre.
    """
    with open(MINE, encoding="utf-8") as f:
        return norm(phrase) in norm(f.read())


def main():
    print("p7. Every remaining quotation in 108, opened and tested.")
    print()
    print("Two directions per entry:")
    print("  SRC  the phrase is present in the file it is attributed to")
    print("  MINE the phrase is present in 108 itself, so the list has not drifted")
    print()
    ok = bad = missing_file = 0
    unquoted = []
    for label, key, phrase in CITES:
        if label not in ATTRIBUTION_ONLY and not in_my_file(phrase):
            unquoted.append((label, key))
        hit = check(key, phrase)
        if hit is None:
            missing_file += 1
            mark = "NOFILE"
        elif hit:
            ok += 1
            mark = "ok "
        else:
            bad += 1
            mark = "FAIL"
        print(f"  [{mark}] `{key}`  {label}")
        if hit is not True:
            print(f"          wanted: {norm(phrase)[:100]}")
    print()
    print(f"checked: {len(CITES)}   ok: {ok}   failed: {bad}   file missing: {missing_file}")
    print()
    print("=== the other direction: does 108 actually quote each of these? ===")
    print()
    print(f"  entries checked both ways:     {len(CITES) - len(ATTRIBUTION_ONLY)}")
    print(f"  entries cited without quoting: {len(ATTRIBUTION_ONLY)} (listed in the source of this probe)")
    print()
    if unquoted:
        print(f"  entries present in the source and ABSENT from 108: {len(unquoted)}")
        for label, key in unquoted:
            print(f"    `{key}`  {label}")
        print()
        print("  Each is either a stale checker entry or a quotation the file lost.")
    else:
        print("  0 entries. Every phrase checked against a source is also in 108.")
    print()

    print("=== mutation test ===")
    print()
    mutants = [
        ("a phrase op did not say", "104", "Both, it's arithmetic"),
        ("a real phrase in the wrong file", "97", "Neither, it's ergonomics"),
        ("a near-miss on a real quotation", "106",
         "cost-only is correct and complete; a fidelity column would measure a variable"),
    ]
    caught = 0
    for label, key, phrase in mutants:
        if check(key, phrase) is not True:
            caught += 1
            print(f"  [caught] {label}")
        else:
            print(f"  [MISSED] {label}")
    print()
    print(f"mutants caught: {caught} of {len(mutants)}")
    return 1 if (bad or missing_file or unquoted or caught != len(mutants)) else 0


if __name__ == "__main__":
    sys.exit(main())
