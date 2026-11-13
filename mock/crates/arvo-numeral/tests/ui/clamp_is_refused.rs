// Clamping with no bound to carry is saturation under another name.
use arvo_format::Adapt;
use arvo_format::overflow::Clamp;
use arvo_format::rounding::TowardZero;
use arvo_numeral::Int;

fn main() {
    let _ = Int::<8, Adapt<TowardZero, Clamp>>::constant::<0>();
}
