# 147. Signature in part: one clause is false, and it is my sentence that made it false

**I sign in part.** Most of what the candidate says about my work is represented correctly, including both
concessions, and I confirm those below clause by clause. Two predicates are wrong and one of them is not
merely wide but **false**, with a measurement that closes it. Both are mine to raise, because both trace to
my own files.

The headline, because it is the thing the signature exists to catch:

> **`146` section 5.5's first predicate is false, and it is false because of a sentence I wrote at
> `142:388-389` that reasoned from the shared contaminated mechanism instead of measuring.** The candidate
> asks me to confirm which of my conclusions ran through the one-sided-clamp congruence. Exactly one did in
> a load-bearing way, that one is wrong, and it is now in the canon candidate.

`147_probes/r1` measures it: fusing a multiply-add on an **unsigned** type under **nearest-half-even**
rounding changes the answer at 12.50% of triples under wrapping and up to 2.18% under saturating. The
candidate's clause says that cell is free for every rounding position it names.

---

## Gates

**Canon gate: passed.** Checked against `INTENTS.md`. Nothing here proposes a design decision, presumes the
strategy set closed, or argues for downgrading the storage-minimising concern (I17, `INTENTS.md:363-383`).
Section 4 declines to invent a predicate rather than inventing one, which is what I16
(`INTENTS.md:317-331`) and I13's own scope limit (`INTENTS.md:263-267`) both point at.

**Test gate: inherited, as the dispatch permits.** 123 across 13, `bitpack-write-contend-shared` under
`-- --test-threads=1`. I opened `137_probes/g0_test_gate.out` rather than taking the candidate's word for
it, and the substance checks out: thirteen per-crate result lines summing to 123, with
`bitpack-write-contend-shared` at 15 passed in 6.61s. I did not rerun. The livelock diagnosis is mine from
`139` and now has four independent reproductions, which I did not solicit and which is the right number for
a claim that costs an hour to check.

**Two citation imprecisions in the candidate, found by opening.** Neither changes a conclusion and both are
worth recording because the panel has now had five instances of this class. `146:21-22` says the gate file
"ends `123 passed across 13 crates, 0 failed`"; that string is at line 16 of a 22-line file, which then
continues with five lines about the concurrency diagnosis. And `146:42-43` cites `140:6` for the specific
blindness disclaimer; the sentence "I checked that `139_probes/` exists and did not open it" is entirely on
`140:5`, and `140:6` carries the next sentence. The candidate's own range citation at `140:3-7` is
correct.

---

## 1. The contamination, scoped by the only person who can scope it

`146` section 1.1 says that wherever the one-sided-clamp congruence is the mechanism, the two cold
derivations are one instance wearing two hats, and scopes that to "the unsigned half of the fusion result,
the unsigned accumulator cells, and `140`'s own refuted P3", adding that it "does not reach anything
measured rather than mechanised".

I was asked to confirm which of my conclusions actually ran through it. The census, by command:

```
grep -n "congruence\|one-sided" 139_*.md 142_*.md 139_probes/*.rs 142_probes/*.rs
```

Four occurrences in my corpus, and they are not the same kind of thing.

**One is the declaration itself** (`139:10`), which is where I said the rules load automatically and named
the file.

**Two are post-hoc explanations of measurements that had already refuted my predictions.** At
`139:174-176` the congruence explains why unsigned `F = 0` gave 2 classes when I had predicted 3. At
`142_probes/q3:28-29` it is prediction G2, which the exhaustive sweep then confirmed. In both cases the
number came from an exhaustive enumeration and the congruence came afterward, or came first and was
checked. **Neither conclusion rests on it**, and either survives the congruence being deleted from my
context, because the instrument would have produced the same table.

**One is load-bearing, unmeasured, and wrong.** `142:388-389`, in the section saying what the firewall
becomes:

> and it is free under unsigned regardless of mode by the congruence argument

That sentence generalises across the whole rounding axis from a mechanism rather than from a sweep. My
`q2` part B pins `wrap(..., w, true)`, so every one of my six-mode measurements is **signed wrapping**. My
`139` `p2` swept unsigned, at toward-zero only, because its shift is `p / (1 << sh)` and nothing else. So
at the moment I wrote "regardless of mode" I had measured exactly one rounding mode on an unsigned domain
and reasoned my way across the other five.

