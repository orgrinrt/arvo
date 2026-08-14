# P9. `cargo test` hangs in `bitpack-write-contend-shared`, and passes in 2.6s serially

Found while running the standing test gate, not while looking for it. Recorded here because the
check was performed by hand and would otherwise evaporate, and because the next member to run the
suite will lose the same forty-five minutes.

## The defect

Run from `mock/benches/variants/bitpack-write-contend-shared/`:

```
cargo test --release                        # hangs. killed at 45 minutes, no progress.
cargo test --release -- --test-threads=1    # ok. 15 passed; 0 failed; finished in 2.60s
```

Both invocations were run on this host, on `nightly-2026-05-28`,
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`. The serial run's tail is quoted at the bottom.

## The cause, from the crate's own source

`src/stress.rs:66-72`, the crate saying the load-bearing half itself:

```
/// The pool is a single per-process `OnceLock` sized on first use (matching
/// the timed bench, which is one process per `(bench, size)` row and never
/// mixes thread counts within a run). `cargo test` runs every `#[test]` in
/// one process, so every stress test in this file shares that pool and must
/// agree on one thread count; four cores is the more contended of the two the
/// timed bench declares, so it is the one this file uses.
```

The comment identifies that every test in the file shares one process-wide pool, and draws the
conclusion that they must **agree on a thread count**. They do: `STRESS_THREADS = 4` for all of
them. The conclusion it does not draw is that they must not run **at the same time**, and libtest
runs tests concurrently by default.

So two `#[test]` functions enter `pool::write_pass` on one shared four-worker pool simultaneously,
and each waits on a barrier the other's workers are also waiting on.

## The evidence that it is that and not merely slowness

`sample(1)` on the hung process, full output in `p9_hang_sample.txt`. The thread list:

```
1387 Thread_30834039   DispatchQueue_1: com.apple.main-thread  (serial)
1387 Thread_30834052: write-contend-1
1387 Thread_30834053: write-contend-2
1387 Thread_30834054: write-contend-3
1387 Thread_30834055: stress::naive_kernel_corruption_rate_under_real_concurrency
1387 Thread_30834056: stress::naive_kernel_never_corrupts_when_the_split_is_aligned
```

Two test threads, both live, and the stack under each bottoms out in the same place:

```
1387 ...bench_bitpack_write_contend_shared6stress16corruption_count + 496
  1387 ...bench_bitpack_write_contend_shared4pool10write_pass + 144,140,...
```

**1387 of 1387 samples in `pool::write_pass`**, for both tests at once, with only three pool workers
between them. That is two consumers of a single-consumer pool, not a long computation: the serial run
does the identical work in 2.6 seconds.

The counts also rule out the innocent explanation. The stress module runs 500 + 3000 + 1000 = 4500
concurrent trials in total, and it completes all of them in 2.60 seconds when run serially, so a
throughput argument for a 45 minute run does not survive the arithmetic.

## Why this matters beyond the one crate

`RULES.md` warns that a batch runner under a short cap silently reports twelve of thirteen crates,
and attributes the long pole to `wide-rung-shared` taking 107 seconds. On this host that figure did
not reproduce: `wide-rung-shared` runs its 30 tests in **4.48 seconds**, 8 seconds including a build
in a fresh `CARGO_TARGET_DIR`.

The actual long pole is this hang, and a hang is worse than a slow crate in the specific way that
matters here. A slow crate finishes if you wait. A hang does not, so the crate never reports, and a
runner that moves on after a timeout records a suite with a crate silently missing rather than a
crate that failed. Every member who has run this suite and reported green has either run it serially,
skipped this crate, or not watched it finish.

## What the fix is not

It is not lowering the trial counts, and it is not deleting the stress tests. They are the best tests
in this corpus: 4500 real concurrent trials, a positive control
(`guarded_kernel_never_corrupts_under_real_concurrency`), a negative control at an aligned split
(`naive_kernel_never_corrupts_when_the_split_is_aligned`), and a deliberate refusal to assert a
threshold on a scheduler-dependent corruption rate, with the reason written out at `stress.rs:105-111`.

The defect is that a process-wide single-consumer resource is used from tests that libtest is free to
run concurrently. Whoever owns this crate should decide the remedy; the candidates are a mutex around
the pool's use, `#[serial]`-style sequencing, or moving the stress module to its own integration test
binary so it has the process to itself. None of that is my call and none of it is in my dispatch.

## The serial run, quoted

```
test stress::guarded_kernel_never_corrupts_under_real_concurrency ... ok
test stress::naive_kernel_corruption_rate_under_real_concurrency ... ok
test stress::naive_kernel_never_corrupts_when_the_split_is_aligned ... ok
test stress::stress_size_is_actually_misaligned ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.60s
```
