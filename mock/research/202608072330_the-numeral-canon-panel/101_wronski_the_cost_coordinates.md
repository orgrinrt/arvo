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

## 3. Normalisation is a change of basis on the weighting

`100` section 8 is the sharpest thing the unit has found by accident, and its remedy is stated as a cost:
a design shipping normalised costs "has to state the normalisation range as **declared constants**, because
it is part of the semantics rather than a presentation detail. What that costs is a number somebody has to
get right once" (`100:895-897`).

**The algebra says there is no cost, because there is no second thing to state.** A fixed per-coordinate
affine map is a reparameterisation of the weighting and nothing else.

### 3.1 The theorem, and the test that it holds on real tables

Let the objective be `sum_i w_i c_i` and map each coordinate by a fixed affine transform
`c_i -> (c_i - b_i) / a_i` with `a_i > 0`. Then

```
sum_i w'_i (c_i - b_i)/a_i  =  sum_i (w'_i / a_i) c_i  -  sum_i w'_i b_i / a_i
```

and the second term does not depend on the arm. So the argmin under `w'` on the transformed coordinates is
the argmin under `w_i = w'_i / a_i` on the raw ones. The map is a bijection on weightings that preserves
every section either can produce: **normalising with declared constants and not normalising at all are one
model written in two coordinate systems**, and neither can express a section the other cannot.

`101_probes/p2_normalisation_is_a_change_of_basis.py` block A tests it on every committed family with a
control arm, over 2000 random weightings each, with the declared range deliberately not equal to the data's
own extremes so the test is of a fixed transform rather than of min-max in disguise:

```
  bitpack-carrier-width    regions=  6 arms=6  identical sections: 2000/2000
  bitpack-contend-decode   regions=  6 arms=5  identical sections: 2000/2000
  bitpack-contention       regions= 12 arms=6  identical sections: 2000/2000
  bitpack-wide             regions=  6 arms=4  identical sections: 2000/2000
```

**Three consequences, and the first is the one a canon can carry.**

**The weights carry the units.** A weight on a time coordinate is per nanosecond and a weight on a size
coordinate is per byte. "Half speed, half size" is not a statement until the exchange rate is named, because
the exchange rate is the weight. A normalised weighting that looks unit-free has hidden its units in the
range constants, which is why those constants have to be declared: not because normalisation is a semantic
choice, but because **a weighting is meaningless without its units and normalisation is where they went**.

**A weighting is a ray, not a point.** Scaling every weight leaves every argmin unchanged, so with `d`
coordinates the weighting space is `(d-1)`-dimensional. At two coordinates it is a single number: how many
nanoseconds one byte is worth. Section 4.3 uses that.

**The normalisation question is therefore not a design fork.** There is one model, plus one thing not to do.

### 3.2 The one thing not to do, on four families rather than one

Block B runs `100`'s two perturbations under each transform. Dropping an arm dominated at every region, and
adding an arm strictly worse than every real arm on every coordinate, are both things no weighting can
rationally respond to.

```
  bitpack-carrier-width    raw                      A moved     0/2000   B moved     0/2000
  bitpack-carrier-width    min-max (arm-set range)  A moved   961/2000   B moved     1/2000
  bitpack-carrier-width    frozen (declared range)  A moved     0/2000   B moved     0/2000

  bitpack-contend-decode   min-max (arm-set range)  A moved     0/2000   B moved     2/2000
  bitpack-contention       min-max (arm-set range)  A moved   252/2000   B moved     0/2000
  bitpack-wide             min-max (arm-set range)  A moved     0/2000   B moved   176/2000
```

Raw and frozen move nothing, 0 of 2000, on both perturbations, in 4 of 4 families. Min-max moves something
in 4 of 4. The added arm was never selected at any weighting in any run, which the probe asserts rather than
assumes.

**This is `100`'s F-100-6 corroborated by a second instrument on three further families**, and it is
independent in the way that matters: `100`'s probe uses three coordinates including a declared size and
sweeps an extremity factor, mine uses two measured coordinates and a fixed factor. The mechanism survives
both.

The carrier row is worth reading twice. **Dropping an arm that no weighting can select changes the answer
for 961 of 2000 weightings.** That is not a corner case; under min-max it is close to a coin flip.

### 3.3 The corollary, and it bites `100`'s own mechanism

`100` section 6 proposes a compile-time differential and section 7 measures that it wants a tolerance band,
stated throughout as a percentage "of the region's achievable objective range" (`100:836`). That denominator
is `worst - best` over the arm set at the region, which is data-dependent in exactly the way section 3.2 is
about.

