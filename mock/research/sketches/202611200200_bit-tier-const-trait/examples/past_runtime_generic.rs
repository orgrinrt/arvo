use bit_tier_const_trait::{BitAccess, Bits};
fn eighth<B: BitAccess>(b: B) -> arvo_format::Bool {
    b.bit::<8>()
}
fn main() {
    let _ = eighth(Bits::<8>::masked(std::hint::black_box(1))).get();
}
