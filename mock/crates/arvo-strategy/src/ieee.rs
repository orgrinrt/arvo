//! IEEE float-width seal.
//!
//! `Ieee` carries the bit-width of an IEEE 754 float type plus a
//! sealed-implementor commitment that pairs the width with the const
//! `Identity` and `Bounded` surfaces shipping from
//! `arvo-strategy::arith` for `f32` and `f64`. Relocates from
//! `arvo/src/float.rs` in round 202605030400 so the trait declaration
//! sits in the same crate as its supertraits.
//!
//! `FromU8Ieee` is the lossless `u8 -> IEEE float` bridge. It rides
//! alongside `Ieee` because both target the same f32 / f64 boundary
//! and both carry the same orphan-rule constraints.
//!
//! The seal admits `f32` and `f64` today. Future widenings (`f16` /
//! `bf16` / `f128`) extend the seal here plus the existing
//! `impl_bounded_identity_f!` macro invocation in
//! `arvo-strategy::arith` so the supertraits stay satisfied.

use crate::const_convert::ConstFrom;
use crate::identity::{Additive, Bounded, Identity, Multiplicative};
use crate::Width;

mod sealed {
    /// Hidden supertrait used to seal `Ieee`.
    pub trait Sealed {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// IEEE float width marker. Sealed: implementable only for `f32`
/// and `f64`.
///
/// Carries both identities and `[const] Bounded` as supertraits so
/// consumers reach for `<F as Identity<Additive>>::IDENTITY` / `<F as Identity<Multiplicative>>::IDENTITY`
/// / `<F as Bounded>::MIN` / `<F as Bounded>::MAX` rather than
/// type-specific inherent constants. The supertrait commitment is
/// load-bearing: every future float implementor (the seal admits
/// `f32` / `f64` today; later additions like `f16` / `bf16` / `f128`
/// would require seal expansion in this file plus `arvo-strategy::arith`'s
/// `impl_bounded_identity_f!` extension to cover the new type at the
/// substrate-level Bounded / Identity boundary).
pub const trait Ieee:
    sealed::Sealed
    + Copy
    + Default
    + PartialEq
    + PartialOrd
    + [const] Identity<Additive>
    + [const] Identity<Multiplicative>
    + [const] Bounded
    + 'static
{
    /// Width of this IEEE type in bits.
    const WIDTH: Width;
}

const impl Ieee for f32 {
    const WIDTH: Width = <Width as ConstFrom<u16>>::const_from(32);
}

const impl Ieee for f64 {
    const WIDTH: Width = <Width as ConstFrom<u16>>::const_from(64);
}

/// Lossless `u8 -> IEEE float` bridge.
pub const trait FromU8Ieee: Ieee {
    /// Convert the given `u8` into this IEEE float type.
    fn from_u8_ieee(n: u8) -> Self;
}

const impl FromU8Ieee for f32 {
    #[inline(always)]
    fn from_u8_ieee(n: u8) -> Self {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: IEEE float construction at substrate boundary; lossless 0..=255 -> f32; tracked: #312
        n as f32
    }
}

const impl FromU8Ieee for f64 {
    #[inline(always)]
    fn from_u8_ieee(n: u8) -> Self {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: IEEE float construction at substrate boundary; lossless 0..=255 -> f64; tracked: #312
        n as f64
    }
}
