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

/// The citations that already exist, pinned by count.
///
/// They sit in landed member files, which are the record and are not edited, so
/// the repair is a later pass rewriting them as slugs rather than an edit here.
/// What this pins is that nobody writes a new one.
#[test]
fn no_new_line_citation_into_the_registry_is_written() {
    /// Measured over the committed tree. Lower it as citations are rewritten
    /// as slugs; never raise it.
    const CEILING: usize = 32;

    let found = corpus::line_citations_into_the_registry(&corpus::panel_dir());
    assert!(
        found.len() <= CEILING,
        "{} line citations into a registry file, against a ceiling of {CEILING}. A registry \
         file gains rows constantly and every insertion moves the lines under it, so name the \
         row's slug rather than raising this number: {found:#?}",
        found.len()
    );
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
