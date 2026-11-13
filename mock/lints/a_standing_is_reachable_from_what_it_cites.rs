//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A standing asserting several arrivals cites more than one file.
//!
//! `standing` is the field that decides whether a proposal is close to canon,
//! and the schema constrains its spelling and nothing else. A row may say
//! `cross_topic`, the strongest tier the panel produces, while citing one file
//! once, and every gate stays green. That was measured rather than assumed: the
//! transcript is in `214_probes/standing_is_unconstrained.txt`, together with
//! the positive control setting the same field to `seventeen_experts` and
//! watching the schema refuse it by name, so the green is a fact about what the
//! gate reads rather than about whether it runs.
//!
//! Independence is between authors, and a numbered member file has one author,
//! so one citation cannot exhibit two arrivals however the note describes them.
//! The row may still be right. What it cannot be is checked, and a claim nobody
//! can locate is one nobody can overturn.
//!
//! A necessary condition rather than a sufficient one. Nothing here can
//! tell whether two authors really arrived separately, and it does not pretend
//! to. What it establishes is that a reader could go and find out.
//!
//! A lint rather than a tool. There is a state it refuses: a row claiming
//! more arrivals than its citations can carry. The fix is per row and it is
//! additive, so the answer is not an inventory somebody ranks.
//!
//! # What the corpus carries, and why the finding blocks a new one only
//!
//! Twenty-nine committed rows are in this state, and the crate this came from
//! held them under a ratchet with its reasoning written out: the list is not
//! asserted empty because it is not empty, and the work of emptying it is per
//! row and per topic rather than mechanical. What was pinned is that it does not
//! grow, because a row porting a claim out of a consolidation without its
//! establishing files is added one row at a time by whoever is porting that
//! topic.
//!
//! So the ceiling is carried across and the severity is not touched. The
//! alternative considered was declaring the lint at `warn` in `mockspace.toml`,
//! which states the true thing about the backlog and reports every offending row
//! at every gate. It was refused on one ground: a warning refuses nothing, so
//! the twenty-ninth row and the thirtieth are treated alike, and the thirtieth
//! is the whole point. Severity is also the one dial this workspace reserves for
//! a human, and a ratchet needs nobody to turn it.
//!
//! The contract's objection to a threshold is real and is answered by where this
//! one came from: it is a measurement of the committed corpus rather than a
//! number somebody chose, it only ever falls, and the finding names every row it
//! counted, so a ceiling that went green because the predicate broke reports
//! zero of twenty-nine rather than passing quietly.
//!
//! # What the unit tests here cannot ask
//!
//! That the committed canon agrees with this predicate. A unit test cannot
//! build a `RegistryView` from `mock/registry/`, because that needs a TOML
//! parser the generated pack has no route to depend on. `cargo mock --lint-only`
//! is where the predicate meets the real rows, and it runs this over all of
//! them at every gate.
use mockspace::{Lint, LintError, RepoContext, RepoLint, Severity};

use crate::canon_citations::{
    Consolidation,
    arrivals,
    consolidation_numbers,
    files_cited,
    is_a_consolidation,
    members_named,
    number_of,
};
use crate::canon_rows::{finding, text};
use crate::panel_corpus::{markdown, panel_dir};
pub fn repo_lint() -> Box<dyn RepoLint> {
    Box::new(AStandingIsReachableFromWhatItCites {
        ceiling: CEILING,
    })
}

/// The rows in this state, measured over the committed canon.
///
/// Lower it as topics are second-read; never raise it. Under the rule that
/// two agreeing experts ratify, these are exactly the rows eligible for
/// promotion, and not one of them names a second source.
const CEILING: usize = 29;

const LINT: &str = "a-standing-is-reachable-from-what-it-cites";

/// The standings that assert more than one independent arrival.
///
/// `contested` is absent on purpose: it says somebody disagreed, which is not a
/// count of arrivals and is not inflated by resting on one file.
const MULTI_ARRIVAL: [&str; 3] = ["two_experts", "three_or_more", "cross_topic"];

/// The lint, carrying the ceiling it grandfathers.
///
/// A field rather than a constant read inside the predicate, so a test can build
/// one at zero and drive the whole lint. A ceiling only ever exercised at the
/// number it was measured with is a ceiling nobody has seen fire.
struct AStandingIsReachableFromWhatItCites {
    ceiling: usize,
}

impl Lint for AStandingIsReachableFromWhatItCites {
    fn name(&self) -> &'static str {
        LINT
    }

    fn default_severity(&self) -> Severity {
        Severity::HARD_ERROR
    }
}
impl RepoLint for AStandingIsReachableFromWhatItCites {
    fn check_repo(&self, ctx: &RepoContext) -> Vec<LintError> {
        let panel = markdown(ctx.view(), &panel_dir(ctx.mock_dir));
        let names: Vec<&str> = panel
            .iter()
            .filter_map(|p| p.file_name().and_then(|n| n.to_str()))
            .collect();
        let numbers = consolidation_numbers(names.iter().copied());
        let consolidations: Vec<Consolidation> = panel
            .iter()
            .filter_map(|p| {
                let name = p.file_name()?.to_str()?;
                if !is_a_consolidation(name) {
                    return None;
                }
                let number = number_of(name)?;
                let text = ctx.view().text(p)?;
                Some(Consolidation {
                    number,
                    members: members_named(&text, number, &numbers),
                })
            })
            .collect();
        let found: Vec<LintError> = ctx
            .registry()
            .rows_in("proposal")
            .iter()
            .filter_map(|q| check(ctx.registry(), q, &consolidations))
            .collect();
        if found.len() <= self.ceiling {
            return Vec::new();
        }
        found
    }
}

/// One proposal, against the citations its standing would need.
fn check(
    reg: &mockspace::RegistryView,
    q: &str,
    consolidations: &[Consolidation],
) -> Option<LintError> {
    let standing = text(reg, q, "standing")?;
    if !MULTI_ARRIVAL.contains(&standing) {
        return None;
    }
    let files = files_cited(reg, q);
    if arrivals(&files, consolidations) >= 2 {
        return None;
    }
    let named = files.iter().copied().collect::<Vec<_>>().join("`, `");
    let named = if named.is_empty() { "nothing at all".to_string() } else { named };
    Some(finding(
        LINT,
        None,
        format!(
            "`{q}` sets `standing` to `{standing}`, which asserts more than one independent \
             arrival, and `provenance` names `{named}`, which is one arrival: one file has one \
             author, and a member file with the consolidation over it is one author's account \
             twice. A reader cannot reach a second arrival from this row. Cite the files that \
             reached it, or state the standing the citations support."
        ),
    ))
}

#[cfg(test)]
#[path = "a_standing_is_reachable_from_what_it_cites/tests.rs"]
mod tests;
