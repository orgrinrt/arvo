# 20. What the benches already know

**Date:** 2026-08-08. **Author:** Fog persona. **Status:** complete. Nothing here settles anything.

This file reads a body of evidence the panel has never once opened. `mock/benches/` holds 147 committed
harness runs. Nineteen files of design argument were written without a single one of them naming that
directory, and `19` found it.

Two jobs. The first is op's, undischarged since `142b`: the bench work behind Warm's headroom rule is "one
instance of evidence, completely unaudited by a second expert set of eyes". I am the second set of eyes.
The second is the panel's: what the repository already knows, where the existing benches mislead, and what
is missing.

A bench that exists is hard to deny; where it is deniable the thing is still settling. That is op's own
position and it applies to everything below, including my own numbers.

## Zero: the brief's own claims, checked before reasoning from them

The dispatch told me to assume it is wrong somewhere and to verify its cheap factual claims. Two were
checkable.

**"147 committed findings files."** True.
`find mock/benches -name '*findings*' -type f | wc -l` returns `147`.

**The packed-storage numbers quoted from `bitpack-sequential-sum_n16384`.** True as quoted, and I
reproduced the medians myself from the CSV rather than from the findings prose. What is *not* true is the
inference drawn from them, and that is section 2.5.

**One claim in the brief is wrong in a way that matters.** It says `mock/benches/` "is the only thing in
this workspace that can price anything". It is the only thing that *can*; it is not the case that
everything in it *does*. Six of its 691 measured cells are physically impossible, its one cross-family
comparison is a splice across two source commits, and its output-fidelity checks did not run in any of the
147. The directory is evidence. It is not automatically correct evidence, and the panel swinging from
"unpriced" to "priced, here is the table" would be replacing one error with another.

## What I read, and what I did not

Read in full: `RULES.md`, `01`, `04`, `19`, the `Cold`-correction and claim-classification sections of
`MORNING.md`, `seed/SETTLED_container.md`.

In `mock/benches/`: the `warm-container-shared`, `warm-clamp-shared`, `bitpack-shared` and
`bitpack-footprint-shared` variant crates in source, every `*.meta.json`, and every `*.csv`
programmatically. I read **11** findings files in prose and derived the rest of my numbers from the CSVs
directly, which is the more reliable route and is why my tables do not always match the findings prose.

**Findings files I did not read and did not use:** the whole `decimal-quantiser-radix-sweep`,
`quantiser-vs-fadd-subnormal-sweep`, `spectral-bisection`, `structural-decomposition` and
`fnv1a-vs-xxhash3` families, roughly 30 files. They bear on the float and algorithm surfaces rather than on
this panel's open questions. I swept their CSVs for the impossible-throughput signature and they are clean;
beyond that they are uncovered by me.

**Not covered:** I did not re-run any bench on the harness. Every timing I quote is from the committed
artifacts. My own probes are ad-hoc quick spikes with no substance as measurements, and are used only for
existence and refutation claims, which is the whole of what they are capable of.

## One: the audit of the headroom bench body

This is op's instruction. The verdict first, then the reasoning.

**The bench crate is good work and its arms are real competitors, not a strawman. Its headline conclusion
survives my reading and is understated rather than overstated. But it measures one semantics, that
semantics may not be the one `Warm` has, and the answer under the other semantics is different in
direction, not just in size.**

### 1.1 The arms are competitors

`mock/benches/variants/warm-container-shared/src/lib.rs` declares six arms over one shared transform. No arm
carries its own copy of the arithmetic, which is the failure mode that turns a bench into four
possibly-drifted programs. The arms are `headroom` (the shipped `rung(rung_bits(W)+1)` rule),
`minimum` (the deletion `140` proposed), `native` (minimum container with the projection written once, which
is what a Rust programmer writes by hand), `kernel` and `lanes_deferred` (eight lane accumulators), and
`plusone` (`rung(W+1)`).

`plusone` deserves its own line because it is the piece most benches in this workspace do not have. It is a
**control**: at each width it compiles to byte-identical code to one of the other two container arms, so the
gap between `plusone` and the arm it aliases is that run's own noise floor. `plusone_is_never_a_third_container`
and `the_control_arm_aliases_a_real_arm_at_every_swept_width` assert it rather than assuming it. A bench that
carries its own noise floor as an arm is rare and it is the right shape.

The crate also records a discarded first cut: the step cycle originally used only affine operations, LLVM
composed the chain into one multiply-add for the lazily-projected form and could not for the eager one, and
the sweep was therefore comparing a collapsed chain against an uncollapsed one. Those numbers were thrown
away and an exclusive or was introduced to break the composition. **A bench that documents its own discarded
run is doing the job.**

### 1.2 The headline survives, and the mechanism is not the one that was reported

