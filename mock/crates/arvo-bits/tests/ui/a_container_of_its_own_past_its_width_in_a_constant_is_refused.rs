// The same container, written to at index 128 through a `[const]` bound in a
// constant, is refused at build by the tier's proof.

#![feature(const_trait_impl)]

use arvo_bits::{BitAccess, BitAccessAt, BitWidth, Bits, Within};
use arvo_format::{Bool, Width};

#[derive(Clone, Copy)]
struct Pair([Bits<64>; 2]);

const impl BitWidth for Pair {
    const WIDTH: Width = Width::bits(128);
}

const impl BitAccessAt for Pair {
    fn bit_at<const I: u32>(self, _: Within<Self, I>) -> Bool {
        Bool::of((self.0[(I / 64) as usize].raw() >> (I % 64)) & 1 == 1)
    }

    fn with_bit_at<const I: u32>(self, _: Within<Self, I>, _: Bool) -> Self {
        self
    }
}

const fn past<B: [const] BitAccess>(b: B) -> Bool {
    b.bit::<128>()
}

const PAST: Bool = past(Pair([Bits::<64>::masked(0); 2]));

fn main() {
    let _ = PAST;
}
