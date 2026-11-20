// A 32-bit word whose fields sum to thirty-one bits, read in a call at run
// time, is refused at build.

use arvo_bits::{Bits, Field, Layout, Word};
use notko_hlist::{Cons, Empty};

#[derive(Clone, Copy)]
struct Id;
#[derive(Clone, Copy)]
struct Kind;

impl Field for Id {
    type Value = Bits<28>;
}

impl Field for Kind {
    type Value = Bits<3>;
}

struct Short;

impl Layout for Short {
    type Container = Bits<32>;
    type Fields = Cons<Id, Cons<Kind, Empty>>;
}

fn main() {
    let word = Word::<Short>::of(Bits::masked(0));
    let _ = word.get(Id);
}
