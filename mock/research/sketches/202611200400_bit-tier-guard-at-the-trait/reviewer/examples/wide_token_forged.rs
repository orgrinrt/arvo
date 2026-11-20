// Can a consumer forge the token past the width? Field is private, so only NEW.
use arvo_bits::Bits;
use bit_tier_guard_reviewer::{Access, At, Wide};
fn main() {
    let w = Wide([Bits::<64>::masked(0); 4]);
    let _ = w.read_at::<300>(At::<Wide, 300>::NEW);
}
