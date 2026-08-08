# 23. The sentences a canon could carry

**Date:** 2026-08-08. **Author:** Peyton Jones persona. **Status:** inventory, not consolidation.
Nothing here settles, per `04`.

This is a stocktaking file. The panel has run twenty-two files and the question put to it now is not a
design question at all: of everything the night produced, what could a canon actually carry, what is
each one worth on the provenance ladder, and do the survivors fit together into anything with a spine.

Three things it deliberately is not. It is not the consolidation, which is the canon candidate and is
not written tonight. It promotes nothing, and where a sentence needs an open question answered before
it can be written at all, it is listed as blocked with the question named. It does not adjudicate, so
where two files disagree the disagreement is recorded and left standing.

The registers throughout are the three `04` permits. *This appears to hold.* *This is what the evidence
supports.* *This is blocked on an open question.* I have tried to use them exactly and not as softeners.

## The answer, before the working

**Thirty candidate sentences.** Of those, **twenty-one are carryable now** in some form, **seven are
blocked** on a named open question, and **two I am refusing as canon** with reasons. On the ladder:
**zero ratified**, **five at two experts**, **twenty at one expert**, and five rows carry no rung
because they are refusals or blocked without a derivation behind them.

Every count in this file is produced by a command named beside it, per `RULES.md:124-126`, and I have
corrected two of them mid-file after running the command rather than leaving the estimate I wrote
first. The correction is visible in section nine and is the only reason the two-expert count is five
rather than three.

They do not cohere into a canon. They cohere into **three separate clusters with one genuine
load-bearing joint between two of them and no joint at all to the third**, plus a fourth cluster that
is about instruments rather than about numerals and should not enter a canon at all. The spine is
missing in a specific and nameable place, and section six names it.

The checkpoint's verdict at `19:716` **held when it was written and has been partly overtaken by two
files written after it**. Section eight is that argument. The short form: the verdict's stated reason
was that the canon must assert doability and the doability for arvo is a workload claim nobody had
made. `20` and `22` made workload claims. What they measured, however, is not what the verdict wanted
measured, so the verdict survives in a narrower and sharper form than it was stated in.

## Canon gate

Ran before the assigned work, against `RULES.md`, `01` and `04`.

**Aligned, and the alignment is the unusual part.** This dispatch is an inventory that promotes
nothing, which is precisely what `04:27-33` demands of every file tonight: "A file may say *this
appears to hold* ... A file may not say *this is settled*." An inventory that rates rungs and refuses
to promote is the one shape of file that cannot violate it.

Two things I checked rather than assumed.

**There is nothing ratified for this file to conflict with.** `01` section 0 establishes that op's word
ratifies only after convergence, and `04:35-37` extends it: "even convergence between experts does not
settle anything tonight". `SETTLED.md:3-14` then carries its own warning that every row it marks
RATIFIED was classified under the superseded reading. So the correct posture for an inventory is that
**every row in `SETTLED.md` is at best an ack until someone reads the record behind it**, and I have
treated it that way throughout. Where I cite a `SETTLED.md` row I say what it is resting on.

**The one thing that would have made this dispatch misaligned is absent.** If the panel had a
consolidation on disk, an inventory of carryable sentences would be a second consolidation by another
name and would collide with `RULES.md:140-147`. It does not. `CANON_CANDIDATE.md` in this directory is
the predecessor's carried artifact, dated 2026-08-07 22:38, before this panel's first expert file at
23:50. Verified:

```
$ stat -f "%Sm %N" -t "%Y-%m-%d %H:%M" CANON_CANDIDATE.md 02_carried_*.md
   2026-08-07 22:38 CANON_CANDIDATE.md
   2026-08-07 23:50 02_carried_what_replaces_the_two_refutations.md
```

So there is no live consolidation for this to duplicate, and the dispatch's own instruction that "the
consolidation is the canon candidate and nothing may be promoted tonight" is consistent with the tree.

## Checking the brief, before reasoning from it

Four checkable claims. Three hold. One is off in a way that matters to how this file's own findings
should be read.

**"A checkpoint's verdict, after eighteen files: converging on a methodology for evaluating canons,
rather than on a canon."** Substantially right, wrong in one word, and the word carries the argument.
`19:682` titles the section "Canon, or a methodology for evaluating canons". But the verdict sentence
at `19:716-717` is:

> converging on a methodology **for the mechanism**, and not yet on a canon, because the doability the
> canon has to assert is a workload claim and the panel has not made one.

"For the mechanism" and "for evaluating canons" are different diagnoses with different remedies. The
first says the panel built instruments for a thing rather than describing the thing. The second says
the panel built a theory of canon-quality. `19`'s own reason clause settles which it meant, and it is
the first. This matters because the remedy `19` names, at `19:722-725`, is a bench and a second read
rather than more theory, and **both were then done**, in `20` and `22`. Section eight.

**"It counted five canon-shaped sentences in one stretch, more than either earlier stretch, so the
trend is up."** Holds. `19:690-701` lists exactly five, then names `13`'s and `11`'s and `08`'s from
the earlier stretches. `19:896` marks the selection as its own guess, which the brief does not carry
and which is fair for it to have dropped.

**"A checkpoint found the map inflating a rung, and a later check found four more."** Off by three, and
in the direction that overstates how much new checking has happened. `21:140-141` says plainly: "Four,
and three of them were already found and flagged by earlier persona checkpoints." So the honest form is
that `21` found four rung errors of which **one** was new (`21` §2.3, `20`'s contamination declaration
dropped), and the other three were re-flags of standing unrepaired findings. The correction is not
pedantry: the brief's version reads as an escalating discovery rate, and the truth is a **static defect
count that nobody has repaired**, which is a worse condition and a different remedy.

**"Its third clause is ambiguous with three readings and the panel never recorded which it answers."**
Holds, at `17:215-219`, and `17` states it as a question for op rather than a defect to fix.

I also checked the forbidden-feature list against the workspace rule, since a sentence resting on a
misremembered ban would be worthless. `generic_const_exprs` forbidden (op, 2026-07-28); full
`specialization` forbidden; `generic_const_args` requires `-Znext-solver=globally`, which is excluded.
`min_generic_const_args` is on the allowed list. The brief is accurate.

## What I counted as a carryable sentence, and how I rated it

A sentence goes on this list when it is **about numerals, their derivation, or their laws**, and when
it could be written in a canon under `RULES.md:70-77`: intent, theory, shape, notation and pseudocode
permitted, the concrete spelling of an implementation forbidden.

Three classes of true and valuable finding are therefore **excluded**, and I want that stated up front
because excluding them is most of why my count is lower than a naive read of the night would suggest:

**Findings about instruments.** That the erasure oracle has two false-negative regimes, that a
one-output derivation passes the certifying check, that a defect matrix catches 174 of 420 where the
naive suite does not. These are true, they are among the best work in the panel, and they belong in the
audit trail. A canon that carried them would be describing its own verification apparatus, which fails
permanence the moment the apparatus is rebuilt.

**Findings about the record.** That a compression dropped a qualifier, that a count is disputed at 81
against zero, that a bench harness never called its validation pass. Audit trail.

**Findings about Rust.** That a type alias does not check its bounds, that `E0119` refuses two honest
readings of a cross-family join. These are load-bearing *reasons* for canon sentences and they are not
themselves canon sentences, because a canon that says "Rust refuses this" is describing a toolchain.
The canon sentence in each case is the design consequence, and I have listed the consequence.

On rungs I have been strict, per `RULES.md:199-215`. The default dispatch shape in this panel makes
TWO EXPERTS unreachable, so a sentence gets that rung **only** where a dispatch explicitly inverted the
order and the specific claim was derived before the predecessor was read. Three dispatches did that:
`13` against `12`, `16` against `15`, and `20`'s sections outside 1.5. Everything else is ONE EXPERT
however many files agree with it.

