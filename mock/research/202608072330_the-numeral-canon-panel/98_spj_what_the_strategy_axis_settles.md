# 98. What the strategy axis settles

**Predecessors:** `93` and `94`, the unit's cold pair, and `97`, which attacked both. **Probes:**
`98_probes/`, eleven of them, each committed as it ran.

This is the fourth file of the unit and the last before the checkpoint that goes to op, so `95`
governs its shape more sharply than it governed `97`'s: attack is still available and a fifth
refutation is worth less to this unit than a stated shape. What follows is a second read on the most
consequential proposal in the unit, a correction to it that strengthens rather than weakens it, and a
statement of what a strategy is, offered for the consolidation to take, argue with, or leave. Op
decides; nothing here settles anything.

I should say at the outset that I found the predecessors' work good, and that the two places where I
disagree with `97` are places where its own evidence pointed further than its prose went. That is the
happiest kind of disagreement to have.

## 0. The gates

**Canon gate: passed.** Checked against `INTENTS.md` I1 through I17. The assigned question is licensed
by I1, which op demoted to open in his own words: "the strategy set is not closed at exactly four"
(`INTENTS.md:56`). Nothing in the intents forecloses asking what a strategy is.

One thing on this gate did give me pause and I want to record it rather than let it pass, because it is
the shape op has now rejected three times. `97` section 8 proposes that "the constraint the canon puts
on that table is that it be **explicable by one statement of what matters**". A canon-level constraint
on what shape every strategy's table may take is adjacent to I16, where op declined to rank four
const-time constructions and said "We shouldn't police what kind of laws there are or what shapes they
take" (`INTENTS.md:309-311`). I considered returning early on that.

I did not, and here is the reasoning, offered so that somebody can disagree with it. I16 is about how a
**law** is expressed, and its test is functional: a law must "actually work", meaning reach one lowered
path. `97`'s proposal is not about a law's expression; it is a candidate **definition of the thing
itself**. Defining what a strategy is, which is the unit's assigned question, is not policing, and a
definition that excluded nothing would define nothing. Section 3 below then removes the sharp edge
anyway, because it shows the property is better obtained by construction than imposed as a check, and a
property that holds by construction polices nobody.

**Test gate: passed. 123 tests across 13 crates, all green, and I read the bodies.**

There is no arvo suite; `mock/crates/` is empty by design. So I gated the bench variant crates, which
are the only executable surface my question touches, and I ran them myself rather than taking `93`'s,
`94`'s or `97`'s word for it, since three unratified files agreeing is shared drift.

```
grep -rc '#\[test\]' mock/benches/variants/ | grep -v ':0$'   # 124 attributes
cd mock/benches/variants/<crate> && cargo test                # per crate
```

Per crate: `bitpack-carrier-shared` 9, `bitpack-contend-shared` 12, `bitpack-footprint-shared` 6,
`bitpack-plan-shared` 5, `bitpack-shared` 3, `bitpack-wide-shared` 6, `bitpack-write-contend-shared`
15, `quantiser-fadd-shared` 1, `quantiser-radix-shared` 3, `satfold-shared` 11, `warm-clamp-shared` 7,
`warm-container-shared` 15, `wide-rung-shared` 30. **Total 123.**

The grep counts 124 because `bitpack-write-contend-shared/src/stress.rs:68` contains the literal
`#[test]` inside a doc comment. That is the whole of the discrepancy and I checked it rather than
reporting the grep.

**Two corrections to my predecessors' accounts, both minor and both worth having on the record.** `94`
section 0 reports 108 tests and says `bitpack-write-contend-shared` "exceeded my command budget"; it
runs in 4.7 seconds once built, and the cost is the build rather than the tests. `97` section 0 reports
96 across eleven crates; the corpus is 123 across thirteen. Neither difference changes anything either
file concluded, and I raise it only because `RULES.md` says counts are measurements.

**The bodies are real work, and saying so is a result.** I read `bitpack-carrier-shared`'s simd tests
in full because `97`'s section 2 and my own p1 and p6 both rest on that family. Its
`padal_agrees_across_a_drain_boundary` runs sizes on both sides of the drain period rather than a
convenient sample, comparing against a ground truth computed from a separate dense column
(`mock/benches/variants/bitpack-carrier-shared/src/lib.rs:513-516`), and its
`drain_period_cannot_overflow_a_lane` asserts the period **two-sidedly**: safe at `drain` and unsafe at
`drain + 1`, so a needlessly short period fails the test as loudly as an unsafe one. That is a test that
can fail for two different right reasons, which is rarer than it should be.

`warm-container-shared` carries a test that names itself an ad-hoc spike with no substance in its own
doc comment (`diag_sat_lanes_actually_runs`), and records that its first version reported zero
nanoseconds because it exclusive-ored into a sink an even number of times and the loop was provably
dead. A suite that documents its own past lie is not a decorative suite.

I found nothing to refuse on.

**The known gate hole is unchanged.** `96` establishes that the harness has never invoked the
per-variant `validate_output`, the fix is upstream and arvo's pin has not moved. I rest nothing on a
per-variant validator. Every number I read from a CSV is a timing, and the cross-arm agreement it would
have checked is checked instead by the crates' own tests, which I read.

## 1. The answer, before the working

Five claims, in the order they matter.

**One. `97`'s rationalisability criterion holds, I derived it independently before reading its
argument, and it is a rung on a ladder rather than a property.** Between "any table" and "one weighting"
there are at least five distinct conditions, and the two that matter are far apart: on the committed
carrier data, of 46656 sections, **2048 are Pareto-admissible and 58 are realisable by a strictly
positive weighting**. Section 2.

**Two. `97`'s headline number and `97`'s own dominance finding are about different rungs, and 63 of its
72 sections select an arm it says no weighting can select.** I reproduced its model exactly and got 72
and 9, the two numbers it reports, from an independent implementation. What separates them is whether a
weight may be zero. The number with the property `97` wants is 9, not 72, and the correction
**strengthens** its conclusion. Section 2.3.

**Three. The property is better obtained by generating the table from the weighting than by checking
the table against one, and the measurement that says so is about the table's stability.** Resampling the
committed run's own 80 samples per arm, a fixed weighting produces **30 distinct sections for one
weighting and 77 for another**, and for the third the committed section is not even the modal one. A
section is not stable across a rerun of the same bench on the same afternoon. Checking an unstable
artifact for a property is worth less than deriving it from a stable one. Section 3.

