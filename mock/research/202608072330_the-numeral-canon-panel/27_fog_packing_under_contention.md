# 27. Packing under memory-system contention

**Date:** 2026-08-08. **Member:** Fog. **Status:** in progress, written to disk early and extended
in place per `RULES.md`.

**Assignment:** measure the packed-against-dense trade under real memory-system contention, extend
`26`'s carrier sweep past one core, and say whether the break-even carrier width moves.

Written as a sequence rather than a report. Attempt, what it hit, what was tried against it, what
that measured, next attempt.

## 0. Gate and orientation, recorded before any work

**Canon gate.** Nothing ratified bears on this dispatch. `01_op_answers.md` section 0 states that op's
word is the only thing that ratifies and that ratification comes after the experts converge, so there
is nothing here to defend or to conflict with. `26` is one expert's measurement and this file is a
second, independent instrument pointed at the same trade. Gate passes; the assigned work proceeds.

**Toolchain, verified against the pin rather than assumed.**

```
$ cd /Users/orgrinrt/Dev/clause-dev/arvo && rustc --version
rustc 1.98.0-nightly (57d06900f 2026-05-27)
```

Matches the brief's `nightly-2026-05-28`.

**Host, read fresh.**

```
$ sysctl -n hw.perflevel0.l1dcachesize hw.perflevel0.l2cachesize \
        hw.perflevel0.physicalcpu hw.perflevel1.physicalcpu \
        hw.physicalcpu hw.memsize machdep.cpu.brand_string
131072          # P-core L1D, 128 KB
12582912        # P-cluster L2, 12 MB
4               # 4 performance cores
4               # 4 efficiency cores
8
8589934592      # 8 GB
Apple M1
$ sysctl -n hw.perflevel1.l1dcachesize hw.perflevel1.l2cachesize
65536           # E-core L1D, 64 KB
4194304         # E-cluster L2, 4 MB
```

Four performance cores sharing one 12 MB L2 and one memory controller. That topology is the whole
subject of this file.

**Working tree was dirty on arrival and is not mine.** Nine `mock/benches/warm-clamp-arity-l2_n*`
files, three modified and six untracked, belong to another dispatch. No `git restore`, no
`git clean`, no `git stash`, no branch switch, no `git add -A`. Every stage below names its paths.
Branch: `feat/arvo-shape-topic`.

**The panel's commit log was not read before the answer was on disk**, per `RULES.md`.

## 1. The brief's factual claims, and `26`'s numbers, checked before reasoning from them

The brief asserts three things from `26` and asks that they be checked rather than inherited. All
three are recomputed from the committed csvs alone, by `27_probes/check_26_numbers.py`, which reads
`mock/benches/bitpack-carrier-width_n*.csv` and copies no number out of `26`'s prose.

```
$ python3 27_probes/check_26_numbers.py
```

Full output at `27_probes/check_26_numbers.out`. Warm-mode median picoseconds per element:

| n | d16 | d16-control | d32 | d64 | packed | packed-simd |
|---|---|---|---|---|---|---|
| 16,384 | 84.5 | 85.2 | 82.8 | 80.4 | 128.3 | 120.0 |
| 131,072 | 86.7 | 86.7 | 85.4 | 96.0 | 128.7 | 121.1 |
| 1,048,576 | 84.8 | 84.9 | 83.8 | 111.1 | 126.9 | 119.7 |
| 2,097,152 | 84.8 | 85.4 | 86.9 | 128.0 | 127.2 | 119.5 |
| 4,194,304 | 87.1 | 88.1 | 96.6 | 149.3 | 129.3 | 120.6 |
| 8,388,608 | 87.9 | 87.7 | 88.3 | 145.7 | 131.3 | 122.4 |

Forty warm samples per arm per size. This reproduces `26` section 7 cell for cell.

**Break-even carrier width, recomputed.** Interpolating dense cost linearly between the measured d32
and d64 points and solving for where it equals the packed cost:

| n | against `packed` | against `packed-simd` |
|---|---|---|
| 2,097,152 | 7.92 bytes | 7.17 bytes |
| 4,194,304 | 6.48 bytes | 5.82 bytes |
| 8,388,608 | 6.99 bytes | 6.38 bytes |

So the brief's "5.8 to 7.0 bytes" is exactly the range spanned by the two largest sizes. **Claim
confirmed.** The smaller sizes give nonsense (negative at n = 16,384, where d64 is faster than d16
because everything is cache-resident), which is the arithmetic reporting honestly that there is no
crossing when nothing is bandwidth-bound.

**Flatness, recomputed.** Spread of per-element cost across the whole 512-fold sweep:

| Arm | Spread |
|---|---|
| d16 | 4.1% |
| d16-control | 3.8% |
| packed | 3.5% |
| packed-simd | 2.4% |
| d32 | 16.7% |
| d64 | 85.8% |

**Claim confirmed, with one refinement the brief flattens.** The brief says "prefetch hid capacity
effects entirely in the single-threaded case". That is true of d16 and of both packed arms. It is
false of d64, which moves 86 percent across the sweep, and getting weakly false of d32 at 17 percent.
Prefetch hides capacity right up to the point where the stream's byte demand meets the machine's
supply, and then it stops hiding anything, because a prefetcher cannot deliver bytes the memory
controller does not have. That distinction is the mechanism this whole file turns on and it is
already visible at one core.

### 1.1 The number the brief does not state, and which decides whether the host can show the effect

Same probe, converting per-element cost to implied read bandwidth:

| n | d16 | d32 | d64 | packed | packed-simd |
|---|---|---|---|---|---|
| 16,384 | 23.7 | 48.3 | 99.5 | 12.7 | 13.5 |
| 1,048,576 | 23.6 | 47.7 | 72.0 | 12.8 | 13.6 |
| 4,194,304 | 23.0 | 41.4 | **53.6** | 12.6 | 13.5 |
| 8,388,608 | 22.7 | 45.3 | **54.9** | 12.4 | 13.3 |

The 16,384 row is L1-resident and its 99.5 GB/s is a cache figure, not a memory figure. The two
largest rows are the memory figures, and they say something the panel has not yet said out loud:

