# 80. When the deriving happens, and what is left at each stage

**Author lens:** Rompf. Staging and partial evaluation. For any computation the first question is which
part is known now and which part is deferred into the program that gets emitted, and most confusions about
a pipeline turn out to be a misplaced answer to that question.

**Position:** second attacker in the derived-algebraic-laws unit, after the two cold derivations (`76`,
`77`) and after `79`, which I read once it landed. My assigned question is what "derived" means for a law
and **when** each part of it happens.

**Probes:** six, committed at `80_probes/` with their sources, transcripts and emitted assembly, before
this file. Two of them are committed twice, once broken and once repaired, because both first runs
measured something other than what they claimed and the way they were wrong is part of what they
established. The last two exist because the first four produced a consequence I did not believe on
re-reading, and both of them attack it: one asks whether the wall can simply be switched off, the other
asks whether the consequence holds for the laws it was drawn about.

## 0. Gates, and coverage

**Canon gate: passes, situation two.** `mock/canon/` does not exist, `mock/crates/` is empty by the
declared mutation order, and this panel is writing the first canon. `INTENTS.md` holds exactly one
RATIFIED entry, I13 (`INTENTS.md:177-198`), ratified narrowly on op's own instruction that it means no
more than he said. The other eleven are STATED, under the standing instruction that nothing about them is
absolute (`INTENTS.md:40-41`). Nothing below settles anything.

**Test gate: no suite exists.** The mock workspace has no members. The substitute is the probe discipline
and I applied it to my own instruments rather than only to other people's: two of my four probes failed
their first run in ways that would have produced confident wrong headlines, and both failures are on disk
with a note naming the defect (`80_probes/NOTE_p1a_first_run.md`,
`80_probes/NOTE_p2_first_run.md`). One of those failures is the exact "setup that helps" shape the
workspace test gate names: the instrument fed the implementation inputs on which the expensive path is
never entered, so the expensive path was never measured, and every number it printed looked reasonable.

**Read end to end:** `INTENTS.md`, `RULES.md`, `OPTIONS.md`, `DROPLIST.md`, `76`, `77`, `79`,
`74_giesen_consolidation_the_number_system_concept.md`. **Read at the source, in the specific ranges I
cite:** `68_leroy_what_the_pipeline_certifies.md` sections 2.1 through 2.4 (`68:92-220`),
`28_op_answers_two.md` sections through batch one Q3 (`28:1-120`), `seed/OLD_SETTLED_container.md:20-55`,
`79_probes/p1_compositional_predicate_search.rs` and `79_probes/p1_output.txt`.

**Not read:** every file numbered `01` through `27`, `29` through `67`, `69` through `73`, `75`, and every
probe directory other than `79_probes/`. Everything I say about `35`, `42`, `43`, `55b`, `57b`, `60`,
`62`, `63` is routed through `OPTIONS.md`, `74` or `79`'s own account of it, named as such at each point,
and inherits their errors if any.

**Nothing here is priced.** No bench ran. Every instruction count below is read off emitted assembly and is
an ad-hoc quick spike with no substance for any how-much question, and is called that where it appears.

## 1. Breaking my own brief first, and one thing it got right for the wrong reason

### 1.1 The acceptance criterion is op's words, from a body op has demoted, re-entered as live vocabulary by his own answer in this panel

Both cold derivations flagged that they could not trace the sentence the dispatch handed them. `77:4-11`
says it plainly: "I did not find it, word for word, inside `INTENTS.md`'s twelve entries during this cold
pass... Whoever reconciles this file against the panel should confirm the wording against its actual
source." `76:5-9` used the sentence as given without flagging it. `79` does not mention the criterion
anywhere: grepping it for "acceptance criterion", "derive the matching", "then validate, and erase" and
"provenance" returns nothing. So `77`'s request has gone two files without an answer.

I traced it. It is verbatim op, and it is at `seed/OLD_SETTLED_container.md:33-36`, which quotes him from
the closed formalization panel at `135b:12-16`:

> There *is* a way to express usage through bits and bytes *and* have the typestate derive the matching
> container and numeral representations, then validate, and erase on lowering to be exactly what you
> describe before that caveat. Anything less than that, no caveats left, is unacceptable for this design
> and canon.

**Two things follow and they pull in opposite directions.** The `seed/` file classifies this as RATIFIED,
and `INTENTS.md:27-33` says exactly that classification "is not to be trusted" and that the rung must not
be imported again. `RULES.md:525-548` goes further about the whole body it belongs to: op's prior calls
"are not calls, not ratified intents, and not canon", "explicitly connected to a *failure*", and "none of
them relate to this new panel or its convergence or settled intents, and should not act as if it did."

**But op re-entered this specific sentence's vocabulary into this panel himself.** `28:67-95` records him
being asked which reading of "then validate" he meant and answering, which is an engagement with the
criterion's own words in this panel, in his own voice, dated to this panel's second day.

So the honest status: the derive/validate/erase framing is **op's own vocabulary, live in this panel by his
own use of it, and not a ratified requirement**. A canon sentence that treats "then validate, and erase, no
caveats left" as a governing acceptance test is citing a demoted body. A canon sentence that uses derive,
validate and erase as the names for three things a typestate does is on much safer ground, because he used
those names here. I use them in the second sense throughout and nowhere in the first.

### 1.2 My brief describes one decomposition of "validate" and there are two, orthogonal, and op has already answered on the other one

My dispatch says both cold derivations missed a fork between compile-time-per-type and runtime-per-datum
validation. That fork is real and it is `68`'s (`68:112-133`), carried as Q-A at `74:1029-1037`.

It is not op's fork. Op was asked about "then validate" in this panel and given three readings, which are
**admissibility** (the typestate refuses declarations it cannot serve), **usage** (it refuses operations
violating the declared invariants) and **self-validation** (the derived container actually holds the
declared range). He answered "Usage, Admissibility, Self-validation, All that makes sense", with the
challenge route open (`28:82-95`, carried at `OPTIONS.md:57-88`).

`68` itself states the relation between the two decompositions and it is the load-bearing sentence nobody
in this unit has cited: "The compile-time verb validates **the derivation** (op's Q1 enumeration, all three
of whose parts, admissibility, usage, self-validation, are compile-time acts). The runtime verb validates
**a datum at an ingest boundary**, which is not among Q1's three parts at all" (`68:126-129`).

