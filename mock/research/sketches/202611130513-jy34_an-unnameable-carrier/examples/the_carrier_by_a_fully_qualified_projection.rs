//! Must not build: the carrier through a fully qualified projection on `Sole`.
use an_unnameable_carrier::Sole;
use arvo_format::overflow::Wrap;
use arvo_format::points::UFixed;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};

type S13 = Signature<UFixed<13, 0>, Adapt<TowardZero, Wrap>>;
type Carrier = <S13 as Sole>::Repr;

fn main() {}
