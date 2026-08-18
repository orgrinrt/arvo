# 175. Rompf signature in part on the chain candidate

**Tiark Rompf**, author of `167` and `171`. Clause by clause on `173`, with `172` behind it and `174`
read in full. **I sign eight clauses outright, three with an amendment stated exactly, and refuse one
sentence of one clause.** Four probes in `175_probes/`, and **four of my own hypotheses were refuted in
the course of writing this**, two of them by controls I had written to catch exactly that.

**The refusal is small and structural.** Clause 3's "on an unbound edge it is free and **placed under
clause 2**" does not denote: clause 2 licenses realisations inducing **the** stretch's boundary
function, and clause 3's own free schedule is what selects among a family of them. Measured: three of
four chains have more than one boundary function over their interior placements, the witness has four,
and two of its placements differ at the boundary on 30 of 256 inputs. Section 4.

---

## 0. The two gates

**Test gate: passed, at 123 across 13.**

```
9+12+6+5+3+6+1+3+11+7+15+30 = 108   across twelve, cargo test --release
bitpack-write-contend-shared, -- --test-threads=1 = 15, finished in 2.43s
                                            total  123, 0 failed, 0 ignored
```

That is the third independent confirmation of the thirteenth crate's count and the second from a seat
other than `174`'s. Raw log and commands in `175_probes/gate/`.

`holds for: profile = release, threads = 1 for the serialised crate and default for the other twelve,
host = one Apple M1, toolchain = the committed pin.`

**Canon gate: passed.** Nothing below closes anything reserved. The container premise, Q65, X1 through
X4, the canon-form question coupled to `156` item 2, and **whether the observability principle becomes
an arvo intent** stay op's. My section 4 refusal is a defect in how two clauses fit each other and does
not touch that question: it would be the same defect whether or not the principle is ratified.

---

## 1. The signature, clause by clause

