//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Nothing reads `decider`, nothing reads `bound`, and nothing reads a ruling's
//! `answers` backward.
//!
//! Three fields carry the whole of who owes an answer, and at the commit this
//! was written against no check, no lint and no tool named any of them.
//! Measured by `209_probes/which_registry_fields_the_tooling_reads.sh`, which
//! searches the quoted literal a check would use, over `mock/checks`,
//! `mock/lints` and `mock/tools`, at that commit and at HEAD: `decider` 0,
//! `bound` 0, `unblocks` 0, `answers` 0. The control in the same run is the
//! fields already read there, `rung` 25, `keywords` 1 and `provenance` 1, so
//! the instrument reaches the trees it was pointed at.
//!
//! An earlier wording here put `answers` at six and called them incidental
//! prose. That figure came from grepping the bare word, which also matches
//! ordinary sentences; `bound` scores nine at the same commit from "bounded"
//! and "boundary" alone, which is a measurement of English rather than of the
//! tooling. The probe searches the literal for that reason.
//!
//! What that leaves unguarded is a queue that cannot tell a settled question
//! from an open one. The `question` namespace is defined as "Something the canon
//! has not settled", the rendered document is a table of nine columns and the
//! incoming `answers` edge is not one of them, so a question op has already
//! answered renders as open, reads as open, and counts as open in every roster
//! built by reading `decider`.
//!
//! The three arms below are the three statements the schema already makes and
//! nothing enforces. Each runs against a planted input as well as the committed
//! canon, because an arm that has only ever seen a clean canon has returned an
//! empty list and established nothing.

use std::collections::{BTreeMap, BTreeSet};

use arvo_checks::{canon, parse, Finding, Registry};

/// The phrase this namespace already uses to say a question is settled.
///
/// Not invented here, and not a bare word either. Five rows arrived at the same
/// construction independently, before any check existed to ask for it:
/// "Recorded as answered at `28` batch one", "Recorded as answered by op on
/// 2026-08-14", "Recorded as closed by op on 2026-08-14", "Recorded as answered
/// on 2026-08-14". The arm codifies what those five established.
///
/// **The bare words do not work and the first cut of this arm used them.** They
/// appear on 54 lines of the question registry in ordinary prose, including "a
/// further item folds in here rather than being answered on its own", which is a
/// row saying the opposite of settled. Worse, `id` is a field like any other, so
/// a planted row named `answered_twice` matched itself and the control that
/// should have caught it passed. Both were caught by the planted inputs below
/// and neither would have been caught by the committed canon alone.
const SETTLED_PHRASES: &[&str] = &["recorded as answered", "recorded as closed"];

/// Where the phrase may sit. `note` is where all five put it and `bound` is
/// where a returned-then-settled question would. Deliberately not every field:
/// scanning `id`, `asks` and `options` is what made the first cut useless.
const PROSE_FIELDS: &[&str] = &["note", "bound"];

/// Which questions each ruling names as settled, read backward.
///
/// The forward edge is `ruling.answers`, a `question[]`. The registry has no
/// reverse index and neither does the rendered document, so a reader standing on
/// a question row cannot see that a ruling settles it without grepping the other
/// file. This is that grep, done once.
fn answered_by(reg: &Registry) -> BTreeMap<String, Vec<String>> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for ruling in reg.of("ruling") {
        for question in ruling.list("answers") {
            out.entry(question.clone())
                .or_default()
                .push(ruling.id.clone());
        }
    }
    out
}

/// A ruling naming a question that is not declared anywhere.
///
/// Referential integrity on the one edge that closes a question. A dangling
/// entry here reads as a settled question and points at nothing, which is worse
/// than an open one, because the row it would have closed stays open silently.
fn answers_naming_no_question(reg: &Registry) -> Vec<Finding> {
    let declared: BTreeSet<&str> = reg.slugs("question").into_iter().collect();
    let mut out = Vec::new();
    for ruling in reg.of("ruling") {
        for question in ruling.list("answers") {
            if !declared.contains(question.as_str()) {
                out.push(Finding::new(
                    "answers-names-no-question",
                    ruling.addr(),
                    format!("`answers` names `question::{question}`, which is declared nowhere"),
                ));
            }
        }
    }
    out
}

/// A question filed as op's while carrying the record that he handed it back.
///
/// The schema states this one itself, in the `bound` field's own description:
/// "A `decider = op` row carrying no `bound` has simply not been asked yet; one
/// carrying a `bound` and still naming him is a filing error." It has never been
/// checked. The canon is clean of it today, which is exactly the condition under
/// which an unenforced rule stops being true without anybody noticing.
fn op_deciders_carrying_a_bound(reg: &Registry) -> Vec<Finding> {
    reg.of("question")
        .filter(|q| q.get("decider") == Some("op") && q.has("bound"))
        .map(|q| {
            Finding::new(
                "op-decider-carrying-a-bound",
                q.addr(),
                "carries a `bound`, which records that it was put to him and returned, and still \
                 names him as the decider. One of the two is wrong.",
            )
        })
        .collect()
}

