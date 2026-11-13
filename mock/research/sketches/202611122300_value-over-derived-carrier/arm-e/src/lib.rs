//! Arm E. `min_generic_const_args`.
//!
//! No per-width list: the rung is an associated const of the slot family at
//! every width, computed by `narrowest_carrier`, and used as a const argument to
//! a family of four impls, one per rung. The expected outcome is a refusal, and
//! its text is the result (`outputs/arm-e.txt`).

#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

use arvo_format::Slots;
use arvo_format::slots::Signed;
use arvo_placement::narrowest_carrier;

/// The rung a slot family lands on, in bits.
///
/// `type const` is this nightly's spelling; the `#[type_const]` attribute the
/// brief named is not known to it (feature `attribute-syntax`,
/// `outputs/arm-e-attribute-syntax.txt`).
#[cfg(not(feature = "attribute-syntax"))]
pub trait RungOf {
    type const RUNG: u32;
}

/// The brief's spelling, kept so its refusal has a source.
#[cfg(feature = "attribute-syntax")]
pub trait RungOf {
    #[type_const]
    const RUNG: u32;
}

impl<const W: u32> RungOf for Signed<W>
where
    Signed<W>: Slots,
{
    #[cfg(not(any(feature = "const-block", feature = "attribute-syntax")))]
    type const RUNG: u32 = narrowest_carrier(<Signed<W> as Slots>::WIDTH).count();
    // The wrapping the first refusal asks for.
    #[cfg(feature = "const-block")]
    type const RUNG: u32 = const { narrowest_carrier(<Signed<W> as Slots>::WIDTH).count() };
    #[cfg(feature = "attribute-syntax")]
    #[type_const]
    const RUNG: u32 = narrowest_carrier(<Signed<W> as Slots>::WIDTH).count();
}

/// A rung, by its width in bits.
pub struct Rung<const BITS: u32>;

/// The signed machine integer of a rung: the four-impl family.
pub trait SignedCarrier {
    type T: Copy;
}
impl SignedCarrier for Rung<8> {
    type T = i8;
}
impl SignedCarrier for Rung<16> {
    type T = i16;
}
impl SignedCarrier for Rung<32> {
    type T = i32;
}
impl SignedCarrier for Rung<64> {
    type T = i64;
}

/// The carrier of a signed width, through the rung, with no per-width list.
pub type SignedRepr<const W: u32> = <Rung<{ <Signed<W> as RungOf>::RUNG }> as SignedCarrier>::T;

/// One use at a concrete width, so the projection is resolved and not only parsed.
pub const THIRTEEN: SignedRepr<13> = 0;
