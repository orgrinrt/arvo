// Proposed saturating-widen macro body for Precise, threaded with $container = i16, $bits = 8.
const fn i_logical_clamp_i16<const N: u16>(v: i16) -> i16 {
    let hi: i16 = (1i16 << (N - 1)) - 1;
    let lo: i16 = -(1i16 << (N - 1));
    if v < lo { lo } else if v > hi { hi } else { v }
}
const fn i_mul_fixed_precise<const N: u16, const FRAC: u16>(a: i16, b: i16) -> i16 {
    // DoubleLogical: i16 container holds the 2x product for in-range 8-bit logical operands.
    let prod = a.wrapping_mul(b); // wrapping in-container; saturation handled by logical clamp
    i_logical_clamp_i16::<N>(prod >> FRAC)
}
fn main() {
    const T: i16 = i_mul_fixed_precise::<8, 4>(100, 100);
    assert_eq!(T, 127);
    println!("body shape OK");
}
