use super::*;

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
    for tier in TIERS {
        assert!(
            text.contains(tier.word()),
            "{} missing: {text}",
            tier.word()
        );
    }
}

#[test]
fn the_report_orders_the_tally_strongest_first_and_the_rows_weakest_first() {
    // Nothing pinned either order, so a change from tier-major to slug-major, or
    // a flip of the body, would move every line of the output and no arm would
    // fire. Both orders carry meaning and they are deliberately opposite: the
    // tally reads as a ladder from met downward, and the body puts what is least
    // reached at the top, where a reader looking for work finds it first.
    let v = view(&[
        ("obligation::zeta", &[("what", "a")]),
        ("obligation::alpha", &[("what", "b")]),
        ("obligation::mid", &[("what", "c")]),
        (
            "ruling::ratifies_zeta",
            &[("rung", "ratified"), ("obligation", "zeta")],
        ),
        (
            "ruling::ratifies_alpha",
            &[("rung", "ratified"), ("obligation", "alpha")],
        ),
        ("proposal::proposes_mid", &[("obligation", "mid")]),
    ]);
    let (_, text) = run(&v, &[]);
    let at = |needle: &str| {
        text.find(needle)
            .unwrap_or_else(|| panic!("{needle} missing from the report: {text}"))
    };

    // The tally, strongest first, in the order the ladder itself declares.
    let mut previous = 0;
    for tier in TIERS {
        let here = at(tier.word());
        assert!(
            here > previous,
            "the tally follows the ladder and `{}` is out of place: {text}",
            tier.word()
        );
        previous = here;
    }

    // The body, weakest first, with two rows at one tier so the ordering inside
    // a tier is exercised by the same report.
    assert!(
        at("mid") < at("alpha"),
        "a weaker tier's rows come before a stronger tier's, whatever the slugs \
         sort like: {text}"
    );
    assert!(
        at("alpha") < at("zeta"),
        "inside one tier the rows are in the registry's own slug order: {text}"
    );
    assert!(
        text.contains("weakest first"),
        "the body says which way round it is, because the tally above it is the \
         other way and a reader cannot infer either from two rows: {text}"
    );
}

#[test]
fn a_ruling_whose_kind_is_a_refusal_still_tiers_by_its_rung_today() {
    // **This records what the walk does, not what it should do.** The walk reads
    // `rung` and never `kind`, so a `ruling` at `ratified` whose kind is
    // `refusal` or `deferral` tiers as met, and no fixture anywhere planted one
    // until this arm.
    //
    // Nothing is misreported on the committed registry: eight of the ninety-four
    // rulings carry one of those two kinds and none of them names an obligation.
    // Whether a refusal that named an obligation has met it is a canon reading,
    // which is two independent agreements and not this arm's to make. So the
    // assertion is the current answer, and when somebody settles the question
    // this is the arm that fails and says where the decision lives.
    for kind in ["refusal", "deferral"] {
        let v = view(&[
            DEMAND,
            (
                "ruling::he_said_so",
                &[
                    ("rung", "ratified"),
                    ("kind", kind),
                    ("obligation", "the_thing"),
                ],
            ),
        ]);
        assert_eq!(
            tier(&v),
            Reach::Ratified,
            "a `{kind}` at `ratified` currently reads as met, which is the \
             behaviour recorded rather than the behaviour argued for"
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
