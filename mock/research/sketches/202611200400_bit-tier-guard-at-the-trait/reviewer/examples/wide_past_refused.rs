use arvo_bits::Bits;
use bit_tier_guard_reviewer::{Access, Wide};
fn g<B: Access>(b: B) -> bool {
    b.bit::<256>().get()
}
fn main() {
    let w = Wide([Bits::<64>::masked(0); 4]);
    let _ = g(w);
}
