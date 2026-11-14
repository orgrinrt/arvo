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

use notko::Maybe;

use super::{At, Ratio, Sat, exact_product, members, quantum};
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::{Exact, Fraction, Remainder};
use crate::format::Format;
use crate::multiplication::{is_multipliable, product_position};
use crate::points::{Biased, Floating, Integer, UFixed};
use crate::quantum::Constant;
use crate::slots::{Signed, Slot, Unsigned};
use crate::tests::dispatch::PerFormat;
use crate::tests::grid::Grid;
use crate::wide::Wide;
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
    #[inline(never)]
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
    // `2^127`, `10^38` and `(2^63)^2` are under `2^128`, so each remainder is
    // held over its whole denominator, past what a `Fraction` carries. Read
    // against the value by hand: the rational oracle's own integer cannot form
    // these, and the edge sweep checks them against the wide one.
    assert_eq!(
        product_position::<At<Signed<4>, -127>>(Slot::at(1), Slot::at(1)),
        Exact::at(Slot::ZERO, Remainder::of(1, 1 << 127))
    );
    assert_eq!(
        product_position::<At<Signed<4>, -127>>(Slot::at(-1), Slot::at(1)),
        Exact::at(Slot::at(-1), Remainder::of((1 << 127) - 1, 1 << 127))
    );
    assert_eq!(
        product_position::<Dec<Signed<4>, -38>>(Slot::at(3), Slot::at(7)),
        Exact::at(Slot::ZERO, Remainder::of(21, 10u128.pow(38)))
    );
    // A phase of `-1 / 2^63` at exponent zero: one times one is
    // `((2^63 - 1)^2 + 2^63) / 2^126`, which is `2^126 - 2^63 + 1` over `2^126`,
    // odd over a power of two and so in lowest terms.
    assert_eq!(
        product_position::<Phased<Signed<4>, 0, 1, { i64::MIN }>>(Slot::at(1), Slot::at(1)),
        Exact::at(
            Slot::ZERO,
            Remainder::of((1 << 126) - (1 << 63) + 1, 1 << 126)
        )
    );
}

