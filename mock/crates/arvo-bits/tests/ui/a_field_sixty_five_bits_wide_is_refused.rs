// A field whose value, a consumer's own `Lane`, claims sixty-five bits, in a
// 64-bit word, is refused at build with the width's own message before any
// shift is worked out.

#![feature(const_trait_impl)]

use arvo_bits::{Bits, Field, Lane, Layout, Word};
use arvo_format::Width;
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
struct Big;

impl Field for Big {
    type Value = Wider;
}

struct Word64;

impl Layout for Word64 {
    type Container = Bits<64>;
    type Fields = Cons<Big, Empty>;
}

fn main() {
    let _ = Word::<Word64>::of(Bits::masked(0)).bits();
}
