//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conditional obligations, which a schema can state none of.
//!
//! Every arm here is required-if-something-else, and each has both directions
//! planted: the case that must be reported, and the neighbouring case that must
//! not be, because an arm that reports everything passes the first half of its
//! own control and is useless.

use arvo_checks::{canon, parse, shape};

#[test]
fn the_committed_canon_leaves_no_refusal_without_an_alternative() {
    let found = shape::refusals_without_an_instead(&canon());
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn the_committed_canon_has_an_instrument_behind_every_measurement() {
    let found = shape::measured_without_evidence(&canon());
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn the_committed_canon_agrees_with_itself_about_regions() {
    let found = shape::predicate_disagrees_with_the_sentence_kind(&canon());
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn the_committed_canon_stamps_nothing_on_an_ack() {
    let found = shape::stamps_from_an_unratified_ruling(&canon());
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn the_committed_canon_leaves_no_row_unfindable() {
    let found = shape::rows_with_no_keywords(&canon());
    assert!(found.is_empty(), "{found:#?}");
}

/// The rulings with nothing verbatim behind them, pinned by name.
///
/// This one does not assert an empty list, because the list is not empty and
/// will not become empty: for these four the corpus itself holds no words of
/// op's, only somebody's report of which option he took. Asserting zero would
/// be a red test nobody can fix, and ignoring it would stop the arm reporting a
/// fifth.
///
/// So the known hole is written down and anything else fails. The most
/// consequential of the four governs when anything becomes canon at all, and
/// its only record is an agent's sentence reading "He took the third."
#[test]
fn the_rulings_with_no_verbatim_are_the_four_the_corpus_has_no_words_for() {
    const KNOWN: &[&str] = &[
        "ruling::the_branch_waits_for_the_canon",
        "ruling::the_canon_is_written_once_at_the_end",
        "ruling::the_d_numbered_decisions_are_dead",
        "ruling::the_family_question_wants_the_comparison_first",
    ];
    let mut found: Vec<String> = shape::rulings_with_no_verbatim(&canon())
        .into_iter()
        .map(|f| f.at)
        .collect();
    found.sort();
    assert_eq!(
        found, KNOWN,
        "a ruling claims op's authority with no words of his behind it. Either quote the \
         source, or add it here with the reason the corpus holds none."
    );
}

#[test]
fn a_ruling_with_a_quote_is_not_reported_and_one_without_is() {
    let reg = parse(
        "planted.toml",
        r#"
[[ruling]]
id = "his_own_words"
says = "the strategy set is not closed at four"
quote = "the strategy set is not closed at exactly four"

[[ruling]]
id = "somebody_elses_words"
says = "he took the third option"
"#,
    );
    let found = shape::rulings_with_no_verbatim(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(
        found[0].at.contains("somebody_elses_words"),
        "{}",
        found[0].at
    );
    assert_eq!(found[0].kind, "ruling-carries-no-verbatim");
}

/// A proposal has no `quote` by construction: there are no words but the
/// panel's, which is what `says` holds. Reporting one would be reporting the
/// namespace rather than a defect in a row.
#[test]
fn a_proposal_is_not_asked_for_a_verbatim() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
says = "the partition is derivable without the observability rule"
"#,
    );
    assert!(shape::rulings_with_no_verbatim(&reg).is_empty());
}

#[test]
fn a_refusal_with_no_alternative_is_reported_in_either_namespace() {
    let reg = parse(
        "planted.toml",
        r#"
[[ruling]]
id = "a_refusal"
kind = "refusal"

[[proposal]]
id = "another_refusal"
kind = "refusal"

[[ruling]]
id = "a_refusal_that_answers"
kind = "refusal"
instead = "the overlay stays, and it costs the support this brings"
"#,
    );
    let found = shape::refusals_without_an_instead(&reg);
    assert_eq!(
        found.len(),
        2,
        "both namespaces carry the obligation and the one that meets it is not reported: \
         {found:#?}"
    );
    assert!(found.iter().all(|f| f.kind == "refusal-owes-an-instead"));
}

/// A deferral owes the same sentence for a different reason.
///
/// Op declining to make a call and handing it back is a distinct act from
/// refusing a thing, and it was being recorded as `refusal` because nothing
/// else fitted. A reader of a design would take that as arvo refusing to do
/// something, which is the opposite of what happened. What a deferral owes is
/// who it went back to.
#[test]
fn a_deferral_owes_the_same_sentence_and_says_which_kind_it_was() {
    let reg = parse(
        "planted.toml",
        r#"
[[ruling]]
id = "handed_back_with_nothing"
kind = "deferral"

[[ruling]]
id = "handed_back_to_somebody"
kind = "deferral"
instead = "the panel settles it: impl detail, optimal and converged to by experts, iteratively"
"#,
    );
    let found = shape::refusals_without_an_instead(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(
        found[0].says.contains("deferral"),
        "the report names which kind it was, or a reader fixing it writes the wrong \
         sentence: {}",
        found[0].says
    );
}

#[test]
fn a_measurement_with_no_instrument_is_reported_and_an_argument_is_not() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_bare_number"
sentence_kind = "measured"

[[proposal]]
id = "a_reasoned_claim"
sentence_kind = "argument"

[[proposal]]
id = "a_real_measurement"
sentence_kind = "measured"
evidence = ["the_sweep"]
"#,
    );
    let found = shape::measured_without_evidence(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(found[0].at.contains("a_bare_number"), "{}", found[0].at);
}

#[test]
fn an_imposed_proposition_carrying_a_region_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_firewall"
sentence_kind = "normative"
predicate = ["fraction_width: 0"]
"#,
    );
    let found = shape::predicate_disagrees_with_the_sentence_kind(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "an-imposed-proposition-carries-a-region");
}

#[test]
fn an_established_claim_with_no_region_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_theorem_with_no_region"
sentence_kind = "theorem"
"#,
    );
    let found = shape::predicate_disagrees_with_the_sentence_kind(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "an-established-claim-carries-no-region");
}

