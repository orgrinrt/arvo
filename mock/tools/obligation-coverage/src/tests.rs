//! Every arm driven against a built registry, including the ones that must not
//! fire. A tool whose only test is "it printed something" reports its own
//! iteration count.

use std::collections::BTreeMap;

use mockspace::tool::{Outcome, Tool, ToolContext};
use mockspace::RegistryView;

use super::{preconditions, reach, tally, ObligationCoverage, Reach};

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

/// One obligation and one row naming it from each tiering namespace, so a test
/// can pick which edges exist.
fn with(edges: &[(&str, &str)]) -> RegistryView {
    let mut rows: Vec<(&str, &[(&str, &str)])> =
        vec![("obligation::the_thing", &[("what", "a demand")])];
    let held: Vec<(String, [(&str, &str); 1])> = edges
        .iter()
        .map(|(q, field)| ((*q).to_string(), [(*field, "the_thing")]))
        .collect();
    let refs: Vec<(&str, &[(&str, &str)])> = held
        .iter()
        .map(|(q, f)| (q.as_str(), f.as_slice()))
        .collect();
    rows.extend(refs);
    view(&rows)
}

#[test]
fn a_ruling_meets_an_obligation_and_a_proposal_only_proposes_it() {
    // The distinction the obligation file states in as many words: a proposal
    // is proposed rather than met, and reporting it otherwise closes a gap op
    // has never seen.
    let met = reach(&with(&[("ruling::he_said_so", "obligation")]));
    assert_eq!(met["the_thing"].0, Reach::Met);
    let proposed = reach(&with(&[("proposal::a_claim", "obligation")]));
    assert_eq!(proposed["the_thing"].0, Reach::Proposed);
}

#[test]
fn a_retirement_is_a_closed_route_and_not_an_answer() {
    let closed = reach(&with(&[("retirement::a_dead_end", "obligation")]));
    assert_eq!(closed["the_thing"].0, Reach::RouteClosed);
}

#[test]
fn an_obligation_nothing_names_reaches_nothing() {
    let alone = reach(&view(&[("obligation::the_thing", &[("what", "a demand")])]));
    assert_eq!(alone["the_thing"].0, Reach::Nothing);
    assert!(alone["the_thing"].1.is_empty());
}

#[test]
fn the_strongest_edge_decides_the_tier() {
    // A ruling and a retirement both naming one obligation is met, not
    // route-closed. An implementation that took the last edge it walked would
    // pass every arm above, because each of those plants one edge.
    let both = reach(&with(&[
        ("retirement::a_dead_end", "obligation"),
        ("ruling::he_said_so", "obligation"),
    ]));
    assert_eq!(both["the_thing"].0, Reach::Met);
    assert_eq!(
        both["the_thing"].1.len(),
        2,
        "both rows are named as having got it there, whichever decided the tier"
    );
}

#[test]
fn a_slug_naming_no_obligation_contributes_nothing_rather_than_panicking() {
    let v = view(&[
        ("obligation::the_thing", &[("what", "a demand")]),
        ("ruling::he_said_so", &[("obligation", "a_ghost")]),
    ]);
    assert_eq!(reach(&v)["the_thing"].0, Reach::Nothing);
}

#[test]
fn several_obligations_on_one_row_are_all_reached() {
    // The field is a `string[]` and arrives joined, so a reader taking the whole
    // value as one slug reaches neither. Every arm above plants one entry and
    // would pass through that defect.
    let v = view(&[
        ("obligation::first", &[("what", "one")]),
        ("obligation::second", &[("what", "two")]),
        ("ruling::he_said_so", &[("obligation", "first, second")]),
    ]);
    let r = reach(&v);
    assert_eq!(r["first"].0, Reach::Met);
    assert_eq!(r["second"].0, Reach::Met);
}

#[test]
fn a_precondition_is_never_a_tier_and_never_counted_as_coverage() {
    // The arithmetic temptation, refused. An obligation with a precondition and
    // nothing else is further from met than one with nothing at all.
    let v = view(&[
        ("obligation::the_thing", &[("what", "a demand")]),
        ("law::a_result", &[("precondition_for", "the_thing")]),
    ]);
    assert_eq!(reach(&v)["the_thing"].0, Reach::Nothing);
    assert_eq!(preconditions(&v)["the_thing"].len(), 1);
}

#[test]
fn a_precondition_is_read_from_both_namespaces_that_can_establish_one() {
    for ns in ["law", "proposal"] {
        let v = view(&[
            ("obligation::the_thing", &[("what", "a demand")]),
            (
                match ns {
                    "law" => "law::a_result",
                    _ => "proposal::a_result",
                },
                &[("precondition_for", "the_thing")],
            ),
        ]);
        assert_eq!(
            preconditions(&v)["the_thing"].len(),
            1,
            "a `{ns}` row can establish a precondition and is not read"
        );
    }
}

