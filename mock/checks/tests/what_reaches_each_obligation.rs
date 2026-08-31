//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The exhaustiveness bar, measured rather than asserted.
//!
//! Op will not review the canon until it is exhaustive enough that a full design
//! and then a full implementation can be done from it. The obligations are the
//! demand written from outside the canon, so what reaches them is the one
//! measurement of that bar the canon cannot make come out right by agreeing with
//! itself.
//!
//! It was measured before by a shell probe whose own header said it was a net
//! rather than a test, and one of its figures reached op wrong. The instrument
//! moved here. What is pinned below is the shape of the answer, not the answer:
//! the numbers move as the canon is written and the tests say which direction is
//! allowed.

use arvo_checks::obligation::{self, Reach};
use arvo_checks::{canon, parse};

/// Coverage may improve and may not regress.
///
/// A floor on what is answered and a ceiling on what is untouched, rather than
/// an equality, because an equality fails on progress and gets raised without
/// being read.
#[test]
fn obligation_coverage_does_not_regress() {
    let reg = canon();
    let tally = obligation::tally(&reg);
    let total: usize = tally.values().sum();

    assert!(
        total >= 13,
        "the obligation namespace lost rows, from 13: {tally:?}. An obligation is deleted only \
         when the consumer stops needing the thing, which is not something this canon has \
         happened to yet."
    );
    // The ceiling counts `route-closed` as unanswered, because it is. The first
    // version of this test capped `nothing` alone, and adding the retirement
    // edge then moved three obligations out of `nothing` and dropped the count
    // by three with no canon progress at all. A closed route says a way to the
    // thing does not work; nobody has delivered the thing either way.
    let unanswered = tally["nothing"] + tally["route-closed"];
    assert!(
        unanswered <= 11,
        "{unanswered} obligations are answered by nothing, against a ceiling of 11. Every one is \
         a thing a consumer asked for that the canon does not deliver: {tally:?}"
    );
    assert!(
        tally["met"] + tally["proposed"] >= 2,
        "obligations reached by a ruling or a proposal fell below 2: {tally:?}"
    );
}

