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
the 95th percentile changes which coordinate is being weighed.** On the carrier family `{median, p95}`
reaches one section, which is what no second coordinate at all reaches, and the 95th percentile correlates
with the median at 0.978 to 0.998 across four families. And the two admissibility tests anti-correlate at
-0.64 to -0.71 across three, which makes the criterion position-dependent: separation for the first
coordinate, expressiveness for every later one. Section 5.

**Eight. Q44's guarantee comes from either of two knobs and the register only turns one.** Requiring the
named arm to be the unique argmin buys exactly what requiring strictly positive weights buys, 9 sections and
0 selecting a dominated arm, without forbidding a zero weight. And the 72-against-9 gap is one tie: two arms
declare the same 13 bits, so a pure-size weighting ties them at all six regions and `2^6` sections become
weakly rationalisable. Section 9b.

**Nine. Two of op's four stated strategy intents name a quantity the table shape cannot hold, and one of
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
dimensions for "Pareto analysis" (`bench-core/src/lib.rs:98-107`, `bench-core/src/lib.rs:182-193`). The
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
get right once" (`100:925-927`).

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
stated throughout as a percentage "of the region's achievable objective range" (`100:866`). That denominator
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
it lists as live and untested (`100:1102-1104`). It is not a refinement; a single band across coordinates in
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
resampling, 54 of 60 arm pairs separated against 43 of 60 (`100:803-812`). It concludes that three separate
negative findings about that coordinate "was a fact about the interquartile range and not about tail
behaviour" (`100:841`).

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
on this row alone; the claim I do rest is 5.3's, which is exact. Section 5.4 replaces the signal half with
a pairwise separation count and keeps the floor half, recomputed at 1000 resamples on shared draws rather
than 2000 independent ones. The two runs agree to within 2.5% of each row's own value on the carrier family,
the low-floor rows to 0.03 percentage points and the worst row by 2.3 of 149, so the floor is not sensitive
to how it was drawn. I checked that rather than assuming it, and an earlier draft of this sentence claimed
0.03 across the board, which was wrong on the three widest rows.

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

### 5.4 And the two tests are not independent, which turns the composition into something better

That is where I would have left it, and leaving it there would have been the weaker answer. Two criteria
that disagree are worth measuring against each other, and `100`'s p8 already contains the statistic to do
it with: the fraction of arm pairs whose bootstrap interval for the difference excludes zero, which is a
better separation measure than section 5.1's dynamic range and is the one thing I had recorded as unsettled.

`101_probes/p11_separation_against_expressiveness.py` puts all three numbers in one table.

```
  bitpack-carrier-width   arms=5 regions=6 pairs=60
    estimator            floor %    separated  sections
    median                  1.16     57 of 60          1
    mean                    1.45     59 of 60          1
    iqr  (spread)          48.28     43 of 60          6
    idr  (spread)          52.15     47 of 60          7
    mad  (spread)          60.81     37 of 60          8
    p95  (level)            5.66     54 of 60          1
    p99  (level)            7.36     54 of 60          1
    p95-med (excess)      101.18     38 of 60          3
    p99-med (excess)      149.05     39 of 60          2
    correlation between separation and sections, over the 9 candidates: -0.641
```

```
  bitpack-contend-decode   correlation: -0.705
  bitpack-wide             correlation: -0.673
```

**The two tests anti-correlate, at -0.64, -0.71 and -0.67 across 3 of 3 enumerable families.** The best
separators add the fewest sections and the worst separators add the most, and once seen the mechanism is
not subtle: **an estimator separates the arms well exactly when it agrees with coordinate one, and agreeing
with coordinate one is what makes it add nothing.**

So the composition is not "two criteria to be balanced". It is **position-dependent**, and that is a
sharper answer than the one section 5.3 was heading for:

- **For the first coordinate, separation is the criterion.** A coordinate that cannot tell the arms apart
  cannot order them, and the median separates 57 of 60 on the carrier family.
- **For every coordinate after the first, high separation is evidence against.** What a later coordinate is
  for is disagreement between strategies, and the measure of that is how many sections it makes reachable.
  A candidate that separates like the first coordinate is the first coordinate wearing a different name.
- **The floor applies at every position**, because an estimator whose own noise exceeds the differences it
  must resolve cannot support either job. It is what rules out the excess forms on
  `bitpack-contend-decode`, at floors of 272% and 405%.

