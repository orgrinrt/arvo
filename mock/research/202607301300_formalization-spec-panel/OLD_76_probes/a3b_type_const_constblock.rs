//! A3b. As A3, with rustc's suggested const block on the inductive step.
//! A3. Escape 2 taken all the way down, following rustc's own suggestion.
//!
//! A2's `type const` attempt failed because the projection it reads
//! (`Pos::VAL`) is an ordinary associated const. rustc's suggestion is to make
//! that one `type const` as well, so this file makes the whole chain
//! `type const`, which is the honest form of the successor feature's answer.
//!
//! The decisive question is whether a `type const` right-hand side may use a
//! generic parameter, because the inductive encoding's whole content is
//! `2 * P::VAL`. A recorded 2026-05-29 sketch finding says it may not, for a
//! different shape; this checks the shape the unification actually needs.
#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

use core::marker::PhantomData;

pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct Z;
pub struct Pz<P>(PhantomData<P>);

pub trait Pos {
    type const VAL: usize;
}
impl Pos for H {
    type const VAL: usize = 1;
}
impl<P: Pos> Pos for O<P> {
    type const VAL: usize = { 2 * P::VAL };
}

pub trait Nat {
    type const VAL: usize;
}
impl Nat for Z {
    type const VAL: usize = 0;
}
impl<P: Pos> Nat for Pz<P> {
    type const VAL: usize = P::VAL;
}

pub trait Capacity {
    type Array<T>;
}
impl<N: Nat> Capacity for N {
    type Array<T> = [T; <N as Nat>::VAL];
}

pub type C8 = Pz<O<O<O<H>>>>;
pub fn take(_: <C8 as Capacity>::Array<u32>) {}
