# 101. The cost coordinates

**Predecessors:** `93` and `94`, the unit's cold pair; `97`, which attacked both; `98`, which second-read
`97` and proposed inverting it; `100`, which attacked that and found its own boundary. **Probes:**
`101_probes/`, committed as each ran.

This is the sixth file of the unit, in the half `95` points at convergence. The unit has settled on an
object: a strategy is a weighting over cost coordinates plus a cost table. My question is the object's
inside. What are the coordinates, how are they estimated, how are they normalised, and what does the
corpus fail to measure.

Three things up front, because they are the file in miniature.

**The corpus measures one coordinate.** Of the harness CSV's seventeen columns, nine carry information
across all 254 committed files and 104080 rows, and exactly three of those nine vary between arms at a
fixed region: `e2e_ns`, `algo_ns` and `bridge_ns`, which are one timing and its decomposition. Eight
columns are identically empty or zero everywhere, including every column that could carry an accuracy,
a fidelity, a setup cost or an instruction count. Six of those eight are **reachable**: the harness
fills them, and no arvo variant asks it to. Section 2.

**A coordinate set is a ceiling on how many strategies can exist, and it is countable exactly.** With one
coordinate the weighting cancels and every strategy agrees, by algebra rather than by luck: exactly one
section is reachable. With two it is nine on the committed carrier table, which is `97`'s own published
number recovered as a fact about the coordinate set rather than about rationalisability. With three it is
forty-two. **So a strategy whose intent names a quantity with no coordinate is not unmeasured, it is
inexpressible**, and on the corpus as it stands accuracy is such a quantity. Section 4.

**Normalisation is not a decision about the costs. It is a change of basis on the weighting.** A fixed
per-coordinate affine map is a bijection on weightings that preserves every section, so declared-range
normalisation and raw coordinates are one model in two coordinate systems: 2000 of 2000 identical sections
on 4 of 4 families. Only a data-dependent map is a second model, which is exactly `100`'s independence
failure, reproduced here on four families rather than one. The corollary bites something in `100`: its
tolerance band is stated as a fraction of the achievable range, which is data-dependent in the same way,
and an arm no weighting can select widens a 1% band by 59 to 185 times. Section 3.

## 0. The gates

**Canon gate: passed.** Checked against `INTENTS.md` I1 through I17.

The assigned question is licensed twice over. I8 says all strategies "should be decided by measurement,
just measuring different things, and ... they weigh different measurements differently"
(`INTENTS.md:137-139`). The coordinates are what "different measurements" names, so asking what they are
is asking what op's stated mechanism operates on. I13 requires every finding to carry the region it holds
in, and a region is stated over dimensions, so a census of the dimensions the corpus can even express is
prior to any predicate written in this panel.

One thing I checked because it would have been a refusal. **I8's second half is not an open question and
I do not treat it as one.** The entry records op's correction: the clause about the weightings probably
agreeing is "just filler noise I mused on the spot", and `INTENTS.md:155-156` says plainly that whether
they agree in practice "is an ordinary empirical question about arvo, answerable by measurement like any
other, and it is not an intent op owes". Section 4 measures a quantity adjacent to that question. It
reports it as an ordinary measurement and puts nothing to op.

**A second check, on whether my question is the shape op has rejected.** "Which estimator should a
coordinate use" is a which-single-policy fork if answered with a winner.
`never-ask-which-single-rule-governs.md` names it and `100` section 0 records that Q43 was written that
way. Section 5 answers it with a criterion and two measured admissibility tests instead, and the answer it
reaches is explicitly per family and per coordinate. Where the two tests disagree, which they do, I say so
rather than ranking them.

**Test gate: passed, and I ran the crates my work rests on rather than citing a count.**

There is no arvo suite; `mock/crates/` is empty by design. I gated the bench variant crates my arguments
read from:

```
bitpack-carrier-shared    9 passed      bitpack-contend-shared   12 passed
bitpack-wide-shared       6 passed      warm-clamp-shared         7 passed
bitpack-footprint-shared  6 passed      warm-container-shared    15 passed
```

All green, and each matches `98`'s per-crate figure, which `100` verified as a fourth party. I did not
recount the whole corpus; `100` did that from source and I take its 123 without adding a fifth opinion,
which would be an echo rather than an instance.

**The one gate finding is mine and it is about an instrument the unit rests on.** Four committed families
carry a byte-identical noise-floor control arm, and `100`'s central result and my section 5 both use that
pair as a calibration. The byte-identity is asserted in three module headers and **checked in one**:
`bitpack-carrier-d16-control/src/lib.rs:1-8` names a script that diffs the two built dylibs, while
`bitpack-contend-d16-control/src/lib.rs:1-5` and `bitpack-wide-d16-control/src/lib.rs:1-6` assert it and
name nothing. `101_probes/p0_control_identity_on_every_pair.sh` builds all six crates and checks all three:

