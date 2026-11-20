//! A consumer with no feature line: the handle's fields written and read in
//! constants, through the shape at the head of the branch.

use arvo_bits::{Bits, Field, Layout, Word};
use arvo_format::{Bool, Width};

struct Handle;
struct Id;
struct Kind;
struct Flag;

impl Layout for Handle {
    type Container = Bits<32>;
    type Fields = (Id, (Kind, (Flag, ())));
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

const H: Word<Handle> = Word::<Handle>::of(Bits::masked(0))
    .with::<Kind>(Bits::masked(5))
    .with::<Flag>(Bool::TRUE);

const KIND: Bits<3> = H.get::<Kind>();

fn main() {
    assert_eq!(KIND.raw(), 5);
    assert!(H.get::<Flag>().get());
}
