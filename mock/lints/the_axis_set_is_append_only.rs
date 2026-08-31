//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! An axis that was declared and is no longer there.
//!
//! `dimension.toml`'s own header says the set is append-only and that a check
//! pins it rather than the sentence asking. **It said that twice and no check
//! existed**, which is the shape a claim of totality takes when nobody names
//! the mechanism behind it.
//!
//! What is actually at stake is narrow and worth stating exactly. Adding an
//! axis reaches nobody: a predicate's absence quantifies over the world rather
//! than over this file, so a row written before an axis was declared was always
//! as narrow as it now reads. **Deleting or renaming one is what rewrites
//! meaning**, because a span written over that axis becomes unparseable and a
//! written absence becomes nonsense, and both fail silently: the corpus still
//! reads, and the sentence it now says is a different one.
//!
//! **The mechanism is a roster**, and it is read in both directions. Every id
//! that has ever been declared is written down here, and each must still
//! resolve to a row, which is the arm that refuses a deletion. Every row in the
//! namespace must appear on the roster, which is the arm that keeps the first
//! one complete. A rename trips both, the deletion half naming what stopped
//! resolving and the addition half naming what nothing would catch next time.
//!
//! The second arm is there because the first one alone is only append-only over
//! the part somebody remembered to list. An axis declared and never rostered is
//! invisible to it in either direction, so its deletion months later is silent,
//! and the omission happens at the one moment nobody is thinking about
//! deletion, which is the commit that declares the axis. Read the two arms
//! together and the roster is complete by construction rather than by memory.
//!
//! **No ceiling and no threshold.** The refused state is not a population that
//! grew, it is one entry that stopped resolving or one row nobody listed, and
//! there is no number to grandfather because the committed registry satisfies
//! both arms exactly today.
//!
//! **Growing the roster is what an addition costs**, one line, in the commit
//! that declares the row. Shrinking it is the thing to refuse: an author who
//! deletes an axis and deletes its roster entry to match has made the lint
//! green by removing the check, so the roster is append-only for the same
//! reason the registry is.
//!
//! # What the unit tests here cannot ask
//!
//! That the shipped roster matches the committed registry. A unit test cannot
//! build a `RegistryView` from `mock/registry/`, because that needs a TOML
//! parser the generated pack has no route to depend on. `cargo mock
//! --lint-only` is where the roster meets the real rows, and it runs this at
//! every gate. What the tests below do instead is drive the whole predicate on
//! planted rosters and planted registries, including the case that must fire.
//!
//! **It was driven against them once, deliberately.** Renaming `radix` in the
//! committed registry produces this arm naming `radix` as gone, and produces two
//! `every-predicate-names-a-declared-axis` errors in the same run, which is the
//! harm this describes arriving on cue: two predicates written over that axis
//! stopped parsing the moment it was renamed. That check reports the symptom and
//! this one reports the cause.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_rows::finding;
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(TheAxisSetIsAppendOnly { roster: ROSTER })
}

const LINT: &str = "the-axis-set-is-append-only";

/// The namespace whose rows are the axes.
const NAMESPACE: &str = "dimension";

/// Every axis id that has ever been declared.
///
/// **Append only.** A new axis adds a line here in the same commit that adds
/// the row. Removing a line is removing the check, not satisfying it.
const ROSTER: &[&str] = &[
    "integer_width",
    "fraction_width",
    "total_width",
    "signedness",
    "overflow_policy",
    "rounding",
    "operation",
    "arity",
    "chain_length",
    "target_features",
    "threads",
    "container",
    "alignment",
    "access_pattern",
    "strategy",
    "radix",
    "ambient_domain",
    "accumulator_width",
    "build_profile",
    "toolchain",
    "operand_window",
    "occupancy",
    "association",
    "leaf_aliasing",
];

/// The lint, carrying the roster it pins.
///
/// A field rather than the constant read inside the predicate, so a test can
/// drive it with a planted roster against a planted registry. A roster only
/// ever exercised against the rows it was copied from is a roster nobody has
/// seen fire.
struct TheAxisSetIsAppendOnly {
    roster: &'static [&'static str],
}
impl Lint for TheAxisSetIsAppendOnly {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for TheAxisSetIsAppendOnly {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let rows = ctx.registry.rows_in(NAMESPACE);
        let gone: Vec<&str> = self
            .roster
            .iter()
            .copied()
            .filter(|id| {
                let qualified = format!("{NAMESPACE}::{id}");
                !rows.iter().any(|q| *q == qualified.as_str())
            })
            .collect();
        let unrostered: Vec<&str> = rows
            .iter()
            .filter_map(|q| q.strip_prefix(NAMESPACE).and_then(|r| r.strip_prefix("::")))
            .filter(|id| !self.roster.contains(id))
            .collect();

