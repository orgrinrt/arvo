//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The two halves of a sentence in `RULES.md` that asserted a totality and had
//! checked five files.
//!
//! It read "no unprefixed archive citation remains, and no `OLD_` citation
//! names a file that does not exist". The five files that pass repointed were
//! clean, and they were the whole of what had been checked. Dozens more were
//! not, two of them in the intent catalogue every dispatch reads first.
//!
//! The count is in `no_new_unprefixed_archive_citation_is_written` and nowhere
//! else, for the reason written there: the first repair put it in prose and it
//! was stale within the hour.
//!
//! **The same sentence was then asserted over one archive of two.** These arms
//! keyed on the literal `seed/`, a four-file archive, while the identical
//! rename had been applied to the closed formalization panel, 203 files at
//! another address. Seven citations into it were written in the dead spelling
//! and three of those sat in living ledgers, which is exactly the population the
//! first arm exists to protect. Nothing reported any of them, and nothing could:
//! the arm's name said "the archive" over a corpus that has two. The arms now
//! run over `corpus::ARCHIVES`, and `both_archives_are_covered_by_the_same_arms`
//! is what keeps that true when a third is added.

use std::fs;

use arvo_checks::corpus;

#[test]
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

/// The ones that stay, counted here and nowhere else.
///
/// They sit in landed files and committed probe output, which are the record
/// and are not edited. The reading rule is one sentence: prepend `OLD_`. What
/// this pins is that nobody writes a new one.
///
/// **The number lives in this test rather than in `RULES.md`.** The first
/// repair wrote it into the prose and it was stale within the hour, twice over:
/// the repair itself removed citations, and the paragraph announcing the count
/// then contained the string it was counting. A number in prose has no way to
/// know it stopped being true.
///
/// Which is why the files *about* this class are excluded below. Every one of
/// them contains `seed/SETTLED_` because it is discussing the spelling, and
/// counting those would mean the count rises whenever somebody writes about the
/// problem.
#[test]
fn no_new_unprefixed_archive_citation_is_written() {
    /// Files that carry the string because they are about it.
    const ABOUT_THE_CLASS: &[&str] = &[
        "RULES.md",
        "179_lamport_porting_ops_rulings_into_the_registry.md",
        "180_kiselyov_porting_the_options_register_and_the_droplist.md",
    ];
    /// Measured after the three living ledgers were repointed. Raising this is
    /// not the fix for a failure; repointing the new citation is.
    const CEILING: usize = 110;

    let dir = corpus::panel_dir();
    let mut total = 0usize;
    let mut worst: Vec<String> = Vec::new();
    for entry in walk(&dir) {
        let name = entry
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if ABOUT_THE_CLASS.contains(&name.as_str()) {
            continue;
        }
        let Ok(text) = fs::read_to_string(&entry) else {
            continue;
        };
        let n = text.matches("seed/SETTLED_").count();
        if n > 0 {
            total += n;
            worst.push(format!("{name}: {n}"));
        }
    }
    worst.sort();
    assert!(
        total <= CEILING,
        "{total} unprefixed archive citations against a ceiling of {CEILING}. One has been \
         written since the repair; repoint it rather than raising the number. Per file: \
         {worst:#?}"
    );
}

/// The same count for the closed formalization panel, which had no ceiling
/// because it had no arm.
///
/// **Its residue is member files and committed probe output, so it is the same
/// kind of number as the one above and gets the same treatment.** A numbered
/// member file is the record: repointing its citations would be editing history
/// to make a checker green, which the arms here decline to do for the seed
/// archive and decline to do for this one on the same ground. The three that
/// sat in living ledgers were repointed rather than counted, because a living
/// ledger is edited by definition and is the half this is a gate for.
///
/// **The counter is narrow on purpose, and the first version of it was wrong in
/// a way worth keeping written down.** It counted every mention of the archive
/// prefix that was not already `OLD_`, on the reasoning that a superset is the
/// safe direction for a ceiling. It is not: the directory itself was never
/// renamed, only the files inside it, so a sentence naming
/// `202607301300_formalization-spec-panel/` as a place is a live reference and
/// counting it as a dead spelling put the number at 15 against a real 5. A
/// ceiling inflated by legitimate prose stops measuring the thing it is named
/// for, and rises whenever somebody mentions the directory.
#[test]
fn no_new_unprefixed_closed_panel_citation_is_written() {
    /// Files that carry the string because they are about it.
    const ABOUT_THE_CLASS: &[&str] = &[
        "RULES.md",
        "179_lamport_porting_ops_rulings_into_the_registry.md",
        "180_kiselyov_porting_the_options_register_and_the_droplist.md",
        "207_mcsherry_op_material_in_the_dead_panel.md",
    ];
    /// Measured after the three living ledgers were repointed. Raising this is
    /// not the fix for a failure; repointing the new citation is.
    const CEILING: usize = 4;

    let dir = corpus::panel_dir();
    let mut total = 0usize;
    let mut worst: Vec<String> = Vec::new();
    for entry in walk(&dir) {
        let name = entry
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if ABOUT_THE_CLASS.contains(&name.as_str()) {
            continue;
        }
        let Ok(text) = fs::read_to_string(&entry) else {
            continue;
        };
        let n = dead_closed_panel_citations(&text);
        if n > 0 {
            total += n;
            worst.push(format!("{name}: {n}"));
        }
    }
    worst.sort();
    assert!(
        total <= CEILING,
        "{total} unprefixed closed-panel citations against a ceiling of {CEILING}. One has \
         been written since the repair; repoint it rather than raising the number. Per file: \
         {worst:#?}"
    );
}

