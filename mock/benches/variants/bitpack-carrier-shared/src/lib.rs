//! Shared data model for the carrier-width bench: prices packing against the
//! width of the carrier a consumer would otherwise have reached for.
//!
//! ## The variable every prior bitpack bench held fixed
//!
//! Six bitpack bench families are already committed in this directory. Every
//! one of them compares a packed 13-bit column against a **`u16`** dense
//! column, because `u16` is what `bitpack-shared`, `bitpack-plan-shared` and
//! `bitpack-footprint-shared` all declare their dense region as. `u16` is the
//! tightest native carrier a 13-bit value can have, so those benches measure
//! packing at its **least** favourable footprint ratio: 1.625 bytes against
//! 2, a saving of 0.375 bytes per element, under 19 percent.
//!
//! That is not the ratio the substrate's own stated case turns on. The claim
//! is that a consumer would otherwise have reached for a wider primitive, and
//! that the bits saved compound across an entity count. Against `u32` the
//! saving is 2.375 bytes per element; against `u64` it is 6.375, seventeen
//! times the `u16` figure. Whether packing pays is a different question at
//! each of the three, and no committed artifact holds more than one of them.
//!
//! So this crate carries the identical logical column in **four** carriers at
//! once, and every arm reads its own region of one shared input:
//!
//! | Region | Bytes per element | Saving packing buys against it |
//! |---|---|---|
//! | `d64` (`[u64]`) | 8 | 6.375 |
//! | `d32` (`[u32]`) | 4 | 2.375 |
//! | `d16` (`[u16]`) | 2 | 0.375 |
//! | `packed` (13 bits) | 1.625 | n/a, this is the packed arm |
//!
//! ## Why the transform is a minimal wrapping sum, deliberately
//!
//! The per-element work is the smallest non-deletable reduction available: a
//! wrapping accumulate of the masked value. That choice is not laziness and
//! it is not neutral, so it is stated rather than left for a reader to infer.
//!
//! A minimal transform is the **most** favourable case for packing on the
//! memory axis, because the fewer instructions sit between two loads the more
//! completely the loop is bound by bytes moved, which is the only axis on
//! which packing can win. It is simultaneously the **least** favourable case
//! on the ALU axis, because a dense read of a native carrier vectorises to a
//! widening add while the packed decode stays scalar, so the decode surcharge
//! is paid at full weight with nothing to hide behind.
//!
//! Those two pull in opposite directions and both are real. The other end of
//! the trade is already covered: `bitpack-kernel-amortisation` puts a heavier
//! per-element kernel behind the same decoders. This bench is the memory end,
//! and reading it as the whole answer would be reading half of one.
//!
//! A wrapping sum is not deletable. Every element contributes to the
//! accumulator, so no fixpoint exists for the optimiser to fold the loop
//! into, which is the failure mode that put six cells above this host's
//! memory roofline elsewhere in this directory. The scaling check in
//! `26_probes/` tests that property against the measured numbers rather than
//! asserting it.
//!
//! ## What this crate does not re-derive
//!
//! The packed decode is `bench_bitpack_plan_shared::sum_windowed` with that
//! crate's own `Pack<13>` plan, imported unmodified, and the packer is its
//! `pack`. A second copy of the period, group, window and lane arithmetic
//! would be a second transform over the same layout, drifting from the one
//! the decoder-shape bench already measured, which is exactly what the
//! one-transform-one-layout discipline forbids. The three dense sums are new
//! here because no crate in this directory has a `u32` or `u64` carrier at
//! all.
//!
//! ## Sizing
//!
//! `build_input_bytes` writes straight into a heap `Vec<u8>` and never
//! materialises a `CarrierColumn` value, the same real override
//! `bitpack-footprint-shared` documents at its own module head: `Self::Input`
//! is `MAX_N`-sized for every monomorphisation, so there is no small `N` that
//! makes a by-value construction safe. `build_input` is `unreachable!()` for
//! that reason and not as a shortcut.
//!
//! This crate is bench infrastructure, not shipping arvo source, matching
//! every sibling variant crate here: no `#![no_std]`, `std` used freely.