        let mut out = Vec::new();
        if !gone.is_empty() {
            out.push(finding(
                LINT,
                None,
                format!(
                    "{} declared axes no longer resolve to a row: {gone:?}. The set is \
                     append-only, because a span written over an axis becomes unparseable when \
                     the axis goes and a written absence becomes nonsense, and both fail \
                     silently. Restore the row under its original id. A rename is a deletion and \
                     needs the old id kept, or every predicate written in it stops parsing. \
                     Deleting the roster entry instead removes the check rather than satisfying \
                     it.",
                    gone.len()
                ),
            ));
        }
        if !unrostered.is_empty() {
            out.push(finding(
                LINT,
                None,
                format!(
                    "{} axes are declared and not rostered: {unrostered:?}. Nothing would catch \
                     their deletion, so the arm above cannot see them and the set is append-only \
                     only over the part somebody remembered to list. Add each id to `ROSTER` in \
                     the commit that declares it, which is the one moment nobody is thinking \
                     about deletion.",
                    unrostered.len()
                ),
            ));
        }
        out
    }
}
#[cfg(test)]
mod tests {
    use mockspace::Lint;

    use super::TheAxisSetIsAppendOnly as AppendOnly;
    use crate::canon_lint_testkit::{
        assert_findings_block, assert_not_declared_off, assert_registered, findings, view,
    };

