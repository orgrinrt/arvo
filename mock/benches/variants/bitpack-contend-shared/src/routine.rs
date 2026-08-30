//! The harness-facing routine: one row of the sweep, keyed `N * 10 + T`.

use bench_bitpack_carrier_shared::{sum_d32, sum_d64, Sum, LOGICAL_BITS};
use bench_bitpack_plan_shared::sum_naive;
use mockspace_bench_core::Routine;

use crate::input::{build_bytes, Layout, MAX_THREADS};

// ── the routine ────────────────────────────────────────────────────────────

/// One row of the sweep: `KEY = N * 10 + T`.
///
/// The two parameters travel together because the bench macro dispatches on
/// exactly one const parameter and the harness keys a row by its `n`. Packing
/// them into one integer is the idiom every sibling bench in this directory
/// already uses (`warm-clamp-arity` keys are `W * 1000 + ...`). Decoding is
/// arithmetic on associated consts, which is ordinary const evaluation and needs
/// nothing in type position, so `generic_const_exprs` stays out of it.
pub struct Contend<const KEY: usize>;

impl<const KEY: usize> Contend<KEY> {
    /// Elements in the column.
    pub const N: usize = KEY / 10;
    /// Threads walking it.
    pub const T: usize = KEY % 10;
    /// Refuses at monomorphisation when a key cannot be split cleanly.
    ///
    /// Two conditions, both of which would otherwise fail silently at a read
    /// rather than loudly at compile time: the thread count must be in range,
    /// and each slice must start on a packed-period boundary so the packed arm's
    /// byte offset is a whole number of bytes.
    pub const KEY_SPLITS: () = {
        assert!(Self::T >= 1 && Self::T <= MAX_THREADS);
        assert!(Self::N % (Self::T * 8) == 0);
    };
}

impl<const KEY: usize> Routine for Contend<KEY> {
    type Input = Layout;
    type Output = Sum;

    fn build_input(_seed: u64) -> Self::Input {
        unreachable!(
            "Contend::build_input is never called by the real bench path \
             (routine_bridge! only takes build_input_bytes as a function pointer) \
             and is not safe to call at any KEY: Self::Input is MAX_N-sized for \
             every monomorphisation. Use build_input_bytes."
        )
    }

    fn build_input_bytes(seed: u64) -> Vec<u8> {
        let () = Self::KEY_SPLITS;
        build_bytes(Self::N, seed)
    }

    /// The carrier crate's four independent checks, at this row's `N`.
    ///
    /// Ground truth from the `u16` region, then the `u32` and `u64` regions
    /// against it, then the packed region through `sum_naive`, an index-driven
    /// decoder no timed arm here runs. A defect shared between `pack` and
    /// `sum_windowed`, which touch the same period arithmetic, is therefore not
    /// invisible.
    fn validate_output(input: &Self::Input, output: &Self::Output) -> Result<(), &'static str> {
        let n = Self::N;
        let mut expect: u64 = 0;
        for &v in input.d16[..n].iter() {
            expect = expect.wrapping_add(v as u64);
        }
        if output.value != expect {
            return Err("column sum mismatch: the timed arm produced a different \
                 value stream than the u16 ground truth");
        }
        if sum_d32(&input.d32[..n], n) != expect {
            return Err("u32 carrier region disagrees with the u16 ground truth");
        }
        if sum_d64(&input.d64[..n], n) != expect {
            return Err("u64 carrier region disagrees with the u16 ground truth");
        }
        let packed_bytes = (n * LOGICAL_BITS) / 8 + 16;
        if sum_naive(&input.packed[..packed_bytes], n) != expect {
            return Err("packed region mismatch: sum_naive's independent decode \
                 disagrees with the u16 ground truth");
        }
        Ok(())
    }

    /// Elements in the pass, not elements per thread, so per-element cost is
    /// wall time over `N` and composes directly with the single-core sweep.
    fn ops_per_call(_input: &Self::Input) -> u64 {
        Self::N as u64
    }
}
