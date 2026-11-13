//! The tests of `canon_citations`.

#![cfg(test)]

use super::*;
use crate::canon_lint_testkit::view;
use crate::canon_rows::JOIN;

#[test]
fn a_citation_ending_in_a_line_number_names_the_file_before_it() {
    assert_eq!(
        file_named("panel::202608072330_the-numeral-canon-panel::65_knuth_number_systems::519"),
        Some("65_knuth_number_systems")
    );
}

#[test]
fn a_citation_ending_in_a_heading_names_the_file_before_it_too() {
    assert_eq!(
        file_named("panel::202608072330_the-numeral-canon-panel::OPTIONS::#q41-whether-an-arm"),
        Some("OPTIONS")
    );
}

#[test]
fn a_citation_with_no_anchor_ends_at_the_file() {
    // The case that separates this from `rsplit`: there is nothing to
    // strip, so the last segment is the answer rather than the one before.
    assert_eq!(
        file_named("panel::202608072330_the-numeral-canon-panel::74_giesen_consolidation"),
        Some("74_giesen_consolidation")
    );
    assert_eq!(file_named("panel::AGREEMENTS"), Some("AGREEMENTS"));
}

#[test]
fn a_one_segment_citation_names_no_file() {
    assert_eq!(file_named("panel"), None);
    assert_eq!(file_named(""), None);
}

#[test]
fn a_two_segment_citation_keeps_its_last_segment_even_when_that_is_a_number() {
    // `segments.len() >= 3` is what decides it. Stripping here would leave
    // the root as the file, which names no file at all.
    assert_eq!(file_named("panel::519"), Some("519"));
}

#[test]
fn the_dropped_disjunct_changed_no_answer() {
    // The control on the simplification in `is_anchor`. Every segment the
    // removed clause would have matched begins with `#`, which the first
    // test already takes, so this asserts the two spellings agree on the
    // shape the clause was written for rather than asserting a fixture.
    let with_a_dash = "#q41-whether-an-arms-predicate-may-read-data";
    assert!(is_anchor(with_a_dash));
    assert!(with_a_dash.contains('-') && with_a_dash.starts_with('#'));
    // And a segment carrying a dash without the hash is not an anchor,
    // which is the file name half of every panel citation.
    assert!(!is_anchor("202608072330_the-numeral-canon-panel"));
}

#[test]
fn two_anchors_into_one_file_are_one_source() {
    // The reason this returns a set. A count of `provenance` entries reads
    // two anchors into one member file as two arrivals, and a member file
    // has one author.
    let one = "panel::202608072330_the-numeral-canon-panel::74_giesen_consolidation";
    let joined = [format!("{one}::543"), format!("{one}::251")].join(JOIN);
    let v = view(&[("proposal::p", &[("provenance", &joined)])], &[]);
    let files = files_cited(&v, "proposal::p");
    assert_eq!(files.len(), 1, "{files:?}");
    assert!(files.contains("74_giesen_consolidation"), "{files:?}");
}

#[test]
fn two_anchors_into_two_files_are_two_sources() {
    let root = "panel::202608072330_the-numeral-canon-panel";
    let joined = [
        format!("{root}::65_knuth_number_systems_derived_cold::519"),
        format!("{root}::66_dolan_number_systems_derived_cold::255"),
    ]
    .join(JOIN);
    let v = view(&[("proposal::p", &[("provenance", &joined)])], &[]);
    assert_eq!(files_cited(&v, "proposal::p").len(), 2);
}

#[test]
fn a_row_with_no_provenance_cites_no_file() {
    let v = view(&[("proposal::p", &[("says", "something")])], &[]);
    assert!(files_cited(&v, "proposal::p").is_empty());
    assert!(files_cited(&v, "proposal::nosuch").is_empty());
}

#[test]
fn an_entailment_check_on_a_consolidation_is_not_one() {
    // The four arms that separate this from a substring match, each with a
    // real file behind it. An independent read of a consolidation is the
    // opposite of a compression, and a substring match would take it.
    assert!(!is_a_consolidation(
        "75_arntzen_entailment_check_on_the_number_system_consolidation"
    ));
    assert!(!is_a_consolidation(
        "64_ringer_entailment_check_on_the_format_consolidation"
    ));
    assert!(is_a_consolidation(
        "74_giesen_consolidation_the_number_system_concept"
    ));
    assert!(is_a_consolidation(
        "63_spj_consolidation_the_format_concept"
    ));
}

