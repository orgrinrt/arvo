#![no_std]
use core::marker::PhantomData;
pub struct Idx<const N: u32>;
pub trait ToNat<M> {
    const VAL: u32;
}
pub struct Arvo; // arvo's own marker, the default
impl ToNat<Arvo> for Idx<3> {
    const VAL: u32 = 3;
}
impl ToNat<Arvo> for Idx<13> {
    const VAL: u32 = 13;
}
// the numeral: marker defaulted, so a consumer never writes it
pub struct Fixed<const I: u32, const F: u32, S, M = Arvo>(PhantomData<(S, M)>)
where
    Idx<I>: ToNat<M>,
    Idx<F>: ToNat<M>;
pub struct Warm;
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, S>;
// a law generic over the marker, so consumer widths flow through arvo's own code
pub fn widths<const I: u32, const F: u32, S, M>(_: &Fixed<I, F, S, M>) -> (u32, u32)
where
    Idx<I>: ToNat<M>,
    Idx<F>: ToNat<M>,
{
    (<Idx<I> as ToNat<M>>::VAL, <Idx<F> as ToNat<M>>::VAL)
}
