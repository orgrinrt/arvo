use arvo_bits::Bits;
fn main() {
    let _ = Bits::<64>::masked(1).with_bit::<64>(arvo_format::Bool::TRUE);
}
