#!/usr/bin/env python3
"""p6. Open every citation this file makes and test its CONTENT, not its resolution.

`RULES.md` records that one member found seven of its own citations wrong by doing
this, and calls the resulting probe the cheapest correctness tool the panel has.
This is that tool pointed at `102`.

Two kinds of check:

  A. every heading I cite by name exists in the file I attribute it to;
  B. every phrase I quote appears verbatim in the file I attribute it to.

A citation that resolves is not a citation that says what I claim, so B is the one
that matters.

Run:  python3 p6_check_my_own_citations.py
"""

import pathlib
import re
import sys


def norm(text):
    """Collapse whitespace and blockquote markers.

    A quotation that is wrapped across lines in the source, or carried in a
    blockquote, is still a verbatim quotation. Searching the raw text reports a
    wrapping artifact as a miscitation, which is a false alarm that would hide the
    real ones underneath it. Normalising both sides is what makes the check about
    the words rather than about the line breaks.
    """
    stripped = re.sub(r"(?m)^\s*(>|///|//!|//)\s?", "", text)
    return re.sub(r"\s+", " ", stripped).strip()

HERE = pathlib.Path(__file__).resolve().parent
PANEL = HERE.parent
BENCHES = PANEL.parents[1] / "benches"

# ---------------------------------------------------------------------------
# A. headings
# ---------------------------------------------------------------------------

HEADINGS = [
    ("INTENTS.md", "## I3."),
    ("INTENTS.md", "## I4."),
    ("INTENTS.md", "## I5."),
    ("INTENTS.md", "## I6."),
    ("INTENTS.md", "## I7."),
    ("INTENTS.md", "## I8."),
    ("INTENTS.md", "## I9."),
    ("INTENTS.md", "## I13."),
    ("INTENTS.md", "## I15."),
    ("INTENTS.md", "## I17."),
    ("25_torvalds_what_a_strategy_is.md", "## 7. The sentences"),
    ("25_torvalds_what_a_strategy_is.md", "## 8. What is op's"),
    ("97_dolan_the_strategy_space_attacked.md", "### 2.1 What each of the three actually says"),
    ("97_dolan_the_strategy_space_attacked.md", "### 3.2 The distinction that actually does the work is polarity"),
    ("98_spj_what_the_strategy_axis_settles.md", "### 4.1 A priority is not an exchange rate"),
    ("98_spj_what_the_strategy_axis_settles.md", "### 4.2 What is not expressible"),
    ("98_spj_what_the_strategy_axis_settles.md", "### 4.3 One of op's strategies is defined by a coordinate"),
    ("98_spj_what_the_strategy_axis_settles.md", "## 5. What the committed corpus can and cannot weigh"),
    ("100_xu_generating_the_table_attacked.md", "### 6.1 The same thing as arms with predicates"),
    ("101_wronski_the_cost_coordinates.md", "### 2.3 The storage coordinate is declared"),
    ("101_wronski_the_cost_coordinates.md", "### 4.2 The ceiling, and it is what a coordinate is for"),
    ("101_wronski_the_cost_coordinates.md", "### 6.1 Accuracy cannot be a per-arm scalar"),
    ("101_wronski_the_cost_coordinates.md", "### 6.2 The fidelity coordinate is reachable"),
    ("101_wronski_the_cost_coordinates.md", "## 7. The region set"),
    ("OPTIONS.md", "### Q43."),
    ("OPTIONS.md", "### Q44."),
    ("OPTIONS.md", "### Q47."),
    ("OPTIONS.md", "### Q48."),
    ("88_op_the_intent_is_not_every_clause_and_there_is_no_universal.md", "## 1. A strategy is a preset"),
    ("88_op_the_intent_is_not_every_clause_and_there_is_no_universal.md", "## 4. There is no universal answer"),
    ("85_op_no_runtime_checks_ever_and_stop_policing_law_shapes.md", "## 1. Never any runtime checks, ever"),
    ("85_op_no_runtime_checks_ever_and_stop_policing_law_shapes.md", "## 2. Stop policing what shape a law takes"),
]

# ---------------------------------------------------------------------------
# B. verbatim phrases
# ---------------------------------------------------------------------------