**This is where `100`'s p8 goes wrong and it is a subtle place to go wrong.** Separation is the right
statistic and it was applied at the wrong coordinate position. The 95th percentile separating 54 of 60 where
the interquartile range separates 43 of 60 is not evidence that it is the better third coordinate; on this
family it is the reason it is not a third coordinate at all, and the section count says so exactly: 1
against 6.

**What I decline to do is rank the estimators.** The floor and the section count are both per family and
both come from the corpus, so a design gates on them rather than arguing about them, and the answer for a
family with an unresolvable spread is not the same as for one with a clean one. That is a predicate rather
than a preference.

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

## 7. The region set, and a predicate this unit has been getting wrong

Section 6.1 concludes that chain length belongs in the region rather than the coordinate vector. Checking
whether the corpus already does that for anything turned up a defect in the unit's own predicates.

**Three of the four control-bearing families are threaded benches, and their thread count is the last digit
of the region key.** `mock/benches/bench.toml` declares `threaded = true` on six sections, and the
contention crate documents the encoding in its own source: "One row of the sweep: `KEY = N * 10 + T`", with
`N` the elements and `T` the "threads walking it"
(`mock/benches/variants/bitpack-contend-shared/src/routine.rs:11-24`).
`mock/benches/variants/bitpack-wide-shared/src/lib.rs:102` says it carries "the contention crate's
encoding unchanged".

`101_probes/p8_the_region_key_carries_the_thread_count.py` decodes every committed key:

```
  family                        regions   element counts   thread counts
  bitpack-contend-best                4                2   [1, 4]
  bitpack-contend-decode              6                3   [1, 4]
  bitpack-contention                 12                4   [1, 2, 4]
  bitpack-wide                        6                3   [1, 4]
  bitpack-write-contend-race          6                2   [1, 2, 4]
  bitpack-write-contend-safe          6                2   [1, 2, 4]
```

**So `100`'s F-100-3b carries `threads = 1` (`100:520`) for a finding computed over three families whose
regions span 1, 2 and 4 threads.** That is not conservatism. A section over `bitpack-contention`'s twelve
regions is a vector with entries measured at three different thread counts, and it cannot be reproduced from
single-threaded data at all, so `threads = 1` names a region the finding does not live in. `RULES.md` is
explicit that a fixed value means established there and only there.

I do not think this changes any conclusion `100` draws, and I say so plainly: the mechanism it establishes,
that a control arm trades places wherever one exists, is if anything better supported by spanning three
thread counts than by one. What changes is the region, which under I13 is what makes a finding usable by an
arm. My own findings in section 12 carry the decoded sets.

**And the shape is worth having for its own sake.** The thread count is a **region dimension** in this
corpus and not a coordinate: it indexes which measurement you are reading rather than being a number an arm
is scored on. That is exactly the answer section 6.1 reaches for chain length, arrived at from the opposite
direction. The corpus already does it for threads; nothing does it for chains; and both belong on the same
side of the line.

The general form, which is the piece of the object my question was really about:

> A quantity belongs in the **region** when a strategy's answer may differ across it, and in the **cost
> vector** when a strategy's answer is scored on it. Element count, width, arity, thread count and chain
> length are the first kind. Time, size, spread and fidelity are the second. Putting one on the wrong side
> is not a modelling preference: a region dimension in the cost vector makes the table lie by averaging, and
> a coordinate in the region key makes it unable to trade at all.

## 8. What I keep

**`97`'s and `98`'s counts, at a third independent instance.** 72 and 9 reproduce exactly from an instrument
written from interval and polygon clipping without reading either predecessor's. `RULES.md` puts the bar at
three independent instances and those two numbers now clear it. I add only the reading: 9 is a fact about a
two-coordinate model, and is the first measurement in this unit of how many strategies a coordinate set can
distinguish.

**`98`'s F-98-9, that no committed family carries a column for accuracy or divergence.** My census crosses
the same corpus from the schema side and from the variant side and finds nothing that changes it. What I add
is where the gap is: not the column, which exists, but a routine nobody wrote and a default nobody turned
off.

**`100`'s F-100-6, the independence failure under min-max normalisation.** Corroborated by a second
instrument on three further families, with the mechanism unchanged. What I add is that the remedy costs
nothing, because declared-range normalisation is not a second model.

**`100`'s F-100-3 and F-100-3b, the control arm carrying the section's instability.** Corroborated from a
different direction: exact interval arithmetic on the point estimates, no resampling, finds the control arm
in the argmin in eight of the nine cells of the carrier family's exchange-rate axis. The predicate wants the
decoded thread counts, per section 7, and the mechanism stands.

