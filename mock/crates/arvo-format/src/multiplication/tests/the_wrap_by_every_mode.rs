//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The wrapping product past the slot index under every mode, against a
//! reference that reads no step code.
//!
//! The reference forms the position from the two members' values in `Big`: at
//! phase `n / d` and quantum `q` a member at slot `a` is `(a + n/d) q`, so the
//! product sits at `((a d + n)(b d + n) q - n d) / d^2` in slots. It floors that
//! ratio, rounds it by the mode's own rule (toward zero by the position's sign,
//! half to even by the floor's parity, stochastic against the dither) and wraps
//! the result onto the range modulo its count. The formats are the region the
//! wrapping admission covers: a nonzero phase, signed and unsigned ranges, ranges
//! at both ends of the index, and odd and even counts.

use notko::Maybe;

use super::*;
use crate::apply::{Dither, Fraction};
use crate::multiplication::mul;
use crate::rounding::{Ceil, HalfEven, HalfUp, Mode, Rounding, Stochastic, TowardZero};
use crate::slots::{Signed, Unsigned};
use crate::tests::big::Big;
use crate::width::Width;

/// 199 slots ending at the top of the index: an odd count.
struct Top199;

impl Slots for Top199 {
    const MAX: Slot = Slot::at(i128::MAX);
    const MIN: Slot = Slot::at(i128::MAX - 198);
    const WIDTH: Width = Width::bits(8);
}

/// 200 slots starting at the bottom of the index: an even count.
struct BottomOf200;

impl Slots for BottomOf200 {
    const MAX: Slot = Slot::at(i128::MIN + 199);
    const MIN: Slot = Slot::at(i128::MIN);
    const WIDTH: Width = Width::bits(8);
}

/// `-2^62` to `2^62`: an odd count across zero.
struct OddAcrossZero;

impl Slots for OddAcrossZero {
    const MAX: Slot = Slot::at(1 << 62);
    const MIN: Slot = Slot::at(-(1 << 62));
    const WIDTH: Width = Width::bits(64);
}

type HalfPhased = Grid<BinaryRationals, Constant<3>, Signed<64>, 1, 2>;
type ThirdPhased = Grid<BinaryRationals, Constant<2>, Unsigned<64>, 1, 3>;
type AtTheTop = Grid<BinaryRationals, Constant<-1>, Top199, 0, 1>;
type AtTheBottom = Grid<BinaryRationals, Constant<0>, BottomOf200, 0, 1>;
type QuarterAcross = Grid<BinaryRationals, Constant<5>, OddAcrossZero, 1, 4>;

type Wr<F, R> = Signature<F, Adapt<R, Wrap>>;

/// The phase as `(n, d)` and the quantum as `(numerator, denominator)`, in `Big`.
fn parts<F: Format>() -> (Big, Big, Big, Big) {
    let n = Big::of(i128::from(F::PHASE.numerator()));
    let d = Big::of(i128::from(F::PHASE.denominator()));
    let r = radix::<F>().base() as i128;
    let e = <F::Quantum as Quantum>::BASE.power();
    let (qn, qd) = if e >= 0 {
        (Big::power(r, e as u32), Big::of(1))
    } else {
        (Big::of(1), Big::power(r, e.unsigned_abs()))
    };
    (n, d, qn, qd)
}

/// Whether a value is zero.
fn is_zero(v: Big) -> bool {
    v == Big::of(0)
}

/// The member the reference puts the product of `a` and `b` at.
fn reference<F: Format>(mode: Mode, dither: (i128, i128), a: i128, b: i128) -> i128 {
    let (n, d, qn, qd) = parts::<F>();
    let fa = Big::of(a).times(d).plus(n);
    let fb = Big::of(b).times(d).plus(n);
    let num = fa.times(fb).times(qn).plus(n.times(d).times(qd).negated());
    let den = d.times(d).times(qd);
    let (floor, rest) = num.floor_over_big(den);
    let inexact = !is_zero(rest);
    // Twice the rest against the denominator, as the sign of their difference.
    let against_half = rest.plus(rest).plus(den.negated());
    let above_half = !against_half.is_negative() && !is_zero(against_half);
    let at_half = is_zero(against_half);
    let floor_is_odd = floor.floor_over_big(Big::of(2)).1 == Big::of(1);
    let up = match mode {
        Mode::Floor => false,
        Mode::Ceil => inexact,
        Mode::TowardZero => inexact && num.is_negative(),
        Mode::HalfUp => above_half || at_half,
        Mode::HalfEven => above_half || (at_half && floor_is_odd),
        // Up when the rest is above the dither, `rest / den > dn / dd`.
        Mode::Stochastic => {
            let gap = rest
                .times(Big::of(dither.1))
                .plus(den.times(Big::of(dither.0)).negated());
            !gap.is_negative() && !is_zero(gap)
        },
    };
    let rounded = if up { floor.plus(Big::of(1)) } else { floor };
    let lo = <F::Slots as Slots>::MIN.index();
    let hi = <F::Slots as Slots>::MAX.index();
    let count = Big::of(hi).plus(Big::of(lo).negated()).plus(Big::of(1));
    let offset = rounded.plus(Big::of(lo).negated()).floor_over_big(count).1;
    match Big::of(lo).plus(offset).index() {
        Maybe::Is(slot) => slot,
        Maybe::Isnt => unreachable!("a member of the range is in the index"),
    }
}

