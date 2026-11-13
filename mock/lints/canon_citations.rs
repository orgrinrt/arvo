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

/// The number a panel file is filed under, the digits before its first `_`.
///
/// `None` for a ledger, a probe artifact or anything else the panel did not
/// number.
pub fn number_of(file: &str) -> Option<u32> {
    let stem = file.split('_').next()?;
    if stem.is_empty() || !stem.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    stem.parse().ok()
}

/// The numbers of the topic consolidations among the panel's file names.
///
/// Sorted and deduplicated, so [`members_named`] can leave them out of a
/// consolidation's members.
pub fn consolidation_numbers<'a>(files: impl IntoIterator<Item = &'a str>) -> Vec<u32> {
    let mut numbers: Vec<u32> = files
        .into_iter()
        .filter(|f| is_a_consolidation(f))
        .filter_map(number_of)
        .collect();
    numbers.sort_unstable();
    numbers.dedup();
    numbers
}

/// A topic consolidation and the panel files its own text says it compresses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Consolidation {
    pub number:  u32,
    pub members: BTreeSet<u32>,
}

/// The paragraph openers after which a consolidation lists what it read.
const READ_MARKERS: [&str; 3] = ["**Read end to end", "**Read in full", "**What was read"];

/// The panel files a consolidation names as its members, by number.
///
/// Read from the consolidation's own text rather than from the numbering. Two
/// places name them: the preamble before the first `## ` heading, where a
/// consolidation says what it compresses, and the paragraphs it opens with
/// `Read end to end`, `Read in full` or `What was read`. A member is a backticked
/// token opening on a file number (`` `76` ``, `` `51:93-117` ``, `` `16_probes/p5b` ``,
/// `` `55b` ``). Numbers at or above the consolidation's own and numbers of other
/// consolidations are left out: a consolidation reads another one as input
/// without compressing it.
///
/// **Where this is wrong.** A consolidation that names its members somewhere
/// else, or in prose with no backticks, has none read here, so a cited member
/// beside it is no absorption and the pair counts as two arrivals. That errs
/// toward trusting a second source, which is the direction the tier asks for
/// only where the consolidation really did rest on the file; a file it never
/// named can never be taken for a member.
pub fn members_named(text: &str, own: u32, other_consolidations: &[u32]) -> BTreeSet<u32> {
    let mut region = String::new();
    let mut in_preamble = true;
    let mut in_read = false;
    for line in text.lines() {
        if line.starts_with("## ") {
            in_preamble = false;
        }
        if line.trim().is_empty() {
            in_read = false;
        }
        if READ_MARKERS.iter().any(|m| line.starts_with(m)) {
            in_read = true;
        }
        if in_preamble || in_read {
            region.push_str(line);
            region.push('\n');
        }
    }
    region
        .split('`')
        .skip(1)
        .step_by(2)
        .filter_map(file_number_opening)
        .filter(|n| *n < own && !other_consolidations.contains(n))
        .collect()
}

/// The file number a backticked token opens on, when what follows it is the end
/// of the token or a separator a file reference uses.
fn file_number_opening(token: &str) -> Option<u32> {
    let digits = token.chars().take_while(char::is_ascii_digit).count();
    if digits == 0 {
        return None;
    }
    let rest = &token[digits ..];
    let rest = rest
        .strip_prefix(|c: char| c.is_ascii_lowercase())
        .unwrap_or(rest);
    let ends = rest.is_empty() || rest.starts_with(['_', ':', '/', '.', ' ', '-']);
    if !ends {
        return None;
    }
    token[.. digits].parse().ok()
}

/// How many independent arrivals a set of cited files holds.
///
/// Every cited file that is not a consolidation is one arrival, since two
/// members of one unit (a pair of blind cold derivations, an attacker) reached
/// their claims separately. A consolidation is one arrival only when none of
/// its own members is cited beside it: it compresses them, so it agrees with
/// them by construction and adds no second author. Which files are its members
/// is what its own text says, read by [`members_named`]. Two consolidations of
/// two topics are two arrivals, and so are a member of one topic and the
/// consolidation of another, and so is a later check of a consolidation, which
/// no consolidation names.
pub fn arrivals(files: &BTreeSet<&str>, consolidations: &[Consolidation]) -> usize {
    let members: Vec<&str> = files
        .iter()
        .copied()
        .filter(|f| !is_a_consolidation(f))
        .collect();
    let absorbed = |c: &str| {
        let Some(n) = number_of(c) else { return false };
        let Some(own) = consolidations.iter().find(|k| k.number == n) else {
            return false;
        };
        members
            .iter()
            .any(|m| number_of(m).is_some_and(|mn| own.members.contains(&mn)))
    };
    let lone = files
        .iter()
        .copied()
        .filter(|f| is_a_consolidation(f) && !absorbed(f))
        .count();
    members.len() + lone
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
#[path = "canon_citations/tests.rs"]
mod tests;