**`100` section 7.1's use of the control pair as a calibration.** This is the best instrument the unit has
found and section 5 generalises it from one statistic to any: every estimator has a floor and the corpus
measures it. Keeping it and extending it is the result.

**`97`'s three-layer polarity reading and `94`'s W9.** Untouched, and I have no evidence bearing on either.

**The cost table's shape.** Section 6.1 could have been an argument for a richer cell and is not: the table
survives, with chain length moved into the region where the corpus already puts thread count.

## 9. Shapes found and not taken

**1. Measure the storage coordinate rather than declaring it.** Rejected on the reasoning, not deferred: the
bytes per element of a declared layout is exactly known, and measuring it would replace an exact number with
an estimate. What follows instead is that the coordinate set is **mixed**, and a tolerance band derived from
measurement uncertainty must not be applied to a declared coordinate. Section 2.3.

**2. Drop the spread coordinate entirely.** `98` p12 finds its differences not distinguishable from zero at
the comparisons that mattered, and `100` reads that as a fact about the interquartile range. Section 5.3
kills the drop: on the carrier family the spread coordinate is the only one of the three that adds any
reachable section, so dropping it leaves a design with one axis and one strategy.

**3. Use the 95th percentile as `100` proposes.** Closed by section 5.3 on the family it was measured on: it
reaches one section, which is what no second coordinate at all reaches.

**4. A tail excess, `p95 - median`, as the third coordinate.** **Live, and the candidate whose case
changed most while I measured it.** It adds 3, 9 and 5 sections on the three enumerable families, beating the
level it is built from in 3 of 3, and it is the shape a tail-weighing intent actually asks for. Section 5.4
then measured that its poor separation is expected of any real second coordinate rather than evidence
against it, so the case against it rests entirely on its floor: 101% on carrier and 405% on
`bitpack-contend-decode`, against the interquartile range's 48% and 135%. **On a family whose instrument
resolves it, it is the better third coordinate; on this corpus it is not resolved.** That is a predicate, and
it is measurable before anything is built.

**5. Turn on `--perf-counters` and add an instruction-count coordinate.** **Live and unbuilt.** It needs
`sudo` on the Apple-Silicon host the corpus already ran on, and it is the only candidate coordinate that
does not move with machine load. I did not run it: taking a privileged bench run on a shared clone is not
mine to do, and the PMU claim is exclusive while it is held
(`bench-harness/src/perf.rs:16-21`).

**6. Add a fidelity coordinate to a real bench family.** **Live and unbuilt, and p7 is the feasibility half
of it done.** What remains is a routine, a second arm that rounds differently, `outputs_may_differ`, a
`bench.toml` section and a run. I did not do it because adding a family to the shipped corpus is a design
act this unit has not agreed on, and because `95` scopes harness work to what the panel's continuation
needs. That scope call is mine and is attackable.

**7. Use `setup_ns` and the breakeven the harness derives from it.** **Live and unbuilt.** The harness
states `k* = (S_b - S_a) / (I_a - I_b)` as computable directly from a matrix run
(`bench-core/src/lib.rs:427-429`). That is a region boundary derived from measurement rather than declared,
which is the shape every predicate in this panel is reaching for by hand.

**8. Report a strategy's margin rather than its weights.** **Live and untested as a mechanism.** Section 4.3
computes it; nothing has built it into a design or priced its const-time cost.

**9. Per-region normalisation.** `100` lists it as untested (`100:1090-1092`). Section 3.1 closes it on the
algebra: any transform whose scale is read off the data breaks independence, and reading it off a region's
arms rather than the whole table narrows the reach without changing the mechanism. The frozen transform
already costs nothing, so there is nothing to gain by a partial fix.

**10. Weight the coordinates on a log scale rather than linearly.** Not tested. A weighted sum of logarithms
is a weighted geometric mean, which is scale-invariant per coordinate and would make the units question
disappear rather than answering it. It is a different objective with different laws, and every count in
section 4 would have to be recomputed under it. I name it because the unit has assumed a linear objective
throughout without anyone saying so, including me.

## 9b. What this does to the register's live options

`RULES.md` asks each file to say which options its findings fit, which they fit badly, and which they kill.
I read `OPTIONS.md` Q14, Q15, Q43, Q44 and Q45 and nothing else in that file, so this covers those five.

**Q14, the exchange rate at which a strategy's preference yields. Fits well, and the cost of one option
drops.** Q14's options are a stated rate per objective, a lexicographic ordering with no rate, a
consumer-supplied rate, or silence. Section 4.3 measures the object those options range over: the rate axis
is partitioned into 8 to 12 cells per family and the interior cells are 0.29 to 1.48 decades wide, so **a
stated rate has to be right to within a factor of a few, not right.** That is the cheapest of the four
options getting considerably cheaper, and it also gives the consumer-supplied variant a well-defined
acceptance test: does the consumer's rate land in a cell, and how far is it from the edge.

