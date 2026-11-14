// The host door takes the host's integers of 8 to 64 bits and no wider.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::Uint;

fn main() {
    let _ = Uint::<9, Adapt<TowardZero, Wrap>>::from_index(1i128);
}
