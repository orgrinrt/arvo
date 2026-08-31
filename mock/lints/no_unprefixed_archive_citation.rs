//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! An archive was prefixed, so a citation written in the dead spelling resolves
//! to nothing.
//!
//! Each archive directory was renamed file by file so every dead file carries an
//! `OLD_` prefix and reads as dead at a glance. The pass that landed it repointed
//! five files and wrote into `RULES.md` that no unprefixed citation remained,
//! which was a claim over the whole tree made from a check over five files.
//! Dozens still carried one, two of them in the intent catalogue every dispatch
//! reads first, where the provenance of two of op's intents resolved to nothing.
//!
//! **Two populations, and only one of them is repairable.** A living ledger is
//! edited by definition, so a dead citation in one is a defect somebody fixes,
//! and it is refused outright. A numbered member file is the record: repointing
//! its citations would be editing history to make a checker green. What is
//! refused there is a *new* one, so that half is a ratchet against a measured
//! count, and the repair is upstream of the file, in what the next seat is
//! briefed to write.
//!
//! **Written over every archive rather than over the one it was found in.** The
//! arms it came from keyed on the literal `seed/`, one archive of four files,
//! while the identical rename had been applied to a second archive of 203. Seven
//! citations into that one were written in the dead spelling, three of them in
//! living ledgers, every one resolving to nothing, with nothing reporting any of
//! it. A law asserted over one of the shapes it covers reports clean over the
//! shape it never reached.
//!
//! **A lint rather than a tool.** A citation that resolves to nothing is a
//! refused state on both halves. The ceilings do not make it a report: they
//! grandfather a population that cannot be repaired without editing the record,
//! and every citation above them is refused.
//!
//! **No count appears in this comment.** The first repair put one in prose and
//! it was stale within the hour, twice over: the repair itself removed citations,
//! and the paragraph announcing the count then contained the string it was
//! counting. The numbers are the constants below, where the lint can read them.
use std::path::Path;

use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::panel_corpus::{
    archive_citations, finding, markdown, panel_dir, shown, ARCHIVES, LIVING_LEDGERS,
};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(NoUnprefixedArchiveCitation)
}

const NAME: &str = "no-unprefixed-archive-citation";

/// Files carrying a dead spelling because they are writing about it.
///
/// Every one of these discusses the rename, so counting them would make the
/// number rise whenever somebody documents the problem, which is the opposite of
/// what a ratchet is for.
const ABOUT_THE_CLASS: [&str; 4] = [
    "RULES.md",
    "179_lamport_porting_ops_rulings_into_the_registry.md",
    "180_kiselyov_porting_the_options_register_and_the_droplist.md",
    "207_mcsherry_op_material_in_the_dead_panel.md",
];

/// The standing residue in files that are the record, measured over the
/// committed tree.
///
/// **Lower it as files are superseded; never raise it.** A citation appearing
/// above this was written after the repair, and the fix is to repoint that one
/// rather than to move the number.
const CEILING: usize = 90;

struct NoUnprefixedArchiveCitation;
impl Lint for NoUnprefixedArchiveCitation {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for NoUnprefixedArchiveCitation {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        check(&panel_dir(ctx.mock_dir), CEILING)
    }
}

/// One dead citation, with where it sits and what should have been written.
struct Dead {
    at: String,
    line: usize,
    cited: String,
    archive: &'static str,
    repair: String,
}

/// Both halves, over one panel directory.
///
/// `ceiling` is a parameter so a planted tree can drive the ratchet with a
/// ceiling of zero. A ratchet tested only against the number it was measured
/// with is a ratchet nobody has seen fire.
fn check(dir: &Path, ceiling: usize) -> Vec<LintError> {
    let dead = dead_citations(dir);
    let (in_ledgers, in_the_record): (Vec<&Dead>, Vec<&Dead>) =
        dead.iter().partition(|d| is_a_living_ledger(&d.at));

    let mut out: Vec<LintError> = in_ledgers
        .iter()
        .map(|d| {
            finding(
                NAME,
                &d.at,
                d.line,
                format!(
                    "cites `{}`, which does not exist: the {} archive is prefixed, so the \
                     file is `{}`. This is a ledger a reader treats as current, so a citation \
                     in it that resolves to nothing is read as provenance that was checked.",
                    d.cited, d.archive, d.repair
                ),
            )
        })
        .collect();

    let counted: Vec<&&Dead> = in_the_record
        .iter()
        .filter(|d| !is_about_the_class(&d.at))
        .collect();
    if counted.len() > ceiling {
        let mut per_file: Vec<String> = counted.iter().map(|d| d.at.clone()).collect();
        per_file.sort();
        per_file.dedup();
        out.push(finding(
            NAME,
            "the panel's landed files",
            0,
            format!(
                "{} unprefixed archive citations sit in files that are the record, against a \
                 ceiling of {ceiling}. A numbered member file is written once, so repointing \
                 one would be editing history to make a checker green, and what this refuses \
                 is a new one. Repoint the citation that was just written rather than raising \
                 the number. In: {per_file:?}",
                counted.len()
            ),
        ));
    }
    out
}

