# 92. Pricing the reassociation: what `80`'s instruction counts are worth as time

The unit's magnitudes were unpriced. `80` section 5.3 and `82` section 9 report inner-loop
instructions per element for a reassociated saturating reduction, read by hand off emitted assembly,
and both say plainly that instructions per element is not time and that nothing was timed (`80:495`,
`82:514`). `90` R10 carries the numbers forward with the same disclaimer (`90:426-437`). `87` section
3 records that this unit has run no bench at all, that two files claim vectorisation wins measured
only that way, and that under the workspace rules a bench fork is the coordinator's call rather than
op's.

This file closes that gap on the harness. Eleven arms, twelve reduction lengths, both operators, two
column sizes, aligned and offset, plus a const-gate section with a size-matched control. Sources under
`mock/benches/variants/satfold-*`, registered in `mock/benches/bench.toml`, artifacts committed as
`mock/benches/satfold-*_n*.{csv,meta.json,_findings.md}`, derived tables and disassembly at
`92_probes/`.

**The short version.** The instruction counts survive as an ordering at the one shape `80` measured
and nowhere else. Every arm's advantage has a threshold in the reduction length, below which it is
worth nothing and at one length is worth less than nothing. The arm `80` singled out as the lesson of
its section, the one it called worse than doing nothing, is **faster than the arm it was measured
against at every one of the twelve lengths here**, by up to 14.5x.

The mechanism is not instruction count and never was. It is the length of the loop-carried dependency
chain, which is the reduction length itself. Everything below falls out of that.

## 0. Gates, and coverage

**The canon gate passed.** There is no canon: `mock/canon/` does not exist, `mock/crates/` is empty by
the declared mutation order, and `INTENTS.md`, `RULES.md` and op's `83`, `85`, `87`, `88` are what
governs. The work is aligned with op's own word rather than merely permitted by it. `87` section 3
names pricing as a dispatch inside a unit and says it "is not a unit and is not deferred either". I13
makes an arm's region the deliverable, and a crossover is a region. Nothing here recommends one
approach for all cases; section 9 is a set of regions, and the composition is section 3.4.

**The test gate refused three times before it passed**, and the refusals are section 2.3. Applied to a
bench the gate is one question: could the instrument have produced a different answer. Two of the
three refusals were the instrument telling me a defect shape was not expressible at some lengths; the
third told me my written account of the first was wrong.

**Read in full:** `INTENTS.md`, `RULES.md`, op's `83`, `85`, `87`, `88`; `80` sections 5.1 to 5.3 with
`80_probes/p3_asm_report.txt`, `p4_asm_report.txt`, `p4_what_the_law_unlocks.rs`; `82` section 9 with
`82_probes/p4_asm_report.txt`; `89` section 3, section 8 and F4; `90` R10 and sections 5 and 8;
`OPTIONS.md:1870-1931`; the `benchmarking` skill; `variants/warm-clamp-shared/src/lib.rs` and one
committed findings file end to end.

**Not read:** the rest of the panel, the format and number-system units, `91` beyond confirming it
exists and says nothing about pricing.

**Coverage bounds are section 12**, and two of them govern how any of this may be cited. Nothing here
is signed and nothing here is a fixed-point numeral. This is `u8` saturating addition, which is what
`80` and `82` measured, and therefore the only shape on which their numbers can be checked at all.

## 1. Breaking the brief first: two of its premises are wrong

**The brief says `89` "built a const gate that selects between arms and claims it erases completely,
with the selection carrying no trace in the emitted code".** `89` claims no such thing. Its F4
(`89:526-533`) is entirely compile-time: which cfg settings accept, which refuse, and that violating
fragment membership inverts the verdict rather than degrading it. The words belong to `80` section
5.1, which measured a const-gated arm at 3 and 6 instructions against a value-gated one at 13, and to
`82`'s F11, which found three licensed declarations assembling to one symbol.

So the claim exists, it is worth testing, it is not `89`'s, and an expert who went looking for it there
would have reported the wrong absence. I tested it where it lives. Section 6.

**The brief offers the wrapping control as a competitor arm.** It cannot be one: a wrapping fold does
not compute the saturating answer. Reading it as an arm is the strawman failure in the direction
nobody expects, since it would be a strawman that *wins*. Section 4 keeps it as a **ceiling**, measured
over the identical arm set as a second operator rather than as one extra variant, which turns one
number into a per-arm answer and keeps every arm inside a row computing the same value.

## 2. The instrument, and the five things checked before any number

### 2.1 The workload, and the degeneracy that voids a saturating bench by default

A saturating `u8` accumulator over a long slice pins at 255 after a handful of elements and stops
depending on its input. The prior panel's `141` shipped six void rows to exactly this and
`bench-warm-clamp-shared`'s module documentation records it. The consequence for an instrument is
worse than a wasted row: if every arm returns 255 whatever the input, the cross-check between arms
cannot fail and a wrong arm passes.

