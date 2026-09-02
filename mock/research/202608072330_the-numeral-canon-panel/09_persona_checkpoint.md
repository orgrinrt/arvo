# 09. Persona checkpoint on the first stretch

**Date:** 2026-08-08, overnight. **Author:** the `orgrinrt` persona, standing in while op sleeps.
**Status:** PERSONA. No authority. Nothing here ratifies, nothing here settles, nothing here is op's
word. Per `04` and `RULES.md`, every call below is the persona's and is logged as such in
`PERSONA_CALLS.md`.

Where I am guessing at what op would think rather than reading it in the record, I say so inline with
the word **guessing**, and section 10 lists every instance in one place. Where op's recorded words
appear to cut against my instinct, his govern and I write the conflict down rather than resolving it.

**What I read.** `04`, `01`, `RULES.md`, `00_brief.md`, `05`, and `03`, `06`, `07`, `08` in full, in
that order, plus `MORNING.md` checked line by line against the four. `SETTLED.md` in two passages that
a specific claim sent me to. I did not read `CANON_CANDIDATE.md`, `DROPLIST.md`, `02_carried`, `seed/`,
or the predecessor's tree. I spot-checked four probe outputs by opening them rather than trusting the
files that cite them, and section 9 says which.

## What I would put in front of op, and it is not what MORNING leads with

Three things, in this order.

### 1. The one task op actually assigned is the one nobody took, and there is a contradiction sitting inside it

`01` section 2 is op's own words, and it is not a question waiting on him. It is a standing
instruction:

> I said fresh eyes on it, that holds.

`00_brief.md:87` carries it as one of the three things "the panel does first" and says op flags his own
recall as approximate, so the task includes verifying the properties hold as stated.

**Four dispatches ran and not one of them took it.** Grepping the panel, `container derivation` appears
in the four expert files only in passing: `03:319` as an aside about a negative-width floor, `06:693`
as an admitted historical observation about a tree the brief declares nuked, `08:456` and `08:731` as a
downstream cost of something else. Nobody located the attempt. Nobody opened it.

And there is a live contradiction inside the task that makes it sharper than a to-do. Op, at
`01:73-77`:

> pretty sure that did not contain any enumeration, and had the contracts and typestate work fully
> (within its framework ...) without forbidden features. It was pretty nice.

`SETTLED.md:149-153`, which is agent summary:

> every gate-free candidate the panel built either enumerated widths or degraded the diagnostic to
> something unreadable, and op declined the trade each time rather than picking a side. The erasure
> gate is ratified and its mechanism is not, which is the largest structural gap in the panel.

Those two cannot both be right. Under the provenance ladder op's words outrank the summary, and op
himself says his recall is approximate, so neither settles it. **What settles it is opening the
candidate**, which is cheap and mechanical: check it for `#![feature(...)]`, check whether a width
table appears anywhere in it, read its diagnostic. `08` did exactly that kind of work for the
segmented-typestate probe three files had named as owed, and it took one probe.

That the panel's own index calls this "the largest structural gap" while four dispatches went
elsewhere is the finding, not the gap itself.

### 2. `MORNING.md` promotes an inherited claim to independent corroboration, in the document op reads first

`MORNING.md:116-119`:

> **`06`'s D0/D1 split is the fibre/index line falling out of the mathematics**, rather than a taxonomy
> imposed on the sites. Two experts arrived at the same boundary from opposite directions, one by
> enumerating twenty sites and one by setting up the adjunction, **neither having derived it from the
> other.**

`07` says the opposite about itself, twice. Its reading list at `07:10-13` has
`06_kiselyov_where_a_numeral_is_inferred.md` read in full **before** deriving. `07:940-942`:

> Where this file agrees with `03` or `06` I read both before deriving, so the agreement is inherited
> rather than found; what is independent is the measurement.

And `07:942-943` puts **"section 1.4's claim that the D0/D1 split is the fibre/index split"** first on
its own list of things owed a second read. `07:988-990` names the only two places its agreement is
genuinely independent, and section 1.4 is not one of them.

So a ONE EXPERT claim, flagged by its own author as inherited and explicitly queued for a second read,
arrives in op's morning summary as two experts converging from opposite directions. That is exactly the
rung inflation `RULES.md:28-40` exists to prevent, and it is the failure mode
`a-compression-is-checked-by-someone-else.md` describes: the compressor supplies the missing context
from memory and the sentence reads fine.

