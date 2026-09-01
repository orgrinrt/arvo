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
//! Eleven occurrences over eleven distinct slugs across six files, all of them
//! numbered member files and none a ledger, so the ledger arm starts at zero
//! and refuses the first one written.
//!
//! **Read off the finding rather than bisected for.** Setting the ceiling to
//! zero prints the population and the slugs in it; searching for the largest N
//! that passes finds the same number and shows none of that, so it cannot say
//! whether the number is right for the right reason.
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
//! **Four are real, and they are two kinds.**
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
//! # A trailing underscore is kept rather than trimmed
//!
//! The token runs to the first character that cannot be in a slug, so
//! `..._top_` is read with its underscore and does not resolve. Trimming it
//! would not resolve either, since the trimmed form is no row, so nothing is
//! lost by keeping it and something is given up by cutting it: a scanner that
//! drops a trailing separator has guessed at what a writer meant, and the guess
//! is wrong wherever a slug legitimately continues past that point. A defect
//! that disappears into a passing check is the failure this lint is about.
//!
//! # A trailing `...` says a cut happened, and is read as one
//!
//! Four occurrences over two distinct slugs are written with an ellipsis after
//! them: `proposal::a_min_plus_fold_needs_an_absorbing_top_...` against a row
//! ending `_and_wrapping_supplies_none`, and
//! `proposal::the_multiplicative_guard...` against one ending
//! `_grows_linearly_and_the_saving_is_adaptation_fusion`. The second reads as a
//! clean concept name rather than a cut-off one, which is why it sat among the
//! paraphrases until somebody measured it against the ids.
//!
//! `...` is outside the slug charset, so it cannot be part of a spelling and
//! the author put it there to say the id continues. That is the whole
//! difference from the trailing underscore above, which is inside the charset
//! and says nothing. **The dots say a cut happened; they do not say what was
//! cut**, so resolving one still means picking a row, and what makes that
//! defensible is that exactly one row carries the prefix rather than the
//! author's punctuation.
//!
//! **An elision is never counted, in either population.** Whether one resolves
//! is a question about the registry rather than about the prose, so counting
//! them would make the ceiling a function of the registry: one added row
//! sharing a prefix turns a resolving elision ambiguous and puts the gate over
//! its ceiling, with no repair available, since the member file is the record
//! and the ceiling may not be raised. Measured, not supposed: planting one
//! schema-valid row on the `the_multiplicative_guard` prefix moved the count by
//! three with no member file touched. So an ambiguous or unmatched elision is
//! reported at its own `file:line` instead, where a new row produces a specific
//! finding about a specific citation and the count stays a function of the
//! prose alone.
//!
//! # A line break inside a slug is a wrap rather than a truncation
//!
//! These ids run to ninety characters and the prose wraps at eighty-five, so a
//! citation starting anywhere but the left margin gets cut by the break. The
//! scanner read one line at a time and reported the first half, which names no
//! row and reads exactly like the near miss above. **The writer had written the
//! whole slug.**
//!
//! So where a token runs to the very end of its line and the next line opens
//! with the rest of a real id, the citation resolves and is not reported. The
//! guard is that the token has to reach the line's end: a slug the writer cut
//! himself is followed by a backtick, a dot or a space, which ends the token
//! before the break and leaves the near-miss arm untouched. That is the whole
//! of the difference between the two, and both arms are pinned by tests.
//!
//! It stayed invisible because the count is what anybody reads. Two of the
//! standing population were wrapped citations of rows that exist, and one of
//! them was written up in this file as a writer's near miss against a row
//! ending `_side`. Nobody opened the line.
//!
//! **What this does not fix is the rendered document.** A code span broken
//! across a source line renders with a space in the middle of the slug, so the
//! citation reads wrong on the page even though it resolves here. Repairing
//! that means reflowing a member file, which is the record, so it is left and
//! said rather than done.
use std::path::Path;

use mockspace::{Lint, LintError, RegistryView, RepoContext, RepoLint, Severity};