So the workload folds a column of `n` bytes as `n / L` independent reductions of length `L`, combining
the per-reduction bytes with a rotate and an exclusive or, and `L` is the swept axis. Elements are
drawn so reductions **alternate** between an expected sum of 160, under the limit, and 500, over it, so
exactly half clamp by construction.

That alternation replaced a first design drawing every reduction from one distribution centred on the
limit, and the arithmetic is worth stating because it is the shape that ships: at `L = 4096` the small
column holds eight reductions, so the chance of all eight landing on one side of the limit is about
one in 125 per seed, and the harness runs a hundred seeds per row. A degeneracy check that fires on
half the runs is not a check.

`Case::validate_output` enforces the intent rather than trusting the comment: it rejects a run where
every reduction returned the same byte, and one where the saturated fraction leaves 20% to 80%.

### 2.2 The oracle runs, which in this directory it does not by default

This is a defect in the sibling benches and it is worth more than my numbers.

The harness picks its validation strategy from one flag. With `outputs_may_differ` false, which is the
default, it does byte-exact cross-variant comparison and **never calls `validate_output`**. The pinned
revision is `084e780`, and the code is explicit about it at
`bench-harness/src/validation.rs:105-113`:

```
    // The validator is only meaningful when the Routine actually
    // declared one; we cannot tell from the bridge alone, so use
    // outputs_may_differ as the consent signal.
    let validator: Option<fn(&[u8], &[u8]) -> Result<(), String>> =
        if routine.bridge.outputs_may_differ {
```

**Twelve crates in `mock/benches/variants/` write a `validate_output`. Not one sets the flag.** Ten
are the shared data-model crates the live benches route through; two are the standalone benches
currently disabled by the crate-tree deletion.

```
$ grep -ln "fn validate_output" variants/*/src/lib.rs
variants/bitpack-carrier-shared/src/lib.rs
variants/bitpack-footprint-shared/src/lib.rs
variants/bitpack-shared/src/lib.rs
variants/bitpack-plan-shared/src/lib.rs
variants/bitpack-wide-shared/src/lib.rs
variants/quantiser-fadd-shared/src/lib.rs
variants/quantiser-radix-shared/src/lib.rs
variants/satfold-shared/src/lib.rs
variants/spectral-bisection/src/lib.rs
variants/structural-decomposition/src/lib.rs
variants/warm-clamp-shared/src/lib.rs
variants/warm-container-shared/src/lib.rs
variants/wide-rung-shared/src/lib.rs

$ grep -ln "outputs_may_differ" variants/*/src/lib.rs
variants/satfold-shared/src/lib.rs
```

So every oracle in this directory is dead in the harness path, and the committed logs say so in the
one word that changes between the two branches (`bench-harness/src/validation.rs:461` against `:466`):

```
mock/research/202608072330_the-numeral-canon-panel/27_probes/wide_run.log:2:
  Validation OK: all 4 variants produce identical output
```

That is the byte-comparison branch. This bench's own log reads `produce valid output`
(`92_probes/const_gate_run.log:4`), which is the other one.

**And two panel sentences are wrong because of it.** `26:220-221` says the naive decoder "exists in the
sweep as the independent oracle the validation uses". It does not; the harness never called it.
`26:408-411` says `CarrierColumn::validate_output` "runs four independent checks rather than one" and
concludes "A defect shared between `pack` and `sum_windowed`, which touch the same period arithmetic,
is therefore not invisible." The four checks exist and are correct and the harness never ran them, so
a defect shared between two functions in the same shared crate is **exactly** what the validation
that did run cannot see, since it compares the arms against each other. The claim is the reverse of
the truth and it is the load-bearing sentence of that file's correctness argument.

`26:414` then says "Section 7.2 demonstrates the same refusal against the live harness by injecting a
defect into a built arm." Section 7.2 of that file is a crossover table with no defect injection in
it. And injecting a defect into **one** arm is caught by byte comparison, so even had it been there it
could not distinguish the validator from the comparison, which is the undiscriminating-control failure
this unit has already found four instances of (`89` section 1). Five.

The remedy is one line per crate and I have not made it, because changing another bench's validation
strategy invalidates its committed artifacts and that is a decision for whoever owns those rows.
**Anyone citing a bitpack, quantiser, warm-container, warm-clamp or wide-rung row as validated against
an independent reference is citing something that did not happen.** Cross-variant agreement did
happen, on a hundred seeds, and is a real if weaker check.

### 2.3 Four wrong kernels, and the one that could not be caught

`cargo test --release -p bench-satfold-shared`, eleven tests, transcript at `92_probes/tests.txt`.

A kernel using the **wrong operator** is caught at every length on both operators. One **dropping a
lane of sixteen** is caught from `L = 16` up, and at 8 and 15 a reduction has no index congruent to 15
so the defect is the identity there. One **dropping the ragged tail** is caught wherever
`L % 16 != 0`. Both scopes were found by the assertion failing:

```
assertion `left != right` failed: a dropped lane was not caught at l=8
```

which is the instrument reporting that a defect shape is not expressible at that length, and the
correct response is to scope the claim rather than to widen the test until it passes.

