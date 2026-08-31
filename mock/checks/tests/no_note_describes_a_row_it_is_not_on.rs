//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A note goes stale silently, because nothing re-reads it when the row moves.
//!
//! The row changes and the note does not, so it keeps reading as a caveat a
//! reader should honour while describing a row from some rounds ago. Nothing in
//! the schema can catch it: both halves are valid, and only their disagreement
//! is wrong.
//!
//! Fifteen rows carried "`evidence` is empty and the measured-implies-evidence
//! check is red on this row", written truthfully at a time when `probe.toml` did
//! not exist. It exists now, the edges were wired, and thirteen of the fifteen
//! notes then said the opposite of their own row.
//!
//! **The repair went further than the thirteen**, because the clause was a
//! snapshot of a check's colour, and a check reports its own colour. A note
//! asserting one is a second copy of a fact with nothing keeping the two in
//! step, so the clause came out of all fifteen and what stayed is the durable
//! half: which instruments exist, and what they measured.

use arvo_checks::{canon, parse, shape};

/// The committed canon carries none.
#[test]
fn no_note_claims_an_empty_field_that_is_not() {
    let found = shape::notes_claiming_an_empty_field_that_is_not(&canon());
    assert!(
        found.is_empty(),
        "{} note(s) describe a row other than the one they sit on: {found:#?}",
        found.len()
    );
}

/// The control, both ways round in one input.
///
/// The false claim must fire and the true one must not, or the arm is a ban on
/// notes mentioning a field at all.
#[test]
fn a_false_claim_fires_and_a_true_one_does_not() {
    let reg = parse(
        "planted",
        r#"
[[proposal]]
id = "the_note_is_stale"
says = "Something."
evidence = ["a_probe"]
note = "`evidence` is empty, so nobody can check this."

[[proposal]]
id = "the_note_is_true"
says = "Something else."
note = "`evidence` is empty, so nobody can check this."
"#,
    );
    let found = shape::notes_claiming_an_empty_field_that_is_not(&reg);
    assert_eq!(found.len(), 1, "exactly the stale one: {found:#?}");
    assert!(found[0].at.contains("the_note_is_stale"), "{found:#?}");
    assert!(
        found[0].says.contains("holds 1 entry"),
        "the count is reported, so a reader can see how far the note has drifted: {found:#?}"
    );
}

/// The word `empty` is ordinary and a note is allowed to use it.
///
/// Without this the arm would fire on every note that mentions a field and
/// somewhere says the word, which in this corpus is most of them: an empty
/// region, an empty intersection, an empty result being the finding.
#[test]
fn the_word_empty_about_something_else_is_left_alone() {
    let reg = parse(
        "planted",
        r#"
[[proposal]]
id = "a_row_about_empty_regions"
says = "Something."
evidence = ["a_probe"]
note = "The `evidence` here rests on an instrument whose covered set is empty at width two, and the intersection of the two predicates is empty."
"#,
    );
    assert!(
        shape::notes_claiming_an_empty_field_that_is_not(&reg).is_empty(),
        "a note may say `empty` about the thing a field describes"
    );
}

/// A field named far from the phrase is not this claim.
///
/// The window is deliberately short. A note mentioning `evidence` in one clause
/// and saying something is empty three sentences later is talking about two
/// things, and joining them would make the arm fire on prose that is correct.
#[test]
fn a_field_named_far_from_the_phrase_is_not_a_claim_about_it() {
    let reg = parse(
        "planted",
        r#"
[[proposal]]
id = "two_separate_clauses"
says = "Something."
evidence = ["a_probe"]
note = "The `evidence` is one instrument and its author bounded it exactly, measured at the widths in its table and no further. The region it establishes is empty above that."
"#,
    );
    assert!(
        shape::notes_claiming_an_empty_field_that_is_not(&reg).is_empty(),
        "two clauses about two things are not one claim about one"
    );
}

/// It reads the row's own field names, so it needs no editing when one is added.
///
/// `retirement` gained an `obligation` field after this arm was written, and a
/// word list would have had to gain it too, silently missing it until somebody
/// noticed.
#[test]
fn it_works_on_a_field_the_arm_never_heard_of() {
    let reg = parse(
        "planted",
        r#"
[[retirement]]
id = "a_route"
claim = "A way."
why = "No."
kind = "wrong"
obligation = ["a_thing", "another_thing"]
provenance = ["panel::x"]
note = "`obligation` is empty on this row."
"#,
    );
    let found = shape::notes_claiming_an_empty_field_that_is_not(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(found[0].says.contains("holds 2 entries"), "{found:#?}");
}
