// Wrapping on a floating format, the pair that turns on the overflow policy.
//
// A union of magnitude ranges at more than one quantum has a gap, so no
// reduction of it is a group, and wrapping is refused from the declared
// signature. Saturating the same format is admitted. The two files differ in the
// policy and nothing else.

use core::num::NonZeroU128;

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::apply::{Dither, Scaled, land};
use arvo_format::overflow::Wrap;
use arvo_format::points::Floating;
use arvo_format::quantum::Exponent;
use arvo_format::rounding::TowardZero;

type Sig = Signature<Floating<4, -6, 8>, Adapt<TowardZero, Wrap>>;

const _LANDED: () = {
    let _ = land::<Sig>(
        Scaled::of(1, NonZeroU128::MIN, Exponent::ZERO),
        Dither::UNUSED,
    );
};

fn main() {}
