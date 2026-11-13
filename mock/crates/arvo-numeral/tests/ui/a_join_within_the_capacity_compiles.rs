// Two accumulators join into one whose capacity holds both.
use arvo_format::overflow::Wrap;
use arvo_format::points::UFixed;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};
use arvo_numeral::Accumulator;

type Term = Signature<UFixed<4, 0>, Adapt<TowardZero, Wrap>>;
type Sum = Signature<UFixed<8, 0>, Adapt<TowardZero, Wrap>>;

fn main() {
    let a = Accumulator::<Term, Sum, 8>::empty();
    let b = Accumulator::<Term, Sum, 8>::empty();
    let _ = Accumulator::<Term, Sum, 16>::plus(a, b);
}
