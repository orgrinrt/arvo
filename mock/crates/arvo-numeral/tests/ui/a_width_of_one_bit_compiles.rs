// The twin of `a_width_of_zero_bits_is_refused`: one bit has a slot range.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Uint};

fn main() {
    let _ = Uint::<1, Adapt<TowardZero, Wrap>>::constant::<0>();
}
