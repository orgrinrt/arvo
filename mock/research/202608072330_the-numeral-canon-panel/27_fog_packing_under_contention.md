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

A measurement that disagrees with arithmetic means one of them is misconfigured, and knowing which
is the job, so the bound goes first.

Let an arm have compute-bound per-element cost `c` picoseconds and carry `w` bytes per element. With
`T` threads each walking a disjoint slice of one column, aggregate byte demand is `T * w / c`. Where
that exceeds the machine's aggregate supply `B`, the arm stops being compute-bound and its
per-element cost becomes `T * w / B`, which is the same for every arm of the same width regardless of
how cheap its kernel is.

Taking `c` from the single-core table above and `B` at the 55 GB/s a single core already demonstrates
is reachable:

| Arm | `w` | `c` at T=1 | Demand at T=4 | Predicted cost at T=4 |
|---|---|---|---|---|
| d16 | 2 | 87 ps | 92 GB/s | bandwidth-bound, about 145 ps |
| d32 | 4 | 88 ps | 182 GB/s | bandwidth-bound, about 291 ps |
| d64 | 8 | 146 ps | 219 GB/s | bandwidth-bound, about 582 ps |
| packed-simd | 1.625 | 122 ps | 53 GB/s | at the edge, about 122 to 130 ps |

The prediction is therefore sharp and falsifiable: **at four cores every dense arm becomes
bandwidth-bound and the packed arm does not**, because the packed decode is expensive enough that
four copies of it still fit under the ceiling. If that holds, the break-even carrier width collapses
from 5.8 to 7.0 bytes toward **2 bytes or below**, and packing would win against `u16`, which is the
carrier it has lost to at every size in every committed bench in this directory.

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
