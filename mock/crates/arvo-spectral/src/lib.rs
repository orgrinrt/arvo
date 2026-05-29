//! arvo-spectral — spectral graph methods (L3).
//!
//! Laplacian construction, power iteration, Fiedler vector, spectral
//! bisection, and k-way partitioning over const-generic dense matrices.
//! `#![no_std]`, no alloc, no platform dependency. Every size is const
//! at type level; every public function uses arvo's newtype surface
//! (`Cap` as const-generic size, `USize` for counts and indices).

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

pub mod fiedler;
pub mod laplacian;
pub mod matrix;
pub mod operator;
pub mod partition;
pub mod power;

pub use fiedler::{dense_laplacian_lambda_max_bound, fiedler_vector};
pub use laplacian::laplacian;
pub use matrix::Matrix;
pub use operator::{LinearOperator, SparseLaplacian};
pub use partition::{SpectralBipartitioner, k_way_partition, spectral_bisection};
pub use power::power_iteration;