use bench_bitpack_plan_shared::{pack, sum_naive, Pack, Packing};
use mockspace_bench_core::Routine;

/// Logical field width, unchanged from every sibling bitpack bench in this
/// directory: 13 bits, chosen because it does not divide eight.
pub const LOGICAL_BITS: usize = 13;
/// `(1 << LOGICAL_BITS) - 1`.
pub const MASK13: u64 = (1u64 << LOGICAL_BITS) - 1;

/// The plan the packed arm decodes through. Re-exported rather than
/// redefined; `bitpack-footprint-shared` names its own alias the same way.
pub type Plan13 = Pack<LOGICAL_BITS>;

/// The largest column this bench sweeps: 8,388,608 elements.
///
/// Chosen so the four regions together stay near the footprint bench's own
/// committed allocation (roughly 125 MiB against its 116 MiB) while the
/// widest carrier still clears this host's L2 by a wide margin. At `MAX_N`
/// the `u64` region is 64 MiB, five times the 12 MiB
/// `hw.perflevel0.l2cachesize` this host reports, and the packed region is
/// 13.0 MiB, only 1.08 times it. That gap is the whole point of the sweep:
/// it is the size band where a packed column fits and a dense one does not.
///
/// Every size declared for this bench is a multiple of eight, which is
/// `Pack<13>::P`, the width-13 period `sum_windowed`'s safety contract
/// requires.
pub const MAX_N: usize = 8_388_608;

/// Region sizes in bytes at `MAX_N`. The packed region carries 16 bytes of
/// read headroom for `sum_windowed`'s widest window, the identical
/// construction `bitpack-plan-shared` uses at its own smaller `MAX_N`.
pub const D64_BYTES: usize = MAX_N * 8;
pub const D32_BYTES: usize = MAX_N * 4;
pub const D16_BYTES: usize = MAX_N * 2;
pub const PACKED_BYTES: usize = (MAX_N * LOGICAL_BITS) / 8 + 16;

/// Byte offsets of each region inside the combined layout. The regions are
/// ordered by descending alignment (8, 4, 2, 1) so every one lands naturally
/// aligned with no interior padding, which is what makes `TOTAL_INPUT_BYTES`
/// equal `size_of::<CarrierColumn<N>>()` exactly. That equality is what the
/// framework's default `validate_output_bytes` relies on when it casts the
/// input buffer to `&Self::Input`, so it is asserted in the tests below
/// rather than trusted.
pub const OFF_D64: usize = 0;
pub const OFF_D32: usize = OFF_D64 + D64_BYTES;
pub const OFF_D16: usize = OFF_D32 + D32_BYTES;
pub const OFF_PACKED: usize = OFF_D16 + D16_BYTES;
/// Total heap allocation `build_input_bytes` makes.
pub const TOTAL_INPUT_BYTES: usize = OFF_PACKED + PACKED_BYTES;

/// splitmix64, matching every sibling shared crate's own copy. That
/// duplication predates this bench and stays out of its scope.
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

/// The output every arm writes: the wrapping sum of the decoded column.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Sum {
    pub value: u64,
}

/// The dense read at a 64-bit carrier: one value per 8-byte slot.
#[inline(always)]
pub fn sum_d64(vals: &[u64], n: usize) -> u64 {
    let mut s = 0u64;
    for i in 0..n {
        s = s.wrapping_add(vals[i] & MASK13);
    }
    s
}

/// The dense read at a 32-bit carrier: one value per 4-byte slot.
#[inline(always)]
pub fn sum_d32(vals: &[u32], n: usize) -> u64 {
    let mut s = 0u64;
    for i in 0..n {
        s = s.wrapping_add((vals[i] as u64) & MASK13);
    }
    s
}

/// The dense read at a 16-bit carrier: one value per 2-byte slot. This is the
/// carrier every prior bitpack bench in this directory measures against, kept
/// here so the sweep contains the already-measured point and a reader can
/// check this bench against them.
#[inline(always)]
pub fn sum_d16(vals: &[u16], n: usize) -> u64 {
    let mut s = 0u64;
    for i in 0..n {
        s = s.wrapping_add((vals[i] as u64) & MASK13);
    }
    s
}

