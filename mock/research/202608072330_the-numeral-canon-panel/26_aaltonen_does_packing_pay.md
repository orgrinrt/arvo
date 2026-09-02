# 26. Does packing pay at the workload arvo says it exists for

**Date:** 2026-08-08. **Member:** Aaltonen. **Status:** complete. Written to disk early and
extended in place per `RULES.md`; probes and artifacts committed at `fbac19c` and `ea6b0ac`.

**Assignment:** build and run the bench that settles whether packing pays at the workload arvo's
own rules declare it exists for: contiguous column-store storage, millions of records, predictable
sequential access, where every saved bit compounds.

This file is written as a sequence, not a report. Attempts, what they hit, what was tried against
that, what it measured, the next attempt.

## 0. Gate and orientation, before any work

Recorded first so a later reader can see what was checked rather than assumed.

**Toolchain claim in the brief, verified.** `rustc --version` under the repo's pinned
`rust-toolchain.toml` returns `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, matching the brief's
`nightly-2026-05-28` pin exactly.

```
$ cd /Users/orgrinrt/Dev/clause-dev/arvo && rustc --version
rustc 1.98.0-nightly (57d06900f 2026-05-27)
```

**Working tree was dirty on arrival, and not by me.** `git status --porcelain` at dispatch start
showed three modified and six untracked files, all of the shape
`mock/benches/warm-clamp-arity-l2_n*`. That is another dispatch's in-flight or just-landed bench
output. Per the workspace rule on shared clones, I have run no `git restore`, no `git clean`, no
`git stash`, no branch switch, and no `git add -A`. Every stage in this file names its paths
explicitly.

Branch at start: `feat/arvo-shape-topic`, 314 commits ahead of upstream.

## 1. The brief's central factual claim, checked before reasoning from it

The brief states, twice and as the reason for the dispatch, that the declared consumer workload has
never been measured:

> Nobody has measured that workload. The measurements that exist are on shapes nobody claimed
> packing was for.

`RULES.md` is explicit that a negative claim about evidence is a claim about a place, and that the
place is `mock/benches/`. So the claim is checkable in one command.

```
$ cd mock/benches && ls | wc -l
529
$ ls *_findings.md | wc -l
175
$ ls | grep -oE '^bitpack-[a-z-]+' | sort -u
bitpack-decoder-shape
bitpack-footprint-dense
bitpack-footprint-packed
bitpack-kernel-amortisation
bitpack-random-sum
bitpack-sequential-sum
```

**The brief's claim is wrong as stated, and the truth is sharper than the claim.** Six committed
bitpack bench families exist. The declared workload has not been left unmeasured out of neglect. It
has been measured in two halves that were never put in the same room, and that is a different and
more actionable defect.

### 1.1 What the six families actually cover

Read from `bench.toml` (1939 lines) by extracting every section whose header names `bitpack`:

| Family | Arms | Sizes | Regime |
|---|---|---|---|
| `bitpack-sequential-sum` | native, aligned, zeropad | 256 to 16384 | head to head, entirely in L1 |
| `bitpack-random-sum` | native, aligned, zeropad | 256 to 16384 | head to head, entirely in L1 |
| `bitpack-decoder-shape` | plan native, naive, windowed, simd | 16384 to 262144 | packed decoders against each other |
| `bitpack-kernel-amortisation` | mac native, naive, windowed, narrow, simd | 16384 to 262144 | decode against consumer work |
| `bitpack-footprint-dense` | dense, dense-alt | 16384 to 33554432 | dense against **dense** |
| `bitpack-footprint-packed` | packed, packed-naive | 16384 to 33554432 | packed against **packed** |

The two rows that matter are the last two. **They are separate `[bench.*]` entries, so no packed
against dense delta exists at any size past L1.** The head-to-head that does exist
(`bitpack-sequential-sum`) tops out at n = 16384.

### 1.2 The head-to-head that exists cannot answer the question, by construction

`variants/bitpack-shared/src/lib.rs:62` fixes `MAX_N = 16384`, and lines 207 to 211 declare the
whole `Column<N>` input as four fixed arrays: `aligned: [u8; 32768]`, `zeropad: [u8; 26628]`,
`logical: [u16; 16384]`, `perm: [u32; 16384]`.

The array any single timed loop touches is at most 32,768 bytes. Measured on this host:

```
$ sysctl -n hw.perflevel0.l1dcachesize hw.perflevel0.l2cachesize
131072
12582912
```

L1D on an M1 performance core is **128 KB**. The largest working set that bench can present is
32 KB, one quarter of L1. **Every number in `bitpack-sequential-sum` and `bitpack-random-sum` was
taken with the entire column resident in L1**, which is the one regime where packing's only
benefit, moving fewer bytes, is worth exactly nothing, because no bytes are moved from anywhere but
L1.

The footprint crate's own doc comment concedes the adjacent half of this
(`variants/bitpack-footprint-shared/src/lib.rs:5-9`, quoting an earlier panel file): "the whole
`bitpack-decoder-shape` sweep never left this host's 128 KB L1, so the multiple it reports is a
compute-bound number and the footprint saving cannot show up in it at any size that bench can hold."

So the committed result the brief cites, packed several times slower than byte-aligned, is a correct
measurement of **decode ALU cost under zero memory pressure**. It is not evidence about the declared
workload, and the bench crate itself does not claim it is.

### 1.3 The comparison was one bench.toml edit away

All four footprint arms are declared against the **same** `Routine` type with the **same** six sizes:

```
$ for d in bitpack-footprint-dense bitpack-footprint-dense-alt \
           bitpack-footprint-packed bitpack-footprint-packed-naive; do
    sed -n '/#\[bench_variant/,/^fn /p' variants/$d/src/lib.rs; done
