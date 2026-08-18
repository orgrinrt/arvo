# P6 findings: the hang reproduced independently, and the three-way timing dispute is a profile difference

All commands run in `/Users/orgrinrt/Dev/clause-dev/arvo/mock/benches`, toolchain
`nightly-2026-05-28`, host aarch64-apple-darwin, `hw.ncpu = 8`, `hw.perflevel0.physicalcpu = 4`.
Transcript in `transcript.txt` beside this file.

**Ad-hoc quick spike with no substance as a benchmark.** These are wall-clock readings from
`cargo test`'s own summary line, not harness output. They are used only to separate two profiles
that differ by a factor of thirty, which is far outside anything the method's imprecision reaches.
Nothing here prices a design decision.

## F14. The hang, found blind in phase one, is a second independent instance

I hit it during my own phase-one test gate, before reading any panel file: the crate did not finish
after **26 minutes at 575% CPU** on a 4-performance-core host. `110` had already found it and
diagnosed it better, with a `sample(1)` stack sample showing 1387 of 1387 samples in
`pool::write_pass` (`110`'s test-gate section, `110_probes/p9_hang_sample.txt`).

Mine is a second instance arrived at differently, which is what the rung asks for. `110` diagnosed by
stack sampling; I diagnosed by isolation, which produces a different and complementary datum:

```
guarded_kernel_never_corrupts_under_real_concurrency  alone : ok, 0.31s
naive_kernel_corruption_rate_under_real_concurrency   alone : ok, 1.86s
naive_kernel_never_corrupts_when_the_split_is_aligned alone : ok, 0.59s
the three together, --test-threads=1                        : ok, 3.12s
the three together, default parallelism                     : DID NOT COMPLETE in 180s (repeated)
```

**Each test alone is under two seconds. Together they never finish.** That is the mechanism stated as
a measurement rather than as a reading of a stack.

**The mechanism, from the source.** `pool.rs:132-161`'s `write_pass` is a single-coordinator protocol
over one process-wide pool (`pool.rs:34`, `static POOL: OnceLock<PoolHandle>`). A coordinator stores
`kernel`, `vals`, `out`, `n` into shared fields with `Relaxed` stores, resets `done` to 0, bumps
`generation` with a Release, then spins on `p.done.load(Acquire) != threads - 1`. Nothing excludes a
second coordinator. Two concurrent callers interleave their stores into the same fields, and
`p.done.store(0, Relaxed)` by one resets the counter the other is waiting on, so that one spins
forever. The wait is `std::hint::spin_loop()` with no yield, which is why a hung coordinator burns a
core rather than blocking.

**And it is worse than a liveness bug, which I do not think has been said.** The workers read `vals`
and `out` as raw pointers from the shared fields (`pool.rs:107-109`). Under two concurrent
coordinators a worker can execute one test's kernel against the **other test's buffers**. Those
buffers are per-trial locals inside `corruption_count` (`stress.rs:41-56`), so a worker can write
through a pointer to a buffer whose trial has already returned. That is a use-after-free reachable
from `cargo test`, not merely a wrong answer.

The consequence for the suite: **under the default runner the three stress tests do not measure what
they claim.** `naive_kernel_never_corrupts_when_the_split_is_aligned` is the control that establishes
the observed corruption is the boundary-byte race; if corruption can also arrive from cross-test
pointer mixing, the control no longer isolates the hazard. The fix `110` and I both reach is the same
and it is one attribute: the stress tests are serialised, or moved behind `#[ignore]` as
`stress.rs:1` already says they are "run outside the timed bench path".

`holds for:` toolchain `nightly-2026-05-28`, host aarch64-apple-darwin with 4 performance cores,
`bitpack-write-contend-shared`, `STRESS_THREADS = 4`, debug and release profiles both, libtest
default parallelism, threads > 1.

## F15. The 107s / 4.05s / 4.25s disagreement about `wide-rung-shared` is not a disagreement

`111` section 0.2 records three measurements and treats them as a discrepancy: its own 4.25s, `110`'s
4.05s, and "the 107 seconds a previous brief attributed to it", called "a third measurement".

They are two profiles, not three measurements of one thing. Measured back to back on one host in one
session:

```
cargo test --manifest-path variants/wide-rung-shared/Cargo.toml -- --test-threads=1
  test result: ok. 30 passed ... finished in 109.08s          # debug, the cargo default

cargo test --release --manifest-path variants/wide-rung-shared/Cargo.toml
  test result: ok. 30 passed ... finished in 3.78s            # release
```

**29x.** `110`'s own transcript shows it ran `cargo test --release`, and `111`'s F111-13 carries
`--release, --test-threads=1` in its predicate. Both release numbers are right. The 107s figure is
also right, for the default profile, and the panel has been treating a correct number as suspect.

My own first reading of 115.43s was worse than either: it was taken while the hung
`bitpack-write-contend-shared` run was saturating the machine at 575% CPU, so it is contaminated and I
withdraw it. The clean debug number on this host is 109.08s serialised and 91.36s at default
parallelism.

**The general point is the one worth keeping.** A bare wall-clock number for a test suite carries no
meaning without its profile, and `cargo test` defaults to debug while every measurement this panel
trusted was taken at `--release`. Under `every-finding-carries-its-predicate.md` the profile is a
dimension, and none of the three original figures listed it.

`holds for:` toolchain `nightly-2026-05-28`, host aarch64-apple-darwin, `wide-rung-shared`, profiles
in {debug, release}, `--test-threads` in {1, default}, threads = 1 for the release/debug comparison.

## F16. `cargo test` at the bench root runs zero tests, and reports ok

```
cd mock/benches && cargo test --offline
  running 0 tests
  test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The variant crates are path dependencies rather than workspace members (`mock/benches/Cargo.toml`
lists them under `[dependencies]`, and `mock/Cargo.toml:38` excludes `benches` from the workspace), so
a root `cargo test` compiles them and tests none of them, then prints a green line.

This is a trap rather than a defect in any test: an agent that runs the obvious command and reads the
obvious output has measured nothing and been told it passed. I did it first and caught it only
because 0 was an implausible count. It is worth a line in whatever brief tells the next member to run
the gate.

`holds for:` toolchain `nightly-2026-05-28`, host aarch64-apple-darwin, the repository state at this
commit, cargo's default test target selection.

## What would refute each

F14: a run where the three stress tests complete under default parallelism on a host with more cores
than pool workers, which would make the interleaving rarer without removing it. That would narrow the
predicate rather than refute the mechanism, which is readable in `pool.rs` directly.
F15: a debug-profile run of `wide-rung-shared` completing in single-digit seconds, or a release run
taking a hundred. Neither occurred in four runs here.
F16: a cargo configuration under which the root test target reaches path dependencies. I know of none
and did not search exhaustively.