**`146` section 5.5's first predicate is that sentence turned into a predicate**, listing all six modes on
the unsigned row. Nobody widened anything; the candidate carried my claim faithfully, and my claim was the
widening.

### So the scoping needs two corrections, in opposite directions

**Narrower than stated, on the mechanism.** `146` says the contamination reaches "the unsigned half of the
fusion result". It reaches half of that half. `r1` question B isolates it by making the clamp two-sided on
an unsigned domain, which is an ordinary range policy (a clamp to `[lo, hi]` with `lo > 0`) rather than a
trick:

| unsigned, floor | F=0 | F=1 | F=2 | F=3 | F=4 | F=5 | low clamp engaged |
|---|---|---|---|---|---|---|---|
| wrapping | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0 |
| saturating, one-sided | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0 |
| saturating, **two-sided** | **3.36%** | **3.99%** | **5.50%** | **9.04%** | **14.36%** | **23.20%** | 167616 |

The wrapping zero survives making the clamp two-sided because wrapping has no clamp: it rests on reduction
being a ring homomorphism, which is `141` F3's absorption, a theorem this topic proved rather than a shared
input. **Only the saturating zero rests on the one-sidedness.** So the contaminated cell is
`signedness = unsigned, overflow = saturating`, and the unsigned wrapping cell is clean and independently
supported.

**Wider than stated, on the reach.** `146` says the contamination "does not reach anything measured rather
than mechanised". That is true of every measurement in my corpus and **false of the one clause it
produced**, because `142:388-389` is not a measurement and it is now section 5.5's first predicate. The
criterion is right and the sweep for it missed a sentence, which is the harder half: a mechanised claim is
easy to find when it is labelled as a mechanism and invisible when it is written as a conclusion.

---

## 2. Dissent one: 5.5's unsigned predicate is false

`146` section 5.5:

> **Fusing a multiply-add is a free lowering where the axes make it answer-preserving.** Under unsigned
> range policies that is every rounding position, by the one-sided-clamp congruence.

with the predicate listing `rounding in {floor, ceiling, toward zero, away from zero, nearest-half-up,
nearest-half-even}` at `signedness = unsigned`.

`147_probes/r1` question A, `W = 6`, exhaustive over all 262144 triples per cell:

| unsigned | F=0 | F=1 | F=2 | F=3 | F=4 | F=5 |
|---|---|---|---|---|---|---|
| wrapping, floor / ceiling / nearest-half-up | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| wrapping, toward-zero / away-from-zero | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| wrapping, **nearest-half-even** | 0.00% | **12.50%** | **12.50%** | **9.38%** | **6.25%** | **3.91%** |
| saturating, the other five | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| saturating, **nearest-half-even** | 0.00% | **0.93%** | **1.61%** | **2.02%** | **2.18%** | **2.08%** |

**Five of six modes are free on unsigned and the sixth is not.** My prediction A2 held: toward-zero and
away-from-zero come out free on an unsigned domain because on non-negative values they coincide with floor
and ceiling and inherit equivariance there, which is why the "regardless of mode" sentence felt right and
is wrong by exactly one mode. Nearest-half-even's failure needs no negative value at all: its tie break
reads the parity of the result and adding an integer changes that parity, so `rne(1/2) = 0` while
`rne(1/2 + 1) = 2`.

The controls hold. C3: every mode is 0.00% at `F = 0` under wrapping, so the instrument is measuring
rounding rather than something else. C1: the signed wrapping rows reproduce `142` F142-3 cell for cell,
including toward-zero at 1.64 / 5.54 / 12.34 / 22.22 / 33.40, so this is the same measurement as my earlier
one rather than a different one that happens to disagree.

**The clause should read**, and this is the repair rather than only the objection:

> Under unsigned range policies fusion is free at every **translation-equivariant** rounding position, and
> additionally at toward-zero and away-from-zero, which coincide with floor and ceiling on a non-negative
> domain. It is **not** free at nearest-half-even.

That is one predicate rather than two, and it composes with the signed wrapping clause the candidate
already carries, because both are the same condition: **fusion is free where the rounding position is
translation equivariant on the domain the type actually ranges over.** Unsigned widens the equivariant set
by two modes because the domain excludes the negatives where they differ from their equivariant twins. The
one-sided-clamp congruence is what carries the saturating half; equivariance is what carries the rounding
half; and the mode the congruence cannot save is the one where the rounding half fails.

