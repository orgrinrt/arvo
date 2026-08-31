//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A question somebody has answered does not go back into the queue.
//!
//! Three fields carry the whole of who owes an answer, and when this was first
//! written no check, no lint and no tool named any of them. Measured rather than
//! asserted, by a probe searching the quoted literal a check would use over
//! `mock/checks`, `mock/lints` and `mock/tools`: `decider` 0, `bound` 0,
//! `unblocks` 0, `answers` 0, against a control of `rung` 25 in the same run, so
//! the instrument reached the trees it was pointed at.
//!
//! What that left unguarded is a queue that cannot tell a settled question from
//! an open one. The `question` namespace is defined as "Something the canon has
//! not settled", the rendered document is a table of nine columns and the
//! incoming `answers` edge is not one of them, so a question op has already
//! answered renders as open, reads as open, and counts as open in every roster
//! built by reading `decider`. That has cost him a round trip once already,
//! which is the reason the `bound` field exists at all.
//!
//! # The second hop, which is the correction this port carries
//!
//! **The check this came from read `ruling.answers` and nothing else, and that
//! misses a question settled two edges away.** A `ruling` at `rung = "ratified"`
//! carries `ratifies`, naming `proposal` rows; those proposals carry their own
//! `answers` edges. A question reached that way is settled by something op
//! ratified, and no reading of `ruling.answers` alone can see it. Measured on
//! the committed registry when the gap was found:
//! `ruling::the_format_spine_is_canon` is ratified and ratifies four proposals,
//! three of which answer a question, and all three of those questions read as
//! open.
//!
//! **The ratification is what does the settling, not the proposal.** A proposal
//! that answers a question and has not been ratified settles nothing, which is
//! why the second hop reads `rung` rather than following every `ratifies` edge
//! it finds. An unratified row carrying one is a different defect and
//! `an-unratified-ruling-stamps-a-proposal` refuses it.
//!
//! **Which definition of settled this is.** Four notions of the word are in use
//! across the registry. This one is the row's own `answered` field, or one of
//! the two recorded-as phrases in `note` or `bound`, with the settling edge
//! being `ruling.answers` directly or `ruling.ratifies` into a
//! `proposal.answers`. It is the definition the check already used, with the
//! reach of the edge corrected. It is not a fifth notion and the correction is
//! deliberately not a redefinition.
//!
//! **What `mock/tools/unasked-questions` does differently is correct there and
//! stays.** That tool excludes `proposal.answers` outright, because a proposal
//! does not settle what it names while it is a proposal, so a question with only
//! a proposal against it is genuinely still open. The two-hop case is different:
//! there the ratifying ruling is what settles it.
//!
//! # What the unit tests here cannot ask
//!
//! That the committed canon agrees with this predicate. A unit test cannot build
//! a `RegistryView` from `mock/registry/`, because that needs a TOML parser the
//! generated pack has no route to depend on. `cargo mock --lint-only` is where
//! the predicate meets the real rows, and it runs this over all of them at every
//! gate.
use std::collections::{BTreeMap, BTreeSet};

use mockspace::{Lint, LintError, RegistryView, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, has, list, slug, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ASettledQuestionDoesNotSitInTheQueue)
}

const LINT: &str = "a-settled-question-does-not-sit-in-the-queue";

/// The phrase this namespace already uses to say a question is settled.
///
/// Not invented here, and not a bare word either. Five rows arrived at the same
/// construction independently, before any check existed to ask for it:
/// "Recorded as answered at `28` batch one", "Recorded as answered by op on
/// 2026-08-14", "Recorded as closed by op on 2026-08-14", "Recorded as answered
/// on 2026-08-14".
///
/// **The bare words do not work and the first cut used them.** They appear on 54
/// lines of the question registry in ordinary prose, including "a further item
/// folds in here rather than being answered on its own", which is a row saying
/// the opposite of settled. Worse, `id` is a field like any other, so a planted
/// row named `answered_twice` matched itself and the control that should have
/// caught it passed.
const SETTLED_PHRASES: [&str; 2] = ["recorded as answered", "recorded as closed"];

/// Where the phrase may sit.
///
/// `note` is where all five put it and `bound` is where a returned-then-settled
/// question would. Deliberately not every field: scanning `id`, `asks` and
/// `options` is what made the first cut useless.
const PROSE_FIELDS: [&str; 2] = ["note", "bound"];

