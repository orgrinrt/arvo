//! p3: the same name observed at one pinned value across compilations.
//!
//! Hypothesis: `PlatformUnsigned` is one name whose realisations disagree across
//! targets on a nullary observation, the width. The ratified admission rule says
//! that where two realisations of one name disagree the signature is missing a
//! coordinate; this crate pins the observation at 32 and lets each target say
//! whether it is that realisation. Passing at 32-bit targets and refusing
//! elsewhere is the disagreement shown as a build result rather than argued.
#![no_std]

use arvo_format::ambient::UnsignedBinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::{Slots, Unsigned};

pub struct PlatformUnsigned;

impl Format for PlatformUnsigned {
    type Ambient = UnsignedBinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Unsigned<{ usize::BITS }>;

    const PHASE: Phase = Phase::ZERO;
}

const _: () = assert!(
    <<PlatformUnsigned as Format>::Slots as Slots>::WIDTH.count() == 32,
    "on this target the name PlatformUnsigned does not denote the 32-bit set"
);
