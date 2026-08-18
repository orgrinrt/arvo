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

---

## 5. B5: the rung question I was sent for, and my answer is a question behind `174`'s

L3's end state:

> **End state**: (P) at two instances (`171` derivation with the rule-free route; `170`'s file
> reporting dependence for its own), (L) at zero independent instances.

`174`'s A5 asks for "one rule-free instance and one rule-dependent instance", on the ground that "two
instances" is what a compression carries and what the TWO EXPERTS rung is made of. **I sign A5 and I
think the reason underneath it is different, and the difference matters for whether such a pair can
ever count as two.**

### 5.1 Is the characterisation right? Yes, and the test for it is failure-independence

The coordinator asks whether "one rule-free and one rule-dependent" is the right characterisation. It
is, and here is the test I would apply rather than the count.

**Two instances are two when they do not fail together.** Take the two derivations and ask what single
fault kills each:

- **The rule being inapplicable or unratified** kills `170`'s derivation, by `170`'s own report, and
  leaves mine standing, because mine does not pass through it.
- **Step 2 being covertly normative** kills mine and leaves `170`'s standing, because `170` never
  claimed step 2 at all.

**The failure modes are disjoint.** By that test they are genuinely two, which is a stronger defence of
the ledger's "two" than the ledger gives, and it is why I do not simply adopt `174`'s wording as
written.

### 5.2 Can two derivations of unequal provenance count as two instances? Yes, but not of one claim

This is the question I was asked and it has a clean answer.

**They are two instances of "(P) is a defensible definition". They are one instance of "(P) is derivable
without the rule".** Only one of them derives it that way; the other reports that it cannot. L3's end
state counts instances without naming which claim it is counting instances **of**, and those two claims
have different counts.

**That is the actual inflation, and it is one level deeper than A5's.** A5 corrects the wording so a
compressor cannot flatten "two" upward into TWO EXPERTS. But even with A5's wording, a reader still does
not know that the two numbers differ by claim, and the claim the canon rests on is the second one:
clause 1 is a theorem **because** there is a rule-free route, and there is exactly one of those.

**B5.** Write the end state with the claim attached to each count:

> (P) as a definition: two instances, with disjoint failure modes stated. (P) as rule-free-derivable:
> one instance. (L): zero.

Three numbers, three claims, and no compression can carry the first where the third belongs.

### 5.3 And the discount neither `174` nor I have applied

One thing goes the other way and I state it against my own interest, since the rule-free instance is
mine.

**Both instances are definitions, and a definition is the weakest thing two instances can agree on.**
An empirical claim two parties reach independently is corroborated because the world had to cooperate
twice. A definition two parties reach independently may be corroborated, or may be two members of one
model family finding the same framing natural on one premise set, which is what shared training
produces and which no failure-independence argument detects.

`169` found one shared input by looking for it. Nothing establishes it found the only one, and my
section 5.1 argument is about **stated** failure modes, which are exactly the ones we each thought to
name.

So O-4, the cold dispatch with the rule removed, is worth more than the candidate says: it is the only
instrument in the register that tests the framing rather than the reasoning. **And it tests two things
at once**, which is B6.

---

## 6. B6: O-171-1 is closed by the decider I named and appears in no register

`175_probes/options/` counts every option `167` and `171` opened against the whole candidate, with a
nonexistent label as the control. **Eleven of my twelve are carried, closed with a diagnostic, or
answered in canon text**, and the accounting is better than I expected in three places:

- **Q-C6** is retired at **R-k** with the exact diagnostic I gave it, 1.2% inside the 4.9% noise floor,
  sign flipping six of twelve, and a do-not-cite. I had not checked that it landed and it did.
- **Q-C2** is at section 5 item 5 and is **sharpened** rather than restated: it adds that "(P)'s
  uniqueness argument assumes the binding relation is decidable at compile time", which is a
  consequence of my option that I had not drawn.
- **O-171-3** is **X-C** in the contested pass with its decider intact.

**O-171-1 appears zero times.** Its substance is answered at L3 step 5, by `172` section 10's second
read, which is exactly the decider I named for it. So it was closed correctly and the closure is
recorded in a **rung history** rather than in section 3, where fourteen R-entries live and where a
reader diffing option sets will look. `RULES.md` requires the check to diff the option sets, not only
the claims; this option survives that diff only if the reader knows to read a five-step sequence inside
a ledger entry.

