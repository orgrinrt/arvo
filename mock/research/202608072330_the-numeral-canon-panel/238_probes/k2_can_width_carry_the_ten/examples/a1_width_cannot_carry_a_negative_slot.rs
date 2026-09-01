// `Slots::MIN` retyped to `Width`, at the sign the shipped impls actually use.
//
// `Signed<8>` sets `MIN = -(1i64 << 7)`, so a signed slot range's lowest index
// is negative on every width the crate admits. `Width` is a count of bits,
// `repr(transparent)` over an unsigned integer, and its only constructor takes
// one, so the value has nowhere to go.
//
// This is one of the six the first option cannot reach, and the cheapest to
// read.

use arvo_format::width::Width;

pub trait SlotsByWidth {
    const MIN: Width;
}

pub struct MyRange;

impl SlotsByWidth for MyRange {
    const MIN: Width = Width::bits(-128);
}

fn main() {}