/// Reclassifying an obligation as route-closed must not read as coverage.
///
/// The instrument's own control, and it caught a real defect in the test above
/// before anything else did: adding one `retirement.obligation` edge moved three
/// obligations out of `nothing` and made the canon look three closer to done.
#[test]
fn a_closed_route_does_not_improve_coverage() {
    let base = r#"
[[obligation]]
id = "a_thing"
need = "A thing."
consumer = "any"
why = "Because."
provenance = ["panel::x"]
"#;
    let with_route = format!(
        "{base}
[[retirement]]
id = \"the_route\"
claim = \"A way.\"
why = \"No.\"
kind = \"wrong\"
obligation = [\"a_thing\"]
provenance = [\"panel::x\"]
"
    );

    let before = obligation::tally(&parse("planted", base));
    let after = obligation::tally(&parse("planted", &with_route));

    assert_eq!(before["nothing"], 1, "{before:?}");
    assert_eq!(after["nothing"], 0, "the tier moved, which is the point of the field: {after:?}");
    assert_eq!(
        before["nothing"] + before["route-closed"],
        after["nothing"] + after["route-closed"],
        "and the unanswered total did not move, which is the point of this test"
    );
    assert_eq!(after["met"] + after["proposed"], 0, "nothing was answered: {after:?}");
}

/// A proposal never counts as met, however many of them there are.
///
/// The obligation file states this and nothing enforced it. It is the tier most
/// likely to be quietly collapsed, because collapsing it makes the canon look
/// finished.
#[test]
fn a_proposal_does_not_meet_an_obligation() {
    let reg = parse(
        "planted",
        r#"
[[obligation]]
id = "a_thing_somebody_needs"
need = "A thing."
consumer = "any"
why = "Because."
provenance = ["panel::x"]

[[proposal]]
id = "a_claim_about_the_thing"
says = "The thing works this way."
obligation = ["a_thing_somebody_needs"]
"#,
    );
    let got = obligation::reach(&reg);
    assert_eq!(
        got["a_thing_somebody_needs"].0,
        Reach::Proposed,
        "a proposal reaches an obligation and does not meet it: {got:?}"
    );
    assert!(obligation::tally(&reg)["met"] == 0, "nothing here is met");
}

/// A retirement is a closed route and is neither an answer nor an absence.
///
/// The whole reason the field was added: `refsto` returns nothing for an
/// obligation nobody looked at and returned nothing for one whose only known
/// route was tried and failed, and nothing returns nothing loudly.
#[test]
fn a_retirement_is_a_closed_route_and_not_an_answer() {
    let reg = parse(
        "planted",
        r#"
[[obligation]]
id = "a_route_was_tried"
need = "A thing."
consumer = "any"
why = "Because."
provenance = ["panel::x"]

[[obligation]]
id = "nobody_looked"
need = "Another thing."
consumer = "any"
why = "Because."
provenance = ["panel::x"]

[[retirement]]
id = "the_route"
claim = "This is how you get the thing."
why = "It is not."
kind = "wrong"
obligation = ["a_route_was_tried"]
provenance = ["panel::x"]
"#,
    );
    let got = obligation::reach(&reg);
    assert_eq!(got["a_route_was_tried"].0, Reach::RouteClosed, "{got:?}");
    assert_eq!(got["nobody_looked"].0, Reach::Nothing, "{got:?}");
    assert_ne!(
        got["a_route_was_tried"].0, got["nobody_looked"].0,
        "the two used to be indistinguishable, which is the defect the field closes"
    );

    let found = obligation::obligations_whose_only_route_is_closed(&reg);
    assert_eq!(found.len(), 1, "only the one with a closed route: {found:#?}");
    assert!(found[0].at.contains("a_route_was_tried"), "{found:#?}");
}

/// The best tier wins where several reach one obligation.
#[test]
fn the_strongest_edge_decides_the_tier() {
    let reg = parse(
        "planted",
        r#"
[[obligation]]
id = "reached_three_ways"
need = "A thing."
consumer = "any"
why = "Because."
provenance = ["panel::x"]

[[retirement]]
id = "a_dead_route"
claim = "One way."
why = "No."
kind = "wrong"
obligation = ["reached_three_ways"]
provenance = ["panel::x"]

[[proposal]]
id = "a_claim"
says = "Another way."
obligation = ["reached_three_ways"]

[[ruling]]
id = "op_said_so"
kind = "intent"
rung = "in_force"
says = "The way."
obligation = ["reached_three_ways"]
provenance = ["panel::x"]
"#,
    );
    let got = obligation::reach(&reg);
    assert_eq!(got["reached_three_ways"].0, Reach::Met, "{got:?}");
    assert_eq!(
        got["reached_three_ways"].1.len(),
        3,
        "all three edges are kept, so a reader sees what a met obligation rests on: {got:?}"
    );
}

/// The control on the instrument, and it is the one that matters.
///
/// `reach` tiers by a fixed list of namespaces. A namespace gaining the field
/// and not appearing there contributes nothing, so coverage reads better than it
/// is, in the flattering direction, with no error anywhere.
#[test]
fn an_edge_from_an_untiered_namespace_is_reported() {
    let reg = parse(
        "planted",
        r#"
[[obligation]]
id = "a_thing"
need = "A thing."
consumer = "any"
why = "Because."
provenance = ["panel::x"]

[[law]]
id = "some_law"
says = "A law."
obligation = ["a_thing"]
"#,
    );
    let found = obligation::obligation_edges_from_an_untiered_namespace(&reg);
    assert_eq!(found.len(), 1, "the law's edge is invisible to the tiering: {found:#?}");
    assert!(found[0].says.contains("law"), "{found:#?}");

    assert_eq!(
        obligation::reach(&reg)["a_thing"].0,
        Reach::Nothing,
        "and this is exactly what it reads as, which is why the arm above exists"
    );
}

/// The same control, run against the committed canon, where it must be silent.
#[test]
fn the_canon_has_no_untiered_obligation_edges() {
    let found = obligation::obligation_edges_from_an_untiered_namespace(&canon());
    assert!(
        found.is_empty(),
        "a namespace gained an `obligation` field and the coverage measurement does not know \
         what a row there means: {found:#?}"
    );
}

/// A slug naming no obligation row, planted and then checked against the canon.
#[test]
fn an_obligation_edge_naming_nothing_is_reported() {
    let reg = parse(
        "planted",
        r#"
[[obligation]]
id = "the_real_one"
need = "A thing."
consumer = "any"
why = "Because."
provenance = ["panel::x"]

[[proposal]]
id = "a_claim"
says = "Something."
obligation = ["the_real_one", "a_slug_that_was_renamed"]
"#,
    );
    let found = obligation::obligation_edges_naming_nothing(&reg);
    assert_eq!(found.len(), 1, "only the orphan: {found:#?}");
    assert!(found[0].says.contains("a_slug_that_was_renamed"), "{found:#?}");

    assert!(
        obligation::obligation_edges_naming_nothing(&canon()).is_empty(),
        "the canon carries an obligation edge naming no row"
    );
}
