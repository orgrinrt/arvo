//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Whether a format's representable set is one arithmetic progression.
//!
//! The set's own question, where `is_constant_family` asks the law's slope.
//! Wrapping is admitted exactly on such a set, so this is what the landing reads
//! for a wrapping signature. The phase is one offset shared by every magnitude and
//! moves the whole set, so it is not read.

use crate::format::{Format, radix};
use crate::quantum::Quantum;
use crate::slots::Slots;
use crate::width::Bool;

/// Whether the members of `F`, over every magnitude, are one arithmetic progression.
///
/// A constant quantum is one. Over several magnitudes the members are the finest
/// grid's slot range and each coarser grid's multiples of its own step, and in
/// units of the finest step the members a coarser magnitude adds past what the
/// finer ones hold must be at most the one next point at each end, since the next
/// two of its points are two finest steps apart. A one-slot range adds one member
/// per magnitude, `slot radix^(k |SLOPE|)`, and three of those are never equally
/// spaced, so it is one exactly while it has at most two. Over a radix of two or
/// more every set this admits spans at most two magnitudes with slots within -1 to
/// 1, and a power or product past the slot index is a member past a gap.
#[must_use]
#[inline]
pub const fn is_one_progression<F: Format>() -> Bool {
    let () = <F::Quantum as Quantum>::ADMITTED;
    let () = <F::Slots as Slots>::ADMITTED;
    let count = <F::Quantum as Quantum>::MAGNITUDES.count();
    let slope = <F::Quantum as Quantum>::SLOPE.power();
    if slope == 0 || count <= 1 {
        return Bool::TRUE;
    }
    let lowest = <F::Slots as Slots>::MIN.index();
    let highest = <F::Slots as Slots>::MAX.index();
    if lowest == highest {
        return Bool::of(lowest == 0 || count <= 2);
    }
    // The ratio of one magnitude's step to the next finer one's.
    let mut ratio: i128 = 1;
    let mut taken = 0;
    while taken < slope.unsigned_abs() {
        ratio = match ratio.checked_mul(radix::<F>().base() as i128) {
            Some(next) => next,
            None => return Bool::FALSE,
        };
        taken += 1;
    }
    // What the magnitudes walked so far hold, in finest steps: `lo` to `hi`, every
    // point between them a member.
    let (mut lo, mut hi) = (lowest, highest);
    let mut scale: i128 = 1;
    let mut m = 1;
    while m < count {
        scale = match scale.checked_mul(ratio) {
            Some(next) => next,
            None => return Bool::FALSE,
        };
        let (Some(top), Some(bottom)) = (highest.checked_mul(scale), lowest.checked_mul(scale))
        else {
            return Bool::FALSE;
        };
        if top > hi {
            if top - hi != 1 {
                return Bool::FALSE;
            }
            hi = top;
        }
        if bottom < lo {
            if lo - bottom != 1 {
                return Bool::FALSE;
            }
            lo = bottom;
        }
        m += 1;
    }
    Bool::TRUE
}
