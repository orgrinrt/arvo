use arm_a::Num;
use arvo_format::overflow::Wrap;
use arvo_format::points::Integer;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};

type S13 = Signature<Integer<13>, Adapt<TowardZero, Wrap>>;

fn main() {
    let _: Option<Num<S13, i32>> = None;
}
