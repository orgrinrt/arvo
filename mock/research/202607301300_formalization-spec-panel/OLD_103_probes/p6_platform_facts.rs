//! Probe 6. Which of the platform crate's facts are settled at compile time.
//!
//! The design has ratified a test for this (consolidation ten, section 1.26):
//! an environment parameter is the ambient control state a lowering's
//! correctness is conditional on, and the operative question is whether a
//! linked library can change the fact at runtime. A fact it cannot change is
//! not environment; it is settled where the code is emitted.
//!
//! Applied to the five items D27 sends to `arvo-platform`. Everything below is
//! either a const assertion that must hold at compile time or a static
//! assertion of a layout the language guarantees. If the file compiles, none of
//! these facts needed a runtime check, and none of them is an assumption.
#![no_std]
#![allow(dead_code)]

use core::mem::{align_of, size_of};
use core::num::NonZeroUsize;

// 1. Pointer width. A cfg, not an observation.
const PTR_BITS: usize = usize::BITS as usize;
const _: () = assert!(PTR_BITS == 16 || PTR_BITS == 32 || PTR_BITS == 64 || PTR_BITS == 128);
const _: () = assert!(size_of::<usize>() * 8 == PTR_BITS);

// The refusal a platform crate owes: a target the design cannot serve is a
// statically known falsehood and refuses at declaration, per the cannot-check
// versus cannot-provide split. This is the shape, not a proposed threshold.
const _: () = assert!(
    PTR_BITS >= 32,
    "arvo's capacity model needs at least a 32-bit index domain"
);

// 2. The truth primitive's layout. A language guarantee, not a host measurement.
const _: () = assert!(size_of::<bool>() == 1);
const _: () = assert!(align_of::<bool>() == 1);

// 3. The niche. `Option<bool>` collapsing to one byte is the same discriminant
//    elision the seal chapter accounts for, and it is checkable here.
const _: () = assert!(size_of::<Option<bool>>() == 1);

// 4. The nonzero carrier. This one IS a trusted-base entry: the width claim
//    rests on std's documented elision for the `NonZero` family, and the
//    assertion below is the mechanism that keeps the entry honest rather than
//    a proof of it.
const _: () = assert!(size_of::<Option<NonZeroUsize>>() == size_of::<usize>());

// 5. Endianness. Also a cfg, and worth naming because it is the one platform
//    fact with a value-layer consequence (the byte image).
#[cfg(target_endian = "little")]
const ENDIAN_LITTLE: bool = true;
#[cfg(target_endian = "big")]
const ENDIAN_LITTLE: bool = false;
const _: () = assert!(ENDIAN_LITTLE || !ENDIAN_LITTLE);

pub const fn ptr_bits() -> usize {
    PTR_BITS
}
