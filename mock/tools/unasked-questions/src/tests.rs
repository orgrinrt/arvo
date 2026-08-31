//! Every arm driven against a built registry, including the ones that must not
//! fire. A tool whose only test is "it printed something" reports its own
//! iteration count.

use std::collections::BTreeMap;

use mockspace::RegistryView;
use mockspace::tool::{Outcome, Tool, ToolContext};

use super::UnaskedQuestions;

/// A registry with the rows a test names, and the reverse edges it declares.
///
/// `edges` maps a row to the rows depending on it, which is what the engine
/// computes and what [`answered_by`](super::answered_by) reads. An empty map
/// gives a view whose `referrers` answers empty for everything, so every test
/// about settledness declares its edges rather than inferring them from an
/// `answers` field the view does not interpret.
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
    let rep = UnaskedQuestions.run(&ctx);
    // An inconclusive verdict carries its reason on the outcome and leaves
    // `output` empty, so a test reading `output` alone cannot tell a refusal
    // from a silent pass.
    let text = match &rep.outcome {
        Outcome::Inconclusive { reason } => reason.clone(),
        _ => rep.output.clone(),
    };
    (rep.outcome, text)
}

const HIS: &[(&str, &str)] = &[("decider", "op"), ("asks", "Which one?")];

#[test]
fn an_empty_registry_is_inconclusive_rather_than_clean() {
    // "Nothing is waiting on him" and "I could not tell" are different answers,
    // and reporting the second as the first is how a tool says the canon is
    // settled when it has read nothing.
    let (outcome, _) = run(&view(&[], &[]), &[]);
    assert!(
        matches!(outcome, Outcome::Inconclusive { .. }),
        "an empty registry must not report clean"
    );
}

#[test]
fn a_registry_where_nothing_names_him_reports_nothing_waiting() {
    // The positive control for the whole tool. Without it every assertion below
    // is satisfied by one that reports everything as waiting always, which is a
    // prompt that fires forever and gets ignored.
    let (outcome, text) = run(
        &view(
            &[
                ("question::a", &[("decider", "panel")]),
                ("question::b", &[("decider", "measurement")]),
            ],
            &[],
        ),
        &[],
    );
    assert!(matches!(outcome, Outcome::Clean { examined: 2 }));
    assert!(
        text.contains("Nothing is waiting on him"),
        "expected the nothing-waiting report, got:\n{text}"
    );
}

#[test]
fn a_question_naming_him_with_no_ruling_is_reported_as_waiting() {
    let (_, text) = run(&view(&[("question::open_one", HIS)], &[]), &[]);
    assert!(text.contains("open_one"), "{text}");
    assert!(
        text.contains("1 still waiting on him"),
        "expected a count of one, got:\n{text}"
    );
}

/// The whole reason this tool exists. A ruling answering a question settles it,
/// the row keeps naming him because he is who settled it, and every roster built
/// by reading `decider` alone puts it back in the queue.
#[test]
fn a_question_a_ruling_answers_is_excluded_and_said_to_be() {
    let (_, text) = run(
        &view(
            &[
                ("question::settled_one", HIS),
                ("question::open_one", HIS),
                ("ruling::he_said_so", &[("rung", "stated")]),
            ],
            &[("question::settled_one", &["ruling::he_said_so"])],
        ),
        &[],
    );
    assert!(
        text.contains("1 still waiting on him"),
        "the settled one must not be counted as waiting:\n{text}"
    );
    assert!(
        text.contains("Excluded, because a ruling answers each"),
        "exclusion is reported rather than silent:\n{text}"
    );
    assert!(
        text.contains("settled_one   <- he_said_so"),
        "the report names which ruling settles it:\n{text}"
    );
}

/// A proposal carries an `answers` field too, and the schema is explicit that it
/// does not settle what it names while it is a proposal. A namespace-blind
/// reading of the reverse edges would hide exactly the questions most likely to
/// look answered and not be.
#[test]
fn a_proposal_answering_a_question_does_not_settle_it() {
    let (_, text) = run(
        &view(
            &[
                ("question::open_one", HIS),
                ("proposal::the_panel_thinks", &[("says", "probably this")]),
            ],
            &[("question::open_one", &["proposal::the_panel_thinks"])],
        ),
        &[],
    );
    assert!(
        text.contains("1 still waiting on him"),
        "a proposal is not a ruling and settles nothing:\n{text}"
    );
    assert!(
        !text.contains("Excluded"),
        "nothing was excluded, so nothing should be reported as excluded:\n{text}"
    );
}

