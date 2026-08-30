# 59. Persona checkpoint five: unit two, mid-topic

**Date:** 2026-08-09. **Position:** the checkpoint slot of unit two, after `55` derived the format
concept cold, `56` attacked it, `55b` replied, `57` adjudicated the refutation and second-read the
grading, and `58` split the unit's strongest result at the fraction boundary. Four experts remain
before the consolidation.

## What I am, and what this file cannot do

I am the `orgrinrt` persona standing in for op while he sleeps. **I have no authority.** Nothing below
ratifies anything, becomes canon, or may be quoted to a later expert as a warrant. Op has ruled every
persona call from the prior panel unratified, and that ruling covers this file in advance. Every call I
make is marked as the persona's and lands in `PERSONA_CALLS.md` marked persona-decided. An expert who
disagrees with any of it should say so and keep working; my disagreeing with an expert is not a finding
against that expert, it is one more unratified opinion in a panel that already has plenty.

Where anything here conflicts with op's recorded words, his govern and the conflict is written down
rather than resolved. Where I recommend a register change, I recommend it; I have edited neither
`OPTIONS.md` nor `INTENTS.md`.

The slot exists to **redirect the second four while that still costs four files rather than nine**. A
checkpoint that summarises has wasted itself, so sections 4 and 5 are the part of this file that cannot
be recovered later, and sections 1 through 3 exist to earn them.

**What I read.** `INTENTS.md`, `00_brief.md`, `RULES.md`, then `55`, `56`, `55b`, `57`, `58` in full;
`42` sections 3 and 5.2 with `42_probes/p3`'s source and output; the probe **sources** for
`55_probes/p2`, `p4`, `p5`, `56_probes/q1`, `57_probes/p2`, `p3`, `p4`, `p5`, `p6`, `58_probes/p2`,
`35_probes/p3`, `42_probes/p1`; the outputs of `56_probes/q1`, `57_probes/p1`, `p2`, `58_probes/p2`;
`OPTIONS.md`'s Q3, Q5, Q6, Q11, Q12, Q14 and Q17. **Not opened:** `08`, `35`, `18`, `20`, `25`, `40`,
`43`, `DROPLIST.md`, `seed/`, `archive/`. Section 7 bounds what that costs. **I did not re-run any
probe**, and every count below is either read from a committed output file or produced by a command I
name.

---

## 1. The audit: where the unit is weaker than it reads

The headline first, because it is not a criticism of any of the five and it is the most consequential
thing I found. **This unit's independence bookkeeping is clean.** `55` was genuinely cold and said so;
`56`, `55b`, `57` and `58` each stated their reading order explicitly, each named which claims were
first-read and owed a second, and each re-ran the probes it argued with before arguing. `57:10-13`
re-ran four instruments byte-identically. `58:16-18` re-ran three. `55b:9-11` re-ran `q1` and notes
`56` had re-run its three, so the probe base of that exchange is mutually regenerated. Unit one
inflated a rung three times; unit two has not inflated one, and the reason is that every member did the
bookkeeping the rules ask for rather than leaving it to the dispatcher.

That said, five things read stronger than they are.

### 1a. The absorption-equals-coherence identification is argued in one paragraph and never measured, and three files now rest on it

This is the audit finding I would act on first.

`57:277-278` argues the identification: `56`'s C-law is `rho(a op b) == rho(rho(a) op rho(b))`, and
"with `b` drawn from `Q` so that `rho(b) = b`, that is `rho(rho(a) op b) == rho(a op b)`, which is
absorption." On the strength of that, `57` concludes at `57:286-288` that "coherence is not a coarsening
of a finer true statement. It is the statement", and recommends at `57:576-581` that the register carry
the criterion as absorption with `p2`'s biconditional counts.

The two predicates are not quantified the same way in the two probes that carry them. I opened both
sources.

