//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A definition that does not say what it defines.
//!
//! `defines` is optional on the namespace, because most rows in it are not
//! definitions. On a row that is one, the field is what the row is for: without
//! it nothing says which term is being stipulated, and nothing can tell whether
//! the term is stipulated twice.
//!
//! **A lint rather than a tool.** One field, one edit, one refused state, and
//! the row's author is the only person who knows the answer, which is not the
//! same as a question a runner has to be asked.
//!
//! **It pairs with `a-term-is-defined-twice` and the two do not overlap.** That
//! one skips a row with no term, because a row defining nothing collides with
//! nothing. This one reports it. Between them every definition either names a
//! term or is reported here, which is what makes the collision check's
//! selection complete rather than merely non-empty.
//!
//! **What the port loses.** The crate this came from asserted the committed
//! canon holds no definition without a term. A unit test cannot build a
//! `RegistryView` from the real registry, since that needs a TOML parser the
//! generated pack may not depend on, so that arm is `cargo mock --lint-only`
//! over the real rows now.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, text};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ADefinitionNamesItsTerm)
}

const LINT: &str = "a-definition-names-its-term";

/// The namespace whose rows carry `sentence_kind` and `defines`.
const NAMESPACE: &str = "proposal";

/// The kind of sentence that stipulates a term.
const DEFINITION: &str = "definition";

struct ADefinitionNamesItsTerm;
impl Lint for ADefinitionNamesItsTerm {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for ADefinitionNamesItsTerm {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        ctx.registry
            .rows_in(NAMESPACE)
            .iter()
            .filter(|q| text(ctx.registry, q, "sentence_kind") == Some(DEFINITION))
            .filter(|q| text(ctx.registry, q, "defines").is_none())
            .map(|q| {
                finding(
                    LINT,
                    None,
                    format!(
                        "`{q}` has `sentence_kind` `definition` and an empty `defines`, so \
                         nothing says which term is being stipulated and nothing can tell \
                         whether it is stipulated twice."
                    ),
                )
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use crate::canon_lint_testkit::{
        assert_findings_block, assert_not_declared_off, assert_registered, findings, view,
    };

    fn found(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        findings(&super::ADefinitionNamesItsTerm, &view(rows, &[]))
    }

    #[test]
    fn a_definition_that_says_nothing_about_what_it_defines_is_reported() {
        let f = found(&[
            (
                "proposal::a_stipulation_of_nothing",
                &[("sentence_kind", "definition")],
            ),
            (
                "proposal::a_stipulation_of_something",
                &[("sentence_kind", "definition"), ("defines", "chain")],
            ),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("a_stipulation_of_nothing"), "{}", f[0]);
        assert!(f[0].contains("defines"), "{}", f[0]);
    }

    #[test]
    fn a_defines_present_and_blank_names_no_term() {
        // The field being present is not what was asked for, and a row carrying
        // a blank one tells a reader exactly what a missing one does.
        for blank in ["", " ", "\n", "\t  \n"] {
            let f = found(&[(
                "proposal::p",
                &[("sentence_kind", "definition"), ("defines", blank)],
            )]);
            assert_eq!(f.len(), 1, "{blank:?} passed as a term: {f:?}");
        }
    }

    #[test]
    fn control_a_row_that_is_not_a_definition_owes_no_term() {
        // The whole discrimination. `defines` is optional across the namespace,
        // so a lint keyed on the missing field alone would report nearly all of
        // it, and the arm above would still pass.
        for kind in [
            "measured",
            "normative",
            "theorem",
            "argument",
            "enumeration",
            "",
        ] {
            let f = found(&[("proposal::p", &[("sentence_kind", kind)])]);
            assert!(f.is_empty(), "{kind:?} was asked for a term: {f:?}");
        }
    }

    #[test]
    fn a_row_with_no_sentence_kind_at_all_is_not_a_definition() {
        let f = found(&[("proposal::p", &[("says", "something")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn a_ruling_is_not_read_whatever_it_carries() {
        // Only `proposal` declares the pair. Reading every namespace would be
        // deciding a rule for namespaces this was not written for, and every
        // arm above plants one namespace so none of them can tell.
        let f = found(&[("ruling::r", &[("sentence_kind", "definition")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn every_offending_definition_is_reported_rather_than_the_first() {
        let f = found(&[
            ("proposal::a", &[("sentence_kind", "definition")]),
            ("proposal::b", &[("sentence_kind", "definition")]),
        ]);
        assert_eq!(f.len(), 2, "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(found(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(&[("proposal::p", &[("sentence_kind", "definition")])], &[]);
        assert_findings_block(&super::ADefinitionNamesItsTerm, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::ADefinitionNamesItsTerm);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::ADefinitionNamesItsTerm.name(),
            "a-definition-names-its-term"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("a-definition-names-its-term");
    }
}