`SETTLED_container.md:410-413` records the headroom rule as "measured and condemned by op at `139b:12-22` as
producing unacceptable inflation (reported as roughly 1600 instructions against 81 at 64 bits over a naive
count, later corrected to 339 against 81)". That is an instruction count, not a measurement, and the number
moved by a factor of five between two tellings.

Here is the harness's answer, medians of `algo_ns` in warm mode, computed from the CSVs:

**Wrapping (`op=0`), 8192 elements, 3 operations per element:**

| W | headroom | minimum | native | kernel | lanes-deferred | headroom/minimum |
|---|---|---|---|---|---|---|
| 8  | 8415 | 190  | 187  | 424  | 424  | **44.24x** |
| 13 | 8387 | 8591 | 397  | 826  | 829  | **0.98x** |
| 16 | 8351 | 397  | 396  | 802  | 807  | **21.02x** |
| 32 | 5718 | 813  | 802  | 896  | 898  | **7.03x** |
| 60 | 8385 | 8447 | 2423 | 2226 | 2232 | **0.99x** |
| 64 | 6015 | 2459 | 2446 | 2255 | 2241 | **2.45x** |

The ratio is not monotone, not smooth, and not a single number. It splits cleanly along one line, and the
line is not the one anybody named.

**At W = 8, 16, 32, 64 the declared width exactly fills its minimum container.** There, `minimum`'s
projection is `mask_to(W)` with `W >= C::BITS`, which is the identity and emits nothing, so the arm reduces
to native machine arithmetic and vectorises. Headroom costs 44x, 21x, 7x, 2.45x.

**At W = 13 and 60 it does not.** There, `minimum`'s projection is a real `and` after every operation, which
blocks vectorisation, so `minimum` is already as slow as `headroom` and the container is worth nothing:
0.98x and 0.99x, with headroom marginally ahead.

So the honest statement of the cost is a mechanism rather than a multiple:

> The headroom rule guarantees `C > W` at every width, and therefore guarantees the projection is a real
> instruction at every width. Its cost is not that a wider container is slower. Its cost is that it removes
> the case where the projection would have been free.

That is a better argument against the rule than the one in the record, it is width-banded rather than
uniform, and it holds at both element counts: the same table at 1048576 elements gives 40.69x, 1.00x,
20.40x, 6.93x, 1.01x, 2.39x, which is the same shape.

**And the second-order finding is larger than the first.** `native` differs from `minimum` only in writing
the projection once instead of after every operation. Its medians across the six widths are 187, 397, 396,
802, 2423, 2446: exactly a doubling per container rung, and **completely independent of whether the width
fills its rung**. `native` is fully vectorised everywhere. At W = 13 it is **21.6x** faster than `minimum`
(397 against 8591) with the identical container.

So at the two sub-rung widths, the container is not the cost and deleting headroom buys nothing. Deferring
the projection buys 21.6x. `SETTLED_container.md:422-424` records `142` as having refuted `141`'s
monotonicity claim and recovered "41.0x of a 44x loss" by attacking the losing arm; I did not read `142`
before deriving the table above, and it lands in the same place by a different route.

### 1.3 The one cell the arm set does not fill

The arms cover `{headroom} x {eager serial}` and `{minimum} x {eager serial, eager lanes, lazy serial,
lazy lanes}`. **There is no headroom arm with the projection deferred.** Headroom is measured in one
configuration and its competitor in four, and the one configuration headroom gets is its worst.

I do not think this changes the verdict, and I want to be exact about why rather than wave it through.
Under wrapping, reduction modulo `2^W` factors through reduction modulo `2^C` for any `C >= W`, so a lazy
headroom arm computes the same value as a lazy minimum arm with strictly more container. It cannot win. The
missing cell would make headroom look worse, not better.

But that argument is a proof about wrapping, and it is exactly the argument that does not carry to
clamping. Which is the next section.

### 1.4 The semantics the bench implements may not be the semantics `Warm` has

This is the finding I would put in front of op first, and I am handing the call back rather than resolving
it.

The `warm-container-*` family implements `Warm` as **wrapping** at the declared width, in all six arms.
`warm-clamp-shared/src/lib.rs` says so in its own opening paragraph and gives the reason it exists:

> File `141` benched the container fork with six arms, and every one of them implements `Warm` as
> **wrapping** at the declared width `W`. The standing base's ratified fixed-point preset table
> (`124_consolidation_twelve.md:2604-2612`, ratified in full at `70b`) gives `Warm` and `Cold` the
> resolution **clamp**, and gives `ReduceModulo` (wrapping) to `Hot` alone. So `141`'s `Warm` half measures
> `Hot`'s resolution.

Against that, `seed/SETTLED_container.md:405-408` records the opposite:

> **The `70b`-ratified `Warm` clamp cell in the preset table.** A previously-ratified fixed cell value.
> Declared stale by op's own restated intent ruling [...] (`142b:24-26`) Absorbed into survivor item 13.

