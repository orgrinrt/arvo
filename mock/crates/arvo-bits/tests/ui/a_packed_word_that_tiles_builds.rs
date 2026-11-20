// The same shapes as the refused cases, with fields that tile their word, build:
// in a constant and in a call at run time, every field read and written.

#![feature(const_trait_impl)]

use arvo_bits::{Bits, Field, Layout, Word};
use arvo_format::{Bool, Width};

struct Handle;
struct Id;
struct Kind;
struct Flag;

impl Layout for Handle {
    type Container = Bits<32>;
    type Fields = (Kind, (Flag, (Id, ())));
}

impl Field for Id {
    type Layout = Handle;
    type Value = Bits<28>;
    const BELOW: Width = Width::bits(0);
}

impl Field for Kind {
    type Layout = Handle;
    type Value = Bits<3>;
    const BELOW: Width = Width::bits(28);
}

impl Field for Flag {
    type Layout = Handle;
    type Value = Bool;
    const BELOW: Width = Width::bits(31);
}

const KIND: Bits<3> = Word::<Handle>::of(Bits::masked(0))
    .with::<Kind>(Bits::masked(5))
    .get::<Kind>();

fn main() {
    let word = Word::<Handle>::of(Bits::masked(0))
        .with::<Id>(Bits::masked(1))
        .with::<Flag>(Bool::TRUE);
    let _ = (KIND, word.get::<Id>(), word.get::<Kind>(), word.get::<Flag>(), word.bits());
}
