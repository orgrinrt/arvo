# 78. The thread axis: what Q32 got right, what it got wrong, and what a write costs

**Date:** 2026-08-09. **Member:** Aaltonen. **Status:** complete. Probes and artifacts committed
alongside this file; bench artifacts committed in `mock/benches/`.

**Assignment:** every doability claim in this panel is workload evidence as much as type-system
evidence, and `OPTIONS.md` Q32 says no instrument here has measured above one thread. Build and run
real benches on the mockspace harness that put the thread axis under the packing trade, with real
competitor arms and cross-validated arms, and report where the answer holds and where it does not.

Written as a sequence: attempt, what it hit, what was tried against it, what it measured, the next
attempt.

## 0. Gate and orientation

**Canon gate.** No ratified canon exists for this question. `01_op_answers.md` section 0 is explicit
that op's word is the only thing that ratifies and ratification follows convergence, so there is
nothing here to defend or contradict. This file is a new instrument, not a design decision.

**Toolchain, verified rather than assumed.**

```
$ rustc --version
rustc 1.98.0-nightly (57d06900f 2026-05-27)
```

Matches the brief's `nightly-2026-05-28` pin.

**The tree at dispatch start.** `mock/crates/` is empty (deleted for the canon work, commit
`929c46ed`). `mock/benches/` holds the panel's committed harness output. Branch
`feat/arvo-shape-topic`. `git status --porcelain` at dispatch start showed roughly fifty modified or
untracked paths, none of them mine; another expert's in-flight work. No `git restore`, no `git clean`,
no `git stash`, no branch switch. Every stage in this file names its own paths.

## 1. The brief's central claim, checked before reasoning from it

The brief states, quoting `OPTIONS.md` Q32 nearly verbatim, that "every instrument this panel has
built runs on one thread" and that "nothing in three units has touched it." Per `RULES.md` and the
panel's own recorded lesson (`RULES.md:358-380`, "the reading list needs a slot for the repository"),
a negative claim about evidence is a claim about a place and is checkable in one command.

```
$ ls mock/benches/variants | grep -c contend
16
```

