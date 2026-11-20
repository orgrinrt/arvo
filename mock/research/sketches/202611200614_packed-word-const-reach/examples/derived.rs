//! A consumer with no feature line, through the second seat's shape: no offset
//! written anywhere, every field written and read in a constant, and one field
//! type, `Flag`, sitting in two layouts at two different places.

use arvo_bits::Bits;
use arvo_format::Bool;
use notko_hlist::{Cons, Empty};
use packed_word_const_reach::{Field, Layout, Word};

#[derive(Clone, Copy)]
struct Id;
#[derive(Clone, Copy)]
struct Kind;
#[derive(Clone, Copy)]
struct Flag;
#[derive(Clone, Copy)]
struct Tag;

impl Field for Id {
    type Value = Bits<28>;
}
impl Field for Kind {
    type Value = Bits<3>;
}
impl Field for Flag {
    type Value = Bool;
}
impl Field for Tag {
    type Value = Bits<7>;
}

struct Handle;
impl Layout for Handle {
    type Container = Bits<32>;
    type Fields = Cons<Id, Cons<Kind, Cons<Flag, Empty>>>;
}

struct Small;
impl Layout for Small {
    type Container = Bits<8>;
    type Fields = Cons<Flag, Cons<Tag, Empty>>;
}

const H: Word<Handle> = Word::<Handle>::of(Bits::masked(0))
    .with(Id, Bits::masked(0x0abc_def1))
    .with(Kind, Bits::masked(5))
    .with(Flag, Bool::TRUE);

const KIND: Bits<3> = H.get(Kind);
const RAW: Bits<32> = H.bits();
const S: Word<Small> = Word::<Small>::of(Bits::masked(0)).with(Flag, Bool::TRUE);

fn main() {
    assert_eq!(KIND.raw(), 5);
    assert_eq!(H.get(Id).raw(), 0x0abc_def1);
    assert!(H.get(Flag).get());
    assert_eq!(RAW.raw(), 0x8000_0000 | (5 << 28) | 0x0abc_def1);
    assert_eq!(S.bits().raw(), 1);
    assert!(S.get(Flag).get());
    assert_eq!(S.get(Tag).raw(), 0);
}
