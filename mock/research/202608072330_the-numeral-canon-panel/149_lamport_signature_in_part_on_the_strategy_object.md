# 149. Signature in part on the strategy object

I am `141`, resumed. I attacked both cold derivations, much of what survived is mine, and I was asked
whether the candidate represents it.

**I sign in part.** The ledger is honest about my work, all nine of my findings reach the candidate, and
the one contest against me is correct and I accept it on my own instrument rather than on `142`'s. But
one predicate block in section 5.5 is **widened past what anything established, and the region it
wrongly claims is false**, measured at up to 12.50% of triples. That is a dissent on a clause rather
than on the candidate, and the repair is smaller than the defect: the two clauses `146` needs there are
one clause, and the one clause is more correct than either.

The order below is the order the dispatch asked for, except that the dissent comes first because it is
the only thing that must change before signature.

---

## 0. Gates

**Canon gate: passed.** Checked against `INTENTS.md` entry by entry. Nothing here proposes a design
decision, presumes the set closed at four (I1 is open at `INTENTS.md:51-61`), or argues for
deprioritising the storage-minimising concern (I17 at `INTENTS.md:363-383`); section 8 argues the
concern's boundary is one property rather than three tables, which is a sharpening and not a demotion.
My section 6 leans on I13 at `INTENTS.md:214-235`, the one RATIFIED entry, and specifically on its
notation. Section 7 leans on I16 at `INTENTS.md:317-331`, that the canon does not police what shape a
law takes, and it is the reason my answer there is what it is.

**Test gate: passed, and I am the member who ran it.** I ran all thirteen crates in my own prior
dispatch on this tree: twelve green under `cargo test --manifest-path <crate>/Cargo.toml` at 108 tests,
and `bitpack-write-contend-shared` at 15 passed in 46.65s under `-- --test-threads=1`. 123 across 13.
I did not re-run it this dispatch and I state the reason rather than leaving it implicit:
`df` on this host reports 6.1 GiB free of 228, and thirteen release builds do not fit in that. It is
the same reason `145` gives at `145:39-40`. `146` inherits from `137_probes/g0_test_gate.out`, which I opened; it ends
`123 passed across 13 crates, 0 failed` at line 16, and that matches my own run.

**Test quality:** five members have now read bodies in five crates and reported the same verdict. I was
one of them and I am not making it a sixth by re-reading. Nothing I built this dispatch is a test.

---

## 1. The dissent: section 5.5's first predicate block is widened, and the region it adds is false

### 1.1 The claim

`146:420-427`, first clause of section 5.5:

> **Fusing a multiply-add is a free lowering where the axes make it answer-preserving.** Under
> unsigned range policies that is every rounding position, by the one-sided-clamp congruence.

with the predicate

> *holds for: W = 6; F in {0, 1, 2, 3, 4, 5}; signedness = unsigned; overflow in {wrap, saturating};
> rounding in {floor, ceiling, toward zero, away from zero, nearest-half-up, nearest-half-even}; ...*

`145` A1 carries the identical block at `145:336-339`, with the argument kind at `145:345-348` given as
"closure for the unsigned half (a one-sided clamp of a monotone operation is a congruence, so reducing
early and late land in the same place)".

### 1.2 Why I doubted it before measuring anything

The congruence argument is about the **reduction**. The arm being fused relocates two things. `142`'s
own derivation writes the pair as `R(rnd(x) + c)` against `R(rnd(x + c))`, and its `q2` implements the
fused arm as `R(rnd(ab + c*2^F))`, so the rounding moves as well. A congruence of the reduction says
nothing about a rounding relocation, and the clause bought a six-mode region with an argument that
covers one of the two moves.

And the sweep that would have caught it was never run. **`142` `q2` part B is signed only**: its loop
calls `wrap(..., w, true)` with the signed flag fixed true, at `142_probes/q2_equivariance_partitions_the_rounding_axis.rs:236`.
The unsigned rows in `139`, in my own `141`, and in `143` are all at truncate toward zero. Nobody swept
the unsigned half over six modes, and the predicate claims it.

### 1.3 The measurement

`149_probes/y1_the_unsigned_half_over_six_modes.rs`, exhaustive over all triples at `W = 6`, six modes
implemented from their definitions.

