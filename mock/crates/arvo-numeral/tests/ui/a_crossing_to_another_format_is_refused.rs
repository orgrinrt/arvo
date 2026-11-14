// A crossing keeps the slot, so its target shares the format.
use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::Integer;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};
use arvo_numeral::Num;

type Here = Signature<Integer<8>, Adapt<TowardZero, Wrap>>;
type There = Signature<Integer<9>, Adapt<TowardZero, Saturate>>;

fn main() {
    let a = Num::<Here>::constant::<1>();
    let _ = a.under::<There>();
}