Fix: restore `07`'s own wording. One line.

### 3. The stretch has one genuine three-instance result and it got overturned in the same stretch

The best-corroborated claim here is `03`'s cross-kind F2 witness, `U<0,1>` against `U<2,0>` with two
incomparable minimal upper bounds. Three independent instruments produce it: `03`'s Python and Rust
pair (`i1.out` Q5, `i3.out` C3), `07`'s `p7`, and `08`'s `i2`. That clears `RULES.md:116-118`'s
three-instance bar, which almost nothing else here does.

And `08:56-60` then shows the antichain is an artifact of the **named** shapes rather than of the
concept: the join of `03`'s own witness pair is `{0, 1/2, 1, 2, 3}`, it contains both operands, and it
sits strictly inside both minimal upper bounds `03` named. `03` predicted the repair would be "a shape
strictly between the operands and the current minimal ones" (`03:186-187`); that shape exists and is
segmented, which is why no uniform shape sits between and why `03` could not find it.

A fact verified three ways, whose significance reversed once someone widened the search space. Worth
op seeing as a pair, because it is the cleanest illustration in the stretch of why corroboration of a
measurement is not corroboration of what the measurement means.

## The inherited-versus-independent audit

`08:739-744` flags its own agreement as inherited. So does `07:940-943`. So does `06:536-541`. All three
noticed. What none of them said is that **the panel's dispatch shape makes it unavoidable.**

`RULES.md:163` mandates "One expert at a time, never in parallel. Sequential and cumulative: each reads
the ones before it." `RULES.md:28-30` defines the TWO EXPERTS rung as "each having derived its own
answer **before** reading the other. That ordering is the whole content of the rung."

Those two rules are in direct tension, and under the current shape the second is structurally
unreachable. Every member reads every predecessor in full before deriving, by instruction. The rung can
then only be reached by accident, when a member happens to attack a claim by a different method, which
is what `07` did on the product form and nothing else.

The audit, claim by claim.

| Claim | Files | Real rung |
|---|---|---|
| Named operations derive result numerals by formula, not join | `03` opt H, `06` sec 6 | ONE EXPERT plus one independent measurement (`06` p1: 1175/1296, 0/1296) |
| D0/D1 split is the fibre/index split | `06` sec 3, `07` sec 1.4 | **ONE EXPERT**, inherited, owed a second read by `07`'s own list |
| `06`'s tight product form is the least containing numeral | `06` sec 7.1, `07` sec 3.3 | **TWO EXPERTS**, genuinely different derivations, 400/400 |
| Negative integer width is needed | `03` sec 3.1, `06` sec 7.2, `07` sec 3.3 | Three arrivals, and `06` **corrects** `03` rather than agreeing |
| Cross-kind F2 witness and its two minimal upper bounds | `03`, `07` p7, `08` i2 | Three independent instruments. Meaning then overturned by `08` |
| The two closure conditions are one formula's codomain | `07` sec 3.1 | ONE EXPERT, and it **contradicts** `03`'s framing of them as two jobs |
| Gradual underflow is the meet of a fixed format and a float | `08` sec 4.2 | ONE EXPERT, two independent methods inside one file (enumeration and trait solver) |
| Suite counts 91 / 83, and no source for this surface | all four | Corroborated four times and worth nothing |

Two entries there are worth reading as the panel's real yield: the tight product form, and the
negative-width caller. Everything else with several files behind it is a chain.

One correction to `RULES.md`'s own bar as MORNING applies it. `MORNING.md:130` says of the product
form "That is two instances arrived at differently, **which is the bar**." `RULES.md:116` says "Three
independent ones is the bar." Two experts agreeing is the provenance rung; three instances is the
evidence bar; MORNING has merged them. The honest sentence is that the product form has two of the
three instances it needs.

## Other defects in `MORNING.md`, itemised

Checked line by line against the four files. These are ordered by how much they mislead.

**`MORNING.md:14`, and it doubles a number.**

> `06` was dispatched to test that head-on and enumerated **twenty sites** where the design produces a
> numeral the consumer did not write.

