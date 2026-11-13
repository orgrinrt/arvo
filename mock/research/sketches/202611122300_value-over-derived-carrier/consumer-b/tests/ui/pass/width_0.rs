use arm_b::Num;
use arvo_format::overflow::Wrap;
use arvo_format::points::Integer;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};

type S = Signature<Integer<1>, Adapt<TowardZero, Wrap>>;

fn main() {
    let _ = Num::<S>::from_raw(0);
}