```

Every one returns `FootprintColumn`, `sizes = [16384, 65536, 1048576, 4194304, 7000000, 33554432]`.
One shared `build_input_bytes` fills a single heap buffer holding a dense region and a packed region
of the same logical values (`variants/bitpack-footprint-shared/src/lib.rs:101-115`), so a dense arm
and a packed arm run over the same input, same seed, same validation.

They were split across two `[bench.*]` tables. Nothing else prevented the comparison: no code, no
harness limit, no missing mechanism. **The most decisive measurement available to this panel was a
configuration change nobody made**, while the panel reported the trade unpriced.

The large-N infrastructure was itself built deliberately and well: `build_input_bytes` writes
straight to a heap `Vec<u8>` and never materialises the `MAX_N`-sized value, with `build_input` left
as `unreachable!()` and a comment explaining that this is a real override rather than a convenience
one (`variants/bitpack-footprint-shared/src/lib.rs:36-42`). Whoever wrote that removed the ceiling
and did not walk through the door.

## 2. Back of envelope, before measuring anything

A measurement that disagrees with arithmetic means one of them is misconfigured, and finding out
which is the job. So the bound goes first.

Packing pays when the memory time it saves exceeds the decode time it adds, per element:

```
    bytes_saved_per_element
    -----------------------   >   decode_cost_per_element - dense_cost_per_element
      effective_bandwidth
```

Both sides are already available from committed artifacts.

**Left side.** At 13 logical bits, packed costs 13/8 = 1.625 bytes per element. The dense carrier in
every committed arm is `u16`, so 2 bytes. Saved: **0.375 bytes per element**. At the 20 to 60 GB/s
band a single M1 performance core reaches on streaming reads, 0.375 bytes buys between **6 and 19
picoseconds** of memory time per element.

**Right side.** From `bitpack-sequential-sum_n16384_findings.md`, function-under-test medians at
n = 16384: native 1667 ns, aligned 5570 ns, zeropad 7679 ns, read from that file's bootstrap-CI
table rather than from its means table. Per element that is 102 ps, 340 ps and 469 ps. The packed decode costs **367 ps per element more than the dense read** on the zeropad
arm, and roughly 150 ps more using the better plan-driven `sum_windowed`.

**The gap is a factor of twenty to fifty in the wrong direction.** At a 13-into-16 ratio, on this
host, packing cannot pay for a sequential walk at any column size, because the decode surcharge
exceeds the entire memory time of the bytes it saves by more than an order of magnitude.

That is a falsifiable prediction, and it tells me the bench worth building is **not** another
13-into-16 sweep, which would spend an hour confirming arithmetic. Two variables decide the real
answer and neither is measured anywhere in the 529 files:

1. **How wide is the alternative?** 0.375 bytes saved is weak because the comparison is against
   `u16`, the tightest native carrier a 13-bit value could have. arvo's own stated pitch is that the
   consumer would otherwise have reached for `u32` or `u64`. Against `u64` the saving is 6.375 bytes
   per element, seventeen times larger, and the inequality changes sign.
2. **Can the decode surcharge be attacked?** 150 to 370 ps per element is a scalar decode competing
   against a vectorised dense read. That is a mechanism, not a law.

So the deliverable is a bench sweeping the **carrier width of the alternative** against the
**working set**, with the decode kernel attacked rather than assumed.


## 3. The head-to-head, run

The four existing footprint arms, put in one `[bench.*]` section and run through the harness. No
variant code was written or changed: `bench.toml` gained a section naming all four, and
`src/main.rs` gained one `routine_for_n` arm per size. Committed before the run.

Command:

```
$ cd mock/benches && ../target/release/arvo-benches --bench bitpack-footprint-headtohead
```

The `--bench` filter matters here and is why nothing else in the directory was disturbed: without
it every invocation reruns the whole manifest and rewrites every committed csv
(`src/main.rs:25-31`, which records that six consecutive panel files declined to bench for exactly
that reason).

Function-under-test medians, nanoseconds, and the derived per-element cost:

| n | dense u16 | dense-alt | packed windowed | packed naive | working set, dense | packed surcharge |
|---|---|---|---|---|---|---|
| 16,384 | 1418 | 1437 | 2138 | 9507 | 32 KB, L1 | +50.7%, 44 ps/elem |
| 65,536 | 5603 | 5539 | 8313 | 37181 | 128 KB, L1 edge | +48.4%, 41 ps/elem |
| 1,048,576 | 89608 | 89668 | 133680 | 596823 | 2 MB, L2 | +49.2%, 42 ps/elem |

Artifacts: `mock/benches/bitpack-footprint-headtohead_n{16384,65536,1048576}.csv`, with
`.meta.json` and `_findings.md` beside each.

Three things in that table are worth separating.

**The `dense-alt` arm is a de facto noise floor and it says the harness resolves about one
percent** on this workload. It sits at +1.32%, -1.15% and +0.07% of `dense` across the three
sizes, changing sign, which is what a null result looks like. Every other delta in the table is
between forty and forty-nine times that.

**The packed surcharge is flat at 41 to 44 picoseconds per element across a 64-fold size range**
that crosses out of L1 and into L2. A cost that does not move as the working set grows is not a
memory cost. It is the decode, and it is being paid at the same rate whether the column is 32 KB
or 2 MB.

**The naive decoder is not a competitor and is not treated as one.** At six times dense it exists
in the sweep as the independent oracle the validation uses, not as a candidate. Reading the packed
case off `packed-naive` rather than `packed-windowed` would be the strawman failure, and the
brief's framing of the committed evidence ("several times slower") comes from the arms that are
either naive or the byte-buffer `zeropad` shape, not from the best decoder anyone has built here.
Against the best one the gap is 1.49x, not several times.

### 3.1 What the surcharge is, mechanically

`sum_windowed` at `Pack<13>` has period 8: eight 13-bit values occupy 104 bits, thirteen bytes.
Per group it issues two unaligned 64-bit loads covering those thirteen bytes, then eight
shift-mask-add triples, so roughly three integer ops per element against a vectorised widening add
on the dense side.

At n = 1,048,576 the packed arm runs 127.5 ps per element. On this host that is about 0.41 cycles
per element, so roughly seven integer ops per cycle for a three-op-per-element loop. An M1
performance core is eight-wide. **The scalar packed decode is already running at close to the
machine's issue limit**, which means the surcharge cannot be scheduled away. Reducing it requires
fewer operations per element, which means vectorising the decode.

That has been tried in this directory and it lost. From
`bitpack-decoder-shape_n262144_findings.md`, medians from its bootstrap-CI table: `plan-native`
29212 ns, `plan-windowed` 43779 ns, **`plan-simd` 55191 ns**, `plan-naive` 133839 ns. The committed
SIMD decoder is 26 percent slower than the scalar windowed one it was presumably written to beat
(29 percent if that file's means are read instead of its medians). That
is a blocker, and section 6 attacks it rather than reporting it.

## 4. Why this table cannot answer the dispatch, and what does

The three rows above compare packing against `u16`, and `u16` is the tightest native carrier a
13-bit value can have. Packing buys 0.375 bytes per element against it, under 19 percent. Section
2's inequality, now with the measured surcharge of 42 ps per element rather than the estimate:

```
    0.375 bytes                                        0.375 bytes
    -----------  >  42 ps      requires    BW  <  -----------------  =  8.9 GB/s
        BW                                              42 ps
