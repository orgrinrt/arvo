//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Walking the panel's own prose, which the registry's checker never sees.
//!
//! **This file declares no lint.** It is the shared half of the arms that read
//! the research tree rather than the registry: the ledger list, the archive
//! table, the file walkers, and the two citation finders. Each lint over the
//! corpus lives in its own file and takes a directory, so the same predicate
//! runs against the committed panel and against a planted tree with nothing
//! swapped out.
//!
//! The engine resolves the citations inside registry rows. Nothing resolves the
//! ones in the corpus those rows are drawn from, and that is where the rot is: a
//! member file cites the ledger it was written against, the ledger's headings
//! move, and the citation keeps reading as a citation.
//!
//! **No count appears in this comment.** The first repair put one here and it
//! was stale within the hour, twice over: the repair itself removed citations,
//! and the paragraph announcing the count then contained the string it was
//! counting. The numbers live in the tests, which measure them.

use std::fs;
use std::path::{Path, PathBuf};

use mockspace::LintError;

/// One finding against a file in the corpus, at the severity that blocks every
/// gate.
///
/// A corpus finding does name a file and a line, unlike a registry one: a
/// member file is written once and never edited, so the line stays true, and a
/// reader given the line can open it. The tree walked is named as the crate so
/// the rendered location reads `research/<file>:<line>`.
pub fn finding(
    lint: &'static str,
    file: &str,
    line: usize,
    message: String,
) -> LintError {
    let mut e = LintError::error("research".to_string(), line, lint, message);
    e.path = Some(file.to_string());
    e
}

/// The panel whose corpus these arms walk, relative to the mock directory.
pub const PANEL: &str = "research/202608072330_the-numeral-canon-panel";

/// A directory of dead files, every one prefixed `OLD_`, and how a citation
/// into it is spelled.
///
/// `at` is relative to the panel directory, so a planted tree can carry either
/// archive and the arms need no absolute path.
pub struct Archive {
    /// What this archive is called when a report has to name it.
    pub name:   &'static str,
    /// The string a citation into it begins with.
    pub prefix: &'static str,
    /// Where the files sit, relative to the panel directory.
    pub at:     &'static str,
}

/// Every archive the rename covers.
///
/// **Adding one here is the whole of what it takes to cover it**, which is the
/// point: the second archive existed for as long as the first and was reachable
/// only by editing two functions, so nobody did. The arms were written against
/// the literal string `seed/`, one archive of four files, while the same rename
/// had been applied to a second archive of 203, and seven citations into it
/// were written in the dead spelling with nothing reporting any of them.
pub const ARCHIVES: &[Archive] = &[
    Archive {
        name:   "seed",
        prefix: "seed/",
        at:     "seed",
    },
    Archive {
        name:   "closed formalization panel",
        prefix: "formalization-spec-panel/",
        at:     "../202607301300_formalization-spec-panel",
    },
];

/// The ledgers that are still being written, and are therefore read as current.
///
/// The same list the registry-side arm refuses line citations into, and for the
/// same reason: these are the files a reader treats as saying what is true now.
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

/// The registry files, by stem, as a citation writes them.
///
/// A citation writes the filename rather than a path, so this matches on the
/// name. A hyphenated file is named for the namespace it opens with.
pub const REGISTRY_FILES: &[&str] = &[
    "proposal",
    "ruling",
    "question",
    "probe",
    "law",
    "retirement",
    "obligation",
    "dimension",
    "topic",
    "strategy",
];

/// The panel directory inside one mock workspace.
pub fn panel_dir(mock_dir: &Path) -> PathBuf {
    mock_dir.join(PANEL)
}

/// Every `.md` file under a directory, sorted, so a report is stable.
pub fn markdown(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    collect(dir, &mut out, &["md"]);
    out.sort();
    out
}

/// Every committed script under a directory, sorted.
pub fn scripts(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    collect(dir, &mut out, &["sh", "py", "rs", "awk"]);
    out.sort();
    out
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>, extensions: &[&str]) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out, extensions);
        } else if path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| extensions.contains(&e))
        {
            out.push(path);
        }
    }
}

/// How a file is named in a report: relative to the tree that was walked.
pub fn shown(path: &Path, dir: &Path) -> String {
    path.strip_prefix(dir).unwrap_or(path).display().to_string()
}

/// Every `<prefix><file>.md` a line mentions, for one archive's prefix.
///
/// Reads the bare path rather than a reference expression, because the corpus
/// writes these in prose and in backticks rather than as `{{ seed::... }}`.
/// Anything after the `.md` (a line number, a range, a section) is dropped: the
/// question here is only whether the file is there.
///
/// **A citation naming a subdirectory is not matched**, since the extension
/// finder stops at the first `.md` and a path with a directory component keeps
/// its slashes. That is a known residue rather than an oversight: two citations
/// into a probe directory of the closed panel are outside every arm here, and
/// both name a directory rather than a file.
pub fn archive_citations(line: &str, prefix: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = line.as_bytes();
    let mut i = 0;
    while let Some(at) = line[i..].find(prefix) {
        let start = i + at;
        // A prefix that is part of a longer word is somebody else's path.
        if start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'/') {
            i = start + prefix.len();
            continue;
        }
        let rest = &line[start..];
        let end = rest
            .find(".md")
            .map(|e| e + 3)
            .unwrap_or_else(|| rest.find(char::is_whitespace).unwrap_or(rest.len()));
        let cite = rest[..end].to_string();
        if cite.ends_with(".md") {
            out.push(cite);
        }
        i = start + end.max(prefix.len());
    }
    out
}

