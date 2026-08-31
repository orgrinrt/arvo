//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Reading the shape of a citation, which four of the canon lints ask about.
//!
//! **This file declares no lint.** The engine scans each `mock/lints/*.rs` for
//! `lint()`, `cross_lint()`, `repo_lint()` and `message_lint()`, includes the
//! file as a module either way, and registers only what it found, so a module
//! defining none of them compiles into the pack and reaches nothing. That is
//! what `canon_rows.rs` and `panel_corpus.rs` already do, and it is why these
//! readers can sit beside the lints rather than inside one of them.
//!
//! A citation is `root::path::anchor`, the path may be any depth, and the
//! anchor is optional. Everything here is one of two questions about that
//! shape: which file it lands in, and whether the last segment is a line number
//! rather than a heading. The engine resolves a citation and reports one that
//! does not, so nothing here repeats that; what is left is the part the
//! resolution cannot see, which is whether an anchor that resolves is telling
//! the truth.
//!
//! **No reverse edges are read here and none would help.** `referrers` is built
//! over fields the configuration types as row references, and `provenance` and
//! `lives` are typed `ref[]`, which points into the research tree rather than
//! at another row. A citation has no target row for an edge to land on.

use std::collections::BTreeSet;

use mockspace::RegistryView;

use crate::canon_rows::list;

/// The fields that hold a citation.
///
/// `provenance` everywhere, plus `lives` on a probe, which points at the
/// committed instrument. Reading only `provenance` would leave out the one
/// namespace whose whole subject is where the evidence sits.
pub const CITATION_FIELDS: [&str; 2] = ["provenance", "lives"];

/// The ledgers in the panel tree that are edited after they are cited.
///
/// Named by file stem, which is how a citation writes them. Everything else
/// under that root is a numbered member file or a probe artifact, and both are
/// written once.
pub const LIVING_LEDGERS: [&str; 9] = [
    "AGREEMENTS",
    "OPTIONS",
    "DROPLIST",
    "RULES",
    "INTENTS",
    "PRIOR_CALLS",
    "HANDLES",
    "PERSONA_CALLS",
    "SEED_TALKING_POINTS",
];

/// The segment separator inside a citation.
const SEP: &str = "::";

/// Which file a citation names, as the segment before its anchor.
///
/// The path may have any depth, so the file is the last segment that is not the
/// anchor. An anchor is a line number or a `#heading`, and anything else means
/// the citation ends at the file with no anchor to strip.
pub fn file_named(citation: &str) -> Option<&str> {
    let segments: Vec<&str> = citation.split(SEP).collect();
    if segments.len() < 2 {
        return None;
    }
    let last = segments[segments.len() - 1];
    if is_anchor(last) && segments.len() >= 3 {
        Some(segments[segments.len() - 2])
    } else {
        Some(last)
    }
}

/// Whether a segment is an anchor rather than the file itself.
///
/// The crate this came from carried a third disjunct here, that the segment
/// contains a `-` and begins with `#`. Every string satisfying it satisfies the
/// heading test on its own, so it decided nothing and is gone. Stated rather
/// than dropped quietly, because the two spellings agree on every input and a
/// later reader comparing them would otherwise be looking for the difference.
fn is_anchor(segment: &str) -> bool {
    segment.starts_with('#') || (!segment.is_empty() && segment.chars().all(|c| c.is_ascii_digit()))
}

/// The distinct files a row's `provenance` names.
///
/// Distinct rather than counted, because two anchors into one file are one
/// author, and a count of entries reads them as two.
pub fn files_cited<'a>(reg: &'a RegistryView, q: &str) -> BTreeSet<&'a str> {
    list(reg, q, "provenance")
        .into_iter()
        .filter_map(file_named)
        .collect()
}

/// Whether a panel file is one of the topic consolidations.
///
/// Matched on a whole underscore-separated segment rather than on a substring.
/// `entailment_check_on_the_format_consolidation` ends with the word and is an
/// independent check of a consolidation rather than one, and
/// `porting_the_four_consolidations` is plural and is a port. A substring match
/// would take both.
pub fn is_a_consolidation(file: &str) -> bool {
    let stem = file.strip_suffix(".md").unwrap_or(file);
    let mut segments = stem.split('_');
    if segments.clone().any(|s| s == "entailment") {
        return false;
    }
    segments.any(|s| s == "consolidation")
}

/// Whether the last segment of a citation is a line number rather than a
/// heading.
pub fn is_line_anchor(citation: &str) -> bool {
    citation
        .rsplit(SEP)
        .next()
        .is_some_and(|last| !last.is_empty() && last.chars().all(|c| c.is_ascii_digit()))
}

/// Which living ledger a citation names, if it names one.
///
/// Matched against every segment rather than against the file alone, and with
/// the extension stripped, because a citation may write `AGREEMENTS` or
/// `AGREEMENTS.md` and the engine finds the file either way.
pub fn ledger_named(citation: &str) -> Option<&'static str> {
    citation.split(SEP).find_map(|segment| {
        let stem = segment.split('.').next().unwrap_or(segment);
        LIVING_LEDGERS.iter().copied().find(|l| *l == stem)
    })
}

#[cfg(test)]
mod tests {
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
}
