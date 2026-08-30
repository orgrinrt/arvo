//! Probe 3. The scissors, stated on one grammar so both blades are visible in
//! one file.
//!
//! Probe 2 claim B established that the inductive numeral's value IS readable,
//! gate-free, as an ordinary associated const whose expression mentions a
//! generic parameter. This probe takes that same const and puts it in array
//! length position. Nothing else changes.
//!
//! Claim: the identical `VAL`, identical impls, identical grammar, refuses in
//! type position and only in type position.
//!
//! This matters because the ratified sentence (`91:796-802`, via `79` section
//! 4) says the array grammar's pairing is forced. `106` section 6.2 sharpened
//! that to "forced given the choice of an inductive numeral". This probe
//! sharpens it once further, and the extra turn is what probe 4 exploits:
//! **the pairing is forced only if the array is obtained BY ITS LENGTH.**
//! Nothing in the design requires that. It is the one construction everybody
//! reached for because `[T; N]` is how Rust spells a fixed-size buffer.
//!
//! MUST BE COMPILED FROM INSIDE THE TREE (pinned nightly-2026-05-28).
#![no_std]

use core::marker::PhantomData;

pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);

pub trait Pos {
    const VAL: usize;
}
impl Pos for H {
    const VAL: usize = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: usize = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: usize = 2 * P::VAL + 1;
}

pub type N13 = I<O<I<H>>>;

// Blade one: VALUE position. Compiles. The generic parameter is inside the
// expression, the recursion is three levels deep, and this is stable Rust.
pub const THIRTEEN: usize = <N13 as Pos>::VAL;
const _: () = assert!(THIRTEEN == 13);

// A const fn over the same value, also fine, also gate-free. Consumers can
// have the number at compile time; that was never in doubt and no file has
// said otherwise in these words.
pub const fn count<C: Pos>() -> usize {
    C::VAL
}
const _: () = assert!(count::<N13>() == 13);

// Blade two: TYPE position. The same const, same impls, same grammar.
pub trait Capacity: Pos {
    type Array<T: Copy>: Copy;
}
impl<C: Pos> Capacity for C {
    type Array<T: Copy> = [T; <C as Pos>::VAL];
}
