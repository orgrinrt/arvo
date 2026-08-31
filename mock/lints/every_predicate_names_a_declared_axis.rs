//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A predicate names axes somebody declared, in the form the axis declared.
//!
//! The notation is only exact if the vocabulary is closed. Three states per axis
//! and no fourth: listed with a range or `any`, listed with a fixed value, or
//! absent, and absent says the claim holds in no situation where that axis
//! exists at all. **That last reading is what makes the closed set load-bearing
//! rather than tidy**: an axis nobody declared cannot be absent from anything,
//! because nobody knew to look for it, so an undeclared axis silently converts
//! the strongest negative statement in the notation into a shrug.
//!
//! An entry is `<dimension slug>: <values>`. The slug side is checked here. The
//! values side is not, and deliberately: the grammars differ per axis and are
//! prose on the `dimension` row rather than a pattern, because `I in 1..=64`,
//! `operation in {add, mul}` and `target features = host default` have nothing
//! in common a regex would capture without also accepting everything.
//!
//! **A lint rather than a tool.** There is no legitimate reading of an entry
//! naming an axis nothing declares: either the axis exists and is spelled wrong,
//! or it does not exist and the predicate is claiming a region in a vocabulary
//! nobody agreed. Both are fixed by editing the row, and both cost every other
//! predicate in the canon while they stand.
//!
//! Three ways of breaking the one contract, carried as finding kinds so a
//! severity can be set per one of them: an entry nothing can parse, an entry
//! naming an axis nothing declares, and an axis listed with no values at all.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, predicate_entries, slug, split_axis};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(EveryPredicateNamesADeclaredAxis)
}

/// The fields that hold a predicate, and which namespace each belongs to.
///
/// Named rather than discovered, because a field's type is `string[]` and a
/// walker over every `string[]` would check `keywords` and `options` too, and
/// report every bare word in them as a malformed entry.
const PREDICATE_FIELDS: [(&str, &str); 3] = [
    ("proposal", "predicate"),
    ("law", "holds"),
    ("law", "fails"),
];

/// The namespace declaring the closed set of axes.
const DIMENSION: &str = "dimension";

struct EveryPredicateNamesADeclaredAxis;
impl Lint for EveryPredicateNamesADeclaredAxis {
    fn name(&self) -> &'static str {
        "every-predicate-names-a-declared-axis"
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for EveryPredicateNamesADeclaredAxis {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let declared: Vec<&str> = ctx
            .registry
            .rows_in(DIMENSION)
            .iter()
            .map(|q| slug(q))
            .collect();
        let mut out = Vec::new();
        for (namespace, field) in PREDICATE_FIELDS {
            for q in ctx.registry.rows_in(namespace) {
                for entry in predicate_entries(ctx.registry, q, field) {
                    out.extend(check(q, field, entry, &declared));
                }
            }
        }
        out
    }
}

