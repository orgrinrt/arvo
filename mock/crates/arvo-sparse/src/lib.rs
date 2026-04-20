//! arvo-sparse — L2 sparse matrix substrate.
//!
//! Fixed-size sparse matrix storage (`Csr`) and structural-analysis
//! algorithms over `BitMatrix64` adjacency: reverse Cuthill-McKee
//! reordering (`rcm_reorder`), block-diagonal detection
//! (`block_diagonal`), and Dulmage-Mendelsohn decomposition
//! (`dulmage_mendelsohn`).
//!
//! `#![no_std]`, no alloc, const-generic sizing. L2 of the arvo
//! stack; depends on arvo and arvo-bitmask.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub mod block;
pub mod csr;
pub mod dm;
pub mod rcm;

pub use block::block_diagonal;
pub use csr::Csr;
pub use dm::{DulmageMendelsohn, dulmage_mendelsohn};
pub use rcm::rcm_reorder;
