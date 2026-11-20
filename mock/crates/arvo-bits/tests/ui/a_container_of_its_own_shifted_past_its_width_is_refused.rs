// A container written here with no assertion of its own, shifted up by 128 of
// its 128 bits at run time, is refused at build by the tier's proof.

use arvo_bits::{BitSequence, BitSequenceBy, BitWidth, Bits, Within};
use arvo_format::Width;

#[derive(Clone, Copy, PartialEq, Debug)]
struct Pair([Bits<64>; 2]);

impl BitWidth for Pair {
    const WIDTH: Width = Width::bits(128);
}

impl BitSequenceBy for Pair {
    fn shift_up_by<const K: u32>(self, _: Within<Self, K>) -> Self {
        let wide = ((self.0[1].raw() as u128) << 64 | self.0[0].raw() as u128) << K;
        Pair([Bits::<64>::masked(wide as u64), Bits::<64>::masked((wide >> 64) as u64)])
    }

    fn shift_down_by<const K: u32>(self, _: Within<Self, K>) -> Self {
        let wide = ((self.0[1].raw() as u128) << 64 | self.0[0].raw() as u128) >> K;
        Pair([Bits::<64>::masked(wide as u64), Bits::<64>::masked((wide >> 64) as u64)])
    }

    fn rotate_up_by<const K: u32>(self, _: Within<Self, K>) -> Self {
        let wide = ((self.0[1].raw() as u128) << 64 | self.0[0].raw() as u128).rotate_left(K);
        Pair([Bits::<64>::masked(wide as u64), Bits::<64>::masked((wide >> 64) as u64)])
    }

    fn rotate_down_by<const K: u32>(self, _: Within<Self, K>) -> Self {
        let wide = ((self.0[1].raw() as u128) << 64 | self.0[0].raw() as u128).rotate_right(K);
        Pair([Bits::<64>::masked(wide as u64), Bits::<64>::masked((wide >> 64) as u64)])
    }
}

fn main() {
    let _ = Pair([Bits::<64>::masked(1); 2]).shift_up::<128>();
}
