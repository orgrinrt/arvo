//! arvo-comb — L2 combinatorial algorithms.
//!
//! Sequential interval grouping, matrix-chain DP, and first-fit
//! bin packing over const-generic fixed-size inputs. Every algorithm
//! is stack-only and returns a fixed-size output plus a count.
//!
//! Numeric types enter through trait bounds from arvo: `Add` for
//! accumulation, `TotalOrd` for min/max selection, `Copy` for
//! stack work, `FromConstant` for zero init. The crate does not
//! import `UFixed` / `IFixed` directly; consumers pick their
//! concrete numeric type.
//!
//! `#![no_std]`, no alloc. Depends on arvo and arvo-bitmask only.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub mod dp;
pub mod greedy;
pub mod range;

pub use dp::matrix_chain_dp;
pub use greedy::greedy_group;
pub use range::Range;