It also names what a lexicographic ordering is in these terms: the limit as one rate goes to zero or
infinity, which is the outermost cell at either end. So the lexicographic option is not a different kind of
answer, it is the two unbounded cells of the same axis, and section 9b's Q44 entry below is about exactly
those two cells.

**Q15, whether the axes are independently resolvable. Fits badly, and I add a mechanism rather than a
verdict.** Q15 is about resolving the strategy's per-axis assignment. Section 4.2 measures the adjacent
thing: with one coordinate every weighting agrees, so a per-axis argmin is not merely cheaper than a joint
resolution, it is a different object that cannot express disagreement at all. That does not settle Q15 and
it does say what the cheapest option costs.

**Q43, checked against generated. Fits, and I add nothing to the fork.** I agree with `100` that the fork
has no consumer-visible content and that the composition is what replaces it. What I add sits underneath:
section 3.3 says the differential's tolerance must be stated per coordinate in the coordinate's own units
rather than as a fraction of the achievable range, because the second form inherits the independence failure
the same composition is trying to avoid.

**Q44, strictly positive against non-negative. A fourth option, measured, and it costs nothing the other
three cost.** Q44 records (a) require strict positivity, costing "the ability to express a strategy that
genuinely does not care about a coordinate"; (b) allow non-negativity and lose the guarantee; (c) allow it
and carry the dominated-arm check separately.

The property has **two independent knobs and the register's options only turn one of them.**
`101_probes/p10_the_two_knobs_are_separable.py` runs all four cells on `97`'s model:

```
     weights     minima  sections   selecting a dominated arm
      w >= 0       weak        72   63
      w >= 0     unique         9    0
       w > 0       weak         9    0
       w > 0     unique         9    0
```

**Requiring the named arm to be the unique argmin buys exactly what requiring strict positivity buys, and
it does not forbid a zero weight.** That is a fourth option: **(d) allow non-negative weights and require a
unique argmin.** It keeps the guarantee, keeps the ability to zero a coordinate, and asks the design for
something it should want anyway, since a strategy whose table names an arm that merely ties is a strategy
whose table does not determine its own answer.

**And the 63 has an exact mechanism, which nobody had named.** Block B: `bitpack-carrier-packed` and
`bitpack-carrier-packed-simd` both carry 13 bits, so at a pure-size weighting they tie at all six regions.
That is `2^6 = 64` sections made weakly rationalisable by one tie, of which 63 name `packed` somewhere and
one is the all-simd section already reachable. **The whole 72-against-9 gap is one tie between two arms with
the same declared size.** `98`'s account of it, that a zero weight admits an arm the weighting is
indifferent about, is right and this is the arithmetic under it.

Block B also measures the other half of (a)'s stated cost. At two coordinates a zero weight is the limit of
positive ones, and the section at `r = 0` is reproduced at `r = 1e-1` through `r = 1e-12`. So under (a) a
strategy that does not care about a coordinate is expressible **to any tolerance**, just not exactly. What
(a) actually forbids is not indifference; it is admitting an arm that only ties.

**My own probe got this wrong first and the error is the interesting part.** Its first version used strict
Pareto domination, found one dominated arm where `97` and `98` name two, and reported 0 in the dominated-arm
column of all four cells, silently losing the predecessors' 63. `bitpack-carrier-packed` is dominated only
if equality on a coordinate counts, and it must: if `b <= a` everywhere and `b < a` somewhere then
`<w, a - b> > 0` for every `w > 0`. The output is kept at
`101_probes/p10_first_version_used_strict_domination.out`.

**Q45, what to do about arms no weighting can select. I add a discriminator that separates (a) from (c).**
Q45 offers (a) drop them as measurement spend with no decision attached, (b) they indicate a missing
coordinate, (c) keep them as a documented negative control.

Section 3.2 measures what keeping them costs, and the answer is conditional: **under raw or frozen
coordinates a dominated arm changes nothing, 0 of 2000 weightings in 4 of 4 families; under min-max
normalisation against the arm set, dropping one changes the answer for 961 of 2000 weightings on the carrier
family.** So (c) is safe exactly when the coordinates are not normalised against the arm set, and unsafe
otherwise. That is a predicate rather than a preference, and it means Q45 and the normalisation question are
the same question asked twice.

