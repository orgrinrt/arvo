// Probe 3, file 82. Where file 81's const-position rule actually bites, and where it does
// not.
//
// File 81 section 2.1 states it as a blanket: "a fact the fourth rule requires to be
// settled at compile time has to be written in a const position to be settled there. An
// associated const on the layout type is; a `const fn` called from the decode is not."
// (81:236-238). That sentence, taken literally, condemns file 79's `last_index` and
// `in_bounds`, which are `const fn`s called from value position (79_probes/probe_1:117,
// probe_2:139), and it would condemn most of the tower's projection surface.
//
// But file 79's bodies are `<C as Dec>::Out::VAL` and `i < C::SIZE`: a single read of an
// associated const whose value the trait solver has already produced. File 81's failing
// case is a `const fn` whose BODY performs recursion LLVM then has to fold. Those are two
// different things and the spec should not spell them as one, because a blanket rule that
// condemns the harmless shape produces noise and gets ignored.
//
// This probe compiles three shapes at -O on the pinned toolchain and counts instructions
// in each emitted function, to establish where the boundary sits.
//
//   A. `const fn` body reading an associated const (file 79's shape)
//   B. `const fn` body computing by recursion (file 81's shape)
//   C. associated const holding the computed value (file 81's fix)
//
// Compile: rustc --edition 2021 --crate-type=lib -O --emit=obj, then objdump -d.
// Outcome in OUTCOMES.md.
#![no_std]
#![allow(dead_code)]

use core::marker::PhantomData;

pub trait Nat {
    const VAL: usize;
}
pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);
impl Nat for H {
    const VAL: usize = 1;
}
impl<P: Nat> Nat for O<P> {
    const VAL: usize = 2 * P::VAL;
}
impl<P: Nat> Nat for I<P> {
    const VAL: usize = 2 * P::VAL + 1;
}

pub type W13 = I<O<I<H>>>; // 13
const _: () = assert!(<W13 as Nat>::VAL == 13);

// ---- shape A: a const fn whose body is one associated-const read ----

pub const fn assoc_read<N: Nat>() -> usize {
    N::VAL
}

#[unsafe(no_mangle)]
pub fn shape_a_bound_check(i: usize) -> bool {
    i < assoc_read::<W13>()
}

// ---- shape B: a const fn whose body recurses ----
//
// The decoder's period, `8 / gcd(W, 8)`, exactly as file 81 first wrote it.

pub const fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub const fn period_fn<N: Nat>() -> usize {
    8 / gcd(N::VAL, 8)
}

#[unsafe(no_mangle)]
pub fn shape_b_period(_x: usize) -> usize {
    period_fn::<W13>()
}

// ---- shape C: the same quantity as an associated const ----

pub trait Packed {
    const W: usize;
    const PERIOD: usize;
    const GROUP_BYTES: usize;
    const MASK: u32;
}
impl<N: Nat> Packed for N {
    const W: usize = N::VAL;
    const PERIOD: usize = 8 / gcd(N::VAL, 8);
    const GROUP_BYTES: usize = N::VAL * (8 / gcd(N::VAL, 8)) / 8;
    const MASK: u32 = (1u32 << N::VAL) - 1;
}

#[unsafe(no_mangle)]
pub fn shape_c_period(_x: usize) -> usize {
    <W13 as Packed>::PERIOD
}

// ---- the shape that matters: the quantity used inside a loop ----
//
// Shape B and shape C differ only in where the period is written. Both loops do the same
// arithmetic. Everything else is identical, including the unroll opportunity.

#[unsafe(no_mangle)]
pub fn loop_from_const_fn(buf: &[u8], n: usize, out: &mut [u32]) {
    let p = period_fn::<W13>();
    let g = <W13 as Nat>::VAL * p / 8;
    let mut group = 0;
    let mut i = 0;
    while i + p <= n {
        let base = group * g;
        let mut j = 0;
        while j < p {
            let bit = j * <W13 as Nat>::VAL;
            let byte = base + bit / 8;
            let sh = bit % 8;
            let w = u32::from_le_bytes([buf[byte], buf[byte + 1], buf[byte + 2], buf[byte + 3]]);
            out[i + j] = (w >> sh) & ((1u32 << <W13 as Nat>::VAL) - 1);
            j += 1;
        }
        group += 1;
        i += p;
    }
}

#[unsafe(no_mangle)]
pub fn loop_from_assoc_const(buf: &[u8], n: usize, out: &mut [u32]) {
    const P: usize = <W13 as Packed>::PERIOD;
    const G: usize = <W13 as Packed>::GROUP_BYTES;
    const W: usize = <W13 as Packed>::W;
    const M: u32 = <W13 as Packed>::MASK;
    let mut group = 0;
    let mut i = 0;
    while i + P <= n {
        let base = group * G;
        let mut j = 0;
        while j < P {
            let bit = j * W;
            let byte = base + bit / 8;
            let sh = bit % 8;
            let w = u32::from_le_bytes([buf[byte], buf[byte + 1], buf[byte + 2], buf[byte + 3]]);
            out[i + j] = (w >> sh) & M;
            j += 1;
        }
        group += 1;
        i += P;
    }
}
