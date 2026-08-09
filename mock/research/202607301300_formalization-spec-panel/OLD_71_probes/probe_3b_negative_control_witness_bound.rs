// probe 3b: negative control for the opt-in witness bound. EXPECTED TO
// FAIL with E0277 at the call sites naming NanOnly and NoSpecials: a
// consumer that states it needs the in-band overflow witness is refused a
// numeral whose far point is finite, at the exact call site, with the
// bound's name in the diagnostic. both no-infinity members are exercised,
// not one.

pub struct NoSpecials;
pub struct NanOnly;
pub struct InfOnly;
pub struct IeeeSpecials;

pub trait Specials {
    const HAS_INF: bool;
}
impl Specials for NoSpecials {
    const HAS_INF: bool = false;
}
impl Specials for NanOnly {
    const HAS_INF: bool = false;
}
impl Specials for InfOnly {
    const HAS_INF: bool = true;
}
impl Specials for IeeeSpecials {
    const HAS_INF: bool = true;
}

pub trait AbsorbingFarPoint: Specials {}
impl AbsorbingFarPoint for InfOnly {}
impl AbsorbingFarPoint for IeeeSpecials {}

pub fn fold_needing_inband_witness<S: AbsorbingFarPoint>() {}

fn main() {
    fold_needing_inband_witness::<NanOnly>(); // E0277 expected here
    fold_needing_inband_witness::<NoSpecials>(); // and here
}