The fourth is the useful one. A kernel dropping the **last single element** of each reduction is caught
up to `L = 1024` and **not at 4096**: there the element draw is Bernoulli at 255/4096 and the small
column holds eight reductions, so with probability around 0.6 no reduction's last element is nonzero.
That bound is now a test asserting both halves, so a change to the distribution that moves it fails
loudly.

It also caught me. My first written account of it said the bound was 256, because the original test
asserted the defect was caught everywhere and panicked at the first length where it was not; that
panic named 4096 and said nothing about 1024, and I wrote 1024 into the prose anyway. Running the
scoped assertion is what corrected it:

```
a one-element defect became visible at l=1024; the element distribution moved and
this bound has to be re-derived
```

**So this bench's cross-check is not sensitive to a one-element defect at `L = 4096`.** It is sensitive
to every defect shape the arms can have, because the arms differ in association order and in remainder
handling rather than in which elements they touch.

### 2.4 The timed region holds the kernel and nothing else

The variant crates are built without fat LTO, matching every sibling family here, so a kernel reaching
the timed region through a cross-crate call would be timed with that call in it. Every kernel, the
column driver and the macro-generated arm entry carry `#[inline(always)]`, and the check is the
disassembly: across all thirteen dylibs the only `bl` instructions inside `bench_entry` are into
`core::panicking::panic_fmt` and `core::panicking::panic_cannot_unwind`, and the non-panic call list is
empty (`92_probes/disasm.txt`).

The same dump reproduces `80`'s one qualitative claim and extends it. `uqadd` appears in `lanes16`,
`lanes16_constl`, `lanes64`, `neon`, `neon8` and `gate_true`, and in **none** of `seq`, `iterfold`,
`nolaw`, `lanes4_idx`, `gate_false`. Two of those absences are new: the licensed-but-bounds-unprovable
arm gets no vector saturating add at all, and neither does the bounds proof without the law.

### 2.5 The noise floor, measured with an unimprovable control

Section 6 produced, incidentally, the best noise-floor control available: two dylibs whose
`bench_entry` is **byte-identical**. `satfold-lanes16-3` and `satfold-gate-true` hash to
`58ba8128fad06b30aa0bda07b9fc167b8fe2c91c` on the normalised instruction stream. Their medians:

| L | lanes16-3 | gate-true | CIs |
|---|---|---|---|
| 64 | 4068 ns | 4069 ns | [4011, 4139] and [4014, 4098], overlapping |
| 256 | 1509 ns | 1438 ns | [1471, 1534] and [1435, 1460], **disjoint** |
| 4096 | 1530 ns | 1548 ns | [1526, 1544] and [1546, 1561], **disjoint** |

**Identical machine code produces disjoint confidence intervals at two of three rows, differing by up
to 4.9%.** So on this bench a difference under about 5% between two separately built dylibs is not a
difference, whatever the interval says, and every ratio below is read against that floor. The
significance machinery is measuring within-row batch variation and cannot see the between-dylib
component; `26` built a dedicated control arm for the same reason (`26:417-424`), and here it fell out
of the erasure test for free.

## 3. The length sweep, and where each arm's advantage begins

Small column, 32 KiB, L1-resident, aligned, saturating. Speedup against the fold as written; full
medians, per-element costs and intervals at `92_probes/tables.txt`, raw run at `92_probes/l1_run.log`.

| L | iterfold | nolaw | lanes4-idx | lanes16 | lanes16-constl | lanes64 | neon | neon8 |
|---|---|---|---|---|---|---|---|---|
| 8 | 1.01 | 0.99 | **1.08** | 1.01 | 1.01 | 1.00 | 1.00 | 1.01 |
| 15 | 1.01 | 1.01 | **1.28** | 0.99 | 0.99 | 1.00 | 1.01 | 0.98 |
| 16 | 0.99 | 1.00 | 1.32 | 0.99 | 1.00 | 0.99 | **6.77** | 6.79 |
| 17 | 1.00 | 1.00 | 1.36 | 0.99 | 1.00 | 0.99 | 6.48 | 6.42 |
| 32 | 1.00 | 1.03 | 1.72 | **1.66** | 1.67 | 1.03 | 17.07 | 17.33 |
| 63 | 1.00 | 1.27 | 10.93 | 2.60 | 2.62 | **1.00** | 6.82 | 6.57 |
| 64 | 1.01 | 1.31 | 14.51 | 7.05 | 6.88 | **3.71** | 55.49 | 53.53 |
| 65 | 1.04 | 1.29 | 14.11 | 6.24 | 6.34 | 3.70 | 52.17 | 52.46 |
| 128 | 1.00 | 1.31 | 13.90 | 15.73 | 15.62 | 6.88 | 102.97 | 103.88 |
| 256 | 1.00 | 1.30 | 5.39 | 26.72 | 26.66 | 13.79 | 135.10 | 140.03 |
| 1024 | 1.00 | 1.31 | 3.16 | 49.57 | 49.45 | 37.65 | 133.36 | **178.19** |
| 4096 | 1.00 | 1.30 | 2.78 | 27.30 | 27.35 | **62.26** | 115.23 | 163.58 |