/// The field whose whole purpose is to say what settled a question.
///
/// A row carrying one says it is settled by construction and needs no phrase. It
/// was added to the schema after the check was written, for answers that mint no
/// ruling, and the check never learned about it: four questions op had answered
/// were reported as open with the answer sitting in the row.
const ANSWER_FIELD: &str = "answered";

/// The rung at which a ruling's stamp settles what it ratifies.
const RATIFIED: &str = "ratified";

struct ASettledQuestionDoesNotSitInTheQueue;
impl Lint for ASettledQuestionDoesNotSitInTheQueue {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for ASettledQuestionDoesNotSitInTheQueue {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let reg = ctx.registry;
        let mut out = answers_naming_no_question(reg);
        out.extend(op_deciders_carrying_a_bound(reg));
        out.extend(settled_questions_that_do_not_say_so(reg));
        out
    }
}

/// Which questions each ruling settles, read backward, over both hops.
///
/// The forward edge is `ruling.answers`, and the second hop is
/// `ruling.ratifies` into a ratified proposal's own `answers`. The registry has
/// no reverse index for either and neither does the rendered document, so a
/// reader standing on a question row cannot see that a ruling settles it without
/// grepping the other file. This is that grep, done once, over both routes.
///
/// The value names the ruling rather than the proposal in the two-hop case,
/// because the ruling is what did the settling and is what a reader has to open.
fn answered_by(reg: &RegistryView) -> BTreeMap<String, BTreeSet<String>> {
    let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for ruling in reg.rows_in("ruling") {
        let name = slug(ruling).to_string();
        for question in list(reg, ruling, "answers") {
            out.entry(question.to_string()).or_default().insert(name.clone());
        }
        if text(reg, ruling, "rung") != Some(RATIFIED) {
            continue;
        }
        for stamped in list(reg, ruling, "ratifies") {
            let proposal = format!("proposal::{stamped}");
            for question in list(reg, &proposal, "answers") {
                out.entry(question.to_string()).or_default().insert(name.clone());
            }
        }
    }
    out
}

/// An `answers` edge naming a question that is not declared anywhere.
///
/// Referential integrity on the edges that close a question. A dangling entry
/// reads as a settled question and points at nothing, which is worse than an
/// open one, because the row it would have closed stays open silently.
///
/// Both namespaces carrying the edge are read, which is wider than the check
/// this came from. A `proposal.answers` naming nothing is the same broken
/// pointer, and it becomes load-bearing the moment a ruling ratifies that
/// proposal.
fn answers_naming_no_question(reg: &RegistryView) -> Vec<LintError> {
    let declared: BTreeSet<&str> = reg.rows_in("question").iter().map(|q| slug(q)).collect();
    let mut out = Vec::new();
    for namespace in ["ruling", "proposal"] {
        for row in reg.rows_in(namespace) {
            for question in list(reg, row, "answers") {
                if !declared.contains(question) {
                    out.push(finding(
                        LINT,
                        Some("answers-names-no-question"),
                        format!(
                            "`{row}` has `answers` naming `question::{question}`, which is \
                             declared nowhere. An edge that closes a question and points at \
                             nothing leaves the row it meant to close open, silently."
                        ),
                    ));
                }
            }
        }
    }
    out
}

/// A question filed as op's while carrying the record that he handed it back.
///
/// The schema states this one itself, in the `bound` field's own description: a
/// `decider = op` row carrying no `bound` has simply not been asked yet; one
/// carrying a `bound` and still naming him is a filing error. The canon is clean
/// of it today, which is exactly the condition under which an unenforced rule
/// stops being true without anybody noticing.
fn op_deciders_carrying_a_bound(reg: &RegistryView) -> Vec<LintError> {
    reg.rows_in("question")
        .iter()
        .filter(|q| text(reg, q, "decider") == Some("op"))
        .filter(|q| has(reg, q, "bound"))
        .map(|q| {
            finding(
                LINT,
                Some("op-decider-carrying-a-bound"),
                format!(
                    "`{q}` carries a `bound`, which records that it was put to him and \
                     returned, and still names him as the decider. One of the two is wrong."
                ),
            )
        })
        .collect()
}

