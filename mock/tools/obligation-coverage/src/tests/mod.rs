//! Every arm driven against a built registry, including the ones that must not
//! fire. A tool whose only test is "it printed something" reports its own
//! iteration count.
//!
//! **Every planted ruling carries a rung, and that is the whole repair here.**
//! The suite this replaces planted rows carrying an `obligation` field and
//! nothing else, so every ruling any arm ever saw was rungless and the arm
//! named for the top tier asserted it for that rungless row. Setup that helps,
//! exactly: every input was one the implementation handled, so the path that
//! breaks was never entered and the one property the file asserts about itself
//! in prose had no arm anywhere.

use std::collections::BTreeMap;

use mockspace::tool::{Outcome, Tool, ToolContext};
use mockspace::RegistryView;

use super::{preconditions, reach, stamps, tally, ObligationCoverage, Reach, TIERS};

mod contract;
mod ladder;
mod preconditions;
mod report;
/// One file per section, split at the headings this file already carried when it
/// reached nine hundred lines. The helpers below are what every section shares
/// and are the reason the split is a module rather than eight loose files.
mod rung;
mod stamps;
mod tally;
mod walk;

/// A registry with the rows a test names.
///
/// The reverse edges are passed empty throughout, and deliberately: nothing here
/// reads `referrers`. Every edge this tool walks is a forward one it reads off
/// the row itself, which is what lets it distinguish an edge from a `ruling`
/// from an edge from a `retirement`. The engine's reverse index knows a row is
/// referenced and does not know through which field, and the field is the whole
/// of what decides a tier.
fn view(rows: &[(&str, &[(&str, &str)])]) -> RegistryView {
    let mut r: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    for (q, fields) in rows {
        r.insert(
            (*q).to_string(),
            fields
                .iter()
                .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
                .collect(),
        );
    }
    RegistryView::new(r, BTreeMap::new())
}

fn run(v: &RegistryView, args: &[&str]) -> (Outcome, String) {
    let crates = Default::default();
    let dirs: Vec<std::path::PathBuf> = Vec::new();
    let ctx = ToolContext {
        mock_dir: std::path::Path::new("."),
        repo_root: std::path::Path::new("."),
        all_crates: &crates,
        src_dirs: &dirs,
        args,
        stdin: None,
        registry: v,
    };
    let rep = ObligationCoverage.run(&ctx);
    // An inconclusive verdict carries its reason on the outcome and leaves
    // `output` empty, so a test reading `output` alone cannot tell a refusal
    // from a silent pass.
    let text = match &rep.outcome {
        Outcome::Inconclusive { reason } => reason.clone(),
        _ => rep.output.clone(),
    };
    (rep.outcome, text)
}

/// The one obligation every fixture below is about.
const DEMAND: (&str, &[(&str, &str)]) = ("obligation::the_thing", &[("what", "a demand")]);

/// That obligation and nothing reaching it.
fn alone() -> RegistryView {
    view(&[DEMAND])
}

/// A ruling at a named rung naming the obligation directly.
fn ruling_at(rung: &str) -> RegistryView {
    view(&[
        DEMAND,
        (
            "ruling::he_said_so",
            &[("rung", rung), ("obligation", "the_thing")],
        ),
    ])
}

/// A ruling at a named rung stamping a proposal that names the obligation.
///
/// The two-hop shape: the ruling carries no `obligation` edge of its own, so
/// anything this reaches is reached through `ratifies` and through nothing else.
fn stamped_by(rung: &str) -> RegistryView {
    view(&[
        DEMAND,
        (
            "ruling::he_said_so",
            &[("rung", rung), ("ratifies", "a_claim")],
        ),
        ("proposal::a_claim", &[("obligation", "the_thing")]),
    ])
}

/// A proposal naming the obligation with nothing stamping it.
fn unstamped() -> RegistryView {
    view(&[
        DEMAND,
        ("proposal::a_claim", &[("obligation", "the_thing")]),
    ])
}

/// A retirement naming the obligation and nothing else doing so.
fn retired() -> RegistryView {
    view(&[
        DEMAND,
        ("retirement::a_dead_end", &[("obligation", "the_thing")]),
    ])
}

/// The tier the fixtures above put `the_thing` at.
fn tier(v: &RegistryView) -> Reach {
    reach(v)["the_thing"].0
}