/// Every citation into an archive written without the prefix the rename gave it.
fn dead_citations(dir: &Path) -> Vec<Dead> {
    let mut out = Vec::new();
    for path in markdown(dir) {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let at = shown(&path, dir);
        for (n, line) in text.lines().enumerate() {
            for archive in ARCHIVES {
                for cited in archive_citations(line, archive.prefix) {
                    let file = cited.trim_start_matches(archive.prefix);
                    if file.starts_with("OLD_") {
                        continue;
                    }
                    out.push(Dead {
                        at: at.clone(),
                        line: n + 1,
                        repair: format!("{}OLD_{file}", archive.prefix),
                        cited,
                        archive: archive.name,
                    });
                }
            }
        }
    }
    out
}

/// Whether a file is one a reader treats as current, and may therefore edit.
///
/// **Classified by what the file is, not by what it is called.** An earlier
/// version asked whether the basename began with digits, and a probe's own
/// `FINDINGS.md` two directories down inside a landed probe directory then
/// counted as editable, where it is exactly as much the record as the member
/// file that produced it. The only editable files are the ledgers and they sit
/// at the panel root.
fn is_a_living_ledger(at: &str) -> bool {
    !at.contains('/') && LIVING_LEDGERS.contains(&at)
}

fn is_about_the_class(at: &str) -> bool {
    let name = at.rsplit('/').next().unwrap_or(at);
    ABOUT_THE_CLASS.contains(&name)
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use crate::canon_lint_testkit::{
        assert_findings_block_at, assert_not_declared_off, assert_registered, ctx_at, plant,
        planted_tree, view,
    };
    use crate::panel_corpus::{panel_dir, PANEL};

    /// A panel tree with the files named, and the findings over it.
    ///
    /// The panel sits one directory down, because the closed archive is
    /// addressed as `../202607301300_formalization-spec-panel` and has to land
    /// inside the fixture rather than beside it in the shared temporary
    /// directory, where two arms would write into one archive.
    fn findings(what: &str, files: &[(&str, &str)], ceiling: usize) -> Vec<String> {
        let panel = planted_tree(what).join("panel");
        std::fs::create_dir_all(&panel).expect("a planted panel");
        for (at, text) in files {
            plant(&panel, at, text);
        }
        super::check(&panel, ceiling)
            .into_iter()
            .map(|e| e.message)
            .collect()
    }

    /// The two archives, planted with one present file each so the arms have
    /// both a case that must fire and a case that must not.
    const PRESENT: [(&str, &str); 2] = [
        ("seed/OLD_SETTLED_laws.md", "# laws\n"),
        (
            "../202607301300_formalization-spec-panel/OLD_143b_op.md",
            "# ruling\n",
        ),
    ];

    #[test]
    fn a_ledger_citing_the_dead_spelling_is_reported_with_the_repair() {
        let mut files = PRESENT.to_vec();
        files.push((
            "INTENTS.md",
            "Quoted at `seed/SETTLED_strategy.md` section 2.\n",
        ));
        let f = findings("archive-ledger", &files, usize::MAX);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("seed/SETTLED_strategy.md"), "{}", f[0]);
        assert!(
            f[0].contains("seed/OLD_SETTLED_strategy.md"),
            "the report names the file to write instead: {}",
            f[0]
        );
    }

    #[test]
    fn both_archives_are_covered_by_the_same_arm() {
        // **The arm that would have caught the defect it was written for.** A
        // fixture planting only into `seed/` passes for as long as the second
        // archive is invisible, and keeps passing. Planting the identical
        // citation into each and requiring a finding from each is what makes the
        // coverage a property rather than a coincidence of which archive
        // somebody happened to test.
        let mut files = PRESENT.to_vec();
        files.push((
            "INTENTS.md",
            "One at `seed/SETTLED_strategy.md` and one at \
             `mock/research/202607301300_formalization-spec-panel/143b_op.md`.\n",
        ));
        let f = findings("archive-both", &files, usize::MAX);
        assert_eq!(
            f.len(),
            2,
            "one per archive, in the same ledger line: {f:?}"
        );
        assert!(
            f.iter().any(|m| m.contains("seed/OLD_SETTLED_strategy.md")),
            "{f:?}"
        );
        assert!(
            f.iter()
                .any(|m| m.contains("formalization-spec-panel/OLD_143b_op.md")),
            "the closed panel's repair is named, which is the half that was missing: {f:?}"
        );
    }

    #[test]
    fn control_a_prefixed_citation_is_the_repaired_form_and_is_silent() {
        let mut files = PRESENT.to_vec();
        files.push((
            "INTENTS.md",
            "Quoted at `seed/OLD_SETTLED_laws.md` section 2.\n",
        ));
        assert!(findings("archive-repaired", &files, usize::MAX).is_empty());
    }

    #[test]
    fn control_a_prefix_inside_a_longer_path_is_somebody_elses() {
        // Without this the arm reports every mention of a research directory
        // anywhere, and every arm above still passes.
        let mut files = PRESENT.to_vec();
        files.push((
            "RULES.md",
            "Sketches live at `mock/research/seed/SETTLED_laws.md` and elsewhere.\n",
        ));
        assert!(findings("archive-longer-path", &files, usize::MAX).is_empty());
    }

    #[test]
    fn a_member_file_is_counted_rather_than_refused_and_the_ratchet_fires_above_the_ceiling() {
        // The two halves in one fixture. The citation sits in a file that is the
        // record, so it is not refused on its own; it is refused because the
        // population exceeded what was measured.
        let mut files = PRESENT.to_vec();
        files.push(("42_member.md", "See `seed/SETTLED_container.md` for it.\n"));
        assert!(
            findings("archive-under", &files, 1).is_empty(),
            "one citation under a ceiling of one is the grandfathered population"
        );
        let f = findings("archive-over", &files, 0);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("ceiling of 0"), "{}", f[0]);
        assert!(f[0].contains("42_member.md"), "{}", f[0]);
    }

    #[test]
    fn control_a_file_writing_about_the_dead_spelling_is_not_counted() {
        // Every one of those files carries the string because it is discussing
        // it, so counting them makes the number rise whenever somebody documents
        // the problem.
        let mut files = PRESENT.to_vec();
        files.push((
            "179_lamport_porting_ops_rulings_into_the_registry.md",
            "The dead spelling is `seed/SETTLED_laws.md` and the live one is prefixed.\n",
        ));
        assert!(findings("archive-about", &files, 0).is_empty());
    }

    #[test]
    fn a_ledger_citation_is_refused_whatever_the_ceiling_says() {
        // The discrimination between the two halves. A ledger is editable, so
        // its citations are never grandfathered, and a lint that ran everything
        // through the ratchet would report nothing here.
        let mut files = PRESENT.to_vec();
        files.push(("OPTIONS.md", "At `seed/SETTLED_strategy.md`.\n"));
        let f = findings("archive-ledger-not-grandfathered", &files, usize::MAX);
        assert_eq!(f.len(), 1, "{f:?}");
    }

    #[test]
    fn a_panel_directory_that_is_not_there_is_silent_rather_than_a_panic() {
        let dir = planted_tree("archive-absent");
        assert!(super::check(&dir.join("nothing"), 0).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let dir = planted_tree("archive-severity");
        plant(&dir, "mock/registry/keep.toml", "");
        plant(
            &dir,
            &format!("mock/{PANEL}/INTENTS.md"),
            "At `seed/SETTLED_strategy.md`.\n",
        );
        let empty = view(&[], &[]);
        let mock = dir.join("mock");
        assert!(
            panel_dir(&mock).is_dir(),
            "the fixture is where the lint looks"
        );
        assert_findings_block_at(&super::NoUnprefixedArchiveCitation, &ctx_at(&mock, &empty));
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::NoUnprefixedArchiveCitation);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(super::NoUnprefixedArchiveCitation.name(), super::NAME);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered(super::NAME);
    }
}
