//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The panic verdicts, which report rather than diverge.
//!
//! Section nine of the parent's numbering, in a file of its own because the
//! parent had passed 500 lines. It reads the parent's window and sweep.

use super::{MAX5, MIN5, every_position, round_slot};
use crate::adapt::{Adapt, Signature};
use crate::apply::{Dither, Exact, Fraction, adapt, panic_on_inexact, panic_on_overflow};
use crate::overflow::Wrap;
use crate::points::Integer;
use crate::rounding::{Ceil, Floor, Mode, Stochastic};
use crate::slots::Slot;

#[test]
fn the_panic_verdicts_report_and_the_crate_stays_total() {
    assert!(!panic_on_inexact(Exact::on_grid(Slot::at(3))).get());
    assert!(panic_on_inexact(Exact::between(Slot::at(3), Fraction::of(1, 4))).get());

    type Down = Signature<Integer<5>, Adapt<Floor, Wrap>>;
    type Up = Signature<Integer<5>, Adapt<Ceil, Wrap>>;
    type Dice = Signature<Integer<5>, Adapt<Stochastic, Wrap>>;
    let d = Dither::UNUSED;

    // On the grid the rounded slot is the position's own, so the verdict is the
    // range test on that slot.
    assert!(!panic_on_overflow::<Down>(Exact::on_grid(MAX5), d).get());
    assert!(!panic_on_overflow::<Down>(Exact::on_grid(MIN5), d).get());
    assert!(panic_on_overflow::<Down>(Exact::on_grid(Slot::at(MAX5.index() + 1)), d).get());
    assert!(panic_on_overflow::<Down>(Exact::on_grid(Slot::at(MIN5.index() - 1)), d).get());

    // Off the grid the verdict is about the slot the position rounds to, which is
    // why it takes the position. A tie on the top slot rounds out of the range
    // under `Ceil` and stays under `Floor`; one below the bottom does the reverse.
    // A verdict over the position's own slot would answer the same for both modes
    // at both ends, and one of the two would be wrong each time.
    let top_tie = Exact::between(MAX5, Fraction::HALF);
    assert!(panic_on_overflow::<Up>(top_tie, d).get());
    assert!(!panic_on_overflow::<Down>(top_tie, d).get());
    let under = Exact::between(Slot::at(MIN5.index() - 1), Fraction::HALF);
    assert!(panic_on_overflow::<Down>(under, d).get());
    assert!(!panic_on_overflow::<Up>(under, d).get());

    // And the dither is read where the mode reads it: a quarter past the top slot
    // rounds up exactly when the dither falls below a quarter.
    let quarter = Exact::between(MAX5, Fraction::of(1, 4));
    assert!(panic_on_overflow::<Dice>(quarter, Dither::at(Fraction::of(1, 8))).get());
    assert!(!panic_on_overflow::<Dice>(quarter, Dither::at(Fraction::of(7, 8))).get());

    // The verdict agrees with what the map does: wherever it reports nothing, the
    // completion had nothing to do, so the map returned the rounded slot as it
    // was. Over the whole sweep, both modes, which leaves the window both ways.
    let mut reported = 0;
    for e in every_position() {
        for (verdict, got, rounded) in [
            (
                panic_on_overflow::<Down>(e, d),
                adapt::<Down>(e, d),
                round_slot(Mode::Floor, e, d),
            ),
            (
                panic_on_overflow::<Up>(e, d),
                adapt::<Up>(e, d),
                round_slot(Mode::Ceil, e, d),
            ),
        ] {
            if verdict.get() {
                reported += 1;
                assert!(
                    rounded < MIN5.index() || rounded > MAX5.index(),
                    "the verdict reported {e:?}, whose rounded slot is in the window"
                );
            } else {
                assert_eq!(
                    got.index(),
                    rounded,
                    "the verdict passed {e:?} and the map still completed it"
                );
            }
        }
    }
    assert!(reported > 0, "the sweep never left the window");

    // Totality is unaffected by the verdicts: the map still returns a slot for a
    // position both verdicts refuse.
    let refused = Exact::between(Slot::at(MAX5.index() + 5), Fraction::of(1, 4));
    assert!(panic_on_inexact(refused).get());
    assert!(panic_on_overflow::<Down>(refused, d).get());
    let got = adapt::<Down>(refused, d);
    assert!(got.is_within(MIN5, MAX5).get());
}
