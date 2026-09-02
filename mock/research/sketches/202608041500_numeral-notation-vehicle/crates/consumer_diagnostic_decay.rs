//! FAILS test, on purpose: the DECAY half of file 56 section 3.2's finding
//! (Shape 1, an operation generic over the raw encoding rather than the
//! face), reproduced against types this macro emits rather than a
//! hand-written stand-in. `needs_encoding` is bound on `Bias` (the raw,
//! open trait), not on `NumeralFace`, so both `N37::Encoding` and
//! `N38::Encoding` decay to their fully-expanded `BPos<Pz<...>, H>` nest
//! the moment they cross this boundary, exactly as file 56 predicted for
//! ANY operation defined at the raw-encoding layer, regardless of how
//! legible the declaration site was.

#[path = "tower.rs"]
mod tower;
use tower::*;

extern crate numeral_pm;
use numeral_pm::numeral_face;

numeral_face!(N37 = 37);
numeral_face!(N38 = 38);

// generic over the RAW encoding (Bias), not over the face: this is
// exactly Shape 1 from file 56 section 3.2, "the operation generic over
// the raw Nat, the face only a call-boundary label."
fn needs_encoding<B: Bias>(_: core::marker::PhantomData<B>) {}

fn main() {
    needs_encoding::<<N37 as NumeralFace>::Encoding>(core::marker::PhantomData);
    needs_encoding::<<N38 as NumeralFace>::Encoding>(
        core::marker::PhantomData::<<N37 as NumeralFace>::Encoding>,
    );
}