#[test]
fn a_port_of_the_consolidations_is_not_a_consolidation() {
    // Plural is what keeps it out, and that is thin. Asserted here so a
    // rename to the singular changes the answer visibly in a test rather
    // than silently in a count.
    assert!(!is_a_consolidation(
        "182_orchard_porting_the_four_consolidations"
    ));
}

#[test]
fn the_extension_does_not_change_the_verdict() {
    assert!(is_a_consolidation(
        "74_giesen_consolidation_the_number_system_concept.md"
    ));
    assert!(!is_a_consolidation("65_knuth_number_systems_derived_cold"));
}

#[test]
fn a_line_anchor_is_digits_and_a_heading_is_not() {
    let root = "panel::202608072330_the-numeral-canon-panel::OPTIONS";
    assert!(is_line_anchor(&format!("{root}::2656")));
    assert!(!is_line_anchor(&format!("{root}::#q41-whether-an-arm")));
    assert!(!is_line_anchor(root));
    assert!(!is_line_anchor(""));
}

const PANEL_FILES: [&str; 6] = [
    "76_cold_derivation.md",
    "79_attacking_the_two_cold_derivations.md",
    "90_giesen_consolidation_derived_algebraic_laws.md",
    "98_second_read.md",
    "106_giesen_consolidation_the_strategy_axis.md",
    "AGREEMENTS.md",
];

fn set<'a>(files: &[&'a str]) -> BTreeSet<&'a str> {
    files.iter().copied().collect()
}

#[test]
fn a_files_number_is_the_digits_before_its_first_underscore() {
    assert_eq!(number_of("79_fallin_attacking.md"), Some(79));
    assert_eq!(number_of("244_orchard_consolidation"), Some(244));
    assert_eq!(number_of("AGREEMENTS"), None);
    assert_eq!(number_of("_leading"), None);
    assert_eq!(number_of("12ab_mixed"), None);
    assert_eq!(number_of(""), None);
}

#[test]
fn the_consolidations_are_read_off_the_file_names_sorted() {
    let numbers = consolidation_numbers(PANEL_FILES);
    assert_eq!(numbers, vec![90, 106]);
    let shuffled = consolidation_numbers([PANEL_FILES[4], PANEL_FILES[2], PANEL_FILES[4]]);
    assert_eq!(shuffled, vec![90, 106], "sorted and each once");
}

/// The units the arrival arms read against, each naming its members as a
/// consolidation's own text does.
fn units() -> Vec<Consolidation> {
    let unit = |number: u32, members: &[u32]| {
        Consolidation {
            number,
            members: members.iter().copied().collect(),
        }
    };
    vec![
        unit(63, &[55, 56]),
        unit(74, &[65, 66, 67]),
        unit(90, &[76, 77, 79, 80]),
        unit(106, &[93, 94, 97, 98]),
    ]
}

const PREAMBLE: &str = "# 90. Consolidation\n\n**Position:** members compressed: `76`, \
    `77` (cold), `79`, `80` (attackers), `51:93-117`, the check `91`, and `74` read as \
    input.\n\n## 1. Body\n\nThis cites `82` outside the preamble.\n";

#[test]
fn the_preamble_names_the_members_and_the_body_does_not() {
    let m = members_named(PREAMBLE, 90, &[63, 74, 90, 106]);
    assert_eq!(m, [51, 76, 77, 79, 80].into_iter().collect(), "{m:?}");
}

/// A number at or above the consolidation's own is a later file, and the
/// number of another consolidation is an input it read, not a member.
#[test]
fn a_later_file_and_another_consolidation_are_not_members() {
    let m = members_named(PREAMBLE, 90, &[63, 74, 90, 106]);
    assert!(!m.contains(&91), "a check numbered after it: {m:?}");
    assert!(!m.contains(&74), "a consolidation read as input: {m:?}");
}