Block C measures it. Adding the same unselectable arm, under a speed-only weighting so the objective is a
time in nanoseconds:

```
  bitpack-carrier-width
        region   range without    range with   band 1% grows by
         16384           734.3       66336.5              90.3x
       1048576         79328.8     5335695.0              67.3x
       8388608        519025.8     39341739.4             75.8x

  bitpack-contend-decode
      41943044         70651.8     7820515.4             110.7x
      83886084         74120.0    13710231.1             185.0x
```

**A 1% band becomes a 59% to 185% band**, so a differential that refused a defect before accepts it after,
and the arm that caused it is one nobody can ever select. `100`'s frozen-range remedy does not reach this:
it fixes the coordinate transform and the band's denominator is a separate quantity computed after it.

**What replaces it, and `100` already measured the replacement.** Section 7.1 reads the control pair's
apparent gap as a percentage of runtime, per region: a median of 0.273% and a maximum of 0.544%. That is a
band in the coordinate's own units relative to the region's own value, with no arm set in the denominator,
and it is the natural floor because it is the instrument's own resolution. So the band should be stated
**per coordinate, per region, relative to the coordinate's own magnitude**, calibrated by the control pair.
`100` built the calibration and then expressed the band in a different currency.

That also disposes of `100`'s shape 8, "a per-coordinate tolerance band rather than one global band", which
it lists as live and untested (`100:1072-1074`). It is not a refinement; a single band across coordinates in
different units is not expressible without a normalisation, and the normalisation is a weighting, so a
global band is a band whose width depends on the weighting it was supposed to be independent of.

## 4. What a coordinate set can express

### 4.1 The instrument, calibrated before anything rests on it

`101_probes/p4_what_a_coordinate_buys.py` counts, exactly, how many sections a weighting can reach. A
section is rationalisable when some `w` makes the named arm an argmin at every region, which is a
homogeneous linear feasibility problem; normalised to the simplex it is an interval at two coordinates and a
polygon at three, clipped in exact `Fraction` arithmetic. I wrote it from the geometry rather than reading
`97_probes/p9_the_decider.py` or `98_probes/cone.py`, so that agreement would be evidence.

```
  6 regions, 5 arms, 2 coordinates (median algo_ns, bits per element)
  rationalisable, w >= 0    this probe:       72    97 and 98 report      72
  rationalisable, w  > 0    this probe:        9    97 and 98 report       9
```

**Three exact implementations, written from three different geometries, agreeing on both counts.** `97`
built the decider, `98` reimplemented it and reproduced the counts, and this is the third. `RULES.md` puts
the bar at three independent instances and this clears it for those two numbers specifically.

**The first version of my instrument reported 9 for both**, because it welded two independent knobs
together: whether a weight may be zero, and whether the named arm must be the unique argmin or merely one of
the minima. `97`'s 72 is the first knob open and the second closed. That output is kept at
`101_probes/p4_first_version_conflated_two_knobs.out`, because a calibration that fails first and passes
after a stated fix is worth more than one that passes immediately.

### 4.2 The ceiling, and it is what a coordinate is for

The same instrument, over coordinate sets. Strict positivity, so no strategy is ignoring a coordinate
outright:

```
  coordinates                       arms  sections   w >= 0   w > 0
  {time}                               5     15625        1       1
  {time, size}                         5     15625       72       9
  {time, spread}                       5     15625        6       6
  {time, size, spread}                 5     15625      101      42
  {time}                               6     46656        1       1
  {time, size}                         6     46656       72       9
  {time, spread}                       6     46656       10      10
  {time, size, spread}                 6     46656      117      58
```

**With one coordinate the weighting cancels and exactly one section is reachable.** Every strategy agrees,
by algebra, not by luck and not by the arms happening to be similar. That is the degenerate case and it is
worth naming because it is the case a design falls into without noticing: a coordinate set of size one is a
design with one strategy wearing several names.

Each coordinate multiplies what the space can express: 1, then 9, then 42. **So the number of
distinguishable strategies is a property of the coordinate set, and the corpus's coordinate set is what caps
it.** Not the vocabulary, not the number of markers, not how carefully the weights are chosen.

The sharp form, and it is the answer to the part of my question about what the corpus fails to measure:

> **A strategy whose intent names a quantity with no coordinate is not unmeasured. It is inexpressible.**
> There is no axis along which it can differ from any other strategy, so it and its opposite are the same
> point in the space, whatever the canon calls them.

On the corpus as it stands, with time the only measured coordinate and size declared, a strategy weighing
accuracy is exactly that: a name for a point that coincides with every other point.

### 4.3 The cells, and the margin a weighting has

