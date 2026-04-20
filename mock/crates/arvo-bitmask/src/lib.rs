//! arvo-bitmask — L2 bit storage.
//!
//! Fixed-width bitmask types (`Mask64`, `Mask256`) and bit-matrix
//! adjacency structures (`BitMatrix64`, `BitMatrix256`) built on
//! arvo-bits bit-level contracts. `NodeId` newtypes node indices at
//! the type level. `propagate_dirty` OR-propagates a change flag
//! through DAG adjacency.
//!
//! `#![no_std]`, no alloc, const-generic sizing. L2 of the arvo
//! stack; depends on arvo and arvo-bits.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub mod dirty;
pub mod mask;
pub mod matrix;
pub mod node;
pub mod ops;

pub use dirty::{propagate_dirty_256, propagate_dirty_64};
pub use mask::{Mask, Mask256, Mask64};
pub use matrix::{BitMatrix256, BitMatrix64, cap_size};
pub use node::NodeId;
pub use ops::{SetBitsIter256, SetBitsIter64};