| unsigned, wrapping | F=0 | F=1 | F=2 | F=3 | F=4 | F=5 |
|---|---|---|---|---|---|---|
| floor | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| ceiling | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| toward-zero | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| away-from-zero | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| nearest-half-up | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| **nearest-half-even** | 0.00% | **12.50%** | **12.50%** | **9.38%** | **6.25%** | **3.91%** |

| unsigned, saturating | F=0 | F=1 | F=2 | F=3 | F=4 | F=5 |
|---|---|---|---|---|---|---|
| **nearest-half-even** | 0.00% | **0.93%** | **1.61%** | **2.02%** | **2.18%** | **2.08%** |

The other five modes are 0.00% at every cell under saturating too.

**So the block claims a region containing a case where the claim is false, at up to 12.50% of
triples.** Under I13's notation that is worse than an omission: an over-wide predicate is a claim that
something holds where it does not, and it is the failure the notation exists to make visible.

The controls hold. Ties are present at every `F` under unsigned (65536, 65536, 49152, 32768, 20480
triples), so half-even's nonzero is not an artifact of a domain with no ties, and the five zeros are
over cells where the rounding is inexact at up to 233472 triples. The mutation control, a fused arm that
rounds with floor regardless of mode, fires everywhere except one place, and I state that place rather
than let a uniform column imply uniform strength: **it cannot fire on toward-zero under unsigned**,
because there toward-zero is floor and the mutant is the arm. That mode's zero rests on a definitional
identity instead, and I say so.

And the cross-check that makes this about the claim rather than about my rounding: my six modes
reproduce `142` `q2` part B's signed wrapping table exactly, toward-zero at 1.64 / 5.54 / 12.34 / 22.22
/ 33.40 and half-even at 12.50 / 12.50 / 9.38 / 6.25 / 3.91.

**One of my four predictions fell.** I predicted the unsigned half-even rate would be close to but not
identical to the signed one. Under wrapping it is **identical**, digit for digit, and under saturating
it is nowhere near, at 0.93 to 2.18 against 12.50 to 3.91. Wrong in both directions at once.

### 1.4 The repair is smaller than the defect, and it is one clause instead of two

The obvious repair is to strike `nearest-half-even` from the unsigned block. That is correct and it is
not the best available, because it leaves the candidate with two clauses carrying two different argument
kinds for one phenomenon.

The better repair follows from asking why toward-zero shows 0.00% under unsigned while sitting on the
**non-equivariant** side of `142`'s partition. It is a quantifier. `142` tests equivariance over the
whole rational line. What an arm needs is equivariance **on the domain the cell reaches**, and under
unsigned every quantity entering the rounding is non-negative, where toward-zero is floor and
away-from-zero is ceiling and both are equivariant. Nearest-half-even survives as the only failure
because its tie rule reads the parity of the result rather than the sign of the argument.

`149_probes/y2_equivariance_is_domain_restricted.rs`:

| mode | equivariant unrestricted | equivariant on the non-negative domain |
|---|---|---|
| floor | true | true |
| ceiling | true | true |
| toward-zero | false | **true** |
| away-from-zero | false | **true** |
| nearest-half-up | true | true |
| nearest-half-even | false | false |

**The domain-restricted test predicts the measured fusion table at 12 of 12 (mode, signedness)
combinations. The unrestricted test mispredicts exactly 2**, which are the two the table disagrees on,
and the two tests disagree on exactly those two modes, so the refinement is not empty.

> **So the clause is one sentence, not two: fusing a multiply-add is answer-preserving where the
> rounding position is translation equivariant on the domain the cell reaches.** Under unsigned that is
> five of six positions; under signed wrapping it is three of six; and the signedness stops being a
> case split and becomes what determines the domain.

That is strictly better than what the candidate has. It is one clause where there were two, it carries
one argument kind where there were two, it says why rather than tabulating what, and it is correct at
the cell where the current wording is false.

**What I am asking for, precisely.** Section 5.5's first block and `145` A1's first block are replaced
by the single clause above, with the region expressed as the property rather than as a mode list. If the
panel prefers to keep the enumerated form, then `nearest-half-even` comes out of the unsigned list and
the two clauses stand as they are, which is correct and weaker. Either is a signature; the current text
is not.

---

## 2. The contest against my replacement B, accepted

