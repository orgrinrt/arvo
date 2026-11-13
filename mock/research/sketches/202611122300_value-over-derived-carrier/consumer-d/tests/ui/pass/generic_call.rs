use arm_d::Num;
use arvo_format::overflow::Wrap;
use arvo_format::points::Integer;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};

type S40 = Signature<Integer<40>, Adapt<TowardZero, Wrap>>;

fn generic<S: common::Sole<Repr = i64>>(a: Num<S>, b: Num<S>) -> Num<S> {
    a.add(b)
}

fn main() {
    let (a, b) = (Num::<S40>::from_raw(1), Num::<S40>::from_raw(2));
    let _ = generic::<S40>(a, b);
}
