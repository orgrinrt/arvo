//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Citations in the panel's own prose, which the registry's checker never sees.
//!
//! The engine resolves the citations inside registry rows. Nothing resolves the
//! ones in the corpus those rows are drawn from, and that is where the rot is:
//! a member file cites the ledger it was written against, the ledger's headings
//! move, and the citation keeps reading as a citation.
//!
//! Two arms, both narrow, both about the seed archive.
//!
//! The seed directory was renamed so every dead file carries an `OLD_` prefix
//! and reads as dead at a glance. The pass that landed it repointed five files
//! and wrote into `RULES.md` that no unprefixed archive citation remained,
//! which was a claim over the tree made from a check over five files. Measured:
//! **45 files, 118 citations**, two of them in the intent catalogue every
//! dispatch reads first, where the provenance of two of op's intents resolved
//! to nothing.
//!
//! So the arms are the two halves that sentence promised: nothing unprefixed in
//! a file still being written, and no `OLD_` citation naming a file that is not
//! there.
//!
//! **The landed member files are out of scope on purpose.** 116 of the 118 are
//! in numbered files, which are written once and are the record. Repointing
//! them would be editing history to make a checker green, and the reading rule
//! is one sentence: prepend `OLD_`.

use std::fs;
use std::path::{Path, PathBuf};

use crate::{repo, Finding};

/// The panel whose corpus these arms walk.
const PANEL: &str = "mock/research/202608072330_the-numeral-canon-panel";

/// The ledgers that are still being written, and are therefore read as current.
///
/// The same list `citation.rs` refuses line citations into, and for the same
/// reason: these are the files a reader treats as saying what is true now.
pub const LIVING_LEDGERS: &[&str] = &[
    "AGREEMENTS.md",
    "OPTIONS.md",
    "DROPLIST.md",
    "RULES.md",
    "INTENTS.md",
    "PRIOR_CALLS.md",
    "HANDLES.md",
    "PERSONA_CALLS.md",
    "SEED_TALKING_POINTS.md",
];

/// The panel directory in this repository.
pub fn panel_dir() -> PathBuf {
    repo().join(PANEL)
}

/// An unprefixed citation into the seed archive, in a file still being written.
pub fn unprefixed_archive_citations_in_living_ledgers(dir: &Path) -> Vec<Finding> {
    let mut out = Vec::new();
    for name in LIVING_LEDGERS {
        let path = dir.join(name);
        let Ok(text) = fs::read_to_string(&path) else {
            continue; // a ledger a panel does not have is not this arm's finding
        };
        for (n, line) in text.lines().enumerate() {
            for cite in seed_citations(line) {
                let file = cite.trim_start_matches("seed/");
                if file.starts_with("OLD_") {
                    continue;
                }
                out.push(Finding::new(
                    "unprefixed-archive-citation-in-a-living-ledger",
                    format!("{name}:{}", n + 1),
                    format!(
                        "cites `{cite}`, which does not exist: the seed archive is prefixed, \
                         so the file is `seed/OLD_{file}`. This is a ledger a reader treats \
                         as current, so a citation in it that resolves to nothing is read as \
                         provenance that was checked."
                    ),
                ));
            }
        }
    }
    out
}

/// An `OLD_`-prefixed citation naming a file that is not in the archive.
///
/// The other half of the sentence, and the half that would fail if the archive
/// were tidied again. Walks the whole panel rather than the ledgers, because an
/// `OLD_` citation is written deliberately and a dangling one is wrong wherever
/// it sits.
pub fn archive_citations_naming_nothing(dir: &Path) -> Vec<Finding> {
    let mut out = Vec::new();
    let mut files = Vec::new();
    collect_md(dir, &mut files);
    files.sort();
    for path in files {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let shown = path
            .strip_prefix(dir)
            .unwrap_or(&path)
            .display()
            .to_string();
        for (n, line) in text.lines().enumerate() {
            for cite in seed_citations(line) {
                let file = cite.trim_start_matches("seed/");
                if !file.starts_with("OLD_") {
                    continue; // the arm above owns those
                }
                if !dir.join("seed").join(file).exists() {
                    out.push(Finding::new(
                        "archive-citation-names-nothing",
                        format!("{shown}:{}", n + 1),
                        format!(
                            "cites `{cite}`, and the archive holds no such file. An `OLD_` \
                             citation is written deliberately, so a dangling one is a \
                             rename nobody followed rather than a typo."
                        ),
                    ));
                }
            }
        }
    }
    out
}

