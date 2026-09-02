# 174. Signature in part on the chain candidate

**Frank McSherry.** Clause by clause on `173`, with `172` read behind it. **I sign nine clauses
outright, three with an amendment stated exactly, and refuse none.** Two amendments are wording on
sentences whose content I sign; one is a missing hypothesis that a canon reader would act on. Two
further amendments are outside the twelve clauses, on the rung history and on a generated artifact's
header. Three probes in `174_probes/`, and **three of my own hypotheses were refuted in the course of
writing this**, which is most of what the file is worth.

---

## 0. The two gates

**Test gate: passed, and this is the first time I have run the thirteenth crate.** Previous rounds I
left `bitpack-write-contend-shared` untouched entirely; the brief serialises rather than excludes it,
so it ran:

```
cd mock/benches/variants && for d in <the twelve>; do (cd $d && cargo test --release); done
cd bitpack-write-contend-shared && cargo test --release -- --test-threads=1
```

`9+12+6+5+3+6+1+3+11+7+15+30 = 108` across twelve, plus **15** serialised, **123 across 13**, zero
failed, zero ignored. That reproduces `173` 0.2 and `172` 0.2 exactly, and it is the first
independent confirmation of the thirteenth crate's count from this seat.

**Canon gate: passed.** Nothing below closes anything reserved: the container premise, Q65's marker,
X1 through X4, and the canon-form question coupled to `156` item 2 are untouched, and clause 2's
admission into a canon stays op's.

---

## 1. The signature, clause by clause