`06` enumerated twenty sites **total**, of which eight are D0, where the consumer names the target and
the design produces nothing. `06:219-220` states it as the point of the exercise: "**Eight turn out not
to be sites at all**, which is a result rather than an absence of one". Counting the distinct sites
where the design does produce an unwritten numeral gives D1's eight plus D2's three, with site one
appearing in both, so **ten**, not twenty. MORNING has doubled the inference surface and deleted
`06`'s headline result in the same sentence.

`06`'s own section heading at line 208 invited it: "every place a numeral appears that the consumer did
not spell", above a table that includes eight places the consumer did spell. Both want fixing.

**`MORNING.md:141` and `07:617` collide with the panel's own vocabulary rule.**

`07` heads section 4.3 "The cross-kind case is **priced**, not closed" and then writes at `07:934`
"**Everything here is unpriced.** No bench harness run bears on any of it." `RULES.md:119-122` reserves
that word: "Where nothing has been measured on the harness, the magnitude is **unpriced**, and that word
is used rather than reaching for a number." MORNING carries the first sense into op's summary. What
`07` measured is a **count of shapes**, 16 to 34 percent more of them. That is a size, not a price. The
word should be "sized".

**`MORNING.md:225` is stale.** It says `07` is running. `07` and `08` have both landed and are both
summarised above it in the same file.

**`MORNING.md:220-223`, the open list, was written after `06` and never updated.** It names mixed-numeral
addition, closure under formula clamps, `06` section 10, and the family question. It omits `07`'s
saturating-top question, `07`'s compatibility condition (the thing `07:918-922` says it most wanted to
build and did not), and `08`'s question one, which `08:709` calls the one everything else is downstream
of. Three of the stretch's four files produced open items that are not in the open list.

**`MORNING.md:214-215`** says "Both experts kept a broken instrument rather than deleting it" and names
`03` and `06`. Four experts did. `07:896-898` keeps its asm comparison's first wrong run in the tool's
own docstring; `08:630-661` keeps four defects including one it names as "the failure `RULES.md` warns
about by name". Undercounting the honesty here matters, because that discipline is the main reason to
believe the numbers.

## Per file: what holds, what is thin, what I would push on

### `03`

**Holds.** The three failure modes, F1, F2, F3, each with a witness and each taking a different repair.
That is the single most reusable piece of vocabulary the stretch produced and every later file uses it.
The finding at section 3.2 that reading A's two admissions do not close the cross-kind join, argued by
a counting argument (the exact union needs seven values and seven is not a power of the radix) and
reproduced by three instruments. The step-set reframing at 5.2, where radix 2 against 4 behaves as one
family, 2 against 3 fails as F1, and 2 against 6 fails as F2, and the three outcomes track step-set
nesting exactly. Keeping `i1_shape_space.py` unfixed with its setup-that-helps named in its own header.

**Thin.** Section 5.2 leaves 31 cross-radix pairs unclassified and then argues from one line why they
are probably box artifacts. `03` marks it ("I am not calling them artifacts on the measurement"), which
is honest, and it is still a hole in the strongest support reading E has.

**Would push.** Section 1.1's "the meet has no caller" was `03`'s highest-value open item and `06`
half-overturned it within one dispatch by finding a caller for the admission, not the operation. The
lesson is that "I looked and did not find one" over four documents is a weak negative, and `03` says so.
I would not have let it carry as much of section 4.4's weight as it did.

### `06`

**Holds, and this is the best result in the stretch.** Section 1.1: `E0119` refuses the two honest
readings of a cross-family join declared side by side. I opened `06_probes/p4_arm3.out` rather than
trusting the citation, and it is real: `conflicting implementations of trait JoinNum<Flt<_,_,_>> for
type Uni<_,_>`. That converts a large open structural question into a small one, because it says a
design that infers a cross-family target is picking in one impl whatever the order does, and it holds
however the family question is answered. It is `a-test-that-cannot-compile-is-the-finding.md` working
as intended.

Second: section 7.1 found, unasked, that a canon sentence claiming the derived numeral is tightest
would be **false** at 461 of 6561 pairs, derived the tight form, built it, and built a negative control
that fails at `E0080`. That is the shape of contribution this workspace wants and it should be said
plainly.

Third: catching that `03`'s citation of `SETTLED_laws.md:165-178` supports the map's shape and not its
content, with the grep in the file. Exactly the check `RULES.md` asks for.

