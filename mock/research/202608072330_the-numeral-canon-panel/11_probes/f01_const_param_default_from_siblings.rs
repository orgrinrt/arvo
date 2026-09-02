//! f01. Can a const parameter's DEFAULT be computed from its sibling const
//! parameters?
//!
//! If yes, route 15's arity vanishes: the byte count is derived by the default,
//! the consumer writes `Fixed<13, 3>` at arity two, there is no bridge and no
//! table, and the algebra is closed because no output width can be unshipped.
//! That would be a genuine fourteenth route and would satisfy every part of the
//! ratified gate at once.
//!
//! rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib --emit=metadata \
//!       -o out/f01.meta f01_const_param_default_from_siblings.rs
#![no_std]
#![crate_type = "lib"]

pub struct Hot;

pub const fn bytes(i: usize, f: usize) -> usize {
    (i + f + 7) / 8
}

#[repr(transparent)]
pub struct Fixed<const I: usize, const F: usize, S, const B: usize = { bytes(I, F) }> {
    raw: [u8; B],
    _m: core::marker::PhantomData<S>,
}

pub type A = Fixed<13, 3, Hot>;
const _: () = assert!(core::mem::size_of::<A>() == 2);
