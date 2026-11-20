// A 32-bit word of a 28-bit and a 4-bit field, which sum to thirty-two but
// share bit twenty-seven, built in a constant, is refused at build.

#![feature(const_trait_impl)]

use arvo_bits::{Bits, Field, Layout, Word};
use arvo_format::Width;

struct Handle;
struct Id;
struct Kind;

impl Layout for Handle {
    type Container = Bits<32>;
    type Fields = (Kind, (Id, ()));
}

impl Field for Id {
    type Layout = Handle;
    type Value = Bits<28>;
    const BELOW: Width = Width::bits(0);
}

impl Field for Kind {
    type Layout = Handle;
    type Value = Bits<4>;
    const BELOW: Width = Width::bits(27);
}

const HANDLE: Word<Handle> = Word::<Handle>::of(Bits::masked(0));

fn main() {
    let _ = HANDLE.bits();
}
