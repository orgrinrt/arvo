use super::*;

// --- the tally ----------------------------------------------------------------

#[test]
fn the_tally_counts_every_obligation_once_and_at_one_tier() {
    let v = view(&[
        ("obligation::ratified", &[("what", "a")]),
        ("obligation::in_force", &[("what", "b")]),
        ("obligation::stated", &[("what", "c")]),
        ("obligation::proposed", &[("what", "d")]),
        ("obligation::unsettled", &[("what", "e")]),
        ("obligation::closed", &[("what", "f")]),
        ("obligation::nothing", &[("what", "g")]),
        (
            "ruling::r",
            &[("rung", "ratified"), ("obligation", "ratified")],
        ),
        (
            "ruling::f",
            &[("rung", "in_force"), ("obligation", "in_force")],
        ),
        ("ruling::s", &[("rung", "stated"), ("obligation", "stated")]),
        (
            "ruling::o",
            &[("rung", "open"), ("obligation", "unsettled")],
        ),
        ("proposal::p", &[("obligation", "proposed")]),
        ("retirement::x", &[("obligation", "closed")]),
    ]);
    let t = tally(&v);
    for word in [
        "ratified",
        "in_force",
        "stated",
        "proposed",
        "unsettled",
        "route-closed",
    ] {
        assert_eq!(t[word], 1, "{word} in {t:?}");
    }
    assert_eq!(t["nothing"], 1);
    assert_eq!(t.values().sum::<usize>(), 7, "one obligation, one tier");
}

#[test]
fn the_tally_names_every_tier_even_where_none_sits_there() {
    // A tier missing from the report reads as a tier nobody has reached, and a
    // reader cannot tell that from a tier the tool forgot to print.
    let t = tally(&alone());
    assert_eq!(t.len(), TIERS.len(), "{t:?}");
    assert_eq!(t["ratified"], 0);
    assert_eq!(t["nothing"], 1);
}