And survivor item 13 is op ratifying, twice, that Warm "should behave like native primitives in regular old
rust would".

**So the same round contains a bench built on the premise that Warm clamps and a checkpoint recording op
killing that premise.** I cannot tell from the artifacts which came first within the round, and it does not
matter much: what matters is that the record currently supports both readings and the two instruments give
different answers.

This is not academic. Under wrapping the container can only ever be overhead, because the projection is a
ring homomorphism and both containers reach the same value. Under clamping it is not a homomorphism, and the
extra bits are what keep the interior clamps dead. The direction of the answer changes.

**The question that belongs to op: does `Warm` wrap or clamp?** One sentence decides which of the two
committed bench families is the instrument for the headroom question, and the two of them disagree.

### 1.5 The clamp family, which is the other instrument, and what it says

Read after I had derived the sections above, and I note that I read the commit subject line
`docs: panel file 142, excel everywhere, and the headroom is a fold quantity` while establishing commit
order, which carries a conclusion. Everything above this paragraph predates that; treat this subsection as
contaminated and worth an independent read.

`warm-clamp-arity-w13`, W = 13, 8192 elements, chunked clamping fold, arity swept. Medians from the CSVs,
with the fold arity decoded from the key scheme the crate documents (`KEY = W*10000 + NC*1000 + LOG2A*10 + OP`):

| fold arity | head | min | accfit | acc64 | accfit-dyn |
|---|---|---|---|---|---|
| 2   | 564 | 255  | 258 | 828 | 4329 |
| 4   | 564 | 318  | 273 | 712 | 1918 |
| 8   | 535 | 524  | 536 | 595 | 1160 |
| 16  | 330 | 1085 | 286 | 550 | 755  |
| 64  | 235 | 7134 | 208 | 537 | 279  |
| 256 | 223 | 9789 | 190 | 520 | 195  |

**The headroom arm goes from 2.2x worse at arity 2 to 44x better at arity 256.** Under clamping, the
conclusion of the wrapping bench reverses.

And the crossover is predicted rather than observed after the fact. The crate states the interior-safety
predicate as `W + ceil(log2 n) <= width(accumulator)`. For `min` at W = 13 the accumulator is `u16`, so the
arm is interior-safe exactly while `13 + ceil(log2 a) <= 16`, that is `a <= 8`. **The measured crossover is
at arity 8**, to the row. A bench whose numbers land where its own stated predicate puts them is the
strongest internal consistency check available, and this one has it.

`accfit`, the arm that picks the narrowest accumulator satisfying the predicate, is at or near best at every
arity. It is the design's own rule and it beats both the shipped rule and its proposed deletion.

**So the headroom question is not "keep or delete".** On this evidence it is: under wrapping the doubling is
overhead and the projection is the real cost; under clamping the accumulator width is a quantity that
depends on the fold arity, and a fixed doubling is a crude approximation of it that is too much at low arity
and too little at high. Neither committed family supports a single verdict, and the two of them together
support a composition.

### 1.6 Verdict on op's instruction

The bench body is sound in construction, honest about its own discarded run, and carries a noise-floor
control most benches here do not. Its conclusion about the wrapping case survives a second read and is
better stated as a mechanism than as a multiple. **Keeping it is the result**, with three qualifications:
its arm matrix is asymmetric in headroom's disfavour, its semantics may be the wrong one for the strategy it
names, and it has a sibling family that reverses its direction on the axis it did not sweep.

What it does **not** establish, and was read as establishing: that headroom should be deleted for every
strategy. Six of the six `Precise` cells in that family are void, which is section two.

## Two: where the existing benches mislead

My subject is whether a benchmark measures what it claims. Six findings, in descending order of how much
they change.

### 2.1 Six committed cells are physically impossible, and the harness recommends them

`precise-container-width-l1_*`, the saturating half of the container fork, reports the
`warm-container-kernel` arm at **3.3 to 4.2 ns per call for 8192 elements at 3 operations each**, at all six
widths. The findings file computes the corresponding throughput and prints it:

> Peak throughput: **12365.283 Gops/s** (warm-container-kernel; best 20% batches)

The host is an Apple M1, recorded in every `meta.json`. Taking a generous ceiling of 4 GHz and four
128-bit vector pipes at 16 lanes of `u8`, the machine's arithmetic roofline is 256 Gops/s. The six files
report 12365, 12603, 14979, 15420, 15697 and 17022 Gops/s. **The lowest is 48x the ceiling and the highest
is 66x.**

The harness does not flag it. It does the opposite:

> ### warm-container-kernel dominates: 190167% faster than the next best
> _Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.
>
> ### warm-container-minimum is an outlier: 2534.9x slower than the field

