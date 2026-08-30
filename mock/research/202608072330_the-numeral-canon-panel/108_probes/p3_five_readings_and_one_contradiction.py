#!/usr/bin/env python3
"""p3. Two textual findings, each pinned by opening every passage it rests on.

FINDING ONE. Op's one sentence at `88` has five incompatible readings in this
panel, and the pair's leg (a) is one of them. Op flagged his own difficulty
wording it and has since declined to rule on the nearest question (`104` s3).

FINDING TWO. The pair contains a contradiction between its own clauses, and it
was carried from `102` into `106` unchanged. Component two is defined as
ranging over arms that produce the answer component one fixed; the same files
map op's accuracy intent onto component two as a weighting over a fidelity
coordinate; and a fidelity coordinate over answer-equivalent arms measures a
constant, which `106` says itself.

Every claim below is a (file, phrase) pair that is OPENED and checked, not
resolved. `RULES.md`: "Check your own citations before shipping, by opening
them. A reference that resolves is not a reference that says what you claim."

The checker is mutation-tested at the end: three mutants, each a plausible
near-miss, must all be caught. A checker that has never failed has not been
tested either.

Run from the probe directory.
"""

import os
import re
import sys

PANEL = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")

FILES = {
    "25": "25_torvalds_what_a_strategy_is.md",
    "40": "40_leijen_what_the_axes_actually_are.md",
    "88": "88_op_the_intent_is_not_every_clause_and_there_is_no_universal.md",
    "93": "93_orchard_the_strategy_axis_derived_cold.md",
    "94": "94_wingo_the_strategy_axis_derived_cold.md",
    "97": "97_dolan_the_strategy_space_attacked.md",
    "98": "98_spj_what_the_strategy_axis_settles.md",
    "101": "101_wronski_the_cost_coordinates.md",
    "102": "102_torvalds_does_the_mechanism_serve_the_intents.md",
    "103": "103_mcsherry_what_the_corpus_can_and_cannot_show.md",
    "104": "104_op_the_imitation_is_ergonomic_and_i9_is_not_his_to_settle.md",
    "106": "106_giesen_consolidation_the_strategy_axis.md",
    "107": "107_arntzen_entailment_check_on_the_strategy_consolidation.md",
    "INTENTS": "INTENTS.md",
}


def norm(s):
    """Whitespace-normalise and strip blockquote and doc-comment markers, so a
    quotation wrapped across lines or carried inside a `>` block still matches.
    Neither normalisation can make an absent phrase appear."""
    s = re.sub(r"(?m)^\s*[>#]*\s?", " ", s)
    s = re.sub(r"\s+", " ", s)
    return s.strip()


_cache = {}


def body(key):
    if key not in _cache:
        with open(os.path.join(PANEL, FILES[key]), encoding="utf-8") as f:
            _cache[key] = norm(f.read())
    return _cache[key]


def check(key, phrase):
    return norm(phrase) in body(key)


# ---------------------------------------------------------------------------
# FINDING ONE: five readings of one sentence
# ---------------------------------------------------------------------------

READINGS = [
    (
        "op, the sentence itself",
        "88",
        "Mostly option 1, but a little bit of option 3 with it. Hard to put into words, "
        "hopefully you get my meaning here",
    ),
    (
        "op flags his own difficulty, in the same file",
        "88",
        "Op flagged his own difficulty putting it into words, so a later expert finding the two "
        "readings pull apart somewhere has found something real",
    ),
    (
        "93: a RANKING of two readings, neither decomposed",
        "93",
        "The point reading is mostly it and the weighting reading is a little bit of it",
    ),
    (
        "97: TIERS, and the surviving bit is the rationalisability constraint",
        "97",
        "The little bit of option 3 that survives into the design tier is one checkable constraint: "
        "the table must be rationalisable",
    ),
    (
        "98: a RUNG on a five-step ladder, strictly between the ends",
        "98",
        "a little bit of option 3",
    ),
    (
        "102: a DECOMPOSITION into two components, not a proportion",
        "102",
        "a decomposition rather than a proportion",
    ),
    (
        "106: the NAMED BINDING, a point in each component",
        "106",
        "It also makes `88`'s \"mostly option 1, a little bit of option 3\" read cleanly: mostly the "
        "point, with a weighting attached",
    ),
]

# ---------------------------------------------------------------------------
# FINDING TWO: the contradiction, four passages
# ---------------------------------------------------------------------------

