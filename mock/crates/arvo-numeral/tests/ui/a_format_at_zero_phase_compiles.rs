// The twin of `a_phased_format_is_refused`.
use arvo_format::overflow::Wrap;
use arvo_format::points::Biased;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};
use arvo_numeral::Num;

fn main() {
    let _ = Num::<Signature<Biased<8, 0, 0>, Adapt<TowardZero, Wrap>>>::constant::<0>();
}
