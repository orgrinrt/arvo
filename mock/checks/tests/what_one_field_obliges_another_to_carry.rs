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