And (b) gains an instance rather than losing one. `98` tested whether a third coordinate un-dominates an arm
and withdrew the rescue. Section 4.2 measures the general version: a third coordinate takes the reachable
section count from 9 to 42, so the coordinate set does change which arms are selectable, and `98`'s specific
rescue failing is a fact about that coordinate rather than about the mechanism.

## 10. Located disagreement

**With `100`, on the tail estimator: I break part of it and keep the method.** Its F-100-7 says estimating
the tail coordinate by the 95th percentile rather than the interquartile range is "strictly better on both
axes" (`100:812`). Both axes it measures are resolution axes. On the third axis, whether the coordinate
still exists, the 95th percentile reaches one section on the carrier family, the same as having no second
coordinate. So the swap buys stability by deleting the axis, and the constructive replacement is the excess
form.

What I do not dispute: the interquartile range is a noisy estimator, the resolution measurements are right,
and the general move of asking which statistic a coordinate is estimated by is the most useful thing in that
section. `100` should answer, and should be **resumed** rather than re-dispatched.

**With `100`, on the tolerance band: the mechanism is right and its currency is wrong.** Section 3.3. A band
stated as a fraction of the achievable objective range inherits the independence failure `100` itself found.
The fix is `100`'s own control-pair calibration, expressed per coordinate.

**With the unit as a whole, on `threads = 1`.** Section 7. Every finding in this unit computed over the
contention, decode or wide families carries a predicate naming a region the finding does not live in. This
is mechanical rather than a judgement, and it is cheap to correct in each author's own file, which is where
`RULES.md` says a widened or corrected predicate belongs.

**With myself, on whether the interquartile range is admissible: opened and closed.** Section 5.1's signal
statistic is a dynamic range and one extreme arm can carry it, so I rebuilt it as `100` p8's pairwise
separation count. Section 5.4. The rebuild did not settle the question I asked it: it dissolved it, by
measuring that separation and expressiveness anti-correlate, so the two tests are not two opinions about one
property. What remains open is whether the anti-correlation holds on a family with more than five arms,
which the enumeration cannot reach.

## 11. For op, and it is nothing

Deliberately, and there are three things I considered putting up.

**Which estimator a coordinate should use** is the shape op has rejected four times. Section 5 answers it as
two measured criteria with a per-family answer.

**Whether the weightings agree in practice** is answered in `INTENTS.md:155-156` as an ordinary empirical
question that is not an intent he owes. Section 4 measures the ceiling that bears on it and reports it as a
measurement.

**Whether arvo should measure accuracy at all** looks like a question for him and is not. I5 already
requires it: "should not lose it for nothing, instead, provable meaningful gains" (`INTENTS.md:102-103`) is a
constraint over two quantities and the corpus carries one. So the accuracy coordinate is not a new intent
somebody has to want; it is the missing half of an intent op has already stated.

The two items `99` carries for op, both about I3, are untouched by anything here.

## 12. Findings, each with its predicate

Notation per I13 and `RULES.md`: a dimension listed with a range or `any` was established across it, listed
with a fixed value was established there only, and absent means the finding does not hold anywhere that
dimension is present.

**F-101-1. Of the harness CSV's seventeen columns, nine carry information across the committed corpus and
exactly three of those vary between arms at a fixed region: `e2e_ns`, `algo_ns` and `bridge_ns`. Eight are
identically empty or zero in every row: `cooldown_ms`, `score`, `input_tag`, `instructions`, `cycles`,
`setup_ns`, `first_ns`, `digest`.**
`holds for: files = the 254 committed mock/benches/*.csv, rows = 104080, groups tested = 248 (family,
region) pairs, schema = the seventeen columns at bench-harness/src/harness.rs:752 in the pinned checkout,
threads any, target features any, host any (a census of committed files)`
Evidence: `101_probes/p1_the_coordinate_census.py`. A census of committed artifacts, not a bench.

**F-101-2. Of 94 variant crates, 0 implement `score_output`, `score_label` or `score_dimensions`, 15
implement `validate_output`, and 1 mentions `outputs_may_differ`. All 82 measured call sites use the
`timed!` constructor and none uses any other, which is why `setup_ns`, `first_ns` and `digest` are zero.**
`holds for: variant crates = 94, source files scanned = 114, at the committed tree, threads any, target
features any, host any`
Evidence: `101_probes/p1_the_coordinate_census.py`.

