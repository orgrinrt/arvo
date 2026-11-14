// The sibling under saturation is admitted, so the override is held.
use arvo_format::Adapt;
use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::TowardZero;
use arvo_numeral::Int;

fn main() {
    let a = Int::<8, Adapt<TowardZero, Wrap>>::constant::<1>();
    let _ = a.with_overflow::<Saturate>();
}
