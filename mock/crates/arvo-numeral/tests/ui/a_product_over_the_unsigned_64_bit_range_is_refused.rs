// The format does not admit the product over the unsigned 64-bit range yet,
// so the value has none there. The refusal is the format's own.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Quiet, Uint};

fn main() {
    let a = Uint::<64, Adapt<TowardZero, Wrap>, Quiet>::constant::<3>();
    let _ = a.mul(a);
}