**One M1 performance core, on a sequential read, already sustains 53 to 55 GB/s.** This part's
theoretical peak is 68.25 GB/s (LPDDR4X-4266 across a 128-bit bus), so a single core is already at
roughly 80 percent of everything the machine has.

That is the answer to "can the host show the effect", and it is emphatic. It also sharpens the
prediction below, because a machine where one core reaches 80 percent of the ceiling is a machine
where contention does not arrive gradually at eight cores. It arrives at two.

## 2. The prediction, written before the bench exists

A measurement that disagrees with arithmetic means one of them is misconfigured, and knowing which is
the job, so the bound goes first.

Let an arm have single-core compute-bound per-element cost `c` picoseconds and carry `w` bytes per
element. `T` threads walk disjoint slices of one column, and per-element cost is reported over the
whole column, so a perfectly scaling arm reads `c / T`. Aggregate byte demand at that rate is
`T * w / c`, and where it exceeds the machine's aggregate supply `B` the arm stops scaling and its
per-element cost floors at `w / B`, which depends on the carrier's width and on nothing else. So

```
    cost(T)  =  max( c / T ,  w / B )
```

and the whole question is which term wins for which carrier at which `T`.

**Corrected in place.** The first version of this section wrote the bandwidth term as `T * w / B`,
which is the cost per element *per thread* rather than per element of the column, and it predicted
that every arm gets slower as cores are added. That is dimensionally wrong: aggregate bytes are fixed
at `N * w` however many threads move them. The corrected table is below and the numbers in it are the
ones the run is measured against.

Taking `c` from the single-core table above and `B = 55 GB/s`, the rate one core already demonstrates
is reachable:

| Arm | `w` | `c` at `T=1` | `c/4` | `w/B` | Predicted at `T=4` |
|---|---|---|---|---|---|
| d16 | 2 | 87 ps | 21.8 ps | 36.4 ps | bandwidth-bound at 36 ps |
| d32 | 4 | 88 ps | 22.0 ps | 72.7 ps | bandwidth-bound at 73 ps |
| d64 | 8 | 146 ps | 36.5 ps | 145.5 ps | bandwidth-bound at 146 ps, no gain at all |
| packed-simd | 1.625 | 122 ps | 30.5 ps | 29.5 ps | compute-bound at 31 ps, on the edge |

The prediction is therefore sharp and falsifiable: **at four cores every dense carrier is
bandwidth-bound and the packed arm is not**, because the packed decode is expensive enough that four
copies of it still fit under the ceiling while a cheap dense loop over a wide carrier does not. The
sharpest single consequence, and the easiest to check, is that **`d64` should gain nothing whatsoever
from extra cores**, since it is already at `w / B` with one.

If it holds, the break-even carrier width collapses from 5.8 to 7.0 bytes toward **2 bytes or below**,
and packing would win against `u16`, the carrier it has lost to at every size in every committed bench
in this directory.

This is exactly the shape a bench can be built to produce by accident, so section 3 states what was
chosen and what each choice biases, and section 4 states the controls that would catch it.

## 3. What "contention" was made to mean, and what that choice biases

Written before any code, because the trap the brief names is real: a contended workload can be
constructed to favour either side.

**The shape chosen: one column, `T` threads, disjoint contiguous slices, one pass.** Thread `i`
walks `[i*N/T, (i+1)*N/T)` of the same region the single-threaded arms walk, with the identical
kernel. The timed region covers the whole pass, so wall time divided by `N` is directly comparable to
`26`'s per-element numbers, and the `T = 1` row of this bench is a cross-check against `26`'s
committed csvs rather than a new claim.

This is the declared workload's own shape. `arvo-toolbox-not-policer.md` describes consumers with
"millions of entities" and "thousands of systems mutating them per frame" over contiguous
column-store storage. One system's pass over one column, split across cores, is that. It is also
hilavitkutin's morsel model, which is the engine this substrate exists under.

**What the shape biases, stated rather than left to a reader.**

It is the *honest* case rather than the favourable one, and specifically it is less favourable to
packing than the obvious alternative. Giving each thread its own full-size column would multiply the
aggregate footprint by `T` and put every arm deeper into the bandwidth wall sooner; splitting one
column keeps aggregate bytes fixed at `N * w` and only raises the *rate* at which they are demanded.
The first would have exaggerated the effect. The second measures it.

The per-element work is `26`'s minimal wrapping sum, unchanged, for the reason `26` states in its own
crate doc comment: minimal work maximises the share of the loop that is bytes moved, which is the
only axis packing can win on, while simultaneously letting the dense arms vectorise against a scalar
packed decode. Both halves are real and they pull opposite ways. Keeping it identical is also what
makes the two files compose instead of sitting beside each other.

**What is deliberately not done.** No artificial memory pressure from unrelated threads, no cache
flushing between calls beyond what the harness's own cold mode already does, no oversubscription.
Adding a bandwidth-hungry sibling process would produce a contended number that measures the sibling.

## 4. The harness already has a threading contract, which changes the plan `26` set aside

`26` section 13 lists "a multi-threaded arm inside one variant" as a route it considered and
rejected, on the ground that the harness times one process. Reading the harness rather than inferring
from it says the picture is better than that:

`bench-harness/src/config.rs:102-109` declares a per-bench `threaded` flag, documented as "whether
variants spawn their own threads inside the timed run block", which "disables the worker's P-core
self-pin (spawned threads never inherit the pin, and pinning only the coordinating thread skews a
threaded workload)". `harness.rs:155-161` and `harness.rs:799-801` are where it takes effect, and
`harness.rs:621-622` passes it through to each spawned worker process.

So spawning threads inside a timed region is a supported and named contract of this harness, not a
subversion of it. `arvo/mock/benches/bench.toml` uses it nowhere, which is why nobody in this panel
knew it was there.

One consequence has to be handled rather than inherited. `pin_to_perf_cores`
(`bench-core/src/counter.rs:139-151`) is a `pthread_set_qos_class_self_np(0x21, 0)` call, QoS
`USER_INTERACTIVE`, which biases the calling thread to P cores. Under `threaded = true` the harness
skips it entirely, so without further action the coordinating thread and every spawned worker land
wherever macOS puts them, which on a 4P + 4E part means an unknown mix of two core types with
different clocks, different L1 sizes and a different L2. That would not be a measurement of
contention; it would be a measurement of the scheduler.

