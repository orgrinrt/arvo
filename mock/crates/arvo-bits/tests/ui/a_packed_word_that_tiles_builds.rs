// The same shapes as the refused cases, with fields that tile their word, build:
// in a constant and in a call at run time, every field read and written.

use arvo_bits::{Bits, Field, Layout, Word};
use arvo_format::Bool;
use notko_hlist::{Cons, Empty};

#[derive(Clone, Copy)]
struct Id;
#[derive(Clone, Copy)]
struct Kind;
#[derive(Clone, Copy)]
struct Flag;

impl Field for Id {
    type Value = Bits<28>;
}

impl Field for Kind {
    type Value = Bits<3>;
}

impl Field for Flag {
    type Value = Bool;
}

struct Handle;

impl Layout for Handle {
    type Container = Bits<32>;
    type Fields = Cons<Id, Cons<Kind, Cons<Flag, Empty>>>;
}

const HANDLE: Word<Handle> = Word::<Handle>::of(Bits::masked(0))
    .with(Id, Bits::masked(1))
    .with(Kind, Bits::masked(2))
    .with(Flag, Bool::TRUE);
const KIND: Bits<3> = HANDLE.get(Kind);

fn main() {
    let word = Word::<Handle>::of(Bits::masked(0)).with(Flag, Bool::TRUE);
    let _ = (KIND, word.get(Id), word.get(Kind), word.get(Flag), word.bits());
}