Bootstrap confidence intervals, adjusted p-values, sign tests, autocorrelation diagnostics, tier detection,
and a per-pass consistency table, all applied to a number 66x above what the silicon can do. **There is no
roofline check anywhere in the analysis.** Everything the harness computes is relative, so a number outside
physics passes every test it has.

**The mechanism, established rather than asserted.** `20_probes/p3_dylib_probe/` loads the shipped cdylibs
and calls `bench_entry` exactly as the harness does, timing from outside with a clock the variant cannot
influence:

```
KEY 130103 : W=13 n=8192 op=1 D=3
arm                                  run_ticks   outer ns/call          output value
headroom                                438.27           18274                  8191
minimum                                 474.06           19762                  8191
native                                  389.08           16216                  8191
kernel                                    0.12               7                  8191
lanes_deferred                            0.11               8                  8191
```

The value is correct. The call returns in 7 ns. So it is not a timing artifact: the work is not happening.

`20_probes/p4_absorbing_fixpoint.rs` establishes why, and made three predictions that all held. At W = 13
the step sequence is `min(v+k, lim)`, `sat_sub(v, k)`, `min(v+k, lim)` with `k = 2731` and `lim = 8191`.
Interval arithmetic alone proves every step result is at least `k`, the lane accumulator is
`min(lane + step, lim)`, so **after three elements every lane is provably `lim` and the remaining 1021
iterations per lane are the identity.** LLVM peels, proves the constant, and deletes the loop:

```
arm                                              ns/call      answer
saturating, const k and lim (bench shape)              1        8191
saturating, opaque k and lim (test shape)           4011        8191
wrapping,   const k and lim (no fixpoint)           3093        4096

saturating answer is input-insensitive past element 3: a=8191 b=8191 same=true
wrapping   answer IS input-sensitive:                 a=4096 b=4928 same=false
```

Three consequences, and the second is worse than the first.

**The measurement is void in all six cells.** Not noisy. Measuring a register copy.

**The bench's correctness argument is void in those cells too.** The crate's own doc rests the arm set on
"the harness's cross-variant byte comparison is live on every run and an arm that computes something else is
refused rather than reported as fast". In the saturating cells **the answer does not depend on the input**,
so every arm returns the same constant and an arm that read no data at all would pass. The check is
vacuous exactly where it was needed.

**And the test written to catch this is defeated by its own instrument.** `diag_sat_lanes_actually_runs`
asserts the lane fold takes more than 100 ns and passes, reporting 1311 ns. It passes because it calls
`run_sat_lanes(black_box(&vals), black_box(k))`, and `black_box` on `k` is what hides the constant from the
range analysis that proves the fixpoint. **The diagnostic proves the loop runs in a configuration the bench
does not use.** This is the "setup that helps" shape at its most subtle: the assertion is real, the value is
real, and the configuration is the one where the bug cannot appear.

The same crate's sensitivity test, `the_oracle_is_sensitive_to_a_perturbed_column`, calls
`reference(&vals, 13, 0, 3)`: `op = 0`, the wrapping case, at 64 elements. **The saturating case at the
bench's own element count is never asserted sensitive.**

**Scope, measured rather than estimated.** Sweeping every committed CSV for cells whose warm median
`algo_ns` is under 100 ns returns 12 of 691. Six are the `precise-container-width-l1` kernel cells above.
The other six are at small `n` or on `u8` data and are within the machine's roofline when the arithmetic is
done; I checked each and none is impossible. **The damage is exactly six cells in one family and one arm.**

**And this was already found.** `warm-clamp-shared`'s doc records it: "`141` section 3 records that its
saturating fold over 8192 terms was constant-folded by LLVM". The next bench in the sequence was built to
fix it, with `chunked_answer_depends_on_every_element_the_clamp_did_not_absorb` and
`the_clamp_fires_on_a_real_fraction_of_chunks_at_every_chunked_key` as the checks the void run needed and
did not have. Both are real tests, and I read their bodies rather than their names. Git confirms the order:
`ccf0509` committed the container family, `defc747` committed the clamp crate, same day.

So the finding is not that nobody noticed. It is that **the six findings files still sit in
`mock/benches/`, still declare a 190167% winner, and carry no mark.** A later reader sent to price the
container fork, which is exactly what I was sent to do, finds them by name and nothing on the artifact says
otherwise. The correction lives in a bench crate's doc comment one commit later.

### 2.2 The counter's aarch64 read is not a barrier

`mockspace/bench-core/src/counter.rs:40`:

```rust
core::arch::asm!("mrs {}, CNTVCT_EL0", out(reg) val, options(nostack, nomem));
```

`nomem` is a promise to LLVM that the asm neither reads nor writes memory, so loads and stores may be
reordered across it. The timing bracket is therefore not a scheduling barrier for the work it brackets. The
x86_64 arm of the same function carries an `lfence` and no `nomem`; **the weak path is the one this host
runs on.**

