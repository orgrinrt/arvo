// The index goes out only as a host integer that holds every slot of the range.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::Int;

fn main() {
    let a = Int::<8, Adapt<TowardZero, Wrap>>::constant::<1>();
    let _ = a.index::<i8>();
}
