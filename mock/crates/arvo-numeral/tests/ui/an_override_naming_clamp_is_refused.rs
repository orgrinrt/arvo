// The alias accepts any policy, and the sibling still owes its own admission.
use arvo_format::Adapt;
use arvo_format::overflow::{Clamp, Wrap};
use arvo_format::rounding::TowardZero;
use arvo_numeral::Int;

fn main() {
    let a = Int::<8, Adapt<TowardZero, Wrap>>::constant::<1>();
    let _ = a.with_overflow::<Clamp>();
}
