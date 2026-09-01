//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A registry slug named in the panel's prose that resolves to no row.
//!
//! The registry's own edges are checked and clean: a typed field naming a row
//! that is not there is refused, over all 697 rows. Prose is where the same
//! citation is written far more often and nothing was reading it, so a slug in
//! a sentence could name a row that never existed, or a row whose id it very
//! nearly has, and the sentence keeps reading as a citation either way.
//!
//! **The near miss is the one worth the lint.** A slug that names nothing
//! obvious gets caught eventually by a reader who goes looking. A slug missing
//! one trailing word off a real id does not, because the reader recognises the
//! row it almost names and supplies the rest, which is the same failure a line
//! citation into a moved file produces and by the same mechanism.
//!
//! **Two populations**, on the split the two ledger-citation lints already use
//! and for the same reason. A ledger is edited as the panel moves, so a
//! dangling citation in one is repaired and this refuses it outright. A member
//! file is written once and is the record, so its citations are counted rather
//! than rewritten: repointing one would be editing history to make a checker
//! green. The repair for that population is upstream, in the brief, and a seat
//! that queries the registry before citing it writes none of these.
//!
//! # What the standing population is, measured
//!
//! Sixteen occurrences over fourteen distinct slugs, all fourteen in numbered
//! member files and none in a ledger, so the ledger arm starts at zero and
//! refuses the first one written.
//!
//! **Seven are deliberate**, planted by seats testing citation checkers of
//! their own: `control_broken_file`, `control_broken_topic`,
//! `control_row_planted_by_189_delete_me`, `a_question_that_does_not_exist`,
//! `no_such_topic_exists`, and the two spelled `x`. They are counted rather
//! than special-cased, because a rule that skips anything containing `control`
//! is a rule an ordinary citation can be written past, and because the ceiling
//! is a statement about what the corpus holds rather than about what deserves
//! to be there.
//!
//! **Seven are real, and they are two kinds.** Two are truncations of a slug
//! that does exist: `ruling::the_warrant_is_a_token_and_a_clause_on_the_values`
//! is missing the `_side` its row carries, which the corpus otherwise spells in
//! full, and `proposal::a_min_plus_fold_needs_an_absorbing_top_` still
//! carries the underscore that got cut. Five name no row at all and are
//! paraphrases of a claim rather than citations of it.
//!
//! # A trailing underscore is kept rather than trimmed
//!
//! The token runs to the first character that cannot be in a slug, so
//! `..._top_` is read with its underscore and does not resolve. Trimming it
//! would make that citation resolve to a different row and the defect would
//! disappear into a passing check, which is the failure this lint is about.
use std::path::Path;

use mockspace::{Lint, LintError, RegistryView, RepoContext, RepoLint, Severity};

use crate::panel_corpus::{finding, markdown, panel_dir, shown, LIVING_LEDGERS};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(NoProseCitationIntoNothing)
}

const NAME: &str = "no-prose-citation-into-nothing";

/// The standing count in files that are the record, measured over the committed
/// tree.
///
/// Sixteen occurrences over fourteen slugs when the class was first measured.
/// It falls when a slug is declared under the id the prose already used. **Do
/// not raise it.**
const CEILING: usize = 16;

struct NoProseCitationIntoNothing;
impl Lint for NoProseCitationIntoNothing {
    fn name(&self) -> &'static str {
        NAME
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for NoProseCitationIntoNothing {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        check(&panel_dir(ctx.mock_dir), ctx.registry, CEILING)
    }
}

fn check(dir: &Path, reg: &RegistryView, ceiling: usize) -> Vec<LintError> {
    let namespaces: Vec<&str> = reg.namespaces().collect();
    let found = citations(dir, &namespaces, reg);
    let (in_ledgers, in_the_record): (
        Vec<&(String, usize, String)>,
        Vec<&(String, usize, String)>,
    ) = found.iter().partition(|(at, _, _)| is_a_living_ledger(at));

    let mut out: Vec<LintError> = in_ledgers
        .iter()
        .map(|(at, line, cited)| {
            finding(
                NAME,
                at,
                *line,
                format!(
                    "cites `{cited}`, which resolves to no row. This file is edited, so fix \
                     the citation: query the registry for the id it meant, and mind that a \
                     slug one word short of a real one reads as a citation and is not. Where \
                     no row says it, the sentence is a paraphrase and cites nothing."
                ),
            )
        })
        .collect();

    if in_the_record.len() > ceiling {
        let mut slugs: Vec<String> = in_the_record.iter().map(|(_, _, s)| s.clone()).collect();
        slugs.sort();
        slugs.dedup();
        let mut per_file: Vec<String> = in_the_record.iter().map(|(at, _, _)| at.clone()).collect();
        per_file.sort();
        per_file.dedup();
        out.push(finding(
            NAME,
            "the panel's landed files",
            0,
            format!(
                "{} prose citations resolve to no row, against a ceiling of {ceiling}. \
                 Rewriting one would be editing a member file, which is the record, so what \
                 this refuses is a new one. Brief the next seat to query the registry before \
                 citing it; do not raise the number. {} distinct slug(s) across {} file(s): \
                 {slugs:?}",
                in_the_record.len(),
                slugs.len(),
                per_file.len()
            ),
        ));
    }
    out
}

