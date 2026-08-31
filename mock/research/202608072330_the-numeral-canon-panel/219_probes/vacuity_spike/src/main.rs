//! Does an empty registry look like a clean one, for every arm the crate ships?
//!
//! Seat 220 found that `arvo_checks::load` returns `Ok(empty)` for a path that
//! is not a directory (`checks/src/lib.rs`, `walk`'s first two lines), so
//! `canon()`'s `.expect("mock/registry is readable")` cannot fire. It measured
//! eight arms and predicated its finding at eight of thirty. This measures all
//! thirty, and separates the two failure shapes, because the crate has two:
//! twenty-five arms take a `Registry` and five take a `&Path`.
//!
//! An arm is VACUOUS when it says nothing about the empty input and nothing
//! about the real one either, because then no test written over it can tell the
//! two apart. An arm is GUARDED when it fires on the real input, because a
//! non-empty assertion beside it would notice the silence.
//!
//! The control is the second column: if every arm returned zero on the real
//! corpus too, this program would be measuring its own wiring rather than the
//! crate, so at least one arm must be non-zero there.

use std::path::Path;

use arvo_checks::{
    citation, comments, corpus, load, obligation, predicate, provenance, shape, Registry,
};

fn main() {
    let empty: Registry = load(Path::new("/nonexistent/registry/for/this/spike"))
        .expect("this expect is the point: a missing directory is not an error");
    // `canon()` is NOT used here, and the reason is the finding itself.
    //
    // The first version of this spike called it. It returned **zero rows** and
    // its `.expect("mock/registry is readable")` did not fire, because `repo()`
    // pops two directories off `CARGO_MANIFEST_DIR` and this spike does not sit
    // where the crate does. Without the control on the next line I would have
    // reported all thirty arms vacuous, which would have been a measurement of
    // my own wiring rather than of the crate. That is seat 220's finding,
    // reproduced by accident a second time, in a second way, by somebody who
    // had already read it and still walked into it.
    let real: Registry = load(&repo_registry()).expect("the registry path is given explicitly");

    println!("rows in the empty registry: {}", empty.rows.len());
    println!("rows in the committed canon: {}", real.rows.len());
    assert_eq!(empty.rows.len(), 0, "control: the empty side must be empty");
    assert!(!real.rows.is_empty(), "control: the real side must not be");
    println!();

    let mut vacuous = 0usize;
    let mut guarded = 0usize;

    macro_rules! arm {
        ($name:literal, $f:path) => {{
            let e = $f(&empty).len();
            let r = $f(&real).len();
            let verdict = if r == 0 {
                vacuous += 1;
                "VACUOUS"
            } else {
                guarded += 1;
                "guarded"
            };
            println!(
                "  {:<8} {:<58} empty {:>3}   canon {:>3}",
                verdict, $name, e, r
            );
        }};
    }

    println!("=== the 25 arms that take a Registry ===");
    arm!(
        "citation::line_citations_into_living_ledgers",
        citation::line_citations_into_living_ledgers
    );
    arm!(
        "citation::citations_with_no_target",
        citation::citations_with_no_target
    );
    arm!(
        "comments::comments_counting_their_own_rows",
        comments::comments_counting_their_own_rows
    );
    arm!(
        "obligation::obligation_edges_naming_nothing",
        obligation::obligation_edges_naming_nothing
    );
    arm!(
        "obligation::obligation_edges_from_an_untiered_namespace",
        obligation::obligation_edges_from_an_untiered_namespace
    );
    arm!(
        "obligation::obligations_whose_only_route_is_closed",
        obligation::obligations_whose_only_route_is_closed
    );
    arm!(
        "obligation::preconditions_from_a_namespace_that_cannot_establish_one",
        obligation::preconditions_from_a_namespace_that_cannot_establish_one
    );
    arm!(
        "obligation::unanswered_obligations_carrying_a_precondition",
        obligation::unanswered_obligations_carrying_a_precondition
    );
    arm!(
        "predicate::undeclared_dimensions",
        predicate::undeclared_dimensions
    );
    arm!(
        "predicate::repeated_dimensions",
        predicate::repeated_dimensions
    );
    arm!(
        "provenance::standing_claims_more_arrivals_than_it_cites",
        provenance::standing_claims_more_arrivals_than_it_cites
    );
    arm!(
        "provenance::a_proposal_resting_only_on_a_consolidation",
        provenance::a_proposal_resting_only_on_a_consolidation
    );
    arm!(
        "provenance::an_imposition_resting_on_an_instrument",
        provenance::an_imposition_resting_on_an_instrument
    );
    arm!(
        "shape::rulings_with_no_verbatim",
        shape::rulings_with_no_verbatim
    );
    arm!(
        "shape::refusals_without_an_instead",
        shape::refusals_without_an_instead
    );
    arm!(
        "shape::measured_without_evidence",
        shape::measured_without_evidence
    );
    arm!(
        "shape::measurements_resting_on_an_unusable_instrument",
        shape::measurements_resting_on_an_unusable_instrument
    );
    arm!(
        "shape::predicate_disagrees_with_the_sentence_kind",
        shape::predicate_disagrees_with_the_sentence_kind
    );
    arm!(
        "shape::stamps_from_an_unratified_ruling",
        shape::stamps_from_an_unratified_ruling
    );
    arm!("shape::a_term_defined_twice", shape::a_term_defined_twice);
    arm!(
        "shape::definitions_with_no_term",
        shape::definitions_with_no_term
    );
    arm!(
        "shape::rows_restating_a_retired_claim",
        shape::rows_restating_a_retired_claim
    );
    arm!("shape::rows_with_no_keywords", shape::rows_with_no_keywords);
    arm!(
        "shape::notes_claiming_an_empty_field_that_is_not",
        shape::notes_claiming_an_empty_field_that_is_not
    );
    arm!(
        "shape::retirements_too_short_to_find",
        shape::retirements_too_short_to_find
    );

    println!();
    println!("=== the 5 arms that take a directory ===");
    let nodir = Path::new("/nonexistent/panel/for/this/spike");
    // `corpus::panel_dir()` is crate-relative for the same reason `canon()` is,
    // so it does NOT resolve from this spike. The first run of this section
    // reported all five directory arms vacuous, and that was a fact about my
    // path rather than about the crate. This control is what caught it, and it
    // is the per-column control the whole-run control could not be.
    let panel = repo_registry()
        .parent()
        .expect("mock/")
        .join("research/202608072330_the-numeral-canon-panel");
    assert!(
        panel.is_dir(),
        "control: the real column must name a directory that exists, or both \
         columns measure the same nothing: {}",
        panel.display()
    );
    macro_rules! dirarm {
        ($name:literal, $f:path) => {{
            let e = $f(nodir).len();
            let r = $f(&panel).len();
            let verdict = if r == 0 {
                vacuous += 1;
                "VACUOUS"
            } else {
                guarded += 1;
                "guarded"
            };
            println!(
                "  {:<8} {:<58} missing {:>3}   panel {:>3}",
                verdict, $name, e, r
            );
        }};
    }
    dirarm!(
        "corpus::unprefixed_archive_citations_in_living_ledgers",
        corpus::unprefixed_archive_citations_in_living_ledgers
    );
    dirarm!(
        "corpus::archive_citations_naming_nothing",
        corpus::archive_citations_naming_nothing
    );
    dirarm!(
        "corpus::line_citations_into_the_registry",
        corpus::line_citations_into_the_registry
    );
    dirarm!(
        "corpus::probes_reading_another_tree",
        corpus::probes_reading_another_tree
    );
    dirarm!(
        "corpus::line_citations_into_living_ledgers_in_prose",
        corpus::line_citations_into_living_ledgers_in_prose
    );

    println!();
    println!("of 30 arms: {vacuous} say nothing on either input, {guarded} would notice");
    assert!(
        guarded > 0,
        "control: if nothing fires on the real corpus this measures its own wiring"
    );
}

/// The registry, by an explicit path from this file's own location, because the
/// crate's `repo()` is relative to the crate and this spike is not the crate.
fn repo_registry() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .find(|p| p.join("mock/registry").is_dir())
        .expect("a tree containing mock/registry")
        .join("mock/registry")
}
