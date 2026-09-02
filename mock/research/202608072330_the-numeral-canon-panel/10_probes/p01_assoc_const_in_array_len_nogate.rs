//! P01. BASELINE, no features. Can an associated const of the same impl,
//! computed from the impl's own const generic params, be used as an array
//! length in an associated TYPE of that impl?
//! If yes, the whole (I,F) -> container derivation needs no structural nat
//! and therefore no per-width bridge.
#![no_std]
#![crate_type = "lib"]

pub struct Key<const I: u32, const F: u32>;

pub trait Pick {
    const WORDS: usize;
    type C;
}

impl<const I: u32, const F: u32> Pick for Key<I, F> {
    const WORDS: usize = ((I + F) as usize + 63) / 64;
    type C = [u64; Self::WORDS];
}
