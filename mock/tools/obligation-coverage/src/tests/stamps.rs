use super::*;

// --- the stamp is followed, and only from a ratification ----------------------

#[test]
fn a_proposal_a_ratified_ruling_stamps_reaches_the_top_tier() {
    // `proposal.obligation` is documented as what a proposal would meet if it
    // were stamped, and `ruling.ratifies` is the stamp. Reading the naming
    // namespace alone files a stamped proposal as proposed forever, which is
    // the shipped defect in the other direction.
    let v = stamped_by("ratified");
    assert_eq!(tier(&v), Reach::Ratified);
    assert_eq!(
        stamps(&v).get("a_claim").map(Vec::len),
        Some(1),
        "the stamp is collected keyed by the proposal's slug"
    );
}

#[test]
fn control_a_proposal_stamped_by_an_unratified_ruling_stays_proposed() {
    // The control on the arm above, and the case the walk must fail on. A stamp
    // from below `ratified` is a hard error at the gate under
    // `an-unratified-ruling-stamps-a-proposal`, and a measurement that assumed
    // the gate had run would report the proposal as canon on exactly the row
    // that gate exists to catch.
    //
    // Fails if `stamps()` collects the edge without reading the stamper's rung.
    for rung in ["in_force", "stated", "open", "whatever_comes_next"] {
        let v = stamped_by(rung);
        assert_eq!(
            tier(&v),
            Reach::Proposed,
            "a stamp from a ruling at `rung = {rung}` is not a stamp"
        );
        assert!(stamps(&v).is_empty(), "{rung}");
    }
}

#[test]
fn the_report_names_the_ruling_that_stamped_a_proposal() {
    // `ratified` on a proposal-sourced line claims a ruling's authority, so the
    // line names which ruling and the two-hop path can be checked rather than
    // taken.
    let (_, text) = run(&stamped_by("ratified"), &[]);
    assert!(text.contains("proposal::a_claim"), "{text}");
    assert!(text.contains("stamped by ruling::he_said_so"), "{text}");
}

#[test]
fn several_stamps_on_one_ruling_are_all_followed() {
    // `ratifies` is a `proposal[]` and arrives joined, so a reader taking the
    // whole value as one slug follows neither. Every arm above plants one entry
    // and would pass through that defect.
    let v = view(&[
        ("obligation::first", &[("what", "one")]),
        ("obligation::second", &[("what", "two")]),
        (
            "ruling::he_said_so",
            &[("rung", "ratified"), ("ratifies", "a_claim, another_claim")],
        ),
        ("proposal::a_claim", &[("obligation", "first")]),
        ("proposal::another_claim", &[("obligation", "second")]),
    ]);
    let r = reach(&v);
    assert_eq!(r["first"].0, Reach::Ratified);
    assert_eq!(r["second"].0, Reach::Ratified);
}

#[test]
fn a_stamp_naming_no_proposal_changes_nothing_and_does_not_panic() {
    let v = view(&[
        DEMAND,
        (
            "ruling::he_said_so",
            &[("rung", "ratified"), ("ratifies", "a_ghost")],
        ),
        ("proposal::a_claim", &[("obligation", "the_thing")]),
    ]);
    assert_eq!(tier(&v), Reach::Proposed);
}

#[test]
fn an_unstamped_proposal_only_proposes_it() {
    // The distinction the obligation file states in as many words: a proposal is
    // proposed rather than met, and reporting it otherwise closes a gap op has
    // never seen.
    assert_eq!(tier(&unstamped()), Reach::Proposed);
}
