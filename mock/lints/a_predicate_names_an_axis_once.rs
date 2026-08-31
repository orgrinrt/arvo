//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! One axis named twice in one predicate is two regions and no rule.
//!
//! A predicate is exact because each axis is in exactly one of three states:
//! listed with a range or `any`, listed with a fixed value, or absent. Two
//! entries for one axis is neither of the two positive states and is not
//! absence either. It is two regions with nothing saying which governs, and a
//! reader gating an arm on it has to pick, which is the reader deciding what
//! the claim was.
//!
//! The schema cannot see it: a `string[]` with two entries is a valid array
//! whatever the entries say. `every-predicate-names-a-declared-axis` cannot
//! see it either, because both entries name a declared axis and carry values,
//! so each is well formed on its own.
//!
//! **A lint rather than a tool, and a hard zero.** There is no legitimate
//! reading of two regions on one axis, and the repair is one edit: write the
//! one region the claim was established in.
//!
//! # Reading the entries back out of the joined field
//!
//! A `string[]` arrives joined with `", "`, and a predicate's values side is
//! prose that carries the separator freely, so splitting on it alone would cut
//! `signedness: signedness in {unsigned, signed}` in two. `predicate_entries`
//! is the reader that recovers the boundary, and its residue is stated at that
//! function. What that residue costs here is a false report rather than a
//! missed one, which is the direction to fail in.
//!
//! **How much that reader buys this particular question is small, and saying so
//! is the honest version.** A naive split adds fragments, and a fragment either
//! has no colon, so it is skipped, or opens with a slug and a colon, which is
//! the one place `predicate_entries` splits as well. So the two readers agree
//! on every shape the canon carries today, and they stop agreeing the moment a
//! values side spells out a slug and a colon of its own. The reader is used
//! because that day arrives silently and because a second spelling of the
//! boundary is a second thing to keep true.
//!
//! # What the unit tests here cannot ask
//!
//! That the committed canon names no axis twice. A unit test cannot build a
//! `RegistryView` from `mock/registry/`, because that needs a TOML parser the
//! generated pack has no route to depend on. `cargo mock --lint-only` is where
//! the predicate meets the real rows, and it runs this over all of them at
//! every gate.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::{finding, predicate_entries, split_axis};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(APredicateNamesAnAxisOnce)
}

const LINT: &str = "a-predicate-names-an-axis-once";

/// The fields that hold a predicate, and which namespace each belongs to.
///
/// Named rather than discovered, because a field's type is `string[]` and a
/// walker over every `string[]` would check `keywords` and `options` too, and
/// a repeated bare word in one of those is not a repeated axis.
///
/// The same three pairs are spelled in `every_predicate_names_a_declared_axis.rs`.
/// They belong beside `predicate_entries` in `canon_rows.rs`, which is the
/// reader written for them, and moving them there is an edit to a shared module
/// this port does not make.
const PREDICATE_FIELDS: [(&str, &str); 3] = [
    ("proposal", "predicate"),
    ("law", "holds"),
    ("law", "fails"),
];

