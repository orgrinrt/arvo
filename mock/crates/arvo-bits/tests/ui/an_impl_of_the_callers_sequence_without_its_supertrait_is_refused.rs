// A container implementing `BitSequence` with no `BitSequenceBy`, the last
// branch of the seal, does not build: the caller's trait asks for the
// implementor's, so no container moves its bits without the proof.

use arvo_bits::{BitSequence, BitWidth, Bits};
use arvo_format::Width;

#[derive(Clone, Copy)]
struct Pair([Bits<64>; 2]);

impl BitWidth for Pair {
    const WIDTH: Width = Width::bits(128);
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
