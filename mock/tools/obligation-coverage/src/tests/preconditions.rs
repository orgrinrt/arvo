use super::*;

// --- preconditions, which are never a tier ------------------------------------

#[test]
fn a_precondition_is_never_a_tier_and_never_counted_as_coverage() {
    // The arithmetic temptation, refused. An obligation with a precondition and
    // nothing else is further from met than one with nothing at all.
    let v = view(&[
        DEMAND,
        ("law::a_result", &[("precondition_for", "the_thing")]),
    ]);
    assert_eq!(reach(&v)["the_thing"].0, Reach::Nothing);
    assert_eq!(preconditions(&v)["the_thing"].len(), 1);
}

#[test]
fn a_precondition_is_read_from_both_namespaces_that_can_establish_one() {
    for q in ["law::a_result", "proposal::a_result"] {
        let v = view(&[DEMAND, (q, &[("precondition_for", "the_thing")])]);
        assert_eq!(
            preconditions(&v)["the_thing"].len(),
            1,
            "`{q}` can establish a precondition and is not read"
        );
    }
}

#[test]
fn a_precondition_from_a_namespace_the_walk_does_not_read_contributes_nothing() {
    // The control on the instrument, and the case a lint of its own refuses:
    // an edge from anywhere else is invisible here and the obligation reads
    // less encumbered than it is.
    let v = view(&[
        DEMAND,
        ("probe::an_instrument", &[("precondition_for", "the_thing")]),
    ]);
    assert!(preconditions(&v)["the_thing"].is_empty());
}

#[test]
fn a_stamp_does_not_turn_a_proposals_precondition_into_coverage() {
    // A stamped proposal reaches the top tier through `obligation`. Its
    // `precondition_for` edges are a different field meaning a different thing,
    // and the stamp does not move them: a precondition somebody established
    // still leaves its obligation further from met rather than nearer.
    let v = view(&[
        DEMAND,
        ("obligation::other", &[("what", "another")]),
        (
            "ruling::he_said_so",
            &[("rung", "ratified"), ("ratifies", "a_claim")],
        ),
        (
            "proposal::a_claim",
            &[("obligation", "the_thing"), ("precondition_for", "other")],
        ),
    ]);
    let r = reach(&v);
    assert_eq!(r["the_thing"].0, Reach::Ratified);
    assert_eq!(r["other"].0, Reach::Nothing, "a precondition is not a tier");
    assert_eq!(preconditions(&v)["other"].len(), 1);
}
