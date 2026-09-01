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

use super::{preconditions, reach, stamps, tally, ObligationCoverage, Reach};

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

#[test]
fn the_strongest_edge_decides_the_tier_across_rungs() {
    // A `stated` ruling and a ratified one on one obligation is `ratified`, not
    // `stated`. An implementation that took the last edge it walked would pass
    // every arm above, because each of those plants one edge.
    let v = view(&[
        DEMAND,
        (
            "ruling::an_ack",
            &[("rung", "stated"), ("obligation", "the_thing")],
        ),
        (
            "ruling::he_said_so",
            &[("rung", "ratified"), ("obligation", "the_thing")],
        ),
    ]);
    let r = reach(&v);
    assert_eq!(r["the_thing"].0, Reach::Ratified);
    assert_eq!(
        r["the_thing"].1.len(),
        2,
        "both rows are named as having got it there, whichever decided the tier"
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

// --- the ladder and the tier words --------------------------------------------

#[test]
fn every_tier_has_a_word_of_its_own() {
    // `tally` keys a map by `word()`, so two variants sharing a word merge their
    // counts silently and the report loses a tier without printing anything
    // different. Nothing else would catch it.
    let mut seen: Vec<&str> = super::TIERS.iter().map(|t| t.word()).collect();
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
    for pair in super::TIERS.windows(2) {
        assert!(
            pair[0] < pair[1],
            "`TIERS` is out of order at {:?} then {:?}",
            pair[0].word(),
            pair[1].word()
        );
    }
    assert_eq!(super::TIERS.first().copied(), Some(Reach::Ratified));
    assert_eq!(super::TIERS.last().copied(), Some(Reach::Nothing));
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
    assert_eq!(t.len(), super::TIERS.len(), "{t:?}");
    assert_eq!(t["ratified"], 0);
    assert_eq!(t["nothing"], 1);
}

// --- the report ---------------------------------------------------------------

#[test]
fn a_registry_with_no_obligations_is_inconclusive_rather_than_clean() {
    // The verdict that exists so a broken run cannot claim a pass it never
    // established. An empty demand side measures nothing, and reporting `Clean`
    // over it would say the canon is exhaustive because nobody wrote down what
    // it owes.
    let (outcome, text) = run(&view(&[]), &[]);
    assert!(matches!(outcome, Outcome::Inconclusive { .. }), "{text}");
    assert!(text.contains("no `obligation` rows"), "{text}");
}

#[test]
fn the_report_names_every_tier_and_the_rows_that_got_each_there() {
    let v = view(&[
        ("obligation::reached", &[("what", "a")]),
        ("obligation::nothing", &[("what", "d")]),
        (
            "ruling::he_said_so",
            &[("rung", "ratified"), ("obligation", "reached")],
        ),
    ]);
    let (outcome, text) = run(&v, &[]);
    assert!(matches!(outcome, Outcome::Clean { examined: 2 }), "{text}");
    assert!(text.contains("ruling::he_said_so"), "{text}");
    for tier in super::TIERS {
        assert!(
            text.contains(tier.word()),
            "{} missing: {text}",
            tier.word()
        );
    }
}

#[test]
fn the_report_says_which_tier_is_met() {
    // The word `met` is not a tier any more, so the sentence that says which
    // tier is one has to be in the report rather than only in the source.
    let (_, text) = run(&ruling_at("ratified"), &[]);
    assert!(text.contains("only tier that is met"), "{text}");
}

#[test]
fn the_report_marks_a_route_closed_obligation_rather_than_letting_it_read_as_untouched() {
    let (_, text) = run(&retired(), &[]);
    assert!(
        text.contains("named only by a retirement"),
        "a retired route reads identically to nobody having looked on a flat \
         list, which is the distinction the field was added for: {text}"
    );
}

#[test]
fn the_report_names_an_unanswered_obligation_carrying_a_precondition() {
    for v in [
        view(&[
            DEMAND,
            ("law::a_result", &[("precondition_for", "the_thing")]),
        ]),
        view(&[
            DEMAND,
            ("retirement::a_dead_end", &[("obligation", "the_thing")]),
            ("law::a_result", &[("precondition_for", "the_thing")]),
        ]),
        view(&[
            DEMAND,
            (
                "ruling::he_said_so",
                &[("rung", "open"), ("obligation", "the_thing")],
            ),
            ("law::a_result", &[("precondition_for", "the_thing")]),
        ]),
    ] {
        let (_, text) = run(&v, &[]);
        assert!(
            text.contains("answered by nothing and carry an established precondition"),
            "{text}"
        );
        assert!(text.contains("the_thing"), "{text}");
    }
}

#[test]
fn control_an_answered_obligation_carrying_a_precondition_is_not_in_that_list() {
    // The pair is about being stuck, so an obligation something constructive
    // reaches is not one however many preconditions were established along the
    // way. Over all four answering tiers, because the predicate deciding this
    // is where the not-equals form used to be and where a new tier would land
    // on the wrong side.
    for reaching in [
        vec![(
            "ruling::he_said_so",
            &[("rung", "ratified"), ("obligation", "the_thing")][..],
        )],
        vec![(
            "ruling::he_said_so",
            &[("rung", "in_force"), ("obligation", "the_thing")][..],
        )],
        vec![(
            "ruling::he_said_so",
            &[("rung", "stated"), ("obligation", "the_thing")][..],
        )],
        vec![("proposal::a_claim", &[("obligation", "the_thing")][..])],
    ] {
        let mut rows: Vec<(&str, &[(&str, &str)])> = vec![
            DEMAND,
            ("law::a_result", &[("precondition_for", "the_thing")]),
        ];
        rows.extend(reaching.iter().copied());
        let (_, text) = run(&view(&rows), &[]);
        assert!(
            !text.contains("answered by nothing and carry an established precondition"),
            "{text}"
        );
    }
}

#[test]
fn one_obligation_can_be_read_in_full_by_its_slug() {
    let v = view(&[
        (
            "obligation::the_thing",
            &[("what", "a demand"), ("note", "a note")],
        ),
        ("obligation::other", &[("what", "another")]),
        (
            "ruling::he_said_so",
            &[("rung", "ratified"), ("obligation", "the_thing")],
        ),
    ]);
    let (outcome, text) = run(&v, &["the_thing"]);
    assert!(matches!(outcome, Outcome::Clean { examined: 1 }), "{text}");
    assert!(text.contains("a demand"), "{text}");
    assert!(text.contains("a note"), "{text}");
    assert!(text.contains("tier: ratified"), "{text}");
    assert!(text.contains("rung = ratified"), "{text}");
    assert!(
        !text.contains("another"),
        "the other row is not reported: {text}"
    );
}

#[test]
fn a_slug_that_names_nothing_is_inconclusive_rather_than_an_empty_report() {
    let (outcome, text) = run(&alone(), &["nosuch"]);
    assert!(matches!(outcome, Outcome::Inconclusive { .. }), "{text}");
    assert!(
        text.contains("spelling"),
        "an empty report here would read as an obligation nothing reaches: {text}"
    );
}

// --- the contract it declares about itself ------------------------------------

#[test]
fn it_declares_itself_as_the_shape_it_is_and_no_run_returns_a_blocking_finding() {
    // The contract's own enforcement on a `no-failing-case` tool: a run may not
    // return a finding that blocks a gate. Driven over the registries above
    // rather than asserted about the declaration alone, because the declaration
    // is what the tool says and the outcome is what it does.
    assert!(matches!(
        ObligationCoverage.not_a_lint(),
        mockspace::tool::NotALint::NoFailingCase
    ));
    for v in [
        view(&[]),
        alone(),
        ruling_at("ratified"),
        ruling_at("stated"),
        ruling_at("open"),
        stamped_by("ratified"),
        stamped_by("stated"),
        unstamped(),
        retired(),
    ] {
        let (outcome, text) = run(&v, &[]);
        assert!(
            !matches!(outcome, Outcome::Findings(_)),
            "a no-failing-case tool returned findings: {text}"
        );
    }
}

#[test]
fn it_answers_to_the_name_the_subcommand_uses() {
    assert_eq!(ObligationCoverage.name(), "obligation-coverage");
}