```
bitpack-carrier-d16 vs bitpack-carrier-d16-control  IDENTICAL   (50497 instruction lines each)
bitpack-contend-d16 vs bitpack-contend-d16-control  IDENTICAL   (55678 instruction lines each)
bitpack-wide-d16    vs bitpack-wide-d16-control     DIFFER      (55513 vs 55513 lines, 6 differing)
```

**The wide pair is not byte-identical.** Three sites load a vector register from the constant pool at
offset zero in one dylib and at a non-zero offset in the other, same opcode, same registers, same
instruction count. That is a literal-pool shift, and the variant name string differing in length is the
obvious cause. It is almost certainly benign and it is not what the header says, and nobody had looked.

The first run of that script reported all three pairs differing, on 16, 8 and 30 lines that were every one
an `adrp` page number off by one. `otool` prints a page-relative operand as a bare decimal, so the address
normalisation misses it. That output is kept at `101_probes/p0_before_the_adrp_normalisation.out` rather
than deleted, because the differences it shows are the evidence that nothing else differs.

## 1. The answer, before the working

**One. The realised coordinate set is one timing.** 254 committed CSVs, 104080 rows, 17 columns. Nine
columns carry information and three of those vary between arms at a fixed region: `e2e_ns`, `algo_ns`,
`bridge_ns`. Eight are identically empty or zero: `cooldown_ms`, `score`, `input_tag`, `instructions`,
`cycles`, `setup_ns`, `first_ns`, `digest`. Section 2.

**Two. Six of the eight dead columns are reachable and the distance to each is short.** `setup_ns`,
`first_ns` and `digest` are filled by a constructor no arvo variant uses: all 82 measured call sites use
`timed!`, and zero use anything else. `score` is filled by `Routine::score_output` and
`Routine::score_dimensions`, implemented by 0 of 94 variant crates. `instructions` and `cycles` need a
flag and root on this host. Section 2.2.

**Three. The storage coordinate, the one arvo exists for, is declared rather than measured.** Every probe
in this unit that weighs storage types the bytes per arm into its own source, and no bench in the corpus
measures a size. Section 2.3.

**Four. Frozen-range normalisation and raw coordinates are the same model.** A fixed per-coordinate affine
map with positive scale is a bijection on weightings preserving every section: 2000 of 2000 identical, on
4 of 4 families. The weights therefore carry the units, and a weighting is a rate of exchange rather than
a set of shares. Section 3.

**Five. `100`'s independence failure reproduces on four families, and its own tolerance inherits it.**
Dropping an arm dominated everywhere moves the section for 961 of 2000 weightings under min-max on the
carrier family and 252 of 2000 on contention, while raw and frozen move 0 of 2000 in 4 of 4 families on
both perturbations. And a band stated as a percentage of the achievable range grows 59 to 185 times when
an arm no weighting can select is added. Section 3.2.

**Six. The number of strategies a coordinate set can distinguish is exact and small.** On the committed
carrier table with the control dropped: `{time}` reaches 1 section, `{time, size}` reaches 9, `{time,
spread}` reaches 6, `{time, size, spread}` reaches 42. The instrument reproduces `97`'s published 72 and 9
before anything rests on it. Section 4.

**Seven. An estimator choice is a coordinate choice, and `100`'s replacement of the interquartile range by
the 95th percentile changes which coordinate is being weighed.** Section 5.

**Eight. Two of op's four stated strategy intents name a quantity the table shape cannot hold, and one of
them cannot be a per-arm scalar at all.** Precise's intent is accuracy "especially within chains and ops,
not only alone" (`INTENTS.md:126-127`), and a chain's error is not a function of its arms' per-operation
errors: the per-operation ranking and the chain ranking cross at a computable length. The constructive
answer is that chain length is a region dimension rather than a coordinate, which the corpus already
half-does. Section 6.

## 2. What the corpus actually measures

`101_probes/p1_the_coordinate_census.py` reads every committed CSV and every variant source.

### 2.1 The schema against the corpus

```
committed CSV files: 254   data rows: 104080

columns carrying information :  9  ['run', 'pass', 'mode', 'variant', 'batch_idx',
                                    'e2e_ns', 'algo_ns', 'bridge_ns', 'batch_count']
columns identically empty/zero:  8  ['cooldown_ms', 'score', 'input_tag', 'instructions',
                                     'cycles', 'setup_ns', 'first_ns', 'digest']

of the columns carrying information, which VARY BETWEEN ARMS at a fixed region
    algo_ns        varies                 (248 groups tested)
    bridge_ns      varies                 (248 groups tested)
    e2e_ns         varies                 (248 groups tested)
    batch_count    constant across arms   (248 groups tested)
    batch_idx      constant across arms   (248 groups tested)
    pass           constant across arms   (248 groups tested)
    run            constant across arms   (248 groups tested)
```

A column can be a cost coordinate only if it can separate two arms measured at the same region. `run` is 1
in every row, `batch_count` is 100 in every row, and `pass` and `batch_idx` index the sample rather than
measure it. So the corpus's coordinate surface is **one timing and its decomposition into the measured
call and the bridge**, plus the `mode` split into cold and warm, which is a region dimension rather than a
coordinate.

