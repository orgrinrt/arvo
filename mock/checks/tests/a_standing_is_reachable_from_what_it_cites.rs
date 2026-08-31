//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! `standing` decides how close a proposal is to canon, and until these arms
//! ran, nothing read it except the schema, which reads its spelling.
//!
//! The measurement is in `214_probes/standing_is_unconstrained.txt`: a row set
//! to `cross_topic`, the strongest tier the panel produces, while citing one
//! consolidation once, passed `cargo mock --lint-only` and the whole of this
//! crate's suite. The positive control beside it sets the same field to
//! `seventeen_experts` and the schema refuses it by name, so the gate is live
//! and what it validates is the spelling.
//!
//! Both arms are necessary conditions. Neither can tell whether two authors
//! really arrived separately, and neither pretends to. What they establish is
//! that a reader can go and find out.

use arvo_checks::{canon, parse, provenance};

/// The list is not asserted empty, because it is not empty and the work of
/// emptying it is per row and per topic rather than mechanical.
///
/// What is pinned is that it does not grow. A row porting a claim out of a
/// consolidation without its establishing files is the defect this arm names,
/// and it is added one row at a time by whoever is porting that topic, so a
/// ratchet is what catches it at the moment it happens.
///
/// The number is the count after the number-system topic was second-read, which
/// took it from 65 to 48. One of that topic's rows is deliberately still in
/// it: the consolidator's own contribution, where the consolidation genuinely
/// is the source and the flag is honest signal rather than a defect to repair.
/// Lowering it as topics are repaired is the intended direction and the
/// assertion says so; raising it needs a reason in the same commit.
const CONSOLIDATION_ONLY_CEILING: usize = 48;

#[test]
fn no_proposal_newly_rests_only_on_a_consolidation() {
    let found = provenance::a_proposal_resting_only_on_a_consolidation(&canon());
    assert!(
        found.len() <= CONSOLIDATION_ONLY_CEILING,
        "a proposal citing only a consolidation rests on a compression that op ruled has \
         no standing beyond being one. {} rows do, and the ceiling is {}. The repair is \
         additive: the consolidation names its establishing files per sentence, so add \
         them beside it.\n{found:#?}",
        found.len(),
        CONSOLIDATION_ONLY_CEILING
    );
}

/// Same shape, same reason, and this one matters more.
///
/// Under the rule that two agreeing experts ratify, these are exactly the rows
/// eligible for promotion, and not one of them names a second source. A row
/// asserting several arrivals while citing one file is the inflation the
/// schema comment warns about, in the one form that is mechanically visible.
const MULTI_ARRIVAL_ON_ONE_SOURCE_CEILING: usize = 29;

#[test]
fn a_multi_arrival_standing_names_more_than_one_file() {
    let found = provenance::standing_claims_more_arrivals_than_it_cites(&canon());
    assert!(
        found.len() <= MULTI_ARRIVAL_ON_ONE_SOURCE_CEILING,
        "{} rows assert more than one independent arrival while citing at most one file, \
         and the ceiling is {}.\n{found:#?}",
        found.len(),
        MULTI_ARRIVAL_ON_ONE_SOURCE_CEILING
    );
}

#[test]
fn a_two_expert_row_citing_one_file_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
standing = "two_experts"
provenance = ["panel::202608072330_the-numeral-canon-panel::74_giesen_consolidation_the_number_system_concept::614"]
"#,
    );
    let found = provenance::standing_claims_more_arrivals_than_it_cites(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "standing-rests-on-one-source");
    assert!(found[0].says.contains("two_experts"), "{}", found[0].says);
}

