use arvo_bits::Bits;
fn main() {
    let _ = Bits::<64>::masked(1).rotate_up::<64>();
}
