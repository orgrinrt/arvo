use super::*;

// --- the rung decides a ruling's tier -----------------------------------------

#[test]
fn each_rung_lands_at_its_own_tier() {
    // Over all four declared rungs in one arm rather than one rung sampled, so a
    // mapping right for `ratified` and wrong for `in_force` cannot pass. The
    // values are `mockspace.toml`'s own, from the `rung` field's `values` list.
    for (rung, want) in [
        ("ratified", Reach::Ratified),
        ("in_force", Reach::InForce),
        ("stated", Reach::Stated),
        ("open", Reach::Unsettled),
    ] {
        assert_eq!(
            tier(&ruling_at(rung)),
            want,
            "a ruling at `rung = {rung}` should tier at `{}`",
            want.word()
        );
    }
}

#[test]
fn a_stated_ruling_does_not_reach_the_top_tier() {
    // The arm that would have caught the shipped defect, named for it. The walk
    // read the namespace, so every ruling met its obligation whatever rung it
    // sat at, and the one row that produced the repository's only `met` was a
    // `stated` ruling whose own note records op declining to bless it.
    //
    // Fails on any implementation that tiers from the namespace.
    assert_ne!(tier(&ruling_at("stated")), Reach::Ratified);
    assert_eq!(tier(&ruling_at("stated")), Reach::Stated);
}

#[test]
fn a_rung_the_walk_cannot_read_does_not_reach_the_top_tier() {
    // The pessimistic direction, on purpose: a rung the tool does not know must
    // never read stronger than it is, because the flattering direction is the
    // one a clean run cannot be told apart from a real one.
    //
    // An absent rung is the case the schema says cannot happen, `rung` being
    // required, and is planted anyway because a loader defect would otherwise
    // land the row at the top tier in silence.
    for v in [
        ruling_at("whatever_comes_next"),
        ruling_at(""),
        view(&[
            DEMAND,
            ("ruling::he_said_so", &[("obligation", "the_thing")]),
        ]),
    ] {
        assert_eq!(tier(&v), Reach::Unsettled);
    }
}

#[test]
fn the_report_prints_a_rulings_rung_beside_it() {
    // `unsettled` holds a ruling at `open` and one whose rung could not be read,
    // so the tier alone cannot tell them apart and the line has to.
    let (_, open) = run(&ruling_at("open"), &[]);
    assert!(open.contains("rung = open"), "{open}");
    let (_, absent) = run(
        &view(&[
            DEMAND,
            ("ruling::he_said_so", &[("obligation", "the_thing")]),
        ]),
        &[],
    );
    assert!(absent.contains("rung = (absent)"), "{absent}");
}
