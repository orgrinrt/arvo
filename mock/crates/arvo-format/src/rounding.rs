//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The rounding mode: what happens between grid points.
//!
//! Six names and no more. This is the one closed enumeration in the design, and
//! it is closed because a ratification closed it rather than because six felt
//! like enough.
//!
//! The retired word is absent on purpose, both of its spellings. On a signed
//! domain it named two operations that genuinely differ, so a reader coming from
//! the hardware and a reader coming from C would each have been certain they knew
//! which one was meant. Dropping bits off a two's complement value is `Floor` and
//! it is not `TowardZero`; the note is here so the hardware operation does not get
//! read back into the name.

/// What a value between two grid points becomes.
///
/// Closed: the six below are the whole of it. An implementor outside this crate
/// is not a seventh mode, it is a mode this crate does not know about, and the
/// vocabulary is ratified rather than extensible.
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
    /// To the nearest, and a tie goes away from zero.
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
/// Nearest, ties away from zero.
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

/// Whether the mode's result is a function of the value alone.
///
/// True for five of the six. The odd one out is why a chain carrying it is not
/// reproducible without saying more, and what more it needs is open.
#[must_use]
pub const fn is_deterministic(mode: Mode) -> bool {
    !matches!(mode, Mode::Stochastic)
}

/// Whether the mode always moves toward one fixed direction, regardless of where
/// the value sits between its neighbours.
///
/// The directed modes are the ones a narrowing composes through, and that
/// composition is open in the registry rather than settled here.
#[must_use]
pub const fn is_directed(mode: Mode) -> bool {
    matches!(mode, Mode::TowardZero | Mode::Floor | Mode::Ceil)
}