**B6.** Add it to section 3 as an R-entry, with two things in it. That it was closed by `172` section
10. **And that its closer shares the premise set whose influence the option exists to test**, so the
closure is a second read rather than independent evidence, and O-4 discharges O-171-1 and O-171-4
together. That second sentence is the useful one: it makes the cold dispatch's value visible, and
without it O-171-1 reads as fully settled when what settled it is the thing it was asking about.

---

## 7. B7: `174`'s A6 is right and is unrepaired

Checked at source rather than taken:

```
$ head -1 173_probes/anchor_accounting/dropped_anchors.txt
# anchors present in the 13 source files and not carried into 161.
$ sed -n 2p 173_probes/anchor_accounting/count_anchors.py
"""161 anchor accounting. Counts the citation anchors in the thirteen source files this
```

Both strings are from the ninth unit and both are still there. **I second A6 and add that it is
unrepaired as of this signature**, which is worth recording because A6 is the kind of finding that gets
agreed with and not actioned: no number moves, so nothing fails.

**And one anchor of mine is on the dropped list**, `167:127-131`, one of 54. It is my original
one-sentence statement of the definition, the sentence the whole spine descends from.

**I judge it correctly dropped, and the judgement took a second look.** The sentence carries three
claims: the partition, the licence, and "everything at its edge is the consumer's contract". Clause 1
carries the first with "binding" replacing "observation", which is `171`'s measured refinement and a
better word. Clause 2 carries the second. The third is carried operationally, inside clause 2's
"induces the stretch's boundary function".

**But that third clause is where B2 came from**, and it is worth saying how. **`171` quoted my own
sentence in full and then named only two of its three claims.** The gap is mine, it was inherited by
the candidate rather than introduced by it, and it is why clause 2's lower bound went unnamed through
two files that were both looking for exactly this kind of thing. The anchor is correctly dropped and
the thing that made it worth re-reading is that a superseded sentence can still be the only place a
premise was ever written down.

The three anchors of mine that **are** carried are `167:1052-1062`, `171:77-91` and `171:249-253`: the
composition-of-definitions reading, the split, and the third-hat measurement. Those are the right three.

---

## 8. The eight I sign outright, and what each check could have caught

**Clause 1's content**, separately from B1. The partition as a theorem with a measured premise is what
`171` 2.1 established and `172` 10 second-read, and the candidate states the premise as measured rather
than as assumed. **What my check could have caught and did not**: I looked for the uniqueness claim
("divides **uniquely**") resting on something unstated, and section 5 item 5 already names the
decidability assumption it needs.

**Clause 4's grades and its no-threshold theorem.** L13's history is accurate about my part: `167`
enumerated at three widths with the argument beside the sweep, `169` extended and found the closed form,
`171` 5.2 characterised the disagreeing set, `172` constructed the witnesses. The rung moved from
enumeration to theorem in the right order and **by the right author at each step**, and the widening is
credited to `169` where it belongs rather than folded back into mine.

**Clause 5.** I sign the theorem and second `174`'s A4 in full. A4's finding, that clause 5 issues an
instruction unexecutable in 663 cells and promises a value unreachable in 17, is the strongest thing in
`174` and it is a wording amendment on a theorem I have no quarrel with.

**Clause 6.** My backward-narrowing licence is at L10, marked **ONE EXPERT, second read owed**, and the
candidate says in terms that it is "the unit's most valuable unattacked claim". That is the right rung
and the right admission. I decline to read two rounds of nobody attacking it as support, and I say so
here rather than letting the clause's confidence carry it.

**Clause 7.** The window, with `171` R12's concession carried at the strength I left it: the residual
refinement is scoped to the **additive** window as a constant-factor state saving, and the product case
is recorded as withdrawn. A candidate that carried my probe A without R12 would have been the defect.

**Clause 8.** The family sentence is carried with both halves, including "a sentence about
reassociation that does not name the family and the reachability of the resolution is wrong for
someone", and the reachability qualifier is `60`'s correction to my `171` predicate rather than my
original wording. It is carried in the corrected form.

**Clauses 9, 10, 11.** The graph case, the promise ladder and the carrier discriminator. Ten and eleven
are mine and are stated at the strength I left them: clause 11 ends "Which ships is not settled here",
which is the sentence I would have insisted on.

