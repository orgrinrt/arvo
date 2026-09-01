//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The ambient domain a format's values are drawn from.
//!
//! Half of a format's identity, the other half being the representable set. Two
//! formats with the same representable set under different ambient algebras are
//! two formats, which is why the domain is carried rather than assumed.
//!
//! The radix is a coordinate of the domain and carries a type of its own, so a
//! crate outside this one supplies it in the same type the shipped domains do.

use crate::width::Bool;

/// The base a positional notation counts in.
///
/// A coordinate of the domain rather than of the step law: the same law at radix
/// two and radix ten describes different values. It is a base and not a count of
/// anything, which is why it is this rather than a `Width`.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Radix(u32);

impl Radix {
    /// Base two.
    pub const BINARY: Self = Self(2);

    /// Base ten.
    pub const DECIMAL: Self = Self(10);

    /// A radix from a base.
    #[must_use]
    pub const fn of(base: u32) -> Self {
        Self(base)
    }

    /// The base, for the one place a host contract needs it back.
    ///
    /// The unwrap door, declared as one. `repr(transparent)` and this accessor are
    /// the whole observation surface.
    #[must_use]
    pub const fn base(self) -> u32 {
        self.0
    }

    /// Whether two domains count in the same base.
    #[must_use]
    pub const fn equals(self, other: Self) -> Bool {
        Bool::of(self.0 == other.0)
    }

    /// Whether this base describes a positional notation at all.
    ///
    /// The condition `Ambient::ADMITTED` refuses, written over the coordinate so
    /// the obligation and the verdict read one predicate rather than restating it.
    /// At one the step never changes with the exponent, so every magnitude names
    /// the same value; at zero the quantum is not a number at a negative exponent.
    #[must_use]
    pub const fn is_positional(self) -> Bool {
        Bool::of(self.0 >= 2)
    }
}

/// The domain a representable set is a subset of.
///
/// The radix sits here because it is a property of the domain rather than of the
/// step law: the same law at radix 2 and radix 10 describes different values.
pub trait Ambient {
    /// The base positional notation counts in.
    const RADIX: Radix;

    /// Whether the domain carries values below zero.
    ///
    /// Read as a coordinate of the domain rather than of the storage. A format
    /// over an unsigned domain has no negative member regardless of what any
    /// carrier could hold.
    const SIGNED: Bool;

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
    /// check, in the shape `Slots` and `Quantum` already carry it.
    ///
    /// **It fires at codegen, not at `cargo check`**, so `cargo build` refuses
    /// and `cargo check` does not. The guarantee is that an inadmissible domain
    /// cannot reach a produced binary; it can reach a passing check, which is why
    /// every predicate reading it stays total.
    ///
    /// ```compile_fail
    /// use arvo_format::ambient::{Ambient, Radix};
    /// use arvo_format::format::{radix, Format, Phase};
    /// use arvo_format::quantum::Constant;
    /// use arvo_format::slots::Signed;
    /// use arvo_format::width::Bool;
    ///
    /// struct Unary;
    ///
    /// impl Ambient for Unary {
    ///     const RADIX: Radix = Radix::of(1);
    ///     const SIGNED: Bool = Bool::TRUE;
    /// }
    ///
    /// struct OverUnary;
    ///
    /// impl Format for OverUnary {
    ///     type Ambient = Unary;
    ///     type Quantum = Constant<0>;
    ///     type Slots = Signed<8>;
    ///     const PHASE: Phase = Phase::ZERO;
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
    /// use arvo_format::ambient::{Ambient, Radix};
    /// use arvo_format::format::{radix, Format, Phase};
    /// use arvo_format::quantum::Constant;
    /// use arvo_format::slots::Signed;
    /// use arvo_format::width::Bool;
    ///
    /// struct Binary;
    ///
    /// impl Ambient for Binary {
    ///     const RADIX: Radix = Radix::of(2);
    ///     const SIGNED: Bool = Bool::TRUE;
    /// }
    ///
    /// struct OverBinary;
    ///
    /// impl Format for OverBinary {
    ///     type Ambient = Binary;
    ///     type Quantum = Constant<0>;
    ///     type Slots = Signed<8>;
    ///     const PHASE: Phase = Phase::ZERO;
    /// }
    ///
    /// fn main() {
    ///     assert_eq!(radix::<OverBinary>(), Radix::BINARY);
    /// }
    /// ```
    ///
    /// It refuses what the assertion below names and nothing further.
    const ADMITTED: () = {
        assert!(
            Self::RADIX.is_positional().get(),
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
pub const fn is_admissible_ambient<A: Ambient>() -> Bool {
    A::RADIX.is_positional()
}

/// The rationals at radix two, which is where the fixed-point and binary floating
/// families live.
pub struct BinaryRationals;

impl Ambient for BinaryRationals {
    const RADIX: Radix = Radix::BINARY;
    const SIGNED: Bool = Bool::TRUE;
}

/// The non-negative rationals at radix two.
pub struct UnsignedBinaryRationals;

impl Ambient for UnsignedBinaryRationals {
    const RADIX: Radix = Radix::BINARY;
    const SIGNED: Bool = Bool::FALSE;
}

/// The rationals at radix ten, which is where the decimal conventions live.
///
/// Present because the radix is a coordinate and a design that carried only one
/// value of it would have hardcoded a choice the canon leaves open. Nothing in
/// this crate is specialised to radix two.
pub struct DecimalRationals;

impl Ambient for DecimalRationals {
    const RADIX: Radix = Radix::DECIMAL;
    const SIGNED: Bool = Bool::TRUE;
}
