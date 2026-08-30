//! Shared data model for the footprint bench: prices `Layout::Bitpacked`'s own
//! intent (a smaller column fits where a larger one does not) rather than its
//! decode cost at cache-resident sizes, which every prior bench in this
//! directory measured instead (`81_fog_is_the_bitpack_cost_inherent.md`
//! section 1.1: the whole `bitpack-decoder-shape` sweep never left this host's
//! 128 KB L1, so the multiple it reports is a compute-bound number and the
//! footprint saving cannot show up in it at any size that bench can hold).
//!
//! Reuses `bench-bitpack-plan-shared`'s transform, unmodified: `Pack<W>` for
//! the compile-time period/group/window/lane plan, `sum_native` for the dense
//! floor, `sum_windowed` for the best decode-dominated packed decode file 81
//! measured (section 4.1, 1.50x against dense, the winner for a plain sum),
//! `sum_naive` as an independent oracle for the packed correctness check, and
//! `Sum` as the shared output shape. Nothing here re-derives the period,
//! group, window or lane arithmetic; a bench that reintroduced that math would
//! be measuring a second, possibly-drifted copy of the transform file 81
//! already fixed, which is exactly what the one-transform-one-layout
//! discipline exists to forbid.
//!
//! ## Why this crate's `build_input_bytes` is a real override, not a
//! convenience one
//!
//! `mockspace_bench_core::Routine`'s default `build_input_bytes`
//! (`bench-core/src/lib.rs:148-158`) calls `Self::build_input(seed)` first,
//! which returns `Self::Input` **by value**. Every existing bitpack bench in
//! this directory (`bitpack-shared`, `bitpack-plan-shared`) relies on that
//! default, and their own `Input` struct is `repr(C)` at a few hundred KB, so
//! the by-value return never gets close to a stack limit. A footprint bench
//! needs sizes that clear a 12 MB L2, and a value of that size returned by a
//! function whose body constructs it through a named local (`let mut col =
//! Column::<N>::default(); ...; col`, the exact shape every sibling shared
//! crate uses) is not guaranteed elided by the compiler: the finding this
//! bench exists to test was called blocked on exactly this ceiling
//! (`81:449-452`, `95b:154`).
//!
//! `mockspace_bench_core::ByteRoutine` already ships the fix, shipped and
//! tested (`bench-core/src/byte_routine.rs:76-93`), and its own doc comment
//! states the purpose in the same words this crate needed: "never
//! materialising `[u8; IN]` on the stack. This removes the practical ceiling
//! on IN". `FootprintColumn::build_input_bytes` below is the identical shape
//! at this crate's own data model: it writes directly into a heap `Vec<u8>`
//! and never constructs a `FootprintColumn` value of any kind, so nothing
//! stack-sized proportional to `N` exists at any point during input
//! construction, for every `N` this bench declares including the one at
//! sixty-four times the host's L2.
//!
//! `FootprintColumn::build_input` (the typed, by-value form the `Routine`
//! trait still requires a definition for) is `unreachable!()` below, and this
//! is not a shortcut: a first draft wrote it out in full, the way
//! `bitpack-plan-shared`'s own `PlanColumn::build_input` is written, on the
//! stated assumption that it would be safe to call at this crate's own small
//! test sizes because only a few thousand elements would be touched. Running
//! that draft's own test suite (`cargo test -p bench-bitpack-footprint-shared`)
//! overflowed the stack on `FootprintColumn::<16384>::build_input`, in debug
//! mode, before a single element of the RNG loop ran. The reason is the same
//! fact the rest of this module states about the real bench sizes: `Self::Input`
//! is `FootprintColumn<N>`, whose fields are declared at `MAX_N`
//! (33,554,432) **regardless of which `N` a given monomorphisation
//! represents**, so `FootprintColumn::<16384>`'s own by-value construction is
//! exactly as large (about 116 MiB) as `FootprintColumn::<33554432>`'s. There
//! is no small `N` at which the typed path is safe to call; this is stronger
//! than the module doc comment's original claim.
//!
//! `unreachable!()`'s panic is itself not test-observable in-process: merely
//! being in a position to call a function whose return type is
//! `FootprintColumn<N>` reserves the ABI-level return slot for that huge
//! aggregate at the call site, before the callee's body runs, so even a
//! `#[should_panic]` test wrapping the call overflows the stack (`SIGABRT`)
//! rather than catching an unwind; that failure was produced once, by hand,
//! and is not kept as a standing test because keeping it would crash the
//! whole test binary on every run, taking every other test's signal with it.
//! `mockspace_bench_macro::routine_bridge!` takes
//! `build_input_bytes` as a function pointer and never references
//! `build_input` itself, confirmed by grepping the harness and macro crates
//! (`grep -rn "build_input(" bench-core bench-harness bench-macro`, the only
//! non-test, non-default-body call site is `bench-core/src/lib.rs:149`,
//! inside the default `build_input_bytes` this crate does not use), so the
//! unreachable body is never hit by the real bench path either.
//!
//! Bench infrastructure, not shipping arvo source: `std` used freely,
//! matching every sibling variant crate in this directory.

