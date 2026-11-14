// The same range at exponent one, refused: slot b names 2b there and b in the
// bytes, a quantum the `V` halves alone cannot tell apart.
//
// A divisor taken from a member of a second format `G` beside `UFixed<8, 0>`,
// the bytes. The pair that turns on one token of `G`: every refused file
// differs from `a_member_divisor_from_a_zero_free_range_on_the_grid_is_admitted`
// in one place, and that file names `Member` from the crate root.

use arvo_format::format::{Format, Phase};
use arvo_format::points::UFixed;
use arvo_format::quantum::Constant;
use arvo_format::slots::{Slot, Slots};
use arvo_format::width::Width;
use arvo_format::{Divisor, Member};

struct Range;

impl Slots for Range {
    const MAX: Slot = Slot::at(255);
    const MIN: Slot = Slot::at(1);
    const WIDTH: Width = Width::bits(16);
}

struct G;

impl Format for G {
    type Ambient = arvo_format::ambient::BinaryRationals;
    type Quantum = Constant<1>;
    type Slots = Range;

    const PHASE: Phase = Phase::of(0, 1);
}

const DIVISOR: Divisor<UFixed<8, 0>> = Divisor::member(Member::<G>::constant::<7>());

fn main() {
    let _ = DIVISOR.slot();
}
