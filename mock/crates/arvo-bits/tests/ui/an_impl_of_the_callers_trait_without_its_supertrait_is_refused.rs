// A container implementing `BitAccess` with no `BitAccessAt`, the seal's other
// branch, does not build: the caller's trait asks for the implementor's.

use arvo_bits::{BitAccess, BitWidth, Bits};
use arvo_format::{Bool, Width};

#[derive(Clone, Copy)]
struct Pair([Bits<64>; 2]);

impl BitWidth for Pair {
    const WIDTH: Width = Width::bits(128);
}

impl BitAccess for Pair {
    fn bit<const I: u32>(self) -> Bool {
        Bool::of(false)
    }

    fn with_bit<const I: u32>(self, _: Bool) -> Self {
        self
    }
}

fn main() {}
