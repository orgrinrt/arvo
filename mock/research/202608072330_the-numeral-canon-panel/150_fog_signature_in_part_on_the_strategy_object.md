# 150. Signature in part on the strategy object

I am `144`, resumed to sign `146`. **I sign in part.** Most of the candidate's second component is my
work and it is represented well; two of the four things I was asked to settle come back with answers that
change what the candidate should say, and one of them is against a file rather than against the candidate.

The outcome first.

**The conflict `145` found inside my file is real, and both readings of it are one dimension short.** The
per-coordinate baseline division carries two independent choices, units and baseline provenance, and my
two sections varied one each while holding the other implicit. They are orthogonal, `q1` measures the
two-by-two, and **the cross-target invariance appears in exactly one cell of four and needs both choices**.
So there is no operation that is required in one section and forbidden in the other. What is wrong in my
file is one clause, and I name it below.

**My gate has a direction and it is neither of the two `145` implemented.** `144_probes/p10c` asks an arm
to *earn* non-dominance by beating every rival somewhere above the band; both of `z2`'s readings ask it to
*escape* an established domination. Mine is stricter than either, it is **monotone non-increasing in the
gate strength**, which is `z2`'s own refuted prediction B2 holding for the reading `144` used, and at
`k = 0.5` it leaves **4 askable families rather than 29 or 20**. That is a larger narrowing of my own
finding than `145` reported, and it is against me.

**And the zero survives all three readings.** `q2` re-runs the unreachability survey with only the
non-dominance reading varied: zero established instances under earned, escaped-conservative and
escaped-symmetric alike. So the direction changes what F144-18's zero is a statement *about* and does not
change the zero. `146` section 5.6 should say that, and it currently says something weaker and, on the
count, wrong.

**One dissent is against `145` rather than `146`, and it is contradicted by `145`'s own committed output.**
`145` section 2 states that at `overflow = wrap` all four lowering arms conform, citing its z3 C4. z3's C4
table prints `2` for `toward_zero/wrap` at `W4 F1 s` and `W4 F2 s`, and z3's own summary line two lines
above its verdict reads `overflow = wrap conforming-arm counts observed: [2, 4]`. `q3` reproduces the
split independently at `W = 6`. The structural claim survives and gets sharper: **the conforming-arm count
is a function of the whole assignment, rounding included, not of the overflow position.**

**The quantifier `145` added to my visibility property is right, it composes rather than replaces, and my
own committed artifact is consistent with it.** With one addition: whether the maximal observation set is
load-bearing is itself shape-dependent, which sharpens `146` section 6.3 rather than softening it.

Five probes, one prediction of mine refuted, and the refuted one is the one I most wanted confirmed.

---

## 0. Gates

**Canon gate: passed.** Checked against `INTENTS.md` entry by entry. I1 is demoted to open at
`INTENTS.md:51-61`, so signing on what a strategy is remains licensed. Nothing here argues for dropping or
downgrading the storage-minimising concern (I17, `INTENTS.md:363-383`). I13 at `INTENTS.md:214-235` is the
rung my findings are written against and its scope limit at `:263-267` is respected: I cite the dimension
list as this panel's notation rather than as ratified authority. No predicate below carries a hedge token.
I16 at `INTENTS.md:317-331` bounds section 2's replacements, which name what a design must declare and not
what shape the declaration takes.

**Test gate: passed, on my own run rather than an inheritance.** I ran all thirteen crates in full earlier
in this sitting for `144`: 78 tests across eleven crates, 30 in `wide-rung-shared` at 133.69s, and 15 in
`bitpack-write-contend-shared` under `-- --test-threads=1` at 3.03s in release. 78 + 30 + 15 = 123.

Nothing under `mock/crates`, `mock/benches/variants` or `mock/benches/src` has changed since, which is a
command rather than a memory:

```
$ git log --oneline --name-only 244aabb9..HEAD -- mock/crates mock/benches/variants mock/benches/src
(no output)
```

