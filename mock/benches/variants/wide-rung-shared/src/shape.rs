//! The three payload shapes the ratified rule forks between, as pure
//! arithmetic on the declared width.
//!
//! `seed/SETTLED_container.md:345-350` assigns a **ragged** payload (sized to
//! the exact bit count) to `Cold` and `Precise`, and a **word-rounded**
//! payload (whole 64-bit limbs) to `Hot` and `Warm`. `15` section 5 records a
//! third shape that is live in the same region: `Hot`'s wide arm at align 16,
//! which pads a 30-byte payload to 32.
//!
//! Every function here is `const` and takes the declared width, so a caller
//! passing a constant `W` folds all of it away. Nothing in this file depends
//! on a container type, which is the point: the shapes are a property of the
//! width, and the loaders in `load.rs` are what turn a shape into machine
//! accesses.

/// Limbs of 64 bits needed to hold `w` bits.
///
/// This is the limb count every arm computes in, regardless of how many
/// bytes its payload occupies. The arms differ in the **stride** between
/// elements, never in the arithmetic width.
pub const fn limbs_of(w: u32) -> usize {
    (w as usize).div_ceil(64)
}

/// Bytes in a **ragged** payload: the exact bit count rounded up to a byte.
pub const fn rag_bytes(w: u32) -> usize {
    (w as usize).div_ceil(8)
}

/// Bytes in a **word-rounded** payload: whole 64-bit limbs.
pub const fn wr_bytes(w: u32) -> usize {
    8 * limbs_of(w)
}

/// Bytes in a word-rounded payload, computed by rounding the ragged size up
/// to a multiple of eight rather than by counting limbs.
///
/// Provably equal to [`wr_bytes`] at every width, since
/// `ceil(ceil(w/8)/8) * 8 == ceil(w/64) * 8`. It exists to give the bench a
/// **noise floor**: the arm built on this function compiles to byte-identical
/// code to the arm built on `wr_bytes`, so the spread between those two arms
/// on a given row is that row's own run-to-run variation, and any difference
/// between two real arms smaller than that gap is not signal.
///
/// `wordround_alias_is_never_a_distinct_stride` asserts the equality across
/// the whole swept width set rather than leaving it as an argument.
pub const fn wr_bytes_alias(w: u32) -> usize {
    rag_bytes(w).div_ceil(8) * 8
}

/// Bytes in a **16-byte-aligned** payload: the ragged size padded to a
/// multiple of sixteen, which is the SSE2 and NEON baseline `15` section 5
/// attributes to `Hot` above the native rungs.
pub const fn a16_bytes(w: u32) -> usize {
    rag_bytes(w).div_ceil(16) * 16
}

/// Mask for the most significant limb of a `w`-bit numeral.
///
/// `u64::MAX` when `w` is a multiple of 64, since `1 << 64` is not
/// representable and the limb is full. This is the one branch in the whole
/// shape module and it folds at every constant width.
pub const fn top_mask(w: u32) -> u64 {
    if w.is_multiple_of(64) {
        u64::MAX
    } else {
        (1u64 << (w % 64)) - 1
    }
}

/// Bytes the widest swept payload occupies, which is what every input region
/// is reserved at.
///
/// A region length that is an expression of the routine's own const generic
/// would need `generic_const_exprs`, which is forbidden
/// (`unstable-features.md`, op 2026-07-28). So every region is declared at
/// this constant and only the prefix a key actually uses is written or read.
/// `bench-warm-container-shared` records the same constraint and the same
/// remedy at its own `Cols`.
pub const MAX_STRIDE: usize = 32;

/// The declared widths this bench sweeps.
///
/// All six are above the 128-bit native rung, which is where
/// `seed/SETTLED_container.md:337-341` puts the fork. Two of them are chosen
/// because the shapes collide there and that collision is a free second
/// control: at `W = 192` the ragged and word-rounded payloads are both 24
/// bytes, and at `W = 256` both are 32, so those two arms must land on top of
/// each other at those widths or something is wrong with the instrument
/// rather than with the design.
///
/// `W = 200` is the numeral the ratified claim was counted at: a 25-byte
/// ragged payload against a 32-byte word-rounded one is exactly the "twenty
/// five bytes against thirty two" in `137b:47-53`.
pub const SWEPT_WIDTHS: [u32; 6] = [129, 160, 192, 200, 232, 256];

/// Small element count. At the widest stride this is 64 KiB, half of this
/// host's 128 KiB L1 data cache, so every arm is cache-resident and the
/// reading is compute-bound. That isolates the instruction half of the trade.
pub const N_SMALL: usize = 2_048;