/// The control that makes the arm above mean something: two files pass.
///
/// An arm refusing every multi-arrival standing would be refusing the tier
/// rather than checking it, and would read exactly the same on a clean canon.
#[test]
fn a_two_expert_row_citing_two_files_is_fine() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
standing = "two_experts"
provenance = [
  "panel::202608072330_the-numeral-canon-panel::65_knuth_number_systems_derived_cold::519",
  "panel::202608072330_the-numeral-canon-panel::66_dolan_number_systems_derived_cold::255",
]
"#,
    );
    assert!(
        provenance::standing_claims_more_arrivals_than_it_cites(&reg).is_empty(),
        "two files is what a two-expert standing has to name"
    );
}

/// One expert citing one file is the ordinary honest row and must pass.
#[test]
fn a_one_expert_row_citing_one_file_is_fine() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
standing = "one_expert"
provenance = ["panel::202608072330_the-numeral-canon-panel::67_kiselyov_which_prefix_earns_the_word::613"]
"#,
    );
    assert!(provenance::standing_claims_more_arrivals_than_it_cites(&reg).is_empty());
}

/// `contested` says somebody disagreed rather than that several arrived, so it
/// is not inflated by resting on one file and must not be reported.
#[test]
fn a_contested_row_citing_one_file_is_fine() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
standing = "contested"
provenance = ["panel::202608072330_the-numeral-canon-panel::67_kiselyov_which_prefix_earns_the_word::613"]
"#,
    );
    assert!(provenance::standing_claims_more_arrivals_than_it_cites(&reg).is_empty());
}

/// Two citations into one file are one file, which is the case a naive count of
/// provenance entries gets wrong and the case the registry actually contains.
#[test]
fn two_anchors_into_one_file_are_one_source() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
standing = "two_experts"
provenance = [
  "panel::202608072330_the-numeral-canon-panel::74_giesen_consolidation_the_number_system_concept::543",
  "panel::202608072330_the-numeral-canon-panel::74_giesen_consolidation_the_number_system_concept::251",
]
"#,
    );
    let found = provenance::standing_claims_more_arrivals_than_it_cites(&reg);
    assert_eq!(
        found.len(),
        1,
        "two anchors into one file is one author, not two: {found:#?}"
    );
}

#[test]
fn a_row_resting_only_on_a_consolidation_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
standing = "one_expert"
provenance = ["panel::202608072330_the-numeral-canon-panel::74_giesen_consolidation_the_number_system_concept::475"]
"#,
    );
    let found = provenance::a_proposal_resting_only_on_a_consolidation(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "proposal-rests-only-on-a-consolidation");
}

/// The repair passes: the consolidation stays and an establishing file joins it.
///
/// Without this the arm would be telling authors to drop the consolidation
/// citation, which is the wrong fix. The wording came from the consolidation
/// and that is worth recording; what was missing is where the claim came from.
#[test]
fn a_consolidation_beside_its_source_is_fine() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
standing = "one_expert"
provenance = [
  "panel::202608072330_the-numeral-canon-panel::74_giesen_consolidation_the_number_system_concept::526",
  "panel::202608072330_the-numeral-canon-panel::71_orchard_what_crosses_between_two_systems::399",
]
"#,
    );
    assert!(
        provenance::a_proposal_resting_only_on_a_consolidation(&reg).is_empty(),
        "the repair is additive and must not itself be reported"
    );
}

/// A row citing only member files is the shape the arm exists to leave alone.
#[test]
fn a_row_citing_only_member_files_is_fine() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
standing = "one_expert"
provenance = ["panel::202608072330_the-numeral-canon-panel::68_leroy_what_the_pipeline_certifies::226"]
"#,
    );
    assert!(provenance::a_proposal_resting_only_on_a_consolidation(&reg).is_empty());
}

/// An entailment check *of* a consolidation is not a consolidation.
///
/// It is an independent read, which is the opposite of a compression, and a
/// substring match on the word would take it. This is the case that made the
/// arm match whole segments.
#[test]
fn an_entailment_check_on_a_consolidation_is_not_one() {
    assert!(!provenance::is_a_consolidation(
        "75_arntzen_entailment_check_on_the_number_system_consolidation"
    ));
    assert!(!provenance::is_a_consolidation(
        "64_ringer_entailment_check_on_the_format_consolidation"
    ));
    assert!(provenance::is_a_consolidation(
        "74_giesen_consolidation_the_number_system_concept"
    ));
    assert!(provenance::is_a_consolidation(
        "63_spj_consolidation_the_format_concept"
    ));
}

