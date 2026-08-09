#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features, dead_code)]
pub struct W<const N: u32>;
pub trait S {
    type const V: u32;
}
impl<const P: u32> S for W<P> {
    type const V: u32 = P;
}
pub fn f<const P: u32, const R: u32>(_: W<P>)
where
    W<P>: S<V = { R }>,
{
}