/// A question a ruling settles, whose own row does not say so.
///
/// The consequence is not cosmetic. Every roster of what op owes is built by
/// reading `decider`, the rendered table does not carry the incoming edge, and
/// so a question he answered goes back into the queue. That has already cost him
/// a round trip once, which is the reason the `bound` field exists at all.
fn settled_questions_that_do_not_say_so(reg: &Registry) -> Vec<Finding> {
    let settled = answered_by(reg);
    let mut out = Vec::new();
    for question in reg.of("question") {
        let Some(rulings) = settled.get(&question.id) else {
            continue;
        };
        let says_so = PROSE_FIELDS
            .iter()
            .filter_map(|f| question.get(f))
            .any(|text| {
                let lower = text.to_lowercase();
                SETTLED_PHRASES.iter().any(|p| lower.contains(p))
            });
        if !says_so {
            out.push(Finding::new(
                "settled-question-reads-as-open",
                question.addr(),
                format!(
                    "`ruling::{}` names it as answered and nothing in the row says so, so it \
                     renders as an open question and counts as one",
                    rulings.join("`, `ruling::"),
                ),
            ));
        }
    }
    out
}

// --- the committed canon ---------------------------------------------------

#[test]
fn the_committed_canon_answers_no_question_it_does_not_declare() {
    let found = answers_naming_no_question(&canon());
    assert!(
        found.is_empty(),
        "a ruling closes a question that is declared nowhere: {found:#?}"
    );
}

#[test]
fn the_committed_canon_files_no_returned_question_as_his() {
    let found = op_deciders_carrying_a_bound(&canon());
    assert!(
        found.is_empty(),
        "the schema's own rule, in the `bound` field's description: {found:#?}"
    );
}

#[test]
fn every_settled_question_in_the_committed_canon_says_it_is_settled() {
    let found = settled_questions_that_do_not_say_so(&canon());
    assert!(
        found.is_empty(),
        "these render as open and go back into the queue he already answered: {found:#?}"
    );
}

/// The three arms above are assertions about a clean canon and would all pass
/// over an empty one. This is what says the population is not empty.
///
/// Deliberately a floor rather than the number. The count moves every time a
/// ruling closes a question, which is the point of the panel, and pinning it
/// would make the test a burden that gets relaxed rather than a control that
/// holds. Six questions carry an answering ruling as this is written.
#[test]
fn the_committed_canon_has_questions_a_ruling_has_settled() {
    let reg = canon();
    let settled = answered_by(&reg);
    assert!(
        settled.len() >= 4,
        "the arms above pass vacuously if no ruling answers anything: {settled:#?}"
    );
    let declared: BTreeSet<&str> = reg.slugs("question").into_iter().collect();
    for question in settled.keys() {
        assert!(
            declared.contains(question.as_str()),
            "`{question}` is named as settled and is not a question"
        );
    }
}

// --- planted inputs, one per direction --------------------------------------

#[test]
fn a_ruling_answering_nothing_that_exists_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[question]]
id = "a_real_one"
asks = "Is it?"
decider = "op"

[[ruling]]
id = "he_said_so"
answers = ["a_real_one", "a_ghost"]
"#,
    );
    let found = answers_naming_no_question(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "answers-names-no-question");
    assert_eq!(found[0].at, "ruling::he_said_so");
    assert!(
        found[0].says.contains("a_ghost"),
        "the report names which one: {}",
        found[0].says
    );
}

#[test]
fn a_ruling_answering_only_real_questions_is_not_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[question]]
id = "a_real_one"
asks = "Is it?"
decider = "op"

[[ruling]]
id = "he_said_so"
answers = ["a_real_one"]
"#,
    );
    assert!(answers_naming_no_question(&reg).is_empty());
}

#[test]
fn a_question_filed_as_his_while_carrying_a_bound_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[question]]
id = "handed_back"
asks = "Which set ships?"
decider = "op"
bound = "Put to him and returned. Bounded by soundness."
"#,
    );
    let found = op_deciders_carrying_a_bound(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "op-decider-carrying-a-bound");
    assert_eq!(found[0].at, "question::handed_back");
}

