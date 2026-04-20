//! arvo-tensor — L2 fixed-shape tensor wrappers.
//!
//! Rank-N containers with type-level capacity. `Array<T, N>` is rank-1,
//! `Matrix<W, N>` is rank-2 N×N. Both wrap stack-allocated arrays under
//! `#[repr(transparent)]`, exposing typed `get` / `set` / `from_fn`
//! methods that hide the raw-usize indexing inside their bodies.
//!
//! `cap_size(c: Cap) -> usize` is the canonical `Cap` → `usize`
//! projection required by nightly `generic_const_exprs` in array-length
//! position (language grammar constraint; tracked #121). Consumers reach
//! it through either `arvo_tensor::cap_size` directly or via the
//! re-exports in arvo-bitmask / arvo-spectral.
//!
//! The `Enumerator` trait supplies `.enumerated()` yielding `(USize, T)`
//! pairs — the typed-index parallel of `core::iter::Iterator::enumerate`
//! whose raw `usize` leaks the substrate boundary.
//!
//! `#![no_std]`, no alloc, no platform dep. L2 of the arvo stack; depends
//! on arvo L0 only.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub mod array;
pub mod cap;
pub mod enumerator;
pub mod matrix;

pub use array::Array;
pub use cap::cap_size;
pub use enumerator::Enumerator;
pub use matrix::Matrix;
