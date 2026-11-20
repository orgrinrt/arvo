// A proof built by hand rather than through `NEW`.
use core::marker::PhantomData;

use arvo_bits::Bits;
use bit_tier_guard_sealed::{BitAccessAt, Wide, Within};
fn main() {
    let w = Wide([Bits::<64>::masked(0); 4]);
    let _ = w.bit_at::<300>(Within(PhantomData));
}
