//! A dedicated correctness stress test, run outside the timed bench path.
//!
//! `kernels::tests` pins that the encode logic is correct, including under a
//! misaligned split, as long as the calls are sequential. Nothing races when
//! calls are sequential; the hazard this crate exists to demonstrate only
//! shows up when two threads perform their boundary write at the same
//! instant, which one pass through the harness's own validation (a single
//! seed, once) is not statistically powerful enough to catch reliably. This
//! module repeats the pass many times, with a freshly zeroed output buffer
//! each time and real OS threads through the same pool the bench uses, and
//! counts how often the decoded result disagrees with the ground truth.
//!
//! `for_each_boundary_first` (`kernels.rs`) orders each thread's own boundary
//! element before its interior elements specifically so this test has a
//! chance of observing the race: it concentrates every thread's riskiest
//! write at the moment right after the pool releases every worker together,
//! which is when two threads are most likely to be inside their own boundary
//! write at once. A sequential in-order pass would spread the boundary writes
//! across the whole duration of a much longer pass and might never observe
//! the race in a bounded number of trials on this host.

#![cfg(test)]

use crate::input::{build_bytes, OFF_PACKED_OUT, OFF_VALS};
use crate::pool::write_pass;
use crate::{kern_packed_guarded, kern_packed_plain};
use bench_bitpack_plan_shared::{sum_naive, MASK13};

fn truth(vals: &[u16], n: usize) -> u64 {
    let mut s = 0u64;
    for &v in &vals[..n] {
        s = s.wrapping_add((v & (MASK13 as u16)) as u64);
    }
    s
}

/// Run `trials` real concurrent passes at `(n, threads)` with the given
/// kernel, each with a fresh input and a fresh zeroed output buffer, and
/// return the number that disagreed with the ground truth.
fn corruption_count(n: usize, threads: usize, trials: usize, kernel: crate::WriteKernel) -> usize {
    let mut mismatches = 0usize;
    for trial in 0..trials {
        let mut buf = build_bytes(n, 0xC0FF_EE00 ^ trial as u64);
        let vals_ptr = buf[OFF_VALS..].as_ptr() as *const u16;
        let want = {
            let vals = unsafe { std::slice::from_raw_parts(vals_ptr, n) };
            truth(vals, n)
        };
        let out_ptr = buf[OFF_PACKED_OUT..].as_mut_ptr();
        unsafe { write_pass(threads, n, vals_ptr, out_ptr, kernel) };
        let got = sum_naive(&buf[OFF_PACKED_OUT..], n);
        if got != want {
            mismatches += 1;
        }
    }
    mismatches
}

/// A small, deliberately misaligned size, chosen for fast iteration rather
/// than for realism: the timed bench's own "race" sizes exist for that.
/// `4094 / 4 = 1023`, `1023 % 8 = 7`, which lands the split mid-byte, pinned
/// by the same assertion the timed bench's sizes use rather than trusted from
/// hand arithmetic.
const STRESS_N: usize = 4094;

/// The pool is a single per-process `OnceLock` sized on first use (matching
/// the timed bench, which is one process per `(bench, size)` row and never
/// mixes thread counts within a run). `cargo test` runs every `#[test]` in
/// one process, so every stress test in this file shares that pool and must
/// agree on one thread count; four cores is the more contended of the two the
/// timed bench declares, so it is the one this file uses.
const STRESS_THREADS: usize = 4;

#[test]
fn stress_size_is_actually_misaligned() {
    use crate::input::{slice_bounds, split_is_guarded};
    let (_, hi0) = slice_bounds(0, STRESS_N, STRESS_THREADS);
    assert!(
        split_is_guarded(hi0, STRESS_N),
        "STRESS_N={STRESS_N} t={STRESS_THREADS} produced an unguarded boundary"
    );
}

/// The guarded kernel never corrupts a misaligned split, across many
/// concurrent trials.
#[test]
fn guarded_kernel_never_corrupts_under_real_concurrency() {
    let bad = corruption_count(STRESS_N, STRESS_THREADS, 500, kern_packed_guarded);
    assert_eq!(bad, 0, "guarded kernel corrupted at t={STRESS_THREADS}");
}