/// Every `<namespace>::<slug>` the panel's prose carries that names no row.
fn citations(dir: &Path, namespaces: &[&str], reg: &RegistryView) -> Vec<(String, usize, String)> {
    let mut out = Vec::new();
    for path in markdown(dir) {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let at = shown(&path, dir);
        for (n, line) in text.lines().enumerate() {
            for cited in tokens(line, namespaces) {
                if reg.row(&cited).is_none() {
                    out.push((at.clone(), n + 1, cited));
                }
            }
        }
    }
    out
}

/// Every qualified slug on one line, read to its own boundaries.
///
/// A token starts where a namespace is preceded by something that cannot be in
/// a slug, and runs to the first character that cannot be either. Both halves
/// are load-bearing: without the left boundary
/// `arvo_proposal::a_row` reports the wrong namespace, and without the right
/// one a truncated slug is read as the longer row it is missing a word from.
fn tokens(line: &str, namespaces: &[&str]) -> Vec<String> {
    let bytes = line.as_bytes();
    let mut out = Vec::new();
    for ns in namespaces {
        let mut from = 0;
        while let Some(hit) = line[from..].find(&format!("{ns}::")) {
            let start = from + hit;
            from = start + ns.len() + 2;
            if start > 0 && is_slug_byte(bytes[start - 1]) {
                continue;
            }
            let rest = &line[from..];
            let end = rest
                .find(|c: char| !is_slug_byte(c as u8) || !c.is_ascii())
                .unwrap_or(rest.len());
            if end == 0 {
                continue;
            }
            out.push(format!("{ns}::{}", &rest[..end]));
        }
    }
    out
}

fn is_slug_byte(b: u8) -> bool {
    b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_'
}