**The claim is false, and it was false when Q32 was written.** Two files already exist and are
extensive: `26_aaltonen_does_packing_pay.md` built `bitpack-contend-shared` (commit `3454060d`, "bench:
add the contention sweep, one column split T ways") and `27_fog_packing_under_contention.md` extended
it through four more commits, the last being `41d2de2e` ("bench: add the wide sweep, both columns
several times past L2"). Both are committed, both are real threaded benches on the mockspace harness,
both measure `T = 1, 2, 4` over the exact packing trade Q32 asks about, with real thread pools, QoS
pinning, false-sharing-safe partials, and independent fidelity checks including deliberate fault
injection. `32_op_arvo_adapts_to_the_cores_it_finds.md` is op's own ratifiable-intent answer to a
question `27` returned, dated the same day.

**Where Q32's register entry went wrong, mechanically.** Its own file history shows it: `OPTIONS.md`
lists Q32 as added late, positioned immediately after a `73`-sourced amendment to Q21, and it names I10
from file `32`, which is numbered *after* `26` and `27` in the panel's own sequence. Whoever wrote the
Q32 entry had `26`, `27` and `32` all available and stated the opposite of what they show. This is not
a small transcription slip: Q32's own listed exposures ("a sub-byte packed field shared between
threads is a read-modify-write on a byte two threads both want") is a real and correctly-identified
mechanism, just one `27` had already extensively measured for reads, and its blanket framing
("nothing... has touched it") erased that work from the register a third time, the same loss mechanism
`RULES.md:1770-1776` already names for lost options.

**What is genuinely still open**, once the false half of Q32 is set aside: `26` section 12 and `27`
section 13 both explicitly flag the same residue. `26`: "Writes. Every arm reads... The trade is not
symmetric with the read case and is unmeasured." `27`: "Writes. Every arm reads. A pass that writes a
packed column pays an encode surcharge and gets the saving on both sides... unmeasured." **Nobody has
measured a write.** Q32's own sharpest sentence, the read-modify-write-on-a-shared-byte exposure, is
real precisely because it is a write hazard, and reads never had it: `27` section 3 pins that its
reads are always split on period boundaries, so no byte a read touches is ever also touched by another
thread. A write does not get that for free unless the caller's split happens to respect the period,
and nothing in this panel or in `hilavitkutin`'s morsel model guarantees that a generic parallel split
does.

**This is the hole this file goes into.** Not "is packing's read-side trade regime-sensitive", which
`27` already answered in full generality (`27:653-708`, the inequality survives one core to four,
unchanged in form). The hole is: what does a **write** to a packed column cost under contention, is it
even **correct** when the caller's thread split does not respect the packed period, and if not, what
does making it correct cost.

## 2. What "the write hazard" is, precisely, before building anything

A logical column of `N` 13-bit values, packed with no inter-value padding (period 8, group 13 bytes,
matching `26`/`27`'s own `Plan13`). A thread walking `[lo, hi)` writes bits `[lo * 13, hi * 13)`. If
`lo * 13` is not a multiple of 8, the byte containing that bit also holds the tail of element `lo - 1`,
owned by the previous thread. If `hi * 13` is not a multiple of 8, the byte containing it also holds
the head of element `hi`, owned by the next thread. Since a 13-bit field spans at most three bytes,
each internal thread boundary has **at most one** such shared byte at each end, computable from `lo`
and `hi` alone with no knowledge of the neighbour: `split_is_guarded(at, n) = at != 0 && at != n && (at
* 13) % 8 != 0`.

Two ordinary (non-atomic) read-modify-write operations on that byte, issued from two different
threads with no synchronisation, is a data race in the ordinary sense: whichever store lands last
silently discards the other's bits.

**Whether a real parallel split hits this is a property of the caller, not of the packing.** `26`'s and
`27`'s own contention bench forces every split onto a period boundary (`KEY_SPLITS`, `bitpack-contend-
shared/src/routine.rs:29-33`), which is exactly why their reads never see it. A generic morsel split
(`N` elements into `T` roughly equal pieces) has no reason to respect an arbitrary field's period, and
nothing in `hilavitkutin`'s own model was read as guaranteeing it.

## 3. Building the instrument

New crate `bench-bitpack-write-contend-shared`
(`mock/benches/variants/bitpack-write-contend-shared/`), modelled on `bitpack-contend-shared`'s own
shape: a persistent thread pool (`pool.rs`), a `Layout` with a read-only `vals` truth region and two
scratch output regions (`dense_out`, `packed_out`), and a `WriteContend<KEY>` routine with `KEY = N *
10 + T`, the same idiom.

**What differs from the read bench, and why.** A write kernel's result is not a return value; it is
the state of the buffer it wrote, so correctness is checked by decoding what was written and comparing
to the `vals` truth, through the same independent `sum_naive` decoder every read bench in this
directory uses as its own ground-truth check (`bitpack-write-contend-shared/src/lib.rs`,
`decode_packed_sum`). A defect in a write kernel and a defect in the shared decode would have to agree
by coincidence to hide from this check, which is the same argument `26` and `27` make for their own
cross-checks.

**Three encoders, one shared per-byte primitive.**

- `write_packed_plain`: naive, index-driven, OR-based, one to three byte read-modify-writes per
  element, no atomics ever. Correct where the split is period-aligned by construction. Not correct
  otherwise, which is the point of building it.
- `write_packed_guarded`: identical, except the one byte at each end of a thread's own range that
  `split_is_guarded` names goes through an atomic fetch-or (`AtomicU8::from_ptr`, `Relaxed`); every
  other byte is plain. Computed from `(lo, hi, n)` alone, no coordination with the neighbour beyond the
  atomic instruction itself.
- `write_packed_windowed`: the mechanism attack in section 7, built after the first timed numbers
  showed the naive encoder was not merely unsafe but very slow.

**Adversarial ordering, stated rather than hidden.** Every kernel processes each thread's own
boundary-adjacent elements first, then its interior (`for_each_boundary_first`,
`kernels.rs:96-118`). A sequential in-order pass would spread a thread's riskiest write across the
whole duration of its slice; processing it immediately after every thread is released together
concentrates the moment two threads are most likely to be mid-write on the same byte. This is stated
the same way `27` states its own choice of the less favourable contention shape (`27:180-213`): the
honest test is the one built to observe the hazard, not the one built to hide it.

## 4. Correctness, demonstrated rather than asserted

**Sequential calls never race, at any split, which is the control that separates encoding correctness
from concurrency correctness.** `kernels.rs::tests` runs every encoder through sequential (non-
concurrent) splits at both period-aligned and deliberately misaligned `(N, T)` pairs; all pass. The
misalignment itself is asserted rather than assumed:
`chosen_sizes_land_where_the_bench_needs_them_to` computes the actual split points for the sizes this
crate uses and checks they land where the file claims.

**Real concurrent corruption, measured.** `stress.rs` runs the naive encoder through the actual thread
pool, with a fresh zeroed buffer per trial, at a small deliberately misaligned size (`N = 4094`, `T =
4`, chosen so `1023 % 8 = 7`, verified in the same file) and counts disagreements with ground truth.

```
naive_kernel_corruption_rate_under_real_concurrency: t=4 497/3000 trials disagreed with ground truth
```

Rerun independently:

```
naive_kernel_corruption_rate_under_real_concurrency: t=4 575/3000 trials disagreed with ground truth
```

**16.6% and 19.2% of passes, on the same host, same kernel, same split, real OS threads.** Not a
theoretical race: a measured, reproducible failure rate on Apple M1 hardware.

**Two controls isolate the mechanism.** The guarded kernel, same size, same thread count, same real
concurrency: `guarded_kernel_never_corrupts_under_real_concurrency`, 0 of 500 across both runs. The
naive kernel at a period-aligned size instead: `naive_kernel_never_corrupts_when_the_split_is_aligned`,
0 of 1000. **The corruption is present only when both conditions hold at once: misaligned split and
real concurrency.** Neither alone produces it. Thirteen unit tests total in this crate, all pass.

**The harness's own validation independently caught the same defect, unprompted, in the timed sweep.**
`bitpack-write-contend-race` at `n = 655342` (`N = 65534`, `T = 2`):

```
Validating 3 variants × 100 seeds...
MISMATCH seed=1248791805238062209 (#42):
  bitpack-write-dense vs bitpack-write-unsound
  first diff at byte 1: 44 vs 32
MISMATCH seed=2230964230368286664 (#56):
  bitpack-write-dense vs bitpack-write-unsound
  first diff at byte 1: 19 vs 15
error: bench `bitpack-write-contend-race` n=655342 failed validation, refusing to report timings for
arms that do not agree: validation failed for variant bitpack-write-unsound: 2 mismatches across 100
seeds
```

The mockspace harness refused to report a timing for the whole size row rather than time an arm it had
not shown to be correct. That refusal is a stronger statement than any timing number would have been:
`41_dispatcher_note_no_bench_here_has_ever_checked_its_answers.md` found the harness's own auto-
validation is otherwise never invoked (`run_orchestrator` never calls `validate::validate`); `26` and
`27` had to build their own fidelity checks to compensate. This bench is the first in the directory
whose validation gate fired on a genuine defect during an ordinary run rather than during a deliberate
fault-injection test.

**Consequence for the timed comparison.** The unsound arm cannot be timed at `T > 1` through the
harness, by the harness's own correct refusal. `bench.toml` keeps it only at the `T = 1` rows (no
boundary exists there; all three arms agree and the row runs clean) and drops it from the `T = 2` and
`T = 4` rows, with the mismatch above committed as the reason.

## 5. The safe comparison, timed

`bitpack-write-contend-safe`: every internal boundary period-aligned by construction
(`chosen_sizes_land_where_the_bench_needs_them_to` pins `N = 65536` and `N = 2097152` at `T = 1, 2, 4`
all land clean). Dense `u16` write against the naive packed encoder, run through the harness.

Function-under-test medians, nanoseconds:

| N | T | dense | packed (naive) | ratio |
|---|---|---|---|---|
| 65,536 | 1 | 9781 | 215387 | 22.0x |
| 65,536 | 2 | 8529 | 169221 | 19.8x |
| 65,536 | 4 | 7936 | 121776 | 15.3x |
| 2,097,152 | 1 | 291690 | 6297861 | 21.6x |
| 2,097,152 | 2 | 237868 | 4537385 | 19.1x |
| 2,097,152 | 4 | 348726 | 6208342 | 17.8x |

**The packed write loses to the dense write by fourteen to twenty-two times, at every size and every
thread count measured.** This is a different regime from anything `26` or `27` reported for reads,
where the widest single-core loss was under two times and packing won outright past four bytes of
carrier width or under contention. Section 7 attacks the mechanism; section 8 states why the gap does
not close the way the read case's did.

## 6. Does contention narrow the write gap the way it narrowed the read gap

The ratio does shrink with thread count (22.0x to 15.3x at the small size, 21.6x to 17.8x at the
large one), the same direction `27` found for reads, but nowhere near the same magnitude: `27` found
the read break-even carrier width fall by a factor of two and a half to four between one core and
four, flipping sign against `u32`. Here the ratio moves by at most 30%, never approaches parity, and
the encoding stays the dominant cost at every thread count measured.

**Dense itself does not scale with thread count here, which is the mechanism.** At `N = 2,097,152`:
291690ns at `T=1`, 237868ns at `T=2` (1.23x), 348726ns at `T=4` (0.84x, slower than one thread). `27`'s
own dense read scaled 2.96 to 3.78x across the same thread range at comparable sizes
(`27:388-392`). **A write moves twice the bytes a read of the same element count does**, the input
read plus the output write, so a write-bound loop meets this host's aggregate memory bandwidth ceiling
(`27` measured roughly 60 GB/s aggregate, `27:413-417`) at a lower thread count than a read-only loop
does. The `T=4` regression specifically is consistent with a second, host-topology mechanism `27`
already named as untested: this M1 has four performance cores, and a `T=4` pool plus the coordinator's
own timing loop is five participants contending for four P-cores, which `27` declined to measure past
`T=4` for exactly this reason (`27:839-842`). Both mechanisms move the same direction and I did not
separate them; naming both is more honest than picking one.

## 7. The mechanism attack: the naive encoder was not merely unsafe, it was slow

Before trusting the 15x to 22x gap in section 5 as a property of packing rather than of one
unoptimised encoder, the encoder itself needed the same scrutiny `26` gave the read-side decoder
(section 8 of that file, which found the committed SIMD decoder losing to the scalar one and fixed
it).

`write_packed_plain` costs up to three byte-granular read-modify-writes per element, with the byte
index and bit shift recomputed from a fresh division and modulus every element, and no batching. The
read side's `sum_windowed` (`bitpack-plan-shared/src/lib.rs:213-235`) does the equivalent job with two
unaligned 64-bit loads and eight shift-mask extracts per **group** of eight elements, because a
group's window layout is fixed at the width and known at monomorphisation. The encode side has the
identical structure available and nothing in this crate had used it until this section.

`write_packed_windowed` builds each group's window value in registers (one OR per lane into up to two
64-bit accumulators) and merges each window into the buffer with one unaligned load, OR, unaligned
store. The merge, not a blind store, because a group's two windows can overlap in byte range, so a
plain store from the second window would erase bits the first window's write already deposited into
the shared bytes.

Pinned by two new tests: `windowed_agrees_single_call` (0, 1, 2 periods, and 65536 elements) and
`windowed_agrees_split_across_period_aligned_calls` (both crate sizes, `T = 1, 2, 4`). Both pass.

Function-under-test medians, nanoseconds, windowed against the naive baseline it replaces:

| N | T | naive | windowed | improvement | windowed vs dense |
|---|---|---|---|---|---|
| 65,536 | 1 | 215387 | 130007 | 39.6% | 13.3x |
| 65,536 | 2 | 169221 | 116896 | 30.9% | 13.7x |
| 65,536 | 4 | 121776 | 97654 | 19.8% | 12.3x |
| 2,097,152 | 1 | 6297861 | 3671753 | 41.7% | 12.6x |
| 2,097,152 | 2 | 4537385 | 3157149 | 30.4% | 13.3x |
| 2,097,152 | 4 | 6208342 | 4120791 | 33.6% | 11.8x |

**A real win, twenty to forty-two percent depending on size and thread count, from the same
observation `26` made on the read side: a byte-at-a-time encoder pays for work a group-at-a-time one
does not need to repeat.** And a real limit: even attacked once, the packed write still loses to dense
by twelve to fourteen times, an order of magnitude past anything the read side ever showed.

### 7.1 What the windowed attack does not reach, and why

The windowed merge does one unaligned load and one unaligned store per window, against the read side's
one load per window. A write is structurally a round trip through the same cache line a read only
visits once, and that is not a naive-encoder artefact; it is what OR-based accumulation into a shared
byte costs. Two further attacks were not attempted and are named for whoever picks this up:

**Vectorising the windowed merge.** `26` section 8 and `27` section 8 both found the read-side decode
was latency-bound on a loop-carried accumulator and fixed it with independent NEON accumulators
(`sum_padal_pipe2`, `sum_padal_pipe4`, worth thirty-three to forty percent on top of the first SIMD
attempt). The write side's window-merge loop is not yet vectorised at all; whether the same pairwise-
independent-chain trick applies to a load-OR-store merge, where the store itself is a dependency the
read-only decode never had, is untested.

**A windowed-and-guarded encoder.** Section 8 below measures the atomic guard on top of the naive
encoder, not the windowed one. Building the windowed encoder's group merge so only the specific window
overlapping a guard byte takes the atomic path, and every other window in the group stays plain, was
not attempted; nothing about the group structure prevents it, and it is the more valuable of the two
because it is what a real caller with an arbitrary morsel split would want to run.

## 8. What correctness costs on top of the naive encoder

`bitpack-write-contend-race`: deliberately misaligned splits (`N = 65534`, `N = 2097150`, both
verified misaligned by the same test that verifies the safe sizes are aligned). Dense against the
guarded (correct-at-any-split) packed encoder.

| N | T | dense | packed (guarded) | ratio |
|---|---|---|---|---|
| 65,534 | 1 | 8696 | 234465 | 27.0x |
| 65,534 | 2 | 7644 | 162291 | 21.2x |
| 65,534 | 4 | 8341 | 146410 | 17.6x |
| 2,097,150 | 1 | 311143 | 6953439 | 22.4x |
| 2,097,150 | 2 | 317970 | 6191502 | 19.5x |
| 2,097,150 | 4 | 306426 | 5115274 | 16.7x |

**Against the naive (unsound, correct only where the split happens to be period-aligned) encoder,
measured at the one row it could be timed** (`N = 65534`, `T = 1`, no boundary exists, all three arms
agree): naive 220915ns, guarded 234465ns, a 6.1% overhead for the atomic guard. At `N = 2097150`, `T =
1`: naive 6382456ns, guarded 6953439ns, 8.9%.

**The safety fix costs six to nine percent on top of an unsafe baseline, and the baseline itself is
the expensive part.** The guarded arm's absolute cost tracks the naive encoder's cost (both are byte-
at-a-time), not the atomic instruction's cost. Section 7.1's unattempted windowed-and-guarded encoder
is where that fix would actually pay off, by carrying the twenty to forty percent windowed win forward
onto a kernel that is also correct at an arbitrary split.

## 9. The answer, with predicates

Every claim below states the region it holds in. Nothing outside the stated region is claimed.

**Packing a 13-bit field into a written column loses to a dense `u16` write.**

```
holds for: W = 13, carrier = u16, access = sequential, threads = 1, 2, 4,
           N = 65536, 2097152, host = Apple M1 (4 P-core, 12 MiB shared L2,
           ~60 GB/s aggregate sequential read/write bandwidth, per 27:413-417)
```

By eleven point eight to twenty-two times against the windowed encoder, and by fourteen to twenty-
seven times against the naive one. The magnitude is far past anything the read side of this trade ever
showed at any width or thread count `26` or `27` measured.

**A naive parallel write to a packed column, split at an arbitrary boundary, corrupts its own output.**

```
holds for: W = 13, split misaligned to the packing period,
           threads = 4, N = 4094 (stress), N = 65534, 2097150 (harness),
           host = Apple M1
```

Sixteen point six and nineteen point two percent of trials (real repeated measurement, not a
theoretical race), and independently caught by the mockspace harness's own cross-arm validation on an
ordinary run, at `N = 65534`, `threads = 2`.

**Guarding the one shared byte at each internal boundary with an atomic fetch-or restores correctness
at any split, at a cost of six to nine percent over the unsafe baseline.**

```
holds for: W = 13, threads = 1 (measured overhead point; only T=1 row ran
           all three arms), host = Apple M1
```

**A windowed (group-at-a-time) encoder beats the naive (element-at-a-time) one by nineteen point eight
to forty-one point seven percent, at a period-aligned split.**

```
holds for: W = 13, split period-aligned, threads = 1, 2, 4,
           N = 65536, 2097152, host = Apple M1
```

**Write bandwidth saturates this host at a lower thread count than read bandwidth does.** The dense
write does not scale past `T = 2` at `N = 2097152` on this host, against the dense read's roughly
three-fold scaling to `T = 4` that `27` measured at comparable sizes.

```
holds for: carrier = u16, write (not read), access = sequential,
           N = 2097152, threads = 1, 2, 4, host = Apple M1
```

### 9.1 What this says about Q32 and about op's I10

Op's I10 (`32_op_arvo_adapts_to_the_cores_it_finds.md`) takes no stance on thread count and says arvo
adapts to what it finds, most efficient in each situation, without sacrificing soundness (per `34`,
hard for every strategy but `Hot`). Applied here: a consumer choosing `Cold`-style packing for a
column it will **write** in parallel needs to know two things this file establishes and neither prior
file measured. First, the write-side trade is not the read-side trade; a design sentence carrying
`27`'s read-side inequality and applying it to writes would be wrong by an order of magnitude, not by
degree. Second, correctness under an arbitrary parallel split is not automatic the way it is for
reads; a design that lets a consumer write a packed column from several threads without stating the
split-alignment requirement, or without providing the guarded path, licenses the corruption measured
in section 4.

Q32 as currently written in `OPTIONS.md` should be corrected rather than left standing; section 11
states the correction and applies it.

## 10. What I did not cover

**Vectorising the encoder, and the windowed-guarded encoder.** Named in 7.1. The single largest
remaining lever on the numbers in this file.

**Threads past four.** `27`'s own reason applies unchanged here: eight threads on this part mixes four
performance cores with four efficiency cores of different clock and cache, and a static equal-slice
split would measure the slowest thread rather than the memory system.

**Widths other than thirteen, and carriers other than `u16`.** One point in a two-dimensional space,
inherited from every sibling bench in this directory rather than chosen.

**Random or strided write access.** Every arm here writes sequentially. `26` and `27` both name this
gap on the read side too; it is wider open on the write side, since a strided write to a packed
column multiplies the number of guarded bytes rather than confining them to `T - 1` boundaries.

**Reads that follow the write in the same pass**, which is the shape a real column update actually
has (read old value, compute, write new value) rather than the pure encode this file measures.

**Cold mode.** Every number in this file is warm. `27` found cold mode moves the read-side verdict
further in packing's favour at some sizes (`27` section 7.4); whether cold mode narrows or widens the
write-side gap is unmeasured.

**More than one host.** One M1, stated in every predicate above. `27`'s own section 10.2 lists what a
different core-to-bandwidth ratio or a different last-level cache size would move; the same caveats
apply here and are not repeated.

## 11. The register correction

`OPTIONS.md` Q32's opening sentence, "Every instrument this panel has built runs on one thread," is
false and was false when written; `26` and `27` predate it. Its exposures are real for writes and
unmeasured for reads (reads were measured and the read-side inequality survives contention per `27`,
unchanged in form). The correction below is written as an addendum in `OPTIONS.md` itself rather than
silently edited in place, per this panel's own discipline against rewriting a committed entry; see
that file for the addendum text and the section 12 pointer.

## 12. Alternatives I did not take, for whoever picks this up

**A windowed-and-guarded encoder**, section 7.1's second unattempted attack. The cheapest remaining
lever on the correctness-with-performance question, and the one a real design would actually ship.

**NEON pairwise accumulation on the encode side**, mirroring `26`/`27`'s `sum_padal_pipe2`/`pipe4`
on the decode side. Untested whether the same independent-chain trick applies to a store-terminated
loop.

**Measuring the write-then-read-back pattern directly**, rather than a pure encode, which is closer to
what a consumer's actual column update looks like and would price the round trip this file's numbers
do not include.

**Perf counters** (the harness supports `--perf-counters` under sudo, per `27` section 14) to settle
directly whether the `T=4` dense-write regression in section 6 is bandwidth saturation, P-core
oversubscription, or both, rather than inferring it from the two mechanisms named without separating
them.

**Cold mode across the whole matrix**, which `27` found moved its verdict; whether it moves this file's
verdict the same direction is a cheap rerun of the existing bench sections with `--mode cold`, not
attempted here for time.