**Why this matters past the wording.** A design that took the clause at face value would fuse a
multiply-add on an unsigned type under the IEEE default rounding mode and change answers at up to one
triple in eight, having read a canon sentence saying the cell is free. That is the exact failure the
firewall exists to prevent, arriving through the canon rather than through a cost model.

---

## 3. Dissent two: 5.5's accumulator predicate lists two dimensions my instrument does not carry

Smaller, and worth stating precisely because `146` section 1.6 says its own instrument cannot catch it.

The candidate's accumulator clause carries:

> *holds for: W = 4; F in {0, 1, 2}; signedness in {unsigned, signed}; overflow in {wrap, saturating};
> rounding in {toward zero, floor}; accumulator width varied above the declared width; threads = 1;
> target features = host. **Argument kind: exhaustive enumeration, three instruments**, intersection per
> 1.6.*

My F142-5 predicate, which is one of the three instruments, reads:

```
holds for: numeral fixed-point, W = 4, accumulator width in {W, W+1, W+2, 2W},
           signedness in {unsigned, signed}, overflow in {wrap, saturating},
           fold length n in {1, 2, 3, 4}, operation = sum fold, ...
```

**It carries no `F` and no `rounding`, and that is deliberate rather than an omission.** `142_probes/q3`'s
sum fold has no fractional bits and no rounding step, on purpose, so that a rounding effect could not
masquerade as an accumulator effect. I said so in `142` section 5: "That isolation is why I trust the
sum-fold rows."

So an intersection over three instruments cannot list `F in {0, 1, 2}` or `rounding in {toward zero,
floor}`, because one of the three ranges over neither. Under the notation an absent dimension is the
strongest negative statement available, so mine does not claim `F = 1` let alone `F in {0, 1, 2}`.

`146` section 1.6 anticipates exactly this: `z6` "matches dimension names, so it catches a dimension one
instance never varied", and "cannot check that a listed dimension was swept rather than pinned". The case
here is one step past that: a dimension one instance never **named**, which reads as absent to `z6` and
should have narrowed the intersection to nothing on that axis rather than being taken from the other two.

**Two honest repairs, and I would take the first.** State the intersection over `141` and `143` and record
mine as contributing the fold shape and the schedule dimension rather than the `F` range. Or drop `F` and
`rounding` from the intersection and let the union carry them, which is what 1.6's own rule says an
intersection is for.

Neither changes the finding. The cell is still exactly signed saturating and I still reproduce it.

---

## 4. The firewall, which is mine, shipped unpredicated

**I accept shipping it unpredicated, and I think declining to invent one was the right call rather than a
gap the candidate failed to close.** But the candidate states the cost slightly wrong, and there is a
predicate available that it already has and does not claim.

### Why a numeral predicate would misdescribe it

I13's dimension vocabulary ranges over computations: width, fraction width, signedness, strategy, overflow,
rounding, operation, arity, chain length, target features, thread count. Those are properties of a thing
being computed.

The firewall is not a claim about a computation. It is a claim about **what a design permits a cost model
to do**. Writing `W = 6, F in {0..5}` on it would assert that no cost model may move an answer at width six
and says nothing about width seven, which is not a narrower version of the claim, it is a different and
incoherent claim. A predicate that cannot be false in the region it names is not a predicate.

And I13's own scope limit says the dimension list and the exactness bar are elaboration rather than
ratified (`INTENTS.md:263-267`). So requiring numeral dimensions on a design proposition is applying the
elaboration past what op ratified, and `146` is right not to.

### What is predicable, and the candidate already has it

The firewall's **enforcement obligation** is predicable, and its dimensions are the assignment set and the
arm set rather than the numeral's shape:

> The firewall is enforceable with no mechanism beyond declaration exactly when every lowering arm the
> design admits realises the denotation of some assignment in the assignment set.

That is `146` section 5.4's second clause and `145_probes/z3`'s condition, and it carries a predicate
already. **So the candidate should say the firewall's gateable form is that condition**, rather than that
the proposition has no predicate at all. The difference matters for exactly the reason 6.1 gives: a
sentence with no predicate cannot be composed with an arm, and this one can, through the obligation it
imposes on the arm set.

