// stands in for arvo: owns Idx, ToNat and the digit structs.
#![no_std]
use core::marker::PhantomData;
pub struct End;
pub struct N1<T>(PhantomData<T>);
pub struct N4<T>(PhantomData<T>);
pub struct Idx<const N: u32>;
#[diagnostic::on_unimplemented(
    message = "no numeral is declared for width {Self}",
    note = "declare it once with `nat!(14)` in the crate that owns the numeral"
)]
pub trait ToNat {
    type N;
}
impl ToNat for Idx<13> {
    type N = N1<N4<End>>;
}
