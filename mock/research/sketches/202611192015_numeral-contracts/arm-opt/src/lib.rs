//! Arm two: the contracts are const traits behind the `const` feature, and the
//! same traits plain without it, by two module paths, the shape
//! `notko-hlist`'s `Cardinal` and `arvo-numeral`'s count take.
//!
//! The question this arm asks is the unification case: one crate in a build
//! turns `const` on, and a consumer that bounds plainly and names no feature
//! still builds.

#![no_std]
#![cfg_attr(feature = "const", feature(const_trait_impl))]

#[cfg(feature = "const")]
mod const_path;
#[cfg(not(feature = "const"))]
mod plain_path;

#[cfg(feature = "const")]
pub use const_path::{Order, Sum, Zero};
#[cfg(not(feature = "const"))]
pub use plain_path::{Order, Sum, Zero};
