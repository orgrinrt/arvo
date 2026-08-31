//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The registry is the most edited tree here and nobody was watching it.
//!
//! The panel's living ledgers were the obvious moving target and a check
//! refuses line citations into those. The registry is worse: it is the working
//! surface, every seat writes rows into it, and a row inserted anywhere shifts
//! every line below it in that file.
//!
//! It has already happened, and the seat that caused it reported it against
//! itself: a member file cites `proposal.toml:211` and four more, a later pass
//! inserted fourteen lines into that file, and those citations now name
//! different rows. **The reader lands on a real row**, which is what makes this
//! worse than a citation that fails to resolve.
//!
//! A row has a slug and the slug survives every insertion. That is the repair,
//! and it is why this is a catalogue rather than a permanent allowance.

use std::fs;

use arvo_checks::corpus;

/// A ledger carries none, and a ledger is a file anybody may edit.
///
/// **This is the half that is repairable and therefore the half that is a
/// gate.** A living ledger is rewritten as the panel moves, so a line citation
/// in one is a defect somebody fixes rather than a fact about the record.
#[test]
fn no_ledger_cites_the_registry_by_line() {
    let found: Vec<_> = corpus::line_citations_into_the_registry(&corpus::panel_dir())
        .into_iter()
        .filter(|f| !in_a_member_file(&f.at))
        .collect();
    assert!(
        found.is_empty(),
        "a file that gets edited cites a registry file by line. Name the row's slug: {found:#?}"
    );
}

/// The member files carry a ceiling, because a member file is not edited.
///
/// **The first version of this test capped both together and was wrong about
/// what a ceiling is for.** It read as pinning the corpus while the number was
/// still moving: a seat working on a branch cut before this check landed added
/// thirteen, none of which is repairable, and the ceiling then reported a
/// regression where there was only a merge.
///
/// A numbered member file is the record. Repointing its citations would be
/// editing history to make a checker green, which is exactly what the archive
/// arms in this module decline to do for the same reason. So what this measures
/// is the standing cost, and **the repair is upstream of the file**: a seat
/// briefed to name slugs writes none of these, and the number stops growing
/// because nobody adds to it rather than because anybody cleaned it.
#[test]
fn the_member_files_line_citations_do_not_grow() {
    /// Measured over the committed tree. Every one is unrepairable by
    /// construction; the number falls only when a file is superseded.
    const CEILING: usize = 45;

    let found: Vec<_> = corpus::line_citations_into_the_registry(&corpus::panel_dir())
        .into_iter()
        .filter(|f| in_a_member_file(&f.at))
        .collect();
    assert!(
        found.len() <= CEILING,
        "{} line citations into a registry file from member files, against a ceiling of \
         {CEILING}. A registry file gains rows constantly and every insertion moves the lines \
         under it, so the citation still resolves and names a different row. Brief the next \
         seat to write slugs; do not raise this: {found:#?}",
        found.len()
    );
}

/// Whether the citation sits in something written once rather than something edited.
///
/// **Classified by what the file is, not by what it is called.** The first
/// version asked whether the basename began with digits, and a probe's own
/// `FINDINGS.md` two directories down inside a landed probe directory then
/// counted as editable, where it is exactly as much the record as the member
/// file that produced it. The only editable files are the ledgers and they sit
/// at the panel root.
fn in_a_member_file(at: &str) -> bool {
    let file = at.split(':').next().unwrap_or(at);
    !(!file.contains('/') && corpus::LIVING_LEDGERS.contains(&file))
}

/// The control, and both directions.
///
/// A slug citation must pass or the arm is a ban on mentioning the registry at
/// all, and a line citation into an unrelated `.toml` must pass or it reports
/// every manifest anybody names.
#[test]
fn a_line_is_reported_and_a_slug_is_not() {
    let tmp = tempdir();
    fs::write(
        tmp.join("42_member.md"),
        "See `proposal.toml:211` and `proposal-the-later-topics.toml:454`.\n\
         The row is `proposal::composition_contracts_above_the_numeral`.\n\
         Built from `Cargo.toml:17` and `mockspace.toml:311`.\n",
    )
    .unwrap();

    let found = corpus::line_citations_into_the_registry(&tmp);
    assert_eq!(
        found.len(),
        2,
        "both registry citations are reported, the slug is not, and neither a manifest nor \
         the project config is a registry file: {found:#?}"
    );
    assert!(
        found.iter().any(|f| f.says.contains("proposal.toml:211")),
        "{found:#?}"
    );
    assert!(
        found
            .iter()
            .any(|f| f.says.contains("proposal-the-later-topics.toml:454")),
        "a registry file filed by subject carries a hyphenated name and is still a registry \
         file: {found:#?}"
    );

    fs::remove_dir_all(&tmp).ok();
}

/// A `.toml` mention with no line is not a citation, so the arm must not fire
/// on prose naming a file.
#[test]
fn naming_a_registry_file_without_a_line_is_left_alone() {
    let tmp = tempdir();
    fs::write(
        tmp.join("43_member.md"),
        "Every row lives in `proposal.toml` or `ruling.toml`, and neither is cited by line.\n",
    )
    .unwrap();
    assert!(
        corpus::line_citations_into_the_registry(&tmp).is_empty(),
        "naming the file is how anybody talks about it"
    );
    fs::remove_dir_all(&tmp).ok();
}

fn tempdir() -> std::path::PathBuf {
    let base = std::env::temp_dir().join(format!(
        "arvo-checks-reg-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    fs::create_dir_all(&base).unwrap();
    base
}
