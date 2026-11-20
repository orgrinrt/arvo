#![feature(const_trait_impl)]
// Index 8 of `Eight` through a `[const]` bound in a constant.
use arvo_bits::Bits;
use arvo_format::Bool;
use bit_tier_guard_sealed::{BitAccess, Eight};
const fn past<B: [const] BitAccess>(b: B) -> Bool {
    b.bit::<8>()
}
const PAST: Bool = past(Eight(Bits::<8>::masked(0)));
fn main() {
    let _ = PAST;
}
