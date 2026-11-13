//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The exact step against the values it multiplies, and what the obligation
//! admits.
//!
//! The denotation multiplies the two values with the quantum and the phase in and
//! reads the product back as a position. A step that left the quantum and the
//! phase out, the slots' product alone, is kept beside it as the control: it has
//! to be wrong somewhere and right wherever the quantum is one and the phase
//! zero, which says the step reads the scale and the phase and that the
//! denotation would see a step that did not.
//!
//! The conditions are asked through `is_multipliable`, each on both sides of its
//! boundary. The forcing side is the compile-fail cases.

use super::{At, Ratio, Sat, exact_product, members, quantum};
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::{Exact, Fraction};
use crate::format::Format;
use crate::multiplication::{is_multipliable, product_position};
use crate::points::{Biased, Floating, Integer, UFixed};
use crate::quantum::Constant;
use crate::slots::{Signed, Slot, Unsigned};
use crate::tests::dispatch::PerFormat;
use crate::tests::grid::Grid;
use crate::width::Bool;

/// A decimal format over `S` at the quantum `10^E` and zero phase.
type Dec<S, const E: i32> = Grid<DecimalRationals, Constant<E>, S, 0, 1>;

/// A binary format over `S` at the quantum `2^E` and the phase `N / D`.
type Phased<S, const E: i32, const N: i64, const D: i64> =
    Grid<BinaryRationals, Constant<E>, S, N, D>;

// --- the denotation ------------------------------------------------------------

/// The denotation at one format, with the step that leaves the quantum and the
/// phase out.
#[derive(Default)]
struct Denotation {
    formats:             u32,
    pairs:               u64,
    control_wrong:       u64,
    control_wrong_plain: u64,
}

impl PerFormat for Denotation {
    fn run<F: Format>(&mut self) {
        let q = quantum::<F>();
        let plain = q == Ratio::whole(1) && Ratio::phase_of::<F>() == Ratio::whole(0);
        self.formats += 1;
        for a in members::<F>() {
            for b in members::<F>() {
                let want = exact_product::<F>(a, b).position();
                assert_eq!(product_position::<F>(a, b), want, "{a:?} * {b:?} at {q:?}");
                self.pairs += 1;
                let control = Ratio::whole(a.index() * b.index()).position();
                if control != want {
                    self.control_wrong += 1;
                    if plain {
                        self.control_wrong_plain += 1;
                    }
                }
            }
        }
    }
}

/// Every exponent from minus eight to eight over one slot range.
macro_rules! every_exponent {
    ($walk:ident; $slots:ty) => {
        every_exponent!(@ $walk; $slots;
            -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8);
    };
    (@ $walk:ident; $slots:ty; $($e:literal),+) => {
        $( $walk.run::<At<$slots, { $e }>>(); )+
    };
}

#[test]
fn the_exact_step_is_the_product_of_the_values_read_back_as_a_position() {
    let mut walk = Denotation::default();
    every_exponent!(walk; Signed<3>);
    every_exponent!(walk; Unsigned<3>);
    walk.run::<Integer<4>>();
    walk.run::<UFixed<4, -2>>();
    assert_eq!(walk.formats, 2 * 17 + 2);
    assert_eq!(walk.pairs, 2 * 17 * 64 + 2 * 256);
    assert!(
        walk.control_wrong > 0,
        "the quantum-free step agreed everywhere"
    );
    assert_eq!(walk.control_wrong_plain, 0);
}

#[test]
fn the_exact_step_reads_the_phase_and_the_radix() {
    let mut walk = Denotation::default();
    walk.run::<Biased<4, 0, 1>>();
    walk.run::<Biased<4, -1, 1>>();
    walk.run::<Biased<4, 2, 3>>();
    walk.run::<Phased<Signed<3>, 0, 1, 3>>();
    walk.run::<Phased<Unsigned<3>, -2, 2, 3>>();
    walk.run::<Phased<Signed<3>, 1, -5, 3>>();
    walk.run::<Phased<Signed<3>, -1, 7, -2>>();
    walk.run::<Dec<Signed<3>, -2>>();
    walk.run::<Dec<Unsigned<3>, 1>>();
    walk.run::<Dec<Signed<3>, 0>>();
    walk.run::<Grid<DecimalRationals, Constant<-1>, Signed<3>, 1, 3>>();
    assert_eq!(walk.formats, 11);
    assert_eq!(walk.pairs, 3 * 256 + 8 * 64);
    // Every one of these reads a phase or a quantum other than one, apart from
    // the decimal one at exponent zero, which is the plain integer grid.
    assert!(walk.control_wrong > 0);
    assert_eq!(walk.control_wrong_plain, 0);
}