#[test]
fn a_read_end_to_end_paragraph_names_members_too() {
    let text = "# 53\n\nNo members in the preamble.\n\n## 1. Body\n\n**Read end to end:** \
                `44`, `45`, `46`, `INTENTS.md`,\n`00_brief.md`.\n\nProse citing `47`.\n";
    let m = members_named(text, 53, &[53]);
    assert_eq!(m, [0, 44, 45, 46].into_iter().collect(), "{m:?}");
}

#[test]
fn a_token_that_is_a_count_or_a_path_is_no_file_number() {
    assert_eq!(file_number_opening("76"), Some(76));
    assert_eq!(file_number_opening("55b"), Some(55));
    assert_eq!(file_number_opening("16_probes/p5b"), Some(16));
    assert_eq!(file_number_opening("51:93-117"), Some(51));
    assert_eq!(file_number_opening("12ab_mixed"), None);
    assert_eq!(file_number_opening("INTENTS.md"), None);
    assert_eq!(file_number_opening("300ns"), None);
}

#[test]
fn a_member_and_the_consolidation_over_it_are_one_arrival() {
    let files = set(&["79_attacking_the_two_cold_derivations", "90_giesen_consolidation_x"]);
    assert_eq!(arrivals(&files, &units()), 1);
}

/// The control: a refusal of every consolidation would pass the arm above
/// and read the same here, so two topics arriving apart must stay two.
#[test]
fn consolidations_of_two_topics_are_two_arrivals() {
    let files = set(&["63_spj_consolidation_the_format", "106_giesen_consolidation_axis"]);
    assert_eq!(arrivals(&files, &units()), 2);
}

/// The control for the absorption: two blind members of one unit are two
/// authors, and the consolidation over them adds none. Filing every file of
/// a unit as one arrival would refuse the `two_experts` tier itself.
#[test]
fn two_members_of_one_unit_stay_two_arrivals_with_their_consolidation_beside_them() {
    let pair = set(&["65_knuth_derived_cold", "66_dolan_derived_cold"]);
    assert_eq!(arrivals(&pair, &units()), 2);
    let with =
        set(&["65_knuth_derived_cold", "66_dolan_derived_cold", "74_giesen_consolidation_x"]);
    assert_eq!(
        arrivals(&with, &units()),
        2,
        "the consolidation is absorbed"
    );
}

#[test]
fn a_member_of_one_topic_and_the_consolidation_of_another_are_two_arrivals() {
    let files = set(&["79_attacking", "106_giesen_consolidation_axis"]);
    assert_eq!(arrivals(&files, &units()), 2);
}

#[test]
fn two_members_of_two_topics_are_two_arrivals() {
    let files = set(&["76_cold", "98_second_read"]);
    assert_eq!(arrivals(&files, &units()), 2);
}

#[test]
fn a_file_with_no_number_or_no_consolidation_after_it_is_its_own_arrival() {
    let files = set(&["AGREEMENTS", "OPTIONS", "300_late"]);
    assert_eq!(arrivals(&files, &units()), 3);
    assert_eq!(
        arrivals(&set(&["79_a", "80_b"]), &[]),
        2,
        "no consolidation known"
    );
}

#[test]
fn an_empty_set_is_no_arrivals() {
    assert_eq!(arrivals(&BTreeSet::new(), &units()), 0);
}

#[test]
fn a_ledger_is_found_with_and_without_its_extension() {
    let root = "panel::202608072330_the-numeral-canon-panel";
    assert_eq!(
        ledger_named(&format!("{root}::OPTIONS::2656")),
        Some("OPTIONS")
    );
    assert_eq!(
        ledger_named(&format!("{root}::AGREEMENTS.md::468")),
        Some("AGREEMENTS")
    );
}

#[test]
fn a_numbered_member_file_is_not_a_ledger() {
    // The control. An arm matching every panel file would refuse the
    // corpus's own citation style, and the corpus is right about it: a
    // numbered member file is written once, so a line into it is honest.
    let root = "panel::202608072330_the-numeral-canon-panel";
    assert_eq!(
        ledger_named(&format!(
            "{root}::109_bellard_the_primitive_derived_cold::156"
        )),
        None
    );
    assert_eq!(ledger_named(""), None);
}
