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
//! **Seven are real, and they are three kinds.**
//!
//! **Three stop part way through a slug that does exist**, so each is a strict
//! prefix of a real id: `ruling::the_warrant_is_a_token_and_a_clause_on_the_values`
//! against a row ending `_side`, `proposal::a_min_plus_fold_needs_an_absorbing_top_`
//! against one ending `_and_wrapping_supplies_none`, and
//! `proposal::the_multiplicative_guard` against one ending
//! `_grows_linearly_and_the_saving_is_adaptation_fusion`. The third reads as a
//! clean concept name rather than as a cut-off one, which is why it was filed
//! among the paraphrases until somebody measured it against the ids.
//!
//! **Two name a slug that exists under a different namespace.**
//! `proposal::staged_narrowing_depends_on_its_staging` is a `probe` row, and
//! `ruling::observability_is_relative_to_a_declared_signature` is a `proposal`
//! row, cited correctly elsewhere in the corpus as one. The slug is not the
//! defect in either; the namespace in front of it is.
//!
//! **Two name nothing anywhere** and are paraphrases of a claim rather than
//! citations of it:
//! `proposal::a_strategys_weighting_is_rationalisable_from_the_arms_it_selects`
//! and `retirement::dl_the_associativity_gate_on_the_algorithm_crates`, whose
//! `dl_` is the spelling a retirement id carries, against no such row.
//!
//! # Two things a line-at-a-time scanner reads as a near miss and are not
//!
//! A citation is not always confined to one line and is not always written in
//! full, and reading a line at a time turns both into dangling slugs. Neither
//! misleads a reader, so neither is the failure above, and both arrived in the
//! corpus and put the count over its ceiling with nothing wrong in either file.
//!
//! **A slug wrapped through a line break is one citation.** The ids here run to
//! sixty characters and a markdown file wraps at a column, so the break lands
//! inside the slug and the first half names no row. The whole id is in the
//! file. So a token that ends a line is joined with the slug characters the
//! next line opens with, and where the join is a row the citation resolves.
//! **A truncation that happens to end a line is still reported**, because the
//! join then names nothing either.
//!
//! **An elided citation says it is elided.** `proposal::a_law_is_inherited...`
//! is the writer telling the reader the rest is cut, which is the opposite of
//! a slug one word short reading as whole. It resolves where the written part
//! is a prefix of exactly one row, at a word boundary. **Two candidates is not
//! a resolution**: the reader cannot supply the rest either, so that is the
//! near miss again and is reported. So is a prefix matching no row, and so is
//! an elision stopping mid-word.
//!
//! # A trailing underscore is kept rather than trimmed
//!
//! The token runs to the first character that cannot be in a slug, so
//! `..._top_` is read with its underscore and does not resolve. Trimming it
//! would not resolve either, since the trimmed form is no row, so nothing is
//! lost by keeping it and something is given up by cutting it: a scanner that
//! drops a trailing separator has guessed at what a writer meant, and the guess
//! is wrong wherever a slug legitimately continues past that point. A defect
//! that disappears into a passing check is the failure this lint is about.
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
///
/// A citation is not always confined to one line and is not always written in
/// full, and the two shapes that follow from that are read here rather than in
/// `tokens`, because both need something the line itself does not carry: the
/// next line, or the registry.
fn citations(dir: &Path, namespaces: &[&str], reg: &RegistryView) -> Vec<(String, usize, String)> {
    let mut out = Vec::new();
    for path in markdown(dir) {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let at = shown(&path, dir);
        let lines: Vec<&str> = text.lines().collect();
        for (n, line) in lines.iter().enumerate() {
            for cited in tokens(line, namespaces) {
                if reg.row(&cited.qualified).is_some() {
                    continue;
                }
                let resolved = match cited.tail {
                    Tail::Bounded => false,
                    Tail::RanToEndOfLine => {
                        let next = lines.get(n + 1).copied().unwrap_or("");
                        let joined = format!("{}{}", cited.qualified, leading_slug_run(next));
                        reg.row(&joined).is_some()
                    }
                    Tail::Elided => elision_resolves(&cited.qualified, reg),
                };
                if !resolved {
                    out.push((at.clone(), n + 1, cited.qualified));
                }
            }
        }
    }
    out
}

