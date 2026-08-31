//! Every classification arm driven against a built registry, and the controls
//! that must stay silent. A vocabulary checker whose tests only feed it bad
//! spellings reports that it can find the thing it was handed.

use std::collections::BTreeMap;

use mockspace::RegistryView;
use mockspace::tool::{Outcome, Tool, ToolContext};

use super::{CLOSING_RULING, RATIFIED, RoundingVocabulary, Standing, classify, modes_in};

fn view(rows: &[(&str, &[(&str, &str)])]) -> RegistryView {
    let mut r: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    for (q, fields) in rows {
        r.insert(
            (*q).to_string(),
            fields
                .iter()
                .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
                .collect(),
        );
    }
    RegistryView::new(r, BTreeMap::new())
}

/// The ruling every run checks its own copy against. Present in each test that
/// is not about the self-check, because without it every run is inconclusive
/// and every other assertion would pass for the wrong reason.
fn ruling() -> (&'static str, &'static [(&'static str, &'static str)]) {
    (
        CLOSING_RULING,
        &[(
            "says",
            "The rounding mode vocabulary is `toward_zero`, `floor`, `ceil`, `half_up`, \
             `half_even`, `stochastic`.",
        )],
    )
}

fn run(rows: &[(&str, &[(&str, &str)])], args: &[&str]) -> (Outcome, String) {
    let mut all: Vec<(&str, &[(&str, &str)])> = vec![ruling()];
    all.extend_from_slice(rows);
    let v = view(&all);
    let crates = Default::default();
    let dirs: Vec<std::path::PathBuf> = Vec::new();
    let ctx = ToolContext {
        mock_dir: std::path::Path::new("."),
        repo_root: std::path::Path::new("."),
        all_crates: &crates,
        src_dirs: &dirs,
        args,
        stdin: None,
        registry: &v,
    };
    let rep = RoundingVocabulary.run(&ctx);
    let text = match &rep.outcome {
        Outcome::Inconclusive { reason } => reason.clone(),
        _ => rep.output.clone(),
    };
    (rep.outcome, text)
}

/// How many entries the report says need attention, read off the line the
/// report prints rather than off the outcome. The outcome is `Clean` by
/// design: nothing here gates, so a findings verdict would block a commit
/// over a row whose correct repair nobody knows.
fn count(_outcome: &Outcome, out: &str) -> usize {
    out.lines()
        .find_map(|l| l.strip_suffix(" entr(y/ies) need attention."))
        .and_then(|n| n.trim().parse().ok())
        .unwrap_or(0)
}

// -------------------------------------------------------------------------
// The self-check, which gates every other result.
// -------------------------------------------------------------------------

#[test]
fn a_missing_closing_ruling_stops_the_run_rather_than_reporting_clean() {
    // Without the row there is no ratified set, so a clean verdict here would
    // say the canon agrees with a vocabulary nothing ratified.
    let v = view(&[("proposal::a", &[("predicate", "rounding: floor")])]);
    let crates = Default::default();
    let dirs: Vec<std::path::PathBuf> = Vec::new();
    let ctx = ToolContext {
        mock_dir: std::path::Path::new("."),
        repo_root: std::path::Path::new("."),
        all_crates: &crates,
        src_dirs: &dirs,
        args: &[],
        stdin: None,
        registry: &v,
    };
    let rep = RoundingVocabulary.run(&ctx);
    assert!(matches!(rep.outcome, Outcome::Inconclusive { .. }));
}

#[test]
fn a_ruling_that_dropped_a_name_stops_the_run_and_says_which() {
    let stale = vec![
        (
            CLOSING_RULING,
            &[(
                "says",
                "The rounding mode vocabulary is `toward_zero`, `floor`, `ceil`, `half_up`.",
            )][..],
        ),
        ("proposal::a", &[("predicate", "rounding: floor")][..]),
    ];
    let v = view(&stale);
    let crates = Default::default();
    let dirs: Vec<std::path::PathBuf> = Vec::new();
    let ctx = ToolContext {
        mock_dir: std::path::Path::new("."),
        repo_root: std::path::Path::new("."),
        all_crates: &crates,
        src_dirs: &dirs,
        args: &[],
        stdin: None,
        registry: &v,
    };
    let rep = RoundingVocabulary.run(&ctx);
    match rep.outcome {
        Outcome::Inconclusive { reason } => {
            assert!(reason.contains("half_even"), "{reason}");
            assert!(reason.contains("stochastic"), "{reason}");
            assert!(
                !reason.contains("`floor`"),
                "names only what is missing: {reason}"
            );
        }
        other => panic!("expected a refusal, got {other:?}"),
    }
}

// -------------------------------------------------------------------------
// Classification, arm by arm.
// -------------------------------------------------------------------------

