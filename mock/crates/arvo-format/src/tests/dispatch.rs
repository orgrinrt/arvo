//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! One dispatch over the adaptation, and one walk over the admitted widths.
//!
//! A law quantified over every mode and every shipped policy is a law over
//! eighteen signatures, and each of them is a distinct type, so a suite walking
//! them needs a match that turns the two runtime names into a type. The symmetry
//! cross, the totality of the applied map and addition's suite all walk that
//! matrix, and the match lives here once rather than once per suite, so a mode or
//! a policy the vocabulary gains is one arm in this file and every suite then
//! reaches it.
//!
//! The width walk is the same shape one coordinate over. A law stated for every
//! admitted width is a law over a hundred and twenty-eight formats, each a type,
//! and the walk names them from one literal list whose coverage a test checks
//! against `ADMITTED_WIDTHS` rather than trusting.

use crate::adapt::{Adapt, DeclaredSignature, Signature};
use crate::format::Format;
use crate::overflow::{Clamp, Policy, Saturate, Wrap};
use crate::points::{Integer, UFixed};
use crate::rounding::{Ceil, Floor, HalfEven, HalfUp, Mode, Stochastic, TowardZero};

/// Something to do at one declared signature.
///
/// What comes back is what a consumer declaring that signature gets, because
/// every arm of the dispatch below is a real instantiation.
pub(crate) trait PerSignature {
    /// What it answers with.
    type Out;

    /// Do it at one signature.
    fn run<S: DeclaredSignature>(&self) -> Self::Out;
}

/// The signature over `F` that the two names select, and `what` run at it.
pub(crate) fn at<F: Format, P: PerSignature>(mode: Mode, policy: Policy, what: &P) -> P::Out {
    macro_rules! at_policy {
        ($md:ty) => {
            match policy {
                Policy::Wrap => what.run::<Signature<F, Adapt<$md, Wrap>>>(),
                Policy::Saturate => what.run::<Signature<F, Adapt<$md, Saturate>>>(),
                Policy::Clamp => what.run::<Signature<F, Adapt<$md, Clamp>>>(),
            }
        };
    }
    match mode {
        Mode::TowardZero => at_policy!(TowardZero),
        Mode::Floor => at_policy!(Floor),
        Mode::Ceil => at_policy!(Ceil),
        Mode::HalfUp => at_policy!(HalfUp),
        Mode::HalfEven => at_policy!(HalfEven),
        Mode::Stochastic => at_policy!(Stochastic),
    }
}

/// Something to do at one format.
pub(crate) trait PerFormat {
    /// Do it at one format.
    fn run<F: Format>(&mut self);
}

/// `what` run at every admitted width of both slot families.
///
/// `Integer<W>` is the signed family and `UFixed<W, 0>` the unsigned one, both
/// at a constant quantum and no phase, so the only thing moving across the walk
/// is the slot range. The list is written out because a const generic argument
/// has to be a literal here; the test that uses the walk checks it against the
/// admitted set.
pub(crate) fn every_width<P: PerFormat>(what: &mut P) {
    macro_rules! walk {
        ($($w:literal),+ $(,)?) => {
            $(
                what.run::<Integer<$w>>();
                what.run::<UFixed<$w, 0>>();
            )+
        };
    }
    walk!(
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
    );
}