/// How a citation ended, which decides what it takes to resolve it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tail {
    /// It stopped at a character that cannot be in a slug, so it is the whole
    /// of what the writer wrote and it stands or falls on its own.
    Bounded,
    /// It ran to the end of the line, so the line break may be sitting inside
    /// it. A markdown file wraps at a column and a slug here is long enough to
    /// be wrapped through.
    RanToEndOfLine,
    /// It stopped at an ellipsis, which is the writer saying the rest is cut.
    Elided,
}

/// One qualified slug and how it ended.
#[derive(Debug, Clone)]
struct Cited {
    qualified: String,
    tail: Tail,
}

/// The slug characters a continuation line opens with, if any.
fn leading_slug_run(line: &str) -> &str {
    let end = line
        .find(|c: char| !is_slug_byte(c as u8) || !c.is_ascii())
        .unwrap_or(line.len());
    &line[..end]
}

/// Whether an elided citation names exactly one row and so cites it.
///
/// The elision has to land on a word boundary and has to leave one candidate.
/// Several candidates is not a resolution: a reader cannot supply the rest
/// either, which is the same failure as a truncation and is reported as one.
fn elision_resolves(qualified: &str, reg: &RegistryView) -> bool {
    let Some((ns, _)) = qualified.split_once("::") else {
        return false;
    };
    let prefix = format!("{qualified}_");
    let rows = reg.rows_in(ns);
    let mut hits = rows.iter().filter(|q| q.starts_with(&prefix));
    hits.next().is_some() && hits.next().is_none()
}

