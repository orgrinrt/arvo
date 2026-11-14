//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Quotients whose position lies far past the slot index, admitted and adapted
//! from their own floor.
//!
//! At a fine quantum a narrow range divides into positions far past `2^127`. The
//! map reads the position's floor where it is, so the wrap, the saturation, and
//! the sign and parity the rounding modes read are those of the true floor.

use crate::adapt::{Adapt, Signature};
use crate::ambient::BinaryRationals;
use crate::apply::Dither;
use crate::division::{Divisor, div, is_divisible, quotient_position};
use crate::overflow::{Saturate, Wrap};
use crate::quantum::Constant;
use crate::rounding::{Floor, HalfEven, HalfUp, Rounding, TowardZero};
use crate::slots::{Signed, Slot};
use crate::tests::grid::Grid;

/// The signed 64-bit range at `2^-64`, where `MIN / -1` is the position `2^127`.
type Fine = Grid<BinaryRationals, Constant<-64>, Signed<64>, 0, 1>;

/// The signed 64-bit range at phase one half and `2^-66`. No member is the zero,
/// and the quotient by the member at slot 0, `1/2`, is `(1 + 2a) 2^66 - 1/2`: a
/// tie past the index whose floor is odd.
type HalfFine = Grid<BinaryRationals, Constant<-66>, Signed<64>, 1, 2>;

#[test]
fn a_narrow_range_at_a_fine_quantum_is_divisible() {
    assert!(is_divisible::<Fine>().get());
    assert!(is_divisible::<HalfFine>().get());
}

#[test]
fn min_over_minus_one_at_two_to_the_minus_64_wraps_to_zero() {
    let min = Slot::at(i128::from(i64::MIN));
    let minus_one = Divisor::<Fine>::constant::<-1>();
    // `2^127` is one past the index's top, and `2^127` modulo `2^64` is zero.
    let at = quotient_position::<Fine>(min, minus_one);
    assert!(at.is_on_grid().get());
    assert_eq!(at.slot(), Slot::at(i128::MAX));
    assert_eq!(
        div::<Signature<Fine, Adapt<Floor, Wrap>>>(min, minus_one, Dither::UNUSED),
        Slot::ZERO
    );
    assert_eq!(
        div::<Signature<Fine, Adapt<Floor, Saturate>>>(min, minus_one, Dither::UNUSED),
        Slot::at(i128::from(i64::MAX))
    );
}

/// The quotient of `a` by the member at slot 0 of `HalfFine`, under `R` and
/// wrapping.
fn half_fine<R: Rounding>(a: i64) -> Slot {
    div::<Signature<HalfFine, Adapt<R, Wrap>>>(
        Slot::at(i128::from(a)),
        Divisor::<HalfFine>::of(Slot::ZERO),
        Dither::UNUSED,
    )
}

#[test]
fn a_tie_past_the_index_reads_its_own_floor_s_parity_and_sign() {
    // `(1 + 2a) 2^66` is zero modulo `2^64`, so the floor wraps to `-1` and the
    // step above it to 0. At `a = 1` and `a = -1` the position is inside the
    // index, the control that the far ones are read by the same rule; past
    // `|a| = 2^60` it is past the index, and `slot()` answers the end.
    for a in [1, (1 << 62) + 3, i64::MAX, -1, -(1 << 62) - 5, i64::MIN] {
        let at = quotient_position::<HalfFine>(Slot::at(i128::from(a)), Divisor::of(Slot::ZERO));
        assert!(at.is_tie().get(), "{a}");
        let slot = match a {
            1 | -1 => ((1 + 2 * i128::from(a)) << 66) - 1,
            _ if a < 0 => i128::MIN,
            _ => i128::MAX,
        };
        assert_eq!(at.slot(), Slot::at(slot), "{a}");
        assert_eq!(half_fine::<Floor>(a), Slot::at(-1), "{a}");
        assert_eq!(half_fine::<HalfUp>(a), Slot::ZERO, "{a}");
        // The floor is odd, so half to even steps up.
        assert_eq!(half_fine::<HalfEven>(a), Slot::ZERO, "{a}");
        // Toward zero steps up below zero only.
        let toward = if a < 0 { Slot::ZERO } else { Slot::at(-1) };
        assert_eq!(half_fine::<TowardZero>(a), toward, "{a}");
    }
}

#[test]
fn a_far_quotient_saturates_on_its_own_side() {
    let top = Slot::at(i128::from(i64::MAX));
    let bottom = Slot::at(i128::from(i64::MIN));
    let by = Divisor::<HalfFine>::of(Slot::ZERO);
    type Sat = Signature<HalfFine, Adapt<HalfEven, Saturate>>;
    assert_eq!(div::<Sat>(Slot::at(5), by, Dither::UNUSED), top);
    assert_eq!(div::<Sat>(Slot::at(-5), by, Dither::UNUSED), bottom);
}
