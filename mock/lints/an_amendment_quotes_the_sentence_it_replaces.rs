//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A ruling that amends another without quoting the sentence it replaces.
//!
//! `amends` is the edge for a row that replaces part of an earlier one and
//! leaves the rest standing. The edge alone says that something changed and
//! not what, so a reader of the amended row cannot tell which of its sentences
//! still hold. `amended` carries the replaced sentence, copied from the target's
//! `says`, and this holds the two fields to each other.
//!
//! Three ways to break it, reported apart because each is fixed differently:
//!
//! - `unquoted`: a row named under `amends` with no `amended` entry for it.
//! - `stray`: an `amended` entry whose slug is not under `amends`, or that has
//!   no `<slug>:` at all.
//! - `not-in-target`: a quoted sentence the target's `says` does not hold,
//!   which is a paraphrase, a sentence of another field, or a target since
//!   edited. Compared with runs of whitespace taken as one space, since a
//!   TOML string may wrap where the quotation does not.
//!
//! A lint rather than a tool, because there is no legitimate reading of either
//! state: an amendment that does not say what it amends is an edge with no
//! content, and a quotation the target does not hold quotes nothing.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, list, split_axis, split_entries, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(AnAmendmentQuotesTheSentenceItReplaces)
}

const LINT: &str = "an-amendment-quotes-the-sentence-it-replaces";

/// The one namespace carrying the edge.
const NAMESPACE: &str = "ruling";

/// The edge, and the field quoting what it replaces.
const EDGE: &str = "amends";
const QUOTED: &str = "amended";

/// The target field a quotation is looked for in.
const SAYS: &str = "says";

struct AnAmendmentQuotesTheSentenceItReplaces;
impl Lint for AnAmendmentQuotesTheSentenceItReplaces {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}

