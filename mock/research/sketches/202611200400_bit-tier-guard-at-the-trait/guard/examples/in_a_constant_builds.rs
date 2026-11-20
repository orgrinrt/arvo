#![feature(const_trait_impl)]
// The caller's trait through a `[const]` bound, evaluated in a constant.
use arvo_bits::Bits;
use arvo_format::Bool;
use bit_tier_guard_sealed::{BitAccess, Eight, Wide};
const fn top<B: [const] BitAccess>(b: B) -> Bool {
    b.bit::<7>()
}
const fn last<B: [const] BitAccess>(b: B) -> Bool {
    b.with_bit::<255>(Bool::of(true)).bit::<255>()
}
const TOP: Bool = top(Eight(Bits::<8>::masked(0x80)));
const LAST: Bool = last(Wide([Bits::<64>::masked(0); 4]));
fn main() {
    assert!(TOP.get());
    assert!(LAST.get());
    println!("ok");
}
