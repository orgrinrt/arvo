//! Probe 8: file 47 established that stating a mismatch as a trait bound
//! (E0277) rather than a type equality (E0308) is what makes
//! `#[diagnostic::on_unimplemented]` reachable at all (`47:probe_6`,
//! `Definite`). Probe 7 established that a concrete newtype-per-numeral
//! keeps the diagnostic's own type names readable. This probe combines
//! both levers on the numeral-mismatch case specifically, to see the best
//! message the two known-good instruments can jointly produce.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   probe_8_the_strongest_combination.rs

#![allow(dead_code)]

pub trait Numeral {
    const DISPLAY_VALUE: u64;
}

pub struct Q37;
impl Numeral for Q37 {
    const DISPLAY_VALUE: u64 = 37;
}
pub struct Q53;
impl Numeral for Q53 {
    const DISPLAY_VALUE: u64 = 53;
}

#[diagnostic::on_unimplemented(
    message = "expected accumulator width `{Wanted}`, this one is `{Self}`",
    label = "declared with the wrong numeral face",
    note = "faces are minted only by the numeral-literal macro; if this is the \
            right VALUE but the wrong SPELLING, re-emit it from the macro rather \
            than editing the face by hand"
)]
pub trait SameFaceAs<Wanted: Numeral>: Numeral {}
impl<N: Numeral> SameFaceAs<N> for N {}

pub struct Accumulator<N: Numeral>(core::marker::PhantomData<N>);

pub fn needs_q37<N: SameFaceAs<Q37>>(_: Accumulator<N>) {}

pub fn consumer(acc: Accumulator<Q53>) {
    needs_q37(acc);
}
