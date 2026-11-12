//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The exact step against the values it multiplies, and what the obligation
//! refuses.
//!
//! The denotation multiplies the two values with the quantum in and reads the
//! product back as a position. A step that left the quantum out, the slots'
//! product alone, is kept beside it as the control: it has to be wrong wherever
//! the exponent is not zero and right where it is, which says the step reads the
//! scale and that the denotation would see a step that did not.
//!
//! The refusals are asked through `is_multipliable`, each on both sides of its
//! boundary. The forcing side is the compile-fail cases.

use super::{At, Ratio, Sat, members, quantum};
use crate::ambient::DecimalRationals;
use crate::apply::{Exact, Fraction};
use crate::format::Format;
use crate::multiplication::{is_multipliable, product_position};
use crate::points::{Biased, Floating, Integer, UFixed};
use crate::quantum::Constant;
use crate::slots::{Signed, Slot, Unsigned};
use crate::tests::dispatch::PerFormat;
use crate::tests::grid::Grid;
use crate::width::Bool;

// --- the denotation ------------------------------------------------------------

/// The value of the member at `slot` in a zero-phase format.
fn value<F: Format>(slot: Slot) -> Ratio {
    Ratio::whole(slot.index()).times(quantum::<F>())
}

/// The denotation at one format, with the step that leaves the quantum out.
#[derive(Default)]
struct Denotation {
    formats:             u32,
    pairs:               u64,
    control_wrong:       u64,
    control_wrong_whole: u64,
}

impl PerFormat for Denotation {
    fn run<F: Format>(&mut self) {
        let q = quantum::<F>();
        self.formats += 1;
        for a in members::<F>() {
            for b in members::<F>() {
                // A value `v` sits at position `v / q` at zero phase.
                let want = value::<F>(a).times(value::<F>(b)).over(q).position();
                assert_eq!(product_position::<F>(a, b), want, "{a:?} * {b:?} at {q:?}");
                self.pairs += 1;
                let control = Ratio::whole(a.index() * b.index()).position();
                if control != want {
                    self.control_wrong += 1;
                    if q == Ratio::whole(1) {
                        self.control_wrong_whole += 1;
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
    // The control is wrong somewhere and nowhere the quantum is one.
    assert!(
        walk.control_wrong > 0,
        "the quantum-free step agreed everywhere"
    );
    assert_eq!(walk.control_wrong_whole, 0);
}

#[test]
fn the_exact_step_holds_at_the_largest_remainder_denominator() {
    // `2^-62` leaves a remainder over up to `2^62`, the largest power of two a
    // position's denominator holds.
    let mut walk = Denotation::default();
    walk.run::<At<Signed<4>, -62>>();
    walk.run::<At<Unsigned<4>, -62>>();
    walk.run::<At<Signed<4>, -61>>();
    assert_eq!(walk.pairs, 3 * 256);
    // One times one at `2^-62` is the slot zero and a remainder of `1 / 2^62`.
    assert_eq!(
        product_position::<At<Signed<4>, -62>>(Slot::at(1), Slot::at(1)),
        Exact::between(Slot::ZERO, Fraction::of(1, 1 << 62))
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
}

#[test]
fn a_slot_product_past_the_index_saturates_on_its_sign() {
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
    // And a zero product stays zero however far it moves.
    assert_eq!(
        product_position::<At<Signed<8>, 8>>(Slot::ZERO, top),
        Exact::on_grid(Slot::ZERO)
    );
}

// --- the refusals --------------------------------------------------------------

#[test]
fn the_obligation_refuses_a_phase() {
    assert!(is_multipliable::<At<Signed<8>, 0>>().get());
    assert!(
        !is_multipliable::<Grid<crate::ambient::BinaryRationals, Constant<0>, Signed<8>, 1, 1>>()
            .get()
    );
    assert!(!is_multipliable::<Biased<8, 0, 1>>().get());
    assert!(is_multipliable::<Biased<8, 0, 0>>().get());
}

#[test]
fn the_obligation_refuses_a_radix_other_than_two() {
    assert!(!is_multipliable::<Grid<DecimalRationals, Constant<0>, Signed<8>, 0, 1>>().get());
    assert!(!is_multipliable::<Grid<DecimalRationals, Constant<-2>, Signed<8>, 0, 1>>().get());
}

#[test]
fn the_obligation_refuses_a_magnitude_indexed_quantum() {
    assert!(!is_multipliable::<Floating<4, -3, 4>>().get());
    assert!(!is_multipliable::<Floating<24, -149, 254>>().get());
    assert!(is_multipliable::<Integer<4>>().get());
}

#[test]
fn the_obligation_refuses_a_remainder_the_position_cannot_hold() {
    assert!(is_multipliable::<At<Signed<4>, -62>>().get());
    assert!(!is_multipliable::<At<Signed<4>, -63>>().get());
    assert!(!is_multipliable::<At<Signed<4>, { i32::MIN }>>().get());
}

#[test]
fn the_obligation_refuses_a_product_the_index_cannot_carry() {
    // The signed 64-bit range's largest product is 2^126.
    assert!(is_multipliable::<Integer<64>>().get());
    // The unsigned one's top corner is just under 2^128, and one bit narrower
    // it is under 2^126.
    assert!(!is_multipliable::<UFixed<64, 0>>().get());
    assert!(is_multipliable::<UFixed<63, 0>>().get());
    // An up move counts: 2^126 moved one place is past the index, and the
    // unsigned 63-bit corner moved one place is under 2^127.
    assert!(!is_multipliable::<At<Signed<64>, 1>>().get());
    assert!(is_multipliable::<UFixed<63, 1>>().get());
    assert!(!is_multipliable::<UFixed<63, 2>>().get());
    // A down move never refuses on carrying.
    assert!(is_multipliable::<UFixed<63, -40>>().get());
    // A move wider than the index refuses any range with a nonzero corner.
    assert!(!is_multipliable::<At<Signed<2>, 127>>().get());
    assert!(is_multipliable::<At<Signed<2>, 124>>().get());
}

/// The verdict and the step, bound at check time.
const _MULTIPLIABLE: Bool = is_multipliable::<Integer<8>>();
const _REFUSED: Bool = is_multipliable::<Floating<4, -3, 4>>();
const _QUARTER: Exact = product_position::<UFixed<8, -2>>(Slot::at(1), Slot::at(3));
const _SIXTEEN: Exact = product_position::<At<Signed<8>, 2>>(Slot::at(2), Slot::at(2));

#[test]
fn the_step_and_the_verdict_evaluate_at_check_time() {
    assert!(_MULTIPLIABLE.get());
    assert!(!_REFUSED.get());
    assert_eq!(_QUARTER, Exact::between(Slot::ZERO, Fraction::of(3, 4)));
    assert_eq!(_SIXTEEN, Exact::on_grid(Slot::at(16)));
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
