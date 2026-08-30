# 151. The candidate revised against four signatures

**`146` stays exactly as landed.** All four signatures cite it by line and this file supersedes it by
naming what changes. An agent outside the topic checks the compression after this.

**Member:** Leroy, resumed. Read for this file: `147`, `148`, `149`, `150` in full with their probe
outputs; `146` and `145`, which are mine; and greps and direct reads into `139` through `144` wherever
a repair needed a citation rather than a memory. Nothing else new. Five probes in `151_probes/`, each
committed with its output as it ran.

---

## 0. What this supersedes, and what stands

**Superseded.**

- **`146` section 5.5's first predicate block**, which is **false**. Section 1.
- `146` section 5.5's container predicate, whose intersection is empty on two dimensions rather than
  merely missing one. Section 4.
- `146` section 1.1's contamination scoping, in three directions. Section 2.
- `146` section 6.1's *reason* for shipping the firewall unpredicated, and its account of the cost.
  Section 3.
- `146` section 6.3, which gains a dependency on 6.1 rather than standing beside it. Section 3.3.
- `146` section 6.2, which is one choice short, and whose claim that `144` "does not say so" is
  withdrawn. Section 5.1.
- `146` section 4's rendering of F144-2, missing the coordinate count. Section 5.3.
- `146` section 5.6's scope sentence and its "one in nine". Sections 5.2 and 5.4.
- `146` section 5.7, which drops `144` F144-10. Section 5.4.
- `146` section 10's statement of the markdown class, which is narrower than the evidence. Section 7.
- Two citation imprecisions in `146` sections 0.2 and 1.1. Section 7.
- And in `145`: section 2's wrapping clause, contradicted by its own committed output, and section 7's
  generalisation of the gate finding. Sections 5.2 and 8.

**Stands.** Everything else in `146`, and specifically what each signature confirms by name: the
ordering finding and its three-way refinement, the rescoping as a narrowing rather than a refutation,
the one-expert placements, the retirements including F2' and the seven-arm table, the count clause, the
join at 5.4, the withdrawal at 1.4, the portability inversion at 5.7, and section 7's refusal to reopen
the rounding topic.

**Added.** One clause `149` earned this round and nobody had: the three tables in 5.5 are one question.
Section 6.

---

## 1. The clause that is false, and the repair

`147` and `149` independently found `146:420-427` false. Neither read the other, and two unratified
files agreeing is not corroboration, so I measured it rather than conceding on two reports.

`151_probes/v1`, exhaustive over all 262144 triples per cell at `W = 6`, in Python against their two in
Rust:

| unsigned, wrapping | F=0 | F=1 | F=2 | F=3 | F=4 | F=5 |
|---|---|---|---|---|---|---|
| floor, ceiling, toward-zero, away-from-zero, half-up | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| **nearest-half-even** | 0.00% | **12.50%** | **12.50%** | **9.38%** | **6.25%** | **3.91%** |

Under saturating, nearest-half-even is 0.93% to 2.18% and the other five are zero everywhere. That is
`147` F147-1 and `149` F149-2, reproduced.

**Every control holds.** My integer fast path agrees with the rational definitions at every one of six
modes by four denominators by 81 numerators. The signed wrapping rows reproduce `142` F142-3 digit for
digit, toward-zero at 1.64 / 5.54 / 12.34 / 22.22 / 33.40 and half-even at 12.50 / 12.50 / 9.38 / 6.25 /
3.91, so this is the same measurement as the topic's and not a different one that happens to disagree.
The reach control finds inexact shifts at 65536 to 233472 triples per `F`, and the non-vacuity mutant
differs at 258048 of 262144.

**So the clause claims a region containing a case where it is false, at up to one triple in eight, under
the IEEE default rounding mode.** That is worse than an omission. A design taking the candidate at face
value would fuse a multiply-add on an unsigned type under half-even and change answers, having read a
canon sentence saying the cell is free. It is the failure the firewall exists to prevent, arriving
through the canon rather than through a cost model.