CONTRADICTION = [
    (
        "C1. 106 defines component two as ranging over answer-equivalent arms",
        "106",
        "a **weighting over cost coordinates**, which selects among the arms that produce the answer "
        "the first component fixed",
    ),
    (
        "C1b. 102 states the same clause, in the wording 106 compressed",
        "102",
        "Its second is a weighting over cost coordinates, which selects among the arms that produce "
        "that value",
    ),
    (
        "C2. 102 says op's accuracy and speed intents range over arms that DISAGREE",
        "102",
        "I5 trades accuracy for speed, I7 buys accuracy with speed, I3 asks for a particular answer",
    ),
    (
        "C2b. and names that explicitly",
        "102",
        "Each ranges over arms that **disagree**",
    ),
    (
        "C3. 102 nonetheless maps I7 onto component two as a weighting",
        "102",
        "I7, accuracy first | weighting, over a computed coordinate, with depth in the region",
    ),
    (
        "C4. 106 says a fidelity coordinate over answer-equivalent arms measures a constant",
        "106",
        "cost-only is correct and complete; a fidelity column would measure a constant",
    ),
    (
        "C5. 106 carries the remedy for I7 as the missing fidelity hook",
        "106",
        "The missing piece is\n`score_output`",
    ),
]

# ---------------------------------------------------------------------------
# FINDING THREE: what the two-space structure's provenance actually is
# ---------------------------------------------------------------------------

PROVENANCE = [
    (
        "P1. 40 states the two spaces, 53 files before 93 and 57 before 97",
        "40",
        "A strategy lives in the **objective** space. A mechanism assignment is what a strategy "
        "produces when it is applied to evidence",
    ),
    (
        "P2. 93's cold derivation records that 40 got there first",
        "93",
        "That is op's \"mostly option 1, a little bit of option 3\" derived before he said it",
    ),
    (
        "P3. 93 claims THREE independent instances including 25",
        "93",
        "The credit for the two-space reading is `40`'s and `25`'s, and mine is a third independent "
        "arrival at it",
    ),
    (
        "P4. but 40's own account of 25 says 25 does NOT carry the second space",
        "40",
        "What it does not carry is the thing that generates the graph, and op supplied that afterwards",
    ),
    (
        "P5. and 40 declares itself a refinement OF 25, so the two are not independent",
        "40",
        "That is a refinement of `25` rather than a replacement",
    ),
    (
        "P6. 97 credits 40 for the observable definition, honestly",
        "97",
        "This is a derivation from `40:398`'s definition and `40` section 5.4's compile",
    ),
    (
        "P7. 106 attributes that same definition to 97",
        "106",
        "`97` defines an observable coordinate as one whose movement **changes the value the program "
        "computes**",
    ),
    (
        "P8. op declined the nearest question and returned it to the panel",
        "104",
        "I think the intent is clear and this is impl detail that already had answer: optimal and "
        "converged to by experts",
    ),
]


def run(title, rows):
    print(f"=== {title} ===")
    print()
    ok = 0
    bad = 0
    for label, key, phrase in rows:
        hit = check(key, phrase)
        if hit:
            ok += 1
        else:
            bad += 1
        print(f"  [{'ok ' if hit else 'FAIL'}] {label}")
        print(f"         in `{key}`: {norm(phrase)[:96]}...")
    print()
    print(f"  {ok} present, {bad} absent")
    print()
    return bad


def main():
    print("p3. Five readings of one sentence, and a contradiction inside the pair.")
    print()
    bad = 0
    bad += run("FINDING ONE: five incompatible readings of op's `88` answer", READINGS)
    bad += run("FINDING TWO: the pair's clause three against its own consequences", CONTRADICTION)
    bad += run("FINDING THREE: where the two-space structure actually comes from", PROVENANCE)

    print("=== mutation test of this checker ===")
    print()
    mutants = [
        ("a phrase op did not say", "88", "Mostly option 3, but a little bit of option 1 with it"),
        ("a real phrase in the wrong file", "101", "a decomposition rather than a proportion"),
        (
            "a near-miss on a real quotation",
            "106",
            "a weighting over cost coordinates, which selects among the arms that produce the answer "
            "the second component fixed",
        ),
    ]
    caught = 0
    for label, key, phrase in mutants:
        hit = check(key, phrase)
        if not hit:
            caught += 1
        print(f"  [{'caught' if not hit else 'MISSED'}] {label}")
    print()
    print(f"  {caught} of {len(mutants)} mutants caught")
    print()

    print("=== verdict ===")
    print()
    print(f"absent citations across all three findings: {bad}")
    print(f"mutants caught: {caught} of {len(mutants)}")
    if bad == 0 and caught == len(mutants):
        print()
        print("Every passage the file's textual findings rest on is present as quoted,")
        print("and the instrument that says so has been made to fail three ways.")
    return 1 if (bad or caught != len(mutants)) else 0


if __name__ == "__main__":
    sys.exit(main())
