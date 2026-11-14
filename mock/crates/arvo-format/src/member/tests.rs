//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The constant door at the ends of each range, and the perimeter check that
//! reports a planted door without the range's refusal.
//!
//! The adapted door is walked with the map in `apply/tests/totality.rs`, at every
//! admitted width of both families. The refusal itself is a build failure and
//! lives in `tests/ui` as a pair.

use super::Member;
use crate::ambient::BinaryRationals;
use crate::format::Format;
use crate::points::{Integer, UFixed};
use crate::quantum::Constant;
use crate::slots::{Slot, Slots};
use crate::tests::grid::Grid;
use crate::width::Width;

/// The slots one to 255, a consumer's range that leaves the zero out.
struct OneTo255;

impl Slots for OneTo255 {
    const MAX: Slot = Slot::at(255);
    const MIN: Slot = Slot::at(1);
    const WIDTH: Width = Width::bits(8);
}

type Positive = Grid<BinaryRationals, Constant<0>, OneTo255, 0, 1>;

/// How many of the members lie outside their format's range.
fn outside<F: Format>(members: &[Member<F>]) -> usize {
    let (min, max) = (<F::Slots as Slots>::MIN, <F::Slots as Slots>::MAX);
    members
        .iter()
        .filter(|m| !m.slot().is_within(min, max).get())
        .count()
}

#[test]
fn the_constant_door_holds_each_end_of_the_range() {
    const SIGNED: [Member<Integer<8>>; 3] =
        [Member::constant::<-128>(), Member::constant::<0>(), Member::constant::<127>()];
    const UNSIGNED: [Member<UFixed<8, -3>>; 2] =
        [Member::constant::<0>(), Member::constant::<255>()];
    const POSITIVE: [Member<Positive>; 2] = [Member::constant::<1>(), Member::constant::<255>()];
    assert_eq!(SIGNED.map(|m| m.slot().index()), [-128, 0, 127]);
    assert_eq!(UNSIGNED.map(|m| m.slot().index()), [0, 255]);
    assert_eq!(POSITIVE.map(|m| m.slot().index()), [1, 255]);
    assert_eq!(outside(&SIGNED), 0);
    assert_eq!(outside(&UNSIGNED), 0);
    assert_eq!(outside(&POSITIVE), 0);
}

#[test]
fn the_perimeter_check_reports_a_constant_door_with_no_range_check() {
    // The planted door is the constant door with its obligation left out. Fed one
    // past each end, it yields members the check has to report; the shipped door
    // refuses the same constants at build time.
    let planted = |b: i128| Member::<Positive>::assumed(Slot::at(b));
    assert_eq!(outside(&[planted(0), planted(256)]), 2);
    // The control: the same planted door inside the range is not reported.
    assert_eq!(outside(&[planted(1), planted(255)]), 0);
}