**`147` traces it to its own sentence and the candidate carried it faithfully.** `142:388-389` says
fusion "is free under unsigned regardless of mode by the congruence argument", generalising across the
rounding axis from a mechanism rather than from a sweep, and `142`'s own `q2` part B, which is F142-2's instrument, pins the signed
flag at `142_probes/q2_equivariance_partitions_the_rounding_axis.rs:236`. Nobody swept the unsigned half
over six modes. `149` reaches the same place from the argument kind: a congruence of the reduction says
nothing about a rounding relocation, and the arm relocates both.

### 1.1 The repair, which is `149`'s and is one clause where the candidate had two

The obvious repair is to strike nearest-half-even from the unsigned list. `149` proposes better, and I
checked it because it is the part that decides whether the fix is one clause or two.

`v1` A3 and A4: equivariance restricted to the domain the cell reaches moves toward-zero and
away-from-zero from the non-equivariant side to the equivariant side, giving **five against one under
unsigned and three against three under signed**. And the restricted test **predicts the measured table
at 12 of 12** (mode, signedness) cells while the unrestricted test **mispredicts exactly 2**, both under
unsigned, both in the direction of calling a free arm unavailable.

> **Fusing a multiply-add is answer-preserving exactly where the rounding position is translation
> equivariant on the domain the cell reaches.** Under unsigned that is five of six positions, because
> the domain excludes the negatives where toward-zero and away-from-zero differ from their equivariant
> twins. Under signed wrapping it is three of six. **Signedness stops being a case split and becomes
> what determines the domain.**

*holds for: W = 6; F in {0, 1, 2, 3, 4, 5}; signedness in {unsigned, signed}; overflow in {wrap,
saturating}; rounding in {floor, ceiling, toward zero, away from zero, nearest-half-up,
nearest-half-even}; operation = multiply-add; arity = 3; chain length = 2; container width = declared
width; inputs exhaustive over the declared range; threads = 1; target features = host
(aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f). **Argument kind: equivariance**, with the
domain restriction as the quantifier, on three independent instruments (`147` F147-1, `149` F149-1,
`151_probes/v1`).*

**One expert on the formulation, three instruments on the table underneath it.** `149` names the
specific exposure itself: `y2` decides equivariance over a window it chose, and a clean sweep does not
settle the property the way a counterexample settles its negation. My `v1` uses a different window and
agrees, which is a second instrument on the restricted test and not a proof that the window is the
right one. If the formulation falls, the enumerated fallback stands on the measurement alone.

---

## 2. The contamination scoping, wrong in three directions

`146:45-56` scoped the shared workspace rule to "the unsigned half of the fusion result, the unsigned
accumulator cells, and `140`'s own refuted P3", adding that it "does not reach anything measured rather
than mechanised". Three corrections, from two signers, and I reproduced both mechanical ones.

**Narrower on the mechanism** (`147`). Only the unsigned **saturating** cell rests on the one-sidedness.
`147` F147-2, reproduced at `v2` B4: making the clamp two-sided on an unsigned domain takes the
saturating fusion cell from 0.00% to between 6.19% and 15.50% with the low clamp engaged at 13792 triples, and leaves the wrapping cell at
0.00% at every `F`, because wrapping has no clamp and rests on `141` F3's absorption theorem, which this
topic proved rather than inherited.

**Wider on the reach** (`147`). "Does not reach anything measured rather than mechanised" is true of
every measurement in `139`'s and `142`'s corpus and **false of the one clause those measurements
produced**, because `142:388-389` is not a measurement and it became section 5.5's first predicate. The
criterion was right and the sweep for it missed a sentence, which is the harder half: a mechanised claim
is easy to find when it is labelled as a mechanism and invisible when it is written as a conclusion.

**The third item is refuted outright** (`148`). `140`'s P3 was on the **overflow** axis while the
contaminating rule concerns the **intermediate** axis. `v2` B1: at a single addition the two
intermediate positions coincide at every overflow position **including wrapping**, where the congruence
says nothing at all, so a zero there is structural rather than congruential. The congruence needs two
reduction sites and a single addition has one. Both controls fire, on `148_probes/p1` and on my `v2`: at unsigned subtraction the two saturating
positions separate at 496 of 1024 pairs, and at a multiply-add the congruence is visibly the mechanism,
differing at 11986 triples signed and 0 unsigned.

