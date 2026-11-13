//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Arithmetic the tests carry themselves, for the operations the value hands
//! to the format.
//!
//! The quotient, the conversion and the product at a quantum exponent other
//! than zero are the format's function called on slots, so that function
//! cannot be what they are checked against. What is here computes the same
//! results from the members' values in wide integers: a quotient of two
//! integers rounded by a mode, and a completion onto a range. It reads what a
//! signature declares, its mode, policy, range and exponent, and calls nothing
//! of the format to compute.
//!
//! Every format the tests name is binary, which `exponent` asserts.

use arvo_format::overflow::{Overflow, Policy};
use arvo_format::rounding::{Mode, Rounding};
use arvo_format::{Adaptation, DeclaredSignature, Format, Quantum, radix};
use arvo_numeral::Sole;

use super::range;

/// The rounding mode the signature declares.
fn mode<S: DeclaredSignature>() -> Mode {
    <<S::Adaptation as Adaptation>::Rounding as Rounding>::MODE
}

/// Whether the signature completes by saturation. The other policy a value
/// admits is wrapping.
fn saturates<S: DeclaredSignature>() -> bool {
    matches!(
        <<S::Adaptation as Adaptation>::Overflow as Overflow>::POLICY,
        Policy::Saturate
    )
}

/// The quantum's exponent of a binary format: a slot is that power of two.
fn exponent<F: Format>() -> i32 {
    assert_eq!(radix::<F>().base(), 2, "the tests' arithmetic is binary");
    <F::Quantum as Quantum>::BASE.power()
}

/// `num / den` rounded to an integer by the mode.
pub fn rounded(num: i128, den: i128, mode: Mode) -> i128 {
    assert!(den != 0, "a ratio over zero");
    let (num, den) = if den < 0 { (-num, -den) } else { (num, den) };
    let floor = num.div_euclid(den);
    let rest = num.rem_euclid(den);
    if rest == 0 {
        return floor;
    }
    let up = match mode {
        Mode::Floor => false,
        Mode::Ceil => true,
        Mode::TowardZero => num < 0,
        // A tie goes toward positive infinity.
        Mode::HalfUp => 2 * rest >= den,
        Mode::HalfEven => 2 * rest > den || (2 * rest == den && floor.rem_euclid(2) == 1),
        Mode::Stochastic => unreachable!("a value admits no stochastic mode"),
    };
    if up { floor + 1 } else { floor }
}

/// An integer completed onto the signature's range by its policy.
pub fn completed<S: Sole>(x: i128) -> i128 {
    let (min, max) = range::<S>();
    if saturates::<S>() {
        x.clamp(min, max)
    } else {
        min + (x - min).rem_euclid(max - min + 1)
    }
}

/// What an operation gives: the slot of its result, and whether the rounded
/// result was a member before any completion.
pub struct Landed {
    pub slot: i128,
    pub fits: bool,
}

/// `num / den` slots landed on `S`: rounded by its mode, then completed.
fn landed<S: Sole>(num: i128, den: i128) -> Landed {
    let (min, max) = range::<S>();
    let at = rounded(num, den, mode::<S>());
    Landed {
        slot: completed::<S>(at),
        fits: at >= min && at <= max,
    }
}

/// `x` times two to the `power`, as a ratio.
fn scaled(x: i128, power: i32) -> (i128, i128) {
    if power >= 0 { (x << power, 1) } else { (x, 1 << -power) }
}

/// The quotient of the members at slots `a` and `b`. The two quanta cancel,
/// so the quotient is `a / b` as a number, and its slot is that over the
/// quantum.
pub fn quotient<S: Sole>(a: i128, b: i128) -> Landed {
    let (num, den) = scaled(a, -exponent::<S::Format>());
    landed::<S>(num, den * b)
}

/// The product of the members at slots `a` and `b`: the slots' product times
/// the quantum squared, and its slot is that over the quantum once.
pub fn product<S: Sole>(a: i128, b: i128) -> Landed {
    let (num, den) = scaled(a * b, exponent::<S::Format>());
    landed::<S>(num, den)
}

/// The member at slot `a` of `S`, on the grid of `T`.
pub fn converted<S: Sole, T: Sole>(a: i128) -> Landed {
    let (num, den) = scaled(a, exponent::<S::Format>() - exponent::<T::Format>());
    landed::<T>(num, den)
}