#[test]
fn each_of_the_six_classifies_as_ratified() {
    for m in RATIFIED {
        assert_eq!(classify(m), Standing::Ratified, "{m}");
    }
}

#[test]
fn the_two_non_modes_are_not_reported_as_unknown() {
    // `exact` is a value of the axis by `dimension::rounding`'s own wording and
    // `any` is the notation's universal. Reporting either as an unknown mode
    // would put two legitimate forms in the section reserved for a canon
    // question.
    assert_eq!(classify("exact"), Standing::NotAMode);
    assert_eq!(classify("any"), Standing::NotAMode);
}

#[test]
fn an_alias_names_what_it_should_become() {
    assert_eq!(classify("toward zero"), Standing::Alias("toward_zero"));
    assert_eq!(classify("toward-zero"), Standing::Alias("toward_zero"));
    assert_eq!(classify("ceiling"), Standing::Alias("ceil"));
    assert_eq!(classify("nearest-half-up"), Standing::Alias("half_up"));
    assert_eq!(classify("nearest-half-even"), Standing::Alias("half_even"));
    assert_eq!(
        classify("round to nearest even"),
        Standing::Alias("half_even")
    );
}

#[test]
fn the_retired_word_is_caught_in_every_spelling_and_inside_a_phrase() {
    for m in ["truncate", "trunc", "truncation"] {
        assert_eq!(classify(m), Standing::Retired, "{m}");
    }
    // The phrase that disambiguates itself still carries the retired word, and
    // is still reported: the repair is an edit somebody makes, not one this
    // tool assumes.
    assert_eq!(classify("truncate toward zero"), Standing::Retired);
}

#[test]
fn a_word_merely_containing_a_retired_one_is_not_caught() {
    // Matching on a substring rather than a whole word would report any mode
    // whose name happened to contain the letters, and the report would then be
    // noise on rows with nothing wrong.
    assert_eq!(classify("untruncated"), Standing::Unknown);
}

#[test]
fn nearest_alone_is_underspecified_rather_than_an_alias() {
    // Two of the six are nearest-with-a-tie-rule, so mapping the bare word to
    // either would pick one on the row's behalf.
    assert_eq!(classify("nearest"), Standing::Underspecified);
}

#[test]
fn a_mode_outside_the_six_is_unknown() {
    assert_eq!(classify("away from zero"), Standing::Unknown);
}

#[test]
fn classification_ignores_case_and_surrounding_space() {
    assert_eq!(classify("  Floor "), Standing::Ratified);
    assert_eq!(classify("CEILING"), Standing::Alias("ceil"));
}

// -------------------------------------------------------------------------
// Reading a values side.
// -------------------------------------------------------------------------

#[test]
fn a_set_yields_every_member() {
    assert_eq!(
        modes_in("in {floor, ceiling, toward zero}"),
        ["floor", "ceiling", "toward zero"]
    );
}

#[test]
fn a_repeated_axis_name_is_stripped_rather_than_read_as_a_mode() {
    // Several rows are written `rounding: rounding = nearest`, which is the
    // axis grammar's own form. Reading the repeat as a mode would report
    // `rounding` itself as outside the six on every one of them.
    assert_eq!(modes_in("rounding = nearest"), ["nearest"]);
    assert_eq!(modes_in("rounding any"), ["any"]);
    assert_eq!(
        modes_in("rounding in {truncate, nearest}"),
        ["truncate", "nearest"]
    );
}

#[test]
fn a_trailing_clause_is_commentary_rather_than_a_second_mode() {
    // `rounding = nearest, against a phase-zero mutant` is one mode and a note
    // about the run. Splitting on the comma would invent a mode named after
    // the note.
    assert_eq!(
        modes_in("rounding = nearest, against a phase-zero mutant"),
        ["nearest"]
    );
}

#[test]
fn an_empty_values_side_names_no_mode() {
    assert!(modes_in("").is_empty());
    assert!(modes_in("   ").is_empty());
}

// -------------------------------------------------------------------------
// The whole run.
// -------------------------------------------------------------------------

#[test]
fn a_canon_with_no_rounding_entry_is_inconclusive_rather_than_clean() {
    // "Every mode is spelled right" and "I found no modes" are different
    // answers, and a tool that conflates them reports the canon as sound when
    // it has read nothing.
    let (outcome, _) = run(&[("proposal::a", &[("predicate", "threads: 1")])], &[]);
    assert!(matches!(outcome, Outcome::Inconclusive { .. }));
}

#[test]
fn control_a_canon_spelling_every_mode_correctly_is_clean() {
    let (outcome, out) = run(
        &[
            (
                "proposal::a",
                &[("predicate", "rounding: floor, threads: 1")],
            ),
            (
                "law::b",
                &[("holds", "rounding: in {toward_zero, half_even}")],
            ),
        ],
        &[],
    );
    assert!(matches!(outcome, Outcome::Clean { .. }), "{out}");
}

