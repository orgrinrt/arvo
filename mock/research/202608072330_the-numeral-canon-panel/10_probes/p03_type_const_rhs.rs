//! P03. Follow rustc's own suggestion from P02: declare the associated const
//! as `type const`. Does its RHS admit the impl's generic const params?
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
    type const WORDS: usize = ((I + F) as usize + 63) / 64;
    type C = [u64; <Key<I, F> as Pick>::WORDS];
}
