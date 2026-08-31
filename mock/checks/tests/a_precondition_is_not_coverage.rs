//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The edge four dispatches wanted, and the one way it could go wrong.
//!
//! An obligation is a consumer need in the consumer's own terms. A consumer asks
//! for a graph ordering; what it needs first is that the addition beneath the
//! ordering be associative, because arvo uses however many cores it detects and
//! splitting a reduction changes the association order. **No consumer document
//! says that, because the consumer cannot know it**: it is a fact about two
//! layers interacting, discoverable only by deriving it.
//!
//! So the gap was never in the enumeration and could not be closed by
//! enumerating harder. The registry could hold both sentences and had no way to
//! join them, and four separate dispatches hit that wall from four directions.
//!
//! **The danger is arithmetic.** An obligation with four established
//! preconditions looks better attended than one with none, and is worse off. So
//! a precondition is reported beside the coverage and never inside it, and this
//! file exists to pin that.

use arvo_checks::obligation::{self, Reach};
use arvo_checks::{canon, parse};

/// A precondition moves no tier, in either direction.
#[test]
fn a_precondition_is_not_an_answer() {
    let reg = parse(
        "planted",
        r#"
[[obligation]]
id = "a_thing"
need = "A thing."
consumer = "any"
why = "Because."
provenance = ["panel::x"]

[[proposal]]
id = "a_result_it_depends_on"
says = "Something the thing needs."
precondition_for = ["a_thing"]
"#,
    );
    let got = obligation::reach(&reg);
    assert_eq!(
        got["a_thing"].0,
        Reach::Nothing,
        "a precondition is not an answer and must not read as one: {got:?}"
    );
    assert!(
        got["a_thing"].1.is_empty(),
        "and it is not evidence for the tier either: {got:?}"
    );
    assert_eq!(obligation::preconditions(&reg)["a_thing"].len(), 1);
}

/// A law may establish one as much as a proposal may.
///
/// The reassociation case is a measurement; the associativity it rests on is a
/// law. If only proposals could carry the edge, the commonest shape would have
/// nowhere to go.
#[test]
fn a_law_can_establish_a_precondition() {
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
id = "a_law_it_rests_on"
statement = "The operation is associative."
precondition_for = ["a_thing"]
"#,
    );
    assert_eq!(obligation::preconditions(&reg)["a_thing"].len(), 1);
    assert_eq!(obligation::reach(&reg)["a_thing"].0, Reach::Nothing);
}

/// Unanswered with a precondition is worse than unanswered, and is reported.
#[test]
fn an_unanswered_obligation_with_a_precondition_is_reported() {
    let reg = parse(
        "planted",
        r#"
[[obligation]]
id = "encumbered"
need = "A thing."
consumer = "any"
why = "Because."
provenance = ["panel::x"]

[[obligation]]
id = "merely_unanswered"
need = "Another thing."
consumer = "any"
why = "Because."
provenance = ["panel::x"]

[[obligation]]
id = "already_met"
need = "A third thing."
consumer = "any"
why = "Because."
provenance = ["panel::x"]

[[proposal]]
id = "a_dependency"
says = "Something."
precondition_for = ["encumbered", "already_met"]

[[ruling]]
id = "op_met_it"
kind = "intent"
rung = "in_force"
says = "The way."
obligation = ["already_met"]
provenance = ["panel::x"]
"#,
    );
    let found = obligation::unanswered_obligations_carrying_a_precondition(&reg);
    assert_eq!(
        found.len(),
        1,
        "only the unanswered one with a dependency: the plain one has no dependency and the met \
         one is not unanswered: {found:#?}"
    );
    assert!(found[0].at.contains("encumbered"), "{found:#?}");
    assert!(
        found[0].says.contains("further from met"),
        "the report says which way it cuts, because the arithmetic instinct is the other way: \
         {found:#?}"
    );
}

/// The control on the walk itself.
///
/// `preconditions` reads a fixed list of namespaces. An edge from anywhere else
/// contributes nothing, so the obligation reads less encumbered than it is,
/// which is the flattering direction and therefore the one to guard.
#[test]
fn an_edge_from_a_namespace_that_cannot_establish_one_is_reported() {
    let reg = parse(
        "planted",
        r#"
[[obligation]]
id = "a_thing"
need = "A thing."
consumer = "any"
why = "Because."
provenance = ["panel::x"]

[[retirement]]
id = "a_dead_route"
claim = "A way that does not work."
why = "It does not."
kind = "wrong"
precondition_for = ["a_thing"]
provenance = ["panel::x"]
"#,
    );
    let found = obligation::preconditions_from_a_namespace_that_cannot_establish_one(&reg);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(found[0].says.contains("retirement"), "{found:#?}");
    assert!(
        obligation::preconditions(&reg)["a_thing"].is_empty(),
        "and this is exactly what it reads as, which is why the arm above exists"
    );
}

/// The committed canon carries no such edge.
#[test]
fn the_canon_has_no_precondition_from_an_unread_namespace() {
    let found = obligation::preconditions_from_a_namespace_that_cannot_establish_one(&canon());
    assert!(found.is_empty(), "{found:#?}");
}

/// The first filed precondition is present and reaches an unanswered obligation.
///
/// Pinned so the field does not become a declaration nothing constrains, which
/// is the failure a schema addition falls into by default: it reads as a
/// contract and is a comment with a type until something is filed under it.
#[test]
fn the_reassociation_precondition_is_filed_and_lands_on_an_open_obligation() {
    let reg = canon();
    let pre = obligation::preconditions(&reg);
    let on = &pre["ordering_a_directed_acyclic_graph"];
    assert!(
        on.iter()
            .any(|r| r.contains("splitting_a_reduction_is_sound")),
        "the reassociation measurement is a precondition of the graph ordering: {on:?}"
    );
    assert_ne!(
        reg.of("obligation")
            .find(|r| r.id == "ordering_a_directed_acyclic_graph")
            .map(|_| obligation::reach(&reg)["ordering_a_directed_acyclic_graph"].0),
        Some(Reach::Met),
        "and it is not met, so the pair the registry could not express before now exists"
    );
}
