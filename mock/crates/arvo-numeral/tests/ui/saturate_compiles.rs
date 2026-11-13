// The twin of `clamp_is_refused`.
use arvo_format::Adapt;
use arvo_format::overflow::Saturate;
use arvo_format::rounding::TowardZero;
use arvo_numeral::Int;

fn main() {
    let _ = Int::<8, Adapt<TowardZero, Saturate>>::constant::<0>();
}
