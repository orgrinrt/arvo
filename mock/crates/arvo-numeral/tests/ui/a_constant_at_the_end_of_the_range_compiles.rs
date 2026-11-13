// The twin of `a_constant_outside_the_range_is_refused`.
use arvo_format::Adapt;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Uint, Wrapping};

fn main() {
    let _ = Uint::<4, Adapt<TowardZero, Wrapping>>::constant::<15>();
}