/// Whitespace runs as one space, so a wrapped quotation still matches.
fn squeeze(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

impl RepoLint for AnAmendmentQuotesTheSentenceItReplaces {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let reg = ctx.registry();
        let mut out = Vec::new();
        for q in reg.rows_in(NAMESPACE) {
            let targets = list(reg, q, EDGE);
            let entries = text(reg, q, QUOTED).map(split_entries).unwrap_or_default();
            let mut quoted: Vec<&str> = Vec::new();
            for entry in &entries {
                let Some((slug, sentence)) = split_axis(entry) else {
                    out.push(finding(
                        LINT,
                        Some("stray"),
                        format!(
                            "`{q}` has an `amended` entry with no `<slug>:` in front of it: \
                             \"{entry}\". Write it `<slug>: <the sentence>`, the slug one of \
                             the rows under `amends`."
                        ),
                    ));
                    continue;
                };
                if !targets.contains(&slug) {
                    out.push(finding(
                        LINT,
                        Some("stray"),
                        format!(
                            "`{q}` quotes a sentence of `{slug}` under `amended`, and `amends` \
                             does not name `{slug}`. Add it to `amends` where this row replaces \
                             that sentence, or take the entry out where it does not."
                        ),
                    ));
                    continue;
                }
                quoted.push(slug);
                let target = format!("{NAMESPACE}::{slug}");
                // A target naming no row is the engine's `unknown-row-reference`.
                let Some(says) = text(reg, &target, SAYS) else {
                    continue;
                };
                let wanted = squeeze(sentence);
                if wanted.is_empty() || !squeeze(says).contains(&wanted) {
                    out.push(finding(
                        LINT,
                        Some("not-in-target"),
                        format!(
                            "`{q}` says it replaces \"{sentence}\" in `{slug}`, and that \
                             row's `says` does not hold the sentence. Copy it from `{slug}`'s \
                             `says` as it stands, not a paraphrase of it."
                        ),
                    ));
                }
            }
            for target in &targets {
                if !quoted.contains(target) {
                    out.push(finding(
                        LINT,
                        Some("unquoted"),
                        format!(
                            "`{q}` names `{target}` under `amends` and quotes none of its \
                             sentences under `amended`. Add `{target}: <the sentence this \
                             replaces>`, copied from its `says`; where it replaces nothing it \
                             says, the edge is `corrects`, and where it replaces all of it, \
                             `supersedes`."
                        ),
                    ));
                }
            }
        }
        out
    }
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use crate::canon_lint_testkit::{
        assert_findings_block,
        assert_not_declared_off,
        assert_registered,
        findings,
        view,
    };

    fn found(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        findings(
            &super::AnAmendmentQuotesTheSentenceItReplaces,
            &view(rows, &[]),
        )
    }

    const OLD: (&str, &[(&str, &str)]) = ("ruling::old", &[(
        "says",
        "The first sentence stands. The second, with a comma, goes.",
    )]);

    #[test]
    fn an_edge_with_no_quotation_is_reported() {
        let f = found(&[OLD, ("ruling::new", &[("amends", "old")])]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(
            f[0].contains("ruling::new") && f[0].contains("amended"),
            "{}",
            f[0]
        );
    }

    #[test]
    fn a_quotation_the_target_does_not_hold_is_reported() {
        // The planted case: a paraphrase in place of the sentence.
        let f = found(&[
            OLD,
            ("ruling::new", &[
                ("amends", "old"),
                ("amended", "old: The second sentence is gone."),
            ]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("does not hold"), "{}", f[0]);
    }

    #[test]
    fn a_quotation_naming_a_row_not_amended_is_reported() {
        let f = found(&[
            OLD,
            ("ruling::new", &[
                ("amends", "old"),
                (
                    "amended",
                    "old: The first sentence stands., other: Anything.",
                ),
            ]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("`other`"), "{}", f[0]);
    }

    #[test]
    fn a_quotation_with_no_edge_at_all_is_reported() {
        let f = found(&[
            OLD,
            ("ruling::new", &[(
                "amended",
                "old: The first sentence stands.",
            )]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
    }

    #[test]
    fn an_empty_quotation_quotes_nothing() {
        let f = found(&[OLD, ("ruling::new", &[("amends", "old"), ("amended", "old: ")])]);
        assert_eq!(f.len(), 1, "{f:?}");
    }

    #[test]
    fn control_a_verbatim_sentence_carrying_a_comma_passes() {
        // The entry separator is a comma too, so the split must keep a
        // sentence whole where no `<slug>:` follows the comma.
        let f = found(&[
            OLD,
            ("ruling::new", &[
                ("amends", "old"),
                ("amended", "old: The second, with a comma, goes."),
            ]),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_a_quotation_wrapped_differently_still_matches() {
        let f = found(&[
            OLD,
            ("ruling::new", &[
                ("amends", "old"),
                ("amended", "old: The first\n  sentence   stands."),
            ]),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn two_targets_each_owe_their_own_quotation() {
        let other = ("ruling::other", &[("says", "One. Two.")][..]);
        let f = found(&[
            OLD,
            other,
            ("ruling::new", &[
                ("amends", "old, other"),
                ("amended", "old: The first sentence stands."),
            ]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("`other`"), "{}", f[0]);
        let f = found(&[
            OLD,
            other,
            ("ruling::new", &[
                ("amends", "old, other"),
                ("amended", "old: The first sentence stands., other: Two."),
            ]),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_a_row_with_neither_field_is_silent() {
        assert!(found(&[OLD, ("ruling::new", &[("supersedes", "old")])]).is_empty());
    }

    #[test]
    fn a_proposal_is_not_read() {
        let f = found(&[("proposal::p", &[("amends", "old")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_an_edge_into_nothing_is_left_to_the_engine() {
        let f = found(&[("ruling::new", &[
            ("amends", "gone"),
            ("amended", "gone: Words."),
        ])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(&[OLD, ("ruling::new", &[("amends", "old")])], &[]);
        assert_findings_block(&super::AnAmendmentQuotesTheSentenceItReplaces, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::AnAmendmentQuotesTheSentenceItReplaces);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::AnAmendmentQuotesTheSentenceItReplaces.name(),
            "an-amendment-quotes-the-sentence-it-replaces"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("an-amendment-quotes-the-sentence-it-replaces");
    }
}
