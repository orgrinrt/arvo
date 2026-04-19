//! arvo-graph — L2 graph algorithms.
//!
//! DAG analysis over `BitMatrix64` adjacency: topological sort, rank
//! computation, connected components, longest-path DP, waist detection,
//! and spanning-tree decomposition. Every algorithm is stack-only and
//! const-generic on node count `N` (up to 64).
//!
//! Weight types enter through trait bounds from arvo: `TotalOrd` for
//! max-selection, `FromConstant` for zero init, `core::ops::Add` for
//! accumulation. The crate does not import `UFixed` / `IFixed`
//! directly; consumers pick a concrete numeric type.
//!
//! `#![no_std]`, no alloc. Depends on arvo and arvo-bitmask only.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub mod topo;

pub use topo::{renumber, topo_sort};