**And one thing `146` did not say, which `149` reports against itself.** `141` inherited the same
workspace rule, so its `p4` T1 is not a third independent **mechanism** on the unsigned half either. The
column convergence is three instruments and, on the unsigned half specifically, fewer than three
independent mechanisms. That follows from what `146` says and `146` does not draw it.

**And one the ledger should keep** (`148`): `139` and `140` made the *same wrong prediction* at that
shape, three where the answer is two, by different mechanisms. That coincidence is worth a later
reader's attention precisely because it is the shape a shared input produces even where the input is not
the cause.

---

## 3. The firewall: both signers accept it unpredicated and both say my reason was wrong

### 3.1 The reason, replaced

`146:557-560` gave the reason as the dimension vocabulary: a predicate over widths and operations would
misdescribe a claim about what a cost model may do. `149` is right that this invites the reply "then
find the right dimensions", and its reason is categorical and is the one to carry:

> A predicate records the region in which a claim was **established**. The firewall is not established,
> it is **imposed**. Applying the notation to it would say the constraint holds only where somebody
> measured, and under I13's absence rule that means the design is free to violate it everywhere
> unmeasured. **The notation would not merely misdescribe the claim, it would invert it.**

So the predicate discipline governs findings and this is not one. `RULES.md:486-541` says so in its own
first sentence, and nothing in the discipline is being waived. `147` reaches the same acceptance from
the other side: I13's own scope limit at `INTENTS.md:263-267` says the dimension list is elaboration
rather than ratified, so requiring numeral dimensions on a design proposition applies the elaboration
past what op ratified.

### 3.2 The cost, which `146` overstated, and what is gateable

`147` is right that `146:561-566` overstates it. The firewall's gateable form is **the enforceability
condition at `146` section 5.4**, which is `145_probes/z3`'s and already carries a predicate over the
assignment and arm sets. A sentence that cannot be composed with an arm was the stated cost, and this
one can be, through the obligation it imposes on the arm set.

**And `149` supplies a fourth piece nobody had**, which turns the constraint from a sentence a reviewer
agrees with into a condition an arm is tested against:

> **The violation predicate.** An arm that relocates a reduction violates the firewall exactly where
> the reduction is not a congruence for the following operation on the reachable domain. An arm that
> relocates a rounding violates it exactly where the rounding is not translation equivariant on the
> reachable domain.

So the honest statement is **four pieces, three of them gateable**: an unpredicated imposed intent; a
predicated enforceability condition on the arm set; a predicated measurement of the consequence
(`144` F144-15, on `144_probes/p6`); and a predicated violation test for a candidate arm (`149` F149-1
and F149-3, on `149_probes/y1`, `y2` and `y4`). The
ungateable one is ungateable because it is an intent, not because nobody did the work. And `147` asks
that what it is quantified over be written down rather than left for a later reader to decide: **cost
models and arms.**

### 3.3 And 6.3 inherits 6.1, which `146` presents as independent

`148`'s qualification, and it is a structural claim about the candidate rather than a measurement.

Section 6.3 closes the count's second argument by declaring that an axis visible under the maximal
observation set **is** component one. That declaration is not forced by the visibility measurement. It
is forced by the firewall, which 6.1 records as unpredicated, and the axis it classifies is the one
`144` F144-16's biconditional ranges over.

`148_probes/p2` makes it concrete rather than arguing it: the same axis, the same measurement, two
classifications. A live axis differing at 9722 triples is component one under the firewall and under
declared slack 0 and 8, and **component two once the slack reaches 16**, with nothing about the arms
changing. Three controls fire, including a dead axis that is component two under every rule, so the
classifier does not manufacture policy content.

**This does not weaken 6.3 and `148` does not ask for it to be withdrawn.** What it changes is that op
should have the coupling before answering 6.1 rather than after: **if a canon may not carry an
unpredicated proposition, 6.3 does not survive unchanged.** They are one decision.

`148` names its own exposure: the SLACK rule is its construction of what a non-firewall design would do,
and the slack-zero control is what makes it not a strawman. A different non-firewall classification
under which the live axis stays component one would refute it.

---

## 4. The intersection is not merely an upper bound, it is empty, and my instrument was measuring names

