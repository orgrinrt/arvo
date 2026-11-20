// A word held in a consumer's own `Lane` claiming sixty-five bits, its fields
// summing to sixty-five, is refused at build with the container's own message.

#![feature(const_trait_impl)]

use arvo_bits::{Bits, Field, Lane, Layout, Word};
use arvo_format::{Bool, Width};
use notko_hlist::{Cons, Empty};

#[derive(Clone, Copy)]
struct Wider;

const impl Lane for Wider {
    const WIDTH: Width = Width::bits(65);

    fn from_low(_: Bits<64>) -> Self {
        Wider
    }

    fn low(self) -> Bits<64> {
        Bits::masked(0)
    }
}

#[derive(Clone, Copy)]
struct Low;
#[derive(Clone, Copy)]
struct Top;

impl Field for Low {
    type Value = Bits<64>;
}

impl Field for Top {
    type Value = Bool;
}

struct Over;

impl Layout for Over {
    type Container = Wider;
    type Fields = Cons<Low, Cons<Top, Empty>>;
}

fn main() {
    let _ = Word::<Over>::of(Wider).bits();
}
