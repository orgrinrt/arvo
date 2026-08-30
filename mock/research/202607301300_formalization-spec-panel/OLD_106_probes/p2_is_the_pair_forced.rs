//! Probe 2. `79:148-172` and `91:796-802` state the capacity pairing is
//! "forced by the language, not chosen". Probe 1 confirmed the naive form needs
//! `generic_const_exprs`, which is FORBIDDEN. This probe checks the other half
//! of that claim against a feature the workspace PERMITS.
//!
//! `unstable-features.md` allowed table: `min_generic_const_args` (#132980),
//! "the deliberate sound, minimally-scoped successor to `generic_const_exprs`".
//! Its stated purpose is to permit a PATH in const-argument position, as
//! distinct from an expression. `<N as Nat>::VAL` is a path.
//!
//! `79:153-157` says the successor "cannot express" the inductive step either.
//! That is a claim about a feature's behaviour, so it is exactly the kind of
//! claim to re-check on the current pin rather than inherit.
//!
//! MUST BE COMPILED FROM INSIDE THE TREE. `rust-toolchain.toml` resolves to
//! nightly-2026-05-28; outside the tree rustup resolves to stable 1.94.0, which
//! does not parse `type const` at all and reports a misleading parse error.
//!
//! Claims:
//!   A. `min_generic_const_args` rejects a plain associated const in array
//!      position and names the successor form.
//!   B. what the successor form does when written out.
#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

use core::marker::PhantomData;

pub struct Z;
pub struct S<P>(PhantomData<P>);

pub trait Nat {
    // CLAIM B. `type const`, the form rustc's own diagnostic on claim A asks
    // for. Uncomment the plain form and comment this one to reproduce claim A.
    type const VAL: usize;
    // const VAL: usize;   // claim A: rejected, "not defined as `type const`"
}
impl Nat for Z {
    type const VAL: usize = 0;
}
impl<P: Nat> Nat for S<P> {
    type const VAL: usize = P::VAL + 1;
}

pub trait Capacity: Nat {
    type Array<T: Copy>: Copy;
}

pub struct Slot<N>(PhantomData<N>);
impl<N: Nat> Nat for Slot<N> {
    type const VAL: usize = N::VAL;
}
impl<N: Nat> Capacity for Slot<N> {
    // The one line under test. No `const K`, no pair, no agreement to check.
    type Array<T: Copy> = [T; <N as Nat>::VAL];
}

pub type N3 = S<S<S<Z>>>;

const LEN: usize = core::mem::size_of::<<Slot<N3> as Capacity>::Array<u8>>();
const VALX: usize = <Slot<N3> as Nat>::VAL;
const _: () = assert!(LEN == VALX);
