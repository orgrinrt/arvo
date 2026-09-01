//! What the admission contract actually asks a candidate to supply, measured
//! from outside the crate that defines it.
//!
//! Seat 242. The ratified clause is `the_concept_is_closed_and_the_inventory_is
//! _open`: a new instance "earns admission by supplying the concept's
//! obligations". This probe asks which obligations exist.
//!
//! Default build: the positive arm, an outside crate writing all ten ratified
//! coordinates at values no shipped point uses.
//!
//! Each feature is one negative control. A control that BUILDS is a coordinate
//! with no obligation on it.

use arvo_format::{
    contains, exponent_at, has_additive_identity, is_constant_family,
    magnitude_in_range, radix, slot_count, slot_in_range, Ambient, Format, Quantum, Slots,
};
use arvo_format::Width;
// NOT re-exported at the crate root, unlike `slot_count` and `slot_in_range`
// declared beside it. Reached through the module.
use arvo_format::slots::is_admissible;

// ---------------------------------------------------------------- positive arm

/// Radix seven. Nothing in the crate ships a radix that is not 2 or 10.
pub struct Septenary;
impl Ambient for Septenary {
    const RADIX: u32 = 7;
    const SIGNED: bool = true;
}

/// Slope two: the step exponent moves two per magnitude. The crate ships slope
/// 0 (`Constant`) and slope 1 (`Indexed`) and nothing else, and
/// `is_constant_family` is documented as "the axis the two families differ on
/// and the only one", so a slope of two is outside both shipped families.
pub struct SteepQuantum;
impl Quantum for SteepQuantum {
    const BASE: i32 = -5;
    const SLOPE: i32 = 2;
    const MAGNITUDES: u32 = 4;
}

/// An asymmetric slot range that is not a two's-complement or unsigned window.
/// `Signed<N>` and `Unsigned<N>` are the only shipped shapes and both are
/// symmetric-or-anchored; this is neither, and its span is not a power of two.
pub struct Lopsided;
impl Slots for Lopsided {
    const MIN: i64 = -11;
    const MAX: i64 = 20;
    const WIDTH: Width = Width::bits(5);
}

/// Half-step phase, on a radix-seven domain, over a slope-two quantum.
pub struct Outsider;
impl Format for Outsider {
    type Ambient = Septenary;
    type Quantum = SteepQuantum;
    type Slots = Lopsided;
    const PHASE_NUM: i64 = 1;
    const PHASE_DEN: i64 = 2;
}

fn positive_arm() {
    // All ten coordinates were written here and none of them is a shipped point.
    assert_eq!(radix::<Outsider>(), 7);
    assert!(<Outsider as Format>::Ambient::SIGNED);

    // Slope two: -5, -3, -1, 1 across the four magnitudes.
    assert_eq!(exponent_at::<SteepQuantum>(0), -5);
    assert_eq!(exponent_at::<SteepQuantum>(3), 1);
    assert!(!is_constant_family::<SteepQuantum>());
    assert!(magnitude_in_range::<SteepQuantum>(3));
    assert!(!magnitude_in_range::<SteepQuantum>(4));

    // The asymmetric window, and the obligation forcing on it.
    let () = <Lopsided as Slots>::ADMITTED;
    assert!(is_admissible::<Lopsided>());
    assert_eq!(slot_count::<Lopsided>(), 32);
    assert!(slot_in_range::<Lopsided>(-11) && slot_in_range::<Lopsided>(20));
    assert!(!slot_in_range::<Lopsided>(21));

    // Nonzero phase takes zero off the grid, which is the coordinate's stated job.
    assert!(!has_additive_identity::<Outsider>());
    assert!(contains::<Outsider>(0, 0));
    assert!(!contains::<Outsider>(0, 4));

    println!("POSITIVE ARM: an outside crate wrote all ten coordinates and every derived \
              quantity resolved. The inventory is open in fact, not only in prose.");
}

// ------------------------------------------------------- negative control: phase