and a spot re-run confirms it: `satfold-shared` at 11 passed, `bitpack-write-contend-shared` at 15 passed
under the flag. **So the `-- --test-threads=1` requirement now has five reproductions**, `139` at 7.97s,
`141` at 46.65s, `143` at 2.57s, mine at 3.03s and this one at 3.22s, against `139`'s livelock diagnosis
in `pool.rs`, which nobody has needed to re-derive.

**Test quality.** I read bodies in `bitpack-carrier-shared` for `144` and reported the verdict there. I
add nothing to it here and I do not make it a sixth reading of a sixth crate; five members have now read
five crates and agree.

---

## 1. The two things only I could settle

### 1.1 The baseline conflict is two orthogonal choices, and my sections varied one each

`145` section 3.5 and `146` section 6.2 report that my sections 6 and 4.3 conflict, reproduce my 24.6% at
894 of 3640 on an independently written instrument, and offer the dissolution that the per-coordinate
division is a declaration of the weighting's units. `145` is right that something is wrong and right that
it is mine. The dissolution is one dimension short.

**The per-coordinate form carries two choices and neither file separates them.**

**Units.** Is a weighting declared in absolute cost units or in units of a named baseline arm? This is
what my section 6 varied, on one target.

**Baseline provenance.** Is the baseline arm's cost vector fixed once, or re-measured on each target? This
is what my section 4.3 varied, and it varied it *while holding units at baseline-relative*, without naming
either as a choice.

`q1` runs the two-by-two as a factorial on one shared population, measuring the rate at which a fixed
weighting's selection moves across a target change:

| units | baseline | pure per-coordinate rescale | arbitrary target change |
|---|---|---|---|
| absolute | fixed | 12.0% | 85.0% |
| absolute | per-target | 12.0% | 85.0% |
| relative | fixed | 11.6% | 87.4% |
| relative | per-target | **0.0%** | 83.4% |

**The invariance is in one cell of four and needs both choices.** Relative units with a baseline fixed
once do not give it, at 11.6%. That is the finding `145`'s units reading cannot state, because it treats
units as the whole of the mechanism.

Four controls fire. The two absolute rows are identical digit for digit, which they must be since the
baseline never enters an absolute-units selection, and my first run drew fresh tables per cell and reported
12.0% against 19.3%, which I corrected to a shared draw rather than shipping a difference that cannot
exist. The identical-target column is exactly zero in every cell. The arbitrary-change column is non-zero
in every cell, so the invariant cell is a change of units and not a claim that targets do not matter, which
reproduces `145` z5's E6 on my instrument. And a fixed baseline under relative units does move under a
rescale, so the provenance axis is measuring something.

**And there is a consequence `145` does not draw, which is the one that makes the two operations stop
competing.** Under baseline-relative units on the simplex, the baseline arm's own reported figure is
identically the weight sum, hence exactly 1, checked at all 5460 cases with zero exceptions. So every arm's
figure is *already* stated against the named arm, the reporting normalisation `140`'s obligation asks for
is division by one, and the selector's ranking and the reported-figure ranking are the same ranking because
they are the same number. **Under relative units there is no second operation to place.**

**What is actually wrong in my file, and it is one clause.** My section 6's repair reads:

> Every strategy's cost claim is stated against the same named arm, and the comparison is made after the
> weighting, on the weighted scalar. A per-coordinate normalisation applied before the weighting is a
> change of weighting, and the arm a consumer asked for is not the arm they get.

The second sentence's second half presumes the consumer asked in absolute units, which is exactly the
undeclared choice. Under relative units the per-coordinate figure **is** the arm they asked for. The clause
should be scoped to the obligation it repairs, which is about cost claims, and should read that the
comparison **for a cost claim in absolute units** is made after the weighting.

**Where `145` and `146` overstate, and it is checkable.** `145` section 3.5 says the two sections "conflict
and the file does not say so", and `146` section 6.2 inherits it. My section 6's closing paragraph says:

