//! Shared data model for the `Layout::Bitpacked` access-pattern bench.
//!
//! Answers the question `74b_op_checkpoint_eighteen.md` sent to a compute-side
//! dispatch: whether a byte-aligned-slot reading of `Layout::Bitpacked` (every
//! value has an independent byte image, one rounded-up-to-a-byte slot per
//! value, packed contiguously) and a zero-inter-value-padding reading (fields
//! packed to the bit, no slot rounding at all) are one strategy or two, and
//! what each costs a consumer walking a column.
//!
//! Field width is 13 bits throughout, matching file 32's own model
//! (`32_aaltonen_does_identity_lower_well.md` section 4, "the same
//! non-power-of-two shape arvo's own bitfield examples use") and file 73's
//! `probe_3` (`73_arntzen_the_byte_image.md` section 5). Thirteen does not
//! divide eight, which is exactly the property that makes the two readings
//! diverge: a byte-aligned slot for a 13-bit field is 16 bits (2 bytes,
//! 3 padding bits per slot); a zero-padding pack has no padding at all, so
//! consecutive fields straddle byte boundaries at a period of 13 bits.
//!
//! `pack_aligned` / `extract_aligned` implement the byte-aligned-slot reading.
//! `pack_zeropad` / `extract_zeropad` implement the zero-inter-value-padding
//! reading. Both extraction functions are the one transform each reading
//! owns; every variant crate in this bench (`bitpack-aligned-seq`,
//! `bitpack-aligned-rand`, `bitpack-zeropad-seq`, `bitpack-zeropad-rand`)
//! calls the shared function rather than re-deriving the bit arithmetic, so a
//! fix to the extraction shape lands once.
//!
//! `Column<const N: usize>` is the one `Routine` type this bench sweeps,
//! matching the harness's own constraint (`mockspace-bench-macro`'s
//! `#[bench_variant]` requires exactly one const generic parameter on the
//! annotated function, monomorphised once per `sizes = [...]` entry). Its
//! backing arrays are declared at `MAX_N`-sized fixed literal lengths
//! (`MAX_ALIGNED_BYTES`, `MAX_ZEROPAD_BYTES`, both computed once below from
//! `MAX_N` and never as an expression of the generic `N`), because a field
//! length that is itself an expression of a struct's own const generic
//! parameter (`[u8; N * 2]`) needs `generic_const_exprs`, forbidden per
//! `unstable-features.md`. Every method below uses `N` (a plain `usize`
//! value in scope inside the `impl`, not a type-position expression) to
//! bound loops and slice the fixed-size backing arrays down to the bytes a
//! given size actually needs; the unused tail past what a smaller `N`
//! requires is zeroed by `Default` and never read by any in-bounds index.
//! The tradeoff this buys: `Column<256>`'s `Input` struct has the same
//! footprint (`MAX_ALIGNED_BYTES + MAX_ZEROPAD_BYTES + MAX_N*2 + MAX_N*4`
//! bytes, roughly 154KB) as `Column<16384>`'s, which costs something in
//! `build_input`'s zeroing pass, outside the timed region
//! (`mockspace_bench_core::timed!`'s `run { ... }` block is the only part
//! the harness's counter samples), but costs nothing inside the timed
//! extraction loop, which only ever touches indices `0..N`.
//!
//! This crate is bench infrastructure, not shipping arvo source (matching
//! every existing variant crate in this directory): no `#![no_std]`, `std`
//! used freely.

use mockspace_bench_core::Routine;

/// The logical field width every value in this bench carries.
pub const LOGICAL_BITS: u32 = 13;
/// `(1 << LOGICAL_BITS) - 1`, the mask recovering a 13-bit field from a wider read.
pub const MASK13: u32 = (1u32 << LOGICAL_BITS) - 1;

/// The largest column size this bench sweeps. Every `Column<N>` for `N <=
/// MAX_N` uses backing arrays sized for `MAX_N` (see the module doc comment).
pub const MAX_N: usize = 16384;
/// Byte-aligned-slot buffer size at `MAX_N`: two bytes per value, no gap.
pub const MAX_ALIGNED_BYTES: usize = MAX_N * 2;
/// Zero-inter-value-padding buffer size at `MAX_N`: `ceil(MAX_N * 13 / 8)`
/// data bytes plus 4 bytes of read headroom for `extract_zeropad`'s wide
/// unaligned load on the last field. `MAX_N` (16384) is a multiple of 8, so
/// `MAX_N * 13` is a multiple of 8 and the data-byte count has no fractional
/// remainder to round up.
pub const MAX_ZEROPAD_BYTES: usize = (MAX_N * LOGICAL_BITS as usize) / 8 + 4;