### 3.1 Every arm has a threshold and it is its own lane count

`lanes16` is at parity with the fold as written for `L <= 17` (0.99 to 1.00, intervals overlapping)
and first pays at `L = 32`, at 1.66x. `lanes64` is at parity through `L = 63` and first pays at
`L = 64`, at 3.71x. `neon` is at parity at 8 and 15 and pays from 16.

The reason is mechanical and it is why `80`'s single-shape measurement could not see it. Below its
lane count an arm's vectorised path is never entered: `chunks_exact(16)` over a 15-element slice yields
nothing and the whole reduction goes through the scalar remainder loop, which is the fold as written.
At exactly the lane count it is entered once, and the horizontal combine then costs as much as the
work it replaced: `lanes16`'s combine is sixteen scalar saturating adds, which is the same sixteen
adds, which is why `L = 16` is 0.99x and not 6x.

`neon` escapes that at `L = 16` precisely because its combine is a four-step vector tree rather than
sixteen scalar steps, and that is the whole of its 6.77x there.

### 3.2 The ragged lengths cost more than the ragged elements

`L = 63` against `L = 64` is the sharpest boundary in the table. `lanes64` goes from 3.71x to
**1.00x**, exactly the fold as written, because `chunks_exact(64)` yields nothing at 63. `neon` goes
from 55.49x to 6.82x, an eightfold loss, because it falls back to its 16-element path. `lanes16` goes
from 7.05x to 2.60x.

`L = 65` costs almost nothing by comparison: `lanes64` 3.70x against 3.71x, `neon` 52.17x against
55.49x. So the penalty is not "ragged"; it is **being one element short of the first full vector**,
and one element long is nearly free. The predicate a design would gate on is `L >= lanes`, not
`L % lanes == 0`.

### 3.3 The mechanism, which is not instruction count

The fold as written costs 0.244 ns per element at `L = 8` and 1.273 at `L = 4096`, a factor of 5.2,
with the same instructions per element throughout. Nothing about its instruction stream changes with
`L`.

What changes is how many independent reductions the machine has in flight. Each reduction starts from
a fresh accumulator, so at `L = 8` the 4096 reductions are 4096 independent three-instruction chains
and the machine overlaps them; at `L = 4096` there are eight, and the cost is the latency of one
saturating add times 4096, eight times over. **The fold as written is latency-bound in the reduction
length, and reassociation is the removal of that dependency, not the removal of instructions.**

Two things follow, and both contradict a reading based on instruction counts.

**More instructions can be faster.** `lanes4-idx` is `80`'s `sat_sum_lanes`, measured by `80` at 8.500
instructions per element against the fold's 6.000 and reported as "worse than doing nothing"
(`80:472`, `80:477-479`). It is **faster at every length here**, from 1.08x at `L = 8` to 14.51x at
`L = 64`. It emits no vector instruction at all (`92_probes/disasm.txt`: `uqadd` 0). It wins by having
four dependency chains instead of one, and the bounds-check instructions `80` counted against it are
free because the machine was never throughput-bound.

**Its advantage then decays**, 14.51x at 64 to 2.78x at 4096, which is the other half of the same
mechanism: four chains stop being enough once the chains themselves are long, and the vector arms with
16 and 64 lanes take over exactly there.

### 3.4 The composition, since no arm is the answer

Reading down the table, on this host, at these shapes:

- **`L < 16`:** no reassociation pays. The only arm above parity is `lanes4-idx`, at 1.08x to 1.28x,
  which is inside the 5% floor at `L = 8` and outside it at 15. Nothing vectorises.
- **`16 <= L < 32`:** hand-written NEON only, 6.4x to 6.8x. Every compiler-vectorised arm is at parity.
- **`32 <= L < 64`:** `lanes16` enters at 1.66x, NEON at 17x, `lanes64` still at parity.
- **`L >= 64`:** all of them, and the ordering among them changes twice more. `lanes16` beats
  `lanes64` until `L = 1024` and loses to it at 4096. `neon` beats `neon8` until `L = 256` and loses
  to it at 1024 and 4096.
- **Anywhere `L` is one element short of an arm's lane count**, that arm is its own fallback.

That is five regions and a boundary rule, which is what I13 asks for and is the honest shape of the
answer. It is not a recommendation that the design carry five arms; it is where the regions are.

## 4. The wrapping ceiling, per arm

`80:490` says the licensed arm "lands within 13% of the density the backend achieves on the case where
it needs to be told nothing", from 0.141 against 0.125 instructions per element. Measured as time, over
the identical arm set with the operator swapped, saturating divided by wrapping:

| L | seq | nolaw | lanes4-idx | lanes16 | lanes64 | neon | neon8 |
|---|---|---|---|---|---|---|---|
| 8 | 2.65 | 2.70 | 2.48 | 2.62 | 2.65 | 2.68 | 2.66 |
| 16 | 6.91 | 6.75 | 5.10 | 7.03 | 6.30 | **1.03** | 1.03 |
| 64 | 73.3 | 60.7 | 5.35 | 10.8 | 18.4 | **0.98** | 1.06 |
| 256 | 156.0 | 120.6 | 12.8 | 6.27 | 12.1 | 0.94 | **0.87** |
| 4096 | 146.9 | 67.2 | 26.3 | 5.20 | 2.90 | 1.48 | **1.00** |