/// Citations naming a file in the closed panel by its dead spelling.
///
/// A mention of the directory is not one of these, which is the whole reason
/// this is a function rather than a `matches` call.
fn dead_closed_panel_citations(text: &str) -> usize {
    const PREFIX: &str = "formalization-spec-panel/";
    text.match_indices(PREFIX)
        .filter(|(at, _)| {
            let rest = &text[at + PREFIX.len()..];
            let token: String = rest
                .chars()
                .take_while(|c| !c.is_whitespace() && *c != '`' && *c != ')' && *c != '"')
                .collect();
            token.ends_with(".md") && !token.starts_with("OLD_")
        })
        .count()
}

/// The counter reports a dead file citation and leaves a live directory
/// reference alone.
///
/// Both halves planted, because a counter that reports everything passes the
/// first half of its own control and measures nothing.
#[test]
fn the_closed_panel_counter_reads_files_and_not_the_directory() {
    assert_eq!(
        dead_closed_panel_citations(
            "the panel at `mock/research/202607301300_formalization-spec-panel/` holds it"
        ),
        0,
        "the directory was never renamed, so naming it is a live reference"
    );
    assert_eq!(
        dead_closed_panel_citations(
            "quoted at `mock/research/202607301300_formalization-spec-panel/OLD_13c_op.md`"
        ),
        0,
        "an already-prefixed citation is the repaired form"
    );
    assert_eq!(
        dead_closed_panel_citations(
            "quoted at `mock/research/202607301300_formalization-spec-panel/13c_op.md`"
        ),
        1,
        "a file named without the prefix is the dead spelling"
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
    assert!(
        unprefixed[0].at.starts_with("INTENTS.md:1"),
        "{}",
        unprefixed[0].at
    );
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
    assert!(
        dangling[0].says.contains("container"),
        "{}",
        dangling[0].says
    );

    fs::remove_dir_all(&tmp).ok();
}

/// The arms cover every archive, not the one they were written against.
///
/// **This is the arm that would have caught the defect it was written for.**
/// The two above plant only into `seed/`, so they passed for as long as the
/// second archive was invisible, and they would have kept passing. Planting the
/// identical pair of citations into each archive and requiring the same two
/// findings from each is what makes the coverage a property rather than a
/// coincidence of which archive somebody happened to test.
#[test]
fn both_archives_are_covered_by_the_same_arms() {
    let base = tempdir().join("two-archives");
    fs::remove_dir_all(&base).ok();
    let panel = base.join("panel");
    fs::create_dir_all(panel.join("seed")).unwrap();
    let closed = base.join("202607301300_formalization-spec-panel");
    fs::create_dir_all(&closed).unwrap();

    // One file present in each archive and one absent from each, so the
    // dangling arm has both a case that must fire and a case that must not.
    fs::write(panel.join("seed/OLD_SETTLED_laws.md"), "# laws\n").unwrap();
    fs::write(closed.join("OLD_143b_op.md"), "# ruling\n").unwrap();

    fs::write(
        panel.join("INTENTS.md"),
        "One at `seed/SETTLED_strategy.md` and one at \
         `mock/research/202607301300_formalization-spec-panel/143b_op.md`.\n",
    )
    .unwrap();
    fs::write(
        panel.join("42_member.md"),
        "Present: `seed/OLD_SETTLED_laws.md`. Absent: \
         `mock/research/202607301300_formalization-spec-panel/OLD_999_nothing.md`.\n",
    )
    .unwrap();

    let unprefixed = corpus::unprefixed_archive_citations_in_living_ledgers(&panel);
    assert_eq!(
        unprefixed.len(),
        2,
        "one unprefixed citation per archive, in the same ledger line: {unprefixed:#?}"
    );
    assert!(
        unprefixed
            .iter()
            .any(|f| f.says.contains("seed/OLD_SETTLED_strategy.md")),
        "the seed archive's repair is named: {unprefixed:#?}"
    );
    assert!(
        unprefixed
            .iter()
            .any(|f| f.says.contains("formalization-spec-panel/OLD_143b_op.md")),
        "the closed panel's repair is named, which is the half that was missing: \
         {unprefixed:#?}"
    );

    let dangling = corpus::archive_citations_naming_nothing(&panel);
    assert_eq!(
        dangling.len(),
        1,
        "`OLD_999_nothing.md` is absent from the closed panel and `OLD_SETTLED_laws.md` is \
         present in seed, so exactly one dangles and the present one must not be reported: \
         {dangling:#?}"
    );
    assert!(
        dangling[0].says.contains("OLD_999_nothing.md"),
        "{}",
        dangling[0].says
    );

    fs::remove_dir_all(&base).ok();
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
