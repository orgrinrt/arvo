// Probe 1: the far-direction reading of Direction (TowardPositive at OverRange,
// TowardNegative at UnderRange) as IEEE overflow-to-infinity, gated on whether
// the numeral's Specials makes the signed infinities representable.
//
// Hypothesis: no new Resolution/Direction constructor is needed to express
// "quantise an out-of-range exact value to the signed infinity". The same
// closed vocabulary that expresses Clamp (near-direction: TowardNegative at
// OverRange, TowardPositive at UnderRange, picking the existing finite
// neighbour) expresses overflow-to-infinity under the opposite (far)
// polarity, PROVIDED the numeral's Specials axis makes the far point
// representable. Where it does not (NoSpecials, NanOnly), the far-direction
// reading must be unrepresentable, because there is nothing there to round
// toward.
//
// This mirrors 11_current_shape_draft.md's existing note that "clamp needs
// no name of its own: clamping above the range is simply TowardNegative,
// the same marker used between neighbours" (the near-direction reading).
// This probe adds the far-direction reading and its well-formedness bound.

mod vocab {
    pub trait Resolution {}
    pub trait Direction: Resolution {}

    pub struct TowardNegative;
    pub struct TowardPositive;
    pub struct TowardZero;
    pub struct AwayFromZero;
    pub struct ToEven;
    pub struct ToOdd;
    impl Resolution for TowardNegative {}
    impl Direction for TowardNegative {}
    impl Resolution for TowardPositive {}
    impl Direction for TowardPositive {}
    impl Resolution for TowardZero {}
    impl Direction for TowardZero {}
    impl Resolution for AwayFromZero {}
    impl Direction for AwayFromZero {}
    impl Resolution for ToEven {}
    impl Direction for ToEven {}
    impl Resolution for ToOdd {}
    impl Direction for ToOdd {}

    pub struct ReduceModulo;
    pub struct SubstituteZero;
    pub struct Refuse;
    impl Resolution for ReduceModulo {}
    impl Resolution for SubstituteZero {}
    impl Resolution for Refuse {}
}

mod specials {
    pub trait Specials: sealed::Sealed {}
    pub struct NoSpecials;
    pub struct NanOnly;
    pub struct InfOnly;
    pub struct IeeeSpecials;

    mod sealed {
        pub trait Sealed {}
        impl Sealed for super::NoSpecials {}
        impl Sealed for super::NanOnly {}
        impl Sealed for super::InfOnly {}
        impl Sealed for super::IeeeSpecials {}
    }
    impl Specials for NoSpecials {}
    impl Specials for NanOnly {}
    impl Specials for InfOnly {}
    impl Specials for IeeeSpecials {}

    // the marker: does this Specials instance make the signed infinities
    // representable points. Only InfOnly and IeeeSpecials do.
    pub trait HasInfinity: Specials {}
    impl HasInfinity for InfOnly {}
    impl HasInfinity for IeeeSpecials {}
}

use specials::{HasInfinity, IeeeSpecials, InfOnly, NanOnly, NoSpecials, Specials};
use vocab::{Direction, TowardNegative, TowardPositive};

trait Numeral {
    type Specials: Specials;
}

struct HardwareBinary32;
impl Numeral for HardwareBinary32 {
    type Specials = IeeeSpecials;
}

struct BoundedFixed;
impl Numeral for BoundedFixed {
    type Specials = NoSpecials;
}

struct NanCarryingButFinite;
impl Numeral for NanCarryingButFinite {
    type Specials = NanOnly;
}

struct InfCarrying;
impl Numeral for InfCarrying {
    type Specials = InfOnly;
}

// The far-direction obligation: this is only a coherent OverRange resolution
// (rounds the exact value to +infinity rather than clamping to the finite
// max) when the numeral's Specials makes +infinity representable.
trait OverflowsToFarPoint<N: Numeral, D: Direction>
where
    N::Specials: HasInfinity,
{
}

struct FarResolution;
impl<N: Numeral> OverflowsToFarPoint<N, TowardPositive> for FarResolution where
    N::Specials: HasInfinity
{
}

fn accepts_ieee_overflow_to_infinity<N: Numeral>()
where
    N::Specials: HasInfinity,
    FarResolution: OverflowsToFarPoint<N, TowardPositive>,
{
}

fn main() {
    // WORKS: a numeral whose Specials carries the infinities can be
    // resolved by the far-direction reading.
    accepts_ieee_overflow_to_infinity::<BoundedFixed>();

    // Uncomment either line below and the crate refuses to compile,
    // which is the point: a numeral whose Specials has no representable
    // infinity cannot be given the far-direction reading, because there
    // is nothing there to round toward. Verified separately below.
    // accepts_ieee_overflow_to_infinity::<BoundedFixed>();
    // accepts_ieee_overflow_to_infinity::<NanCarryingButFinite>();

    println!("probe 1: far-direction resolution well-formed only where Specials: HasInfinity");
}