At two coordinates the weighting is one number, so the section is a piecewise-constant function of it and
the pieces are intervals. `101_probes/p3_the_exchange_rate_and_its_cells.py` computes them exactly from the
pairwise ties, `r* = -(x_a - x_b)/(y_a - y_b)`, which is every boundary there is.

```
  bitpack-carrier-width   coordinate 2 = declared bytes/elem   arms=6 regions=6
            r from           r to  decades  section
                 0           9.55      inf  d32,d32,d16,d16,d16,control
              9.55         50.425     0.72  control,d32,d16,d16,d16,control
            50.425        1507.33     1.48  control,control,d16,d16,d16,control
           1507.33        12157.7     0.91  simd,control,d16,d16,d16,control
           12157.7        94932.3     0.89  simd,simd,d16,d16,d16,control
           94932.3         183061     0.29  simd,simd,simd,d16,d16,control
            183061         376255     0.31  simd,simd,simd,simd,d16,control
            376255         766780     0.31  simd,simd,simd,simd,simd,control
            766780            inf      inf  simd,simd,simd,simd,simd,simd
```

Nine cells, which is the same 9 the exact enumeration reports, by a different route: an interval count
against a feasibility count over 15625 candidates. Eight to twelve cells on the other families.

**Two things follow, and both are design objects the unit does not currently have.**

**Two weightings in the same cell are the same strategy.** So a design does not need its weights to be
right, only to land in the right cell, and the cells here are 0.29 to 1.48 decades wide in the interior.
That is a robustness statement about the whole weighting question: the number nobody wants to defend has to
be correct to within a factor of a few.

**A strategy has a margin**, the distance from its declared exchange rate to the nearest cell boundary. It
is computable at const time from the same table the section came from, it is in the exchange rate's own
units, and it is the honest thing to report next to a strategy: not "this weighting is right" but "this
weighting is 0.4 decades from changing its mind".

And this is `97`'s cone in coordinates. Cone membership of a stated weighting and "the stated rate lies in
the cell whose section is the shipped table" are the same test. What the cell adds is a width, which the
cone did not report.

**The control arm is in the section at almost every exchange rate**, which is `100`'s F-100-3 arriving from
a completely different instrument. `100` found it by bootstrap resampling; this finds it by exact interval
arithmetic on the point estimates, with no resampling anywhere. The byte-identical twin is not merely
winning under noise: at the committed point estimates it is the argmin at some region in eight of the nine
cells. That is a second independent instance of the mechanism, and it strengthens `100`'s reading rather
than qualifying it.

## 5. Estimation, where an estimator choice turns out to be a coordinate choice

`100` section 7.3 swaps the interquartile range for the 95th percentile as the estimator of its third
coordinate and reports it strictly better on both axes it measures: 3 distinct sections against 161 under
resampling, 54 of 60 arm pairs separated against 43 of 60 (`100:774-783`). It concludes that three separate
negative findings about that coordinate "was a fact about the interquartile range and not about tail
behaviour" (`100:811`).

**The measurement is right and the conclusion needs one more test, which reverses part of it.**

The 95th percentile of a batch of timings is a **level**: it contains the median and adds a tail on top. The
interquartile range is a **spread**: the median cancels out of it by construction. An estimator that is
stable and separating because it is largely re-measuring coordinate one looks exactly like one that is
stable and separating because it resolves coordinate three, on every statistic `100` reports.

`101_probes/p5_an_estimator_is_a_coordinate_choice.py` runs three tests over nine candidates on all four
control-bearing families.

### 5.1 Every estimator has a resolution and the corpus measures it

Two byte-identical arms differ only by measurement error, so the resampled distribution of `E(A) - E(B)` is
a pure noise sample for estimator `E`. That is a floor in the estimator's own units, per family, from the
corpus itself, and it generalises `100` section 7.1 from one statistic to any.

```
  bitpack-carrier-width         floor %   signal %  signal/floor
    median                         1.18      78.15          66.4
    iqr  (spread)                 47.95    3552.33          74.1
    p95  (level)                   5.65      99.57          17.6
    p95-med (excess)             101.39    1331.80          13.1

  bitpack-contend-decode        floor %   signal %  signal/floor
    median                        18.36      53.44           2.9
    iqr  (spread)                132.19      93.85           0.7
    p95  (level)                 143.19      54.75           0.4
    p95-med (excess)             399.96      94.78           0.2
```

**The instrument's resolution differs by more than an order of magnitude between families.** On the carrier
family the median resolves 66 times its own floor; on `bitpack-contend-decode` it resolves 2.9 times, and
every spread and tail candidate there sits at or below 1, meaning the corpus cannot separate those arms on
that coordinate at all. That is the per-family predicate `100` section 4.4 reaches for, measured per
estimator rather than per family.

