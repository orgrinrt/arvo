//! Every arm driven against a built registry, including the ones that must not
//! fire. A tool whose only test is "it printed something" reports its own
//! iteration count.

use std::collections::BTreeMap;

use mockspace::tool::{Outcome, Tool, ToolContext};
use mockspace::RegistryView;

use super::AwaitingARuling;

/// A registry with the rows a test names, and the reverse edges it declares.
///
/// `edges` maps a row to the rows depending on it, which is what the engine
/// computes and what [`citers`](super::citers) reads. Passing an empty map gives
/// a view whose `referrers` answers empty for everything, which is why the
/// ordering test declares its edges explicitly rather than inferring them.
fn view(rows: &[(&str, &[(&str, &str)])], edges: &[(&str, &[&str])]) -> RegistryView {
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
    let mut e: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (q, who) in edges {
        e.insert(
            (*q).to_string(),
            who.iter().map(|s| (*s).to_string()).collect(),
        );
    }
    RegistryView::new(r, e)
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
    let rep = AwaitingARuling.run(&ctx);
    // An inconclusive verdict carries its reason on the outcome and leaves
    // `output` empty, so a test reading `output` alone sees nothing and cannot
    // tell a refusal from a silent pass. Surfacing both here is what lets the
    // assertions below be about what the tool said rather than about which
    // field it happened to use.
    let text = match &rep.outcome {
        Outcome::Inconclusive { reason } => reason.clone(),
        _ => rep.output.clone(),
    };
    (rep.outcome, text)
}

#[test]
fn an_empty_registry_is_inconclusive_rather_than_clean() {
    // The distinction that matters: "nothing is waiting" and "I could not tell"
    // are different answers, and reporting the second as the first is how a
    // tool says the canon is settled when it has read nothing.
    let (outcome, _) = run(&view(&[], &[]), &[]);
    assert!(
        matches!(outcome, Outcome::Inconclusive { .. }),
        "an empty registry must not report clean"
    );
}

#[test]
fn a_fully_ratified_registry_reports_nothing_waiting() {
    // The positive control for the whole tool. Without it every assertion below
    // is satisfied by a tool that reports everything as waiting always, which is
    // a prompt that fires forever and therefore gets ignored.
    let v = view(
        &[
            ("ruling::a", &[("rung", "ratified")]),
            ("ruling::b", &[("rung", "in_force")]),
            ("ruling::c", &[("rung", "open")]),
        ],
        &[],
    );
    let (outcome, out) = run(&v, &[]);
    assert!(matches!(outcome, Outcome::Clean { .. }), "{out}");
    assert!(out.contains("nothing is awaiting a ruling"), "{out}");
    assert!(
        !out.contains("owed"),
        "nothing is owed when nothing waits: {out}"
    );
}

#[test]
fn open_and_in_force_are_not_awaiting_a_ruling() {
    // `open` is op having explicitly declined to settle it and `in_force` is
    // enforced independently of him. Putting either to him is asking a question
    // he has already answered, which is the failure the whole workflow exists to
    // avoid, so it is pinned rather than left to the filter's spelling.
    let v = view(
        &[
            ("ruling::waiting", &[("rung", "stated")]),
            ("ruling::declined", &[("rung", "open")]),
            ("ruling::enforced", &[("rung", "in_force")]),
            ("ruling::blessed", &[("rung", "ratified")]),
        ],
        &[],
    );
    let (_, out) = run(&v, &[]);
    assert!(out.contains("waiting"), "{out}");
    for excluded in ["declined", "enforced", "blessed"] {
        assert!(
            !out.contains(excluded),
            "`{excluded}` must not be in the batch: {out}"
        );
    }
    assert!(out.contains("1 of 4"), "{out}");
}

#[test]
fn a_row_with_no_standing_field_is_not_swept_in() {
    // A missing field reads as `None`, and treating that as "awaiting" would put
    // a row to op on the strength of a schema gap rather than of anything he
    // said. The opposite default is the safe one: say nothing about a row that
    // does not declare where it stands.
    let v = view(&[("ruling::silent", &[("topic", "t")])], &[]);
    let (_, out) = run(&v, &[]);
    assert!(out.contains("nothing is awaiting"), "{out}");
}

