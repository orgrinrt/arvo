//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The two halves of a sentence in `RULES.md` that asserted a totality and had
//! checked five files.
//!
//! It read "no unprefixed archive citation remains, and no `OLD_` citation
//! names a file that does not exist". Measured across the panel when somebody
//! finally looked: 45 files, 118 unprefixed citations, two of them in the
//! intent catalogue. The five files the pass actually repointed were clean, and
//! they were the whole of what had been checked.

use std::fs;

use arvo_checks::corpus;

#[test]
#[ignore = "catalogue: OPTIONS.md:111,148,296,562 still cite `seed/SETTLED_laws.md` and \
            `seed/SETTLED_container.md`; repointing waits until the dispatch reading that \
            file has returned, because a document must not move under a member reading it. \
            INTENTS.md and PERSONA_CALLS.md are already repointed. Remove this attribute in \
            the same commit as the OPTIONS.md repoint."]
fn no_living_ledger_cites_the_archive_by_its_dead_name() {
    let found = corpus::unprefixed_archive_citations_in_living_ledgers(&corpus::panel_dir());
    assert!(
        found.is_empty(),
        "a ledger a reader treats as current cites a file the rename removed: {found:#?}"
    );
}

#[test]
fn every_archive_citation_in_the_panel_names_a_file_that_is_there() {
    let found = corpus::archive_citations_naming_nothing(&corpus::panel_dir());
    assert!(
        found.is_empty(),
        "an `OLD_` citation is written deliberately, so a dangling one is a rename nobody \
         followed: {found:#?}"
    );
}

/// The 116 that stay, counted, so the number in `RULES.md` is measured rather
/// than remembered and a later pass cannot quietly grow it.
///
/// These sit in landed numbered files, which are the record and are not edited.
/// The reading rule is one sentence: prepend `OLD_`. What this pins is that
/// nobody adds more.
#[test]
fn the_landed_files_carry_the_count_the_rule_states_and_no_more() {
    let dir = corpus::panel_dir();
    let mut total = 0usize;
    let mut files = 0usize;
    for entry in walk(&dir) {
        let Ok(text) = fs::read_to_string(&entry) else {
            continue;
        };
        let n = text.matches("seed/SETTLED_").count();
        if n > 0 {
            files += 1;
            total += n;
        }
    }
    assert!(
        total <= 116 && files <= 43,
        "{files} files carry {total} unprefixed archive citations, against the 43 and 116 \
         recorded in RULES.md after the two in INTENTS.md were repointed. A new one has been \
         written; repoint it rather than raising this number."
    );
}

/// The control. Both arms must fire on a planted tree, or the three tests above
/// are three ways of saying an empty directory has no problems.
#[test]
fn both_arms_fire_on_a_planted_tree() {
    let tmp = tempdir();
    fs::create_dir_all(tmp.join("seed")).unwrap();
    fs::write(tmp.join("seed/OLD_SETTLED_laws.md"), "# laws\n").unwrap();
    fs::write(
        tmp.join("INTENTS.md"),
        "Quoted at `seed/SETTLED_strategy.md` section 2.\n",
    )
    .unwrap();
    fs::write(
        tmp.join("42_member.md"),
        "See `seed/OLD_SETTLED_container.md:33` and `seed/OLD_SETTLED_laws.md:145`.\n",
    )
    .unwrap();

    let unprefixed = corpus::unprefixed_archive_citations_in_living_ledgers(&tmp);
    assert_eq!(unprefixed.len(), 1, "{unprefixed:#?}");
    assert!(unprefixed[0].at.starts_with("INTENTS.md:1"), "{}", unprefixed[0].at);
    assert!(
        unprefixed[0].says.contains("seed/OLD_SETTLED_strategy.md"),
        "the report names the file to write instead: {}",
        unprefixed[0].says
    );

    let dangling = corpus::archive_citations_naming_nothing(&tmp);
    assert_eq!(
        dangling.len(),
        1,
        "`OLD_SETTLED_container.md` is absent and `OLD_SETTLED_laws.md` is present, so \
         exactly one is dangling: {dangling:#?}"
    );
    assert!(dangling[0].says.contains("container"), "{}", dangling[0].says);

    fs::remove_dir_all(&tmp).ok();
}

/// A `seed/` that is part of a longer path is not an archive citation, or the
/// arm reports every mention of a research directory anywhere.
#[test]
fn a_seed_inside_a_longer_path_is_left_alone() {
    let tmp = tempdir();
    fs::create_dir_all(tmp.join("seed")).unwrap();
    fs::write(
        tmp.join("RULES.md"),
        "Sketches live at `mock/research/seed/OLD_SETTLED_laws.md` and elsewhere.\n",
    )
    .unwrap();
    assert!(
        corpus::unprefixed_archive_citations_in_living_ledgers(&tmp).is_empty(),
        "a prefixed citation is fine wherever it sits"
    );
    fs::remove_dir_all(&tmp).ok();
}

fn tempdir() -> std::path::PathBuf {
    let base = std::env::temp_dir().join(format!(
        "arvo-checks-corpus-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    fs::create_dir_all(&base).unwrap();
    base
}

fn walk(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(walk(&path));
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
    out
}
