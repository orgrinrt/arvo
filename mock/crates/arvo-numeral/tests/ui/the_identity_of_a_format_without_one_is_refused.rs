// A half-step phase has no member that adds to every member without moving
// it. The identity's own assertion fires first, naming that.
use arvo_format::points::Biased;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};
use arvo_numeral::{Additive, Identity, Num, Wrapping};

fn main() {
    let _ = <Num<Signature<Biased<8, 0, 1>, Adapt<TowardZero, Wrapping>>> as Identity<Additive>>::IDENTITY;
}
