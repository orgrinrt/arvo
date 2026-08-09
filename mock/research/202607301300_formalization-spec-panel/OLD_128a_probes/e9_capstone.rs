//! E9 capstone: both problems at once. The width bridge AND the container
//! projection, with the enumeration over the HARDWARE LADDER (6 rungs, the
//! codomain) instead of over the widths (the domain).
//!
//! Compare: sketch 202607282100 is GCE-free but enumerates one impl row per
//! width per family (`src/lib.rs:109`). This enumerates nothing over widths.
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]

pub struct Hot;
pub struct Warm;

// ---- the width carrier: ONE impl, any width, no cap ----------------------
pub struct Wid<const N: u16>;
pub trait Nat {
    const VAL: u16;
}
impl<const N: u16> Nat for Wid<N> {
    const VAL: u16 = N;
}

// ---- the bridge: arithmetic in value position, path in type position -----
pub trait Sum<const I: u16, const F: u16> {
    type const TOTAL: u16;
}
pub struct Bridge;
impl<const I: u16, const F: u16> Sum<I, F> for Bridge {
    type const TOTAL: u16 = const { I + F };
}
pub type PrecisionOf<const I: u16, const F: u16> = Wid<{ <Bridge as Sum<I, F>>::TOTAL }>;

// ---- the ladder: SIX rows, the hardware's own list, not the widths -------
pub trait Rung {
    type T;
}
pub struct R<const TAG: u8>;
impl Rung for R<0> {
    type T = u8;
}
impl Rung for R<1> {
    type T = u16;
}
impl Rung for R<2> {
    type T = u32;
}
impl Rung for R<3> {
    type T = u64;
}
impl Rung for R<4> {
    type T = u128;
}
impl Rung for R<5> {
    type T = [u8; 32];
} // the wide bucket

/// The classifier: ordinary const fn, value position, unrestricted.
pub const fn rung_of(n: u16) -> u8 {
    if n <= 8 {
        0
    } else if n <= 16 {
        1
    } else if n <= 32 {
        2
    } else if n <= 64 {
        3
    } else if n <= 128 {
        4
    } else {
        5
    }
}

pub trait Pick {
    type const TAG: u8;
}
impl<const N: u16> Pick for Wid<N> {
    type const TAG: u8 = const { rung_of(N) };
}

/// The projection. `W::TAG` is a path. Nothing computes in type position.
pub trait Container {
    type T;
}
impl<const N: u16> Container for Wid<N>
where
    R<{ <Wid<N> as Pick>::TAG }>: Rung,
{
    type T = <R<{ <Wid<N> as Pick>::TAG }> as Rung>::T;
}

// ---- verification: resolution, not merely parsing ------------------------
pub const fn same<A, B>() -> bool {
    core::mem::size_of::<A>() == core::mem::size_of::<B>()
}

const _: () = assert!(<PrecisionOf<13, 3> as Nat>::VAL == 16);
const _: () = assert!(<PrecisionOf<8, 8> as Nat>::VAL == 16);
const _: () = assert!(<PrecisionOf<40, 30> as Nat>::VAL == 70);
const _: () = assert!(<PrecisionOf<10000, 10000> as Nat>::VAL == 20000);

// canonicity: the two spellings meet at ONE type
pub fn takes16(_: Wid<16>) {}
pub fn canonicity() {
    takes16(PrecisionOf::<13, 3> {});
    takes16(PrecisionOf::<8, 8> {});
}

// container resolution at widths no table holds
const _: () = assert!(core::mem::size_of::<<Wid<13> as Container>::T>() == 2);
const _: () = assert!(core::mem::size_of::<<Wid<47> as Container>::T>() == 8);
const _: () = assert!(core::mem::size_of::<<Wid<7> as Container>::T>() == 1);
const _: () = assert!(core::mem::size_of::<<Wid<20000> as Container>::T>() == 32);
// and through the bridge, end to end
const _: () = assert!(core::mem::size_of::<<PrecisionOf<13, 3> as Container>::T>() == 2);
const _: () = assert!(core::mem::size_of::<<PrecisionOf<40, 30> as Container>::T>() == 16);
