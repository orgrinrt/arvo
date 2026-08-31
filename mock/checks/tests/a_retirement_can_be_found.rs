//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A retirement too short to find pins nothing and reports the wrong thing.
//!
//! The schema asks a `claim` to be the sentence a reader would quote, close
//! enough that a search finds it. A claim of a few words is neither: too short
//! to match the sentence it retires, long enough to match ordinary prose on the
//! same subject.
//!
//! **Both halves were observed on one row.** `No repair.` is how its source
//! abbreviates a claim, and the restatement check then reported a live row
//! saying "no repair at a homogeneous container" as a restatement of it. That
//! phrase is the **restated** form, which the same retirement endorses. So the
//! retirement pinned nothing it meant to pin and reported the one thing it
//! meant to protect.
//!
//! The full sentence was still recoverable from the provenance the row already
//! carried, which is what makes the repair cheap and the rule worth having.

use arvo_checks::{canon, parse, shape};

/// No retirement in the canon is too short to find.
#[test]
fn every_retirement_carries_a_findable_claim() {
    let found = shape::retirements_too_short_to_find(&canon());
    assert!(
        found.is_empty(),
        "{} retirement(s) hold a claim nothing can be pinned to: {found:#?}",
        found.len()
    );
}

/// The control: short fires, long does not.
#[test]
fn a_short_claim_is_reported_and_a_sentence_is_not() {
    let reg = parse(
        "planted",
        r#"
[[retirement]]
id = "an_abbreviation"
claim = "No repair."
why = "Because."
kind = "wrong"
provenance = ["panel::x"]

[[retirement]]
id = "a_sentence"
claim = "Two names for one primitive is a compile error with no in-language repair."
why = "Because."
kind = "wrong"
provenance = ["panel::x"]
"#,
    );
    let found = shape::retirements_too_short_to_find(&reg);
    assert_eq!(found.len(), 1, "only the abbreviation: {found:#?}");
    assert!(found[0].at.contains("an_abbreviation"), "{found:#?}");
    assert!(found[0].says.contains("2 word"), "the count is reported: {found:#?}");
}

/// A short claim reports no live row, which is the other half of the defect.
///
/// This is the case that had to fail and did: before the floor, the whole
/// two-word claim was the match, so a live row using the ordinary phrase was
/// reported as restating a retired one.
#[test]
fn a_short_claim_cannot_report_a_live_row() {
    let reg = parse(
        "planted",
        r#"
[[retirement]]
id = "an_abbreviation"
claim = "No repair."
why = "Because."
kind = "wrong"
provenance = ["panel::x"]

[[proposal]]
id = "a_row_using_the_ordinary_phrase"
says = "A missed merge costs nothing at a monomorphic site, one threaded parameter at a polymorphic signature, and no repair at a homogeneous container."
"#,
    );
    assert!(
        shape::rows_restating_a_retired_claim(&reg).is_empty(),
        "a two-word claim matches the subject rather than the claim, so it reports nothing"
    );
    assert_eq!(
        shape::retirements_too_short_to_find(&reg).len(),
        1,
        "and the finding lands on the retirement, which is where it belongs"
    );
}

/// A real restatement is still caught, so the floor did not disarm the arm.
#[test]
fn a_genuine_restatement_of_a_full_sentence_is_still_reported() {
    let reg = parse(
        "planted",
        r#"
[[retirement]]
id = "a_sentence"
claim = "A component is an output of the derivation when the consumer did not write it, the machine needs it, and a downstream site that holds the other components cannot recover it."
why = "It is not a definition."
kind = "wrong"
provenance = ["panel::x"]

[[proposal]]
id = "a_row_restating_it"
says = "A component is an output of the derivation when the consumer did not write it, the machine needs it, and a downstream site holding the other components cannot recover it."
"#,
    );
    let found = shape::rows_restating_a_retired_claim(&reg);
    assert_eq!(found.len(), 1, "the run still matches through a reworded tail: {found:#?}");
    assert!(found[0].at.contains("a_row_restating_it"), "{found:#?}");
}

/// A claim exactly at the floor is findable and reports nothing.
#[test]
fn a_claim_at_the_floor_is_left_alone() {
    let reg = parse(
        "planted",
        r#"
[[retirement]]
id = "exactly_five_words"
claim = "The strategy set is closed."
why = "Op reopened it."
kind = "wrong"
provenance = ["panel::x"]
"#,
    );
    assert!(
        shape::retirements_too_short_to_find(&reg).is_empty(),
        "five words is the floor and the floor passes"
    );
}
