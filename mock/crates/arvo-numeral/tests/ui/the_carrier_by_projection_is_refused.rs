// A consumer generic over `Sole` cannot name the carrier by projecting it.
use arvo_format::Adapt;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Num, Sole, Wrapping};

type I8 = Int<8, Adapt<TowardZero, Wrapping>>;

fn carrier<S: Sole>(_: Num<S>) -> Option<S::Repr> {
    None
}

fn main() {
    let _ = carrier(I8::constant::<1>());
}