> And section 4.3 is the other face of the same coin: the per-coordinate form is exactly the transformation
> that makes a weighting travel across a rescaled target. It is not wrong, it is a different operation with
> a different purpose, and using one where the other is meant is the failure.

and my O-144-C states the fork: "whether the design wants one baseline for reporting or one for
normalising. They are different objects and the measurement says they cannot be the same one." So the file
does say so, at the level of *two operations*, and what it does not say is *which two choices generate
them*. That is the correction I accept and the one I am supplying.

**What the design owes, replacing `146` C2's single declaration with two.** Which units a weighting is
expressed in, and whether the named baseline is fixed once or re-measured per target. **Only the
conjunction buys portability**, and the second declaration is what decides whether a cost claim compares
across targets at all, because a per-target baseline makes "0.8 of baseline" two different absolute costs
on two machines. That trade is not in either file and it is the thing op is actually choosing between.

### 1.2 The gate is earned, not escaped, and the zero survives every reading

`145` z2 reports that F144-18's gate does not state its direction of conservatism, that its two readings
give 29 askable families against 20, and that its prediction B2, that the established non-dominated count
falls with gate strength, was refuted because a gated dominance test has two comparisons pulling opposite
ways.

**Both of z2's readings phrase non-dominance as escaping domination. `p10c` does not.** My gate asks, for
every rival, whether the arm beats that rival at some coordinate by more than the band. It is a positive
claim the arm has to earn, and it is a third reading. So the question is answerable rather than open.

**I meant the earned reading and I should have written it down.** The gate exists because I had just
published a false positive, so the thing it guards is the positive claim. Guarding the negative claim
instead admits arms that merely escape an established domination, which is the wrong direction for a
finding whose failure mode is calling an arm special when it is not.

`q2` implements all three and runs them over the same 35 families, with the unreachability test held fixed
at `p10c`'s so only the non-dominance reading varies:

| k | earned, mean / askable | escaped-conservative | escaped-symmetric |
|---|---|---|---|
| 0.0 | 3.03 / 29 | 3.03 / 29 | 3.03 / 29 |
| 0.25 | 1.26 / 11 | 3.06 / 29 | 1.77 / 20 |
| 0.5 | **0.77 / 4** | 3.14 / 29 | 1.89 / 20 |
| 2.0 | 0.60 / 2 | 3.26 / 29 | 2.23 / 21 |

Three things follow and two of them are against me.

**The earned reading is monotone non-increasing in k**, checked at every family and every step. A gate that
has to earn a positive claim has one comparison, and one comparison cannot pull two ways. **So z2's B2
holds for the reading `144` used and was refuted against two readings `144` did not use.** That is a
correction to `145` section 7's third disagreement, which generalises the non-monotonicity to "a gated
dominance test" and to `144`'s gate specifically.

**The earned reading is strictly stricter**, a subset of escaped-conservative in every family and strictly
so in 29 of 35. The discriminator control is the one that shows the readings are genuinely different: on a
family of exact duplicates, escaped-conservative reports four arms established non-dominated and earned
reports **zero**, because no arm beats an identical one anywhere either.

**So F144-18's zero is a statement about 4 families, not 29 and not 20.** That is a much larger narrowing
than `145` found, it is against my own finding, and `146` sections 1.4 and 5.6 should carry the number for
the gate `144` actually used rather than bracketing it with two that were not.

**And the zero itself survives all three.** `q2`'s CC4 finds zero established non-dominated linearly
unreachable arms under every reading, which refutes my own prediction that a looser reading would resurrect
an instance. **That is the result that matters and neither file has it**: the direction of conservatism
changes the scope of F144-18 and does not change its verdict, so O-139-C closes on the same side however
the gate is read. That strengthens the finding while narrowing it, and both halves belong in the candidate.

---

## 2. Dissents, each with the clause and what it should say

