// The twin of `a_product_over_the_unsigned_64_bit_range_is_refused`.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Quiet, Uint};

fn main() {
    let a = Uint::<63, Adapt<TowardZero, Wrap>, Quiet>::constant::<3>();
    let _ = a.mul(a);
}