Stated the way I would want it read: an unpredicated intent, plus a predicated obligation a design is
checked against, plus a predicated measurement of the consequence at 5.4's first clause. Three pieces, two
of them gateable, and the ungateable one is ungateable because it is an intent rather than because nobody
did the work.

**And I would not want it narrowed later by someone deciding what it was quantified over**, which 6.1 names
as the risk. It is quantified over cost models and arms. Writing that down costs one sentence and removes
the ambiguity the section is worried about.

---

## 5. My claims as represented, checked one at a time

**The equivariance generalisation.** Section 4 sorts it under "Equivariance" and 5.5's second block gives
`signedness = signed; overflow = wrap; rounding in {floor, ceiling, nearest-half-up}`. Correct, and the
restriction to the three equivariant modes is the right way to state where fusion is free rather than a
narrowing of my finding, which was the partition over all six. **Signed.**

**Fusion is an axis position already.** 5.5's third block: `W in {4, 6}; F in {0 .. W-1}; signedness in
{unsigned, signed}; overflow in {wrap, saturating}; rounding = truncate toward zero`, 6,356,992 triples,
cross-pairing control at 757,954. That is F142-1 exactly, dimension for dimension, with the control that
makes it non-vacuous carried rather than dropped. The attribution is right too: `141` F7 first and by a
harder route, mine from `139`'s own two committed probes. **Signed.**

**The accumulator refinement.** 5.5's fifth block reproduces F142-6's predicate exactly, including both
multiplier schedules and the fold length, and section 4 sorts it under "Bounded to the sweep, two
schedules", which is the correct kind and the correct bound. 1.2 records the 672-against-0 result with the
right reading, that the cell is real and the reachability is a kernel property. **Signed.**

**The counting concession.** 1.8 records "the count is not a property of the design at all" as retired,
too strong, with my own table as the evidence against it, conceded at `142` section 6. That is what
happened. 5.3 states the replacement as a quotient with the monotonicity theorem attached, which is `141`'s
and is attributed to `141`. **Signed.**

**The repair's retirement.** 1.8 says my declared-slack repair is conceded by its own author on
`142_probes/q1`, from `139`'s own committed probes rather than from `141`'s instrument, and gives the
cell-by-cell accounting to `141` F4, F5 and F6. Asked whether that is fair: **yes, and it is more
generous than it needs to be.** `141` found it first and I found the worse version of it, which is that
`139` had already modelled the pair as an axis two probes earlier and then asked for a mechanism to license
the difference. The candidate could have said that more sharply and the record is accurate as written.

**The chain clause.** 5.2's second paragraph carries the concession I made against myself in `139` phase
two, that observability belongs to the chain rather than the axis, and supports it with `141`'s T5.
Correct. **Signed.**

**5.8.** Attributed to me as a derivation from the object at one expert, unchallenged. That is the right
rung: nobody has second-read it and nobody should record it as though they had. **Signed, at that rung.**

**The container-width narrowing.** Section 11 says container width equals declared width in every
fixed-point instrument in the topic and calls it "the narrowing `139` reported against itself and every
later file inherited". Accurate, including that I reported it against myself rather than being caught.
**Signed.**

---

## 6. My rung and my coverage statement, confirmed

`146` section 1.1 says the commit ordering establishes my within-file half and nothing about the
between-file half, and that blindness rests on the coverage statements. Checked against the log:

```
79a87f45 09:51  139 probe p1        ...  d10129ed 10:11  139 probe p6 output
861f89bd 10:17  139 phase one
a60f1a47 10:19  140 phase one
```

**The timestamps are right and so is the reading.** My six probes and their predictions landed before my
phase one, which is the within-file half. `140`'s phase one is two minutes after mine was in the tree,
which establishes nothing about it and is why `140`'s explicit disclaimer at `140:5` is doing the work
rather than the ordering.

**My coverage statement says what the ledger claims it says.** `139:3-6` names what I read, states that the
file is committed before any of it is opened, and the next paragraph declares the workspace-rules
contamination unprompted. I confirm both are accurate: phase one was committed at 10:17 and `OPTIONS.md`
Q51 was opened after, and phase two is appended rather than folded in.

One thing I want on the record about that declaration, because it bears on how much the ledger should
lean on a self-declaration. **Mine was accurate and it was not sufficient.** I declared the shared input,
scoped it correctly to my two measured results, and then wrote a third claim from the same mechanism
fifteen sections later without noticing it belonged to the same class. A coverage statement is a claim
about what an author knows they did, and the thing that caught this was a measurement, not a re-read.

---

## 7. What I sign, what I dissent from, what I did not touch

**Signed:** 5.1, 5.2, 5.3, 5.4's proposition and enforceability condition, 5.5's second, third and fifth
blocks, 5.8, the whole of 1.8 as it concerns me, 1.2's reading of the rescoping, 1.3's refusal to give my
re-derivation the two-expert rung, and section 4's sorting of my results into "Equivariance" and "Bounded
to the sweep".

**Dissented, with the measurement attached:** 5.5's first block, which is false at nearest-half-even.
5.5's fourth block's predicate, which lists `F` and `rounding` over an intersection one instrument does not
carry.

**Refined rather than dissented:** 1.1's contamination scoping, narrower on the mechanism and wider on the
reach, per section 1.

**Amended:** 6.1, which should name the enforceability condition as the firewall's gateable form rather
than saying it has none.

**Did not check, and nobody should read my signature as covering it:** all of 5.6 and 5.7, which are
`144`'s; 1.4's withdrawal and the gate-direction question at 1.7 C5; every one of `145`'s five one-expert
results in 1.5, including `z3` and `z4`, which I cite for what they claim rather than having verified;
`140`'s closure asymmetry and declared-width companion rule; 6.2's units reading; the anchor accounting in
section 10. I read `146` in full, `141` in full, my own two files, `INTENTS.md`, and
`131_leroy_formalising_the_rounding_axis.md:315-330` for the cross-topic question. I did **not** read
`140`, `143`, `144` or `145`, so where the candidate characterises them I am relying on the candidate, and
my section 3 objection is about my own predicate rather than about `141`'s or `143`'s.

**And the largest thing still untouched is still untouched by me.** `144` has now measured the weighting
half my `p4` opened, which was the gap I named at the end of `142`. I have not read it and I am not
signing 5.6 or 5.7 on the strength of a summary.

---

## 8. Findings

**F147-1. Fusing a multiply-add on an unsigned domain is answer-preserving at five of six rounding
positions and changes the answer at nearest-half-even.**

```
holds for: numeral fixed-point unsigned, W = 6, F in {0, 1, 2, 3, 4, 5},
           overflow in {wrap, saturating},
           rounding in {floor, ceiling, toward zero, away from zero,
                        nearest-half-up, nearest-half-even},
           operation = multiply-add, arity = 3, chain length = 2,
           container width = declared width, inputs exhaustive over the declared range,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F147-2. The one-sided-clamp congruence is load-bearing for the unsigned saturating cell only; the
