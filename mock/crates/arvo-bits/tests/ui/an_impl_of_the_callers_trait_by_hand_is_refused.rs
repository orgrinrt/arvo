// A container implementing `BitAccess` itself, beside `BitAccessAt`, to take an
// index without the proof, does not build: the tier's one impl already covers it.

use arvo_bits::{BitAccess, BitAccessAt, BitWidth, Bits, Within};
use arvo_format::{Bool, Width};

#[derive(Clone, Copy, PartialEq, Debug)]
struct Pair([Bits<64>; 2]);

impl BitWidth for Pair {
    const WIDTH: Width = Width::bits(128);
}

impl BitAccessAt for Pair {
    fn bit_at<const I: u32>(self, _: Within<Self, I>) -> Bool {
        Bool::of((self.0[(I / 64) as usize].raw() >> (I % 64)) & 1 == 1)
    }

    fn with_bit_at<const I: u32>(self, _: Within<Self, I>, to: Bool) -> Self {
        let mut out = self;
        let word = self.0[(I / 64) as usize].raw() & !(1u64 << (I % 64));
        out.0[(I / 64) as usize] = Bits::<64>::masked(word | ((to.get() as u64) << (I % 64)));
        out
    }
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
