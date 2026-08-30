#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features, dead_code)]
pub struct Warm;
pub struct Number<const P: u32, S>(u128, core::marker::PhantomData<S>);
impl<const P: u32, S> Clone for Number<P, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const P: u32, S> Copy for Number<P, S> {}
impl<const P: u32, S> Number<P, S> {
    pub const fn new(raw: u128) -> Self {
        Number(raw, core::marker::PhantomData)
    }
}

/// Door two: the arithmetic lives in an ordinary associated-const body, where
/// generic parameters are unrestricted because the result is a value.
pub struct Pair<const P: u32, const Q: u32>;
pub trait Sum {
    type const S: u32;
}
impl<const P: u32, const Q: u32> Sum for Pair<P, Q> {
    type const S: u32 = P + Q;
}

/// The relation is a where clause, so it is checked where the signature is,
/// not where the function is instantiated.
pub fn mul<const P: u32, const Q: u32, const R: u32>(
    a: Number<P, Warm>,
    b: Number<Q, Warm>,
) -> Number<R, Warm>
where
    Pair<P, Q>: Sum<S = { R }>,
{
    Number(a.0 * b.0, core::marker::PhantomData)
}

pub fn wants32(_: Number<32, Warm>) {}
pub fn ok() {
    let a: Number<16, Warm> = Number::new(1);
    let b: Number<16, Warm> = Number::new(2);
    wants32(mul(a, b));
}
