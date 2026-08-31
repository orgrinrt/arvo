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

/// A measurement resting on an instrument nobody should quote.
///
/// **The gate above was accidentally strong while the `probe` namespace was
/// empty**: a `measured` row could name no probe and was reported, and it could
/// name no usable one either, because there were none of any kind. The seat
/// that filled the namespace predicted, before its own run, that finishing the
/// job would weaken the check, and it was right. Naming a probe is now enough
/// to pass, and a probe can be defective, withdrawn, or one whose own `control`
/// field says nothing was run.
///
/// So citing is not the bar. **What a measurement owes is an instrument whose
/// figures may be used**, and the three states this reports are the three ways
/// one cannot be:
///
/// - `defective`, where a known defect means the numbers are wrong.
/// - `withdrawn`, where the author retracted it.
/// - `sound` on paper with a `control` saying no case that had to fail was run,
///   which is the quietest of the three and the one the corpus is full of: a
///   probe that cannot come out any other way produces a number and is not an
///   instrument.
pub fn measurements_resting_on_an_unusable_instrument(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for row in reg.of("proposal") {
        let Some(kind) = row.get("sentence_kind") else {
            continue;
        };
        if !RAN_SOMETHING.contains(&kind) {
            continue;
        }
        for cited in row.list("evidence") {
            let Some(probe) = reg.of("probe").find(|p| &p.id == cited) else {
                continue; // a slug naming no row is the engine's report, not this arm's
            };
            let standing = probe.get("standing").unwrap_or("");
            if standing == "defective" || standing == "withdrawn" {
                out.push(Finding::new(
                    "measurement-rests-on-an-unusable-instrument",
                    row.addr(),
                    format!(
                        "`evidence` names `probe::{cited}`, whose `standing` is `{standing}`. \
                         Its figures are not to be used, so a claim resting on it is not a \
                         measurement. Cite a sound instrument, or mark the sentence as the \
                         argument it now is."
                    ),
                ));
                continue;
            }
            if standing == "uncontrolled" {
                out.push(Finding::new(
                    "measurement-rests-on-an-uncontrolled-instrument",
                    row.addr(),
                    format!(
                        "`evidence` names `probe::{cited}`, whose own `control` says no case \
                         that had to fail was run. An instrument that cannot come out any \
                         other way produces a number and is not an instrument, so what this \
                         row has is a figure rather than a measurement."
                    ),
                ));
            }
        }
    }
    out
}

