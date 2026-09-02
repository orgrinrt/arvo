//! The input: one logical column of wide numerals, stored three times, once
//! per payload shape.
//!
//! Each arm reads a contiguous array at its own stride, so the footprint
//! difference between the shapes is a real difference in bytes touched rather
//! than a conversion inserted at the load. That is the whole reason the
//! regions are separate: a single shared region read at three pitches would
//! measure three walks over the same working set and the footprint half of the
//! trade would vanish.
//!
//! Every region is reserved at `MAX_STRIDE` for `N_LARGE` elements regardless
//! of which key a monomorphisation represents, because a region length that is
//! an expression of the routine's own const generic needs
//! `generic_const_exprs`, which is forbidden. Only the prefix a key uses is
//! ever written or read.

use crate::shape::{MAX_STRIDE, N_LARGE, N_SMALL};
use crate::wide::mask_w_dyn;

/// Bytes reserved per region, including the tail the over-reading loader
/// needs at the final element. `MAX_STRIDE` of slack is far more than the
/// seven bytes `the_overread_slack_is_bounded_by_the_reserved_tail` pins, and
/// it keeps every region start a multiple of `MAX_STRIDE`.
pub const REGION_BYTES: usize = N_LARGE * MAX_STRIDE + MAX_STRIDE;

/// The same logical column at three strides.
///
/// `align(16)` so that the word-rounded and align-16 regions are aligned for
/// the direct 64-bit loads their arms use. `REGION_BYTES` is a multiple of 32,
/// so every region start inherits the alignment of the whole. Whether the
/// pointer the harness hands over is itself aligned is checked at run time by
/// `assert_aligned` rather than assumed: a silently misaligned direct load is
/// undefined behaviour that this target happens to tolerate, which is exactly
/// the kind of thing that reads as a valid measurement.
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Column {
    pub ragged: [u8; REGION_BYTES],
    pub wordround: [u8; REGION_BYTES],
    pub align16: [u8; REGION_BYTES],
}

/// Fails loudly if the harness handed over a buffer the direct loads cannot
/// legally use.
#[inline(always)]
pub fn assert_aligned(col: &Column) {
    let addr = col as *const Column as usize;
    assert!(
        addr.is_multiple_of(16),
        "harness handed an input buffer at {addr:#x}, which is not 16-aligned; \
         the aligned 64-bit loads in the word-rounded and align-16 arms would be \
         unsound and every number from this run is void"
    );
}

/// Total bytes a built input occupies.
pub const TOTAL_INPUT_BYTES: usize = 3 * REGION_BYTES;

/// Declared width of a key.
pub const fn key_w(key: usize) -> u32 {
    (key / 1_000) as u32
}
/// Element-count class: 0 is [`N_SMALL`], 1 is [`N_LARGE`].
pub const fn key_nc(key: usize) -> usize {
    (key / 100) % 10
}
/// Operations applied per element before the accumulation.
pub const fn key_d(key: usize) -> usize {
    key % 100
}
/// Elements a key sweeps.
pub const fn key_n(key: usize) -> usize {
    if key_nc(key) == 0 {
        N_SMALL
    } else {
        N_LARGE
    }
}

struct SplitMix64(u64);
impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

/// Writes the low `bytes` bytes of a four-limb value, little-endian.
fn store(buf: &mut [u8], off: usize, bytes: usize, v: [u64; 4]) {
    let mut le = [0u8; 32];
    for i in 0..4 {
        le[i * 8..i * 8 + 8].copy_from_slice(&v[i].to_le_bytes());
    }
    buf[off..off + bytes].copy_from_slice(&le[..bytes]);
}

/// Reads `bytes` bytes back as a four-limb value.
pub fn load_bytes(buf: &[u8], off: usize, bytes: usize) -> [u64; 4] {
    let mut le = [0u8; 32];
    le[..bytes].copy_from_slice(&buf[off..off + bytes]);
    let mut v = [0u64; 4];
    for i in 0..4 {
        v[i] = u64::from_le_bytes(le[i * 8..i * 8 + 8].try_into().unwrap());
    }
    v
}

