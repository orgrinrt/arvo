#!/usr/bin/env python3
"""p11: every file:line in 107 is opened and its CONTENT tested, not merely
resolved. A citation landing two lines from its content still resolves, and
only reading the target and testing for an expected phrase catches it.

Whitespace is normalised and blockquote/doc-comment markers stripped on both
sides, because a quotation wrapped across lines or carried inside a `>` block
is still verbatim. That normalisation cannot make an absent phrase appear.
"""
import re, pathlib, sys

HERE = pathlib.Path(__file__).resolve().parent
PANEL = HERE.parent
ARVO = PANEL.parents[2]
WS = ARVO.parent

def norm(s):
    s = re.sub(r"^\s*(>|///|//!)\s?", "", s, flags=re.M)
    return re.sub(r"\s+", " ", s).strip()

# (what 107 claims, path relative to panel unless absolute-ish, line span, phrase)
CITES = [
 ("102 concedes the polarity rung in its own words",
  PANEL/"102_torvalds_does_the_mechanism_serve_the_intents.md", (255, 265),
  "would earn the rung. I did"),
 ("93's F9 complementary sentence, which I18 overturns",
  PANEL/"93_orchard_the_strategy_axis_derived_cold.md", (605, 618),
  "not a weaker check and not a debug-only one"),
 ("106 says its census script IS the set difference",
  PANEL/"106_giesen_consolidation_the_strategy_axis.md", (1225, 1232),
  "The census and the set difference are"),
 ("106 then assigns the set difference to the check after it",
  PANEL/"106_giesen_consolidation_the_strategy_axis.md", (1260, 1268),
  "What the check after this file should run"),
 ("87's sentence, both halves",
  PANEL/"87_op_the_canon_is_written_once_at_the_end.md", (24, 30),
  "a dropped item is a defect rather than a closed question"),
 ("98 on what a build arm may and may not move",
  PANEL/"98_spj_what_the_strategy_axis_settles.md", (476, 488),
  "For an **observable** one it is forbidden by the same section"),
 ("102 naming the build-condition seam as unnoticed",
  PANEL/"102_torvalds_does_the_mechanism_serve_the_intents.md", (150, 160),
  "These are disjoint sets of things and nothing has noticed"),
 ("97's criterion, stated",
  PANEL/"97_dolan_the_strategy_space_attacked.md", (705, 712),
  "respects every ordered nesting of operations the law contains"),
 ("the workspace rule as corrected during this unit",
  WS/".claude/rules/arvo-always-optimal-internals.md", (60, 70),
  "necessary and it is not sufficient, and this rule previously said it was"),
 ("102's keyword filter, the mechanism behind its false claim",
  PANEL/"102_probes/p1_the_corpus_compares_cost_at_a_fixed_answer.py", (84, 90),
  "agree|match|same|identical|disagree"),
 ("bitpack-shared's doc comment is the test helper's, at 264",
  ARVO/"mock/benches/variants/bitpack-shared/src/lib.rs", (262, 268),
  "Cross-checks both extraction paths against the logical ground truth"),
 ("the module doc's actual first line",
  ARVO/"mock/benches/variants/bitpack-shared/src/lib.rs", (1, 3),
  "Shared data model for the `Layout::Bitpacked` access-pattern bench"),
 ("the satfold medians 93 and 106 both cite",
  ARVO/"mock/benches/satfold-const-gate_n10000_findings.md", (92, 96),
  "38391ns"),
 ("97's F-H, the recovered-laws finding 106 drops",
  PANEL/"97_dolan_the_strategy_space_attacked.md", (798, 808),
  "declared non-negative operand window recovers additive"),
 ("97's F-B, the structural bound 106 drops",
  PANEL/"97_dolan_the_strategy_space_attacked.md", (283, 291),
  "polynomial against exponential in the number of regions"),
 ("98's F-98-5, the 47x bound on the counts",
  PANEL/"98_spj_what_the_strategy_axis_settles.md", (851, 859),
  "varies by a factor of 47"),
 ("98's F-98-7, a hard bound has no weighting",
  PANEL/"98_spj_what_the_strategy_axis_settles.md", (869, 876),
  "is not\nrealisable by any non-negative weighting"),
 ("98 section 4.1, op's four refusals",
  PANEL/"98_spj_what_the_strategy_axis_settles.md", (518, 528),
  "Four intents, four refusals of the absolute reading"),
 ("102 keeping 98's exchange-rate reading independently",
  PANEL/"102_torvalds_does_the_mechanism_serve_the_intents.md", (707, 714),
  "exchange-rate reading of op's four intents"),
 ("102's own table conceding I5's bar",
  PANEL/"102_torvalds_does_the_mechanism_serve_the_intents.md", (565, 572),
  "wants a sound-against-unsound bench that does not exist"),
 ("93's F1 predicate, carrying the signedness 106 drops",
  PANEL/"93_orchard_the_strategy_axis_derived_cold.md", (542, 550),
  "signedness = unsigned"),
 ("99 recording the rule as a live licence to emit a wrong rewrite",
  PANEL/"99_persona_checkpoint_eight.md", (1, 200),
  "That was a live licence to emit a wrong rewrite"),
 ("22's -dirty precondition, uncited in the unit",
  PANEL/"22_xu_the_bench_that_was_missing.md", (185, 196),
  "the second size row of any run is dirty"),
]

ok = bad = 0
for label, path, (a, b), phrase in CITES:
    if not path.exists():
        print(f"FAIL  {label}: path does not exist: {path}"); bad += 1; continue
    lines = path.read_text(errors="replace").splitlines()
    seg = norm("\n".join(lines[a-1:b]))
    if norm(phrase) in seg:
        ok += 1
    else:
        print(f"FAIL  {label}\n      {path.name}:{a}-{b}\n      wanted: {norm(phrase)[:90]}")
        bad += 1

print(f"\ncitations checked: {ok+bad}   ok: {ok}   failed: {bad}")

# mutation test: the checker must be able to fail.
print("\n--- mutation test, three ways ---")
for lbl, path, span, phrase in [
    ("a phrase op did not say", CITES[4][1], (24,30), "a dropped item is fine"),
    ("a real phrase at the wrong span", CITES[7][1], (10,20), "respects every ordered nesting"),
    ("a real phrase in the wrong file", CITES[0][1], (255,265), "Shared data model for the"),
]:
    lines = path.read_text(errors="replace").splitlines()
    seg = norm("\n".join(lines[span[0]-1:span[1]]))
    print(f"  {'CAUGHT ' if norm(phrase) not in seg else 'MISSED '} {lbl}")
sys.exit(1 if bad else 0)
