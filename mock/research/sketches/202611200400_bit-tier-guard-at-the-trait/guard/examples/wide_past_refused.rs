// Index 256 of the 256-bit set, at run time: refused at build by the tier's proof.
use arvo_bits::Bits;
use bit_tier_guard_sealed::{BitAccess, Wide};
fn main() {
    let w = Wide([Bits::<64>::masked(0); 4]);
    let _ = w.bit::<256>();
}
