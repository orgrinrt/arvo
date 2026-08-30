# 120. Partial signature on the candidate

`115`'s author, resumed to check `119` and sign or dissent. `119` was drafted for two signatures and says
so; this is one of them.

**The answer is a partial signature.** I cosign every clause that carries my work except three, all three
in section 4.10, and I name one omission in the ledger that is not a dissent. I accept F118-1, having
reproduced it, and I accept both things `119` retires about me, one of which is an error of mine that my
own instrument could not have caught.

Four probes, `t1` through `t4`, each committed with its output as it ran and each carrying the case that
must fail.

---

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` with its "How to read an entry" section as normative.

**I13 is what the whole of section 2 below is doing**, and it is the reason the check is a dimension diff
rather than a reading. An absent dimension does not narrow a claim, it says the claim holds nowhere that
dimension is present, so a compression that drops one has widened rather than tightened and the sentence
reads better for it. That failure is invisible to a reader comparing prose, which is why `t2` diffs the
dimension sets mechanically.

**I15 bears on my third dissent.** A clause asserting that a certificate is computable from a derivation's
syntax is a claim about what is available before any value exists, which is exactly the thing I15 makes
load-bearing, and it is stated in `119` without the marking my own file put on it.

**I14 holds.** No Rust is compiled here beyond `115`'s own towers, imported rather than rebuilt, with only
the recursion-limit attribute substituted. Zero feature gates, no `dyn`, no `TypeId`.

**Nothing to hand back.** Every judgement below is a measurement or a citation, and none of it needs a call
op has not made.

### 0.2 Test gate: passed, at 123 across 13, and it is the fourteenth count

Per crate, serially, per `110` F14's workaround. `t0_test_gate_run.txt` carries the toolchain line and the
per-crate results. **123 passed, zero failed, zero ignored**, and the attribute-only count is 123, so the
count and the run agree. I did not re-audit test bodies; `115` section 0.2 records what I read then and
nothing here touches a new crate.

---

## 1. The verdict, stated first

> **Cosigned.** Section 4.7 entirely, including its first predicate, which carries **thirteen of my
> thirteen dimensions with none dropped, none added and no substantive value change**. Ledger entries A2,
> A9, A11, A16 and A17 as they concern me, including every place they record my position as a reproduction
> rather than as an independent derivation, which is what it was. Section 1.3's two retirements about me.
>
> **Dissent, three clauses, all in section 4.10.** Its mechanism sentence attributes a multiplicative law
> to a recursion depth, which is measurably wrong and merges two quantities the sitting kept apart. Its
> predicate drops the recursion limit, which is precisely the dimension that discriminates the two
> mechanisms. And it states the syntactic half of my F115-5 flat, where my own file marks that half as an
> inference rather than a measurement.
>
> **One omission that is not a dissent.** The ledger has no entry for the re-aiming itself, which is
> F115-1 and is what makes 4.7's closing sentence true. A2 records which check; A9 records which
> conditions; nothing records that the certificate is sound when aimed at the check the character selects.
>
> **F118-1 accepted**, reproduced on my own instrument including the one thing its statement asserts that
> arithmetic on my published figures does not give.

---

## 2. My six findings, checked dimension by dimension

The dispatch asks for this rather than for a reading, and it is right to: a shorter predicate reads as a
narrower claim and under I13 is a wider one. `t2` parses both sides into dimension maps and diffs them,
with a control that deletes one dimension and alters one value on the candidate side and must report both.
The control fired.

### 2.1 F115-1 into section 4.7's first predicate: faithful, thirteen of thirteen

```
  F115-1 -> 119 4.7 first predicate
    dimensions mine 13, candidate 13
    DROPPED (in mine, absent from the candidate): none
    added   (in the candidate, absent from mine): none
```

Three value differences, and none is substantive. `{wrap, sat}` against `{wrapping, saturating}` is
wording. `one-sided [0, b] exhaustive` against `one-sided exhaustive` is wording, since `[0, b]` is the
one-sided form. The third is worth a sentence rather than a dissent.

**`discharge check = root under a homomorphism and per node otherwise` (`119:505-506`) is a wider phrase
than my `root under wrap and per node under sat`.** It is safe as written, because the same predicate
bounds `overflow behaviour in {wrapping, saturating}` and the phrase cannot reach past its own predicate.
It would stop being safe the moment that dimension were widened without the phrase being revisited,
because the sentence would then extend to a behaviour that is a homomorphism and is not wrapping, which
nobody has measured. I note it rather than object to it, and I would rather it were noted in the candidate
than in my file.

**So I cosign 4.7 in full**, including its two-predicate structure, which the candidate flags to me at
`119:684` as a change to my finding's shape. It is not a change. F115-1 and F115-2 were measured in one
run and reported in one table, F115-1 giving the conjunction sound at both policies and F115-2 giving
condition (a) alone sufficient at wrap. Two condition sets is what that pair says jointly, and stating it
as two is stating my pair rather than reshaping either.

### 2.2 F115-4 into section 4.10: one dimension dropped

```
  F115-4 -> 119 4.10
    dimensions mine 10, candidate 8
    DROPPED (in mine, absent from the candidate): ['recursion limit']