/// A row whose only referrer is a ruling in a namespace that merely starts with
/// the same letters. `starts_with` on a bare name would match `rulings::` and
/// anything else sharing the prefix, so the separator is part of the test.
#[test]
fn a_namespace_merely_starting_with_the_same_letters_does_not_settle() {
    let (_, text) = run(
        &view(
            &[
                ("question::open_one", HIS),
                ("rulingsketch::not_a_ruling", &[("says", "a note")]),
            ],
            &[("question::open_one", &["rulingsketch::not_a_ruling"])],
        ),
        &[],
    );
    assert!(
        text.contains("1 still waiting on him"),
        "only the `ruling` namespace settles:\n{text}"
    );
}

/// The schema's own rule, in the `bound` field's description: a row carrying a
/// bound and still naming him is a filing error, because a bound is written at
/// the moment the decider moved off him.
#[test]
fn a_row_carrying_a_bound_and_naming_him_is_reported_at_the_top() {
    let (_, text) = run(
        &view(
            &[(
                "question::handed_back",
                &[
                    ("decider", "op"),
                    ("asks", "Which set?"),
                    ("bound", "Put to him and returned."),
                ],
            )],
            &[],
        ),
        &[],
    );
    assert!(
        text.starts_with("1 row(s) carry a `bound` and still name op"),
        "the filing error leads the report:\n{text}"
    );
    assert!(text.contains("handed_back"), "{text}");
}

#[test]
fn a_bound_on_a_row_naming_the_panel_is_not_a_filing_error() {
    let (_, text) = run(
        &view(
            &[(
                "question::handed_back",
                &[("decider", "panel"), ("bound", "Put to him and returned.")],
            )],
            &[],
        ),
        &[],
    );
    assert!(
        !text.contains("filing error"),
        "a returned question naming the panel is the repair, not the defect:\n{text}"
    );
}

/// Ordering is a suggestion and is still asserted, because an ordering nobody
/// checks is an ordering that silently stops holding.
#[test]
fn a_question_naming_what_it_unblocks_sorts_above_one_that_does_not() {
    let (_, text) = run(
        &view(
            &[
                ("question::names_nothing", HIS),
                (
                    "question::names_something",
                    &[
                        ("decider", "op"),
                        ("asks", "Which one?"),
                        ("unblocks", "Four topics."),
                    ],
                ),
            ],
            &[],
        ),
        &[],
    );
    let a = text.find("names_something").expect("both are listed");
    let b = text.find("names_nothing").expect("both are listed");
    assert!(
        a < b,
        "the one naming what it unblocks comes first:\n{text}"
    );
    assert!(
        text.contains("unblocks nothing named"),
        "and the other says why it is last:\n{text}"
    );
}

/// An `unblocks` present but empty is the same as absent. A field somebody added
/// and left blank must not buy a place at the top of the queue.
#[test]
fn an_empty_unblocks_does_not_count_as_naming_something() {
    let (_, text) = run(
        &view(
            &[(
                "question::blank",
                &[("decider", "op"), ("asks", "Which?"), ("unblocks", "   ")],
            )],
            &[],
        ),
        &[],
    );
    assert!(
        text.contains("unblocks nothing named"),
        "whitespace is not a statement of what it unblocks:\n{text}"
    );
}

#[test]
fn more_referrers_sorts_higher_among_rows_that_all_unblock_something() {
    let with = &[
        ("decider", "op"),
        ("asks", "Which one?"),
        ("unblocks", "Something."),
    ];
    let (_, text) = run(
        &view(
            &[
                ("question::few", with),
                ("question::many", with),
                ("proposal::one", &[("says", "x")]),
                ("proposal::two", &[("says", "y")]),
            ],
            &[
                ("question::many", &["proposal::one", "proposal::two"]),
                ("question::few", &["proposal::one"]),
            ],
        ),
        &[],
    );
    let a = text.find("  many").expect("both are listed");
    let b = text.find("  few").expect("both are listed");
    assert!(a < b, "more referrers comes first:\n{text}");
}

