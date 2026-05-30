//! arvo-sparse — L2 sparse matrix substrate.
//!
//! Fixed-capacity sparse matrix storage (`Csr`) and structural-analysis
//! algorithms over `BitMatrix<Bits<64, Hot, Unsigned>>` adjacency: reverse Cuthill-McKee
//! reordering (`rcm_reorder`), block-diagonal detection
//! (`block_diagonal`), and Dulmage-Mendelsohn decomposition
//! (`dulmage_mendelsohn`).
//!
//! The capacity is a TYPE (`C: Capacity` / `R: Capacity` / `NNZ: Capacity`,
//! arvo-tensor's `Capacity`), so no `cap_size` expression sits in type
//! position and the crate no longer threads `generic_const_exprs` over
//! capacity arithmetic. Storage is the associated array `C::Array<X>`;
//! a body that needs the count as a value reads `cap_size(C::CAP)`.
//!
//! `#![no_std]`, no alloc, const-capacity sizing. L2 of the arvo
//! stack; depends on arvo, arvo-bitmask, and arvo-tensor.

#![no_std]
#![feature(const_trait_impl)]

pub mod adjacency;
pub mod block;
pub mod csr;
pub mod dm;
pub mod rcm;

pub use adjacency::{
    BandwidthReducer, BidirectionalSparseAdjacency, BipartiteStructuralAnalysis,
    BlockPartitioner, SparseAdjacency,
};
pub use block::{block_diagonal, block_diagonal_via};
pub use csr::{Csr, CsrBidirectional};
pub use dm::{
    DulmageMendelsohn, classification_to_mask, dulmage_mendelsohn, dulmage_mendelsohn_via,
};
pub use rcm::{rcm_reorder, rcm_reorder_via};
