#![feature(const_trait_impl)]
use bit_tier_const_trait::{BitAccess, Bits};
const fn eighth<B: [const] BitAccess>(b: B) -> arvo_format::Bool {
    b.bit::<8>()
}
const _REFUSED: arvo_format::Bool = eighth(Bits::<8>::masked(1));
fn main() {}
