//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The gap a seat named about its own work, closed as a check.
//!
//! 176 retirements sit in this canon, each holding the struck-out sentence in
//! the words somebody would search for. The seat that wired the first answering
//! edges had not read any of them, and said so: a claim it wired to a question
//! could be one the corpus had already retired, and nothing in its process
//! would have caught it.
//!
//! That is not a stale row. A retired claim wired to a question reports the
//! question **settled**, by a sentence the corpus says must not be cited, and
//! the reader who follows the edge meets an answer where a retirement should
//! have stopped them.

use arvo_checks::{canon, parse, shape};

/// Two live rows carry a retired sentence, pinned by name.
///
/// The arm fired on its first run over the committed canon, which is what the
/// seat that named the gap expected and could not check for itself.
///
/// **The first is the one worth reading.** A live row defines the output of the
/// container derivation in the exact sentence a retirement strikes out, and
/// that retirement is not a quibble: the defined set appears on both sides of
/// its own third clause with an antitone operator, so adding a fact removes
/// another and neither fixpoint tie-breaks. Two experts on the reading, one
/// verified formalisation, and a replacement already named. So the canon
/// currently carries as a definition a sentence the corpus established is not
/// one.
///
/// Neither is repaired here, and neither is a repair anybody should make in
/// passing: the retirement's replacement is three separated predicates, one per
/// job the sentence was doing at once, which is a rewrite rather than an edit.
/// **What this pins is that a third does not appear**, and the two are on the
/// worklist by name.
#[test]
fn the_rows_restating_a_retired_claim_are_the_two_already_named() {
    const KNOWN: &[&str] = &[
        "proposal::an_axis_the_realisation_map_does_not_read_is_not_a_type_parameter",
        "proposal::an_output_of_a_derivation_is_a_fact_a_downstream_site_cannot_recover",
    ];
    let mut found: Vec<String> = shape::rows_restating_a_retired_claim(&canon())
        .into_iter()
        .map(|f| f.at)
        .collect();
    found.sort();
    found.dedup();
    assert_eq!(
        found, KNOWN,
        "a live row carries a sentence the corpus retired. Either the row is right and the \
         retirement is wrong, in which case the retirement says so, or the retirement is \
         right and the row answers nothing. Repair it rather than adding it here."
    );
}

/// The control, and the reason the arm is not a way of saying two files exist.
///
/// A verbatim carry must fire; a row about the same subject in its own words
/// must not. Both directions, because an arm reporting every row that mentions
/// a retired subject would be reporting the subject and would be switched off
/// within a day.
#[test]
fn a_verbatim_carry_fires_and_a_shared_subject_does_not() {
    let reg = parse(
        "planted.toml",
        r#"
[[retirement]]
id = "the_figure_that_matched_no_sweep"
claim = "the sweep separates twenty one thousand two hundred and four of thirty two thousand seven hundred and sixty eight cells"
why = "the figure appears in no committed artifact and no sweep of that shape can produce it"
kind = "wrong"

[[proposal]]
id = "carries_it_wholesale"
says = "As established, the sweep separates twenty one thousand two hundred and four of thirty two thousand seven hundred and sixty eight cells, so the split is real."

[[proposal]]
id = "about_the_same_subject_in_its_own_words"
says = "The separating cells were counted directly and the count is far smaller than the one reported."

[[ruling]]
id = "an_unrelated_call"
says = "The strategy set is not closed at exactly four."
"#,
    );
    let found = shape::rows_restating_a_retired_claim(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(
        found[0].at.contains("carries_it_wholesale"),
        "a row writing about the subject in its own words is not restating the claim: {}",
        found[0].at
    );
    assert!(
        found[0].says.contains("the_figure_that_matched_no_sweep"),
        "the report names which retirement, so a reader can decide which of the two is \
         wrong: {}",
        found[0].says
    );
}

/// A ruling is read too, because op restating a retired claim is the same
/// defect and a worse one: it would carry his authority.
#[test]
fn a_ruling_is_read_as_well_as_a_proposal() {
    let reg = parse(
        "planted.toml",
        r#"
[[retirement]]
id = "a_struck_sentence"
claim = "the container premise is localised to a single clause of the candidate"
why = "two signatures independently found it reaches three clauses, and a third made it four"
kind = "wrong"

[[ruling]]
id = "a_call_carrying_it"
says = "Given that the container premise is localised to a single clause of the candidate, proceed."
"#,
    );
    let found = shape::rows_restating_a_retired_claim(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(found[0].at.starts_with("ruling::"), "{}", found[0].at);
}

/// A short retirement needs the whole of itself to match.
///
/// Below the distinctive-run length there is no run to be distinctive, so
/// anything looser reports a shared subject. This is the boundary stated as a
/// test rather than left to whoever next reads the constant.
#[test]
fn a_short_claim_matches_only_in_full() {
    let reg = parse(
        "planted.toml",
        r#"
[[retirement]]
id = "a_short_one"
claim = "the strategies are partially ordered"
why = "nothing establishes an ordering and two of the four are incomparable"
kind = "wrong"

[[proposal]]
id = "carries_all_of_it"
says = "It follows that the strategies are partially ordered by how many chain laws each honours."

[[proposal]]
id = "carries_part_of_it"
says = "The strategies are compared on how many chain laws each honours."
"#,
    );
    let found = shape::rows_restating_a_retired_claim(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(found[0].at.contains("carries_all_of_it"), "{}", found[0].at);
}

/// Punctuation and emphasis differences do not hide a carry, because a
/// quotation lifted into prose picks up neither reliably.
#[test]
fn a_carry_survives_a_difference_in_punctuation_and_case() {
    let reg = parse(
        "planted.toml",
        r#"
[[retirement]]
id = "a_struck_sentence"
claim = "A law verdict is established at the widths the instrument reached and no further"
why = "the sentence was carried without the widths, and read as a universal"
kind = "unpredicated"

[[proposal]]
id = "carries_it_reformatted"
says = "**a law verdict is established at the widths the instrument reached, and no further.**"
"#,
    );
    assert_eq!(
        shape::rows_restating_a_retired_claim(&reg).len(),
        1,
        "a carry that gained a comma and lost its capital is the same carry"
    );
}