**D1. `145` section 2, the wrapping clause.** It reads: "at `overflow = wrap` all four lowering arms in
the topic's own arm set conform to every intermediate position, because the absorption theorem (`141` F3)
makes fused and stepwise the same denotation there; at `overflow = sat` the four split two and two."

That is contradicted by `145`'s own z3 C4 output, which prints `2 conforming arms` for
`toward_zero/wrap/stepwise` and `toward_zero/wrap/exact` at `W4 F1 s` and `W4 F2 s`, and whose summary line
reads `overflow = wrap conforming-arm counts observed: [2, 4]` two lines above a verdict saying every arm
conforms under wrapping. **The verdict contradicts its own table.** That is the same defect `145` section 9
records against its own z4, occurring a second time in the same file and not caught.

The mechanism is `142` F142-2: the absorption theorem relocates a *reduction*, and the residual difference
is a *rounding* relocation, which is answer-preserving exactly for translation-equivariant positions.
Toward-zero is not one. `q3` reproduces the split independently at `W = 6` against z3's `W = 4`:

| signedness | overflow | rounding | extensional classes among the four arms |
|---|---|---|---|
| unsigned | wrap or sat | floor or toward-zero | 1 at every `F` |
| signed | wrap | floor | 1 at every `F` |
| signed | wrap | toward-zero | 1 at `F = 0`, **2** at `F` in {1, 2, 3} |
| signed | sat | floor or toward-zero | 2 at every `F` |

*It should say:* the number of arms component two ranges over is a function of the **whole assignment**,
rounding included. Under wrapping it is one class for translation-equivariant rounding positions and two
otherwise; under saturating on signed shapes it is two at every rounding; on unsigned shapes it is one
everywhere by the one-sided-clamp congruence.

**`146` does not carry the wrong version**, and its section 5.5 states the corrected shape already:
"closure for the unsigned half and equivariance for the signed wrapping half". So this dissent is against
`145` and against the reading the coordinator relayed to me, not against the candidate.

**D2. `146` section 6.2 and `145` section 3.5, "and the file does not say so".** Quoted and answered in
1.1. My section 6 says it and O-144-C states the fork. *It should say:* the file names two operations and
does not name the two choices that generate them, and one clause of its repair overreaches.

**D3. `146` section 4, on F144-2.** It says the arm "loses by at least one unit at every point of the
simplex" and does not say which simplex. `q4` recomputes with the same imported solver: **+1 in two
coordinates and exactly 0 in the three-coordinate embedding `139` actually swept**, where every arm scores
zero once all the weight is on the constant coordinate. That is my own F144-4, which `146` states in 5.6
without connecting it to the sentence in section 4 that needs it. *It should say:* at every point of the
two-coordinate simplex, with a pointer to the dead-coordinate result for why the embedding matters.

**D4. `146` drops F144-10, and it is a correction rather than a number.** `q4` confirms 44.3% appears
nowhere in the candidate as a magnitude, so no new hazard is created. What is lost is that `139:396`'s
44.3% stays in `139` with nothing beside it saying it sits at the **2nd percentile** of a distribution whose
median is 90.2%. A candidate is what a later reader reads instead of the topic, so a correction it drops is
a correction that stops travelling. *It should carry:* one clause in 5.7 saying the cross-target switch
rate is a fact about how much two cost tables differ and not about weightings, with F144-10's predicate.

**D5. `146` section 5.6's scope sentence.** "over 29 askable families under the conservative gate reading
and 20 under the symmetric one" brackets the answer with two readings `144` did not use. *It should say:*
4 askable families under the gate `144` used, 29 and 20 under the two `145` implemented, **and zero
established instances under all three**, which is `q2` CC4 and which is the part that makes the sentence
worth having.

**D6, minor. `146` section 5.6's "roughly one non-dominated arm in nine".** F144-5's range is 6.6% to
12.7% across nine cells; one in nine is 11.1%, which is the `d = 3, n = 8` cell at 11.7% rather than the
range. That is the right cell to quote, since three cost coordinates is what `139` proposes, and the
sentence should name it rather than reading as a summary of the range.

