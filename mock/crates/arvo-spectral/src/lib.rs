//! arvo-spectral — spectral graph methods (L3).
//!
//! Laplacian construction, power iteration, Fiedler vector, spectral
//! bisection, and k-way partitioning over const-generic dense matrices.
//! `#![no_std]`, no alloc, no platform dependency. Every size is const
//! at type level; every public function uses arvo's newtype surface
//! (`Cap` as const-generic size, `USize` for counts and indices).

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub mod fiedler;
pub mod laplacian;
pub mod matrix;
pub mod power;

pub use fiedler::fiedler_vector;
pub use laplacian::laplacian;
pub use matrix::Matrix;
pub use power::power_iteration;