So the shape is a three by two grid: three things validated, two binding times, and op has answered on one
axis and not the other. Of the six cells, **the panel's law-layer evidence occupies one**: usage, at
compile time. `76`'s and `77`'s probes both refuse a bad instantiation, which is usage. `79` says of its
own work that "Every predicate this file states is a `Q-A`, compile-time reading claim" (`79:322-325`).
Nothing in this unit has instrumented admissibility of a *law declaration*, self-validation of a law, or
any runtime cell.

I surface that and do not answer it, per the dispatch. What I add below in section 6 is that one of the two
axes is not merely a cost question: it decides whether an entire class of the panel's own measured law
regions can be an arm at all.

## 2. The pipeline read as a staging problem, which is what it is

Op's three verbs name three things that happen at three different times, and the whole difficulty in this
unit is that for a **container** all three happen at one time and for a **law** they do not.

For a container the pipeline is a single-stage partial evaluation and it closes cleanly. `container_of(W,
S)` is a function of the typestate, computed at monomorphisation. Validating it is a predicate on the
computed answer, O(1) per type, at the same moment. Erasing it leaves the container, which was the point:
you derived `u32` and what remains is `u32` arithmetic. **The output of the derivation is the thing the
program runs on.**

For a law none of that holds, and the reason is that a law is not a value computed from the typestate. It is
a **quantified proposition over the value domain**, and quantified propositions over a domain of size
`2^(W·k)` are not the kind of object that fits in the stage the container fits in.

So I read op's sentence as a staging schedule and find it has one stage missing at each end.

**Stage minus one, design time.** Where a law's truth is actually established. Exhaustive sweeps, closed-form
arguments, the group-theoretic collapse of wrapping, `57b`'s H1 and H2 as `79` reports them
(`79:112-121`). Its output is a **verdict plus a transfer proviso**, and it lives in the audit trail. This
stage is entirely outside the compiler and this unit has been writing as though it were inside it.

**Stage zero, monomorphisation.** Where the typestate decides **which** verdicts apply to this
instantiation, composes them, and refuses where a required one is absent. This is where `77`'s
resolve-lattice const assertions live (`77:129-140`), where `76`'s representability check lives
(`76:60-72`), and where I13's const predicates live. Nothing here re-establishes truth. It **routes** truth.

**Stage one, runtime.** Where, ideally, nothing law-shaped happens at all.

**And the verb op's sentence does not contain.** For a container, erase is the end because the container is
the output. For a law, erasing the derivation leaves nothing, unless the law **selected** a different
lowering on the way out. A law that is derived, validated and erased and then emits the same code as if it
had never been consulted has cost compile time and bought nothing.

So the pipeline for a law is **derive, validate, select, erase**, and select is the only stage at which a
law does any work. I13 is the statement of that missing verb: "we are not writing a generalization, rather
a bunch of arms with const predicates that optimize each little 'sometimes'" (`INTENTS.md:183-188`). An arm
is a selection. Neither cold derivation asked what the law was for; both built derive, validate and erase
for a law and reported the mechanism as working. `76`'s congruence-closure section comes closest, since a
law read as a rewrite rule is a selection waiting to happen, and it then follows `42` into refusing the
rewriting engine (`76:299-309`) and stops, correctly refusing the engine and leaving "then what does the
law do" unanswered.

## 3. The law layer has `68`'s hole, one coordinate up, and the repair only reaches model widths

### 3.1 A law stated as a marker is a declaration checked against nothing

`68:145-148` states the container-layer lesson: "a representation's declared properties are worth exactly
nothing unless validation runs through the maps; validation of declarations against declarations is paper
checking paper." It established that with a mutant whose overstated declared window compiled clean through
an entire validation suite (`68:136-151`).

**A law is a declaration.** `80_probes/p1a_declared_law_lies.rs` is the same mutant at the law layer.
Two overflow policies over a four-bit signed window, one marker contract `AssocAdd`, one consumer bounded on
it that reassociates a fold into a balanced tree. Both policies declare the marker. One of the two
declarations is false.

```
      Wrap  declares AssocAdd: yes   vectors: 65536   left-fold != tree-fold: 0
 SatSigned  declares AssocAdd: yes   vectors: 65536   left-fold != tree-fold: 16268
            witness [-8, -8, -8, 1]: left=-7 tree=-8

control, arity 2 (no grouping choice exists): Wrap 0, SatSigned 0
```

The compiler raised nothing. The licensed consumer returns a different answer on 16,268 of 65,536 vectors,
24.8%, with no signal at the failure site. The arity-2 control is zero for both policies, so the instrument
is measuring grouping and not something else.

The first run of this probe is on disk and was wrong: the wrapping map added offset representations rather
than values, and the fold was seeded with a literal identity, so `Wrap` reported 65,536 of 65,536 and the
headline looked like a strong result. `80_probes/NOTE_p1a_first_run.md` names both defects.

### 3.2 The repair works, and it works at four bits

`80_probes/p1b_computed_law_refuses.rs` replaces the author-written marker with a blanket impl whose
associated const runs the law over the policy's own map. The permission stops being writable. The honest
instantiation compiles and runs; the instantiation `p1a` was able to write is `E0080`, with the message
naming the reason:

```
error[E0080]: evaluation panicked: this policy's addition is not associative over the model
window, so a reassociating consumer may not be instantiated at it
   evaluation of `<SatSigned as AssocProven>::PROOF` failed here
```

This is the shape both cold derivations built for representability, applied to a law, and it is what `76`
and `77` reported as the mechanism working. It does work. Section 4 is about how far.

## 4. Validate-by-sweep is bounded, the bound collapses with arity, and the bound falls on the wrong side

### 4.1 The frontier is a curve in (width, arity), not a width

`68:196-211` established the const-eval wall for one law at one arity: signed saturating associativity,
arity 3, refused at width 9 under `deny(long_running_const_eval)`. The workspace droplist separately
records that the wall is a total-step-count budget rather than a width ceiling
(`DROPLIST.md:234-235`). Neither states the consequence.

`80_probes/p2_frontier.py` emits, for each arity K, a top-level `const` that enumerates every K-tuple over
the signed window of width W and **counts** violations rather than returning early, so the domain is
visited whatever the verdict turns out to be. It walks W upward per K until rustc stops accepting.
Full transcript at `80_probes/p2_frontier_output.txt`, machine-readable at `80_probes/p2_frontier.json`.

