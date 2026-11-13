// No ladder carrier holds sixty-five bits.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Uint};

fn main() {
    let _ = Int::<65, Adapt<TowardZero, Wrap>>::constant::<0>();
}
