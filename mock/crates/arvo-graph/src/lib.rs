//! arvo-graph — L2 graph algorithms.
//!
//! DAG analysis over `BitMatrix<Bits<64, Hot, Unsigned>>` adjacency: topological sort, rank
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
// WATCH-tier unstable feature, soundness-vetted in the stack sweep (task #626).
// `generic_const_exprs` is used here only for const-expression bounds and const-
// generic array lengths (`[(); cap_size(N)]:` array sizing, `[(); EXPR]:` compile-
// time assertions, width arithmetic in const-generic position). Its one known
// unsoundness (#97156, const `TypeId` resolved into types with higher-ranked-trait-
// bound subtyping) is unreachable: the stack bans `TypeId`. Builds clean on the
// pinned nightly. Migration to `generic_const_args` is tracked: #628.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]


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
