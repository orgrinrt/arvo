//! Probe 4b: probe 4's three committed refusals.
//!
//! SECTION 1 is the negative control on CLAIM A: is the widened return type
//! CHECKED, or merely inferred? A deliberately wrong result numeral must fail,
//! or the projection is decoration.
//!
//! SECTION 2 is CLAIM D's payoff: a constant the numeral cannot hold is refused
//! at the call site, by name, before any code runs.
//!
//! SECTION 3 is the same refusal at the constant the review already found by
//! another route: `Q0_15` has no ONE, so `lambda_max_bound`'s literal 2 has
//! nowhere to land, and `arvo-spectral` would be told so instead of computing
//! a `sigma` out of range.
//!
//! EXPECTED: FAILS x3. Errors verbatim in OUTCOMES.md.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib -L . --extern tower=libtower.rlib
//!   --extern p4=./libp4.rlib probe_4b_the_constant_and_the_numeral_refuse.rs
//!   (p4 built from probe 4 with --crate-name p4)

#![allow(dead_code)]

use p4::{lambda_max_bound, upward_rank_widening, Cap64, Num, Ranks, U8Num, Q0_15};

// SECTION 1. Eight-bit weights over sixty-four nodes widen to fourteen, not to
// ten. Annotating ten must be refused.
pub fn wrong_result_numeral() -> Ranks<Cap64, Num<p4::P10Public>> {
    upward_rank_widening::<Cap64, Num<p4::P8Public>>()
}

// SECTION 2. A constant outside the numeral's range.
pub fn three_hundred_in_a_byte() -> U8Num {
    <U8Num as p4::FromConstantKeyed<300>>::get()
}

// SECTION 3. `lambda_max_bound` needs a 2; `Q0_15` spans `[0, 1)`.
pub fn sigma_on_a_fractional_numeral() -> Q0_15 {
    lambda_max_bound::<Q0_15>()
}