/// `Format::PHASE_DEN` is documented "One for an unbiased grid, two for the
/// half-step bias. Never zero." Nothing in the crate reads it, so nothing can
/// enforce that.
#[cfg(feature = "phase_den_zero")]
mod phase_den_zero {
    use super::*;
    pub struct ZeroDen;
    impl Format for ZeroDen {
        type Ambient = Septenary;
        type Quantum = SteepQuantum;
        type Slots = Lopsided;
        const PHASE_NUM: i64 = 1;
        const PHASE_DEN: i64 = 0; // a phase of one over zero
    }
    pub fn run() {
        // Every law the crate has over a format still answers.
        assert!(contains::<ZeroDen>(0, 0));
        assert!(!has_additive_identity::<ZeroDen>());
        assert_eq!(radix::<ZeroDen>(), 7);
        println!("CONTROL phase_den_zero: BUILT. PHASE_DEN = 0 is admitted. \
                  Every law over the format passed with a phase of 1/0.");
    }
}

// ------------------------------------------------------- negative control: radix

/// A radix of one is not a positional notation: one digit cannot denote a
/// second value. `Ambient` carries no obligation.
#[cfg(feature = "radix_one")]
mod radix_one {
    use super::*;
    pub struct UnaryDomain;
    impl Ambient for UnaryDomain {
        const RADIX: u32 = 1;
        const SIGNED: bool = true;
    }
    pub struct OverUnary;
    impl Format for OverUnary {
        type Ambient = UnaryDomain;
        type Quantum = SteepQuantum;
        type Slots = Lopsided;
        const PHASE_NUM: i64 = 0;
        const PHASE_DEN: i64 = 1;
    }
    pub fn run() {
        assert_eq!(radix::<OverUnary>(), 1);
        assert!(contains::<OverUnary>(0, 0));
        println!("CONTROL radix_one: BUILT. RADIX = 1 is admitted as an ambient domain.");
    }
}

// -------------------------------------------------- negative control: magnitudes

/// `MAGNITUDES = 0` makes `magnitude_in_range` false for every magnitude, so
/// `contains` is false everywhere: the representable set is empty. A format
/// with no members is admitted. `Quantum` carries no obligation.
#[cfg(feature = "magnitudes_zero")]
mod magnitudes_zero {
    use super::*;
    pub struct NoMagnitudes;
    impl Quantum for NoMagnitudes {
        const BASE: i32 = 0;
        const SLOPE: i32 = 0;
        const MAGNITUDES: u32 = 0;
    }
    pub struct Empty;
    impl Format for Empty {
        type Ambient = Septenary;
        type Quantum = NoMagnitudes;
        type Slots = Lopsided;
        const PHASE_NUM: i64 = 0;
        const PHASE_DEN: i64 = 1;
    }
    pub fn run() {
        let mut any = false;
        let mut m = 0u32;
        while m < 64 {
            if contains::<Empty>(0, m) {
                any = true;
            }
            m += 1;
        }
        assert!(!any, "expected an empty representable set");
        // And it still claims an additive identity it cannot hold.
        assert!(has_additive_identity::<Empty>());
        println!("CONTROL magnitudes_zero: BUILT. An empty representable set is admitted, \
                  and `has_additive_identity` says true of a set with no members.");
    }
}

// ------------------------------------- positive control: the one obligation that exists

/// The instrument's own control. `Slots::ADMITTED` is the crate's one real
/// admission check. If THIS builds, the probe cannot detect a refusal at all
/// and every "BUILT" above is worthless.
#[cfg(feature = "inverted_slots")]
mod inverted_slots {
    use super::*;
    pub struct Inverted;
    impl Slots for Inverted {
        const MIN: i64 = 40;
        const MAX: i64 = -40;
        const WIDTH: Width = Width::bits(5);
    }
    pub fn run() {
        assert!(!is_admissible::<Inverted>());
        // Forcing the obligation is what must refuse at codegen.
        let () = <Inverted as Slots>::ADMITTED;
        println!("CONTROL inverted_slots: BUILT. The instrument cannot detect a refusal; \
                  every other result in this probe is void.");
    }
}

fn main() {
    positive_arm();
    #[cfg(feature = "phase_den_zero")]
    phase_den_zero::run();
    #[cfg(feature = "radix_one")]
    radix_one::run();
    #[cfg(feature = "magnitudes_zero")]
    magnitudes_zero::run();
    #[cfg(feature = "inverted_slots")]
    inverted_slots::run();
}