| arity | widest width rustc will evaluate | tuples there | first refused width |
|---|---|---|---|
| 1 | 19 | 2^19 | 20 |
| 2 | 9 | 2^18 | 10 |
| 3 | 5 | 2^15 | 6 |
| 4 | 4 | 2^16 | 5 |
| 5 | 3 | 2^15 | 4 |
| 6 | 2 | 2^12 | 3 |
| 8 | 1 | 2^8 | 2 |
| 12 | 1 | 2^12 | 2 |

Every refusal in the table is `refuse-long-running`, not a different error. The first run of this sweep is
on disk and was wrong in two ways, both instructive: the check returned early on the first counterexample,
so at every arity where the law is false the domain was never enumerated and the probe reported
`arity=3 width=16 tuples=2^48 accept 1.00s`; and the tuple count rode in a `u64` linear index, so four of
its six frontier points were parse errors reported as const-eval walls
(`80_probes/NOTE_p2_first_run.md`).

**A chain law of length n is an arity-n law.** I7 is op's statement that the accuracy-first strategy is
"accurate ... especially within chains and ops, not only alone" (`INTENTS.md:119-121`). At chain length 8,
the widest width whose chain law can be exhaustively validated by the compiler is **one bit**. arvo's
widths are not one bit.

**That consequence, stated flat, is too broad, and section 4.5 is my own correction to it.** It holds for
one kind of chain statement and is false for another, and separating them is more useful than either the
claim or its retraction.

### 4.2 The asymmetry runs the wrong way for a design that wants to license arms

`80_probes/p2b_swept_verdict_at_shipped_width.rs` is `p1b`'s construction with the width changed to 8,
compiled twice.

```
=== sat arm (law false) ===
error[E0080]: evaluation panicked: not associative at this width
rustc ... 0.03s user 0.08s system  0.504 total

=== wrap arm (law true) ===
error: constant evaluation is taking a long time
   = note: `#[deny(long_running_const_eval)]` on by default
rustc ... 4.45s user 0.02s system  4.479 total
```

**A negative verdict is cheap, because the evaluator hits a counterexample and stops. A positive verdict at
the same width and arity does not compile at all**, because there is no counterexample to stop at and the
whole domain has to be visited.

The verdict that licenses an arm is the positive one. So the mechanism this unit reported as working
produces, at a shipped width, exactly the verdict that cannot license anything, and refuses to produce the
one that can.

This also relocates a claim of my own predecessor rather than contradicting it. `76:74-83` chose a narrow
domain for its const-evaluated law probe, "16 values, 4,096 triples, small enough for const-eval to finish
without hitting the long-running-const-eval wall". That is the frontier, observed, and reported as an
implementation nuisance to be routed around. It is the boundary of the mechanism.

### 4.3 The escape is a closed form, and a closed form is a declaration again, so it gets checked on a model band

Nothing scales except a verdict computable in constant time from the typestate. Wrapping addition realises
the cyclic group of its width and every group is associative, so its verdict is `true` at every width with
nothing enumerated. `OPTIONS.md:1144-1147` already carries this as a drafting note: wrapping's four
separately measured properties are one theorem rather than four facts.

But a closed form is exactly what section 3.1 shows is worth nothing on its own. `80_probes/
p2c_closed_form_checked_on_a_model.rs` builds the thing that is neither: the closed-form verdict is what an
arm gates on, and it is **cross-checked against the swept verdict at every width the sweep can reach**, at
compile time, with the agreement itself an assertion.

```
p2c: closed-form verdict, cross-checked against the sweep on a model band
  model band: widths 2..=5
  agreement over the band, evaluated at compile time: true
  closed_verdict(wrap, w = 64) = true   (constant time, no enumeration)
  closed_verdict(sat,  w = 64) = false
  reassociating_consumer::<Policy<0>>() = licensed
```

Perturbing one entry of the closed form is refused:

```
error[E0080]: evaluation panicked: the closed-form law verdict disagrees with the swept verdict
somewhere in the model band, so the closed form is wrong and no arm may be gated on it
```

What is unchecked afterwards is then exactly one named thing, the transfer of the agreement from widths 2
through 5 to width 64, rather than the whole verdict. That is `68`'s transfer proviso
(`68:213-219`) arriving at the law layer with a mechanism attached rather than as prose.

Two costs, stated. The cross-check compiles in 4.04 seconds on this host, which is close enough to the
guard's own threshold that a slower host may see it refuse; the model band is a budget and it is nearly
spent at four widths and one law. And a design carrying many laws pays that budget per law.

### 4.4 The guard can be switched off, and switching it off buys three bits

`68:206-208` observes that `long_running_const_eval` is allowable, so the wall is a default refusal rather
than an absolute one, and calls the cost of allowing it "unpriced and rapidly growing". That is the obvious
attack on section 4.1 and it deserved measuring rather than repeating.

`80_probes/p5_allow_the_guard.py` runs p2's arity-3 check with `#![allow(long_running_const_eval)]` at
increasing widths, on the true-verdict policy so nothing exits early. Transcript at
`80_probes/p5_allow_the_guard_output.txt`.

| policy | width | triples | outcome | wall clock |
|---|---|---|---|---|
| wrap (verdict true) | 6 | 2^18 | accept | 5.85s |
| wrap | 7 | 2^21 | accept | 49.06s |
| wrap | 8 | 2^24 | accept | 370.95s |
| wrap | 9 | 2^27 | timeout at the probe's own 900s cap | 900.06s |
| saturate (verdict false, counted not short-circuited) | 6 | 2^18 | accept | 3.80s |
| saturate | 7 | 2^21 | accept | 29.73s |
| saturate | 8 | 2^24 | accept | 254.85s |
| saturate | 9 | 2^27 | timeout at the probe's own 900s cap | 900.06s |

The growth is 8.4x and then 7.6x per bit for wrap, and 7.8x then 8.6x for saturate, which is the enumeration's own
`2^3` and not an artifact of anything. The saturating policy is cheaper per tuple and grows at the same
rate, which is the control: the cost is the domain size, not the operation. Seconds here are an ad-hoc
quick spike with no substance; the ratio is the only part worth reading and it is the ratio the domain size
already predicts. The full transcript on disk is the authority for the rows above.