`148` finds `146` section 5.5's container predicate omitting `signedness` because the three instruments
**partition** that dimension rather than overlapping on it, and names the general defect in my `z6`: it
intersects over dimension **names**, so a dimension every instance lists with disjoint values vanishes
while looking present in every input. It says it did not check the other rows.

**I fixed the instrument and checked them.** `151_probes/v2` parses values rather than names and
intersects those. The result is worse than `148` reported and **my prediction of what it would find was
refuted**:

| convergence row | dimensions whose value intersection is EMPTY |
|---|---|
| the container is answer-invisible at the column | `W`, `rounding` |
| the accumulator is visible exactly at signed saturating | `rounding` |
| the class count is monotone and not strict | `assignments`, `witness sets` |
| fusion is an axis position already | none |

**At the container row the empty dimensions are not `signedness`.** `139` omits signedness entirely, so
the name-level check already dropped it, and what actually intersects to nothing is `W` (`139` swept
{3, 5, 6, 7, 11} where the other two swept 4) and `rounding` (`139` swept truncate where `141` swept
floor and toward-zero). `148`'s diagnosis of the mechanism is right and its identification of the
carrier at this row is not.

**Three of four rows carry it**, which nobody had checked.

**So a clause built on one of these rows claims nothing where those dimensions are present**, and
`148`'s repair is the right shape applied per dimension rather than only to signedness: cite the single
instance that spans what the clause wants to claim, for that dimension. `141:149-152` spans both
signednesses alone, so it is the citation for that axis, and `142` F142-5 and `143` F1 are the fold
instruments the accumulator row rests on. The container clause becomes two sentences: the
three-instrument intersection as written, plus one instance establishing the dimensions the intersection
cannot. The three instruments are `139_probes/p6`, `140_probes/p3` and `141_probes/p4`.

**And my probe produced a false finding on its first run, which no control caught.** It reported
`threads` intersecting to empty, which is impossible since every block in the topic says `threads = 1`.
The cause was my block pattern over-capturing past the predicate into surrounding prose, so a trailing
dimension swallowed a paragraph. I caught it by finding the number implausible, which is the weaker way.
The added control drops keys that are not dimension-shaped and names them; its **first version rejected
whole blocks and rejected 42 of 42**, leaving every row empty, which is the same defect inverted: a
check that cannot pass is as uninformative as one that cannot fail. Both versions are in the probe.

---

## 5. `150`'s four

### 5.1 The baseline conflict is two orthogonal choices, and `146`'s claim that `144` is silent is withdrawn

