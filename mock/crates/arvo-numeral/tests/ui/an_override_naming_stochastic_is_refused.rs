// A value type written through the alias is refused as the written one is.
use arvo_format::overflow::Wrap;
use arvo_format::points::Integer;
use arvo_format::rounding::{Stochastic, TowardZero};
use arvo_format::{Adapt, Signature, WithRounding};
use arvo_numeral::Num;

type Wrapping = Signature<Integer<8>, Adapt<TowardZero, Wrap>>;

fn main() {
    let _ = Num::<WithRounding<Wrapping, Stochastic>>::constant::<0>();
}