#[test]
fn a_batch_at_or_past_four_says_a_round_is_owed_and_below_it_says_hold() {
    let four: Vec<(&str, &[(&str, &str)])> = vec![
        ("question::a", HIS),
        ("question::b", HIS),
        ("question::c", HIS),
        ("question::d", HIS),
    ];
    let (_, text) = run(&view(&four, &[]), &[]);
    assert!(text.contains("a round of asking is owed"), "{text}");

    let (_, text) = run(&view(&four[..3], &[]), &[]);
    assert!(text.contains("Fewer than the 4"), "{text}");
}

#[test]
fn an_unknown_slug_is_inconclusive_and_says_it_is_about_the_spelling() {
    let (outcome, text) = run(&view(&[("question::open_one", HIS)], &[]), &["nope"]);
    assert!(matches!(outcome, Outcome::Inconclusive { .. }));
    assert!(text.contains("spelling"), "{text}");
}

#[test]
fn one_row_reports_its_fields_and_whether_anything_answered_it() {
    let (_, text) = run(
        &view(
            &[
                (
                    "question::open_one",
                    &[
                        ("decider", "op"),
                        ("asks", "Which reading?"),
                        ("unblocks", "The chain topic."),
                    ],
                ),
                ("question::settled_one", HIS),
                ("ruling::he_said_so", &[("rung", "stated")]),
            ],
            &[("question::settled_one", &["ruling::he_said_so"])],
        ),
        &["open_one"],
    );
    assert!(text.contains("Which reading?"), "{text}");
    assert!(text.contains("The chain topic."), "{text}");
    assert!(
        text.contains("Nothing references it"),
        "an unreferenced row says so, which is the finding:\n{text}"
    );

    let (_, text) = run(
        &view(
            &[
                ("question::settled_one", HIS),
                ("ruling::he_said_so", &[("rung", "stated")]),
            ],
            &[("question::settled_one", &["ruling::he_said_so"])],
        ),
        &["settled_one"],
    );
    assert!(
        text.contains("Answered.") && text.contains("he_said_so"),
        "asking about a settled row must say so before it reaches him:\n{text}"
    );
}

/// The qualified form addresses the same row as the bare slug. A tool answering
/// only one of the two spellings is one a caller has to guess at.
#[test]
fn a_row_is_addressable_by_slug_or_by_qualified_name() {
    let v = view(&[("question::open_one", HIS)], &[]);
    let (_, by_slug) = run(&v, &["open_one"]);
    let (_, by_qualified) = run(&v, &["question::open_one"]);
    assert_eq!(by_slug, by_qualified);
}

#[test]
fn a_question_answered_on_its_own_row_is_settled() {
    // The defect this closes, and it is the tool's own subject one level up: it
    // detected settlement only through a ruling's `answers` edge, and not every
    // answer of his mints a ruling. Three were settled in one round where one
    // refined a bound, one reshaped the question rather than picking any option
    // it was given, and one was a plain yes. All three would have gone back to
    // him in the next batch, from the tool built to stop exactly that.
    let v = view(
        &[
            (
                "question::settled_on_the_row",
                &[
                    ("decider", "op"),
                    ("answered", "he reshaped it rather than picking an option"),
                ],
            ),
            ("question::still_open", &[("decider", "op")]),
        ],
        &[],
    );
    let (_, out) = run(&v, &[]);
    assert!(out.contains("still_open"), "{out}");
    assert!(
        !out.contains("settled_on_the_row") || out.contains("Excluded"),
        "an answered row is excluded rather than offered: {out}"
    );
}

#[test]
fn an_empty_answered_field_does_not_settle_anything() {
    // The control, and it is the one that matters: a field present but blank is
    // how a half-written repair reads, and treating that as settled would drop a
    // live question out of the queue silently, which is worse than offering one
    // twice.
    let v = view(
        &[(
            "question::hollow",
            &[("decider", "op"), ("answered", "   ")],
        )],
        &[],
    );
    let (_, out) = run(&v, &[]);
    assert!(
        out.contains("hollow"),
        "a blank `answered` is not an answer: {out}"
    );
}