/// The naive kernel, run at the exact same misaligned split under exact same
/// real concurrency. Whatever this measures is the finding: a positive count
/// is the hazard demonstrated on real hardware; a zero count says this host
/// did not expose it in this many trials, which is itself a result, and is
/// reported as one, not read as a proof of safety.
#[test]
fn naive_kernel_corruption_rate_under_real_concurrency() {
    let trials = 3000usize;
    let bad = corruption_count(STRESS_N, STRESS_THREADS, trials, kern_packed_plain);
    eprintln!(
        "naive_kernel_corruption_rate_under_real_concurrency: t={STRESS_THREADS} \
         {bad}/{trials} trials disagreed with ground truth"
    );
    // Not an assertion of a specific rate: the honest claim this test can
    // make is that the kernel and the harness around it are exercised
    // correctly, which `guarded_kernel_never_corrupts_under_real_concurrency`
    // establishes as the control. The corruption count itself is read from
    // stderr and reported in the findings, not gated here, because a
    // scheduler-dependent rate is not a fact this test should assert a
    // threshold on.
}

/// Control: the naive kernel at the exact same thread count and real
/// concurrency, but at a size whose split IS period-aligned. If the naive
/// kernel's corruption above is genuinely the boundary-byte race and not some
/// other defect in the pool or the test harness, this must be zero.
#[test]
fn naive_kernel_never_corrupts_when_the_split_is_aligned() {
    const ALIGNED_N: usize = 4096; // 4096 / 4 = 1024, 1024 % 8 == 0
    let bad = corruption_count(ALIGNED_N, STRESS_THREADS, 1000, kern_packed_plain);
    assert_eq!(
        bad, 0,
        "naive kernel corrupted at an aligned split; the hazard is not what \
         this crate thinks it is"
    );
}

/// Two passes entered at once both finish, which is the property that was
/// missing rather than the property nobody had thought about.
///
/// Every test above shares one process-wide pool and `cargo test` runs them on
/// their own threads, so they were already doing this and the suite wedged.
/// Four processes were found sitting on it between two and four and a half
/// hours, every sample inside `pool::write_pass`'s completion spin, and
/// `cargo mock test` had been reported twice as producing no output at all,
/// which is what a hang looks like from outside.
///
/// The failure this asserts against is not only the hang. Before the pool took
/// a lock, a second pass overwrote the first's `vals` and `out` while its
/// workers were running, so a pass that happened not to hang had written
/// through another pass's pointers and returned a count about nothing.
///
/// It fails rather than hangs when the lock goes, which is the whole reason for
/// the channel: a plain join would reproduce the wedge instead of reporting it,
/// and a suite that hangs is the thing being fixed.
#[test]
fn two_passes_entered_at_once_both_finish() {
    use std::sync::mpsc;
    use std::time::Duration;

    // Long enough that a loaded machine does not fail this spuriously, short
    // enough to report rather than wedge. A pass at this size is microseconds
    // and the runs below are two hundred of them.
    const PATIENCE: Duration = Duration::from_secs(60);
    const TRIALS: usize = 200;

    let (tx, rx) = mpsc::channel();
    for arm in 0..2 {
        let tx = tx.clone();
        std::thread::Builder::new()
            .name(format!("concurrent-pass-{arm}"))
            .spawn(move || {
                let bad = corruption_count(STRESS_N, STRESS_THREADS, TRIALS, kern_packed_guarded);
                let _ = tx.send((arm, bad));
            })
            .expect("spawning a concurrent pass");
    }
    drop(tx);

    for _ in 0..2 {
        match rx.recv_timeout(PATIENCE) {
            Ok((arm, bad)) => assert_eq!(
                bad, 0,
                "arm {arm} saw the guarded kernel corrupt {bad} of {TRIALS} trials, so the \
                 two passes reached each other's buffers rather than merely queueing"
            ),
            Err(_) => panic!(
                "a pass did not return within {PATIENCE:?}. Two passes are in flight on one \
                 process-wide pool, so each is spinning on a `done` counter the other reset. \
                 `pool::ONE_PASS_AT_A_TIME` is what serialises them."
            ),
        }
    }
}

/// Sanity: the guarded and naive kernels have identical entry points for the
/// pool, so this test compiles them against the same call site the stress
/// tests above use.
#[allow(dead_code)]
fn _shape_check() {
    let _: crate::WriteKernel = kern_packed_guarded;
    let _: crate::WriteKernel = kern_packed_plain;
}
