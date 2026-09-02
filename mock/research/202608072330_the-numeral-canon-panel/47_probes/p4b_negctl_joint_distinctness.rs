// p4b: the negative control for p4 arm B. EXPECTED TO FAIL TO COMPILE; the .err is the result.
//
// p4 arm A shows a componentwise assertion over the flat pair is satisfied by a derivation that
// has already collapsed. This file shows the joint assertion, once a single subject exists to
// be its subject, is NOT satisfied by the collapse: three distinct strides give three distinct
// reified results, and claiming otherwise is refused.
//
// Without this file, p4's `assert_same::<Pair13, Pair<u16, 13>>()` would be the "asserting a
// value against itself" shape and would prove nothing.
//
//   rustc +nightly-2026-05-28 --edition 2021 --crate-type lib p4b_negctl_joint_distinctness.rs
//
// No #![feature] gate.

#![no_std]

pub trait SameType<T: ?Sized> {}
impl<T: ?Sized> SameType<T> for T {}
pub const fn assert_same<A: SameType<B> + ?Sized, B: ?Sized>() {}

pub struct Pair<C: Copy, const STRIDE: u32>(core::marker::PhantomData<C>);

pub type Pair9 = Pair<u16, 9>;
pub type Pair13 = Pair<u16, 13>;
pub type Pair16 = Pair<u16, 16>;

// Two of the eight Cold widths that share a carrier. Refused.
const _: () = assert_same::<Pair13, Pair16>();

// The extremes of the same fibre. Refused.
const _: () = assert_same::<Pair9, Pair16>();

// And the carrier-only view of the same two, which is what a one-output derivation returning a
// machine type hands downstream. NOT refused, and that is the point: the same collapse that
// the joint assertion catches, the carrier-only view certifies as fine.
const _: () = assert_same::<u16, u16>();