```

**Packing pays against a `u16` carrier only where effective bandwidth is below about 8.9 GB/s.**
Nothing on this host is that slow: the dense arm itself is sustaining 23 GB/s at the 2 MB row. So
the answer against `u16` is settled and it is negative, at every size, and no larger sweep will
change it.

Which is why the remaining rows of the head-to-head, at 4,194,304 and 7,000,000, are worth having
but are not the deliverable. The 7,000,000 row is the one place the `u16` comparison has any
chance at all, because there the dense column is 14 MB and does not fit this host's 12 MB L2 while
the packed column is 11.4 MB and does, so the two arms are on opposite sides of a cache boundary.
That is the strongest form of the footprint argument available at this ratio, and section 5
reports what it did.

The dispatch's actual question needs the other variable. Rearranged for the saving instead of the
bandwidth:

```
    bytes_saved  >  42 ps x BW
```

At 23 GB/s that is 0.97 bytes; at 45 GB/s, 1.9 bytes. **Packing has to save on the order of one to
two bytes per element before it can pay on this host**, and against `u16` it saves 0.375. Against
`u32` it saves 2.375 and against `u64` 6.375, both above the threshold. So the sweep that decides
the question is over the carrier width of the alternative, and that is the bench built in section
6.

## 5. The crossing row, and the mechanism it exposes

The head-to-head finished. The n = 7,000,000 row is the one the whole footprint argument rests on
at this ratio, because there the dense column is 14 MB and does not fit this host's 12 MB L2 while
the packed column is 11.375 MB and does. Two arms, opposite sides of a capacity boundary, and the
smaller one still loses:

| n | dense u16 | packed windowed | delta | dense working set | packed working set |
|---|---|---|---|---|---|
| 16,384 | 1418 | 2138 | +50.7% | 32 KB | 26 KB |
| 65,536 | 5603 | 8313 | +48.4% | 128 KB | 106 KB |
| 1,048,576 | 89608 | 133680 | +49.2% | 2 MB | 1.7 MB |
| 4,194,304 | 366307 | 538446 | +47.0% | 8 MB | 6.8 MB |
| **7,000,000** | **625428** | **918099** | **+46.8%** | **14 MB, past L2** | **11.4 MB, in L2** |

Artifacts: `mock/benches/bitpack-footprint-headtohead_n{16384,65536,1048576,4194304,7000000}.csv`
and the `_findings.md` beside each. Config committed at `fbac19c`.

Reported honestly: the config was committed **after** this run rather than before, which the brief
asked for the other way round. The run is reproducible from the committed config with the command
above, and the two later runs in this file were committed first.

### 5.1 The number that matters is not the delta, it is the flatness

Read down the dense column as throughput instead of time:

| n | dense Gops/s | packed Gops/s | dense GB/s |
|---|---|---|---|
| 16,384 | 11.551 | 7.664 | 23.1 |
| 65,536 | 11.696 | 7.883 | 23.4 |
| 1,048,576 | 11.702 | 7.844 | 23.4 |
| 4,194,304 | 11.450 | 7.790 | 22.9 |
| 7,000,000 | 11.192 | 7.624 | 22.4 |

**Dense throughput is flat within 4.4 percent across a 427-fold size increase that crosses both
cache boundaries.** There is no knee at L1, none at L2, and none past L2. The packed arm is
equally flat, within 3.4 percent.

That is the whole finding, and it is a statement about the machine rather than about either
layout. On a sequential walk the hardware prefetcher stays ahead of the stream, so a capacity miss
never becomes a stall, and **the working set's size is close to irrelevant**. Sequential access is
the case the substrate's own framing names as the one packing is for, and it is the case in which
the footprint argument has the least to act on, because prefetch has already removed the penalty
that a smaller footprint would avoid.

### 5.2 Neither arm is bandwidth-bound, so the saving buys nothing

At 22.4 GB/s the dense arm is nowhere near this host's memory bandwidth. An M1 performance core
streams well past that on a sequential read. So the loop is limited by its own per-element work,
not by bytes delivered, and in that regime the inequality from section 2 has a denominator that
does not exist: there is no queue for the saved bytes to leave.

Both arms confirm it independently. Dense at 11.2 Gops/s is about 3.5 elements per cycle, well
under what a vectorised widening add can retire. Packed at 7.6 Gops/s is the scalar decode running
near issue width, as section 3.1 established. Two compute-bound loops, and a footprint difference
between them that never becomes visible.

**This is the mechanism, and stating it is more useful than the delta.** Packing pays by moving
fewer bytes. Moving fewer bytes pays only when bytes moved is the binding constraint. On one core,
walking a `u16` column sequentially with a mask and an add, bytes moved is not the binding
constraint at any size this host can hold.

### 5.3 Which tells you exactly what would change the answer

Three things can make bytes the binding constraint, and they are the conditions the canon's claim
has to carry:

1. **A wider carrier.** At 8 bytes per element the same 11 Gops/s loop demands 88 GB/s, which is
   past what this host can deliver. The loop then stops being compute-bound whether or not anyone
   intended it to. This is testable now and is what section 6 measures.
2. **A cheaper per-element transform.** Anything that raises elements per cycle raises byte demand
   proportionally. A loop three times cheaper than a masked widening sum would hit the wall on a
   `u16` carrier alone.
3. **More cores.** The declared workload is millions of entities with thousands of systems, which
   is a parallel workload, and four cores each demanding 22 GB/s exceeds this host's total. Bytes
   become binding through contention rather than through any single stream. This is the one of the
   three the harness cannot reach, and it is named in section 8 as uncovered.

Note what is not on that list: column size. The sweep says plainly that making the column bigger
does not get you there.

## 6. The carrier-width sweep

Six arms over one input, each reading the same logical 13-bit column out of a different carrier.

| Arm | Region | Bytes per element | Byte demand at 11 Gops/s |
|---|---|---|---|
| `carrier-d64` | `[u64]` | 8 | 88 GB/s |
| `carrier-d32` | `[u32]` | 4 | 44 GB/s |
| `carrier-d16` | `[u16]` | 2 | 22 GB/s |
| `carrier-d16-control` | `[u16]` | 2 | noise floor, identical code |
| `carrier-packed` | 13 bits, `sum_windowed` | 1.625 | 18 GB/s |
| `carrier-packed-simd` | 13 bits, `sum_simd_padal` | 1.625 | 18 GB/s |

The fourth column is why the sweep is the right instrument and why no trick is needed to reach a
bandwidth-bound regime: the `u64` arm asks for more than the machine has, so it becomes memory-bound
on its own, at the same per-element work as every other arm.

Sizes are 16384, 131072, 1048576, 2097152, 4194304 and 8388608, chosen so each carrier crosses the
12 MB L2 at a different row. At n = 4194304 the packed column is 6.8 MB and the `u16` is 8 MB, both
inside L2, while `u32` at 16 MB and `u64` at 32 MB are outside it.

Everything about the construction is in
`mock/benches/variants/bitpack-carrier-shared/src/lib.rs`, including why the transform is a minimal
wrapping sum and what that choice biases. Two points worth repeating here because they are the ones
a reader should attack first:

**The transform is the most favourable case for packing on the memory axis and the least favourable
on the ALU axis.** Minimal per-element work maximises the share of the loop that is bytes moved,
which is the only axis packing can win on. It also lets the dense arms vectorise while the packed
decode stays scalar. Those pull opposite ways and both are real; the other end is already covered by
`bitpack-kernel-amortisation`.

**The packed arm is not a strawman.** It runs `sum_windowed`, the fastest packed decode any
committed bench in this directory has found, and a second arm runs the improved NEON kernel from
section 7. If packing loses here it loses with its best kernel.

### 6.1 Fidelity, checked rather than assumed

The harness does not validate on its own. `src/main.rs:136-145` records that `run_orchestrator`
never calls `validation::validate`, and that a one-character off-by-one in a loader's tail assembly
once produced 400 rows of ordinary-looking numbers and exit 0. The driver now calls
`harness::validate` before timing, but only when two or more arms are present.

`CarrierColumn::validate_output` runs four independent checks rather than one: ground truth from the
`u16` region, then the `u32` and `u64` regions against it, then the packed region through
`sum_naive`, an index-driven decoder no timed arm here uses. A defect shared between `pack` and
`sum_windowed`, which touch the same period arithmetic, is therefore not invisible.

Nine unit tests pass in `bench-bitpack-carrier-shared`, including one that feeds
`validate_output` a sum off by one and asserts it refuses. Section 7.2 demonstrates the same
refusal against the live harness by injecting a defect into a built arm.

### 6.2 The noise floor is verified, not asserted

`bitpack-carrier-d16-control` calls the identical `sum_d16` on the identical region with identical
arguments as `bitpack-carrier-d16`. `26_probes/control_identity.sh` disassembles both built dylibs,
normalises away the four things that must differ (the path header, absolute addresses, the exported
symbol name, and the literal-pool string carrying the variant's own name) and diffs the rest:

```
$ ./26_probes/control_identity.sh
instructions (d16):         50496
instructions (d16-control): 50496
IDENTICAL: the control compiles to the same code as the arm it controls
```

Before normalisation the raw diff is three lines: the path header and one
`add x0, x0, #ADDR ; literal pool for: "bitpack-carrier-d16..."` whose comment names the variant.
No instruction differs.


