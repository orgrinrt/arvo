#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features, dead_code)]
pub struct W<const N: u32>;

pub trait Fwd {
    type const V: u32;
}
pub trait Rev {
    type const V: u32;
}
pub struct P2<const I: u32, const F: u32>;
impl<const I: u32, const F: u32> Fwd for P2<I, F> {
    type const V: u32 = const { I + F };
}
impl<const I: u32, const F: u32> Rev for P2<I, F> {
    type const V: u32 = const { F + I };
}

pub type Fw<const I: u32, const F: u32> = W<{ <P2<I, F> as Fwd>::V }>;
pub type Rv<const I: u32, const F: u32> = W<{ <P2<I, F> as Rev>::V }>;

// concrete: both reduce by const eval
pub fn concrete(x: Fw<13, 3>) -> Rv<13, 3> {
    x
}

// generic: equality is definitional
pub fn generic<const I: u32, const F: u32>(x: Fw<I, F>) -> Rv<I, F> {
    x
}
