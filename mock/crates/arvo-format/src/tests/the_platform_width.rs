//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The two platform-width points, against the host's own pointer-sized integers.
//!
//! The aliases read `usize::BITS`, so within one compilation they are points at
//! one width and the contract cannot tell them from the literal point there. What
//! is worth asserting is what that width buys: that the range the ladder admits
//! at the pointer width is exactly the host integer's, and that an unsigned or a
//! signed error word at an API position is a member where the pointer is wide
//! enough and is not where it is narrower. The second half is asserted at every
//! pointer width a target has, not only at the one this suite was compiled for,
//! because the literal points at 16, 32 and 64 bits are what the alias names on
//! those targets.

use notko::Maybe;

use crate::format::{Format, cancelling_slot, contains, has_additive_identity, step_exponent};
use crate::points::{ISize, Integer, UFixed, USize};
use crate::quantum::{Exponent, Magnitude, Quantum};
use crate::slots::{Slot, SlotCount, Slots, declared_slot_width, slot_count};
use crate::width::{Bool, Width};

/// The ends of `USize`, bound at compile time so the width is read in a const
/// context as well as at run time.
const USIZE_LOWEST: Slot = <<USize as Format>::Slots as Slots>::MIN;
const USIZE_HIGHEST: Slot = <<USize as Format>::Slots as Slots>::MAX;
const ISIZE_LOWEST: Slot = <<ISize as Format>::Slots as Slots>::MIN;
const ISIZE_HIGHEST: Slot = <<ISize as Format>::Slots as Slots>::MAX;
const USIZE_HAS_AN_IDENTITY: Bool = has_additive_identity::<USize>();

#[test]
fn the_platform_width_ranges_are_the_host_pointer_integers_own() {
    assert_eq!(USIZE_LOWEST, Slot::at(usize::MIN as i128));
    assert_eq!(USIZE_HIGHEST, Slot::at(usize::MAX as i128));
    assert_eq!(ISIZE_LOWEST, Slot::at(isize::MIN as i128));
    assert_eq!(ISIZE_HIGHEST, Slot::at(isize::MAX as i128));

    // The two differ, so the pair is not one range read twice.
    assert_ne!(USIZE_LOWEST, ISIZE_LOWEST);
    assert_ne!(USIZE_HIGHEST, ISIZE_HIGHEST);
}

#[test]
fn the_platform_width_points_are_integers_at_the_pointer_width() {
    let pointer = Width::bits(usize::BITS);
    let count = SlotCount::of(1i128 << usize::BITS);
    assert_eq!(declared_slot_width::<<USize as Format>::Slots>(), pointer);
    assert_eq!(declared_slot_width::<<ISize as Format>::Slots>(), pointer);
    assert_eq!(slot_count::<<USize as Format>::Slots>(), count);
    assert_eq!(slot_count::<<ISize as Format>::Slots>(), count);

    // Integral: one magnitude, the quantum at exponent zero, no phase.
    for magnitudes in [
        <<USize as Format>::Quantum as Quantum>::MAGNITUDES.count(),
        <<ISize as Format>::Quantum as Quantum>::MAGNITUDES.count(),
    ] {
        assert_eq!(magnitudes, 1);
    }
    assert_eq!(step_exponent::<USize>(Magnitude::SMALLEST), Exponent::of(0));
    assert_eq!(step_exponent::<ISize>(Magnitude::SMALLEST), Exponent::of(0));
    assert!(<USize as Format>::PHASE.is_zero().get());
    assert!(<ISize as Format>::PHASE.is_zero().get());

    // Zero is a member of both and is the identity.
    assert!(USIZE_HAS_AN_IDENTITY.get());
    assert!(has_additive_identity::<ISize>().get());
    assert_eq!(
        cancelling_slot::<USize>(Magnitude::SMALLEST),
        Maybe::Is(Slot::ZERO)
    );
    assert_eq!(
        cancelling_slot::<ISize>(Magnitude::SMALLEST),
        Maybe::Is(Slot::ZERO)
    );
}

#[test]
fn the_ends_of_the_platform_width_points_are_members_and_one_past_them_is_not() {
    let at = |slot: i128| Slot::at(slot);
    let m = Magnitude::SMALLEST;
    assert!(contains::<USize>(at(0), m).get());
    assert!(contains::<USize>(at(usize::MAX as i128), m).get());
    assert!(!contains::<USize>(at(-1), m).get());
    assert!(!contains::<USize>(at(usize::MAX as i128 + 1), m).get());
    assert!(contains::<ISize>(at(isize::MIN as i128), m).get());
    assert!(contains::<ISize>(at(isize::MAX as i128), m).get());
    assert!(!contains::<ISize>(at(isize::MIN as i128 - 1), m).get());
    assert!(!contains::<ISize>(at(isize::MAX as i128 + 1), m).get());
}