```

The other differences are `114`'s region rather than mine, correctly, since A16 records my file as the
reproduction: its fold lengths, its toolchain wording, `lib` against `library`. Those are expected and I
raise none of them.

**The dropped dimension is real and section 5.1 is where it matters.** I take it up there.

**And one defect the diff found in my own predicate rather than in the candidate.** F115-4 writes the
toolchain as two comma-separated fragments, `toolchain = nightly-2026-05-28, rustc 1.98.0-nightly (...)`,
and the notation reads a comma as a dimension boundary, so the second fragment parses as a nameless
dimension. That is my formatting and nobody else's, it is recorded in `t2`'s output as a parse note rather
than counted as a drop, and the fix is to write one value rather than two.

### 2.3 The other four

**F115-2** appears at A9 as "measured it alone at wrapping, 9408 and 1200 firing at zero violations",
which is exactly what the finding says and exactly what `s1_output.txt` prints.

**F115-3** appears at A2 as a reproduction at 38 and 34. Correct, and correctly a reproduction.

**F115-5** appears at A17 as ONE EXPERT, unreproduced. Correct.

**F115-6** appears in section 1.3 with the ratio unchanged at 32 and the composition argument surviving.
Correct.

---

## 3. The rungs, and whether my independence is honestly recorded

The dispatch names overclaiming my own independence as the failure to look for. Checked entry by entry.

**My position on the root mechanism was reached after reading `114`, and `119` says so everywhere it
applies to me.** A1 records `115` s1 as an indirect reproduction. A2 records F115-3 as one of three
reproductions. A16 records F115-4 as a reproduction and then downgrades it further on F118-16's provenance
correction. A9 does not credit me with the joint claim and credits `118`, which is right: my two halves
give the wrap side and `114`'s p1 ablation gives the sat side, and neither of us ran the other's.

**Nothing of mine is recorded as independent where it was not.** I looked for the opposite failure too and
found one instance of understatement, which is section 4 below, and it is an omission rather than a
misrecording.

**A1's own warning is right and worth repeating rather than correcting.** `119:697-700` says the rung
column is where it is most likely to have flattered the sitting and names A1 as reading like three files
agreeing when it is one derivation and two reproductions. That is the honest shape and my file is one of
the two reproductions.

---

## 4. The ledger has no entry for the re-aiming, which is F115-1

Not a dissent. An omission, and the only place my work is understated.

Section 4.7 closes with "So neither the check nor its certificate is chosen. Both are consequences of the
character." Two ledger entries support half of that each. **A2** establishes which check the character
selects. **A9** establishes which conditions the certificate needs. Neither establishes the sentence that
joins them: that the certificate, aimed at the check the character selects, is **sound**, which is what
makes "both are consequences" a claim rather than a juxtaposition.

That is F115-1, measured at zero violations and zero unsound cells across all four primitives on the
enumeration A9 and 4.7 both use, and it is the repair that section 1.3 credits at `119:310-312` as having
superseded `114` section 3.1's framing. **So the candidate knows the claim exists and files it under
framings retired rather than under agreements.** A retirement list records what to stop citing; it is not
where a live claim that a canon clause rests on should live.

What I would add, as a ledger entry rather than as a change to section 4:

> **A-new. Aimed at the check the character selects, the structural certificate is sound at both
> behaviours.** Derived: `115` F115-1 on `115_probes/s1`, zero violations and zero unsound cells at
> `uW3/sat`, `iW3/sat`, `uW3/wrap` and `iW3/wrap`, with an always-fire control producing violations equal
> to the conservative count on every row and a root-check-at-`sat` control unsound on 38 and 34.
> **Rung: ONE EXPERT, and it is the same enumeration A9 and 4.7 rest on.**

I state it as ONE EXPERT rather than higher. `114` F114-4 measures the sat half over a wider region and
`118` measures the condition split, but neither ran the conjunction against the policy-selected check at
both behaviours, which is the claim.

---

## 5. Three dissents, all in section 4.10

### 5.1 The mechanism sentence is wrong, and the measurement is decisive

`119:566-567`:

> The cost is a **recursion depth in the trait solver**, proportional to the derivation's size times the
> carrier's per-node state.

**That attributes a multiplicative law to a depth, and the sitting has two quantities here rather than
one.** F114-18 measures a **cell count**, `L(2L - 1)` against `2(2L - 1)`, which is the derivation's size
times the per-node state and is multiplicative and correct. F114-17 measures a **recursion wall**, and
`114`'s own explanation of it is that the affine tower's obligation chain is "the spine depth plus the
vector length rather than the spine depth alone", which is additive. The candidate has given the depth the
cell count's law.

Both readings are consistent with a single wall observation, so only a sweep over the limit separates
them. `t3` runs it, on `115`'s own towers imported rather than rebuilt, substituting only the
recursion-limit attribute:

```
   limit  expensive wall   cheap wall  wall/limit
      16               8           16       0.500
      24              12           24       0.500
      32              16           32       0.500
      48              24           48       0.500
      64              32           64       0.500

   limit  measured  additive pred  multiplicative pred
      16         8            8.0                 11.3
      24        12           12.0                 13.9
      32        16           16.0                 16.0
      48        24           24.0                 19.6
      64        32           32.0                 22.6

  mean absolute error, additive       : 0.00
  mean absolute error, multiplicative : 3.79
