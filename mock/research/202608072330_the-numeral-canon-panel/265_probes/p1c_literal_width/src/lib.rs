//! p1c: the control for p1. Same declaration, width written as the literal 32.
//!
//! Hypothesis: a platform-width type is an ordinary point of the ratified
//! parameterisation whose width coordinate is bound by the compilation target
//! rather than written as a literal. If that is right, this crate is a `Format`
//! at every target the shipped `Slots` ladder admits, and the one thing that
//! moves across targets is the value of the width coordinate.
//!
//! The control crate beside this one differs in exactly one token per format:
//! the width is the literal `32` instead of `usize::BITS` / `isize::BITS`.
//!
//! Every arithmetic here is done one domain wider than the slot index, so that
//! the only thing that can refuse this crate on a 64-bit target is the absence of
//! `Unsigned<64>` and `Signed<64>` from the shipped ladder.
#![no_std]

use arvo_format::ambient::{BinaryRationals, UnsignedBinaryRationals};
use arvo_format::format::{Format, Phase, contains};
use arvo_format::quantum::{Constant, Magnitude};
use arvo_format::slots::{Signed, Slot, SlotCount, Unsigned, declared_slot_width, slot_count};
use arvo_format::width::{Bool, Width};

/// The platform-width unsigned integer: quantum one, phase zero, one magnitude,
/// slot range `0 ..= 2^W - 1` with `W` the target's pointer width.
pub struct PlatformUnsigned;

impl Format for PlatformUnsigned {
    type Ambient = UnsignedBinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Unsigned<32>;

    const PHASE: Phase = Phase::ZERO;
}

/// The platform-width signed integer, the same with a two's complement range.
pub struct PlatformSigned;

impl Format for PlatformSigned {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<32>;

    const PHASE: Phase = Phase::ZERO;
}

/// `2^W - 1` computed wide, so the shift itself cannot refuse at `W = 64`.
const TOP: i64 = ((1i128 << 32u32) - 1) as i64;

/// The width coordinate, read back through the contract rather than the literal.
pub const WIDTH: Width = declared_slot_width::<<PlatformUnsigned as Format>::Slots>();

/// The cardinality of the representable set, a constant of the type at this
/// compilation.
pub const COUNT: SlotCount = slot_count::<<PlatformUnsigned as Format>::Slots>();

/// The representable set is closed at the top: `2^W - 1` is a member and `2^W`
/// is not. Evaluated at compile time, so a wrong answer refuses the build.
pub const TOP_IS_IN: Bool = contains::<PlatformUnsigned>(Slot::at(TOP), Magnitude::SMALLEST);
pub const PAST_TOP_IS_OUT: Bool =
    contains::<PlatformUnsigned>(Slot::at(TOP.wrapping_add(1)), Magnitude::SMALLEST).not();

const _: () = assert!(
    WIDTH.count() == 32u32,
    "the width coordinate is the pointer width"
);
const _: () = assert!(
    COUNT.count() as i128 == 1i128 << 32u32,
    "the set has 2^W members"
);
const _: () = assert!(TOP_IS_IN.get(), "2^W - 1 is representable");
const _: () = assert!(PAST_TOP_IS_OUT.get(), "2^W is not representable");
