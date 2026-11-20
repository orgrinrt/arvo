//! hilavitkutin's identity handle: twenty-eight bits of id at the bottom,
//! three of kind above, one flag on top, every field read and written in a
//! constant.

#![feature(const_trait_impl)]

use arvo_bits::Bits;
use arvo_format::{Bool, Width};
use packed_word::{Field, Layout, Word};

struct Handle;
struct Id;
struct Kind;
struct Flag;

impl Layout for Handle {
    type Container = Bits<32>;
    type Fields = (Flag, (Kind, (Id, ())));
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
    .with::<Id>(Bits::masked(0x0abc_def1))
    .with::<Kind>(Bits::masked(5))
    .with::<Flag>(Bool::TRUE);

const ID: Bits<28> = H.get::<Id>();
const KIND: Bits<3> = H.get::<Kind>();
const FLAG: Bool = H.get::<Flag>();
const RAW: Bits<32> = H.bits();

fn main() {
    assert_eq!(ID.raw(), 0x0abc_def1);
    assert_eq!(KIND.raw(), 5);
    assert!(FLAG.get());
    assert_eq!(RAW.raw(), 0x8000_0000 | (5 << 28) | 0x0abc_def1);
    let back = Word::<Handle>::of(RAW);
    assert_eq!(back, H);
    let cleared = H.with::<Kind>(Bits::masked(0));
    assert_eq!(cleared.get::<Id>().raw(), 0x0abc_def1);
    assert!(cleared.get::<Flag>().get());
    assert_eq!(cleared.get::<Kind>().raw(), 0);
}
