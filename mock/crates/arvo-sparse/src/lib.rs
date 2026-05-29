//! arvo-sparse — L2 sparse matrix substrate.
//!
//! Fixed-size sparse matrix storage (`Csr`) and structural-analysis
//! algorithms over `BitMatrix<Bits<64, Hot, Unsigned>>` adjacency: reverse Cuthill-McKee
//! reordering (`rcm_reorder`), block-diagonal detection
//! (`block_diagonal`), and Dulmage-Mendelsohn decomposition
//! (`dulmage_mendelsohn`).
//!
//! `#![no_std]`, no alloc, const-generic sizing. L2 of the arvo
//! stack; depends on arvo and arvo-bitmask.

#![no_std]
#![feature(adt_const_params)]
// WATCH-tier unstable feature, soundness-vetted in the stack sweep (task #626).
// `generic_const_exprs` is used here only for const-expression bounds and const-
// generic array lengths (`[(); cap_size(N)]:` array sizing, `[(); EXPR]:` compile-
// time assertions, width arithmetic in const-generic position). Its one known
// unsoundness (#97156, const `TypeId` resolved into types with higher-ranked-trait-
// bound subtyping) is unreachable: the stack bans `TypeId`. Builds clean on the
// pinned nightly. Migration to `generic_const_args` is tracked: #628.
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

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
