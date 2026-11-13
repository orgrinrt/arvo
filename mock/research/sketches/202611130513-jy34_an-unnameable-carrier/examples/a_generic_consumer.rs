//! Must build: a consumer generic over `Sole` uses the value without naming
//! the carrier, so `S: Sole` is enough for `Num<S>` to be well formed.
use an_unnameable_carrier::{Num, Sole};
use arvo_format::overflow::Wrap;
use arvo_format::points::UFixed;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};

fn twice<S: Sole>(a: Num<S>) -> (Num<S>, Num<S>) {
    (a, a)
}

fn main() {
    type S13 = Signature<UFixed<13, 0>, Adapt<TowardZero, Wrap>>;
    let z = Num::<S13>::zero();
    let (a, b) = twice(z);
    assert!(a.same(b));
    assert_eq!(core::mem::size_of::<Num<S13>>(), 2);
}
