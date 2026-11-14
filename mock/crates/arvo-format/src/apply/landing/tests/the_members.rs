//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Every member of a format lands on itself.
//!
//! A member is `phase radix^BASE + slot radix^s` at some magnitude, and it lands
//! exactly, on the grid, at the finest magnitude that holds it: the first, from
//! the finest, at which its distance from the phase's point is a whole number of
//! steps inside the slot range. That reads the membership predicate and nothing
//! else, so no position formula is shared with the code under test.

use super::*;

/// How many members of `F` land anywhere but on themselves, under the mode `R`.
fn strays<F: Format, R: crate::rounding::Rounding>() -> usize
where
    Under<F, R>: DeclaredSignature<Format = F>,
{
    let r = radix::<F>().base() as i128;
    let (mut p, mut q) = (F::PHASE.numerator() as i128, F::PHASE.denominator() as i128);
    if q < 0 {
        (p, q) = (-p, -q);
    }
    let base = <F::Quantum as Quantum>::BASE.power();
    let count = <F::Quantum as Quantum>::MAGNITUDES.count();
    let (lo, hi) = (
        <F::Slots as crate::slots::Slots>::MIN.index(),
        <F::Slots as crate::slots::Slots>::MAX.index(),
    );
    let finest_first = |i: u32| {
        if <F::Quantum as Quantum>::SLOPE.power() < 0 {
            Magnitude::at(count - 1 - i)
        } else {
            Magnitude::at(i)
        }
    };
    let mut wrong = 0;
    for m in 0 .. count {
        let s = step_exponent::<F>(Magnitude::at(m)).power();
        for k in lo ..= hi {
            // The member as `num / q * r^e`, `e` the lesser of the two exponents.
            let e = if p == 0 { s } else { s.min(base) };
            let phase = if p == 0 { 0 } else { p * power(r, base - e) };
            let num = phase + k * q * power(r, s - e);
            // The finest magnitude holding it, and its slot there.
            let mut want = None;
            for i in 0 .. count {
                let at = finest_first(i);
                let t = step_exponent::<F>(at).power();
                let slot = if t <= s {
                    k * power(r, s - t)
                } else {
                    let ratio = power(r, t - s);
                    if k % ratio != 0 {
                        continue;
                    }
                    k / ratio
                };
                if (lo ..= hi).contains(&slot) {
                    want = Some((at, slot));
                    break;
                }
            }
            let (at, slot) = want.expect("a member is held at its own magnitude");
            let got = land::<Under<F, R>>(
                Scaled::of(num, nz(q as u128), Exponent::of(e)),
                Dither::UNUSED,
            );
            let lands = matches!(got, Maybe::Is(l) if l.magnitude() == at
                && l.exact() == Exact::on_grid(Slot::at(slot)));
            wrong += usize::from(!lands);
        }
    }
    wrong
}

#[test]
fn every_member_of_a_phased_law_lands_on_itself() {
    assert_eq!(strays::<PhasedFour, TowardZero>(), 0);
    assert_eq!(strays::<PhasedFour, HalfEven>(), 0);
    assert_eq!(strays::<FallingPhased, TowardZero>(), 0);
    assert_eq!(strays::<FallingPhased, Ceil>(), 0);
}

#[test]
fn every_member_of_a_law_through_zero_lands_on_itself() {
    assert_eq!(strays::<Floating<4, -6, 8>, Floor>(), 0);
    assert_eq!(strays::<FallingFour, HalfUp>(), 0);
    assert_eq!(strays::<DecimalFive, HalfEven>(), 0);
}
