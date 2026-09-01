// The two ratios the crate carries are two coordinates.
//
// A phase offsets the whole grid and a fraction is a position between two of its
// points, and they hold the same pair of numbers, which is exactly when a design
// starts passing one where it meant the other.

use arvo_format::apply::{Exact, Fraction};
use arvo_format::format::Phase;
use arvo_format::slots::Slot;

fn main() {
    let _ = Exact::between(Slot::ZERO, Phase::halves(1));
    let _: Phase = Fraction::HALF;
    // And the two consts the phase replaced are gone rather than deprecated.
    let _ = Phase::of(1, 2).numerator() + Phase::PHASE_DEN;
}