`20_probes/p2_counter_nomem_isolated.rs` shows the serial fold reading 356032 ns and then 47040 ns across
two runs of the same binary with `nomem`, against 8192 ns and 23202 ns without it, on identical work. That
is not a clean effect and I am not claiming a direction from it. What it does establish is that **the
bracket is unstable in a way that depends on an asm option**, which is not a property a timing instrument
should have.

This is not what caused 2.1, and I want to be exact: `p2`'s lane arm reads zero with both counter forms
because loop-invariant motion hoists it out of the repetition, which is my probe's shape rather than the
harness's. The harness calls an opaque cdylib symbol and is immune to that. `nomem` is a separate,
smaller defect that I found while chasing the first one and could not close.

I attacked it and did not finish. What would close it: rebuild one variant against a `bench-core` with
`nomem` dropped and re-run one bench family on the harness, comparing the artifact trails. I did not do
that because it changes a dependency shared by every consumer repo and that is not my call to make inside a
panel dispatch.

### 2.3 The fidelity checks did not run, in any of the 147

Two columns in every committed CSV are supposed to carry the harness's independent check on a variant:
`digest`, the reps-invariant fidelity witness, and `score`, the output scorer.

```
digest values across all committed CSVs: {'0': 55280}
rows 55280 with a score 0
```

**Zero of 55280 rows carries either.** The digest is documented as "all zero for plain timed! variants", and
every variant in this directory is a plain `timed!` variant, so this is the harness working as built rather
than a failure. The consequence is the same either way: **the only check on what a variant computed is the
variant crate's own unit tests**, and 2.1 is what that looks like when those tests are configured where the
bug cannot appear.

### 2.4 `e2e` is not an independent measurement

`bench-harness/src/harness.rs:410-412`:

```rust
let bridge = call_accum.saturating_sub(algo_accum);
batch_e2e_ticks += (fw_end - fw_start).saturating_sub(bridge);
```

`bridge` is defined as everything the variant's own self-report does not account for, and `e2e` is the outer
time minus `bridge`. So when a variant under-reports its own `run_ticks`, the harness attributes the real
cost to bridge and the e2e column moves with it. The two columns then agree, and a findings file reports a
consistent wrong number twice.

In the six void cells this is visible: `algo_ns` 3.7, `e2e_ns` 65.4, `bridge_ns` 2.5. The outer measurement
did not contradict the inner one, because it is not independent of it.

### 2.5 The `Cold` correction in `MORNING.md` is a splice, not a measurement

This is the part I most expected to confirm and could not.

`MORNING.md:43-73` reports the packed-storage trade as priced, and the paragraph that carries the scale
claim is:

> And at seven million elements the footprint benches put packed at 1157 microseconds against dense at 810,
> roughly 1.43x, on the same machine and pin nine minutes apart. **So the penalty turns with scale, and
> nobody in eighteen files said so.**

The two medians are correct. The inference is not supported, for three reasons I can produce with a command.

**No committed run contains both a packed arm and a dense arm.** `bitpack-footprint-packed`'s arms are
`{packed, packed-naive}`; `bitpack-footprint-dense`'s are `{dense, dense-alt}`. The families are disjoint.
Each beats its own sibling: packed beats packed-naive by 4.31x, dense beats dense-alt by 0.6%. Under this
workspace's own standard, **a bench with no real competitor is a demonstration**, and the packed-versus-dense
comparison at 7M is a subtraction across two demonstrations rather than a measurement the harness made.

**The two runs being subtracted are at different source commits.** The `meta.json` files:

| file | timestamp | git_commit |
|---|---|---|
| `bitpack-footprint-packed_n7000000` | 1785896144 | `75710b6-dirty` |
| `bitpack-footprint-dense_n7000000`  | 1785896673 | `350953f-dirty` |

Nine minutes apart, same machine, same pin, **different commit**. And the dense family's own size sweep
changes commit mid-sweep: its four smaller sizes are at `75710b6-dirty` and only the 7M row is at
`350953f-dirty`.

**And the family that does have real arms in one file says the penalty is flat, not rising.**
`bitpack-sequential-sum` carries three arms, and `bitpack-shared`'s doc names what each is: `native` is the
dense reading at a native container (13 bits in a `u16`), `aligned` is one rounded-up-to-a-byte slot per
value, `zeropad` is packed to the bit with fields straddling byte boundaries.

| n | native | aligned | zeropad | aligned/native | zeropad/native |
|---|---|---|---|---|---|
| 256   | 21   | 86   | 116  | 4.10x | 5.52x |
| 4096  | 407  | 1395 | 1934 | 3.43x | 4.75x |
| 16384 | 1667 | 5570 | 7679 | 3.34x | 4.61x |

