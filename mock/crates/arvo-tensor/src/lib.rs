//! arvo-tensor: L2 fixed-shape tensor wrappers.
//!
//! Rank-N containers with type-level capacity. `Array<T, N>` is rank-1,
//! `Matrix<W, N>` is rank-2 N×N. Both wrap stack-allocated arrays under
//! `#[repr(transparent)]`, exposing typed `get` / `set` / `from_fn`
//! methods that hide the raw-usize indexing inside their bodies.
//!
//! `cap_size(c: Cap) -> usize` is the canonical `Cap` → `usize` projection.
//! The containers read it in value position to recover a capacity's count
//! (`cap_size(C::CAP)`); it no longer appears in type position here. The `cap`
//! inverse builds a `Cap` from a `usize` for the `Dim<N>` associated const.
//!
//! The `Enumerator` trait supplies `.enumerated()` yielding `(USize, T)`
//! pairs, the typed-index parallel of `core::iter::Iterator::enumerate`
//! whose raw `usize` leaks the substrate boundary.
//!
//! `#![no_std]`, no alloc, no platform dep. L2 of the arvo stack; depends
//! on arvo L0 only.

#![no_std]
// No `generic_const_exprs` / `adt_const_params` gates: the capacity is a TYPE
// (`C: Capacity`, backing array `C::Array<T>`), so no `cap_size` expression sits
// in type position and no `Cap` const generic appears. The GCE surface the
// `Array<T, const N: Cap>` form needed is gone (the gate-drop bonus of the
// capacity-as-type migration; obviates the #628 GCE-to-GCA migration for this
// crate). `cap_size` survives only as a value-position const fn over `C::CAP`.

pub mod array;
pub mod cap;
pub mod capacity;
pub mod enumerator;
pub mod matrix;

pub use array::Array;
pub use cap::{cap, cap_size};
pub use capacity::{Capacity, Dim};
pub use enumerator::Enumerator;
pub use matrix::Matrix;
