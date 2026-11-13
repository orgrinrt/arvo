// The twin of `the_identity_of_a_format_without_one_is_refused`.
use arvo_format::overflow::Wrap;
use arvo_format::points::Biased;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};
use arvo_numeral::{Additive, Identity, Num};

fn main() {
    let _ =
        <Num<Signature<Biased<8, 0, 0>, Adapt<TowardZero, Wrap>>> as Identity<Additive>>::IDENTITY;
}
