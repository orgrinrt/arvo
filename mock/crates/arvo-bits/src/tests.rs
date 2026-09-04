//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The laws `DESIGN.md.tmpl` names, each pinned at the width chosen to exercise
//! it rather than at a width chosen for convenience.
//!
//! There is exactly one generic code path in this crate, unlike
//! `arvo_format::slots`'s per-width `admit_widths!` macro, so the distinct
//! behaviours to cover are the boundary at `N = 1`, the special-cased boundary
//! at `N = 64` where the mask formula takes the shift-overflow case explicitly,
//! and the ordinary interior case. The widths below are chosen to hit those
//! three and nothing else, plus one exhaustive sweep at a width cheap enough to
//! run in full.

use crate::Bits;

/// Construction at the narrow boundary never rejects; it wraps.
///
/// `N = 1` is the smallest admitted width, where every value above the single
/// bit gets dropped, including the value two, which lands on the same result
/// as zero.
#[test]
fn construction_wraps_rather_than_rejects_at_the_narrow_boundary() {
    assert_eq!(Bits::<1>::masked(0).raw(), 0);
    assert_eq!(Bits::<1>::masked(1).raw(), 1);
    assert_eq!(Bits::<1>::masked(2).raw(), Bits::<1>::masked(0).raw());
}

/// Construction at the wide boundary exercises the `MASK` special case.
///
/// `N = 64` is the one width where `(1u64 << N) - 1` would overflow the shift,
/// which is why the mask computation takes it as its own branch rather than
/// trusting the general formula. `u64::MAX` has to round-trip exactly, or the
/// special case is wrong.
#[test]
fn construction_at_the_wide_boundary_keeps_every_bit() {
    assert_eq!(Bits::<64>::masked(u64::MAX).raw(), u64::MAX);
}

/// Round-trip at a width in the interior, the obligation's own motivating
/// width, for a value already within `N` bits and for one that is not.
#[test]
fn masked_and_raw_round_trip_within_the_declared_width() {
    let within: u64 = (1 << 27) | 0x0AB_CDEF;
    assert_eq!(Bits::<28>::masked(within).raw(), within);

    let above: u64 = within | (1 << 30);
    assert_eq!(Bits::<28>::masked(above).raw(), above & 0x0FFF_FFFF);
}

/// An exhaustive sweep at `N = 8`, the whole value space at that width and
/// cheap enough to run in full rather than sampled.
#[test]
fn every_byte_round_trips_exactly_at_eight_bits() {
    for raw in 0u64 ..= u64::from(u8::MAX) {
        assert_eq!(Bits::<8>::masked(raw).raw(), raw);
    }
}

/// Widening never drops a bit, because the wider mask is a strict superset of
/// the narrower one.
#[test]
fn cast_widening_drops_nothing() {
    let x: u64 = (1 << 27) | 0x0AB_CDEF;
    assert_eq!(Bits::<28>::masked(x).cast::<32>().raw(), x);
}

/// Narrowing masks rather than refusing, dropping exactly the bits above the
/// new width.
#[test]
fn cast_narrowing_masks_the_dropped_bits() {
    let y: u64 = (1 << 30) | 0x0AB_CDEF;
    assert_eq!(Bits::<32>::masked(y).cast::<28>().raw(), y & 0x0FFF_FFFF);
}

/// `M == N` is not a special-cased no-op that skips masking; it is `masked`
/// applied to the same bound, including for a value with bits set above the
/// mask that a shortcut identity path would have let through unmasked.
#[test]
fn cast_at_the_same_width_still_masks() {
    let x_above_mask: u64 = (1 << 30) | 0x0AB_CDEF;
    assert_eq!(
        Bits::<28>::masked(x_above_mask).cast::<28>().raw(),
        x_above_mask & 0x0FFF_FFFF
    );
}

/// The derived `PartialEq` reads the wrapped value rather than being vacuous.
#[test]
fn derived_equality_reads_the_wrapped_value() {
    assert_eq!(Bits::<8>::masked(3), Bits::<8>::masked(3));
    assert_ne!(Bits::<8>::masked(3), Bits::<8>::masked(4));
}