**F-101-3. Two of the three declared byte-identical control pairs are instruction-identical after
normalising the path header, absolute addresses, `adrp` page numbers, the exported symbol names and the
literal-pool comment; the third, `bitpack-wide-d16` against its control, differs at three sites in the
addressing form of a constant-pool vector load, at equal instruction counts of 55513.**
`holds for: pairs = {bitpack-carrier-d16, bitpack-contend-d16, bitpack-wide-d16} each against its declared
control, target = aarch64-apple-darwin, rustc 1.98.0-nightly (57d06900f), release profile as each crate
declares it, threads any (a compile-time artifact), target features baseline`
Evidence: `101_probes/p0_control_identity_on_every_pair.sh`.

**F-101-4. A per-coordinate affine transform with a range frozen as declared constants produces the same
section as raw coordinates under the reparameterised weighting, for 2000 of 2000 random weightings on each
of four families. Under min-max normalisation whose range is read off the arm set, dropping an arm dominated
at every region moves the section for 961 of 2000 weightings on the carrier family and 252 of 2000 on
contention, and adding an arm strictly worse everywhere moves it for 1, 2, 0 and 176 of 2000; under raw and
under frozen coordinates both perturbations move nothing, 0 of 2000, in 4 of 4 families.**
`holds for: families {bitpack-carrier-width, bitpack-contend-decode, bitpack-contention, bitpack-wide},
regions in {6, 12}, arms in {4, 5, 6, 7}, cost coordinates = 2 (median algo_ns, interquartile range of
algo_ns), draws = 2000, seed 20260814, declared range = (0.5 * min, 3 * max) per coordinate, synthetic arm
factor = 32, cost source = the committed CSVs, threads in {1, 2, 4} (per the region keys decoded in
F-101-8), host = the one those runs were taken on`
Evidence: `101_probes/p2_normalisation_is_a_change_of_basis.py`.

**F-101-5. A tolerance stated as a percentage of the region's achievable objective range grows by a factor
of 59.1 to 185.0 when an arm that no weighting can select is added to the table.**
`holds for: families {bitpack-carrier-width, bitpack-contend-decode}, regions = 6 each, arms in {5, 6, 7},
cost coordinates = 2, weighting = speed only so the objective is a time in nanoseconds, synthetic arm factor
= 32, cost source = the committed CSVs, threads in {1, 4}, host = the one those runs were taken on`
Evidence: `101_probes/p2_normalisation_is_a_change_of_basis.py` block C.

**F-101-6. The number of sections a strictly positive weighting can reach on the committed carrier table is
1 for one coordinate, 9 for two, and 42 for three, with the control arm dropped; 1, 9 and 58 with it kept.
The instrument reproduces `97`'s published 72 at non-negative weights and 9 at strictly positive weights
exactly.**
`holds for: family = bitpack-carrier-width, regions = 6, arms in {5, 6}, coordinate sets {median algo_ns},
{median algo_ns, bits per element}, {median algo_ns, interquartile range}, {median algo_ns, bits per element,
interquartile range}, cost source = the committed CSVs with bits per element declared as 16, 32, 64, 13, 13,
exact rational arithmetic, threads = 1, host = the one those runs were taken on`
Evidence: `101_probes/p4_what_a_coordinate_buys.py`, cross-checked by
`101_probes/p3_the_exchange_rate_and_its_cells.py`, which computes the two-coordinate count as an interval
count rather than a feasibility count and also reports 9.

**F-101-7. On `{median algo_ns, candidate}` over the committed carrier table with the control dropped, the
95th and 99th percentiles reach 1 section, which is what the median alone reaches, while the interquartile
range reaches 6, the median absolute deviation 8, and `p95 - median` 3. The 95th percentile correlates with
the median at 0.978 to 0.998 across four families. Each estimator's resolution against its family's
byte-identical control pair ranges from 1.18% for the median on the carrier family to 400% for `p95 - median`
on `bitpack-contend-decode`.**
`holds for: families {bitpack-carrier-width, bitpack-contend-decode, bitpack-wide} for the section counts and
{those three, bitpack-contention} for the correlations and floors, regions = 6 (12 for contention), arms in
{3, 4, 5, 6}, estimators {median, mean, P75-P25, P90-P10, median absolute deviation, P95, P99, P95 minus
median, P99 minus median}, 80 samples per arm per region, 2000 bootstrap resamples, seed 20260814, cost
source = the committed CSVs, threads in {1, 2, 4}, host = the one those runs were taken on`
Evidence: `101_probes/p5_an_estimator_is_a_coordinate_choice.py`. The `bitpack-contention` section count is
not reported: 5 arms over 12 regions is 244 million sections and the enumeration is exact rather than
sampled.

