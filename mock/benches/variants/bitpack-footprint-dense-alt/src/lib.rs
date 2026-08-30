//! Second dense variant, existing only so `bitpack-footprint-dense` has a
//! real cross-validation partner. The harness's own `validation::validate`
//! requires at least two variants per bench section
//! (`bench-harness/src/validation.rs:73`, `bench-harness/src/driver/
//! mod.rs:378-435`); a single-variant section is never validated at all, and
//! the first sweep of this bench ran exactly that way. This crate closes the
//! gap: a genuinely different code shape (fold over an iterator, rather than
//! `sum_native`'s indexed loop) over the identical `input.logical` region, so
//! the harness's cross-variant byte comparison has something real to check.
//! See `bench-bitpack-footprint-shared`.

use bench_bitpack_footprint_shared::{FootprintColumn, MASK13};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

/// Deliberately not `sum_native`: a fold over an iterator rather than an
/// indexed loop, so this variant is a second, independently-written
/// implementation of the same reduction rather than a re-export of the one
/// under test.
#[inline(always)]
fn sum_dense_fold(vals: &[u16], n: usize) -> u64 {
    vals[..n].iter().fold(0u64, |acc, &v| {
        acc.wrapping_add((v & (MASK13 as u16)) as u64)
    })
}

#[bench_variant(
    FootprintColumn,
    "bitpack-footprint-dense-alt",
    sizes = [16384, 65536, 1048576, 4194304, 7000000, 33554432]
)]
fn run_footprint_dense_alt<const N: usize>(
    input: &<FootprintColumn<N> as Routine>::Input,
    output: &mut <FootprintColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = sum_dense_fold(&input.logical, N);
        }
    }
}
