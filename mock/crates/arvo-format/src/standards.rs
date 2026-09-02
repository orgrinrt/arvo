//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The industrial conventions, as instances of the open inventory.
//!
//! A consumer already thinking in a standard's vocabulary keeps thinking in it
//! and gets arvo underneath. There is no adapter and no second implementation of
//! anything: a convention names a width, a scaling and a rounding-and-overflow
//! pair, and each of those is one of this crate's coordinates under a different
//! word.
//!
//! **Nothing behind the naming is what makes it honest.** A convention needing a
//! computation to work would not be one arvo supports badly, it would be a gap in
//! the primitives, which is what the standards bound is for. So the interesting
//! output here is a refusal, and both of the ones below are claims about this
//! crate rather than about MATLAB.
//!
//! These sit beside `points` for the same reason those are there: the concept is
//! closed and the inventory is open, so an instance joins by supplying the
//! obligations rather than by amending anything.

use crate::ambient::{BinaryRationals, UnsignedBinaryRationals};
use crate::format::{Format, Phase};
use crate::quantum::{Exponent, MagnitudeCount, Quantum};
use crate::slots::{Signed, Slots, Unsigned};

/// MATLAB's fraction length, as the quantum law it names.
///
/// The convention counts fractional bits and this crate carries the exponent of
/// the step, and the two are negations of each other. Written as
/// `Constant<{-F}>` that negation is an expression in a const generic argument,
/// which needs a feature this workspace forbids; written as a law of its own it
/// is an ordinary associated const in an impl body, where an arbitrary
/// expression is legal.
///
/// `F` is signed because MATLAB's fraction length runs negative, and a negative
/// one lands on a positive exponent with no case of its own.
pub struct FractionLength<const F: i32>;

impl<const F: i32> Quantum for FractionLength<F> {
    const BASE: Exponent = Exponent::of(-F);
    // The constant family. A fixed-point convention has one step rather than a
    // ladder of them, so nothing moves with magnitude and there is one magnitude
    // to move over.
    const SLOPE: Exponent = Exponent::ZERO;
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::ONE;
}

/// A signed MATLAB `fi` of `W` bits at fraction length `F`.
///
/// The bound is the slot range's own, so a word length the ladder does not admit
/// is refused where it is written. MathWorks documents the word length to 65535
/// and the ladder stops at 62, so this refuses declarations a consumer writes
/// without thinking about them. That is a gap in the primitives and it is pinned
/// as a compile-fail case rather than worked around here.
pub struct Fi<const W: u32, const F: i32>;

impl<const W: u32, const F: i32> Format for Fi<W, F>
where
    Signed<W>: Slots,
{
    type Ambient = BinaryRationals;
    type Quantum = FractionLength<F>;
    type Slots = Signed<W>;
    // Binary-point scaling puts the grid through zero. The coordinate is written
    // rather than assumed, because the same convention's slope-and-bias scaling
    // does not, and that is a different declaration rather than a flag on this
    // one.
    const PHASE: Phase = Phase::ZERO;
}

/// An unsigned MATLAB `fi` of `W` bits at fraction length `F`.
pub struct Ufi<const W: u32, const F: i32>;

impl<const W: u32, const F: i32> Format for Ufi<W, F>
where
    Unsigned<W>: Slots,
{
    type Ambient = UnsignedBinaryRationals;
    type Quantum = FractionLength<F>;
    type Slots = Unsigned<W>;
    const PHASE: Phase = Phase::ZERO;
}

/// A `fi` under a `fimath`, which is the declared signature the pair names.
///
/// The convention's math settings are part of the signature rather than an
/// argument to an operation, because two settings over one format are two
/// declared signatures and not one with a switch.
pub type FiMath<const W: u32, const F: i32, R, O> =
    crate::adapt::Signature<Fi<W, F>, crate::adapt::Adapt<R, O>>;

/// An unsigned `fi` under a `fimath`.
pub type UfiMath<const W: u32, const F: i32, R, O> =
    crate::adapt::Signature<Ufi<W, F>, crate::adapt::Adapt<R, O>>;

/// MATLAB's rounding methods, under MATLAB's names.
///
/// Re-exports rather than new types, so the mapping is something the compiler
/// agrees with rather than a sentence in a document. Four of the six land here.
pub mod rounding_method {
    pub use crate::rounding::{Ceil as Ceiling, Floor, HalfEven as Convergent, TowardZero as Zero};

    // FIXME: MATLAB's Nearest, ties toward positive infinity, has no mode here.
    // This crate names one nearest-not-to-even mode and what it means is open:
    // `question::which_tie_direction_an_unqualified_nearest_names` says `half_up`
    // is two operations under a reading nobody has settled. Mapping it either way
    // closes that inside a design. Unblocked by that row.

    // FIXME: MATLAB's Round, ties toward the greater absolute value, has no mode
    // here for the same reason, and additionally
    // `question::is_the_rounding_vocabulary_complete_at_six` treats ties away
    // from zero as a mode outside the ratified six. Unblocked by that row.
}

/// MATLAB's overflow actions, under MATLAB's names.
///
/// Both land, so this map is total and nothing here is marked.
pub mod overflow_action {
    pub use crate::overflow::{Saturate, Wrap};
}
