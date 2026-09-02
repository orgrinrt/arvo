//! Is the line between what renders today and what does not a principled one?
//!
//! Eighteen public types across the three crates derive `Debug` and thirty-five
//! do not. A census by grep says which, and a census by grep is a claim about a
//! text. This asserts the property instead: everything that renders holds a
//! value, and everything that does not is zero sized.
//!
//! It matters because the obvious act on a type with no `Debug` is to add one,
//! and if the line is principled then the thirty-five are not an oversight and
//! adding derives to them is replacing a rule with a list.
//!
//! Asserted as `const` items, so a wrong entry is a build failure rather than a
//! test that has to be run. The two controls are at the bottom.

#![no_std]

use core::mem::size_of;

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::ambient::{BinaryRationals, DecimalRationals, UnsignedBinaryRationals};
use arvo_format::overflow::{Clamp, Saturate, Wrap};
use arvo_format::points::{Biased, Floating, Integer, UFixed};
use arvo_format::quantum::{Constant, Indexed};
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, Stochastic, TowardZero};
use arvo_format::slots::{Signed, Unsigned};
use arvo_format::standards::{Fi, FractionLength, Ufi};
use arvo_format::{Arity, Bool, Exponent, Magnitude, MagnitudeCount, Radix, Slot, SlotCount, Width};
use arvo_format::apply::{Dither, Exact, Fraction};
use arvo_format::format::Phase;
use arvo_format::overflow::Policy;
use arvo_format::rounding::Mode;
use arvo_placement::objective::{Access, Footprint};
use arvo_placement::{Carrier16, Carrier32, Carrier64, Carrier8, Objective, Occupancy, Placement};
use arvo_strategy::presets::{Cold, Hot, Precise, Warm};

/// Every type that does not render is zero sized. Thirty-five of them, listed in
/// full rather than sampled, because a sample chooses what not to find out.
const _NON_RENDERERS_ARE_ZERO_SIZED: () = {
    assert!(size_of::<Adapt<TowardZero, Wrap>>() == 0);
    assert!(size_of::<Signature<Integer<8>, Adapt<TowardZero, Wrap>>>() == 0);
    assert!(size_of::<BinaryRationals>() == 0);
    assert!(size_of::<UnsignedBinaryRationals>() == 0);
    assert!(size_of::<DecimalRationals>() == 0);
    assert!(size_of::<Integer<8>>() == 0);
    assert!(size_of::<Integer<32>>() == 0);
    assert!(size_of::<UFixed<8, -4>>() == 0);
    assert!(size_of::<Biased<8, 0, 1>>() == 0);
    assert!(size_of::<Floating<23, -126, 254>>() == 0);
    assert!(size_of::<Wrap>() == 0);
    assert!(size_of::<Saturate>() == 0);
    assert!(size_of::<Clamp>() == 0);
    assert!(size_of::<Constant<0>>() == 0);
    assert!(size_of::<Indexed<-3, 4>>() == 0);
    assert!(size_of::<TowardZero>() == 0);
    assert!(size_of::<Floor>() == 0);
    assert!(size_of::<Ceil>() == 0);
    assert!(size_of::<HalfUp>() == 0);
    assert!(size_of::<HalfEven>() == 0);
    assert!(size_of::<Stochastic>() == 0);
    assert!(size_of::<Signed<8>>() == 0);
    assert!(size_of::<Unsigned<8>>() == 0);
    assert!(size_of::<FractionLength<4>>() == 0);
    assert!(size_of::<Fi<16, 8>>() == 0);
    assert!(size_of::<Ufi<16, 8>>() == 0);
    assert!(size_of::<Carrier8>() == 0);
    assert!(size_of::<Carrier16>() == 0);
    assert!(size_of::<Carrier32>() == 0);
    assert!(size_of::<Carrier64>() == 0);
    assert!(size_of::<Footprint>() == 0);
    assert!(size_of::<Access>() == 0);
    assert!(size_of::<Hot>() == 0);
    assert!(size_of::<Cold>() == 0);
    assert!(size_of::<Precise>() == 0);
    assert!(size_of::<Warm>() == 0);
};

/// Every type that renders holds a value. Eighteen, in full.
///
/// This is the control on the assertion above. Without it the block would pass
/// against a crate in which every public type were a marker, and the split would
/// be a fact about arvo shipping nothing rather than about where the line falls.
const _RENDERERS_HOLD_A_VALUE: () = {
    assert!(size_of::<Width>() > 0);
    assert!(size_of::<Bool>() > 0);
    assert!(size_of::<Radix>() > 0);
    assert!(size_of::<Exponent>() > 0);
    assert!(size_of::<Magnitude>() > 0);
    assert!(size_of::<MagnitudeCount>() > 0);
    assert!(size_of::<Slot>() > 0);
    assert!(size_of::<SlotCount>() > 0);
    assert!(size_of::<Arity>() > 0);
    assert!(size_of::<Phase>() > 0);
    assert!(size_of::<Fraction>() > 0);
    assert!(size_of::<Exact>() > 0);
    assert!(size_of::<Dither>() > 0);
    assert!(size_of::<Mode>() > 0);
    assert!(size_of::<Policy>() > 0);
    assert!(size_of::<Occupancy>() > 0);
    assert!(size_of::<Objective>() > 0);
    assert!(size_of::<Placement>() > 0);
};

#[cfg(test)]
mod tests {
    use super::*;

    /// The second control, on the instrument rather than on the population.
    /// `size_of` must be able to report both answers, or the two blocks above
    /// are one tautology written twice.
    #[test]
    fn the_control_size_of_discriminates() {
        assert_eq!(size_of::<Integer<8>>(), 0);
        assert_eq!(size_of::<Width>(), 4);
        assert_ne!(size_of::<Integer<8>>(), size_of::<Width>());
    }

    /// The consequence, stated as a test so it is not only prose: the four
    /// points of the parameterisation hold no value, so "debug output from every
    /// numeral" has nothing to range over in this tree.
    #[test]
    fn the_four_points_hold_no_value() {
        assert_eq!(size_of::<Integer<32>>(), 0);
        assert_eq!(size_of::<UFixed<8, -4>>(), 0);
        assert_eq!(size_of::<Biased<8, 0, 1>>(), 0);
        assert_eq!(size_of::<Floating<23, -126, 254>>(), 0);
    }
}
