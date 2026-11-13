// Sixteen is not a member of four unsigned bits.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Uint};

fn main() {
    let _ = Uint::<4, Adapt<TowardZero, Wrap>>::constant::<16>();
}
