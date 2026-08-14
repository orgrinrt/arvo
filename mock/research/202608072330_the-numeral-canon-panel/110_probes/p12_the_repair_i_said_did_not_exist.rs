//! P12. Compiling the repair my F8 said did not exist.
//!
//! `110` F8 says two names for one primitive is "a wall rather than a slow path"
//! and that "there is no repair". `112` section 7 says the cost is not a
//! property of the split but of **where the two spellings meet**, and that at a
//! polymorphic site the repair is one threaded parameter, which it compiled.
//!
//! I am about to withdraw a claim of my own on the strength of that, so I compile
//! it myself rather than accept the report. Three sites, three outcomes, in one
//! file, on `nightly-2026-05-28` with no feature gate.
//!
//! Build: rustc --edition 2021 --crate-type lib p12_the_repair_i_said_did_not_exist.rs
//! Expected: compiles. The unrepairable case is in the companion, which must fail.

#![no_std]

/// The axis-carrying parameterisation from `p6_noncanonical_wall.rs`: a radix
/// parameter that `R` does not read at `F = 0`, so the two spellings denote one
/// primitive and are two types.
pub struct FxAxes<const RADIX: u32>(pub i128);

pub const BINARY: u32 = 2;
pub const DECIMAL: u32 = 10;

// ---------------------------------------------------------------------------
// Site one: a monomorphic call. Each spelling has its own call site and neither
// mentions the other, so a spurious parameter costs nothing at all.
// ---------------------------------------------------------------------------

pub fn double_binary(x: FxAxes<BINARY>) -> FxAxes<BINARY> {
    FxAxes(x.0 * 2)
}

pub fn double_decimal(x: FxAxes<DECIMAL>) -> FxAxes<DECIMAL> {
    FxAxes(x.0 * 2)
}

pub fn monomorphic_site() -> (i128, i128) {
    (
        double_binary(FxAxes::<BINARY>(21)).0,
        double_decimal(FxAxes::<DECIMAL>(21)).0,
    )
}

// ---------------------------------------------------------------------------
// Site two: a polymorphic signature. THIS IS THE REPAIR F8 SAID DID NOT EXIST.
//
// F8's claim was about making two type constructors applied to different
// arguments into ONE TYPE, which is still correct and still needs a feature the
// design forbids. But "one function over both" does not require one type: it
// requires abstracting over the parameter, which is ordinary Rust and needs no
// gate. The two remain distinct types and one body serves both.
// ---------------------------------------------------------------------------

pub fn double_any_radix<const R: u32>(x: FxAxes<R>) -> FxAxes<R> {
    FxAxes(x.0 * 2)
}

pub fn polymorphic_site() -> (i128, i128) {
    // one generic function, both spellings, no cast, no feature gate
    (
        double_any_radix(FxAxes::<BINARY>(21)).0,
        double_any_radix(FxAxes::<DECIMAL>(21)).0,
    )
}

/// And it composes: a caller that wants to be written once threads the parameter
/// on, which is the cost `112` names. It is real and it is bounded, and it is
/// viral in exactly the way a type parameter is viral.
pub fn sum_any_radix<const R: u32>(xs: &[FxAxes<R>]) -> i128 {
    let mut acc = 0i128;
    let mut i = 0;
    while i < xs.len() {
        acc += xs[i].0;
        i += 1;
    }
    acc
}

pub fn polymorphic_site_composed() -> (i128, i128) {
    let bin = [FxAxes::<BINARY>(1), FxAxes::<BINARY>(2)];
    let dec = [FxAxes::<DECIMAL>(10), FxAxes::<DECIMAL>(20)];
    (sum_any_radix(&bin), sum_any_radix(&dec))
}

// ---------------------------------------------------------------------------
// Site three is the homogeneous container, and it is in the companion file
// because it is expected to FAIL. A `[FxAxes<2>; 2]` has exactly one element
// type, so no signature, bound, blanket impl or const predicate lets two
// spellings share one array, one slice or one column. Parametric abstraction
// does not reach it because the abstraction is over the element type and there
// is only one.
//
// That is where I17's storage-minimising path lives, which is why the spurious
// parameter is not a tidiness question.
// ---------------------------------------------------------------------------

/// A column is homogeneous by construction. This compiles only because both
/// elements are the SAME spelling; the companion shows what happens otherwise.
pub fn storage_site_same_spelling() -> i128 {
    let column: [FxAxes<BINARY>; 2] = [FxAxes(1), FxAxes(2)];
    sum_any_radix(&column)
}