#[test]
fn the_exact_step_holds_at_the_largest_remainder_denominator() {
    let mut walk = Denotation::default();
    walk.run::<At<Signed<4>, -62>>();
    walk.run::<At<Unsigned<4>, -62>>();
    walk.run::<Dec<Signed<4>, -18>>();
    walk.run::<Phased<Signed<4>, 0, 1, 3_037_000_499>>();
    walk.run::<Phased<Signed<4>, -60, 1, 2>>();
    assert_eq!(walk.pairs, 5 * 256);
    // One times one at `2^-62` is the slot zero and a remainder of `1 / 2^62`,
    // and at `10^-18` a remainder of `1 / 10^18`.
    assert_eq!(
        product_position::<At<Signed<4>, -62>>(Slot::at(1), Slot::at(1)),
        Exact::between(Slot::ZERO, Fraction::of(1, 1 << 62))
    );
    assert_eq!(
        product_position::<Dec<Signed<4>, -18>>(Slot::at(1), Slot::at(1)),
        Exact::between(Slot::ZERO, Fraction::of(1, 1_000_000_000_000_000_000))
    );
}

#[test]
fn a_negative_product_moved_down_floors_and_reduces_its_remainder() {
    // -3 * 1 at a quarter is -3/4: the slot -1 and a quarter.
    assert_eq!(
        product_position::<At<Signed<4>, -2>>(Slot::at(-3), Slot::at(1)),
        Exact::between(Slot::at(-1), Fraction::of(1, 4))
    );
    // -2 * 1 at a quarter is -1/2: the slot -1 and a half, not two quarters.
    assert_eq!(
        product_position::<At<Signed<4>, -2>>(Slot::at(-2), Slot::at(1)),
        Exact::between(Slot::at(-1), Fraction::HALF)
    );
    // At a phase of a half and quantum one, the members at zero are a half, and
    // a half squared sits at a quarter less the phase: the slot -1 and 3/4.
    assert_eq!(
        product_position::<Biased<4, 0, 1>>(Slot::ZERO, Slot::ZERO),
        Exact::between(Slot::at(-1), Fraction::of(3, 4))
    );
}

#[test]
fn a_position_past_the_index_saturates_on_its_sign() {
    type F = Integer<8>;
    let top = Slot::at(i128::MAX);
    let bottom = Slot::at(i128::MIN);
    assert_eq!(product_position::<F>(top, top), Exact::on_grid(top));
    assert_eq!(product_position::<F>(bottom, bottom), Exact::on_grid(top));
    assert_eq!(product_position::<F>(top, bottom), Exact::on_grid(bottom));
    assert_eq!(
        product_position::<F>(Slot::at(-2), top),
        Exact::on_grid(bottom)
    );
    // Moved up past the index on a positive exponent, the same way.
    assert_eq!(
        product_position::<At<Signed<8>, 4>>(Slot::at(i128::MAX / 4), Slot::at(2)),
        Exact::on_grid(top)
    );
    // Past the four limbs, `-2^200` moved up `2^124`, still on the sign.
    assert_eq!(
        product_position::<At<Signed<2>, 124>>(Slot::at(-(1 << 100)), Slot::at(1 << 100)),
        Exact::on_grid(bottom)
    );
    // And a zero product stays zero however far it moves.
    assert_eq!(
        product_position::<At<Signed<8>, 8>>(Slot::ZERO, top),
        Exact::on_grid(Slot::ZERO)
    );
    assert_eq!(
        product_position::<At<Signed<2>, 124>>(Slot::ZERO, top),
        Exact::on_grid(Slot::ZERO)
    );
    // At a phase, the factors past the index are still read on their sign.
    assert_eq!(
        product_position::<Biased<8, 0, 1>>(bottom, top),
        Exact::on_grid(bottom)
    );
}

// --- the conditions ------------------------------------------------------------

#[test]
fn the_obligation_refuses_a_phase_that_names_nothing() {
    assert!(!is_multipliable::<Phased<Signed<8>, 0, 1, 0>>().get());
    assert!(!is_multipliable::<Phased<Signed<8>, 0, 0, 0>>().get());
    assert!(is_multipliable::<Phased<Signed<8>, 0, 1, 1>>().get());
}

#[test]
fn the_obligation_admits_a_phase() {
    assert!(is_multipliable::<Biased<8, 0, 1>>().get());
    assert!(is_multipliable::<Biased<8, 0, 0>>().get());
    assert!(is_multipliable::<Phased<Signed<8>, -3, 2, 3>>().get());
    assert!(is_multipliable::<Phased<Unsigned<8>, 2, -1, 3>>().get());
    assert!(is_multipliable::<Biased<64, 0, 1>>().get());
}

#[test]
fn the_obligation_admits_a_radix_other_than_two() {
    assert!(is_multipliable::<Dec<Signed<8>, 0>>().get());
    assert!(is_multipliable::<Dec<Signed<8>, -2>>().get());
    assert!(is_multipliable::<Dec<Unsigned<8>, 3>>().get());
}

