#![no_std]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

//! arvo-narrow-contracts. Narrow trait declaration.
//!
//! `Narrow<T>` const trait. The unified mask-and-cast bridge that
//! replaces the per-size `from_raw_uN` / `to_raw_uN` helpers from
//! the prior round. Concrete impls live in `arvo-narrow` (cross-
//! primitive) and `arvo-bitmask` (mask-using).
//!
//! See `DESIGN.md` for the full surface.

/// Narrowed-result expression alias.
///
/// Cosmetic alias for the result of `narrow_to::<N>` so consumer
/// signatures read `fn foo() -> Narrowed<13, u16>` instead of bare
/// `u16`. The const-generic `N` is documentation only at the type
/// level (the value has been narrowed to `N` bits but the carrier
/// is still `T`); the precise shape (whether it gains an associated
/// type, becomes a const-bounded newtype, or stays a transparent
/// alias) is open per the doc CL's DOC-iteration carve-out. The
/// transparent form below is the cleanest landing for round
/// 202604271346; future rounds may tighten if a concrete need
/// surfaces.
pub type Narrowed<const N: u8, T> = T;

/// Narrow `Self` to the lowest `N` bits as type `T`.
///
/// `Self` is a wider raw value (a bare primitive or a `Bits<M, S>`
/// with `M > N`). `T` is the target container type. The default
/// body composes `Mask<W>::mask_for_width(N)` from
/// `arvo-mask-contracts` with `BitLogic::and` from
/// `arvo-bits-contracts`, then the `as` operator (sound under the
/// mask precondition) to cast to the target primitive type.
///
/// Per topic Q-C, the trait declaration lives here and the concrete
/// impls live in `arvo-bitmask` (mask-side) and `arvo-narrow`
/// (cross-primitive). Const trait so consumers can call
/// `wide.narrow_to::<13>()` in const fn bodies under generic
/// `Narrow<T>` bounds.
pub const trait Narrow<T> {
    /// Truncate to the lowest `N` bits and return as `T`.
    fn narrow_to<const N: u8>(self) -> T
    where
        Self: Sized;
}