**Thin.** The D-carve is good and the twenty-site enumeration claims exhaustiveness on the strength of
one author walking the operation surface twice. `06:707` lists it as owed a second read, correctly. The
first pass missed every unary operation, which is the tell that a third pass would find more.

**Would push.** Section 4.3's quadratic-versus-linear family cost is presented as arithmetic and is a
model, not a measurement: "eight sites take a formula per unordered pair of families" assumes every D1
site needs a distinct cross-family formula, and several plainly do not (exact scaling by a power of the
radix does not care what family the operand is in). The conclusion it drives, that cross-family answers
belong in D2, is probably right for other reasons. The count should not be quoted as though it were
measured.

### `07`

**Holds.** Section 4.2, the saturating-top question, is the cheapest high-value item in the stretch:
one sentence of canon, 512 of 1024 unsound under the point reading against 0 of 65,536 under the
absorbing reading, identical arithmetic, and the design's own algorithm crates already behaving as
though the answer were given. Section 2.4's refined composition law, arrived at **after** the author's
own prediction over-fired and tested decisively by moving a pivot on and off the coarse grid. Section
4.1's sufficiency condition reducing to `F_acc >= F_elem` with the range half needing no bound, built
as a bound, refusing at type check rather than at monomorphisation, and compared against a real
ten-instruction fold. Section 5.1 restating every result without the frame's vocabulary, and section
5.2 recommending the frame stay out of the canon. That last one is the right instinct and it is already
in the file.

**Thin.** Section 6.5's congruence-times-interval identification is flagged unverified by its own
author, correctly, and section 5.3 names the precedent for why that kind of claim carries a discount
here. Leave it flagged.

**Would push, hard.** Section 4.3, the Moore completion. `07` says "I am not proposing it" and then
hands it a number, and MORNING carries the number into op's summary as a headline. Look at what is being
bought: a third family of segmented shapes, none named by either family, 16 to 34 percent enlargement,
in order to close a join that `06` proved no operation computes and that `E0119` says has no expressible
form as a projection. Then `08:70-75` shows the completion does not even reach the tapers. This is the
direction most likely to eat a week and produce nothing, and it currently sits on the map with an
attractive number beside it and no caller behind it. Mark it as such rather than leaving it neutral.

### `08`

**Holds, and it did the thing three files said was owed.** `p3_segmented_typestate.rs` carries a
canonical exponent as a type-level list, computes both lattice operations as associated types, refuses
a false ordering at type check, and is gate-free. I opened `08_probes/p3_asm.out`: twenty-two lines,
one symbol with a body, that body a single `ret`, six aliases folded onto it including the unguarded
baseline. Three files named this probe as owed and none wrote it.

Section 7's route-closing is the best-disciplined in the stretch, including getting a real finding out
of its own wrong test: `i1` tested for phase zero, which is Flocq's `generic_format`, and called a
half-unit-biased format outside; the correction establishes that the design's concept is **wider** than
`generic_format` in the phase coordinate and **narrower** in the exponent coordinate. `08:633-634` says
it would not have found the first half without running the wrong test. Keep that in the trail.

Section 3.5 is the strongest support the founding premise gets: no representation in the survey needs a
second format concept, arrived at by looking for a counterexample and not finding one.

**Thin, and it matters for how the erasure claim is read.** `08`'s erasure evidence is weaker than
`07`'s and MORNING presents them at equal weight. A function whose body is a bare `ret` erases because
it is the identity; it demonstrates the type-level machinery costs nothing and says nothing about a real
conversion body. `07`'s `p6` compares a guarded fold against a real ten-instruction baseline, which is
the stronger form. `08:692` marks its own as "an existence claim about erasure, not a measurement",
correctly.

**Would push.** Section 4.1 lists "the design keeps carrying a ratified name, `canonical_exponent`, for
a concept nothing implements" as one of three things excluding the function space costs. It is a good
catch as a fact and it is not an argument. A ratified naming call for a concept the design chose to
instantiate at two points is not a debt. Drop it from the cost list; keep it as the observation it is.

## Is this stretch aimed at anything

Partly, and better than I expected going in. I will be specific in both directions.

**The order theory is earning its keep more than the volume suggests.** Three of the four consumer-facing
items the stretch produced came out of it: the tightness sentence being false, the saturating-top
question, and the fold sufficiency check. `07` itself recommends the vocabulary stays out of the canon
and restates every result without it. That is the correct disposition and it is already written.

