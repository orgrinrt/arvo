//! Every arm driven against a built registry, including the ones that must not
//! fire. A tool whose only test is "it printed something" reports its own
//! iteration count.

use std::collections::BTreeMap;

use mockspace::tool::{NotALint, Outcome, Tool, ToolContext};
use mockspace::RegistryView;

use super::{has_no_verbatim, RulingsWithNoVerbatim};

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

fn run(v: &RegistryView, args: &[&str]) -> (Outcome, String) {
    let crates = Default::default();
    let dirs: Vec<std::path::PathBuf> = Vec::new();
    let ctx = ToolContext {
        mock_dir: std::path::Path::new("."),
        repo_root: std::path::Path::new("."),
        all_crates: &crates,
        src_dirs: &dirs,
        args,
        stdin: None,
        registry: v,
    };
    let rep = RulingsWithNoVerbatim.run(&ctx);
    let text = match &rep.outcome {
        Outcome::Inconclusive { reason } => reason.clone(),
        _ => rep.output.clone(),
    };
    (rep.outcome, text)
}

/// The predicate alone, over one planted row.
fn holed(fields: &[(&str, &str)]) -> bool {
    let v = view(&[("ruling::subject", fields)]);
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
    has_no_verbatim(&ctx, "ruling::subject")
}

#[test]
fn a_row_carrying_his_words_is_not_a_hole_and_one_without_is() {
    assert!(!holed(&[
        ("says", "he took the third"),
        ("quote", "the third one")
    ]));
    assert!(holed(&[("says", "he took the third")]));
}

#[test]
fn a_quote_that_is_only_whitespace_is_no_verbatim() {
    // The field being present is not the thing asked for. A row carrying an
    // empty `quote` satisfies a predicate written as "has the field" and tells a
    // reader nothing.
    for blank in ["", " ", "\n", "\t  \n"] {
        assert!(holed(&[("says", "x"), ("quote", blank)]), "{blank:?}");
    }
}

#[test]
fn control_a_ruling_the_experts_ratified_is_out_of_scope() {
    // The whole discrimination. Such a row never passed through him, so there is
    // no verbatim to have lost, and a tool reporting it would be listing the
    // ratification route rather than a hole. Every other arm here plants a row
    // with no `ratified_by` and would pass through this defect.
    assert!(!holed(&[
        ("says", "the experts converged"),
        ("ratified_by", "experts")
    ]));
    // And the other values do not get the carve-out.
    assert!(holed(&[("says", "he stamped it"), ("ratified_by", "both")]));
    assert!(holed(&[("says", "he stamped it"), ("ratified_by", "op")]));
}

#[test]
fn a_proposal_is_not_read_at_all() {
    // A `proposal` carries no `quote` by construction, because there are no
    // words but the panel's. Reading that namespace would report all of it.
    let v = view(&[
        ("ruling::his_own", &[("says", "x"), ("quote", "y")]),
        ("proposal::a_claim", &[("says", "the panel's own words")]),
    ]);
    let (outcome, text) = run(&v, &[]);
    assert!(matches!(outcome, Outcome::Clean { examined: 1 }), "{text}");
    assert!(!text.contains("a_claim"), "{text}");
}

#[test]
fn the_report_names_every_hole_rather_than_the_first() {
    let v = view(&[
        ("ruling::a", &[("says", "x")]),
        ("ruling::b", &[("says", "y")]),
        ("ruling::c", &[("says", "z"), ("quote", "his words")]),
    ]);
    let (_, text) = run(&v, &[]);
    assert!(text.contains("2 of 3"), "{text}");
    assert!(text.contains("  a\n"), "{text}");
    assert!(text.contains("  b\n"), "{text}");
    assert!(!text.contains("  c\n"), "{text}");
}

#[test]
fn the_report_shows_whether_somebody_has_already_looked() {
    // The distinction between a hole nobody has examined and one whose reason is
    // written down. Both are listed, because a note is prose and this is a list,
    // and only the display separates them.
    let v = view(&[
        (
            "ruling::examined",
            &[
                ("says", "x"),
                ("note", "The corpus holds no verbatim; this is the record."),
            ],
        ),
        ("ruling::unexamined", &[("says", "y")]),
    ]);
    let (_, text) = run(&v, &[]);
    assert!(text.contains("The corpus holds no verbatim"), "{text}");
    assert!(text.contains("no note either"), "{text}");
}

#[test]
fn a_canon_where_every_ruling_carries_his_words_says_so_rather_than_printing_nothing() {
    let v = view(&[("ruling::a", &[("says", "x"), ("quote", "his words")])]);
    let (outcome, text) = run(&v, &[]);
    assert!(matches!(outcome, Outcome::Clean { examined: 1 }), "{text}");
    assert!(text.contains("every one of the 1 rulings"), "{text}");
}

#[test]
fn a_registry_with_no_rulings_is_inconclusive_rather_than_clean() {
    // A clean verdict over an empty population is vacuous, and this is the
    // whole reason the third outcome exists.
    let (outcome, text) = run(&view(&[]), &[]);
    assert!(matches!(outcome, Outcome::Inconclusive { .. }), "{text}");
}

#[test]
fn one_row_can_be_read_in_full_by_its_slug() {
    let v = view(&[
        (
            "ruling::a",
            &[("says", "the thing"), ("note", "the reason")],
        ),
        ("ruling::b", &[("says", "another")]),
    ]);
    let (outcome, text) = run(&v, &["a"]);
    assert!(matches!(outcome, Outcome::Clean { examined: 1 }), "{text}");
    assert!(text.contains("the thing"), "{text}");
    assert!(text.contains("the reason"), "{text}");
    assert!(text.contains("rests on his words: no"), "{text}");
    assert!(!text.contains("another"), "{text}");
}

#[test]
fn a_slug_that_names_nothing_is_inconclusive_rather_than_an_empty_report() {
    let v = view(&[("ruling::a", &[("says", "x")])]);
    let (outcome, text) = run(&v, &["nosuch"]);
    assert!(matches!(outcome, Outcome::Inconclusive { .. }), "{text}");
    assert!(text.contains("spelling"), "{text}");
}

#[test]
fn it_declares_itself_as_the_shape_it_is_and_no_run_returns_a_blocking_finding() {
    // The contract's own enforcement on a `no-failing-case` tool: a run may not
    // return a finding that blocks a gate. Driven over the registries rather
    // than asserted about the declaration alone, because the declaration is what
    // the tool says and the outcome is what it does.
    assert!(matches!(
        RulingsWithNoVerbatim.not_a_lint(),
        NotALint::NoFailingCase
    ));
    for v in [
        view(&[]),
        view(&[("ruling::a", &[("says", "x")])]),
        view(&[("ruling::a", &[("says", "x"), ("quote", "y")])]),
    ] {
        let (outcome, text) = run(&v, &[]);
        assert!(
            !matches!(outcome, Outcome::Findings(_)),
            "a no-failing-case tool returned findings: {text}"
        );
    }
}

#[test]
fn it_answers_to_the_name_the_subcommand_uses() {
    assert_eq!(RulingsWithNoVerbatim.name(), "rulings-with-no-verbatim");
}
