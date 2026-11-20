// No slot range exists at sixty-five bits, so `Slots` is refused before any carrier is asked.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Uint};

fn main() {
    let _ = Int::<65, Adapt<TowardZero, Wrap>>::constant::<0>();
}