**The 13% figure is right about instruction density and wrong about time by an order of magnitude for
the arm it was said of.** `lanes64`, which is `80`'s 0.141 arm, costs **2.90x** the wrapping form at
`L = 4096`, not 1.13x.

But the ceiling is reachable, and the gap is not the saturating instruction. Hand-written NEON is at
0.98 to 1.06 from `L = 16` up, and the eight-accumulator version is at 1.00 at `L = 4096` and
**0.87 at `L = 256`**, faster than the wrapping fold the backend vectorises unaided. So `uqadd` is not
meaningfully more expensive than `add` on this host. The whole of the 146.9x at `L = 4096` is the
compiler declining to reassociate an operation it cannot prove associative, and the whole of the
residual 2.90x for `lanes64` is the compiler's reassociation being worse than a hand-written one.

The other end of the table is the one nobody would predict from any of this: **at `L = 8` the
saturating operator costs 2.6x across every arm without exception**, including the ones that vectorise
nothing. At that length the wrapping fold vectorises and the saturating fold does not, and no arm
changes that.

## 5. Alignment, and what did not move

Column start offset one byte from a 64-byte boundary, divided by the aligned row, same arm and length:

| L | seq | nolaw | lanes4-idx | lanes16 | lanes64 | neon |
|---|---|---|---|---|---|---|
| 16 | 0.983 | 0.983 | 0.983 | 0.986 | 0.975 | 0.990 |
| 64 | 0.987 | 0.983 | 0.983 | 0.985 | 0.985 | 0.990 |
| 256 | 1.005 | 0.998 | 1.015 | 1.015 | 1.001 | 1.000 |
| 4096 | 1.006 | 1.002 | 1.008 | 1.014 | 0.975 | 1.006 |

Every entry is within 2.5% and the sign is not consistent, which is inside the noise floor section 2.5
measured. **Misaligning the column start costs nothing here**, including for the arms doing 16-byte
vector loads. That is a measured negative on the axis the dispatch flagged as most likely to turn a
clean speedup into a loss, and it is a fact about this host: an M1 has no penalty for unaligned NEON
loads that do not cross a page.

The **other** thing that did not move is the static lever. `lanes16-constl` is `lanes16` with the
reduction length lifted from a runtime value to a const generic, so the kernel sees a known length and
a known-empty remainder. It is within 0.99 to 1.02 of `lanes16` at every length on every row, which is
inside the floor. **Knowing the fold length at compile time buys nothing for this kernel**, and that
belongs on the droplist so nobody spends the effort again. It is one instance and does not generalise
to static knowledge as a class; the workspace rule `small-wins-compound-into-the-program.md` says a shortcut measuring
flat is a result about that shortcut.

## 6. The const gate erases, as an identity rather than as a small cost

The claim under test is `80` section 5.1's and `82`'s F11: a const-gated arm carries no trace of its
predicate. The test is a gated arm and the same arm ungated, plus a gate over a law that is false so
it selects the fallback, which is the control that makes the pair mean anything.

Both verdicts are computed rather than asserted: `saturating_add_is_associative_at(6)` and
`saturating_sub_is_associative_at(6)` are the same exhaustive sweep in a `const fn` over two
operations, 262,144 triples each, and a test asserts one holds and the other does not.

**Byte-identical.** The first comparison was contaminated: `satfold-lanes16` declares twenty-seven
sizes because it is an arm in five sections, and `satfold-gate-true` declares three, so their
`bench_entry` hashes differed for a reason with nothing to do with the gate. `satfold-lanes16-3` is
the size-matched control, and against it:

```
  lanes16          dd2e4e7b40ad8136d0c0c7da8d190f17fe0b4423
  lanes16_3        58ba8128fad06b30aa0bda07b9fc167b8fe2c91c
  gate_true        58ba8128fad06b30aa0bda07b9fc167b8fe2c91c
  gate_false       f336e41478f345636a5b7376307c79ea95956863
  seq              4a5e9a45f3e9aabe817f31a4a3c0a86af55b4f44
```

The full dumps are at `92_probes/bench_entry_lanes16_3.s` and `bench_entry_gate_true.s`, 289
instructions each, and `diff` reports exactly two differing lines, both being objdump printing the
dylib's own filename.

**The harness agrees, using its own instrument, once that instrument was connected.** It exports an
automatic disassembly comparison as `check_disasm_duplicates` (`bench-harness/src/lib.rs:84`), which
the `benchmarking` skill lists among the reasons to use the framework, and **arvo's driver never
called it**, so no bench in this repository has ever been told that two of its arms compile to the
same machine code. Wiring it in is four lines and it now reports, on every row of this section:

```
  WARNING: 1 variant pair(s) have identical machine code:
    release == release
```

