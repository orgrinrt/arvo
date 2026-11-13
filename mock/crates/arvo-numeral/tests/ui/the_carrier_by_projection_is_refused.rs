// A consumer generic over `Sole` cannot name the carrier by projecting it.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Num, Sole};

type I8 = Int<8, Adapt<TowardZero, Wrap>>;

fn carrier<S: Sole>(_: Num<S>) -> Option<S::Repr> {
    None
}

fn main() {
    let _ = carrier(I8::constant::<1>());
}
