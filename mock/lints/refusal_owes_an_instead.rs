//! A refusal owes what happens instead.
//!
//! The `ruling` namespace's own field documentation says the distinction is
//! what the row owes: a refusal owes what is done in place of the refused
//! thing. A refusal with nowhere to go is the shape that gets repealed
//! wholesale by whoever needed the exception once, which costs the refusal and
//! everything decided on the strength of it.
//!
//! **A lint rather than a tool.** There is no legitimate state where a refusal
//! has no answer to "then what": if the honest answer is that nothing replaces
//! it, that sentence is the `instead` and writing it costs one line.
//!
//! **Two namespaces, and the test is the schema rather than the word.** A
//! namespace is read here when it declares `kind` with `refusal` among its
//! values and declares `instead`, which is `ruling` and `proposal` and nothing
//! else. A namespace using the word `refusal` for something of its own means
//! whatever its own schema says, and reporting it would be this lint deciding a
//! rule for a namespace it was not written for. `mechanism` declares no `kind`
//! at all.
//!
//! [`NAMESPACES`] is a written list because nothing a lint is handed carries a
//! field declaration. Not `RegistryView`, which holds rows, their values and
//! the reverse edges, and not any other field of `RepoContext`.
//!
//! One route remains and is refused rather than impossible: `RepoContext`
//! carries `repo_root`, so a lint could read and parse `mockspace.toml` itself.
//! That moves the same parsing problem inside the cdylib and further from a
//! test, since the testkit stubs `repo_root` to `"."`, and it would duplicate a
//! parse the engine has already done and could disagree with it.
//!
//! A written list goes stale in silence, so
//! `canon_lint_testkit::a_lint_naming_the_namespaces_it_reads_agrees_with_the_schema`
//! reads this file and `mockspace.toml` and fails when the two disagree about
//! which namespaces declare `instead`.
//!
//! **That guard did not exist when this paragraph first claimed it did**, which
//! is the shape `a-claim-of-totality-names-what-enforces-it.md` names: a claim
//! of coverage naming an enforcer that was not there, sitting above a list the
//! same paragraph warned would go stale in silence. Two independent readers
//! found it within an hour of each other. It exists now.
//!
//! **What it compares is `instead` and not the `kind`-with-`refusal` half.** A
//! value set is written as an array across several lines and reading it would
//! need the TOML parser the generated pack has no route to depend on. So a
//! namespace that gained `instead` is caught and one that gained `refusal` to an
//! existing `kind` is not, which is stated here rather than left for somebody to
//! discover.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(RefusalOwesAnInstead)
}

/// The namespaces whose schema declares both halves of what this lint asks.
///
/// Read by the schema guard in `canon_lint_testkit.rs`, out of this file's
/// text, so the spelling of this line is load-bearing beyond the compiler.
const NAMESPACES: [&str; 2] = ["proposal", "ruling"];

