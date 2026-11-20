//! Must fail: the fields sum to the word's width, but two share a bit and the
//! top bit belongs to none.

#![feature(const_trait_impl)]

use arvo_bits::Bits;
use arvo_format::Width;
use packed_word::{Field, Layout, Word};

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

const K: Bits<4> = Word::<Handle>::of(Bits::masked(0)).get::<Kind>();

fn main() {
    let _ = K;
}
