//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The ambient domain a format's values are drawn from.
//!
//! Half of a format's identity, the other half being the representable set. Two
//! formats with the same representable set under different ambient algebras are
//! two formats, which is why the domain is carried rather than assumed.

/// The domain a representable set is a subset of.
///
/// The radix sits here because it is a property of the domain rather than of the
/// step law: the same law at radix 2 and radix 10 describes different values.
pub trait Ambient {
    /// The base positional notation counts in.
    const RADIX: u32;

    /// Whether the domain carries values below zero.
    ///
    /// Read as a coordinate of the domain rather than of the storage. A format
    /// over an unsigned domain has no negative member regardless of what any
    /// carrier could hold.
    const SIGNED: bool;

    /// What an implementor owes, checked rather than asked for.
    ///
    /// The radix is at least two. Below that the coordinate does not describe a
    /// positional notation: at one the step never changes with the exponent so
    /// every magnitude names the same value, and at zero the quantum is not a
    /// number at negative exponents at all. The cancellation the additive
    /// identity turns on is undefined in both.
    ///
    /// `proposal::the_concept_is_closed_and_the_inventory_is_open` says a new
    /// instance earns admission by supplying the concept's obligations, which is
    /// what makes admission a check rather than a negotiation. This is that
    /// check, in the shape `Slots` already carries it.
    ///
    /// **It fires at codegen, not at `cargo check`**, so `cargo build` refuses
    /// and `cargo check` does not. The guarantee is that an inadmissible domain
    /// cannot reach a produced binary; it can reach a passing check, which is why
    /// every predicate reading it stays total.
    ///
    /// ```compile_fail
    /// use arvo_format::ambient::Ambient;
    /// use arvo_format::format::{radix, Format};
    /// use arvo_format::quantum::Constant;
    /// use arvo_format::slots::Signed;
    ///
    /// struct Unary;
    ///
    /// impl Ambient for Unary {
    ///     const RADIX: u32 = 1;
    ///     const SIGNED: bool = true;
    /// }
    ///
    /// struct OverUnary;
    ///
    /// impl Format for OverUnary {
    ///     type Ambient = Unary;
    ///     type Quantum = Constant<0>;
    ///     type Slots = Signed<8>;
    ///     const PHASE_NUM: i64 = 0;
    ///     const PHASE_DEN: i64 = 1;
    /// }
    ///
    /// fn main() {
    ///     let _ = radix::<OverUnary>();
    /// }
    /// ```
    ///
    /// The control, which is what says the refusal above is this obligation and
    /// not something else in the program: the same shape at radix two builds.
    ///
    /// ```
    /// use arvo_format::ambient::Ambient;
    /// use arvo_format::format::{radix, Format};
    /// use arvo_format::quantum::Constant;
    /// use arvo_format::slots::Signed;
    ///
    /// struct Binary;
    ///
    /// impl Ambient for Binary {
    ///     const RADIX: u32 = 2;
    ///     const SIGNED: bool = true;
    /// }
    ///
    /// struct OverBinary;
    ///
    /// impl Format for OverBinary {
    ///     type Ambient = Binary;
    ///     type Quantum = Constant<0>;
    ///     type Slots = Signed<8>;
    ///     const PHASE_NUM: i64 = 0;
    ///     const PHASE_DEN: i64 = 1;
    /// }
    ///
    /// fn main() {
    ///     assert_eq!(radix::<OverBinary>(), 2);
    /// }
    /// ```
    ///
    /// It refuses what the assertion below names and nothing further.
    const ADMITTED: () = {
        assert!(
            Self::RADIX >= 2,
            "a radix below two is not a positional notation: at one every magnitude names the \
             same value, and at zero the quantum is not a number at a negative exponent"
        );
    };
}

/// Whether an ambient domain meets what the contract asks of it.
///
/// The law returning a verdict rather than asserting one, so a construction that
/// compiles and is wrong can be reported on without forcing the const that would
/// refuse it. That is what lets the wrong construction live permanently in a test
/// rather than in a scratch file somebody deletes.
#[must_use]
pub const fn is_admissible_ambient<A: Ambient>() -> bool {
    A::RADIX >= 2
}

/// The rationals at radix two, which is where the fixed-point and binary floating
/// families live.
pub struct BinaryRationals;

impl Ambient for BinaryRationals {
    const RADIX: u32 = 2;
    const SIGNED: bool = true;
}

/// The non-negative rationals at radix two.
pub struct UnsignedBinaryRationals;

impl Ambient for UnsignedBinaryRationals {
    const RADIX: u32 = 2;
    const SIGNED: bool = false;
}

/// The rationals at radix ten, which is where the decimal conventions live.
///
/// Present because the radix is a coordinate and a design that carried only one
/// value of it would have hardcoded a choice the canon leaves open. Nothing in
/// this crate is specialised to radix two.
pub struct DecimalRationals;

impl Ambient for DecimalRationals {
    const RADIX: u32 = 10;
    const SIGNED: bool = true;
}
