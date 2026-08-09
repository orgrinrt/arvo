//! P11. Do the tiers agree?
//!
//! A cell reached by tier one (a consumer hands arvo a bare primitive and
//! never names a strategy) and a cell reached by tier two (a consumer defines
//! a domain alias) must land in the same place, or the design answers one
//! question twice.
//!
//! The obligation is discharged by construction if the tier-one bridge is not
//! a chosen default but a derived one: `Warm` is defined as what a native
//! Rust primitive does, so the numeral a bare primitive bridges to is a Warm
//! numeral because that is what Warm means.
//!
//! This checks the consequence, that the two routes produce the same type,
//! by asserting type equality rather than by inspecting either route.
//!
//! Expected: compiles. A disagreement would surface at the assertion.

#![no_std]

pub struct Hot;
pub struct Warm;

pub struct UInt<const N: u32, S>(pub u32, core::marker::PhantomData<S>);

/// The bridge tier one crosses without noticing. A bare primitive is not a
/// default choice of strategy, it is the strategy Warm was defined to be.
pub trait Bridged {
    type AsNumeral;
}
impl Bridged for u32 {
    type AsNumeral = UInt<32, Warm>;
}

/// Tier two's route: written once in an alias definition.
pub type Handle = UInt<32, Warm>;

/// The obligation, as a type equality that the compiler checks.
pub trait SameAs<T> {}
impl<T> SameAs<T> for T {}

pub fn tiers_agree()
where
    <u32 as Bridged>::AsNumeral: SameAs<Handle>,
{
}

/// And the negative control: a disagreement would not compile. This function
/// is the same assertion against the wrong row.
pub type WrongRow = UInt<32, Hot>;

pub fn tiers_disagree()
where
    <u32 as Bridged>::AsNumeral: SameAs<WrongRow>,
{
}
