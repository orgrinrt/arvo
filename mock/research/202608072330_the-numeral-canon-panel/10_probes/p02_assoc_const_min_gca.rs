//! P02. Same as P01 but under `min_generic_const_args`, which is ALLOWED by
//! the workspace unstable-features rule. Two spellings of the same path:
//! `Self::WORDS` and the fully qualified `<Key<I,F> as Pick>::WORDS`.
#![no_std]
#![crate_type = "lib"]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

pub struct Key<const I: u32, const F: u32>;

pub trait Pick {
    const WORDS: usize;
    type C;
}

impl<const I: u32, const F: u32> Pick for Key<I, F> {
    const WORDS: usize = ((I + F) as usize + 63) / 64;
    type C = [u64; <Key<I, F> as Pick>::WORDS];
}