/// A port of several consolidations is not one of them either.
///
/// `182_orchard_porting_the_four_consolidations` is plural, which is what keeps
/// it out, and that is thin. It is asserted here so that if the file is ever
/// renamed to the singular the arm's behaviour changes visibly in a test rather
/// than silently in a count.
#[test]
fn a_port_of_the_consolidations_is_not_a_consolidation() {
    assert!(!provenance::is_a_consolidation(
        "182_orchard_porting_the_four_consolidations"
    ));
}

/// The extension is optional in a citation, so both spellings must agree.
#[test]
fn the_extension_does_not_change_the_verdict() {
    assert!(provenance::is_a_consolidation(
        "74_giesen_consolidation_the_number_system_concept.md"
    ));
}

/// A row with no provenance at all is not this arm's report to make.
///
/// The schema requires the field, so an absent one is already refused, and an
/// arm firing here would be reporting the same defect twice under two names.
#[test]
fn a_row_with_no_provenance_is_left_to_the_schema() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
standing = "one_expert"
"#,
    );
    assert!(provenance::a_proposal_resting_only_on_a_consolidation(&reg).is_empty());
}

/// Neither arm reads a namespace other than `proposal`.
///
/// A ruling cites op's own file and has no standing field in this sense; a
/// probe's `lives` is not provenance of a claim. An arm walking every row would
/// report both and mean nothing by it.
#[test]
fn a_ruling_is_not_read_as_a_proposal() {
    let reg = parse(
        "planted.toml",
        r#"
[[ruling]]
id = "a_call"
standing = "two_experts"
provenance = ["panel::202608072330_the-numeral-canon-panel::74_giesen_consolidation_the_number_system_concept::614"]
"#,
    );
    assert!(provenance::standing_claims_more_arrivals_than_it_cites(&reg).is_empty());
    assert!(provenance::a_proposal_resting_only_on_a_consolidation(&reg).is_empty());
}

/// Nothing may be imposed and established at once.
///
/// A hard zero rather than a ratchet: unlike the two above, this one is not a
/// population to work down. It is a contradiction between two fields, and a row
/// carrying it is wrong at the moment it is written.
#[test]
fn no_imposition_rests_on_an_instrument() {
    let found = provenance::an_imposition_resting_on_an_instrument(&canon());
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn a_normative_row_with_evidence_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
sentence_kind = "normative"
evidence = ["an_instrument"]
"#,
    );
    let found = provenance::an_imposition_resting_on_an_instrument(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(found[0].kind, "an-imposition-resting-on-an-instrument");
}

/// A `definition` is the other region-free kind and is caught the same way.
#[test]
fn a_definition_with_evidence_is_reported() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
sentence_kind = "definition"
evidence = ["an_instrument"]
"#,
    );
    assert_eq!(
        provenance::an_imposition_resting_on_an_instrument(&reg).len(),
        1
    );
}

/// The controls: a measured row with evidence is the ordinary correct shape,
/// and a normative row without evidence is the other correct shape. An arm
/// refusing either would be refusing the schema rather than checking it.
#[test]
fn a_measured_row_with_evidence_is_fine() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
sentence_kind = "measured"
evidence = ["an_instrument"]
"#,
    );
    assert!(provenance::an_imposition_resting_on_an_instrument(&reg).is_empty());
}

#[test]
fn a_normative_row_without_evidence_is_fine() {
    let reg = parse(
        "planted.toml",
        r#"
[[proposal]]
id = "a_claim"
sentence_kind = "normative"
"#,
    );
    assert!(provenance::an_imposition_resting_on_an_instrument(&reg).is_empty());
}
