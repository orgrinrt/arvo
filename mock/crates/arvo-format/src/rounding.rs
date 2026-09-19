//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The rounding mode: what happens between grid points.
//!
//! Six names and no more. This is the one closed enumeration in the design, and
//! it is closed by the design rather than because six felt like enough.
//!
//! The retired word is absent on purpose, both of its spellings. On a signed
//! domain it named two operations that genuinely differ, so a reader coming from
//! the hardware and a reader coming from C would each have been certain they knew
//! which one was meant. Dropping bits off a two's complement value is `Floor` and
//! it is not `TowardZero`; the note is here so the hardware operation does not get
//! read back into the name.
//!
//! A note travels with `HalfUp` for the same kind of reason. It is `floor(x + q/2)`,
//! a tie going toward positive infinity whatever the sign, so a tie at `-2.5 q`
//! goes to `-2 q`. It is not the `HALF_UP` of Java or Python, which sends that tie
//! the other way; that operation is not one of the six and is reached by shifting
//! the position half a step away from zero and then rounding with `TowardZero`.

/// What a value between two grid points becomes.
///
/// Closed: the six below are the whole of it. An implementor outside this crate
/// is not a seventh mode, it is a mode this crate does not know about, and the
/// vocabulary is closed rather than extensible.
pub trait Rounding {
    /// Which of the six, as a value a const predicate can gate on.
    const MODE: Mode;
}

/// The six names, as one value.
///
/// An enumeration rather than six unrelated markers because an arm gating on the
/// mode needs something to compare, and a const predicate over a closed set is
/// exactly what the design is made of.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    /// Toward zero, so away from negative infinity below zero and away from
    /// positive infinity above it. Not the same operation as `Floor` on a signed
    /// domain, which is the whole reason the ambiguous word was retired.
    TowardZero,
    /// Toward negative infinity. Equal to dropping bits off a two's complement
    /// value, on every row measured.
    Floor,
    /// Toward positive infinity.
    Ceil,
    /// To the nearest, and a tie goes toward positive infinity at every sign,
    /// `floor(x + q/2)`. Not Java's or Python's `HALF_UP`: a tie at `-2.5 q`
    /// goes to `-2 q`.
    HalfUp,
    /// To the nearest, and a tie goes to the neighbour with an even slot.
    HalfEven,
    /// To one of the two neighbours, with probability from the position between
    /// them. The one mode whose result is not a function of the value alone,
    /// which is why it carries its own questions about seeding and keying, and
    /// those are open in the registry rather than answered here.
    Stochastic,
}

/// Toward zero.
pub struct TowardZero;
/// Toward negative infinity.
pub struct Floor;
/// Toward positive infinity.
pub struct Ceil;
/// Nearest, a tie toward positive infinity at every sign, `floor(x + q/2)`.
pub struct HalfUp;
/// Nearest, ties to even.
pub struct HalfEven;
/// Nearest by chance, weighted by position.
pub struct Stochastic;

impl Rounding for TowardZero {
    const MODE: Mode = Mode::TowardZero;
}
impl Rounding for Floor {
    const MODE: Mode = Mode::Floor;
}
impl Rounding for Ceil {
    const MODE: Mode = Mode::Ceil;
}
impl Rounding for HalfUp {
    const MODE: Mode = Mode::HalfUp;
}
impl Rounding for HalfEven {
    const MODE: Mode = Mode::HalfEven;
}
impl Rounding for Stochastic {
    const MODE: Mode = Mode::Stochastic;
}

/// Every mode the vocabulary carries, for a test that wants the whole matrix
/// rather than the rows somebody remembered.
pub const ALL_MODES: [Mode; 6] = [
    Mode::TowardZero,
    Mode::Floor,
    Mode::Ceil,
    Mode::HalfUp,
    Mode::HalfEven,
    Mode::Stochastic,
];

// No predicate over this enumeration says whether a mode is deterministic or
// directed, for the same reason as in `overflow`. Determinism is measured by
// varying the dither the applied map reads, and direction by looking at the slots
// it returns.