/// Large element count, chosen so the two arms straddle this host's 12 MiB L2
/// at the ratified numeral. At `W = 200` the word-rounded column is 14.7 MB
/// and the ragged column is 11.5 MB, so the footprint half of the trade is
/// the difference between missing L2 and not.
pub const N_LARGE: usize = 458_752;

#[cfg(test)]
mod tests {
    use super::*;

    /// The control arm is a control. If this ever fails, the noise-floor
    /// reading in the findings is meaningless and the arm has silently become
    /// a fifth container.
    #[test]
    fn wordround_alias_is_never_a_distinct_stride() {
        for w in 1u32..=512 {
            assert_eq!(
                wr_bytes(w),
                wr_bytes_alias(w),
                "the alias stride diverges from the word-rounded stride at W={w}, \
                 so the control arm is measuring a different container and the \
                 noise floor it reports is not a noise floor"
            );
        }
    }

    /// The claim under test, restated as arithmetic. `137b:47-53` says the
    /// numeral it counted has a 25-byte ragged payload and a 32-byte
    /// word-rounded one. If `W = 200` does not produce those two numbers, this
    /// bench is not pointed at the cell the ratified rule was decided on.
    #[test]
    fn w200_is_the_numeral_the_ratified_claim_was_counted_at() {
        assert_eq!(rag_bytes(200), 25);
        assert_eq!(wr_bytes(200), 32);
        assert_eq!(
            wr_bytes(200) - rag_bytes(200),
            7,
            "the ratified seven bytes per value"
        );
    }

    /// The two collision widths are collisions. These are the bench's second
    /// and third noise-floor readings and they are free, so they are asserted
    /// rather than assumed.
    #[test]
    fn ragged_and_wordround_collide_at_exactly_the_multiples_of_sixty_four() {
        for w in SWEPT_WIDTHS {
            let collides = rag_bytes(w) == wr_bytes(w);
            assert_eq!(
                collides,
                w % 64 == 0,
                "at W={w} the ragged payload is {} bytes and the word-rounded one is {}; \
                 they collide exactly when the width is a whole number of limbs",
                rag_bytes(w),
                wr_bytes(w)
            );
        }
    }

    /// No swept payload exceeds the reserved region stride. A width that did
    /// would read past its region and every number from the run would be void.
    #[test]
    fn every_swept_shape_fits_the_reserved_stride() {
        for w in SWEPT_WIDTHS {
            assert!(rag_bytes(w) <= MAX_STRIDE, "ragged at W={w}");
            assert!(wr_bytes(w) <= MAX_STRIDE, "word-rounded at W={w}");
            assert!(a16_bytes(w) <= MAX_STRIDE, "align-16 at W={w}");
        }
    }

    /// The over-reading loader reads whole limbs past the end of a ragged
    /// payload. This pins how much slack the ragged region needs; the region
    /// is declared with `MAX_STRIDE` bytes of it, and this asserts that is
    /// enough at every swept width rather than trusting the arithmetic.
    #[test]
    fn the_overread_slack_is_bounded_by_the_reserved_tail() {
        let mut worst = 0usize;
        for w in SWEPT_WIDTHS {
            let slack = wr_bytes(w) - rag_bytes(w);
            worst = worst.max(slack);
        }
        assert_eq!(worst, 7, "worst-case over-read past a ragged payload");
        assert!(worst <= MAX_STRIDE);
    }

    /// Above the native rungs means above the native rungs.
    #[test]
    fn every_swept_width_is_above_the_widest_native_container() {
        for w in SWEPT_WIDTHS {
            assert!(
                w > 128,
                "W={w} is inside the native rung ladder, where this bench has \
                 nothing to say and the committed container benches already do"
            );
        }
    }

    /// The top-limb mask is the identity exactly on whole-limb widths, and
    /// keeps exactly `w mod 64` bits otherwise. Getting this wrong makes every
    /// arm agree on a wrong value, which the cross-variant check cannot see.
    #[test]
    fn the_top_limb_mask_keeps_exactly_the_bits_the_width_declares() {
        assert_eq!(top_mask(192), u64::MAX);
        assert_eq!(top_mask(256), u64::MAX);
        assert_eq!(top_mask(129), 1);
        assert_eq!(top_mask(160), (1u64 << 32) - 1);
        assert_eq!(top_mask(200), (1u64 << 8) - 1);
        assert_eq!(top_mask(232), (1u64 << 40) - 1);
        for w in SWEPT_WIDTHS {
            let kept = top_mask(w).count_ones() as usize;
            let expect = if w % 64 == 0 { 64 } else { (w % 64) as usize };
            assert_eq!(kept, expect, "at W={w}");
            assert_eq!(
                (limbs_of(w) - 1) * 64 + expect,
                w as usize,
                "the limb count and the top mask must together account for exactly W bits, at W={w}"
            );
        }
    }
}