use crate::panel_corpus::{finding, markdown, panel_dir, shown, LIVING_LEDGERS};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(NoProseCitationIntoNothing)
}

const NAME: &str = "no-prose-citation-into-nothing";

/// The standing count in files that are the record, measured over the
/// committed tree: eleven occurrences over eleven slugs across six files.
///
/// Established by setting this to zero and reading the finding, which prints
/// the population and the slugs in it. Bisecting for green-at-N and red-at-N-1
/// arrives at the same number and shows neither, so it cannot say whether the
/// number is right for the right reason.
///
/// It falls when a slug is declared under the id the prose already used, it
/// fell by two when the scanner stopped cutting a citation at a line break,
/// and by four more when a trailing `...` stopped being read as a spelling.
/// **Do not raise it.**
const CEILING: usize = 11;

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

    // An elision never joins a counted population, whichever file it sits in.
    //
    // Whether an elided spelling resolves is a question about the registry
    // rather than about the prose, so counting them would make the ceiling a
    // function of the registry: one added row sharing a prefix turns a
    // resolving elision ambiguous and puts the gate over its ceiling with no
    // permitted repair, since the member file is the record and the ceiling
    // may not be raised. Reported per site instead, where a new row produces
    // a specific finding about a specific citation and the count stays a
    // function of the prose alone.
    let (elided, whole): (Vec<&Cited>, Vec<&Cited>) = found
        .iter()
        .partition(|c| matches!(c.how, How::Elided { .. }));

    let mut out: Vec<LintError> = elided
        .iter()
        .map(|c| {
            let How::Elided { rows } = c.how else {
                unreachable!("partitioned on this")
            };
            let why = if rows == 0 {
                "reaches no row at all, so there is nothing it could be short for".to_string()
            } else {
                format!(
                    "reaches {rows} rows, so nothing here can tell which one was meant \
                     without guessing"
                )
            };
            finding(
                NAME,
                &c.at,
                c.line,
                format!(
                    "cites `{}...`, cut short, and the prefix {why}. Spell it out, or pick \
                     the id that says what the sentence needs.",
                    c.text
                ),
            )
        })
        .collect();

    let (in_ledgers, in_the_record): (Vec<&&Cited>, Vec<&&Cited>) =
        whole.iter().partition(|c| is_a_living_ledger(&c.at));

    out.extend(in_ledgers.iter().map(|c| {
        finding(
            NAME,
            &c.at,
            c.line,
            format!(
                "cites `{}`, which resolves to no row. This file is edited, so fix \
                 the citation: query the registry for the id it meant, and mind that a \
                 slug one word short of a real one reads as a citation and is not. Where \
                 no row says it, the sentence is a paraphrase and cites nothing.",
                c.text
            ),
        )
    }));

    if in_the_record.len() > ceiling {
        let mut slugs: Vec<String> = in_the_record.iter().map(|c| c.text.clone()).collect();
        slugs.sort();
        slugs.dedup();
        let mut per_file: Vec<String> = in_the_record.iter().map(|c| c.at.clone()).collect();
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

/// One citation the prose carries that names no row, and how it was spelled.
struct Cited {
    at:   String,
    line: usize,
    /// What to put in front of a reader, which is not always the token as
    /// scanned: a citation wrapped inside a code span is reported rejoined,
    /// because the rejoined string is the one the author wrote and the one a
    /// search for it will find.
    text: String,
    how:  How,
}

/// How the prose spelled the citation, which decides what failing to resolve
/// it means.
enum How {
    /// Spelled out. Failing to resolve is a citation into nothing, and it
    /// counts against the ceiling when it sits in a member file.
    Whole,
    /// Cut short with a trailing `...`, carrying how many rows the prefix
    /// reaches. Never counted: see [`check`].
    Elided { rows: usize },
}

/// Every `<namespace>::<slug>` the panel's prose carries that names no row.
fn citations(dir: &Path, namespaces: &[&str], reg: &RegistryView) -> Vec<Cited> {
    let mut out = Vec::new();
    for path in markdown(dir) {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let at = shown(&path, dir);
        let lines: Vec<&str> = text.lines().collect();
        for (n, line) in lines.iter().enumerate() {
            let carry = lines
                .get(n + 1)
                .copied()
                .map(leading_slug_run)
                .unwrap_or("");
            for (cited, elided) in tokens(line, namespaces) {
                if reg.row(&cited).is_some() {
                    continue;
                }
                if elided {
                    let rows = rows_the_prefix_reaches(&cited, reg);
                    if rows == 1 {
                        continue;
                    }
                    out.push(Cited {
                        at: at.clone(),
                        line: n + 1,
                        text: cited,
                        how: How::Elided { rows },
                    });
                    continue;
                }
                if line.ends_with(&cited) && reg.row(&format!("{cited}{carry}")).is_some() {
                    continue;
                }
                out.push(Cited {
                    at: at.clone(),
                    line: n + 1,
                    text: if ends_inside_a_code_span(line, &cited) {
                        format!("{cited}{carry}")
                    } else {
                        cited
                    },
                    how: How::Whole,
                });
            }
        }
    }
    out
}

/// How many rows in the cited namespace the elided spelling is a prefix of.
///
/// `rows_in` hands back qualified identifiers rather than bare slugs, so the
/// prefix tested is the whole `<namespace>::<slug>` and not the slug alone. A
/// first version compared the bare half against the qualified list, matched
/// nothing anywhere, and reported every elision exactly as before.
fn rows_the_prefix_reaches(cited: &str, reg: &RegistryView) -> usize {
    let Some((ns, _)) = cited.split_once("::") else {
        return 0;
    };
    reg.rows_in(ns)
        .iter()
        .filter(|id| id.starts_with(cited))
        .count()
}

/// Whether the line stops mid-citation with a code span still open, which is
/// what a wrapped citation looks like from its first line.
///
/// This decides only what string gets printed, never whether lines are joined.
/// Ending on the citation is on its own too loose to report a fusion from: a
/// citation that genuinely names nothing frequently ends a line of ordinary
/// prose, and gluing the next line's first word onto it prints a string
/// nobody wrote. The unclosed span is the part that says a wrap happened.
fn ends_inside_a_code_span(line: &str, cited: &str) -> bool {
    line.ends_with(cited) && line.bytes().filter(|b| *b == b'`').count() % 2 == 1
}

/// The run of slug characters a line opens with, which is what a wrapped
/// citation leaves on the second line.
fn leading_slug_run(line: &str) -> &str {
    let end = line
        .find(|c: char| !is_slug_byte(c as u8) || !c.is_ascii())
        .unwrap_or(line.len());
    &line[..end]
}

/// Every qualified slug on one line, read to its own boundaries.
///
/// A token starts where a namespace is preceded by something that cannot be in
/// a slug, and runs to the first character that cannot be either. Both halves
/// are load-bearing: without the left boundary
/// `arvo_proposal::a_row` reports the wrong namespace, and without the right
/// one a truncated slug is read as the longer row it is missing a word from.
fn tokens(line: &str, namespaces: &[&str]) -> Vec<(String, bool)> {
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
            // `...` is outside the slug charset, so it cannot be part of a
            // spelling and the author put it there to say a cut happened. A
            // trailing `_` is inside the charset and says nothing, which is
            // why the two are read differently.
            out.push((
                format!("{ns}::{}", &rest[..end]),
                rest[end..].starts_with("..."),
            ));
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
    fn a_slug_wrapped_across_a_line_break_resolves_rather_than_reading_as_a_prefix() {
        // The corpus wraps at eighty-five columns and these ids run to ninety,
        // so this is the ordinary shape rather than an edge. Read one line at a
        // time it is indistinguishable from the truncation above, and the whole
        // slug is there.
        let f = findings(
            "prose-cite-wrapped",
            ONE_ROW,
            &[(
                "42_member.md",
                "That is what `ruling::the_warrant_is_a_token_and_a_clause_on_the_values\n\
                 _side` defines the token for.\n",
            )],
            0,
        );
        assert!(
            f.is_empty(),
            "a wrapped citation of a real row fired: {f:?}"
        );
    }

    #[test]
    fn a_wrapped_slug_that_still_names_no_row_fires() {
        // The case that must fail, and without it the arm above is satisfied by
        // a scanner that stopped reporting wrapped citations altogether.
        let f = findings(
            "prose-cite-wrapped-dangling",
            ONE_ROW,
            &[(
                "42_member.md",
                "See `ruling::the_warrant_is_a_token_and_a_clause_on_the_values\n\
                 _but_no_such_row` here.\n",
            )],
            0,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("_on_the_values"), "{}", f[0]);
    }

    #[test]
    fn a_slug_the_writer_closed_is_not_joined_with_the_next_line() {
        // What keeps the near-miss arm alive. The backtick ends the token
        // before the break, so what follows is a new sentence rather than the
        // rest of the id, and joining the two would resolve a citation the
        // writer really did cut short. The guard is that the token has to run
        // to the line's last byte.
        let f = findings(
            "prose-cite-closed-then-continues",
            ONE_ROW,
            &[(
                "42_member.md",
                "See `ruling::the_warrant_is_a_token_and_a_clause_on_the_values`.\n\
                 _side is a phrase that happens to open this line.\n",
            )],
            0,
        );
        assert_eq!(f.len(), 1, "the closed truncation stopped firing: {f:?}");
        assert!(f[0].contains("_on_the_values"), "{}", f[0]);
    }

    #[test]
    fn a_citation_ending_the_file_has_no_continuation_to_read() {
        // There is no next line to carry, and reaching for one is how this
        // arrives as a panic in a lint rather than as a finding.
        let f = findings(
            "prose-cite-last-line",
            ONE_ROW,
            &[(
                "42_member.md",
                "ends at `ruling::the_warrant_is_a_token_and_a_clause_on_the_values",
            )],
            0,
        );
        assert_eq!(f.len(), 1, "{f:?}");
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

    /// Two rows sharing a prefix, for the arms about what an elision reaches.
    const TWO_ROWS_ONE_PREFIX: &[(&str, &[(&str, &str)])] = &[
        (
            "ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side",
            &[("says", "a thing")],
        ),
        (
            "ruling::the_warrant_is_a_token_and_a_clause_on_the_other_side",
            &[("says", "another thing")],
        ),
    ];

    #[test]
    fn an_ellipsis_after_a_prefix_resolves_to_the_one_row_it_reaches() {
        let f = findings(
            "prose-cite-elided",
            ONE_ROW,
            &[(
                "42_member.md",
                "See `ruling::the_warrant_is_a_token_and_a_clause...` for the shape.\n",
            )],
            0,
        );
        assert!(f.is_empty(), "an unambiguous elision fired: {f:?}");
    }

    #[test]
    fn the_same_prefix_without_the_ellipsis_is_still_reported() {
        // The control, and without it the arm above is satisfied by a scanner
        // that resolves every prefix. The dots are the whole difference.
        let f = findings(
            "prose-cite-elided-control",
            ONE_ROW,
            &[(
                "42_member.md",
                "See `ruling::the_warrant_is_a_token_and_a_clause` for the shape.\n",
            )],
            0,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("ceiling of 0"), "{}", f[0]);
        assert!(
            f[0].contains("ruling::the_warrant_is_a_token_and_a_clause\""),
            "reported as the longer row it is a prefix of: {}",
            f[0]
        );
    }

    #[test]
    fn an_ellipsis_reaching_two_rows_is_reported_rather_than_picked_between() {
        let f = findings(
            "prose-cite-elided-ambiguous",
            TWO_ROWS_ONE_PREFIX,
            &[(
                "42_member.md",
                "See `ruling::the_warrant_is_a_token_and_a_clause...` for the shape.\n",
            )],
            usize::MAX,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("reaches 2 rows"), "{}", f[0]);
    }

    #[test]
    fn an_ellipsis_reaching_no_row_is_reported_like_any_dangling_citation() {
        let f = findings(
            "prose-cite-elided-nothing",
            ONE_ROW,
            &[("42_member.md", "See `ruling::no_such_prefix_anywhere...`.\n")],
            usize::MAX,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("reaches no row at all"), "{}", f[0]);
    }

    #[test]
    fn an_ellipsis_does_not_borrow_rows_from_another_namespace() {
        // `rows_in` is asked for the cited namespace only. A version that
        // searched every row resolves this, because the `ruling` row carries
        // the prefix. The second row exists so `proposal` is a namespace the
        // scanner looks for at all.
        let f = findings(
            "prose-cite-elided-wrong-ns",
            &[
                (
                    "ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side",
                    &[("says", "a thing")],
                ),
                ("proposal::something_else_entirely", &[("says", "a thing")]),
            ],
            &[(
                "42_member.md",
                "See `proposal::the_warrant_is_a_token_and_a_clause...`.\n",
            )],
            usize::MAX,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("reaches no row at all"), "{}", f[0]);
    }

    #[test]
    fn a_row_that_makes_an_elision_ambiguous_does_not_move_the_counted_population() {
        // The property that keeps the ceiling a statement about the prose. An
        // elision folded into the count lets a registry addition put the gate
        // over its ceiling with no member file touched and no repair
        // available, since the file is the record and the number may not be
        // raised.
        let prose = &[(
            "42_member.md",
            "See `ruling::the_warrant_is_a_token_and_a_clause...`, and \
             `ruling::a_row_that_was_never_written` too.\n",
        )];
        let before = findings("prose-cite-count-before", ONE_ROW, prose, 0);
        let after = findings(
            "prose-cite-count-after",
            TWO_ROWS_ONE_PREFIX,
            prose,
            0,
        );

        let counted = |f: &[String]| {
            f.iter()
                .find(|m| m.contains("against a ceiling of"))
                .cloned()
                .unwrap_or_else(|| panic!("no counted finding: {f:?}"))
        };
        assert!(
            counted(&before).contains("1 prose citations"),
            "{}",
            counted(&before)
        );
        assert!(
            counted(&after).contains("1 prose citations"),
            "the added row moved the count: {}",
            counted(&after)
        );
        assert_eq!(after.len(), 2, "the ambiguity is reported on its own: {after:?}");
    }

    #[test]
    fn a_citation_wrapped_inside_a_code_span_is_reported_rejoined() {
        // What a reader has to search for is the string the author wrote, not
        // the half of it that happens to sit on the first line.
        let f = findings(
            "prose-cite-fused",
            ONE_ROW,
            &[(
                "42_member.md",
                "See `ruling::the_warrant_is_a_token_and_a_clause_on_the_values\n\
                 _but_no_such_row` here.\n",
            )],
            0,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(
            f[0].contains("_on_the_values_but_no_such_row"),
            "reported only the first line's half: {}",
            f[0]
        );
    }

    #[test]
    fn a_dangling_citation_merely_ending_a_line_is_not_glued_to_the_next() {
        // The control for the arm above, and the reason the rejoin is gated on
        // an unclosed code span rather than on the token ending the line. Here
        // the span is closed, so the next line is ordinary prose and printing
        // it as part of the citation would print a string nobody wrote.
        let f = findings(
            "prose-cite-not-fused",
            ONE_ROW,
            &[(
                "42_member.md",
                "See `ruling::a_row_that_was_never_written`\n\
                 and_then_a_sentence continues.\n",
            )],
            0,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(
            !f[0].contains("and_then_a_sentence"),
            "glued the next line on: {}",
            f[0]
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