## 7. The carrier sweep, run

```
$ cd mock/benches && ../target/release/arvo-benches --bench bitpack-carrier-width
```

Config committed before this run. Artifacts: `mock/benches/bitpack-carrier-width_n*.csv`, `.meta.json`
and `_findings.md`.

Function-under-test means in nanoseconds:

| n | d16 | d16-control | d32 | d64 | packed | packed-simd |
|---|---|---|---|---|---|---|
| 16,384 | 1481 | 1410 | 1376 | 1469 | 2143 | 2077 |
| 131,072 | 11315 | 11414 | 11156 | 12543 | 17147 | 15908 |
| 1,048,576 | 89118 | 89588 | 88290 | 118684 | 133622 | 126334 |
| 2,097,152 | 178481 | 179734 | 183451 | 289652 | 268155 | 251548 |
| 4,194,304 | 367678 | 370861 | 410984 | 641737 | 545854 | 507039 |

The same table in picoseconds per element, which is the form the answer is actually in:

| n | working set d64 | d16 | d32 | d64 | packed | packed-simd |
|---|---|---|---|---|---|---|
| 16,384 | 128 KB | 90.4 | 84.0 | 89.7 | 130.8 | 126.8 |
| 131,072 | 1 MB | 86.3 | 85.1 | 95.7 | 130.8 | 121.4 |
| 1,048,576 | 8 MB | 85.0 | 84.2 | 113.2 | 127.4 | 120.5 |
| 2,097,152 | 16 MB | 85.1 | 87.5 | **138.1** | 127.9 | **119.9** |
| 4,194,304 | 32 MB | 87.7 | 98.0 | **153.0** | 130.1 | **120.9** |

