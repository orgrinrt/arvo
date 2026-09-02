//! The seal adversary, in its shipping compile-fail shape.
//!
//! DESTINATION: `mock/crates/arvo/tests/seal_adversary.rs`, fixtures at
//! `mock/crates/arvo/tests/ui/seal_*.rs`, using the trybuild convention
//! already shipping at `mock/crates/arvo/tests/no_multiplicative_identity.rs`
//! (`t.compile_fail(...)` / `t.pass(...)` per case, `Cargo.toml` already
//! carries `trybuild = "1"` as a dev-dependency).
//!
//! WHAT CHANGES ON THE MOVE, stated once so it is not repeated eight times.
//! Every fixture in `ui/` was written against `vu_core`, a hand-built
//! stand-in crate (this directory's own `tower.rs`, an unmodified copy of
//! `46_probes/probe_2_vu_core_lib.rs`) built and linked by a direct `rustc
//! --extern vu_core=libvu_core.rlib` invocation, because the sealed tower
//! (`Pos`/`Nat`/`Adjustment`/`Bias`) has no shipped source anywhere in this
//! repository today (`grep -rln "Adjustment\|Bias\|Numeral" mock/crates/
//! --include="*.rs"` returns nothing, reproduced in the OUTCOMES.md
//! alongside this file). Once the tower lands as real source, three things
//! happen and nothing else:
//!
//! 1. This directory's `tower.rs` is deleted. Trybuild resolves the crate
//!    under test (`arvo`, or whatever crate ends up owning the tower)
//!    through the enclosing package's own `Cargo.toml`, the same way
//!    `no_multiplicative_identity.rs` already resolves `arvo::ufixed::UFixed`
//!    with no manual `--extern` anywhere in that file.
//! 2. Every `use vu_core::bias::Bias;` / `use vu_core::nat::{...}` line in
//!    `ui/*.rs` becomes `use arvo::<wherever the tower lands>::{...}`. The
//!    attack bodies, the const values, the fn-forcing shape: unchanged.
//! 3. Every `.stderr` file's module path segment (`nat::sealed::PosSealed`,
//!    `bias::bias_sealed::BiasSealed`) is re-derived from the real module
//!    tree. The error CODE (E0277, E0603, E0271, E0117, E0210, E0038) is a
//!    fact about the language's coherence and privacy rules, not about this
//!    stand-in's module names, and does not change.
//!
//! WHY A `.stderr` FILE IS THE POINT, NOT A NICETY. A `compile_fail` case
//! with no `.stderr` on record only checks that the crate failed to build.
//! Nothing distinguishes "refused because the seal is intact" from "refused
//! because of a typo in the fixture," and a suite that cannot tell those
//! apart is exactly the adversary-fails-for-the-wrong-reason failure file 46
//! (`46_probes/probe_3d_malformed_types_refused.rs`'s own header) records
//! finding in its own first draft (bare type aliases, which "COMPILED CLEAN
//! while testing nothing," because a type alias defers its bound checks).
//! Every `.stderr` alongside this harness is captured verbatim from a fresh
//! `rustc +nightly-2026-05-28` run against this directory's own `tower.rs`
//! (the exact commands are in OUTCOMES.md), so the error CODE is pinned now.
//! The exact rendered TEXT is expected, not yet confirmed, to match trybuild's
//! own `cargo`-driven rendering byte for byte; running this harness for real
//! (`TRYBUILD=overwrite cargo test --test seal_adversary`) the first time it
//! executes against the shipped tower is what confirms or corrects that, per
//! trybuild's own standard workflow, and it is the same workflow every other
//! `.stderr` file in `mock/crates/arvo/tests/ui/` was bootstrapped from.
//!
//! WHAT THE SUITE COVERS. All four introduction routes the design's own
//! seal guarantee is stated over (`49_consolidation_four.md:364-378`, "In
//! this language there are exactly four routes by which a downstream crate
//! can introduce a new obligation for a foreign trait"), plus the two
//! adversaries specific to this tower's own history (the fabricated-`Pos`
//! replay that reopened the seal a second time, the malformed-type replay
//! that must be fn-forced or it tests nothing) and the type-erasure route
//! that the workspace's own no-`dyn` rule forecloses before coherence is
//! even asked, plus one POSITIVE control: legitimate downstream extension
//! by structural recursion over the public constructors must still compile,
//! because a seal that also refuses the extension the design's own
//! `arvo-toolbox-not-policer.md` rule promises is not a seal, it is a wall.

#[test]
fn seal_refuses_every_introduction_route() {
    let t = trybuild::TestCases::new();

    // Route 1: direct impl of a sealed trait on a local type. E0277 x4,
    // one per carrier (Pos, Nat, Adjustment, Bias). All four in one file
    // because the seal's own checklist (49:379-388) is a per-carrier
    // discipline and a suite that pins only one carrier is a suite that
    // would have missed exactly the gap file 46 found: `Adjustment` was
    // open twice while `Pos`/`Nat`/`Bias` were already sealed.
    t.compile_fail("tests/ui/seal_direct_impl_all_four_carriers.rs");

    // Route 2: implement the private supertrait itself. E0603, the route
    // unnameable before any trait solving happens.
    t.compile_fail("tests/ui/seal_supertrait_unnameable.rs");

    // Regression check: the specific attack that reopened the seal a
    // second time (a foreign Pos with a lying, unconditional Gcd impl),
    // replayed against the CURRENT tower. If this ever compiles clean,
    // the seal has regressed exactly the way it regressed twice before.
    t.compile_fail("tests/ui/seal_fabricated_pos_replay.rs");

    // Malformed genuine types at a bounded position (unreduced Adjustment,
    // unreduced Bias, a padded Pos), forced through a fn signature so a
    // bare type alias cannot defer the check and pass the test by
    // testing nothing.
    t.compile_fail("tests/ui/seal_malformed_types_fn_forced.rs");

    // Route 3: re-implement a sealed trait, or a helper the tower
    // consumes, on a GENUINE inhabitant. E0117, the orphan rule, before
    // any seal or overlap check is consulted.
    t.compile_fail("tests/ui/seal_reimpl_on_genuine_inhabitant.rs");

    // Route 4: a downstream blanket impl over an uncovered type
    // parameter, the route that would mint inhabitants wholesale.
    // E0210.
    t.compile_fail("tests/ui/seal_downstream_blanket.rs");

    // The type-erasure route, checked for completeness even though the
    // workspace's own no-dyn rule forecloses it independently. E0038.
    t.compile_fail("tests/ui/seal_dyn_refused.rs");

    // The positive control: legitimate downstream extension by
    // structural recursion over the public constructors must still
    // compile. A seal with no working positive control is a policer,
    // not a toolbox (arvo-toolbox-not-policer.md).
    t.pass("tests/ui/seal_extension_positive_control.rs");
}
