// Index 256 through a function generic over the caller's trait.
use arvo_bits::Bits;
use bit_tier_guard_sealed::{BitAccess, Wide};
fn top<B: BitAccess>(b: B) -> bool {
    b.bit::<256>().get()
}
fn main() {
    let _ = top(Wide([Bits::<64>::masked(0); 4]));
}
