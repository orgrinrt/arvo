// The twin of `the_field_named_from_outside_is_refused`.
use arvo_format::Adapt;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Uint, Wrapping};

fn main() {
    let a = Int::<8, Adapt<TowardZero, Wrapping>>::constant::<1>();
    let _ = a.raw();
}