**F-101-8. Six of the 49 committed families are declared `threaded = true`, and every one of them encodes
its thread count in the last digit of its region key. `bitpack-contention` spans threads 1, 2 and 4;
`bitpack-contend-decode`, `bitpack-wide` and `bitpack-contend-best` span 1 and 4; `bitpack-carrier-width` is
not threaded.**
`holds for: families = all 49 committed, keys = every committed region key, encoding = KEY = N * 10 + T per
bitpack-contend-shared/src/routine.rs:11-24, threads any, target features any, host any (a census of
committed files)`
Evidence: `101_probes/p8_the_region_key_carries_the_thread_count.py`.

**F-101-9. For two fixed-point arms, one on a grid four times finer with truncation and one on the declared
grid with round-to-nearest, the mean absolute error ranking against exact arithmetic reverses at chain length
4: the finer truncating arm is twice as accurate at one operation and four times worse at sixty-four. The
control pair, the same grid with truncation against round-to-nearest, does not cross at any length up to 96.**
`holds for: model = accumulation of quantised products, fractional bits F = 8, grids {2^-10 truncating,
2^-8 round-half-to-even}, chain lengths 1 through 96, streams = 4000, seed 20260814, inputs uniform on the
declared grid excluding zero, reference = exact rational arithmetic, threads any, target features any, host
any (a computation over a model, not a measurement)`
Evidence: `101_probes/p6_accuracy_is_not_a_per_arm_scalar.py`.

**F-101-10. A `Routine` carrying a fidelity metric compiles against the pinned `mockspace-bench-core` with no
feature gate and no fork; the descriptor `routine_bridge!` builds carries a working scorer through the byte
bridge and a `Some` label, which is the signal `bench-harness/src/harness.rs:228-231` tests before writing
the `score` column; an otherwise identical routine declaring no label produces `None`; and the scored routine
sets `outputs_may_differ`, without which the harness compares the arms byte for byte and two arms that round
differently cannot both be run.**
`holds for: mockspace-bench-core at the pin in mock/benches/Cargo.lock (bce17f6c), rustc 1.98.0-nightly
(57d06900f), edition 2024, target = aarch64-apple-darwin, feature gates = 0, seeds = 8, threads any, target
features any`
Evidence: `101_probes/p7_a_fidelity_coordinate/`.

**F-101-12. Across the three enumerable control-bearing families, an estimator's arm-pair separation and the
number of sections it makes reachable as a second coordinate are negatively correlated, at -0.641, -0.705 and
-0.673 over nine candidates. On the carrier family the median separates 57 of 60 pairs and reaches 1 section,
while the median absolute deviation separates 37 of 60 and reaches 8.**
`holds for: families {bitpack-carrier-width, bitpack-contend-decode, bitpack-wide}, regions = 6, arms in
{3, 4, 5} with the control dropped, estimators = the nine at NAMES in the probe, 80 samples per arm per
region, 1000 bootstrap resamples, seed 20260814, separation = a 95% percentile interval excluding zero,
expressiveness = the exact count reachable by a strictly positive weighting on {median, candidate}, cost
source = the committed CSVs, threads in {1, 4}, host = the one those runs were taken on`
Evidence: `101_probes/p11_separation_against_expressiveness.py`. `bitpack-contention` is excluded from the
correlation: 5 arms over 12 regions is 244 million sections and the count is exact rather than sampled.

**F-101-11. On `97`'s two-coordinate carrier model, rationalisability has two independent knobs and either
one alone collapses the count from 72 to 9 with no section selecting a dominated arm: allowing a zero weight
while requiring the named arm to be the unique argmin gives 9, and requiring strictly positive weights while
allowing ties gives 9. The 63-section gap is produced by a single tie: `bitpack-carrier-packed` and
`bitpack-carrier-packed-simd` both declare 13 bits, so a pure-size weighting ties them at all six regions,
making `2^6` sections weakly rationalisable of which 63 name the dominated arm. The section a zero weight on
size selects is reproduced by strictly positive exchange rates from `1e-1` down to `1e-12`.**
`holds for: family = bitpack-carrier-width, regions = 6, arms = 5 (control dropped), cost coordinates = 2
(median algo_ns, bits per element declared as 16, 32, 64, 13, 13), domination = weak Pareto at every region,
exact rational arithmetic, cost source = the committed CSVs, threads = 1, host = the one those runs were
taken on`
Evidence: `101_probes/p10_the_two_knobs_are_separable.py`.

## 13. What I did not do, and what I could not settle

