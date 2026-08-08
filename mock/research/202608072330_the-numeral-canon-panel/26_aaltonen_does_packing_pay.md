# 26. Does packing pay at the workload arvo says it exists for

**Date:** 2026-08-08. **Member:** Aaltonen. **Status:** in progress, written to disk early and
extended in place per `RULES.md`.

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
(`variants/bitpack-footprint-shared/src/lib.rs:5-8`, quoting an earlier panel file): "the whole
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
one (`variants/bitpack-footprint-shared/src/lib.rs:20-56`). Whoever wrote that removed the ceiling
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
n = 16384: native 1667.3 ns, aligned 5574 ns, zeropad 7678.9 ns. Per element that is 102 ps, 340 ps
and 469 ps. The packed decode costs **367 ps per element more than the dense read** on the zeropad
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
`bitpack-decoder-shape_n262144_findings.md`, function-under-test means: `plan-native` 29549 ns,
`plan-windowed` 43388 ns, **`plan-simd` 55824 ns**, `plan-naive` 133317 ns. The committed SIMD
decoder is 29 percent slower than the scalar windowed one it was presumably written to beat. That
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