use bench_bitpack_plan_shared::{pack, Pack, Sum};
use mockspace_bench_core::Routine;

pub use bench_bitpack_plan_shared::{sum_naive, sum_native, sum_windowed};

/// Logical field width, unchanged from every sibling bitpack bench in this
/// directory (`bitpack-shared`, `bitpack-plan-shared`): 13 bits, the
/// non-power-of-two shape file 32 chose specifically because it does not
/// divide eight.
pub const LOGICAL_BITS: usize = 13;
pub const MASK13: u64 = (1u64 << LOGICAL_BITS) - 1;

/// The largest column this bench sweeps: 33,554,432 elements. At two bytes
/// per dense element that is 64 MiB, sixteen times past this host's own
/// 12 MiB L2 (`hw.perflevel0.l2cachesize`, read fresh this session, matching
/// what file 81 read on its own host). Every size this bench declares is a
/// multiple of eight (`Pack<13>::P`, the width-13 period), which
/// `sum_windowed`'s own safety contract requires.
pub const MAX_N: usize = 33_554_432;
/// Packed bytes at `MAX_N`, plus 16 bytes of read headroom for
/// `sum_windowed`'s widest window, the identical construction
/// `bitpack-plan-shared` uses at its own, smaller `MAX_N`.
pub const MAX_PACKED_BYTES: usize = (MAX_N * LOGICAL_BITS) / 8 + 16;
/// Byte offset of the packed region inside the combined input layout: the
/// dense/logical region occupies exactly `MAX_N * 2` bytes with no trailing
/// padding (`u16`'s alignment already satisfies the `u8` field that follows).
pub const PACKED_OFFSET: usize = MAX_N * 2;
/// Total heap allocation `build_input_bytes` makes: the dense region plus the
/// packed region, matching `size_of::<FootprintColumn<N>>()` exactly for
/// every `N` this bench declares, which is what makes the framework's own
/// default `validate_output_bytes` (a pointer cast to `&Self::Input`) sound
/// without this crate needing to override it too.
pub const TOTAL_INPUT_BYTES: usize = PACKED_OFFSET + MAX_PACKED_BYTES;

/// splitmix64, matching every sibling shared crate's own copy (each shared
/// bench crate in this harness carries one; that duplication predates this
/// bench and stays out of this dispatch's scope).
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

/// The combined footprint-column layout: a `MAX_N`-wide dense region
/// followed by a `MAX_N`-wide packed region, matching `bitpack-plan-shared`'s
/// own `PlanColumn` shape (a struct field length that is itself an expression
/// of the struct's own const generic parameter needs `generic_const_exprs`,
/// forbidden per `unstable-features.md`; the fixed-`MAX_N`-then-slice pattern
/// is the established dodge, used identically here). Every dense **and**
/// packed variant reads through this one type, so the same seed produces the
/// same logical value stream in both: dense stores it directly, packed stores
/// the identical value bit-packed. Only `build_input_bytes` ever produces a
/// value of this type at the sizes this bench actually declares; see the
/// module doc comment for why `build_input` itself stays unreachable there.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FootprintColumn<const N: usize> {
    pub logical: [u16; MAX_N],
    pub packed: [u8; MAX_PACKED_BYTES],
}

