//! The harness-facing routine: one row of the sweep, keyed `N * 10 + T`.
//!
//! Unlike the read bench's `Contend`, this routine's ground truth is a single
//! thing: the `vals` region `build_bytes` fills. Every arm writes its own
//! encoding of `vals` into its own scratch region and reports back a decoded
//! checksum as `Output::value`; `validate_output` only has to agree that
//! checksum with the sum of `vals` itself, so a write that dropped or
//! corrupted a value shows up as a wrong number regardless of which encoding
//! produced it.

use mockspace_bench_core::Routine;

use crate::input::{build_bytes, Layout, MAX_THREADS};

/// One row of the sweep: `KEY = N * 10 + T`, the same idiom `Contend` uses.
pub struct WriteContend<const KEY: usize>;

impl<const KEY: usize> WriteContend<KEY> {
    pub const N: usize = KEY / 10;
    pub const T: usize = KEY % 10;
    /// Refuses at monomorphisation when a key cannot be split cleanly. Checks
    /// only the thread-count range: whether a given `(N, T)` lands its
    /// internal boundaries on a period boundary is exactly the axis this
    /// bench varies on purpose, so nothing here refuses a misaligned key the
    /// way the read bench's `KEY_SPLITS` does.
    pub const KEY_RANGE: () = {
        assert!(Self::T >= 1 && Self::T <= MAX_THREADS);
    };
}

/// The decoded checksum every arm reports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sum {
    pub value: u64,
}

impl<const KEY: usize> Routine for WriteContend<KEY> {
    type Input = Layout;
    type Output = Sum;

    fn build_input(_seed: u64) -> Self::Input {
        unreachable!(
            "WriteContend::build_input is never called by the real bench path \
             (routine_bridge! only takes build_input_bytes as a function \
             pointer) and is not safe to call at any KEY: Self::Input is \
             MAX_N-sized for every monomorphisation. Use build_input_bytes."
        )
    }

    fn build_input_bytes(seed: u64) -> Vec<u8> {
        let () = Self::KEY_RANGE;
        build_bytes(Self::N, seed)
    }

    /// The one ground truth: the sum of `vals[..N]`, masked to the logical
    /// width the same way every write kernel masks it. Every arm's own
    /// `run_*` function decodes what it wrote and reports that decoded sum as
    /// `output.value`; this check does not need to know which region was
    /// written or how, only that the round trip landed on the same numbers.
    fn validate_output(input: &Self::Input, output: &Self::Output) -> Result<(), &'static str> {
        let n = Self::N;
        let mut expect: u64 = 0;
        for &v in input.vals[..n].iter() {
            expect = expect.wrapping_add((v & bench_bitpack_plan_shared::MASK13 as u16) as u64);
        }
        if output.value != expect {
            return Err("decoded sum mismatch: the write pass produced a value \
                 stream different from the vals ground truth");
        }
        Ok(())
    }

    fn ops_per_call(_input: &Self::Input) -> u64 {
        Self::N as u64
    }
}
