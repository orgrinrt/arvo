//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A value divided by a value read at run time, against the tests' own
//! arithmetic.
//!
//! The divisor is a member of a second format on the dividend's grid whose
//! range leaves the zero out. The value read at run time is landed there by
//! `Member::landed`, which is in range by its totality and reads nothing to
//! decide anything, and `Divisor::member` takes it. The zero is the caller's to
//! branch on before, as a site whose zero has an outcome of its own does. Here
//! the branch only counts the zero, so that the sweep is seen to take it.
//!
//! Every member of the 8-bit ranges divides by every other value of its range,
//! landed in a divisor range that holds it or one it passes, at three
//! exponents, under the ten adaptations.

mod common;

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::overflow::Saturate;
use arvo_format::quantum::Constant;
use arvo_format::rounding::HalfEven;
use arvo_format::slots::{Slot, Slots};
use arvo_format::width::Width;
use arvo_format::{Adapt, Member, Signature};
use arvo_numeral::{Divisor, Num, Quiet, Sole};
use common::{oracle, range, value};

/// The slots 1 to 255, the unsigned 8-bit range less the zero.
struct Unsigned;

impl Slots for Unsigned {
    const MAX: Slot = Slot::at(255);
    const MIN: Slot = Slot::at(1);
    const WIDTH: Width = Width::bits(8);
}

/// The slots 1 to 127, the positive half of the signed 8-bit range.
struct Positive;

impl Slots for Positive {
    const MAX: Slot = Slot::at(127);
    const MIN: Slot = Slot::at(1);
    const WIDTH: Width = Width::bits(8);
}

/// The slots -128 to -1, inside the signed 8-bit range.
struct Negative;

impl Slots for Negative {
    const MAX: Slot = Slot::at(-1);
    const MIN: Slot = Slot::at(-128);
    const WIDTH: Width = Width::bits(8);
}

/// A format on the radix-two grid at exponent `E` over the range `R`.
struct Beside<R, const E: i32>(core::marker::PhantomData<R>);

impl<R: Slots, const E: i32> Format for Beside<R, E> {
    type Ambient = BinaryRationals;
    type Quantum = Constant<E>;
    type Slots = R;

    const PHASE: Phase = Phase::ZERO;
}

#[derive(Default)]
struct Counts {
    checked: u64,
    wrong:   u64,
    zeros:   u64,
    landed:  u64,
    passed:  u64,
    first:   Option<(i128, i128, i128, i128, bool, bool)>,
}

/// Every member of `S` over every value of `S` landed in the divisor's range `G`,
/// the zero branched on first. A value past `G` lands at the end of `G` it
/// passed, which is the saturation the landing names, and the quotient is by
/// that slot.
fn by_values<S: Sole, G: Format>(counts: &mut Counts) {
    let (min, max) = range::<S>();
    for n in min ..= max {
        // The value read at run time, a count or a length at a real site.
        let read: Num<S, Quiet> = value::<S>(n);
        if n == 0 {
            counts.zeros += 1;
            continue;
        }
        let g_min = <G::Slots as Slots>::MIN.index();
        let g_max = <G::Slots as Slots>::MAX.index();
        let member = Member::<G>::landed::<Signature<G, Adapt<HalfEven, Saturate>>>(read.slot());
        let lands = n.clamp(g_min, g_max);
        if lands != n {
            counts.passed += 1;
        }
        if member.slot().index() != lands {
            counts.landed += 1;
        }
        let by = Divisor::<S::Format>::member(member);
        for a in min ..= max {
            counts.checked += 1;
            let expected = oracle::quotient::<S>(a, lands);
            let dividend = value::<S>(a);
            let got = dividend.div(by);
            let fits = dividend.fits_div(by).get();
            if got.slot().index() != expected.slot || fits != expected.fits {
                counts.wrong += 1;
                if counts.first.is_none() {
                    counts.first =
                        Some((a, n, got.slot().index(), expected.slot, fits, expected.fits));
                }
            }
        }
    }
}

#[test]
fn a_value_divides_by_a_value_read_at_run_time_at_three_exponents() {
    let mut c = Counts::default();
    macro_rules! sweep {
        ($e:literal) => {{
            fn unsigned<S: Sole>(c: &mut Counts) {
                by_values::<S, Beside<Unsigned, $e>>(c);
                by_values::<S, Beside<Positive, $e>>(c);
            }
            fn signed<S: Sole>(c: &mut Counts) {
                by_values::<S, Beside<Positive, $e>>(c);
                by_values::<S, Beside<Negative, $e>>(c);
            }
            every_adaptation!(arvo_format::points::UFixed<8, $e>, unsigned(&mut c));
            every_adaptation!(arvo_format::points::Fixed<8, $e>, signed(&mut c));
        }};
    }
    sweep!(0);
    sweep!(-3);
    sweep!(-7);
    assert_eq!(
        c.wrong, 0,
        "`div` by a member divisor disagrees with the oracle on {} of {} quotients, the first (a, b, got, expected, fits, expected fits) {:?}",
        c.wrong, c.checked, c.first
    );
    // Each range: 255 divisors by 256 dividends, unsigned onto 1 to 255 and onto
    // 1 to 127, signed onto the positive half and the negative.
    assert_eq!(c.checked, 3 * 10 * 4 * 255 * 256, "the sweep's size");
    // The zero is met once a pass, and the branch keeps it off the divisor.
    assert_eq!(c.zeros, 3 * 10 * 4, "the zero branch");
    // Every landing is the end of the range it passed, or the value itself.
    assert_eq!(
        c.landed, 0,
        "`Member::landed` lands a value off the saturation"
    );
    // Past the range: 128 unsigned onto 1 to 127, the 128 negatives onto the
    // positive half, the 127 positives onto the negative.
    assert_eq!(
        c.passed,
        3 * 10 * (128 + 128 + 127),
        "the values landed from past the range"
    );
}