#[test]
fn the_exact_step_is_the_denotation_at_the_deepest_formats_the_rational_oracle_forms() {
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
fn a_position_past_the_index_is_exact_and_past_the_limbs_saturates() {
    type F = Integer<8>;
    let top = Slot::at(i128::MAX);
    let bottom = Slot::at(i128::MIN);
    let whole = |a: i128, b: i128| {
        let Maybe::Is(at) = Wide::of(a).times(Wide::of(b)) else { unreachable!() };
        Exact::floored(at, Remainder::ZERO)
    };
    // Two slots outside the range, multiplied whole: the floor is held in four
    // limbs, and `slot()` answers the index's end on the product's side.
    let up = product_position::<F>(top, top);
    assert_eq!(up, whole(i128::MAX, i128::MAX));
    assert_eq!(up.slot(), top);
    assert_eq!(
        product_position::<F>(bottom, bottom),
        whole(i128::MIN, i128::MIN)
    );
    let down = product_position::<F>(top, bottom);
    assert_eq!(down, whole(i128::MAX, i128::MIN));
    assert_eq!(down.slot(), bottom);
    assert_eq!(
        product_position::<F>(Slot::at(-2), top),
        whole(-2, i128::MAX)
    );
    // Moved up past the index on a positive exponent, the same way.
    assert_eq!(
        product_position::<At<Signed<8>, 4>>(Slot::at(i128::MAX / 4), Slot::at(2)),
        whole(i128::MAX / 4, 32)
    );
    // Past the four limbs, `-2^200` moved up `2^124`, saturated at the limbs' end
    // on the product's sign.
    assert_eq!(
        product_position::<At<Signed<2>, 124>>(Slot::at(-(1 << 100)), Slot::at(1 << 100)),
        Exact::floored(Wide::end(true), Remainder::ZERO)
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
    // At a phase of a half, `(MIN + 1/2)(MAX + 1/2) - 1/2` is `MIN MAX - 3/4`:
    // the factors past the index are read exactly on their sign, and the floor
    // is one under the whole product with a quarter over it.
    let Maybe::Is(product) = Wide::of(i128::MIN).times(Wide::of(i128::MAX)) else {
        unreachable!()
    };
    let Maybe::Is(floor) = product.plus(Wide::of(-1)) else { unreachable!() };
    assert_eq!(
        product_position::<Biased<8, 0, 1>>(bottom, top),
        Exact::floored(floor, Remainder::reduced(Wide::of(1), Wide::of(4)))
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
fn the_obligation_holds_out_a_step_the_four_limbs_do_not_form() {
    // `2^255` and `10^77` are the largest powers the four limbs form.
    assert!(is_multipliable::<At<Signed<4>, -127>>().get());
    assert!(is_multipliable::<At<Signed<4>, -128>>().get());
    assert!(is_multipliable::<At<Signed<4>, -255>>().get());
    assert!(!is_multipliable::<At<Signed<4>, -256>>().get());
    assert!(!is_multipliable::<At<Signed<4>, { i32::MIN }>>().get());
    assert!(is_multipliable::<Dec<Signed<4>, -38>>().get());
    assert!(is_multipliable::<Dec<Signed<4>, -77>>().get());
    assert!(!is_multipliable::<Dec<Signed<4>, -78>>().get());
    // The phase's denominator enters squared and is at most `2^63`, so at a
    // non-negative exponent every phase is held, the least value's included,
    // and at `2^-64` too, where `2^32` squared once reached `2^128`.
    assert!(is_multipliable::<Phased<Signed<4>, 0, 1, { i64::MIN }>>().get());
    assert!(is_multipliable::<Phased<Signed<4>, 3, 1, { i64::MAX }>>().get());
    assert!(is_multipliable::<Phased<Signed<4>, -64, 1, 4_294_967_296>>().get());
    assert!(is_multipliable::<Phased<Signed<4>, -64, 1, { i64::MIN }>>().get());
    // At `2^-130` the least value's `2^63` squared reaches `2^256`, and
    // `2^63 - 1` squared stays under it.
    assert!(is_multipliable::<Phased<Signed<4>, -130, 1, { i64::MAX }>>().get());
    assert!(!is_multipliable::<Phased<Signed<4>, -130, 1, { i64::MIN }>>().get());
    // Reduced first: `2 / -2^63` is `-1 / 2^62`, and
    // `8589934590 / 8589934590` is a whole slot.
    assert!(is_multipliable::<Phased<Signed<4>, -130, 2, { i64::MIN }>>().get());
    assert!(is_multipliable::<Phased<Signed<4>, -255, 8_589_934_590, 8_589_934_590>>().get());
    // And it meets the quantum's: a half at `2^-253` is `2^255`, at `2^-254`
    // `2^256`.
    assert!(is_multipliable::<Phased<Signed<4>, -253, 1, 2>>().get());
    assert!(!is_multipliable::<Phased<Signed<4>, -254, 1, 2>>().get());
    // The phase's term can leave the limbs before `D` does: at `(2^63 - 1) / 2`
    // and `2^-193`, `D` is `2^195` and the term `(2^63 - 1) 2^194` is past
    // `2^256`, one place up it is under it.
    assert!(is_multipliable::<Phased<Signed<4>, -192, { i64::MAX }, 2>>().get());
    assert!(!is_multipliable::<Phased<Signed<4>, -193, { i64::MAX }, 2>>().get());
}

#[test]
fn the_obligation_reads_nothing_of_the_slot_index() {
    // The signed 64-bit range's largest product is 2^126, and the unsigned
    // one's top corner is just under 2^128 at exponent zero, past the index; both
    // are admitted, and so is every move of them the four limbs form.
    assert!(is_multipliable::<Integer<64>>().get());
    assert!(is_multipliable::<UFixed<64, 0>>().get());
    assert!(is_multipliable::<UFixed<64, -1>>().get());
    assert!(is_multipliable::<UFixed<64, 1>>().get());
    assert!(is_multipliable::<At<Signed<64>, 1>>().get());
    assert!(is_multipliable::<UFixed<63, 2>>().get());
    // The numerator is what bounds an up move: the corner `4 2^e` is formed in
    // the four limbs at `e = 253` and not at `254`, and `4 10^e` at `76` and not
    // at `77`.
    assert!(is_multipliable::<At<Signed<2>, 127>>().get());
    assert!(is_multipliable::<At<Signed<2>, 253>>().get());
    assert!(!is_multipliable::<At<Signed<2>, 254>>().get());
    assert!(is_multipliable::<Dec<Signed<2>, 38>>().get());
    assert!(is_multipliable::<Dec<Signed<2>, 76>>().get());
    assert!(!is_multipliable::<Dec<Signed<2>, 77>>().get());
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
