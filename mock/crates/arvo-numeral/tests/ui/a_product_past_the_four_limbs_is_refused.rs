// At a quantum of two to the -256 the product's denominator is two to the 256th,
// past the four 64-bit limbs the format forms its step in. The refusal is the format's own.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Quiet, UFix};

fn main() {
    let a = UFix::<8, -256, Adapt<TowardZero, Wrap>, Quiet>::constant::<3>();
    let _ = a.mul(a);
}