I have also applied a distinction the panel has not been applying, and it changes several rows.
**Independent derivation and independent instrumentation are different things.** `07` corroborating
`06`'s tight product form at 400 of 400 by a derivation that never uses `06`'s inequality is a second
*instance of evidence* and not a second *expert*, because `07` read `06` in full first and says so. The
map conflated these at `MORNING.md`'s "which is the bar" line, `21` §2.1 caught it for the fourth time,
and the conflation is exactly what would inflate this inventory if I let it.

---

## The inventory

Rows are grouped by what they are about rather than by which file produced them, because the grouping
is itself the coherence finding in section six. Each row gives the sentence as a canon would write it,
its evidence, its rung, and the two tests.

### Cluster A: what a numeral is

The concept layer. This is the cluster in the best shape, and it is also the one furthest from
anything a consumer touches.

#### S1. The boundary of the format concept

> A representation is a numeral when a datum denotes exactly one rational, when the denotable
> magnitudes in each binade of some admitted radix form a single arithmetic progression at a single
> phase whose step is that radix raised to some power, and when the value set is fixed by the type
> alone.

**Evidence.** `08:552-560`, offered by its author as "one sentence, offered as the thing to attack".
`08:562-575` gives the reason each clause is where it is. `08_probes/i1b.out` for the classification of
twenty-one representations, eighteen inside. `08:20-32` restates the test in the file's summary.

**A note on how I have split it.** S1 and S2 are **one sentence in the source**. I have split them
because they do different jobs, one defining membership and one saying what non-membership means, and a
canon may well want them in two places. The split is mine and the source should be read as one.

**Rung: ONE EXPERT**, and its author says so about this exact sentence rather than in general.
`08:696-703`:

> Every section is a first read, and where this file agrees with `03`, `06` or `07` I read all three
> before deriving, so the agreement is inherited. What is independent is the measurement. Specifically
> owed a second read: ... section 5's boundary sentence, **which is the thing I would most want
> attacked**.

That is a member naming its own strongest result as the one most owed a check, which is the behaviour
`RULES.md:99-101` asks for and is rarer than it should be.

**One limit the sentence carries and a canon must carry with it.** `08:578-585` states two things the
boundary does not decide: which exponent shapes the design admits, which `08` calls "the real question",
and anything at all about the endpoints, which `08` held aside throughout including in its probe. So S1
is a membership test and **not** a specification of the design's shape space. A canon that carried it as
though it were would have skipped the harder question.

**Permanence: passes.** It quantifies over representations rather than over an implementation, and
nothing in it names a language or a container.

**Equivalence: passes, and this is the strongest equivalence case in the inventory.** Three teams
applying that test to the same twenty-one representations would partition them identically, because the
test is mechanical. That is not a claim about the sentence's beauty, it is the observation that `08`
ran the test and it carved the whole survey.

#### S2. The failing clause names the layer

> A representation outside the numeral concept fails exactly one of the three clauses, and which clause
> it fails names the layer it belongs to instead. Failing the denotation clause makes it an
> approximation domain. Failing the progression clause makes it a different number system. Failing the
> type-fixity clause makes it a storage encoding.

**Evidence.** `08:556-560`, the second half of the sentence quoted under S1, with the per-clause
reasons at `08:562-575` and the classifications in `08_probes/i1b.out`. `18:432-436` reaches the same
shape for the denotation clause specifically and by a different route, pricing it rather than
classifying it.

**Rung: ONE EXPERT.** `18`'s agreement is on one of the three clauses and `18` read `08` first.

**Permanence: passes.** **Equivalence: passes**, with a caveat I will not soften: it passes *given* S1,
and inherits every uncertainty in S1's third clause. If the type-fixity clause moves, S2's third arm
moves with it.

**This is the row I would most want a second, order-inverted read on**, because it is doing the most
architectural work of anything in the inventory. It is the sentence that tells a canon where its own
boundaries are, and it currently rests on one expert who declared its own agreement inherited.

#### S3. A partitioning set denotation is free; an overlapping one is not

> Where a datum denotes a set rather than a point, the design pays nothing if the denoted sets
> partition the values, and pays the value-level total order if they overlap. Since the order is a
> precondition of the law layer, admitting an overlapping set denotation is a change to the canon
> rather than the addition of a type.

**Evidence.** `18:432-436` for the distinction; `18:193` and `18:199-200` for the cost, decidability
falling from 42.05 percent to 35.45 percent at the next width against 100 percent for point, cell and
absorbing denotations; `18_probes/`.

**Rung: ONE EXPERT.**

**Permanence: passes.** It is a statement about what a denotation choice costs, and the cost is stated
in terms of a law rather than of a mechanism.

**Equivalence: passes.** Three teams told this would all exclude intervals and all admit rounding
cells, which is the observable consequence.

**This is the best single sentence the panel produced tonight** and I want to say why rather than
asserting it. Before `18`, the design excluded intervals and admitted rounding cells and there was no
principle distinguishing them; `18:435-436` says so directly, that "until now that has looked like an
accident of which cases came up". One sentence turned two unrelated decisions into one rule with a
stated price. That is exactly the compression a canon is for.

#### S4. The sign domain determines nothing on its own

> The numeral's declared members determine a value set and the order is defined on value sets. The
> radix and the exponent form determine the grid and the phase. The sign domain determines nothing
> alone: together with the precision and the radix it fixes where the value set's two endpoints fall,
> and it moves neither the grid nor the phase. It is a presentation parameter of the range rather than
> a coordinate of the order.

**Evidence.** `02:226-238`, which states it as a canon sentence and applies both tests to itself.
`SETTLED.md:121` carries the negative half at TWO EXPERTS from the closed panel, `146` and `148`,
"refutation only".