/// The combined layout: four carriers of the same logical column.
///
/// A struct field length that is itself an expression of the struct's own
/// const generic parameter needs `generic_const_exprs`, forbidden per
/// `unstable-features.md`. The fixed-`MAX_N`-then-slice pattern is the
/// established dodge in this directory and is used identically here.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CarrierColumn<const N: usize> {
    pub d64: [u64; MAX_N],
    pub d32: [u32; MAX_N],
    pub d16: [u16; MAX_N],
    pub packed: [u8; PACKED_BYTES],
}

impl<const N: usize> Routine for CarrierColumn<N> {
    type Input = CarrierColumn<N>;
    type Output = Sum;

    /// Unreachable at every `N`, and not as a shortcut: `Self::Input` is
    /// `MAX_N`-sized (roughly 125 MiB) for every monomorphisation, so no
    /// small case makes a by-value construction safe.
    fn build_input(_seed: u64) -> Self::Input {
        unreachable!(
            "CarrierColumn::build_input is never called by the real bench path \
             (routine_bridge! only takes build_input_bytes as a function pointer) \
             and is not safe to call at any N: Self::Input is MAX_N-sized for \
             every monomorphisation. Use build_input_bytes."
        )
    }

    /// The heap-only path. Nothing stack-sized proportional to `N` or `MAX_N`
    /// exists at any point. Only the `N`-proportional prefix of each region
    /// is written; the tail past it stays at the zero-page mapping a fresh
    /// zeroed allocation gives, so it costs no real work at small `N`.
    fn build_input_bytes(seed: u64) -> std::vec::Vec<u8> {
        let mut rng = SplitMix64(seed ^ 0xB179_ACC0_0001_5EED);
        let vals: std::vec::Vec<u16> = (0..N).map(|_| (rng.next() & MASK13) as u16).collect();

        let mut buf = std::vec![0u8; TOTAL_INPUT_BYTES];
        for (i, &v) in vals.iter().enumerate() {
            let w = v as u64;
            buf[OFF_D64 + i * 8..OFF_D64 + i * 8 + 8].copy_from_slice(&w.to_le_bytes());
            buf[OFF_D32 + i * 4..OFF_D32 + i * 4 + 4].copy_from_slice(&(w as u32).to_le_bytes());
            buf[OFF_D16 + i * 2..OFF_D16 + i * 2 + 2].copy_from_slice(&v.to_le_bytes());
        }
        let packed_bytes = (N * LOGICAL_BITS) / 8 + 16;
        pack(&vals, &mut buf[OFF_PACKED..OFF_PACKED + packed_bytes]);
        buf
    }

