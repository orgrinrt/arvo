use arvo_bits::Bits;
use bit_tier_guard_reviewer::{Access, Wide};
fn g<B: Access>(b: B) -> bool {
    b.bit::<255>().get()
}
fn main() {
    let w = Wide([
        Bits::<64>::masked(0),
        Bits::<64>::masked(0),
        Bits::<64>::masked(0),
        Bits::<64>::masked(1 << 63),
    ]);
    assert!(w.bit::<255>().get());
    assert!(g(w));
    println!("ok");
}
