//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A standing is a claim about the work behind a row, so it is read against
//! what the row cites rather than against what its note says.
//!
//! `standing` is the field that decides whether a proposal is close to canon,
//! and the schema constrains its spelling and nothing else. A row may say
//! `cross_topic`, the strongest tier the panel produces, while citing one file
//! once, and every gate stays green. That was measured rather than assumed: the
//! transcript is in `214_probes/`, together with the positive control showing
//! the schema does refuse a value outside the set, so the green is a fact about
//! what the gate reads and not about whether it runs.
//!
//! Two arms, and they are necessary conditions rather than sufficient ones.
//! Neither can tell whether two authors really arrived separately. Both can
//! tell whether a reader could go and find out, which is the property that
//! decays silently and the one a later reader needs.
//!
//! # Why a consolidation is not the source
//!
//! `ruling::the_canon_is_written_once_at_the_end` states it: a consolidation is
//! the topic's best available compression and **has no standing beyond that**,
//! and it is input rather than canon in miniature. A consolidation names, per
//! sentence, the member files that established each claim. A row that cites
//! only the consolidation has replaced those with a pointer at the compression,
//! so the claim survives and the ability to check it does not.
//!
//! The remedy is additive and cheap: keep the consolidation citation, which is
//! where the wording came from, and add the establishing files it names. It has
//! been applied by hand once already, to
//! `proposal::conversion_and_resolution_are_one_obligation_at_two_arities`,
//! whose own note records the repair. This arm is that repair as a class.

use std::collections::BTreeSet;

use crate::{Finding, Registry};

/// The standings that assert more than one independent arrival.
///
/// `contested` is absent on purpose: it says somebody disagreed, which is not a
/// count of arrivals and is not inflated by resting on one file.
const MULTI_ARRIVAL: &[&str] = &["two_experts", "three_or_more", "cross_topic"];

/// The path segment separating a citation's parts.
const SEP: &str = "::";

/// Which file a citation names, as the segment before its anchor.
///
/// A citation is `root::path::anchor` and the path may have any depth, so the
/// file is the last segment that is not the anchor. An anchor is a line number
/// or a `#heading`; anything else means the citation ends at the file and there
/// is no anchor to strip.
fn file_named(citation: &str) -> Option<&str> {
    let segments: Vec<&str> = citation.split(SEP).collect();
    if segments.len() < 2 {
        return None;
    }
    let last = segments[segments.len() - 1];
    let is_anchor = last.starts_with('#')
        || (!last.is_empty() && last.chars().all(|c| c.is_ascii_digit()))
        || last.contains('-') && last.chars().next().is_some_and(|c| c == '#');
    if is_anchor && segments.len() >= 3 {
        Some(segments[segments.len() - 2])
    } else {
        Some(last)
    }
}

/// The distinct files a row's provenance names.
fn files_cited(row: &crate::Row) -> BTreeSet<&str> {
    row.list("provenance")
        .iter()
        .filter_map(|c| file_named(c))
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

/// A standing asserting several arrivals while citing at most one file.
///
/// Independence is between authors, and a numbered member file has one author,
/// so one citation cannot exhibit two arrivals however the note describes them.
/// The row may still be right. What it cannot be is checked, and a claim nobody
/// can locate is one nobody can overturn.
pub fn standing_claims_more_arrivals_than_it_cites(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for row in reg.of("proposal") {
        let Some(standing) = row.get("standing") else {
            continue;
        };
        if !MULTI_ARRIVAL.contains(&standing) {
            continue;
        }
        let files = files_cited(row);
        if files.len() >= 2 {
            continue;
        }
        let named = files
            .iter()
            .copied()
            .next()
            .unwrap_or("nothing at all")
            .to_string();
        out.push(Finding::new(
            "standing-rests-on-one-source",
            row.addr(),
            format!(
                "`standing` is `{standing}`, which asserts more than one independent \
                 arrival, and `provenance` names `{named}` and no second file. One file \
                 has one author, so a reader cannot reach the second arrival from this \
                 row. Cite the files that reached it, or state the standing the citations \
                 support."
            ),
        ));
    }
    out
}

/// A proposal whose whole provenance is a consolidation.
///
/// `ruling::the_canon_is_written_once_at_the_end` puts a consolidation at input
/// rather than at source: it is a compression with no standing beyond that. The
/// consolidation names its own establishing files per sentence, so the repair
/// is to add them beside it rather than to move the citation.
pub fn a_proposal_resting_only_on_a_consolidation(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for row in reg.of("proposal") {
        let files = files_cited(row);
        if files.is_empty() || !files.iter().all(|f| is_a_consolidation(f)) {
            continue;
        }
        let named: Vec<&str> = files.iter().copied().collect();
        out.push(Finding::new(
            "proposal-rests-only-on-a-consolidation",
            row.addr(),
            format!(
                "`provenance` names {named:?} and nothing else, and every one of those is \
                 a consolidation. A consolidation is the topic's best available \
                 compression and has no standing beyond that, so this row rests on a \
                 restatement rather than on what established the claim. The consolidation \
                 names the establishing files per sentence; add them beside it."
            ),
        ));
    }
    out
}

/// A row imposed rather than established, pointing at the instrument that
/// established it.
///
/// The registry's own header states the test: *a claim that could be measured
/// false is not `normative` however definitional its grammar, and it carries
/// the region it was established in or it is not here at all.* An `evidence`
/// entry names a probe, a probe is an instrument, and an instrument that could
/// have returned the other answer is what `measured` means. So the two fields
/// contradict each other, and the contradiction matters because `normative` is
/// one of the two kinds that carry no region: filing a measured claim as an
/// imposition silently widens it from the model width it was established at to
/// everywhere, without touching the predicate the widening would have shown up
/// in.
pub fn an_imposition_resting_on_an_instrument(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for row in reg.of("proposal") {
        let kind = row.get("sentence_kind").unwrap_or("");
        if kind != "normative" && kind != "definition" {
            continue;
        }
        let evidence = row.list("evidence");
        if evidence.is_empty() {
            continue;
        }
        out.push(Finding::new(
            "an-imposition-resting-on-an-instrument",
            row.addr(),
            format!(
                "`sentence_kind` is `{kind}`, which says the claim is imposed rather than \
                 established and is why it carries no region, and `evidence` names \
                 {evidence:?}. An instrument that could have returned the other answer is \
                 what `measured` means. Either the claim is measured and owes the region \
                 its instrument was run at, or it is imposed and owes no evidence."
            ),
        ));
    }
    out
}