One pair, which by instruction count can only be `gate_true` and `lanes16_3`. The label is useless
because `bench-harness/src/disasm.rs:288-289` takes `path.rsplit('/').nth(1)`, the second-from-last path component,
which is always the parent directory. That is an upstream one-character defect and it is why nobody
would have noticed the instrument was silent even if it had been called.

**The false gate selects.** `gate_false` is 183 instructions with zero `uqadd` and times at 1.00x of
the fold as written on all three rows, to three digits. So the gate is doing the selecting, and the
agreement of the other two is not two names for one thing.

**Q39 consequence.** `OPTIONS.md:1920-1924` prices Q39's option (b), the value-gated arm, at `80`'s 13
instructions against 6 and 3, and I15 closed it on principle (`85:20-25`). Option (a) now has its
positive measurement rather than only its rival's negative one: the const-gated arm is not
approximately free, it is **the same program**.

## 7. Attacking the plateau: eight accumulators

`neon` does not improve monotonically. Its per-element cost is 0.00867 ns at `L = 256`, 0.00946 at
1024 and 0.01105 at 4096: it gets **worse** as the reduction lengthens, which is the signature of a
loop-carried dependency rather than of a bandwidth limit, since bandwidth does not care about `L`. At
`L = 256` the small column holds 128 independent reductions to overlap; at 4096 it holds eight, and
the four accumulator chains are then the whole of the available parallelism.

Reporting that would have been half the work. `satfold-neon8` is the same kernel with eight
accumulators and 128 elements per iteration, which is one const generic on the shared kernel and
nothing else:

| L | neon | neon8 | ratio |
|---|---|---|---|
| 256 | 135.10x | 140.03x | 1.04 |
| 1024 | 133.36x | **178.19x** | 1.34 |
| 4096 | 115.23x | **163.58x** | 1.42 |

Flat within the noise floor at every length up to 256, and 34% and 42% at the two longest. **The
plateau was accumulator-chain latency and it is recoverable**, which also closes section 4's residual:
`neon8` at `L = 4096` reaches exactly the wrapping fold's time.

Where it stops: at 0.00778 ns per element, `neon8` is moving 32,768 bytes in 255 ns, which is
128 GB/s, roughly 40 bytes per cycle at this host's clock. An M1 firestorm core issues at most three
16-byte loads per cycle. **The arm is at the load-port ceiling and sixteen accumulators cannot help**,
which is why I did not build that one.

## 8. What the working set does, and does not, change

16 MiB column, past this host's 12 MiB L2, against the 32 KiB column at the same lengths and arms.
Speedups against the fold as written:

| L | arm | 32 KiB | 16 MiB |
|---|---|---|---|
| 16 | neon | 6.77 | 6.61 |
| 16 | everything else | 0.99 to 1.32 | 1.00 to 1.33 |
| 64 | neon | 55.49 | **40.90** |
| 64 | lanes4-idx | 14.51 | 13.38 |
| 64 | lanes16 | 7.05 | 6.99 |
| 64 | lanes64 | 3.71 | 3.70 |
| 64 | nolaw | 1.31 | 1.32 |
| 4096 | neon | 115.23 | **61.88** |
| 4096 | lanes64 | 62.26 | 50.15 |
| 4096 | lanes16 | 27.30 | 25.59 |
| 4096 | lanes4-idx | 2.78 | 2.77 |
| 4096 | nolaw | 1.30 | 1.30 |

**The working set caps only the arms that were already near the load-bandwidth ceiling.** Everything
below about 20 GB/s is unchanged to within 2%, and the fold as written costs the same per element at
both sizes (1.273 ns against 1.277 at `L = 4096`), meaning it never touches memory bandwidth at all:
at 0.78 GB/s it is roughly eighty times below what the same column delivers to the wrapping arms.

The cap itself is around 48 GB/s: `neon` at `L = 4096` runs at 90 GB/s on the small column and 48.5
on the large one, and `lanes64` at 39.3. So the ratio a reassociation buys is bounded above, at these
shapes, by roughly 48 GB/s divided by the scalar arm's 0.78, which is the 62x observed.

One row measures the wrapping form at 16 MiB, and every arm lands within 9% of every other, at 0.0166
to 0.0181 ns per element. That is the memory system and nothing else, and it is what the saturating
arms would look like if the operator cost nothing.

## 9. Findings, in the required predicate notation

Absence of a dimension is the strongest negative statement in the notation and is meant everywhere it
appears. **No finding here lists a strategy**, because none was measured: this bench times machine
kernels over byte slices, not arvo types, so nothing below may be read as a statement about `Hot`,
`Cold` or any successor, and any such reading is drift.

Shared by every finding unless overridden: `element type = u8 unsigned, width = 8, F = 0, arity = fold
over a runtime-length slice, operation = add, container = contiguous byte slice, access pattern =
sequential, host = Apple M1 aarch64-apple-darwin, target features = aarch64 baseline with no
target-feature flags, toolchain = nightly-2026-05-28, threads = 1, harness = mockspace-bench-harness
at 084e780`.