**So allowing the guard moves the arity-3 frontier from width 5 to width 8, and width 8 costs six minutes
of compile time per law.** Width 9 did not finish inside fifteen minutes, and the measured ratio puts it
near fifty. Extrapolating the measured ratio, width 10 is on the order of hours and width 16
is on the order of centuries. The guard is not the wall. The guard is a courtesy that stops you walking
into the wall, and the wall is `2^(W·k)`.

That closes the cheapest escape from section 4.2's asymmetry, and it closes it in the direction that makes
section 4.3's closed form more necessary rather than less.

**One number that travels with this wall is quoted without the thing it was counted over, and it does not
transfer.** `unstable-features.md:41-45` states that "an exhaustive check at a real width is not available:
measured on the pinned nightly, the cost quadruples per bit, reaching 28.45 seconds at eight bits, and
rustc refuses at nine", citing a source file its own name describes as being about a union computation.
`68:209` inherits "quadrupling per bit" from that rule while applying it to an arity-3 associativity check.

A growth of 4x per bit is an enumeration of size `2^(2W)`. An exhaustive arity-3 check is `2^(3W)` and must
grow 8x per bit, which is what I measure: 8.4x and then 7.6x. The two figures are not in conflict; they are
about different arities, and the rule states the ratio without stating the arity, so it cannot be applied to
a law of any arity other than the one it was measured at.

The wall itself stands and I reproduce it. What does not transfer is the rate, and it is the failure
`74:942-943` names in this panel's own words: a number carries what was counted.

### 4.5 Attacking my own consequence: the arity axis bites on schedule laws and not on grouping laws

Section 4.1's chain consequence, read flat, says a chain law of length 8 is unvalidatable above one bit.
On re-reading I did not believe it, for a reason that has nothing to do with the measurement: **grouping is
not an arity-n property.** If a binary operation is associative, every parenthesisation of a chain of any
length agrees, by the generalized associative law, which is a theorem of universal algebra, independent of
width, of the operation and of n. So the arity-n verdict for a grouping question is obtained by **lifting**
the arity-3 verdict through a proof, and the frontier's arity axis never touches it.

`80_probes/p6_which_chain_laws_reduce_to_arity_three.rs` checks that this is what actually happens, and
then finds the kind of chain statement for which it does not.

```
GROUPING: disagreeing tuples out of all tuples, over every parenthesisation
     n                   wrap      saturate (signed)
     2            0 / 256                0 / 256
     3            0 / 4096             952 / 4096
     4            0 / 65536          28917 / 65536
     5            0 / 1048576       623049 / 1048576
```

The arity-3 verdict determines every higher arity in both directions. Wrap is zero everywhere; saturate is
nonzero from 3 and stays so. As a side effect the 952 at n = 3 independently reproduces the count `74`
corrected from a consolidation that had attached it to the wrong operation (`74:939-943`, addition 952,
multiplication 160, over the 4096 triples of the signed four-bit window).

```
SCHEDULE: stepwise rounding against one rounding at the end, F = 4
  operands swept over [-64, 64) step 7
     n      disagreeing           of
     2                0          361
     3             2654         6859
     4            77399       130321
     5          1750983      2476099
```

**This one does not lift, and it cannot.** At n = 2 the two schedules are the same function, because one
rounding happens either way, so the arity-2 verdict is vacuously clean and carries no information. Every
higher n is a fresh statement, and there is no lower-arity statement that implies it, so no lifting theorem
is available at any n.

**And it is exactly the kind I7 is about.** Op's accuracy-first intent is stated over chains
(`INTENTS.md:119-121`), and what a widened intermediate buys is a schedule fact, not a grouping fact.
`79:230-239` brings `63`'s C9 to this unit, that a chain is a composition of exact operations together with
a schedule of adaptation points and the schedule is part of the function's meaning, and `79:241-251` reports
`63`'s measurement that the multiplicative fold's accumulator saving is rounding-conditional and breaks at
length 5. Both are statements about the schedule, and both are in the region where nothing lifts.

**So the corrected consequence, which is narrower and more useful than what section 4.1 said:** the const
evaluator's arity axis bites on chain laws exactly where no lifting theorem exists, which is the
schedule-conditional ones, which is the ones op's accuracy intent is stated over. Grouping-type chain laws
cost arity 3 forever, and section 4.1's table says arity 3 reaches width 5 by default and width 8 with the
guard allowed. Neither number reaches a shipped width either, so section 4.3's closed form is still the
only route at any width arvo uses; what changes is that for grouping laws the closed form has a **proof**
behind it rather than a structural argument about the representation, and a proof is the cheapest kind of
stage-minus-one output there is.

## 5. The select stage: it erases, and the erasing is not the interesting part

### 5.1 A const-gated arm erases completely, a value-gated one costs both arms

`80_probes/p3_select_and_erase.rs` builds one computation with two lowerings, `general` and a `fused` form
legal only where distributivity holds, and selects between them three ways. Emitted assembly at
`80_probes/p3_lib.s`, extracted at `80_probes/p3_asm_report.txt`.

```
sel_static::<0>   3 instructions   add ; mul ; ret
sel_static::<8>   6 instructions   mov ; madd ; madd ; asr ; add ; ret
sel_dynamic      13 instructions   ... both arms computed ... cmp w3, #0 ; csel x0, x8, x9, ne ; ret
```

The static arms carry no trace of the predicate. The dynamic arm materialises **both** lowerings and picks
with a `csel`, and it is worse than either static arm, not worse than the better one: 13 against 6 against
3. A law whose predicate is not const is not the compile-time win minus a check. It is worse than never
having had the law.

That is I13's mechanism validated at the law layer, and as far as I can tell it is the first time this
panel has looked at what a law-gated arm emits.

### 5.2 And the same probe refuted the thesis it was built for

The assembler emitted three symbol aliases:

```
only_general_f8 = sel_static::<8>
only_fused_f0   = sel_static::<0>
only_general_f0 = sel_static::<0>
```

The first two say the selection reached the right arm. **The third says that at F = 0 the general form and
the fused form are the same symbol**, because LLVM performed the distributive rewrite itself. On that
shape, in that region, the select stage bought nothing at all.

The instrument is `68`'s. `68:179-195` used assembler symbol aliasing (`_add_trusted = _add_bare`) to
establish operation erasure at one instance. The same aliasing here establishes something different and
uncomfortable: that two expressions the design considers algebraically distinct are one function to the
backend, which is what makes the arm redundant rather than what makes it correct.