unsigned wrapping cell rests on absorption.** Making the clamp two-sided on an unsigned domain takes the
saturating cell from 0.00% to between 3.36% and 23.20% and leaves the wrapping cell at 0.00%.

```
holds for: numeral fixed-point unsigned, W = 6, F in {0, 1, 2, 3, 4, 5},
           overflow in {wrap, saturate one-sided, saturate two-sided with the low bound at 8 of 0..63},
           rounding = floor,
           operation = multiply-add, arity = 3, chain length = 2,
           container width = declared width, inputs exhaustive over the declared range,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

---

## 9. Coverage and bounds

**One probe**, `147_probes/r1`, committed with its output before this file. Five predictions, four
confirmed, one refuted in a way that made the finding: B1 predicted the two-sided clamp would leave
wrapping at zero, which held, and A2 predicted toward-zero and away-from-zero would be free on unsigned,
which held and is the reason my wrong sentence felt right.

**Everything is `threads = 1`**, so under the notation none of it holds anywhere threads exist.
**Everything is at `W = 6`**, with no transfer argument to 64 bits. **Container width equals declared
width.** **I priced nothing**; no claim here is a bench result.

**Where a second pair of eyes is owed.** F147-1's repair sentence, which says unsigned widens the
equivariant set by two modes because the domain excludes the negatives. That is an argument from the shape
of the two rounding functions and I have measured its consequence rather than the argument. If it is
wrong, the measurement stands and the explanation moves.

**And I am not the second reader on `145`'s `z4`**, which places equivariance against the rounding topic's
law enumeration and reaches conclusions adjacent to mine. Two files now say related things about
equivariance and neither has read the other's instrument, which is the condition 1.1 exists to flag.
Somebody should look at that before it becomes a convergence by adjacency.
