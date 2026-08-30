#![no_std]
#![allow(dead_code)]
extern crate arvo;
pub use arvo::*;

#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a width",
    label = "a width is written `w!(1 3)` for 13, or one of the `W*` aliases",
    note = "widths are type-level numerals; arvo admits any width"
)]
pub trait WrittenWidth {
    type N: Nat;
}
impl<T: Nat> WrittenWidth for T {
    type N = T;
}

pub type UF<Iw, Fw, S> =
    Number<FixedNumeral<Sum<<Iw as WrittenWidth>::N, <Fw as WrittenWidth>::N>, NonNegative>, S>;
