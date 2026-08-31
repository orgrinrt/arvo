//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A row nobody will find, because it uses none of the words they will search
//! for.
//!
//! Cheap, and it has already cost this project the same question twice: a row
//! existed, answered the question, and could not be found, because the asker
//! reached for a different word than the author had.
//!
//! **A lint rather than a tool.** One field, one edit, and the refused state is
//! exact: the row is in a namespace a reader searches and carries nothing to
//! search it by. Nothing here is a ranking of how findable a row is, which
//! would be a tool and would want a threshold nobody can justify.
//!
//! **Only the namespaces a reader searches.** A `dimension` or a `topic` is
//! found by enumeration, because the whole of each is short and a reader reads
//! all of it. `probe` joined the list late, reported by the seat that filled
//! it: a reader hunting for the instrument behind a figure searches for what it
//! measured, and a probe row is exactly as unfindable without keywords as any
//! other.
//!
//! **What the port loses.** The crate this came from asserted the committed
//! canon leaves no row unfindable. A unit test cannot build a `RegistryView`
//! from the real registry, since that needs a TOML parser the generated pack
//! may not depend on, so that arm is `cargo mock --lint-only` over the real
//! rows now.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, list};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(ARowCarriesKeywords)
}

const LINT: &str = "a-row-carries-keywords";

/// The namespaces a reader reaches by searching rather than by reading whole.
const SEARCHED: [&str; 6] = [
    "ruling",
    "proposal",
    "question",
    "obligation",
    "retirement",
    "probe",
];

struct ARowCarriesKeywords;
impl Lint for ARowCarriesKeywords {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for ARowCarriesKeywords {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        SEARCHED
            .iter()
            .flat_map(|ns| ctx.registry.rows_in(ns))
            .filter(|q| list(ctx.registry, q, "keywords").is_empty())
            .map(|q| {
                finding(
                    LINT,
                    None,
                    format!(
                        "`{q}` carries no `keywords`. A search over row text finds whichever \
                         word the author reached for, and the reader reaches for a different \
                         one."
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
        findings(&super::ARowCarriesKeywords, &view(rows, &[]))
    }

    #[test]
    fn a_row_with_no_keywords_is_reported_only_in_the_namespaces_a_reader_searches() {
        let f = found(&[
            ("ruling::unfindable", &[("says", "something")]),
            ("dimension::found_by_enumeration", &[("what", "an axis")]),
            ("ruling::findable", &[("keywords", "width, carrier")]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("ruling::unfindable"), "{}", f[0]);
        assert!(f[0].contains("keywords"), "{}", f[0]);
    }

    #[test]
    fn all_six_searched_namespaces_are_read_in_one_run() {
        // The flat_map folds six namespaces into one iterator, and a fold that
        // returned only the first would satisfy the arm above, which plants
        // one. This is the only arm that can tell.
        let rows: Vec<(&str, &[(&str, &str)])> = vec![
            ("ruling::a", &[("says", "x")]),
            ("proposal::b", &[("says", "x")]),
            ("question::c", &[("asks", "x")]),
            ("obligation::d", &[("owes", "x")]),
            ("retirement::e", &[("claim", "x")]),
            ("probe::f", &[("standing", "sound")]),
        ];
        let f = found(&rows);
        assert_eq!(f.len(), 6, "{f:?}");
        for who in [
            "ruling::a",
            "proposal::b",
            "question::c",
            "obligation::d",
            "retirement::e",
            "probe::f",
        ] {
            assert!(
                f.iter().any(|m| m.contains(who)),
                "{who} was not read: {f:?}"
            );
        }
    }

    #[test]
    fn a_namespace_found_by_enumeration_is_left_alone() {
        // The discrimination, and the reason the list is written rather than
        // "every namespace". A reader reads the whole of these, so keywords buy
        // nothing and reporting them would turn most of the registry red.
        let f = found(&[
            ("dimension::d", &[("what", "an axis")]),
            ("topic::t", &[("what", "a subject")]),
            ("law::l", &[("holds", "fraction_width: 0")]),
            ("strategy::s", &[("what", "a weighting")]),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn a_keywords_field_present_and_holding_nothing_is_no_keywords() {
        // The engine writes an empty array as the empty string, so a row that
        // carries the field and fills it with nothing arrives looking exactly
        // like one that never wrote it, and neither can be searched.
        for blank in ["", " ", ", ", "  ,  "] {
            let f = found(&[("ruling::r", &[("keywords", blank)])]);
            assert_eq!(f.len(), 1, "{blank:?} passed as keywords: {f:?}");
        }
    }

    #[test]
    fn one_keyword_is_enough() {
        // The refused state is nothing to search by, not too little. A count
        // threshold here would be a number nobody justified, which is what
        // makes this a lint rather than a ranking.
        let f = found(&[("ruling::r", &[("keywords", "width")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(found(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let v = view(&[("ruling::r", &[("says", "something")])], &[]);
        assert_findings_block(&super::ARowCarriesKeywords, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::ARowCarriesKeywords);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(super::ARowCarriesKeywords.name(), "a-row-carries-keywords");
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("a-row-carries-keywords");
    }
}
