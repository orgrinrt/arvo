// A difference of two unsigned sums goes below zero, so it is held on a signed grid.
use arvo_format::overflow::Wrap;
use arvo_format::points::{Integer, UFixed};
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};
use arvo_numeral::Accumulator;

type Term = Signature<UFixed<4, 0>, Adapt<TowardZero, Wrap>>;
type Sum = Signature<Integer<8>, Adapt<TowardZero, Wrap>>;

fn main() {
    let a = Accumulator::<Term, Sum, 2>::empty();
    let b = Accumulator::<Term, Sum, 2>::empty();
    let _ = Accumulator::<Term, Sum, 2>::minus(a, b);
}