    /// One planted registry read against one planted roster.
    fn found(rows: &[(&str, &[(&str, &str)])], roster: &'static [&'static str]) -> Vec<String> {
        findings(&AppendOnly { roster }, &view(rows, &[]))
    }

    /// A registry holding exactly these axis ids and nothing else.
    fn axes(ids: &[&str]) -> Vec<(String, Vec<(String, String)>)> {
        ids.iter()
            .map(|id| {
                (
                    format!("dimension::{id}"),
                    vec![("what".to_string(), "an axis".to_string())],
                )
            })
            .collect()
    }

    /// The borrowing dance the planted-view helper wants.
    fn plant(ids: &[&str]) -> Vec<String> {
        let owned = axes(ids);
        let borrowed: Vec<(&str, Vec<(&str, &str)>)> = owned
            .iter()
            .map(|(k, fs)| {
                (
                    k.as_str(),
                    fs.iter().map(|(f, v)| (f.as_str(), v.as_str())).collect(),
                )
            })
            .collect();
        let rows: Vec<(&str, &[(&str, &str)])> =
            borrowed.iter().map(|(k, fs)| (*k, fs.as_slice())).collect();
        found(&rows, ROSTER_TWO)
    }

    const ROSTER_TWO: &[&str] = &["radix", "signedness"];

    #[test]
    fn a_registry_holding_every_rostered_axis_is_silent() {
        assert!(plant(&["radix", "signedness"]).is_empty());
    }

    #[test]
    fn a_rostered_axis_that_is_gone_is_named() {
        let f = plant(&["radix"]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("signedness"), "{}", f[0]);
        assert!(
            !f[0].contains("radix"),
            "the surviving axis was named: {}",
            f[0]
        );
    }

    #[test]
    fn every_missing_axis_is_named_rather_than_the_first() {
        let f = plant(&[]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("radix"), "{}", f[0]);
        assert!(f[0].contains("signedness"), "{}", f[0]);
        assert!(f[0].contains('2'), "the count is missing: {}", f[0]);
    }

    #[test]
    fn an_axis_declared_and_not_rostered_is_reported_as_unprotected() {
        // Growth is still free in the sense that matters: an addition is not
        // refused and nothing has to be undone. What it is not is invisible.
        // A roster the deletion arm reads is append-only only over the part
        // somebody remembered to list, so an unrostered axis could be deleted
        // later in silence. This says so at the one moment the omission is
        // cheap to repair, which is the commit that declares it.
        let f = plant(&["radix", "signedness", "an_axis_declared_later"]);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("an_axis_declared_later"), "{}", f[0]);
        assert!(
            f[0].contains("not rostered"),
            "the unprotected arm did not fire; this may be the deletion arm: {}",
            f[0]
        );
        assert!(
            !f[0].contains("no longer resolve"),
            "an addition was reported as a deletion: {}",
            f[0]
        );
    }

    #[test]
    fn a_rename_fires_on_both_halves_and_says_which_is_which() {
        // A rename is a deletion and an addition, and both arms now have
        // something to say about it. The deletion is what turns a written span
        // unparseable; the addition is what nothing would catch next time.
        let f = plant(&["radix", "signedness_but_renamed"]);
        assert_eq!(f.len(), 2, "both halves are reported: {f:?}");
        let gone = f
            .iter()
            .find(|m| m.contains("no longer resolve"))
            .unwrap_or_else(|| panic!("no deletion finding: {f:?}"));
        assert!(gone.contains("signedness"), "{gone}");
        assert!(gone.contains("rename"), "the repair is not stated: {gone}");
        let added = f
            .iter()
            .find(|m| m.contains("not rostered"))
            .unwrap_or_else(|| panic!("no unprotected finding: {f:?}"));
        assert!(added.contains("signedness_but_renamed"), "{added}");
    }

    #[test]
    fn a_row_in_another_namespace_does_not_satisfy_the_roster() {
        // The discrimination the predicate rests on. A check written over
        // every namespace's rows would count `ruling::signedness` as the axis
        // and pass, and every arm above plants one namespace so none can tell.
        let f = found(
            &[
                ("dimension::radix", &[("what", "an axis")]),
                ("ruling::signedness", &[("says", "something")]),
            ],
            ROSTER_TWO,
        );
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("signedness"), "{}", f[0]);
    }

    #[test]
    fn an_axis_whose_id_merely_contains_a_rostered_one_does_not_satisfy_it() {
        // A predicate written with `contains` rather than equality passes on
        // `dimension::signedness_of_the_accumulator`, which is a different
        // axis and leaves every span written over the old id unparseable.
        // Both arms discriminate, and both have to: the deletion arm must not
        // read the longer id as the rostered one, and the reverse arm must not
        // read the rostered one as covering the longer id.
        let f = plant(&["radix", "signedness_of_the_accumulator"]);
        assert_eq!(f.len(), 2, "{f:?}");
        let gone = f
            .iter()
            .find(|m| m.contains("no longer resolve"))
            .unwrap_or_else(|| panic!("the longer id was read as the rostered axis: {f:?}"));
        assert!(gone.contains("\"signedness\""), "{gone}");
        let added = f
            .iter()
            .find(|m| m.contains("not rostered"))
            .unwrap_or_else(|| panic!("the rostered id was read as covering the longer one: {f:?}"));
        assert!(added.contains("signedness_of_the_accumulator"), "{added}");
    }

    #[test]
    fn control_an_empty_registry_names_the_whole_roster_rather_than_panicking() {
        let f = found(&[], ROSTER_TWO);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(
            f[0].contains("radix") && f[0].contains("signedness"),
            "{}",
            f[0]
        );
    }

    #[test]
    fn an_empty_roster_is_loud_rather_than_vacuously_silent() {
        // This arm used to assert the opposite, and pinning that was the
        // defect: with only the deletion arm, an empty roster passed over any
        // registry at all, so the emptier the roster the quieter the lint. The
        // reverse arm inverts that, and an empty roster is now the loudest
        // state rather than the safest.
        const NONE: &[&str] = &[];
        let f = found(&[("dimension::radix", &[("what", "an axis")])], NONE);
        assert_eq!(f.len(), 1, "{f:?}");
        assert!(f[0].contains("radix"), "{}", f[0]);
        assert!(f[0].contains("not rostered"), "{}", f[0]);
    }

    #[test]
    fn control_nothing_declared_and_nothing_rostered_is_the_one_silent_vacuum() {
        // The genuine vacuous case, kept so the arm above is read as a real
        // finding rather than as the lint firing on everything. With no rows
        // and no roster there is nothing to lose and nothing unprotected.
        const NONE: &[&str] = &[];
        assert!(found(&[], NONE).is_empty());
    }

    #[test]
    fn the_shipped_roster_pins_something_and_holds_no_duplicate() {
        // A roster that went empty, or that pins one id twice, would pass every
        // arm above while checking less than it reads as checking.
        assert!(super::ROSTER.len() >= 24, "{}", super::ROSTER.len());
        let mut sorted = super::ROSTER.to_vec();
        sorted.sort_unstable();
        let before = sorted.len();
        sorted.dedup();
        assert_eq!(before, sorted.len(), "the roster names an axis twice");
        assert!(
            super::ROSTER.contains(&"association") && super::ROSTER.contains(&"leaf_aliasing"),
            "the two axes declared with this lint are not pinned"
        );
    }

    #[test]
    fn its_findings_block_every_gate() {
        assert_findings_block(&AppendOnly { roster: ROSTER_TWO }, &view(&[], &[]));
    }

    #[test]
    fn it_is_not_declared_off_so_it_runs_at_all() {
        assert_not_declared_off(&AppendOnly {
            roster: super::ROSTER,
        });
    }

    #[test]
    fn it_answers_to_the_name_the_gate_and_the_config_use() {
        assert_eq!(
            AppendOnly {
                roster: super::ROSTER
            }
            .name(),
            "the-axis-set-is-append-only"
        );
    }

    #[test]
    fn it_reaches_the_pack_the_engine_is_handed() {
        assert_registered("the-axis-set-is-append-only");
    }
}