    /// Four independent checks, not one.
    ///
    /// The ground truth is taken from the `u16` region, then every other
    /// region is checked against it, and the packed region is checked twice:
    /// once through whatever decode the timed arm ran (the `output` value)
    /// and once through `sum_naive`, an index-driven decoder no timed path in
    /// this bench uses. A defect shared between `pack` and `sum_windowed`,
    /// which touch the same period and group arithmetic, is therefore not
    /// invisible the way it would be if the only oracle were derived from the
    /// same construction.
    fn validate_output(input: &Self::Input, output: &Self::Output) -> Result<(), &'static str> {
        let mut expect: u64 = 0;
        for &v in input.d16[..N].iter() {
            expect = expect.wrapping_add(v as u64);
        }
        if output.value != expect {
            return Err("column sum mismatch: the timed arm produced a different \
                 value stream than the u16 ground truth");
        }
        if sum_d32(&input.d32[..N], N) != expect {
            return Err("u32 carrier region disagrees with the u16 ground truth: \
                 build_input_bytes filled the two regions inconsistently");
        }
        if sum_d64(&input.d64[..N], N) != expect {
            return Err("u64 carrier region disagrees with the u16 ground truth: \
                 build_input_bytes filled the two regions inconsistently");
        }
        let packed_bytes = (N * LOGICAL_BITS) / 8 + 16;
        if sum_naive(&input.packed[..packed_bytes], N) != expect {
            return Err("packed region mismatch: sum_naive's independent decode \
                 disagrees with the u16 ground truth, so build_input_bytes packed the \
                 column incorrectly");
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

    /// The layout equality the framework's default `validate_output_bytes`
    /// relies on when it casts the input buffer to `&Self::Input`. Asserted
    /// rather than trusted: a padding byte anywhere between the four regions
    /// would make the cast read the wrong offsets while every timed number
    /// still looked ordinary.
    #[test]
    fn total_input_bytes_matches_struct_size() {
        assert_eq!(
            TOTAL_INPUT_BYTES,
            core::mem::size_of::<CarrierColumn<256>>(),
            "declared buffer size and struct size disagree, so the harness cast is unsound"
        );
        assert_eq!(
            TOTAL_INPUT_BYTES,
            core::mem::size_of::<CarrierColumn<{ MAX_N }>>(),
            "struct size varies with N, which it must not"
        );
    }

    /// Every region is naturally aligned, which is what makes the sums above
    /// plain aligned loads rather than unaligned ones on any target.
    #[test]
    fn regions_are_naturally_aligned() {
        assert_eq!(OFF_D64 % 8, 0, "u64 region misaligned");
        assert_eq!(OFF_D32 % 4, 0, "u32 region misaligned");
        assert_eq!(OFF_D16 % 2, 0, "u16 region misaligned");
    }

    /// Decodes a `build_input_bytes` buffer by hand, without casting it to
    /// `&CarrierColumn`, so nothing here needs the `MAX_N`-wide type in
    /// scope. Checks all four carriers agree element by element, not merely
    /// in their sum: a permutation defect or a compensating pair of errors
    /// passes a sum check and fails this one.
    fn check_size<const N: usize>() {
        for seed in 0u64..4 {
            let buf = <CarrierColumn<N> as Routine>::build_input_bytes(seed);
            assert_eq!(buf.len(), TOTAL_INPUT_BYTES);
            let packed_bytes = (N * LOGICAL_BITS) / 8 + 16;
            for i in 0..N {
                let v16 =
                    u16::from_le_bytes([buf[OFF_D16 + i * 2], buf[OFF_D16 + i * 2 + 1]]) as u64;
                let v32 = u32::from_le_bytes([
                    buf[OFF_D32 + i * 4],
                    buf[OFF_D32 + i * 4 + 1],
                    buf[OFF_D32 + i * 4 + 2],
                    buf[OFF_D32 + i * 4 + 3],
                ]) as u64;
                let mut b8 = [0u8; 8];
                b8.copy_from_slice(&buf[OFF_D64 + i * 8..OFF_D64 + i * 8 + 8]);
                let v64 = u64::from_le_bytes(b8);
                assert!(v16 <= MASK13, "value at {i} exceeds the 13-bit field");
                assert_eq!(v32, v16, "u32 carrier disagrees at index {i}, seed {seed}");
                assert_eq!(v64, v16, "u64 carrier disagrees at index {i}, seed {seed}");
            }
            // the packed region, through the decoder no timed arm here runs
            let mut expect = 0u64;
            for i in 0..N {
                expect = expect.wrapping_add(u16::from_le_bytes([
                    buf[OFF_D16 + i * 2],
                    buf[OFF_D16 + i * 2 + 1],
                ]) as u64);
            }
            assert_eq!(
                sum_naive(&buf[OFF_PACKED..OFF_PACKED + packed_bytes], N),
                expect,
                "packed region decode disagrees with the u16 ground truth, seed {seed}"
            );
            // and through the decoder the packed arm does run
            let windowed = unsafe {
                bench_bitpack_plan_shared::sum_windowed::<Plan13>(
                    &buf[OFF_PACKED..OFF_PACKED + packed_bytes],
                    N,
                )
            };
            assert_eq!(
                windowed, expect,
                "sum_windowed disagrees with the u16 ground truth, seed {seed}"
            );
        }
    }

    #[test]
    fn carrier_16384_roundtrips() {
        check_size::<16384>();
    }

    #[test]
    fn carrier_131072_roundtrips() {
        check_size::<131072>();
    }

    /// The three dense transforms agree with each other and with the packed
    /// decode on the same column. Without this, an arm reading the wrong
    /// region would still produce a plausible number.
    #[test]
    fn all_four_transforms_agree() {
        const N: usize = 16384;
        let buf = <CarrierColumn<N> as Routine>::build_input_bytes(7);
        let col: &CarrierColumn<N> = unsafe { &*(buf.as_ptr() as *const CarrierColumn<N>) };
        let a = sum_d16(&col.d16[..N], N);
        let b = sum_d32(&col.d32[..N], N);
        let c = sum_d64(&col.d64[..N], N);
        let packed_bytes = (N * LOGICAL_BITS) / 8 + 16;
        let d = unsafe {
            bench_bitpack_plan_shared::sum_windowed::<Plan13>(&col.packed[..packed_bytes], N)
        };
        assert_eq!(a, b, "u16 and u32 arms disagree");
        assert_eq!(a, c, "u16 and u64 arms disagree");
        assert_eq!(a, d, "u16 and packed arms disagree");
    }

    /// `validate_output` refuses a wrong answer. A validation pass that
    /// cannot fail is not a validation pass, and this bench's whole fidelity
    /// argument rests on the harness calling it, so the refusal is pinned
    /// here as well as demonstrated against the live harness in `26_probes/`.
    #[test]
    fn validate_output_rejects_a_wrong_sum() {
        const N: usize = 16384;
        let buf = <CarrierColumn<N> as Routine>::build_input_bytes(3);
        let col: &CarrierColumn<N> = unsafe { &*(buf.as_ptr() as *const CarrierColumn<N>) };
        let good = Sum {
            value: sum_d16(&col.d16[..N], N),
        };
        assert!(<CarrierColumn<N> as Routine>::validate_output(col, &good).is_ok());
        let bad = Sum {
            value: good.value.wrapping_add(1),
        };
        assert!(
            <CarrierColumn<N> as Routine>::validate_output(col, &bad).is_err(),
            "validate_output accepted a sum off by one, so it would accept a broken arm"
        );
    }
}

/// A second reduction shape over `bitpack-plan-shared`'s group decode.
///
/// The committed SIMD arm (`sum_simd`) loses to the scalar `sum_windowed` by
/// 29 percent at n = 262144 (`bitpack-decoder-shape_n262144_findings.md`:
/// 55824 ns against 43388 ns), which is the wrong way round for a vector
/// kernel and is a mechanism worth attacking rather than reporting.
///
/// Disassembling it says where the time goes. The inner loop is sixteen
/// instructions per group of eight values, and only seven of them decode:
///
/// ```text
/// ldr q6, [x12, x10]        tbl.16b v7, {v6}, v1      tbl.16b v6, {v6}, v2
/// ushl.4s v7, v7, v3        ushl.4s v6, v6, v4
/// and.16b v7, v7, v5        and.16b v6, v6, v5
/// ext.16b v16, v7, v7, #8   add.2s v7, v16, v7        uaddw.2d v0, v0, v7
/// ext.16b v7, v6, v6, #8    add.2s v6, v7, v6         uaddw.2d v0, v0, v6
/// add x10, x10, #0xd        cmp x10, x11              b.ne
/// ```
///
/// The four table loads are already hoisted into the prologue, so that is not
/// the defect. The defect is the last six instructions before the loop
/// arithmetic: widening eight 16-bit lanes into a 64-bit accumulator every
/// group costs six instructions, which is 46 percent of the loop and is more
/// than the decode it is reducing.
///
/// `UADALP` does the same accumulate in one instruction, pairwise adding eight
/// 16-bit lanes into four 32-bit lanes. The reason the obvious version cannot
/// be written directly is overflow: a 32-bit lane takes two fields per group,
/// so it holds at most `2 * groups * MASK`, and at `W = 13` that exceeds
/// `u32::MAX` after 262,160 groups. The fix is a two-level accumulator. Run
/// `DRAIN` groups into 32-bit lanes, fold that block into a 64-bit total, and
/// repeat. At the largest column this bench sweeps that is four drains in
/// 1,048,576 groups, so the widening cost is paid roughly a quarter of a
/// million times less often than `sum_simd` pays it.
///
/// The group decode itself is `bench_bitpack_plan_shared::neon::decode_group`,
/// imported unmodified. Nothing about the gather, shift or mask arithmetic is
/// re-derived here; only the reduction differs, which is the whole point of
/// the arm.
///
/// # Safety
/// Identical to `sum_windowed`'s: `buf` holds `n * W` bits plus 16 bytes of
/// read headroom and `n` is a multiple of `K::P`. Both are declaration-time
/// facts about the column rather than runtime conditions.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub unsafe fn sum_simd_padal<K: Packing>(buf: &[u8], n: usize) -> u64 {
    use core::arch::aarch64::*;

