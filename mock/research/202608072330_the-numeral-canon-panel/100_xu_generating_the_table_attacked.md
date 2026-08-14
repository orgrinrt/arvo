# 100. Generating the table, attacked

**Predecessors:** `93` and `94`, the unit's cold pair; `97`, which attacked both; `98`, which second-read
`97` and proposed inverting it. **Probes:** `100_probes/`, eleven of them plus one shared instrument
and two build scripts, each committed as it ran.

This is the fifth file of the unit and the first of the four that `95` points at convergence. So the
shape is fixed before the content is: where I break something I say what replaces it, where a proposal
survives I take it further and carry it as shape, and where two members cannot be reconciled I state the
disagreement precisely rather than settling it to fit the budget. Op decides; nothing here settles
anything.

Three things up front, because they are the file in miniature.

**Q43 as posed is a fork with no consumer-visible content, and the fork underneath it has plenty.**
Check-against-a-weighting and generate-from-a-weighting are two maintainer workflows that emit the same
artifact. The axis that changes what a consumer can do is the one `93` named and nobody picked: whether
the compiler is handed a **winner table** or a **cost table**. p3 compiles both and the linker merged
them into one symbol, so the second costs nothing at the point of use and buys a consumer a weighting
nobody tabulated.

**Generating does not remove the check, it relocates it onto the generator, where rationalisability is
almost blind.** p2 injects five ordinary generator defects and measures three detectors: the
rationalisability criterion catches 0 of 190 unit errors and 0 of 147 column swaps, because a generator's
mistake is the correct answer to a different question and a different question still has a weighting.

**`98`'s motivating measurement reproduces, and its inference holds on some families and not others.**
p1 finds that every one of speed-first's flips is between `bitpack-carrier-d16` and
`bitpack-carrier-d16-control`, an arm and its own **byte-identical noise-floor control**, and that
dropping the control leaves **one distinct section across 2000 resamples** where `98` reports thirty. p7
then runs the same decomposition on all four committed families that carry a control arm. **The
mechanism generalises: a control arm trades places in 4 of 4.** The conclusion does not: dropping it
leaves the section free on two families and costing 1.1% and 3.8% of runtime on the other two. That is a
predicate rather than a refutation, and section 4 states it.

## 0. The gates

**Canon gate: passed, with one finding recorded rather than let past.**

Checked against `INTENTS.md` I1 through I17. The assigned question is licensed by I1, demoted to open in
op's own words (`INTENTS.md:56`), and by I13's arms-with-predicates framing. Nothing in the intents
forecloses asking how a strategy's table is produced.

