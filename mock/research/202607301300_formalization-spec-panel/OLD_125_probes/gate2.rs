#![no_std]
#![allow(dead_code)]
extern crate arvo;
pub use arvo::*;

#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a width arvo can write",
    label = "expected a width here",
    note = "a width is written `w!(1 3)` for 13, `b!(1 1 0 1)` in binary, or a `W*` alias"
)]
pub trait WidthPair<Rhs> {
    type Out: Nat;
}
#[diagnostic::do_not_recommend]
impl<A: Nat + NatAdd<B>, B: Nat> WidthPair<B> for A {
    type Out = <A as NatAdd<B>>::Out;
}

pub type UF<Iw, Fw, S> = Number<FixedNumeral<<Iw as WidthPair<Fw>>::Out, NonNegative>, S>;