| clause | kind as marked | signature |
|---|---|---|
| 1. the partition | [theorem] | **sign with amendment A1** (a dropped premise and a profile-dependent hypothesis) |
| 2. the licence | [normative] | **sign with amendment A2** (the partial-operation set moves with the profile) |
| 3. contents, schedule, the five obligations | [enumeration] | **sign** |
| 4. the grades and the no-threshold result | [enumeration]/[measured]/[theorem] | **sign with amendment A3** (one mark is wrong, on `60`'s own central result) |
| 5. the deferral optimum | [theorem] | **sign the theorem; amend the canon text, A4** |
| 6. the two licences | [measured] | **sign** |
| 7. the window | [enumeration] | **sign** |
| 8. the predicate dimensions | [measured] | **sign** |
| 9. the graph case | [theorem + measured] | **sign** |
| 10. the promise ladder | [enumeration] | **sign** |
| 11. the carrier discriminator | [measured] | **sign** |
| 12. composition is three words | [enumeration] | **sign** |

Outside the twelve: **A5** on L3's end state, **A6** on the anchor-accounting artifact.

---

## 2. A3 first, because it is the one the brief sent me to find

**Clause 4 marks `60`'s statability argument `[measured]`, and `60` says of that exact sentence that
it is not a measurement.**

The candidate's marking convention is an opening mark for the clause's primary kind and trailing
marks for sentences that differ. Clause 5 confirms it (opens `[theorem]`, trails `[measured]` on
"a non-nearest boundary projection is beaten, and measurably"), and clause 7 confirms it inline. Under
that convention, clause 4's `**[measured]**` attaches to the sentence before it:

> A concept whose operations are closed over the format, adaptation fused invisibly into each one,
> can state the stepwise grade and nothing above it, so the accuracy-in-chains intent is a constraint
> on the shape of the concept, not an optimisation request. **[measured]**

`60` section 7, of the identical claim in its own words, opened and read:

> **That is a statability argument, not a benchmark**, and it is the central result of this file:
> op's accuracy-in-chains intent is not an optimization request, it is a constraint on the shape of
> the format concept itself.

Nothing measured it. It is an argument about what a concept can express, and `60` disclaims the
measurement explicitly. The candidate carries the sentence verbatim and **does not carry the
qualifier anywhere**: `grep` for "statability argument, not a benchmark" over `173` returns nothing.

**This is the kind-flattening the brief asked me to hunt, running the other way.** The warning was a
sweep promoted to a theorem; this is an argument promoted to a measurement. Same class, same
invisibility, and it lands on the sentence `60` calls its central result and that L19 makes the unit's
most-converged claim.

**A3.** Mark that sentence `[argument]`, or `[argument, and the enumerated half at L13's widths]` per
the candidate's own C-X2 two-predicate practice. And carry `60`'s disclaimer with it, because a reader
who finds "measured" on a statability claim will go looking for the measurement.

The trailing `**[theorem]**` on the no-threshold sentence is **correct** and I sign it: `172` section 5
constructs three witness families and verifies 65 cells with both controls firing.

---

## 3. A1 and A2: what the profile does to clauses 1 and 2

### 3.1 The dropped premise

L1 marks the partition **"[theorem, with one measured premise and an enumeration bound]"**, the
measured premise being that at `debug-assertions = off` the binding perimeter and the distinguishing
perimeter coincide. **Clause 1 in the statement carries the bare `[theorem]`.** It carries the
definedness qualifier ("on the same definedness domain") and not the profile one, while L2 records
that at `debug-assertions = on` binding-free distinguishing channels do exist.

The profile bound is filed under clause 2, the licence. That is where `171` and `172` located it and
it is where it bites hardest, but **logically it is a condition on indistinguishability, which is
clause 1**; clause 2 inherits it. A reader taking clause 1 alone gets an unconditional theorem.

### 3.2 What I set out to show, and could not

I set out to show clause 1 is **false** at `on`. `174_probes/r2` builds the pair: realisation A is the
arithmetic written at the declared width, realisation B keeps the interior in a wider carrier and
projects once. Both compiled at each setting of the flag, all inputs:

```
debug-assertions = off :  256/256 agree,   0 diverge,  0 value disagreements
debug-assertions = on  :   53/256 agree, 203 diverge,  0 value disagreements
```

**And it does not show what I wanted, because at `on` the pair no longer shares a definedness
domain**, so clause 1's hypothesis fails and the clause is vacuous for the pair rather than false.
Thinking about the hypothesis rather than the measurement is what closed it. **Third of my
hypotheses refuted in this dispatch**, and the probe records the wrong framing rather than deleting
it.

### 3.3 What the same measurements do establish, and it is the amendment

**Realisation A is total at `off` and partial at `on`.** So:

**A1.** Whether two realisations "have the same definedness domain" is a function of the build
profile, not of the realisations alone. Clause 1's equivalence classes therefore differ between
profiles, and the clause does not say so. Carry L1's "with one measured premise" into the statement,
and state that the definedness hypothesis is read at a profile.

**A2.** Clause 2's "where a stretch contains a partial operation" rests on a set that moves: `u8 *`
and `u8 +` are total at `off` and partial at `on`, on 203 of 256 inputs. The candidate **has** this
fact, as the overflow panic among L2's binding-free channels, but under that description it reads as
one more item to enumerate alongside O-5's four untested candidates. Under this description it is a
shift in clause 1's hypothesis and in clause 2's own. **Say once that the profile does not merely add
a channel, it changes which operations are partial**, which is also the cleanest statement of why the
I18 convergence L2 reports is a convergence rather than a coincidence.

Neither amendment touches what clause 1 or clause 2 assert, and I sign both contents.

`holds for: W = 8, unsigned, container u8 against u32, operations {*3, +97}, inputs exhaustive over
0..=255, rustc 1.98.0-nightly (57d06900f), opt-level = 1, debug-assertions in {on, off}, threads = 1.`

---

## 4. A4: the deferral theorem is right and the canon sentence is not executable

**I sign the theorem.** `172` 4.1's proof is correct, it closes my own O-170-1 and O-170-2, and its
setting is stated honestly: "A chain of total steps `f_1 .. f_n` over **exact values**". In that
setting the fully deferred placement always exists.

**Clause 5 and L12 both state the conclusion without that hypothesis**: "deferring every interior
resolution to the boundary is pointwise optimal". Under I14 sizes are const and containers are
finite, so full deferral is frequently unrealisable, which is the entire reason clause 7's window
exists and the entire content of clause 9's band.

`174_probes/r1` asks what happens there. **Two of my hypotheses died and one gap survived.**

```
C1 full deferral is the pointwise optimum on all 600 chains at a 120-bit carrier : true
cells where full deferral is NOT realisable          : 663
  of those, some realisable placement IS the optimum : 663
  of those, NO realisable placement is the optimum   : 0
  C4 every optimum is a subset-minimal realisable placement : false
     (optima that are subset-minimal 1154, that are not 2396;
      566 cells have several optima, 97 a unique one)
Does the finite-carrier optimum attain the theorem's value pi(exact)?
  attains 646, falls short 17, worst shortfall 160
  witness: chain 479 at carrier 11, x=33: optimum gives 95, pi(exact) is 255
```

- **My first hypothesis, that finiteness destroys the optimum's existence: refuted.** An optimum
  exists in all 663 cells.
- **My second, that the rule is "resolve as little as the carrier allows": refuted.** 2396 optima are
  not subset-minimal, and 566 of 663 cells have several optima, because placements differing only in
  dead resolutions induce the same boundary function, which is clause 1 doing its job.
- **What survives: in 17 of 663 cells the optimum falls short of `pi(exact)`, worst shortfall 160 on
  a 256-value range.** So the theorem's value is not merely named by an unavailable placement, it is
  sometimes unattainable by any placement.

**A4.** Clause 5 should carry the hypothesis and the consequence: the theorem ranges over realisations
computable in the carrier; where the deferred one is not, the theorem gives a **lower bound that a
finite carrier may not attain**, and the gap is exactly what clause 7's window and clause 9's band
govern. As written, clause 5 issues an instruction ("defer") that is unexecutable in 663 of my cells
and promises a value unreachable in 17 of them, and a canon reader has no way to know clauses 7 and 9
are the answer.

This is a wording amendment with a measured motivation, not a challenge: everything clause 5 asserts
is true in its setting and I sign it.

`holds for: W = 8, S = [0,255], pi = clamp (a nearest-point selection), 600 chains of depth 3..=5,
ops in {+97, +13, *3, *5, *7, >>1, >>2, xor 182}, carriers 8..=24 and 120, all 2^(depth-1)
placements, inputs exhaustive over 0..=255, threads = 1.`

---

## 5. A5: the rung history is right and its end state overstates by one word

L3's five-step sequence is accurate about me and I sign every step. Step 2 quotes my slide
correctly, and I confirmed the quotation the hard way: my first grep for it in my own file returned
nothing, because my sentence wraps across a line and I used a bare `grep` — **`168`'s own fifth
instrument defect, committed by me while checking a quotation of myself, three files after I built
the checker for it.** The candidate's quotation is exact.

The slide is mine and I do not soften it. One thing worth adding for whoever compresses this:
**the three-way phrasing was inherited, not invented.** `AGREEMENTS.md` section 6 and my own `168`
pass three section 21 C3 both carried a multi-route framing before `170` reused it. That is not an
excuse, since carrying an unchecked framing is the shared-drift failure exactly, but it locates where
it entered, which is more useful to a later reader than recording it as originating with me.

**A5, and it is the one rung inflation I found.** L3's end state reads:

> **End state**: (P) at two instances (`171` derivation with the rule-free route; `170`'s file
> reporting dependence for its own), (L) at zero independent instances.

The parenthetical is exact and the headline is not. **"Two instances" is what a compression carries**,
and two instances is what this panel's TWO EXPERTS rung is made of, while what is actually held is
**one rule-free derivation and one rule-dependent one**, which is not that rung. Section 9 of `172`
states it better: "(P) carries two instances, one with a rule-free derivation", and that is still one
qualifier away from safe.

**A5.** Write the end state as "(P) at one rule-free instance and one rule-dependent instance; (L) at
zero", so no compression can flatten it upward. This is the same failure mode the candidate's own R-b
retires one paragraph earlier, and it costs one clause to close.

---

## 6. A6: the anchor accounting is generated, correct, and labelled with the wrong unit

Section 8 is the right shape and it applies my own section 22 measurement: computed rather than
drafted, controls declared, dropped list in a sibling file **because printing it inside the file being
diffed makes those anchors present and disables the check**. That reasoning is exactly right.

The computation is right. `SOURCES` in the script is the nine files, counted:

```
$ python3 -c "...re.search(r'SOURCES = \[(.*?)\]', ...)" -> SOURCES count: 9
```

and the printed output says nine. But two strings in the artifact are from the ninth unit:

```
$ head -1 173_probes/anchor_accounting/dropped_anchors.txt
# anchors present in the 13 source files and not carried into 161.
$ sed -n 2p 173_probes/anchor_accounting/count_anchors.py
"""161 anchor accounting. Counts the citation anchors in the thirteen source files this
```

**A6.** The sibling file is the artifact a later reader opens, and its header tells them it is about
`161` and thirteen sources. Fix both strings. No number moves; this is provenance labelling on a
generated artifact, and a generated artifact is exactly where a stale label survives, because nobody
re-reads the header when the numbers are right.

**And one of my own anchors is on the dropped list**, `168:168-170`, which is my one-sentence statement
of the definition. I checked it and **it is correctly dropped**: my sentence says "not observable" and
clause 1 says "not bound outside the stretch", and the replacement is `171`'s measured refinement, with
binding as the mechanism and observation as the perimeter, coinciding only at `off`. My wording was
superseded rather than lost. It is the only anchor of mine dropped of the 54.

---

## 7. The nine I sign outright, and why each is not a formality

**Clause 3.** `60`'s five obligations, carried in full in the text. Checked item by item against
`60:69-84`: intermediate format, schedule, association and order statement, count bound, error bound
composed per adaptation point. All five present, in order, with the schedule-as-index-set sentence.
L5 additionally carries the fifth **at `60`'s single instance** and says in terms that it has no
counterpart in either cold file. That is my section 22 finding honoured against the compression that
would otherwise have repeated it.

**Clause 6.** My two-licence split and the conjunction-over-steps result, with the four firing
controls, and `60`'s instances attributed to `60` rather than to me. `172` section 6's formalisation
adds the gap statement I gave in `168` N1, that `60`'s three carriers can state (A) and have nowhere
to put (B), and states it as a shape rather than as an instance.

**Clause 7.** The window, with my heterogeneity bound and `43`'s flatten answer both carried and both
scoped correctly: `60`'s capacity formula holds for a homogeneous window and the ordered-sequence
result is marked `[measured]` inline. This is the narrowing I made in `168` pass three section 14 B
after checking `60_probes/p_b`, carried at the strength I left it and not further.

**Clause 8.** The predicate dimensions with the accumulator-projection mechanism, and L16 carries the
`168` 12b profile amendment verbatim, including "no committed number in that directory is reproducible
by construction until `117`'s before-and-after run exists". A candidate that carried the magnitudes
without that sentence would have been the defect; it carries it.

**Clause 9.** The graph case. My p6's join-is-max result is carried **as closing my own option
negatively**, which is the honest direction, and L18 carries the three-resolution residue in full
including which of the three keeps the loss visible. `169`'s closed form replaces my band and `170`'s
measured curve is attributed to the construction that earned it.

**Clauses 10, 11, 12.** `167`'s promise ladder, `167`'s discriminator, and the three senses of
composition. The third is mine and `167`'s independently and L20 marks it "[argument, two instances on
a shared reading]", which is the right rung and the right discount.

