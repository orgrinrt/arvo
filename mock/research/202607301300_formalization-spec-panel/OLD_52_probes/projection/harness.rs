//! The projection-chain compile-fail pair, in its shipping shape.
//!
//! DESTINATION: `mock/crates/arvo/tests/grade_projection_chain.rs`,
//! fixtures at `mock/crates/arvo/tests/ui/grade_projected.rs` and
//! `mock/crates/arvo/tests/ui/reduce_bound_wall.rs`, using the same
//! trybuild convention as `seal_adversary.rs` (this directory's sibling).
//!
//! WHAT THIS PINS. Section 1.11 of `49_consolidation_four.md` (lines
//! 306-324) states the design rule the grade projection depends on for its
//! safety: "every trait in a chain that reaches a consumer-facing signature
//! either pattern-matches on constructor heads or has finite, non-recursive
//! obligations; `Reduce`, and anything routed through it, never appears in
//! such a chain, only at concrete numerals." File 47's fold-grade projection
//! (`ui/grade_projected.rs`, this directory's copy of
//! `47_probes/probe_3_the_grade_is_projected.rs`) stays clear of the
//! composition wall (`E0275`, `overflow evaluating the requirement`) only
//! because it happens to obey that rule, not because anything in its own
//! signature states it. File 48 (`ui/reduce_bound_wall.rs`, this directory's
//! copy of `48_probes/probe_1_the_wall_is_one_refactor_away.rs`) shows the
//! collision is one where-clause away: routing the fold's own safety margin
//! through `Reduce` (a "publish the reduced headroom ratio" refactor that
//! reads as a reasonable simplification, not as a red flag) reproduces the
//! identical divergence in a consumer-facing signature.
//!
//! WHY THE PAIR SHIPS TOGETHER, NEVER SEPARATELY. A positive control alone
//! (grade_projected compiles) proves the projection works today; it says
//! nothing about which refactors would break it tomorrow. A negative
//! control alone (reduce_bound_wall fails) proves ONE bad shape fails; it
//! says nothing about whether the good shape still works. Shipped as a
//! pair, a future change that either (a) breaks the working projection or
//! (b) accidentally makes the wall-triggering refactor start compiling
//! (meaning the trait-solver's eager-confirmation behaviour changed, per
//! this file's own second stderr note) is caught by name, immediately,
//! rather than discovered when a THIRD file tries the same "simplify the
//! headroom computation" refactor for its own unrelated reason and hits an
//! E0275 nobody can explain without re-deriving the whole wall history
//! (files 41, 42, 46, 48).
//!
//! WHAT CHANGES ON THE MOVE. Identical shape to the seal package's own
//! header: this directory's `tower.rs` / `vu_nat.rs` / `vu_bias.rs` are
//! deleted (trybuild resolves the real crate automatically once the tower
//! ships), `use tower::{...}` in both ui files becomes `use arvo::<the
//! tower's eventual module path>::{...}`, the `.stderr` for
//! `reduce_bound_wall.rs` needs no module-path edit at all: `E0275`,
//! `overflow evaluating the requirement`, and the `ExactDivOdd`/`Reduce`
//! names it carries are facts about the SEALED TOWER's own internal
//! recursion, unaffected by which downstream crate imports it.
//! `grade_projected.rs` needs no `.stderr` at all (`t.pass`, not
//! `t.compile_fail`); a change to its own body that starts failing IS the
//! regression this half of the pair exists to catch.

#[test]
fn grade_projection_avoids_the_wall_one_refactor_reopens_it() {
    let t = trybuild::TestCases::new();

    // Positive control: the fold's published grade, projected as an
    // associated-type chain rather than a caller-declared const. Must
    // compile with no unstable feature.
    t.pass("tests/ui/grade_projected.rs");

    // Negative control: the plausible refactor (route the fold's own
    // safety margin through Reduce) that reopens the composition wall
    // files 41/42/46 already closed once, inside a consumer-facing
    // signature this time rather than at an abstract type-solver probe.
    t.compile_fail("tests/ui/reduce_bound_wall.rs");
}
