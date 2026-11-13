// Two signatures are two types, and `add` takes one.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Uint};

fn main() {
    let a = Int::<8, Adapt<TowardZero, Wrap>>::constant::<1>();
    let b = Int::<9, Adapt<TowardZero, Wrap>>::constant::<1>();
    let _ = a.add(b);
}
