//! p4: the obligation's range, checked against the platform width per target.
//!
//! `obligation::a_platform_sized_unsigned_integer_at_an_api_position` asks for
//! an unsigned integer of the platform's size whose range covers a unix errno
//! and a Windows GetLastError value. errno is a C `int` whose kernel-side bound
//! is 4095 (Linux `MAX_ERRNO`); GetLastError returns a `DWORD`, an unsigned
//! 32-bit value. So the obligation holds at a pointer width exactly when the
//! set reaches `2^32 - 1`.
#![no_std]

use arvo_format::ambient::UnsignedBinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::{Unsigned, slot_count};

pub struct PlatformUnsigned;

impl Format for PlatformUnsigned {
    type Ambient = UnsignedBinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Unsigned<{ usize::BITS }>;

    const PHASE: Phase = Phase::ZERO;
}

const MEMBERS: i128 = slot_count::<<PlatformUnsigned as Format>::Slots>().count() as i128;

const _: () = assert!(
    MEMBERS > 4095,
    "a unix errno (bounded by MAX_ERRNO, 4095) fits"
);
const _: () = assert!(
    MEMBERS > u32::MAX as i128,
    "a Windows GetLastError DWORD does not fit the platform width on this target"
);
