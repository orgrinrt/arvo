//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What one field obliges another to carry.
//!
//! A schema can require a field always or never. Every obligation here is
//! conditional on a value in a sibling field, which is the shape a schema
//! cannot state and the shape most of this canon's contracts take: a refusal
//! owes an alternative, a measured claim owes an instrument, an imposed
//! proposition owes no region and must not carry one.
//!
//! Each arm is a separate function rather than one sweep, so a report names the
//! contract that was broken rather than the file that broke several.

use crate::{Finding, Registry};

/// A refusal that names nothing in its place closes a question and leaves it
/// with no answer.
///
/// Both namespaces that carry a `refusal` kind, because the obligation is the
/// same wherever the refusal came from.
pub fn refusals_without_an_instead(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for namespace in ["ruling", "proposal"] {
        for row in reg.of(namespace) {
            if row.get("kind") != Some("refusal") {
                continue;
            }
            if row.get("instead").is_none_or(str::is_empty) {
                out.push(Finding::new(
                    "refusal-owes-an-instead",
                    row.addr(),
                    "the kind is `refusal` and `instead` is empty. A refusal naming no \
                     alternative closes the question it answered and leaves nothing in its \
                     place, so the next reader reopens it."
                        .to_string(),
                ));
            }
        }
    }
    out
}

/// A claim marked as measured with no instrument behind it.
///
/// The mark is what a later reader trusts. A row saying `measured` and citing
/// no probe is asking for a measurement's authority on an argument's evidence,
/// and the marking convention exists precisely because a canon that states a
/// theorem and a measurement in one voice loses the distinction.
pub fn measured_without_evidence(reg: &Registry) -> Vec<Finding> {
    reg.of("proposal")
        .filter(|row| row.get("sentence_kind") == Some("measured"))
        .filter(|row| row.list("evidence").is_empty())
        .map(|row| {
            Finding::new(
                "measured-claim-cites-no-probe",
                row.addr(),
                "`sentence_kind` is `measured` and `evidence` is empty. Name the committed \
                 instrument, or mark the sentence as the argument it is. A measurement with \
                 no instrument is an argument wearing a number."
                    .to_string(),
            )
        })
        .collect()
}

/// A region on an imposed proposition inverts it, and its absence anywhere else
/// hides one.
///
/// An imposed proposition says the design shall do a thing. Writing a predicate
/// on it says the design may violate it everywhere the predicate does not
/// reach, which is the opposite of what it means. Every other kind of sentence
/// is established somewhere and nowhere else, and leaving the region out claims
/// the whole space.
pub fn predicate_disagrees_with_the_sentence_kind(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for row in reg.of("proposal") {
        let normative = row.get("sentence_kind") == Some("normative");
        let has_predicate = !row.list("predicate").is_empty();
        if normative && has_predicate {
            out.push(Finding::new(
                "an-imposed-proposition-carries-a-region",
                row.addr(),
                "`sentence_kind` is `normative` and `predicate` is set. A region on an \
                 imposed proposition says the design may violate it everywhere the region \
                 does not reach, which inverts it."
                    .to_string(),
            ));
        }
        if !normative && !has_predicate {
            out.push(Finding::new(
                "an-established-claim-carries-no-region",
                row.addr(),
                "`sentence_kind` is not `normative` and `predicate` is empty, so the claim \
                 reads as holding everywhere while having been established somewhere. Write \
                 the region it was established in, however narrow: `threads = 1` is a real \
                 region and a real finding."
                    .to_string(),
            ));
        }
    }
    out
}

/// A ruling that stamps a proposal without itself being ratified.
///
/// Ratification is what a stamp is. A row at any other rung claiming to promote
/// something is promoting it on nobody's authority, and the proposal it names
/// then reads as canon on the strength of an ack.
pub fn stamps_from_an_unratified_ruling(reg: &Registry) -> Vec<Finding> {
    reg.of("ruling")
        .filter(|row| !row.list("ratifies").is_empty())
        .filter(|row| row.get("rung") != Some("ratified"))
        .map(|row| {
            let rung = row.get("rung").unwrap_or("(absent)");
            Finding::new(
                "an-unratified-ruling-stamps-a-proposal",
                row.addr(),
                format!(
                    "`ratifies` is set and `rung` is `{rung}`. A stamp is a ratification; at \
                     any other rung the proposal it names becomes canon on the strength of an \
                     ack, which op's own correction says an ack is not."
                ),
            )
        })
        .collect()
}

/// A row nobody will find, because it uses none of the words they will search for.
///
/// Cheap, and it has already cost this project the same question twice: a row
/// existed, answered the question, and could not be found because the asker
/// reached for a different word than the author had. Only the two namespaces a
/// reader searches are covered; a `dimension` or a `topic` is found by
/// enumeration.
pub fn rows_with_no_keywords(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for namespace in ["ruling", "proposal", "question", "obligation", "retirement"] {
        for row in reg.of(namespace) {
            if row.list("keywords").is_empty() {
                out.push(Finding::new(
                    "row-carries-no-keywords",
                    row.addr(),
                    "`keywords` is empty. A search over row text finds whichever word the \
                     author reached for, and the reader reaches for a different one."
                        .to_string(),
                ));
            }
        }
    }
    out
}