/// Every `<stem><extension>:<line>` a line mentions, for a set of stems.
///
/// One finder for both line-citation arms, because they differ only in the
/// extension and the list of stems they accept, and two copies of this walk had
/// already drifted apart on which characters a stem may carry.
pub fn line_citations(line: &str, extension: &str, stems: &[&str]) -> Vec<String> {
    let needle = format!("{extension}:");
    let mut out = Vec::new();
    for (at, _) in line.match_indices(&needle) {
        let after = &line[at + needle.len()..];
        let digits: String = after.chars().take_while(char::is_ascii_digit).collect();
        if digits.is_empty() {
            continue;
        }
        let before = &line[..at];
        let start = before
            .rfind(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '_')
            .map_or(0, |i| i + 1);
        let stem = &before[start..];
        // A hyphenated registry file is named for the namespace it opens with,
        // and a ledger carries no hyphen, so taking the leading segment covers
        // both without a second rule.
        let base = stem.split('-').next().unwrap_or(stem);
        if stems.contains(&base) {
            out.push(format!("{stem}{extension}:{digits}"));
        }
    }
    out
}

/// The ledger stems, without the extension, as a line citation writes them.
pub fn ledger_stems() -> Vec<&'static str> {
    LIVING_LEDGERS
        .iter()
        .map(|l| l.trim_end_matches(".md"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_archive_citation_is_read_out_of_prose_and_a_longer_word_is_not_one() {
        assert_eq!(
            archive_citations("see `seed/OLD_thing.md` for it", "seed/"),
            ["seed/OLD_thing.md"]
        );
        // The discrimination: a prefix inside a longer path or word belongs to
        // somebody else, and taking it would report a file this archive never
        // held.
        assert!(archive_citations("mock/research/seed/x.md", "seed/").is_empty());
        assert!(archive_citations("theseed/x.md", "seed/").is_empty());
    }

    #[test]
    fn an_archive_citation_drops_whatever_follows_the_extension() {
        assert_eq!(
            archive_citations("seed/OLD_a.md:44 and seed/b.md#head", "seed/"),
            ["seed/OLD_a.md", "seed/b.md"]
        );
    }

    #[test]
    fn a_line_citation_needs_digits_and_a_stem_the_caller_named() {
        assert_eq!(
            line_citations("cites proposal.toml:211 here", ".toml", REGISTRY_FILES),
            ["proposal.toml:211"]
        );
        // The three controls. No digits is a filename rather than a citation, a
        // stem nobody named is another project's file, and a hyphenated
        // registry file is named for the namespace it opens with.
        assert!(line_citations("proposal.toml: see below", ".toml", REGISTRY_FILES).is_empty());
        assert!(line_citations("Cargo.toml:12", ".toml", REGISTRY_FILES).is_empty());
        assert_eq!(
            line_citations("law-the-later-topics.toml:9", ".toml", REGISTRY_FILES),
            ["law-the-later-topics.toml:9"]
        );
    }

    #[test]
    fn a_ledger_line_citation_reads_the_ledger_stems_and_nothing_else() {
        let stems = ledger_stems();
        assert_eq!(
            line_citations("see AGREEMENTS.md:120", ".md", &stems),
            ["AGREEMENTS.md:120"]
        );
        // A numbered member file is written once and never edited, which is
        // what makes a line citation into one honest. Reporting those would
        // turn the arm into noise over most of the tree.
        assert!(line_citations("see 214_seat.md:120", ".md", &stems).is_empty());
    }

    #[test]
    fn several_citations_on_one_line_are_all_reported_rather_than_the_first() {
        assert_eq!(
            line_citations("RULES.md:4 and OPTIONS.md:9", ".md", &ledger_stems()),
            ["RULES.md:4", "OPTIONS.md:9"]
        );
    }

    #[test]
    fn every_ledger_stem_is_matched_so_the_list_and_the_finder_agree() {
        // The list is written once and read twice, here and by the registry
        // side. A stem the finder cannot match is a ledger nothing protects,
        // and nothing else would say so.
        for stem in ledger_stems() {
            let line = format!("{stem}.md:1");
            assert_eq!(
                line_citations(&line, ".md", &ledger_stems()),
                [format!("{stem}.md:1")],
                "`{stem}` is on the ledger list and the finder does not see it"
            );
        }
    }

    #[test]
    fn a_walk_over_a_directory_that_is_not_there_is_empty_rather_than_a_panic() {
        let nowhere = Path::new("/nonexistent-tree-for-this-test");
        assert!(markdown(nowhere).is_empty());
        assert!(scripts(nowhere).is_empty());
    }

    #[test]
    fn the_panel_directory_hangs_off_the_mock_directory_it_is_handed() {
        assert_eq!(
            panel_dir(Path::new("/x/mock")),
            Path::new("/x/mock").join(PANEL)
        );
    }
}