So the question a law layer has to answer is sharper than whether the law holds. It is **whether the law
lets the design reach a lowering the backend could not reach on its own**, and that region is strictly
narrower than the region where the law is true. This is the microkernelling shape the workspace already
names: the typestate knows something the backend never learns, and the win exists only where the backend
could not have proved it.

### 5.3 Where a law does pay, and the two attacks it took to get there

`80_probes/p4_what_the_law_unlocks.rs` takes the candidate a backend structurally cannot do for itself: a
**reduction** of saturating additions. A backend will not reassociate a reduction it cannot prove
associative, and saturating addition is not associative in general. Unsigned saturating addition is, which
the probe re-establishes exhaustively at runtime, 0 of 16,777,216 triples failing, independently
reproducing `76`'s probe1b finding from a different instrument.

From `80_probes/p4_asm_report.txt`, inner loop bodies only:

| arm | inner-loop instructions | elements per iteration | instructions per element | vector `uqadd` |
|---|---|---|---|---|
| `sat_sum_seq`, the fold as written | 6 | 1 | 6.000 | no |
| `sat_sum_lanes`, law used badly | 34 | 4 | 8.500 | no |
| `sat_sum_lanes16`, law plus bounds proof | 4 | 16 | 0.250 | yes |
| `sat_sum_lanes64`, plus unroll and tree combine | 9 | 64 | 0.141 | yes |
| `wrap_sum_seq`, control | 8 | 64 | 0.125 | n/a |

**The first attempt was worse than doing nothing.** `sat_sum_lanes` uses the law correctly and indexes the
slice, so the bounds are not provable, so the backend emitted four calls to panic paths and abandoned
vectorisation: 8.50 instructions per element against the unlicensed 6.00. The law was true, the arm was
legal, and the arm lost.

**The first attack supplied the missing proof rather than a missing intrinsic.** Iterating
`chunks_exact(16)` gives an element of known length, so no bound has to be proved, and the loop collapses
to four instructions per sixteen elements with one `uqadd.16b`. **The second attack** unrolled to four
vector accumulators and folded the horizontal combine as a tree, both licensed by the same law, reaching
0.141.

The control is the honest comparator: `wrap_sum_seq` is wrapping addition, associative unconditionally, and
the backend vectorises it with no help from any typestate, reaching 0.125. So the law-licensed saturating
arm lands within 13% of the density the backend achieves on the case where it needs to be told nothing.
Every reassociated arm agrees with the sequential one on 0 disagreements over lengths 0 to 300 and 64
seeds.

**Instructions per element is not time.** Nothing here is timed, no bench ran, and every ratio above is an
ad-hoc quick spike with no substance for a how-much question. The qualitative fact a spike can carry is
that the vector saturating-add instruction appears **only** in the arms the law licensed and never in the
one the backend produced on its own.

## 6. Against `79`: P4 is a trajectory predicate, not a const predicate, and that is where the two readings of "validate" actually bite

`79` is the strongest file in this unit on **which dimensions** a law's region has, and it does not ask when
any of them is known. Its headline result is P4, a four-way case split that carves the holding region of
the composed law `(a+b)-c == a+(b-c)` for unsigned saturating `u8` with zero residue in both directions
(`79:64-78`). I opened its probe and its output and both hold up.

**P4 is a predicate on the data.** Its cases are whether `a+b` clamps at the ceiling, whether `b-c` clamps
at the floor, and in the mixed cases whether a specific operand equals zero (`79:72-74`).
`79_probes/p1_compositional_predicate_search.rs` contains no `const fn` and no `const` item; every
candidate is an ordinary function of `(a, b, c)` evaluated in `fn main`. So P4 is established by a runtime
sweep and, more importantly, **P4 can only be evaluated at runtime**, because `a`, `b` and `c` are values.

I13 says "a bunch of arms with **const predicates**" (`INTENTS.md:184-185`). A case split on whether a
particular addition clamped is not a const predicate. It is a branch, per datum, and section 5.1 measured
what that costs: both arms materialised plus the select, worse than the unlicensed form.

So `79:322-325`'s statement that "Every predicate this file states is a `Q-A`, compile-time reading claim"
is not right about its own result. P4 is a Q-A **runtime** object wearing a compile-time file's clothes,
and the file's own framing of it as "I13's own shape" (`79:80-93`) puts it in the wrong stage.

**This is not a demotion of `79`'s finding. It is a relocation, and the relocation is the general result.**
The panel has been producing two kinds of law region and calling both predicates:

**Typestate predicates.** Functions of the type: `F == 0`, sign domain, overflow policy, representable-set
symmetry, container width. Known at stage zero, gateable, erasing, and what I13 names.

**Trajectory predicates.** Functions of the values flowing through: no clamp event occurred, an operand is
zero, the running accumulator did not reach an endpoint. Known only at stage one. A characterisation of
where a law holds, and not an arm.

`OPTIONS.md:1113-1115` carries `42`'s reachability condition, that "associativity survives exactly when the
fold's actual trajectory cannot reach both clamped endpoints", contested in its quoted form by `55b`. What
matters here is not which form is right but that the same entry describes the condition as a fact about
"a *specific fold's declared operand range*". **Declared** is the word that moves it across the stage
boundary. A trajectory condition over a declared range is a typestate fact; the same condition over actual
values is not. `42` got the staging right and `79` did not, and the whole difference is whether the
predicate reads a declaration or a datum.

**And this is where the runtime reading of "validate" stops being a cost question.** If validation may run
per datum, a trajectory predicate becomes gateable at a per-datum price, and P4 becomes an arm that runs
where the law holds and a general path where it does not. If it may not, P4 is not an arm at all unless
somebody lifts its value conditions into declarations, and that lifting is design work nobody has done. So
Q-A at the law layer decides whether an entire class of the panel's own measured regions is reachable.
I surface that and do not answer it.

