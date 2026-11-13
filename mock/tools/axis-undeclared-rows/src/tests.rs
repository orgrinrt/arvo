//! The reader driven against a planted registry: the case that must be found,
//! the controls that must stay silent, and the shapes the first hand search
//! missed.

use std::collections::BTreeMap;

use mockspace::RegistryView;
use mockspace::tool::{Outcome, Tool, ToolContext};

use super::{AxisUndeclaredRows, says_an_axis_is_undeclared, sentences};

type Rows<'a> = &'a [(&'a str, &'a [(&'a str, &'a str)])];

fn view(rows: Rows) -> RegistryView {
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

/// Two declared dimensions every arm plants beside its rows.
const DIMENSIONS: [(&str, &[(&str, &str)]); 2] = [
    ("dimension::rounding", &[(
        "keywords",
        "[\"rounding\", \"tie\"]",
    )]),
    ("dimension::encoding_family", &[(
        "keywords",
        "[\"encoding\"]",
    )]),
];

fn run(rows: Rows, args: &[&str]) -> (Outcome, String) {
    let mut all: Vec<(&str, &[(&str, &str)])> = DIMENSIONS.to_vec();
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
    let rep = AxisUndeclaredRows.run(&ctx);
    let text = match &rep.outcome {
        Outcome::Inconclusive {
            reason,
        } => reason.clone(),
        _ => rep.output.clone(),
    };
    (rep.outcome, text)
}

/// The count the report states on its first line.
fn count(out: &str) -> usize {
    out.split_whitespace()
        .next()
        .and_then(|n| n.parse().ok())
        .unwrap_or(0)
}

// --- the case that must be found ---

#[test]
fn a_gap_saying_an_axis_is_undeclared_is_reported_with_the_dimension_beside_it() {
    let (_, out) = run(
        &[("law::a_law", &[(
            "gap",
            "The result is stated over rounding, and no `dimension` row declares an \
                 encoding axis, so it cannot state the other half.",
        )])],
        &[],
    );
    assert_eq!(count(&out), 1, "{out}");
    assert!(out.contains("law::a_law"), "{out}");
    assert!(out.contains("`gap`"), "{out}");
    assert!(
        out.contains("dimension::rounding") && out.contains("dimension::encoding_family"),
        "names the declared dimensions the sentence mentions: {out}"
    );
}

#[test]
fn a_note_is_read_as_well_as_a_gap() {
    let (_, out) = run(
        &[("proposal::p", &[(
            "note",
            "The assignment set is an undeclared axis here.",
        )])],
        &[],
    );
    assert_eq!(count(&out), 1, "{out}");
    assert!(out.contains("`note`"), "{out}");
}

/// The first hand search missed two because the phrase was split over a TOML
/// line continuation. The view holds the joined value, and a hard wrap or a
/// doubled space inside the phrase must not hide it either.
#[test]
fn a_phrase_split_over_a_line_break_is_still_found() {
    let (_, out) = run(
        &[("law::wrapped", &[(
            "gap",
            "No `dimension`\n   row declares   the range's geometry axis.",
        )])],
        &[],
    );
    assert_eq!(count(&out), 1, "{out}");
}

#[test]
fn the_row_argument_narrows_the_report_to_one_row() {
    let rows: Rows = &[
        ("law::a", &[("gap", "This axis is undeclared.")]),
        ("law::b", &[("gap", "That axis is undeclared too.")]),
    ];
    let (_, all) = run(rows, &[]);
    assert_eq!(count(&all), 2, "{all}");
    let (_, one) = run(rows, &["law::b"]);
    assert_eq!(count(&one), 1, "{one}");
    assert!(one.contains("law::b") && !one.contains("law::a"), "{one}");
}

// --- the controls that must stay silent ---

#[test]
fn control_a_sentence_about_something_else_is_left_alone() {
    let (_, out) = run(
        &[("law::a", &[(
            "gap",
            "The instrument is undeclared in the probe row. A dimension row exists for \
                 rounding.",
        )])],
        &[],
    );
    assert_eq!(
        count(&out),
        0,
        "undeclared with no axis word, and an axis word with no cue: {out}"
    );
}

#[test]
fn control_other_fields_are_not_read() {
    let (_, out) = run(
        &[("law::a", &[
            ("says", "This axis is undeclared."),
            ("because", "undeclared axis"),
        ])],
        &[],
    );
    assert_eq!(count(&out), 0, "{out}");
}

#[test]
fn control_a_row_with_no_dimension_beside_its_sentence_says_so() {
    let (_, out) = run(
        &[("law::a", &[(
            "gap",
            "No axis for the staged width is declared. It is an undeclared axis.",
        )])],
        &[],
    );
    assert!(
        out.contains("declared dimensions in the sentence: none"),
        "{out}"
    );
}

#[test]
fn control_a_registry_with_no_dimensions_is_inconclusive_not_clean() {
    let v = view(&[("law::a", &[("gap", "An undeclared axis.")])]);
    let crates = Default::default();
    let dirs: Vec<std::path::PathBuf> = Vec::new();
    let ctx = ToolContext {
        mock_dir:   std::path::Path::new("."),
        repo_root:  std::path::Path::new("."),
        all_crates: &crates,
        src_dirs:   &dirs,
        args:       &[],
        stdin:      None,
        registry:   &v,
    };
    let rep = AxisUndeclaredRows.run(&ctx);
    assert!(
        matches!(rep.outcome, Outcome::Inconclusive { .. }),
        "{:?}",
        rep.outcome
    );
}

#[test]
fn control_nothing_found_says_which_phrasings_were_read() {
    let (outcome, out) = run(&[("law::a", &[("gap", "Nothing of the kind.")])], &[]);
    assert!(matches!(outcome, Outcome::Clean { .. }), "{outcome:?}");
    assert!(out.contains("phrasings this reads"), "{out}");
}

// --- the reader's own pieces ---

#[test]
fn a_full_stop_inside_backticks_or_a_number_does_not_end_a_sentence() {
    let s = sentences("See `a.rs` and 0.5 here. Then this. And `x. y` too");
    assert_eq!(s.len(), 3, "{s:?}");
}

#[test]
fn the_cue_and_the_axis_word_are_both_needed() {
    assert!(says_an_axis_is_undeclared("This axis is undeclared"));
    assert!(says_an_axis_is_undeclared(
        "no `dimension` row declares it, an axis"
    ));
    assert!(!says_an_axis_is_undeclared("This axis is declared"));
    assert!(!says_an_axis_is_undeclared("The thing is undeclared"));
    assert!(!says_an_axis_is_undeclared(
        "The axisymmetric result is undeclared"
    ));
}
