// Probe 1b: the const-expression route to a numeral-sized byte buffer, refused.
// Companion to probe_1 (the associated-type route that works). Expected fail.

pub trait HasStoredWidthBits {
    const STORED_WIDTH_BITS: u16;
}

pub const fn byte_width_of(bits: u16) -> u16 {
    (bits + 7) / 8
}

pub struct ThirteenBitDense;
impl HasStoredWidthBits for ThirteenBitDense {
    const STORED_WIDTH_BITS: u16 = 13;
}

// this is the route the spine rule forbids: a computed quantity used directly
// in an array-length position inside a function generic over the numeral.
pub fn zero_bytes<N: HasStoredWidthBits>() -> [u8; byte_width_of(N::STORED_WIDTH_BITS) as usize] {
    [0u8; byte_width_of(N::STORED_WIDTH_BITS) as usize]
}

fn main() {
    let _a = zero_bytes::<ThirteenBitDense>();
}