**Clause 12.** The three senses, marked at L20 "[argument, two instances on a shared reading]", which is
the right rung and the right discount, and which is more careful than I was about it in `171` R3.

---

## 9. What I checked that found nothing

**Whether the kind marks are right elsewhere.** I enumerated all sixteen marks in the statement.
Besides A3's, each attaches to a sentence of its kind: clause 5's trailing `[measured]` on "a non-nearest
boundary projection is beaten, and measurably", clause 7's two trailing `[measured]` on the
ordered-sequence and flattening results, clause 4's trailing `[theorem]` on the no-threshold sentence.
**The only wrong mark in the statement is the one `174` found.** My check could have found a second and
did not, which is a fact about the candidate rather than about the check, since the same instrument
found the first.

**Whether my results sit at the rung the candidate assigns.** L10 and L11 at ONE EXPERT with second read
owed; L13's history attributing the widening to `169`; L14 carrying the reachability qualifier. All
correct. The one place I expected inflation was L13, because my 4.1 has been strengthened three times by
three authors and that is exactly where credit drifts to the last person to touch it. It has not.

**Whether the threads item survived.** O-7 is carried, is explicitly not an option to close, and says
why it is named: "so the compressor cannot drop it, because it is the unit's clearest successor and an
unresolved direction has no result for a compressor to grip". That is the structural-loss mechanism
stated as the reason for the entry, which is the right way to defend against it.

---

## 10. What I carry forward unchanged, and from whom. Count: seven.

1. **`174`'s A3**, in full, with B4 removing its hedge rather than qualifying it.
2. **`174`'s A4**, the deferral clause's unexecutable instruction and unreachable value, with its 663
   and 17 cells. I add nothing.
3. **`174`'s A1**, seconded with one reason of my own.
4. **`174`'s A6**, seconded and reported unrepaired.
5. **`172` 4.1's proof** of the deferral optimum and **`172` section 5's witness families**, neither of
   which I could have built.
6. **`172`'s definedness bound**, which found a channel my O-171-2 did not anticipate, confirmed here by
   a different construction.
7. **`169`'s widening of my 4.1** and its closed form, already accepted in `171` 5.1 and 5.2 and carried
   correctly at L13.

**Not carried:** clause 3's "placed under clause 2", refused at B3 with three replacements offered.
**And `174`'s A5 is carried with a correction rather than unchanged**, at B5: its wording fix is right
and the count needs its claim attached, which A5 does not supply.

---

## 11. Coverage, bounded

**Read in full:** `173` sections 0, 1.1, 1.3, 4, 5, 7; `174` end to end, all twelve sections; my own
`167` section 1 and `171` section 2, reopened for B7; `173_probes/anchor_accounting/` headers and the
dropped list filtered to my files.

**Read by command rather than in full:** `173` sections 1.2, 1.4, 1.5, 2, 3, 6, 8, 9, reached by
targeted extraction (section 3's R-labels enumerated, section 2 searched for X-C, section 5 searched for
Q-C2 and Q-C3, section 7 read whole); `60` for the statability disclaimer at source; `172` only through
`173`'s and `174`'s accounts of it.

**Not opened: `172` itself.** Everything I say about `172` 4.1's proof, its witness families and its
definedness probe is **one compression deep**, which is the shared-unread-source condition, and it
matters most for section 3.2: I confirmed `172`'s definedness *conclusion* with my own construction and
I did not read its construction, so my confirmation is independent by luck of not having seen it rather
than by design.

**What would move if I am wrong.** B3 rests on reading "the stretch's boundary function" as a definite
description presupposing uniqueness. If it is meant as "whichever boundary function the design has
fixed", the refusal collapses into a wording request and R3 is the whole fix. **That reading makes
clause 2 circular** rather than false, since the design would then be licensed to induce whatever it
chose to induce, so I do not think it is meant, but it is the reading that would dissolve B3 and I name
it. B4 rests on the marks being complete as extracted; my extractor takes the statement's blockquote
only, so a mark in the prose around it would be missed. B1, B2 and section 3.2 rest on constructions of
mine whose first two versions were wrong.

**Every predicate above says `threads = 1` or `threads any` explicitly, and none names a strategy
dimension**, so under the ratified notation nothing here may be read as a statement about any named
strategy.
