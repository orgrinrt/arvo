//! Probe 3: what a consumer reads when `nat!` is asked for a number outside
//! the table, which file 47 never compiled and which decides how far the
//! table shape stretches.
//!
//! File 47's proposal 3.1 bounds the table at 0..=1024 and its section 5
//! flags the powers-of-two gap. File 43's exact division subfamily makes the
//! gap the common case rather than the edge: division by any representable
//! constant is exact (`43:164-203`), and the constants a DSP consumer divides
//! by (a sample rate, a window length) sit at 44100 and 48000, far past any
//! table a metadata budget tolerates at 668 bytes per asserted row.
//!
//! CLAIM: the out-of-range failure is at least honest and names the missing
//! row, so a bounded table fails loudly rather than silently. The verbatim
//! error is the deliverable.
//!
//! EXPECTED: FAILS, cannot-find-type in module `n`.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   probe_3_the_out_of_range_diagnostic.rs

#![allow(dead_code)]
#![feature(macro_metavar_expr_concat)]

pub struct H;
pub struct O<P>(core::marker::PhantomData<P>);
pub struct I<P>(core::marker::PhantomData<P>);
pub struct Pz<P>(core::marker::PhantomData<P>);

pub mod n {
    pub type N37 = super::Pz<super::I<super::O<super::I<super::O<super::O<super::H>>>>>>;
}

macro_rules! nat {
    ($v:literal) => { $crate::n::${concat(N, $v)} };
}

// In range: fine.
pub type Covered = nat!(37);

// Out of range: the sample rate file 43's exact-division subfamily divides by.
pub type SampleRate = nat!(48000);
