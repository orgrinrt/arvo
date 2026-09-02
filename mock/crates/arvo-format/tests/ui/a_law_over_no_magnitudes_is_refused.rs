// A quantum law that ranges over no magnitudes, which describes no values.
//
// The condition lived in a doc comment on `MAGNITUDES` and nothing held it, so
// this compiled and `contains` and `has_additive_identity` then disagreed about
// whether zero was in the set: the first read the magnitude range and said
// nothing is a member, the second read only the slot range and said the identity
// is there.
//
// The obligation is a const, so it is evaluated where it is forced. Forcing it in
// a const context is what makes the refusal reachable at check time rather than
// only at codegen, which is the qualification `Quantum::ADMITTED` states about
// itself.

use arvo_format::quantum::{magnitude_in_range, Exponent, Magnitude, MagnitudeCount, Quantum};
use arvo_format::width::Bool;

struct NoMagnitudes;

impl Quantum for NoMagnitudes {
    const BASE: Exponent = Exponent::ZERO;
    const SLOPE: Exponent = Exponent::ONE;
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(0);
}

const _REFUSED: Bool = magnitude_in_range::<NoMagnitudes>(Magnitude::SMALLEST);

fn main() {}
