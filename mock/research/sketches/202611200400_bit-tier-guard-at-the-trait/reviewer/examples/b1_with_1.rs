use arvo_bits::Bits;
fn main() {
    let _ = Bits::<1>::masked(1).with_bit::<1>(arvo_format::Bool::TRUE);
}
