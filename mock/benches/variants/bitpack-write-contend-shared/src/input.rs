//! The column layout, the input builder, and where each thread's slice ends.
//!
//! Three regions: a read-only `vals` truth array, a `dense_out` scratch region
//! sized for a `u16` write, and a `packed_out` scratch region for a 13-bit
//! packed write. Every arm reads the same `vals` and writes its own scratch
//! region, so a decode of `dense_out` or `packed_out` after the pass is the
//! correctness check: it must reproduce `vals` exactly, and a race that drops
//! or corrupts a write shows up as a decoded sum that disagrees with the
//! ground truth computed straight from `vals`.

use bench_bitpack_plan_shared::{LOGICAL_BITS, MASK13};

/// Elements the layout is sized for. Kept well below the carrier crate's
/// 8,388,608 because this bench's question is the write hazard at a thread
/// boundary, not a bandwidth ceiling a much larger sweep would locate; `26` and
/// `27` already own that question for reads.
pub const MAX_N: usize = 4_194_304;

pub const VALS_BYTES: usize = MAX_N * 2;
pub const DENSE_OUT_BYTES: usize = MAX_N * 2;
/// Packed bytes at `MAX_N`, plus 16 bytes of read headroom for the widest
/// window a decode's last lane can open, matching the carrier crate's own
/// headroom constant.
pub const PACKED_OUT_BYTES: usize = (MAX_N * LOGICAL_BITS) / 8 + 16;

pub const OFF_VALS: usize = 0;
pub const OFF_DENSE_OUT: usize = OFF_VALS + VALS_BYTES;
pub const OFF_PACKED_OUT: usize = OFF_DENSE_OUT + DENSE_OUT_BYTES;
pub const TOTAL_BYTES: usize = OFF_PACKED_OUT + PACKED_OUT_BYTES;

/// The three regions, named without a const parameter so one type serves every
/// `KEY` the way `bitpack-contend-shared::Layout` does for the read bench.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Layout {
    pub vals: [u16; MAX_N],
    pub dense_out: [u16; MAX_N],
    pub packed_out: [u8; PACKED_OUT_BYTES],
}

/// The largest thread count the pool supports.
pub const MAX_THREADS: usize = 8;

/// The half-open element range thread `index` walks. Identical shape to the
/// read bench's `slice_bounds`: the last thread takes the remainder, and
/// nothing here requires `n` to be a multiple of `threads` or of the packed
/// period. Whether a given `(n, threads)` pair lands each internal boundary on
/// a period boundary is exactly the property the "safe" and "race" sections of
/// this bench choose in opposite directions, on purpose.
#[inline]
pub fn slice_bounds(index: usize, n: usize, threads: usize) -> (usize, usize) {
    let span = n / threads;
    let lo = index * span;
    let hi = if index + 1 == threads { n } else { lo + span };
    (lo, hi)
}

/// splitmix64, matching every sibling bench in this directory so seeds carry
/// the same value stream across files that compare against each other.
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

/// Build the input for a runtime element count. `vals` is filled with random
/// 13-bit values; both scratch regions are zeroed, which is what the OR-based
/// packed encoder requires and what every write arm assumes on entry.
pub fn build_bytes(n: usize, seed: u64) -> Vec<u8> {
    let mut rng = SplitMix64(seed ^ 0xB179_ACC0_0002_5EED);
    let mut buf = vec![0u8; TOTAL_BYTES];
    for i in 0..n {
        let v = (rng.next() & MASK13) as u16;
        buf[OFF_VALS + i * 2..OFF_VALS + i * 2 + 2].copy_from_slice(&v.to_le_bytes());
    }
    buf
}

/// Whether the byte containing the split point at element index `at` (bit
/// offset `at * LOGICAL_BITS`) is shared between the element ending there and
/// the element starting there. `at == 0` and `at == n` are never shared: there
/// is no element on the far side of either edge.
#[inline]
pub fn split_is_guarded(at: usize, n: usize) -> bool {
    at != 0 && at != n && !(at * LOGICAL_BITS).is_multiple_of(8)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The layout is exactly the byte count the offsets predict, which is what
    /// the harness's cast of the raw input buffer relies on.
    #[test]
    fn layout_size_matches_total_bytes() {
        assert_eq!(core::mem::size_of::<Layout>(), TOTAL_BYTES);
    }

    /// The sizes chosen for the "safe" section land every internal boundary on
    /// a period boundary (period 8 at width 13), and the sizes chosen for the
    /// "race" section do not. Asserted here rather than trusted from hand
    /// arithmetic, because a size that was meant to be misaligned and turned
    /// out aligned would silently turn the race arm into a second safe arm and
    /// the whole demonstration into a no-op.
    #[test]
    fn chosen_sizes_land_where_the_bench_needs_them_to() {
        let safe_sizes = [65536usize, 2_097_152usize];
        for &n in &safe_sizes {
            for &t in &[2usize, 4usize] {
                let (_, hi0) = slice_bounds(0, n, t);
                assert!(
                    !split_is_guarded(hi0, n),
                    "safe size n={n} t={t} produced a guarded boundary at {hi0}, \
                     which was meant to be period-aligned"
                );
            }
        }

        let race_sizes = [65534usize, 2_097_150usize];
        for &n in &race_sizes {
            for &t in &[2usize, 4usize] {
                let (_, hi0) = slice_bounds(0, n, t);
                assert!(
                    split_is_guarded(hi0, n),
                    "race size n={n} t={t} produced an unguarded boundary at {hi0}, \
                     which was meant to be misaligned"
                );
            }
        }
    }
}