**Four. Every one of op's four strategy intents states a primary concern and then explicitly refuses to
make it absolute, and that refusal has exact operational content.** A priority order over coordinates is
realisable as a weighting on any finite model, in 1200 of 1200 checks, and it carries strictly less
information: on the real table the six priority orders reach **4 sections and weightings reach 58**. So
op's four refusals are the difference between a lexicographic order and a finite exchange rate, and they
are measurable. Section 4. This is `40` section 5.3's reading, derived independently and now with the
gap counted.

**Five. Two of op's four intents have no coordinate to weigh in the committed corpus, and one of them is
his most-repeated call.** Every one of the thirteen bench crates forces its arms to agree on the answer,
correctly, which means no family carries a column for how wrong an arm is or how far it departs from a
reference semantics. And the shipped rule the imitation intent produced is Pareto-dominated on both
machine coordinates in **18 of 22 committed runs** of the family built to test it. Section 5.

## 2. Second read on rationalisability

`97`'s section 2.4 is the most consequential proposal in this unit, it is offered as canon-tier content
for op's own "a little bit of option 3", and it rested on one expert. This section is the second read.

**Independence, bounded honestly.** My brief told me `97`'s headline: rationalisability, 72 of 15625,
section design-tier and weighting canon-tier. So I cannot claim to have started blind, and I say so.
What I could still do, and did, was derive the criterion and its structure from op's intents and from
the mathematics before opening `97`, build my instruments from scratch, and run them on data I chose
myself. Probes p1 through p5 were written, run and **committed before I read `97`**, which the commit
order shows. p6 and after were written afterwards and are second reads in the ordinary sense.

### 2.1 What the criterion is, derived rather than restated

Fix a set of regions `R`, a set of arms `A`, and a cost map `c : R × A → Q^d` with lower better on every
coordinate. A **section** is any `σ : R → A`. Sections are the objects `25` section 7 names
(`25:533-534`), and the question is which of them a weighting can produce.

The ordinary answer is revealed preference: `σ` is rationalisable by `w` when

> for all `r ∈ R` and all `a ∈ A`,   `⟨w, c(r, σ(r))⟩ ≤ ⟨w, c(r, a)⟩`.

That is a homogeneous linear system in `w`, so it is decidable and the feasible set is a cone. Nothing
here is new mathematics and `97` says so too; what matters is that the question is decidable rather
than a matter of taste, and that it is not one question.

**It is five questions, because the conditions between "any section" and "one weighting" form a ladder,
and the rungs are not the same distance apart.**

- **L0**, any section. `|A|^|R|`.
- **L1**, Pareto-admissible: `σ(r)` is never strictly dominated at `r`.
- **L2**, order-rationalisable: one total preorder over cost vectors, monotone with respect to the
  componentwise order, makes every choice a minimum simultaneously.
- **L3**, linearly rationalisable with `w ≥ 0`, not all zero.
- **L4**, the same with `w > 0`: every coordinate carries weight.
- **L5**, `w > 0` and the choice is the unique minimiser, so it is forced rather than permitted.

**Why the ladder and not a binary, and this is the part I most want carried.** "A strategy is a table"
is L0. "A strategy is a weighting" is L4. Op's answer at `88:20-21` was "Mostly option 1, but a little
bit of option 3 with it", and he flagged his own difficulty wording it. On a binary that sentence has no
reading. On a ladder it has an obvious one: **a rung strictly between the ends**, and the panel's job is
to say which rung and what it costs. That is a better home for op's sentence than either pole, and it is
why I think `97` is right that rationalisability is the checkable content of it, and understating the
answer by naming only one rung.

### 2.2 The ladder measured, exactly, on committed data

