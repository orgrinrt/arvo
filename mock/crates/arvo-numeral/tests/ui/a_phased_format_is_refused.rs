// At a half-step phase the sum of two slots is not the slot of the sum.
use arvo_format::points::Biased;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};
use arvo_numeral::{Num, Wrapping};

fn main() {
    let _ = Num::<Signature<Biased<8, 0, 1>, Adapt<TowardZero, Wrapping>>>::constant::<0>();
}