struct APredicateNamesAnAxisOnce;
impl Lint for APredicateNamesAnAxisOnce {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for APredicateNamesAnAxisOnce {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let mut out = Vec::new();
        for (namespace, field) in PREDICATE_FIELDS {
            for q in ctx.registry.rows_in(namespace) {
                let mut seen: Vec<&str> = Vec::new();
                for entry in predicate_entries(ctx.registry, q, field) {
                    // An entry with no colon is the other lint's report to
                    // make. Reporting it here too would put one defect in two
                    // buckets a severity cannot separate.
                    let Some((axis, _)) = split_axis(entry) else {
                        continue;
                    };
                    if seen.contains(&axis) {
                        out.push(finding(
                            LINT,
                            None,
                            format!(
                                "`{q}` names `{axis}` more than once in `{field}`. Two \
                                 regions on one axis and nothing says which holds; write \
                                 the one region the claim was established in."
                            ),
                        ));
                    } else {
                        seen.push(axis);
                    }
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use mockspace::{Lint, RepoLint};

    use crate::canon_lint_testkit::{
        assert_findings_block, assert_not_declared_off, assert_registered, ctx, view,
    };
    use crate::canon_rows::JOIN;

    fn predicate(entries: &[&str]) -> String {
        entries.join(JOIN)
    }

    fn findings(rows: &[(&str, &[(&str, &str)])]) -> Vec<String> {
        let v = view(rows, &[]);
        super::APredicateNamesAnAxisOnce
            .check_repo(&ctx(&v))
            .into_iter()
            .map(|e| e.message)
            .collect()
    }

    #[test]
    fn one_axis_named_twice_is_reported() {
        let p = predicate(&["signedness: unsigned", "signedness: signed"]);
        let f = findings(&[("law::distributivity", &[("holds", &p)])]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("signedness"), "{}", f[0]);
        assert!(f[0].contains("law::distributivity"), "{}", f[0]);
    }

    /// The control. Two different axes is the ordinary shape of every predicate
    /// in the canon, and a lint refusing it would refuse the notation itself.
    #[test]
    fn control_two_different_axes_are_silent() {
        let p = predicate(&["signedness: unsigned", "fraction_width: F = 0"]);
        let f = findings(&[("law::distributivity", &[("holds", &p)])]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// An axis named three times is two repeats and reports twice.
    ///
    /// The count matters: reporting once per row would hide how far the row is
    /// from the one region it was meant to state.
    #[test]
    fn a_third_naming_is_a_second_finding() {
        let p = predicate(&["threads: 1", "threads: 2", "threads: any"]);
        let f = findings(&[("proposal::a_claim", &[("predicate", &p)])]);
        assert_eq!(f.len(), 2, "{f:?}");
    }

    /// `holds` and `fails` are counted apart, and both are read.
    ///
    /// An axis in each is the ordinary shape of a law: the region it holds in
    /// and the region it fails in are two claims about one axis, which is the
    /// notation working rather than a repeat. A predicate reading both fields
    /// into one set would report every well-formed law in the canon.
    #[test]
    fn control_one_axis_in_holds_and_in_fails_is_not_a_repeat() {
        let f = findings(&[(
            "law::associativity",
            &[
                ("holds", "fraction_width: F = 0"),
                ("fails", "fraction_width: F > 0"),
            ],
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// And a repeat inside `fails` alone is still caught, which is the half a
    /// walk wired to `holds` would miss while every arm above stayed green.
    #[test]
    fn a_repeat_in_the_failing_region_is_reported() {
        let p = predicate(&["signedness: unsigned", "signedness: signed"]);
        let f = findings(&[("law::associativity", &[("fails", &p)])]);
        assert_eq!(f.len(), 1, "{f:?}");
    }

    /// An entry whose values carry the separator is one entry.
    ///
    /// The shape most of the canon's real predicates take. A split on the
    /// separator alone would cut this into four, and the axis would then appear
    /// once with three tails naming nothing, so nothing would be reported here
    /// either. The arm that follows is what makes this one mean something.
    #[test]
    fn control_an_entry_whose_values_carry_the_separator_is_not_cut_in_two() {
        let p = predicate(&[
            "signedness: signedness in {unsigned, signed}",
            "container: interval numerals containing zero, 100 of them",
        ]);
        let f = findings(&[("proposal::a_claim", &[("predicate", &p)])]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// And the repeat is still found when the values carry the separator.
    ///
    /// A reader that gave up on the boundary and treated the whole field as one
    /// entry would pass the arm above and fail this one.
    #[test]
    fn a_repeat_is_found_through_values_that_carry_the_separator() {
        let p = predicate(&[
            "signedness: signedness in {unsigned, signed}",
            "container: interval numerals containing zero, 100 of them",
            "signedness: unsigned",
        ]);
        let f = findings(&[("proposal::a_claim", &[("predicate", &p)])]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("signedness"), "{}", f[0]);
    }

    /// A malformed entry is the other lint's report and is skipped here.
    #[test]
    fn control_an_entry_with_no_colon_is_left_to_the_other_lint() {
        let f = findings(&[(
            "proposal::a_claim",
            &[("predicate", "mostly small fractions")],
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// `keywords` is a list of bare words, and a repeated one is not a repeated
    /// axis. A walker over every array field would report it and the report
    /// would be noise nobody reads.
    #[test]
    fn control_a_keywords_list_is_not_read_as_a_predicate() {
        let f = findings(&[(
            "proposal::a_claim",
            &[("keywords", "width, width, signedness")],
        )]);
        assert!(f.is_empty(), "{f:?}");
    }

    /// An axis undeclared by any `dimension` row is still an axis for this
    /// question. Naming a wrong axis twice is two regions the same way naming a
    /// right one twice is, and the undeclared half is the other lint's report.
    #[test]
    fn a_repeat_of_an_undeclared_axis_is_still_a_repeat() {
        let p = predicate(&["phase_of_the_moon: waxing", "phase_of_the_moon: waning"]);
        let f = findings(&[("proposal::a_claim", &[("predicate", &p)])]);
        assert_eq!(f.len(), 1, "{f:?}");
    }

    #[test]
    fn control_a_row_with_no_predicate_owes_nothing() {
        let f = findings(&[("proposal::a_claim", &[("says", "something")])]);
        assert!(f.is_empty(), "{f:?}");
    }

    #[test]
    fn control_an_empty_registry_is_silent_rather_than_a_panic() {
        assert!(findings(&[]).is_empty());
    }

    #[test]
    fn its_findings_block_every_gate() {
        let p = predicate(&["threads: 1", "threads: 2"]);
        let v = view(&[("proposal::a", &[("predicate", &p)])], &[]);
        assert_findings_block(&super::APredicateNamesAnAxisOnce, &v);
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&super::APredicateNamesAnAxisOnce);
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            super::APredicateNamesAnAxisOnce.name(),
            "a-predicate-names-an-axis-once"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("a-predicate-names-an-axis-once");
    }
}