`150` accepts that something is wrong and that it is `144`'s, and says both `145`'s reading and `146`'s
are one dimension short. The per-coordinate division carries two independent choices: **units** (is a
weighting declared in absolute cost or in units of a named baseline arm) and **baseline provenance** (is
the baseline's cost vector fixed once or re-measured per target). `144` section 6 varied the first and
section 4.3 varied the second while holding the first implicit.

`151_probes/v3` runs the two-by-two on one shared population, and the structure reproduces:

| units | baseline | pure per-coordinate rescale | arbitrary target change |
|---|---|---|---|
| absolute | fixed | 16.6% | 87.1% |
| absolute | per-target | 16.6% | 87.1% |
| relative | fixed | 15.2% | 87.1% |
| relative | **per-target** | **0.0%** | 84.1% |

That is `150` F150-1. **The invariance is one cell of four and needs both choices.** Relative units with a baseline fixed once
do not give it. Three controls fire: the two absolute rows are identical digit for digit, as they must
be since the baseline never enters an absolute-units selection; every cell moves under an arbitrary
target change, so the invariant cell is a change of units and not a claim that targets do not matter.
My rates differ from `150`'s 12.0 / 11.6 because the draw differs; the cell structure and the zero are
identical.

**And `150`'s consequence, which neither `145` nor `146` drew, is the one that dissolves the conflict
properly.** `150` F150-2, reproduced at `v3` C4: under baseline-relative units on the simplex the baseline arm's own
reported figure is **identically 1** at all 3640 cases, and the reported ranking equals the selection ranking at all
3640. So the reporting normalisation `140`'s obligation asks for is **division by one**, and there is no
second operation to place. The two sections stop competing rather than needing a rule about which wins.

**And `146`'s sentence that `144` "does not say so" is withdrawn.** It does, at the level of two
operations, at `144`'s own section 6 closing paragraph and at its O-144-C, which states the fork
directly. What it does not say is **which two choices generate them**, and that is the correction
`150` supplies about its own file.

**What the design owes is two declarations, not one**, replacing `146` C2: which units a weighting is
expressed in, and whether the named baseline is fixed once or re-measured per target. Only the
conjunction buys portability, and the second decides whether a cost claim compares across targets at
all, because a per-target baseline makes "0.8 of baseline" two different absolute costs on two machines.
That trade is in neither file, and it composes with `144` F144-11's inversion and F144-12's rescale
result and F144-13's 24.6%, which are what the two choices were measured against.

### 5.2 The gate has a direction and it is neither of the two I implemented

`150` says `144_probes/p10c` asks an arm to **earn** non-dominance by beating every rival somewhere
above the band, where both of `z2`'s readings ask it to **escape** an established domination.

`151_probes/v4` implements all three over the same 35 families, and every prediction confirms:

| k | earned | escaped-conservative | escaped-symmetric |
|---|---|---|---|
| 0.0 | 3.03 / 29 | 3.03 / 29 | 3.03 / 29 |
| 0.25 | 1.26 / 11 | 3.06 / 29 | 1.77 / 20 |
| **0.5** | **0.77 / 4** | 3.14 / 29 | 1.89 / 20 |
| 2.0 | 0.60 / 2 | 3.26 / 29 | 2.23 / 21 |

That is `150` F150-3. **The earned reading is monotone non-increasing**, is a subset of
escaped-conservative in every family and every `k`, and leaves **4 askable families at `k = 0.5`**
(`150` F150-4), reproducing `150`'s number exactly on a
separately written implementation. `150`'s discriminator control fires: on a family of exact duplicates
escaped-conservative reports four arms non-dominated and earned reports zero.

**So `145` section 7's third disagreement is wrong as generalised.** `z2`'s B2 refutation is correct
about the two readings it implemented and false as a claim about "a gated dominance test": a gate that
has to earn a positive claim has one comparison, and one comparison cannot pull two ways.

**And the zero survives all three** (`150` F150-5), which is `150_probes/q2`'s CC4 and which refuted its
own prediction. So
the direction of conservatism changes what F144-18's zero is a statement **about** and not the zero
itself, and O-139-C closes on the same side however the gate is read.

> `146` section 5.6's scope sentence is replaced: **4 askable families under the gate `144` used, 29 and
> 20 under the two `145` implemented, and zero established instances under all three.** The last clause
> is what makes the sentence worth having, and neither `145` nor `146` had it.

### 5.3 F144-2 needs its coordinate count

`146` section 4 says the arm "loses by at least one unit at every point of the simplex" without saying
which simplex. `150` F150-8, reproduced at `v3` C5: the optimum is **+1 at two cost coordinates and exactly 0 at
three** with a constant third, and the three-coordinate zero needs no solver, because at `w = (0, 0, 1)` every arm
scores zero on the constant coordinate and ties by inspection. `139`'s hull-interior control arm at
(4, 4) comes out at `-1`, so the procedure distinguishes a selectable arm from an unselectable one.

That is `144` F144-4, which `146` states in 5.6 without connecting it to the sentence in section 4 that
needs it, and the reachability half is `144` F144-6's construction rather than this one. The clause gains `cost coordinates = 2` and a pointer to the dead-coordinate result.

### 5.4 Two smaller repairs

**`144` F144-10 goes back in.** `146` dropped it. Nothing in the candidate cites 44.3% as a magnitude, so no
hazard was created, but `139:396`'s 44.3% stays in `139` with nothing beside it recording that it sits at
the **2nd percentile** of a distribution whose median is 90.2%. A candidate is what a later reader reads
instead of the topic, so a correction it drops is a correction that stops travelling. Section 5.7 gains
one clause: the cross-target switch rate is a fact about how much two cost tables differ and not about
weightings.

**"Roughly one non-dominated arm in nine" names its cell.** `144` F144-5's range is 6.6% to 12.7% across nine
cells; one in nine is the `d = 3, n = 8` cell at 11.7%, which is the right one to quote since three cost
coordinates is what `139` proposes, and the sentence should say so rather than reading as a summary of
the range.

---

## 6. The clause `149` earned: the three tables are one question

`149` did not re-propose its unlabelled replacements on the strength of having said them once. It tested
the claim, and `y4` establishes it with a mutation control:

| reduction | signedness | congruence | fusion difference | accumulator visible |
|---|---|---|---|---|
| wrapping | either | true | 0 | 0 |
| saturating | unsigned | true | 0 | 0 |
| saturating | signed | **false** | **760** | **476** |

The congruence property predicts the fusion cell at 4 of 4 and the accumulator cell at 4 of 4, and the
two coincide. The reach controls hold, and a deliberately non-congruent reduction shows a nonzero fusion
difference and a visible accumulator **in exactly the cells where wrapping and saturating are all zero**,
so the congruence test drives the prediction rather than something incidental.

> **A relocation is free exactly where the thing relocated commutes with what it is relocated across, on
> the domain the cell reaches.** A reduction relocation is free where the reduction is a congruence. A
> rounding relocation is free where the rounding is translation equivariant. The fusion cell, the
> accumulator cell and the intermediate axis are the same question asked three times.

*holds for: W = 4; F in {0, 1}; signedness in {unsigned, signed}; overflow in {wrap, saturating};
accumulator width in {W, W + 2}; fold length = 3; operations {multiply-add, sum fold}; arity = 3; chain
length in {1, 2, 3}; container width = declared width; threads = 1; target features = host. **Argument
kind: closure with a mutation control**, one expert (`149` F149-3).*

**Section 5.5's three blocks stay underneath it as the instances that establish it.** This is not a
replacement of the tables and `149` does not ask for one. It is a statement of what they are three
instances of, and it is what a canon is for: it survives a rewrite, and a later reader needing the answer
at a shape nobody swept gets it from the property and cannot get it from the tables.

---

## 7. The closed rounding topic, now settled by two readings, with a warning

`149` agrees with `146` section 7 that the rounding candidate at `132`, revised at `136`, should not be
reopened, and gives firmer ground. `146`'s reason was about ownership, which invites the reply that a law
table missing a law should gain it. `149`'s is structural:

> The property is equivariance **on the reachable domain**, which is a property of a (mode, domain)
> pair. Toward-zero has it under unsigned and lacks it under signed. **A per-mode law table has no cell
> to put that in.**

`v1` A3 measures exactly that split, so I have it on my own instrument as well as `149`'s `y2`. And the
naming obligation inside `141`'s retired replacement B survives as `131` F131-3, with `142` F142-4 and
`149` F149-4 measuring what the swap costs on signed shapes.

**And the warning, which is the part to carry.** `142:266-269` recommends that the canon record, per
mode, whether it is translation equivariant, and `142`'s O-142-A proposes it, resting on F142-2's unrestricted partition. **Adopting that into `132`
as worded would introduce an error**, because the value it would record is the unrestricted one, which
mispredicts 2 of 12 cells, both under unsigned, both in the direction of calling a free arm unavailable.
The rounding candidate would go from incomplete to wrong, and it would go there through the mechanism
this panel has been most careful about: a true finding carried across a boundary where its quantifier
does not hold.

**So: do not reopen `132`, and do not adopt O-142-A as worded.** If op reads the reopening question the
other way that is op's call, and what this file adds is that the version most likely to be proposed is
the one that would do damage.

---

## 8. Small and exact

**Two citation imprecisions in `146`, both found by two signers independently and both confirmed.**
`146:21-22` says `137_probes/g0_test_gate.out` "ends" with `123 passed across 13 crates, 0 failed`; that
string is at **line 16 of a 22-line file**, which continues with five lines on the concurrency
diagnosis. And `146:42` cites `140:6` for the blindness disclaimer; the sentence is entirely on
**`140:5`**. The containing range citation `140:3-7` is correct.

**The markdown class is wider than `146` section 10 states, and `150` is the sixth instance.** Its
checker was defeated by blockquote markers, a token none of the previous five used, because collapsing
whitespace leaves a stray `>` mid-sentence that stripping backticks and emphasis does not touch. Its
reading is that the fix is a normaliser written against a grammar rather than another token in a strip
list, and that it did not write one.

**I wrote one.** `151_probes/v5` normalises against markdown's inline grammar, stripping block prefixes
per line before whitespace collapses (which is the only point at which a `>` is distinguishable from a
`>` in prose), then images before links, then code spans and emphasis as runs rather than characters. It
recovers the phrase under **all eleven constructs tested**: plain, backticks, bold, italic, blockquote,
heading, list item, and three nobody has hit yet, nested emphasis, link syntax and a hard line break.
Both controls hold: a phrase nobody wrote stays absent under every construct, and a real phrase read at
the wrong span still fails, so it has not become a substring search. It is the fifth instrument in this
panel after `119_probes/r1`, `132_probes/w1`, `145_probes/z7` and `146_probes/w2`, and the first written
against a grammar.

**And `148`'s three-way refinement of the ordering finding, accepted.** `146` said the ordering runs the
wrong way for `140` and establishes nothing about the between-file half. `148` adds a third commit:
its four phase-one probes landed at `33c9b212` **10:14:56**, two minutes and eleven seconds before
`139`'s file existed. So the ordering establishes nothing about `140`'s **prose** and does establish
something about its **measurements**, which is the part a later reader would most want dated. `148` does
not ask for a stronger rung and I am not giving one; it is a correction to the ledger's precision.

---

## 9. What I got wrong, collected

Six things across `145` and `146`, four of them found by signers.

1. **`146` section 5.5's first predicate is false.** `147` and `149`, reproduced at `v1`. It came from
   `142:388-389` and the candidate carried it faithfully, which is not a defence: a candidate is where a
   claim stops being one author's.
2. **`145` section 2's wrapping clause is contradicted by its own committed output.** `150` D1, confirmed
   at source in `v4` D6: `145_probes/z3_output.txt:88` reads `overflow = wrap conforming-arm counts
   observed: [2, 4]`, and four rows show a wrapping assignment with two conforming arms, two lines above
   a verdict saying every arm conforms. **That is the same defect `136` section 9 records against `x4`,
   occurring a second time in the same file and not caught by me.** The corrected statement is `150`'s:
   the conforming-arm count is a function of the **whole assignment**, rounding included, not of the
   overflow position, which is `150` F150-6 on `150_probes/q3`. The mechanism is `142` F142-2: absorption
   relocates a reduction, the residual is a rounding relocation, and toward-zero is not equivariant.
3. **`145` section 7 generalised the gate finding past the readings it implemented.** `150`, reproduced
   at `v4`. B2 holds for the reading `144` used.
4. **`146`'s contamination scoping was wrong in three directions**, two of them mechanical and both
   reproduced at `v2`.
5. **`146`'s reason for the unpredicated firewall was the weaker one**, and its account of the cost
   overstated it. `149` and `147`.
6. **`z6` intersected names rather than values**, so it reported a dimension present when the instances
   partitioned it. `148` named the class; `145_probes/z6` is the instrument, and `v2` fixed it and found
   the defect in three of four rows.

**And two defects in this file's own probes**, recorded rather than repaired quietly. `v2`'s first run
produced a false finding that no control caught, and its first control over-corrected to rejecting every
block. Both are in the probe.

---

## 10. What only op decides

Unchanged from `146` section 8 except where a signature moved it.

**Whether a canon may carry an unpredicated proposition** (`146` 6.1), and this now reaches further than
`146` states: per section 3.3 it decides section 6.3 as well. **They are one decision and op should have
that before answering.**

**Which selector the design ships** (`144` O-144-A), with the corpus answer now invariant to the gate
reading and its scope narrowed to 4 askable families.

**Which units a weighting is expressed in, and whether the named baseline is fixed once or re-measured
per target.** Two declarations rather than one, per section 5.1, and only the conjunction buys
portability.

**The operation set the design ships**, which is what the shape-to-count table waits on, and `150` adds
F150-7 adds that it waits harder at unsigned, where a design shipping four of the five operations would report the
intermediate axis dead and be wrong.

**Whether the accumulator cell is stated conservatively or with a schedule dimension** (`142` O-142-B).

**Whether the default rounding position is chosen for familiarity or for what it licenses**, sharper
after section 1: the IEEE default is the one mode that is not free under either signedness.

**How many named presets there are and what they are called.**

---

## 11. Anchor accounting

Counted on `151_probes/v5`, which reuses `119_probes/r1`'s stripper, with this section excluded from the
computation. The union is this topic's twelve files, `139` through `150`.

```
  class                    in the union   in 151   not carried
  finding ids                        50       33            17
  option ids                         18        5            13
  probe stems                        38       21            22
  line anchors (panel)              100       14            88
```

**The table is measured rather than predicted, and that sentence is here because the first draft of it
was not.** I wrote it from expectation, `v5` reported 12 findings carried where I had written 33, and
restoring the load-bearing anchors at their points of use is what closed the gap. **That is the third
consecutive candidate of mine whose accounting table was drafted from memory**, and the pattern is worth
naming rather than fixing quietly: the table sits at the end, the anchors are in the body, and nothing
about writing the number forces a reader of it to have run the instrument.

**What the seventeen uncarried findings are.** `141`'s F4 through F9 and `142`'s F142-1 and F142-6, plus
`144`'s F144-1, F144-3, F144-7, F144-8, F144-9, F144-14, F144-17 and F144-19. Every one supports a clause
this revision does not change, so each lives in `146`, which stands, and in its own file. The last entry
in `v5`'s list is `F99`, which is `149`'s deliberate nonexistent-id control and is correctly absent.

**Thirteen option ids and eighty-eight line anchors are not carried**, and both counts are the shape of a
revision rather than a defect. Four signature files cite `146` and each other by line in making their
cases; this file cites the clauses it changes.

**A revision's `not carried` count is not comparable to a compression's**, and the reason is visible in
the shape: `146` carried 33 of 35 findings because it was stating the whole topic, and this file carries
33 of 50 because it is stating what changed against a union that grew by four signature files. What matters is that every clause this file supersedes
carries the anchors it rests on, which is a hand check against `v5`'s lists rather than a count.

**No line anchor into shipped source is carried and none is owed**, because the code tier is the one that
gets rewritten.

---

## 12. Coverage, bounded

**Read in full:** `147`, `148`, `149`, `150`, and `145` and `146`, which are mine. **Opened at source
rather than recalled:** `137_probes/g0_test_gate.out` in full; `140:3-7`; `142_probes/q2`'s signed flag
at `:236` via `149`'s citation; `145_probes/z3_output.txt` in full; `145:110`; `146:21-22`, `146:42`,
`146:420-427`, `146:557-566`; the committed CSVs under `mock/benches/`.

**Not read:** the probe sources of `147` through `150`, except where an output is quoted; `139` through
`144` end to end, which I read for `145` and `146` and did not re-read; `40`, `93`, `102`, `106`, `107`,
`108`, so where Q51 compresses them I rely on Q51's account.

**Test gate: passed, at 123 across 13**, inherited from `137_probes/g0_test_gate.out` with the citation
corrected in section 8. `150` re-ran it this sitting and reports nothing under `mock/crates`,
`mock/benches/variants` or `mock/benches/src` changed since, by command rather than by memory, and the
`-- --test-threads=1` requirement now has **five** reproductions. I did not re-run: `df` reports 4.3 GiB
free and thirteen release builds do not fit, which is why every probe here is Python.

**Canon gate: passed.** Nothing proposes a design decision or presumes the strategy set closed. Section 6
states a property and leaves its construction open per I16 at `INTENTS.md:317-331`. No predicate carries
a hedge token.

**Built:** five probes, twenty-nine predictions, **twenty-eight confirmed and one refuted** (`v2` B5,
which named the wrong dimension as the empty one). Two defects in my own instruments, in section 9.

**I priced nothing.** No claim here is a bench result. `v4` reads committed harness output and does
arithmetic on it. **Everything fixed-point is at model widths**, `W` in {4, 5, 6}, with no transfer
argument to 64 bits, and **everything is `threads = 1`** except where a predicate says the harness ran it
otherwise.
