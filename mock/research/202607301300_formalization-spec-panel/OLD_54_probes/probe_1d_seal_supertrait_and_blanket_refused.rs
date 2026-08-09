//! Probe 1d (negative control): routes two and four, the supertrait itself and a downstream
//! blanket.
//!
//! rustc --edition 2021 --crate-type lib --extern vu54=libvu54.rlib probe_1d_seal_supertrait_and_blanket_refused.rs
//!
//! Route two: name the private supertrait to satisfy it directly. Expected E0603, the
//! module is private, so the trait has no path a downstream can write.
//!
//! Route four: a blanket impl over a type parameter, which is how a downstream launders a
//! foreign trait onto types it does not own. Expected E0210, uncovered type parameter.
//!
//! Route three (re-impl on an existing inhabitant) is refused by the orphan rule before any
//! seal is consulted and is not repeated here; file 46 established it for the tower's own
//! carriers and the argument is identical for these four, since it is a property of
//! coherence rather than of the trait.

#![allow(dead_code)]

use vu54::numeral::Specials;

// route two: the supertrait is unnameable.
pub struct A;
impl vu54::numeral::specials_sealed::SpecialsSealed for A {}

// route four: a downstream blanket over an uncovered parameter.
pub trait MyMarker {}
impl<T: MyMarker> Specials for T {
    const INF: bool = true;
    const NAN: bool = true;
    const NAN_DATA_MIN: u32 = 0;
}