The fix is for the variant to pin every participating thread identically, including the one that
called it. Symmetric, stated, and under the bench's control rather than the OS's.

## 5. Building it: the blockers, in the order they were hit

Recorded as they happened, including the routes that were wrong, because the wrong ones are the
tempting ones.

The bench is `mock/benches/variants/bitpack-contend-*`, seven crates, registered in
`mock/benches/bench.toml` under `[bench.bitpack-contention]` with `threaded = true`, and committed
before the run at `3454060` rather than after it.

### 5.1 The blocker that would have made the whole thing a thread-spawn benchmark

At `n = 1048576` a single pass costs about 90 microseconds. A thread spawn on macOS costs tens of
microseconds, so an arm that spawned `T` threads inside the timed region would have been reporting
thread creation with a column walk attached, and the reported speedup would have been an artifact of
how long the walk was relative to the spawn.

The fix is a persistent pool, created once per worker process, where the timed region publishes a job
and spins. Two things had to be decided rather than assumed.

**The job cannot be a closure.** A persistent pool outlives any borrow, so a closure would have to be
boxed on every call, putting an allocation inside the measured region. The job is a raw function
pointer plus a base pointer, an element count and a thread count, all in atomics, published through
one release store on a generation counter. That costs no allocation and the indirect call is paid
once per slice, amortised over at least `N / T` elements.

**The partials false-share by default.** Apple's L2 line is 128 bytes, so two `AtomicU64` partials
inside one 64-byte line still share an L2 line and every store bounces it between cores. Each partial
is on its own `#[repr(align(128))]` line. Untreated, this would have shown up as a scaling failure
and been read as contention, which is the exact confusion this bench exists to avoid.

The coordinator takes slice 0 itself rather than waiting on `T` workers, so a pass uses exactly `T`
cores and none is idle inside the measured region.

### 5.2 Eight tests, and what each is for

`cargo test -p bench-bitpack-contend-shared --release`, all eight pass.