#[test]
fn the_batch_threshold_reads_at_four_and_not_at_three() {
    // Both sides of op's number, because a threshold tested on one side is a
    // constant asserted against itself.
    let three: Vec<(&str, &[(&str, &str)])> = vec![
        ("ruling::a", &[("rung", "stated")]),
        ("ruling::b", &[("rung", "stated")]),
        ("ruling::c", &[("rung", "stated")]),
    ];
    let (_, out) = run(&view(&three, &[]), &[]);
    assert!(out.contains("hold until more"), "three must hold: {out}");
    assert!(!out.contains("owed"), "{out}");

    let four: Vec<(&str, &[(&str, &str)])> = vec![
        ("ruling::a", &[("rung", "stated")]),
        ("ruling::b", &[("rung", "stated")]),
        ("ruling::c", &[("rung", "stated")]),
        ("ruling::d", &[("rung", "stated")]),
    ];
    let (_, out) = run(&view(&four, &[]), &[]);
    assert!(out.contains("owed"), "four must be owed: {out}");
    assert!(!out.contains("hold until more"), "{out}");
}

#[test]
fn the_batch_is_ordered_by_what_depends_on_a_row() {
    // The ordering is the tool's only judgement, so it is the thing most worth
    // pinning. A row three others rest on is a more expensive thing to be wrong
    // about than one nothing has been built on.
    let v = view(
        &[
            ("ruling::lonely", &[("rung", "stated")]),
            ("ruling::popular", &[("rung", "stated")]),
            ("ruling::middling", &[("rung", "stated")]),
            ("ruling::filler", &[("rung", "stated")]),
        ],
        &[
            ("ruling::popular", &["proposal::x", "question::y", "law::z"]),
            ("ruling::middling", &["proposal::x"]),
        ],
    );
    let (_, out) = run(&v, &[]);
    let at = |s: &str| {
        out.find(s)
            .unwrap_or_else(|| panic!("{s} missing from {out}"))
    };
    assert!(at("popular") < at("middling"), "3 citers before 1: {out}");
    assert!(at("middling") < at("lonely"), "1 citer before 0: {out}");
    assert!(out.contains("(3 rows cite it)"), "{out}");
    assert!(
        out.contains("(1 row cites it)"),
        "singular, not `1 rows`: {out}"
    );
}

#[test]
fn a_row_read_by_slug_carries_his_words_apart_from_the_prose() {
    // The whole point of the single-row arm. What goes to op is usually the gap
    // between his quote and what somebody wrote on top of it, so both have to be
    // present and distinguishable in the output.
    let v = view(
        &[(
            "ruling::a_thing",
            &[
                ("rung", "stated"),
                ("quote", "his exact words here"),
                ("says", "the restatement somebody wrote"),
                ("note", "and the reading on top of it"),
            ],
        )],
        &[("ruling::a_thing", &["proposal::rests_on_it"])],
    );
    let (_, out) = run(&v, &["a_thing"]);
    assert!(out.contains("his exact words here"), "{out}");
    assert!(out.contains("the restatement somebody wrote"), "{out}");
    assert!(out.contains("and the reading on top of it"), "{out}");
    assert!(
        out.contains("proposal::rests_on_it"),
        "citers must be named: {out}"
    );
    assert!(out.contains("inherit"), "and what it costs them: {out}");
}

#[test]
fn an_unknown_slug_is_inconclusive_and_says_so_about_the_spelling() {
    // Not `Clean` with an empty body, which would read as "that row is fine".
    let v = view(&[("ruling::real", &[("rung", "stated")])], &[]);
    let (outcome, out) = run(&v, &["nonexistent"]);
    assert!(matches!(outcome, Outcome::Inconclusive { .. }), "{out}");
    assert!(out.contains("spelling"), "{out}");
}

