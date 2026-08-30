#!/usr/bin/env python3
"""Every quotation and named claim in 106 is opened and its CONTENT tested, not
merely resolved. 25_probes/ established this as the cheapest correctness tool
the panel has, and it has fired on every member file that ran it.

Each row is (what 106 claims, where, a phrase that must be present at the
target). Whitespace is normalised on both sides and leading blockquote markers
are stripped, because a verbatim quotation wrapped across lines in markdown, or
carried inside a `>` block, is still verbatim and a raw substring test rejects
it for a reason unrelated to accuracy. Neither repair can make an absent phrase
appear. Run from the panel directory.
"""
import re, sys, os

def norm(s):
    s = re.sub(r'^\s*>\s?', '', s, flags=re.M)   # blockquote markers
    s = re.sub(r'^\s*//[/!]?\s?', '', s, flags=re.M)  # doc-comment prefixes
    return re.sub(r'\s+', ' ', s).strip()

C = [
 ("I1 demotes the set to open, op's words",
  "INTENTS.md", "the strategy set is not closed at exactly four"),
 ("I16 forbids policing law shapes",
  "INTENTS.md", "We shouldn't police what kind of laws there are or what shapes they take"),
 ("I18 bounds the panic to dev and debug",
  "INTENTS.md", "Dev and debug only. It does not survive into a release artifact"),
 ("I13 is the ratified predicated-arms entry",
  "INTENTS.md", "we are not writing a generalization, rather a bunch of arms"),
 ("I15 is never any runtime checks",
  "INTENTS.md", "Never any runtime checks, ever"),
 ("op: I3 is ergonomics",
  "104_op_the_imitation_is_ergonomic_and_i9_is_not_his_to_settle.md", "Neither, it's ergonomics"),
 ("op declines Q50",
  "104_op_the_imitation_is_ergonomic_and_i9_is_not_his_to_settle.md",
  "this is impl detail that already had answer: optimal and converged to by experts"),
 ("op: notko renames",
  "105_op_notko_renames.md", "notko renames"),
 ("op at 95: a unit must converge",
  "95_op_the_panel_runs_to_ratification_and_units_must_converge.md",
  "has to end with solutions and agreements at least with something"),
 ("102 says its polarity agreement is inherited",
  "102_torvalds_does_the_mechanism_serve_the_intents.md", "would earn the rung. I did\nnot."),
 ("102's bitpack-shared claim, which 106 refutes",
  "102_torvalds_does_the_mechanism_serve_the_intents.md",
  "no cross-arm agreement assertion of\neither kind"),
 ("102's pair, stated",
  "102_torvalds_does_the_mechanism_serve_the_intents.md",
  "an assignment on the observable policy axes"),
 ("102 concedes 25 on polarity not on counting",
  "102_torvalds_does_the_mechanism_serve_the_intents.md",
  "three of `25`'s four axes change the value the program computes and one does not"),
 ("102's ring boundary",
  "102_torvalds_does_the_mechanism_serve_the_intents.md",
  "become visible at the first step that is not a ring operation"),
 ("103 inherits the wrong half",
  "103_mcsherry_what_the_corpus_can_and_cannot_show.md",
  "That is right about the tests and it understates"),
 ("103's arm-by-arm validation shape",
  "103_mcsherry_what_the_corpus_can_and_cannot_show.md",
  "validated arm by arm, each against its own\ndeclared semantics"),
 ("103: nobody attacked the generate-check fork",
  "103_mcsherry_what_the_corpus_can_and_cannot_show.md", "Nobody attacked the fork itself"),
 ("103: barrier is the absent coordinate",
  "103_mcsherry_what_the_corpus_can_and_cannot_show.md",
  "The barrier is the absent coordinate, not the arm sets"),
 ("103's 53.72% on identical exact input",
  "103_mcsherry_what_the_corpus_can_and_cannot_show.md", "53.7220%"),
 ("94 names only the bijection",
  "94_wingo_the_strategy_axis_derived_cold.md",
  "The body does assert a real property (the\npermutation is a bijection)"),
 ("94's W8, ulp bound not preserved by multiplication",
  "94_wingo_the_strategy_axis_derived_cold.md",
  "is preserved by addition and is not\npreserved by multiplication"),
 ("94's correctness-vs-profitability predicate",
  "94_wingo_the_strategy_axis_derived_cold.md",
  "a correctness predicate must be const, and a profitability predicate merely wants to be"),
 ("97 reads bitpack-shared as redundancy",
  "97_dolan_the_strategy_space_attacked.md", "they are a **redundancy**"),
 ("97's three-order conservatism result",
  "97_dolan_the_strategy_space_attacked.md",
  "wrapping and saturating are incomparable in three of the four configurations"),
 ("97 on the observable layer reporting",
  "97_dolan_the_strategy_space_attacked.md",
  "On the observable mechanism coordinates: no join exists, so the operation reports"),
 ("97: the closure is the free join semilattice",
  "97_dolan_the_strategy_space_attacked.md", "the free join\nsemilattice on `d` generators"),
 ("98's nothing-to-check clause, which 106 corrects",
  "98_spj_what_the_strategy_axis_settles.md",
  "so there is\n  nothing to check and nothing to police"),
 ("98's strict-positivity theorem",
  "98_spj_what_the_strategy_axis_settles.md",
  "A weighting whose coordinates all carry strictly positive weight cannot select a Pareto-dominated arm"),
 ("93's F9 complementary sentence, overturned by I18",
  "93_orchard_the_strategy_axis_derived_cold.md",
  "not a weaker check and not a debug-only one"),
 ("93's escalation cost",
  "93_orchard_the_strategy_axis_derived_cold.md", "Nobody asked and everybody pays"),
 ("93's sixth axis",
  "93_orchard_the_strategy_axis_derived_cold.md", "Reproducibility across targets and builds"),
 ("93's unregistered encoding fork",
  "93_orchard_the_strategy_axis_derived_cold.md",
  "Neither is obviously right and the register\ndoes not carry the fork"),
 ("100's symbol alias",
  "100_xu_generating_the_table_attacked.md", "_e2_weighted = _e1_named"),
 ("100's 0-of-489 detector result",
  "100_xu_generating_the_table_attacked.md", "0 of 190"),
 ("100's control-pair calibration floor",
  "100_xu_generating_the_table_attacked.md", "median 0.273%, max 0.544%"),
 ("100 on P4 having no region dimension",
  "100_xu_generating_the_table_attacked.md", "a cost per arm with **no region dimension**"),
 ("101's inexpressibility sentence",
  "101_wronski_the_cost_coordinates.md",
  "is not unmeasured. It is inexpressible"),
 ("101's region-against-cost-vector line",
  "101_wronski_the_cost_coordinates.md",
  "A quantity belongs in the **region** when a strategy's answer may differ across it"),
 ("101's divergence-coordinate reading of I3, overturned by 104",
  "101_wronski_the_cost_coordinates.md", "Both readings need a **divergence**\ncoordinate"),
 ("101's anti-correlation of the two estimator tests",
  "101_wronski_the_cost_coordinates.md", "-0.641"),
 ("101's normalisation change-of-basis result",
  "101_wronski_the_cost_coordinates.md", "identical sections: 2000/2000"),
 ("101 on p95 reaching one section",
  "101_wronski_the_cost_coordinates.md",
  "`{median, p95}` reaches exactly one section"),
 ("22 diagnosed the -dirty suffix as harness noise",
  "22_xu_the_bench_that_was_missing.md",
  "harness writes its artifacts into the tree it then hashes"),
 ("bitpack-shared asserts both arms against ground truth",
  "../../benches/variants/bitpack-shared/src/lib.rs",
  'assert_eq!(a, expect, "aligned mismatch'),
 ("bitpack-shared's second arm too",
  "../../benches/variants/bitpack-shared/src/lib.rs",
  'assert_eq!(z, expect, "zeropad mismatch'),
 ("the carrier control names its check",
  "../../benches/variants/bitpack-carrier-d16-control/src/lib.rs",
  "The byte-identity is not assumed"),
 ("the wide control names none",
  "../../benches/variants/bitpack-wide-d16-control/src/lib.rs",
  "The noise floor: byte-identical to `bitpack-wide-d16`"),
 ("101 measured the wide pair differing",
  "101_probes/p0_control_identity_on_every_pair.out", "DIFFER"),
 ("satfold gate-true median",
  "../../benches/satfold-const-gate_n10000_findings.md", "1438.1 ns"),
]

ok = bad = 0
for what, path, phrase in C:
    if not os.path.exists(path):
        print(f"FAIL  [missing file] {path}  ({what})"); bad += 1; continue
    body = norm(open(path, encoding="utf-8", errors="replace").read())
    if norm(phrase) in body:
        ok += 1
    else:
        print(f"FAIL  {path}\n      for: {what}\n      wanted: {norm(phrase)[:110]}")
        bad += 1

print(f"\ncitations checked: {len(C)}   ok: {ok}   failed: {bad}")
sys.exit(1 if bad else 0)
