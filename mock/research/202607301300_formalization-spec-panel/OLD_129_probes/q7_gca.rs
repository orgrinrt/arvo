#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
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

pub trait Prod<const P: u32, const Q: u32> {
    type const SUM: u32;
}
pub struct Pair<const P: u32, const Q: u32>;
impl<const P: u32, const Q: u32> Prod<P, Q> for Pair<P, Q> {
    type const SUM: u32 = const { P + Q };
}

pub fn mul<const P: u32, const Q: u32>(
    a: Number<P, Warm>,
    b: Number<Q, Warm>,
) -> Number<{ <Pair<P, Q> as Prod<P, Q>>::SUM }, Warm> {
    Number(a.0 * b.0, core::marker::PhantomData)
}

pub fn wants32(_: Number<32, Warm>) {}
pub fn ok() {
    let a: Number<16, Warm> = Number::new(1);
    let b: Number<16, Warm> = Number::new(2);
    wants32(mul(a, b));
}

// the same wrong generic wrapper as q6: does GCA refuse it uninstantiated?
pub fn square_wrong<const P: u32>(x: Number<P, Warm>) -> Number<P, Warm> {
    mul::<P, P>(x, x)
}
