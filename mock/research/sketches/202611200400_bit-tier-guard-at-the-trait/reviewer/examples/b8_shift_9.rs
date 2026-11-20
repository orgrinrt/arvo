use arvo_bits::Bits;
fn main() {
    let _ = Bits::<8>::masked(1).shift_down::<9>();
}
