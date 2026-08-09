#![no_std]
extern crate arvocore;
use arvocore::{Idx, Nat, ToNat};
// the consumer's OWN marker, local to this crate
pub struct MyWidths;
// can a downstream crate populate the bridge for a width arvo never listed?
impl ToNat<MyWidths> for Idx<4095> {
    type Out = Nat<4095>;
    const VAL: u32 = 4095;
}
impl ToNat<MyWidths> for Idx<777> {
    type Out = Nat<777>;
    const VAL: u32 = 777;
}
const _: () = assert!(<Idx<4095> as ToNat<MyWidths>>::VAL == 4095);
const _: () = assert!(<Idx<777> as ToNat<MyWidths>>::VAL == 777);