The finding: **Q43 is written in `OPTIONS.md` as a two-way fork over a whole category**, "(a) Check a
table against a weighting" against "(b) Generate the table from the weighting"
(`OPTIONS.md:2059-2063`). That is the shape op has now rejected three times in one sitting: at `88`
section 4 ("we don't need to settle for one universal solution, it's the anti-pattern I've already
named"), at `85` section 2 declining to rank four const-time constructions, and at `83` rejecting the
typestate-against-values fork outright. `never-ask-which-single-rule-governs.md` names it.

I did not return early on it, and the reasoning is offered so somebody can disagree. The question has a
non-policy reading, which is "what does each production order cost and where does each hold", and that
reading is answerable and is what the brief asks for. Answering it as **arms with predicates** rather than
as a winner is what sections 1 through 5 do. I flag it because if the register carries Q43 forward as a
binary into the consolidation, the consolidation inherits the shape rather than the answer.

**A second and smaller one, on I16.** `98` section 0 records the same worry about `97`'s
canon-level constraint on table shape and resolves it by saying section 3 removes the sharp edge, because
a property holding by construction polices nobody. I agree with the resolution and I would add that
section 4 below sharpens it: the canon-level statement that survives is about what a strategy **is**, and
both production orders are design-tier mechanisms the canon should decline to rank. That is I16 applied
one level up from where op applied it.

**Test gate: passed. 123 tests across 13 crates, and I read the bodies of the ones my work rests on.**

There is no arvo suite: `cargo test --manifest-path mock/Cargo.toml` errors with "the manifest is
virtual, and the workspace has no members", which is the intended state. So I gated the bench variant
crates and ran them myself rather than taking three unratified files' word for it.

```
grep -rc '#\[test\]' mock/benches/variants/ | grep -v ':0$'    # 124 attributes, 13 crates
cd mock/benches/variants/<crate> && cargo test                 # per crate
```

Per crate: `bitpack-carrier-shared` 9, `bitpack-contend-shared` 12, `bitpack-footprint-shared` 6,
`bitpack-plan-shared` 5, `bitpack-shared` 3, `bitpack-wide-shared` 6, `quantiser-fadd-shared` 1,
`quantiser-radix-shared` 3, `satfold-shared` 11, `warm-clamp-shared` 7, `warm-container-shared` 15,
`wide-rung-shared` 30, `bitpack-write-contend-shared` 15. **Total 123, all green.**

The grep counts 124 because `mock/benches/variants/bitpack-write-contend-shared/src/stress.rs:68` contains
the literal `#[test]` inside a doc comment. I opened it rather than reporting the grep.

**`99` records that three members reported 108, 96 and 123, and that the coordinator had not
independently recounted. The count is 123 and `98`'s account of it is exactly right**, verified here by a
fourth party from the source rather than from any file's report. The two lower figures are `94` not
running the slow crate and `97` running eleven of the thirteen.

**And the slow crate is slow for a reason nothing in it declares, which is a finding of its own.** `98`
reports `bitpack-write-contend-shared` running in 4.7 seconds once built. Under `cargo test`'s default
parallel runner it did not complete in ten minutes of wall clock on this host, twice. Run
single-threaded it completes in **45.22 seconds, 15 passed**:

```
./target/debug/deps/bench_bitpack_write_contend_shared-<hash> --test-threads=1
test result: ok. 15 passed; 0 failed; ... finished in 45.22s
```

The crate's own comment explains the mechanism without drawing the conclusion: "`cargo test` runs every
`#[test]` in one process, so every stress test in this file shares that pool and must agree on one thread
count" (`mock/benches/variants/bitpack-write-contend-shared/src/stress.rs:66-72`). Fifteen tests running
concurrently against one shared four-thread pool is the contention. **The suite is effectively
serial-only and nothing in the crate says so**, so the next person to gate this corpus will lose the same
twenty minutes I did unless somebody writes it down. That is what I am doing here.

**The bodies I depended on are real work.** My arguments rest on the `bitpack-carrier-width_n*` CSVs, so I
read `bitpack-carrier-shared`'s tests in full. `all_four_transforms_agree` compares three dense arms and
the packed decode against each other on one column
(`mock/benches/variants/bitpack-carrier-shared/src/lib.rs:373-388`), and
`validate_output_rejects_a_wrong_sum` feeds the validator a sum off by one and asserts it refuses
(`.../src/lib.rs:394-411`), with its own doc comment stating why: "a validation pass that cannot fail is
not a validation pass". A suite carrying a negative control on its own validator is not decorative.

**And I read the arm my central finding turns on, which nobody in the unit had.**
`mock/benches/variants/bitpack-carrier-d16-control/src/lib.rs:1-8` declares itself:

> Noise-floor control. Calls the identical `sum_d16` on the identical region with the identical arguments
> as `bitpack-carrier-d16`, so the two arms differ only in the exported symbol name and must compile to
> the same machine code. Any measured gap between them is the harness's own resolution on this workload,
> and every other delta in the run is read against it. The byte-identity is not assumed:
> `26_probes/control_identity.sh` extracts both function bodies from the two built dylibs and diffs them.

I found nothing to refuse on.

**The known gate hole is unchanged and I rest nothing on it.** `96` establishes the harness never invoked
the per-variant validators; the pin is now bumped (`cd44aeaf`) and no run has been retaken. Every number I
read from a CSV is a timing.

## 1. The answer, before the working

**One. The check does not disappear under generation; it relocates onto the generator, and
rationalisability cannot see it there.** Five ordinary generator defects, three detectors, on the
committed carrier table and on 400 random models. Rationalisability at `w >= 0` catches **0 of 190**
unit errors, **0 of 147** column swaps and **0 of 152** dropped coordinates. Cone membership of the
**stated** weighting catches **100%** of all three. Section 2.

**Two. `98`'s strictly-positive rung buys nothing against a generator, and the predicate under which its
real guarantee is reachable is narrower than stated.** I predicted the strict rung would catch a dropped
coordinate and was **wrong, 0 of 312**. Chasing why: the rung detects exactly one event, a section
selecting a Pareto-dominated arm, and that event needs a pair of arms tied on every coordinate the
weighting reads and differing on one it does not. 0 of 230 on independent arms, **230 of 285** on that
shape, and the two columns are identical in all four families. Section 3.

**Three. `98`'s section instability is carried by the harness's own noise-floor control arm in every
family that has one, and what remains after dropping it is free on two families and not on two.** On the
carrier family, dropping `bitpack-carrier-d16-control` takes speed-first from 31 distinct sections across
2000 resamples to **1**, at a mean regret of **0.00048** of the region's achievable range. Across all four
control-bearing families a control arm trades places in **4 of 4**, and the residual cost of holding the
committed section ranges from **0.045%** to **3.83%** slower. Section 4.

**Four. Both encodings compile, and the linker merged them.** The winner-table encoding and the
cost-table-plus-const-argmin encoding emit **byte-identical bodies**, to the point that the assembler
wrote `_e2_weighted = _e1_named`. A consumer's own weighting reaches an arm the named strategy does not
pick, in one tail branch. Zero feature gates, `no_std`, on the pin. Section 5.

**Five. The two proposals are not alternatives, and their composition is a compile-time assertion.**
Ship the weighting and the generated table, and assert at const time that the table is the argmin of the
weighting. Rationalisability is true by construction, which is `98`'s win; the check exists and is
mechanical, which is `97`'s; it catches the generator defects rationalisability cannot; it costs nothing
at runtime and refuses at build time, which is I15's shape. A mutant with one table entry changed fails to
compile with `E0080`. Section 6.

**Six, and it started as a cost of that assertion and ended as a defect in an estimator.** Stated as
equality the assertion false-alarms on a rerun in five of six configurations, up to 93.8%, so it wants a
band, and the bench measures the band's floor itself: the apparent gap between two byte-identical arms is
a median of 0.273%. For one weighting no band both absorbed the noise and refused the defects, which I was
going to report as a located cost. **It is not a cost, it is the interquartile range.** Estimating the
tail by the 99th percentile instead takes that weighting's section from 161 distinct to 2, its separated
arm pairs from 43 of 60 to 54 of 60, and its usable band window from empty to 0.5% through 10%.
Section 7.

**Seven, outside the question and the sharpest thing I found by accident.** Min-max normalising the cost
coordinates, which `98`'s p10 and my own p1 both do, breaks independence of irrelevant alternatives:
**an arm no weighting can ever select changes what every weighting selects**, at up to 6 of 6 regions.
Under raw coordinates it cannot, in 6 of 6 controls, and freezing the normalisation range as declared
constants restores it, also 6 of 6. Section 8.

**Eight. The one cost the encoding has is compile time, and it is unpriced because the instrument does
not exist.** The harness's CSV schema is entirely runtime measurements of an already-loaded cdylib and its
only interaction with the compiler is recording `rustc --version`; I checked rather than assumed. On the
two axes a spike can reach it costs nothing: identical emitted instructions, and **zero bytes**, with a
control showing the exact 240 bytes the cost table would occupy if it were not const-consumed. Section 9.

## 2. Attack one: does generating eliminate the check

`98` section 3 is the unit's strongest candidate and `99` says to attack it first. Its central claim:

> **So invert it. State the weighting; derive the table from it.** ... rationalisability is **true by
> construction** rather than a property to be verified, so there is nothing to check and nothing to
> police (`98:398-402`).

**The first clause is right. The second does not follow**, and the gap is not a quibble because the whole
argument for the inversion rests on it.

### 2.1 What generation actually removes and what it admits

Generating replaces a human writing a table with a tool computing one. It removes every defect a human
writing a table can introduce: a stale row, a misread findings file, a typo, a row nobody updated when the
arm set changed. That is real and it is what "true by construction" names.

It admits a defect class a human cannot produce: **the tool computes the wrong argmin.** And the crucial
property is that a tool's wrong argmin is, in general, the **right argmin of something else**. A
coordinate read in the wrong unit is the exact argmin of the correspondingly rescaled weighting. A column
swap is the exact argmin of the permuted one. So the output is still rationalisable, and the criterion
`97` built and `98` corrected to the strict rung is looking for exactly the property those defects
preserve.

`100_probes/p2_generation_relocates_the_check.py` measures it rather than arguing it. Five generators,
each carrying one defect somebody makes reading a CSV, and three detectors. The predictions were written
into the probe's docstring before it ran.

```
generic models, ties improbable: 200 models
  generator       differs  D1 caught  D1s caught  D2 caught  D3 caught
  G1 unit             190    0 (  0.0%)    0 (  0.0%)  190 (100.0%)  190 (100.0%)
  G2 swap             147    0 (  0.0%)    0 (  0.0%)  147 (100.0%)  147 (100.0%)
  G3 tiebreak           0    0   (n/a)    0   (n/a)    0   (n/a)    0   (n/a)
  G4 offbyone         200  200 (100.0%)  200 (100.0%)  200 (100.0%)  200 (100.0%)
  G5 dropped          152    0 (  0.0%)    0 (  0.0%)  152 (100.0%)  152 (100.0%)

models carrying a deliberate tie at one region: 200 models
  G3 tiebreak          48    0 (  0.0%)    0 (  0.0%)    0 (  0.0%)   48 (100.0%)
```

D1 is rationalisability, D1s the same at strictly positive weights, D2 membership of the **stated**
weighting in the section's feasible cone, D3 recomputation by an independent argmin.

**Rationalisability catches nothing on three of the four defect classes that change the answer.** It
catches the off-by-one, because a cyclically permuted assignment usually is not any weighting's argmin.

**Cone membership of the stated weighting catches every one of them** except the tie-break, which is not a
defect: at a tie both arms are weak argmins under the stated weighting and the weighting genuinely does
not choose. D3 reports the tie-break as a defect 48 of 48 times, which is a false positive, because two
tie-break policies disagree about a choice nobody made.

So the detectors are ordered, exactly:

- **D1 is nearly vacuous on generated output.** True by construction, and construction is what is
  suspect.
- **D3 is sound and not tie-break invariant.** It flags a disagreement about a choice the weighting
  declines to make.
- **D2 is the differential a generated table wants.** It is `97`'s decider, unchanged, in a different
  role: not a constraint on a human's table but the acceptance test on a tool's output.

**On the committed carrier table**, converted to exact rationals with the control arm dropped, the same
pattern holds at both weightings tried, and G3 is identical to correct because that table has no exact
ties among its five arms. I record that as a bound: the tie-break invariance of D2 is established on
constructed models and is not observed on the real table, because the real table has near-ties rather than
exact ones. Section 4 shows the near-ties are pervasive, and a near-tie becomes an exact tie the moment a
generator rounds or uses a robust summary.

### 2.2 What replaces "nothing to check"

**`98`'s proposal survives and its justification changes.** The reason to generate is not that there is
nothing left to check. It is that generation removes a defect class outright and leaves one that is
detectable by a decider the panel has already built twice, in exact arithmetic, at no compile-time cost.

That is a stronger position than the one `98` argued, because it does not depend on a claim that is false.
And it is the meta-compiler discipline rather than a departure from it: deriving every artifact from one
definition removes drift between the artifacts, and it does not remove the need to test the derivation.
Two tiers generated from one semantic definition still get a conformance suite, precisely because the
generator is now the only thing that can be wrong and there is no longer a twin to disagree with it.

**F-100-1. Rationalisability is not a detector of a generator defect. A generator reading a coordinate in
the wrong unit, in the wrong order, or not at all emits a section that is rationalisable at both the
non-negative and the strictly positive rung, in 0 of 489 opportunities to be caught. Membership of the
stated weighting in the section's feasible cone catches all 489, and is invariant to tie-break policy
where recomputation is not.**
`holds for: regions = 5, arms = 5, cost coordinates = 3, 400 random models across two families, plus the
committed bitpack-carrier-width_n* table at 6 regions, 5 arms, 3 coordinates with the noise-floor control
dropped, arithmetic exact rational, defect classes {unit scale, coordinate swap, tie-break policy, region
off-by-one, dropped coordinate}, threads = 1, target features any`
Evidence: `100_probes/p2_generation_relocates_the_check.py`, `100_probes/cone.py`.

## 3. Attack two: what the strictly-positive rung is a detector for

`98`'s correction to `97` is that the figure carrying the no-dominated-arm guarantee is 9 rather than 72,
because a zero weight admits an arm no strictly positive weighting can reach, and 63 of the 72 do. I
reproduced neither count and I did not need to: `98` reproduced `97`'s two figures exactly from an
independent implementation, which is the strongest corroboration this panel can produce, and a third
implementation adds nothing. What I attacked instead is what the rung is **good for**.

**I predicted, in p2's docstring before running, that the strict rung would catch a generator that drops a
coordinate**, on the reasoning that a dropped coordinate is a zero weight and `98` establishes that a zero
weight is what admits an unreachable arm. **The prediction was wrong: 0 of 152 and 0 of 160.**

`100_probes/p2b_why_the_strict_rung_caught_nothing.py` chases the mechanism, which is the interesting
part.

```
  independent          differs 230   strict rung caught   0 (0.0%)   selects a dominated arm   0 (0.0%)
  shared-coord         differs 241   strict rung caught   0 (0.0%)   selects a dominated arm   0 (0.0%)
  duplicate            differs 234   strict rung caught   0 (0.0%)   selects a dominated arm   0 (0.0%)
  tied-except-dropped  differs 285   strict rung caught 230 (80.7%)  selects a dominated arm 230 (80.7%)
```

**The two columns are identical in every family.** That is what "detector for exactly one event" looks
like when it is measured rather than asserted: the rung fires precisely when the section selects an arm
Pareto-dominated on the full coordinate set, and never otherwise.

The mechanism, and the hypothesis was stated before the sweep. A zero weight sits on the boundary of the
**weight simplex**, not on the boundary of the **admitting cone**. As long as every difference-vector
inequality holds strictly, a small positive weight preserves them all, so the zero-weight section is also
strictly rationalisable and the rung sees nothing. The inequalities go tight, and the hazard becomes real,
exactly when two arms carry the same value on a coordinate.

And the loose form of that hypothesis is refuted by its own sweep. Arms merely **sharing** one coordinate
do not suffice, 0 of 241. Two **identical** arms do not suffice, 0 of 234, because identical arms do not
dominate each other. What is needed is exact: **a pair tied on every coordinate the effective weighting
reads and differing on one it does not.**

That shape is not hypothetical. It is what `bitpack-carrier-packed` and `bitpack-carrier-packed-simd` are:
both decode the same 13-bit column, so they carry identical bits per element and differ only in time. A
zero weight on **time** cannot tell them apart, which is `97`'s own reconciliation of 9 with 72 and `98`'s
63 of 72, arrived at here from the other end.

**So Q44's answer is (a), require strict positivity, and its predicate is sharper than the option states.**
Strict positivity buys the no-dominated-arm guarantee; the guarantee is reachable exactly where the arm
set contains such a pair, which is a property of the arm set checkable directly and once, before any
weighting exists; and it buys nothing whatever against a generator.

That last clause matters for how Q43 and Q44 compose. **They are independent.** A canon requiring strict
positivity has not thereby acquired any protection against the generation order's defect class, and a
design that generates has not thereby acquired the no-dominated-arm guarantee. Two constraints, two
predicates, no overlap.

**F-100-2. The strictly-positive rationalisability rung is a detector for exactly one event, a section
selecting a Pareto-dominated arm, and for no other. It is reachable only where the arm set contains a pair
of arms tied on every coordinate the effective weighting reads and differing on one it does not: 0 of 230
on independently drawn arms, 0 of 241 on arms sharing one coordinate, 0 of 234 on identical arms, 230 of
285 on that exact shape.**
`holds for: regions = 5, arms = 5, cost coordinates = 3, 300 models per family across four families,
arithmetic exact rational, threads = 1, target features any`
Evidence: `100_probes/p2b_why_the_strict_rung_caught_nothing.py`. The two measured columns coincide in
all four families.

## 4. Attack three: is the section actually unstable

`98` section 3's motivation is a measurement, F-98-10, and the checkpoint carries it forward as the
unit's reason to change shape: "a section is not stable across a rerun of the same bench on the same
afternoon" (`99:60-63`).

**The measurement reproduces. The inference does not.**

### 4.1 The distinct-section count is a product over regions

`98` reports 30, 8 and 77 distinct sections and modal rates of 26.6%, 44.0% and 12.2%. I reproduce 31, 8
and 83 with an independently written extraction and bootstrap, with 27.2%, 44.8% and 12.4%. The small
differences are the RNG stream, since I draw arms in sorted order and `98` in CSV order, which is worth
one sentence on its own: **the distinct-section count is itself unstable to the bootstrap's stream**, 30
against 31 and 77 against 83, so it is being read to more precision than it carries.

The statistic compounds. If each of six regions holds its pick with probability `p` independently, the
whole section reproduces with probability `p^6`. At `p = 0.95` that is 0.74; at `p = 0.80` it is 0.26. **A
26.6% modal rate is what roughly 80% per-region stability looks like**, and reporting it as a fact about
the table folds the region count into the number.

### 4.2 Every speed-first flip is an arm against its own byte-identical twin

This is the finding. `98`'s own p10 output already contains it and reads past it
(`98_probes/p10_is_the_table_stable_enough_to_be_an_object.out:8-13`): at speed-first, the five regions
where the pick moves are, every one of them, `d16` against `d16-control`.

`bitpack-carrier-d16-control` is a noise-floor control arm. Its own header says the two "differ only in
the exported symbol name and must compile to the same machine code", that "any measured gap between them
is the harness's own resolution on this workload", and that the byte-identity is checked rather than
assumed by a script that diffs the two built dylibs
(`mock/benches/variants/bitpack-carrier-d16-control/src/lib.rs:1-8`).

So the argmin is being asked to choose between two copies of one arm, and the thing it chooses on is the
quantity the bench declares to be its own measurement error. `100_probes/p1_what_the_instability_is_made_of.py`
drops it:

```
ARM SET: five, noise-floor control dropped

speed-first
  committed section : ['packed-simd', 'd16', 'd16', 'd16', 'd16', 'd16']
  distinct sections : 1
  modal appears     : 2000/2000 (100.0%)  == committed
  per region:  regret = (held - best) / (worst - best), 0 is best
    n=   16384  flip   0.0%  ...  n= 8388608  flip   0.0%
```

**One distinct section, 2000 of 2000, zero flips at every region.** Where `98` reports thirty distinct
sections and a 26.6% modal rate, the five-arm table is perfectly stable. Tail-first falls from 83 distinct
sections to 40. Storage-first is unchanged at 8, and its flips are between `packed` and `packed-simd`,
which are exactly tied on the coordinate storage-first names as dominant, so the choice is being made by
the two coordinates it weights at 1/32.

`98` knew the sixth arm is the control: its p13 reports "including the control arm takes L0 from 15625 to
46656 and leaves L1, L3 and L4 at 144, 72 and 9 unchanged" (`98:285-286`). The connection to the
instability result was not made, and it is the whole of the speed-first result.

### 4.3 And where the flips are real, they are free

A flip statistic says a pick moved. It does not say moving cost anything. The scale-free way to ask is
where the held choice falls in the region's achievable objective range, `(held - best) / (worst - best)`,
zero being the best arm available and one the worst.

```
speed-first, six arms:      regret mean 0.00048   max 0.01825
storage-first, six arms:    regret mean 0.00001   max 0.00077
tail-first, six arms:       regret mean 0.00295   max 0.07786
```

Across every region and all 2000 resamples, holding the committed section costs a mean of five parts in
ten thousand of the achievable range, and never more than 7.8% of it in the single worst case on the
weighting whose coordinate `98`'s own p12 found unresolvable at the comparisons that mattered.

**A defect in the first version of this probe, kept rather than deleted.** It reported regret as
`(held - best) / best` on the min-max normalised objective, which is meaningless: min-max normalisation
puts the origin at the global minimum, so a region whose best arm scores near zero produces a near-zero
denominator and a regret in the thousands of percent. It reported a maximum of 5773%. The output is at
`100_probes/p1_v1_relative_regret_is_meaningless.out` and the probe's docstring says what replaced it and
why. Differences of a min-max normalised objective are meaningful and levels are not.

### 4.4 And then the finding met its own boundary, on three more families

One family is one instance and `RULES.md` puts the bar at three, so I went looking for the effect
elsewhere before resting anything on it. Three more committed families declare a byte-identical
noise-floor control arm in their own words, and the four appear across `bitpack-carrier-width`,
`bitpack-contention`, `bitpack-contend-decode` and `bitpack-wide`.

`100_probes/p7_the_control_arm_across_every_family_that_has_one.py` runs the same decomposition on all
four, deliberately on **one** cost coordinate, the median `algo_ns`. With one coordinate the weighting
drops out entirely: the section is just the per-region fastest arm, so every distinct section beyond the
first is measurement noise and nothing else. That also makes regret directly interpretable as how much
slower the held section runs.

```
  family                      distinct with  distinct without   mean regret   worst
  bitpack-carrier-width                  29                 3       0.045%    3.23%
  bitpack-contend-decode                202               160       3.832%  264.42%
  bitpack-contention                    987               136       1.080%   31.52%
  bitpack-wide                           14                10       0.154%   21.75%
```

**The mechanism generalises and the conclusion does not, and the split is the finding.**

**A control arm trades places in 4 of 4 families.** That is the part that holds: wherever a bench declares
two arms byte-identical and both are competitive, the argmin spends part of its time choosing between
them, and what it chooses on is the quantity the bench declares to be its own error.

**Dropping the control stabilises exactly one family.** Carrier falls from 29 distinct to 3 with a 62.5%
modal rate. Contention falls 987 to 136 and is still unstable. Decode falls 202 to 160. Wide barely moves.

**And the residual instability is free on two families and not on two.** Carrier costs 0.045% and wide
0.154%, which is nothing. Contention costs 1.08% and decode costs 3.83% on average with a worst resample
at 264%, which is not nothing.

**So `98`'s inference is right about some of this corpus.** "A section is not stable enough to be the
object a check is applied to" is unsupported on the family `98` measured, where the instability is an
instrument artifact costing four hundredths of a percent, and it is supported on `bitpack-contend-decode`,
where real arms trade places at a real cost. `98` reached a true conclusion from a family that does not
support it, which is a different thing from reaching a false one, and the correction is to attach the
predicate rather than to withdraw the claim.

**The predicate, stated as one.** The section is stable enough to be checked where the arms competing at a
region are separated by more than the harness's own resolution, and not otherwise. That resolution is
measured, per family, by the control pair, and section 7.1 reads it off. So this is not a judgement call:
it is a number the corpus already carries, per family, and the design can gate on it.

### 4.5 What survives of `98` section 3's motivation

**What survives, and more of it than I expected when I started.** The section is a measured artifact, it
moves, on two of four families it moves at a cost, and a canon sentence naming it would be wrong by the
following week. `40` section 3.2's permanence argument and `98`'s sharpening of it both stand, and nothing
here argues the section belongs in the canon.

**What changes is the reach.** "A section is not stable" is a per-family fact with a measurable
discriminator, not a property of sections, and on the one family it was measured on it is mostly the
instrument. Both halves of that sentence are needed and neither alone is honest.

**And the contending-set instrument does not rescue it either, which refutes my own first hypothesis.**
`97` section 5 used contending sets at a tolerance rather than a strict argmin, on the ground that a
strict argmin over near-ties measures noise, and I expected that instrument to make the section a stable
object. It does not do so cleanly. p1 measures set stability at four tolerances and the result is
non-monotone: speed-first is 100% stable at a 5% band and 23.4% at a 10% band, because the set's
**membership boundary** is itself noisy and a tolerance landing near an arm's position flips membership as
readily as the argmin flips. The tolerance set helps where a band brackets a gap and not otherwise.

**F-100-3. On the committed carrier run, the section's instability under resampling is carried by the
noise-floor control arm and by pairs tied on the weighted coordinate. Dropping the control takes
speed-first from 31 distinct sections across 2000 resamples to 1 with zero per-region flips, and
tail-first from 83 to 40. Holding the committed section rather than each resample's own argmin costs a
mean of 0.00048 and a maximum of 0.018 of the region's achievable objective range.**
`holds for: regions = 6, arms in {5, 6}, cost coordinates = 3 (median algo_ns, declared bytes per element,
interquartile range), cost source = committed bitpack-carrier-width_n* CSVs, 80 samples per arm per
region, 2000 bootstrap resamples, seed 20260814, weightings as instantiated in the probe, host = the one
those runs were taken on, threads = 1`
Evidence: `100_probes/p1_what_the_instability_is_made_of.py`. An uncertainty estimate over a committed
artifact, not a bench; no measurement was taken.

**F-100-3b. Across every committed family declaring a byte-identical noise-floor control arm, that arm is
one of the arms trading places under resampling, in 4 of 4. Dropping it reduces the distinct-section count
in 4 of 4, from 29 to 3, 202 to 160, 987 to 136 and 14 to 10. The residual cost of holding the committed
section rather than each resample's own fastest arm is 0.045%, 3.832%, 1.080% and 0.154% of runtime.**
`holds for: families {bitpack-carrier-width, bitpack-contend-decode, bitpack-contention, bitpack-wide},
regions in {6, 12}, arms in {4, 5, 6}, cost coordinates = 1 (median algo_ns), 80 samples per arm per
region, 2000 bootstrap resamples, seed 20260814, host = the one those runs were taken on, threads = 1`
Evidence: `100_probes/p7_the_control_arm_across_every_family_that_has_one.py`. An uncertainty estimate
over committed artifacts, not a bench; no measurement was taken.

## 5. The fork that has consumer-visible content, and it is not Q43's

`93` named a fork in its own phase-two withdrawal and said the register did not carry it (`93:966-973`):

> **Two encodings exist and they are not equivalent in what they let a consumer do.** `94`'s W1 bakes the
> *winner* per region as an associated const ... My P4 bakes the *cost table* and computes an argmin at
> const time, which also erases completely. The difference is what a consumer can bring: under the second,
> a weighting nobody named selects an arm nobody tabulated for it.

`98` section 3.1 identifies this with check-against-generate and cites `93`'s P4 as evidence that both
sides compile. **Two problems, and the second is the substantive one.**

### 5.1 `93`'s P4 does not establish the half it is cited for

`93`'s P4 declares `const ARM_COST: [[u32; AXES]; ARMS]`, a cost per arm with **no region dimension**
(`93_probes/p4_preference_erases.rs:47`). Its const argmin runs once over three arms and is a constant
fold with nothing region-shaped in it. The encoding the fork is about indexes cost **by region**, with the
region as a const generic parameter, so the argmin runs per monomorphisation. That is a different
compilation question and P4 does not answer it.

This is not a criticism of P4, which established what it set out to and says so. It is a citation that
reaches one step further than its source.

### 5.2 The two generate-mechanisms are different mechanisms, and `98` runs them together

`98` section 3 argues for generation on stability grounds and then clarifies in 3.2 that generation is
offline: "The generation happens once, by a tool, from a stated weighting and a committed cost table, and
its output is committed beside them" (`98:473-477`).

Under that reading **the compiler is handed a winner table**, identical in shape to a hand-written one,
and a consumer cannot bring a weighting. Under `93`'s P4 reading the compiler is handed a **cost table**
and a consumer can. Those have different consumer-facing consequences, and `98`'s section 3.1 cites
evidence for the second while its section 3.2 describes the first.

**So Q43's (a) and (b) differ in nothing a consumer can observe.** Both emit a winner table; one has a
human write it and a check confirm it, the other has a tool write it. That is a maintainer workflow
question, and it is real, and it is small.

**The axis with consumer-visible content is what reaches the compiler**, and on op's own priority axis it
is the one that matters: `arvo-toolbox-not-policer.md` says the substrate ships the choice rather than
making it, and I11 says the value is what composes on top.

### 5.3 Both encodings compiled, and the linker merged them

`100_probes/p3_three_encodings.rs` compiles four entry points at `no_std`, zero feature gates, no `dyn`,
no `TypeId`, no `generic_const_exprs`, on `nightly-2026-05-28`, with the selection forced through an
inline `const { }` block so the claim is about const solving rather than about the backend folding a const
fn call.

```
_e1_named:
	b	__RNvCs7jv63BkYSe1_18p3_three_encodings4arm0

_e3_direct:
	b	__RNvCs7jv63BkYSe1_18p3_three_encodings4arm4

_e4_consumer:
	b	__RNvCs7jv63BkYSe1_18p3_three_encodings4arm4

	.globl	_e2_weighted
_e2_weighted = _e1_named
```

**`_e2_weighted = _e1_named`.** The cost-table encoding with a region-indexed const argmin and the
winner-table encoding produced bodies identical enough that the assembler emitted an alias rather than a
second function. Not "the same number of instructions", not "equivalent": the same symbol.

**`e4_consumer` is the expressiveness claim made concrete.** It is a weighting nobody tabulated, supplied
the way a consumer supplies one, and it reaches `arm4` at a region where the named strategy reaches
`arm0`, in one tail branch, exactly as `e3_direct` reaches `arm4` by calling it directly. A compile-time
assertion pins that it resolves differently from the named strategy, so the comparison cannot go vacuous:
removing that assertion and re-pointing the consumer weighting at the same arm was the first version's
failure and the assertion fired on it.

**So the cost-table encoding costs nothing at the point of use and buys a consumer a weighting nobody
named.** On the emitted-code axis the fork is decided. What remains is compile time, and section 9 says
what is and is not known about it.

**F-100-4. A region-indexed cost table resolved by a const argmin over a stated weighting emits a body
byte-identical to a winner-table lookup, to the point of being emitted as a symbol alias, and a weighting
supplied by a consumer reaches an arm the named strategy does not select at that region in one tail
branch.**
`holds for: regions = 6, arms = 5, cost coordinates = 2, target = aarch64-apple-darwin, rustc
1.98.0-nightly (57d06900f), edition 2024, opt-level 3, panic = abort, feature gates = 0, no_std,
threads any (a compile-time artifact), target features baseline`
Evidence: `100_probes/p3_three_encodings.rs` and `100_probes/p3_run.sh`, with
`100_probes/p3_mutant_generator_bug.rs` as the negative control. This is a shape, not a price: the
compile-time cost is unpriced and is said to be unpriced.

## 6. What replaces the fork: the differential is a compile-time assertion

`95` asks a unit to end in agreement with something. This is where I think `97` and `98` agree and neither
saw it, and it falls out of the two previous sections rather than being proposed on taste.

`97` wants the table constrained by a check. `98` wants it generated so no check is needed. Section 2 says
the generated table needs a check anyway, on the generator. Section 5 says the encoding that costs nothing
carries the cost table into the compiler. Put those together and **the check is an assertion over two
artifacts that are both already present**:

> the committed winner table equals the argmin of the stated weighting over the committed cost table, at
> every region.

`p3` compiles exactly that, as a `const` item that loops over the regions. It costs one const evaluation,
nothing at runtime, and it **refuses at build time** rather than reporting, which is the shape I15 asks
for: "We catch invalids on compile time, and unused paths we clear out when lowered"
(`INTENTS.md:290-292`).

`100_probes/p3_mutant_generator_bug.rs` changes one entry of the winner table, which is the defect class
p2 measures as G1 and G2, and it must fail:

```
error[E0080]: evaluation panicked: the committed winner table disagrees with the argmin of the
weighting that is supposed to have generated it
   --> p3_mutant_generator_bug.rs:169:9
```

A check nobody has seen fail is not a check, and this one has been seen to fail twice: once as a mutant
and once accidentally, when the first version of `p3` carried a hand-written winner table computed wrong,
and the assertion caught it before the file compiled at all.

**Why this is the composition rather than a compromise.** Under it:

- rationalisability is true by construction, which is exactly `98`'s win and it is kept whole;
- the check exists, is mechanical, and runs where the artifact is written, which is `97`'s;
- it catches the generator defect class rationalisability is blind to, which section 2 measures;
- the winner table stays available for the strategies the library names, so nothing about the named
  strategies' compile time changes;
- the cost table stays available for a consumer's own weighting, which is the toolbox posture.

**And there is an asymmetry worth naming, because it is the reason a generator is safer here than
elsewhere.** Under the winner-table encoding the generator is an offline tool run once per table, and its
output is a separate artifact that can drift from its input. Under the cost-table encoding the generator
is a `const fn` in the library, shared by every strategy and every consumer weighting, so there is exactly
one of it and the named strategies' compile-time assertions are its conformance suite. A consumer's own
weighting goes through the same function the shipped strategies already assert against. **One definition,
every artifact derived from it, and the derivation tested by the artifacts that have a known answer.**

### 6.1 The same thing as arms with predicates, which is the form I13 asks for

Written as one recommendation it reads like a single answer, which is the shape op has rejected. It is
four arms with four predicates, and each applies on its own region and nowhere else.

**Arm A, generate and assert.** Ship the weighting, the cost table and the generated winner table, with a
const assertion that the third is the argmin of the first over the second. `holds where every coordinate
the strategy weighs has a measurable value in the corpus, and the arms competing at each region are
separated by more than the harness's own resolution there`. That predicate is checkable before the arm is
built, against the control pair, per section 7.1.

**Arm B, check without generating.** Ship the table and check it offline. `holds where a coordinate the
strategy weighs has no measurable value, so no weight vector can be written`. That is `98`'s own reason for
calling its proposal the target and `97`'s the interim, and section 5 of `98` says two of op's four intents
are in exactly this region today.

**Arm C, the band rather than the equality.** State the differential with a tolerance. `holds where the
region's competing arms are separated by less than the coordinate's resolution but more than zero`, which
is the near-tie case section 7 measures, and the band's floor is the control pair's gap.

**Arm D, no differential at all.** `holds where the coordinate's resolution does not separate the arms it
must distinguish`, which section 7.3 shows is a property of the estimator rather than of the coordinate,
and which therefore usually calls for changing the statistic before accepting the arm.

The four compose because their predicates are disjoint and each is a const-checkable property of the
corpus, not a judgement.

This is what I would offer the consolidation, as a suggestion.

## 7. The cost of that assertion, which looked like a tension and was an estimator

Stated as **equality**, the assertion has a defect section 4 predicts: at a region where two arms sit a
hair apart, the argmin moves under the run's own noise, so regenerating the table from a fresh measurement
changes the committed winner and the assertion refuses a design nobody changed.

`100_probes/p4_the_differential_wants_a_band.py` measures it over all six weighting-by-arm-set
combinations. **The first version of that probe ran one combination, got 0%, and would have reported the
concern refuted on a setup that could not have shown it**, which is `the-test-gate.md`'s "setup that
helps" committed by me; the fix was the whole matrix and the docstring records it.

```
  arm set: six arms
    weighting         0.0%    0.5%    1.0%    2.0%    5.0%   10.0%
    speed-first      72.8%   16.8%    3.5%    0.0%    0.0%    0.0%
    storage-first    55.2%    0.0%    0.0%    0.0%    0.0%    0.0%
    tail-first       93.8%   71.5%   52.1%   25.2%    0.5%    0.0%

  arm set: five, control dropped
    speed-first       0.0%    0.0%    0.0%    0.0%    0.0%    0.0%
    storage-first    52.8%    0.0%    0.0%    0.0%    0.0%    0.0%
    tail-first       85.5%   43.1%   17.9%    3.0%    0.0%    0.0%
```

The `0.0%` column is the equality form. It false-alarms on five of six configurations, up to 93.8%. **So
the equality assertion is the wrong form and a band is the right one**, except on the one configuration
section 4 shows is perfectly stable, where equality is free.

### 7.1 The bench measures the band's floor itself

How wide is not a matter of taste, and the answer is sitting in every committed run. The noise-floor
control pair is a **calibration**: two arms the bench guarantees are byte-identical, so any apparent gap
between them is the instrument.

```
     region    d16 median  control median        gap
      16384        1401.5          1401.0     0.032%
     131072       11250.6         11221.5     0.260%
    1048576       90231.5         90632.1     0.444%
    2097152      183275.6        183746.0     0.257%
    4194304      372072.2        374096.7     0.544%
    8388608      735425.2        733319.8     0.287%

  apparent difference between two byte-identical arms: median 0.273%, max 0.544%
```

Below that, a differential is asserting a difference the instrument cannot see. **Nobody in this unit has
read that pair as a number**, and it is the natural floor for any tolerance the design states.

### 7.2 And for one weighting the band and the detection window do not both fit

Detection, on raw coordinates, at the same bands:

```
    speed-first     G1 unit       refused up to band 2.0%
    speed-first     G2 swap       refused up to band 2.0%
    speed-first     G4 offbyone   refused up to band 5.0%
    storage-first   G1 unit       refused up to band 10.0%
    storage-first   G2 swap       refused up to band 1.0%
    storage-first   G4 offbyone   refused up to band 1.0%
    tail-first      G1 unit       refused up to band 2.0%
    tail-first      G2 swap       emits the CORRECT table, so nothing can catch it
    tail-first      G4 offbyone   refused up to band 10.0%
```

For **speed-first** a 2% band gives 0% false alarms on both arm sets and still refuses all three defects.
For **storage-first** a 0.5% band gives 0% false alarms and refuses all three. For **tail-first** the
false alarm rate reaches 0% only at 5%, and G1 stops being refused above 2%. **There is no band that
does both.**

That is a located cost and I state it as one rather than choosing a number. It is also not a surprise:
tail-first weighs the interquartile spread, and `98`'s own F-98-12 established that the spread differences
at the comparisons that mattered are not distinguishable from zero. **A coordinate the instrument cannot
resolve cannot support a differential**, and that is a precondition on the coordinate rather than a defect
in the mechanism.

So the arm is predicated: **the differential is available exactly where the coordinate's measurement
resolution is finer than the separation between the arms it must distinguish.** That predicate is
checkable before anything is built, against the control pair, which is the same instrument that sets the
band.

### 7.3 And then the tension turned out to be the estimator, not the coordinate

That is where I would have left it, and leaving it there would have been wrong. The predicate above is
about a coordinate's resolution, and a coordinate's resolution is a property of the **statistic used to
estimate it**, which nobody in this unit had varied.

Every negative finding about the tail coordinate in this unit is measured on one estimator: the
**interquartile range** of the per-batch samples. `98` p10 finds the tail-weighing section the least
stable of three; `98`'s F-98-12 finds its differences not distinguishable from zero at the comparisons
that mattered; section 7.2 above finds no workable band. Three findings, one statistic.

**The interquartile range is the spread of the middle fifty percent. It is, by construction, the
statistic with both tails removed.** A strategy weighing tail behaviour and reading the IQR is reading the
middle. It is also a *difference* of two order statistics, so its sampling variance is roughly the sum of
theirs while its magnitude is the gap between them: high noise, small signal, both from the same
construction.

`100_probes/p8_the_tail_coordinate_is_the_wrong_statistic.py` runs six estimators of the same intent on
the same committed samples.

```
  estimator   distinct            modal    arm pairs separated
  iqr              161     270/2000 ( 13.5%)           43 of 60     ( 71.7%)
  idr               51     389/2000 ( 19.4%)           48 of 60     ( 80.0%)
  mad              264     139/2000 (  7.0%)           36 of 60     ( 60.0%)
  p95                3    1980/2000 ( 99.0%)           54 of 60     ( 90.0%)
  p99                2    1781/2000 ( 89.0%)           54 of 60     ( 90.0%)
  trimsd            46     798/2000 ( 39.9%)           49 of 60     ( 81.7%)
```

**The 95th percentile gives 3 distinct sections against the interquartile range's 161, and separates more
arm pairs while doing it: 54 of 60 against 43 of 60.** Not a trade. Strictly better on both axes, from
reading the tail instead of the middle.

And the band tension does not survive the swap. Section 7.2's false alarm sweep, rerun with each
statistic in the third coordinate:

```
    tail statistic     0.0%    0.5%    1.0%    2.0%    5.0%   10.0%
    iqr               65.1%   22.9%   13.2%    3.0%    0.1%    0.0%
    p95               15.8%    0.0%    0.0%    0.0%    0.0%    0.0%
    p99                0.5%    0.0%    0.0%    0.0%    0.0%    0.0%

  DETECTION: widest band at which each defect is still refused
    tail statistic      G1 unit    G2 swap   G4 offbyone
    iqr               no change  no change          5.0%
    p95               no change  no change         10.0%
    p99               no change  no change         10.0%
```

**Under the interquartile range the window is empty**: 5% is needed before the false alarms stop and 5% is
where detection stops. **Under the 99th percentile the window runs from 0.5% to 10%**, and the equality
form itself false-alarms on only 0.5% of regenerations.

**A bound on that detection column, stated because it is real.** G1 and G2 read "no change" for every
estimator because the table is min-max normalised and section 8 explains why: normalisation is scale
invariant, so a unit error emits the identical table. So the detection half here is exercised by the
off-by-one alone, and the unit and swap defects are exercised in section 7.2's raw-coordinate run rather
than in this one.

**So section 7.2's tension was a fact about the interquartile range and not about tail behaviour**, and
`98`'s F-98-12 wants re-reading the same way: the corpus does resolve the tail, at 54 of 60 arm pairs. It
does not resolve the IQR. That is the difference between a coordinate a design cannot have and a
coordinate somebody estimated with the wrong statistic, and I would not have found it if section 7.2 had
been allowed to stand as a cost.

**F-100-7. On the committed carrier samples, estimating the tail coordinate by the 95th or 99th percentile
rather than by the interquartile range reduces the section's distinct-section count from 161 to 3 or 2
across 2000 resamples, raises the fraction of arm pairs separated by a bootstrap interval excluding zero
from 43 of 60 to 54 of 60, and takes the differential's false alarm rate at a 0.5% band from 22.9% to
0.0% while widening the band at which the off-by-one defect is still refused from 5% to 10%.**
`holds for: regions = 6, arms = 5, cost coordinates in {1, 3}, estimators {P75-P25, P90-P10, median
absolute deviation, P95, P99, 10% trimmed standard deviation}, cost source = committed
bitpack-carrier-width_n* CSVs, 80 samples per arm per region, 2000 bootstrap resamples, 2000 CI resamples,
seed 20260814, noise-floor control excluded, host = the one those runs were taken on, threads = 1`
Evidence: `100_probes/p8_the_tail_coordinate_is_the_wrong_statistic.py`. An uncertainty estimate over a
committed artifact, not a bench; no measurement was taken.

**F-100-5. The compile-time differential stated as equality refuses a design nobody changed on 5 of 6
weighting-by-arm-set combinations, up to 93.8% of regenerations. Stated as a band it does not, and the
band that gives a 0% false alarm rate still refuses every measured generator defect for two of the three
weightings and not for the third. The apparent gap between two arms the bench declares byte-identical is a
median of 0.273% and a maximum of 0.544%.**
`holds for: regions = 6, arms in {5, 6}, cost coordinates = 3, cost source = committed
bitpack-carrier-width_n* CSVs, 80 samples per arm per region, 2000 bootstrap resamples, seed 20260814,
bands in {0, 0.5, 1, 2, 5, 10}% of the region's achievable objective range, defect classes {unit scale,
coordinate swap, region off-by-one}, host = the one those runs were taken on, threads = 1`
Evidence: `100_probes/p4_the_differential_wants_a_band.py`.

## 8. Outside my question, and it bears on every number in the unit

Both `98`'s p10 and my own p1 min-max normalise each cost coordinate over the whole table before applying
the weighting, so that nanoseconds and bytes can be added. It is the obvious move. It has a consequence
nobody in the unit has named, and it is not about noise.

**Min-max normalisation reads its scale off the arm set.** Add an arm, and a coordinate's range can widen;
widen it, and every other arm's normalised value on that coordinate shrinks; shrink it, and the effective
weight on that coordinate falls. So the argmin at a region can move because of an arm that is not, and
could never be, the argmin anywhere.

`100_probes/p4b_normalisation_breaks_independence.py` measures it on the committed table, with the
raw-coordinate case as a control that must show nothing moving.

```
MIN-MAX NORMALISED
  speed-first     A. 1 dominated arm dropped   MOVED     B. unselectable arm added   MOVED
  storage-first   A. 1 dominated arm dropped   MOVED     B. unselectable arm added   MOVED
  tail-first      A. 1 dominated arm dropped   SAME      B. unselectable arm added   SAME

RAW COORDINATES
  speed-first     A. SAME   B. SAME
  storage-first   A. SAME   B. SAME
  tail-first      A. SAME   B. SAME

D. sweeping the unselectable arm's extremity, min-max normalised
      factor  regions whose pick moved, per weighting
        2.00  speed-first 0/6   storage-first 2/6   tail-first 0/6
       32.00  speed-first 4/6   storage-first 3/6   tail-first 0/6
      128.00  speed-first 6/6   storage-first 3/6   tail-first 0/6
```

The added arm is strictly worse than every real arm on every coordinate at every region, and the probe
asserts at every factor that it was never selected. **At factor 128 it changes speed-first's pick at all
six regions without ever being picked itself.**

The control holds: raw coordinates, 6 of 6 SAME, which is the theorem, since an argmin is unaffected by an
alternative that is never the minimum.

**A defect in the first version of this probe, kept.** It dropped the two arms `97` reports as dominated
everywhere on its **two-coordinate** model. On the three-coordinate model used here only
`bitpack-carrier-d64` is, so it removed an arm that is genuinely selected and made its own raw-coordinate
control fire. The dominated set is now computed from the model in use, and the probe's comment records
what happened. That is the same class as `97`'s F11 correction: a finding whose predicate names two
coordinates being applied at three.

**And the remedy is tested rather than asserted, which is the third block in that probe.** With the
normalisation range **frozen as declared constants** instead of read off the arm set, dropping the
dominated arm and adding the unselectable one both leave every section untouched, **6 of 6**, exactly as
the raw control does. The mechanism is why: a fixed affine transform per coordinate turns the weighted sum
into a fixed positively-weighted linear functional of the raw costs, and an argmin of that cannot be moved
by an alternative that is never the minimum.

**Two consequences, and neither is small.**

A design shipping normalised costs has to state the normalisation range as **declared constants**, because
it is part of the semantics rather than a presentation detail. What that costs is a number somebody has to
get right once, and a range far from the data's makes the weights mean something other than intended. What
it buys is independence, measured above.

And a bench arm added as a negative control, which is measurement hygiene and exactly what
`bitpack-carrier-d16-control` is for, could then change what every strategy selects. **That is a coupling
between the instrument and the answer**, and it is worth knowing before anything is built on a normalised
cost table.

**F-100-6. Under min-max normalisation whose range is read off the arm set, adding an arm that is
strictly dominated at every region and therefore selectable by no weighting changes the section at up to
6 of 6 regions, and dropping the arm dominated in every region changes it for 2 of 3 weightings. Under raw
coordinates neither changes anything, in 6 of 6 controls, and under normalisation with the range frozen as
declared constants neither changes anything, in 6 of 6.**
`holds for: regions = 6, arms in {4, 5, 6}, cost coordinates = 3, cost source = committed
bitpack-carrier-width_n* CSVs with the noise-floor control excluded, synthetic arm extremity factors in
{1.01, 1.5, 2, 4, 8, 32, 128}, weightings as instantiated in the probe, threads = 1, target features any`
Evidence: `100_probes/p4b_normalisation_breaks_independence.py`.

## 9. Compile time, which is the one cost the encoding actually has

`arvo-compile-time-last.md` puts compile time last among the costs to minimise and explicitly licenses
paying it for a runtime or correctness win. That is the right frame here and it is not a licence to skip
the question.

**Under `98`'s offline reading the answer is exactly zero**, because the compiler never sees the
generator: it sees a winner table, the same one a person would have written. Nothing to price.

**Under the cost-table encoding there are two distinct const-evaluation loads and they scale
differently**, which is the part worth carrying:

- **Per call site**, `resolve(S::W, I)` inside an inline `const { }` block runs once per monomorphisation
  and costs `A * D` steps. **It does not depend on the region count.**
- **Once per crate**, the differential runs `R * A * D` steps.

So a design carrying many regions pays the region dimension once at the check and never at a call site,
which is the opposite of what "the cost table is bigger" suggests.

**Whether it compiles at all, at sizes a real design would carry**, is an existence question an ad-hoc
spike may answer, and `100_probes/p3b_const_eval_scaling.py` answers it:

```
     R     A    D     cells  agreement steps  result
     6     5    2        60               60  compiles
    64    16    4      4096             4096  compiles
   256    64    4     65536            65536  compiles
  1024    64    4    262144           262144  compiles
```

**No const-evaluation limit is reached at 1024 regions by 64 arms by 4 coordinates**, with the full
differential running at const time. The wall is not near, and `long_running_const_eval` remains a
documented lever nobody has had to reach for.

**The compile-time cost of any of those shapes is UNPRICED and I use that word deliberately.** A
compile-time figure taken outside `mock/benches/` is an ad-hoc quick spike with no substance for a
how-much question, and I checked rather than assumed that the harness cannot price it: its CSV schema is
`run,pass,cooldown_ms,mode,variant,batch_idx,e2e_ns,algo_ns,bridge_ns,batch_count,score,input_tag,instructions,cycles,setup_ns,first_ns,digest`
(`bench-harness/src/harness.rs:752` in the pinned checkout), every field of which is a runtime measurement
of an already-loaded cdylib, and the only thing it does with the compiler is record `rustc --version` as
provenance (`bench-harness/src/env.rs:105`). **There is no compile-time arm to run.** Building one is
upstream work in mockspace, `95` scopes harness work to what the panel's continuation needs, and nothing is
blocked on this today, so I did not build it. That scope call is mine and is attackable.

### 9.1 What I could settle instead: the encoding costs zero bytes

Compile time is a timing and needs the harness. **Binary size is a static fact about an emitted object**,
which an ad-hoc spike may settle, and it is an axis nobody in the unit has looked at. It matters here
because the cost table is `|R| * |A| * |D|` numbers against the winner table's `|R|`, so if it survived
into the artifact the encoding would carry a per-crate size cost that scales with the arm count. A
substrate whose consumers bitpack columns to save bytes would notice.

`100_probes/p9_does_the_cost_table_survive_into_the_binary.sh` compiles `p3` to an object and reads its
sections, with a control that adds one runtime read of the same table so it cannot be const-consumed.

```
the real file: cost table read only at const time
  Section (__TEXT, __text): 848
  Section (__LD, __compact_unwind): 32
  symbols mentioning COST or WINNER: none

the control: same table, one runtime read
  Section (__TEXT, __text): 928
  Section (__TEXT, __const): 240
```

**Zero bytes.** No constant-data section at all, and no symbol for either table. The control shows exactly
240 bytes of `__const`, which is `6 * 5 * 2 * 4`, the cost table to the byte, so the absence in the real
object is const consumption rather than an artifact of nothing referencing it. **The control is the
finding; without it the empty section proves nothing.**

So on the two axes an ad-hoc spike can reach, emitted instructions and emitted bytes, the cost-table
encoding costs nothing over the winner table. The one axis that remains is compile time, and it remains
unpriced.

**F-100-8. A region-indexed cost table and its winner table, consumed by a const argmin, leave no
constant-data section and no symbol in the emitted object. A control adding one runtime read of the same
table emits 240 bytes of `__const`, which is the table exactly.**
`holds for: regions = 6, arms = 5, cost coordinates = 2, target = aarch64-apple-darwin, rustc
1.98.0-nightly (57d06900f), edition 2024, opt-level 3, panic = abort, emit obj, no_std, feature gates = 0,
threads any (a compile-time artifact), target features baseline`
Evidence: `100_probes/p9_does_the_cost_table_survive_into_the_binary.sh` with its runtime-read control.

## 10. The region set is written down three times, which is the join underneath the one being argued about

Both proposals take the **region set** as an input and argue about what is computed from it. On the
evidence in this repository the region set is itself duplicated, and the duplication has already cost a
member of this unit a manual repair.

`97` section 5 says it decoded the arity sweep "from the crate's own key encoding rather than from the
title". That is a reader discovering by hand that the human-facing statement of a region and the machine
one are separate artifacts.

`warm-clamp-shared` declares `KEY = W * 10000 + NC * 1000 + LOG2A * 10 + OP`
(`mock/benches/variants/warm-clamp-shared/src/lib.rs:83`) and ships const-fn decoders for each field.
`mock/benches/bench.toml` writes the keys as integer literals. The block's `title` states the width and
the arity list in prose. **Three statements of one fact, joined by nothing.**

`100_probes/p5_the_region_is_written_down_twice.py` audits it mechanically across the eight `warm-clamp`
blocks, decoding every key and checking it against its own title, having first checked its transcription
of the decoder against the crate source rather than trusting it.

```
  decoder transcription checked against the crate source: ok
  blocks whose title disagrees with its decoded keys: 0
```

**Clean, and that is still a result.** The twins agree today. Nothing keeps them agreeing, and the reader
who wanted to know what `130060` meant had to open a different file to find out.

**What this says about Q43.** A single-definition discipline applied here would generate more than the
winner table. It would generate the region grid, the arm registry and the key encoding from one
declaration, so that the region a strategy selects over and the region the harness measured are the same
object by construction rather than by a reader decoding an integer. A design that automates the
weighting-to-table join and leaves the manifest-to-decoder join manual has automated the one that has not
yet gone wrong.

I offer that as an option to add to the register rather than as a proposal, and I have not built it.

## 10b. Nine shapes found and not taken, with what closed each

A dispatch that found several shapes and shipped one has thrown away most of its value, so these are
listed with what closed them rather than left implicit. Several are still live and are marked as such.

**1. Ship only the winner table and check it offline.** `97`'s shape as proposed. Not taken as the target
because the check is a step somebody runs rather than something the build enforces, and p2 shows the
property it checks is the one a generator preserves. **Still right as the interim**, and `98` says so
itself: where two of op's four intents have no measurable coordinate, no weight vector can be written for
them, so there is nothing to generate from and a checked table is what exists.

**2. Ship only the cost table, with no winner table and no assertion.** One artifact instead of three,
and the consumer's weighting becomes the only path. **Closed by p2**: with nothing to compare the const
argmin's output against, there is no differential at all, and the whole defect class p2 measures becomes
undetectable. The winner table's remaining job is to be the thing the assertion checks.

**3. Check by independent recomputation rather than by cone membership.** Much simpler to write: run a
second argmin and diff. **Closed by p2's G3 column**: recomputation flags a tie-break disagreement as a
defect 48 of 48 times, and a tie is exactly where the weighting declines to choose. Cone membership is the
tie-break-invariant form and costs no more.

**4. Make the section a tolerance-contending SET rather than a point.** `97` section 5's instrument,
which I expected to make the section a stable object. **Closed by p1's own measurement**: set stability is
non-monotone in the tolerance, because the membership boundary is as noisy as the argmin whenever the band
lands near an arm. It helps where a band brackets a gap and not otherwise, which is not a general fix.

**5. Normalise per region rather than globally.** Would narrow section 8's independence failure to the
arms present at that region. **Not tested**, and it does not remove the failure, only its reach. The
frozen-range remedy removes it outright and was tested instead.

**6. Generate the region grid, arm registry and key encoding from one declaration.** Section 10's
proposal. **Live and unbuilt.** It is the join underneath the one this unit is arguing about, and it is
the one a member of this unit already had to repair by hand.

**7. Build a compile-time arm in the bench harness.** The only thing that would price section 9's one
remaining cost. **Live and unbuilt**, on a scope call of mine that is attackable: it is upstream work in
mockspace and nothing is blocked on it today.

**8. A per-coordinate tolerance band rather than one global band.** p8 finds that different estimators of
one coordinate differ by two orders of magnitude in resolution, so a single band width across coordinates
is asserting they are alike when they are not. **Live and untested.**

**9. Select the section by resample agreement rather than from the point estimate.** `98`'s p14 rule,
which it measures as changing the answer for one of three weightings and reproducing 2.1 times more often
where it does. **Not re-tested here.** p1's regret result suggests it buys little on the carrier family
because the point estimate is already within a fraction of a percent of optimal, but that is an inference
from a different measurement and not a test of the rule.

## 11. What I keep, and why keeping it is the result

**`98`'s inversion.** Generating rather than checking is right, for reasons that survive section 2 having
removed one of them. It removes a real defect class outright, and the class it admits is detectable by a
decider that already exists. I would carry `98` section 3 forward with its justification amended rather
than its conclusion.

**`97`'s decider, at the strictly-positive rung, in a new role.** `98` corrected which rung it names and
that correction holds. What I add is where it belongs: not as a constraint on a human's table but as the
acceptance test on a generator's output, checking membership of the **stated** weighting rather than
existence of some weighting. p2 measures that this is the difference between catching 0 and catching 489.

**`98`'s second read of `97`'s counts, unqualified.** Two independent implementations reproducing 72 and 9
exactly is the strongest corroboration available and a third adds nothing. I did not attempt it.

**`93`'s naming of the two encodings**, which is the fork with actual content and which the register was
right to enter even though it entered it under the wrong question. p3 is that fork compiled at the region
dimension P4 did not reach.

**`97`'s three-layer polarity reading.** I did not attack it and I have no evidence bearing on it. I note
that section 6's composition depends on it exactly as `98` says its own proposal does: generating per
build is licensed for unobservable coordinates and forbidden for observable ones.

**`94`'s W9 and the site-carried plan**, and `97` section 5's decision of the intermediate-precision
disagreement. Untouched.

**`98`'s F-98-9**, that no committed bench family carries a column for accuracy or divergence from a
reference. My p5 audit crossed the same corpus from a different angle and found nothing that changes it.

**The four names.** Nothing here argues for renaming anything.

**And `98`'s own withdrawal in its section 11.** Correcting yourself against your own probe and keeping
the refuted one is the discipline working, and `98`'s example is why I kept my own refuted outputs rather
than deleting them. **Seven of my own claims did not survive their own probes and each is corrected in
place rather than removed:**

- section 3, I predicted the strictly-positive rung would catch a dropped coordinate; 0 of 312;
- section 4.3, my first regret statistic had an arbitrary origin and reported a maximum of 5773%, and its
  output is kept at `p1_v1_relative_regret_is_meaningless.out`;
- section 4.4, my headline that the instability is free held on two of four families rather than all;
- section 7, the first version of p4 ran one configuration and would have reported the false-alarm
  concern refuted on a setup that could not have shown it;
- section 7.3, I was about to carry the band tension as a located cost, and it was the estimator;
- section 8, the first version of p4b imported `97`'s two-coordinate dominated set into a
  three-coordinate model and made its own control fire;
- section 8 again, I wrote that normalisation makes a unit defect invisible, when it neutralises it.

Plus two citations, in section 16. The instrument that caught the last two is a probe; the instrument
that caught the other seven was building the control or widening the matrix, which is the same instrument
`98` used on itself.

## 12. Located disagreement, and one report

**With `98`, on whether the section is unstable: converged, at a predicate.** I opened intending to
settle this against `98` and section 4.4 stopped me. On the family `98` measured, the instability is an
arm against its byte-identical control at a cost of four hundredths of a percent, and `98`'s reading of it
does not hold. On `bitpack-contend-decode` it holds outright, at 3.8% of runtime. **Neither of us is
wrong about a different family and both of us were about to state a universal.** What we converge on is
the predicated form: the section is stable enough to be checked where the competing arms are separated by
more than the harness's own resolution, and the corpus measures that resolution per family via the
control pair.

`98` should still answer, and should be **resumed** rather than re-dispatched, because a reply needs the
context that produced the claim. What remains genuinely open between us: p1 and p7 resample one run, which
measures within-run noise only. Rerunning a family rather than resampling it would separate that from
between-run drift, and would move the predicate's threshold in a direction neither of us can currently
predict.

**With `97` and `98` jointly, on whether check-against-generate is a fork at all.** I say it is a
maintainer workflow question with no consumer-visible content, and that the fork with content is
winner-table against cost-table. Both should answer. What would distinguish us: whether a consumer
supplying a weighting is a use case the design wants at all, which is a question about intent rather than
a measurement, and which section 13 does not put to op because `arvo-toolbox-not-policer.md` already
answers it.

**With myself, on the tolerance band: resolved rather than carried.** Section 7.2 found a configuration
where no band worked and I was going to carry it as a located cost. Section 7.3 dissolves it: the
configuration is the tail-weighing one, every negative finding about that coordinate in this unit is
measured on the interquartile range, and swapping to a tail quantile makes it the best-behaved coordinate
of the three rather than the worst. What remains open is whether the same holds on another bench family,
which I did not test, and whether a tail quantile is the statistic a consumer weighing tail behaviour
actually wants, which is a design question no probe of mine touches.

**A state change in the tree that I may have caused, reported rather than left.** At the start of this
dispatch `git status` showed `mock/benches/Cargo.lock` modified against HEAD. It is now clean. I did not
edit it; running the variant crates' suites and the mockspace pre-commit validation are the two things
that touched anything in that directory, and one of them evidently regenerated it back to the committed
content. Nothing is lost relative to what is committed, and whatever the uncommitted difference was is not
recoverable because it was never recorded. `a-shared-clone-is-someone-elses-desk.md` says to say so rather
than to notice quietly, so I am saying so.

**A report rather than a disagreement.** `bitpack-write-contend-shared` needs `--test-threads=1`, per
section 0. Both `98`'s account and mine are correct about their own runs and the variable is the test
runner's parallelism against the crate's shared thread pool. Recorded there rather than here because it
is a gate fact rather than a strategy one.

## 13. For op, and it is nothing

Deliberately. Section 0 records that Q43 as written is a which-single-policy fork, and the honest answer to
it is per-region, so putting it to op would be the fourth instance of the shape he has rejected three
times. Q44 has an answer with a predicate and needs no ruling. The consumer-expressiveness question is
answered by `arvo-toolbox-not-policer.md` and I11 already.

The two items `99` carries for op, both about I3, are untouched by anything here and I add nothing to
them.

## 14. Findings, each with its predicate

Stated above at F-100-1, F-100-2, F-100-3, F-100-3b, F-100-4, F-100-5, F-100-6, F-100-7 and F-100-8. Notation per I13 and `RULES.md`: a dimension listed with a range
or `any` was established across it, listed with a fixed value was established there only, and absent means
the finding does not hold anywhere that dimension is present.

## 15. What I did not do, and what I could not settle

**I did not derive blind, and my brief carried `98`'s headline.** So nothing here claims independence
except where the probe is a genuinely different instrument, which is p1's statistic, p2's detector
comparison and p3's region dimension. p1 deliberately shares `98`'s weightings and bytes-per-element
mapping so that the arm set is the only variable, which makes it a controlled second read rather than an
independent derivation, and I say so. p7 is the one place I went looking for my own finding's boundary
before resting on it, and it found one.

**I read a small part of the panel.** In full: `INTENTS.md`, `RULES.md`, `97`, `98`, `99`, `83`, `85`,
`87`, `88`, `95`. Partially: `93` sections around 940 to 1000, `94` via its probe A,
`OPTIONS.md` Q43 through Q47 and the Q41 addendum. Of the probe directories I opened
`93_probes/p4_preference_erases.rs`, `94_probes/a_choice_function.rs`, `98_probes/p10*.py` and its output,
in full, because sections 4, 5 and 6 rest on what those actually contain rather than on their files'
accounts of them. **Not read:** every other member file, `DROPLIST.md`, `PERSONA_CALLS.md`,
`PRIOR_CALLS.md`, the `SEED_*` files, the archive, `97`'s probes and `35`'s. So where any of this restates
something, I do not know it.

**I did not open `97`'s decider.** I wrote my own cone machinery from the mathematics rather than reading
`97_probes/p9_the_decider.py` or `98_probes/cone.py`, deliberately, so a disagreement between them would
be visible. There is no disagreement to report because I did not reproduce their counts; my instrument
answers a different question.

**Everything is one host and single-threaded.** Every finding above is a `threads = 1` finding, which is a
region rather than a silence, and every timing is read from runs taken on one machine.

**Compile time is unpriced** and I have said so rather than reaching for a number. p3b establishes which
shapes compile and nothing about what they cost.

**I could not settle whether the tolerance band generalises.** Section 7 measures one bench family. The
tension it finds is explained by a coordinate `98` independently found unresolvable, which is two
instances pointing the same way and not a general result.

**I did not attempt the axis set, the chain question, the resolution question, or the three-layer split.**
`99` lists the third of those as something the second four should test and I did not get to it.

**And I did not build the region-declaration generator section 10 describes.** It is an option, it is
unbuilt, and I have named it as an option rather than a proposal.

## 16. Coverage of the citations

Every `file:line` in this document was opened and its content tested rather than merely resolved, by
`100_probes/p6_verify_my_citations.py`, which is `25` section 9's instrument applied rather than admired:
each row carries the phrase the citation is FOR, so a citation drifting onto a neighbouring heading fails
rather than passing on a coincidence.

```
citations checked: 20   ok: 20   failed: 0
```

**It was not clean on the first run and both failures were mine.** The file cited `98:475-477` for "the
generation happens once, by a tool"; the sentence starts at line 474 and the span I wrote missed its first
line while still resolving to text that looked right. And a citation into
`arvo-toolbox-not-policer.md` expected "ships sharp tools" where the rule says "ship sharp tools", which
would have been a misquotation of a workspace rule inside an argument that leans on it. Both corrected
against the files rather than from memory.

That is the ninth and tenth recorded instance of this failure class across two panels, and the number is
reported rather than quietly fixed, because `RULES.md` records that five instances went by before anybody
counted.
