// Eight unsigned bits hold sixteen four-bit terms and do not hold thirty-two.
use arvo_format::overflow::Wrap;
use arvo_format::points::UFixed;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};
use arvo_numeral::Accumulator;

type Term = Signature<UFixed<4, 0>, Adapt<TowardZero, Wrap>>;
type Sum = Signature<UFixed<8, 0>, Adapt<TowardZero, Wrap>>;

fn main() {
    let _ = Accumulator::<Term, Sum, 32>::empty();
}
