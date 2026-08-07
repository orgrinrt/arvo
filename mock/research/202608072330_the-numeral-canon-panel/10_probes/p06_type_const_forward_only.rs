//! P06. Instrument four: is `type const` usable at all when its RHS mentions
//! NO generic parameter, and can a generic impl then read another type's
//! `type const` in const-argument position? This isolates which half is refused:
//! reading a `type const` (fine?) versus computing one from a param (refused).
#![no_std]
#![crate_type = "lib"]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

pub struct Arr<const N: usize>;
pub trait Sized2 {
    type const N: usize;
}
pub struct A;
pub struct B;
impl Sized2 for A {
    type const N: usize = 2;
}
impl Sized2 for B {
    type const N: usize = 8;
}

// read a concrete impl's `type const` in const-argument position
pub type TwoWords = [u64; <A as Sized2>::N];

// now the same read, but through a generic type parameter
pub struct Holder<T: Sized2>(pub [u64; <T as Sized2>::N]);