#[test]
fn asking_about_an_already_settled_row_says_so_before_it_reaches_him() {
    // Reported rather than refused: the row is worth reading, and catching that
    // it is already ratified is exactly what this arm is for.
    let v = view(
        &[(
            "ruling::done",
            &[("rung", "ratified"), ("says", "settled long ago")],
        )],
        &[],
    );
    let (_, out) = run(&v, &["done"]);
    assert!(out.contains("Not awaiting a ruling"), "{out}");
    assert!(
        out.contains("settled long ago"),
        "still reported in full: {out}"
    );
}

#[test]
fn an_empty_field_is_not_printed_as_a_blank_heading() {
    // A row carrying `quote = ""` reads, in a naive rendering, as a row with a
    // quote. That is the same defect as the tool's own subject: something
    // present that means nothing, presented as if it meant something.
    let v = view(
        &[(
            "ruling::hollow",
            &[("rung", "stated"), ("quote", "   "), ("says", "real")],
        )],
        &[],
    );
    let (_, out) = run(&v, &["hollow"]);
    assert!(
        !out.contains("quote:"),
        "a whitespace quote is not a quote: {out}"
    );
    assert!(out.contains("says:"), "{out}");
}

#[test]
fn a_provenance_orders_by_the_panel_file_not_the_panel_directory() {
    use super::panel_number;
    // The trap this exists for: a provenance is
    // `panel::<dir>::<file>::<anchor>` and the directory leads with a twelve
    // digit timestamp. Taking the first number in the whole string takes that
    // timestamp, which is identical for every row in a panel, so every
    // comparison comes out equal and the ordering silently does nothing.
    assert_eq!(
        panel_number("panel::202608072330_the-numeral-canon-panel::95_op_the_panel_runs::#x"),
        Some(95),
        "the file's number, not the directory's timestamp"
    );
    assert_eq!(
        panel_number("panel::202608072330_the-numeral-canon-panel::37_op_warm_imitates::#y"),
        Some(37)
    );
    // A suffixed file number, which the panel uses when a slot is already taken.
    assert_eq!(
        panel_number("panel::202608072330_the-numeral-canon-panel::104b_op_something::#z"),
        Some(104)
    );
}

#[test]
fn a_provenance_that_cannot_be_ordered_reports_nothing() {
    use super::panel_number;
    // Reporting every op file when the ordering is unavailable would be noise
    // dressed as diligence, and a reader who is shown twenty files reads none.
    assert_eq!(panel_number("panel::somedir::no_number_here::#a"), None);
    assert_eq!(panel_number("not-a-provenance"), None);
    assert_eq!(panel_number("panel::only-two-parts"), None);
}

#[test]
fn a_file_number_is_read_from_the_front_and_nowhere_else() {
    use super::leading_number;
    assert_eq!(leading_number("95_op_the_panel_runs.md"), Some(95));
    assert_eq!(leading_number("7_op_early.md"), Some(7));
    // The control: a number that is not at the front is not the file's number.
    // Without this the helper could scan for any digits and would read `202608`
    // out of a filename carrying a date, which is the same defect as the one
    // above wearing a different coat.
    assert_eq!(leading_number("op_file_95.md"), None);
    assert_eq!(leading_number("README.md"), None);
    assert_eq!(leading_number(""), None);
}

#[test]
fn only_ops_own_files_count_as_his() {
    use super::is_op_file;
    // His own, including the suffixed slots the panel uses when a number is
    // already taken.
    assert!(is_op_file("95_op_the_panel_runs_to_ratification.md"));
    assert!(is_op_file("206_op_the_canon_test.md"));
    assert!(is_op_file("104b_op_the_imitation_is_ergonomic.md"));

    // The false positive that shipped for one run and diluted the list:
    // a member's file *about* his material, carrying none of his words.
    assert!(
        !is_op_file("207_mcsherry_op_material_in_the_dead_panel.md"),
        "a file about his material is not a file of his"
    );
    assert!(!is_op_file("201_mcsherry_is_the_bar_met.md"));
    assert!(!is_op_file("SEED_TALKING_POINTS.md"));
    assert!(
        !is_op_file("op_no_leading_number.md"),
        "the convention leads with a number"
    );
}
