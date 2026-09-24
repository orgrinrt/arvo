//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The two escapes that read across the segments of one clause: a list, whose
//! items the segments are, and a contrast against the settled denotation, whose
//! conjunction rule is bounded by the reading's own segment.
//!
//! Both went wrong on the same thing, a stretch of the clause taken whole where
//! only one segment of it bears on the question, so the walks here put every
//! delimiter pair and every conjunction in the segments that should not count,
//! beside a control where the same words stand in the one that does.

use super::reading::{CONJUNCTIONS, DENOTES, QUALIFIED, READINGS, hits};

/// The readings found in `text`, as written.
fn found(text: &str) -> Vec<&'static str> {
    hits(text, None).into_iter().map(|h| h.reading).collect()
}

#[test]
fn a_list_with_the_two_in_different_items_is_silent() {
    assert!(found("rounding: in {floor, ceil, toward_zero, away from zero, half_up}").is_empty());
    assert!(found("The modes floor, away from zero, and half_up.").is_empty());
}

#[test]
fn control_a_list_with_both_in_one_item_fires() {
    assert_eq!(
        found("rounding: in {floor, half_up away from zero, ceil}"),
        ["away from zero"]
    );
}

#[test]
fn every_reading_is_a_list_item_only_where_its_item_holds_nothing_else() {
    // Every reading as an item of its own beside the mode's item, and the same
    // item with a word in front of it, which makes it a clause about the item
    // before it rather than a term in a list.
    for r in READINGS {
        let direction = if *r == QUALIFIED { "up " } else { "" };
        let clause = format!("rounding: in {{floor, half_up, a tie {direction}{r}, ceil}}");
        assert_eq!(found(&clause), [*r], "{clause}");
        // The magnitude reading needs a direction word in its own item to be a
        // reading at all, so its item is never bare and it has no list form.
        if *r != QUALIFIED {
            let bare = format!("rounding: in {{floor, half_up, {r}, ceil}}");
            assert!(found(&bare).is_empty(), "{bare}");
        }
    }
}

#[test]
fn control_a_parenthetical_is_not_a_list() {
    // Two commas, and an item of it carrying a clause rather than a term. The
    // reader that counted commas let both of these through.
    assert_eq!(
        found(
            "`half_up` rounds to nearest and, for negative inputs, rounds the tie to the \
               larger magnitude."
        ),
        ["larger magnitude"]
    );
    assert_eq!(
        found("For `half_up` the tie at -2.5 goes to -3, away from zero, like Java."),
        ["away from zero"]
    );
}

#[test]
fn an_empty_segment_between_two_delimiters_is_not_a_list_item() {
    // A closing bracket and then a comma cut an empty segment between them, and
    // read as an item it is a bare name beside the reading, which made each of
    // these a list. Every bracket pair the cut knows, with and without a
    // conjunction opening the reading's item, at two aside lengths: the walk
    // covered only a two-word aside before, and a longer one is not a narrower
    // case of it, since `is_bare` counts the words in the aside's own segment
    // rather than in the one holding the reading.
    for (open, close) in [("(", ")"), ("{", "}")] {
        for lead in ["", "but ", "and "] {
            for aside in ["see below", "see the note below"] {
                let text =
                    format!("`half_up` sends a tie {open}{aside}{close}, {lead}away from zero.");
                assert_eq!(found(&text), ["away from zero"], "{text}");
            }
        }
    }
    // The control: a real list with a bracketed item still reads as a list,
    // because the neighbour looked for is the nearest item holding text.
    assert!(
        found("The modes floor, away from zero, half_up (the ruled one), and ceil.").is_empty()
    );
    // A one-word aside is not walked above: `listed_apart` counts an item's
    // words rather than checking for a name, so a bare single word beside the
    // reading reads as a list neighbour and silences it wrongly.
    // `sentences/catalogue.rs` carries the sentence it costs.
    assert!(found("`half_up` sends a tie (aside) away from zero.").is_empty());
}

#[test]
fn a_clause_stating_the_denotation_is_not_also_giving_the_other_reading() {
    // Every spelling of the denotation, each ahead of the reading in one
    // clause, walked whole rather than sampled.
    for d in DENOTES {
        let text = format!("`half_up` sends a tie {d}, where ties away from zero would give -3.");
        assert!(found(&text).is_empty(), "{text}");
    }
    // The control: the same sentence without the denotation fires, so the
    // silence is the statement rather than the shape of the contrast.
    assert_eq!(
        found(
            "`half_up` sends a tie to the nearest slot, where ties away from zero would give -3."
        ),
        ["away from zero"]
    );
    // And a denied denotation does not excuse the reading that follows it,
    // which is what keeps the escape from being a way to write the defect.
    for d in DENOTES {
        let text = format!("`half_up` is not {d}; a tie goes away from zero.");
        assert!(!found(&text).is_empty(), "{text}");
    }
}

#[test]
fn a_denotation_conjoined_with_the_other_reading_does_not_excuse_it() {
    // Every spelling of the denotation against every conjunction. A clause that
    // says the mode goes toward positive infinity and away from zero asserts
    // both, and the escape above is for a contrast, not for a conjunction.
    for d in DENOTES {
        for c in CONJUNCTIONS {
            let text = format!("`half_up` goes {d} {c} away from zero.");
            assert_eq!(found(&text), ["away from zero"], "{text}");
            // A comma before the conjunction puts the reading in a segment of
            // its own, and the conjunction opening that segment still joins it.
            let apart = format!("`half_up` goes {d}, {c} away from zero.");
            assert_eq!(found(&apart), ["away from zero"], "{apart}");
            // The conjunction stands somewhere in the reading's own segment
            // other than directly in front of the reading, with words of its
            // own between the two: the escape looks for a conjunction anywhere
            // in that stretch rather than only immediately before the reading,
            // so this is not a narrower case of the two above.
            let mid = format!("`half_up` goes {d}, {c} somehow it turns away from zero.");
            assert_eq!(found(&mid), ["away from zero"], "{mid}");
        }
    }
}

#[test]
fn a_conjunction_in_an_earlier_segment_leaves_the_contrast_standing() {
    // Every conjunction in the segment between the denotation and the contrast,
    // joining two worked values rather than the reading to the statement. This
    // is the oracle's own comment in `arvo-format`, which the conjunction rule
    // refused while it read the whole stretch rather than the reading's segment.
    for d in DENOTES {
        for c in CONJUNCTIONS {
            let text = format!(
                "`half_up` sends a tie {d} at every sign, {c} `-2.5` goes to `-2`, where ties \
                 away from zero would give `-3`."
            );
            assert!(found(&text).is_empty(), "{text}");
        }
    }
    // The control: the same sentence with the denotation taken out fires, so the
    // silence is the statement standing ahead rather than the comma.
    for c in CONJUNCTIONS {
        let text = format!(
            "`half_up` sends a tie to the nearest slot, {c} `-2.5` goes to `-2`, where ties away \
             from zero would give `-3`."
        );
        assert_eq!(found(&text), ["away from zero"], "{text}");
    }
}
