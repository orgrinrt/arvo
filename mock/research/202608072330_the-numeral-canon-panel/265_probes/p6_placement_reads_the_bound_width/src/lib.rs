//! p6: the placement tier over a target-bound width.
//!
//! Hypothesis: nothing in the placement tier needs a platform-width concept. The
//! ladder is a list of carriers the host offers, it does not consult the target,
//! and handing it the pointer width as a `Width` returns the machine word at
//! every pointer width the ladder reaches, because the machine word is the
//! narrowest carrier covering the pointer width by definition of pointer width.
//!
//! The first half compiles at every target, since it touches no `Slots` impl.
//! The second half derives a sole-occupancy placement over the platform-width
//! signature, and inherits p1's refusal at 64 bits for the same reason p1 does.
#![no_std]

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::ambient::UnsignedBinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::overflow::Wrap;
use arvo_format::quantum::Constant;
use arvo_format::rounding::HalfEven;
use arvo_format::slots::Unsigned;
use arvo_format::width::Width;
use arvo_placement::objective::{Access, Footprint};
use arvo_placement::{LADDER, Occupancy, Placement, derive_sole, narrowest_carrier};

// --- the ladder over the pointer width, no format involved -------------------

/// The narrowest shipped carrier covering the pointer width.
pub const CARRIER: Width = narrowest_carrier(Width::bits(usize::BITS));

const _: () = assert!(
    CARRIER.count() == usize::BITS,
    "the narrowest carrier covering the pointer width is the machine word"
);

/// And the ladder itself is the same list at every target: it names what the
/// host language offers, not what the target's word is.
const _: () = assert!(LADDER[0].count() == 8 && LADDER[3].count() == 64);

// --- a sole-occupancy placement of the platform-width signature --------------

pub struct PlatformUnsigned;

impl Format for PlatformUnsigned {
    type Ambient = UnsignedBinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Unsigned<{ usize::BITS }>;

    const PHASE: Phase = Phase::ZERO;
}

type Sig = Signature<PlatformUnsigned, Adapt<HalfEven, Wrap>>;

pub const SOLE_FOOTPRINT: Placement = derive_sole::<Sig, Footprint>();
pub const SOLE_ACCESS: Placement = derive_sole::<Sig, Access>();

const _: () = assert!(
    SOLE_FOOTPRINT.carrier.count() == usize::BITS,
    "carrier is the word"
);
const _: () = assert!(
    SOLE_ACCESS.carrier.count() == usize::BITS,
    "carrier is the word"
);
const _: () = assert!(matches!(SOLE_FOOTPRINT.occupancy, Occupancy::Sole));
const _: () = assert!(
    SOLE_FOOTPRINT.output_count().count() == 1,
    "one output at sole"
);
