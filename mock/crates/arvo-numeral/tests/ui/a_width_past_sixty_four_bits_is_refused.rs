// No ladder carrier holds sixty-five bits.
use arvo_format::Adapt;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Uint, Wrapping};

fn main() {
    let _ = Int::<65, Adapt<TowardZero, Wrapping>>::constant::<0>();
}
