//! P05. Instrument three, a different position: pass a value computed from a
//! generic const param as a const ARGUMENT to another type, in a where-clause
//! and in a field type. This is the position the recursive-bridge idea needs.
#![no_std]
#![crate_type = "lib"]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

pub struct Idx<const N: u32>;
pub trait ToNat {
    type N;
}

// A recursive bridge: halve the const and recurse. If this were accepted the
// per-width bridge dissolves entirely.
impl<const N: u32> ToNat for Idx<N>
where
    Idx<const { N / 2 }>: ToNat,
{
    type N = ();
}