That is worth stating plainly because the unit's three-coordinate model reads as three measurements and is
not: `100` p1's coordinates are the median of `algo_ns`, a declared constant, and the interquartile range
of the same `algo_ns` samples the median is taken over
(`100_probes/p1_what_the_instability_is_made_of.py:118-128`). Two of the three come from one column of
eighty numbers per arm per region.

### 2.2 Six dead columns are reachable, and the distance is short

The eight dead columns are not alike, and treating them as one absence is what makes the gap look larger
than it is.

**`setup_ns`, `first_ns` and `digest`** are structural. The harness's own documentation says they are
"populated by the matrix scaffold ... the plain `timed!` / `timed_calibrated!` constructors measure only
`run_ticks` and leave them zero", and that "a zero in any of the three latter fields means not measured by
this constructor, never a measured zero" (`bench-core/src/lib.rs:421-438` in the pinned checkout). The
census counts **82 uses of `timed!` across 82 files and zero uses of anything else**. So those three
columns are zero because every arvo variant chose the constructor that leaves them zero.

Each of the three is a coordinate somebody in this unit has wanted. `setup_ticks` is "the one-time build
cost S", and the harness states the breakeven it enables directly: `k* = (S_b - S_a) / (I_a - I_b)`
(`bench-core/src/lib.rs:427-429`). That is a per-region amortisation threshold computed from measurements
rather than declared, and it is exactly the shape a region predicate wants. `first_ticks` is the cold
first-touch pass, which is a coordinate for a strategy that cares about a path taken once. `digest` is "a
reps-invariant fidelity witness" (`bench-core/src/lib.rs:432-435`), and fidelity is the word for the
coordinate two of op's intents need.

**`score`** is a value the routine computes about its own output: `Routine::score_output` returns an
`Option<f64>` where "lower = better" and `Routine::score_dimensions` returns a vector of labelled
dimensions for "Pareto analysis" (`bench-core/src/lib.rs:99-110`, `bench-core/src/lib.rs:182-193`). The
census counts **0 of 94 variant crates implementing either**, against 15 implementing `validate_output`
and 1 mentioning `outputs_may_differ`.

So the finding `98` reports and `99` carries forward, that no family carries a column for accuracy or for
divergence from a reference, is right and is narrower than it sounds. **The harness has a multi-dimensional
quality-coordinate surface and arvo uses none of it.** What is missing is not the column and not the
plumbing; it is a routine that computes a number about its own output, and thirteen shared crates that
each define a validator asserting the arms agree.

**`instructions` and `cycles`** need a flag rather than a code change: they are gated behind the
`perf-counters` feature and root on an Apple-Silicon host, and return zeros otherwise
(`bench-harness/src/perf.rs:9-14`). Every committed run was taken on `Apple M1`, per the meta files, so
this coordinate is one `sudo` away on the hardware the corpus already used. It is the one coordinate that
is machine-independent in the way a design would want: an instruction count does not move with the
machine's load, and `100` section 7.1 measures that the wall-clock instrument's own floor is a median of
0.273%.

**`cooldown_ms` and `input_tag`** are provenance rather than measurement and I claim nothing about them.

### 2.3 The storage coordinate is declared, and it is the one arvo exists for

I17 says the storage-minimising path is not deprioritised and that arguments for downgrading it are not
entertained. `arvo-toolbox-not-policer.md` puts it harder: bitpacked storage "is the reason arvo exists".

**No bench in the corpus measures a size.** The `bitpack-footprint-*` families sound like they do and do
not: their `bench.toml` titles are "Layout::Dense footprint: sequential sum swept past L1 and L2" and
"Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2" (`mock/benches/bench.toml:312`,
`mock/benches/bench.toml:359`), and they measure a timing while the working set sweeps past the cache
levels. That is an excellent way to observe the *consequence* of a footprint and it is not a measurement
of one.

Every probe in this unit that weighs storage declares it. `100_probes/p1_what_the_instability_is_made_of.py:78-87`
says so in its own comment, "a declared property of the arm, not a measurement", and `97`'s model carries
bits per element as 16, 32, 64, 13, 13 (`98_probes/p6_model97.json`).

**Two consequences, and the second is the one I did not expect.**

A declared coordinate is not a measurement, so under I8 a strategy weighing it is not, on that axis,
"decided by measurement". That is not a defect to fix by measuring: bytes per element of a declared layout
is a static fact, exactly known, and measuring it would be worse than declaring it. The honest statement is
that the coordinate set is **mixed**: some coordinates are measured with an uncertainty and a resolution,
and some are declared and exact. Section 5's admissibility criterion is about the measured ones only, and a
design that applies a tolerance band uniformly across coordinates is applying an uncertainty argument to a
number that has none.

And a declared coordinate has no noise, which means it cannot be the reason a section is unstable. `100`'s
instability decomposition is entirely about the two measured coordinates, and the storage coordinate is
inert in it. That is a small strengthening of `100` section 4 rather than a correction.
