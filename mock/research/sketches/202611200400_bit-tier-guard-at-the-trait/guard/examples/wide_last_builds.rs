// Index 255 of the 256-bit set builds and reads back what was written.
use arvo_bits::Bits;
use arvo_format::Bool;
use bit_tier_guard_sealed::{BitAccess, Wide};
fn main() {
    let w = Wide([Bits::<64>::masked(0); 4]).with_bit::<255>(Bool::of(true));
    assert!(w.bit::<255>().get());
    assert!(!w.bit::<254>().get());
    println!("ok");
}