Three of them exist because a specific thing could be silently wrong. `layout_is_independent_of_the_
const_parameter` pins the equality the harness's cast of the raw input buffer relies on, since a
padding byte between regions would read the wrong offsets while every timed number still looked
ordinary. `build_bytes_equals_the_carrier_crates_builder` asserts byte-for-byte equality between this
crate's runtime-`n` builder and the carrier crate's const-`N` one, which is the only thing making the
claim that these numbers compose with `26`'s true rather than hopeful. `slices_tile_the_column_at_
every_thread_count` checks that the slices leave no gap and no overlap, at every `(n, t)` the bench
declares rather than at a sample of them, because a gap in the middle of a large column changes the
answer and a sum check at one thread count would not see it.

`a_split_pass_equals_the_whole_pass_for_every_kernel` runs the whole matrix: five kernels by four
thread counts. `the_pool_computes_the_same_total_as_a_serial_pass` drives the pool itself, twice, the
second time because a generation counter that works once is a defect a single call cannot see.
`validate_output_rejects_a_wrong_sum` pins that the validation can fail.

The refusal `KEY_SPLITS` is a const assertion that fires at monomorphisation for any key whose slices
would not land on a packed-period boundary. Its arithmetic is asserted in a test because a
compile-fail test needs a trybuild harness this directory does not have, which is a real gap and is
named as one rather than papered over.

## 6. The first run, and the two things it said before it said anything about packing

```
$ cd mock/benches && ../target/release/arvo-benches --bench bitpack-contention
```

Log at `27_probes/contention_run.log`. Artifacts: `mock/benches/bitpack-contention_n<KEY>.csv` with
`.meta.json` and `_findings.md` beside each, where `KEY = N * 10 + T`.

The harness validated all six arms against 100 seeds and checked determinism before timing anything,
which is the driver call `26` added at `src/main.rs:150-166`.

### 6.1 The `T = 1` rows reproduce the single-core sweep, which is the cross-check that matters

Picoseconds per element, this bench at `T = 1` against the committed carrier sweep:

| n | arm | this bench, `T = 1` | `bitpack-carrier-width` | difference |
|---|---|---|---|---|
| 1,048,576 | d16 | 84.8 | 84.8 | 0.0% |
| 1,048,576 | d32 | 83.8 | 83.8 | 0.0% |
| 1,048,576 | packed | 127.1 | 126.9 | +0.2% |
| 1,048,576 | packed-simd | 119.5 | 119.7 | -0.2% |
| 4,194,304 | d16 | 88.5 | 87.1 | +1.6% |
| 4,194,304 | d64 | 142.9 | 149.3 | -4.3% |
| 4,194,304 | packed | 128.8 | 129.3 | -0.4% |
| 4,194,304 | packed-simd | 120.9 | 120.6 | +0.2% |

Warm medians both sides, so the comparison is like for like. Two independently written benches, one
going through a function pointer and a pool and the other calling a const-`N` kernel directly, land
within a few percent on every arm. **The two files compose.**

The one place they do not is `n = 16384`, where this bench reads 8 to 9 percent slower on the dense
arms. That is a small-`n` artifact: a pass there costs 1.5 microseconds, so a fixed per-call cost of
about 130 nanoseconds is 9 percent of it, and the same cost is 0.03 percent at `n = 4194304`. The
discriminator is in the table above: the gap is present at 16,384 and absent at 1,048,576 and
4,194,304, which is what a fixed cost does and not what a per-element cost does.

### 6.2 The median is the wrong statistic for a threaded arm, and the control says so

A threaded sample is the wall time of a pass whose duration is the maximum over `T` threads. Any
scheduling interference on any one thread inflates that sample and nothing can deflate it, so the
distribution is one-sided and its median drifts with how much interference a particular worker
process happened to meet.

That is not a theory, it is measurable, and the noise-floor control is what measures it. At
`n = 4194304`, `t = 2`, the two byte-identical `d16` arms differ by **10.0 percent** on warm medians
and by **0.8 percent** on their minima. Across the whole matrix the median-based floor reaches 9
percent and the tenth-percentile floor stays inside 2.3 percent on eight of nine rows.

So the tenth percentile of the warm samples is the estimator used below, with medians printed
alongside in `27_probes/contention_table.out` so the gap between them is visible rather than a choice
a reader has to take on trust. The tenth percentile rather than the strict minimum, because a
minimum is one sample and a percentile is not.

This is a defect in the instrument rather than in the result, and it is the honest bound on what any
threaded measurement here can claim: **fine distinctions below about two percent are not available at
`T > 1`**, where the single-core sweep resolved below one percent.

## 7. The result

Twelve rows, four column sizes by three thread counts, six arms each. Artifacts:
`mock/benches/bitpack-contention_n<KEY>.csv` plus `.meta.json` and `_findings.md`, `KEY = N*10 + T`.
Every derived figure below comes out of `27_probes/contention_table.py`, whose full output is
committed at `27_probes/contention_table.out`.

Warm-mode tenth-percentile picoseconds per element of the whole column, so a perfectly scaling arm
quarters from `t = 1` to `t = 4`:

| n | t | d16 | d16-control | d32 | d64 | packed | packed-simd |
|---|---|---|---|---|---|---|---|
| 16,384 | 1 | 84.0 | 88.5 | 81.2 | 84.5 | 126.8 | 119.5 |
| 16,384 | 2 | 50.3 | 50.3 | 48.3 | 46.7 | 70.6 | 67.3 |
| 16,384 | 4 | 31.1 | 31.0 | 31.8 | 29.7 | 42.3 | 41.1 |
| 1,048,576 | 1 | 84.7 | 84.7 | 83.6 | 99.4 | 126.8 | 119.4 |
| 1,048,576 | 2 | 43.1 | 43.1 | 42.6 | 53.4 | 64.5 | 60.7 |
| 1,048,576 | 4 | 24.2 | 22.5 | 22.6 | 33.6 | 33.5 | 31.6 |
| 4,194,304 | 1 | 87.5 | 85.5 | 88.3 | 131.5 | 127.5 | 119.9 |
| 4,194,304 | 2 | 43.5 | 44.4 | 61.0 | 128.8 | 65.1 | 61.6 |
| 4,194,304 | 4 | 28.8 | 28.6 | 58.6 | 133.4 | 42.1 | 39.4 |
| 8,388,608 | 1 | 86.6 | 86.9 | 86.2 | 128.7 | 130.5 | 122.1 |
| 8,388,608 | 2 | 48.2 | 45.3 | 66.5 | 129.6 | 66.8 | 66.1 |
| 8,388,608 | 4 | 33.5 | 34.2 | 66.6 | 134.7 | 47.3 | 43.5 |

### 7.1 The sharpest prediction was the right one, and it holds exactly

Section 2 said `d64` should gain nothing at all from extra cores, because at one core it is already
at `w / B`. Speedup of each arm against its own `t = 1` row:

| n | t | d16 | d32 | d64 | packed | packed-simd |
|---|---|---|---|---|---|---|
| 1,048,576 | 4 | 3.50 | 3.71 | 2.96 | 3.78 | 3.77 |
| 4,194,304 | 2 | 2.01 | 1.45 | **1.02** | 1.96 | 1.95 |
| 4,194,304 | 4 | 3.03 | 1.51 | **0.99** | 3.03 | 3.04 |
| 8,388,608 | 2 | 1.80 | 1.29 | **0.99** | 1.95 | 1.85 |
| 8,388,608 | 4 | 2.59 | 1.29 | **0.96** | 2.76 | 2.80 |

**At the two largest sizes `d64` returns 0.96 to 1.02 from four cores.** Four cores walking a `u64`
column do the work in the time one core takes, which is what a wall looks like from the inside.
`d32` gets 1.29 to 1.51 and stops. The packed arms and `d16` keep scaling to 2.6 to 3.0.

The 1,048,576 row is the counter-case that makes the reading a mechanism rather than a coincidence:
there `d64`'s column is 8 MB, inside this host's 12 MB L2, and it scales 2.96 like everything else.
The wall is not a property of the carrier. It is a property of the carrier's footprint against the
cache and the memory controller behind it.

### 7.2 The host's ceiling, measured rather than quoted

Implied aggregate read bandwidth, the whole machine rather than one core:

| n | t | d16 | d32 | d64 | packed-simd |
|---|---|---|---|---|---|
| 4,194,304 | 4 | 69.3 | 68.2 | 60.0 | 41.3 |
| 8,388,608 | 2 | 41.5 | 60.1 | 61.7 | 24.6 |
| 8,388,608 | 4 | 59.8 | 60.1 | 59.4 | 37.3 |

**Three arms with completely different kernels and three different carrier widths all stop at 59 to
60 GB/s at `n = 8,388,608`, `t = 4`.** That convergence is the measurement: this host's aggregate
sequential-read bandwidth is about 60 GB/s, against a theoretical 68.25, and a single core reached
53 to 55 of it in the carrier sweep. So the answer to whether the host can show the effect is not
only yes, it is that the host has no headroom left to hide it in.

The `packed-simd` arm is the one that does **not** converge on the ceiling, at 37.3 GB/s. It is the
only arm still limited by its own work rather than by the machine.

### 7.3 The break-even carrier width, which is what the dispatch asked for

Interpolating dense cost linearly between the measured `d32` and `d64` points and solving for where
it equals the packed cost, exactly as the single-core probe does:

| n | t | vs `packed` | vs `packed-simd` |
|---|---|---|---|
| 4,194,304 | 1 | 7.63 | 6.93 |
| 4,194,304 | 2 | 4.24 | 4.03 |
| 4,194,304 | 4 | 3.12 | **2.97** |
| 8,388,608 | 1 | 8.17 | 7.38 |
| 8,388,608 | 2 | 4.01 | 3.97 |
| 8,388,608 | 4 | 2.87 | **2.65** |

**The break-even carrier width moves from about 7 bytes at one core to about 2.7 to 3.0 bytes at
four**, on the same host, same column, same kernels, same input. That is the answer to the dispatch's
second question and it is a factor of two and a half.

Read as the decision a consumer actually makes, `packed-simd` against each real carrier:

| n | t | vs `u16` | vs `u32` | vs `u64` |
|---|---|---|---|---|
| 4,194,304 | 1 | +37.0% | +35.7% | -8.8% |
| 4,194,304 | 4 | +36.6% | **-32.8%** | **-70.5%** |
| 8,388,608 | 1 | +41.0% | +41.7% | -5.1% |
| 8,388,608 | 4 | +30.1% | **-34.6%** | **-67.7%** |

At one core packing pays only against `u64` and by under ten per cent. At four cores it pays against
`u32` by a third and against `u64` by more than two thirds. **The qualitative answer changes: a
consumer packing a 13-bit field out of a `u32` goes from losing 36 per cent to winning 33.**

### 7.4 Warm is the optimistic mode, and the realistic one moves the answer further

Every table above is warm mode, where the harness re-reads the same column call after call and
whatever fits in L2 stays there. The csvs also carry 240 cold rows per arm, where the harness evicts
first.

The distinction is not a technicality here, it is the difference between two workloads. A column
re-read in a tight loop with nothing else running is warm. A column re-read one frame later, after
the thousands of other systems the substrate's own framing describes have each walked their own
columns, is cold. **For the declared workload, cold is the representative mode**, and it is the one
in which a footprint saving has the most to act on, because nothing is resident by the time the pass
comes round again.

Cold-mode tenth percentiles at the two largest sizes:

| n | t | d16 | d32 | d64 | packed-simd | break-even | packed-simd vs `u16` |
|---|---|---|---|---|---|---|---|
| 4,194,304 | 1 | 87.1 | 97.0 | 159.0 | 121.6 | 5.59 | +39.7% |
| 4,194,304 | 2 | 47.4 | 82.3 | 151.2 | 62.4 | 2.85 | +31.6% |
| 4,194,304 | 4 | 43.8 | 81.4 | 155.0 | 41.7 | **1.84** | **-4.8%** |
| 8,388,608 | 1 | 86.8 | 90.5 | 147.2 | 121.0 | 6.16 | +39.4% |
| 8,388,608 | 2 | 49.0 | 74.6 | 141.9 | 65.1 | 3.43 | +32.7% |
| 8,388,608 | 4 | 42.4 | 76.4 | 145.4 | 43.4 | **2.09** | +2.6% |

**In cold mode at four cores the break-even lands on 1.84 to 2.09 bytes, which straddles `u16`.** At
`n = 4,194,304` the packed arm beats the tightest native carrier a 13-bit value can have, by 4.8 per
cent. At `n = 8,388,608` it is 2.6 per cent behind it, which given the noise floor below is a tie.

The mechanism for the difference between warm and cold is visible in `d16`'s own scaling: warm it
returns 3.03 from four cores at `n = 4,194,304` because its 8 MB column lives in the 12 MB L2 across
calls; cold it returns 1.99, because the same column has to come from memory every time. Nothing
about the packed arm changes between the two modes, which is the point: **an arm that is not
bandwidth-bound does not care what the cache did.**

### 7.5 What the controls say, so the deltas above are readable

The `d16-control` arm is byte-identical to `d16` and its gap is measurement rather than code. On the
tenth percentile it is inside 2.4 per cent on ten of twelve rows, with two outliers at -6.0 and -7.1
per cent. On medians it reaches 10 per cent, which is why the tenth percentile is used, and the
comparison between the two estimators is printed in full in the committed probe output.

So the honest resolution of this instrument is about **two to three per cent at `t > 1`**, against
under one per cent for the single-threaded sweep. Every conclusion above rests on a delta of at least
thirty per cent except one: the 2.6 per cent by which packing trails `u16` in cold mode at
`n = 8,388,608`, which is inside the floor and is reported as a tie rather than as a loss.

## 8. The mechanism attack: the packed decode was carrying a dependency it did not need

Section 7.2 has one arm that never reaches the ceiling. Under contention every dense carrier is
limited by the machine and the packed arm is limited by its own decode, which means the decode is the
only thing left that a change can move, and every picosecond off it converts directly into margin
against the carrier packing has never beaten.

`26` section 8.1 named the next step and did not take it: its `UADALP` kernel removed 31 per cent of
the issued instructions and recovered a third of that in time, and it concluded that the decode chain
(`ld`, `tbl`, `tbl`, `ushl`, `and`, `movn`) is a serial dependency through the vector unit which the
reduction had been hiding, so removing the reduction exposed the decode's latency rather than its
throughput. The fix for an exposed latency chain is more independent chains in flight. The accumulate
is loop-carried on one register, so consecutive groups cannot overlap even though nothing about them
is actually dependent.

Two unroll factors were built rather than one, because which wins is a question about register
pressure and issue width on this core and is not answerable from the source. `sum_padal_pipe2` carries
two accumulators, `sum_padal_pipe4` carries four; both are in
`mock/benches/variants/bitpack-contend-shared/src/kernels.rs` and both are pinned by tests at five
sizes, two of them across the drain boundary and two of them not multiples of the unroll factor.

Run as its own bench section so the twelve committed contention rows were not rewritten to answer a
question about one arm:

```
$ cd mock/benches && ../target/release/arvo-benches --bench bitpack-contend-decode
```

Warm-mode tenth percentiles, picoseconds per element:

| n | t | d16 | d16-control | packed-simd | pipe2 | pipe4 |
|---|---|---|---|---|---|---|
| 16,384 | 1 | 84.0 | 83.9 | 117.6 | 70.1 | 70.1 |
| 16,384 | 4 | 31.0 | 31.0 | 41.0 | 27.4 | 27.2 |
| 4,194,304 | 1 | 88.1 | 87.3 | 120.3 | 75.0 | 72.3 |
| 4,194,304 | 4 | 30.4 | 28.3 | 38.5 | 25.3 | 24.7 |
| 8,388,608 | 1 | 87.3 | 87.6 | 122.3 | 74.5 | 73.5 |
| 8,388,608 | 4 | 33.5 | 33.4 | 40.5 | 26.6 | 27.1 |

**Splitting the accumulator is worth 33 to 40 per cent** against `sum_simd_padal`, on every row, warm
and cold. Against the scalar windowed decode the earlier kernel gained 5 to 7 per cent; this one gains
another 40 on top of that.

**Two accumulators is the whole win and four adds nothing.** `pipe2` and `pipe4` are within 2 per cent
of each other everywhere, and identical at `n = 16384`, which says the machine could overlap exactly
two of these chains and that the earlier kernel was leaving one of them idle.

### 8.1 Why the win is larger than the instruction count predicts

Counted from the built dylibs by walking the disassembly and taking the longest backward branch in
each kernel:

| Kernel | Instructions in the loop | Elements per iteration | Per element |
|---|---|---|---|
| `sum_simd_padal` | 12 | 8 | 1.50 |
| `sum_padal_pipe2` | 18 | 16 | 1.125 |
| `sum_padal_pipe4` | 35 | 32 | 1.094 |

`pipe2` issues **25 per cent fewer** instructions per element and takes **40 per cent less** time. A
change that buys more time than it buys instructions has removed a stall rather than work, which is
the direct confirmation of `26`'s diagnosis: the loop was latency-bound on the accumulate chain, not
throughput-bound on the decode. `pipe4`'s further 3 per cent instruction saving buys nothing, because
the stall was already gone.

The compiler helped in a way worth recording, since it is not obvious from the source: at `Pack<13>`
two groups occupy 26 bytes, and the emitted loop uses `ldr q6, [x17], #0x1a`, a post-increment load
with a 26-byte stride, so the two group loads became one addressing pattern rather than two address
computations.