#[test]
fn the_obligation_holds_out_a_magnitude_indexed_quantum() {
    assert!(!is_multipliable::<Floating<4, -3, 4>>().get());
    assert!(!is_multipliable::<Floating<24, -149, 254>>().get());
    assert!(is_multipliable::<Integer<4>>().get());
}

#[test]
fn the_obligation_holds_out_a_remainder_the_position_cannot_hold() {
    // `2^62` and `10^18` are the largest powers a positive signed 64-bit value
    // holds.
    assert!(is_multipliable::<At<Signed<4>, -62>>().get());
    assert!(!is_multipliable::<At<Signed<4>, -63>>().get());
    assert!(!is_multipliable::<At<Signed<4>, { i32::MIN }>>().get());
    assert!(is_multipliable::<Dec<Signed<4>, -18>>().get());
    assert!(!is_multipliable::<Dec<Signed<4>, -19>>().get());
    // The phase's denominator enters squared: `3037000499^2` is under `2^63`
    // and `3037000500^2` is past it.
    assert!(is_multipliable::<Phased<Signed<4>, 0, 1, 3_037_000_499>>().get());
    assert!(!is_multipliable::<Phased<Signed<4>, 0, 1, 3_037_000_500>>().get());
    // Reduced first: `2 / 6074001000` is `1 / 3037000500`, and
    // `6074000998 / 6074000998` is a whole slot.
    assert!(!is_multipliable::<Phased<Signed<4>, 0, 2, 6_074_001_000>>().get());
    assert!(is_multipliable::<Phased<Signed<4>, 0, 6_074_000_998, 6_074_000_998>>().get());
    // And it meets the quantum's: a half at `2^-60` is `2^62`, at `2^-61` `2^63`.
    assert!(is_multipliable::<Phased<Signed<4>, -60, 1, 2>>().get());
    assert!(!is_multipliable::<Phased<Signed<4>, -61, 1, 2>>().get());
}

#[test]
fn the_obligation_holds_out_a_product_the_index_cannot_carry() {
    // The signed 64-bit range's largest product is 2^126.
    assert!(is_multipliable::<Integer<64>>().get());
    // The unsigned one's top corner is just under 2^128 at exponent zero, and
    // under 2^127 once it moves down one place.
    assert!(!is_multipliable::<UFixed<64, 0>>().get());
    assert!(is_multipliable::<UFixed<64, -1>>().get());
    assert!(is_multipliable::<UFixed<64, -62>>().get());
    assert!(is_multipliable::<UFixed<63, 0>>().get());
    // An up move counts: 2^126 moved one place is past the index, and the
    // unsigned 63-bit corner moved one place is under 2^127.
    assert!(!is_multipliable::<At<Signed<64>, 1>>().get());
    assert!(is_multipliable::<UFixed<63, 1>>().get());
    assert!(!is_multipliable::<UFixed<63, 2>>().get());
    // A move wider than the index holds out any range with a nonzero corner, at
    // either radix.
    assert!(!is_multipliable::<At<Signed<2>, 127>>().get());
    assert!(is_multipliable::<At<Signed<2>, 124>>().get());
    assert!(is_multipliable::<Dec<Signed<2>, 37>>().get());
    assert!(!is_multipliable::<Dec<Signed<2>, 38>>().get());
}

/// The verdict and the step, bound at check time.
const _MULTIPLIABLE: Bool = is_multipliable::<Integer<8>>();
const _HELD_OUT: Bool = is_multipliable::<Floating<4, -3, 4>>();
const _QUARTER: Exact = product_position::<UFixed<8, -2>>(Slot::at(1), Slot::at(3));
const _SIXTEEN: Exact = product_position::<At<Signed<8>, 2>>(Slot::at(2), Slot::at(2));
const _PHASED: Exact = product_position::<Biased<8, 0, 1>>(Slot::at(1), Slot::at(1));
const _DECIMAL: Exact = product_position::<Dec<Signed<8>, -1>>(Slot::at(3), Slot::at(4));

#[test]
fn the_step_and_the_verdict_evaluate_at_check_time() {
    assert!(_MULTIPLIABLE.get());
    assert!(!_HELD_OUT.get());
    assert_eq!(_QUARTER, Exact::between(Slot::ZERO, Fraction::of(3, 4)));
    assert_eq!(_SIXTEEN, Exact::on_grid(Slot::at(16)));
    // 1.5 squared is 2.25, at the slot 1.75 under a phase of a half.
    assert_eq!(_PHASED, Exact::between(Slot::at(1), Fraction::of(3, 4)));
    // 0.3 times 0.4 is 0.12, at the slot 1.2 at a tenth.
    assert_eq!(_DECIMAL, Exact::between(Slot::at(1), Fraction::of(1, 5)));
    // Reached through a signature as well, so the composition forces nothing
    // the verdict does not report.
    assert_eq!(
        crate::multiplication::mul::<Sat<Integer<8>>>(
            Slot::at(12),
            Slot::at(12),
            crate::apply::Dither::UNUSED
        ),
        Slot::at(127)
    );
}
