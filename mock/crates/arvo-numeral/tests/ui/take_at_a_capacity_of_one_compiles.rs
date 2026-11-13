// `take` sums a block of at least one term. At a capacity of zero it would
// hand back the slice it was given. The empty accumulator itself is admitted.
use arvo_format::overflow::Wrap;
use arvo_format::points::UFixed;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};
use arvo_numeral::{Accumulator, Num};

type Term = Signature<UFixed<4, 0>, Adapt<TowardZero, Wrap>>;
type Sum = Signature<UFixed<8, 0>, Adapt<TowardZero, Wrap>>;

fn main() {
    let terms = [Num::<Term>::constant::<1>()];
    let _ = Accumulator::<Term, Sum, 0>::empty();
    let _ = Accumulator::<Term, Sum, 1>::take(&terms);
}
