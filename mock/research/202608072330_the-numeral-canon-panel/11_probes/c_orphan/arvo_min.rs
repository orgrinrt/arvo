//! c01a: the "arvo" side. Bridge trait, bridge carrier, arvo's own marker and
//! rows. Compiled as its own crate so the orphan rule is actually in play.
#![no_std]
#![crate_type = "lib"]
#![crate_name = "arvo_min"]

pub struct Term;
pub struct D0<T>(core::marker::PhantomData<T>);
pub struct D1<T>(core::marker::PhantomData<T>);
pub type T13 = D1<D0<D1<D1<Term>>>>;

pub struct Idx<const N: u32>;
pub struct Arvo;
pub trait ToNat<M> {
    type N;
}
impl ToNat<Arvo> for Idx<13> {
    type N = T13;
}

pub struct Fixed<const I: u32, const F: u32, M = Arvo>(
    core::marker::PhantomData<(Idx<I>, Idx<F>, M)>,
)
where
    Idx<I>: ToNat<M>,
    Idx<F>: ToNat<M>;