/// The two halves of that arm point opposite ways, so the pair that satisfies
/// both must be silent or the arm is simply reporting everything.
#[test]
fn the_two_correct_shapes_are_both_silent() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "an_imposed_one"
sentence_kind = "normative"

[[proposal]]
id = "an_established_one"
sentence_kind = "theorem"
predicate = ["fraction_width: 0", "threads: 1"]
"#,
    );
    assert!(
        shape::predicate_disagrees_with_the_sentence_kind(&reg).is_empty(),
        "an imposed proposition with no region and an established one with a region are \
         both correct, and an arm reporting either is reporting the rule rather than a \
         breach of it"
    );
}

#[test]
fn a_stamp_from_anything_but_a_ratification_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[ruling]]
id = "an_ack_that_stamps"
rung = "stated"
ratifies = ["some_claim"]

[[ruling]]
id = "a_real_ratification"
rung = "ratified"
ratifies = ["some_other_claim"]

[[ruling]]
id = "an_ack_that_stamps_nothing"
rung = "stated"
"#,
    );
    let found = shape::stamps_from_an_unratified_ruling(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(
        found[0].at.contains("an_ack_that_stamps"),
        "{}",
        found[0].at
    );
    assert!(found[0].says.contains("stated"), "{}", found[0].says);
}

#[test]
fn a_row_with_no_keywords_is_reported_only_in_the_namespaces_a_reader_searches() {
    let reg = parse(
        "planted.toml",
        r#"
[[ruling]]
id = "unfindable"

[[dimension]]
id = "found_by_enumeration"
what = "an axis"

[[ruling]]
id = "findable"
keywords = ["width", "carrier"]
"#,
    );
    let found = shape::rows_with_no_keywords(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(found[0].at.contains("unfindable"), "{}", found[0].at);
}
