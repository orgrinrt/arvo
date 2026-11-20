// A field whose value is zero bits wide, in a word whose other field fills the
// container, is refused at build with the width's own message.

#![feature(const_trait_impl)]

use arvo_bits::{Bits, Field, Lane, Layout, Word};
use arvo_format::Width;

#[derive(Clone, Copy)]
struct Nothing;

impl Lane for Nothing {
    const WIDTH: Width = Width::bits(0);

    fn from_low(_: Bits<64>) -> Self {
        Nothing
    }

    fn low(self) -> Bits<64> {
        Bits::masked(0)
    }
}

struct Handle;
struct Id;
struct Empty;

impl Layout for Handle {
    type Container = Bits<32>;
    type Fields = (Empty, (Id, ()));
}

impl Field for Id {
    type Layout = Handle;
    type Value = Bits<32>;
    const BELOW: Width = Width::bits(0);
}

impl Field for Empty {
    type Layout = Handle;
    type Value = Nothing;
    const BELOW: Width = Width::bits(32);
}

fn main() {
    let _ = Word::<Handle>::of(Bits::masked(0)).bits();
}
