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
// No `generic_const_exprs` / `adt_const_params` gates: the capacity is a TYPE
// (`N: Capacity`, value storage through `arvo_tensor::Array` / `Matrix` whose
// backing is `Capacity::Array`), so no `cap_size` expression sits in type
// position and no `Cap` const generic appears. The GCE surface the algorithms
// needed under the `const N: Cap` form is gone (the gate-drop bonus of the
// capacity-as-type migration; obviates the #628 GCE-to-GCA migration here).

pub mod binpack;
pub mod dp;
pub mod greedy;
pub mod range;

pub use binpack::bin_pack;
pub use dp::matrix_chain_dp;
pub use greedy::greedy_group;
pub use range::Range;
