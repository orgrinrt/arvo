//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Which member is one, against the members' own values.
//!
//! The expected answer is read from `Ratio`: the member whose value is one, found
//! by walking the range, never from the step. Where the function answers, that
//! member's product with every member is asked of the exact step too, and has to
//! be the other member on the grid. Where it answers nothing, the walk has to
//! find no member at one.
//!
//! The control is a rule that reads the exponent alone and answers `r^-e`, which
//! is right at a zero phase and has to be wrong at a whole phase that is not
//! zero, so the suite would see an answer that left the phase out.

use notko::Maybe;

use super::{Ratio, members, value};
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::Exact;
use crate::format::Format;
use crate::multiplication::{multiplicative_identity, product_position};
use crate::points::{Fixed, Integer, UFixed};
use crate::quantum::Constant;
use crate::slots::{Signed, Slot, Unsigned};
use crate::tests::dispatch::PerFormat;
use crate::tests::grid::Grid;

/// What the walk saw.
#[derive(Default)]
struct Identity {
    formats:       u32,
    with_one:      u32,
    without_one:   u32,
    products:      u64,
    /// Formats with a member at one at which the exponent-only rule names
    /// another slot.
    control_wrong: u32,
}

/// The member whose value is one, by walking the range.
fn member_at_one<F: Format>() -> Maybe<Slot> {
    for slot in members::<F>() {
        if value::<F>(slot) == Ratio::whole(1) {
            return Maybe::Is(slot);
        }
    }
    Maybe::Isnt
}

/// The exponent-only rule's answer, the slot at `r^-e` where that is whole.
fn exponent_only<F: Format>() -> Maybe<Slot> {
    let q = super::quantum::<F>();
    let inverse = Ratio::whole(1).over(q);
    if inverse.is_whole() {
        Maybe::Is(Slot::at(inverse.floor()))
    } else {
        Maybe::Isnt
    }
}

impl PerFormat for Identity {
    fn run<F: Format>(&mut self) {
        self.formats += 1;
        let want = member_at_one::<F>();
        let got = multiplicative_identity::<F>();
        assert_eq!(got, want, "the member at one");
        if want != Maybe::Isnt && exponent_only::<F>() != want {
            self.control_wrong += 1;
        }
        match got {
            Maybe::Is(one) => {
                self.with_one += 1;
                for b in members::<F>() {
                    assert_eq!(
                        product_position::<F>(one, b),
                        Exact::on_grid(b),
                        "one * {b:?}"
                    );
                    assert_eq!(
                        product_position::<F>(b, one),
                        Exact::on_grid(b),
                        "{b:?} * one"
                    );
                    self.products += 2;
                }
            },
            Maybe::Isnt => self.without_one += 1,
        }
    }
}

/// Every width from one to six of both families at `A`, each exponent from minus
/// seven to two, at the phase `N / D`.
macro_rules! walk {
    ($what:expr, $ambient:ty, $n:literal, $d:literal) => {
        walk!(@widths $what, $ambient, $n, $d, [1, 2, 3, 4, 5, 6]);
    };
    (@widths $what:expr, $ambient:ty, $n:literal, $d:literal, [$($w:literal),+]) => {
        $( walk!(@exponents $what, $ambient, $n, $d, $w, [-7, -6, -5, -4, -3, -2, -1, 0, 1, 2]); )+
    };
    (@exponents $what:expr, $ambient:ty, $n:literal, $d:literal, $w:literal, [$($e:literal),+]) => {
        $(
            $what.run::<Grid<$ambient, Constant<$e>, Signed<$w>, $n, $d>>();
            $what.run::<Grid<$ambient, Constant<$e>, Unsigned<$w>, $n, $d>>();
        )+
    };
}