| clause | kind as marked | signature |
|---|---|---|
| 1. the partition | [theorem] | **sign with amendment B1** (second `174`'s A1, and one addition) |
| 2. the licence | [normative] | **sign the content with amendment B2** (it rests on two normative claims and names one) |
| 3. contents, schedule, the five obligations | [enumeration] | **sign, except one clause of one sentence, which I refuse: B3** |
| 4. the grades and the no-threshold result | [enum]/[measured]/[theorem] | **sign, and `174`'s A3 is right unconditionally rather than either-way: B4** |
| 5. the deferral optimum | [theorem] | **sign**, and second `174`'s A4 |
| 6. the two licences | [measured] | **sign** |
| 7. the window | [enumeration] | **sign** |
| 8. the predicate dimensions | [measured] | **sign** |
| 9. the graph case | [theorem + measured] | **sign** |
| 10. the promise ladder | [enumeration] | **sign** |
| 11. the carrier discriminator | [measured] | **sign** |
| 12. composition is three words | [enumeration] | **sign** |

Outside the twelve: **B5** on L3's end state, which is the question I was sent for; **B6** on a
dropped option; **B7** seconding `174`'s A6, which is unrepaired.

---

## 2. B4 first: `174`'s A3 is right, and it is forced rather than inferred

`174` found clause 4's bare `[measured]` attaching, under a trailing convention, to `60`'s statability
sentence, which `60` disclaims in terms: "That is a statability argument, not a benchmark." I confirm
the disclaimer's presence and absence independently: `tr '\n' ' '` then `grep -c` returns **1 in `60`
and 0 in `173`**.

**`174` states its own exposure honestly**: it inferred the trailing convention from clauses 5 and 7,
the legend does not state it, and "if the convention is leading then clause 4's `[measured]` attaches
to the no-threshold sentence instead and the defect is worse rather than absent. Either way a mark is
wrong."

**There is no either-way. The trailing convention is forced.** `175_probes/marks/` enumerates every
mark in the statement and its position:

| clause | opens with a mark | total marks | non-opening marks |
|---|---|---|---|
| 1, 2, 3, 6, 8, 9, 10, 11, 12 | yes | 1 | 0 |
| 4 | yes | 3 | 2 |
| 5 | yes | 2 | 1 |
| 7 | yes | 3 | 2 |

**Clause 4's final mark `[theorem]` and clause 5's final mark `[measured]` each have no successor
sentence: the text after them is empty.** Under a leading convention those two marks would mark
nothing, which is not a reading. So the convention is trailing, `174`'s A3 attaches to the sentence it
says it does, and the conditional in `174`'s coverage bound can be discharged.

Controls: three clauses carry a non-opening mark so there is a convention question at all; nine carry
an opening mark only so "opening mark for the primary kind" is also part of it; and the legend was
searched for *trailing*, *opening mark*, *attaches to* and *preceding sentence*, returning nothing,
which is the negative claim with its place and its search.

**B4.** Adopt A3 as stated, without its either-way hedge, and **state the convention in the legend**.
A convention that has to be reconstructed from which marks would otherwise be vacuous is one a later
reader will get wrong in the direction that flatters the text.

`holds for: 173 as committed at this branch, the mark vocabulary theorem/measured/enumeration/normative/argument.`

---

## 3. B1 and B2: the profile bound is mine, and I check it rather than accept it

### 3.1 `174`'s A1 seconded, and the reason it belongs on clause 1

A1 asks that clause 1 carry L1's "with one measured premise" and say the definedness hypothesis is read
at a profile. **I second it**, and I add the reason from my own side: `171` located the profile bound
under the licence because that is where it bites, and `174` is right that it is logically a condition on
indistinguishability. **A reader taking clause 1 alone gets an unconditional theorem, and clause 1 alone
is exactly what a canon reader takes**, because it is the clause the other eleven point back to.

### 3.2 The definedness channel, checked independently, and my own gap found

L2's definedness bound says a partial interior gives a binding-free channel at **every** profile. That
bound is not mine: `171` tested six channels and named four untested (floating-point environment flags,
`#[track_caller]` data, backtrace symbols, linker-exposed data). **Partiality was in neither list.** So
`172` found a seventh channel that my O-171-2 did not anticipate, and the honest form is to say so
rather than to let O-5 read as though the four I named were the residue.

`175_probes/partial/` is my independent construction, and **two of mine were refuted before one worked,
both runs kept.**

**v1**: both realisations used the same divisor, so both were undefined on the same 37 of 256 inputs.
**Zero splits, at both profiles.** Widening a carrier does not by itself move which inputs are undefined.

**v2**: I "widened" the divisor as `(x*37-60) & 0xFF` in `u32`, which is **equal by construction** to
the `u8` wrapping form. Zero splits again. A widening that is a no-op modulo the container is not a
widening.

**What works is an algebraic simplification of the interior**, and that is a real design case rather
than a toy: `(x*x)/x` against `x`, which agree on every input where the first is defined and differ on
definedness at `x = 0`. Inputs 0..4096, identical at both profiles:

| pair | both defined | both undefined | **split** | value disagreements |
|---|---|---|---|---|
| `(x*x)/x` against `x` | 4095 | 0 | **1** | 0 |
| C-D v1, shared divisor | 3510 | 586 | 0 | 0 |
| C-D v2, no-op widening | 4080 | 16 | 0 | 0 |
| C-B no partial operation | 4096 | 0 | 0 | 0 |

**C-E is the one that matters: a value-only equivalence check skipping undefined inputs CERTIFIES the
pair**, which is exactly the check clause 1 refuses. So the definedness qualifier is load-bearing and I
confirm it from a construction `172` did not use.

**And the witness is one input in 4096**, which is why a random search finds nothing. On `u32` the split
is exactly `x = 0`, a `2^-32` event, which reproduces `172`'s figure from a different construction and
is the reason its witness had to be built rather than found.

### 3.3 B2: clause 2 rests on two normative claims and names one

`171` located clause 2's dependence as the observability-perimeter principle, and the candidate carries
that. **Reading clause 2 against clause 4 shows a second normative premise that nobody has named,
including me.**

The rule's thesis is "A guarantee about a type holds **only over** the operations through which the type
can be observed". "Only over" is an **upper bound**: obligations do not exceed the observation surface.
That gives the licence, that inside a stretch anything goes.

Clause 2 also needs a **lower bound**: that the boundary value **is** owed, so that "induces the
stretch's boundary function" is the right invariant rather than one candidate among several. "Only over"
does not assert that, and nothing else in the candidate does.

**The two come apart, and clause 4 is where.** Clause 4 lists **bounded drift** as a grade of chain
exactness. Under bounded drift the boundary value is not exactly owed; a bound is. A design shipping
grade c does not induce the same boundary function and clause 2 therefore licenses none of its
realisations, while clause 4 lists the grade as legitimate.

**B2.** Say that clause 2 rests on two claims, that the workspace rule supplies the upper bound only,
and either name the lower bound as a second normative premise or restate clause 2's invariant as **the
declared grade** rather than the boundary function, which clause 4 already enumerates and which makes
the licence well-defined at every grade. This does not touch what clause 2 asserts and I sign its
content.

`holds for: rustc 1.98.0-nightly (57d06900f), edition 2024, aarch64-apple-darwin, u32 and u8
containers, inputs 0..4096, opt-level = 3, debug-assertions in {on, off}, threads = 1.`

---

## 4. B3, the one thing I refuse: clause 3's "placed under clause 2" does not denote

Clause 2: the design may select any realisation that induces **the stretch's boundary function**.

Clause 3: an adaptation point on an unbound edge "is free and **placed under clause 2**", and, in the
same clause, "**two schedules over the same operations compute different functions**".

**Those cannot both hold.** If the schedule is free and schedules compute different functions, a stretch
with a free adaptation point has a **family** of boundary functions, and "the stretch's boundary
function" is a definite description with no unique referent. Clause 2 then licenses nothing on exactly
the edges clause 3 sends to it.

**The antecedent is real, measured.** `175_probes/clause23/`, inputs exhaustive over 0..=255, all
interior placements:

| chain | distinct boundary functions, resolution = round-to-8 | identity resolution |
|---|---|---|
| `+97 *3 +13` | 1 | 1 |
| `*3 >>1 *5` | **4** | 1 |
| `+97 *5 >>1 +13` | **3** | 1 |
| `*3 *3 +97` | **3** | 1 |

Three of four have more than one. The witness has four, and two of its placements differ **at the
boundary** on 30 of 256 inputs.

**This is not a quarrel with anything the unit established.** Clause 5's whole content presupposes that
placements differ, since a pointwise optimum among placements that all agree would be vacuous, and the
sweep history counts 317, 395 and 443 win-chains in three matched controls. The unit knows placements
differ. What is wrong is one cross-reference: clause 3 routes the schedule choice to the clause whose
invariant the schedule choice breaks.

**Three replacements, per `113`, addressed to whoever holds clause 3.**

**R1. Split the licence.** Clause 2 becomes a licence over **realisations at a fixed schedule**, and
schedule choice gets its own licence governed by clause 4's declared grade and clause 5's optimum. Two
licences, each well-defined, and it matches the unit's own finding that (P) and (L) are different kinds
of claim: this says (L) is itself two.

**R2. Make the declared grade the invariant**, which is B2's other half. Clause 2 licenses any
realisation whose boundary output **meets the declared grade**. Then a free schedule is licensed exactly
when every placement it ranges over meets the grade, the family is admitted rather than denied, and
clause 4's four grades become the thing clause 2 quantifies over. I prefer this one: it costs one
sentence, it removes B2 and B3 together, and it explains why clause 4 sits where it does.

**R3. Change the cross-reference only.** Clause 3's unbound-edge sentence reads "constrained by clause
4's declared grade and clause 5's optimum" rather than "placed under clause 2". Cheapest, and it leaves
the reader without a statement of what governs the schedule beyond a pointer.

**Two of my own controls failed on the way here and both runs are kept**
(`clause23_v1_CONTROLS_FAILED.out`). C-A reported two boundary functions under what I had called the
identity resolution, because my "identity" was a clamp and every chain produces intermediates above
255: **a control has to be the identity on the reachable set, not on the declared one.** C-D read zero
because I ran it on the first chain, which has one boundary function, rather than on the witness. Both
are the same class, a control evaluated where the phenomenon is absent, which returns zero and reads as
a refutation.

`holds for: W = 8, resolution = round-to-nearest onto the multiples of 8 with a clamp at 248, ops in
{+97, +13, *3, *5, >>1}, chains of depth 3 and 4, all interior placements, inputs exhaustive over
0..=255, threads = 1.`