// There used to be a function here that read a `control` field and guessed
// whether it was an admission that no case had to fail. It is gone, and the
// three versions it went through are the argument for why the answer is a
// declared value rather than a better parser.
//
// **Version one, five fixed phrases: one admission caught in five.** The seat
// writing the rows predicted that before its own run and then measured it per
// row. The misses were the natural phrasings, "None stated in the material
// read" and "None, and none is needed". The same seat reported it had written
// its own admission in the form the matcher could see, and named that as
// adopting an unstated convention rather than letting it pass. A `control`
// field says what is true, not what a substring can find.
//
// **Version two, the opening word with a carve-out for a control that fired:
// six in nine.** A second reader found the first hole: two probes open with the
// identical sentence, "None was run as a case that had to fail", and the
// matcher caught one, because the other contained `reported` inside a
// counterfactual about what a different outcome would have meant. Reading all
// seventy-five fields in full then found two more, failing the other way:
// "no single one of them carries a case that had to fail" and "Neither arm
// carries a planted case that had to fail" open with neither word and carry
// neither phrase, because **the negation sits on a different noun.**
//
// So: a word list cannot tell a report from a counterfactual, and cannot see a
// negation it is not adjacent to. Both directions of failure, on record, from
// two independent reads. Lengthening the list moves the false negative.
//
// **Version three is `standing = "uncontrolled"`**, which is data, is read
// directly, and parses no sentence. Every row now carries a standing somebody
// set by reading its field in full, so there is nothing left to back-stop and
// the guess is deleted rather than kept at half accuracy.
//
// The measurement that closed it: nine of seventy-nine ran with no case that
// had to fail, sixty-seven carry one. **The prose instrument's set is a strict
// subset of the declared one**, which is what says the triage missed nothing
// rather than merely disagreeing with the parser.
//
// ---
//
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
    let definitions: Vec<&Row> = reg
        .of("proposal")
        .filter(|row| row.get("sentence_kind") == Some("definition"))
        .collect();

    let mut seen: BTreeMap<&str, Vec<&Row>> = BTreeMap::new();
    for row in &definitions {
        let Some(term) = row.get("defines") else {
            continue; // reported by the arm below
        };
        // A supersession is one definition replacing another, and only where
        // the row it replaces defines the same term.
        //
        // **The first version skipped any row carrying a `supersedes` at all**,
        // which is the whole edge in one word: a definition that supersedes
        // something unrelated stopped being a definition for this arm's
        // purposes, so a genuine rival pair vanished. Found by a check whose
        // own case-that-had-to-fail refused to fire, which is the better half
        // of the story: the arm was green over a selection this line had
        // emptied.
        let replaces_a_rival = row.list("supersedes").iter().any(|slug| {
            definitions
                .iter()
                .any(|other| &other.id == slug && other.get("defines") == Some(term))
        });
        if replaces_a_rival {
            continue;
        }
        seen.entry(term).or_default().push(row);
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

/// A live claim restating something the corpus retired.
///
/// 176 retirements exist and each holds the retired sentence in the words a
/// later reader would search for, which is what makes this checkable at all.
/// **Nothing was checking it.** The seat that wired the first answering edges
/// named the gap itself: it had not read `retirement.toml`, so a claim it wired
/// to a question could be one somebody had already struck out, and nothing in
/// its process would have caught that.
///
/// The failure is worse than a stale row. A retired claim wired to a question
/// reports that question **settled**, by a sentence the corpus has said must
/// not be cited, and the reader who follows the edge finds an answer rather
/// than a retirement.
///
/// **Deliberately high precision and low recall.** It matches a long verbatim
/// run rather than a paraphrase, so it finds a claim carried over wholesale and
/// misses one reworded. A fuzzy version would report a shared subject as a
/// restatement, and a check nobody believes is a check nobody runs.
pub fn rows_restating_a_retired_claim(reg: &Registry) -> Vec<Finding> {
    /// How many consecutive words make a match distinctive rather than a
    /// coincidence of vocabulary. Eight is long enough that two authors do not
    /// write it twice by accident on this subject, and short enough to survive
    /// a claim being quoted with its edges trimmed.
    const RUN: usize = 8;

    let retired: Vec<(&str, String, Vec<String>)> = reg
        .of("retirement")
        .filter_map(|r| {
            let claim = r.get("claim")?;
            let words: Vec<String> = normalise(claim);
            Some((r.id.as_str(), claim.to_string(), words))
        })
        .collect();

    let mut out = Vec::new();
    for namespace in ["proposal", "ruling"] {
        for row in reg.of(namespace) {
            let Some(says) = row.get("says") else {
                continue;
            };
            let haystack = normalise(says).join(" ");
            for (slug, claim, words) in &retired {
                if words.len() < RUN {
                    // A short claim has no distinctive run, so the whole of it
                    // has to appear or nothing is reported. Anything looser on
                    // a five-word sentence matches the subject rather than the
                    // claim.
                    if !words.is_empty() && haystack.contains(&words.join(" ")) {
                        out.push(hit(row, slug, claim));
                    }
                    continue;
                }
                if words.windows(RUN).any(|w| haystack.contains(&w.join(" "))) {
                    out.push(hit(row, slug, claim));
                }
            }
        }
    }
    out
}

fn hit(row: &Row, slug: &str, claim: &str) -> Finding {
    let shown: String = claim.chars().take(120).collect();
    Finding::new(
        "a-live-row-restates-a-retired-claim",
        row.addr(),
        format!(
            "`says` carries a run of `retirement::{slug}`, whose whole purpose is that the \
             sentence is not cited again: \"{shown}\". If the row is right the retirement is \
             wrong and says so; if the retirement is right this row answers nothing."
        ),
    )
}

/// Lowercase words, punctuation dropped, so a quotation matches the sentence it
/// quotes across a difference in emphasis or a trailing comma.
fn normalise(s: &str) -> Vec<String> {
    s.split_whitespace()
        .map(|w| {
            w.chars()
                .filter(|c| c.is_alphanumeric())
                .flat_map(char::to_lowercase)
                .collect::<String>()
        })
        .filter(|w| !w.is_empty())
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
    // `probe` joined this list late, reported by the seat that filled it: a
    // reader hunting for the instrument behind a figure searches for what it
    // measured, and a probe row is exactly as unfindable without keywords as
    // any other.
    for namespace in [
        "ruling",
        "proposal",
        "question",
        "obligation",
        "retirement",
        "probe",
    ] {
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

/// A `note` claiming one of its own row's fields is empty, where it is not.
///
/// **The commonest way a row goes stale, and the quietest.** A note is written
/// when a field genuinely was empty; a later pass fills the field; nothing
/// re-reads the note. It then reads as a caveat a reader should honour, and
/// what it actually says is what the row looked like some rounds ago.
///
/// Fifteen rows here carried "`evidence` is empty and the measured-implies-
/// evidence check is red on this row", written truthfully when `probe.toml` did
/// not exist. It exists now and the edges were wired, so some of those notes
/// describe their row and some contradict it, and no reader can tell which
/// without opening the row.
///
/// **Matched on the row's own field names**, not on a word list, so the arm
/// works for any field a note might claim is empty and does not need editing
/// when a namespace gains one.
pub fn notes_claiming_an_empty_field_that_is_not(reg: &Registry) -> Vec<Finding> {
    /// The ways this corpus says a field holds nothing.
    ///
    /// A phrase, not a keyword: `empty` alone matches "the empty region" and
    /// every other legitimate use of the word.
    const SAYS_EMPTY: &[&str] = &["is empty", "are empty", "carries none", "carries nothing"];

    let mut out = Vec::new();
    for row in &reg.rows {
        let Some(note) = row.get("note") else {
            continue;
        };
        for field in row.lists.keys() {
            // The claim is written as a backticked field name followed closely
            // by one of the phrases. Close, because a note that mentions
            // `evidence` in one sentence and "is empty" three sentences later
            // about something else is not this.
            let needle = format!("`{field}`");
            let mut from = 0;
            while let Some(at) = note[from ..].find(&needle) {
                let start = from + at + needle.len();
                let window = &note[start .. note.len().min(start + 24)];
                if SAYS_EMPTY.iter().any(|p| window.contains(p)) && !row.list(field).is_empty() {
                    out.push(Finding::new(
                        "note-claims-an-empty-field-that-is-not",
                        row.addr(),
                        format!(
                            "its `note` says `{field}` is empty and `{field}` holds {} entr{}. \
                             The note was true when it was written and a later pass filled the \
                             field, so it now reads as a caveat about the row it sits on and \
                             describes a different one.",
                            row.list(field).len(),
                            if row.list(field).len() == 1 { "y" } else { "ies" }
                        ),
                    ));
                    break;
                }
                from = start;
            }
        }
    }
    out
}
