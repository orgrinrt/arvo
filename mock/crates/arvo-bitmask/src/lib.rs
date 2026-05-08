//! arvo-bitmask. L2 bit storage.
//!
//! Generic fixed-width bitmask chassis (`Mask<W>`) and bit-matrix
//! adjacency chassis (`BitMatrix<W, const N: Cap>`) built on
//! arvo-bits bit-level contracts. `NodeId` newtypes node indices at
//! the type level. `propagate_dirty` OR-propagates a change flag
//! through DAG adjacency.
//!
//! Round 202605031748 (#313) deleted the prior parallel `Mask256`
//! struct and the `Mask64` / `Mask256` / `BitMatrix64` /
//! `BitMatrix256` aliases. Per the workspace's
//! Strategy/Sign-discoverability discipline, the chassis form is
//! the only spelling: consumers name `Mask<Bits<64, Hot, Unsigned>>`
//! and `BitMatrix<Bits<64, Hot, Unsigned>, N>` directly.
//!
//! `#![no_std]`, no alloc, const-generic sizing. L2 of the arvo
//! stack; depends on arvo and arvo-bits.

#![no_std]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub mod dirty;
pub mod mask;
pub mod matrix;
pub mod node;
pub mod ops;

pub use dirty::propagate_dirty;
pub use mask::Mask;
pub use matrix::{BitMatrix, cap_size};
pub use node::NodeId;
pub use ops::SetBitsIter;