/// Every qualified slug on one line, read to its own boundaries.
///
/// A token starts where a namespace is preceded by something that cannot be in
/// a slug, and runs to the first character that cannot be either. Both halves
/// are load-bearing: without the left boundary
/// `arvo_proposal::a_row` reports the wrong namespace, and without the right
/// one a truncated slug is read as the longer row it is missing a word from.
fn tokens(line: &str, namespaces: &[&str]) -> Vec<Cited> {
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
            let after = &rest[end..];
            let tail = if after.is_empty() {
                Tail::RanToEndOfLine
            } else if after.starts_with("...") || after.starts_with('\u{2026}') {
                Tail::Elided
            } else {
                Tail::Bounded
            };
            out.push(Cited {
                qualified: format!("{ns}::{}", &rest[..end]),
                tail,
            });
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

    /// The row the wrap and elision arms are read against.
    ///
    /// Spelled long on purpose: a slug this length is one a markdown file wraps
    /// through, which is the whole of why the wrap arm exists.
    const LONG_ROW: &[(&str, &[(&str, &str)])] = &[(
        "proposal::a_law_is_inherited_where_the_realisation_map_is_a_congruence",
        &[("says", "a thing")],
    )];

    #[test]
    fn a_slug_wrapped_across_a_line_break_is_one_citation_rather_than_two_halves() {
        // The file holds the whole id and the line break sits inside it, so the
        // reader is not misled and there is nothing to report. Before this the
        // scanner read a line at a time and the first half dangled.
        let f = findings(
            "prose-cite-wrapped",
            LONG_ROW,
            &[(
                "42_member.md",
                "and `proposal::a_law_is_inherited_where_the_realisation_map_is_a\n\
                 _congruence` is a row in the canon.\n",
            )],
            0,
        );
        assert!(f.is_empty(), "the wrapped citation was reported: {f:?}");
    }

    #[test]
    fn a_truncation_at_the_end_of_a_line_is_still_named() {
        // The control for the arm above, and the case that would be lost to it.
        // The token ends the line and the next line does not continue it, so
        // joining produces no row and the truncation is reported as before.
        let f = findings(
            "prose-cite-wrapped-control",
            LONG_ROW,
            &[(
                "42_member.md",
                "and `proposal::a_law_is_inherited_where_the_realisation_map_is_a\n\
                 which is a row in the canon.\n",
            )],
            0,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("realisation_map_is_a"), "{}", f[0]);
    }

    #[test]
    fn an_elided_citation_naming_exactly_one_row_resolves() {
        // The writer wrote the ellipsis, so the reader is told the rest is cut
        // and cannot mistake the short form for a whole id. That is the
        // opposite of the near miss this lint exists for.
        let f = findings(
            "prose-cite-elided",
            LONG_ROW,
            &[(
                "42_member.md",
                "`proposal::a_law_is_inherited...` already states that.\n",
            )],
            0,
        );
        assert!(f.is_empty(), "the elided citation was reported: {f:?}");
    }

    #[test]
    fn an_elided_citation_naming_two_rows_is_named() {
        // The control that keeps the arm above a resolution rather than a
        // waiver. Where the elision leaves two candidates a reader cannot
        // supply the rest either, which is the failure the lint is about.
        let rows: &[(&str, &[(&str, &str)])] = &[
            (
                "proposal::a_law_is_inherited_where_the_realisation_map_is_a_congruence",
                &[("says", "a thing")],
            ),
            (
                "proposal::a_law_is_inherited_where_the_nesting_is_flat",
                &[("says", "another thing")],
            ),
        ];
        let f = findings(
            "prose-cite-elided-ambiguous",
            rows,
            &[(
                "42_member.md",
                "`proposal::a_law_is_inherited...` says so.\n",
            )],
            0,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("a_law_is_inherited"), "{}", f[0]);
    }

    #[test]
    fn an_elided_citation_naming_no_row_is_named() {
        // The other control. An ellipsis is not a licence: where nothing starts
        // with the written prefix the sentence still cites nothing.
        let f = findings(
            "prose-cite-elided-nothing",
            LONG_ROW,
            &[(
                "42_member.md",
                "`proposal::a_claim_nobody_wrote...` says so.\n",
            )],
            0,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("a_claim_nobody_wrote"), "{}", f[0]);
    }

    #[test]
    fn an_elision_that_stops_mid_word_does_not_resolve() {
        // The elision has to land on a word boundary. Stopping inside a word
        // leaves a prefix a reader completes by guessing, and guessing is what
        // the trailing-underscore case below already refuses.
        let f = findings(
            "prose-cite-elided-midword",
            LONG_ROW,
            &[("42_member.md", "`proposal::a_law_is_inherit...` says so.\n")],
            0,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("a_law_is_inherit"), "{}", f[0]);
    }

    #[test]
    fn a_trailing_underscore_is_kept_so_the_citation_does_not_resolve() {
        // The fixture declares the trimmed form as a real row, which the corpus
        // does not, so this is the hardest version of the case: even where
        // trimming would resolve, the lint does not trim, and the defect does
        // not vanish into a passing check.
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
    fn a_real_slug_under_the_wrong_namespace_is_named_like_any_other() {
        // Two of the corpus's seven are this and neither is a truncation or a
        // paraphrase: the slug is spelled correctly and sits under a different
        // namespace, so a check that matched on the slug alone would report
        // both resolving. Both namespaces hold a row, so nothing here is caught
        // by the undeclared-namespace arm below.
        let rows: &[(&str, &[(&str, &str)])] = &[
            (
                "probe::staged_narrowing_depends_on_its_staging",
                &[("establishes", "a thing")],
            ),
            (
                "proposal::a_row_so_the_namespace_exists",
                &[("says", "a thing")],
            ),
        ];
        let f = findings(
            "prose-cite-wrong-ns",
            rows,
            &[(
                "42_member.md",
                "See `proposal::staged_narrowing_depends_on_its_staging`.\n",
            )],
            0,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("staged_narrowing"), "{}", f[0]);
        assert!(
            f[0].contains("proposal::"),
            "the finding names the namespace that was cited rather than the one \
             holding the row: {}",
            f[0]
        );

        // The control: the same slug under the namespace that holds it is
        // silent, which is what makes the arm above a statement about the
        // namespace rather than about the slug.
        assert!(
            findings(
                "prose-cite-right-ns",
                rows,
                &[(
                    "42_member.md",
                    "See `probe::staged_narrowing_depends_on_its_staging`.\n",
                )],
                0,
            )
            .is_empty(),
            "the correctly-namespaced citation fired"
        );
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