struct RefusalOwesAnInstead;
impl Lint for RefusalOwesAnInstead {
    fn name(&self) -> &'static str {
        "refusal-owes-an-instead"
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for RefusalOwesAnInstead {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        NAMESPACES
            .iter()
            .flat_map(|ns| ctx.registry.rows_in(ns))
            .filter(|q| ctx.registry.field(q, "kind") == Some("refusal"))
            .filter(|q| {
                ctx.registry
                    .field(q, "instead")
                    .is_none_or(|v| v.trim().is_empty())
            })
            .map(|q| {
                LintError::error(
                    "registry".to_string(),
                    0,
                    "refusal-owes-an-instead",
                    format!(
                        "`{q}` is a refusal and says nothing about what happens instead. A \
                         refusal with nowhere to go gets repealed wholesale by whoever needs \
                         the exception once. Where nothing replaces it, that sentence is the \
                         answer and belongs in the field."
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
    fn row_in(ns: &str, kind: &str, instead: Option<&str>) -> Vec<String> {
        let with = [("kind", kind), ("instead", instead.unwrap_or(""))];
        let without = [("kind", kind)];
        let fields: &[(&str, &str)] = if instead.is_some() { &with } else { &without };
        findings(
            &super::RefusalOwesAnInstead,
            &view(&[(&format!("{ns}::subject"), fields)], &[]),
        )
    }
    fn row(kind: &str, instead: Option<&str>) -> Vec<String> {
        row_in("ruling", kind, instead)
    }
    #[test]
    fn a_refusal_with_no_instead_field_fires_and_names_the_row() {
        let f = row("refusal", None);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("ruling::subject"), "{}", f[0]);
        assert!(f[0].contains("instead"), "{}", f[0]);
    }
    #[test]
    fn control_a_refusal_carrying_an_instead_is_silent() {
        assert!(row("refusal", Some("The owner supplies their own key.")).is_empty());
    }
    #[test]
    fn an_instead_that_is_only_whitespace_is_no_answer_and_fires() {
        // The field being present is not the thing asked for. A row with an
        // empty `instead` satisfies a predicate written as "has the field" and
        // tells a reader nothing, which is the state this lint exists to refuse.
        for blank in ["", " ", "\n", "\t  \n"] {
            let f = row("refusal", Some(blank));
            assert_eq!(f.len(), 1, "{blank:?} passed as an answer");
        }
    }
    #[test]
    fn control_a_row_that_is_not_a_refusal_owes_nothing_whatever_it_carries() {
        // The whole discrimination. A lint firing on a missing `instead` alone
        // would report most of the corpus, and every arm asserting a refusal
        // fires would still pass.
        for kind in ["intent", "ruling", "constraint", ""] {
            assert!(
                row(kind, None).is_empty(),
                "{kind:?} was asked for an instead"
            );
            assert!(row(kind, Some("  ")).is_empty(), "{kind:?}");
        }
    }
    #[test]
    fn a_row_with_no_kind_field_at_all_is_not_a_refusal() {
        let v = view(&[("ruling::subject", &[("says", "something")])], &[]);
        assert!(findings(&super::RefusalOwesAnInstead, &v).is_empty());
    }
    #[test]
    fn a_namespace_that_declares_no_kind_is_left_alone_whatever_a_row_says() {
        // The discrimination, and the reason it is not "every namespace". A
        // namespace using the word `refusal` for something of its own means
        // whatever its own schema says, and reporting it would be the lint
        // deciding a rule for a namespace it was not written for. `mechanism`
        // declares no `kind` field at all, so the value below is a string it
        // has no schema for.
        //
        // This arm fails if the lint ever starts reading every namespace, which
        // is the shape the schema guard in the testkit cannot see, because
        // that one compares two lists and a lint reading all of them has no
        // list to compare.
        let v = view(&[("mechanism::m", &[("kind", "refusal")])], &[]);
        assert!(findings(&super::RefusalOwesAnInstead, &v).is_empty());
    }
    #[test]
    fn a_proposal_refusal_with_no_instead_fires_and_names_the_row() {
        // `proposal` declares `kind` with `refusal` among its values and
        // declares `instead` for it, in the same words the ruling's carries.
        // So this is the same violation in the other namespace, and the lint
        // read one of them for a while.
        let f = row_in("proposal", "refusal", None);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("proposal::subject"), "{}", f[0]);
        assert!(f[0].contains("instead"), "{}", f[0]);
    }
    #[test]
    fn control_a_proposal_refusal_carrying_an_instead_is_silent() {
        assert!(row_in("proposal", "refusal", Some("The panel keeps arguing.")).is_empty());
    }
    #[test]
    fn a_proposal_whose_instead_is_only_whitespace_fires_like_a_ruling_would() {
        for blank in ["", " ", "\n", "\t  \n"] {
            let f = row_in("proposal", "refusal", Some(blank));
            assert_eq!(f.len(), 1, "{blank:?} passed as an answer on a proposal");
        }
    }
    #[test]
    fn control_a_proposal_that_answers_owes_nothing_whatever_it_carries() {
        // The control that separates the widening from a lint reporting every
        // proposal missing an optional field. `instead` is optional in that
        // namespace and every row in the corpus was `kind = "answer"` carrying
        // none when this was written, most of the namespace, so getting it wrong turns
        // the whole namespace red and every arm above still passes. The count
        // is here rather than in the name, where it was a claim this body never
        // makes and would go quietly false on the eleventh.
        assert!(row_in("proposal", "answer", None).is_empty());
        assert!(row_in("proposal", "answer", Some("   ")).is_empty());
    }
    #[test]
    fn every_offending_refusal_is_reported_rather_than_the_first() {
        let v = view(
            &[
                ("ruling::a", &[("kind", "refusal")]),
                ("ruling::b", &[("kind", "refusal")]),
                (
                    "ruling::c",
                    &[("kind", "refusal"), ("instead", "Something else.")],
                ),
            ],
            &[],
        );
        let f = findings(&super::RefusalOwesAnInstead, &v);
        assert_eq!(f.len(), 2, "{f:?}");
        assert!(f.iter().any(|m| m.contains("ruling::a")), "{f:?}");
        assert!(f.iter().any(|m| m.contains("ruling::b")), "{f:?}");
        assert!(!f.iter().any(|m| m.contains("ruling::c")), "{f:?}");
    }
    #[test]
    fn both_namespaces_are_reported_in_one_run_rather_than_the_first_of_them() {
        // The predicate folds two namespaces into one iterator, and a fold that
        // returned only the first would satisfy every arm above: each of those
        // plants one namespace at a time. This is the only arm that can tell.
        let v = view(
            &[
                ("ruling::r", &[("kind", "refusal")]),
                ("proposal::p", &[("kind", "refusal")]),
            ],
            &[],
        );
        let f = findings(&super::RefusalOwesAnInstead, &v);
        assert_eq!(f.len(), 2, "{f:?}");
        assert!(f.iter().any(|m| m.contains("ruling::r")), "{f:?}");
        assert!(f.iter().any(|m| m.contains("proposal::p")), "{f:?}");
    }
    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(findings(&super::RefusalOwesAnInstead, &view(&[], &[])).is_empty());
    }
    #[test]
    fn its_findings_block_every_gate() {
        // The severity that decides a refusal is the one the FINDING carries,
        // not the one `default_severity` returns. Established by mutation: a
        // `default_severity` of `ADVISORY` left the gate printing `[error]`
        // and still refusing, and one `LintError::error` swapped for
        // `LintError::warning` turned the refusal off with the declared
        // default untouched.
        assert_findings_block(
            &super::RefusalOwesAnInstead,
            &view(&[("ruling::subject", &[("kind", "refusal")])], &[]),
        );
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        // The half of the declared default the engine does consult. An `OFF`
        // default that no `[lints]` section overrides skips the lint before
        // its predicate runs, and a dead predicate reports nothing, which
        // reads exactly like a corpus with nothing wrong in it.
        assert_not_declared_off(&super::RefusalOwesAnInstead);
    }
    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::RefusalOwesAnInstead.name(),
            "refusal-owes-an-instead"
        );
    }
    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("refusal-owes-an-instead");
    }
}