---

## 3. What I confirm, and at what strength

**The quantifier on my visibility biconditional is right and it composes.** Visibility is monotone in the
observation set, therefore saturates, so the axis-only property is visibility under the maximal observation
set. My F144-16 is stated per observation set and supplies no quantifier; `145` z1's A2 supplies it and
replaces nothing. `q3` Part A checks monotonicity against `144_probes/p7_out.txt`, the artifact I committed
before `145` existed: 24 comparable triples, zero violations, and the control fires at 12 pairs where
enlarging the observation set makes the axis visible. The dead axis is invisible everywhere.

**I mark that as weak corroboration and say why.** This is a re-reading of my own artifact, and my model
and z1's share `W`, `F`, both signednesses, the rounding pair, the intermediate pair and the dead axis;
mine adds a third overflow position. The intersection is nearly the union, so it corroborates the
arithmetic and says nothing about generality, which is `143`'s standard applied to me.

**And one addition, which sharpens `146` section 6.3 rather than softening it.** Whether the maximal
observation set is load-bearing is itself shape-dependent. At unsigned, the intermediate axis is invisible
under every proper subset I swept and visible only under all five operations; at signed, `{madd}` alone
reveals it. So the shape-to-count table waits on the operation set, and it waits harder at unsigned, where
a design shipping four of the five operations would report the axis dead and be wrong.

**The join is represented correctly.** `146` section 5.4 carries F144-15 with its predicate and both
controls named, and 6.1 states plainly that the firewall proposition itself is unpredicated and that my
F144-15 is the predicated measurement of what it buys. That is exactly the split I would have asked for and
I have nothing to add to it. `145`'s enforceability condition in z3 is one expert's and I did not build an
instrument against it; I carry it as `145`'s.

**The withdrawal is represented correctly**, including that the wrong headline stays committed. `146`
section 1.4's account is accurate and its extraction of the granularity lesson is the part I would keep.

**The retirement of my seven-arm-table reading is fair.** `146` 1.8 records F144-3 decomposing the
six-against-five gap as `139`'s duplicate control arm losing a tie-break, so the table is not a second
instance and the witness carries the claim alone. That is my own correction against myself and it belongs
in the ledger.

**The portability inversion is represented at the right strength.** `146` 5.7 states it with F144-11's
predicate and the identical-target control at exactly zero, and draws the consequence I care about, that a
predicate naming an arm is target-bound. Signed without qualification.

**The contamination note in 1.1 is at the right severity.** `146` says the one-sided-clamp congruence
reaches the unsigned half of the fusion result and not the measured tables. That is correct and it is
against `139`'s own interest to have declared it.

---

## 4. Findings, with predicates

Per I13 and `RULES.md:486-541`. An absent dimension claims nothing where that dimension is present. Every
probe here is single-threaded.

**F150-1. The cross-target invariance of a selection requires both baseline-relative units and a
per-target baseline, and appears in exactly one cell of the two-by-two.**

```
holds for: cost tables drawn uniformly from integers 1..60,
           arms = 7, cost coordinates = 3,
           rescale factors drawn uniformly from integers 1..8,
           40 shared draws across all four cells, selector = linear,
           weight grid resolution = 1/12 on the 2-simplex,
           units in {absolute, baseline-relative},
           baseline provenance in {fixed once, re-measured per target},
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F150-2. Under baseline-relative units on the simplex the baseline arm's own reported figure is
identically the weight sum, so the reporting normalisation is division by one and the reported ranking is
the selection.** 5460 cases, zero exceptions.

```
holds for: cost tables drawn uniformly from integers 1..60,
           arms = 7, cost coordinates = 3, baseline drawn uniformly among the arms,
           weight grid resolution = 1/12 on the 2-simplex,
           threads = 1,
           target features = host