**A bound on this statistic, stated because it matters.** The signal is a dynamic range, `(max - min)/min`
across arms, so one extreme arm can carry it. I do not rest the claim that the interquartile range is usable
on this row alone; the claim I do rest is 5.3's, which is exact.

### 5.2 The 95th percentile is the median again

```
    estimator            carrier   decode  contention     wide
    p95  (level)          0.9978   0.9780      0.9866   0.9892
    p99  (level)          0.9940   0.7354      0.9458   0.9091
    iqr  (spread)         0.5184   0.2756      0.5199   0.3682
    p95-med (excess)      0.7317   0.2911      0.4200   0.6762
```

Pearson correlation against the median across every (arm, region) cell. The 95th percentile correlates at
0.978 to 0.998 in 4 of 4 families. The interquartile range correlates at 0.28 to 0.52.

### 5.3 What each candidate adds, which is the test that decides it

The exact reachable-section count on `{median, candidate}`, by section 4.1's calibrated instrument. One
means the candidate changes nothing: no two strategies can disagree about it.

```
    estimator            carrier   decode     wide
    median                     1        1        1
    iqr  (spread)              6        7        8
    mad  (spread)              8        7        8
    p95  (level)               1        7        3
    p99  (level)               1        5        2
    p95-med (excess)           3        9        5
    p99-med (excess)           2        8        5
```

**On the carrier family, `{median, p95}` reaches exactly one section: the same as median alone.** The
estimator `100` recommends, on the family `100` measured, makes the third coordinate carry no design content
whatever. It is more stable because it stopped being an axis.

The spread estimators reach 6 to 8 sections on every family. The excess forms, a level minus the median,
reach 2 to 9 and beat the level they are built from in 3 of 3 enumerable families.

**So `100`'s F-100-7 is right about resolution and wrong about what to do.** The interquartile range is a
noisy estimator of a real axis; the 95th percentile is a quiet estimator of an axis that is already there.
Trading one for the other buys stability by deleting the coordinate.

**What replaces it, and it is a composition rather than a winner.** If a strategy weighs tail behaviour, the
coordinate is a tail **excess** rather than a tail level, because a level is coordinate one plus a tail and
the design cannot separate them. Two admissibility tests, both measured from the corpus, and they disagree:

- **Resolution**, section 5.1: does the estimator's signal clear its own floor against the control pair?
  On carrier the interquartile range clears it best; the excess forms are noisier.
- **Independence**, section 5.3: does the estimator add reachable sections? There the excess forms beat the
  levels everywhere and the levels collapse to nothing on carrier.

Neither test alone is the answer and I decline to rank them. **The answer is per family and per coordinate,
and both numbers come from the corpus, so a design can gate on them rather than argue about them.** That is
what makes this a predicate rather than a preference.

## 6. What the corpus fails to measure, against what op actually said

Op's stated intents name four quantities. The corpus measures one of them.

**I5, the speed-first intent.** "The intent behind Hot is performance, efficiency, even at the cost of
accuracy or soundness", and it "should not lose it for nothing, instead, provable meaningful gains"
(`INTENTS.md:100-103`). The gain is measured: it is `algo_ns`. **The price is not.** "Should not lose it for
nothing" is a constraint relating two quantities, and the corpus carries one of them, so the constraint
cannot be checked. That is a stronger statement than "two intents have no coordinate": the speed intent,
which looks fully served, contains an accuracy term in its own bound.

**I6 and I17, the storage-first intent.** "It should remain small for memory or disk storage"
(`INTENTS.md:112-113`), and I17 says this path is not deprioritised. The coordinate is declared per arm and
no bench measures it. Section 2.3.

**I7, the accuracy-first intent.** "The most precise possible answer ... especially within chains and ops,
not only alone" (`INTENTS.md:125-127`). No coordinate, and the shape is not the table's. Section 6.1.

**I3 and I4, the imitate-the-native-primitive intent.** "It should behave like native primitives in regular
old rust would" (`INTENTS.md:81-83`), and I4 says imitation serves intuitiveness rather than defining it,
"if mimicking is consistently just worse choice" (`INTENTS.md:92-94`). Both readings need a **divergence**
coordinate: how far this arm's behaviour sits from what a native primitive would have done. `93`'s F8 is a
measurement in that coordinate, taken by hand at fourteen widths, and no bench carries it.

So the map is: one intent's quantity is measured, one is declared, and two have nothing. And the fourth is
the one that decides whether the first intent's own bound is satisfiable.

