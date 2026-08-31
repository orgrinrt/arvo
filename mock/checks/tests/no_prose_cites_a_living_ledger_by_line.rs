//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The largest citation-rot class here, and nothing was watching it.
//!
//! A registry row may not cite a living ledger by line, and a check refuses it.
//! Nothing looked at the prose those rows are drawn from, which is where nearly
//! all of the citations are: a ledger is rewritten as the panel moves, so a line
//! number into one names different text after any edit above it, **and the
//! citation keeps resolving the whole way**.
//!
//! **Found by repair rather than by reading.** Fifteen probes were pointed at
//! the tree they sit in instead of at a clone where the old line numbers still
//! matched. Three independently written citation checkers immediately reported
//! the same ledger's citations failing. None of them could see it before and
//! neither could any committed check.
//!
//! Four sat in ledgers, which are edited and so repairable, and opening them is
//! what shows why the class matters. One pointed at a blank line. One pointed at
//! a passage about a different subject. **One pointed at text that still
//! supported the claim it was cited for**, which is the failure this whole
//! discipline exists to prevent: the reader lands somewhere real, checks it, and
//! finds agreement that nobody wrote.

use arvo_checks::corpus;

/// A ledger cites no other ledger by line.
///
/// The repairable half, and therefore the half that is a gate.
#[test]
fn no_ledger_cites_a_living_ledger_by_line() {
    let found: Vec<_> = corpus::line_citations_into_living_ledgers_in_prose(&corpus::panel_dir())
        .into_iter()
        .filter(|f| !in_a_member_file(&f.at))
        .collect();
    assert!(
        found.is_empty(),
        "a file that gets edited cites a ledger by line, and the ledger moves under it. Cite the \
         heading: {found:#?}"
    );
}

/// The member files carry a ceiling, because a member file is not edited.
///
/// **676 when the class was first measured, across 98 files.** Repointing them
/// would be editing the record to make a checker green, which the archive arms
/// in this module decline to do for the same reason. The repair is upstream of
/// the file: a seat that cites headings writes none of these.
#[test]
fn the_member_files_ledger_citations_do_not_grow() {
    /// Measured over the committed tree. Falls only when a file is superseded.
    const CEILING: usize = 676;

    let found: Vec<_> = corpus::line_citations_into_living_ledgers_in_prose(&corpus::panel_dir())
        .into_iter()
        .filter(|f| in_a_member_file(&f.at))
        .collect();
    assert!(
        found.len() <= CEILING,
        "{} line citations into a living ledger from member files, against a ceiling of \
         {CEILING}. Brief the next seat to cite headings; do not raise this: {found:#?}",
        found.len()
    );
}

/// The control, and it has to fire on the ledgers and not on everything else.
#[test]
fn a_ledger_line_fires_and_a_heading_does_not() {
    let tmp = tempdir();
    std::fs::write(
        tmp.join("42_member.md"),
        "See `RULES.md:163` and `OPTIONS.md:1113-1115`.\n\
         Under \"Panel mechanics\" in `RULES.md`, which is a heading.\n\
         And `109_bellard_the_primitive_derived_cold.md:156`, a member file, written once.\n\
         And `seed/OLD_SETTLED.md:44`, archived and frozen.\n",
    )
    .unwrap();

    let found = corpus::line_citations_into_living_ledgers_in_prose(&tmp);
    assert_eq!(
        found.len(),
        2,
        "both ledger lines fire; the heading, the member file and the archive do not. A member \
         file is written once and an archive is frozen, so a line into either is honest: \
         {found:#?}"
    );
    assert!(
        found.iter().any(|f| f.says.contains("RULES.md:163")),
        "{found:#?}"
    );
    assert!(
        found.iter().any(|f| f.says.contains("OPTIONS.md:1113")),
        "{found:#?}"
    );

    std::fs::remove_dir_all(&tmp).ok();
}

/// Naming a ledger without a line is how anybody talks about one.
#[test]
fn naming_a_ledger_without_a_line_is_left_alone() {
    let tmp = tempdir();
    std::fs::write(
        tmp.join("43_member.md"),
        "The register is `OPTIONS.md` and the droplist is `DROPLIST.md`, cited by neither line.\n",
    )
    .unwrap();
    assert!(
        corpus::line_citations_into_living_ledgers_in_prose(&tmp).is_empty(),
        "naming the ledger is not citing into it"
    );
    std::fs::remove_dir_all(&tmp).ok();
}

/// Whether the citation sits in something written once rather than something edited.
///
/// **Classified by what the file is, not by what it is called.** The first
/// version asked whether the basename began with digits, which made every
/// numbered member file the record and everything else editable. That put a
/// probe's own `FINDINGS.md`, sitting two directories down inside a landed probe
/// directory, on the repairable side, where it is exactly as much the record as
/// the member file that produced it.
///
/// The seat that reported this class caught the same shape in its own
/// classifier on the same day, matching a repository name as a substring and
/// getting the flattering answer. A classifier that partitions by name gets the
/// cases nobody thought of wrong, and it gets them wrong quietly.
fn in_a_member_file(at: &str) -> bool {
    let file = at.split(':').next().unwrap_or(at);
    // The only editable files are the ledgers, and they sit at the panel root.
    !(!file.contains('/') && corpus::LIVING_LEDGERS.contains(&file))
}

fn tempdir() -> std::path::PathBuf {
    let base = std::env::temp_dir().join(format!(
        "arvo-checks-ledger-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    std::fs::create_dir_all(&base).unwrap();
    base
}