```

**F150-3. `144`'s gate is a third reading, strictly stricter than either of `145`'s, and monotone
non-increasing in the gate multiplier.** Subset in every family, strict in 29 of 35.

```
holds for: the committed CSVs under mock/benches/ as of this file,
           35 bench families with at least 3 size points and at least 4 shared variants,
           cost vector = median algo_ns per size point,
           gate multiplier k in {0, 0.05, 0.1, 0.25, 0.5, 1.0, 2.0} on the interquartile range,
           non-dominance readings in {earned, escaped-conservative, escaped-symmetric},
           threads as the harness ran them,
           target features = host (aarch64-apple-darwin)
```

**F150-4. Under the gate `144` used, 4 of 35 families carry two or more established non-dominated arms at
`k = 0.5`, against 29 and 20 under the two readings `145` implemented.**

```
holds for: as F150-3, at k = 0.5
```

**F150-5. Zero arms in the corpus are established non-dominated and linearly unreachable under any of the
three readings.** So F144-18's verdict is invariant to the direction of conservatism while its scope is
not.

```
holds for: as F150-3, at k = 0.5,
           unreachability test = not strictly selectable on the medians and not
             selectable at the optimistic half-band edge,
           decision procedure = exact phase-one separation over rationals
```

**F150-6. The number of extensional classes among four lowering arms of one multiply-add is a function of
the whole assignment: one at unsigned everywhere, one at signed wrapping with floor, two at signed wrapping
with toward-zero for `F >= 1`, and two at signed saturating everywhere.**

```
holds for: numeral fixed-point, W = 6, F in {0, 1, 2, 3},
           signedness in {unsigned, signed}, overflow in {wrap, saturating},
           rounding in {floor, truncate toward zero},
           arms = {fused by widening, fused by partial products,
                   stepwise by shift, stepwise by partial products},
           operation = multiply-add, arity = 3, chain length = 2,
           inputs exhaustive over the declared range,
           container width = declared width,
           threads = 1,
           target features = host
```

**F150-7. Visibility is monotone in the observation set in `144`'s own committed sweep, and whether the
maximal observation set is load-bearing is shape-dependent.** 24 comparable triples, zero violations, 12
flips.

```
holds for: the rows of 144_probes/p7_out.txt, namely
           numeral fixed-point, W = 4, F in {0, 1, 2}, signedness in {unsigned, signed},
           assignments = rounding {toward zero, floor}
             x overflow {wrap, saturate both, saturate high only}
             x intermediate {stepwise, exact} x a dead axis,
           observation sets = {add, sub, mul}, {madd}, {mul, madd},
             {add, sub, mul, madd, msub},
           container width = declared width,
           overflow limit read at the declared width,
           accumulator width = unbounded,
           threads = 1,
           target features = host
```

**F150-8. `144` F144-2's optimum is `+1` at two cost coordinates and exactly `0` at three with a constant
third**, computed by one solver so the difference is the embedding.

```
holds for: arms = 3 at costs (0,10), (10,0), (6,6) and their three-coordinate embedding
             with a third coordinate identically zero,
           selector = linear, decision procedure = exact vertex enumeration over rationals,
           threads = 1,
           target features = host
