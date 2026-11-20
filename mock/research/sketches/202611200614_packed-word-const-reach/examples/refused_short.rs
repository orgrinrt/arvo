//! Must not build: field widths summing to less than the container's.

use arvo_bits::Bits;
use notko_hlist::{Cons, Empty};
use packed_word_const_reach::{Field, Layout, Word};

#[derive(Clone, Copy)]
struct Low;
impl Field for Low {
    type Value = Bits<7>;
}

struct Short;
impl Layout for Short {
    type Container = Bits<8>;
    type Fields = Cons<Low, Empty>;
}

fn main() {
    let w = Word::<Short>::of(Bits::masked(0));
    let _ = w.get(Low);
}
