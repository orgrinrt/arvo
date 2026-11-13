// A value is not its carrier; `raw` is the door.
use arvo_format::Adapt;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Uint, Wrapping};

fn main() {
    let a = Int::<8, Adapt<TowardZero, Wrapping>>::constant::<1>();
    let _: i8 = a;
}