**I did not derive blind.** My brief carried `100`'s three headline findings, so nothing here claims
independence except where the instrument is genuinely different: p4's feasibility geometry, written without
opening either predecessor's decider, and p3's interval arithmetic, which answers the same question by a
route neither used. Section 5's collinearity and section-count tests are new questions rather than second
reads.

**I read a small part of the panel.** In full: `INTENTS.md`, `RULES.md`, `99`, `100`, `96`. In part: `98`
via its p6 output and `p6_model97.json`, `97` via `98`'s reproduction of it and via `99` and `100`'s
accounts, `93` and `94` only through those accounts, and `OPTIONS.md` entries Q14, Q15, Q43, Q44 and Q45 in
full. **Not read:** every other member file, the rest of `OPTIONS.md`, `DROPLIST.md`, `PERSONA_CALLS.md`, `PRIOR_CALLS.md`, the `SEED_*` files,
the archive. So where any of this restates something, I do not know it. In particular **I did not read
`97`'s own file**, only `98`'s reproduction of its model and its published counts, which is exactly the
single-point-of-failure `RULES.md` names; my section 4.1 rests on `98`'s account of `97`'s model being
right, and if it is wrong my calibration is calibrated against the wrong thing. The counts agreeing at both
rungs is weak evidence that it is not.

**I did not run a bench.** Nothing here is a measurement. Every number is a computation over committed
artifacts, a compile-time fact, or arithmetic over a model, and each finding says which. The one coordinate
I could have measured, the instruction count, needs a privileged run on a shared clone.

**Compile time stays unpriced**, and I add nothing to `100`'s account of why.

**The excess estimator is not settled and the shape of the question changed.** Section 5.4 measures that
separation and expressiveness pull opposite ways by construction, so the excess forms scoring badly on
separation is expected rather than disqualifying. What would settle it is their floor, and on
`bitpack-contend-decode` that floor is 272% and 405%, which rules them out there and says nothing about a
family with a cleaner instrument.

**I could not settle what the storage coordinate should be for a layout nobody has declared.** Bytes per
element is exact for a declared layout. For a strategy that chooses a layout, the storage coordinate is a
function of the choice, which makes it an output of the section rather than an input to it. I noticed this
late, did not test it, and name it as an open shape rather than a finding.

**I did not attempt the axis set, the three-layer split, or the chain question as `99` frames it.**

## 14. Coverage of the citations, and a panel-mechanics report

Every `file:line` in this document was opened and its content tested rather than merely resolved, by
`101_probes/p9_verify_my_citations.py`. Each row carries the phrase the citation is for, so a citation that
drifts onto a neighbouring line fails rather than passing on a coincidence.

```
citations checked: 37   ok: 37   failed: 0
```

**Fourteen of the thirty-seven failed on the first run and every one was mine.** The first output is kept at
`101_probes/p9_first_run_fourteen_of_mine_failed.out`. Four kinds:

- **Eight were line citations into `100`, which moved under me while I read it.** Its author is still
  committing to this branch: the file was 1254 lines when I read it and is 1300 now, having gained 46 lines
  above the sections I cite, so every line number I took shifted by twenty to thirty. The numbers in this
  file are pinned to `100` at commit `cad7a505`.
- **Four were a phrase spanning a line break**, which the checker joins with a newline and therefore misses.
  A defect in my expected phrases rather than in the citation, and worth naming because it fails in the safe
  direction.
- **One named a file that does not exist**, `bitpack-wide-shared/src/routine.rs`, for a doc comment that
  lives in that crate's `lib.rs`. I had inferred the path from the sibling crate's layout rather than
  listing the directory, which is the same class of error as citing from memory.
- **One cited a doc comment two lines below where it starts.**

That is the ninth through twenty-second recorded instance of this class across two panels, and the count is
reported rather than quietly fixed, per `RULES.md`.

**The moving-file half is a panel-mechanics report rather than a finding about the work.**
`how-to-run-a-panel.md` says never to edit a document a member is reading, and to prefer a heading anchor
over a line number for anything still growing. Both were live here in a way the rule does not quite cover:
`100` is not a consolidation and not the intent catalogue, it is a finished member file, and finished member
files are exactly what a later member cites by line. The cheap remedy for whoever coordinates the rest of
this unit is to treat a member file as frozen once the next dispatch opens, or to say in the brief that it
is not.

**Everything else in this file was verified the same way and the count is the count.** What the instrument
does not check is whether a cited passage supports the argument I put on it, which no probe can do and which
is what a second reader is for.