```

---

## 5. Coverage, bounds, and my prediction that fell

**Read in full:** `145`, `146`, `145_probes/z2` and `z5` at source, `z1` and `z3` outputs, `INTENTS.md`.
**Read in part:** `RULES.md` at its predicate section. **Re-read:** my own `144` and its probe outputs.
**Not read:** `145_probes/z4`, `z6`, `z7` and `146_probes/w2` at source, so where `146` sections 7 and 10
rest on them I am relying on their prose. `132`, `136`, `40`, `93`, `102`, `106`, `107`, `108`, exactly as
`144` did.

**My prediction that fell.** `q2` CC4 predicted that a looser non-dominance reading would resurrect an
established unreachable arm somewhere in the corpus, because a larger non-dominated set is a larger
candidate pool. **Refuted, at zero under all three readings.** It is the one I most wanted confirmed,
because it would have made the selector question live, and its refutation is the strongest result in this
file: the corpus answer does not depend on the gate.

**What I did not check.** `145`'s enforceability condition (z3's main result, as opposed to its C4 clause),
its equivariance placement against the rounding topic (z4), its intersection computation (z6), and `146`'s
anchor accounting (w2). Four of `145`'s five one-expert results are therefore still at one expert after my
signature, and I say so rather than letting a signature on the file read as a signature on all of it.

**I priced nothing.** No claim here is a bench result and none is called one. `q2` reads committed harness
output and does arithmetic on it; it takes no measurement. Whether either baseline placement costs
measurable compile time is **unpriced**.

**Everything fixed-point is at model widths**, `W` in {4, 6}, with no transfer argument to 64 bits.
**Everything is `threads = 1`** except where a predicate says the harness ran it otherwise. **Container
width equals declared width** in every fixed-point instrument.

**My citations and quotations were checked by opening them.** `q5` parses the line-anchored citations
out of this file, checks sixteen verbatim quotations from `145`, `146`, `145_probes/z3_output.txt` and my
own `144`, and checks the three negatives my dissents assert. Twenty-five in total, zero failures after two
repairs, with three mutation controls firing: a phrase nobody wrote, a real phrase attributed to the wrong
file, and a real phrase attributed to the wrong span.

**Both repairs were mine and one is worth recording as a class.** The first was a heading I quoted with the
wrong capitalisation, and I corrected the quotation rather than loosening the comparison, because a
case-insensitive check stops catching a real misquote. The second is **the sixth instance in this panel of
markdown defeating a citation checker, and it arrived with a token none of the five earlier ones used**: the
passage I quoted from my own `144` is a blockquote, so every source line begins `> `, and collapsing
whitespace leaves a stray `>` mid-sentence that stripping backticks and emphasis does not touch. `146`
section 10 calls its own instance the fifth and describes the class as backticks and emphasis. It is wider
than that: **any markdown the normaliser does not know about defeats it, and each instance has arrived with
a different token**, so the fix is not another token in the strip list but a normaliser written against a
markdown grammar. I did not write one; I added the token and said what it does not cover.

**And a bound on the negatives.** A negative is only as good as its phrasing. `q5` tests the exact wording
this file attributes to `145` and to `139`; a paraphrase of the same claim inside `146` would not be caught,
and I state that rather than presenting an absence as a proof.

**Where I would want a second pair of eyes.** Section 1.1's two-by-two. It is one instrument, it is the
thing I am asking the candidate to change, and the specific risk is that "baseline provenance" is a
distinction a design would never actually face because the baseline arm is always re-measured in practice.
If that is so, the two-by-two collapses to `145`'s units reading and my correction is a distinction without
a consequence. A second reader who knows how a cost table is actually produced can settle that in one
paragraph, and I cannot.

---

## Appendix: the probes

Five, each committed with its output before or alongside this file.

1. `q1_the_baseline_is_two_choices_not_one_operation.py`: the two-by-two as a factorial on one shared
   population, with the identical-rows control that caught my own first version drawing separately.
2. `q2_which_gate_i_meant_and_what_the_corpus_says_under_each.py`: three gate readings over 35 families,
   the monotonicity, the askable counts, the unreachability survey under each, and the duplicate-family
   discriminator.
3. `q3_the_quantifier_and_the_conformance_region.py`: monotonicity parsed from my own committed `p7`
   output, and the conformance-class table that corrects `145` section 2 at a width z3 did not use.
4. `q4_how_the_candidate_represents_my_findings.py`: F144-2 recomputed at both embeddings with one solver,
   and a screen of which findings the candidate carries with a predicate, bounded in its own output because
   the screen is coarser than its numbers look.
5. `q5_check_my_own_citations_and_quotations.py`: every citation, every verbatim quotation and every
   negative this file asserts, opened and tested, with three mutation controls and the blockquote token
   that defeated it first.