/// The pairs read: each end, one and two in from it, the middle and zero where
/// the range holds it.
fn slots<F: Format>() -> impl Iterator<Item = i128> + Clone {
    let lo = <F::Slots as Slots>::MIN.index();
    let hi = <F::Slots as Slots>::MAX.index();
    let ends = [lo, lo + 1, lo + 2, lo + (hi - lo) / 2, hi - 2, hi - 1, hi];
    let near_zero = [-1, 0, 1]
        .into_iter()
        .filter(move |s| (lo ..= hi).contains(s));
    ends.into_iter().chain(near_zero)
}

/// How many pairs `mul` under `R` and wrapping puts anywhere but the reference.
fn strays<F: Format, R: Rounding>(dither: (i64, i64)) -> usize {
    let applied = Dither::at(Fraction::of(dither.0, dither.1));
    let wide = (i128::from(dither.0), i128::from(dither.1));
    let mut strays = 0;
    for a in slots::<F>() {
        for b in slots::<F>() {
            let got = mul::<Wr<F, R>>(Slot::at(a), Slot::at(b), applied).index();
            if got != reference::<F>(R::MODE, wide, a, b) {
                strays += 1;
            }
        }
    }
    strays
}

/// Every mode on one format, with three dithers for the mode that reads one.
fn every_mode<F: Format>() -> [usize; 8] {
    [
        strays::<F, Floor>((0, 1)),
        strays::<F, Ceil>((0, 1)),
        strays::<F, TowardZero>((0, 1)),
        strays::<F, HalfUp>((0, 1)),
        strays::<F, HalfEven>((0, 1)),
        strays::<F, Stochastic>((0, 1)),
        strays::<F, Stochastic>((1, 2)),
        strays::<F, Stochastic>((1, 3)),
    ]
}

#[test]
fn the_wrapping_product_is_the_reference_under_every_mode() {
    assert_eq!(every_mode::<HalfPhased>(), [0; 8], "signed, phase one half");
    assert_eq!(
        every_mode::<ThirdPhased>(),
        [0; 8],
        "unsigned, phase one third"
    );
    assert_eq!(every_mode::<AtTheTop>(), [0; 8], "odd count at the top");
    assert_eq!(
        every_mode::<AtTheBottom>(),
        [0; 8],
        "even count at the bottom"
    );
    assert_eq!(
        every_mode::<QuarterAcross>(),
        [0; 8],
        "odd count across zero"
    );
}

#[test]
fn a_negative_tie_toward_zero_wraps_from_its_own_floor() {
    // `(2^63 - 1 + 1/2)(-3 2^61 + 1/2) 8 - 1/2` is a tie below zero whose floor
    // past the index is one under `2^63 - 2` modulo `2^64`, so toward zero
    // takes it up to `2^63 - 2`. The held pair beside it is the control.
    let a = Slot::at((1 << 63) - 1);
    let b = Slot::at(-3 * (1 << 61));
    assert_eq!(
        mul::<Wr<HalfPhased, TowardZero>>(a, b, Dither::UNUSED),
        Slot::at((1 << 63) - 2)
    );
    assert_eq!(
        mul::<Wr<HalfPhased, TowardZero>>(Slot::at(1), Slot::at(-1), Dither::UNUSED),
        Slot::at(-6)
    );
}

#[test]
fn a_tie_on_an_odd_count_goes_to_its_own_floor_s_even_neighbour() {
    // `a b / 2` with `a b` odd is a tie; the expected members are the wrapped
    // half-to-even roundings of the true position.
    let m = i128::MAX;
    let even_floor = mul::<Wr<AtTheTop, HalfEven>>(Slot::at(m), Slot::at(m - 12), Dither::UNUSED);
    let odd_floor = mul::<Wr<AtTheTop, HalfEven>>(Slot::at(m), Slot::at(m - 10), Dither::UNUSED);
    assert_eq!(
        (even_floor.index(), odd_floor.index()),
        (
            170_141_183_460_469_231_731_687_303_715_884_105_555,
            170_141_183_460_469_231_731_687_303_715_884_105_533
        )
    );
}

#[test]
fn a_range_from_the_bottom_of_the_index_wraps_without_a_panic() {
    let lo = Slot::at(i128::MIN);
    let got = mul::<Wr<AtTheBottom, Floor>>(lo, lo, Dither::UNUSED).index();
    assert_eq!(
        got,
        reference::<AtTheBottom>(Mode::Floor, (0, 1), i128::MIN, i128::MIN)
    );
}

#[test]
fn the_reference_tells_the_modes_apart() {
    // The case that has to fail: a mode checked against another's rule misses,
    // so the sweep can see a wrong rounding.
    let differs = slots::<HalfPhased>().any(|a| {
        slots::<HalfPhased>().any(|b| {
            reference::<HalfPhased>(Mode::TowardZero, (0, 1), a, b)
                != reference::<HalfPhased>(Mode::Floor, (0, 1), a, b)
        })
    });
    assert!(differs);
    let ties_differ = slots::<AtTheTop>().any(|a| {
        slots::<AtTheTop>().any(|b| {
            reference::<AtTheTop>(Mode::HalfEven, (0, 1), a, b)
                != reference::<AtTheTop>(Mode::HalfUp, (0, 1), a, b)
        })
    });
    assert!(ties_differ);
}