**G1. The fold as written is latency-bound in the reduction length, not throughput-bound.** Shared
predicate, plus `policy = saturate, L = 8..=4096 at the twelve values measured, column = 32 KiB`. Cost
per element rises monotonically from 0.244 ns at `L = 8` to 1.273 ns at `L = 4096`, a factor of 5.2,
with instructions per element constant. `92_probes/tables.txt`, section `satfold-length-l1`.

**G2. An arm with more instructions per element is faster at every measured length.** Shared, plus
`policy = saturate, L = 8..=4096 at the twelve values measured, column = 32 KiB`. `lanes4-idx`,
measured by `80` at 8.500 instructions per element against the fold's 6.000 and reported worse than
doing nothing, is 1.08x at `L = 8` and 14.51x at `L = 64`, emitting no vector instruction. **This
refutes `80`'s stated conclusion for that arm as a time claim**, per `RULES.md:509-518` in this
deliverable rather than by editing `80`.

**G3. Each reassociated arm is at parity with the fold as written below its own lane count, and its
advantage begins at that length.** Shared, plus `policy = saturate, column = 32 KiB`. `lanes16` at
parity for `L <= 17` and 1.66x at 32; `lanes64` at parity for `L <= 63` and 3.71x at 64; `neon` at
parity for `L <= 15` and 6.77x at 16. Intervals in `92_probes/tables.txt`.

**G4. One element short of an arm's first full vector costs that arm its whole advantage; one element
long costs almost nothing.** Shared, plus `policy = saturate, column = 32 KiB, L = 63, 64, 65`.
`lanes64` 1.00x, 3.71x, 3.70x. `neon` 6.82x, 55.49x, 52.17x.

**G5. The bounds proof alone buys 1.30x and the law buys the rest.** Shared, plus `policy = saturate,
L = 63..=4096, column = 32 KiB and 16 MiB`. `nolaw`, the identical `chunks_exact(16)` with the chain
left serial, is 1.27x to 1.32x across that range while `lanes16` reaches 49.57x. This reproduces
`82`'s signed attribution claim (`82:493-495`) as a time claim in the unsigned case, which `82` did
not measure and `80` did not run at all.

**G6. Lifting the reduction length from a runtime value to a compile-time constant buys nothing for
this kernel.** Shared, plus `policy = saturate and wrap, L = 8..=4096 at the twelve values, column =
32 KiB and 16 MiB, alignment = 0 and 1`. `lanes16-constl` against `lanes16` is 0.99 to 1.02
everywhere, inside the 4.9% floor of G10.

**G7. Misaligning the column start by one byte costs nothing measurable.** Shared, plus `policy =
saturate, L = 16, 64, 256, 4096, column = 32 KiB, alignment = offset by one byte against 64-byte
aligned`. Ratios 0.975 to 1.015 across eight arms, sign inconsistent.

**G8. The saturating operator itself is not the cost; the compiler's refusal to reassociate is.**
Shared, plus `L = 16..=4096 at the five values measured on both operators, column = 32 KiB`.
Saturating over wrapping is 146.9x for the fold as written at `L = 4096` and 1.00x for the
eight-accumulator hand-written arm at the same length, 0.87x at `L = 256`. **`80:490`'s "within 13%"
does not survive as a time claim for the arm it was said of**: `lanes64` is 2.90x the wrapping form.

**G9. A const-gated arm is byte-identical to the same arm ungated.** Shared, plus `policy = saturate,
L = 64, 256, 4096, column = 32 KiB, encoding = a const fn exhaustive sweep at six bits inside an inline
const block, gate arity = two-way`. Normalised `bench_entry` instruction streams hash equal,
`diff` differs only in objdump's filename banner, and the harness's own duplicate check reports one
identical pair per row. The false-verdict gate selects the fallback and times at 1.00x of it.

**G10. Two dylibs with byte-identical timed regions differ by up to 4.9% with disjoint confidence
intervals.** Shared, plus `policy = saturate, L = 64, 256, 4096, column = 32 KiB`. So on this harness
a between-dylib difference under about 5% is not a difference, and the per-row intervals understate
the true uncertainty of a cross-dylib comparison.

**G11. Doubling a hand-written arm's accumulators recovers its plateau at long reduction lengths and
nothing below them.** Shared, plus `policy = saturate, L = 8..=4096 at the twelve values, column = 32
KiB`. `neon8` over `neon` is within the floor up to `L = 256` and 1.34x at 1024, 1.42x at 4096.

**G12. A working set past every cache level caps only the arms already near the load-bandwidth
ceiling.** Shared, plus `policy = saturate, L = 16, 64, 4096, column = 32 KiB and 16 MiB`. Every arm
below about 20 GB/s has the same ratio at both sizes to within 2%; `neon` falls from 115.23x to
61.88x at `L = 4096`. The fold as written costs 1.273 ns per element at 32 KiB and 1.277 at 16 MiB.