#[test]
fn the_identity_is_the_member_at_one_and_nothing_where_none_is() {
    let mut seen = Identity::default();
    walk!(seen, BinaryRationals, 0, 1);
    walk!(seen, DecimalRationals, 0, 1);
    // 6 widths, 10 exponents, 2 families, 2 radices.
    assert_eq!(seen.formats, 240);
    // Both answers occur, or one side of the law was never asked.
    assert!(
        seen.with_one > 0 && seen.without_one > 0,
        "{} with, {} without",
        seen.with_one,
        seen.without_one
    );
    assert!(seen.products > 0);
    // At a zero phase the exponent-only rule is right wherever a member is one.
    assert_eq!(seen.control_wrong, 0);

    let mut phased = Identity::default();
    walk!(phased, BinaryRationals, 1, 1);
    walk!(phased, BinaryRationals, -2, 1);
    assert_eq!(phased.formats, 240);
    assert!(phased.with_one > 0 && phased.without_one > 0);
    // The control leaves the phase out, so at a whole phase that is not zero it
    // is wrong at every format that has a member at one.
    assert_eq!(phased.control_wrong, phased.with_one);
}

#[test]
fn a_half_phase_has_a_member_at_one_exactly_at_exponent_one() {
    let mut seen = Identity::default();
    walk!(seen, BinaryRationals, 1, 2);
    assert_eq!(seen.formats, 120);
    // `(1/2 + s) 2^e` is one only at `e` one and slot zero, which every range
    // here holds: six widths of two families.
    assert_eq!(seen.with_one, 12);
    assert_eq!(seen.control_wrong, 12);
    type HalfAtOne = Grid<BinaryRationals, Constant<1>, Signed<1>, 1, 2>;
    assert_eq!(
        multiplicative_identity::<HalfAtOne>(),
        Maybe::Is(Slot::at(0))
    );
}

/// Phases whose denominator is three or more, and the decimal radix away from
/// a zero phase. Nothing is expected of these but the walk's own answer: the
/// function has to name the member at one wherever the range holds one, and
/// nothing elsewhere, whatever the phase is.
#[test]
fn the_identity_is_the_member_at_one_at_phases_of_any_denominator() {
    let mut thirds = Identity::default();
    walk!(thirds, BinaryRationals, 1, 3);
    walk!(thirds, BinaryRationals, -2, 3);
    walk!(thirds, BinaryRationals, 3, 4);
    walk!(thirds, BinaryRationals, 5, 7);
    assert_eq!(thirds.formats, 480);

    let mut decimal = Identity::default();
    walk!(decimal, DecimalRationals, 1, 1);
    walk!(decimal, DecimalRationals, -3, 1);
    walk!(decimal, DecimalRationals, 1, 2);
    walk!(decimal, DecimalRationals, 1, 5);
    walk!(decimal, DecimalRationals, 2, 3);
    assert_eq!(decimal.formats, 600);
    // A whole decimal phase leaves members at one, so the answering side of the
    // law is asked here too.
    assert!(decimal.with_one > 0 && decimal.without_one > 0);
    assert!(decimal.products > 0);
}

#[test]
fn the_shipped_points_answer() {
    assert_eq!(
        multiplicative_identity::<Integer<8>>(),
        Maybe::Is(Slot::at(1))
    );
    assert_eq!(multiplicative_identity::<Integer<1>>(), Maybe::Isnt);
    assert_eq!(
        multiplicative_identity::<UFixed<6, -3>>(),
        Maybe::Is(Slot::at(8))
    );
    assert_eq!(multiplicative_identity::<UFixed<3, -3>>(), Maybe::Isnt);
    assert_eq!(multiplicative_identity::<UFixed<6, 3>>(), Maybe::Isnt);
    assert_eq!(
        multiplicative_identity::<Fixed<6, -3>>(),
        Maybe::Is(Slot::at(8))
    );
    // The product over this range is not admitted yet, and one is still a member.
    assert_eq!(
        multiplicative_identity::<UFixed<64, 0>>(),
        Maybe::Is(Slot::at(1))
    );
}