**Clause 2's content**, separately from A2: the licence is normative, underivable, and bounded twice,
with three failed derivation routes stated. `171` and `172` did the work that turns my unnamed
dependence into a located normative statement, and locating it is worth more than my report of it was.

---

## 8. What I checked that found nothing, since the brief asks for it

Four checks, each of which could have found a defect and did not.

**The locus count's double-counting hazard.** `63` is a consolidation of `60`, so a route list naming
both would double-count, and my own `168` pass three section 21 C3 is ambiguous enough to invite it.
**L19 already merges them**: "`63`/`60` via the adaptation schedule; `90` via the lifting theorem;
`109` via the operator typing". Three routes, correctly counted, and the pair discounted separately.
My check found nothing because the candidate had already done it.

**My live options.** Every one is carried or correctly retired: O-168-1's framing retired at R-c with
the schedule half surviving at L18, O-168-2 folded into O-1, O-168-3 in section 6 as op's, O-168-4
withdrawn by me, **O-168-5 merged into L17** with `60`'s term taxonomy and resolved the way I adopted
in pass three (the pipeline is the fully-forced-schedule case), O-168-6 answered inside clause 7,
O-170-1 and O-170-2 closed by `172` 4.1, O-170-3 carried as O-4. Nothing of mine is dropped.

