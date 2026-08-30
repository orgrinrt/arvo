// P2. Does min_generic_const_args accept an associated-const PATH as a const
// argument? If it does, the arithmetic moves into an impl body (where it is
// always legal) and the type position sees only a path, with no table.
#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features, dead_code)]

pub struct Store<const B: usize>([u8; B]);

pub struct M<const N: usize>;

pub trait Bytes {
    const B: usize;
}

impl<const N: usize> Bytes for M<N> {
    const B: usize = (N + 7) / 8; // arithmetic here is ordinary Rust
}

// the attempt: a path, not an expression, in const-argument position
pub struct Derived<const N: usize>(Store<{ <M<N> as Bytes>::B }>);
