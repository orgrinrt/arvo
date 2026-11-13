// `Sole` declares no carrier type to project.
use arvo_format::overflow::Wrap;
use arvo_format::points::Integer;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};
use arvo_numeral::Sole;

type S8 = Signature<Integer<8>, Adapt<TowardZero, Wrap>>;

fn main() {
    let _: Option<<S8 as Sole>::Repr> = None;
}