    // groups a 32-bit lane can absorb before it can overflow: each lane takes
    // two fields per group and every field is at most K::MASK.
    let drain: usize = (u32::MAX as u64 / (2 * K::MASK)) as usize;

    let groups = n / K::P;
    let mut total: u64 = 0;
    let mut base = 0usize;
    let mut done = 0usize;
    unsafe {
        while done < groups {
            let chunk = core::cmp::min(groups - done, drain);
            let mut acc = vdupq_n_u32(0);
            for _ in 0..chunk {
                let lanes = bench_bitpack_plan_shared::neon::decode_group::<K>(buf, base);
                acc = vpadalq_u16(acc, lanes);
                base += K::G;
            }
            let wide = vpaddlq_u32(acc);
            total = total
                .wrapping_add(vgetq_lane_u64(wide, 0))
                .wrapping_add(vgetq_lane_u64(wide, 1));
            done += chunk;
        }
    }
    total
}

#[cfg(test)]
mod simd_tests {
    use super::*;

    /// The improved reduction agrees with the scalar decode it is meant to
    /// replace, at sizes on both sides of one drain boundary. The drain fold
    /// is the part most likely to be wrong and it is invisible below the
    /// drain period, so a test that only ran small sizes would prove nothing
    /// about it.
    #[cfg(target_arch = "aarch64")]
    fn agrees_at<const N: usize>() {
        let buf = <CarrierColumn<N> as Routine>::build_input_bytes(11);
        let bytes = (N * LOGICAL_BITS) / 8 + 16;
        let region = &buf[OFF_PACKED..OFF_PACKED + bytes];
        let scalar = unsafe { bench_bitpack_plan_shared::sum_windowed::<Plan13>(region, N) };
        let vector = unsafe { sum_simd_padal::<Plan13>(region, N) };
        let mut truth = 0u64;
        for i in 0..N {
            truth = truth
                .wrapping_add(
                    u16::from_le_bytes([buf[OFF_D16 + i * 2], buf[OFF_D16 + i * 2 + 1]]) as u64,
                );
        }
        assert_eq!(
            scalar, truth,
            "scalar decode disagrees with ground truth at N={N}"
        );
        assert_eq!(
            vector, truth,
            "sum_simd_padal disagrees with ground truth at N={N}"
        );
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn padal_agrees_below_one_drain() {
        agrees_at::<16384>();
        agrees_at::<131072>();
    }

    /// Past the drain boundary. At W = 13 the drain period is 262,160 groups,
    /// which is 2,097,280 elements, so this size crosses it and the one below
    /// does not.
    #[cfg(target_arch = "aarch64")]
    #[test]
    fn padal_agrees_across_a_drain_boundary() {
        agrees_at::<2097152>();
        agrees_at::<4194304>();
    }

    /// The drain period is what the overflow argument rests on, so it is
    /// asserted rather than left in a comment. A 32-bit lane takes two fields
    /// per group; after `drain` groups the largest value it can hold must
    /// still fit.
    #[test]
    fn drain_period_cannot_overflow_a_lane() {
        let mask = MASK13;
        let drain = u32::MAX as u64 / (2 * mask);
        assert!(
            drain * 2 * mask <= u32::MAX as u64,
            "a 32-bit lane can overflow within one drain period"
        );
        assert!(
            (drain + 1) * 2 * mask > u32::MAX as u64,
            "the drain period is needlessly short, which costs folds for nothing"
        );
    }
}
