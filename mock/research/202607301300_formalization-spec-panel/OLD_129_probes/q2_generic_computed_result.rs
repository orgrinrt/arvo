#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features, dead_code)]
pub struct Warm;
pub struct Number<const P: u32, S>(u128, core::marker::PhantomData<S>);

pub fn mul<const P: u32, const Q: u32>(
    _a: Number<P, Warm>,
    _b: Number<Q, Warm>,
) -> Number<{ P + Q }, Warm> {
    todo!()
}