/// The whole point of a bound is that it sits on a row naming somebody else.
/// An arm firing here would forbid the repair the field exists for.
#[test]
fn a_bound_on_a_row_naming_the_panel_is_left_alone() {
    let reg = parse(
        "planted.toml",
        r#"
[[question]]
id = "handed_back"
asks = "Which set ships?"
decider = "panel"
bound = "Put to him and returned. Bounded by soundness."

[[question]]
id = "not_yet_asked"
asks = "Which word?"
decider = "op"
"#,
    );
    assert!(
        op_deciders_carrying_a_bound(&reg).is_empty(),
        "a returned question names the panel, and an unasked one carries no bound"
    );
}

#[test]
fn a_settled_question_whose_row_is_silent_about_it_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[question]]
id = "already_settled"
asks = "May the canon carry it?"
decider = "op"
note = "The instance is carried separately. Two topics defer to it."

[[ruling]]
id = "the_test_he_gave"
answers = ["already_settled"]
"#,
    );
    let found = settled_questions_that_do_not_say_so(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "settled-question-reads-as-open");
    assert_eq!(found[0].at, "question::already_settled");
    assert!(
        found[0].says.contains("the_test_he_gave"),
        "the report names the ruling to read: {}",
        found[0].says
    );
}

#[test]
fn a_settled_question_that_says_so_is_left_alone() {
    let reg = parse(
        "planted.toml",
        r#"
[[question]]
id = "already_settled"
asks = "May the canon carry it?"
decider = "op"
note = "Recorded as answered on 2026-08-14; the answer is not written here."

[[ruling]]
id = "the_test_he_gave"
answers = ["already_settled"]
"#,
    );
    assert!(settled_questions_that_do_not_say_so(&reg).is_empty());
}

/// An open question saying nothing about being settled is the ordinary case and
/// the arm must not reach it, or it becomes a demand that every question in the
/// namespace announce a closure it has not had.
#[test]
fn an_open_question_is_not_asked_to_announce_a_closure() {
    let reg = parse(
        "planted.toml",
        r#"
[[question]]
id = "still_open"
asks = "Which word?"
decider = "op"
note = "Nobody has settled this."
"#,
    );
    assert!(settled_questions_that_do_not_say_so(&reg).is_empty());
}

/// The phrase may arrive in either prose field, because the five rows that
/// established the convention put it in `note` and a returned-then-settled one
/// would put it in `bound`.
#[test]
fn the_phrase_is_found_wherever_the_row_puts_it() {
    let reg = parse(
        "planted.toml",
        r#"
[[question]]
id = "settled_in_the_bound"
asks = "Which set?"
decider = "panel"
bound = "Put to him and recorded as closed, with the ruling beside it."

[[ruling]]
id = "the_call"
answers = ["settled_in_the_bound"]
"#,
    );
    assert!(settled_questions_that_do_not_say_so(&reg).is_empty());
}

/// The first cut of the arm matched the bare word `answered` in any field, and
/// the row's own `id` is a field. A question slug containing the word passed its
/// own check, which is a test asserting a value against itself with two hops in
/// between.
///
/// Kept as a regression control rather than renamed away, because the slug that
/// exposed it is the shape a real one would take: a question about something
/// having been answered is exactly what this namespace is full of.
#[test]
fn a_slug_containing_the_word_does_not_answer_its_own_question() {
    let reg = parse(
        "planted.toml",
        r#"
[[question]]
id = "answered_twice"
asks = "Which reading?"
decider = "op"
note = "Silent about whether anybody settled it."

[[ruling]]
id = "the_first_word"
answers = ["answered_twice"]

[[ruling]]
id = "the_sharpening"
answers = ["answered_twice"]
"#,
    );
    let found = settled_questions_that_do_not_say_so(&reg);
    assert_eq!(
        found.len(),
        1,
        "the slug is not a settlement record: {found:#?}"
    );
    assert!(
        found[0].says.contains("the_first_word") && found[0].says.contains("the_sharpening"),
        "two rulings may settle one question and both are named: {}",
        found[0].says
    );
}

/// The second half of the same defect. The committed canon carries the sentence
/// "a further item folds in here rather than being answered on its own", which
/// is a row saying it is **not** settled, and the bare-word arm read it as a
/// settlement record and fell silent on a real finding.
#[test]
fn prose_using_the_word_in_passing_is_not_a_settlement_record() {
    let reg = parse(
        "planted.toml",
        r#"
[[question]]
id = "still_open"
asks = "May the canon carry it?"
decider = "op"
note = "A further item folds in here rather than being answered on its own, and the instance is carried separately."

[[ruling]]
id = "the_test_he_gave"
answers = ["still_open"]
"#,
    );
    let found = settled_questions_that_do_not_say_so(&reg);
    assert_eq!(
        found.len(),
        1,
        "the word appears and the row is not claiming to be settled: {found:#?}"
    );
    assert_eq!(found[0].at, "question::still_open");
}
