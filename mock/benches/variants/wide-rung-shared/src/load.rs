//! The five loaders. This is the entire difference between the five arms.
//!
//! Each loader turns a payload shape into machine accesses: it says how many
//! bytes an element occupies (its stride, which is the footprint half of the
//! trade) and how the limbs are read out of them (which is the instruction
//! half). Everything downstream, the arithmetic, the projection, the
//! accumulation and the observed answer, is shared.
//!
//! The arms are competitors rather than a strawman set. Each is what someone
//! would actually write given the payload shape the design assigned:
//!
//! `Ragged` is the byte-exact payload read the obvious safe way: whole
//! unaligned limbs while there are whole limbs left, then the remaining one to
//! seven bytes assembled from the largest power-of-two loads that fit. This is
//! what the ragged shape costs if you write it without tricks.
//!
//! `RaggedOverread` is the same payload read the way someone who has done this
//! before writes it: every limb as one unaligned 64-bit load, with the last
//! load reaching up to seven bytes past the element into its neighbour, and
//! the top limb masked. The mask is required anyway, so the over-read is free
//! once the array carries a tail. It is here because if the ragged shape's
//! whole cost is the partial-word tail, this arm removes it, and the ratified
//! three-instruction figure is then a fact about an implementation rather than
//! about the shape.
//!
//! `WordRound` is the shipped rule for `Hot` and `Warm`: whole 64-bit limbs,
//! naturally aligned, read directly.
//!
//! `WordRoundAlias` computes the identical stride from a different expression
//! and is the **noise floor**. See `shape::wr_bytes_alias`.
//!
//! `Align16` is the SSE2 and NEON baseline `15` section 5 attributes to `Hot`
//! above the native rungs: the ragged size padded to a multiple of sixteen.
//! At three of the six swept widths it coincides with `WordRound`, which makes
//! it a second independent noise-floor reading at those widths and a real
//! third container at the other three.

use crate::shape::{a16_bytes, limbs_of, rag_bytes, top_mask, wr_bytes, wr_bytes_alias};

/// A payload shape, as a stride plus a way to read limbs out of it.
///
/// Every method takes the declared width as a const generic so a caller with a
/// constant width folds the limb count, the tail size and the mask away. The
/// timed path never branches on a width.
pub trait Loader: Copy + 'static {
    /// Name, for the tests and the arm table. Never read on the timed path.
    const NAME: &'static str;

    /// Bytes between consecutive elements.
    fn stride(w: u32) -> usize;

    /// Reads element `i`.
    ///
    /// # Safety
    ///
    /// `base` must point at a region holding at least
    /// `i * stride(W) + stride(W)` initialised bytes, plus, for loaders that
    /// over-read, `wr_bytes(W) - rag_bytes(W)` further readable bytes. The
    /// region layout in `column.rs` provides `MAX_STRIDE` bytes of tail for
    /// exactly this, and `the_overread_slack_is_bounded_by_the_reserved_tail`
    /// pins the requirement.
    unsafe fn load<const W: u32>(base: *const u8, i: usize) -> [u64; 4];
}

/// Byte-exact payload, safe partial-word tail.
#[derive(Clone, Copy)]
pub struct Ragged;

impl Loader for Ragged {
    const NAME: &'static str = "ragged";

    #[inline(always)]
    fn stride(w: u32) -> usize {
        rag_bytes(w)
    }

    #[inline(always)]
    unsafe fn load<const W: u32>(base: *const u8, i: usize) -> [u64; 4] {
        let rag = rag_bytes(W);
        let full = rag / 8;
        let tail = rag % 8;
        let p = unsafe { base.add(i * rag) };
        let mut out = [0u64; 4];
        let mut j = 0usize;
        while j < full {
            out[j] = unsafe { (p.add(j * 8) as *const u64).read_unaligned() };
            j += 1;
        }
        if tail > 0 {
            let q = unsafe { p.add(full * 8) };
            let mut acc = 0u64;
            let mut off = 0usize;
            let mut rem = tail;
            if rem >= 4 {
                acc |= (unsafe { (q.add(off) as *const u32).read_unaligned() } as u64) << (off * 8);
                off += 4;
                rem -= 4;
            }
            if rem >= 2 {
                acc |= (unsafe { (q.add(off) as *const u16).read_unaligned() } as u64) << (off * 8);
                off += 2;
                rem -= 2;
            }
            if rem >= 1 {
                acc |= (unsafe { q.add(off).read() } as u64) << (off * 8);
            }
            out[full] = acc;
        }
        out
    }
}

/// Byte-exact payload, whole-limb loads that reach into the neighbour.
#[derive(Clone, Copy)]
pub struct RaggedOverread;

impl Loader for RaggedOverread {
    const NAME: &'static str = "ragged-overread";

    #[inline(always)]
    fn stride(w: u32) -> usize {
        rag_bytes(w)
    }

    #[inline(always)]
    unsafe fn load<const W: u32>(base: *const u8, i: usize) -> [u64; 4] {
        let n = limbs_of(W);
        let p = unsafe { base.add(i * rag_bytes(W)) };
        let mut out = [0u64; 4];
        let mut j = 0usize;
        while j < n {
            out[j] = unsafe { (p.add(j * 8) as *const u64).read_unaligned() };
            j += 1;
        }
        // The final load may have taken up to seven bytes belonging to the
        // next element. They sit above bit `W mod 64` of the top limb and the
        // projection removes them, which the ragged shape requires anyway.
        out[n - 1] &= top_mask(W);
        out
    }
}

/// Whole 64-bit limbs, aligned.
#[derive(Clone, Copy)]
pub struct WordRound;

impl Loader for WordRound {
    const NAME: &'static str = "wordround";