`98_probes/p2_the_rationalisability_ladder.py`, over the cost table `98_probes/p1_extract_the_cost_table.py`
builds from the six committed `bitpack-carrier-width_n*` runs. Six regions, six arms, three coordinates:
median `algo_ns`, bytes per element (a declared property of the arm, not a measurement), and the
interquartile range of the per-batch samples, which the harness's own findings file for that family
raises as a decision axis in those words ("Speed leader bitpack-carrier-d64 vs stability leader
bitpack-carrier-d32 (+3% speed for 4.1x steadier)").

Exact rational arithmetic throughout, by convex polygon clipping on the weight simplex
(`98_probes/cone.py`), cross-checked against an independently written Fourier-Motzkin implementation
(`98_probes/lp.py`) on 300 sampled sections per rung, agreeing 300 of 300 in all three.

```
  L0 any section                          46656   100.000%
  L1 Pareto-admissible                      2048     4.390%
  L2 order-rationalisable                   2048     4.390%
  L3 linear, w >= 0                          117     0.251%
  L4 linear, w > 0                            58     0.124%
  L5 linear, w > 0, unique argmin             58     0.124%
```

Four things in that table and three of them surprised me.

**L1 = L2 exactly.** Order-rationalisability adds nothing to admissibility here. The middle of the
ladder is empty on this data.

**L4 = L5 exactly.** A weighting that permits a section on this table also forces it. Permission and
determination coincide.

**L3 is not contained in L1**, with 48 exceptions. That is not a bug in the classifier, it is the whole
content of the difference between `w ≥ 0` and `w > 0`, and section 2.3 is what it does to `97`'s
numbers. A weight of zero on a coordinate means the strategy is indifferent along it, and indifference
is what admits an arm that is strictly worse there and no better anywhere else.

**The big gap is L1 to L4: 2048 down to 58, a factor of 35.** That is the price of demanding a weighting
rather than merely demanding coherence.

**And one arm is Pareto-dominated in every region**, `bitpack-carrier-d64`, so no monotone rule reaches
it anywhere.

#### The order rung is real in general and empty here, and I chased the mechanism

L1 = L2 could be a fact or a coincidence, and the difference matters: if the rung is always empty there
is no reason for a canon to name it. `98_probes/p3_a_rung_count_is_a_fact_about_one_table.py` runs 60
random models each from three generators chosen to differ in structure rather than only in seed.

```
generator uniform   L1 == L2 in 18 of 60 models
generator scaled    L1 == L2 in 60 of 60 models
generator tiered    L1 == L2 in 60 of 60 models
```

So the rung is real: in the unstructured regime it separates in 42 of 60 models. It is empty in the two
generators that impose the structure real bench data has, where cost grows with the region index and
footprint is an arm-intrinsic constant.

My first hypothesis for why was that a cycle must leave a region and return, so a table with no
**backward** cross-region Pareto edge cannot separate. The real table has two such edges, so the
hypothesis is refuted by the data it was built to explain, and
`98_probes/p3b_why_the_order_rung_is_empty_on_real_data.py` records it as refuted rather than deleting
it.

What does hold is exact and checkable in one sweep. Build the **union graph**: every Pareto edge, plus
every revealed edge any **admissible** choice could contribute. Every admissible section's graph is a
subgraph of it, so if no strict edge of the union graph lies on a cycle, no admissible section can be
order-irrational. The real table satisfies it, with 383 strict edges and zero on a cycle, and both of
its backward edges point at an arm dominated inside its own region, which nothing admissible ever
chooses. Against the three generators the criterion was not merely sufficient but **exact** in all 120
models: clean in 13, 40 and 40, and L1 = L2 in 13, 40 and 40.

Sufficiency is proved; necessity is empirical over 120 models and is stated as that.

#### And a rung count is a fact about one table, which bounds what any of these numbers support

The same probe measures the L4/L1 ratio across all 180 models: minimum 0.021, median 0.111, maximum
1.000, a spread of **47x**. In some tiered models every admissible section is linearly rationalisable
and the gap vanishes entirely.

So no particular value of that ratio says anything about arvo. What survives across every generator is
the **ordering**: a weighting admits far fewer sections than admissibility does, and both admit far
fewer than the section count. `97`'s F-B makes the same point by a counting bound on hyperplane
arrangements, which is a better argument than mine and reaches further; this is a second instance of it
from a different direction, and I record it as corroboration rather than as news.

### 2.3 Reproducing `97`'s numbers exactly, and where its two sentences part company

`97` section 2.2 reports **72 of 15625** as the headline and **9** as the strict count, on five arms,
six regions and two coordinates. `97` section 10 reports that two arms, `bitpack-carrier-d64` and
`bitpack-carrier-packed`, are "beaten on both time and bits by another arm at all six record counts" and
concludes they "cannot be selected by any weighting-defined strategy".

Both sentences are in the same file and they cannot both be about the same notion of weighting.

`98_probes/p6_reproduce_the_predecessors_count_and_rung_it.py` rebuilds that model from the same CSVs
and classifies every section by rung, using interval arithmetic on the one-dimensional weight simplex
rather than `97`'s extreme-ray enumeration, so the implementations share only their input:

```
  L0 any section                      15625
  L1 Pareto-admissible                  144
  L3 rationalisable, w >= 0              72   <- 97 section 2.2 reports 72 here
  L4 rationalisable, w > 0                9
  L5 forced, w > 0                        9   <- 97 section 2.2 reports 9 strict
```

**Both of `97`'s numbers reproduce exactly, from an independent implementation.** That is the strongest
corroboration available and it should be recorded as such: the arithmetic in `97` section 2 is right.

And then:

```
  sections at L3 that select an arm 97 says no weighting can select: 63 of 72
  sections at L4 that do:                                             0 of 9
```

**63 of the 72 select `bitpack-carrier-packed`**, which `97` itself names as unreachable by any
weighting. The mechanism is exactly L3's: `packed` and `packed-simd` carry the same 13 bits, so a weight
vector with a zero on time cannot tell them apart and weakly admits either, and `97` says so in its own
words while reconciling 9 with 72. The reconciliation is correct arithmetic and the conclusion drawn
beside it belongs to a different rung.

**So the figure with the property `97` wants from it is 9 of 15625, not 72 of 15625.** The correction
makes `97`'s conclusion **stronger**: a weighting admits 0.058% of the sections, not 0.46%.

**What this settles, generally, and it is the one sentence I would most like in the consolidation:**

> A weighting whose coordinates all carry strictly positive weight cannot select a Pareto-dominated arm.
> A weighting permitted a zero weight can. The first is a theorem; the second is a fact about the table.

The theorem is one line: if `b` beats `a` on every coordinate and strictly on one, then `⟨w, b⟩ < ⟨w, a⟩`
for every `w > 0`, so `a` is never an argmin. `98_probes/p9_the_proposal_instantiated.py` exhibits the
failure at a zero weight on the real table and, honestly, also exhibits two zero-weight cases where no
dominated arm is selected, because whether it fails is a property of the table. The guarantee at a zero
weight is not "usually holds", it is **unclaimed**, and a canon that wants it has to ask for strict
positivity rather than hope about the data.

### 2.4 Does the constraint buy what it is argued to buy

`97` section 2.4's argument for requiring rationalisability is not that it is elegant. It is that a
rationalisable table makes the strategy's name **predictive**: a reader "can predict what it will do at
a region nobody has benched", while an irrational table means "the name carries no information beyond
the rows already written". `97` states plainly that this section is "an argument rather than a
measurement", so I measured it.

`98_probes/p7_does_rationalisability_predict.py` is leave-one-out. Take a section at L4, hide one
region, fit the weight cone exactly to the other five, and ask which arms at the hidden region are the
minimiser for some weight still in that cone. The baseline to beat is the number of Pareto-admissible
arms there, because that is what a reader already knows without any weighting.

```
three-coordinate table (time, bytes, spread)
  determinate     162    46.55%      mean surviving arms 1.621
  narrowed        168    48.28%      mean admissible arms 3.667
  vacuous          18     5.17%

97's two-coordinate model
  determinate      38    70.37%      mean surviving arms 1.296
  narrowed          8    14.81%      mean admissible arms 2.333
  vacuous           8    14.81%
```

**The constraint buys real prediction and its dichotomy is too sharp.** The name is not a lookup key: it
narrows from 3.67 candidate arms to 1.62 on one model and from 2.33 to 1.30 on the other, and pins the
hidden row uniquely in 47% and 70% of trials. Nor is it a rule that determines the unmeasured region:
5% and 15% of trials are vacuous, where the constraint predicts nothing at all.

Two bounds on my own probe, stated because they matter. Leave-one-out over regions the table was fitted
on is not the same as a genuinely new region with costs nobody has seen, and the second is what the
argument is about; I measured the first because the second needs a measurement that does not exist. And
the determinate rate is a fact about these two tables, which p3 already showed varies by tens across
tables of the same shape.

**Verdict on the criterion, with its predicate.** It holds, it is decidable exactly, it is the checkable
content of `88:20-21`, and it should be stated at the strictly-positive rung rather than the
non-negative one. Section 3 is where I think it should be applied differently from how `97` proposed.

## 3. Generate the table from the weighting; do not check the table against one

This is the one place I think `97`'s proposal should change shape, and the reason is a measurement it
did not take.

`97` section 8 puts the constraint on the artifact: "What reaches the compiler is a **table**, evaluated
offline by a person, and the constraint the canon puts on that table is that it be **explicable by one
statement of what matters**". So a person measures, writes a table, and a check runs at the point the
table is written to confirm some weighting explains it.

**The table is not stable enough to be the object a check is applied to.**

`98_probes/p10_is_the_table_stable_enough_to_be_an_object.py` takes the committed run's own 80 samples
per arm per region and bootstraps them: 2000 resamples, each producing a table the same run could
plausibly have produced. Then it asks, at a **fixed** weighting so that only the measurement moves, what
section comes out.

```
speed-first     distinct sections across resamples: 30   modal 531/2000 (26.6%), is the committed one
storage-first   distinct sections across resamples:  8   modal 881/2000 (44.0%), is the committed one
tail-first      distinct sections across resamples: 77   modal 243/2000 (12.2%), is NOT the committed one
```

No new measurement was taken and this is not a bench; it is an uncertainty estimate over a committed
artifact, and it measures the weakest perturbation available, which makes any instability it finds a
lower bound. Under that weakest perturbation, a section moves at five of six regions for one weighting,
and for another the section actually committed is not the one the run most often produces.

By contrast, the dominance structure is stable: the set of arms dominated in every region is `{d64}` in
**1996 of 2000** resamples.

**Three consequences, and the third is the proposal.**

**The section cannot be canon, and `97` and `40` are right about that for a stronger reason than
permanence-in-principle.** `40` section 0 argues "a canon stating the table is wrong the next time
somebody measures". The measurement says it is wrong the next time somebody **reruns**, which is a much
shorter horizon.

**A check applied to the table is a check applied to a noisy artifact.** Asking whether some weighting
explains the rows is asking whether a sample happens to be explicable. On this data the answer would
change between reruns, and the check would sometimes pass and sometimes fail on the same design.

**So invert it. State the weighting; derive the table from it.** The design tier holds the weighting,
numerically, and the section is regenerated whenever the measurements are refreshed. Then:

- rationalisability is **true by construction** rather than a property to be verified, so there is
  nothing to check and nothing to police, which also disposes of my canon-gate worry in section 0;
- the instability becomes harmless rather than alarming, because a refreshed measurement produces a
  refreshed section from the same stated intent;
- `97`'s prediction argument gets its strongest form: the name predicts at an unmeasured region because
  the same weighting is what will be applied there, rather than because a written table happened to be
  explicable;
- and the artifact a reader has to trust shrinks from a table of measured winners to a statement of what
  matters, which is the smaller and more stable object.

**This is `97`'s proposal taken one step further rather than a replacement for it.** Its two tiers are
unchanged: objective canon, table design. What changes is the direction of the arrow between them, from
*check* to *generate*, and the reason is that the thing being checked moves under its own noise.

**Two costs of the inversion, stated rather than hidden.** Generating the section from a weighting means
the emitted arm selection can change between builds even with identical source. For an **unobservable**
coordinate that is licensed: `40` section 5.3 says an arm may resolve those however it likes and a later
arm may resolve them differently. For an **observable** one it is forbidden by the same section, so the
generation story applies exactly to the coordinates cost-based selection lives on, which is where it was
always meant to apply.

And it is direct evidence for the axis `93` section 2 says is missing. `93` names
**reproducibility across targets and builds** as a sixth axis nobody has named and reports it as
"demanded, is orthogonal to the five above, and is currently unnameable in the design"
(`93:180-188`). A section that moves under a resample of its own bench run is exactly a design whose
emitted code is not reproducible across builds. `93` derived the axis from what a consumer needs; p10
finds the same thing from the machine. **I did not read `93` before building p10 and I am recording this
as a second instance of `93`'s axis 6 rather than as a new finding.**

## 4. What op's intents say about the shape of a weighting

If a strategy is a weighting, the canon owes what kind of weighting. Op's four statements answer that
more precisely than they look like they do, and the answer is uniform across all four.

### 4.1 A priority is not an exchange rate, and every one of the four intents refuses the priority

Two things "a weighting over measurements" could mean.

A **priority**: the strategy names which measurement outranks which, absolutely, so any gain on a higher
coordinate justifies any loss on a lower one. That is a lexicographic order.

An **exchange rate**: the strategy names how much of one measurement a unit of another is worth, so a
loss on the primary is accepted when the gain elsewhere is large enough and refused when it is not. That
is a weighting with finite ratios.

Now read op's four, and notice that each names a primary concern **and then explicitly refuses to make
it absolute**:

- I4, on the imitation intent: mimicry "does not make it absolutely required, if mimicking is
  consistently just worse choice" (`INTENTS.md:92-94`).
- I5, on the speed intent: it "can sacrifice soundness... but it should not lose it for nothing,
  instead, provable meaningful gains" (`INTENTS.md:102-103`).
- I6, on the storage intent: it "has more leeway to do things non-efficient" and "does not have to drop
  efficiency wins elsewhere" (`INTENTS.md:109-119`).
- I7, on the accuracy intent: it "sacrifices as much performance and efficiency as makes sense"
  (`INTENTS.md:125-127`).

Four intents, four refusals of the absolute reading. That is not a stylistic hedge, and
`98_probes/p4_priority_order_against_exchange_rate.py` measures what it is instead.

**A priority is always realisable as an exchange rate on a finite model.** 1200 lexicographic sections
over 200 random models, one per coordinate permutation, and **1200 of 1200** are realisable by a
strictly positive weight vector. So a canon stating which measurement outranks which has said something
a weighting can implement, and the two formalisms are not rivals.

**The converse fails by a wide margin.** On the real table, the six coordinate permutations produce
**4 distinct sections**, against **58** realisable by a weighting. So 54 of 58 behaviours need an actual
rate and no priority order can express them. The probe exhibits three of the 54, and one with an exact
weight vector that realises it, checked by recomputing the argmin.

**So op's four refusals have exact operational content**: they are the difference between 4 available
behaviours and 58, on this table. That is what "not absolutely required" buys.

**This is `40` section 5.3's reading, arrived at independently.** `40` derives from I5 and `34` that
"accuracy is lexicographically prior for every objective except `Hot`, and finitely weighted for `Hot`",
and notes that a lexicographic term is a weighting with an infinite ratio while a finite one has a rate.
I reached the same distinction from the four intents' wording rather than from one of them, and I
extend it in one direction: it is not only the accuracy term for the speed-first strategy. **Every one
of the four names a finite term on its own primary concern.**

And the two readings compose without a second mechanism. `98_probes/p9_the_proposal_instantiated.py`
section 3 checks that a lexicographic lead coordinate with finite weights on the rest is realisable by
one strictly positive weight vector, over 450 random cases: **450 of 450**. So a strategy can be
lexicographic in one coordinate and finitely weighted in the others, and it is still one weighting.
Which means op's `88` answer of "mostly option 1, a little bit of option 3" has a shape that carries
both readings at once and needs no reconciliation.

### 4.2 What is not expressible, which is a real cost of the definition

A **threshold** rule, "minimise A subject to B at most t", has no weighting at all. The witness is three
alternatives at `(0, 4)`, `(10, 0)` and `(5, 3)` with a bound of 3 on the second coordinate: the rule
picks the third, and preferring it to the first needs `w1 ≥ 5 w0` while preferring it to the second needs
`3 w1 ≤ 5 w0`, which is unsatisfiable for `w0 > 0`. The probe classifies all three choices exactly and
only the rule's choice is unreachable.

**So a canon saying "a strategy is a weighting" has thereby said that no strategy is a hard bound on a
measurement**, and that should be said out loud rather than discovered later. It is not obviously wrong:
none of op's four intents reads as a hard bound, and I5's "should not lose it for nothing" is explicitly
a rate rather than a floor. But a future strategy of the form "as fast as possible subject to never
exceeding N bytes" would be outside the model, and somebody will eventually want one.

**A defect in my own probe, recorded because it is instructive and because the fix is the finding.** The
first version of that check padded the two-coordinate model with a zero third coordinate and handed it
to the three-coordinate solver, which reported the threshold choice as **feasible**. It is: every
difference vector has a zero in the padded slot, so the weight vector `(0, 0, 1)` satisfies every
constraint trivially and rationalises anything. That is not a bug in the solver, it is a true and
useless fact, and it is the same shape as L3's zero weights one level down. **A coordinate no
alternative differs on makes every section weakly rationalisable.** The note is kept in the probe's own
docstring rather than deleted.

### 4.3 One of op's strategies is defined by a coordinate that is not a measurement of the machine

I3 is op's most-repeated call, stated four times over two days: the strategy in question "should behave
like native primitives in regular old rust would" (`INTENTS.md:81`). If a strategy is a weighting, that
call has to be a weighting over something, and the repository has measured exactly this fork.

`98_probes/p5_the_reference_coordinate_is_not_a_machine_cost.py` reads all 22 committed
`warm-container-*` runs and compares the two arms the fork turns on: `headroom`, the shipped rule that
holds a declared width in the rung above the smallest that fits, against `minimum`, which uses the
smallest. Both arms are declared side by side in the same crate and the crate's own load-bearing test
asserts every arm computes the identical value on every key
(`mock/benches/variants/warm-container-shared/src/lib.rs:1356`), so a timing difference is a difference
in cost and not in answer.

```
runs considered: 22
runs where the shipped rule is Pareto-dominated on (time, bytes): 18
runs where it survives: 4
```

The four survivors survive on a sub-one-percent time edge while paying double the bytes, so I tested
that edge rather than accepting it: a two-sided bootstrap over the harness's own samples puts **three of
the four CIs across zero**. The one real survivor is `warm-container-width-l1` at `W = 13`, where
headroom is 1.16% faster for twice the footprint.

**So there is no weighting over (time, bytes) that selects the shipped rule everywhere**, by the theorem
in section 2.3. Whatever the extra byte buys is not on that list of coordinates.

What it does buy is room above the declared width for an intermediate result to sit in, which is a
statement about **where the arithmetic stops agreeing with unbounded arithmetic**. That is a measurement
against a reference semantics rather than against the machine.

**So the canon owes the coordinate set, and the answer is not "the ones a profiler reports".** At least
one coordinate is divergence from a named reference, and by I4 it carries a **finite** weight rather
than an absolute priority, because mimicry is dropped where it is "consistently just worse choice".

`93` reached the same place from the intents alone, naming "divergence from a reference semantics" as
its fourth axis and observing that "an imitation constraint is a different kind of specification from a
weighting" before resolving it exactly this way (`93:172-176`). This is a second, measured instance of
that, and I am recording it as corroboration of `93` rather than as news.

## 5. What the committed corpus can and cannot weigh

A strategy can only be distinguished from another where something varies a coordinate they weigh
differently. That is a fact about what has been measured and it is checkable.

`98_probes/p8_which_intents_the_corpus_can_weigh.py` reads the cross-arm agreement assertions in all
thirteen shared variant crates. **All thirteen force their arms to agree**, with names like
`all_four_arms_agree_with_each_other_and_with_the_oracle_on_every_key`,
`every_arm_agrees_with_the_oracle_on_every_key` and `all_five_arms_agree_at_every_declared_key`.

That is correct bench design and it is the right requirement: without it the fast arm is fast because it
is doing less, which `warm-container-shared`'s own doc comment says in those words. The consequence is
structural rather than a criticism:

**No committed bench family carries a column for how wrong an arm is, nor for how far it departs from a
reference semantics.** So of op's four intents, two have coordinates in the corpus (time and footprint)
and two do not. On this corpus the accuracy-first and imitate-the-native-primitive intents **have no
expression at all**.

**My prediction was too sweeping and the probe found the counterexample rather than assuming it away.**
`bitpack-write-unsound` is an arm that may compute a different answer, deliberately, and its own header
calls it "a demonstration arm, not as a candidate". The difference it carries is a corruption rate from
a data race rather than a quantisation error, and its magnitude is measured in the crate's stress test
as a rate over trials (`naive_kernel_corruption_rate_under_real_concurrency`, 493 of 3000 trials
disagreeing on the run I made) rather than as a bench column. **A rate of wrong answers is exactly the
shape of an accuracy coordinate, and it sits in a test rather than in the CSV, so no weighting can read
it.**

**This is a harness gap and it is inside the panel's scope**, by `95`'s own named exception for work the
panel's continuation needs. What is missing is a family whose arms compute different answers on purpose
with the difference measured against an exact reference and reported as a column. `35`'s law probes
measure exactly that quantity and are not bench arms; the write-unsound arm has exactly that property
and reports no such column; nothing joins the two. Until something does, half of op's stated intents are
unpriceable by construction, and any claim that two strategies differ is a claim about time and bytes
whatever it says it is about.

**And a coordinate nobody named produces a section nobody else produces.**
`98_probes/p9_the_proposal_instantiated.py` section 4 instantiates three weightings on the real table:
speed-first, storage-first, and one that weighs the interquartile spread. All three give **distinct**
sections and none selects a dominated arm. The third is a finding rather than an illustration: the
spread coordinate is named by none of op's four intents, the harness's own findings file raises it as a
decision axis, and a strategy weighing it behaves differently from all the others. Either it belongs in
the coordinate set and no intent names it, or it is noise and should be excluded, and nothing so far
decides which.

## 6. A converged statement, offered

`95` asks a unit to end in agreement with at least something. This is what I believe `93`, `94`, `97`,
`40`, `25` and this file jointly support, written so the consolidation can take it, argue with it, or
leave it. It is a suggestion and op decides.

A strategy is a **consumer-supplied statement of what matters**, over measurements rather than over
implementations. It names no implementation and owns none.

It is a **weighting over a named set of measurement coordinates, and every coordinate carries a
strictly positive weight**. Strict positivity is not a technicality: it is exactly what makes the
guarantee that no strategy selects an arm worse on every coordinate a theorem rather than a hope, and a
strategy that cares little about a coordinate says so with a small weight rather than a zero one, which
I6's "more leeway" reads as naturally and which costs the guarantee nothing.

The weights are **finite ratios rather than absolute priorities**, which is what all four of op's
intents say when they name a concern and then refuse to make it absolute. A priority is the limiting
case and is realisable as a weighting on any finite arm set, so the two are one mechanism and not two.
A **hard bound** on a measurement is not expressible, and that is a real consequence to state rather
than discover.

The **coordinate set includes at least one coordinate that is not a measurement of the machine**:
divergence from a named reference semantics, which is the only thing the imitation intent can be a
weighting over. It carries a finite weight like the rest.

What a strategy **produces**, applied to evidence, is a section: an assignment of an arm to each region.
The section is **generated from the weighting** rather than written and then checked against one,
because a section is not stable across a rerun of the bench that produced it while a weighting is. Under
that order, one weighting explains every row by construction, which is what makes a strategy's name
predictive at a region nobody has measured.

The **canon states the coordinates and each strategy's relative emphasis**. The **design states the
numbers and holds the generated section**. The numbers are not permanent and the section is not stable,
and neither needs to be, because both are regenerable from the sentence above them.

Two strategies **are not ordered by their weightings**, which are incomparable vectors. They may be
ordered by their **observable** coordinates, where an order exists, and that is a different question
with a different answer, which is `97` section 4's three-region answer and which I do not add to.

A strategy whose objective is over a **chain** rather than an operation is a different shape, served by
not quantising in the interior at a width cost linear in the chain length. `93`'s F7 and `94`'s W7 agree
on this from two parameter settings and I add nothing to it.

## 7. What I keep, and why keeping it is the result

**`25` section 7's sentence, at the design tier.** It survives everything in this file. Section 2 says
what constrains it and section 3 says which direction the arrow runs; neither replaces it, and it has
now been paid for three times.

**`40`'s two-space split, its observable classification, and its lexicographic-against-finite reading.**
Section 4.1 is that reading derived independently and extended from one intent to four. `40` got there
first, from op's four intent statements, before op spoke at `88`.

**`97`'s rationalisability criterion, at the strictly-positive rung.** It holds, it is decidable, its
arithmetic reproduces exactly from an independent implementation, and it is the right formal content for
`88:20-21`. My corrections are to which rung it names and to whether it is checked or generated, not to
whether it is right.

**`97`'s three-layer polarity reading.** I did not attack it and I have no evidence bearing on it. I
note that section 3's generation story depends on it: generating a section per build is licensed exactly
for the coordinates `97` and `40` classify as unobservable, and forbidden for the rest, so the polarity
distinction is doing load-bearing work in my own proposal.

**`93`'s sixth axis, reproducibility across targets and builds.** `93` reports it as demanded and
currently unnameable. p10 is a second, measured instance of the same thing, and I would carry it forward
as a real axis rather than as a suspicion.

**`94`'s W9 and the site-carried plan**, and `97` section 5's decision of the disagreement about
intermediate precision. I have no evidence either way and I am not reopening either.

**The four names.** Nothing here argues for renaming anything. Under the weighting reading each is a
named point, which is additive: every existing spelling keeps working.

**The bench corpus's insistence that arms agree.** Section 5 says it forecloses measuring accuracy, and
it is still right. The fix is a family designed to differ, not a weakening of the ones that exist.

## 8. Located disagreement, carried forward as that

**With `97`, on which rung the criterion is stated at.** I think this is settled against `97` rather
than open: p6 reproduces both of its numbers and shows 63 of the 72 select an arm it says no weighting
can select. But `97` should answer rather than have me record a win, and it should be **resumed** rather
than re-dispatched, because a reply needs the context that produced the claim.

**With `97`, on check against generate.** This one I think is genuinely open. My case rests on p10, and
p10 measures resampling noise on one bench family on one host, which is one instance. What would
distinguish us: whether a stated weighting can actually be written down precisely enough to generate a
table, which is a question about whether the coordinates are commensurable at all. Section 5 says two of
the four intents currently have no coordinate, so the answer today is no for half of them, and that is
an argument for `97`'s check as the interim shape and mine as the target.

**With nobody in particular, on the spread coordinate.** It is in the harness's own findings prose, it
is absent from every panel file I read, and it changes which arms are on the Pareto front in 4 of 6
regions. I do not know whether it is a coordinate or noise, and I did not find anything that decides it.

## 9. For op, separately

The checkpoint after this file goes to op, so this section names only what genuinely needs a human, and
deliberately omits the things the canon can answer. In particular I am **not** asking which rung the
canon should require, whether strategies may be hard bounds, or whether the spread coordinate belongs:
those are all "which single policy governs this category" questions, which `88` section 4 rejected and
`never-ask-which-single-rule-governs.md` names, and the honest answers are per-region.

**One thing, and it is an intent rather than a mechanism.** Section 5 establishes that the committed
measurement corpus has no coordinate for accuracy and none for divergence from a reference semantics, so
two of the four intents op has stated cannot currently be expressed as weightings over anything measured.
The panel can build the missing bench family, and `95` puts harness work inside the goal. What the panel
cannot supply is **what the reference is**. I3 says "native primitives in regular old rust would", and
`93`'s F8 measures that this phrase has two readings which agree at every width Rust has a primitive for
and disagree at all fourteen non-native ones it swept, which are precisely the widths arvo exists to
provide. Somebody has to say which reading is meant before a divergence coordinate can be defined, and
that is a statement about intent rather than a measurement.

I would put it to him as: **at a declared width Rust has no primitive for, does the imitation follow the
declared width or the container the value happens to sit in?** `93` states its own reading (declared
width) and marks it as a reading rather than a finding, which is right.

Everything else I found is the panel's to work out.

## 10. Findings, each with its predicate

Notation per I13 and `RULES.md`: listed with a range or `any` means established across it, listed with a
fixed value means established there only, and absent means the finding does not hold anywhere that
dimension is present.

**F-98-1. Between "any section" and "one weighting" there are at least four distinct conditions, and on
committed harness output they are far apart: 46656 sections, 2048 Pareto-admissible, 2048
order-rationalisable, 117 rationalisable at non-negative weights, 58 at strictly positive weights, 58
forced.**
`holds for: regions = 6, arms = 6, cost coordinates = 3 (median algo_ns, declared bytes per element,
interquartile range of the samples), cost source = committed bitpack-carrier-width_n* CSVs, arithmetic
exact rational, threads = 1, target features any`
Evidence: `98_probes/p2_the_rationalisability_ladder.py`, by exact polygon clipping on the weight
simplex, cross-checked against Fourier-Motzkin in `98_probes/lp.py` on 300 sections per rung with 300 of
300 agreeing.

**F-98-2. A strictly positive weighting cannot select a Pareto-dominated arm. A non-negative one can,
and whether it does is a property of the table.**
`holds for: cost coordinates any, arms any, regions any, arithmetic exact rational` for the positive
half, which is a theorem rather than a measurement.
`holds for: the F-98-1 predicate` for the negative half, where 48 of 117 non-negative sections select a
dominated arm.
Evidence: `98_probes/p2_the_rationalisability_ladder.py` for the count,
`98_probes/p9_the_proposal_instantiated.py` for the exhibited failure and for two zero-weight cases that
do not fail.

**F-98-3. `97` section 2.2's two figures reproduce exactly from an independent implementation, and 63 of
its 72 select an arm `97` section 10 says no weighting can select. The figure carrying the property is 9,
not 72.**
`holds for: regions = 6, arms = 5, cost coordinates = 2 (median algo_ns per record, declared bits per
element), cost source = committed bitpack-carrier-width_n* CSVs, arithmetic exact rational, threads = 1,
target features any`
Evidence: `98_probes/p6_reproduce_the_predecessors_count_and_rung_it.py`, by interval arithmetic on the
one-dimensional weight simplex, against `97`'s extreme-ray enumeration, sharing only their input.

**F-98-4. Order-rationalisability is a distinct condition from Pareto-admissibility in general and
coincides with it on structured cost data, and a union-graph acyclicity criterion predicts the
coincidence.**
`holds for: regions = 5, arms = 5, cost coordinates = 3, cost tables from three generators (independent
uniform, region-scaled, tiered), 60 models each, arithmetic exact rational` for the separation, which
occurs in 42 of 60 uniform models and 0 of 120 structured ones.
`holds for: the same, plus the committed carrier table` for the criterion, which was exact in 120 of 120
models and correct on the real table.
Evidence: `98_probes/p3_a_rung_count_is_a_fact_about_one_table.py`,
`98_probes/p3b_why_the_order_rung_is_empty_on_real_data.py`. Sufficiency is proved; necessity is
empirical over 120 models and is claimed as that.

**F-98-5. The ratio of weighting-realisable sections to Pareto-admissible ones varies by a factor of 47
across cost tables of one shape, so a particular value of it is a fact about one table.**
`holds for: regions = 5, arms = 5, cost coordinates = 3, 180 models across three generators, arithmetic
exact rational`
Evidence: `98_probes/p3_a_rung_count_is_a_fact_about_one_table.py`. Minimum 0.021, median 0.111,
maximum 1.000.

**F-98-6. A lexicographic priority over coordinates is realisable by a strictly positive weighting on a
finite model, including when combined with finite weights on the remaining coordinates. The converse
fails: on the committed carrier table the six priority orders reach 4 sections and weightings reach 58.**
`holds for: regions = 4, arms = 5, cost coordinates = 3, 200 random models for the pure priority case
(1200 of 1200 realisable) and 150 for the priority-plus-finite case (450 of 450), arithmetic exact
rational` for the realisability half.
`holds for: the F-98-1 predicate` for the counting half.
Evidence: `98_probes/p4_priority_order_against_exchange_rate.py`,
`98_probes/p9_the_proposal_instantiated.py`.

**F-98-7. A selection rule of the form "minimise one coordinate subject to a bound on another" is not
realisable by any non-negative weighting.**
`holds for: cost coordinates = 2, alternatives = 3, arithmetic exact rational`
Evidence: `98_probes/p4_priority_order_against_exchange_rate.py`, with the algebra stated in closed form
in the probe's own docstring and the exact feasibility computed on the one-dimensional weight simplex.

**F-98-8. The shipped headroom rule is Pareto-dominated on (time, bytes) in 18 of 22 committed
`warm-container-*` runs, and three of the four survivors survive on a time edge whose bootstrap CI
crosses zero.**
`holds for: declared widths in {8, 13, 16, 32, 60, 64}, element counts and operation densities as
declared by the warm-container bench keys, arms = {headroom, minimum}, cost coordinates = 2 (median
algo_ns, declared bytes per element), 22 committed runs, host = the one those runs were taken on,
threads = 1`
Evidence: `98_probes/p5_the_reference_coordinate_is_not_a_machine_cost.py`, reading committed harness
output, with a 2000-resample two-sided bootstrap on the four survivors.

**F-98-9. No committed bench family carries a column for accuracy or for divergence from a reference
semantics, because all thirteen shared variant crates assert cross-arm agreement.**
`holds for: the 13 *-shared variant crates in mock/benches/variants as committed`
Evidence: `98_probes/p8_which_intents_the_corpus_can_weigh.py`. One arm,
`bitpack-write-unsound`, is exempt from the agreement requirement by design; its wrongness is a race
corruption rate measured in a stress test rather than a bench column.

**F-98-10. A section produced by a fixed weighting is not stable under resampling the committed run's
own samples: 30, 8 and 77 distinct sections across 2000 resamples for three weightings, with the
committed section modal in two of the three. The set of arms dominated in every region is stable at 1996
of 2000.**
`holds for: regions = 6, arms = 6, cost coordinates = 3, 80 samples per arm per region, 2000 bootstrap
resamples, seed 20260814, cost source = committed bitpack-carrier-width_n* CSVs, host = the one those
runs were taken on, threads = 1`
Evidence: `98_probes/p10_is_the_table_stable_enough_to_be_an_object.py`. This is an uncertainty estimate
over a committed artifact and not a bench; no measurement was taken.

**F-98-11. Three weightings over the three measured coordinates give three distinct sections on the
committed carrier table, none selecting a dominated arm, and the one weighing sample spread is named by
no stated intent.**
`holds for: the F-98-1 predicate, weightings as instantiated in the probe`
Evidence: `98_probes/p9_the_proposal_instantiated.py`. The weight numbers are scaffolding chosen to
reach the check and are not proposals.

## 11. Reported outside my question

**Every timing figure in this file is read from committed harness output.** I ran no benchmark. Where
nothing has been measured I have said unpriced.

**Adding a coordinate rescued an arm from a domination verdict.** Under (time, bytes),
`bitpack-carrier-packed` is dominated in every region, which is `97` section 10's second arm. Adding the
sample spread coordinate, which the harness's own findings file for that family names as a decision axis,
puts it back on the front in 4 of 6 regions (`98_probes/p1b_domination_is_relative_to_the_coordinate_set.py`).
So "dominated everywhere" is a claim about the coordinate set as much as about the arm, and the honest
form of `97`'s report is that two arms are unreachable **under the two coordinates it measured**. Its
first arm, `d64`, stays dominated under all three, and even that is only a licence to drop it if three
coordinates are the whole of what any strategy weighs, which nothing has established.

**One test name reads more narrowly than its body.** `warm-container-shared`'s
`the_shipped_rule_widens_every_width_to_64` parses at first glance as a claim about a 64-bit container;
its doc comment says "widens every width at or below 64 bits" and its body asserts the headroom container
is twice the minimum for every `W` in 1 to 64, which is the right assertion. I checked it because the
name misled me, and I am recording that it is fine rather than leaving a reader to re-check it.

**The two root templates now carry a superseded banner**, per `97` section 10, so citations into them in
`93` and `94` are low by eight lines. I made no citation into either.

## 12. What I did not do, and what I could not settle

**I did not derive blind.** My brief carried `97`'s headline, so my independence is bounded to the
criterion's structure and my own instruments, and the commit order is the only evidence for even that.
p1 through p5 were committed before I opened `97`; p6 onward were not.

**I read a small part of the panel.** In full: `INTENTS.md`, `RULES.md`, `93` and `94` including both
phases, `97` and its section headings' claims, `83`, `85`, `87`, `88`, `95`. Partially: `25` section 7,
`40` sections 0, 5.3, 5.4, 6.1 and 6.2, `OPTIONS.md` Q5 and Q41. Not read: every other member file,
every consolidation, `DROPLIST.md`, `PERSONA_CALLS.md`, `PRIOR_CALLS.md`, the `SEED_*` files, the
archive, and every probe directory other than my own. So where any of this restates something, I do not
know it.

**I did not open `97`'s probes.** I reproduced its model from the same CSVs rather than reading its
implementation, deliberately, so that p6 is an independent instance rather than a review. The cost is
that I cannot say whether its extreme-ray decider is correctly implemented, only that its outputs match
mine on this input.

**Everything is single-threaded and one host.** Every finding above is a `threads = 1` finding, which is
a region rather than a silence, and every timing is from runs taken on one machine.

**I could not settle whether the spread coordinate belongs in the coordinate set.** It changes the
Pareto front in 4 of 6 regions, it produces a section no other weighting produces, and no intent names
it. Distinguishing "real decision axis" from "measurement noise the harness surfaced" needs repeated
runs on different hosts, which do not exist.

**I could not settle whether the generation order is affordable.** Section 3 argues the table should be
derived from a stated weighting rather than checked against one, and that requires the coordinates to be
commensurable enough for a weight vector to be written down. Section 5 says two of the four intents have
no coordinate at all today, so the honest position is that the generation order is the target and the
check is the interim, and I did not establish how far off the target is.

**I did not attempt the axis set, the resolution question, or the chain question.** `97` section 4 is
where the resolution answer sits and I have no evidence bearing on it. `93`'s F7 and `94`'s W7 own the
chain question and I add nothing.

## 13. Coverage of the citations

Every `file:line` in this document was opened and its content tested rather than merely resolved, by
`98_probes/p11_verify_my_citations.py`, which is `25` section 9's instrument applied rather than
admired. The count and the current state are in that probe's committed output.
