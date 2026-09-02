//! probe 2b: expected-fail. Conformance is refused at the declaration site.
//!
//! A consumer who needs the standard's own `quantize` behaviour (invalid operation
//! rather than a substituted value when the result needs more than `p` digits) states
//! it as a bound. A preset whose range row does not refuse cannot satisfy the bound,
//! and the refusal is a compile error at the call site rather than a runtime check,
//! a value the consumer must inspect, or a grade nobody reads.
//!
//! **This file must NOT compile.** If it ever does, the conformance bound has stopped
//! separating the presets and the guarantee is gone. Expected: `E0277`.

#[path = "probe_2_quantise_as_a_crossing.rs"]
mod q;

use q::{conforming_quantise, Dec3, Warm, E0};

pub fn wrong_preset(vx: i128) -> <<Warm as q::Preset>::OverRange as q::Resolution>::Out {
    // Warm's OverRange row is Clamp, not Refuse, so Warm is not ConformingQuantise.
    conforming_quantise::<Dec3, E0, Warm>(vx)
}