    #[inline(always)]
    fn stride(w: u32) -> usize {
        wr_bytes(w)
    }

    #[inline(always)]
    unsafe fn load<const W: u32>(base: *const u8, i: usize) -> [u64; 4] {
        let n = limbs_of(W);
        let p = unsafe { base.add(i * wr_bytes(W)) } as *const u64;
        let mut out = [0u64; 4];
        let mut j = 0usize;
        while j < n {
            out[j] = unsafe { p.add(j).read() };
            j += 1;
        }
        out
    }
}

/// The control. Identical stride, computed differently.
#[derive(Clone, Copy)]
pub struct WordRoundAlias;

impl Loader for WordRoundAlias {
    const NAME: &'static str = "wordround-alias";

    #[inline(always)]
    fn stride(w: u32) -> usize {
        wr_bytes_alias(w)
    }

    #[inline(always)]
    unsafe fn load<const W: u32>(base: *const u8, i: usize) -> [u64; 4] {
        let n = limbs_of(W);
        let p = unsafe { base.add(i * wr_bytes_alias(W)) } as *const u64;
        let mut out = [0u64; 4];
        let mut j = 0usize;
        while j < n {
            out[j] = unsafe { p.add(j).read() };
            j += 1;
        }
        out
    }
}

/// Ragged size padded to sixteen bytes: the vector-aligned baseline.
#[derive(Clone, Copy)]
pub struct Align16;

impl Loader for Align16 {
    const NAME: &'static str = "align16";

    #[inline(always)]
    fn stride(w: u32) -> usize {
        a16_bytes(w)
    }

    #[inline(always)]
    unsafe fn load<const W: u32>(base: *const u8, i: usize) -> [u64; 4] {
        let n = limbs_of(W);
        let p = unsafe { base.add(i * a16_bytes(W)) } as *const u64;
        let mut out = [0u64; 4];
        let mut j = 0usize;
        while j < n {
            out[j] = unsafe { p.add(j).read() };
            j += 1;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::column::{build_bytes, Column};
    use crate::shape::SWEPT_WIDTHS;

    /// The strides the arms declare are the strides the shape module defines.
    /// A loader that disagreed would read its own region at the wrong pitch
    /// and every arm would still agree, because they would all be reading the
    /// same wrong thing from their own region.
    #[test]
    fn every_loader_declares_the_stride_its_shape_defines() {
        for w in SWEPT_WIDTHS {
            assert_eq!(Ragged::stride(w), rag_bytes(w));
            assert_eq!(RaggedOverread::stride(w), rag_bytes(w));
            assert_eq!(WordRound::stride(w), wr_bytes(w));
            assert_eq!(WordRoundAlias::stride(w), wr_bytes(w));
            assert_eq!(Align16::stride(w), a16_bytes(w));
        }
    }

    /// The load is the only difference between the arms, so if two loaders
    /// ever return different limbs for the same element, the comparison
    /// between their arms is meaningless. Swept over every width and the first
    /// elements of a real built column.
    #[test]
    fn every_loader_recovers_the_same_value_from_its_own_region() {
        for w in SWEPT_WIDTHS {
            let key = w as usize * 1000 + 3;
            let bytes = build_bytes(key, 0xFEED);
            let col = unsafe { &*(bytes.as_ptr() as *const Column) };
            macro_rules! sweep {
                ($($wl:literal),*) => {$(
                    if w == $wl {
                        for i in 0 .. 256usize {
                            let a = unsafe { Ragged::load::<$wl>(col.ragged.as_ptr(), i) };
                            let b = unsafe { RaggedOverread::load::<$wl>(col.ragged.as_ptr(), i) };
                            let c = unsafe { WordRound::load::<$wl>(col.wordround.as_ptr(), i) };
                            let d = unsafe { WordRoundAlias::load::<$wl>(col.wordround.as_ptr(), i) };
                            let e = unsafe { Align16::load::<$wl>(col.align16.as_ptr(), i) };
                            assert_eq!(a, b, "ragged and its over-reading form disagree at W={} i={}", $wl, i);
                            assert_eq!(a, c, "ragged and word-rounded disagree at W={} i={}", $wl, i);
                            assert_eq!(c, d, "the control does not alias its arm at W={} i={}", $wl, i);
                            assert_eq!(a, e, "ragged and align-16 disagree at W={} i={}", $wl, i);
                        }
                    }
                )*};
            }
            sweep!(129, 160, 192, 200, 232, 256);
        }
    }

    /// The over-reading loader is the one that can read past an element, so
    /// the last element is where it breaks if the region has no tail. Asserted
    /// at the true final index rather than at a comfortable one, because a
    /// test that only reads the first 256 elements is a test that never enters
    /// the path that fails.
    #[test]
    fn the_overreading_loader_is_correct_at_the_final_element() {
        for w in SWEPT_WIDTHS {
            let key = w as usize * 1000 + 3;
            let n = crate::shape::N_SMALL;
            let bytes = build_bytes(key, 0x5EED);
            let col = unsafe { &*(bytes.as_ptr() as *const Column) };
            macro_rules! sweep {
                ($($wl:literal),*) => {$(
                    if w == $wl {
                        let i = n - 1;
                        let a = unsafe { Ragged::load::<$wl>(col.ragged.as_ptr(), i) };
                        let b = unsafe { RaggedOverread::load::<$wl>(col.ragged.as_ptr(), i) };
                        assert_eq!(
                            a, b,
                            "at the final element of a W={} column the over-reading loader \
                             picked up bytes the projection did not remove", $wl
                        );
                    }
                )*};
            }
            sweep!(129, 160, 192, 200, 232, 256);
        }
    }
}
