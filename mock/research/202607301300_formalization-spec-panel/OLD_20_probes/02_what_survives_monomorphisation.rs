//! PROBE 2: what a build layer can actually read out of a compiled arvo.
//!
//! File 16 section 8 argued type information is erased by monomorphisation and
//! file 17 section 8 concluded from it that a downstream target reads "nothing"
//! out of the types. That is true of the *type system*: there is no `TypeId`,
//! no reflection, no metadata section rustc emits for generic arguments.
//!
//! It is not true of the *object file*. Monomorphisation does not erase the
//! type; it PRINTS the type, into the symbol name, once per instantiation.
//! Under the v0 mangling scheme the encoding is documented, reversible, and
//! includes const-generic arguments by value.
//!
//! So this probe asks the question the erasure argument skipped: after
//! monomorphisation, for each distinct composition that was instantiated, what
//! does the linker see?
//!
//! Build: see 02_run.sh

#![crate_type = "lib"]
#![no_std]

// Marker types standing in for the spec's `Policy`/`Lowering` axes
// (`11_current_shape_draft.md:151-183`).
pub struct Strict;
pub struct Relaxed;
pub struct Dense;
pub struct Bitpacked;

pub trait Fidelity {
    const REASSOC: bool;
}
impl Fidelity for Strict {
    const REASSOC: bool = false;
}
impl Fidelity for Relaxed {
    const REASSOC: bool = true;
}

/// Stand-in for `Number<N, S>`: two const-generic widths and two marker types.
pub struct Number<const I: u16, const F: u16, S, L>(core::marker::PhantomData<(S, L)>);

/// The operation whose lowering the intent is about.
#[inline(never)]
pub fn sum4<const I: u16, const F: u16, S: Fidelity, L>(
    _n: &Number<I, F, S, L>,
    xs: [f64; 4],
) -> f64 {
    if S::REASSOC {
        (xs[0] + xs[2]) + (xs[1] + xs[3])
    } else {
        ((xs[0] + xs[1]) + xs[2]) + xs[3]
    }
}

// Four distinct compositions, exported so the instantiations survive to the
// object file without an LTO'd binary around them.
type A = Number<7, 9, Strict, Dense>;
type B = Number<7, 9, Relaxed, Dense>;
type C = Number<23, 41, Strict, Bitpacked>;
type D = Number<3, 5, Relaxed, Bitpacked>;

#[no_mangle]
pub fn call_a(n: &A, xs: [f64; 4]) -> f64 {
    sum4(n, xs)
}
#[no_mangle]
pub fn call_b(n: &B, xs: [f64; 4]) -> f64 {
    sum4(n, xs)
}
#[no_mangle]
pub fn call_c(n: &C, xs: [f64; 4]) -> f64 {
    sum4(n, xs)
}
#[no_mangle]
pub fn call_d(n: &D, xs: [f64; 4]) -> f64 {
    sum4(n, xs)
}