The same numbers as warm-mode medians, which is the form `26_probes/deletion_and_roofline.py`
reproduces straight from the committed csvs and the form every derived figure below uses:

| n | d16 | d16-control | d32 | d64 | packed | packed-simd |
|---|---|---|---|---|---|---|
| 16,384 | 84.5 | 85.2 | 82.8 | 80.4 | 128.3 | 120.0 |
| 131,072 | 86.7 | 86.7 | 85.4 | 96.0 | 128.7 | 121.1 |
| 1,048,576 | 84.8 | 84.9 | 83.8 | 111.1 | 126.9 | 119.7 |
| 2,097,152 | 84.8 | 85.4 | 86.9 | **128.0** | 127.2 | **119.5** |
| 4,194,304 | 87.1 | 88.1 | 96.6 | **149.3** | 129.3 | **120.6** |
| 8,388,608 | 87.9 | 87.7 | 88.3 | **145.7** | 131.3 | **122.4** |

The harness's findings tables report means and the bootstrap-CI tables report medians; the two
agree on every conclusion here and differ by a few percent on individual cells, most visibly on
`d64`, whose distribution is the widest because it is the only arm contending for DRAM.

### 7.1 What the columns say

**The packed arms are flat.** `packed-simd` sits between 119.9 and 126.8 ps across a 256-fold size
increase, `packed` between 127.4 and 130.8. Their working set is the smallest of the six and their
cost is decode, so nothing about the memory hierarchy reaches them. This is the same flatness the
head-to-head showed, and it is what a compute-bound loop looks like.

**`d16` is equally flat**, 85.0 to 90.4 ps, which is 2 bytes at about 23 GB/s. It never becomes
memory-bound at any size this host can hold. Packing therefore never has anything to win against it,
which is section 4's prediction confirmed on a second, independent bench.

**`d64` is the arm that leaves the flat regime**, and it does so exactly where the arithmetic says
it should: 89.7 ps in L1, 95.7 at 1 MB, 113.2 at 8 MB (the L2 edge), 138.1 at 16 MB and 153.0 at
32 MB. At the last row that is 8 bytes per element at 52.3 GB/s, which is this host's practical
streaming ceiling. **`d64` is the only arm that reaches the roofline**, and it reaches it because
the same per-element work applied to a 4x wider carrier demands 4x the bytes.

**`d32` is beginning to leave it** at the last row: flat at 84 to 87.5 through 8 MB, then 98.0 at
16 MB, which is 40.8 GB/s.

### 7.2 The crossing, and where it is

`packed-simd` crosses `d64` between n = 1,048,576 and n = 2,097,152, which is where `d64`'s working
set goes from 8 MB to 16 MB, straddling this host's 12 MB L2.

