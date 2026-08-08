//! The column layout, the input builder, and how a pass is split.
//!
//! `Layout` names the carrier crate's four-region struct without a const
//! parameter, `build_bytes` fills it for a runtime element count, and
//! `slice_bounds` says which elements each thread walks. Nothing here runs
//! inside a timed region except `slice_bounds`, which is called once per slice.

use bench_bitpack_carrier_shared::{
    CarrierColumn, LOGICAL_BITS, MASK13, OFF_D16, OFF_D32, OFF_D64, OFF_PACKED, TOTAL_INPUT_BYTES,
};
use bench_bitpack_plan_shared::pack;

/// The four-carrier layout, named without a const parameter.
///
/// Every field of `CarrierColumn<N>` is sized from `MAX_N` rather than from `N`,
/// so the layout is identical at every instantiation and `CarrierColumn<0>` is
/// the same 125 MiB struct as `CarrierColumn<MAX_N>`. Asserted in the tests
/// below rather than trusted, because the harness casts the raw input buffer to
/// this type and a size disagreement would read the wrong offsets while every
/// timed number still looked ordinary.
pub type Layout = CarrierColumn<0>;

/// The largest thread count the pool supports.
pub const MAX_THREADS: usize = 8;

/// The half-open element range thread `index` walks.
///
/// The last thread takes the remainder, which is zero for every key this bench
/// declares (`KEY_SPLITS` refuses any key where it would not be) but is written
/// out anyway so the function is correct for a caller that has not read the
/// refusal.
#[inline]
pub fn slice_bounds(index: usize, n: usize, threads: usize) -> (usize, usize) {
    let span = n / threads;
    let lo = index * span;
    let hi = if index + 1 == threads { n } else { lo + span };
    (lo, hi)
}

/// splitmix64, matching the carrier crate's own private copy. The duplication is
/// what `build_bytes_equals_the_carrier_crates_builder` pins.
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

/// Build the four-carrier input for a runtime element count.
///
/// The carrier crate's `build_input_bytes` takes its count as a const parameter,
/// which this bench cannot use because the count is decoded from `KEY` at
/// runtime. Reproduced here and pinned by a byte-equality test rather than by
/// inspection.
pub fn build_bytes(n: usize, seed: u64) -> Vec<u8> {
    let mut rng = SplitMix64(seed ^ 0xB179_ACC0_0001_5EED);
    let vals: Vec<u16> = (0..n).map(|_| (rng.next() & MASK13) as u16).collect();

    let mut buf = vec![0u8; TOTAL_INPUT_BYTES];
    for (i, &v) in vals.iter().enumerate() {
        let w = v as u64;
        buf[OFF_D64 + i * 8..OFF_D64 + i * 8 + 8].copy_from_slice(&w.to_le_bytes());
        buf[OFF_D32 + i * 4..OFF_D32 + i * 4 + 4].copy_from_slice(&(w as u32).to_le_bytes());
        buf[OFF_D16 + i * 2..OFF_D16 + i * 2 + 2].copy_from_slice(&v.to_le_bytes());
    }
    let packed_bytes = (n * LOGICAL_BITS) / 8 + 16;
    pack(&vals, &mut buf[OFF_PACKED..OFF_PACKED + packed_bytes]);
    buf
}