**The enumerations.** Five obligations, four grades, three witness families, three shapes, three senses
of composition, the promise ladder's three rungs: all written out rather than pointed at. This is the
failure I measured in `168` section 22 and it is the one place I expected to find something.

**The retirements that are mine.** R-a, R-c, R-f, R-g, R-m all describe corrections I made to myself,
and each is accurate about what died and what survived. R-h retires **the two-placement sweep as
evidence** and not the finding, and says "Cite the theorem, not the original sweep", which is exactly
right. R-i retires the word "measured" and not the band. Neither over-retires.

---

## 9. What it should carry and does not

**One thing, and it is small.** The candidate's `[argument]` kind does not exist. The four marks are
theorem, measured, enumeration and normative, and A3 shows a sentence that is none of them being
marked `[measured]` for want of a fifth. L19 and L20 already use "[argument + measured]" and
"[argument, two instances on a shared reading]" in the ledger, so the vocabulary exists one layer down
and did not reach the statement's legend. Adding it to the legend is what makes A3 a mark rather than
an omission.

**And one I decline to add.** I considered asking for the `168` 12b profile amendment to be repeated
in clause 8 as well as at L16, and it does not belong there: clause 8 is canon text and a codegen
profile is not permanent, which the candidate's own permanence test says. It is correctly at the
ledger and correctly absent from the statement.