impl<const N: usize> Routine for FootprintColumn<N> {
    type Input = FootprintColumn<N>;
    type Output = Sum;

    /// Unreachable at every `N` this bench declares, and at every `N` a test
    /// could safely try. See the module doc comment: `Self::Input` is
    /// `MAX_N`-sized regardless of `N`, so there is no small case that makes
    /// a by-value construction safe. `build_input_bytes` below is the only
    /// path that ever produces a value of this shape.
    fn build_input(_seed: u64) -> Self::Input {
        unreachable!(
            "FootprintColumn::build_input is never called by the real bench path              (routine_bridge! only takes build_input_bytes as a function pointer)              and is not safe to call at any N: Self::Input is MAX_N-sized (about              116 MiB) for every monomorphisation. Use build_input_bytes."
        )
    }

    /// The heap-only path. Builds the logical values into a `Vec<u16>` (heap,
    /// proportional to `N`, never `MAX_N`), then fills a `Vec<u8>` sized to
    /// `TOTAL_INPUT_BYTES` (heap, `MAX_N`-sized, but a zero-fill of a fresh
    /// allocation is a zero-page mapping on every target this bench runs on,
    /// so the unused tail past what `N` needs costs no real work; only the
    /// `N`-proportional prefix of each region is ever written). No value of
    /// `Self::Input` exists at any point.
    fn build_input_bytes(seed: u64) -> std::vec::Vec<u8> {
        let mut rng = SplitMix64(seed ^ 0xB179_ACC0_0001_5EED);
        let vals: std::vec::Vec<u16> = (0..N).map(|_| (rng.next() & MASK13) as u16).collect();

        let mut buf = std::vec![0u8; TOTAL_INPUT_BYTES];
        for (i, &v) in vals.iter().enumerate() {
            let b = v.to_le_bytes();
            buf[i * 2] = b[0];
            buf[i * 2 + 1] = b[1];
        }
        let packed_bytes = (N * LOGICAL_BITS) / 8 + 16;
        pack(&vals, &mut buf[PACKED_OFFSET..PACKED_OFFSET + packed_bytes]);
        buf
    }

    fn validate_output(input: &Self::Input, output: &Self::Output) -> Result<(), &'static str> {
        let mut expect: u64 = 0;
        for &v in input.logical[..N].iter() {
            expect = expect.wrapping_add(v as u64);
        }
        if output.value != expect {
            return Err(
                "column sum mismatch: the timed decode produced a different \
                 value stream than the logical ground truth",
            );
        }
        // Second, independent check against the packed region through a
        // decoder this bench's own timed path never runs (`sum_naive`, file
        // 75's index-driven shape), so a bug shared between `build_input_bytes`
        // and `sum_windowed` (both touching the same period/group math) is not
        // invisible to validation the way it would be if the only oracle were
        // itself derived from the same construction.
        let naive_expect = sum_naive(&input.packed[..(N * LOGICAL_BITS) / 8 + 16], N);
        if naive_expect != expect {
            return Err(
                "packed region mismatch: sum_naive's independent decode of the \
                 packed bytes disagrees with the logical ground truth, so build_input_bytes \
                 itself packed the column incorrectly",
            );
        }
        Ok(())
    }

    fn ops_per_call(_input: &Self::Input) -> u64 {
        N as u64
    }
}

/// The plan-driven decode this bench measures for the packed reading: the
/// same `Pack<13>` this crate re-exports from `bitpack-plan-shared`, no
/// second definition.
pub type Plan13 = Pack<LOGICAL_BITS>;

#[cfg(test)]
mod tests {
    use super::*;

    /// Decodes the dense prefix of a `build_input_bytes` buffer directly,
    /// without casting the buffer to `&FootprintColumn` (that cast is what
    /// the harness's own default `validate_output_bytes` performs, exercised
    /// separately below; these tests decode by hand so nothing here needs the
    /// MAX_N-wide type in scope, matching how `build_input_bytes` itself
    /// never constructs one).
    fn logical_values<const N: usize>(buf: &[u8]) -> std::vec::Vec<u16> {
        (0..N)
            .map(|i| u16::from_le_bytes([buf[i * 2], buf[i * 2 + 1]]))
            .collect()
    }

