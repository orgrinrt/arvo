// No slot range exists at zero bits, so no carrier and no value.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Uint};

fn main() {
    let _ = Uint::<0, Adapt<TowardZero, Wrap>>::constant::<0>();
}
