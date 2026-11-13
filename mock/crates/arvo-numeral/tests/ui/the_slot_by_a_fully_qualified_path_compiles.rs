// The twin of `the_carrier_by_a_fully_qualified_projection_is_refused`.
use arvo_format::overflow::Wrap;
use arvo_format::points::Integer;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature, Slot};
use arvo_numeral::{Num, Sole};

type S8 = Signature<Integer<8>, Adapt<TowardZero, Wrap>>;

fn main() {
    let _: Option<Slot> = Some(<Num<S8>>::slot(Num::<S8>::constant::<1>()));
    let _ = <S8 as Sole>::WIDTH;
}
