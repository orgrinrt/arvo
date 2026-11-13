// The twin of `the_one_of_a_format_without_one_is_refused`.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Identity, Int, Multiplicative};

fn main() {
    let _ = <Int<2, Adapt<TowardZero, Wrap>> as Identity<Multiplicative>>::IDENTITY;
}