- `56_probes/q1_two_law_families.rs:10-12` states the C-law "for all a, b in the window", and the
  window is `[-64, 64]` against `Q = [-8, 7]` (`q1`'s header, lines 34 to 35). **Both** arguments are
  reduced, and both range over the ambient window.
- `57_probes/p2_absorption_necessity_sweep.rs:90-102` reduces **only the left** argument, ranges `x`
  over `op(a, b)` for `a, b` in the operand box, and ranges `y` over the operand box. And the operand
  box is not inside `Q`: `57:227-228` sweeps the box independently of the clamp bounds, so `rho(y) = y`
  is not available in general.

So `57`'s bridging step assumes `y ∈ Q`, which its own sweep does not enforce, and the C-law
additionally quantifies over pairs both outside `Q`, which absorption does not reach. The predicates
may still coincide on this pool. Nobody has checked, and the check is one afternoon: run both
predicates over `p2`'s 4248 configurations and diff the boolean vectors.

**Why it matters more than a pedantic quantifier note.** `55b:220-222` now treats coherence-as-hom as
"a joint object", `57` asks `56` to restate the relation with coherence as primary (`57:562-567`), and
the register is being asked to replace Q12's mechanism paragraph on this basis. If the two predicates
separate anywhere, the panel has two criteria wearing one name, in the exact spot where it has already
had one criterion turn out to be a vocabulary slip.

### 1b. `57`'s refutation of `42` is real and its framing overstates what `42` got wrong, and my own dispatch brief compressed the overstatement further

I opened `42:300-347` and `42_probes/p3.out` and `p3.rs`.

`57` is right on the literal point. `42:314-316` reads "associativity of a clamped operation holds
exactly when at most one of its clamps can be triggered by any association order of the specific fold
in question", and `42_probes/p3.out`'s first block is a ceiling clamp with no floor measuring 904 of
3375 failures at `top=3`. One clamp exists, so at most one is triggerable, so the sentence predicts
associativity and the row measures failure. The sentence is refuted by the row, and I regenerated
nothing but read the committed output, which prints exactly those numbers.

What the framing loses is that **`42` published that refutation itself, deliberately, and named the
hypothesis that survives.** `42:307-308`: "It refuted the hypothesis: 904 of 3375 triples failed at
`top = 3`. I kept the run rather than rewriting it, because the failure is the more informative half."
And `42_probes/p3.rs:25-28` states H2, confinement to a half-line, which `57:214-217` confirms is
unrefuted by anything in this panel. So the honest account is: **`42` refuted clamp-counting, stated the
surviving hypothesis in its probe, and then wrote a prose summary whose word "clamps" contradicted its
own table's word "boundary".** `57:189-190` says exactly this ("a vocabulary slip rather than a
measurement error"), to its credit, and then leads with a headline that reads as a much larger
finding.

**And the compression got worse one hop later.** My own dispatch brief for this slot describes `57` as
finding that "`42`'s sentence is refuted by `42`'s own table, row one, a ceiling clamp with no floor,
satisfying 'at most one clamp triggered' trivially while measuring 904 failures." True, and it drops
both the slip framing and the fact that `42` had already refuted clamp-counting and named H2. That is
the panel's recorded compression failure happening again, at the dispatching layer, two files after
`57` warned about a word that "let an absent clamp count as a boundary". I am naming it against my own
brief because the register is about to carry a droplist entry on this and the entry should describe
what actually happened.

**What is genuinely new in `57` and should survive the correction:** absorption stated as an exact
biconditional over a swept space rather than as a hypothesis (`57_probes/p2_output.txt`, 0 sufficiency
and 0 necessity violations for addition over 4248 configurations); the sign-confinement closed form
with 0 mismatches over 100 interval numerals (`57_probes/p1_output.txt:44`); and the mutant case that
shows the closed form is a corollary and not the criterion. None of that is in `42`.

### 1c. "4248 configurations" counts something that is mostly not a numeral

Stating what a count counts, per `RULES.md:124`. `57_probes/p2`'s 4248 is the product of floor in
`{absent} ∪ [-6,6]`, ceiling likewise, and an operand box `[blo, bhi]` with `blo ∈ [-5,0]` and
`bhi ∈ [0,5]`, per `57:227-228`. Most of those configurations are a clamp and an operand set with no
relation to each other. Only `57_probes/p1`'s section 3 sweeps **genuine interval numeral systems**, and
that count is **100**, of which 19 are associative.

That is a strength of the biconditional, not a defect: it holds over a wider class than arvo formats.
But a register line reading "absorption and associativity agree over 4248 configurations" invites a
reader to think 4248 formats were checked. The number of format-shaped configurations checked anywhere
in this unit is 100.

### 1d. `57`'s shared theorem is additive in all of its evidence

`57:520-524` states it as one fact: "Coherence is the statement that the grading collapses. A coherent
reduction needs no grade: its accumulator is the format itself, at any fold length."

Its evidence is `57_probes/p6`, whose sixteen unsigned rows all read "format width already suffices",
and `57_probes/p4` section 2's fold table. I opened `p6`'s source: its operations are `format_range`,
`divergence` and `exact_sum_width` (lines 51, 60, 93) and there is no scale parameter and no
multiplication anywhere in the file. So the collapse half of the theorem is measured **for addition
only**. `58_probes/p2` then measures the multiplicative case and finds the grade does not collapse; its
output's closing block states the contrast in its own words.

`58` files this as "job two's grading does not generalise". It is also a bound on `57`'s shared
theorem, and `57` is not resumed to say whether it accepts that. Left as an open exchange.

### 1e. `55b`'s ring and `56`'s two-by-two are integer-grid results, and only `58`'s file says so

`55b:98` reports wrap's induced structure as "the ring Z/16: associative add and mul, identities 0 and
1, all inverses, distributive". I opened `55_probes/p4_induced_algebra_grades.rs:69-73`: `add` is
`a + b` and `mul` is `a * b`, bare. `58` found this and said so for the semiring row. It applies
identically to the **ring** row, and `58:441-445` correctly records that wrapping multiplication at
`F > 0` is unmeasured, an inference rather than a result.

Same for `56_probes/q1`'s `coherent(*)` column, which reports "wrap coherent(*): true" and "clamp
signed coherent(*): false" in `q1_output.txt`. No scale token appears in `q1`'s source.

So the two-by-two's **multiplicative** entries and the induced **ring** are `F = 0` claims, alongside
the semiring `58` and `57` already bounded. The additive entries transfer, per section 2.

---

## 2. What survives the fraction boundary

`58`'s split is the unit's most consequential finding and it has one author, so I checked its mechanism
myself rather than carrying it, and then extended the count.

**The count.** Of the twenty probe sources under `42_probes/`, `55_probes/`, `56_probes/`, `57_probes/`
and `58_probes/`, **fourteen contain no scale, rescale or fraction token of any kind**. Produced by:

```
for f in {42,55,56,57,58}_probes/*.rs; do
  grep -cE '>> *f|>> *F|/ *scale|/ *s;|1i64 << *f|1 << *f|f: u32|frac' "$f"; done
```

The six that do are `57_probes/p3`, `p4`, `p5`, `58_probes/p1`, `p2`, and `42_probes/p1`. What the count
counts: files whose text mentions a scale, not files whose claims are integer-only. The two sets differ
in one direction only, and the direction matters, which is section 2a.

### 2a. The additive results are unconditional, and the reason is checkable by inspection

`58:69-81` argues that addition never reads the scale, so an additive result measured at `F = 0`
transfers verbatim to every `F`. I checked the argument rather than the file: two raw values `a` and
`b` at a common scale `F` denote `a/2^F` and `b/2^F`, their sum denotes `(a+b)/2^F`, and the raw sum is
`a + b` exactly. There is nothing for `F` to do. A clamp to `[0, M]` acts on the raw value. So the
computation at `F = 3` with modulus `M` is character-for-character the computation at `F = 0` with the
same `M`.

That is stronger than a sweep and `58` is right to say so. It also extends further than `58` took it,
and the extension is worth having because it upgrades a result the register already leans on:

**`35_probes/p3`, which carries Q12's headline divergence table, is additive and same-scale.** Its
`add` is `clip(a + b, w, s, p)` at line 72, with no scale parameter in the file. So the table (unsigned
wrapping 0, signed wrapping 0, unsigned saturating 0, signed saturating 70.1 percent) is
**F-independent by the same argument**, rather than being an integer-grid measurement awaiting a
fractional check. Nobody has said this, and Q12's entry currently reads as though its numbers were
measured on one grid. Same for `57_probes/p6`'s one-bit accumulator gap and `55b`'s pullback counts and
`56`'s `coherent(+)` column and `57`'s absorption biconditional for addition.

So the surviving list, and it is substantial:

| result | survives every F | evidence |
|---|---|---|
| absorption is equivalent to associativity for clamped addition | yes | `57_probes/p2_output.txt`, 0 and 0, plus the F-independence argument |
| saturating addition on an interval numeral containing zero is associative iff sign-confined | yes | `57_probes/p1_output.txt:44`, 100 intervals, 0 mismatches |
| unsigned saturation is an additive commutative monoid | yes | `57_probes/p3_output.txt`'s `+assoc` column, 0 at every row including F = 1, 2, 3 |
| the reassociation divergence table | yes | `35_probes/p3`, additive, same-scale |
| the accumulator is exact-sum-width minus one bit | yes | `57_probes/p6_output.txt`, fifteen rows, additive |
| coherence of wrap and of unsigned add-only saturation, for addition | yes | `56_probes/q1_output.txt` |

And the list that does not survive:

| result | status at F > 0 | evidence |
|---|---|---|
| unsigned saturation is a semiring | dead, 9 of 9 fail | `57_probes/p3_output.txt` section 3 |
| wrap induces the ring Z/2^N | **unmeasured**, multiplicative half only | `55_probes/p4` is bare `a * b`; `58:441-445` |
| absorption is sufficient for clamped multiplication | premise unsatisfiable for eager multiply | `58` section 2.2, witness at F=2, 3, 5, 7 |
| the two-by-two's `coherent(*)` column | F = 0 only | `56_probes/q1` source |
| coherence collapses the grading | additive only in evidence; refuted for multiply | `57_probes/p6` is additive; `58_probes/p2_output.txt` |

### 2b. The F-independence argument has a premise nobody in this unit read, and it is a live register question

This is the finding I would most want the second four to take, and it is the one place where the unit's
strongest surviving claim is conditional on something none of its five files mentions.

`58`'s argument holds for **addition of two values at a common scale**. `OPTIONS.md` Q3 asks whether
mixed-numeral addition exists at all, and lists three options: none exists, it exists and its result
numeral is inferred, or it exists only through an explicit conversion. Q3's own text records it as
still open, with `23` quoted saying "one sentence from op collapses it".

**Not one of the five files in this unit mentions Q3.** `grep -c 'Q3'` over `55`, `55b`, `56`, `57` and
`58` returns 0 for all five.

The consequence is direct. If mixed-numeral addition exists as an operation (Q3's second option), then
adding a value at scale `F` to one at scale `F'` requires aligning them, alignment is a shift, and a
shift is precisely the grid coarsening `57_probes/p4` measured breaking associativity with no clamp
present and `58` argued is structurally unavoidable. **Under Q3's second option, addition acquires
exactly the mechanism that kills multiplication, and every entry in the surviving column above becomes
conditional.** Under Q3's first or third option, they stand as stated, because every addition the design
admits is same-scale.

So the unit has produced a strong unconditional-looking result whose unconditionality is an open design
question's answer. That is not a defect in `58`, which was answering a different question and bounded
itself honestly. It is a gap between two parts of the register that no file has bridged.

### 2c. Where the unit's results sit against arvo's actual domain

Blunt version, because it is the thing a canon draft would get wrong. `UFixed<I, F, S>` is unsigned and
usually has `F > 0`. `IFixed<I, F, S>` is signed. Against that:

- **Unsigned, addition:** the good cell. Coherent, associative, absorbing, no accumulator needed, at
  every F. This is arvo's best case and it is genuinely good.
- **Unsigned, multiplication, F > 0:** not associative, not distributive, accumulator grows linearly in
  fold length with no closed form.
- **Signed, addition:** incoherent, 70.1 percent divergence at n = 8, real accumulator grade.
- **Signed, multiplication, F > 0:** nothing has been measured and both known failure mechanisms apply.

`57:624-626` made the sharp version of this point and nobody followed it: "**The signed case is where
the algebra is worst and it is the case Warm would actually be**", since a general-purpose numeral is
signed and I3 points Warm at Rust's primitives. Three of the four cells above are bad, one of them is
entirely unmeasured, and the unit's law-layer machinery was built and validated almost entirely in the
one good cell.

---

## 3. Two things the unit holds without noticing it holds them

Reported because they are cheap and because a later expert starting from nothing would spend a dispatch
rediscovering them.

**`55b` and `56` have one theorem from two sides.** `56` section 6.2 measures that for a signed value
set, raw-order agreement and raw-adder correctness are mutually exclusive over bijective encodings, and
closes with the redundant-encoding case unexamined (`56:508-510`). `55b:136-138` observes, from the
induced-ring reading, that "a finite cyclic group admits no translation-invariant total order". Those
are the same fact. Raw-adder correctness makes the pattern space a homomorphic image of the wrapping
group; raw-order agreement demands a translation-invariant total order on it; a cyclic group has none.
Stated that way the theorem is width-general and **does not need the bijectivity restriction**, which
would close the hole `56` named. Offered as a hypothesis for someone to test, not as a result: I have
built nothing.

**`57`'s congruence argument is sound and is not written as a proof.** `x ~ y` iff `x == y` or both
`>= M`. I checked it by hand for `M >= 1` across the cases that matter (both saturated, one saturated,
multiplication by zero) and it holds. `57` reports it as measured at five values of M. It is provable
in a paragraph, and the difference between "measured at five widths" and "proved for all widths" is
exactly the difference `57` itself says makes a law layer able to state a thing rather than report it.

---

## 4. Redirecting the second four, in priority order

This is the part of the slot that cannot be recovered later. Four items, ordered by what they unblock,
with what each is for and what would count as done. My reasons are the persona's; an expert who thinks
the ordering is wrong should say so in its file rather than silently reorder.

### P1. The chain question, derived cold

**Why first.** Every one of the five files inherits `55`'s standard model, `computed = adapt(exact)`,
and not one attacks it. That model is **per operation**. `58` then found that a chain of eager
fixed-point multiplies at `F > 0` cannot be written in that shape at all, because the scales do not line
up: `rho` narrows to scale `F`, the exact product lives at scale `2F`, and there is no single reduction
that closes the chain (`58` section 2.2). `58` filed that as a bound on `57`'s theorem. It is also a
statement about the model every file in the unit is standing on, and nobody has taken it that way.

Op's I7 is a chain intent in his own words: Precise is "accurate within chains and ops, not only
alone". The unit has an operation-level format concept and an intent that quantifies over chains, and
the two have not been reconciled.

**The dispatch.** Phase one blind, per `RULES.md:196-207`: premises only, no panel files, no register,
no probes, no commit log. Question: what must the one format concept say about a **chain** of
operations, as distinct from an operation? Then phase two reconciles against `55`, `57`, `58` and Q11.
This unit bought its cold derivation at slot one and got its money's worth; the chain question is
effectively a new claim and deserves the same treatment.

**Done looks like:** a chain-level statement that either factors through the per-operation model or
demonstrates it cannot, with the multiplicative case as the test instance.

### P2. Q3, because the unit's one unconditional result depends on it

**Why second.** Section 2b. Cheap, and it converts a conditional into an unconditional or the reverse.

**The dispatch.** Read Q3 and `58` section 2.1, then measure additive associativity and absorption for
a **mixed-scale** addition under each of Q3's three options, at several scale pairs. Also state, for
each option, what happens to every row of section 2a's surviving column. Report whether Q3 is a question
op has to answer before the consolidation can state anything unconditional about addition, or whether
the answer is the same under all three options.

**Done looks like:** either "the additive results are unconditional under all three of Q3's options,
here is the sweep", or "they hold under options one and three and here is what breaks under option
two", with the second phrasing handed to op as a consequence he should know about before he answers Q3.

### P3. Absorption against coherence, measured

**Why third.** Section 1a. Three files now depend on an identification made in one paragraph, the two
probes quantify differently, and the register is about to be rewritten on the strength of it. Cheapest
item on this list by a distance.

**The dispatch.** Run `56`'s C-law and `57`'s absorption predicate over the same configuration sweep and
diff the boolean vectors. If they agree everywhere, say so and the identification is measured rather
than argued. If they separate, characterise where, and the panel has two criteria to name rather than
one. Either outcome is a result. While in there, the same expert should measure the **wrap ring at
F > 0**, which `58:441-445` names as unmeasured and inferred, and which is one arm away.

**Done looks like:** the diff, with counts, and a statement of which predicate the register should
carry.

### P4. The signed case, which is arvo's default and where nothing has been measured

**Why fourth.** Section 2c. `57` named it and moved on. The law-layer machinery this unit built was
validated overwhelmingly in the unsigned additive cell, and three of arvo's four cells are worse, one of
them entirely unmeasured. A consolidation that states the unit's results without this will describe a
substrate that is better behaved than the one being designed.

**The dispatch.** Measure signed saturating and signed wrapping **multiplication** at `F > 0`: induced
structure, absorption, accumulator grade. Then state, per cell of the sign-by-operation-by-scale cube,
what the law layer may license. Then take `57:624-626` seriously and say what it costs Warm, given I3
and I4, that the general-purpose signed numeral sits in the worst cell.

**Done looks like:** the cube filled in, with the empty cells named as empty rather than inferred.

### If a fifth thing gets touched

The order/adder theorem in section 3, restated from the no-translation-invariant-order fact, which would
close `56`'s redundant-encoding hole for free. And `58`'s `min_w == full_w - F` has two data points
(n = 3 and n = 4) and `58` says so; extending it to n = 5 and 6 is trivial and it is currently a pattern
called exact from two observations.

### And a note to whoever dispatches

`57` and `58` both end with numbered questions to specific predecessors, and `RULES.md:232-235` says an
expert is **resumed, not replaced**. `55`, `55b`, `56` and `57` all have unanswered questions on the
record. Resuming one of them to answer is worth more than a fifth fresh opinion, and it is the mechanism
by which this topic converges rather than relays.

---

## 5. What is op's, and what the experts can still settle

Separated because the panel's expensive habit is asking him things that are not questions for him.
`RULES.md:50-60` forbids escalating a measurement dispute; nothing below is one, because nothing in this
unit is priced and no bench has run on any of it.

### Op's, and I would not answer any of them for him

**Q3: is there a mixed-numeral addition?** Already in the register as unanswered, already flagged by
`23` as collapsible by one sentence from him, and now load-bearing for the unit's strongest result in a
way the register does not record. This is the one thing from unit two I would put in front of him. It is
an intent question about what the design admits, not a measurement.

**What I7 means for product chains.** `58` section 3.4 established that a chain of multiplies cannot be
made exactly reassociable at any bounded accumulator width, the way a chain of sums can. So Precise's
"accurate within chains" is either an unbounded accumulator, which is a real cost growing linearly in
fold length, or a stated error bound instead of an exactness guarantee. That is a choice about what his
own stated intent means, and `OPTIONS.md` Q14 currently has no line saying the choice exists. **I would
not pick.** The persona's view, offered as nothing more: the error-bound reading is the one that
survives contact with fixed-width hardware, and the exactness reading is the one that matches the words
he used, and that tension is exactly why it is his.

**Whether the signed default is acceptable given section 2c.** If Warm is signed and imitates Rust
primitives per I3, it sits in the cell where the algebra is worst and where the accumulator cost is
real. That is a consequence of his stated intents meeting a measurement, and what to do about it is a
design call. It is not ready for him yet: P4 has to fill the empty cells first, because half the
statement is currently inferred.

### The experts can settle, and should not ask

The absorption-versus-coherence identification (P3). The wrap ring at `F > 0`. Redundant encodings.
`min_w == full_w - F` past n = 4. Rounding modes beyond the two `58` tested, which `58:435-439` names
itself. The phase-times-multiplication interaction `56:310-312` left as a gap. Whether the ladder
composes, which `57:651-652` failed to refute over twelve configurations and correctly declines to bank.
Whether the canon states the concept as four slots or as `56`'s identity-plus-realisation split, which
`56:527-529` calls a drafting choice on which it has a preference and no proof: two more files should
converge on it, and only if they cannot does it go anywhere.

### And one thing that is neither

Nothing in unit two is priced. Every number in all five files is a count of counterexamples from a
committed probe. No bench harness has run on any of it, and the accumulator-width results in particular
read as though they had cost implications that have not been measured. `57:503` says this about its own
one-bit finding and is right to. If the consolidation carries a width saving as a design consideration,
it carries it as unpriced.

---

## 6. What the register should gain

Reported, not applied. I have edited neither file.

**Q17 should gain the additive extension.** It currently records the additive half as unconditional,
which is right. It should also record that the same argument makes `35_probes/p3`'s divergence table,
`57_probes/p6`'s one-bit result, `55b`'s pullback counts and `56`'s `coherent(+)` column F-independent,
because every one of them is a same-scale addition and none of them says so. And it should record the
premise: **the argument covers same-scale addition, and Q3 is open.**

**Q17 should gain the two rows `58` did not take.** The induced **ring** for wrap is `F = 0` in exactly
the way the semiring was (`55_probes/p4`'s `mul` is bare), and `56_probes/q1`'s `coherent(*)` column is
`F = 0` too. Both are currently uncaveated.

**Q3 should gain a line saying what now depends on it**, pointing at Q17. It has sat open since `23` on
the grounds that nothing had a caller; something does now.

**Q12's droplist entry, when it is written, should say what actually happened.** `42` refuted
clamp-counting itself, published the refuting row, and stated H2 in `42_probes/p3.rs:25-28`; its prose
summary then used "clamps" where its own table used "boundary". The entry as `57:604-610` proposes it is
accurate about the sentence and reads as a larger indictment than the record supports. Absorption is the
replacement and `p2`'s counts are the evidence; the diagnostic should name the slip rather than the
file.

**Q12 should say what 4248 counts**, per section 1c, and should carry the 100-interval sweep beside it,
because that is the number of format-shaped configurations anyone has checked.

**Q11's accumulator option**, per `58`'s recommendation, needs the additive-only qualification. I have
nothing to add to `58`'s wording except that the qualification is unconditional in F on the additive
side, which strengthens rather than weakens the option for the case it does cover.

---

## 7. Coverage, bounded honestly

**What I opened.** The five unit files in full. `42` sections 3 and 5.2 with its `p3` source and output.
Twelve probe sources, listed in the header, read for their operations rather than in full. Four probe
outputs. Seven register sections.

**What I did not.** `08`, which is the topic's only prior file and which `55` and `56` both engage with
at length. Every statement above about `08` is taken from `55` or `56` and I have marked none of it as
mine, because I made none. `35`, `18`, `20`, `25`, `40`, `43`, `DROPLIST.md`, `seed/`, `archive/`.

**What I did not do.** I re-ran no probe. Every count is read from a committed output or produced by a
grep I printed. Where I say a source contains no scale token, that is a fact about the text and not a
proof that the claim is integer-only; I argued the second separately in section 2a and the argument is
mine and is owed a check.

**What is mine and first-read.** The quantifier gap between `56`'s C-law and `57`'s absorption (1a). The
extension of `58`'s F-independence to `35_probes/p3` and `57_probes/p6` (2a). The Q3 dependency (2b).
The observation that `55b` and `56` hold one theorem from two sides (3). Each is one persona's reading
and none has been attacked.

**What I could not determine.** Whether the C-law and absorption actually separate, which is P3 and
which I did not build. Whether the redundant-encoding generalisation in section 3 holds, which I argued
in a paragraph and did not test. Whether the chain question has an answer, which is P1 and is the reason
it is P1.

**Nothing here settles anything.** There is no canon, the mode is explore, and I have no authority
under it or outside it.
