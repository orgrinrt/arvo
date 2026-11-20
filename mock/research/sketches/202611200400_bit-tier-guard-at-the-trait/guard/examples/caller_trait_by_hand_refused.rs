// A container implementing the caller's trait by hand, skipping the proof.
use arvo_bits::{BitWidth, Bits};
use arvo_format::{Bool, Width};
use bit_tier_guard_sealed::{BitAccess, BitAccessAt, Within};
#[derive(Clone, Copy)]
struct Mine(Bits<8>);
impl BitWidth for Mine {
    const WIDTH: Width = Width::bits(8);
}
impl BitAccessAt for Mine {
    fn bit_at<const I: u32>(self, _: Within<Self, I>) -> Bool {
        Bool::of(false)
    }

    fn with_bit_at<const I: u32>(self, _: Within<Self, I>, _: Bool) -> Self {
        self
    }
}
impl BitAccess for Mine {
    fn bit<const I: u32>(self) -> Bool {
        Bool::of(false)
    }

    fn with_bit<const I: u32>(self, _: Bool) -> Self {
        self
    }
}
fn main() {}