/// A committed probe that reads a tree other than the one it sits in.
///
/// **The quietest instrument defect this corpus has produced, and it was found
/// tenth.** A probe script with an absolute path to somebody's checkout does
/// not fail when run somewhere else: if that checkout exists on the host, and
/// here it does, the script succeeds and reports about a different tree. Twenty
/// of the ones carrying it are citation checkers, which is the cheapest
/// correctness tool the panel has, verifying somebody else's clone.
///
/// They were correct when written, because the panel lived in that clone. They
/// became wrong the moment the arc moved, silently, with nothing in any output
/// saying which tree had been read. **That is the shape worth naming: not a
/// broken probe, a probe that keeps working on the wrong subject.**
///
/// The repair in every case is the same and is one line: resolve the root from
/// the script's own location rather than naming it.
pub fn probes_reading_another_tree(dir: &Path) -> Vec<Finding> {
    let mut out = Vec::new();
    let mut files = Vec::new();
    collect_scripts(dir, &mut files);
    files.sort();
    for path in files {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let shown = path
            .strip_prefix(dir)
            .unwrap_or(&path)
            .display()
            .to_string();
        for (n, line) in text.lines().enumerate() {
            // A home-anchored absolute path is the whole tell. A relative path
            // is resolved against wherever the script runs, which is a
            // different and much louder failure.
            let Some(at) = line.find("/Users/") else {
                continue;
            };
            let rest = &line[at ..];
            let cited: String = rest
                .chars()
                .take_while(|c| !c.is_whitespace() && *c != '"' && *c != '\'' && *c != ')')
                .collect();
            out.push(Finding::new(
                "a-probe-reads-another-tree",
                format!("{shown}:{}", n + 1),
                format!(
                    "names `{cited}`, an absolute path outside this repository. If that path \
                     exists on the host the script succeeds against a tree that is not this \
                     one and says nothing; if it does not, the script fails for a reason \
                     nobody will connect to the move. Resolve the root from the script's own \
                     location."
                ),
            ));
        }
    }
    out
}

fn collect_scripts(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_scripts(&path, out);
        } else if path
            .extension()
            .is_some_and(|e| e == "sh" || e == "py" || e == "rs" || e == "awk")
        {
            out.push(path);
        }
    }
}

/// Every `seed/<file>.md` a line mentions.
///
/// Reads the bare path rather than a reference expression, because the corpus
/// writes these in prose and in backticks rather than as `{{ seed::... }}`.
/// Anything after the `.md` (a line number, a range, a section) is dropped: the
/// question here is only whether the file is there.
fn seed_citations(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = line.as_bytes();
    let mut i = 0;
    while let Some(at) = line[i..].find("seed/") {
        let start = i + at;
        // A `seed/` that is part of a longer word is somebody else's path.
        if start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'/') {
            i = start + 5;
            continue;
        }
        let rest = &line[start..];
        let end = rest
            .find(".md")
            .map(|e| e + 3)
            .unwrap_or_else(|| rest.find(|c: char| c.is_whitespace()).unwrap_or(rest.len()));
        let cite = rest[..end].to_string();
        if cite.ends_with(".md") {
            out.push(cite);
        }
        i = start + end.max(5);
    }
    out
}

fn collect_md(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_md(&path, out);
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
}
