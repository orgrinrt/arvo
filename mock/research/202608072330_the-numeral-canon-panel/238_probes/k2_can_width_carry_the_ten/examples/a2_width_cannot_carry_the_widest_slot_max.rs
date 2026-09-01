// `Slots::MAX` retyped to `Width`, at the widest width the crate admits.
//
// Sign is not the only obstacle. `Unsigned<62>` sets `MAX = (1i64 << 62) - 1`,
// which is 4611686018427387903 and does not fit the unsigned 32-bit integer
// `Width` is transparent over. So even the non-negative half of the slot range
// is outside what the type can hold, at widths the crate ships impls for.

use arvo_format::width::Width;

pub trait SlotsByWidth {
    const MAX: Width;
}

pub struct MyRange;

impl SlotsByWidth for MyRange {
    const MAX: Width = Width::bits(4611686018427387903);
}

fn main() {}
