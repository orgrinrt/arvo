//! Can a platform-width numeral be spelled over the shipped format contract?
//!
//! The question Q26 gates. If a platform-width type is an instance of the shape
//! family, then on a 64-bit target it is that family at 64 bits, and the spelling
//! is one cfg-selected const and an alias. This asks whether that spelling exists.
//!
//! **The case that must fail, stated before the run**: `Integer<64>` and
//! `UFixed<64, 0>` are not `Format`s, because `admit_widths!` in `slots.rs` stops
//! at 62, so `Signed<64>: Slots` and `Unsigned<64>: Slots` have no impl. The
//! refusal is a trait-bound error and it is committed with its stderr in
//! `the_sixty_four_bit_width.stderr`.
//!
//! **The controls**, which are what say the refusal is the width bound and not the
//! alias construction: the identical shape at 32 and at 62 builds, and the
//! cfg-selected const reaches a const generic position at those widths. If the
//! controls failed, the refusal would be a fact about how the alias was written.
//!
//! What builds here is the control half. The refusal half is a separate file
//! compiled on its own, because a crate that does not build proves nothing about
//! the crate that does.

#![no_std]

use arvo_format::points::{Integer, UFixed};
use arvo_format::slots::{Signed, Slots, Unsigned};
use arvo_format::{Format, Magnitude, Slot, Width, contains, has_additive_identity};

/// The target's pointer width as a count of bits, selected by `cfg`.
///
/// The whole of what a platform-width declaration needs: not a mechanism, not an
/// axis, one const whose value the compilation fixes. Sixteen is here because
/// `cfg(target_pointer_width)` has that arm and a platform-width type on such a
/// target is the case where the shipped bound is not reached.
#[cfg(target_pointer_width = "16")]
pub const PLATFORM_BITS: u32 = 16;
#[cfg(target_pointer_width = "32")]
pub const PLATFORM_BITS: u32 = 32;
#[cfg(target_pointer_width = "64")]
pub const PLATFORM_BITS: u32 = 64;

/// A platform-width unsigned numeral at a width the shipped family admits.
///
/// Not the platform's, which is the finding. Thirty-two is written literally so
/// this arm builds on a 64-bit host: it is the control that establishes the
/// spelling works at all, and `THE_ALIAS_AT_THE_REAL_PLATFORM_WIDTH` below is the
/// arm that carries the actual target and is the one that refuses.
pub type USizeAt32 = UFixed<32, 0>;

/// The same, signed.
pub type ISizeAt32 = Integer<32>;

/// The widest the shipped slot family admits, which is 62 and not 64.
pub type USizeAtTheCeiling = UFixed<62, 0>;

/// Control: a cfg-selected const does reach a const generic position.
///
/// The construction Q26's second option would need. If this failed, the refusal
/// recorded beside it would be about const arguments rather than about widths.
/// Sixteen is subtracted so the arm lands inside the admitted range on every
/// target `cfg(target_pointer_width)` has, which is what keeps the control
/// independent of the host it happens to run on.
pub type ACfgSelectedWidth = UFixed<{ PLATFORM_BITS - 16 }, 0>;

/// The controls hold, asserted at const time.
///
/// Each reads a coordinate through the trait rather than off the struct, so a
/// coordinate that stopped being an associated item would fail here.
pub const THE_CONTROLS_HOLD: () = {
    assert!(<USizeAt32 as Format>::Slots::WIDTH.count() == 32);
    assert!(<ISizeAt32 as Format>::Slots::WIDTH.count() == 32);
    assert!(<USizeAtTheCeiling as Format>::Slots::WIDTH.count() == 62);
    assert!(<ACfgSelectedWidth as Format>::Slots::WIDTH.count() == PLATFORM_BITS - 16);

    // The set is a constant of the type: the bounds come back without a value in
    // hand, which is what "constant of the type" means operationally.
    assert!(<Unsigned<32> as Slots>::MIN.index() == 0);
    assert!(<Unsigned<32> as Slots>::MAX.index() == (1i64 << 32) - 1);
    assert!(<Signed<32> as Slots>::MIN.index() == -(1i64 << 31));

    // And the membership predicate answers over it.
    assert!(contains::<USizeAt32>(Slot::ZERO, Magnitude::SMALLEST).get());
    assert!(has_additive_identity::<USizeAt32>().get());
};

/// The width the shipped family stops at, read from the crate rather than typed.
///
/// A literal here would go stale the moment `admit_widths!` moves, and the whole
/// point of this probe is a bound that a later design is expected to move.
pub const THE_ADMITTED_CEILING: Width = {
    let widths = arvo_format::slots::ADMITTED_WIDTHS;
    widths[widths.len() - 1]
};

/// The bound is 62 and the platform is wider, on any 64-bit target.
///
/// Written as an implication rather than as two asserts, so the arm says nothing
/// on a 16-bit or 32-bit target instead of saying something false there.
pub const THE_PLATFORM_IS_PAST_THE_CEILING: () = {
    if PLATFORM_BITS == 64 {
        assert!(THE_ADMITTED_CEILING.count() < PLATFORM_BITS);
    }
};