**Rung: ONE EXPERT for the positive statement.** The negative ("the sign domain is not a partition of
the order") is at two, carried. The positive is `02`'s and has not been second-read. `SETTLED.md:147`
lists "what the sign domain is, as opposed to what it is not" as open, and this row is the candidate
answer to that open item. Nobody has checked it.

**Permanence: passes.** **Equivalence: passes conditionally, and the condition is a live blocker.**
`02:240-252` finds the design has never settled whether `Precision` counts the sign digit, and gives
its own family table producing both answers on adjacent rows at `138:92-96`. Under one reading the
three sign domains form a chain; under the other two are incomparable. **Three teams would not
converge** until that is settled, so S4 passes equivalence only in its qualitative form and fails it in
any form precise enough to tell a consumer whether `IFixed<3, 4>` gives seven integer bits or eight.

**Status: carryable in the qualitative form, blocked in the precise form**, on whether precision counts
the sign digit.

#### S5. BLOCKED. What an absorbing endpoint denotes

The intended sentence was that a saturating numeral's top denotes everything above it, and that this is
sound exactly while the computation stays at the endpoint.

**It cannot be written, and the reason is instructive.** `18` established the sufficient half, 936 of
5184 chains unsound under the absorbing reading once subtraction enters. The "only if" half is refuted
by `18`'s own committed output: `18_probes/p2.out` section B has add-with-multiply-by-zero, an
operation set that decreases, with absorbing sound at **zero** of 512. `21:32-72` and `19` §5.6 both
reproduced this from the same table.

So the design knows a condition that is sufficient and does not know one that is necessary. `19:890-894`
offers a candidate, that every operation must map the absorbed set onto an exactly denotable set, and
marks it as the persona's, unbuilt, and owed to an expert. `18:403-404` had already asked for exactly
this to be second-read. **No second read has run.**

**Named blocker: what the necessary condition is.** One probe of the shape `18` already built would
settle it. This is the cheapest unblocking item in the inventory and it gates S5, gates the `Precise`
on `inexact` question, and gates any canon sentence about saturation.

### Cluster B: the order, and where a numeral is inferred

#### S6. The order is infrastructure; the lattice is a theorem about it

> The canon states the inclusion order on numerals in full, and states the lattice as a property of
> that order scoped to where it holds. The design calls the relation; it does not call the extrema.

**Evidence.** `06:503-506`, resting on `06`'s site enumeration at `06:341-348` and its two genuine
order callers at `06`'s section 5.

**Rung: ONE EXPERT.**

**Permanence: passes**, cleanly. It survives any rewrite because it is a statement about which of two
mathematical objects the design depends on.

**Equivalence: passes.** Three teams would all implement a two-place relation and none would implement
a join operation, which is the observable difference and is a large one.

#### S7. There is no site where nothing determines the numeral

> Every position in the design where a numeral appears is determined by the consumer, by the operands,
> or by a named rule. A position determined by none of those cannot exist, because an associated type
> names exactly one type and declaring both honest readings of a cross-family target is refused for
> conflicting implementations. A design that appears to infer a cross-family target is choosing in one
> implementation rather than computing a least upper bound, whatever its order says.

**Evidence.** `06:341-348` for the D0/D1/D2/D3 classification, 8 / 8 / 3 / 0 sites; `06_probes/`
`p4_cross_family_join.rs` for the `E0119` refusal; `06`'s arm 2 compiling, which establishes that the
type system accepts any cross-family formula and holds no opinion on which is right.

**Rung: ONE EXPERT, compiled.**

**Permanence: passes in its first sentence and fails in its second.** The first sentence is about the
design. The second names a compiler diagnostic, which is `RULES.md:75-77`'s forbidden register and
would also be false in a language without coherence. **The repair is to state the consequence and put
the diagnostic in the audit trail**: the canon says a cross-family target is chosen rather than
computed, and the audit trail says how that was established.

**Equivalence: passes** for the repaired form.

**One fragility I found in the source and am passing on rather than resolving.** `21:122-126` notes
that `06:718` refers to "the six sites that infer nothing" where `06:341` counts eight, and that `06`
classifies site 1 twice. So the *count* in this row is soft. The *emptiness of D3*, which is what the
canon sentence turns on, does not depend on the count and is compiled.

#### S8. The natural product form is not the tightest honest answer

> A canon may not claim that the numeral derived for a product is the tightest numeral holding every
> product of the operands. The sum-of-widths form is tight almost everywhere and wastes exactly one
> bit where an operand denotes only zero, or where the narrower operand's total width is one. A tight
> form exists and its predicate is a one-line condition.

**Evidence.** `06:48` and `06:551` for 6100 of 6561 tight, 461 wasting one bit, characterised exactly
as 160 plus 301; `06`'s section 7.3 for the tight form built gate-free with a negative control failing
at `E0080`; `07:57` and `07:465` for the independent corroboration at 400 of 400 operand pairs by a
derivation that does not use `06`'s inequality.

**Rung: ONE EXPERT, with two independent instances of evidence.** I want to be exact about this because
the map got it wrong four times. `07` read `06` in full before deriving, so it is not a second expert.
Its measurement is arrived at differently, so it is a second *instance*. `RULES.md:116` sets the
evidence bar at three. **This row has two instances and one expert, and is short of both bars.**

**Permanence: passes.** **Equivalence: passes**, and note the row's shape: it is a *prohibition* on a
canon sentence rather than a canon sentence. That is a legitimate and undervalued kind of canon
content. A canon that says "the derived numeral is the tightest honest answer" would be false, and
knowing that a sentence is false is worth as much as knowing one is true.

`06:732-737` states the choice cleanly: change the formula, or change the sentence. Both are
defensible, and `06` does not pick. Neither do I.

#### S9. BLOCKED. What the tightest numeral computation admits

`07:425-431` restates for the canon, without its frame's vocabulary:

> The tightest numeral holding a given set of values is computed by taking the coarsest grid that
> carries every value and then the smallest reach that covers the largest. That computation always has
> an answer. Its answers include the shape that holds only zero, and shapes whose integer width is
> negative. A design that refuses either is a design whose tightest answer sometimes does not exist.

**Evidence.** `07_probes/p3.out` Q2 over 2,796,636 sets: the origin is the answer for exactly 1 input,
namely the singleton zero; negative integer width for 5,487; and **0** inputs need both at once.
`07:415-421` for the Moore condition measured independently by two routines agreeing exactly.

**Rung: ONE EXPERT.**

**Why it is blocked, and this is the sharpest collision in the inventory.** S9 names *negative integer
width* as a thing the design must admit. **S14 removes integer width from the design's coordinates
entirely.** Under `15`'s keying on total and fraction width, the shapes S9 is talking about still exist
and are all naturals, and the sentence "shapes whose integer width is negative" becomes a statement
about a derived view rather than about anything stored.

The two are not in contradiction. They are in **different coordinate systems**, and a canon cannot
carry both spellings of the same claim without a reader concluding the design has two notions of width.
Which spelling the canon uses is downstream of op's own question at `MORNING.md:20`, does a consumer
write the integer width or the total width, and `15:697-707` argues that question is prior to the
family question.

**Named blocker: which coordinates the numeral is keyed on.** Until that is answered, S9 is
unwriteable, not because its content is doubted but because its vocabulary is.

#### S10. The named shapes are meet-closed and not join-closed

> The design's named shapes are closed under the meet and are not closed under the join. Gradual
> underflow is the meet of a fixed-point format and a float. The join of the same two is the mirror
> shape, a float below and a fixed grid above, and the design has no name for it.

**Evidence.** `08:44-52`, measured twice, once by enumeration and once by the trait solver in a
diagnostic (`i2f_meet_lands_in_knee.out`, `p3_negctl2.out`). `08:58` establishes that `03`'s own
witness pair does have a join in the general class, `{0, 1/2, 1, 2, 3}`, strictly inside both minimal
upper bounds `03` named.

**Rung: ONE EXPERT, two instruments.** The two instruments are genuinely different in kind, an
enumeration and a trait solver, which is stronger independence than most pairs in this panel.

**Permanence: passes.** **Equivalence: passes.**

**Its real value is that it dissolves someone else's problem.** `03` measured an antichain in the
cross-kind join and concluded a structural failure. `08` shows the antichain is an artifact of the
**named** shapes rather than of the concept. That is a correction between files that improves both, and
it is why this row belongs beside S1 rather than in Cluster C.

#### S11. BLOCKED, and offered as a shape rather than an answer. The ambient decomposition

`03:655-666` proposes that the canon define meet and join in the ambient lattice of finite rational
sets, where they are total and free, define a partial realisation from a value set to a numeral, and
make every operation the composite. Every structural failure in the design then has one address:
realisation.

`03` is explicit about what this is worth and what it is not, and I am keeping its framing rather than
improving it. It **changes what a canon sentence quantifies over**: adding a family later adds a
theorem about where realisability holds, rather than invalidating a sentence about which operations are
total. `03:661-664` calls that the permanence test coming out clearly different for one of three
readings. And `03:667-669` refuses to call it a fourth answer, because it does not decide whether the
singleton is realisable, "which is op's question in different words".

**Rung: ONE EXPERT**, and `07`'s section 3.2 supports it from another direction by showing the join and
the product numeral are the same function at different arguments, so formula-versus-lattice was never a
real separation.

**Permanence: this row exists because it passes permanence better than the alternatives**, which is an
unusual and interesting basis for a design choice.

**Named blocker: op's family question**, which S11 relocates rather than resolves. `03` says so itself.

**And a cost `03` states that I would not let a canon drop:** the ambient object must be declared a
specification device that no consumer ever holds, because under the erasure gate it has no type-level
existence at all. A canon that introduced the ambient lattice without that sentence would be inviting
an implementer to build it.

### Cluster C: the width surface

This cluster is where the night did its hardest compiled work, and it is the cluster whose sentences
are least like canon sentences. Three of the five are about what a language costs.

#### S12. No finite table is closed under the design's own law algebra

> A design that carries a written table from consumer-facing widths to type-level widths cannot close
> that table, at any size. Multiplication produces an output width from two input widths, so a table
> covering every width a consumer writes does not cover the widths the algebra produces, and adding
> the produced widths by hand fails again one octave up.

**Evidence.** `11:516-601`, compiled in three steps: a failure at 48 from two tabled inputs; the row
added by hand and the failure reappearing at 96 and 32; and the same ladder keyed on nat types instead
of consts compiling three octaves plus a 1636-bit numeral at 208 bytes with no table anywhere.
`11_probes/b01`, `b02`, `b03`.

**Rung: ONE EXPERT, compiled.** `12` and `13` both build on it and neither derived it.

**Permanence: passes**, and it is worth saying why given how implementation-flavoured it sounds. The
sentence quantifies over *any* finite table under *any* algebra containing multiplication. It would be
true in a rewrite, in another language, and of a design that had never heard of Rust. That is the
permanence test doing its job on a sentence that looks like it should fail it.

**Equivalence: passes.** Three teams told this would all refuse a finite width table, which is a large
observable difference.

#### S13. The ceiling belongs to the const surface, not to the bridge

> The mechanism that maps a consumer's written width to a type-level width is total, uncapped and
> free of enumeration in one general rule. What is bounded is the reverse direction, naming a computed
> width back at the const surface. The cost therefore belongs to the surface's choice of coordinate
> and not to the mechanism.

**Evidence.** `11:602-643` and `11_probes/e01`, a total uncapped enumeration-free const-to-type bridge
in one blanket impl, gate-free, overshooting by exactly eight, with the closing division refused in
four further syntactic positions all terminally naming a forbidden feature. `13:597` compiled the
ceiling firing exactly as a missing reverse-table row (`13_probes/p23`, `p24`).

**Rung: TWO EXPERTS.** I first rated this from `MORNING.md:687-693`'s account and then went and
checked, because taking the map's word on a rung is exactly the move `21` found going wrong four
times. `13:1-30` states its own working order in detail and it is stronger than the map's summary:
probes `p01` through `p27` and everything from "Reading the bar" down to "What I did not cover" were
on disk **with `12`, `11` and `10` unread**, and `13` names the two sections that were not, neither of
which is this one. `13:657-658` lists the claim among what it derived, "The ceiling is the cost of
naming outputs, not of the bridge. **Compiled, and it fires as a reverse table miss.**"

So `11` localised the ceiling, `13` reached it without having read `11`, and both compiled it. That is
the rung as `RULES.md:28-30` defines it, and it is the cleanest instance of it in the panel.

**Permanence: passes.** **Equivalence: passes.**

#### S13b. The consumer's written form is not the compiler's parameter form

Added after the check described in S13's rung, which surfaced a claim I had missed and which is at a
higher rung than most of this inventory. Rows are not renumbered, so nothing already cited shifts.

> What a consumer writes and what the type system receives are separate choices. A surface may present
> a width as a written literal while the machinery carries it in whatever form the algebra needs, and
> the two need not be the same kind of thing.

**Evidence.** `12:130-158`, measured off compiling text: the alias sites are seven and eighteen
characters, identical to the current surface at every one of the three consumer tiers, and the same
door runs three multiply octaves against a six-row table containing none of the produced widths
(`12_probes/p02`, `p03`, counted by its own `count.sh`). `13:654-656` reaches it separately: "A
nat-keyed design meets the ergonomics bar with the consumer's spelling unchanged. **Compiled, twice
independently.**"

**Rung: TWO EXPERTS.** `13` derived and compiled it with `12` unread, per `13:1-30`.

**Permanence: passes.** It is a statement about a separation between two surfaces and names neither
language nor mechanism.

**Equivalence: passes**, and the consequence is the whole of Cluster C. A team without this sentence
believes the ergonomics bar constrains the machinery, which is what the panel believed for two
stretches, and `12:62-78` establishes the bar never said it.

**And it is the sentence that dissolved a trade rather than resolving one.** `12:459-481` was asked
whether the ergonomics bar had been priced before the ceiling was visible, found it had not, and then
found the two were not in conflict, so no trade existed to be offered. A canon sentence whose effect is
that a fork someone was about to be handed evaporates is worth more than one that picks a side.

#### S14. Cross once, at literals, in one direction

> A width crosses between the consumer's written form and the type-level form exactly once, at a
> literal, in one direction. Nothing computed ever crosses back.

**Evidence.** `13:597`, drawn from the joined result in S13.

**Rung: ONE EXPERT.** It is `13`'s own synthesis of two predecessors' separate results, so the
underlying facts are at two and the sentence is at one.

**Permanence: passes, and it is the most canon-shaped sentence in this cluster.** `14:456` and
`14:629` both reach for it as the sentence most likely to survive, and I agree with them having checked
it separately: it names a discipline rather than a mechanism, it is nine words, and it would be
meaningful in a rewrite in any language with a phase distinction between values and types.

**Equivalence: passes**, and the consequence is sharp. A team implementing it will not build a reverse
table and therefore will not have a ceiling. A team that does not have this sentence will build one.

#### S15. What Rust's position costs, and that every system pays something

> Three of the four properties that remove the crossing are structurally unavailable in a language that
> checks generic definitions before instantiation, each closed by a different consequence of that
> decision. The fourth, running the derivation at declaration sites over concrete numbers, is not a
> language feature and is available. Every system that removes the crossing pays for it in a currency
> of its own: instantiation-time errors, per-call checking, an incomplete arithmetic decision procedure
> in the compiler, or undecidable conversion.

**Evidence.** `11:441-499`, the four properties and the per-system table. `11:42-43` and `11:405` for the
typenum figures, 1148 rows and 4758 generated lines, re-confirmed exact at `14:369`.

**Rung: ONE EXPERT, and the evidence beneath it is uneven in a way the sentence does not show.**
`11:1041-1080` states its own coverage bound: five of its survey sections are recollection rather than
verification, no Ada, VHDL, Idris, Agda, Coq, Scala or F* toolchain being available, and `11` flags the
Ada entry as the one it most wants checked **because Ada carries the one property that is actually
available here**. So the row of the table that matters most for the design is the row `11` is least
confident about.

**Permanence: passes.** **Equivalence: fails, and it should.** This is not a sentence three teams would
implement. It is a sentence that tells a reader why the design looks the way it does. `14:360-364`
calls it "the most canon-shaped thing produced in two stretches" and I disagree with that assessment on
exactly this ground: it is excellent *rationale* and it is not an intent. A canon carries it as the
reason beside another sentence, never as a sentence of its own.

#### S16. BLOCKED. The surface diagnoses at the site the consumer wrote

The intended sentence is that an undeclared or mistaken width is reported at the position the consumer
typed it, naming what they typed.

`12:362-421` found this fails today, under the current surface and under four of five candidates, and
found it on the exact tier the ratified ergonomics bar governs. An undeclared width at an alias
definition produces **no error at all**, because a type alias does not check its bounds; it surfaces
sixteen lines later at the first use, spanning a name the consumer did not mistype and citing an
internal type. `12:398-421` builds a repair that needs an unvetted feature and fifteen library-side
bounds.

**Rung: TWO EXPERTS for the defect, ONE EXPERT for the repair.** `13:662` carries "Every arrangement
is silent at the alias-definition site. **Compiled, twice independently.**", derived with `12` unread
per `13:1-30`. The repair at `12:398-421` is `12`'s alone. Probes `12_probes/p12`, `p13`, `p14`.

**Why it is blocked and not merely unmet.** A canon may state a diagnostic quality as an intent. But
this one's only known repair requires a feature that has not been through `unstable-features.md`'s
vetting procedure, and `RULES.md:85-86` requires the canon to say which things are doable. **An intent
whose only route needs an unvetted feature is a wish until the feature is vetted.**

**Named blocker: vet the feature `12:398-421` needs, or find a second route.** This is a small, bounded,
entirely tractable piece of work that nobody has been dispatched on.

### Cluster D: the derivation

#### S17. The numeral is keyed on total width and fraction width

> A numeral is keyed on its total width and its fraction width. The integer width is a derived view,
> computed for display and never stored. Total and fraction width are naturals at every site the design
> has a caller for; the integer width is not, so keying on it forces a signed ladder that keying on
> total width does not.

**Evidence.** `15:105-124`, compiled over the whole 81-shape box, 6400 plus 6561 assertions, zero
features, with a negative control that refuses and prints the offending shape in naturals. `15:83` and
`06:609` for the corner's honest size, 7 of 625, 11 of 2401, 15 of 6561. `15`'s `q02` output for the
repeated-squaring case, integer width reaching minus thirty-one in five steps while total width stays
at one.

**Rung: ONE EXPERT.** `16` was dispatched under the inverted order but declared itself contaminated on
the count by a commit subject line, and its independent contribution is about the two outputs rather
than about the keying.

**Permanence: passes.** **Equivalence: passes**, and strongly: it is a statement about which two of
three related quantities are primitive, which is exactly the kind of thing three teams either all get
right or all get differently.

**And it is entangled with S9**, as noted there. This row is also the direct subject of op's own
question at `MORNING.md:20`, which is about the *surface* coordinate rather than the *machinery*
coordinate. `15:697-707` argues the machinery wants total-and-fraction, that the surface can keep
integer-and-fraction through the door at zero cost, and that then the numbers typed and the numbers
stored differ. So S17 as stated is a claim about the machinery and is carryable; the corresponding
claim about the surface is blocked on op.

#### S18. The derivation has two outputs

> The derivation from a numeral produces two answers, not one: what an operation on a single value
> lowers to, and what a run of these values looks like in memory. Every layout quantity a consumer
> needs is a function of those two together with the declared width, which the numeral already carries.

**Evidence.** `15`'s three-input map, twenty-four rung impls, two sign impls, four stride impls and
zero width-keyed impls over four strategies times two signs times every width to two hundred, at zero
features (`15:288-306`). `16:580-586` for the corrected two-output statement, and `16:679-681` for why
the second answer is keyed on the strategy-and-rung pair rather than on either alone. `15:345-349` and
`16` for the two negative controls that pinned the wrong keyings.

**Rung: TWO EXPERTS on the identity of the two outputs and on the keying of the second. ONE EXPERT on
the count**, by `16`'s own declared downgrade at its section 11.

I have checked this rung against `21` rather than taking the map's word, since `21` §2.2 downgraded an
adjacent claim. `21` disputes the *blindness of the certifying check* at two experts and does not
dispute the identity or the keying. So this row's two-expert rating survives `21`'s audit, and the
adjacent row that would have joined it does not.

**Permanence: passes.** **Equivalence: passes**, and here the equivalence test is doing real work
rather than rubber-stamping. `16:639-642` demonstrates that a one-output derivation whose storage is
23.1 percent over passes the design's own certification four of four green. So two teams, one with this
sentence and one without, produce measurably different artifacts that the existing test would not
distinguish. That is the equivalence test detecting a genuine missing intent.

#### S19. `Cold` is a statement about composition, not about a value

> The packed strategy is not a container choice with a field attached. It is a statement about how a
> run of values composes, which is why a single value under it has no distinct standalone form. The
> derivation's two answers are therefore answers to two different questions rather than two coordinates
> of one answer.

**Evidence.** `16:690-694`. Reached from both ends: `15:317-319` reports that a lone numeral has the
same carrier whatever strategy was asked for, and `16`'s section 2 reached the same conclusion from the
other direction, that a lone packed value has to have a size so packing cannot be a statement about the
standalone type.

**Rung: TWO EXPERTS.** The third and last row to earn it. Both experts reached the carrier-identity
fact independently, `16` before reading `15`, and `16` says so specifically at its section 10.2.

**Permanence: passes.** **Equivalence: passes.**

**This is the row that makes S18 make sense**, and I would not carry S18 without it. `16:692-694` puts
it precisely: a canon that says "the derivation produces a container and a stride" invites the reading
that these are two coordinates of one answer; one that says the derivation answers a per-value question
and a per-aggregate question carries the reason. The second is a canon sentence. The first is a field
list.

#### S20. The load type is not the carrier

> The type used to read a packed value is a function of the declared width and is not the carrier. An
> implementer reaching for the carrier because it is the nearest thing to hand reads too few bits at
> many widths, and the error is data-dependent, so a read that is wrong is correct whenever the bits it
> missed happened to be zero.

**Evidence.** `16:384`, `16:515` and `16:675` for the carrier being wrong at 28 of 64 widths, with the
closed form checked against an exhaustive scan of all eight phases for every width 1 to 1024, zero
mismatches. `16`'s `p3` for the data-dependence: zero of sixty-four wrong with small values,
thirty-two of sixty-four with data filling the width.

**Rung: ONE EXPERT.**

**Permanence: passes.** **Equivalence: passes.**

**Why it belongs in a canon at all, since it reads like an implementation note.** Because of the
data-dependence. A relation whose violation is invisible under the obvious test data is exactly the
kind of thing that must be stated where the intent lives, since no downstream test will find it. `16`
makes this argument itself at `16:679-681`, calling it "the sort of thing that belongs in a canon
sentence as a relation rather than being rediscovered", and I think that is right for this specific
reason and would not generalise it.

#### S21. BLOCKED. What the ladder's inputs are

There is a live disagreement in the record that a canon cannot straddle.

`10:195-197` says: "The ladder does not know what a strategy is; it maps a width to a container. Where
the strategy puts the crossover is an input to it." On that basis `10` concludes a canon sentence about
the derivation can be written without waiting for the headroom question.

`15` built a three-input map keyed on strategy, width and sign, and `MORNING.md:130-132` records that
op's own words require the strategy to guide the selection rather than the consumer, noting that the
map document did not contain the word "strategy" anywhere in 468 lines until that correction.

These are not the same design. Under `10`'s, the strategy is upstream of the ladder and the ladder is
strategy-blind. Under `15`'s, the strategy is a key of the ladder. Both compile. The observable
difference is whether the crossover is a property of the ladder or a property of the caller.

**Named blocker: is the strategy an input to the derivation or a selector upstream of it?** I am not
resolving it, and I note that neither file addresses the other. `10` predates `15` and `15` does not
cite `10` on this point.

#### S22. BLOCKED. Whether the two outputs are forced by arithmetic

`16:695-704` establishes something neither file's headline carries. If the `Precise` strategy does not
widen, then over widths 1 to 128 there are 251 distinct extents and **zero** map to more than one
carrier, so the second output is forced only by the type system. If `Precise` does widen, **64 of 251**
extents map to two carriers and the pair is irreducible as a matter of arithmetic.

So `Precise`'s semantics decides whether S18 is a mathematical necessity or a typing convenience.

**Named blocker: what `Precise` means.** `SETTLED.md:180` lists `Precise` on `inexact` as open since
the closed panel's `145`. `15:669-672` reports it did not build `Precise` as anything but the default
strategy with a different name. This is now blocking two rows.

### Cluster E: erasure and what is certified

#### S23. The erasure guarantee decomposes into three parts with three different standards of proof

> Layout erasure, that a typed value occupies what its container occupies, holds by construction from
> the transparent representation, at every optimisation level and on every toolchain. Dispatch erasure,
> that no runtime branch selects on the strategy or the width, holds by construction from
> monomorphisation, conditional on the design's prohibition of dynamic dispatch and runtime type
> identity. Operation erasure, that a derived operation lowers to the described operation, is the only
> part that requires inspecting emitted code.

**Evidence.** `17:221-245`.

**Rung: ONE EXPERT.**

**Permanence: passes.** **Equivalence: passes.**

**And the conditional is load-bearing and keeps getting dropped.** `17:236-237` marks dispatch erasure
"proved by construction, **conditional on the ban list holding**", and both `19` §5.7 and `21` §4.1
found the map carrying the split without the condition. `21:358-360` states the consequence exactly: "A
proof by construction with an unstated side condition reads as unconditional, and that is the
strongest-sounding sentence in the map's section five." **A canon carrying S23 without its condition
would be claiming a theorem it has not got.** I have written the condition into the sentence above
rather than as a footnote, deliberately.

#### S24. The prohibition list is part of the trusted base

> The prohibitions on dynamic dispatch, runtime type identity and unrestricted specialisation are not
> hygiene. They are what makes the dispatch-erasure argument a proof, and what makes a property checked
> at a small model width transfer to the widths the design actually serves. They belong on the written
> list of things the design's correctness rests on.

**Evidence.** `17`'s section 6, a nine-item trusted-base list, of which `17:95-96` says "it is why the
deliverable below is a list rather than a verdict". The transfer argument is independently stated in
the workspace's own `unstable-features.md`, which has a whole section on it.

**Rung: ONE EXPERT within this panel**, and I note that the transfer argument is carried in the
workspace rule from a different panel entirely, which is a second instance arrived at differently
without being a second expert on this dispatch.

**Permanence: passes**, with a caveat about its register. The specific feature names are toolchain
facts and would fail permanence; the *shape*, that the prohibition list is part of the correctness
argument rather than of the style guide, does not.

**Equivalence: passes.**

**This is the row a canon most needs and is least likely to write**, because it looks like process. A
canon that omits it will be re-litigating a feature ban in two years as a matter of taste, with the
argument that made it load-bearing sitting in an audit trail nobody reads. `21:340-350` found the map
had dropped `17`'s entire deliverable, grepping zero hits for the vocabulary. It has been dropped once
already, in this panel, within hours.

#### S25. A packed run's allocation is not the naive product

> The bytes a packed run of values occupies is not the count times the width divided by eight. It is
> that plus the slack the access pattern requires, or the final element is a special case.

**Evidence.** `17:377-379`.

**Rung: ONE EXPERT.**

**Permanence: passes.** **Equivalence: passes.**

**This is a real canon sentence in a small way and I nearly excluded it as an implementation note.**
What earns its place is that it is a *constraint on a formula a reader would otherwise write down
confidently*, in the same category as S8. It is cheap to state and expensive to omit.

### Cluster F: what the strategy axis carries

Both rows here come from the two files written after the checkpoint, and both are measurements rather
than derivations.

#### S26. BLOCKED. What the wide-rung trade is

`22` put the ratified wide-rung rule on the harness and **both halves of its evidence failed**. The
three-instructions-per-operation figure is zero at five of six widths swept including the width where
it was originally counted, follows an exact closed form in the tail residue where it is non-zero, and
is per element rather than per operation. The seven-bytes-per-value figure is correct as arithmetic and
is not a throughput quantity: word-rounded is 0 to 7.5 percent faster at every width while touching up
to 41 percent more bytes.

The sentence `22` supports is at `22:576-579`:

> The strategy axis is not carrying the trade the rule says it carries. It is carrying a footprint
> difference that is real for a consumer counting bytes and invisible to a consumer counting time.

**Evidence.** `mock/benches/variants/wide-rung-shared/`, artifacts at `mock/benches/wide-rung-*`, five
sections, 28 size rows, 11,200 CSV rows, commits `441c0b3` through `1d139a5`. Committed harness output,
which under `RULES.md:119-122` is the only thing in this workspace that can price anything.

**Rung: ONE EXPERT.** A single harness run by a single author.

**Why it is blocked.** `22:614-618` says it plainly and against its own interest: every arm wraps, and
if the wide rung is supposed to clamp then this bench measures the wrong semantics. `20` §5 found two
committed bench families implementing the two readings of the default strategy and **disagreeing in
direction**. So the measurement is sound and its applicability is unknown.

**Named blocker: does the design's default strategy wrap, or clamp?** This is `MORNING.md`'s question
five and `22:632-635` hands op the same sentence `20` did.

**One thing I want to say about this row that is not about its content.** `22` is the only file tonight
that measured rather than argued, and it refuted its own section zero in writing rather than editing it
away. `RULES.md:50-60` says a contested magnitude is not a question for op, it is answered by someone
being confident enough to build the arm. Somebody built the arm. Whatever the canon ends up saying
about the wide rung, this is the shape of evidence it should rest on.

#### S27. What a headroom rule costs, stated as a mechanism

`20:117-119`, and I am quoting it rather than rewriting it because the wording is already right:

> The headroom rule guarantees the container exceeds the width at every width, and therefore guarantees
> the projection is a real instruction at every width. Its cost is not that a wider container is
> slower. Its cost is that it removes the case where the projection would have been free.

**Evidence.** `20:99-104`, six widths, the ratio splitting exactly along filled versus sub-rung widths,
44.2x / 0.98x / 21.0x / 7.0x / 0.99x / 2.45x, and the same shape at a second element count.

**Rung: ONE EXPERT.** `20` derived this in a section outside its declared contamination, which it
confined to its section 1.5.

**Permanence: passes.** It is a statement about what a rule does to a projection, and would survive any
rewrite.

**Equivalence: passes.**

**And its neighbour is larger than it is**, which a canon should carry with it. `20:126-133` finds that
writing the projection once rather than after every operation is worth 21.6x at one width with an
identical container, and is independent of whether the width fills its rung. So at the sub-rung widths
the container is not the cost at all. That is a statement about where a design should put its attention
and it is worth more than the headroom question it was found while answering.

### Two rows I am listing to refuse them

Included because `RULES.md:99-101` makes keeping something a result, and refusing something is the same
act in the other direction.

#### S28. REFUSED as canon. The adjunction frame

`07` built a frame that explained two other files' results and predicted things the panel did not have.
Its own recommendation, at its section 5.2, is to keep the frame in the audit trail and out of the
canon, on the ground that its content restates without it.

**I agree, and for a reason `07` gives against itself.** `07` flags the risk against the record's own
precedent: the finest-view mechanism, whose literature relation was compile-refuted and never repaired,
and which `SETTLED.md:117` still carries as "RATIFIED, minus one part". A canon that imports a
mathematical identification imports the obligation to keep it true. The results survive the frame's
removal; the obligation does not survive the frame's inclusion.

`07` is one of the two or three best files in this panel and it is right to keep itself out of the
canon. That is a contribution, and a rarer one than a finding.

#### S29. REFUSED as canon. Every instrument finding tonight

The erasure oracle's two false-negative regimes, the certifying check's blindness to a one-output map,
the data-dependence of a too-narrow read, the defect matrix catching 174 of 420 against a naive suite's
41 percent, the adversarial-write requirement, the harness never calling its own validation pass.

All true, all valuable, none of it canon. `RULES.md:85-86` is precise about the division: the canon
says which things are doable and the evidence lives in the audit trail. An instrument is evidence
apparatus. A canon carrying it fails permanence at the next rebuild of the apparatus.

**One of these has a consequence outside the panel and should not be filed quietly.** `22`'s finding
that `mockspace-bench-core`'s orchestrator never calls its validation pass, verified in a clean clone,
means no committed bench in this workspace has ever checked that its arms compute the same thing. `22`
fixed it in arvo's own driver and deliberately did not touch the upstream crate. That is correct
handling and the finding still needs somewhere to go.

---

## Counting

Produced with commands, per `RULES.md:124-126`, run from the panel directory.

```
$ F=23_spj_the_sentences_a_canon_could_carry.md
$ grep -c '^#### S' $F                 -> 30
$ grep -c '^#### S.*BLOCKED' $F        ->  7
$ grep -c '^#### S.*REFUSED' $F        ->  2
$ grep -c '^\*\*Rung: TWO EXPERTS' $F  ->  5
$ grep -c '^\*\*Rung: ONE EXPERT' $F   -> 20
```

Thirty rows. Seven blocked, two refused, **twenty-one carryable** in some form. Five at two experts:
S13, S13b, S16 for its defect half, S18 for its identity and keying, S19. Twenty at one expert. Five
rows carry no rung, being the two refusals and three blocked rows with no derivation behind them.
**Zero ratified**, which follows from `04` and is not a defect.

**Two of these numbers are corrections and I am leaving the correction visible.** I wrote "twenty-nine
rows, six blocked, three at two experts" from my own reading before running the commands. The row and
block counts were simply wrong by one each. The rung count was wrong by two for a better reason: I had
rated S13 from the map's account of `13`'s dispatch order, and going to `13:1-30` to check it turned up
two further claims that `13` derived with `12` unread and that I had filed at one expert. So the
correction went in the direction of the panel being in better shape than I had it, which is the less
usual direction and worth saying.

The general lesson is the one this panel keeps paying for. **A rung read off a summary is not a rung.**
`21` §2 found four rung errors in `MORNING.md` and I made a fifth of the same kind inside the file
auditing rungs, in the same session, having read `21` in full first. The fix each time is thirty
seconds of opening the source.

One number deserves comment rather than a row. **Twenty-one carryable sentences from twenty-two files
is not a bad night.** The instinct is to read it as thin. Set against the predecessor panel, which ran
to 320 files and produced `SETTLED.md`'s 63 survivors, the rate here is roughly three times better per
file. I have not verified that comparison beyond dividing the two counts, and I flag it as arithmetic
rather than as a claim about quality.

## Do they cohere?

**No, and the shape of the failure is more useful than the answer.**

They form **six groups, of which three cohere into two joined clusters and three stand apart**. Not a
pile, which would be worse, and not a spine, which is what a canon needs. The labels A through F above
are mine and are how I grouped the rows to write them down; what follows is what the grouping turned
out to mean, which is not the same thing and is the finding.

**Cluster A holds together on its own and is the best-formed thing here.** S1 defines the concept, S2
says what being outside it means, S3 prices the one axis where the concept has a real choice, S4 says
what the sign domain does. Four sentences, mutually supporting, and S1 is the root: S2 depends on it
entirely, S3 is about one of its clauses, S4 refines what its "value set" is determined by. If op
asked which part of tonight is closest to canon, this is the answer.

**Cluster C is the tightest-argued group and the one least like canon.** S12, S13 and S13b are three
compiled results that fit together into one story: the table cannot be closed, the ceiling is not where
anyone thought, and the surface and the machinery were never actually coupled. S14 is the sentence that
story wants to end in. S15 is the rationale beside it and S16 is the defect it leaves standing. That is
a coherent group, and every sentence in it is about the boundary between a written program and a type
system rather than about numerals. A canon needs it and it is not the canon's subject.

**Cluster B mostly holds and has one internal collision.** S6 and S7 fit together well: the order is
infrastructure, and no site needs its extrema. S8 and S10 are consequences of taking the concept
seriously. S9 and S17 are in different coordinate systems for the same claim, which is the collision,
and S11 offers to restructure the whole cluster in a way that would change what all of it quantifies
over.

**The joint between A and B is genuine and load-bearing.** S10 uses S1's concept to dissolve `03`'s
antichain, and S3 uses S6's order as the currency it prices denotation choices in. That is two clusters
depending on each other in both directions, which is what a spine looks like.

**Clusters C, D and E do not join A and B at all**, and this is the finding.

Grepped: I looked for any file connecting the width surface to the format concept, and the derivation
to the order.

```
$ grep -l 'binade' 11_*.md 12_*.md 13_*.md 15_*.md 16_*.md 17_*.md
   -> no matches, exit 1
$ grep -l 'exponent form\|canonical exponent' 11_*.md 12_*.md 13_*.md 15_*.md 16_*.md
   -> no matches, exit 1
```

**Both return nothing.** The concept vocabulary appears in no surface or derivation file. I state the
result rather than only the command, because a bare command with no output beside it is the shape `21`
section 3 found `MORNING.md` using, and it is not evidence of anything on its own.

Cluster C is about how a
width crosses from a value to a type. Cluster D is about what a width maps to. Cluster A is about what
a numeral *is*, and it is about binades, phases and progressions. **These are two different subjects
that both use the word "width", and no sentence in the inventory connects them.**

The consequence is concrete. S1 says a numeral is determined by a radix, an exponent form, a phase and
a value set. S17 says a numeral is keyed on a total width and a fraction width. **Those are not the
same numeral**, or rather, they are the same object described by two vocabularies that have never been
put in the same sentence. A canon carrying both would leave a reader unable to say what the exponent
form does to the keying, or whether the two-output derivation applies to a float.

That is the missing spine, and it is one sentence's worth of missing. It is not a large gap. It is a
gap in a place where nobody has looked because the panel's dispatches split along it: the first stretch
went at the concept and the order, the second and third at the surface and the derivation, and no
dispatch has been pointed at the seam.

**Cluster E stands slightly apart and correctly so.** S23, S24 and S25 are about what is proved rather
than about what is true. They join the rest at exactly one point, S24's transfer argument, which is
what licenses believing anything checked at a model width. That is a real joint and a thin one.

**Cluster F is the least integrated and is the newest.** S26 and S27 are measurements about a strategy
axis whose semantics is one of the design's open questions. They cannot join anything until that
question is answered, which is why both are blocked or half-blocked.

**One tension I want to state plainly rather than leave implied.** S15 says the table is the currency
Rust's position charges, and reads as absolution. S12 says the table cannot be closed, and reads as a
refutation. `11` holds both, at `11:999-1040`, and `MORNING.md:598-600` states the distinction well: a
price is paid once, and this one is charged unboundedly against a ratified property. A canon must not
carry S15 without S12 beside it, because S15 alone would read as a decision to accept the table.

## What is missing

This list is more useful than the inventory, and I have tried to make it specific enough to dispatch
from rather than gesturing at areas.

**The seam sentence.** How the concept vocabulary and the width vocabulary name the same object. This
is the single missing sentence that would join the inventory's two halves, and no file has been pointed
at it. It is the item I would put first.

**What a strategy is.** Measured over the rows: **eight rows mention the strategy axis and three name
it inside their canon sentence, and not one of the thirty says what it is.** S19 says what one strategy
is a statement about. S22 says the design's `Precise` semantics decides whether S18 is forced by
arithmetic or only by the type system. S26 and S27 measure what strategy choices cost. The counting
script for this is in `23_probes/`.
`SETTLED.md:82-87` carries six strategy rows all marked RATIFIED under the superseded reading. **A
canon for arvo's numerals with no sentence defining the strategy axis is missing its most consumer-
visible concept**, and the workspace's own `arvo-toolbox-not-policer.md` says the axis is how the
substrate hands the consumer a choice it cannot make for them.

**What the consumer writes.** Op's own question at `MORNING.md:20`. It blocks S9 and half of S17 and
`15` argues it is prior to the family question.

**A sentence about conversion.** `SETTLED.md:118-120` carries the inclusion conditions at two experts
and the adjudicating-strategy gap as open and disputed. Nothing in tonight's inventory touches
conversion between numerals. That is a large hole in a numeral canon and the panel simply did not go
there.

**A sentence about what an operation is.** `22:604-612` had to decide what an operation is at the wide
rung in order to bench it, and lists that decision under "what I had to decide that the design does not
specify". `MORNING.md:23-24` asks whether the design has a mixed-numeral addition at all, and reports
`06` finding no operation anywhere in the record that adds values from two different numerals. **If the
answer is no, a large part of Cluster B has no caller.** One sentence from op collapses it.

**The necessary condition for an absorbing endpoint.** Blocks S5, blocks `Precise` on `inexact`.

**Whether precision counts the sign digit.** Blocks the precise form of S4.

**What the wide rung's semantics is.** Blocks S26 and reweighs S27.

**Almost nothing about the consumer's experience.** Two rows touch it and neither is what is wanted.
S13b says the written form and the parameter form are separable, which is a statement about the
machinery's freedom rather than about the consumer. S16 is about diagnostics and is blocked. Nothing
says what using the design is supposed to feel like, and the ratified ergonomics bar is the thing
`SETTLED.md:109` warns has already been compressed wrongly once and cost an expert an hour. A canon
that says what a numeral is, how it derives and what erases, and says nothing about what a consumer
writes and reads, has described the machine and not the tool.

**And I want to be careful not to overstate this one**, because `12` and `13` between them did a great
deal of work on exactly the consumer's reading experience, and it produced S16 and a pile of refuted
repairs rather than a sentence. That is the honest position: the work was done, it did not converge,
and the gap is a gap rather than an oversight.

## Is the checkpoint's verdict right?

**It was right when it was written and it has been half-answered since. What survives is narrower,
sharper, and still the most important thing in this file.**

The verdict at `19:716-717` has two parts, and they have fared differently.

**Part one, that the panel converged on methodology for the mechanism rather than on a canon.** This
was accurate for files `02` through `18`. `19:686-688` counts it itself: of the four files in its own
stretch, one is almost wholly methodology and one is design plus one methodological finding. My
inventory bears it out from the other side. Of thirty rows, the ones I refused wholesale (S29) are all
instrument findings, and they represent a large share of the panel's best-argued pages.

**Part two, the reason: that the doability a canon must assert is, for arvo, a workload claim, and the
panel had not made one.** `19:707-714` grounds this in `arvo-toolbox-not-policer.md`, which says
packed storage "is the reason arvo exists" and that the intersection creating the need for arvo is a
set of workload properties.

**Part two has been substantially answered, and the checkpoint predicted how.** `19:722-725` named the
remedy: not a nineteenth expert, but the second set of eyes op asked for plus one bench entry. `20` was
the second set of eyes. `22` built a bench with five real arms, one shared transform, a control
verified by disassembly, 11,200 committed CSV rows, and a validated arm set. Those are workload claims
on the harness, which is the only thing `RULES.md:119-122` allows to price anything.

**So the verdict's stated reason no longer holds in the form it was given.** The panel has made
workload claims. Two of them, on committed harness output.

**And here is why the verdict survives anyway.** The workload claims that landed are about the *wrong
thing* for the canon's purposes, and both files say so themselves.

`22:580-583` reports the strategy axis is carrying a footprint difference invisible to a consumer
counting time, and then immediately says: "`arvo-toolbox-not-policer.md` is explicit that footprint at
the million-element scale is the reason `Cold` exists, and nothing here contradicts that: it says the
bytes do not buy speed, not that the bytes do not matter." That is precise and it is a concession. The
workload claim the canon needs is that packing pays at the scale arvo's consumers run at. `22` measured
one host, one target, one workload family it chose, and 15 MB at the top end.

`19` §1.2 found the repository already holds a packed-storage measurement, and `20` audited it into a
narrower shape: packed is smaller and several times slower, the gap narrowing with scale from 4.10x to
3.34x without closing. That is a workload claim and it points *against* the thing the canon would have
to assert.

So the verdict I would give, having done the inventory, is this. **The panel is no longer converging on
methodology; the last three files broke that pattern decisively. It is still not converging on a canon,
and the reason has changed. It is no longer that no workload claim has been made. It is that the
workload claims now in hand do not support the sentence the canon needs, and nobody has said so out
loud in one place.**

That is a better position than `19` described, because a claim pointing the wrong way is actionable in
a way that an absent claim is not. It is also more uncomfortable, and I would rather state it than
round it off. Two committed measurements say the packed form costs time. The design's own justification
says the packed form is why the design exists. Those can both be true, and the sentence that makes them
both true, something in the shape of *what the axis buys is footprint at a scale where footprint is the
binding constraint*, is not measured anywhere, and it is a canon sentence with a doability obligation
attached.

**One qualification on my own verdict.** `19` had eighteen files and I have twenty-two. Its judgement
was made without `20`, `22` or `21` existing. I am not correcting it; I am reporting that the thing it
diagnosed was acted on within four files, which is the system working, and that acting on it surfaced a
harder problem underneath. That is the ordinary shape of this work and it should not read as a criticism
of the checkpoint.

## Coverage, stated as a bound

**Read in full:** `RULES.md`, `01`, `04`, `SETTLED.md`, `MORNING.md` (1023 lines), `21` (541 lines),
and `19`'s sections 6 through 12.

**Read substantially, at the passages bearing on candidate sentences:** `08` (sections on the boundary
and the meet closure), `11` (sections 4, 5, 6, 7 and the outline), `16` (sections 10 through 13), `17`
(section 2 and the C4 decomposition), `18` (section 3.2 and 3.3), `22` (sections 0, 1, 9, 10, 11), `20`
(section 1.2), `06` (sections 5.3 and 10), `07` (section 3.1), `02` (section 1.4), `12` (section 7),
`03` (section 7.4), `10` (sections 3.4 and 4), `13` (the arrangement D passage), `15` (the coordinate
finding).

**Read only by targeted grep:** `05`, `09`, `14`, and the rest of `03`, `06`, `07`, `08`, `10`, `12`,
`13`, `15`, `17`, `18`, `20`.

**Never opened:** `CANON_CANDIDATE.md`, `DROPLIST.md`, `PERSONA_CALLS.md`, `seed/` in its entirety, and
every probe directory except my own. **This is the largest gap in this file and I want it stated rather than buried.**
Every rung I assigned rests on what the citing file says about its own evidence, cross-checked against
`21`'s independent verification table where `21` covered the claim, which it did for most numeric ones.
I opened no probe myself. So this inventory is a reading of the panel's self-reports, disciplined by one
other reader's audit, and it is not a re-verification.

**I ran a citation check on my own file, and it found three defects.** `23_probes/` carries the two
scripts and `23_probes/RUN.out` carries their output. The first extracts every `file:line` citation in
this file and confirms the range exists; it found **one out of range**, `08:985-987` against a file of
744 lines, which was my transcription of a passage I had located by grep and never opened. Opening it
turned up a better citation and a quotation worth carrying, which is now in S1. The second opens
fifteen citations I had taken from another file's verification table rather than reading, and checks
the content is there; **three were off by one to three lines**, in `16`, `11` and `RULES.md`. All three
are corrected above.

**And running the script now reports one out-of-range citation, which is the dead one quoted two
paragraphs above.** The instrument cannot tell a citation from a narration of a bad citation, so the
honest reading of its output is 118 live citations all resolving, plus one corpse on display. I am
leaving both the corpse and this note, because deleting the evidence to get a clean run is the exact
move `RULES.md:99-101` and this workspace's test gate exist to stop, and because an instrument with a
known false positive is more useful than a quiet one.

So: 118 live citations, all resolving after repair, 15 content-checked, 3 of those 15 initially
wrong. That is a defect rate of one in five on the citations I did not read myself, and it is the
argument for reading them. **None was fabricated**, which is the failure mode `RULES.md:108-114` exists
for, and the difference between an off-by-two line number and a probe that does not exist is the whole
difference between a repair and a void claim.

**What I checked with a command rather than reading:** the panel's file listing and expert-file count;
that `17_probes` carries 21 tracked files against `17:252`'s "Nothing is committed", confirming `19`'s
finding still stands; that `22_probes` carries 16 and `21_probes` 3; the concept-vocabulary greps in the
coherence section; the file timestamps establishing `CANON_CANDIDATE.md` predates this panel.

**What I did not do:**

I did not check whether any candidate sentence contradicts `seed/`. Three of this night's four
compression defects were found by someone reading the establishing text in `seed/` against a summary, so
**a sentence in my inventory could be inheriting a compression defect and I would not know.** That is the
specific risk of this file and I have no way to bound it from where I stand.

I did not re-derive any sentence. This is an inventory, and every row is somebody else's derivation,
which means every row inherits the rung of its source and none is improved by appearing here.

I did not resolve the S9-against-S17 coordinate collision, the S21 ladder-input disagreement, or the
S12-against-S15 tension. All three are recorded and left standing.

I did not attempt the seam sentence I say is missing. I considered it, and it is a derivation rather
than an inventory, so it belongs to a dispatch pointed at it rather than to this one. I would say the
same of anyone who wrote it here: it would be a design contribution smuggled into a stocktaking file,
and it would arrive at one expert with no probe behind it.

**Where I ran out of confidence, in two places.**

First, the rungs on the two rows I have not been able to source to an explicit independence
declaration: S18's identity-and-keying half and S19. Both rest on `16`'s account of its own working
order at its sections 10 and 12. I read those sections and they are specific and credible, and I did
not do for `16` what I did for `13`, which is check the claim against the file's stated ordering line
by line. `16` also declared itself contaminated on an adjacent claim, which cuts both ways: it is
evidence the file is scrupulous, and it is evidence there was something to be contaminated by.
**Treat S18's and S19's rungs as sourced to the file's self-report rather than verified.**

Second, my comparison of this panel's sentence-per-file rate against the predecessor's. Two counts
divided, nothing more, and I said so where it appears.
