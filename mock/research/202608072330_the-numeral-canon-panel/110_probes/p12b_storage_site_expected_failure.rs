//! P12b. The one site with no repair, compiled as an expected failure.
//!
//! Site three from P12. A homogeneous container has exactly one element type, so
//! two spellings of one primitive cannot share an array, a slice or a column, and
//! no amount of parametric abstraction reaches it: the abstraction is over the
//! element type and there is only one.
//!
//! This is where `110` F8's "no repair" is TRUE, and it is the only place it is.
//! It is also, per `112` section 7, exactly the storage boundary I17 protects.
//!
//! Build: rustc --edition 2021 --crate-type lib p12b_storage_site_expected_failure.rs
//! Expected: E0308.

#![no_std]

pub struct FxAxes<const RADIX: u32>(pub i128);

pub fn the_wall() {
    let _column: [FxAxes<2>; 2] = [FxAxes::<2>(1), FxAxes::<10>(2)];
}