The penalty **falls** by about a fifth from 256 to 16384 in both packed readings. So the "turns with scale"
claim comes entirely from splicing that family's small-`n` numbers against the footprint family's large-`n`
numbers, and those are different transforms, different arms, and at 7M a different commit.

The honest statement of what the repository knows about `Cold`: **the walk penalty is measured, with real
competitor arms in one file, at 256 to 16384 elements. The comparison against a dense layout above the cache
is not measured by any single run.** So `19` is right that the panel was wrong to call it unpriced, and the
correction overshot: half of it is priced properly and the half that carries the scale claim is not.

This does not make the footprint benches bad work. Their own doc explains why they are separate families: a
packed column and a dense column of the same element count are different byte sizes, and the harness keys a
bench by one `n` with one shared input. Splitting was the right call for the instrument. The error is
downstream, in reading two files as one comparison.

### 2.6 One provenance fact that bears on all of the above

```
meta files: 147
runs taken from a dirty working tree: 146
```

**One of 147 committed runs was taken from a clean tree.** For the other 146 the recorded commit does not
identify the source that produced the number, so none of them can be re-run with any assurance of running
the same code. The machine and toolchain are uniform and correct across all 147 (`Apple M1`,
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, matching the workspace pin exactly), so the pin discipline
held and the commit discipline did not.

This does not make any number wrong. It makes cross-run comparisons rest on an assumption of source identity
that the metadata explicitly denies, which is precisely what 2.5 depends on.

## Three: what the repository knows that the panel does not

Nineteen files of design argument proceeded without any of this. These are the results that bear on open
questions, with the family and the number.

### 3.1 The largest measured effect in the directory is a law arvo holds and LLVM cannot

`warm-affine-density-w13`, W = 13, 8192 elements, operation count swept, all arms at the same width and the
same data:

| operations per element | headroom | minimum | native | minimum/native |
|---|---|---|---|---|
| 1  | 8173  | 7946  | 332 | 23.9x |
| 2  | 8460  | 8417  | 395 | 21.3x |
| 4  | 8514  | 8365  | 410 | 20.4x |
| 8  | 13702 | 13418 | 402 | 33.4x |
| 16 | 13509 | 13248 | 401 | 33.0x |

`minimum` and `native` use the **identical container**. The only difference is whether the projection is
written after every operation or once at the end. At 16 operations per element that is **33x**.

And the mechanism is not the cost of the removed instruction. The crate states it and the numbers bear it
out: a chain of affine steps composes to one multiply-add by ordinary algebra, LLVM performs that
composition only when it can see the chain, and a mask between the steps is not affine so the compiler has
no licence to move it. Reduction modulo `2^W` is a ring homomorphism, so **arvo has that licence and LLVM
does not**. Removing the interior projections does not save a mask, it hands the chain to the optimiser in
a form it can collapse.

Note the flat rows: `native` is 332, 395, 410, 402, 401 as the operation count goes from 1 to 16. **The
collapsed form costs the same at sixteen operations as at one**, which is the collapse visible directly in
the medians.

This is the workspace's own microkernelling thesis, measured, at a factor of 33, on the harness, committed,
and cited by nothing in this panel. A fixed-point filter, a colour transform and a scale-and-bias pass are
all this shape.

### 3.2 A typestate theorem priced, with the width band where it stops applying

`precise-widening-theorem-l1`. A `W`-bit column saturating-accumulated into a 64-bit accumulator. The
theorem: the exact sum is below `N * 2^W`, so when `W + ceil(log2 N) <= 64` the accumulation cannot reach
the saturation point, every clamp is dead, and the operation is a plain wrapping sum. Both facts are things
arvo holds statically and the machine code cannot carry: the element bound is the declared width, which
does not survive into a loaded value, and the count bound is the column capacity, which LLVM sees as a
runtime slice length.

| W | theorem arm | honest arms | ratio |
|---|---|---|---|
| 8  | 165  | ~5330 | **32.3x** |
| 13 | 679  | ~5290 | 7.8x |
| 16 | 701  | ~5250 | 7.5x |
| 32 | 674  | ~5330 | 7.9x |
| 60 | 5490 | ~5500 | 1.00x |
| 64 | 5228 | ~5300 | 1.01x |

At 8192 elements the predicate is `W + 13 <= 64`, that is `W <= 51`. **The measured effect is present at 8,
13, 16, 32 and absent at 60, 64, exactly where the predicate says.** The instrument agrees with its own
theory at the boundary, which is the check that makes the rest of the family trustworthy.

And what it buys is not one instruction. A saturating fold is a loop-carried dependence through an operation
LLVM will not reassociate, so it is serial at every width and in every container. Deleting the saturation
turns it into a plain wrapping reduction, which vectorises. **The theorem does not make the loop cheaper, it
changes which loop is compiled.** That is a compile-time table cell, decided by a `const fn` of two
quantities the typestate already carries.

