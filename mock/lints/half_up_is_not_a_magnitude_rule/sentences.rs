//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The corpus: sentences the reader has to get right, each beside the near-twin
//! on the other side of the line.
//!
//! A sentence that fires and a sentence that does not are cheap to collect and
//! prove nothing on their own, because a reader can be right about both for the
//! wrong reason. What a pair proves is that the reader is reading the thing the
//! pair differs in: one word moved, one clause boundary added, one negator bound
//! to something else, and the answer flips. A reader passing a pair by accident
//! has to be wrong twice in opposite directions.
//!
//! Two lists. `corpus.rs` holds the pairs the reader gets right, and every one
//! of them is asserted. `catalogue.rs` holds the pairs it gets wrong on one
//! side, each with the side named, and the arm asserting them is ignored as
//! catalogue rather than failing the suite: those are the directions the gate
//! is known to be incomplete in, and the lint's module doc names them.

#[path = "sentences/catalogue.rs"]
mod catalogue;
#[path = "sentences/corpus.rs"]
mod corpus;

use corpus::PAIRS;

use super::reading::hits;

/// One sentence and its twin, differing in one thing, with what that thing is.
pub(super) struct Pair {
    /// The sentence the lint has to refuse.
    pub(super) fires:    &'static str,
    /// The sentence it has to let through.
    pub(super) silent:   &'static str,
    /// The item a doc block would sit on, where the pair is about that reading.
    pub(super) subject:  Option<&'static str>,
    /// What the two differ in, which is what the pair is testing.
    pub(super) differs:  &'static str,
    /// Whether a review measured this one against the reader that stood before
    /// the rewrite.
    pub(super) measured: bool,
}

/// The readings a sentence is refused for, as the reader answers it.
fn found(text: &str, subject: Option<&str>) -> Vec<&'static str> {
    hits(text, subject).into_iter().map(|h| h.reading).collect()
}

/// Whether the reader gets the side of `pair` that has to fire right.
fn fires_right(pair: &Pair) -> bool {
    !found(pair.fires, pair.subject).is_empty()
}

/// Whether the reader gets the side of `pair` that has to stay silent right.
fn silent_right(pair: &Pair) -> bool {
    found(pair.silent, pair.subject).is_empty()
}

/// Every side of `pairs` the reader gets wrong, described.
fn wrong_sides<'a>(pairs: impl IntoIterator<Item = &'a Pair>) -> Vec<String> {
    let mut wrong = Vec::new();
    for pair in pairs {
        if !fires_right(pair) {
            wrong.push(format!(
                "silent, should fire [{}]: {}",
                pair.differs, pair.fires
            ));
        }
        if !silent_right(pair) {
            wrong.push(format!(
                "fires, should be silent [{}]: {}",
                pair.differs, pair.silent
            ));
        }
    }
    wrong
}

#[test]
fn every_pair_fires_on_one_side_and_is_silent_on_the_other() {
    // Every side is read before anything is asserted, so a run reports the whole
    // state of the reader rather than the first sentence it gets wrong.
    let wrong = wrong_sides(PAIRS);
    assert!(
        wrong.is_empty(),
        "{} of {} sides wrong:\n{}",
        wrong.len(),
        2 * PAIRS.len(),
        wrong.join("\n")
    );
}

#[test]
fn the_corpus_holds_what_it_held_and_the_review_measured_eight_of_it() {
    // A count rather than a list, so a pair removed is a failure here rather
    // than a silent narrowing of what the reader is asked.
    assert_eq!(PAIRS.len(), 53);
    assert_eq!(PAIRS.iter().filter(|p| p.measured).count(), 8);
}

#[test]
fn control_no_pair_is_the_same_sentence_twice() {
    // A pair whose two sides are equal, or whose sides differ only outside what
    // the reader looks at, would pass the arm above by construction.
    for pair in PAIRS {
        assert_ne!(pair.fires, pair.silent, "{}", pair.differs);
    }
}

#[test]
fn no_sentence_the_corpus_asserts_is_one_the_catalogue_says_is_wrong() {
    // The corpus asserting a side right and the catalogue naming the same
    // sentence as a wrong side would be a contradiction each arm passes alone.
    // A catalogue twin, which is a side the reader gets right, may appear in
    // both.
    for known in catalogue::CATALOGUE {
        let wrong = known.wrong_side();
        for pair in PAIRS {
            assert_ne!(pair.fires, wrong, "{}", pair.differs);
            assert_ne!(pair.silent, wrong, "{}", pair.differs);
        }
    }
}
