#![no_std]
pub trait Nat {
    const V: usize;
}
pub struct N13;
impl Nat for N13 {
    const V: usize = 13;
}
pub struct N3;
impl Nat for N3 {
    const V: usize = 3;
}
pub struct Arr<X>(core::marker::PhantomData<X>);
// concrete operand: no generic parameter mentioned
pub type Ok1 = [u8; <N13 as Nat>::V];
// generic operand: a TYPE parameter inside an anonymous constant
pub struct Bad<X: Nat>(pub [u8; <X as Nat>::V]);
