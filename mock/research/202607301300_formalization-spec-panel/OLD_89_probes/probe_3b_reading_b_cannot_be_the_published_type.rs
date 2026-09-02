//! Probe 3b (EXPECTED FAIL): reading B's count cannot inhabit the design's own published
//! grade, because that grade is a type.
//!
//! The design's caller contract is an ordinary type mismatch (`37:441-444`): a caller
//! needing a fold whose definedness matches the sequential one takes a `Folded<0>` and is
//! refused a `Folded<1>`. The published grade is an associated-type projection over types
//! (`49:464-475`). Reading A's count is a function of the operation's type and inhabits
//! that contract with no ceremony (probe 3, `_A_ADD_64`). Reading B's count is a function
//! of the data, and this is what happens when it is asked to inhabit the same contract.
//!
//! This is not an argument that reading B is wrong. It is the precise statement of what
//! reading B costs: its count cannot be the published TYPE, so a design taking reading B
//! publishes a value beside the result rather than a parameter of the result's type, and
//! the caller's contract stops being a type mismatch and becomes something the caller
//! must branch on. That is a design consequence, derived here rather than asserted.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_3b_reading_b_cannot_be_the_published_type.rs
//! Outcome: FAILS WITH E0435 (attempt to use a non-constant value in a constant), as intended.
//! rustc 1.98.0-nightly (57d06900f 2026-05-27), aarch64-apple-darwin.

#![no_std]

/// The design's published grade carrier, in the shape file 49 gives it: the grade is a
/// parameter of the result's type, so the caller's contract is a type.
pub struct Folded<const EVENTS: u32>(pub i64);

const FRAC: u32 = 4;

/// Reading A: the count is a function of the operation and the arity, both types. This
/// half compiles, and is here as the positive control so the refusal below is about
/// reading B rather than about the shape of `Folded`.
pub fn fold_reading_a(xs: &[i64; 64]) -> Folded<64> {
    let mut acc = 0i64;
    let mut i = 0usize;
    while i < 64 {
        acc = (acc + xs[i]) >> FRAC << FRAC;
        i += 1;
    }
    Folded(acc)
}

/// Reading B: the count is the number of adds that actually moved the value. It is known
/// only after the data has been walked, and the design's contract wants it in type
/// position.
pub fn fold_reading_b(
    xs: &[i64; 64],
) -> Folded<
    {
        /* nothing statable here */
        0
    },
> {
    let mut acc = 0i64;
    let mut ev = 0u32;
    let mut i = 0usize;
    while i < 64 {
        let exact = acc + xs[i];
        ev += ((exact & ((1i64 << FRAC) - 1)) != 0) as u32;
        acc = exact >> FRAC << FRAC;
        i += 1;
    }
    // The whole point: `ev` is the honest published grade under reading B, and this is the
    // line the design's contract asks for.
    Folded::<ev>(acc)
}