### 3.3 Knowing a fold's arity at compile time, priced in isolation

`warm-clamp-arity-w13`, the `accfit` against `accfit-dyn` pair. Identical arm, identical accumulator rule,
the only difference is whether the fold arity is a const generic or a runtime value:

| fold arity | accfit (const) | accfit-dyn (runtime) | ratio |
|---|---|---|---|
| 2   | 258 | 4329 | **16.8x** |
| 4   | 273 | 1918 | 7.0x |
| 8   | 536 | 1160 | 2.2x |
| 16  | 286 | 755  | 2.6x |
| 64  | 208 | 279  | 1.3x |
| 256 | 190 | 195  | 1.03x |

One static lever, isolated, with everything else held, and its value **falls from 16.8x to nothing as the
arity grows**. That is the shape of an answer, not a number: static knowledge of the arity is worth a great
deal on short folds and nothing on long ones, because on a long fold the per-chunk overhead it removes is
already amortised.

I have not seen this cited anywhere in the panel and it is directly relevant to what the typestate is for.

### 3.4 The strategy axis has a ratified cell that no harness run touches

`SETTLED_container.md` item 14 records as RATIFIED that the wide payload is ragged for `Cold` and `Precise`
and word-rounded for `Hot` and `Warm`, "measured at one numeral: ragged is fourteen instructions and
twenty-five bytes, word-rounded is eleven and thirty-two". That is an instruction count at one numeral, and
by this workspace's own standard it is an ad-hoc quick spike, not a bench. **No committed harness run sweeps
the wide rung at all.** Every bench in `mock/benches/` tops out at W = 64, and the widest container any arm
instantiates is `u128`.

So a ratified rule about the wide rung rests on a count, and the rung it governs is unmeasured. See
section four.

### 3.5 What the packed layouts are actually priced at, and one arm that is dominated

Restating 2.5 as a positive claim, since the panel needs the numbers rather than only the correction. All
at W = 13, from `bitpack-sequential-sum` and `bitpack-random-sum`, real competitor arms in one file:

| access pattern | aligned/native | zeropad/native |
|---|---|---|
| sequential, 256 to 16384 | 3.34x to 4.10x | 4.61x to 5.52x |
| random, 256 to 16384     | 1.29x flat     | 2.16x to 2.19x |

Two things fall out that I have not seen stated.

**The random-access penalty is a third of the sequential one.** Packing costs most where the dense layout
was already streaming well, which is the opposite of the intuition that random access is where the extra
decode hurts. The dense arm loses its advantage when the walk stops being a stream, so the gap closes.

**At W = 13 the byte-aligned reading is strictly dominated.** A 13-bit value in a rounded-up-to-a-byte slot
occupies 2 bytes, and the dense native container for 13 bits is a `u16`, also 2 bytes. **The aligned arm has
the identical footprint to the dense arm at this width and is 3.34x to 4.10x slower sequential.** It buys
nothing at 13 bits. It would buy something at a width where the byte-rounded size falls below the rung size,
17 bits for instance, where three bytes beats a `u32`, and **no committed run sweeps such a width**. So the
one width where the aligned reading is measured is a width where it cannot win.

Only `zeropad` actually saves bytes at 13 bits, and it costs 4.61x sequential and 2.19x random.

## Four: what is not measured that should be

Four benches, each of which would settle something currently open, plus one guard.

**The wide rung.** Nothing above `W = 64` exists. The ragged-against-word-rounded rule is ratified on a
fourteen-versus-eleven instruction count at one numeral, and the whole question of what a numeral costs
above the native containers rests on it. The bench: the existing container sweep extended to
W in {65, 96, 128, 129, 192, 256}, with a ragged arm and a word-rounded arm, at both element counts. The
`Carrier` trait in `warm-container-shared` already abstracts the container; adding a byte-sequence carrier is
the whole of the work. This is the largest hole and it is the cheapest to fill.

**The signed axis.** Every arm in every bench is unsigned. `Bits<N, S, Sign>` carries a sign axis and no
committed run instantiates `Signed`. Sign turns the projection from a mask into a sign-extension, which is a
different instruction with different vectorisation behaviour, so the width-band result in 1.2 cannot be
assumed to transfer. The bench: the same sweep with a signed carrier.

**A width where the aligned packing can win.** Per 3.5, every packing bench runs at W = 13, where the
byte-aligned reading has the same footprint as dense and therefore cannot come out ahead on any axis. The
bench: the same access-pattern sweep at W in {17, 20, 33, 40}, where the byte-rounded size is strictly below
the container rung. Without it the aligned reading has been measured only where it is guaranteed to lose.

