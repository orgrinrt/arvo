// The twin of `the_carrier_by_projection_is_refused`.
use arvo_format::Adapt;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Num, Sole, Wrapping};

type I8 = Int<8, Adapt<TowardZero, Wrapping>>;

fn slot<S: Sole>(v: Num<S>) -> Option<arvo_format::Slot> {
    Some(v.slot())
}

fn main() {
    let _ = slot(I8::constant::<1>());
}
