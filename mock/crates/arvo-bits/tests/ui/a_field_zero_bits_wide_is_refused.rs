// A field whose value, a consumer's own `Lane`, claims zero bits, in a word
// whose other field fills the container, is refused at build with the width's
// own message.

#![feature(const_trait_impl)]

use arvo_bits::{Bits, Field, Lane, Layout, Word};
use arvo_format::Width;
use notko_hlist::{Cons, Empty};

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

#[derive(Clone, Copy)]
struct Id;
#[derive(Clone, Copy)]
struct Hollow;

impl Field for Id {
    type Value = Bits<32>;
}

impl Field for Hollow {
    type Value = Nothing;
}

struct Handle;

impl Layout for Handle {
    type Container = Bits<32>;
    type Fields = Cons<Hollow, Cons<Id, Empty>>;
}

fn main() {
    let _ = Word::<Handle>::of(Bits::masked(0)).get(Id);
}