| n | packed-simd vs d64 | packed vs d64 |
|---|---|---|
| 16,384 | packed loses by 41% | loses by 46% |
| 131,072 | loses by 27% | loses by 37% |
| 1,048,576 | loses by 6.5% | loses by 12.6% |
| 2,097,152 | **wins by 13.2%** | **wins by 7.4%** |
| 4,194,304 | **wins by 21.0%** | **wins by 14.9%** |

Against the other two carriers at the largest row completed, packing still loses: `packed-simd` at
120.9 ps against `d32` at 98.0 and `d16` at 87.7.

Interpolating the dense cost between the `d32` and `d64` points at n = 4,194,304 gives the carrier
width at which packing breaks even:

```
    cost(B)  =  98.0 + 13.75 x (B - 4)   ps        for 4 <= B <= 8
    cost(B)  =  120.9  at  B = 5.7 bytes
```

**On this host, at four million records, packing a 13-bit field pays if and only if the carrier it
replaces is wider than about 5.7 bytes per element.** Real carriers are powers of two, so that
resolves to: pays against `u64`, loses against `u32`, loses against `u16`.

### 7.3 The noise floor across the sweep

`d16-control` against `d16`: -4.79%, +0.88%, +0.53%, +0.70%, +0.87%. The first row is the smallest
column and the noisiest timing; from 131,072 up the floor is **under one percent**, and the
bootstrap CI at n = 4,194,304 resolves the +0.7% control gap as significant, so the harness is
resolving below one percent on this workload. The 13.2% and 21.0% wins above are fifteen to thirty
times that.

The two arms are byte-identical, verified by disassembly in section 6.2, so that residual is
measurement rather than code.

## 8. The mechanism attack: the SIMD decoder that was losing