/// One entry, against the declared vocabulary.
fn check(q: &str, field: &str, entry: &str, declared: &[&str]) -> Option<LintError> {
    const LINT: &str = "every-predicate-names-a-declared-axis";
    let Some((axis, values)) = split_axis(entry) else {
        return Some(finding(
            LINT,
            Some("entry-is-malformed"),
            format!(
                "`{q}` carries `{entry}` in `{field}`, which has no `<dimension>: <values>` \
                 split. An entry nothing can parse is a sentence in a field a checker reads, \
                 and it passes every schema."
            ),
        ));
    };
    if !declared.contains(&axis) {
        return Some(finding(
            LINT,
            Some("undeclared-axis"),
            format!(
                "`{q}` names the axis `{axis}` in `{field}`, which no `dimension` row \
                 declares. Declare it, or use the slug of the axis that already means this. \
                 An undeclared axis cannot be absent from any other predicate, so admitting \
                 one here weakens every predicate in the canon."
            ),
        ));
    }
    if values.is_empty() {
        return Some(finding(
            LINT,
            Some("entry-has-no-values"),
            format!(
                "`{q}` names `{axis}` in `{field}` and gives it nothing. An axis listed with \
                 no values is neither of the two positive states the notation has, and it \
                 reads as a region while naming none."
            ),
        ));
    }
    None
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use crate::canon_lint_testkit::{
        assert_findings_block, assert_not_declared_off, assert_registered, ctx, view,
    };
    use crate::canon_rows::JOIN;

    /// The findings, with the kind each carries, so an arm can assert which of
    /// the three refusals fired rather than only that something did.
    fn findings(rows: &[(&str, &[(&str, &str)])]) -> Vec<(Option<&'static str>, String)> {
        use mockspace::RepoLint;
        let v = view(rows, &[]);
        super::EveryPredicateNamesADeclaredAxis
            .check_repo(&ctx(&v))
            .into_iter()
            .map(|e| (e.finding_kind, e.message))
            .collect()
    }

    /// A predicate written as the row wrote it, joined the way the engine joins
    /// one before a lint ever sees it.
    fn predicate(entries: &[&str]) -> String {
        entries.join(JOIN)
    }

    #[test]
    fn an_axis_nothing_declares_is_reported_and_named() {
        let p = predicate(&["fraction_width: 0", "phase_of_the_moon: waxing"]);
        let f = findings(&[
            (
                "dimension::fraction_width",
                &[("what", "bits below the point")],
            ),
            ("proposal::a_claim", &[("predicate", &p)]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].0, Some("undeclared-axis"));
        assert!(f[0].1.contains("phase_of_the_moon"), "{}", f[0].1);
        assert!(f[0].1.contains("proposal::a_claim"), "{}", f[0].1);
    }

    #[test]
    fn an_entry_with_no_axis_and_values_split_is_a_different_refusal() {
        // A sentence with no colon is a malformed entry rather than an unknown
        // axis, and the two want different fixes: one is a typo and the other
        // is an axis to declare. Reporting both under one kind would put them
        // in one bucket a severity cannot separate.
        let f = findings(&[
            ("dimension::fraction_width", &[("what", "bits")]),
            (
                "proposal::a_claim",
                &[("predicate", "mostly small fractions")],
            ),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].0, Some("entry-is-malformed"), "{f:?}");
    }

    #[test]
    fn an_axis_listed_with_nothing_is_reported() {
        let f = findings(&[
            ("dimension::threads", &[("what", "how many threads")]),
            ("proposal::a_claim", &[("predicate", "threads:")]),
        ]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].0, Some("entry-has-no-values"), "{f:?}");
    }

    #[test]
    fn control_a_declared_axis_carrying_values_is_silent() {
        let p = predicate(&["threads: 1", "fraction_width: F = 0"]);
        let f = findings(&[
            ("dimension::threads", &[("what", "how many")]),
            ("dimension::fraction_width", &[("what", "bits")]),
            ("proposal::a_claim", &[("predicate", &p)]),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn a_laws_failing_region_is_read_as_well_as_its_holding_one() {
        // Easy to get wrong by wiring the walk to `predicate` alone and never
        // noticing, because a law's `holds` would still be read and every arm
        // above would pass.
        let f = findings(&[(
            "law::associativity",
            &[
                ("holds", "nothing_declares_this: everywhere"),
                ("fails", "nor_this: anywhere"),
            ],
        )]);
        assert_eq!(
            f.len(),
            2,
            "both `holds` and `fails` name an undeclared axis here, and a walk reading only \
             one reports one: {f:?}"
        );
        assert!(
            f.iter().any(|(_, m)| m.contains("nothing_declares_this")),
            "{f:?}"
        );
        assert!(f.iter().any(|(_, m)| m.contains("nor_this")), "{f:?}");
    }

    #[test]
    fn control_a_keywords_list_is_not_read_as_a_predicate() {
        // `keywords` is a list of bare words with no colon in them, so a walker
        // over every array field would report each one as malformed and the
        // report would be noise nobody reads.
        let f = findings(&[(
            "proposal::a_claim",
            &[("keywords", "width, signedness, not a predicate entry")],
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn an_entry_whose_values_carry_the_separator_is_not_cut_in_two() {
        // The whole reason the predicate reader exists. Every one of these
        // entries names a declared axis, and a lint splitting the joined field
        // on the separator alone would report `signed}` and `100 of them` as
        // malformed entries: two findings against a row with nothing wrong in
        // it, on the shape most of the canon's real predicates take.
        let p = predicate(&[
            "signedness: signedness in {unsigned, signed}",
            "container: interval numerals containing zero, 100 of them",
        ]);
        let f = findings(&[
            ("dimension::signedness", &[("what", "sign")]),
            ("dimension::container", &[("what", "shape")]),
            ("proposal::a_claim", &[("predicate", &p)]),
        ]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn every_offending_entry_is_reported_rather_than_the_first_in_the_row() {
        let p = predicate(&["no_such_axis: x", "nor_this_one: y"]);
        let f = findings(&[("proposal::a_claim", &[("predicate", &p)])]);
        assert_eq!(f.len(), 2, "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(findings(&[]).is_empty());
    }

    #[test]
    fn control_a_row_with_no_predicate_at_all_owes_nothing() {
        let f = findings(&[("proposal::a_claim", &[("says", "something")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn its_findings_block_every_gate() {
        // The severity that decides a refusal is the one the FINDING carries,
        // not the one `default_severity` returns.
        let v = view(&[("proposal::a", &[("predicate", "no_such_axis: x")])], &[]);
        assert_findings_block(&super::EveryPredicateNamesADeclaredAxis, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::EveryPredicateNamesADeclaredAxis);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::EveryPredicateNamesADeclaredAxis.name(),
            "every-predicate-names-a-declared-axis"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("every-predicate-names-a-declared-axis");
    }
}
