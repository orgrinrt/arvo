// A one-bit signed integer holds zero and minus one, and no member is one.
// The identity's own constant fails, naming that.
use arvo_contracts::{Identity, Multiplicative};
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::Int;

fn main() {
    let _ = <Int<1, Adapt<TowardZero, Wrap>> as Identity<Multiplicative>>::IDENTITY;
}
