// A term enters by its slot index, so the accumulator is on the terms' grid.
use arvo_format::overflow::Wrap;
use arvo_format::points::Fixed;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};
use arvo_numeral::Accumulator;

type Term = Signature<Fixed<6, -3>, Adapt<TowardZero, Wrap>>;
type Sum = Signature<Fixed<16, -3>, Adapt<TowardZero, Wrap>>;

fn main() {
    let _ = Accumulator::<Term, Sum, 4>::empty();
}