QUOTES = [
    ("INTENTS.md", "It should behave like native primitives in regular old rust would"),
    ("INTENTS.md", "does not make it absolutely required, if mimicking is consistently just worse choice"),
    ("INTENTS.md", "the intent behind Hot is performance, efficiency, even at the cost of accuracy or soundness"),
    ("INTENTS.md", "it should not lose it for nothing, instead, provable meaningful gains"),
    ("INTENTS.md", "it should remain small for memory or disk storage, because it's just sitting basically"),
    ("INTENTS.md", "Cold does not *have to* drop efficiency wins elsewhere"),
    ("INTENTS.md", "But if the path fights the intent, then it's not for Cold"),
    ("INTENTS.md", "especially within chains and ops, not only alone"),
    ("INTENTS.md", "They weigh different measurements differently"),
    ("INTENTS.md", "strategies are the variables that change what the \"correct\" answer is for what we choose as the path"),
    ("INTENTS.md", "Never any runtime checks, ever"),
    ("97_dolan_the_strategy_space_attacked.md",
     "a policy coordinate can never be decided by measurement at all, in any phase, because doing so makes the program's answer a function of a benchmark"),
    ("97_dolan_the_strategy_space_attacked.md", "72 of 15625 sections are rationalisable"),
    ("98_spj_what_the_strategy_axis_settles.md",
     "an imitation constraint is a different kind of specification from a weighting"),
    ("98_spj_what_the_strategy_axis_settles.md",
     "runs where the shipped rule is Pareto-dominated on (time, bytes): 18"),
    ("100_xu_generating_the_table_attacked.md", "0.045%"),
    ("100_xu_generating_the_table_attacked.md", "3.83%"),
    ("100_xu_generating_the_table_attacked.md", "a median of 0.273%"),
    ("100_xu_generating_the_table_attacked.md", "**Arm C, the band rather than the equality.**"),
    ("100_xu_generating_the_table_attacked.md",
     "holds where the region's competing arms are separated by less than the coordinate's resolution but more than zero"),
    ("101_wronski_the_cost_coordinates.md",
     "A strategy whose intent names a quantity with no coordinate is not unmeasured. It is inexpressible."),
    ("101_wronski_the_cost_coordinates.md", "CROSSING at chain length k = 4"),
    ("101_wronski_the_cost_coordinates.md",
     "A quantity belongs in the **region** when a strategy's answer may differ across it"),
    ("88_op_the_intent_is_not_every_clause_and_there_is_no_universal.md",
     "Mostly option 1, but a little bit of option 3 with it"),
    ("25_torvalds_what_a_strategy_is.md",
     "named sections over a product of axes rather than values of a single axis"),
    ("25_torvalds_what_a_strategy_is.md", "a function of the build condition"),
    ("RULES.md", "One instance of evidence is never enough"),
]

# source files in the pinned checkout, cited by item name rather than by line
SOURCE_ITEMS = [
    (BENCHES / "variants/wide-rung-shared/src/tests.rs",
     "Agreement between the arms establishes only that they agree"),
    (BENCHES / "variants/warm-container-shared/src/lib.rs",
     "all_four_arms_agree_with_each_other_and_with_the_oracle_on_every_key"),
    (BENCHES / "variants/satfold-shared/src/lib.rs",
     "every arm is compared against an independent reference computed from the same input"),
    (BENCHES / "variants/bitpack-carrier-d16-control/src/lib.rs",
     "The byte-identity is not assumed"),
    (BENCHES / "variants/bitpack-wide-d16-control/src/lib.rs",
     "byte-identical to `bitpack-wide-d16`"),
]

fails = []

print("A. HEADINGS")
for fname, heading in HEADINGS:
    p = PANEL / fname
    if not p.exists():
        fails.append(f"MISSING FILE {fname}")
        print(f"  MISSING FILE  {fname}")
        continue
    ok = any(line.startswith(heading) for line in p.read_text(errors="replace").splitlines())
    print(f"  {'ok  ' if ok else 'FAIL'}  {fname} :: {heading}")
    if not ok:
        fails.append(f"{fname} :: {heading}")

print()
print("B. VERBATIM QUOTATIONS")
for fname, phrase in QUOTES:
    p = PANEL / fname
    if not p.exists():
        fails.append(f"MISSING FILE {fname}")
        print(f"  MISSING FILE  {fname}")
        continue
    text = p.read_text(errors="replace")
    ok = norm(phrase) in norm(text)
    print(f"  {'ok  ' if ok else 'FAIL'}  {fname} :: {phrase[:70]}")
    if not ok:
        fails.append(f"{fname} :: {phrase[:70]}")

print()
print("C. SOURCE ITEMS IN THE TREE")
for path, phrase in SOURCE_ITEMS:
    if not path.exists():
        fails.append(f"MISSING FILE {path}")
        print(f"  MISSING FILE  {path}")
        continue
    ok = norm(phrase) in norm(path.read_text(errors="replace"))
    print(f"  {'ok  ' if ok else 'FAIL'}  {path.name} :: {phrase[:60]}")
    if not ok:
        fails.append(f"{path.name} :: {phrase[:60]}")

print()
print(f"checked: {len(HEADINGS)} headings, {len(QUOTES)} quotations, {len(SOURCE_ITEMS)} source items")
print(f"failures: {len(fails)}")
for f in fails:
    print(f"  {f}")
sys.exit(0)