### 8.2 And that immediately raises the question the result cannot dodge

With the pipelined decode, packing a 13-bit field is **12 to 20 per cent faster than the committed
`u16` dense arm**, at one core and at four, warm and cold. Taken alone that would reverse `26`'s
headline finding outright.

It is not taken alone, because the comparison is now between a dense kernel nobody attacked and a
packed kernel attacked twice, and that is precisely the strawman failure the harness rules name. The
honest next move is to attack the dense side with the same instrument, which section 9 does.

## 9. Attacking the dense side with the same instrument, which is where the answer settles

`sum_d16`'s disassembly is an `ldp q, q` pair, one `bic.8h` mask and four `uaddw` widening adds per
sixteen elements, about one instruction per element, running at 3.5 elements per cycle on an
eight-wide core. That is the same widening cost `UADALP` removed on the packed side, and it is
removable the same way. `sum_d16_padal` and `sum_d32_padal` do it: mask, then pairwise-accumulate into
four independent accumulators, draining on the same period the packed kernel uses. Both are pinned by
tests against the reference kernels they replace, at five sizes, two across the drain boundary and one
not a multiple of the unroll factor.

```
$ cd mock/benches && ../target/release/arvo-benches --bench bitpack-contend-best
```

Six arms: three carriers in their committed form, two of them attacked, and the packed decode before
and after its own attack. Full output at `27_probes/best_table.out`.