**Packed against dense in one file.** The instrument problem is real (different byte sizes for the same
element count) and it has an ordinary solution: key the bench by **byte budget** rather than element count,
so a packed arm and a dense arm at the same `n` hold the same number of bytes and different numbers of
elements, and report per-element cost. That makes the arms comparable in the one dimension the footprint
question is about. Until something of that shape runs, "the penalty turns with scale" is a hypothesis.

**A roofline guard in the harness itself.** Not a bench. The analysis already computes Gops/s and already
knows the host from `meta.json`. A single check that flags any variant whose reported throughput exceeds a
conservative per-host ceiling would have caught 2.1 the moment it was written, and would have cost one
comparison. This is the highest-value change in this file and it belongs upstream in
`bench-harness/src/analysis.rs` rather than in arvo.

**And one that is a test rather than a bench.** Every bench crate here validates its arms by asserting they
agree. In the six void cells that check passed while the arms computed an input-independent constant. The
generalisation of `warm-clamp-shared`'s fix is worth making a standing requirement: **a bench crate asserts
that its answer moves when its input moves, at every key it declares, before it asserts that its arms
agree.** `chunked_answer_depends_on_every_element_the_clamp_did_not_absorb` is the shape, and its own
version covers only the small-`n` keys (it filters on `key_nc(**k) == 0`), so even the fixed version leaves
the large-`n` rows unasserted.

## Five: what is op's, and what is not

**Op's, and it is one sentence.** Does `Warm` wrap or clamp? `warm-clamp-shared`'s own text says the
ratified preset table gives `Warm` the resolution clamp and gives wrapping to `Hot` alone;
`SETTLED_container.md:405-408` records op declaring that exact cell stale under his restated intent that
Warm behaves like a native Rust primitive. Two committed bench families implement the two readings and they
disagree in **direction**, not just magnitude, on the question his standing instruction was about. No amount
of expert convergence settles it, because it is a question about which of his own rulings is live.

**Not op's, and not to be brought to him.** Whether headroom is deleted. The benches exist, they say
different things under different semantics, and by his own standing position that means the thing is still
settling and there is nothing to rule on. The next move is the wide-rung arm and the signed arm, not an
escalation.

**Not op's, and mine to state plainly.** The six `precise-container-width-l1` findings files are void as
measurements. Anything anywhere that cites the `warm-container-kernel` arm under saturating semantics is
citing a register copy. I did not find such a citation in the panel, because no panel file cites this
directory at all, which is the only reason the damage is contained.

## Coverage, and what I could not close

**Established with committed harness data:** the width-banded shape of the headroom cost and its mechanism;
the 21.6x projection effect at sub-rung widths; the affine collapse at 33x; the widening theorem at 7.5x to
32x with its predicate boundary confirmed; the const-arity lever from 16.8x to 1.03x; the clamp-family arity
crossover at the row its own predicate names; the packed-layout walk penalties with real arms.

**Established with ad-hoc quick spikes, which is all they can support:** that the six saturating cells
return in nanoseconds with the correct value (`p3`, an existence claim); that the mechanism is an absorbing
fixpoint provable by interval arithmetic (`p4`, a refutation of the alternative readings, with the
input-insensitivity shown by two columns rather than argued). Neither prices anything and neither is a
bench.

**Established with counts, each reproducible from one command:** 147 findings files; 146 of 147 runs from a
dirty tree; 0 of 55280 rows with a digest or a score; 12 of 691 cells under 100 ns, of which 6 impossible;
6 findings files above the roofline.

**Attacked and not closed.** The `nomem` defect in the counter: I reproduced instability that depends on the
asm option and could not attribute a magnitude to it, because isolating it properly means rebuilding a
shared dependency and re-running a family on the harness, which changes a crate every consumer repo uses. It
is characterised, it is small next to 2.1, and it is left open deliberately.

**Not covered.** Roughly 30 findings files in the quantiser, spectral, structural and hash families, read
only through the impossible-throughput sweep. No re-run of any bench. No reading of `141`, `142` or the
predecessor panel's files beyond the two paragraphs `SETTLED_container.md` quotes and the bench crate doc
comments I cite. Section 1.5 is flagged contaminated by a commit subject line and is owed an independent
read.

## Probes

`20_probes/`, all committed beside this file, all on `nightly-2026-05-28`,
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, with their raw output committed alongside.

`p1_counter_nomem_is_not_a_barrier.rs` is kept although it is **flawed**: its four brackets store into one
output, so three of the four stores are dead and its zero readings prove nothing about the counter. It is
here because the flaw is the reason `p2` exists and a later reader should see the correction rather than
only its result.

`p2_counter_nomem_isolated.rs` is the repaired version, one arm per loop with a live accumulator.

`p3_dylib_probe/` calls the shipped cdylibs through `dlopen` the way the harness does.

`p4_absorbing_fixpoint.rs` is the mechanism, with the three predictions stated before the run.
