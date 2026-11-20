// A 64-bit word whose top field, eight bits wide, starts at bit sixty, past
// where the carrier ends, is refused at build with the reach's own message
// rather than an overflowing shift.

#![feature(const_trait_impl)]

use arvo_bits::{Bits, Field, Layout, Word};
use arvo_format::Width;

struct Handle;
struct Low;
struct Top;

impl Layout for Handle {
    type Container = Bits<64>;
    type Fields = (Top, (Low, ()));
}

impl Field for Low {
    type Layout = Handle;
    type Value = Bits<56>;
    const BELOW: Width = Width::bits(0);
}

impl Field for Top {
    type Layout = Handle;
    type Value = Bits<8>;
    const BELOW: Width = Width::bits(60);
}

fn main() {
    let _ = Word::<Handle>::of(Bits::masked(0)).bits();
}
