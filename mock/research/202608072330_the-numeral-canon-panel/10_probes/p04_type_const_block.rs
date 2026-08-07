//! P04. Follow rustc's second suggestion: wrap the `type const` RHS in a
//! `const { ... }` block. Does the block admit the impl's const params?
#![no_std]
#![crate_type = "lib"]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

pub struct Key<const I: u32, const F: u32>;

pub trait Pick {
    type const WORDS: usize;
    type C;
}

impl<const I: u32, const F: u32> Pick for Key<I, F> {
    type const WORDS: usize = const { ((I + F) as usize + 63) / 64 };
    type C = [u64; <Key<I, F> as Pick>::WORDS];
}
