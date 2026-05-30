//! arvo-spectral — spectral graph methods (L3).
//!
//! Laplacian construction, power iteration, Fiedler vector, spectral
//! bisection, and k-way partitioning over fixed-capacity dense matrices.
//! `#![no_std]`, no alloc, no platform dependency. Every size is const
//! at type level; every public function uses arvo's newtype surface
//! (`Capacity` as the type-level size, `USize` for counts and indices).
//!
//! The capacity is a TYPE (`C: Capacity`, arvo-tensor's `Capacity`), so
//! no `cap_size` expression sits in type position and the crate no
//! longer threads `generic_const_exprs` over capacity arithmetic.
//! Vectors are the associated array `C::Array<F>`; a body that needs
//! the count as a value reads `cap_size(C::CAP)`.

#![no_std]

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
