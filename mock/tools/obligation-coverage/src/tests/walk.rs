use super::*;

// --- the other edges, and the walk itself -------------------------------------

#[test]
fn a_retirement_is_a_closed_route_and_not_an_answer() {
    assert_eq!(tier(&retired()), Reach::RouteClosed);
}

#[test]
fn an_obligation_nothing_names_reaches_nothing() {
    let v = alone();
    assert_eq!(tier(&v), Reach::Nothing);
    assert!(reach(&v)["the_thing"].1.is_empty());
}

/// Every tier one edge can produce, strongest first, with the row that produces
/// it: the namespace it sits in and the fields it carries beside its slug.
///
/// A table rather than a pair, because the arm below plants each of these
/// against each of the others and the interesting property is over all of them
/// at once. `Reach::Nothing` is absent on purpose: it is what an obligation no
/// edge names reaches, so there is no row that produces it and nothing to plant.
type Rung = (Reach, &'static str, &'static [(&'static str, &'static str)]);

const LADDER: [Rung; 6] = [
    (
        Reach::Ratified,
        "ruling",
        &[("rung", "ratified"), ("obligation", "the_thing")],
    ),
    (
        Reach::InForce,
        "ruling",
        &[("rung", "in_force"), ("obligation", "the_thing")],
    ),
    (
        Reach::Stated,
        "ruling",
        &[("rung", "stated"), ("obligation", "the_thing")],
    ),
    (Reach::Proposed, "proposal", &[("obligation", "the_thing")]),
    (
        Reach::Unsettled,
        "ruling",
        &[("rung", "open"), ("obligation", "the_thing")],
    ),
    (
        Reach::RouteClosed,
        "retirement",
        &[("obligation", "the_thing")],
    ),
];

#[test]
fn the_strongest_edge_decides_the_tier_whichever_order_the_walk_takes() {
    // The arm this replaces planted one pair in one arrangement and could not
    // fail: it named the ratified row `he_said_so` and the weak one `an_ack`,
    // `rows_in` yields in slug order, so the strongest edge was the last one
    // walked and an implementation taking the last edge produced the same
    // answer. Mutating `entry.0 = entry.0.min(tier)` to `entry.0 = tier` passed
    // all thirty-four arms.
    //
    // So every pair, both ways round. `a_first` sorts before `b_second`, so
    // giving the stronger row `a_first` walks it first and swapping walks it
    // last.
    //
    // **Only the pairs drawn from one namespace vary.** Across namespaces the
    // order is `EDGES`, fixed at `ruling`, `proposal`, `retirement`, so a
    // `ruling` against a `retirement` is walked rulings-first however the rows
    // are named and the swap changes nothing. Those rows assert the right
    // answer and are not the ones that kill a last-edge implementation; the
    // ruling-against-ruling pairs are.
    for (i, (strong, strong_ns, strong_fields)) in LADDER.iter().enumerate() {
        for (weak, weak_ns, weak_fields) in LADDER.iter().skip(i + 1) {
            for (strong_slug, weak_slug) in [("a_first", "b_second"), ("b_second", "a_first")] {
                let s = format!("{strong_ns}::{strong_slug}");
                let w = format!("{weak_ns}::{weak_slug}");
                let v = view(&[DEMAND, (&s, strong_fields), (&w, weak_fields)]);
                let entry = &reach(&v)["the_thing"];
                assert_eq!(
                    entry.0, *strong,
                    "{strong:?} planted as {s} against {weak:?} as {w} must tier {strong:?}"
                );
                assert_eq!(
                    entry.1.len(),
                    2,
                    "both rows are named as having got it there, whichever decided the tier"
                );
            }
        }
    }
}

#[test]
fn control_the_pairs_that_walk_the_stronger_row_first_are_the_ones_that_bite() {
    // The arm above asserts over thirty views and only some of them can catch a
    // last-edge implementation. This names which, so nobody reads the table as
    // thirty load-bearing rows: a pair bites where the walk reaches the stronger
    // edge first, and then a walk that overwrote would end on the weaker one.
    //
    // Fifteen pairs, each planted twice, and twenty-two of the thirty bite. Six
    // pairs are ruling against ruling and bite in the one arrangement that sorts
    // the stronger slug first. Nine reach across namespaces with the stronger
    // one earlier in `EDGES`, so those bite in both arrangements and the swap
    // buys nothing. The last three are a `proposal` against a `ruling` at
    // `open`, where the weaker row walks first whatever it is called, so the
    // pair asserts the right answer and catches nothing.
    let mut bites = 0;
    for (i, (_, strong_ns, strong_fields)) in LADDER.iter().enumerate() {
        for (_, weak_ns, weak_fields) in LADDER.iter().skip(i + 1) {
            for (strong_slug, weak_slug) in [("a_first", "b_second"), ("b_second", "a_first")] {
                let s = format!("{strong_ns}::{strong_slug}");
                let w = format!("{weak_ns}::{weak_slug}");
                let v = view(&[DEMAND, (&s, strong_fields), (&w, weak_fields)]);
                // What a walk that took the last edge would report, derived the
                // same way the walk derives it: namespace order first, slug
                // order inside one namespace.
                let ns_rank = |ns: &str| match ns {
                    "ruling" => 0,
                    "proposal" => 1,
                    _ => 2,
                };
                let strong_last = (ns_rank(strong_ns), strong_slug) > (ns_rank(weak_ns), weak_slug);
                if !strong_last {
                    bites += 1;
                }
                assert!(
                    reach(&v)["the_thing"].1.len() == 2,
                    "the derivation above is about order and both rows are named either way"
                );
            }
        }
    }
    assert!(
        bites > 0,
        "a table where no arrangement reaches the stronger edge first would assert nothing"
    );
    assert_eq!(
        bites, 22,
        "six ruling pairs bite once each, nine cross-namespace pairs bite twice, \
         and the three where a ruling at `open` walks before a proposal never do"
    );
}

#[test]
fn a_slug_naming_no_obligation_contributes_nothing_rather_than_panicking() {
    let v = view(&[
        DEMAND,
        (
            "ruling::he_said_so",
            &[("rung", "ratified"), ("obligation", "a_ghost")],
        ),
    ]);
    assert_eq!(reach(&v)["the_thing"].0, Reach::Nothing);
}

#[test]
fn several_obligations_on_one_row_are_all_reached() {
    // The field is a `string[]` and arrives joined, so a reader taking the whole
    // value as one slug reaches neither.
    let v = view(&[
        ("obligation::first", &[("what", "one")]),
        ("obligation::second", &[("what", "two")]),
        (
            "ruling::he_said_so",
            &[("rung", "ratified"), ("obligation", "first, second")],
        ),
    ]);
    let r = reach(&v);
    assert_eq!(r["first"].0, Reach::Ratified);
    assert_eq!(r["second"].0, Reach::Ratified);
}
