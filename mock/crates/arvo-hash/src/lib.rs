//! arvo-hash — L2 hash-domain types and algorithm contracts.
//!
//! `ContentHash` aliases `arvo::Bits<64, Hot>` for call-site domain
//! naming. Two trait surfaces:
//!
//! - Streaming `Hasher<const N: u16>` for incremental hashing.
//! - One-shot, const-callable `ConstHash<const N: u16, S, Sign>` for
//!   compile-time content addressing.
//!
//! Algorithms ship as `Fnv1a<const N: u16>` and `XxHash3<const N: u16>`.
//! Per round 4 (#314), each implements `Hasher<N>` once and
//! `ConstHash<N, Hot, Unsigned>` once via bounded-generic impls;
//! the prior 64-impl macro paste pattern is gone.
//!
//! `#![no_std]`, no alloc. Depends on `arvo` (for `Bits` / `Hot` /
//! strategy bounds) and `arvo-bits-contracts` (for `NarrowFromU64`).

#![no_std]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub mod algo;
pub mod aliases;
pub mod fnv1a;
pub mod xxhash3;

pub use algo::{ConstHash, Hasher, fnv1a_64};
pub use aliases::ContentHash;
pub use fnv1a::Fnv1a;
pub use xxhash3::{XxHash3, xxhash3_64};

