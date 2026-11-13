// A consumer generic over the signature bounds it by `Sole` and nothing more,
// and names no feature of this crate or of any other.
use arvo_format::Adapt;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Num, Sole, Uint, Wrapping};

const fn sum<S: Sole>(a: Num<S>, b: Num<S>, c: Num<S>) -> Num<S> {
    a.add(b).add(c)
}

const SIX: Uint<13, Adapt<TowardZero, Wrapping>> = sum(
    Uint::constant::<1>(),
    Uint::constant::<2>(),
    Uint::constant::<3>(),
);

fn main() {
    assert_eq!(SIX, Uint::constant::<6>());
}