### 6.1 Accuracy cannot be a per-arm scalar, and the rankings cross

The cost table holds a vector per (region, arm) cell. Time is such a scalar and so is size. **Accuracy over
a chain is not**, and the failure is not approximation, it is inversion.

`101_probes/p6_accuracy_is_not_a_per_arm_scalar.py` takes two ordinary fixed-point arms. One uses a finer
intermediate grid with truncation, so its per-step error is smaller and biased. The other uses the declared
grid with round-to-nearest, so its per-step error is larger and unbiased. The reference is exact rational
arithmetic, so the error reported is the real one.

```
     chain k     fine+trunc     coarse+rne   winner
           1         0.1188         0.2498   fine+trunc
           2         0.2378         0.3362   fine+trunc
           3         0.3533         0.4100   fine+trunc
           4         0.4711         0.4675   coarse+rne
           8         0.9450         0.6629   coarse+rne
          64         7.5643         1.8805   coarse+rne

  CROSSING at chain length k = 4
```

At one operation the first arm is twice as accurate. At four it is behind, and at sixty-four it is four
times worse. The bias accumulates linearly and the unbiased error accumulates as a random walk, and the
crossing is where they meet. The control pair, the same grid with truncation against round-to-nearest, never
crosses: the rounding arm leads from k = 1 onward, which is what makes the crossing a property of the pair
rather than of the probe.

**So a cell holding one accuracy number per arm names an arm that is wrong for every chain longer than the
crossing, and nothing in the cell says so.** Op's intent is explicit that chains are the case that matters,
so this is not an edge.

**The constructive answer, and it keeps the table's shape.** Chain length is a **region** dimension, not a
coordinate. Indexed that way every cell is a scalar again, the accuracy-weighing strategy selects a
different arm at a different chain length, and that is what I7 says it should do.

**And the corpus already half-does this.** `warm-clamp-shared` encodes its region key as
`KEY = W * 10000 + NC * 1000 + LOG2A * 10 + OP`, where `LOG2A` is the base-two logarithm of the fold arity
and `OP` selects between a chunked fold and "an elementwise clamping chain of four steps"
(`mock/benches/variants/warm-clamp-shared/src/lib.rs:83-89`). Chain shape is already in the region key
there, and the cost vector stays a scalar per arm. The pattern exists; nothing has connected it to the
accuracy question because the accuracy question has no coordinate to connect.

### 6.2 The fidelity coordinate is reachable, and this is compiled rather than argued

Sections 2.2 and 6 rest on a reading of the harness source. `101_probes/p7_a_fidelity_coordinate/` is the
compiled version: a free-standing crate defining a `Routine` whose score is the absolute error of a
fixed-point accumulation against exact arithmetic, in units of the last place.

```
1. THE DESCRIPTOR THE HARNESS READS
   score_label, scored routine   : Some("ulp error")
   score_label, control routine  : None
   outputs_may_differ, scored    : true
   outputs_may_differ, control   : false

2. THE SCORER, THROUGH THE BYTE BRIDGE THE HARNESS ACTUALLY CALLS
           seed       truncating         rounding   closer to exact
              1           9.9297           3.0703   rounding
              2           7.8945           0.1055   rounding
   ...
   rounding closer on 8 of 8 seeds
```

Three things that a source reading cannot establish. The routine compiles against the pinned
`mockspace-bench-core` with no feature gate and no fork. The descriptor `routine_bridge!` builds carries a
working scorer reachable through the byte bridge and a non-`None` label, which is the exact consent signal
`bench-harness/src/harness.rs:228-231` tests before it will write the column. And the control routine,
identical but declaring no label, produces `None`, so the label is the gate rather than the scorer's
existence.

**The third thing is the one nobody in the unit has named.** `outputs_may_differ` is false by default, and
when it is false the harness cross-checks the arms byte for byte. So two arms that round differently
**cannot both be run** in the same family, which is the mechanism behind `98`'s finding that all thirteen
shared crates force their arms to agree. That is not a choice each crate made about its subject; it is the
default, and every crate accepted it. The accuracy coordinate is missing because a switch is off, not
because the harness cannot carry one.

There is more machinery behind that switch than the unit knows about. `bench-harness/src/quality.rs` runs
every variant over 1000 deterministic seeds and reports the mean, min, max and median of the score per
variant, and `Routine::score_dimensions` returns a labelled vector of quality dimensions described in its
own doc comment as being "for Pareto analysis" (`bench-core/src/lib.rs:182-193`). **A multi-coordinate
quality surface with its own estimator already exists in the instrument, and arvo has never called it.**
