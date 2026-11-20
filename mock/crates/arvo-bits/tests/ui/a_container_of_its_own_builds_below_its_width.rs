// A container written here, 128 bits as two `Bits<64>`, with no assertion of its
// own, builds and runs at every index up to its last, read and written through
// the caller's trait and through a function generic over it.

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

fn last<B: BitAccess>(b: B) -> B {
    b.with_bit::<127>(Bool::TRUE)
}

fn main() {
    let empty = Pair([Bits::<64>::masked(0); 2]);
    let top = last(empty);
    assert!(top.bit::<127>().get());
    assert!(!top.bit::<126>().get());
    assert!(!top.bit::<63>().get());
    let low = empty.with_bit::<64>(Bool::TRUE).with_bit::<0>(Bool::TRUE);
    assert_eq!(low.0[0].raw(), 1);
    assert_eq!(low.0[1].raw(), 1);
}
