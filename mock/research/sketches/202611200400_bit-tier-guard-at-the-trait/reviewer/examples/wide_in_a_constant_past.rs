#![feature(const_trait_impl)]
// The trait-owned guard through a `[const]` bound, for `Bits<8>` given a const
// impl of the probe trait; the refusal at 8 must still come from `At::NEW`.
use arvo_bits::{BitWidth, Bits};
use arvo_format::Bool;
use bit_tier_guard_reviewer::{Access, At};
#[derive(Clone, Copy)]
struct B8(Bits<8>);
impl BitWidth for B8 {
    const WIDTH: arvo_format::Width = <Bits<8> as BitWidth>::WIDTH;
}
const impl Access for B8 {
    fn read_at<const I: u32>(self, _: At<Self, I>) -> Bool {
        Bool::of((self.0.raw() >> I) & 1 == 1)
    }
}
const fn top<B: [const] Access>(b: B) -> Bool {
    b.bit::<8>()
}
const TOP: Bool = top(B8(Bits::<8>::masked(0x80)));
fn main() {
    assert!(TOP.get());
    println!("ok");
}