/// splitmix64. Deterministic per seed, matching the sibling quantiser-fadd-shared
/// crate's own generator (each shared bench crate in this harness carries its
/// own copy rather than a shared RNG crate; that duplication predates this
/// bench and is not this dispatch's to fix).
struct SplitMix64(u64);
impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

/// The output every variant writes: the wrapping sum of the extracted column,
/// checked in `validate_output` against the ground-truth sum of the logical
/// values `build_input` generated, independent of which packing produced it.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Sum {
    pub value: u64,
}

/// Byte-aligned-slot pack: each field rounds up to the next byte (16 bits for
/// a 13-bit field here) and slots pack contiguously with no gap between
/// slots. `out` must be at least `2 * vals.len()` bytes.
/// Native-typed Dense extract: the carrier is genuinely `u16`, not
/// reconstructed from a byte buffer. `input.logical` already holds exactly
/// this (each value already masked to 13 bits at generation), so no separate
/// pack step exists for this reading; it is what `Layout::Dense` at a native
/// register width already gives for free, the mechanism `extract_aligned`
/// below approximates through a byte buffer for the sake of comparing it to
/// `extract_zeropad` on identical (byte-addressed) infrastructure. Compare
/// the two disassemblies (`75_probes/codegen.rs`) before treating
/// `extract_aligned`'s cost as what Dense actually ships at.
#[inline(always)]
pub fn extract_native(vals: &[u16], i: usize) -> u16 {
    vals[i] & (MASK13 as u16)
}

pub fn pack_aligned(vals: &[u16], out: &mut [u8]) {
    for (i, &v) in vals.iter().enumerate() {
        let b = (v & MASK13 as u16).to_le_bytes();
        out[i * 2] = b[0];
        out[i * 2 + 1] = b[1];
    }
}

/// Byte-aligned-slot extract: one fixed-stride unaligned 2-byte load, mask.
/// The address is `i * 2`, a shift, not a multiply; no cross-slot read is
/// possible because every slot occupies exactly one whole 2-byte span.
#[inline(always)]
pub fn extract_aligned(buf: &[u8], i: usize) -> u16 {
    let off = i * 2;
    let raw = u16::from_le_bytes([buf[off], buf[off + 1]]);
    raw & (MASK13 as u16)
}

/// Zero-inter-value-padding pack: fields pack to the bit, no slot rounding.
/// `out` must be zeroed before this call and at least
/// `ceil(vals.len() * LOGICAL_BITS / 8) + 4` bytes (the tail 4 bytes are read
/// headroom for `extract_zeropad`'s wide load on the last field, never
/// written with real data here beyond what the last field's bits touch).
pub fn pack_zeropad(vals: &[u16], out: &mut [u8]) {
    for (i, &v) in vals.iter().enumerate() {
        let bit_off = i * LOGICAL_BITS as usize;
        let byte_off = bit_off >> 3;
        let bit_shift = (bit_off & 7) as u32;
        let field = (v as u32) & MASK13;
        let w = u32::from_le_bytes([
            out[byte_off],
            out[byte_off + 1],
            out[byte_off + 2],
            out[byte_off + 3],
        ]);
        let w = w | (field << bit_shift);
        let b = w.to_le_bytes();
        out[byte_off] = b[0];
        out[byte_off + 1] = b[1];
        out[byte_off + 2] = b[2];
        out[byte_off + 3] = b[3];
    }
}

/// Zero-inter-value-padding extract: address is `(i * 13) >> 3`, a multiply
/// then a shift, not a shift alone; the bit shift within the wide read is
/// `(i * 13) & 7`, which cycles through all eight values as `i` advances,
/// never landing on the same byte offset twice in a row. A single 4-byte
/// unaligned load always covers the field: the worst-case bit shift is 7,
/// and `7 + 13 = 20 <= 32`, so no second load or OR is needed at this field
/// width. That is a real, load-bearing fact about *this* width; a design
/// that let the field width grow past 19 bits (`32 - 13`) at whatever the
/// wide-read width is would need the two-load-and-OR shape this bench does
/// not exercise.
#[inline(always)]
pub fn extract_zeropad(buf: &[u8], i: usize) -> u16 {
    let bit_off = i * LOGICAL_BITS as usize;
    let byte_off = bit_off >> 3;
    let bit_shift = (bit_off & 7) as u32;
    let w = u32::from_le_bytes([
        buf[byte_off],
        buf[byte_off + 1],
        buf[byte_off + 2],
        buf[byte_off + 3],
    ]);
    ((w >> bit_shift) & MASK13) as u16
}