#[test]
fn a_laws_failing_region_is_read_as_well_as_its_holding_one() {
    // Easy to get wrong by walking `holds` alone, and every other arm here
    // would still pass.
    let (outcome, out) = run(&[("law::b", &[("fails", "rounding: ceiling")])], &[]);
    assert_eq!(count(&outcome, &out), 1, "{out}");
    assert!(out.contains("law::b"), "{out}");
}

#[test]
fn every_offending_row_is_reported_rather_than_the_first() {
    let (outcome, out) = run(
        &[
            ("proposal::a", &[("predicate", "rounding: ceiling")]),
            ("proposal::b", &[("predicate", "rounding: nearest")]),
            ("law::c", &[("holds", "rounding: away from zero")]),
        ],
        &[],
    );
    assert_eq!(count(&outcome, &out), 3, "{out}");
    for row in ["proposal::a", "proposal::b", "law::c"] {
        assert!(out.contains(row), "{row} missing from {out}");
    }
}

#[test]
fn the_four_kinds_are_reported_apart_from_each_other() {
    // The split is the whole product: one group is a mechanical edit, one needs
    // an instrument opened, one is a canon question. Collapsing them into a
    // count would leave a reader unable to act on any of it.
    let (_, out) = run(
        &[
            ("proposal::a", &[("predicate", "rounding: ceiling")]),
            ("proposal::b", &[("predicate", "rounding: truncate")]),
            ("proposal::c", &[("predicate", "rounding: nearest")]),
            ("proposal::d", &[("predicate", "rounding: away from zero")]),
        ],
        &[],
    );
    assert!(
        out.contains("A different spelling of one of the six (1)"),
        "{out}"
    );
    assert!(out.contains("The retired word (1)"), "{out}");
    assert!(
        out.contains("Names a distinction it does not make (1)"),
        "{out}"
    );
    assert!(out.contains("Outside the six (1)"), "{out}");
}

#[test]
fn an_alias_report_names_the_spelling_to_move_to() {
    let (_, out) = run(
        &[("proposal::a", &[("predicate", "rounding: ceiling")])],
        &[],
    );
    assert!(out.contains("`ceiling` -> `ceil`"), "{out}");
}

#[test]
fn a_clean_row_is_not_listed_under_any_heading() {
    let (_, out) = run(
        &[
            ("proposal::good", &[("predicate", "rounding: floor")]),
            ("proposal::bad", &[("predicate", "rounding: ceiling")]),
        ],
        &[],
    );
    assert!(out.contains("proposal::bad"), "{out}");
    assert!(
        !out.contains("proposal::good"),
        "a correct row is not a finding: {out}"
    );
}

#[test]
fn a_predicate_whose_values_carry_the_separator_is_not_cut_in_two() {
    // The shape most real predicates take. A reader splitting on the separator
    // alone would take `half_even}` for an entry and report the row twice.
    let (outcome, out) = run(
        &[(
            "law::b",
            &[(
                "holds",
                "signedness: in {unsigned, signed}, rounding: in {floor, half_even}, threads: 1",
            )],
        )],
        &[],
    );
    assert!(matches!(outcome, Outcome::Clean { .. }), "{out}");
}

// -------------------------------------------------------------------------
// The single-spelling query.
// -------------------------------------------------------------------------

#[test]
fn one_spelling_reports_only_the_rows_naming_it() {
    let (_outcome, out) = run(
        &[
            ("proposal::a", &[("predicate", "rounding: ceiling")]),
            ("proposal::b", &[("predicate", "rounding: nearest")]),
        ],
        &["ceiling"],
    );
    assert!(
        out.contains("1 entr"),
        "the count is of rows naming it: {out}"
    );
    assert!(out.contains("proposal::a"), "{out}");
    assert!(
        !out.contains("proposal::b"),
        "a row naming a different spelling is out of scope: {out}"
    );
}

#[test]
fn a_spelling_nothing_uses_is_inconclusive_and_says_why() {
    // Not "clean": the mode may be all over the canon under another spelling,
    // which is exactly the thing this tool exists to find.
    let (outcome, text) = run(
        &[("proposal::a", &[("predicate", "rounding: ceiling")])],
        &["stochastic"],
    );
    match outcome {
        Outcome::Inconclusive { .. } => {
            assert!(text.contains("another spelling"), "{text}");
        }
        other => panic!("expected inconclusive, got {other:?}"),
    }
}

#[test]
fn it_answers_to_the_name_the_catalogue_uses() {
    assert_eq!(RoundingVocabulary.name(), "rounding-vocabulary");
}