**Where it is not aimed.** The erasure gate has four clauses at `SETTLED.md:65-71`: the consumer
expresses usage in bits and bytes, the typestate derives container and representation, it validates,
and it erases on lowering, all four at once. The stretch exercised clauses three and four well, in
`06` p3 and p7, `07` p6, and `08` p3. It did not touch clauses one and two at all, and clauses one and
two are the fresh-eyes task. So the honest statement is not that the gate was ignored. It is that the
half of it that already has evidence got more evidence, and the half that has none and is op's standing
instruction got none.

**Note for the dispatcher.** The brief I was handed says "Almost nothing this stretch touched" the
erasure gate. That is wrong on the record: three of the four files ran an assembly read and reported
erasure. Getting that wrong matters, because it points a checkpoint at the wrong absence.

**The workload contact, which is one sentence long in four files.** `08:292-295` notices that
"bitpacked column storage at scale is the workload arvo exists for, and frame-of-reference and delta
encodings are what that workload actually uses", classifies them outside on the locus clause, and gives
them one paragraph at 4.7 concluding the design owes such a consumer a sentence.

The locus argument is sound: no per-datum type can express a constraint holding between data. The
exclusion is right and I am not arguing with it. What I would put in front of op is the shape of the
result rather than the result: **the numeral concept is being made total and beautiful one layer above
where the primary consumer's saved bits actually come from.** That is not a fault in this canon file.
It is a named gap in the map, and per `a-homeless-document-is-a-design-problem.md` a piece with no home
is a design finding rather than a filing problem. Right now the storage-composition question is on
nobody's list.

## What I would refuse

Four, stated as the persona's and carrying no weight beyond argument.

**Refuse `08`'s question one going to op in its current form.** "Is the canonical exponent a member of
the design, or are its two values?" is a cost question with no cost measured. `08:540-550` names the
deciding measurement itself, the compile-time cost of a type-level canonical exponent list at realistic
exponent spans against the droplist's quadratic curve past 4096 rows, and says it did not take it. Op's
own words at `01:96-98` cover this exactly:

> At some point, somebody has to be confident enough about their take on it to write the benches, and
> once benches exist, it's hard to deny what they tell, and if it isn't, then the thing is still
> settling and there's no reason to rule on it

Somebody writes that arm before the question is his. **Guessing** on the reaction: I think he would be
irritated to be handed the question without the number, having said this three weeks running.

**Refuse the Moore completion as a live direction.** Reasons in the `07` section above. It should sit
on the map marked "closes an operation with no caller, and does not reach the tapers", not neutral with
a percentage beside it.

**Refuse the `canonical_exponent` naming debt as an argument.** Reasons in the `08` section above.

**Refuse letting the D0/D1-equals-fibre/index claim harden.** It is genuinely interesting and it is one
expert's, self-flagged, and MORNING has already promoted it once. If it is worth having it is worth a
second read done properly, which means the next member derives its own site classification before
reading `06` and `07`, then reconciles.

## What is genuinely good, specifically

`RULES.md:99-101` makes keeping something a result, and a checkpoint that only attacks is worth nothing,
so this is not padding.

The discipline held. Every number in four files came from a named command in a committed probe
directory. I opened four of those outputs at random and every one matched its citation. All four
members re-ran the counts rather than inheriting them. All four kept broken instruments rather than
deleting them, and two of the four got real findings out of the failures. All four stated coverage
bounds and named what they did not cover. Not one claimed to have run a suite it did not run.

Measured against the predecessor's failure modes as `RULES.md` records them, that is a clean stretch.
The predecessor propagated two floating numbers nobody could reproduce and had one expert's headline
counts turn out to be an artifact of its own enumeration bound. This stretch reproduced the second
failure exactly once, in `08`'s `i2b`, and `08` found it itself, named it as the failure `RULES.md`
warns about by name, superseded three of its own instruments, and kept the counts that died.

The single best individual result is `06`'s `E0119`, for the reason in the `06` section. The single
best unasked result is `06` section 7.1, catching that a canon sentence would be false and then
building the correction. The cheapest high-value open item is `07` section 4.2.

## The process finding

Stated separately because it is about how the panel is run rather than about what it found.

