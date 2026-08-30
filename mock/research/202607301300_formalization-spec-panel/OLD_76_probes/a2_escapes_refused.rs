//! A2. The two escapes that look like they should rescue A1, both refused.
//!
//! A1 fails with rustc naming `generic_const_exprs`, which the ratified
//! `unstable-features.md` forbids outright (op, 2026-07-28). Two candidate
//! escapes exist inside the permitted set, and both are checked here rather
//! than argued, because "the sanctioned successor probably covers it" is
//! exactly the shape of claim this review has twice found false.
//!
//! Escape 1: the const-block form `[T; { <N as Nat>::VAL }]`, which
//! `min_generic_const_args` diagnostics suggest.
//! Escape 2: a `min_generic_const_args` `type const` associated item, the
//! successor's own mechanism for naming a computed const in type position.
#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

use core::marker::PhantomData;

pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct Z;
pub struct Pz<P>(PhantomData<P>);

pub trait Pos {
    const VAL: usize;
}
impl Pos for H {
    const VAL: usize = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: usize = 2 * P::VAL;
}

pub trait Nat {
    const VAL: usize;
}
impl Nat for Z {
    const VAL: usize = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: usize = P::VAL;
}

// Escape 1: wrap the projection in a const block.
pub trait CapacityBlock {
    type Array<T>;
}
impl<N: Nat> CapacityBlock for N {
    type Array<T> = [T; { <N as Nat>::VAL }];
}

// Escape 2: the successor's own `type const` associated item.
pub trait NatTc {
    type const LEN: usize;
}
impl NatTc for Z {
    type const LEN: usize = 0;
}
impl<P: Pos> NatTc for Pz<P> {
    type const LEN: usize = P::VAL;
}

pub trait CapacityTc {
    type Array<T>;
}
impl<N: NatTc> CapacityTc for N {
    type Array<T> = [T; <N as NatTc>::LEN];
}