/// A question a ruling settles, whose own row does not say so.
///
/// The consequence is not cosmetic. Every roster of what op owes is built by
/// reading `decider`, the rendered table does not carry the incoming edge, and
/// so a question he answered goes back into the queue.
fn settled_questions_that_do_not_say_so(reg: &RegistryView) -> Vec<LintError> {
    let settled = answered_by(reg);
    let mut out = Vec::new();
    for q in reg.rows_in("question") {
        let Some(rulings) = settled.get(slug(q)) else {
            continue;
        };
        if says_so(reg, q) {
            continue;
        }
        let by: Vec<&str> = rulings.iter().map(String::as_str).collect();
        out.push(finding(
            LINT,
            Some("settled-question-reads-as-open"),
            format!(
                "`{q}` is answered by `ruling::{}` and nothing in the row says so, so it \
                 renders as an open question and counts as one. Write the answer into \
                 `answered`, or record in `note` that it was answered and by what.",
                by.join("`, `ruling::")
            ),
        ));
    }
    out
}

/// Whether the row's own text records that it was settled.
fn says_so(reg: &RegistryView, q: &str) -> bool {
    if text(reg, q, ANSWER_FIELD).is_some() {
        return true;
    }
    PROSE_FIELDS.iter().any(|f| {
        text(reg, q, f).is_some_and(|v| {
            let lower = v.to_lowercase();
            SETTLED_PHRASES.iter().any(|p| lower.contains(p))
        })
    })
}
#[cfg(test)]
mod tests {
    use mockspace::{Lint, RepoLint};

    use crate::canon_lint_testkit::{
        assert_findings_block, assert_not_declared_off, assert_registered, ctx, view,
    };
    use crate::canon_rows::JOIN;

    /// The findings, as the kind each carries paired with its message.
    fn found(rows: &[(&str, &[(&str, &str)])]) -> Vec<(Option<&'static str>, String)> {
        let v = view(rows, &[]);
        super::ASettledQuestionDoesNotSitInTheQueue
            .check_repo(&ctx(&v))
            .into_iter()
            .map(|e| (e.finding_kind, e.message))
            .collect()
    }

    /// A list field as the row wrote it, joined the way the engine joins one.
    fn cited(entries: &[&str]) -> String {
        entries.join(JOIN)
    }

