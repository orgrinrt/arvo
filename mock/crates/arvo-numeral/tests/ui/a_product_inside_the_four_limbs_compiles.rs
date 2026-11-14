// The twin of `a_product_past_the_four_limbs_is_refused`.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Quiet, UFix};

fn main() {
    let a = UFix::<8, -255, Adapt<TowardZero, Wrap>, Quiet>::constant::<3>();
    let _ = a.mul(a);
}
