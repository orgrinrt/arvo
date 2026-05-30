//! arvo-graph — L2 graph algorithms.
//!
//! DAG analysis over `BitMatrix<Bits<64, Hot, Unsigned>, C>` adjacency:
//! topological sort, rank computation, connected components, longest-path
//! DP, waist detection, and spanning-tree decomposition. Every algorithm
//! is stack-only and generic over the node-count capacity `C: Capacity`.
//!
//! Weight types enter through trait bounds from arvo: `TotalOrd` for
//! max-selection, `FromConstant` for zero init, `core::ops::Add` for
//! accumulation. The crate does not import `UFixed` / `IFixed`
//! directly; consumers pick a concrete numeric type.
//!
//! The node-count capacity is a `Capacity` TYPE (`Dim<N>`), not a `Cap`
//! const generic, so no `cap_size` expression sits in type position and
//! `generic_const_exprs` is not needed. Working arrays are the capacity's
//! backing array `C::Array<T>`; a body reads the count as `cap_size(C::CAP)`.
//!
//! `#![no_std]`, no alloc. Depends on arvo, arvo-bitmask, and arvo-tensor.

#![no_std]


pub mod components;
pub mod path;
pub mod rank;
pub mod spanning;
pub mod topo;
pub mod waist;

pub use components::components;
pub use path::longest_path;
pub use rank::{downward_rank, upward_rank};
pub use spanning::{SpanningTree, spanning_tree};
pub use topo::{renumber, topo_sort};
pub use waist::waist_detect;
