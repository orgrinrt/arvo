// A container implementing `BitSequence` itself, beside `BitSequenceBy`, to take
// an amount without the proof, does not build: the tier's one impl covers it.

use arvo_bits::{BitSequence, BitSequenceBy, BitWidth, Bits, Within};
use arvo_format::Width;

#[derive(Clone, Copy)]
struct Pair([Bits<64>; 2]);

impl BitWidth for Pair {
    const WIDTH: Width = Width::bits(128);
}

impl BitSequenceBy for Pair {
    fn shift_up_by<const K: u32>(self, _: Within<Self, K>) -> Self {
        self
    }

    fn shift_down_by<const K: u32>(self, _: Within<Self, K>) -> Self {
        self
    }

    fn rotate_up_by<const K: u32>(self, _: Within<Self, K>) -> Self {
        self
    }

    fn rotate_down_by<const K: u32>(self, _: Within<Self, K>) -> Self {
        self
    }
}

impl BitSequence for Pair {
    fn shift_up<const K: u32>(self) -> Self {
        self
    }

    fn shift_down<const K: u32>(self) -> Self {
        self
    }

    fn rotate_up<const K: u32>(self) -> Self {
        self
    }

    fn rotate_down<const K: u32>(self) -> Self {
        self
    }
}

fn main() {}