```

**The cheap tower's wall is exactly the limit and the expensive tower's is exactly half of it, at every
limit swept.** Zero error on the additive model. That is `114`'s spine-plus-vector explanation reproduced
as a law rather than as a single point, and it refutes the product. The control fired: the cheap tower's
wall moves across five distinct values, so the instrument is reading the tower's own obligation chain
rather than a fixed property of the file.

**What the clause should say instead:**

> The cost is a recursion depth in the trait solver, equal to the derivation's spine depth **plus** the
> carrier's per-node state rather than times it, so a carrier with per-node state reaches a fixed limit at
> a fraction of the derivation size a stateless one does. The **product** of the two is the count of
> associated-const cells, which is a different quantity and is F114-18's.

### 5.2 The predicate drops the recursion limit, and it is the dimension that decides 5.1

`t2`: `recursion limit` is in F115-4's predicate and absent from 4.10's.

Under I13's absence rule a dimension not listed means the claim holds nowhere that dimension is present,
and a recursion limit is always present. So 4.10's predicate as written claims nothing about any compile,
which is not what the clause intends and is not a wording quibble: **the dropped dimension is exactly the
one 5.1 turns on.** Every number in the clause moves linearly with it, and `119:568` says as much in prose
while the predicate omits it, which is the prose-against-predicate gap the candidate itself flags as C2
for a different file.

`114` F114-17's own predicate has the same omission and the candidate inherited it, so this is a
compression carrying a defect forward rather than introducing one.

**What it should say instead:** list `recursion limit` at the value the cited sweep ran at, and, since
5.1's law is now measured, state the wall as a function of it rather than as a set of lengths.

### 5.3 My own hedge was dropped

`119:559`:

> The certificate in 4.7 is computable from the cheaper carrier and from the derivation's syntax alone, so
> the choice is decidable at the point where it must be made.

Two halves. The cheap-carrier half is F115-5, compiled, with every verdict checked against the model. The
syntax half is not measured, and `115:322` marks it:

> that a proc-macro can compute it is an inference from both conditions being syntactic, not a
> measurement, and I mark it as one.

The candidate states both halves flat. Section 3's doability list carries only the measured half, correctly,
so the two parts of the candidate disagree about what was established.

**This is the dissent I most want recorded, because it is mine.** The dispatch asks whether anything is
asserted that the sitting did not establish, including by me, and this is a hedge I wrote surviving into
my own finding and not surviving into the clause built on it.

**What it should say instead:** keep the measured half and mark the other, for instance that the
certificate is computable from the cheaper carrier, and that both its conditions are functions of the
derivation's syntax, which is an argument rather than a measurement and which nothing has built.

---

## 6. F118-1, reproduced and accepted

The dispatch asks me to reproduce it before accepting or rejecting it. Two of its three numbers are
arithmetic on figures I published side by side: F115-1 reports the conjunction firing on 6336 and 816 at
the two wrapping bases and F115-2 reports condition (a) alone firing on 9408 and 1200, and the differences
are 3072 and 384.

**What is not arithmetic on my figures is the word "exact".** My s1 counted violations, not whether the
declined cells are cells the policy-selected check is exact on. If any of the 3072 were conservative,
condition (a) alone would be certifying a conservative cell and F118-1 would be wrong in my favour. `t1`
measures it:

```
  primitive   fires (a)+(b)  fires (a)  declined  of those EXACT  CONSERVATIVE
  uW3/wrap             6336       9408      3072            3072             0
  iW3/wrap              816       1200       384             384             0
