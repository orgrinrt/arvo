// P3. Follow rustc's own suggestion from P2: declare it as `type const`.
// The recorded prior finding is that a `type const` RHS may not use a generic
// parameter. Check that directly rather than believe it.
#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features, dead_code)]

pub struct Store<const B: usize>([u8; B]);
pub struct M<const N: usize>;

pub trait Bytes {
    type const B: usize;
}

impl<const N: usize> Bytes for M<N> {
    type const B: usize = (N + 7) / 8;
}

pub struct Derived<const N: usize>(Store<{ <M<N> as Bytes>::B }>);