#[test]
fn a_precondition_from_a_namespace_the_walk_does_not_read_contributes_nothing() {
    // The control on the instrument, and the case a lint of its own refuses:
    // an edge from anywhere else is invisible here and the obligation reads
    // less encumbered than it is.
    let v = view(&[
        ("obligation::the_thing", &[("what", "a demand")]),
        ("probe::an_instrument", &[("precondition_for", "the_thing")]),
    ]);
    assert!(preconditions(&v)["the_thing"].is_empty());
}

#[test]
fn the_tally_counts_every_obligation_once_and_at_one_tier() {
    let v = view(&[
        ("obligation::met", &[("what", "a")]),
        ("obligation::proposed", &[("what", "b")]),
        ("obligation::closed", &[("what", "c")]),
        ("obligation::nothing", &[("what", "d")]),
        ("ruling::r", &[("obligation", "met")]),
        ("proposal::p", &[("obligation", "proposed")]),
        ("retirement::x", &[("obligation", "closed")]),
    ]);
    let t = tally(&v);
    assert_eq!(t["met"], 1);
    assert_eq!(t["proposed"], 1);
    assert_eq!(t["route-closed"], 1);
    assert_eq!(t["nothing"], 1);
    assert_eq!(t.values().sum::<usize>(), 4, "one obligation, one tier");
}

#[test]
fn the_tally_names_every_tier_even_where_none_sits_there() {
    // A tier missing from the report reads as a tier nobody has reached, and a
    // reader cannot tell that from a tier the tool forgot to print.
    let t = tally(&view(&[("obligation::the_thing", &[("what", "a")])]));
    assert_eq!(t.len(), 4, "{t:?}");
    assert_eq!(t["met"], 0);
}

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
        ("obligation::met", &[("what", "a")]),
        ("obligation::nothing", &[("what", "d")]),
        ("ruling::he_said_so", &[("obligation", "met")]),
    ]);
    let (outcome, text) = run(&v, &[]);
    assert!(matches!(outcome, Outcome::Clean { examined: 2 }), "{text}");
    assert!(text.contains("ruling::he_said_so"), "{text}");
    assert!(text.contains("met"), "{text}");
    assert!(text.contains("nothing"), "{text}");
}

#[test]
fn the_report_marks_a_route_closed_obligation_rather_than_letting_it_read_as_untouched() {
    let v = with(&[("retirement::a_dead_end", "obligation")]);
    let (_, text) = run(&v, &[]);
    assert!(
        text.contains("named only by a retirement"),
        "a retired route reads identically to nobody having looked on a flat \
         list, which is the distinction the field was added for: {text}"
    );
}

#[test]
fn the_report_names_an_unanswered_obligation_carrying_a_precondition() {
    let v = view(&[
        ("obligation::the_thing", &[("what", "a demand")]),
        ("law::a_result", &[("precondition_for", "the_thing")]),
    ]);
    let (_, text) = run(&v, &[]);
    assert!(
        text.contains("answered by nothing and carry an established precondition"),
        "{text}"
    );
    assert!(text.contains("the_thing"), "{text}");
}

#[test]
fn control_a_met_obligation_carrying_a_precondition_is_not_in_that_list() {
    // The pair is about being stuck, so an obligation that has been met is not
    // one however many preconditions were established along the way.
    let v = view(&[
        ("obligation::the_thing", &[("what", "a demand")]),
        ("ruling::he_said_so", &[("obligation", "the_thing")]),
        ("law::a_result", &[("precondition_for", "the_thing")]),
    ]);
    let (_, text) = run(&v, &[]);
    assert!(
        !text.contains("answered by nothing and carry an established precondition"),
        "{text}"
    );
}

#[test]
fn one_obligation_can_be_read_in_full_by_its_slug() {
    let v = view(&[
        (
            "obligation::the_thing",
            &[("what", "a demand"), ("note", "a note")],
        ),
        ("obligation::other", &[("what", "another")]),
        ("ruling::he_said_so", &[("obligation", "the_thing")]),
    ]);
    let (outcome, text) = run(&v, &["the_thing"]);
    assert!(matches!(outcome, Outcome::Clean { examined: 1 }), "{text}");
    assert!(text.contains("a demand"), "{text}");
    assert!(text.contains("a note"), "{text}");
    assert!(text.contains("ruling::he_said_so"), "{text}");
    assert!(
        !text.contains("another"),
        "the other row is not reported: {text}"
    );
}

#[test]
fn a_slug_that_names_nothing_is_inconclusive_rather_than_an_empty_report() {
    let v = with(&[]);
    let (outcome, text) = run(&v, &["nosuch"]);
    assert!(matches!(outcome, Outcome::Inconclusive { .. }), "{text}");
    assert!(
        text.contains("spelling"),
        "an empty report here would read as an obligation nothing reaches: {text}"
    );
}

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
        with(&[]),
        with(&[("ruling::r", "obligation")]),
        with(&[("retirement::x", "obligation")]),
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
