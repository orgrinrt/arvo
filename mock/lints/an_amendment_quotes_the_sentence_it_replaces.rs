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
//! A proposal carries the same pair for a ratified ruling's `note`:
//! `corrects_note` names the ruling and `note_said` the stale sentence, copied
//! from its `note`, so the registry says the note is stale while the ruling
//! stays as he ratified it. The pairs are read by one check.
//!
//! Four ways to break it, reported apart because each is fixed differently:
//!
//! - `unquoted`: a row named under `amends` with no `amended` entry for it.
//! - `stray`: an `amended` entry whose slug is not under `amends`, or that has
//!   no `<slug>:` at all.
//! - `not-in-target`: a quoted sentence the target's `says` does not hold,
//!   which is a paraphrase, a sentence of another field, or a target since
//!   edited. Compared with runs of whitespace taken as one space, since a
//!   TOML string may wrap where the quotation does not.
//! - `not-ratified`: a note correction on a row that is not ratified, whose
//!   note is edited in place instead.
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

/// One edge that replaces or corrects part of a row, and the field quoting
/// the part. The two pairs differ only in where they sit and what they quote,
/// so one check reads both.
struct Quoting {
    /// The namespace carrying the edge.
    namespace:        &'static str,
    /// The edge, and the field quoting what it is about.
    edge:             &'static str,
    quoted:           &'static str,
    /// The namespace the edge points into, and the field a quotation is
    /// looked for in.
    target_namespace: &'static str,
    target_field:     &'static str,
    /// What to write where the edge names a row and quotes none of it.
    unquoted_hint:    &'static str,
    /// Whether the target must be ratified. A note on any other row is
    /// edited in place, so a correction standing beside it is a second copy.
    ratified_only:    bool,
}

/// `amends` replaces a sentence of a ruling's `says`; `corrects_note` is a
/// proposal saying a sentence of a ratified ruling's `note` is stale, which
/// leaves the ruling as he ratified it.
const QUOTINGS: [Quoting; 2] = [
    Quoting {
        namespace:        "ruling",
        edge:             "amends",
        quoted:           "amended",
        target_namespace: "ruling",
        target_field:     "says",
        unquoted_hint:    "where it replaces nothing it says, the edge is `corrects`, and where \
                        it replaces all of it, `supersedes`.",
        ratified_only:    false,
    },
    Quoting {
        namespace:        "proposal",
        edge:             "corrects_note",
        quoted:           "note_said",
        target_namespace: "ruling",
        target_field:     "note",
        unquoted_hint:    "the proposal's `says` gives what the note should say in its place.",
        ratified_only:    true,
    },
];

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
        for pair in &QUOTINGS {
            let Quoting {
                namespace,
                edge,
                quoted: field,
                target_namespace,
                target_field,
                unquoted_hint,
                ratified_only,
            } = pair;
            for q in reg.rows_in(namespace) {
                let targets = list(reg, q, edge);
                let entries = text(reg, q, field).map(split_entries).unwrap_or_default();
                let mut quoted: Vec<&str> = Vec::new();
                for entry in &entries {
                    let Some((slug, sentence)) = split_axis(entry) else {
                        out.push(finding(
                            LINT,
                            Some("stray"),
                            format!(
                                "`{q}` has a `{field}` entry with no `<slug>:` in front of it: \
                                 \"{entry}\". Write it `<slug>: <the sentence>`, the slug one of \
                                 the rows under `{edge}`."
                            ),
                        ));
                        continue;
                    };
                    if !targets.contains(&slug) {
                        out.push(finding(
                            LINT,
                            Some("stray"),
                            format!(
                                "`{q}` quotes a sentence of `{slug}` under `{field}`, and `{edge}` \
                                 does not name `{slug}`. Add it to `{edge}` where this row is about \
                                 that sentence, or take the entry out where it is not."
                            ),
                        ));
                        continue;
                    }
                    quoted.push(slug);
                    let target = format!("{target_namespace}::{slug}");
                    // A target naming no row is the engine's `unknown-row-reference`.
                    let Some(held) = text(reg, &target, target_field) else {
                        continue;
                    };
                    let wanted = squeeze(sentence);
                    if wanted.is_empty() || !squeeze(held).contains(&wanted) {
                        out.push(finding(
                            LINT,
                            Some("not-in-target"),
                            format!(
                                "`{q}` quotes \"{sentence}\" from `{slug}`, and that row's \
                                 `{target_field}` does not hold the sentence. Copy it from \
                                 `{slug}`'s `{target_field}` as it stands, not a paraphrase of it."
                            ),
                        ));
                    }
                }
                for target in &targets {
                    let row = format!("{target_namespace}::{target}");
                    if *ratified_only
                        && reg.rows_in(target_namespace).iter().any(|r| *r == row)
                        && text(reg, &row, "rung") != Some("ratified")
                    {
                        out.push(finding(
                            LINT,
                            Some("not-ratified"),
                            format!(
                                "`{q}` names `{target}` under `{edge}`, and `{target}` is not \
                                 ratified. Edit its `{target_field}` in place; `{edge}` is for a \
                                 row nobody but him may edit."
                            ),
                        ));
                    }
                    if !quoted.contains(target) {
                        out.push(finding(
                            LINT,
                            Some("unquoted"),
                            format!(
                                "`{q}` names `{target}` under `{edge}` and quotes none of its \
                                 sentences under `{field}`. Add `{target}: <the sentence>`, \
                                 copied from its `{target_field}`; {unquoted_hint}"
                            ),
                        ));
                    }
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

    const RATIFIED: (&str, &[(&str, &str)]) = ("ruling::held", &[
        ("rung", "ratified"),
        ("says", "What he ratified."),
        ("note", "The tree had two crates. Nothing else is said."),
    ]);

    #[test]
    fn control_a_note_correction_quoting_the_note_passes() {
        let f = found(&[
            RATIFIED,
            ("proposal::fix", &[
                ("corrects_note", "held"),
                ("note_said", "held: The tree had two crates."),
            ]),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn a_note_correction_quoting_the_says_is_reported() {
        let f = found(&[
            RATIFIED,
            ("proposal::fix", &[
                ("corrects_note", "held"),
                ("note_said", "held: What he ratified."),
            ]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("`note` does not hold"), "{f:?}");
    }

    #[test]
    fn a_note_correction_with_no_quotation_is_reported() {
        let f = found(&[RATIFIED, ("proposal::fix", &[("corrects_note", "held")])]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("quotes none"), "{f:?}");
    }

    #[test]
    fn a_note_correction_on_an_unratified_row_is_reported() {
        let f = found(&[
            ("ruling::loose", &[
                ("rung", "stated"),
                ("note", "Old words."),
            ]),
            ("proposal::fix", &[
                ("corrects_note", "loose"),
                ("note_said", "loose: Old words."),
            ]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("is not ratified"), "{f:?}");
    }

    #[test]
    fn a_ruling_s_corrects_note_is_not_read() {
        let f = found(&[RATIFIED, ("ruling::other", &[("corrects_note", "held")])]);
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
