use super::*;

// --- the ladder and the tier words --------------------------------------------

#[test]
fn every_tier_has_a_word_of_its_own() {
    // `tally` keys a map by `word()`, so two variants sharing a word merge their
    // counts silently and the report loses a tier without printing anything
    // different. Nothing else would catch it.
    let mut seen: Vec<&str> = TIERS.iter().map(|t| t.word()).collect();
    let before = seen.len();
    seen.sort_unstable();
    seen.dedup();
    assert_eq!(seen.len(), before, "two tiers share a word: {seen:?}");
}

#[test]
fn the_ladder_is_listed_strongest_first() {
    // `TIERS` drives the tally's order and the report's column order, and `Ord`
    // drives which edge wins. The two are written separately and a disagreement
    // would order the report against the ranking it claims to show.
    for pair in TIERS.windows(2) {
        assert!(
            pair[0] < pair[1],
            "`TIERS` is out of order at {:?} then {:?}",
            pair[0].word(),
            pair[1].word()
        );
    }
    assert_eq!(TIERS.first().copied(), Some(Reach::Ratified));
    assert_eq!(TIERS.last().copied(), Some(Reach::Nothing));
}

#[test]
fn answered_holds_exactly_where_something_constructive_reaches_it() {
    // Driven over every tier rather than over the two the old not-equals form
    // named. That form put a newly added tier on the unanswered side in
    // silence, and three tiers were added here, so it would have mis-filed all
    // three.
    for t in [
        Reach::Ratified,
        Reach::InForce,
        Reach::Stated,
        Reach::Proposed,
    ] {
        assert!(t.answered(), "{}", t.word());
    }
    for t in [Reach::Unsettled, Reach::RouteClosed, Reach::Nothing] {
        assert!(!t.answered(), "{}", t.word());
    }
}
