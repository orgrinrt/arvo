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
pub struct Inc<T>(T);
impl<T: S> S for Inc<T> {
    type const V: u32 = <T as S>::V + 1;
}