    fn kinds(rows: &[(&str, &[(&str, &str)])]) -> Vec<Option<&'static str>> {
        found(rows).into_iter().map(|(k, _)| k).collect()
    }

    #[test]
    fn a_ruling_answering_a_question_that_does_not_exist_is_reported() {
        let a = cited(&["a_real_one", "a_ghost"]);
        let f = found(&[
            ("question::a_real_one", &[("asks", "Is it?"), ("answered", "yes")]),
            ("ruling::he_said_so", &[("answers", &a)]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].0, Some("answers-names-no-question"));
        assert!(f[0].1.contains("a_ghost"), "the report names which one: {}", f[0].1);
        assert!(f[0].1.contains("ruling::he_said_so"), "{}", f[0].1);
    }

    #[test]
    fn control_a_ruling_answering_only_real_questions_is_silent() {
        assert!(
            kinds(&[
                ("question::a_real_one", &[("asks", "Is it?"), ("answered", "yes")]),
                ("ruling::he_said_so", &[("answers", "a_real_one")]),
            ])
            .is_empty()
        );
    }

    #[test]
    fn a_dangling_answers_edge_on_a_proposal_is_reported_too() {
        // Wider than the check this came from, which read `ruling.answers`
        // alone. A `proposal.answers` naming nothing is the same broken pointer
        // and becomes load-bearing the moment a ruling ratifies that proposal.
        let f = found(&[("proposal::a_claim", &[("answers", "a_ghost")])]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].0, Some("answers-names-no-question"));
        assert!(f[0].1.contains("proposal::a_claim"), "{}", f[0].1);
    }

    #[test]
    fn a_question_filed_as_his_while_carrying_a_bound_is_reported() {
        let f = found(&[(
            "question::handed_back",
            &[
                ("asks", "Which set ships?"),
                ("decider", "op"),
                ("bound", "Put to him and returned. Bounded by soundness."),
            ],
        )]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].0, Some("op-decider-carrying-a-bound"));
    }

    #[test]
    fn control_a_question_of_his_with_no_bound_and_one_bound_but_handed_on_are_both_fine() {
        // The two correct shapes. A `decider = op` row with no `bound` has
        // simply not been asked yet, and a row carrying a `bound` and naming
        // somebody else is the repair. An arm reporting either would be
        // refusing the namespace rather than checking it.
        assert!(
            kinds(&[
                ("question::not_yet_asked", &[("asks", "?"), ("decider", "op")]),
                (
                    "question::handed_on",
                    &[("asks", "?"), ("decider", "panel"), ("bound", "returned")],
                ),
            ])
            .is_empty()
        );
    }

    #[test]
    fn a_question_a_ruling_answers_whose_row_says_nothing_is_reported() {
        let f = found(&[
            ("question::open_looking", &[("asks", "?"), ("decider", "op")]),
            ("ruling::he_said_so", &[("answers", "open_looking")]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].0, Some("settled-question-reads-as-open"));
        assert!(f[0].1.contains("ruling::he_said_so"), "{}", f[0].1);
    }

    #[test]
    fn a_question_carrying_the_answer_or_the_phrase_is_silent() {
        // The three ways a row says it is settled, each of which has to work or
        // the lint reports rows nobody can quiet.
        for (field, value) in [
            ("answered", "He took the third option."),
            ("note", "Recorded as answered by op on 2026-08-14."),
            ("bound", "Recorded as closed by op on 2026-08-14."),
        ] {
            let rows: [(&str, &[(&str, &str)]); 2] = [
                ("question::settled", &[("asks", "?"), (field, value)]),
                ("ruling::he_said_so", &[("answers", "settled")]),
            ];
            assert!(kinds(&rows).is_empty(), "`{field}` did not read as settled");
        }
    }

    #[test]
    fn an_answered_field_holding_nothing_does_not_count_as_the_answer() {
        // The field being present is not the thing asked for, and a blank one
        // tells a reader exactly what a missing one does.
        for blank in ["", "   ", "\n"] {
            let f = kinds(&[
                ("question::settled", &[("asks", "?"), ("answered", blank)]),
                ("ruling::he_said_so", &[("answers", "settled")]),
            ]);
            assert_eq!(f, [Some("settled-question-reads-as-open")], "{blank:?}");
        }
    }

    #[test]
    fn control_a_bare_word_in_prose_is_not_the_phrase() {
        // The first cut matched bare words, which appear on 54 lines of the
        // question registry in ordinary prose, including a row saying the
        // opposite of settled.
        let f = kinds(&[
            (
                "question::still_open",
                &[
                    ("asks", "?"),
                    ("note", "a further item folds in here rather than being answered on its own"),
                ],
            ),
            ("ruling::he_said_so", &[("answers", "still_open")]),
        ]);
        assert_eq!(f, [Some("settled-question-reads-as-open")], "{f:?}");
    }

    #[test]
    fn control_a_question_nothing_answers_is_not_reported_however_open_it_reads() {
        // The whole discrimination. A lint reporting every question with no
        // `answered` field would report the namespace, which is defined as
        // things the canon has not settled.
        assert!(kinds(&[("question::genuinely_open", &[("asks", "?")])]).is_empty());
    }

    #[test]
    fn a_ratified_ruling_reaches_a_question_through_the_proposal_it_stamps() {
        // The second hop, and the correction this port carries. The check this
        // came from read `ruling.answers` alone and could not see this at all;
        // three questions in the committed registry were settled exactly this
        // way and every one of them read as open.
        let f = found(&[
            ("question::reached_two_hops", &[("asks", "?")]),
            (
                "proposal::a_stamped_claim",
                &[("answers", "reached_two_hops")],
            ),
            (
                "ruling::the_spine_is_canon",
                &[("rung", "ratified"), ("ratifies", "a_stamped_claim")],
            ),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].0, Some("settled-question-reads-as-open"));
        assert!(
            f[0].1.contains("ruling::the_spine_is_canon"),
            "the report names the ruling that did the settling rather than the proposal, \
             because the ruling is what a reader has to open: {}",
            f[0].1
        );
    }

    #[test]
    fn control_an_unratified_ruling_stamping_a_proposal_settles_nothing() {
        // The ratification is what settles it, not the proposal. A row at any
        // other rung carrying `ratifies` is a different defect, refused by
        // `an-unratified-ruling-stamps-a-proposal`, and following its edge here
        // would close a question on nobody's authority.
        for rung in ["stated", "in_force", "open", ""] {
            let f = kinds(&[
                ("question::reached_two_hops", &[("asks", "?")]),
                (
                    "proposal::a_stamped_claim",
                    &[("answers", "reached_two_hops")],
                ),
                (
                    "ruling::he_never_stamped_it",
                    &[("rung", rung), ("ratifies", "a_stamped_claim")],
                ),
            ]);
            assert!(f.is_empty(), "`{rung}` settled a question: {f:?}");
        }
    }

    #[test]
    fn control_a_proposal_answering_a_question_with_nothing_ratifying_it_settles_nothing() {
        // The exclusion `mock/tools/unasked-questions` makes deliberately, and
        // it is correct: a proposal does not settle what it names while it is a
        // proposal. Without this arm the second hop would collapse into
        // following every `proposal.answers`, which is the opposite rule.
        assert!(
            kinds(&[
                ("question::still_open", &[("asks", "?")]),
                ("proposal::a_claim", &[("answers", "still_open")]),
            ])
            .is_empty()
        );
    }

    #[test]
    fn both_hops_reach_the_same_question_and_it_is_reported_once() {
        // A ruling answering a question directly and also ratifying a proposal
        // that answers it is one settling, not two, and a reader given the row
        // twice learns nothing the second time.
        let f = found(&[
            ("question::reached_twice", &[("asks", "?")]),
            ("proposal::a_claim", &[("answers", "reached_twice")]),
            (
                "ruling::he_said_so",
                &[
                    ("rung", "ratified"),
                    ("answers", "reached_twice"),
                    ("ratifies", "a_claim"),
                ],
            ),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
    }

    #[test]
    fn a_ratifies_edge_naming_no_proposal_reaches_nothing_rather_than_panicking() {
        assert!(
            kinds(&[
                ("question::still_open", &[("asks", "?")]),
                (
                    "ruling::he_said_so",
                    &[("rung", "ratified"), ("ratifies", "a_ghost")],
                ),
            ])
            .is_empty()
        );
    }

    #[test]
    fn several_ratified_proposals_on_one_ruling_are_all_followed() {
        // The field is a `string[]` and arrives joined, so a reader taking the
        // whole value as one slug follows none of them. Every arm above plants
        // one entry and would pass through that defect. This is the shape the
        // committed registry actually carries: one ruling stamping four.
        let stamped = cited(&["first", "second"]);
        let f = found(&[
            ("question::one", &[("asks", "?")]),
            ("question::two", &[("asks", "?")]),
            ("proposal::first", &[("answers", "one")]),
            ("proposal::second", &[("answers", "two")]),
            (
                "ruling::the_spine_is_canon",
                &[("rung", "ratified"), ("ratifies", &stamped)],
            ),
        ]);
        assert_eq!(f.len(), 2, "{f:?}");
        assert!(f.iter().any(|(_, m)| m.contains("question::one")), "{f:?}");
        assert!(f.iter().any(|(_, m)| m.contains("question::two")), "{f:?}");
    }

    #[test]
    fn the_three_refusals_are_reported_apart_rather_than_together() {
        // One registry breaking all three, so a lint that folded them into one
        // kind, or dropped one of the three walks, is visible here and nowhere
        // else.
        let k = kinds(&[
            (
                "question::settled_but_silent",
                &[("asks", "?"), ("decider", "op"), ("bound", "returned")],
            ),
            (
                "ruling::he_said_so",
                &[("answers", &cited(&["settled_but_silent", "a_ghost"]))],
            ),
        ]);
        assert_eq!(k.len(), 3, "{k:?}");
        assert!(k.contains(&Some("answers-names-no-question")), "{k:?}");
        assert!(k.contains(&Some("op-decider-carrying-a-bound")), "{k:?}");
        assert!(k.contains(&Some("settled-question-reads-as-open")), "{k:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(kinds(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(
            &[
                ("question::open_looking", &[("asks", "?")]),
                ("ruling::he_said_so", &[("answers", "open_looking")]),
            ],
            &[],
        );
        assert_findings_block(&super::ASettledQuestionDoesNotSitInTheQueue, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::ASettledQuestionDoesNotSitInTheQueue);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(super::ASettledQuestionDoesNotSitInTheQueue.name(), super::LINT);
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered(super::LINT);
    }
}