```

**Every declined cell is exact**, on both wrapping bases, with zero conservative. And the case that must
fail, without which the two-arm conclusion collapses to one arm:

```
  primitive   fires (a)+(b)   viol  fires (a) only   viol
  uW3/sat              6336      0            9408    234
  iW3/sat               816      0            1200     20
```

**Dropping condition (b) at saturation produces violations**, so (b) is load-bearing where the map is not
a homomorphism, and the counter reporting zero at wrap is the same counter. The 234 and 20 are `114`'s p1
ablation figures reproduced on my implementation, which is a fourth exact cross-instrument agreement
between my model and `114`'s after the 28 and 16, the 9408 and 1200, and the 38 and 34 in `115`.

**F118-1 accepted in full**, and its conclusion is the right one: neither my repair alone nor `114`'s, but
two arms with two condition sets. That is what 4.7 states and it is why I cosign 4.7.

---

## 7. The two things `119` retires about me, both accepted

**F115-4's clause that the trait-projection route was untried in `114`.** Wrong, and `t2` part two checks
it against the place rather than arguing about it: `114_probes/p9`'s `selection-assoc` variant contains a
selector trait, an associated type carrying the chosen arm, both `impl Pick for Cond<...>` branches, and
the read through the projection. Six markers of six present, with a control marker absent. My `select_proj`
is structurally that variant.

**The mechanism is worth naming, because my own instrument could not have caught it.** `115`'s citation
checker opens every `file:line` and tests that the cited text is there. An absence claim has no
`file:line` by construction: I asserted something was not in a file I never named. A negative claim about
evidence is a claim about a place, and it can only be checked by someone who names the place. `118` named
it.

**F111-18's "2 against 64" read as a compile-side cost.** Already conceded in `115` section 3 and `119`
records the concession and that F115-6 shows the ratio unchanged. Nothing to add.

---

## 8. What I did not check

**Not read.** `118` itself, `117`, and every panel file before `114` except through `119`'s account.
`119_probes` in full: I did not re-run its anchor instrument or its citation checker, so section 6 of the
candidate is taken on its report. `116` beyond sections 0 to 3, so A3 through A8, A13, A14 and A15 are
outside what I can speak to and I sign none of them either way.

**Not reproduced.** Every figure in `119` that is not mine and not F118-1: `114`'s arm W0 sweeps, the
bilinear formula, `116`'s mutual-exclusion theorem and its ablations, the interval-construction results,
`118`'s locality decomposition. My cosignature covers the clauses that carry my work and the ledger
entries that describe it, not the candidate's measurements generally.

**Not adjudicated, and flagged rather than left silent.** Section 4.5's "**A declared restriction is
therefore the only mechanism that makes both families available at once**" (`119:446`) is a stronger
sentence than the theorem it rests on, which says no map has both characters and therefore that no choice
of behaviour buys them. "Only mechanism" quantifies over mechanisms rather than over maps. That is
`116`'s claim at A6 and its author is the one being asked to sign it, so I raise it and do not press it.

**Citations opened rather than trusted.** `120_probes/t4_check_my_own_citations.py` opens all 11
`file:line` references this file leans on and tests the substring the claim depends on, with whitespace
normalised because a quotation wrapped across two source lines is still verbatim. **11 checked, 0
failing**, plus two deliberately wrong controls which both failed as they must.

**Not established by anything I ran.** Whether derivations are trees or graphs, which C4 names and which
two files including mine depend on. `115` section 8 named it as what would decide my own section 5 against
itself and I still have not looked.

---

## 9. Probe index

- `t0_test_gate_run.txt`. Every test-bearing variant crate, per crate, serially. 123 pass, fourteenth
  count.
- `t1_reproducing_f118_1_on_my_own_instrument.py`, `t1_output.txt`. F118-1's decline counts recomputed
  rather than subtracted, the exactness of every declined cell measured, and the saturation ablation as
  the case that must fail.
- `t2_diffing_the_predicates_dimension_by_dimension.py`, `t2_output.txt`. My predicates against the
  candidate's, parsed and diffed as dimension sets, with a mutated-predicate control; and the absence
  claim in F115-4 checked against `114_probes/p9` with a marker that must be absent.
- `t3_is_the_recursion_depth_additive_or_multiplicative.py`, `t3_output.txt`. Five recursion limits, the
  wall located for both towers at each, and the two candidate laws scored against the measurement.
- `t4_check_my_own_citations.py`, `t4_output.txt`. Every `file:line` this file leans on, opened, with two
  deliberately wrong controls.
