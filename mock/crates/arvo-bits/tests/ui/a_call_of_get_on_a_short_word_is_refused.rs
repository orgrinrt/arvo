// A 32-bit word whose fields sum to thirty-one bits, read in a call at run
// time, is refused at build.

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
    type Value = Bits<3>;
    const BELOW: Width = Width::bits(28);
}

fn main() {
    let _ = Word::<Handle>::of(Bits::masked(0)).get::<Kind>();
}