`142` section 3 contests replacement B, which said spelling the fractional shift as an arithmetic shift
right rather than an integer division is a spelling change with "no semantic content at all on
non-negative values". Its objection at `142:215`: "Calling that a spelling is the same move as calling
fusion a lowering, which is the move `141` correctly refused in my file."

**Accepted, and I reproduced it before accepting it.** `y2` part two, over every multiply at `W = 6`:

| cell | F=0 | F=1 | F=2 | F=3 | F=4 | F=5 |
|---|---|---|---|---|---|---|
| unsigned, wrapping | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| unsigned, saturating | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% | 0.00% |
| signed, wrapping | 0.00% | 12.50% | 25.00% | 34.38% | 40.62% | 44.53% |
| signed, saturating | 0.00% | 2.93% | 9.57% | 20.51% | 34.33% | 44.53% |

`142` F142-4 reports the same two signed rows and the same two unsigned zeros. Digit for digit, on an
instrument I wrote.

**The defect in B is precisely where `142` puts it.** The clause about non-negative values is true. It
was doing work it should not, because the cell B was invoked for is signed wrapping, and there the swap
is a policy change of the exact kind I had refused two sections earlier in my own file. My section 3.6
argued that a cost model permitted to spend the one-unit slack is a cost model permitted to change the
rounding mode, and B spends the same coin from the design side. That is a symmetry I should have seen
and did not.

**B-prime at `142:225` is the correct form and I sign it.** Ship floor as a rounding position and let
the consumer select it. The equivariant region becomes reachable by declaration, a consumer who needs
toward-zero on negatives keeps it, and nobody's answers move without their having asked.

`146:228-231` retires B and records B-prime with the naming obligation from `131` F131-3 surviving
inside it. That accounting is correct and I have no correction to it.

---

## 3. The equivariance chain: does it represent my result

Three steps, and the answer is yes with one correction, which is section 1.4's.

**`141` F5** characterised the residual unit as a rounding relocation and stated when it fires, as a
biconditional: the arms differ exactly when the product quotient and the fused quotient lie on opposite
sides of zero and the shift is inexact, always by exactly one unit. Checked in both directions, zero
exceptions.

**`142` F142-2** generalised past the two modes I tested, to a partition of six modes three against
three, and put nearest-half-even on the non-equivariant side. `142:122-126` declines the two-expert rung
for it, saying it re-derived after reading my prose. That is the right call and I would not have claimed
otherwise.

**`145` `z4`** bounded it against the rounding candidate's own law enumeration: the order bound implies
equivariance, negation symmetry excludes it by a two-line argument, and among modes carrying neither it
is undetermined, witnessed by construction.

**That chain represents my result rather than replacing it**, and the reason is that each step is
strictly wider than the last while none of them contradicts an earlier one. My F5 is about which inputs
differ inside one mode; F142-2 is about which modes have the property; `z4` is about what the property
is bound to. Three different questions, and only the first is mine.

**The one correction is that the chain's second step is stated at the wrong quantifier**, per section
1.4. F142-2's partition is over the whole rational line, and what an arm reads is the property on the
reachable domain. That does not unseat F142-2, which is true as stated about the modes as functions on
`Q`. It means the partition is not the arm's predicate, and `145` A1 and `146` section 5.5 use it as
one.

**And my F5 is not carried into the candidate's statement, correctly.** `146:216-218` cites it in the
ledger for the slack accounting. Its content, the biconditional over which inputs differ, is an
instrument-level fact that `142`'s two-line derivation supersedes as an explanation. A canon carrying it
would be carrying a sweep's detail. I have no complaint.

---

## 4. My twelve replacements and eleven positions, traced

`149_probes/y3_trace_my_own_results_into_the_candidate.py` counts rather than reads, because what I am
looking for is an absence and a reader does not notice one.

**All nine of my findings reach the candidate.** F1 through F9, every one of them present in `146` at
least once, with F2 at eight occurrences and F7 at two.

| id | 142 | 143 | 144 | 145 | 146 |
|---|---|---|---|---|---|
| F1 | 0 | 2 | 0 | 2 | 1 |
| F2 | 6 | 12 | 0 | 3 | 8 |
| F3 | 0 | 4 | 0 | 2 | 3 |
| F4 | 0 | 1 | 1 | 0 | 3 |
| F5 | 1 | 0 | 0 | 0 | 1 |
| F6 | 0 | 0 | 0 | 0 | 1 |
| F7 | 0 | 0 | 0 | 2 | 2 |
| F8 | 0 | 1 | 0 | 1 | 1 |
| F9 | 0 | 3 | 0 | 0 | 3 |

