// A quantum law whose exponent runs past what an exponent carries before it
// reaches its largest magnitude.
//
// The second condition, and it was written nowhere at all. `Exponent::advanced`
// computes `base + rate * magnitude` in the exponent's own width over an index
// cast down into it, so both narrow: without the obligation this law wraps the
// sum, or panics under `overflow-checks`, and either way the crate answers with a
// step law it does not have.

use arvo_format::quantum::{exponent_at, Exponent, Magnitude, MagnitudeCount, Quantum};

struct ReachRunsOff;

impl Quantum for ReachRunsOff {
    const BASE: Exponent = Exponent::of(i32::MAX - 2);
    const SLOPE: Exponent = Exponent::ONE;
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(8);
}

const _REFUSED: Exponent = exponent_at::<ReachRunsOff>(Magnitude::SMALLEST);

fn main() {}
