//! arvo-bitmask. L2 bit storage.
//!
//! Generic fixed-width bitmask chassis (`Mask<W>`) and bit-matrix
//! adjacency chassis (`BitMatrix<W, C: Capacity>`) built on
//! arvo-bits bit-level contracts. `NodeId` newtypes node indices at
//! the type level. `propagate_dirty` OR-propagates a change flag
//! through DAG adjacency.
//!
//! Round 202605031748 (#313) deleted the prior parallel `Mask256`
//! struct and the `Mask64` / `Mask256` / `BitMatrix64` /
//! `BitMatrix256` aliases. Per the workspace's
//! Strategy/Sign-discoverability discipline, the chassis form is
//! the only spelling: consumers name `Mask<Bits<64, Hot, Unsigned>>`
//! and `BitMatrix<Bits<64, Hot, Unsigned>, C>` directly.
//!
//! `#![no_std]`, no alloc, const-generic sizing. L2 of the arvo
//! stack; depends on arvo and arvo-bits.

#![no_std]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![allow(incomplete_features)]
// No `generic_const_exprs` / `adt_const_params` gates: `BitMatrix`'s row count
// is now a `Capacity`
// type (`C`, backing `C::Array<Mask<W>>`), so no `cap_size` expression sits in
// type position. `cap_size` survives only as a value-position const fn over
// `C::CAP`. The GCE surface the `const N: Cap` form needed is gone (the
// gate-drop bonus of the capacity-as-type migration; obviates #628 here).

pub mod dirty;
pub mod mask;
pub mod matrix;
pub mod node;
pub mod ops;

pub use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence};
pub use dirty::propagate_dirty;
pub use mask::Mask;
pub use matrix::{BitMatrix, cap_size};
pub use node::NodeId;
pub use ops::SetBitsIter;
