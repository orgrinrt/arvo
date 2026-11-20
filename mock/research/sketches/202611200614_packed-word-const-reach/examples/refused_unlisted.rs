//! Must not build: a field read through a word whose layout does not list it.

use arvo_bits::Bits;
use arvo_format::Bool;
use notko_hlist::{Cons, Empty};
use packed_word_const_reach::{Field, Layout, Word};

#[derive(Clone, Copy)]
struct Flag;
#[derive(Clone, Copy)]
struct Other;

impl Field for Flag {
    type Value = Bool;
}
impl Field for Other {
    type Value = Bits<7>;
}

struct Small;
impl Layout for Small {
    type Container = Bits<8>;
    type Fields = Cons<Flag, Cons<Rest, Empty>>;
}

#[derive(Clone, Copy)]
struct Rest;
impl Field for Rest {
    type Value = Bits<7>;
}

fn main() {
    let w = Word::<Small>::of(Bits::masked(0));
    let _ = w.get(Other);
}