/// Whether an unsigned and a signed point at `BITS` hold the given error word,
/// as the unsigned magnitude and as its negation.
fn holds_error_word<const BITS: u32>(word: i128) -> (bool, bool)
where
    crate::slots::Unsigned<BITS>: Slots,
    crate::slots::Signed<BITS>: Slots,
{
    let m = Magnitude::SMALLEST;
    (
        contains::<UFixed<BITS, 0>>(Slot::at(word), m).get(),
        contains::<Integer<BITS>>(Slot::at(-word), m).get(),
    )
}

#[test]
fn an_error_word_is_a_member_exactly_where_the_pointer_is_wide_enough() {
    // The largest error number a Linux system call returns is 4095, returned
    // negated in a signed register, so both points hold it at every pointer width
    // a target has.
    for held in [
        holds_error_word::<16>(4095),
        holds_error_word::<32>(4095),
        holds_error_word::<64>(4095),
    ] {
        assert_eq!(held, (true, true));
    }

    // A 32-bit unsigned error word, the shape a Windows error code takes, is a
    // member of the unsigned point at 32 and 64 bits and not at 16.
    let word = u32::MAX as i128;
    assert!(holds_error_word::<32>(word).0);
    assert!(holds_error_word::<64>(word).0);
    assert!(
        !holds_error_word::<16>(word).0,
        "a 16-bit pointer holds a 32-bit word"
    );

    // Its negation needs one more bit than the word, so a signed point holds it
    // only at 64. The control that the signed half can refuse at all.
    assert!(holds_error_word::<64>(word).1);
    assert!(!holds_error_word::<32>(word).1);
    assert!(!holds_error_word::<16>(word).1);

    // And on the target this was compiled for, the alias answers what the
    // literal point at its width answers.
    let m = Magnitude::SMALLEST;
    assert_eq!(
        contains::<USize>(Slot::at(word), m).get(),
        usize::BITS >= 32
    );
}

// --- one arm per pointer width, selected by the target -----------------------
//
// Each arm names the literal width the alias is on that target. Only one compiles
// per target, so the arms are checked by building the suite for each, and no gate
// builds it for any target but the host's.

#[cfg(target_pointer_width = "64")]
#[test]
fn on_a_64_bit_target_the_platform_width_points_are_the_64_bit_points() {
    assert_eq!(
        USIZE_HIGHEST,
        <<UFixed<64, 0> as Format>::Slots as Slots>::MAX
    );
    assert_eq!(ISIZE_LOWEST, <<Integer<64> as Format>::Slots as Slots>::MIN);
    assert_eq!(USIZE_HIGHEST, Slot::at(u64::MAX as i128));
}

// FIXME: compiled only by `cargo check --tests --target i686-unknown-linux-gnu` run
// by hand; no gate builds the suite at a 32-bit target, and a test here cannot
// drive that build because the crate refuses `std::process`.
#[cfg(target_pointer_width = "32")]
#[test]
fn on_a_32_bit_target_the_platform_width_points_are_the_32_bit_points() {
    assert_eq!(
        USIZE_HIGHEST,
        <<UFixed<32, 0> as Format>::Slots as Slots>::MAX
    );
    assert_eq!(ISIZE_LOWEST, <<Integer<32> as Format>::Slots as Slots>::MIN);
    assert_eq!(USIZE_HIGHEST, Slot::at(u32::MAX as i128));
}

// FIXME: never compiled. A test harness needs `std` and no 16-bit target has one,
// so this arm stays unbuilt until a harness exists there; the library alias alone
// checks at `msp430-none-elf` with `-Zbuild-std=core`, by hand and in no gate.
#[cfg(target_pointer_width = "16")]
#[test]
fn on_a_16_bit_target_the_platform_width_points_are_the_16_bit_points() {
    assert_eq!(
        USIZE_HIGHEST,
        <<UFixed<16, 0> as Format>::Slots as Slots>::MAX
    );
    assert_eq!(ISIZE_LOWEST, <<Integer<16> as Format>::Slots as Slots>::MIN);
    assert_eq!(USIZE_HIGHEST, Slot::at(u16::MAX as i128));
}
