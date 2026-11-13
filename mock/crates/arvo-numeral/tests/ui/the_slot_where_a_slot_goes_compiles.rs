// The twin of `a_value_where_a_host_integer_goes_is_refused`.
use arvo_format::Adapt;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Uint, Wrapping};

fn main() {
    let a = Int::<8, Adapt<TowardZero, Wrapping>>::constant::<1>();
    let _: arvo_format::Slot = a.slot();
}
