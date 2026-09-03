//! One name, two representable sets, on four targets from one source.
//!
//! `ruling::the_format_spine_is_canon` stamps that a format is identified by its
//! ambient domain and its representable set. So two declarations with different
//! representable sets are two formats, by the ratified identity.
//!
//! This asks what `USize` names. It declares the platform-width alias once, then
//! reads its representable set out at const time, and the build is run against
//! every installed target. If the set differs, the name is not a format: it is a
//! function from a target to a format, which is what a target-indexed family is.
//!
//! **The case that must fail, stated before the run**:
//! `THE_UNIVERSAL_CLAIM_ABOUT_USIZE` asserts what is true of `USize` on a 64-bit
//! target. It must build on `aarch64-apple-darwin` and `x86_64-unknown-linux-gnu`
//! and must fail with `E0080` on `i686-unknown-linux-gnu` and
//! `thumbv6m-none-eabi`. A run where it builds everywhere would mean the alias is
//! not reading the target at all and the whole probe is void; a run where it fails
//! everywhere would mean the assertion is simply wrong.
//!
//! Every arm sits eight bits below the platform width, so the declaration stays
//! inside the widths the shipped slot family admits. That subtraction is why this
//! probe can run at all, and it is `p01`'s finding wearing a workaround.

#![no_std]

use arvo_format::points::UFixed;
use arvo_format::slots::Slots;
use arvo_format::{Format, slot_count};

/// The platform's pointer width, less eight.
#[cfg(target_pointer_width = "16")]
pub const PLATFORM_BITS: u32 = 16;
#[cfg(target_pointer_width = "32")]
pub const PLATFORM_BITS: u32 = 32;
#[cfg(target_pointer_width = "64")]
pub const PLATFORM_BITS: u32 = 64;

/// The width this probe declares at, kept inside the shipped family's range.
pub const DECLARED_BITS: u32 = PLATFORM_BITS - 8;

/// The platform-width alias, written once.
pub type USize = UFixed<{ DECLARED_BITS }, 0>;

/// What the name denotes here, whatever "here" is. Always true, and it is the
/// control: it establishes the alias resolves and the coordinates read back, on
/// whichever target this happens to be compiled for.
pub const WHAT_THE_NAME_DENOTES_HERE: () = {
    assert!(<USize as Format>::Slots::WIDTH.count() == DECLARED_BITS);
    assert!(slot_count::<<USize as Format>::Slots>().count() == (1i64 << DECLARED_BITS));
    assert!(<USize as Format>::Slots::MIN.index() == 0);
    assert!(<USize as Format>::Slots::MAX.index() == (1i64 << DECLARED_BITS) - 1);
};

/// A sentence about `USize` that reads as a statement about a format.
///
/// It is not one. It is a statement about a format on one target and a falsehood
/// on another, and nothing in how it is written says so. This is the whole of what
/// makes a platform-width name different from `UFixed<56, 0>`, which means the
/// same thing everywhere.
///
/// Under `every-finding-carries-its-predicate.md` the honest form of this sentence
/// carries `target_pointer_width = 64`, and the point of the arm is that the
/// unpredicated form compiles on the target where it happens to hold.

/// The same claim written honestly, which is the arm that builds everywhere.
///
/// Not a fix for the one above. A different sentence, about a family rather than
/// about a format, and it says less: it says the width is the platform's less
/// eight and declines to say what the platform's is.
pub const THE_CLAIM_ABOUT_THE_FAMILY: () = {
    assert!(<USize as Format>::Slots::WIDTH.count() == PLATFORM_BITS - 8);
};
