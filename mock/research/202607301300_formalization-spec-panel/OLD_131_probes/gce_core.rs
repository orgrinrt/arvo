// The SHIPPED shape (arvo-strategy/src/container.rs:254) under GCE, purely as a
// confinement comparison. GCE is forbidden; this measures the status quo only.
#![no_std]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use core::marker::PhantomData;
pub trait Sign: Copy {
    const EXTRA: u32;
}
#[derive(Clone, Copy)]
pub struct Unsigned;
impl Sign for Unsigned {
    const EXTRA: u32 = 0;
}
pub struct Warm;
pub trait Project<const TAG: usize, G: Sign> {
    type T: Copy;
}
pub struct Picker;
impl<G: Sign> Project<0, G> for Picker {
    type T = u16;
}
impl<G: Sign> Project<1, G> for Picker {
    type T = u32;
}
impl<G: Sign> Project<2, G> for Picker {
    type T = u64;
}
pub const fn tag(w: u32) -> usize {
    if w <= 8 {
        0
    } else if w <= 16 {
        1
    } else {
        2
    }
}
pub trait Store<const I: u32, const F: u32, G: Sign> {
    type T: Copy;
}
impl<const I: u32, const F: u32, G: Sign> Store<I, F, G> for Warm
where
    Picker: Project<{ tag(G::EXTRA + I + F) }, G>,
{
    type T = <Picker as Project<{ tag(G::EXTRA + I + F) }, G>>::T;
}

pub struct Fixed<const I: u32, const F: u32, G: Sign, S: Store<I, F, G>> {
    raw: <S as Store<I, F, G>>::T,
    _m: PhantomData<G>,
}
impl<const I: u32, const F: u32, G: Sign, S: Store<I, F, G>> Clone for Fixed<I, F, G, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, G: Sign, S: Store<I, F, G>> Copy for Fixed<I, F, G, S> {}
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, Unsigned, S>;
pub fn mul<
    const I: u32,
    const F: u32,
    const J: u32,
    const K: u32,
    const M: u32,
    const N: u32,
    G: Sign,
    S,
>(
    _a: Fixed<I, F, G, S>,
    _b: Fixed<J, K, G, S>,
) -> Fixed<M, N, G, S>
where
    S: Store<I, F, G> + Store<J, K, G> + Store<M, N, G>,
{
    const { assert!(true) }
    unimplemented!()
}
