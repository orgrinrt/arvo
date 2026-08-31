//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A line citation into a file that moves is a lie that reads as a citation.
//!
//! The `panel` root is declared frozen, which is what makes a line number
//! honest for the numbered member files: each is written once and never edited.
//! The same root holds the panel's living ledgers, and freezing is per root
//! rather than per file, so the declaration permits a line citation into those
//! too. It should not.
//!
//! The failure is the worst shape there is. An edit anywhere above a cited line
//! shifts it, the citation still resolves, the engine reports nothing, and the
//! anchor now points at different text. The only case that fails loudly is a
//! line past the end of the file, and that is the case that matters least.
//!
//! A heading anchor into the same file is fine, and is what to write instead: a
//! rename stops it resolving, which is a report rather than a lie.

use crate::{Finding, Registry};

/// The ledgers in the panel tree that are edited after they are cited.
///
/// Named by file stem, which is how a citation writes them. Everything else
/// under the root is a numbered member file or a probe artifact, both written
/// once.
pub const LIVING_LEDGERS: &[&str] = &[
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

/// The fields that hold citations.
///
/// `provenance` everywhere, plus `lives` on a probe, which points at the
/// committed instrument. Both are declared `ref[]`, so the engine resolves them
/// and this arm only adds what the engine's own check cannot express.
const CITATION_FIELDS: &[&str] = &["provenance", "lives"];

/// Whether the last segment of a citation is a line number rather than an anchor.
fn is_line_anchor(citation: &str) -> bool {
    citation
        .rsplit("::")
        .next()
        .is_some_and(|last| !last.is_empty() && last.chars().all(|c| c.is_ascii_digit()))
}

/// Which living ledger a citation names, if it names one.
fn ledger_named(citation: &str) -> Option<&'static str> {
    citation.split("::").find_map(|segment| {
        let stem = segment.split('.').next().unwrap_or(segment);
        LIVING_LEDGERS.iter().copied().find(|l| *l == stem)
    })
}

/// Line citations into a ledger that is still being written.
pub fn line_citations_into_living_ledgers(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for row in &reg.rows {
        for field in CITATION_FIELDS {
            for citation in row.list(field) {
                if !is_line_anchor(citation) {
                    continue;
                }
                let Some(ledger) = ledger_named(citation) else {
                    continue;
                };
                out.push(Finding::new(
                    "line-citation-into-a-living-ledger",
                    row.addr(),
                    format!(
                        "`{field}` cites `{citation}`, which is a line number into \
                         `{ledger}.md`. That file is still being written, so the line moves \
                         and the citation keeps resolving to whatever ends up there. Cite a \
                         heading instead: a rename fails loudly, a shifted line does not."
                    ),
                ));
            }
        }
    }
    out
}

/// A citation with too few segments to name anything.
///
/// The engine reports an unresolvable one. It cannot report a citation that is
/// merely a root, or a bare filename with no root, because both fail earlier
/// and less legibly than a wrong path does.
pub fn citations_with_no_target(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for row in &reg.rows {
        for field in CITATION_FIELDS {
            for citation in row.list(field) {
                if citation.split("::").count() < 2 {
                    out.push(Finding::new(
                        "citation-names-no-target",
                        row.addr(),
                        format!(
                            "`{field}` carries `{citation}`, which has no `root::path` split. \
                             A citation with one segment names a root and nothing in it."
                        ),
                    ));
                }
            }
        }
    }
    out
}
