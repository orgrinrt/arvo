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

/// The mask at width `N`, built by setting one bit at a time.
///
/// An independent oracle rather than a second copy of `MASK`. The shipped
/// constant is a closed form with a special case at the top width; this folds
/// the bits in one at a time and has no special case, so the two agreeing at a
/// width is evidence rather than a restatement.
const fn mask_by_folding(n: u32) -> u64 {
    let mut m = 0u64;
    let mut i = 0u32;
    while i < n {
        m |= 1u64 << i;
        i += 1;
    }
    m
}

/// The oracle is not vacuous: it disagrees with an off-by-one mask at every
/// admitted width.
///
/// Without this the sweeps below would pass against a broken oracle exactly as
/// they pass against a correct one, and a sweep whose oracle cannot be wrong is
/// a sweep that establishes nothing.
#[test]
fn the_folded_oracle_rejects_an_off_by_one_mask() {
    for n in 1u32 ..= 63 {
        assert_ne!(mask_by_folding(n), mask_by_folding(n + 1), "at N = {n}");
    }
    assert_ne!(mask_by_folding(1), 0);
    assert_eq!(mask_by_folding(64), u64::MAX);
}

/// Every admitted width, not a sample of them.
///
/// The module header above says there is one generic code path and picks four
/// widths on that reasoning. `lib.rs` disagrees with it: `MASK` is
/// `if N == 64 { .. } else { .. }`, which is two paths, and the design says so
/// too. So the widths were a sample after all, and `N` runs over sixty-four
/// values, which is enumerable. Each arm asserts against the folded oracle
/// rather than against the shipped constant.
macro_rules! every_width {
    ($($n:literal),* $(,)?) => {
        /// Construction masks to exactly the folded oracle's bits, at every
        /// admitted width, for the all-ones input.
        #[test]
        fn construction_masks_to_the_oracle_at_every_admitted_width() {
            $(
                assert_eq!(
                    Bits::<$n>::masked(u64::MAX).raw(),
                    mask_by_folding($n),
                    "at N = {}", $n
                );
            )*
        }

        /// The drop law at every admitted width: what comes back is the input
        /// with the bits above `N` removed, and nothing else changed.
        #[test]
        fn construction_drops_exactly_the_bits_above_the_width_everywhere() {
            // A local rather than a const, because
            // `a-contract-coordinate-is-not-a-host-primitive` reads every file in
            // this crate including this one, and it is right to: a const here is a
            // position an outside implementor writes too. These are raw host inputs
            // being fed in, which is the one thing `masked` exists to take, so they
            // are a value in a function rather than a coordinate on a contract.
            let probes: [u64; 8] = [
                0,
                1,
                u64::MAX,
                u64::MAX - 1,
                0x5555_5555_5555_5555,
                0xAAAA_AAAA_AAAA_AAAA,
                0x8000_0000_0000_0000,
                0x0123_4567_89AB_CDEF,
            ];
            $(
                for raw in probes {
                    assert_eq!(
                        Bits::<$n>::masked(raw).raw(),
                        raw & mask_by_folding($n),
                        "at N = {}, raw = {:#x}", $n, raw
                    );
                }
            )*
        }

        /// Idempotence at every admitted width: masking a value that is
        /// already within the width changes nothing.
        #[test]
        fn masking_an_already_narrow_value_is_the_identity_everywhere() {
            $(
                for raw in [0u64, 1, mask_by_folding($n), mask_by_folding($n) >> 1] {
                    assert_eq!(Bits::<$n>::masked(raw).raw(), raw, "at N = {}", $n);
                }
            )*
        }

        /// A cast to every admitted width from every admitted width, all four
        /// thousand and ninety-six pairs, against the oracle. Widening keeps
        /// every bit and narrowing drops exactly those above the target, and
        /// both directions are the one expression `raw & mask(M)`.
        #[test]
        fn cast_lands_on_the_oracle_for_every_pair_of_widths() {
            $( cast_from::<$n>(); )*
        }
    };
}

/// One row of the cast matrix: from `N` to every admitted `M`.
fn cast_from<const N: u32>() {
    macro_rules! to_every_width {
        ($($m:literal),* $(,)?) => {
            let start = Bits::<N>::masked(0x0123_4567_89AB_CDEF);
            $(
                assert_eq!(
                    start.cast::<$m>().raw(),
                    start.raw() & mask_by_folding($m),
                    "from N = {} to M = {}", N, $m
                );
            )*
        };
    }
    to_every_width!(
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
    );
}

every_width!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
);