**A second consequence for how a region is stated.** `79:95-100` writes P4's predicate as `N = 8, S =
saturate (unsigned), F = 0, threads any, features any`. Every dimension in that list is a typestate
dimension. The four-way case split, which is the actual content, appears in prose beside the predicate
rather than inside it, because the notation has no slot for a dimension whose value is not known until the
program runs. That is the notation reporting, correctly, that the finding is not of the kind it was built
to carry.

## 7. Where `76` and `77` actually diverge, and it is not about laws

`76` says a law belongs to the pair of a strategy and a set of operations composed under it. `77` says a
law is a property of a judgment between two expression forms which may be chains of arbitrary finite
length. Both files treat this as one claim reached two ways, and `77:372-378` counts the agreement as two
formal traditions landing on one structural object.

Read as a staging question, they agree exactly on one region and diverge exactly at its edge.

**A chain whose shape is known at stage zero is one big operation.** If the schedule is static, `76`'s
"set of operations composed under a fixed strategy" is a complete description of it, and `77`'s judgment
over that chain is a property of a stage-zero object. The two framings name the same thing.

**A chain whose length is known only at stage one has no stage-zero object to be a property of.** `77`'s
judgment then quantifies over something that does not exist until the program runs, and `76`'s set of
composed operations cannot be written down because nobody knows how many there are.

`79:230-239` brings `63`'s C9, resting on `60`, into this unit: "a chain is a composition of exact
operations together with a schedule of adaptation points, and the schedule is part of the function's
meaning". Read as staging, that sentence says **the schedule is the static part and the operands are the
dynamic part**, and a chain's law is a fact about the residual program the schedule describes. `79` reads
it as a missing predicate dimension, which it also is; what it adds here is that the dimension in question
is the one that decides whether the other dimensions are knowable at all.

Two things in the register are the same finding from other directions, both cited through the register
because I did not open their sources. `OPTIONS.md:1063-1065` reports `35`'s fold refusal, four formulations
refused with four negative controls "locating the boundary at the **runtime trip count** rather than at the
widening, which composes fine in expressions and over static-length lists". That is a staging boundary
stated as a compile error: the accumulator's type wants to be a function of a value that does not exist
until stage one. And `OPTIONS.md:1236-1238` reports `43`'s "capacity static, length dynamic" as the
defining boundary of an aggregate, with `35`'s refusal being that boundary crossed wrongly.

**So the fold refusal is not a limitation of Rust's type system that a design has to work around. It is the
staging boundary reporting its own position.** Capacity is stage zero and length is stage one, and a law
quantified over capacity is derivable while the same law quantified over length is not. That is why the
capacity-keyed accumulator relation in Q11 compiles gate-free and the length-keyed one does not.

## 8. What a generator of laws would have to be given, and why the canon should state the generator

My dispatch asks whether the canon should state the generator rather than the laws. The panel has already
computed the generator's domain, at a different unit, for a different reason.

`74:144-147`, which I read at the source: "A law contract is decided at the pair of identity and selected
adaptation, reads neither encoding nor container, and is undecided by the identity alone: the same ambient
domain, representable set, encoding and container, with only the adaptation moved, flips a bound from
accepted to refused." And `74:507-511`'s N6: what a crossing **means** is decided by the ambient domain,
the representable set and the selected reduction; what it **costs** is decided by the encoding and the
container, "and those two can never change what it computes."

So the generator's signature is a **proper prefix** of the five-choice sequence, and the interesting content
is the invariance rather than the domain: **a law verdict is invariant under any change of encoding or
container.** That is a statement that survives a rewrite in another language in another decade, it forces
three independent implementations to agree, and it names no mechanism. It is the shape a canon sentence
should have.

It also has a direct consequence for stage zero that nobody has stated. If the verdict does not read the
encoding or the container, then **the law layer's stage-zero computation is keyed on strictly less than the
container derivation's is**, so a law verdict can be computed and cached once per identity-and-adaptation
pair and reused across every container that pair is realised in. Whether that matters is unpriced, and it
is a fact about the generator's domain rather than about any implementation of it.

`79:200-219` sharpens the domain further and I agree with it independently: `(operation, strategy)` is not
enough, because `63`'s cube separates two rows on representable-set symmetry with sign, operation and
policy all held fixed. Read against `74:144-147` that is not a surprise, it is the same statement: the
representable set is a coordinate of the identity, and the identity is half of what decides a law contract.
Two files reaching that from opposite ends is worth recording, and I got there from the crossing material
rather than from the cube.

## 9. What survives erasure, and what the trusted base gains

`77:169-186` answers the consumer question well: a consumer is entitled to exactly as much as the
compile-time obligation their own instantiation discharged, and nothing about the erasure carries
information across instantiations.

The staging reading sharpens where the guarantee lives. **After erasure the law is not a property of the
value. It is a property of the generator that emitted the code operating on the value.** The consumer holds
a residual program that was produced under an assumption, and the assumption's evidence is not in the
program, by construction, because erasure is what removed it.

That puts one more item on `68`'s trusted base, and it is the item with the widest blast radius.
`68:221-253` itemises nine, of which the canon-shrinkable ones are the transfer proviso,
validation-through-maps, the construction perimeter, and per-instance operation erasure. The law layer adds
**the verdict table itself**: every closed-form verdict an arm gates on, together with the model band its
agreement was checked over and the transfer proviso to the shipped widths. A wrong entry there does not
fail loudly at one site; it produces arithmetic that is silently wrong at every site that selected on it,
which is `76`'s congruence-closure failure mode (`76:120-131`) with a machine-level mechanism attached.

`p2c` is the smallest thing I know of that makes that item auditable rather than merely stated: the table is
in the source, the agreement over the model band is a compile-time assertion, and the perturbation is
refused.

## 10. Fits against the register

**Kills nothing.** No option in `OPTIONS.md` is closed by anything above, and nothing moves to
`DROPLIST.md`.

**Fits well:**

- **Q1 (`OPTIONS.md:57-88`).** The three parts of op's "all three" answer gain a fourth question each,
  which is when each part runs. Section 1.2's three-by-two grid is the register's Q1 crossed with `74`'s
  Q-A, and the observation that the panel occupies one of six cells belongs under Q1 rather than under Q-A.
- **Q11 (`OPTIONS.md:1055-1086`).** The accumulator-relation option, which keys on capacity because
  capacity is a type, is the staging boundary stated as a design. Section 7 gives it a reason rather than a
  measurement: it works because capacity is stage zero and length is not.
- **Q12 (`OPTIONS.md:1088-1157`).** The reduction-order options are a binding-time fork in disguise.
  "Specify the reduction shape" makes the schedule stage zero and therefore makes chain laws derivable;
  "say nothing, and let the answer depend on the core count" makes it stage one and makes them not.
- **Q25 (`OPTIONS.md:1637-1641`), how the law inventory is named.** Section 4.3's shape is a candidate that
  is neither of the two on offer: a closed-form verdict function, plus a compile-time agreement check
  against the swept verdict over a stated model band, plus the transfer proviso named as the residue.

- **Q12 again (`OPTIONS.md:1088-1157`) gains a second reading from section 4.5.** Its "require
  associativity" option is a grouping question, so its verdict lifts from arity 3 and costs nothing extra
  at any chain length. Its accuracy-across-chains sibling in I7 is a schedule question, so it does not lift
  and its verdict costs arity n. Two options in the same entry sit on opposite sides of the frontier and
  the entry does not currently say so.

**Fits badly, and survives at a cost:**

- Any option that assumes a law's compile-time validation is available at the width it is claimed at. That
  includes both cold derivations' account of their own mechanism. It survives at model widths and it does
  not reach shipped ones, and section 4.3 is the price of keeping it.

**Options added, written out in full so they are not lost.**

**O-F. Where a law verdict's truth is established, as three named alternatives.**
**(a) In the compiler, per instantiation, exhaustively.** The shape both cold derivations built. Cost:
bounded by the frontier in section 4.1, so it reaches arity-3 laws at width 5 and chain laws of length 8 at
width 1, and produces only negative verdicts at shipped widths. Buys: no trusted base item at all, because
nothing is asserted.
**(b) Offline, at a model width, cited in the compiler.** The verdict is established in the audit trail and
the typestate carries a declaration of it. Cost: `68`'s paper-checking-paper hole in full, plus the transfer
proviso. Buys: any width, any arity, constant compile cost.
**(c) Offline as a closed form, cross-checked in the compiler against the sweep over a stated model band.**
Section 4.3, built at `80_probes/p2c_closed_form_checked_on_a_model.rs`. Cost: the model band is a compile
budget spent per law, nearly exhausted at four widths and one law on this host; and the transfer proviso
survives as a single named residue. Buys: the declaration is checked against the maps everywhere it can be,
and the unchecked part is one sentence rather than the whole verdict. **What would distinguish them:**
whether the canon is willing to carry a named trusted-base item for law verdicts. (a) needs none and reaches
almost nothing; (b) and (c) differ only in whether the item is auditable.

**O-G. Whether an arm's predicate may read data.** **(a) Typestate only**, which is I13 read literally: an
arm is selected at monomorphisation and every law region expressed over values is a characterisation rather
than an arm. Cost: every trajectory predicate the panel has measured, including `79`'s P4 and `42`'s
reachability condition in its value-level form, is unusable until somebody lifts its conditions into
declarations. **(b) Typestate or data**, with a value-gated arm permitted where it pays. Cost: section
5.1's measurement, both arms materialised plus a select, worse than the unlicensed form on the one shape
measured; and the erase clause acquires an exception. **(c) Typestate only for selection, data permitted at
a declared ingest boundary**, so a trajectory condition is checked once where values enter and becomes a
typestate fact afterwards. Cost: a door, and the residue `68:179-195` measured for the runtime reading.
**What would distinguish them:** whether any trajectory predicate the panel has measured has a lifting into
a declaration that a consumer would actually write. Nobody has tried to construct one.

**O-H. Which route a law verdict takes to its closed form.** Section 4.5 separates two, and they have
different costs and different failure modes. **(a) A proof, lifting a lower-arity verdict.** Grouping-type
chain laws take this route: the arity-3 verdict plus the generalized associative law gives every arity, at
no additional compile cost and no additional model band. Cost: the proof is stage-minus-one work in the
audit trail and the canon has to say which theorem it rests on, since a wrong lifting is invisible at every
instantiation. **(b) A structural argument about the representation.** Wrapping's group structure takes
this route: the verdict is a fact about what the encoding realises rather than about a lower arity. Cost:
the same, one audit-trail item. **(c) No route, and the verdict stays swept.** Schedule-conditional chain
facts are here, and section 4.1's table is what they cost. **What would distinguish them for a given law:**
whether the law is a consequence of a lower-arity law, which is decidable by inspection rather than by
measurement, and this panel has never asked it of any law it measured.

## 11. Findings, in the required predicate notation

Each names only what was established. Absence of a dimension is the strongest negative claim in this
notation and is meant where it appears.

- **The law layer inherits `68`'s declaration hole (section 3.1).** `N = 4, sign = signed, policy =
  {wrap, saturate}, op = add, F = 0, arity = 4, threads = 1, features any`. A marker contract declared by
  an author is checked by nothing and the licensed consumer disagrees with the sequential one on 16,268 of
  65,536 vectors for the false declaration. Features are `any` because the arithmetic is pure value
  computation with language-specified semantics; threads is `1` because the instrument ran on one and
  nothing about concurrency was checked.

- **The compile-time law-validation frontier (section 4.1).** `toolchain = nightly-2026-05-28, host =
  aarch64-apple-darwin, policy = saturate, sign = signed, F = 0, threads = 1, arity in {1,2,3,4,5,6,8,12},
  width = the walked range per arity`. The widest evaluable width is 19, 9, 5, 4, 3, 2, 1, 1 respectively,
  and every first refusal is `long_running_const_eval`. Not claimed at any other toolchain or host; the
  guard is time-based and both are dimensions of the result.

- **Negative law verdicts are cheap and positive ones are refused, at a shipped width (section 4.2).**
  `toolchain = nightly-2026-05-28, host = aarch64-apple-darwin, N = 8, sign = signed, policy in
  {wrap, saturate}, op = add, F = 0, arity = 3, threads = 1`. The false verdict compiles to `E0080` in
  0.50s total; the true verdict is refused after 4.48s.

- **A closed-form verdict cross-checked against the sweep on a model band compiles, gates at width 64, and
  refuses a perturbation (section 4.3).** `toolchain = nightly-2026-05-28, host = aarch64-apple-darwin,
  policy in {wrap, saturate}, sign = signed, op = add, F = 0, arity = 3, model band = widths 2..=5, gated
  width = 64, threads = 1`.

- **A const-gated law arm erases and a value-gated one materialises both arms (section 5.1).** `toolchain =
  nightly-2026-05-28, host = aarch64-apple-darwin, target = aarch64 baseline, opt = -O, op = the
  general/fused pair in the probe, F in {0, 8}, threads = 1`. 3, 6 and 13 instructions for the two static
  arms and the dynamic one, with one `csel` in the dynamic arm only.

- **At F = 0 the backend performs the distributive rewrite unaided, so the arm buys nothing there (section
  5.2).** Same predicate as above, `F = 0` only. Established by symbol aliasing in the emitted assembly, not
  by comparing bodies.

- **A law-licensed reassociation of a saturating reduction reaches vector saturating add where the
  unlicensed form does not, and the naive licensed form is worse than the unlicensed one (section 5.3).**
  `toolchain = nightly-2026-05-28, host = aarch64-apple-darwin, target = aarch64 baseline NEON, opt = -O,
  N = 8, sign = unsigned, policy = saturate, op = add, F = 0, arity = fold over a runtime-length slice,
  threads = 1`. Inner-loop instructions per element 6.000, 8.500, 0.250, 0.141 against a control at 0.125.
  Unpriced: no bench ran and instructions per element is not time.

- **Unsigned saturating `u8` addition is associative (section 5.3, incidental).** `N = 8, sign = unsigned,
  policy = saturate, op = add, F = 0, arity = 3, threads = 1, features any`. 0 of 16,777,216 triples fail.
  This is a second instrument on `76`'s probe1b finding, not a new one.

- **Allowing `long_running_const_eval` moves the arity-3 frontier from width 5 to width 8 and no further
  in practice (section 4.4).** `toolchain = nightly-2026-05-28, host = aarch64-apple-darwin, policy = wrap,
  sign = signed, op = add, F = 0, arity = 3, threads = 1`. Widths 6, 7 and 8 accept at 5.85s, 49.06s and
  370.95s, a ratio of 8.4x then 7.6x per bit. The extrapolation past width 8 is arithmetic on that ratio and
  is not a measurement.

- **Grouping-type chain laws are determined by their arity-3 verdict; schedule-type chain laws are not
  determined by any lower arity (section 4.5).** `N = 4, sign = signed, policy in {wrap, saturate}, op =
  add, F = 0, arity in {2,3,4,5}, threads = 1` for the grouping half, exhaustive over the window. `N = 7
  effective (operands swept over [-64, 64) step 7), F = 4, op = multiply, schedule in {stepwise,
  round-once}, arity in {2,3,4,5}, threads = 1` for the schedule half, which is a sweep and not exhaustive
  and is stated as such. The grouping half's lifting direction rests on the generalized associative law,
  which is a theorem rather than a measurement, and the measurement is a check that it is what happens.

- **`79`'s P4 is a predicate on values (section 6).** Not a measurement of mine. It is a reading of
  `79_probes/p1_compositional_predicate_search.rs`, which contains no `const fn` and no `const` item, and
  of `79:72-74`, whose cases name clamp events and operand values.

## 12. Where this file is least certain, stated as a floor for whoever attacks it

1. **The frontier table is one host and one toolchain.** The guard is time-based, so a faster machine moves
   every row. What I claim is the shape, that the frontier is a curve in (width, arity) with the arity axis
   collapsing it fastest, and the shape does not depend on the constant. Section 4.4's measurement bounds
   how much a faster host could buy: three bits of width cost sixty-three times the compile time on this
   one, so a machine an order of magnitude faster moves the frontier by one bit.
2. **Section 4.5's schedule half is a sweep, not an exhaustive check**, over one fraction width, one
   operation and a strided operand range. Its grouping half is exhaustive over its window. The asymmetry is
   deliberate, since the grouping half is the one carrying a claim about lifting, but a reader should not
   read the two tables as the same kind of evidence.
3. **Section 5.3's arms are one shape.** A saturating reduction is the case chosen because a backend
   structurally cannot reassociate it. Whether other laws unlock anything is untested, and section 5.2 is
   direct evidence that some do not.
4. **The stage-minus-one framing is mine.** Nothing in op's words names a design-time stage, and the whole
   of section 2's three-stage reading is my synthesis of his three verbs with the frontier measurement. It
   is offered for attack.
5. **My reading of `79`'s P4 as unusable as an arm rests on I13's phrase "const predicates"** and on the
   observation that its cases read values. If op's intent permits a value-gated arm, section 6's conclusion
   weakens to a cost claim, and section 5.1's measurement is the cost.
6. **Everything I say about `35`, `42`, `43`, `60`, `63` is second hand**, through `OPTIONS.md`, `74` or
   `79`. If any of those accounts is wrong, the corresponding paragraph inherits it, and section 7 leans on
   two of them at once.

**Not done, and what it leaves for whoever attacks next.** No attack on section 4.3's cross-check mechanism
itself, which is the piece I would most want broken, since it is the only route to a shipped width that I
found and it rests on the model band being representative. No attempt to construct the lifting section 6
says nobody has tried: taking one trajectory predicate the panel has measured, `79`'s P4 or `42`'s
reachability condition, and finding the declaration a consumer would write that makes it a typestate fact.
That is the cheapest next instance available and it decides Q39. No second shape for section 5.3, so the
one microkernel result is one instance. No measurement at all of the compile cost of section 4.3's
cross-check across several laws, which is the number that decides whether the mechanism scales past the one
law I built it for. And nothing anywhere near a bench: every magnitude in this file is unpriced.

## 13. The two questions in flight, and what this file adds to each without answering either

**Q-A, which verb "validate" is.** Section 1.2 adds that op's own answer sits on a different axis from the
one Q-A names, and that the two compose into a three-by-two grid the panel occupies one cell of. Section 6
adds the part that makes Q-A more than a cost question at this layer: the answer decides whether trajectory
predicates can be arms at all, and therefore whether an entire class of this panel's measured law regions
is reachable. Section 4.2 adds that the compile-time reading, at shipped widths, produces only the verdict
that licenses nothing.

**Q-B, whether the long-standing constraints are op's intents.** Nothing above depends on `no_std`,
`alloc`, `dyn` or `TypeId` either existing or not. Every probe is plain arithmetic and const evaluation; the
frontier is a property of the const evaluator; the emitted assembly is from ordinary monomorphisation.
Section 5.1's claim that a const-gated arm erases would be weakened if dispatch were permitted, since the
selection could then be deferred, but that is an observation about what the ban buys rather than a
dependence on it.

**Nothing here settles anything.** The mode is explore. This file goes to whoever attacks next.
