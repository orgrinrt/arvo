//! The control, as a refusal the suite asserts rather than a build that breaks.
//!
//! It was an example first, which made `cargo test` fail on the control's own
//! success. `trybuild` is what the repository already uses for this, and it
//! asserts the diagnostic rather than merely the failure.

#[test]
fn the_consumer_really_carries_no_gate() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/c1_the_consumer_really_has_no_gate.rs");
}
