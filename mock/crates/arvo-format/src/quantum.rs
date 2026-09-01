//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The quantum law: how the step between neighbouring values changes with magnitude.
//!
//! The canon says membership is one affine predicate, so the law is affine in the
//! magnitude and carries two coordinates rather than a function. A slope of zero
//! is the constant-quantum family, where integers, fixed point and scaled integers
//! live. A nonzero slope is the magnitude-indexed family, where floating point
//! lives and where subnormals fall out of the smallest magnitude rather than out
//! of a special case.
//!
//! Carrying it as coordinates instead of a method is what lets membership be a
//! free `const fn` over a format's associated items. A trait method would have to
//! be const to be reachable at stage zero, and that needs a feature this crate is
//! not permitted.

/// How the quantum varies with magnitude, as an affine law.
///
/// The quantum at magnitude `m` is `radix^(BASE + SLOPE * m)`. The radix lives on
/// the ambient domain because it is a property of the domain rather than of the
/// step law.
pub trait Quantum {
    /// The exponent at magnitude zero.
    const BASE: i32;

    /// The change in exponent per step of magnitude. Zero for the constant family.
    const SLOPE: i32;

    /// How many distinct magnitudes the law ranges over. One for the constant
    /// family, and never zero: a law over no magnitudes describes no values.
    const MAGNITUDES: u32;
}

/// A step that does not change with magnitude.
///
/// `EXP` is the exponent, so `EXP = 0` is the integers and `EXP = -F` is fixed
/// point at fraction width `F`.
pub struct Constant<const EXP: i32>;

impl<const EXP: i32> Quantum for Constant<EXP> {
    const BASE: i32 = EXP;
    const SLOPE: i32 = 0;
    const MAGNITUDES: u32 = 1;
}

/// A step that grows by one exponent per magnitude, which is the floating shape.
///
/// `MIN_EXP` is the exponent at the smallest magnitude and `COUNT` is how many
/// magnitudes there are.
pub struct Indexed<const MIN_EXP: i32, const COUNT: u32>;

impl<const MIN_EXP: i32, const COUNT: u32> Quantum for Indexed<MIN_EXP, COUNT> {
    const BASE: i32 = MIN_EXP;
    const SLOPE: i32 = 1;
    const MAGNITUDES: u32 = COUNT;
}

/// The exponent of the quantum at a magnitude.
///
/// Free rather than a trait method so it is callable at stage zero without the
/// trait having to be const.
#[must_use]
pub const fn exponent_at<Q: Quantum>(magnitude: u32) -> i32 {
    Q::BASE + Q::SLOPE * (magnitude as i32)
}

/// Whether a magnitude is one the law ranges over.
#[must_use]
pub const fn magnitude_in_range<Q: Quantum>(magnitude: u32) -> bool {
    magnitude < Q::MAGNITUDES
}

/// Whether the law is the constant-quantum family.
///
/// This is the axis the two families differ on and the only one, so a claim about
/// one family is a claim predicated on this.
#[must_use]
pub const fn is_constant_family<Q: Quantum>() -> bool {
    Q::SLOPE == 0
}