/// Builds one input buffer for a key.
///
/// Free rather than a method on the routine so the tests can sweep every
/// declared key at run time and the bench path and the tests construct input
/// through the same code.
pub fn build_bytes(key: usize, seed: u64) -> std::vec::Vec<u8> {
    let w = key_w(key);
    let n = key_n(key);
    let rag = crate::shape::rag_bytes(w);
    let wr = crate::shape::wr_bytes(w);
    let a16 = crate::shape::a16_bytes(w);

    let mut rng = SplitMix64(seed ^ 0x00C0_FFEE_0BAD_F00D);
    let mut buf = std::vec![0u8; TOTAL_INPUT_BYTES];
    for i in 0..n {
        let raw = [rng.next(), rng.next(), rng.next(), rng.next()];
        let v = mask_w_dyn(raw, w);
        store(&mut buf, i * rag, rag, v);
        store(&mut buf, REGION_BYTES + i * wr, wr, v);
        store(&mut buf, 2 * REGION_BYTES + i * a16, a16, v);
    }
    buf
}

/// Decodes the logical column back out of one region, for validation.
pub fn decode(
    buf: &[u8],
    region: usize,
    n: usize,
    stride: usize,
    w: u32,
) -> std::vec::Vec<[u64; 4]> {
    (0..n)
        .map(|i| {
            let v = load_bytes(buf, region * REGION_BYTES + i * stride, stride);
            mask_w_dyn(v, w)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::{a16_bytes, limbs_of, rag_bytes, wr_bytes, SWEPT_WIDTHS};

    /// The three regions must hold the identical logical column. If they do
    /// not, the arms were fed different inputs and no comparison between them
    /// means anything. This is the check the whole bench rests on.
    #[test]
    fn the_three_regions_hold_the_same_logical_column() {
        for w in SWEPT_WIDTHS {
            let key = w as usize * 1000 + 3;
            let n = key_n(key);
            let buf = build_bytes(key, 0xC0FFEE);
            let a = decode(&buf, 0, n, rag_bytes(w), w);
            let b = decode(&buf, 1, n, wr_bytes(w), w);
            let c = decode(&buf, 2, n, a16_bytes(w), w);
            assert_eq!(a, b, "ragged and word-rounded regions differ at W={w}");
            assert_eq!(a, c, "ragged and align-16 regions differ at W={w}");
        }
    }

    /// Every stored value is inside the declared width. A value with a stray
    /// high bit would be read differently by arms whose payloads have
    /// different numbers of spare bits, and the disagreement would look like
    /// an arm bug.
    #[test]
    fn every_stored_value_is_inside_the_declared_width() {
        for w in SWEPT_WIDTHS {
            let key = w as usize * 1000 + 3;
            let n = key_n(key);
            let buf = build_bytes(key, 0xBEEF);
            let vals = decode(&buf, 0, n, rag_bytes(w), w);
            let limbs = limbs_of(w);
            for (i, v) in vals.iter().enumerate() {
                assert_eq!(
                    v[limbs - 1] & !crate::shape::top_mask(w),
                    0,
                    "element {i} at W={w} carries a bit above the declared width"
                );
                for l in limbs..4 {
                    assert_eq!(
                        v[l], 0,
                        "element {i} at W={w} has a non-zero limb above the width"
                    );
                }
            }
        }
    }

    /// Different seeds give different columns. A builder that ignored its seed
    /// would make the harness's hundred-seed validation pass a hundred copies
    /// of one check.
    #[test]
    fn the_builder_uses_its_seed() {
        for w in SWEPT_WIDTHS {
            let key = w as usize * 1000 + 3;
            let a = build_bytes(key, 1);
            let b = build_bytes(key, 2);
            assert_ne!(a, b, "the column is seed-independent at W={w}");
        }
    }

    /// The reserved region is large enough for the largest key. If this ever
    /// fails the builder writes out of bounds.
    #[test]
    fn the_reserved_region_holds_the_largest_key() {
        for w in SWEPT_WIDTHS {
            assert!(N_LARGE * a16_bytes(w) + MAX_STRIDE <= REGION_BYTES, "W={w}");
            assert!(N_LARGE * wr_bytes(w) + MAX_STRIDE <= REGION_BYTES, "W={w}");
            assert!(N_LARGE * rag_bytes(w) + MAX_STRIDE <= REGION_BYTES, "W={w}");
        }
    }

    /// Key encoding round-trips over the whole declared matrix.
    #[test]
    fn key_encoding_round_trips() {
        for w in SWEPT_WIDTHS {
            for nc in 0..=1usize {
                for d in [1usize, 2, 3, 4, 8] {
                    let key = w as usize * 1000 + nc * 100 + d;
                    assert_eq!(key_w(key), w);
                    assert_eq!(key_nc(key), nc);
                    assert_eq!(key_d(key), d);
                    assert_eq!(key_n(key), if nc == 0 { N_SMALL } else { N_LARGE });
                }
            }
        }
    }
}
