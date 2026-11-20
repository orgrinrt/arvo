// A word held in a consumer's own `Lane` claiming zero bits, with no fields, is
// refused at build with the container's own message.

#![feature(const_trait_impl)]

use arvo_bits::{Bits, Lane, Layout, Word};
use arvo_format::Width;
use notko_hlist::Empty;

#[derive(Clone, Copy)]
struct Nothing;

const impl Lane for Nothing {
    const WIDTH: Width = Width::bits(0);

    fn from_low(_: Bits<64>) -> Self {
        Nothing
    }

    fn low(self) -> Bits<64> {
        Bits::masked(0)
    }
}

struct Hollow;

impl Layout for Hollow {
    type Container = Nothing;
    type Fields = Empty;
}

fn main() {
    let _ = Word::<Hollow>::of(Nothing).bits();
}