The TWO EXPERTS rung is currently unreachable by construction. `RULES.md:163` mandates cumulative
sequential dispatch with each member reading its predecessors in full; `RULES.md:28-30` defines the
rung as requiring derivation before reading. Every member has noticed and flagged this about itself.
None has said the shape guarantees it.

The workspace already has the mechanism, in `expert-dispatch-defends-the-canon.md`: the second expert
forms and writes down its own reading before reading the first, then reconciles. Applying it costs one
dispatch shape change, not a rule change. Dispatch a member against a **named claim** with the
predecessor withheld, have it write its answer to disk, then hand it the predecessor and have it
reconcile in the same file.

Two claims are worth spending that on before anything else: the D0/D1 enumeration and its
exhaustiveness, and `07` section 4.1's sufficiency inequality with its claim that the range half needs
no bound at all. Both are load-bearing, both are one expert's, and both would be expensive to discover
wrong later.

Second process note. Four files each spent a section reporting that the test gate is vacuous because
the tree is nuked and this surface has no source. That is diligence once and repetition three times.
After this checkpoint the gate section can be one line citing `03` section 0.2, with a fresh count only
if the member touches a surface the earlier ones did not.

## What I would drop

**The Moore completion**, per above. Not deleted, marked.

**The `canonical_exponent` naming debt** as a cost argument, per above.

**`03`'s reading F, the ambient-and-realisation decomposition** (`03:654-674`). `03` says plainly it is
not a fourth answer, that it relocates the question rather than resolving it, and that a reader who took
it for an answer would find the same call one layer down. It has since been superseded anyway: `07`
section 3.2 and `08` section 6 both say the same thing with content attached. Carrying it forward as a
live option costs a reader a concept and buys nothing.

**The 81-versus-zero discrepancy** should be dropped from the open list or promoted to a task, not left
where it is. `SETTLED.md:144-146` says it "will poison a consolidation that quotes either number", and
`03:734-736` says it is untouched. A known poison sitting untouched through four dispatches while a
consolidation is coming is the wrong resting state. Either someone builds the third instrument `150`
says is owed, or the panel commits to quoting neither number.

## Where I am guessing

Marked per the brief, collected here so op can discount them in one place.

I am **guessing** that op would be irritated by `08`'s question one arriving without the compile-time
measurement. The inference is from `01:96-98` and it is an inference about his reaction, not a reading
of his words.

I am **guessing** that the Moore completion is the direction most likely to consume a dispatch and
produce nothing. That is a judgement about where effort goes, built on `06`'s `E0119` and `08`'s slope
argument. Both underlying facts are measured; the prediction is mine.

I am **guessing** that the storage-composition gap is worth putting on the map now rather than after the
numeral canon settles. The locus argument that excludes it is sound and I am not disputing it. What I am
asserting is a priority, which is op's to set.

I am **guessing** that `03`'s 31 unclassified cross-radix pairs are box artifacts, on the same one-line
nesting argument `03` gives and declines to lean on. It should be measured before reading E rests on
5.2.

Everything else above is a reading of a file or of a probe output, with the line cited.

## Coverage, and what this checkpoint did not do

I did not read `CANON_CANDIDATE.md`, `DROPLIST.md`, `02_carried`, `seed/`, or the predecessor's tree.
So where a file claims something is absent from the record, I checked whether the file's own citation
supports it and did not independently confirm the absence.

I opened four probe outputs: `06_probes/p4_arm3.out`, `06_probes/p4_arm2.out`, `08_probes/p3_asm.out`,
and the probe directory listings for all four members to confirm the evidence is committed. It is;
every instrument cited above exists in the panel tree. **I did not re-run any instrument** and I did not
verify any Python enumeration's arithmetic. Every count I quote is a count I read in a `.out` file or in
the file citing it.

I did not check `MORNING.md`'s citation set against the four files by diff, which
`a-compression-is-checked-by-someone-else.md` asks for. I read it line by line and found the four
defects above; a set diff of `file:line` anchors would likely find more, and that check is cheap and is
owed.

**Nothing here is priced.** No bench harness run bears on anything in this file or in the four it
checks, and I am using that word as `RULES.md:119-122` reserves it.

**Nothing here settles.** Per `04`, including the corrections to `MORNING.md`, which are a persona's
reading of what four files say about themselves and should be checked against those files before either
is edited.
