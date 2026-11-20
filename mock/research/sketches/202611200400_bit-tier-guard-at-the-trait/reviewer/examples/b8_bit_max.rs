use arvo_bits::Bits;
fn main() {
    let _ = Bits::<8>::masked(1).bit::<{ u32::MAX }>();
}