### 9.1 The controlled experiment this bench turned into

Pairing each arm with its attacked twin makes the whole file's mechanism testable in one table,
because a kernel improvement on an arm that is already limited by the machine must buy nothing, and
one on an arm that is not must buy everything. Warm mode, what the attack bought:

| n | t | `d16-padal` over `d16` | `d32-padal` over `d32` | `pipe4` over `packed-simd` |
|---|---|---|---|---|
| 4,194,304 | 1 | -62.1% | -23.2% | -41.4% |
| 4,194,304 | 4 | -62.5% | **-2.3%** | -37.3% |
| 8,388,608 | 1 | -60.0% | -24.6% | -40.0% |
| 8,388,608 | 4 | **-16.6%** | **+0.9%** | -37.7% |

**The identical source change is worth a quarter of `d32`'s runtime at one core and nothing at all at
four.** Not approximately nothing: +0.9 per cent at the largest size, which is inside the noise floor
and has the wrong sign. `d16`'s attack goes from -60 per cent to -16.6 per cent as its column crosses
out of L2 and its four copies saturate the machine. The packed arm's attack is worth 37 to 41 per cent
at every size and every thread count, because it is the one arm the machine never limits.

That is the mechanism of this whole file, demonstrated rather than argued: **under contention a dense
column's cost stops being a property of its kernel and becomes a property of its width.**

### 9.2 The answer with both sides attacked

Warm-mode tenth percentiles, picoseconds per element:

| n | t | `d16` | `d16-padal` | `d32` | `d32-padal` | `packed-simd` | `pipe4` |
|---|---|---|---|---|---|---|---|
| 4,194,304 | 1 | 85.7 | 32.5 | 88.5 | 68.0 | 119.8 | 70.2 |
| 4,194,304 | 4 | 28.3 | 10.6 | 58.0 | 56.6 | 37.9 | 23.8 |
| 8,388,608 | 1 | 86.9 | 34.8 | 88.7 | 66.8 | 122.2 | 73.3 |
| 8,388,608 | 4 | 32.5 | 27.1 | 66.4 | 66.9 | 42.3 | 26.4 |

The packed arm against the **best** dense arm of each width, which is the only comparison that is not
a strawman in one direction or the other:

| n | t | mode | `pipe4` vs best `u16` | `pipe4` vs best `u32` |
|---|---|---|---|---|
| 4,194,304 | 1 | warm | +116.4% | +3.3% |
| 4,194,304 | 4 | warm | +123.6% | **-58.0%** |
| 8,388,608 | 1 | warm | +110.7% | +9.6% |
| 8,388,608 | 4 | warm | **-2.5%** | **-60.2%** |
| 4,194,304 | 1 | cold | +59.2% | -17.8% |
| 4,194,304 | 4 | cold | **-4.7%** | **-49.0%** |
| 8,388,608 | 1 | cold | +68.0% | -9.5% |
| 8,388,608 | 4 | cold | **-2.4%** | **-48.4%** |

Section 8's finding does not survive this, and saying so is the point of having built it. **With only
the packed side attacked, packing beat `u16` by 12 to 20 per cent everywhere. With both sides
attacked, packing loses to `u16` by 59 to 123 per cent at one core and ties it under contention.**
The 12-to-20 per cent win was an artifact of comparing an optimised kernel against an unoptimised one,
and the arm that proves it is one anyone could have built in an afternoon.

The one row where warm and cold disagree is `n = 4,194,304`, `t = 4`: warm says packing loses to `u16`
by 124 per cent, cold says it wins by 4.7. The whole difference is that an 8 MB `u16` column stays in
a 12 MB L2 between calls and a 6.8 MB packed column does not need to. At `n = 8,388,608` neither fits
and the two modes agree to within a tenth of a per cent, which is the check that the reading is right.

## 10. The answer

**Does the answer change under contention, and by how much?**