**G13. Twelve bench crates in this repository write an oracle the harness has never called.**
`harness = mockspace-bench-harness at 084e780, encoding = the Routine bridge's outputs_may_differ
flag`. `bench-harness/src/validation.rs:105-113`; `grep -ln "outputs_may_differ" variants/*/src/lib.rs`
returns one file, this bench's. Behavioural confirmation in a committed log at
`27_probes/wide_run.log:2`.

## 10. Fits against the register, and what this kills

**Q39 (whether an arm's predicate may read data), `OPTIONS.md:1907-1931`.** Option (a) gains its
positive measurement: G9. Nothing here reopens (b), which I15 closed on principle.

**Q38 (where a law verdict's truth is established), `OPTIONS.md:1880-1905`.** Untouched. This file
prices what an established verdict is worth, not how it is established.

**R10 of `90` (where a law pays is strictly narrower than where it is true).** Confirmed and made
sharper. R10's narrowing is "where the backend could not have proved it"; G3 adds a second, independent
narrowing on the same axis the law is applied over: **where the reduction is long enough for the
reassociation to have anything to break.** The two are not the same and both bind.

**Kills nothing outright.** It costs `80`'s reading of its own `sat_sum_lanes` row, which was a reading
rather than an option, and it costs the generality of `80:490`'s 13% figure, which was a magnitude
rather than an option.

**Adds one option, which I state and do not resolve.** The measured shape is that reassociation pays
in bands of `L`, and `L` is not typestate: it is a slice length, known at compile time only sometimes.
G6 says lifting it to a constant buys nothing **for the kernel**, but that is not the same question as
whether it can gate an arm, and a band boundary that cannot be read at const time cannot select an arm
under I15 at all. Whether the design wants a `L >= lanes` predicate, what it reads it from, and what
happens where it is unavailable, is Q39's shape one level down and nobody has posed it.

## 11. Unlicensed mechanisms and defects found outside the question asked

**The dead oracles, and two sentences resting on them.** Section 2.2, in full, with the grep, the
harness source and the committed log. `26:220-221` and `26:408-411` state the reverse of what the
validation did, and `26:414` cites a section that contains no such demonstration. Nothing in this
unit's chain depends on those rows, and the container fork does.

**The driver never called the duplicate check.** Section 6. Now wired, four lines at
`mock/benches/src/main.rs`, and it fires immediately on the very first section it was pointed at,
which is a reasonable measure of how long it had been silent.

**The harness's duplicate report names the wrong path component.** `bench-harness/src/disasm.rs:288-289` uses
`rsplit('/').nth(1)`, printing the parent directory, so its output reads `release == release` for
every pair. Upstream, one character, and it makes an otherwise sound instrument useless without a
manual follow-up.

**A `[timing]` block that is global and reads as per-bench.** The `benchmarking` skill describes a
"per-bench `timing` override of `passes`, `runs_per_pass`, `batch_size`, `harness_runs` and
`cooldowns_ms`". In the pinned revision `TimingSection` sits on `BenchManifest` and `BenchSection` has
no timing field, so the block at `bench.toml:160` governs every bench in the file and appears, by
position, to govern only the one above it. I did not touch it, for exactly that reason.

## 12. Coverage, and what was not measured

**Not measured, so not claimed, and under the notation not true:** anything signed; any fractional
width; any width other than 8; any strategy; more than one thread; any target feature beyond the
aarch64 baseline; any host but this M1; any container but a contiguous byte slice; any access pattern
but sequential; multiplication, subtraction, or any operation but addition; any arity but a fold; the
`neon8` and `lanes16-constl` arms at 16 MiB, which were added after those rows ran and are absent from
them; and alignment at 16 MiB.

**Measured once rather than three times.** The L1 length sweep ran twice, before and after `neon8` was
added, and the two runs agree to 0.4% on the fold as written at `L = 4096` (41538 ns against 41713),
which is one reproduction rather than three. Both logs are committed (`92_probes/l1_first_run.log`,
`l1_run.log`). Every other section ran once.

**Interrupted.** The first DRAM run was killed after two of four rows; the remaining rows were moved to
their own `bench.toml` section and rerun, which is why `satfold-length-dram` holds two rows and
`satfold-length-dram-long` holds one. Both logs are committed. No row is a partial measurement.

**The instrument's own blind spot** is section 2.3: no sensitivity to a one-element defect at
`L = 4096`.

**Where I am least certain, as a floor for whoever attacks this.** The dependency-chain mechanism in
3.3 is an inference from the shape of the per-element curve plus the `lanes4-idx` inversion, and it is
not directly instrumented: the harness's `PerfSnapshot` carries instructions and cycles and this run
did not collect them, because the counters need root on this host. A cycles-per-element series across
`L` would confirm or break it in one run and I did not take it. Second, section 4's claim that `uqadd`
is not meaningfully more expensive than `add` rests on `neon8` reaching 1.00x at one length and 0.87x
at another, which is two points and could be two different effects cancelling. Third, the 48 GB/s cap
in section 8 is read off two arms at one length and is not a measured bandwidth.

**Nothing here settles anything.** It prices what was unpriced and it names regions. `87` says the
canon is written once at the end from every consolidation at once, and this is input to that.
