//! Must fail: the fields sum to thirty-one bits in a thirty-two-bit word.

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
    type Value = Bits<3>;
    const BELOW: Width = Width::bits(28);
}

const K: Bits<3> = Word::<Handle>::of(Bits::masked(0)).get::<Kind>();

fn main() {
    let _ = K;
}