Yes, and the size of the change depends on which carrier is being replaced.

> Packing reduces time only where the walk is bound by bytes delivered rather than by per-element
> work. Adding cores does not change what any arm computes; it multiplies the rate at which the
> column's bytes are demanded, while the machine's supply is fixed. So the condition is not the
> column's size and not the thread count on its own, but whether `T · w / c` exceeds the machine's
> aggregate bandwidth, where `w` is the carrier's width in bytes and `c` the per-element cost of the
> kernel reading it. Above that point an arm's cost is `w / B` and depends on nothing else, so cost
> becomes proportional to footprint and the kernel stops mattering.

Instantiated on this host, for a 13-bit field, with the best kernel available on each side:

| The carrier packing replaces | One core | Four cores |
|---|---|---|
| `u64` (8 bytes) | wins by 5 to 9% | wins by 67 to 71% |
| `u32` (4 bytes) | loses by 3 to 10% warm, wins by 10 to 18% cold | **wins by 48 to 60%** |
| `u16` (2 bytes) | loses by 59 to 111% | **ties**, within 2.5% either way, on a column past L2 |

And the break-even carrier width, which is the form the dispatch asked for:

| | one core | four cores |
|---|---|---|
| committed kernels, warm | 6.9 to 7.4 bytes | 2.7 to 3.0 bytes |
| committed kernels, cold | 5.6 to 6.2 bytes | 1.8 to 2.1 bytes |
| best kernels on both sides, warm | above 8 bytes | about 1.7 bytes |

**The break-even moves by a factor of two and a half to four.** It is the single number the canon's
claim turns on and it is not stable under thread count, so a canon that carries the inequality and not
the number is the only version that survives.

### 10.1 What does not change, which is the more consequential half

Two things the contention regime leaves exactly where the single-core sweep put them.

**The direction of the argument.** `26` concluded that packing pays when the bytes it saves, divided
by available bandwidth, exceed the decode cost it adds. Every measurement here obeys that inequality;
what moves is the denominator, because four cores share one supply. The mechanism was right at one
core and is right at four, which is worth stating because it means the single-core work was not
wasted, it was under-parameterised.

**Column size is still not the condition.** `26` said plainly that making the column bigger does not
get you there, and that survives: `d64` is bandwidth-bound at `n = 4,194,304` and at `n = 8,388,608`
and gains nothing from cores at either, while at `n = 1,048,576` it fits in L2 and scales like
everything else. The condition is the working set against the cache, and after that the demanded rate
against the supply. Records are a proxy for both and a bad one.

### 10.2 What the host cannot tell you

Specific, because these are properties of this machine rather than of packing.

**Four performance cores and one memory controller.** This part's cores are unusually strong relative
to its memory system: one core reaches 53 to 55 GB/s of an aggregate 60. On a machine with sixteen
weaker cores behind the same bandwidth, contention would arrive at the same aggregate demand but at a
very different thread count, and the per-core compute floors would be higher, so the crossings would
move. Nothing here says where.

**A 12 MB last-level cache.** Every warm-against-cold disagreement in this file is a statement about
that 12 MB. A machine with a 64 MB last-level cache would keep a `u16` column resident to four times
the record count and would push the crossing correspondingly further out; one with 2 MB would pull it
in. The `n = 4,194,304` row is entirely a story about 8 MB against 12 MB.

**A single NUMA domain and uniform cores.** Threads here contend for one controller. On a multi-socket
part, or an asymmetric one where some threads land on efficiency cores, the wall is not a single
number and the static equal-slice split this bench uses would be the wrong shape.

**Read-only.** Every arm reads. A pass that writes a packed column pays an encode surcharge and gets
the saving on both sides, and nothing here prices it.

**Aggregate bandwidth is measured, not quoted.** The 60 GB/s figure is where three arms with different
kernels and different widths converge, which is the right way to get it, but it is a sequential-read
figure on one access pattern. Random or strided access would give a different and lower number, and
the crossings would move again.

### 10.3 What is op's

Nothing here settles anything and none of it is a call. Two questions surfaced that are his.

**Whether the canon's claim is about one core or about the declared workload.** The substrate's own
rule describes millions of entities and thousands of systems per frame, which is parallel and is what
this file measured. The single-core answer and the four-core answer differ by a factor of two and a
half in the break-even, and against `u32` they differ in sign. Which one the canon means is a question
about intent rather than about a number, and the experts have not converged on it because until now
only one of the two had been measured.

**Whether the canon should carry the inequality alone.** `26` read it as the canon carrying the
conditional and the audit trail carrying the instantiation, and everything measured here supports that
reading and sharpens the reason: the instantiation moved by a factor of four between one core and four
on the same machine, so a number in the canon would have been wrong within a day of being written. But
the reading is still a reading, and `arvo-toolbox-not-policer.md` is the rule it has to be checked
against.

## 11. Fidelity, demonstrated rather than asserted

Four checks, each with its output committed in `27_probes/`.

**The validation fires.** A `wrapping_add(1)` was injected into `kern_d32`, the arm was rebuilt, and
the contention sweep was rerun. The harness refused before timing anything:

```
  Validating 6 variants × 100 seeds...
  MISMATCH seed=17010672633609114990 (#0):
    bitpack-contend-d16 vs bitpack-contend-d32
    first diff at byte 0: 234 vs 235
error: bench `bitpack-contention` n=163841 failed validation, refusing to report timings
  for arms that do not agree: validation failed for variant bitpack-contend-d32:
  100 mismatches across 100 seeds
```

All 100 seeds caught it, the offending arm was named, and no csv was written: the committed
`bitpack-contention_n163841.csv` hashed `d0fec2486861c297eadbd14b0035ffd7` before and after. Full
output at `27_probes/fidelity_injection_output.txt`. The defect was reverted, the arm rebuilt, and all
twelve unit tests pass.

**No loop was deleted and no arm exceeds the machine.**
`27_probes/deletion_and_roofline.py` reads all twenty-two committed rows across the three sections and
checks that per-element cost does not collapse as the column grows, that total time grows close to
linearly, and that no arm whose working set exceeds L2 reports a bandwidth past the ceiling. Every row
passes; the highest figure past L2 anywhere is 73.9 GB/s.

