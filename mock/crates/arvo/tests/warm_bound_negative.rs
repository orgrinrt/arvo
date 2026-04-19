//! Warm const-bound negative compile test.
//!
//! `UFixed<I, F, Warm>` at `I + F > 32` must fail to compile. The
//! mechanism is the absence of a `UContainerFor<BITS>` impl on `Warm`
//! for `BITS > 32` — the trait bound fails to resolve, yielding a
//! compiler error at use.

#[test]
fn warm_bound_negative() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/warm_bound_negative.rs");
}