/// Fisher-Yates over `0..N`, deterministic per seed. Built once in
/// `build_input`, outside the timed region; the sequential variants ignore
/// the result and pay nothing extra for its presence beyond the one build
/// call every size already pays regardless of access pattern.
fn build_permutation<const N: usize>(seed: u64) -> [u32; N] {
    let mut idx = [0u32; N];
    let mut i = 0usize;
    while i < N {
        idx[i] = i as u32;
        i += 1;
    }
    let mut rng = SplitMix64(seed ^ 0xF15F_E5C4_B10C_A11D);
    let mut i = N;
    while i > 1 {
        i -= 1;
        let j = (rng.next() % (i as u64 + 1)) as usize;
        idx.swap(i, j);
    }
    idx
}

/// The one `Routine` type this bench sweeps. See the module doc comment for
/// why the backing arrays are `MAX_N`-sized regardless of `N`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Column<const N: usize> {
    pub aligned: [u8; MAX_ALIGNED_BYTES],
    pub zeropad: [u8; MAX_ZEROPAD_BYTES],
    pub logical: [u16; MAX_N],
    pub perm: [u32; MAX_N],
}

impl<const N: usize> Default for Column<N> {
    fn default() -> Self {
        Column {
            aligned: [0u8; MAX_ALIGNED_BYTES],
            zeropad: [0u8; MAX_ZEROPAD_BYTES],
            logical: [0u16; MAX_N],
            perm: [0u32; MAX_N],
        }
    }
}

impl<const N: usize> Routine for Column<N> {
    type Input = Column<N>;
    type Output = Sum;

    fn build_input(seed: u64) -> Self::Input {
        let mut col = Column::<N>::default();
        let mut rng = SplitMix64(seed ^ 0xB179_ACC0_0001_5EED);
        for i in 0..N {
            col.logical[i] = (rng.next() & MASK13 as u64) as u16;
        }
        let aligned_bytes = N * 2;
        let zeropad_bytes = (N * LOGICAL_BITS as usize) / 8 + 4;
        pack_aligned(&col.logical[..N], &mut col.aligned[..aligned_bytes]);
        pack_zeropad(&col.logical[..N], &mut col.zeropad[..zeropad_bytes]);
        let perm = build_permutation::<N>(seed);
        col.perm[..N].copy_from_slice(&perm);
        col
    }

    fn validate_output(input: &Self::Input, output: &Self::Output) -> Result<(), &'static str> {
        let mut expect: u64 = 0;
        for &v in input.logical[..N].iter() {
            expect = expect.wrapping_add(v as u64);
        }
        if output.value != expect {
            return Err("column sum mismatch: the extraction path produced a \
                 different value stream than the logical ground truth");
        }
        Ok(())
    }

    fn ops_per_call(_input: &Self::Input) -> u64 {
        N as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Cross-checks both extraction paths against the logical ground truth,
    /// every index, every size, 8 seeds each. This is the correctness check
    /// the harness's own `validate_output` also runs per call; this test
    /// exists so `cargo test -p bench-bitpack-shared` catches a packing
    /// defect without needing the full FFI harness in the loop, matching
    /// the sibling quantiser-fadd-shared crate's own `#[cfg(test)]`
    /// discipline.
    fn check_size<const N: usize>() {
        for seed in 0u64..8 {
            let col = <Column<N> as Routine>::build_input(seed);
            for i in 0..N {
                let a = extract_aligned(&col.aligned, i);
                let z = extract_zeropad(&col.zeropad, i);
                let expect = col.logical[i];
                assert_eq!(a, expect, "aligned mismatch at seed {seed} index {i} N={N}");
                assert_eq!(z, expect, "zeropad mismatch at seed {seed} index {i} N={N}");
            }
            // the permutation is a bijection on 0..N: every index appears
            // exactly once.
            let mut seen = std::vec![false; N];
            for &p in col.perm[..N].iter() {
                let p = p as usize;
                assert!(!seen[p], "duplicate permutation index {p} N={N}");
                seen[p] = true;
            }
        }
    }

    #[test]
    fn column256_roundtrips() {
        check_size::<256>();
    }

    #[test]
    fn column4096_roundtrips() {
        check_size::<4096>();
    }

    #[test]
    fn column16384_roundtrips() {
        check_size::<16384>();
    }
}