/// Whether a file is one a reader treats as current, and may therefore edit.
fn is_a_living_ledger(at: &str) -> bool {
    !at.contains('/') && LIVING_LEDGERS.contains(&at)
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use crate::canon_lint_testkit::{
        assert_findings_block_at, assert_not_declared_off, assert_registered, ctx_at, plant,
        planted_tree, view,
    };
    use crate::panel_corpus::PANEL;

    /// One planted registry read against one planted panel tree.
    fn findings(
        what: &str,
        rows: &[(&str, &[(&str, &str)])],
        files: &[(&str, &str)],
        ceiling: usize,
    ) -> Vec<String> {
        let dir = planted_tree(what);
        for (at, text) in files {
            plant(&dir, at, text);
        }
        let reg = view(rows, &[]);
        super::check(&dir, &reg, ceiling)
            .into_iter()
            .map(|e| e.message)
            .collect()
    }

    const ONE_ROW: &[(&str, &[(&str, &str)])] = &[(
        "ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side",
        &[("says", "a thing")],
    )];

    #[test]
    fn a_resolving_citation_is_silent_and_a_dangling_one_is_named() {
        let f = findings(
            "prose-cite-mix",
            ONE_ROW,
            &[(
                "RULES.md",
                "The row is `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`.\n\
                 And `ruling::a_row_that_was_never_written` is not.\n",
            )],
            usize::MAX,
        );
        assert_eq!(f.len(), 1, "only the dangling one fires: {f:?}");
        assert!(f[0].contains("a_row_that_was_never_written"), "{}", f[0]);
    }

    #[test]
    fn a_truncation_of_a_real_slug_is_named_rather_than_read_as_the_row() {
        // The case the lint exists for, and the one a right boundary that
        // stopped at the shorter match would hide. The truncation is a prefix
        // of a row that does exist, so a `contains`-shaped or prefix-shaped
        // check reports it resolving.
        let f = findings(
            "prose-cite-truncated",
            ONE_ROW,
            &[(
                "42_member.md",
                "See `ruling::the_warrant_is_a_token_and_a_clause_on_the_values`.\n",
            )],
            0,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("ceiling of 0"), "{}", f[0]);
        assert!(
            !f[0].contains("_values_side"),
            "the truncation was reported as the row it is missing a word from: {}",
            f[0]
        );
    }

    #[test]
    fn a_trailing_underscore_is_kept_so_the_citation_does_not_resolve() {
        // Trimming it would make this resolve to a different row and the
        // defect would vanish into a passing check.
        let f = findings(
            "prose-cite-underscore",
            &[(
                "proposal::a_min_plus_fold_needs_an_absorbing_top",
                &[("says", "a thing")],
            )],
            &[(
                "42_member.md",
                "See `proposal::a_min_plus_fold_needs_an_absorbing_top_`.\n",
            )],
            0,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("absorbing_top_"), "{}", f[0]);
    }

    #[test]
    fn a_ledger_is_refused_outright_and_a_member_file_is_counted() {
        // The namespace has to hold a row for it to be a namespace at all, per
        // the control below, so the fixture declares one and cites another.
        let files = [("RULES.md", "See `ruling::nothing_here`.\n")];
        let f = findings("prose-cite-ledger", ONE_ROW, &files, usize::MAX);
        assert_eq!(f.len(), 1, "a ledger fires whatever the ceiling is: {f:?}");
        assert!(f[0].contains("This file is edited"), "{}", f[0]);

        let member = [("42_member.md", "See `ruling::nothing_here`.\n")];
        assert!(
            findings("prose-cite-member-under", ONE_ROW, &member, 1).is_empty(),
            "one citation under a ceiling of one is the grandfathered population"
        );
        let over = findings("prose-cite-member-over", ONE_ROW, &member, 0);
        assert_eq!(over.len(), 1, "{over:?}");
        assert!(over[0].contains("is the record"), "{}", over[0]);
    }

    #[test]
    fn a_namespace_the_registry_does_not_have_is_not_a_citation() {
        // The predicate reads the namespaces the registry actually holds, so a
        // rust path is not a citation and neither is a namespace nobody
        // declared. A check written over a hardcoded list gets this wrong the
        // first time a namespace is added or renamed.
        assert!(
            findings(
                "prose-cite-foreign-ns",
                &[("ruling::a_row", &[("says", "a thing")])],
                &[(
                    "42_member.md",
                    "`std::fs` and `core::mem` and `alloc::vec` are paths, and \
                     `notanamespace::a_slug` is not a row.\n",
                )],
                0,
            )
            .is_empty(),
            "something other than a registry namespace was read as a citation"
        );
    }

    #[test]
    fn a_slug_whose_namespace_is_a_suffix_of_a_longer_word_is_not_a_citation() {
        // The left boundary. Without it `arvo_ruling::x` reports a citation in
        // the `ruling` namespace, which is a different token entirely.
        assert!(
            findings(
                "prose-cite-left-boundary",
                &[("ruling::a_row", &[("says", "a thing")])],
                &[(
                    "42_member.md",
                    "The symbol `arvo_ruling::a_row_that_is_gone` is code.\n"
                )],
                0,
            )
            .is_empty(),
            "a longer word ending in a namespace was read as one"
        );
    }

    #[test]
    fn every_dangling_citation_is_named_rather_than_the_first() {
        let f = findings(
            "prose-cite-all",
            &[
                ("ruling::a_row", &[("says", "a thing")]),
                ("proposal::a_row", &[("says", "a thing")]),
            ],
            &[(
                "42_member.md",
                "`ruling::one_gone` and `proposal::two_gone` and `ruling::one_gone` again.\n",
            )],
            0,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("one_gone"), "{}", f[0]);
        assert!(f[0].contains("two_gone"), "{}", f[0]);
        assert!(
            f[0].contains("3 prose citations"),
            "the occurrence count is not the distinct count: {}",
            f[0]
        );
        assert!(
            f[0].contains("2 distinct"),
            "the distinct count is missing: {}",
            f[0]
        );
    }

    #[test]
    fn control_a_registry_with_no_namespaces_finds_nothing_at_all() {
        // The vacuous case, stated so the arms above are read as real. With no
        // namespaces there is no token shape to match, so every arm passes for
        // a reason that has nothing to do with the prose.
        assert!(
            findings(
                "prose-cite-empty-reg",
                &[],
                &[("42_member.md", "See `ruling::nothing_here`.\n")],
                0,
            )
            .is_empty(),
            "a registry holding no rows declares no namespaces, so nothing is a citation"
        );
    }

    #[test]
    fn control_a_line_with_no_citation_is_left_alone() {
        assert!(
            findings(
                "prose-cite-none",
                &[("ruling::a_row", &[("says", "a thing")])],
                &[(
                    "42_member.md",
                    "The ruling says a thing, and the proposal disagrees, both unqualified.\n",
                )],
                0,
            )
            .is_empty(),
            "prose naming a namespace without a slug is not a citation"
        );
    }

    #[test]
    fn a_panel_directory_that_is_not_there_is_silent_rather_than_a_panic() {
        let dir = planted_tree("prose-cite-absent");
        let reg = view(&[("ruling::a_row", &[("says", "a thing")])], &[]);
        assert!(super::check(&dir.join("nothing"), &reg, 0).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let dir = planted_tree("prose-cite-severity");
        plant(
            &dir,
            &format!("mock/{PANEL}/RULES.md"),
            "See `ruling::nothing_here`.\n",
        );
        let reg = view(&[("ruling::a_row", &[("says", "a thing")])], &[]);
        assert_findings_block_at(
            &super::NoProseCitationIntoNothing,
            &ctx_at(&dir.join("mock"), &reg),
        );
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::NoProseCitationIntoNothing);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(super::NoProseCitationIntoNothing.name(), super::NAME);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered(super::NAME);
    }
}