One mistake in writing that probe is recorded in it, because the wrong version is the tempting one.
The first version bounded the per-element ratio on **both** sides, at 0.5 to 3.0, and failed `d64` at
`t = 4` with a ratio of 4.53. That is not a deleted loop, it is a `u64` column crossing out of L2 and
hitting the wall, which is the finding of this whole file. The wrong repair would have been to widen
the upper bound until it passed. The right one is to have no upper bound at all, because a rising
per-element cost was never part of what deletion looks like.

**The noise floor is a real arm, not an assumption.** `bitpack-contend-d16-control` calls the identical
kernel on the identical region with identical arguments, and its gap against `d16` is reported on every
row in section 7.5 rather than quoted once.

**My own citations were checked by opening them**, using the tool `25` built and `26` extended.
`27_probes/verify_my_citations.py` opens every `file:line` this document cites, tests the content of
the cited line rather than its existence, and recomputes every headline number straight from the
committed csvs. It resolves the harness checkout from `mock/Cargo.lock` rather than hardcoding it, so
it fails loudly if the pin moves instead of checking a stale copy. **29 of 29 pass.**

## 12. What I had to decide, and which of it the numbers turn on

Six decisions the design does not specify. The first two change the answer.

**That the dense side had to be attacked too.** Section 8 produced a clean 12-to-20 per cent win for
packing and section 9 destroyed it. Had I stopped at section 8 this file would have reported the
opposite conclusion with an equally clean table behind it. **The answer turns entirely on this**, and
it is the decision most worth a second reader's attention: the bar for a competitor arm is not "the
kernel that was committed", it is "the kernel someone would write if they cared about this arm".

**Warm against cold.** They disagree by a factor of five on one row and agree to a tenth of a per cent
on another, and the difference is whether an 8 MB column stays in a 12 MB L2 between calls. I read
cold as the representative mode for a workload of thousands of systems per frame, because nothing is
resident by the time a pass comes round again, and both are reported throughout so that reading can be
disagreed with without redoing the work. **The answer turns on this too**, at `n = 4,194,304`.

**The tenth percentile rather than the median.** Justified by the control in section 6.2 rather than by
preference, and both are printed everywhere. If a reader prefers medians the conclusions are unchanged
in direction and noisier in magnitude.

**One column split `T` ways rather than `T` columns.** Stated in section 3, and it is the less
favourable of the two available shapes for packing, which is why it was chosen.

**Thread counts of one, two and four, not eight.** Eight threads on this part means four performance
cores and four efficiency cores with different clocks, different L1s and a different L2, and a static
equal-slice split would then be measuring the slowest thread. That is a different regime and it is not
covered.

**Thirteen bits, and carriers of two, four and eight bytes.** Inherited from every sibling bench in the
directory rather than chosen. The break-even is a function of both the field width and the carrier
width and only one of those was swept.

## 13. What I did not cover

**Eight threads, and the efficiency cores.** Named above. The pool supports it and every key is
declared for it; no row was run.

**Writes.** Every arm reads. A pass that writes a packed column pays an encode surcharge and gets the
saving on both sides, and the trade is not symmetric with the read case.

**Random and strided access.** Every arm walks sequentially. Under an access pattern the prefetcher
cannot follow, capacity stops being hidden and the whole shape of this file changes; `26` named this
gap too and it is still open.

**`d64` with its kernel attacked.** `sum_d64` was left as committed, on the reasoning that it is
bandwidth-bound at every row where it matters, which `d32-padal`'s null result at `t = 4` supports by
analogy rather than directly. The `t = 1` break-even interpolations in section 10 inherit whatever
slack `sum_d64` has, and they are the numbers I would trust least in this file.

**Whether `pipe2` and `pipe4` are the best packed decode available.** They are the best in this
directory by 33 to 40 per cent and the instruction counts say the stall is gone, but nothing here
proves a third shape would not do better.

**More than one host.** One M1, four performance cores, a 12 MB L2, about 60 GB/s aggregate. Section
10.2 lists what that specifically cannot tell you.

**Whether the six earlier bitpack families are individually sound.** I read `bitpack-carrier-*`
closely because this file extends it, and took the rest at face value.

**The footprint benefit itself.** Packing 8 bytes to 1.625 is a five-fold reduction in resident memory
and that is real whether or not any loop is faster. A consumer whose column does not fit in the machine
is not choosing between 26 and 33 picoseconds, it is choosing between running and not running. `26`
said this and it is still unpriced.

## 14. Alternatives I did not take, for whoever picks this up

**A larger `MAX_N`, so both arms are comfortably past L2.** The sharpest row in this file,
`n = 8,388,608`, has a 16 MB `u16` column against a 12 MB L2 and a 13.0 MB packed column against the
same, so both are only just past it and partial residency is still in the numbers. A layout carrying
only the `u16` and packed regions could reach 32M records inside the same allocation and put both arms
four to five times past the cache, where the comparison would be purely about bytes. This is the
cheapest remaining improvement to the result and I would take it first.

**Perf counters.** The harness supports `--perf-counters` under sudo and emits instructions and cycles
per sample. Every mechanism claim in sections 8 and 9 is inferred from disassembly plus timing; measured
IPC would settle them directly, and would answer whether the residual gap between `pipe4`'s issue-width
floor and its measured cost is decode latency or something else.

**A thread count sweep at finer resolution.** One, two and four bracket the wall but do not locate it.
Three would say whether `d16` at `n = 8,388,608` saturates between two and four or exactly at four, and
the shape of that curve is the thing a canon sentence about "concurrent consumers" would most want.

**Pinning threads to specific cores rather than to a QoS class.** macOS offers no real affinity on this
part, so the placement is the scheduler's. A Linux host with `sched_setaffinity` would remove one source
of the two-to-three per cent noise floor and make the eight-thread regime measurable.

**Measuring the barrier directly with an empty kernel.** Section 5.1 prices it indirectly from the
small-`n` rows at 50 to 230 nanoseconds. An arm whose kernel returns zero immediately would price it
exactly, and would cost one more variant.

**A `u24` or `u48` carrier.** The break-even lands between two and four bytes under contention and the
sweep has no point in that interval, so every figure in that band is an interpolation between `u16` and
`u32`. A three-byte carrier is a real thing a consumer might use and it sits exactly where the answer is.
