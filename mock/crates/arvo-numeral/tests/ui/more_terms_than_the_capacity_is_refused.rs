// An array goes in whole, so it is no longer than the capacity.
use arvo_format::overflow::Wrap;
use arvo_format::points::UFixed;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};
use arvo_numeral::{Accumulator, Num};

type Term = Signature<UFixed<4, 0>, Adapt<TowardZero, Wrap>>;
type Sum = Signature<UFixed<8, 0>, Adapt<TowardZero, Wrap>>;

fn main() {
    let terms = [Num::<Term>::constant::<1>(); 5];
    let _ = Accumulator::<Term, Sum, 4>::of(&terms);
}
