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
//! Eight of the entries are the sentences a review measured against the reader
//! that stood before this one: six it was silent on and two it fired on. They
//! are marked, so the corpus says where it came from.

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
    /// Whether the review measured this one against the earlier reader.
    pub(super) measured: bool,
}

/// The corpus. A pair is added here rather than as a test of its own, so the
/// count below is what says whether one was dropped.
pub(super) const PAIRS: &[Pair] = &[
    Pair {
        fires:    "Unlike `half_even`, `half_up` sends a tie away from zero, as MATLAB's Round does.",
        silent:   "Unlike `half_up`, MATLAB's Round sends a tie away from zero.",
        subject:  None,
        differs:  "which of the two names the contrast is against",
        measured: true,
    },
    Pair {
        fires:    "With no dither to read, `half_up` sends a tie away from zero.",
        silent:   "With no dither to read, `half_up` sends no tie away from zero.",
        subject:  None,
        differs:  "whether the negator sits in the reading's own segment",
        measured: true,
    },
    Pair {
        fires:    "Except for the unsigned formats, `half_up` takes a tie away from zero.",
        silent:   "`half_up` takes a tie toward positive infinity, except that the alias takes it \
                  away from zero.",
        subject:  None,
        differs:  "whether the clause with the reading in it names the mode",
        measured: true,
    },
    Pair {
        fires:    "`half_up` rounds to nearest and, for negative inputs, rounds the tie to the \
                  larger magnitude.",
        silent:   "`half_up` rounds to nearest and, for negative inputs, never rounds the tie to \
                  the larger magnitude.",
        subject:  None,
        differs:  "a negator in the reading's segment, with a parenthetical between the two either \
                  way",
        measured: true,
    },
    Pair {
        fires:    "For `half_up` the tie at -2.5 goes to -3, away from zero, like Java.",
        silent:   "For `half_up` the tie at -2.5 goes to -2, not away from zero, unlike Java.",
        subject:  None,
        differs:  "a negator in front of the reading, in a sentence cut into three by commas",
        measured: true,
    },
    Pair {
        fires:    "`half_up` is nearest. Its tie goes away from zero.",
        silent:   "`half_up` is nearest. The alias's tie goes away from zero.",
        subject:  None,
        differs:  "whether the second sentence opens with a pronoun for the mode",
        measured: true,
    },
    Pair {
        fires:    "The error of `half_up` is at most half a quantum, and its tie goes up in \
                  magnitude.",
        silent:   "The error of `half_up` is at most half a quantum in magnitude.",
        subject:  None,
        differs:  "whether a direction word stands in front of the magnitude",
        measured: true,
    },
    Pair {
        fires:    "`half_up` bounds the error by `q/2` and sends a tie away from zero at every sign.",
        silent:   "`half_up` bounds the error by `q/2` in magnitude at every sign.",
        subject:  None,
        differs:  "whether the magnitude is the error's or the tie's",
        measured: true,
    },
    Pair {
        fires:    "`half_up` is ties away from zero.",
        silent:   "`half_up` is not ties away from zero.",
        subject:  None,
        differs:  "the negator, with nothing else between the name and the reading",
        measured: false,
    },
    Pair {
        fires:    "`HalfUp` reads the sign at a tie.",
        silent:   "`HalfUp` never reads the sign at a tie.",
        subject:  None,
        differs:  "the negator, on the sign vocabulary rather than the magnitude one",
        measured: false,
    },
    Pair {
        fires:    "`HalfUp` commutes with reflection.",
        silent:   "`HalfUp` never commutes with reflection.",
        subject:  None,
        differs:  "the negator, on the symmetry vocabulary",
        measured: false,
    },
    Pair {
        fires:    "Nearest, and a tie goes away from zero.",
        silent:   "Nearest, and a tie goes toward positive infinity.",
        subject:  Some("HalfUp"),
        differs:  "the reading itself, in a doc block whose item names the mode",
        measured: false,
    },
    Pair {
        fires:    "| `half_up` | ties away from zero |",
        silent:   "| `half_up` | not ties away from zero |",
        subject:  None,
        differs:  "the negator, inside a table cell",
        measured: false,
    },
    Pair {
        fires:    "In `{floor, half_up away from zero, ceil}` the second is wanted.",
        silent:   "In `{floor, away from zero, half_up, ceil}` the second is wanted.",
        subject:  None,
        differs:  "whether the name and the reading are one item of the list or two",
        measured: false,
    },
    Pair {
        fires:    "The rule `half_up` takes a tie away from zero.",
        silent:   "The planted rule `half_up` takes a tie away from zero.",
        subject:  None,
        differs:  "the marker saying the clause is about another rule",
        measured: false,
    },
    Pair {
        fires:    "`half_up` (the nearest mode) sends a tie away from zero.",
        silent:   "`half_up` (the nearest mode) never sends a tie away from zero.",
        subject:  None,
        differs:  "a negator in the reading's segment, with a parenthetical between the name and \
                  the reading",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is not the even rule but sends a tie away from zero.",
        silent:   "`half_up` is not the even rule and never sends a tie away from zero.",
        subject:  None,
        differs:  "whether a conjunction stands between the negator and the reading",
        measured: false,
    },
    Pair {
        fires:    "It is `half_up` which sends a tie away from zero.",
        silent:   "It is not `half_up` which sends a tie away from zero.",
        subject:  None,
        differs:  "a negator ahead of a segment the reading's clause hangs off",
        measured: false,
    },
    Pair {
        fires:    "Java's rule and `half_up` both send a tie away from zero.",
        silent:   "Java's rule, and not `half_up`, sends a tie away from zero.",
        subject:  None,
        differs:  "whether the negator sits in the name's own segment",
        measured: false,
    },
    Pair {
        fires:    "So a tie at -2.5 goes to -3, which is away from zero.",
        silent:   "So a tie at -2.5 goes to -2. That one is ties away from zero, and it is reached \
                  by shifting the position half a step away from zero and rounding toward zero.",
        subject:  Some("HalfUp"),
        differs:  "whether the clause with the reading is about the item or about the rule it was \
                  just contrasted with",
        measured: false,
    },
    Pair {
        fires:    "`half_up` takes a tie away from zero at a negative slot.",
        silent:   "`half_up` takes a tie toward positive infinity at a negative slot, where the \
                  alias goes away from zero.",
        subject:  None,
        differs:  "which clause the mode's name sits in",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is a nearest rule that reads nothing and so falls on the translation \
                  side, where the ties-away rule would fall on the reflection side.",
        silent:   "`half_up` is a nearest rule that reads nothing and so falls on the translation \
                  side, where the ties-away alias would fall on the reflection side.",
        subject:  None,
        differs:  "whether the negator stands behind the reading in the reading's own segment",
        measured: false,
    },
    Pair {
        fires:    "The rounding region commutes with reflection for toward zero and half-even, \
                  and for floor, ceil, half-up or a stochastic decision at a fixed dither.",
        silent:   "The rounding region commutes with reflection for toward zero and half-even, \
                  and not for floor, ceil, half-up or a stochastic decision at a fixed dither.",
        subject:  None,
        differs:  "whether a negator opens the list the name sits in, behind the reading",
        measured: false,
    },
    Pair {
        fires:    "`half_up`, unlike the even rule, sends a tie away from zero.",
        silent:   "`half_up`, unlike the even rule, never sends a tie away from zero.",
        subject:  None,
        differs:  "whether the negator is in the reading's own segment or in an aside between the \
                  name and the reading",
        measured: false,
    },
    Pair {
        fires:    "In `{floor, half_up, a tie away from zero, ceil}` the second is wanted.",
        silent:   "In `{floor, half_up, away from zero, ceil}` the second is wanted.",
        subject:  None,
        differs:  "whether the item holding the reading holds anything besides it",
        measured: false,
    },
    Pair {
        fires:    "`half_up`: a tie goes away from zero.",
        silent:   "`half_even`: a tie goes away from zero.",
        subject:  None,
        differs:  "which mode the clause before the colon names",
        measured: false,
    },
    Pair {
        fires:    "`half_up`; a tie goes away from zero.",
        silent:   "The alias; a tie goes away from zero.",
        subject:  None,
        differs:  "whether the clause before the semicolon names the mode",
        measured: false,
    },
    Pair {
        fires:    "`half_up`? a tie goes away from zero.",
        silent:   "`half_up`? a tie never goes away from zero.",
        subject:  None,
        differs:  "a negator in the reading's segment, across a question mark",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is a mode. Ties go away from zero under it.",
        silent:   "`half_up` is a mode. Ties go away from zero under `Round`.",
        subject:  None,
        differs:  "whether the clause carrying the reading names another rule, where neither opens \
                  with a pronoun",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is a mode. It is nearest. Its tie goes away from zero.",
        silent:   "`half_up` is a mode. `HalfEven` is another. Its tie goes away from zero.",
        subject:  None,
        differs:  "whether the sentence between the name and the pronoun names another rule",
        measured: false,
    },
    Pair {
        fires:    "`half_up` pairs with `q/2`; it sends a tie away from zero.",
        silent:   "`half_up` pairs with `HalfEven`; it sends a tie away from zero.",
        subject:  None,
        differs:  "whether the code span standing after the name is another rule",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is not Java's rule. It sends a tie away from zero.",
        silent:   "`half_up` is not Java's HALF_UP. It sends a tie away from zero.",
        subject:  None,
        differs:  "whether the other operation is named, with no code span on either side",
        measured: false,
    },
    Pair {
        fires:    "Nearest. A tie goes away from zero.",
        silent:   "Nearest. A tie goes away from zero under `HalfEven`.",
        subject:  Some("HalfUp"),
        differs:  "whether the clause names another rule, where the doc block's item would otherwise \
                  reach it",
        measured: false,
    },
    Pair {
        fires:    "`half_up` goes up. And takes ties away from zero.",
        silent:   "`half_up` goes up. And `HalfEven` takes ties away from zero.",
        subject:  None,
        differs:  "whether the clause after the boundary names another rule",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is a mode. It is read up the number line. A tie goes away from zero.",
        silent:   "`half_up` is a mode. Read it as up the number line. A tie goes away from zero.",
        subject:  None,
        differs:  "whether the clause between the name and the reading points back, which is what \
                  carries the name past one clause",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is a mode. The crate sends a tie away from zero.",
        silent:   "`half_up` is a mode. `arvo-format` sends a tie away from zero.",
        subject:  None,
        differs:  "whether the clause opens by naming a subject of its own in a code span",
        measured: false,
    },
    Pair {
        fires:    "`half_up` sends a tie to the nearest slot, where ties away from zero would give \
                  -3.",
        silent:   "`half_up` sends a tie toward positive infinity, where ties away from zero would \
                  give -3.",
        subject:  None,
        differs:  "whether the clause states the settled denotation ahead of the reading",
        measured: false,
    },
    Pair {
        fires:    "`half_up` adds half a step and floors. Either way a tie goes away from \
                  zero.",
        silent:   "`half_up` adds half a step and floors. Toward zero sends a tie away from \
                  zero.",
        subject:  None,
        differs:  "whether the clause after it opens by naming another rule, capitalised",
        measured: false,
    },
];

/// The readings a sentence is refused for, as the reader answers it.
fn found(text: &str, subject: Option<&str>) -> Vec<&'static str> {
    hits(text, subject).into_iter().map(|h| h.reading).collect()
}

#[test]
fn every_pair_fires_on_one_side_and_is_silent_on_the_other() {
    // Every side is read before anything is asserted, so a run reports the whole
    // state of the reader rather than the first sentence it gets wrong.
    let mut wrong = Vec::new();
    for pair in PAIRS {
        if found(pair.fires, pair.subject).is_empty() {
            wrong.push(format!(
                "silent, should fire [{}]: {}",
                pair.differs, pair.fires
            ));
        }
        if !found(pair.silent, pair.subject).is_empty() {
            wrong.push(format!(
                "fires, should be silent [{}]: {}",
                pair.differs, pair.silent
            ));
        }
    }
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
    assert_eq!(PAIRS.len(), 38);
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