Controls: a nonexistent id `F99` is found nowhere; `F7`, which two files rest on, is found four times;
and the possessive trap `146` reported against its own instrument at `146:729-732` is handled, with `F9`
counted four times as a bare id and twice as a possessive and neither read as an id called `F9'`.

**Nothing of mine is promoted above the rung I claimed.** I checked the three places where that was most
likely. `146:163` records `141` F9's width-sensitive-axis half at one expert and says `143` reproduced
the invariance half and explicitly not that one, which is exactly right and is a distinction I did not
draw that sharply myself. `146:128-129` records `145`'s `z1` as a third instance of a **proof** rather
than a third measurement, which is the correct handling of my monotonicity theorem. And `146:111-119`
records `142`'s equivariance work as a re-derivation that does not earn the rung, quoting `142`'s own
statement of that.

**The replacements are a different story and it is mostly the right one.** Only three of my twelve are
named by letter anywhere in the topic: A twice, B six times, C twice. The other nine are carried
unlabelled, or not carried. Resolved by hand, since the count cannot do it:

- **E** (compose the firewall with Q51's denotation repair) is `146` section 5.1's first two sentences.
  Carried, unlabelled, correctly, since it was Q51's repair and not mine.
- **F, G, H** (monotone non-decreasing; state it once as a quotient; a function rather than an absence)
  are all in `146` section 5.3 and `146:222-223`. Carried in substance.
- **J** (predicate the invisibility, column against accumulator) is section 5.5's third and fourth
  blocks. Carried.
- **I** (carry the W result with its own predicate) is in the ledger at `146:163` and **not in section
  5**. I record that as a residue rather than a defect: the invariance is a fact about the axis sets
  swept, my own `p5c` V6 refuted the general form, and section 4's "exhaustive enumeration over a small
  domain" class covers it. A canon sentence would have had to be narrower than the finding.
- **D** (if a tolerance is wanted, make it an axis position rather than a modifier) appears nowhere:
  the string "tolerance" is absent from both `145` and `146`. **That is correct and I want it on the
  record as correct rather than as a drop.** D was explicitly conditional on the slack mechanism being
  wanted for some reason the other replacements did not cover, and `146` section 1.8 retires the
  mechanism with an empty residue. A fallback for a case that no longer exists should not be carried.
- **K and L** (the storage question and the fusion question are one congruence question; a minimising
  choice is a cost choice where the reduction is a congruence and a policy choice where it is not) are
  **not carried as a statement**, and the accounting is worth stating exactly because my first version
  of it was wrong and my own citation checker caught it. The word "congruence" appears in `146` five
  times: in the ledger at lines 48 and 52, in a consequence sentence at 87, in the argument-kind list at
  291, and **at line 421, inside the clause I dissent from in section 1**, where it is the justification
  for the over-wide predicate. So the property is in the candidate exactly once at clause level, and
  that one appearance is the one section 1.3 shows is insufficient. It is doing load-bearing work as a
  justification and is never stated as a relation. That is what I am pressing in section 8, and I have
  earned it this dispatch rather than re-proposing it.

  My `y4` probe header carries the wrong version of this count. It is committed and I am not editing it;
  the correction lives here, which is where the rules put it.

**And the contamination note in section 1.1 is stated at the right severity.** `146:45-56` says that
wherever the one-sided-clamp congruence is the mechanism, the two cold derivations are one instance
wearing two hats, and bounds that to the unsigned half of the fusion result, the unsigned accumulator
cells, and `140`'s refuted P3. I was the third instrument on the column result and I inherited the same
workspace rule, so by the same argument **my `p4` T1 is not a third independent instance of the unsigned
half either**. `146` does not say that and it follows from what `146` says. I am reporting it against
myself: the column convergence is three instruments and, on the unsigned half specifically, fewer
independent mechanisms than three.

---

## 5. The eleven positions I carried, still carried

All eleven survive, and I have nothing to withdraw. Two moved rung and both moved down, which is the
direction that should be reported:

**Position 7** was "fusion under signed saturation is a genuine answer change at 42.14% at F=0",
independently measured. Still holds, now at four instruments.

**Position 8**, `140`'s shared-baseline obligation, I carried forward at one expert and said I was not
its second read. `144` gave it one and found it under-specified rather than wrong, and `145` section 3.5
found a conflict inside the repair. So the position survives with its content changed under it, which is
the mechanism working.

**Position 2**, packing is answer-invisible at the column, is narrowed by my own section 4 note above:
three instruments, fewer than three independent mechanisms on the unsigned half.

---

## 6. The firewall, shipped unpredicated: acceptable, for a different reason, and here is what can be predicated

The candidate ships the central surviving claim with no predicate and says so at `146:416` and in
section 6.1, declining to invent one and stating what the absence costs. I was asked whether that is
acceptable from the member who predicated everything else here.

**It is acceptable, and `146`'s reason is not the strongest one available.**

`146:557-560` gives the reason as "the claim is not the kind of thing a sweep establishes. It is a
design proposition about what a cost model may do, and a predicate over widths and operations would
misdescribe it." That is about the dimension list, and it invites the answer "then find the right
dimensions".

**The sharper reason is a category one, and it comes out of I13's own notation.** A predicate records
the region in which a claim was **established**. The firewall is not established; it is **imposed**. It
is a constraint on what the design may contain, not a report of what some measurement found. Applying
the notation to it would say the constraint holds only where somebody measured, and under I13's absence
rule that would mean the design is free to violate the firewall everywhere unmeasured, which is the
exact opposite of what a constraint means. **The notation would not merely misdescribe the claim, it
would invert it.**

So the right statement is that the predicate discipline governs findings and this is not one, and
`RULES.md:486-541` says so in its own first sentence: "No finding in this panel states a result without
stating the region it holds in." The firewall is not a finding, and nothing in the discipline is being
waived.

**What can be predicated, and I can supply it.** The proposition has three parts and the candidate
already carries two of them predicated: the enforceability condition at section 5.4 and the consequence
at F144-15. There is a fourth, which is the one that makes the constraint checkable rather than
aspirational, and it is what my probes this dispatch produce:

> **The violation predicate.** An arm that relocates a reduction violates the firewall exactly where the
> reduction is not a congruence for the following operation on the reachable domain. An arm that
> relocates a rounding violates it exactly where the rounding is not translation equivariant on the
> reachable domain.

Both halves are predicated below as F149-3 and F149-1, both are const-checkable properties rather than
sweeps, and together they turn "no cost model may move an answer" from a sentence a reviewer agrees with
into a condition an arm can be tested against. **That is what I would add to section 6.1**: not a
predicate on the proposition, which cannot have one, but the predicated condition under which a
candidate arm is refused.

---

## 7. The reach into the closed rounding topic: agree, on firmer ground, and one warning

`146` section 7 reports what the equivariance result does to the rounding candidate at `132`, revised at
`136`, without editing it, and gives its reading at `146:632` that this does not require reopening.

**I agree, and my ground is stronger than the one `146` gives.** `146`'s reason is that the gap is in
what the candidate covers rather than in what it says, and that the property is read by an arm's
predicate, which is this topic's surface. That is true and it is an argument about ownership, which
invites the reply that a law table missing a law should gain it.

**My ground is that the property is not a property of a rounding mode, so it cannot be a column in a
per-mode law table.** Section 1.4 measures this: what an arm reads is equivariance **on the reachable
domain**, which is a property of a (mode, domain) pair. Toward-zero has it under unsigned and lacks it
under signed. A per-mode table has no cell to put that in.

**And that turns into a warning, which is the part I most want recorded.** `142:266-269` recommends that
"the canon should also record, per mode, whether it is translation equivariant", and `142`'s O-142-A
proposes it. **Adopting that into `132` would introduce an error.** The value it would record is the
unrestricted one, and my `y2` measures the unrestricted test mispredicting 2 of 12 cells, both of them
under unsigned, both of them in the direction of saying an arm is unavailable when it is free. So the
rounding candidate would go from incomplete to wrong, and it would go there through the mechanism this
panel has been most careful about, a true finding carried across a boundary where its quantifier does
not hold.

**So: do not reopen `132`, and do not adopt O-142-A as worded.** The property belongs where `146` puts
it, in the arm's predicate in this topic, and it belongs there stated with its domain restriction.

If op reads the reopening question the other way, that is op's call and this file does not make it. What
I am adding is that the version of the reopening most likely to be proposed is the one that would do
damage, and that is worth knowing before the call is made rather than after.

---

## 8. One question, not three: the replacement of mine that is not carried, now earned

Section 4 records that my K and L are absent. I am not re-proposing them on the strength of having said
them once. I tested the claim, because a replacement nobody checked is not a replacement.

**The claim.** Section 5.5 states three clauses with three predicate blocks: fusion is free under
certain axes, fusion changes the answer at signed saturating, the accumulator is free except at signed
saturating. Each is an exhaustive enumeration over its own sweep. They are one thing: all three ask
whether the reduction commutes with what follows it.

`149_probes/y4_one_question_not_three.rs`, `W = 4`, exhaustive:

| reduction | signedness | is a congruence | fusion difference | accumulator visible |
|---|---|---|---|---|
| wrapping | unsigned | true | 0 | 0 |
| wrapping | signed | true | 0 | 0 |
| saturating | unsigned | true | 0 | 0 |
| saturating | signed | **false**, 3728 failures | **760** | **476** |

**The congruence property predicts the fusion cell at 4 of 4 and the accumulator cell at 4 of 4**, and
the two cells coincide. The reach controls hold: the running sum leaves the declared range at 1712 to
3280 sequences per cell and the shifted product leaves it at 1616 to 2304, so the zeros are not vacuous.

The mutation control is the one that makes this a finding rather than an observation. A deliberately
non-congruent reduction, folding an out-of-range value back by complementing, fails the congruence test
in both signednesses and shows a nonzero fusion difference (2096 and 1434) and a visible accumulator
(1772 and 896) **in exactly the cells where wrapping and saturating are all zero**. So the congruence
test is what drives the prediction rather than something incidental to the cells.

> **So the canon has one sentence where the candidate has three tables.** A relocation is free exactly
> where the thing relocated commutes with what it is relocated across, on the domain the cell reaches.
> A reduction relocation is free where the reduction is a congruence. A rounding relocation is free
> where the rounding is translation equivariant. The fusion cell, the accumulator cell and the
> intermediate axis are the same question asked three times.

This is what a canon is for under `the-canon-is-intent-not-implementation.md`: it survives a rewrite,
three teams implementing it independently would agree, and it says why rather than tabulating what.
Three exhaustive tables at `W = 4` and `W = 6` do none of those things, and a later reader who needs the
answer at a shape nobody swept gets it from the property and cannot get it from the tables.

**What I am asking for.** A clause in section 5, of the shape above, with sections 5.5's existing blocks
kept underneath it as the instances that establish it. Not a replacement of the tables. A statement of
what they are three instances of.

---

## 9. Findings, with predicates

**F149-1. Fusing a multiply-add is answer-preserving exactly where the rounding position is translation
equivariant on the domain the cell reaches, and the unrestricted property mispredicts two of twelve
cells.**

```
holds for: numeral fixed-point, W = 6, F in {0, 1, 2, 3, 4, 5},
           signedness in {unsigned, signed}, overflow = wrap,
           rounding in {floor, ceiling, toward zero, away from zero,
             nearest-half-up, nearest-half-even},
           operation = multiply-add, arity = 3, chain length = 2,
           container width = declared width,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F149-2. Under unsigned range policies, nearest-half-even is not a free fusion position**, at up to
12.50% of triples under wrapping and up to 2.18% under saturating, while the other five positions are
zero at every cell.

```
holds for: numeral fixed-point, W = 6, F in {0, 1, 2, 3, 4, 5},
           signedness = unsigned, overflow in {wrap, saturating},
           rounding in {floor, ceiling, toward zero, away from zero,
             nearest-half-up, nearest-half-even},
           operation = multiply-add, arity = 3, chain length = 2,
           container width = declared width,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F149-3. Whether the reduction is a congruence for the following operation predicts both the fusion
cell and the accumulator cell, at four of four each, and the two cells coincide.**

```
holds for: numeral fixed-point, W = 4, F in {0, 1},
           signedness in {unsigned, signed}, overflow in {wrap, saturating},
           accumulator width in {W, W + 2}, fold length = 3,
           operations {multiply-add, sum fold},
           arity in {3}, chain length in {1, 2, 3},
           container width = declared width,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

**F149-4. Replacing toward-zero with floor changes up to 44.53% of multiply answers on signed shapes and
none on unsigned ones.** Second instance of `142` F142-4, on an independently written instrument.

```
holds for: numeral fixed-point, W = 6, F in {0, 1, 2, 3, 4, 5},
           signedness in {unsigned, signed}, overflow in {wrap, saturating},
           rounding in {truncate toward zero, floor},
           operation = multiply, arity = 2,
           container width = declared width,
           threads = 1,
           target features = host (aarch64-apple-darwin, rustc 1.98.0-nightly 57d06900f)
```

Every one of these additionally requires **container width = declared width**, which every instrument in
this topic inherits from the narrowing `139` reported against itself, and the accumulator dimension in
F149-3 is the declared exception.

---

## 10. Coverage, bounded

**Read in full:** `142`, `145`, `146`, `INTENTS.md` at the entries named. **Read in part:** `144` at its
references to my own file and at section 6, via grep rather than end to end; `143` at its references to
mine; `RULES.md` at its predicate section; `145_probes/z4_output.txt` in full; `142_probes/q2`'s part B
loop at source, which is how I established the signed-only sweep.

**Not read:** `144` end to end, so where `146` characterises `144`'s weighting results I am relying on
`146` and I have made no claim about them. `132`, `136`, `131` beyond what `142` and `145` quote, so my
section 7 reasons about the rounding candidate through `z4`'s account of its law enumeration and not
against the candidate itself. That is the specific exposure in the one call that would go to op, and it
is the reason I state my ground as a property of the property rather than as a claim about what `132`
contains. `40`, `93`, `102`, `106`, `107`, `108`: unread, as in my `141`.

**I did not re-run the suite** and I say so above with the reason and with the artifact I opened
instead.

**Everything is `threads = 1`.** No probe here touches concurrency, so under the notation none of these
findings holds anywhere threads exist.

**Every measurement is at model widths**, `W` in {4, 6}. No transfer argument to 64 bits and I am not
offering one.

**I priced nothing.** No claim here is a bench result and none is called one. Whether any of these arms
is faster remains **unpriced**, and section 8's argument does not depend on it: it is about which arms
are available by declaration, not about which is fast.

**My predictions that fell: two.** Y4, that the unsigned half-even rate would be close to but not
identical to the signed one, wrong in both directions. And my mutation control in `y1` came out
**toothless on exactly one mode**, toward-zero under unsigned, which I report rather than let a uniform
column stand.

**Where I would want the second pair of eyes.** Section 1.4's domain-restricted formulation. It rests on
one instrument, it is the piece I am asking the candidate to change on, and the specific thing to check
is whether "the domain the cell reaches" is well defined for an arm the design would actually ship, or
whether I have found a property of my own sweep bounds. `y2` decides equivariance over a window I chose,
and a single counterexample settles the property in one direction but a clean sweep does not settle it in
the other. If a second reader picks the exact product and shift ranges a `W = 6` multiply reaches and the
partition still comes out five against one under unsigned, the formulation is real; if it does not, F149-1
is an artifact and the correct repair falls back to striking nearest-half-even from the enumerated list,
which is section 1.4's stated fallback and which the measurement in section 1.3 establishes on its own.

---

## Appendix: the probes

Four, each committed with its output as it ran.

1. `y1_the_unsigned_half_over_six_modes.rs`: the unsigned half swept over six rounding modes, finding
   nearest-half-even nonzero at up to 12.50%, with tie-reach controls, a mutation control that is
   toothless on one mode and says so, and a cross-check reproducing `142`'s signed table.
2. `y2_equivariance_is_domain_restricted.rs`: the restricted and unrestricted equivariance tests against
   the measured table, 0 and 2 mispredictions of 12; and `142` F142-4 reproduced digit for digit.
3. `y3_trace_my_own_results_into_the_candidate.py`: nine findings traced into five files, with a
   nonexistent-id control, a must-be-present control, and the possessive trap handled.
4. `y4_one_question_not_three.rs`: the congruence property predicting both cells at 4 of 4, with a
   non-congruent reduction as the mutation control firing in the cells where the real ones are zero.
