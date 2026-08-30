//! Probe B: the naive reading of D38's divergence use case ("when a value
//! is in R do this, when it is not R but Z do that"), written the way the
//! topic's own prose suggests: as blanket impls of one consumer trait
//! distinguished only by which membership the numeral satisfies.
//!
//! EXPECTED TO FAIL TO COMPILE with E0119. Associated-type-equality
//! where-clauses do not participate in coherence, so two blanket impls
//! over `N: Numeral` conflict even though their `System =` bounds are
//! mutually exclusive. Neither `min_specialization` (the impls are
//! incomparable, neither specialises the other) nor anything else in the
//! permitted feature set changes this.
//!
//! Compile with: rustc --edition 2021 --crate-type lib

#![no_std]

pub struct Zint;
pub struct Dyadic;

pub trait Numeral {
    type System;
}

pub trait Algo {
    const WHICH: u8;
}

// "when it is Z do that"
impl<N: Numeral<System = Zint>> Algo for N {
    const WHICH: u8 = 1;
}

// "when it is Z[1/2] do this"
impl<N: Numeral<System = Dyadic>> Algo for N {
    const WHICH: u8 = 2;
}