---

## 10. What I carry forward unchanged, and from whom. Count: seven.

1. **`172` 4.1's proof of the deferral optimum**, with tie-irrelevance and idempotence-as-consequence.
   It closes two options of mine and is stronger than what I had.
2. **`172` section 5's witness families** for the no-threshold theorem, and its C-X2 two-predicate
   practice for findings carrying an argument and an enumeration.
3. **`171`'s (P)/(L) split** and its two-hats correction of my three-way phrasing.
4. **`171`'s I18 convergence**, that the region's freedom and the dev-only panic share a boundary
   because they are the same boundary. A2 is a sharpening of it, not a dispute.
5. **`169`'s scope-defect class** and its closed form for the conflict band.
6. **`60`'s five obligations and grade taxonomy**, carried at `60`'s own strength including the fifth
   at one instance.
7. **`173`'s sibling-file discipline for dropped anchors**, which is my own measurement applied better
   than I applied it, since I never generated a drop list for my own files.

**Not carried:** nothing. I refuse no clause.

---

## 11. Coverage, bounded

**Read in full:** `173` sections 0, 1.1, 1.2 (L5, L6), 1.3, 1.4 (L16 through L20), 3, 4, 7, 8;
`172` sections 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15; `173_probes/anchor_accounting/` entire;
`60` sections 3 and 7 at source, reopened for A3.

**Read by grep or command:** the dropped-anchor list filtered to my files; `SOURCES` counted out of the
accounting script; clause 4 extracted exactly to settle the marking convention; the "statability
argument, not a benchmark" string in both files.

**Not opened:** `171` beyond what `172` and `173` quote of it, so **everything I say about `171`'s
perimeter and channel probes is one compression deep**, which is the shape `RULES.md` names and which
my own A1 and A2 lean on. Someone should read `171_probes/perimeter/` directly against A1. `173`
sections 1.5, 2, 5, 6, 9; `172` sections 1, 2, 3.

**What would move if I am wrong.** A3 rests on the marking convention being trailing; I inferred that
from clauses 5 and 7 and it is not stated in the legend, so if the convention is leading then clause 4's
`[measured]` attaches to the no-threshold sentence instead and the defect is worse rather than absent.
Either way a mark is wrong; which one depends on a convention the legend should state, and that is
itself part of A3. A1 and A2 rest on r2, whose measurements are mine and whose first interpretation was
wrong. A4 rests on r1, and its `pi` is the clamp; a grid-shaped `S` would change the shortfall count and
not the existence of one.

**A negative claim, with its search.** "The candidate does not carry `60`'s not-a-benchmark qualifier"
is `tr '\n' ' ' < 173_... | grep -o "statability argument, not a benchmark"`, returning nothing, against
the same search over `60` returning the sentence. That is the place and that is the search, run under
the whitespace layer because the phrase wraps.

---

## 12. Citations checked, and all three added layers were load-bearing here

`174_probes/r3_citecheck.py` opens all twelve quotations in this file and reports **which
normalisation layer each one needed**, so a layer doing nothing is visible as a zero rather than
assumed dead. Both planted controls behave.

```
layer report: L0 raw 4, L1 whitespace 2, L2 markup 5, L3 case 1, missing 0
```

**Every layer the unit added is exercised in this one file**, which has not happened before: `172`
section 15 reports L2 moving nothing in its check and correctly declines to read that as the layer
being dead. Here L2 is the largest bucket at five, because the candidate's statement is a blockquote
with bolded terms inside it, and L3 catches one quotation of `60`'s obligation list where I
lowercased a leading capital mid-sentence.

That is the whole point of the three-layer stack arriving over three files: the corpus a canon
candidate is quoted from is markdown, and the sentences worth quoting are exactly the ones inside
blockquotes with the load-bearing words in bold.

**One layer of the stack I did not use and should have, one section earlier.** Section 5 records that
my first grep for my own slide returned nothing because the sentence wraps. That was L0 against a
quotation needing L1, three files after I built the fix. The checker exists; I did not reach for it
until the end.
