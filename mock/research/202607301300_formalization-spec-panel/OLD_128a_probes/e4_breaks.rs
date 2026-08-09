//! E4: where the const-keyed numeral breaks. Both candidates, separately.
#![no_std]
#![feature(const_trait_impl)]

pub struct Fix<const I: u16, const F: u16>;
pub trait Numeral {
    const PRECISION: u16;
}
impl<const I: u16, const F: u16> Numeral for Fix<I, F> {
    const PRECISION: u16 = I + F;
}

// Storage keyed on the width, as any width-keyed handoff surface must be.
pub struct Bits<const N: u16>;

// BREAK 1: the width-keyed storage of a numeral.
pub trait Stored: Numeral {
    type Carrier;
}
impl<const I: u16, const F: u16> Stored for Fix<I, F> {
    type Carrier = Bits<{ I + F }>;
}

// BREAK 2: the widening multiply's result numeral.
pub trait Mul<Rhs: Numeral>: Numeral {
    type Out: Numeral;
}
impl<const AI: u16, const AF: u16, const BI: u16, const BF: u16> Mul<Fix<BI, BF>> for Fix<AI, AF> {
    type Out = Fix<{ AI + BI }, { AF + BF }>;
}
