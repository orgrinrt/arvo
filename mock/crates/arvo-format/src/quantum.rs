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

    /// What an implementor owes, checked rather than asked for.
    ///
    /// The law ranges over at least one magnitude. A law over none describes no
    /// values, so the set it parameterises is empty, and an empty set contains
    /// nothing, the additive identity included.
    ///
    /// **This is the sentence `MAGNITUDES` above already carried, moved from a
    /// request to a check.** Prose asking an implementor for something is a thing
    /// nothing enforces, and this crate's `Slots` had already made that
    /// distinction while this trait had not.
    ///
    /// **It fires at codegen, not at `cargo check`.** A const is evaluated when
    /// the instantiation is codegened and `check` skips that, so `cargo build`
    /// refuses and `cargo check` does not. Measured rather than assumed:
    /// `rustc --emit=metadata` accepts the refused program below while catching
    /// an ordinary type error in the same invocation. So the guarantee is that an
    /// inadmissible law cannot reach a produced binary, and it can reach a
    /// passing check, which is why the predicates reading a magnitude range stay
    /// total: `has_additive_identity` answers false for an empty range rather
    /// than relying on this to have refused it.
    ///
    /// A doctest is what expresses that refusal, because it builds a binary where
    /// `trybuild` checks one and so never reaches the evaluation.
    ///
    /// ```compile_fail
    /// use arvo_format::quantum::{magnitude_in_range, Quantum};
    ///
    /// struct NoMagnitudes;
    ///
    /// impl Quantum for NoMagnitudes {
    ///     const BASE: i32 = 0;
    ///     const SLOPE: i32 = 0;
    ///     const MAGNITUDES: u32 = 0;
    /// }
    ///
    /// fn main() {
    ///     let _ = magnitude_in_range::<NoMagnitudes>(0);
    /// }
    /// ```
    ///
    /// And the control, which is what says the refusal above is this obligation
    /// rather than anything else in the program: the same shape with one
    /// magnitude builds.
    ///
    /// ```
    /// use arvo_format::quantum::{magnitude_in_range, Quantum};
    ///
    /// struct OneMagnitude;
    ///
    /// impl Quantum for OneMagnitude {
    ///     const BASE: i32 = 0;
    ///     const SLOPE: i32 = 0;
    ///     const MAGNITUDES: u32 = 1;
    /// }
    ///
    /// fn main() {
    ///     assert!(magnitude_in_range::<OneMagnitude>(0));
    /// }
    /// ```
    ///
    /// It refuses what the assertion below names and nothing further. In
    /// particular it does not check that the exponent at the largest admitted
    /// magnitude fits the integer `exponent_at` computes it in, which is a real
    /// hazard at a very large magnitude count and is unaddressed rather than
    /// covered.
    const ADMITTED: () = {
        assert!(
            Self::MAGNITUDES >= 1,
            "a law over no magnitudes describes no values, so it parameterises the empty set \
             rather than a representable one"
        );
    };
}

/// Whether a quantum law meets what the contract asks of it.
///
/// The verdict form, so a construction that compiles and is wrong can be reported
/// on without forcing the const that would refuse it. That is what lets the wrong
/// construction live permanently in a test rather than in a scratch file somebody
/// deletes.
#[must_use]
pub const fn is_admissible_quantum<Q: Quantum>() -> bool {
    Q::MAGNITUDES >= 1
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
    let () = Q::ADMITTED;
    Q::BASE + Q::SLOPE * (magnitude as i32)
}

/// Whether a magnitude is one the law ranges over.
#[must_use]
pub const fn magnitude_in_range<Q: Quantum>(magnitude: u32) -> bool {
    let () = Q::ADMITTED;
    magnitude < Q::MAGNITUDES
}

/// Whether the law is the constant-quantum family.
///
/// This is the axis the two families differ on and the only one, so a claim about
/// one family is a claim predicated on this.
#[must_use]
pub const fn is_constant_family<Q: Quantum>() -> bool {
    let () = Q::ADMITTED;
    Q::SLOPE == 0
}