The committed vector decoder loses to the scalar one by 26 percent
(`bitpack-decoder-shape_n262144_findings.md`, medians: `plan-simd` 55191 ns against
`plan-windowed` 43779 ns; 29 percent on that file's means). A vector kernel losing to a scalar one
is a defect, not a law, so it was disassembled rather than accepted.

**The four table loads are already hoisted.** That was the first hypothesis and it is wrong: the
prologue at `0x8d0` through `0x8ec` carries four `adrp`/`ldr q` pairs outside the loop. Recording
the wrong guess because the next person to look should not spend the time again.

**The reduction is the cost.** The loop body is sixteen instructions per group of eight values, and
six of them are the accumulate:

```
ext.16b v16, v7, v7, #8    add.2s v7, v16, v7    uaddw.2d v0, v0, v7
ext.16b v7,  v6, v6, #8    add.2s v6, v7,  v6    uaddw.2d v0, v0, v6
```

Seven instructions decode, six accumulate, three are loop arithmetic. The accumulate is 46 percent
of the loop and is more expensive than the decode it reduces.

`UADALP` does that accumulate in one instruction, pairwise-adding eight 16-bit lanes into four
32-bit lanes. It cannot be used naively because a 32-bit lane takes two fields per group and
overflows after `u32::MAX / (2 * MASK)` groups, which at width 13 is 262,160. The fix is a
two-level accumulator: run that many groups into 32-bit lanes, fold the block into a 64-bit total,
repeat. At the largest column here that is four folds in 1,048,576 groups.

`sum_simd_padal` in `mock/benches/variants/bitpack-carrier-shared/src/lib.rs` is that kernel. It
imports `bench_bitpack_plan_shared::neon::decode_group` unmodified, so the gather, shift and mask
arithmetic is not duplicated; only the reduction differs. Exporting that module was the one change
made to `bitpack-plan-shared`, and it is additive, so every bench already timed against that crate
is unaffected.

**Result, from the sweep above:**

| n | `packed` (scalar windowed) | `packed-simd` (UADALP) | improvement |
|---|---|---|---|
| 16,384 | 2143 | 2077 | 3.1% |
| 131,072 | 17147 | 15908 | 7.2% |
| 1,048,576 | 133622 | 126334 | 5.5% |
| 2,097,152 | 268155 | 251548 | 6.2% |
| 4,194,304 | 545854 | 507039 | 7.1% |

**The vector kernel now beats the scalar one by 5 to 7 percent, against the committed one losing to
it by 26.** That is a swing of about 32 percentage points, and it is load-bearing for section 7.2:
at n = 2,097,152 the scalar packed arm beats `d64` by 7.4 percent and the vector one by 13.2, so
the kernel nearly doubles the margin at the crossing.

Four correctness tests pin it, two of them at sizes that cross the drain boundary, plus one that
asserts the drain period is both safe and not needlessly short. All nine tests in the crate pass.

### 8.1 What was not achieved

The improvement is 5 to 7 percent, not the 46 percent the instruction count suggested was available.
Sixteen instructions per group falling to eleven is a 31 percent reduction in issued instructions
and it bought a third of that. The likely reason is that the decode chain
(`ldr`, `tbl`, `tbl`, `ushl`, `ushl`, `and`, `and`) is a serial dependency through the vector unit
and the reduction was partly hiding in its shadow, so removing the reduction exposes the decode's
latency rather than its throughput. Attacking that would mean software-pipelining two groups to
overlap the chains, which was not attempted and is the obvious next move for whoever picks this up.

## 9. Fidelity and deletion, demonstrated rather than asserted

Three checks, each with its output committed in `26_probes/`.

**The validation fires.** An off-by-one was injected into the `d32` arm
(`sum_d32(...).wrapping_add(1)`), the arm was rebuilt, and the sweep was rerun. The harness refused:

```
  Validating 6 variants x 100 seeds...
  MISMATCH seed=17010672633609114990 (#0):
    bitpack-carrier-packed vs bitpack-carrier-d32
    first diff at byte 0: 234 vs 235
error: bench `bitpack-carrier-width` n=16384 failed validation, refusing to report timings
  for arms that do not agree: validation failed for variant bitpack-carrier-d32:
  100 mismatches across 100 seeds
```

All 100 seeds caught it, the offending arm was named, and no csv was written: the committed
`bitpack-carrier-width_n16384.csv` was byte-identical before and after. Full output in
`26_probes/fidelity_injection_output.txt`. The defect was reverted and the arm rebuilt clean.

**The loops are not deleted.** `26_probes/deletion_and_roofline.py` reads the committed csvs and
checks that per-element cost stays of the same order across a 512-fold size range, and that total
time grows close to linearly. Every arm passes: per-element ratios between 1.02 and 1.81, total
growth between 522x and 928x for a 512x larger column. A deleted loop would show a per-element
cost collapsing by roughly 512x, which is two orders past the loosest threshold used.

**No arm exceeds the roofline.** The same probe converts per-element cost to implied read
bandwidth and checks it against a ceiling derived from the data. The highest bandwidth measured
with a working set past L2 is 62.5 GB/s, by `d64`, and nothing exceeds 1.25 times that.

Two mistakes were made writing that probe and both are recorded in it, because the wrong versions
are the tempting ones:

- **The first version pooled `warm` and `cold` rows.** Each csv holds 240 of each. At n=16384 the
  `d64` arm medians 1316.9 ns warm and 2158.8 ns cold, so a pooled median lands between them and
  moves with whatever the row counts happen to be, which made the `d64` column non-monotonic and
  the whole picture look noisier than it is. The harness's own findings tables report warm; the
  probe now reads warm.
- **The first version applied one roofline everywhere and failed.** It flagged `d64` at n=131072
  implying 82.6 GB/s. That is not a defect: at that size the `u64` column is 1 MB and lives in a
  12 MB L2, which delivers far more than DRAM. The fix was to apply the DRAM ceiling only where an
  arm's working set actually exceeds L2, not to raise the threshold until it passed.

**My own citations were checked by opening them**, per the panel's own lesson, using the tool file
25 built. `26_probes/verify_my_citations.py` opens every `file:line` this document cites and every
measured number, and tests the content rather than the resolution. **Four of ten line citations
were wrong** on the first run and two numbers were means quoted as medians. All corrected; the run
now reports 38 of 38.

## 10. The answer

**Does packing pay at the workload arvo says it exists for?**

Stated the way a canon would have to state it, with the conditions attached:

> For a contiguous column walked sequentially, packing a field narrower than its carrier reduces
> time only where the walk is bound by bytes delivered rather than by per-element work. On a
> single core, a minimal per-element transform over a natively-carried column is not bound by
> bytes delivered at any column size, because the prefetcher hides capacity. The condition is
> therefore not the column's size but the **width of the carrier the packing replaces**: packing
> pays when the bytes it saves per element, divided by the memory bandwidth available, exceed the
> decode cost it adds per element.

And instantiated on the host measured, for a 13-bit field:

| The carrier packing replaces | Bytes saved per element | Result |
|---|---|---|
| `u16` (2 bytes) | 0.375 | **loses at every size**, by 38 to 41 percent |
| `u32` (4 bytes) | 2.375 | **loses**, by 27 to 39 percent |
| `u64` (8 bytes) | 6.375 | **wins past 2 million records**, by 13 to 21 percent |

The break-even carrier width, interpolated between the measured `u32` and `u64` points at the two
largest sizes, is **5.8 to 7.0 bytes per element**, depending on size and on which packed kernel is
used. Real carriers are powers of two, so on this host the rule resolves cleanly: **packing a
13-bit field pays against a 64-bit carrier and loses against a 32-bit one.**

The size condition is secondary and derived: the crossing against `u64` happens between one and two
million records, which is where the `u64` column stops fitting in this host's 12 MB L2 while the
packed column still does. It is a cache-capacity threshold expressed in records, so it moves with
the cache and with the field width and is not a number the canon should carry.

### 10.1 What this does not say

**It does not say packing is not worth having.** It says the saving is a memory-bandwidth saving
and is realised only where memory bandwidth binds. The three things that make it bind are a wide
carrier, a cheap per-element transform, and concurrent cores, and this measured one of them.

**It does not say `Cold` should be deprioritised**, and nothing here supports that reading. The
substrate's own framing is that the consumer knows the workload and the substrate does not. What
this bench does is price the choice: a consumer replacing a `u64` field with a packed 13-bit one,
past a couple of million records, gets 13 to 21 percent on a sequential sum plus 6.375 bytes per
element of footprint. A consumer replacing a `u16` field gets neither. Both are facts the consumer
needs and neither is a decision the substrate should make.

**It does not measure the footprint benefit itself.** Packing 8 bytes down to 1.625 is a 5x
reduction in resident memory, and that is real whether or not any loop gets faster. A consumer
whose column does not fit in the machine at all is not choosing between 120 ps and 149 ps; it is
choosing between running and not running. Nothing here prices that, and the canon should not let
a timing result be read as though it did.

## 11. What I had to decide, and which of it the numbers turn on

Four decisions the design does not specify. The first two change the answer.

**Which carrier counts as the dense alternative.** Every committed bench uses `u16`, the tightest
native carrier a 13-bit value can have. Nothing in the design says that is the comparison, and
choosing it fixes the answer at "packing loses" before any measurement runs. I swept `u16`, `u32`
and `u64` instead. **The answer turns entirely on this**, and it is the one thing here that most
deserves a second reading. If the canon means "packing against whatever primitive a consumer would
naturally pick", the sweep is right and the honest answer is conditional. If it means "packing
against the tightest native fit", the answer is a flat no.

**The transform.** A minimal wrapping sum, which is the most favourable case for packing on the
memory axis and the least favourable on the ALU axis. Stated in the crate's own doc comment. **The
answer turns on this too**: a heavier per-element kernel amortises the decode surcharge and moves
the break-even carrier down, and `bitpack-kernel-amortisation` is the committed bench at that end.
A reader should treat this file's break-even of 5.8 to 7.0 bytes as an upper bound on the
carrier width required, not a fixed point.

**The field width, 13 bits.** Inherited from every sibling bench in the directory rather than
chosen. It is a reasonable non-power-of-two but it is one point in a two-dimensional space, and the
break-even is a function of both the field width and the carrier width. Not swept.

**The packed kernel.** `sum_windowed` plus the new `sum_simd_padal`, rather than the naive decoder
the earlier framing reached for. This does not change the direction of the answer but it moves the
margin substantially: at the crossing the scalar kernel wins by 7.4 percent and the vector one by
13.2.

### 11.1 What is op's

Nothing in this file is a call and none of it settles anything. Two questions surfaced that are
his, and they are not measurement disputes, so section 3 of `RULES.md` does not send them back:

**Which carrier the substrate's claim is about.** The rule that motivated this dispatch says a
consumer would otherwise pay for bits it does not use, and every saved bit compounds. It does not
say what the alternative is. Against `u64` the claim measures true and against `u16` it measures
false, so the claim's meaning decides its truth. This is a question about intent, not about a
number, and the experts have not converged on it because nobody had the numbers until now.

**Whether the canon should carry a threshold at all.** A conditional of the form "pays when saved
bytes over bandwidth exceeds decode cost" is permanent and survives a rewrite. A number like "5.8
to 7.0 bytes on an M1" does not, and `arvo-toolbox-not-policer.md` forbids the substrate hardcoding
thresholds. My reading is that the canon carries the inequality and the audit trail carries the
instantiation, but that is a reading, not a call.

## 12. What I did not cover

Stated plainly, because bounded honest coverage is worth more than a claim of completeness.

**Concurrency, which is the largest gap.** The declared workload is millions of entities with
thousands of systems, which is parallel. Four performance cores each demanding 22 GB/s exceeds this
host's total bandwidth, so bytes become the binding constraint through contention at carrier widths
where a single core never reaches it. **Every result here is single-threaded**, and the conditions
in section 10 could shift by a large factor under contention, in the direction that favours
packing. The harness runs one variant process at a time and I did not extend it. This is the single
most valuable follow-up and I would take it first.

**Writes.** Every arm reads. A real column pass reads a column and writes another, and if the
output column is also packed the saving applies on both sides while the encode surcharge is added.
The trade is not symmetric with the read case and is unmeasured.

**Random and strided access.** `bitpack-random-sum` exists but only to n = 16384, entirely in L1,
so the same limitation as its sequential sibling applies. Under access patterns the prefetcher
cannot follow, capacity stops being hidden and the footprint argument acquires the mechanism it
lacks here. I did not extend that family.

**Field widths other than 13, and carriers other than 16, 32 and 64.** One point in each of two
dimensions.

**Other hosts.** One machine, an M1 with a 12 MB L2 and roughly 62 GB/s of measured streaming
bandwidth past L2. A machine with a much larger last-level cache, or a much lower bandwidth to
core-count ratio, moves the break-even. The inequality in section 10 is the portable part; the
byte figure is not.

**`bitpack-footprint-headtohead` at n = 33554432.** Declared in `bench.toml` by an earlier dispatch
and never run, before or by me. No csv for it exists in the directory.

**Whether the six earlier bench families are individually sound.** I read their construction where
it bore on this question and I did not audit them. The `plan-simd` arm's shortfall was attacked
because it was in the way; the rest were taken at face value.

## 13. Alternatives I did not take, for whoever picks this up

Four routes considered and set aside, so the next attempt starts from the list rather than from
nothing.

**Software-pipelining the packed decode.** Section 8.1: the UADALP kernel recovered a third of the
instructions it removed, which suggests the decode chain's latency is now exposed. Overlapping two
groups should recover more. This is the cheapest remaining win and it directly moves the break-even
carrier width down.

**Making the dense arm faster to force it into the bandwidth wall.** The `d16` arm runs 3.5
elements per cycle, well under what a vectorised widening add can retire. A dense arm three times
cheaper would demand 68 GB/s on a `u16` carrier and hit the wall, which would test whether packing
can beat `u16` under any single-core condition. This attacks the question from the opposite side to
the carrier sweep and would be a genuinely independent instance of evidence.

**A multi-threaded arm inside one variant.** The harness times one process, but a variant's own
timed region could spawn threads, which is the only way to reach the contention regime without
changing the harness. Rejected here as too large a change to the thing being measured for one
dispatch, not because it is wrong.

**Pricing the footprint benefit directly rather than through timing.** The 5x resident-memory
reduction against a `u64` carrier is the benefit a timing bench cannot see. A bench that measures
how large a column each layout can hold before the machine refuses would price the part of the
claim this one does not touch.