    fn check_size<const N: usize>() {
        for seed in 0u64..4 {
            let buf = <FootprintColumn<N> as Routine>::build_input_bytes(seed);
            assert_eq!(buf.len(), TOTAL_INPUT_BYTES, "N={N}");

            let logical = logical_values::<N>(&buf);
            let mut expect = 0u64;
            for &v in &logical {
                expect = expect.wrapping_add(v as u64);
            }

            let native = sum_native(&logical, N);
            assert_eq!(native, expect, "sum_native mismatch, seed {seed} N={N}");

            let packed_bytes = (N * LOGICAL_BITS) / 8 + 16;
            let packed = &buf[PACKED_OFFSET..PACKED_OFFSET + packed_bytes];
            let windowed = unsafe { sum_windowed::<Plan13>(packed, N) };
            assert_eq!(windowed, expect, "sum_windowed mismatch, seed {seed} N={N}");
            let naive = sum_naive(packed, N);
            assert_eq!(naive, expect, "sum_naive mismatch, seed {seed} N={N}");

            // The default `validate_output_bytes` casts `buf` to
            // `&FootprintColumn<N>`, which is sound only because
            // `build_input_bytes`'s output has exactly `Self::Input`'s size
            // and layout. Exercise that path directly, both a correct sum
            // (accepted) and a perturbed one (refused), since this is what
            // the real harness calls on every validation-mode invocation.
            let good = Sum { value: native };
            let good_bytes = unsafe {
                core::slice::from_raw_parts(
                    &good as *const _ as *const u8,
                    core::mem::size_of::<Sum>(),
                )
            };
            <FootprintColumn<N> as Routine>::validate_output_bytes(&buf, good_bytes)
                .expect("a correct sum must validate");

            let bad = Sum {
                value: native.wrapping_add(1),
            };
            let bad_bytes = unsafe {
                core::slice::from_raw_parts(
                    &bad as *const _ as *const u8,
                    core::mem::size_of::<Sum>(),
                )
            };
            assert!(
                <FootprintColumn<N> as Routine>::validate_output_bytes(&buf, bad_bytes).is_err(),
                "a wrong sum must fail validation, N={N}"
            );
        }
    }

    #[test]
    fn column256_agrees() {
        check_size::<256>();
    }

    #[test]
    fn column4096_agrees() {
        check_size::<4096>();
    }

    #[test]
    fn column16384_agrees() {
        check_size::<16384>();
    }

    #[test]
    fn build_input_bytes_is_deterministic_per_seed() {
        let a = <FootprintColumn<256> as Routine>::build_input_bytes(42);
        let b = <FootprintColumn<256> as Routine>::build_input_bytes(42);
        assert_eq!(a, b);
    }

    #[test]
    fn build_input_bytes_differs_across_seeds() {
        let a = <FootprintColumn<256> as Routine>::build_input_bytes(1);
        let b = <FootprintColumn<256> as Routine>::build_input_bytes(2);
        assert_ne!(a, b);
    }

    /// `sum_naive` genuinely disagrees when fed a corrupted buffer, so the
    /// independent-oracle check inside `validate_output` is not a check that
    /// cannot fail: perturbing one packed byte moves the sum.
    #[test]
    fn sum_naive_is_sensitive_to_packed_corruption() {
        const N: usize = 256;
        let mut buf = <FootprintColumn<N> as Routine>::build_input_bytes(3);
        let packed_bytes = (N * LOGICAL_BITS) / 8 + 16;
        let clean = sum_naive(&buf[PACKED_OFFSET..PACKED_OFFSET + packed_bytes], N);
        buf[PACKED_OFFSET] ^= 0xFF;
        let corrupted = sum_naive(&buf[PACKED_OFFSET..PACKED_OFFSET + packed_bytes], N);
        assert_ne!(
            clean, corrupted,
            "corrupting a packed byte must move the sum"
        );
    }
}
