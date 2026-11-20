// A field naming a layout that does not list it, written in a call at run
// time, is refused at build, though the layout itself tiles its word.

use arvo_bits::{Bits, Field, Layout, Word};
use arvo_format::Width;

struct Handle;
struct Id;
struct Kind;
struct Stray;

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
    const BELOW: Width = Width::bits(28);
}

impl Field for Stray {
    type Layout = Handle;
    type Value = Bits<8>;
    const BELOW: Width = Width::bits(24);
}

fn main() {
    let _ = Word::<Handle>::of(Bits::masked(0)).with::<Stray>(Bits::masked(1));
}
