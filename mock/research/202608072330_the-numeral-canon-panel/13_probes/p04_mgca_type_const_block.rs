// P4. Follow the second suggestion: wrap the RHS in a const block.
#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features, dead_code)]

pub struct Store<const B: usize>([u8; B]);
pub struct M<const N: usize>;

pub trait Bytes {
    type const B: usize;
}

impl<const N: usize> Bytes for M<N> {
    type const B: usize = const { (N + 7) / 8 };
}

pub struct Derived<const N: usize>(Store<{ <M<N> as Bytes>::B }>);
