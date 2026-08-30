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

use std::collections::BTreeMap;

use crate::{Finding, Registry, Row};

/// A ruling recording op's authority with no words of his behind it.
///
/// `says` is required and `quote` is not, which inverts the trust order: a row
/// can carry somebody's restatement of him, pass every check, and be
/// mechanically indistinguishable from one that was invented. The first port
/// landed four of exactly that shape, and one of them governs when anything
/// becomes canon at all, its only record being an agent's sentence reporting
/// which option he took.
///
/// **Not an error in the row.** Sometimes the corpus genuinely holds no
/// verbatim, and the row is the best available record of a real call. What the
/// finding says is that the hole is in the corpus and somebody should know
/// where. A row that has a reason states it in `note`, and this reports it
/// anyway, because a note is prose and this is a list.
pub fn rulings_with_no_verbatim(reg: &Registry) -> Vec<Finding> {
    reg.of("ruling")
        .filter(|row| row.get("quote").is_none_or(str::is_empty))
        .map(|row| {
            Finding::new(
                "ruling-carries-no-verbatim",
                row.addr(),
                "`says` is set and `quote` is not, so what stands behind this is somebody's \
                 restatement of him. Quote the source, or record in `note` that the corpus \
                 holds no verbatim and what it holds instead."
                    .to_string(),
            )
        })
        .collect()
}

/// A refusal or a deferral that names nothing in its place closes a question
/// and leaves it with no answer.
///
/// Both namespaces that carry the kinds, because the obligation is the same
/// wherever the answer came from. A deferral owes the same sentence for a
/// different reason: it says who the question goes back to, and without that it
/// is a question that has stopped being anybody's.
pub fn refusals_without_an_instead(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for namespace in ["ruling", "proposal"] {
        for row in reg.of(namespace) {
            if !matches!(row.get("kind"), Some("refusal" | "deferral")) {
                continue;
            }
            if row.get("instead").is_none_or(str::is_empty) {
                let kind = row.get("kind").unwrap_or("refusal");
                out.push(Finding::new(
                    "refusal-owes-an-instead",
                    row.addr(),
                    format!(
                        "the kind is `{kind}` and `instead` is empty. Closing a question \
                         without naming what happens in its place leaves nothing there, so \
                         the next reader reopens it."
                    ),
                ));
            }
        }
    }
    out
}

/// The sentence kinds that ran an instrument, and therefore owe one.
///
/// `measured` is obvious. **`enumeration` is here because this corpus calls its
/// sweeps enumerations**, which is what they are: somebody walked a bounded set
/// and reported what was in it. A walk over 4096 triples is an instrument
/// however its author labelled the sentence, and gating only `measured` left
/// the check reaching almost nothing the corpus actually ran. Reported by the
/// seat that met it, which found the gate had no purchase on its material.
///
/// `theorem` is not here: a proof owes its route, not a run. Neither is
/// `argument`, which claims no run, nor `normative`, which claims nothing at
/// all.
const RAN_SOMETHING: &[&str] = &["measured", "enumeration"];

/// A claim that ran something, with no instrument behind it.
///
/// The mark is what a later reader trusts. A row saying `measured` and citing
/// no probe is asking for a measurement's authority on an argument's evidence,
/// and the marking convention exists precisely because a canon that states a
/// theorem and a measurement in one voice loses the distinction.
pub fn measured_without_evidence(reg: &Registry) -> Vec<Finding> {
    reg.of("proposal")
        .filter(|row| {
            row.get("sentence_kind")
                .is_some_and(|k| RAN_SOMETHING.contains(&k))
        })
        .filter(|row| row.list("evidence").is_empty())
        .map(|row| {
            let kind = row.get("sentence_kind").unwrap_or("measured");
            Finding::new(
                "measured-claim-cites-no-probe",
                row.addr(),
                format!(
                    "`sentence_kind` is `{kind}` and `evidence` is empty. Name the committed \
                     instrument, or mark the sentence as the argument it is. A measurement \
                     with no instrument is an argument wearing a number, and a sweep is a \
                     measurement whatever its author called the sentence."
                ),
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
/// The two kinds that state no region, for opposite reasons.
///
/// A `normative` sentence imposes rather than establishes, so a region on it
/// says the design may violate it everywhere the region does not reach. A
/// `definition` stipulates what a term means: it is not a claim about where
/// anything holds, so a region on one is a category error rather than a
/// narrowing. Fifteen of the first seventeen rows written were marked
/// `normative` and about half were stipulations, which is what earned the
/// second value.
const REGIONLESS: &[&str] = &["normative", "definition"];

pub fn predicate_disagrees_with_the_sentence_kind(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for row in reg.of("proposal") {
        let kind = row.get("sentence_kind").unwrap_or("");
        let regionless = REGIONLESS.contains(&kind);
        let has_predicate = !row.list("predicate").is_empty();
        if regionless && has_predicate {
            out.push(Finding::new(
                "an-imposed-proposition-carries-a-region",
                row.addr(),
                format!(
                    "`sentence_kind` is `{kind}` and `predicate` is set. Neither kind states \
                     a region: an imposed proposition would be saying the design may violate \
                     it everywhere the region does not reach, and a definition is not a claim \
                     about where anything holds."
                ),
            ));
        }
        if !regionless && !has_predicate {
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

/// Two live definitions of one term.
///
/// A definition stipulates what a term means, so two rows defining the same
/// term are either a supersession, in which case the later one says so, or a
/// disagreement, in which case somebody has to resolve it. Sitting side by
/// side, both are cited and each reader gets whichever they found.
pub fn a_term_defined_twice(reg: &Registry) -> Vec<Finding> {
    let mut seen: BTreeMap<&str, Vec<&Row>> = BTreeMap::new();
    for row in reg.of("proposal") {
        if row.get("sentence_kind") != Some("definition") {
            continue;
        }
        // A supersession is one definition replacing another, not two live ones.
        if !row.list("supersedes").is_empty() {
            continue;
        }
        if let Some(term) = row.get("defines") {
            seen.entry(term).or_default().push(row);
        }
    }
    seen.into_iter()
        .filter(|(_, rows)| rows.len() > 1)
        .map(|(term, rows)| {
            let who: Vec<String> = rows.iter().map(|r| r.addr()).collect();
            Finding::new(
                "a-term-is-defined-twice",
                who.join(", "),
                format!(
                    "`{term}` is defined by {} live rows and none supersedes another. Two \
                     stipulations of one term is a supersession somebody did not record or a \
                     disagreement somebody did not resolve.",
                    rows.len()
                ),
            )
        })
        .collect()
}

/// A definition that does not say what it defines.
pub fn definitions_with_no_term(reg: &Registry) -> Vec<Finding> {
    reg.of("proposal")
        .filter(|row| row.get("sentence_kind") == Some("definition"))
        .filter(|row| row.get("defines").is_none_or(str::is_empty))
        .map(|row| {
            Finding::new(
                "definition-names-no-term",
                row.addr(),
                "`sentence_kind` is `definition` and `defines` is empty, so nothing says \
                 which term is being stipulated and nothing can tell whether it is stipulated \
                 twice."
                    .to_string(),
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
