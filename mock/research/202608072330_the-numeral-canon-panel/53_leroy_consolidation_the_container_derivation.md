# 53. Consolidation: the container derivation's outputs

**Date:** 2026-08-09. **Persona:** Xavier Leroy. **Mode:** explore, do not settle (`00_brief.md`,
`04`, `28`). **Position in the unit:** ninth and last file, the consolidation of `44` through `52`,
and the first consolidation this panel has produced.

**Status: COMPLETE.** Written to disk early and extended in place, per `RULES.md:328-329`.

**What this file is.** The standalone statement of one topic: how many outputs does the container
derivation have, and what are they. It is versioned and complete in itself; a section whose content
did not change across the unit is written out anyway, and a later reader should not need the eight
member files to use it. It is a **canon candidate, not canon**: op's explore-do-not-settle mode is
in force, nothing here ratifies anything, and this file goes to op. Where the unit disagreed, the
disagreement is stated with what would settle it. Every claim carries its rung, and the rungs below
were re-derived from the member files rather than inherited from any account of them, because this
unit's rung was inflated twice before this file was written, once by a member (`44:380-383` against
its own report at `44:72-79`) and once by a dispatching brief (`48:49-79`).

A note on my own position. I wrote `17` in this panel. Two artifacts of mine are load-bearing in
this unit: `17_probes/t2_aggregate_erasure.rs`, which `51` found after a brief claimed no such arm
existed, and `17`'s classification of counts as the panel's most fragile claim class
(`17:678-684`), which section 5 below applies to a contested count. I flag the self-citation so a
reader can weigh it; both artifacts were verified by others (`51:93-117` reproduced t2's assembly
byte for byte; the count discipline is applied here to a dispute I measured myself, with commands).

## 0. Gates

**Canon gate: passes.** There is no ratified canon to defend or diverge from; `mock/canon/` does not
exist on this branch. The fixed material is `01`, `04`, `28`, `INTENTS.md`, the workspace
discipline, the forbidden-feature list, and the acceptance criterion at `00_brief.md:144-146`, which
now reads, in op's restored plural, "the typestate derives the matching container and numeral
**representations**" (drift found by `44:150-177`, restoration recorded in the brief itself at
`00_brief.md:148-153`). This topic is the second noun of that criterion, so the work sits inside the
fixed material. Nothing below proposes anything the forbidden-feature list excludes, and nothing
below settles: the deliverable is candidate sentences with rungs, for op.

**Test gate: nothing to run.** `mock/crates` is empty by construction; there is no suite. The unit's
evidence is committed probes, and this file's own verification acts are in section 1. I applied the
gate's failure kinds while reading: the unit itself caught and recorded one tautological check
(`45_probes/p4`, caught by `46:171-201`, conceded and replaced at `45:591-643`), one
could-not-have-failed harness (`51`'s first matrix, self-caught at `51:209-244`), and one sampled
law (the single-width erasure evidence, `51:284-296`). All three are named in the body below because
they are part of what the unit established about its own instruments.

## 1. What I verified myself, before consolidating

A consolidation is a compression, and its author is the wrong person to check it
(`RULES.md:300-319`), so the entailment pass belongs to someone else. What I can do, and did, is
open every load-bearing citation and re-derive the contested numbers rather than inherit them.

Opened end to end: `44`, `45` (including its reply sections 11 and 12), `46`, `47`, `48`, `49`
(both phases), `50`, `51`, `52`, `INTENTS.md`, `00_brief.md`, `RULES.md`. Opened at the cited
passages: `16:15-33`, `16:95-101`, `16:126-152`, `16:185-189`, `16:272-282`, `16:565-580`,
`16:605-621`, `16:735-745`, `15:315-320`, `15:341-356`, `15:415-430`, `15:550-558`, `17:1-40`,
`17:676-690`, `45_probes/p1_wide_rung_collision.rs:1-5`, `47_probes/p1_single_type_output.rs:1-20`,
`OPTIONS.md` lines 690 through 1078 (the "derivation's outputs" section and its appended
blockquotes; cited below by section name and grep-verified phrase only, per my brief).

Re-derived myself, with the command stated, because a contested count must carry its domain
(`17:682-684`):

```
$ grep -c 'generic parameters may not be used' 16_probes/p5b_const_to_type.err \
    47_probes/p2_scalar_single_output_refused.err 47_probes/p3_access_type_from_const_refused.err \
    50_probes/p5b_negctl_three_facts.err
16_probes/p5b_const_to_type.err:4
47_probes/p2_scalar_single_output_refused.err:6
47_probes/p3_access_type_from_const_refused.err:3
50_probes/p5b_negctl_three_facts.err:3
$ grep -cE '^error' <same four files>
5, 11, 4, 6
```

Section 5 uses these. Also verified: `49`'s phase one is committed at `2430fad7` ("cold derivation
phase one", 228 lines plus probes) with phase two following at `69bba205`, so the cold instance's
ordering claim is real and checkable in the history. The panel's working tree is clean at this
writing: the `46` recovery (`50:96-123`, commit `7a3bddd`) and the `45_probes/p7` rustfmt delta
(`48:289-303`) are both resolved in the record.

Not verified by me: I re-ran no probe. `52` re-ran all eight of `50`'s and reports byte-for-byte
reproduction (`52:88-101`); `51` reproduced `17`'s t2 assembly byte for byte (`51:106-117`); I rely
on both, and say so where it matters.

## 2. The topic, and the form its answer actually has

The topic was dispatched as an arity question: how many outputs, and what are they. The unit's
single most consequential result is that **the arity form of the question is not the one with a
stable answer**, and this was established three separate ways.

`16` itself said it first, before the unit opened: "'How many outputs' needs a criterion for what
counts as an output, or the answer is unfalsifiable: everything downstream is a function of the
declaration, so you can always claim one output and call the rest recomputation" (`16:95-97`). `47`
showed the count is not even well-posed as packaging: any product is one thing, so "two outputs"
versus "one richer output" was never a fork, and the richer single output works exactly when it is a
type with projections, at which point it is the pair wearing one name (`47:83-186`). And `50`
formalised the criterion `16` wrote to make the count decidable, found it does not decide, and `52`
verified the refutation at source (section 6 below).

So this consolidation states the topic's answer in the form that survived: **what must the
derivation's result make available, in what form, and why**, with the count as a consequence rather
than a claim. That reframing is not evasion. Everything the unit established positively is below,
with rungs, and the count question gets the only honest answer it has: it is a function of a
strategy set op has ruled open (`INTENTS.md` I1).

## 3. What is established, finding by finding

### 3.1 More than one fact is forced, unconditionally, by the packing strategy alone

**The claim.** A derivation whose entire result is the standalone machine type destroys, at the
moment it returns, information the design exists to carry. Restricted to unsigned `Cold` at widths
nine through sixteen, all eight declarations map to the same sixteen-bit native container while
their memory layouts in an aggregate are eight different things (`16:126-141`). The failure is
silent and structural: a carrier-only derivation of a thirteen-bit packed value occupies 23.1% more
than the strategy promises, and the panel's own erasure-and-codegen instrument passes it at full
marks because that instrument is per-value by construction (`16` sections 5 and 7; reproduced and
sharpened in `17` section 0, whose stronger reading is that the instrument's green is not merely
uninformative but asserts the `Hot` and `Cold` numerals are the same function).

**What it needs as premises.** Only `Cold`'s stated packing intent (`INTENTS.md` I6, op's words) and
the fact that the implementation language has no native type of arbitrary bit width. It does not
need `Precise`, whose earlier appearance in this claim's framing was corrected during the unit: the
register's "blocked on the `Precise` strategy's undecided semantics" framing was wrong about the
two-versus-one question, which `Cold` alone closes (`45:70-89`, confirmed by `46:50-78`).

**Rung.** The **content** (which facts, what the second is keyed on, what fails without it, which
check is blind) is TWO EXPERTS: `15` and `16` derived it independently by different routes, `15` by
fixing two compiled stride defects (`15:341-354`), `16` by the injectivity argument, and `16`
declared its one contamination precisely, a commit subject that leaked the number two before it
derived anything, so its agreement **on the count** is self-downgraded to worth nothing while the
content predates its reading of `15` (`16:15-33`). The **count as a number** is therefore ONE
EXPERT with a declared leak, and section 6 shows the count is additionally criterion-relative, so
this consolidation does not carry it as a claim at all. On top of the two derivations sit four
reads that found no defect in the argument (`44`, `46`, `47`, `52`), which is worth something and is
not a rung, and **one cold instance at a coarser grain**: `49`, reading only op's intents and the
workspace rules, derived blind that a per-aggregate composition fact must stand beside the storage
fact (its "packing model", conceded on reading to be a weaker spelling of stride: `49:77-93`,
`49:250-258`). The cold instance supports "more than one fact, and the second is aggregate-keyed";
it does not support "exactly two", since `49`'s blind count was six, corrected on reconciliation.

### 3.2 The two facts, named, and what the second is keyed on

**Under the site model the whole unit assumes** (stated as a premise in section 7, because no file
had written it down), the facts the derivation must carry are:

**The carrier.** The machine type an operation lowers to: what a register holds, what an add
operates on (`16:148-150`). It must be **a type, not a bit count**, for two independently compiled
reasons: alignment is a property of a type and rides on it (`16_probes/p7`, `16:605-613`, with
`15`'s residual doubt at `15:553-556` closed by that construction), and a bit-count carrier is
structurally blind to same-size-different-alignment distinctions (`45_probes/p2` finds zero
collisions on a domain where the type-aware instrument finds forty, `45:109-143`).

**The stride.** The bit distance at which elements of an aggregate repeat. For the packing strategy
it is the declared width; otherwise it follows the carrier's size, not the value's rounded-up
bytes. That general keying was reached by both `15` (as a bug fix: `15:345-349`) and `16` (deriving
stride from the carrier's size directly, a different route to the same repair): TWO EXPERTS. The
**wide-rung half** of the keying claim, that above 128 bits the stride belongs to the
strategy-and-rung pair because one strategy pads to align 16, is **ONE EXPERT**: it is `15`'s
compiled result with two negative controls, `16` explicitly disclaims having built the apparatus to
test it (`16:739-742`), and the register's earlier citation of `16` as confirming it was a
misattribution found by `44` (`44:197-256`) and corrected. It remains ONE EXPERT at this writing.

**The best statement of why there are two**, kept from `16` section 12 and carried in the register:
the derivation answers **a per-value question and a per-aggregate question** (`16:691`, phrase
verified in the register's "derivation's outputs" section). `Cold` is the proof that these are
different questions: a lone thirteen-bit value is a `u16` whatever strategy is asked for
(`15:315-320`), and a lone packed value has to have a size, so packing cannot be a statement about
the standalone type at all. TWO EXPERTS, reached from opposite ends and self-reported as such,
verified by `44:389-394` reading both texts; and strengthened by an attack that failed: `49`'s cold
derivation gave `Cold` a divergent standalone storage type, read the panel, and conceded that its
own divergence bought nothing a lone value can use (`49:270-284`).

### 3.3 Packaging was never the fork; the kind boundary is the wall

`47` built both spellings of the "one richer output" alternative. A single **type-valued** output,
with carrier, stride and access width as projections, compiles gate-free and repairs the collapse:
the eight colliding `Cold` widths get eight distinct outputs, with a negative control refusing
three false type equalities (`47_probes/p1`, `p1b`, header opened at
`47_probes/p1_single_type_output.rs:1-20`). A single **value-valued** output, a lossless const
encoding of the same information, is compiled-refused six times across three syntactic positions,
each naming the forbidden `generic_const_exprs` (`47_probes/p2`). The refusal is about direction,
not a broken encoding: the positive control shows type-to-const projection total and free while
const-to-type is refused (`47_probes/p2b`, `47:157-170`).

**So the wall that forces the result's shape was never information loss. It is the kind boundary:**
a compile-time derivation hands a site things of two sorts, and a fact a site needs as a type must
be carried as a type, because the road from computed values back to types is closed under the
forbidden-feature list. This is the best-established mechanism claim in the unit: three authors,
four probe files, four starting points (`16_probes/p5b` trying to recover the carrier from the
stride; `47_probes/p2` from a lossless const superset; `47_probes/p3` from the access width;
`50_probes/p5b` from the stride toward the at-rest type, built as an attack expecting the route to
work, `50:627-633`). One shared assumption bounds it: every one of the four carries the width as a
const at the point of refusal, which is `10`'s bridge problem, filed by `44` section 5 as sitting
underneath this topic and by `47` section 8 as a precondition of one of its verdicts. That is where
the wall should be attacked next, and nobody has.

**Delivery form has a codegen consequence, and it lands on the same side.** `51` swept the erasure
question across 36 widths and found the panel's evidence had been a sampled law: the typestate
packed walk matches its hand-written twin at every width at or below seventeen and stops matching
at eighteen and above, and the failure is a collapse to one accumulator and one element per
iteration, so the typed arm emits fewer instructions and is worse code (`51:284-339`). The collapse
is a conjunction, isolated by a control: the gather written as a loop over an associated const, and
width at least eighteen (`51:416-445`). Two repairs land without changing a character of what a
consumer writes; the one that matters here delivers the access operation as a contract with a flat
impl per access width and erases at **36 of 36** widths (`51:454-479`). ONE EXPERT, one host, one
toolchain, counts read off emitted assembly, nothing timed: what the collapse costs is unpriced,
and `51` says so throughout. Its bearing on this topic is qualitative and real: a fact delivered
as a const that a generic body loops over is not the same design as the same fact delivered as a
contract, and the "two outputs" framing does not distinguish them.

### 3.4 The contingent further fact, and what actually turns on `Precise`

If any strategy diverges what an operation computes in from what a value occupies at rest, the
result must carry a further fact, and under that divergence the pair is not merely irreducible but
**insufficient**: two strategies at the same width can share carrier and stride and differ in
compute form, so the pair stops separating declarations that behave differently (`47:254-292`,
built at `47_probes/p5` with the equality as a must-not-refuse control). `48` restated this without
`47`'s modelling choice (`48:207-244`), and `50` checked both assignments of the one carrier slot
and found the conclusion holds under both, with the correct reason being the kind boundary again:
under the compute-type assignment the pair does separate, and what becomes unreachable is the
at-rest type, across a refused const-to-type step (`50:598-633`). Two builders, one checker;
established mechanically.

**The mechanism is free either way.** A third associated fact compiles gate-free under both
readings of `Precise`, switching on one impl block (`45_probes/p5`, confirmed by `46:226-232`,
rebuilt in two-slot and three-slot forms by `50_probes/p5`). So nothing about this waits on the
type system. It waits on intent, and the unit sharpened what the intent question actually is,
twice:

First, the widening question has a proof attached. If `Precise` means matching the exact
once-rounded chain answer for every representable input, then a wider intermediate (or an
information-equivalent mechanism) is forced by pigeonhole: no fixed-width per-step rounding rule
closes the gap, established exhaustively at four fraction widths by two independently coded
instruments cross-checking to identical counts (`45:180-243`), with the vacuous half of the
original evidence caught by `46` and replaced by a real finite-headroom model whose result is that
most witnesses need one extra bit and a growing tail needs the full doubling (`45:591-643`). The
refuse-on-inexact alternative is not a fork on this axis: per-step and end-of-chain refusal admit
exactly the same chains once zero operands are excluded (theorem, within its stated domain of
total width equal to fraction width), and refusal admits vanishingly few chains at all, which no
reading of "accurate within chains" (I7) survives (`50:519-574`, algebra independently re-derived
by `52:212-225`).

Second, `50` found a reading the whole unit had excluded without noticing: op's own I2 says
`Precise` is most precise "at the price of both storage and compute" (`INTENTS.md` I2), and every
model in the unit assumed its storage equals `Warm`'s (`50:576-594`). Under a wider-at-rest
reading, the insufficiency witness vanishes. One reader; op's to answer; and it changes the shape
of the question before it is put to him, which is why the unit's standing instruction (`48:530-537`,
reaffirmed by events) was to reshape the question first and ask once.

**The question for op, in its post-unit shape** (superseding `45:454-459`'s two-alternative form):
does the wide value every fixed-point multiply forms internally get **carried between operations**
under `Precise`, and does `Precise` pay its I2 storage price at rest, or only in compute? One
answer decides whether the derivation carries a compute-form fact, and section 3.3's mechanism
takes either answer at the cost of one impl.

### 3.5 What is recomputed rather than carried

**Alignment** is not a separate fact: it is a property of the carrier type, recoverable by a
language primitive, established by an adversarial construction of two wide payloads with identical
size and stride at align 1 and align 16 (`16_probes/p7`, `16:605-613`), independently recognised by
`47:296-302` and conceded to by `49`'s reconciliation after its cold derivation had over-counted it
(`49:260-266`). Its consequence is load-bearing: it is a reason the carrier must be a type.

**The packed access width** is a function of width and stride, not a further slot, and the unit
corrected its size twice. `16`'s closed form, `floor((W+6)/8)+1`, is exactly right as a worst case
over all eight bit phases (verified with zero mismatches over widths one to 1024, `50:402-412`).
But a packed run reaches all eight phases only when the stride is odd: the reached set is the
subgroup generated by the stride mod eight, so the closed form over-estimates at 48 of 128 widths
under `Cold`, and `16`'s "carrier is the wrong load type at 28 of 64 widths" becomes 16 of 64
(`50:414-435`, group theory traced against the code by `52:198-210`, with one independent pre-panel
instance of the same period fact committed in the bench harness's own variant source, opened by
both `50` and `52`). Downstream of that correction, `47`'s two-ladder finding **reverses**: from
the worst-case form the native and access partitions share no jump point (`47:344-361`); from the
reached phases the access partition refines the native one, so one ladder keys both at 25 classes
instead of two ladders at five and six (`50:437-449`). `50` files this honestly as a fork rather
than a replacement, since the worst-case form is strategy-independent and the exact form is not.
`47` has not been resumed to answer; the question stands addressed to it at `50:720-726`.

**Whether any site needs the access width as a type at all** remains open (`47:536-541`), and the
adjacent magnitude is now priced: a decode plan derived per element at runtime costs 3.04x to 3.12x
a compile-time plan, stable across four sizes, read off committed harness output with the dense
carrier as competitor arm (`mock/benches/bitpack-decoder-shape`, `50:466-511`, sources opened by
`52`). The caution `50` itself attaches: this prices plan-compile-time against plan-runtime, not
derivation against site.

### 3.6 The fact set is not closed under an open strategy set

A site that re-implements a strategy's rule is wrong the moment a strategy the rule did not
anticipate exists. `50_probes/p3` demonstrates this by construction: a fifth strategy (grid
packing) makes a site-side stride formula silently wrong while agreeing at some widths, the repair
relocates the fact onto the strategy where the derivation still supplies it, and a sixth strategy
(per-element validity padding) breaks the repair by asking a question the schema had no slot for
(`50:274-299`, opened and traced by `52:290-313`). `49` reached the stable shape cold and blind:
the schema of facts is strategy-independent and only the values vary, so a new strategy adds impl
bodies, not fact slots (`49:154-162`, `49:303-314`). Two instances, one cold. The boundary neither
resolves: a strategy that introduces a genuinely **new question**, rather than a new answer, needs
a schema change, and whether that stays cheap as I1 is exercised is open and is not answerable
from six strategies, four of them inherited (`50:770-777`, `52:306-313`).

## 4. The criterion that adjudicated the unit is retired, and what replaces it

### 4.1 The retirement

Every verdict in the unit's first six files was adjudicated against `16:100-101`:

> A component is an output of the derivation when the consumer did not write it, the machine needs
> it, and a downstream site that holds the other components cannot recover it.

`48` (persona, no rung) noticed the sentence is applied two ways inside the file that stated it,
with opposite results (`48:305-345`). `50` formalised it: the defined set appears on both sides of
its own third clause, the operator is antitone (adding a fact removes another), so no least or
greatest fixpoint tie-breaks, and the sentence leaves three background parameters unstated: the
site model, the strategy set, and the kind regime. Enumerated across those parameters, one sentence
admits answers of sizes zero, two, three and four, and the unit's answer is produced by exactly one
cell of sixteen, the cell that reads the strategy set as closed (contradicting op's I1) and the
kind boundary as absent (contradicting the compiled refusals of section 3.3) (`50:132-249`).

`52` second-read this with the order inverted for the one act that needed independence: it read
`16` in full and formed its own reading of the sentence before opening `50` or `48`, reached the
same two-readings finding unaided, then re-ran all eight of `50`'s probes (byte-for-byte
reproduction), hand-traced the solver against the decisive cell, and confirmed the refutation
(`52:51-101`, `52:146-167`). **The refutation holds, at TWO EXPERTS on the reading and one
verified formalisation.** Two corrections from `52`, neither load-bearing, are carried with it:
the equation has at most one solution per fully-specified cell, so the multiplicity comes from the
unstated parameters rather than from simultaneous solutions of one instance (`52:125-144`), and
"non-monotone" is loosely said for antitone.

**What the retirement does not touch.** The two-output finding itself, which rests on the
injectivity argument and the compiled evidence, not on the criterion (`50:679-685`). What is
retired is the belief that `16:100-101` is what established it.

### 4.2 The replacement, in three clauses, with the provenance of each

The sentence was trying to be three predicates at once. Separated (`50` section 3, endorsed and
restated as a specification by `52` section 7):

**A site premise, stated rather than inferred.** A lowering site holds the numeral's full type:
the declaration (width, sign, strategy), the derivation's result, and the language's primitives.
This is every file's unstated practice (`45:314-333` establishes nobody proposes dropping the
strategy early; `46:243-251` confirms the acceptance criterion places erasure at codegen), and
writing it down is what removes the criterion's two-readings ambiguity by fiat. It is a design
commitment for op to bless, not a finding.

**An ownership clause, fixing the fact set.** A fact belongs in the derivation's result exactly
when producing it requires applying a rule the strategy owns; a pure function of what every site
already holds is recomputed, not carried. Provenance, stated exactly because this is the unit's
strongest convergence and also its most inflatable claim: the clause exists in instance form
inside `16` itself, in the passage that does not use the criterion ("emitting the extent and
recomputing the carrier at each use would re-enter, at every use site, the problem the derivation
exists to solve once", `16:280-282`); `49` derived the general form **cold**, before reading any
panel file ("getting its value requires consulting the strategy as an actual decision, not a
formula... cannot safely re-derive it without risking disagreement", `49:96-101`, committed at
`2430fad7` before its phase two); `50` formalised it having read the unit; `52` endorsed it and
checked its boundary probe. So: one cold independent derivation, one in-file precedent in
instance form, one formalisation, one second read. I do not call that three independent
derivations of one principle, and neither does `52`, whose own risk section marks "the three
formulations are the same principle" as the part of its verdict most likely to move
(`52:402-408`). What is safe to say: two independent statements of the ownership idea exist
(`16:280-282` in instance form, `49` cold in general form), and the formalisation that makes it
decidable is `50`'s, verified once.

**A kind clause, fixing the form.** Whatever the ownership clause selects is carried in the sort
its consuming site uses it in: as a type where a generic body would otherwise need to reach a type
from computed values (refused; section 3.3), as a constant otherwise. This is `47`'s sentence
("the derivation's result must make available, as types, every fact a lowering site cannot
recompute from a const", `47:505-510`), which `48` judged true and insufficient as the whole
criterion (`48:388-425`) and `50` relocated as exactly one clause of three (`50:374-391`). Both
judgements are right and neither diminishes it.

**The count is then a consequence, not a clause.** As many facts as there are rules a strategy
owns, in the form each consuming site needs; the number moves when the strategy set moves, and op
has ruled the strategy set open (I1). A canon that fixes an arity here contradicts I1 in a place
nobody would look for the contradiction (`50:394-400`).

## 5. The contested count, resolved by stating domains

The unit's account of the kind-boundary refusals carried three different numbers: twelve (`50`'s
prose arithmetic), thirteen (`52`'s recompilation), and twenty (a regex count recorded in the
register's account of `52`). This is the failure mode `17` named as making counts the panel's most
fragile claim class: never the arithmetic, always an unstated domain (`17:678-684`). Measured
myself over the committed `.err` files, with the commands in section 1:

| domain | count |
|---|---|
| diagnostic instances of "generic parameters may not be used", three files `52` compiled (`16_probes/p5b`, `47_probes/p2`, `47_probes/p3`) | 13 |
| the same instances, adding `50_probes/p5b` | 16 |
| raw `error` headers, the three files | 20 |
| raw `error` headers, all four files | 26 |

All four numbers are correct about their domains. The claim that survives every domain, and the
only one a canon sentence should rest on: **the const-to-type direction is refused by the compiler
in every probe that attempted it, across four probe files by three authors from four starting
points, each refusal naming the forbidden feature.** A count of refusals may be quoted only with
its domain in the same sentence.

## 6. Disagreements: located, resolved, or standing

The unit's characteristic motion, named by `48:246-260` and borne out through file `52`, was that
no member was wrong about a fact; every dispute was about **which question a compiled result
answers**, and each round moved a result one shelf over. The ledger:

**Resolved inside the unit.** The wide-rung alignment forcing: `45` claimed it unconditional, `46`
downgraded it to conditional on an unratified alignment axis (the dead tree's align-16 rule is an
assumption by its own source's word, `15:418-429`, `15:553-556`), `45` conceded and built the
abstract lemma (`45_probes/p7`: the mechanism is unconditional, the instantiation is not), and
`47` relocated it once more: the collision's own probe states its subject as whether the
width-and-stride **pair** determines the carrier (`45_probes/p1_wide_rung_collision.rs:1-2`,
opened), which is a reducibility question about a key nobody proposes, not a sufficiency question
about the derivation's outputs (`47:188-252`). Final state, all parties on record: a real, general,
conditional mechanism, filed under reducibility, waiting on an axis only op can settle. The
tautological `p4` check and its replacement (`46` attack, `45` concession and `p6`) are likewise
closed.

**Corrected, with the corrected party not yet resumed.** `47`'s two-ladder cost (reversed by
`50`'s phase-set correction into a fork; question standing at `50:720-726`). `47`'s section 4
reasoning (conclusion confirmed under both models by `50`, for a different reason than `47` gave;
question standing at `50:724-726`). `48`'s Reading B count of one (corrected by `50:251-261` to
zero, or a two-member set that is not the unit's two, once the kind boundary is honoured; `48` is
a persona and the correction stands unanswered). None of these is a live factual dispute; each is
an unanswered exchange, and resuming `47` is the cheapest way to close two of them.

**Standing, and precisely bounded.** Whether `47`'s kind sentence, `49`'s cold clause and `50`'s
ownership clause are one principle in three vocabularies or two principles (a fact-set clause and
a form clause) that compose: `52` marks this as its weakest link, section 4.2 above carries it
disaggregated for that reason, and a second reader deriving the relationship before reading `52`'s
section 6 would settle it. And the wide-rung stride keying remains ONE EXPERT until someone builds
the wide rung `16` declined to build.

## 7. Candidate canon sentences

Each sentence is tested against permanence (true and useful after a from-scratch rewrite) and
equivalence (three independent implementations behave the same), per `RULES.md:79-83`, and carries
what it rests on. These are candidates for op, not rulings. Vocabulary note for all of them:
"carrier", "stride", "container", "numeral representation" are working names; whether the nouns
themselves are canon vocabulary has never been put to op, and I1's demotion of the strategy names
shows the intent and its era's vocabulary can part ways (`48:544-551` raised this; it is op's).

**S1, the site premise.** *A numeral's declaration (width, sign, strategy) is part of its type for
the whole of its typed existence; erasure happens at lowering and not before. A lowering site
therefore holds the declaration, the derivation's result, and the language's own primitives.*
Permanence: passes; it names no mechanism. Equivalence: passes; it fixes the observation surface
every other sentence quantifies over, and without it the unit's own criterion was readable two
ways. Rests on: universal unstated practice, named as a premise by `50` section 3.1, anchored in
the acceptance criterion's "erase on lowering". Status: a design commitment awaiting op's blessing.

**S2, the ownership clause.** *The derivation is where a strategy's rules are applied, once. Its
result carries every fact whose production applies a rule the strategy owns; a fact that is a pure
function of what every site already holds is recomputed at the site, not carried.* Permanence:
passes. Equivalence: the strongest evidence in the unit is exactly an equivalence trial, one
implementer deriving the clause cold and converging with the panel's formalisation (`49`, `50`).
Rests on: section 4.2's provenance, honestly disaggregated. The known boundary: a strategy
introducing a new question rather than a new answer changes the schema, and the clause's
decidability there is open.

**S3, the two standing questions.** *For every strategy so far conceived, the derivation answers
at least a per-value question and a per-aggregate question: what one value is for the machine, and
how a run of them repeats. These are different questions, and any strategy entitled to make their
answers differ forces both to be carried; the packing strategy is a statement about how a run
composes, never about the standalone value.* Permanence: passes; no language named. Equivalence:
passes on the unit's evidence (two independent derivations plus one cold instance at this grain).
Rests on: sections 3.1 and 3.2. TWO EXPERTS on content; the phrase is `16` section 12's, kept.

**S4, the form clause.** *A compile-time derivation hands a site things of two sorts, values and
types. Projection from a type to its values is total and free; construction of a type from
computed values is outside the contract. A fact a site consumes as a type is therefore carried as
a type.* Permanence: contested and carried honestly: `48` objects that sort vocabulary is one
language family's (`48:410-419`), `47` answers that the boundary exists wherever types and values
are different sorts (`47:451-457`); the establishing evidence is compiled and Rust-specific, the
intent is general, and a rewrite into a language without the boundary would need this sentence
re-derived, which is exactly what a canon's pointer to evidence is for. Equivalence: passes.
Rests on: section 3.3, four probe files, three authors, plus `51`'s codegen result showing the
form choice is observable even where both forms compile.

**S5, the count.** *The canon fixes no output count. The fact set is stated as questions
strategies answer, never as answers enumerated over the strategies that exist today, and its size
is a consequence of a strategy set the design holds open.* Permanence and equivalence: pass by
construction; this sentence exists to keep both passing as I1 is exercised. Rests on: sections
3.6 and 4.2, and op's I1.

**S6, the contingent compute form.** *A strategy entitled to diverge what an operation computes in
from what a value occupies at rest forces a further carried fact, and under such a divergence the
per-value and per-aggregate pair no longer separates declarations that behave differently. Whether
any strategy in the shipped set makes this divergence is an intent decision; the mechanism takes
either answer.* Permanence: passes. Equivalence: passes; the mechanism was built under both
readings by two authors. Rests on: section 3.4. The antecedent is op's (`Precise`), and the
sentence is deliberately not conditioned on `Precise` by name, per I1.

What is deliberately **not** offered as a sentence: `51`'s delivery-form result (one host,
unpriced; it is evidence under S4, and its "access operation as a contract" slot is a live option
in the register, not settled); the access-width closed form and ladder fork (a derived quantity
with a standing fork, section 3.5); and any sentence containing a count of refusals without its
domain (section 5).

## 8. Doability, established, with the evidence

The canon must say which things are doable (`RULES.md:85-87`). Established in this unit, all
gate-free on the pinned `nightly-2026-05-28`, all committed:

The whole result as a trait schema with per-strategy impls, in two-fact, three-fact, and
single-type-with-projections spellings (`16_probes/p6`, `45_probes/p5`, `47_probes/p1`,
`49_probes/p1`, `50_probes/p5`: five builds, five authors, same shape, one of them cold).
Validation as a plain missing-impl refusal, `E0277`, with no separate mechanism (`49_probes/p2`,
the one clause of the acceptance criterion the unit had not explicitly discharged; ONE EXPERT,
uncontested). Per-value erasure to bare masks and moves (`15`'s q12, `17`'s t2 scalar arm,
`49_probes/p3`). Per-aggregate erasure at every width in a 36-width matrix, under contract-shaped
delivery of the access operation (`51`, with the loop-shaped delivery failing at half the widths;
one host). The contingent third fact under both readings of `Precise` at the cost of one impl
block (`45_probes/p5`, `50_probes/p5`). And the negative doability results that bound the space:
const-to-type refused from four starting points (section 3.3), and the value-valued single output
refused outright (`47_probes/p2`).

## 9. Open items, and whose they are

**Op's.** The `Precise` question, in section 3.4's post-unit shape (carried wide product, and the
I2 at-rest price), noting that `44`, `45`, `47` and `48` each called some form of it cheap and
nobody asked, and that the unit's reshaping is now done, so asking is finally the right act.
Whether alignment is a strategy property or an axis of its own (`15:553-556` named it, three files
spent effort on its conditional; only intent settles it). Whether the acceptance criterion's nouns
are canon vocabulary or its era's spelling (`48:544-551`, never asked). And S1 through S6
themselves, which is what this file is for.

**The experts', not to be escalated.** Resume `47` on the ladder fork and the section 4 rewording
(closes two standing exchanges at once). A second, derive-first read on whether the ownership and
kind clauses are one principle or two (settles section 6's standing item and the shape of S2/S4).
Build the wide rung `16` declined, moving the stride keying's wide-rung half off ONE EXPERT. The
three cheap checks the unit kept deferring: stride grid-invariance as a fourth arm on `43`'s
apparatus (`44:344-351`), `Cold`'s wide-rung carrier rule (`45:472-477`), the access partition
above 128 bits (`47:543-545`). Price `51`'s collapse and the packed-versus-native trade on the
harness with the five arms `51` names, and read the LLVM pass output that would turn its
mechanism reading into a fact (`51:672-685`). Second-read `45_probes/p3` and `p6`, still the most
quotable and least checked numbers in the unit (`48:574-576`, `50:801-802`, and now this file).
Attack the kind boundary's shared assumption through `10`'s bridge. And the `21`-style entailment
audit of `OPTIONS.md` against its sources, named as valuable at `44:447-451` and still not run.

**This file's own check.** Per `RULES.md:309-319`, the entailment check on this consolidation is
run by someone who did not write it, from the member files forward, counting and diffing the
citation sets on both sides. I have deliberately kept every `file:line` I rely on in the body
rather than compressing them away; the checker should still run the diff rather than trust that
sentence.

## 10. Process record, for the audit trail

Kept short because the member files carry the detail, and recorded because each item changed how
this unit's claims should be read. The rung was inflated twice by restatement, once inside a
member's own summary section and once in a dispatching brief, and the honest rung had to be
re-derived from the files both times (`48` section 1; this file section 3.1). The register
absorbed corrections mid-unit, before their second reads, and a document in "what is fixed" was
edited while two frozen files' greps of it stood, converting two honest citations into apparent
falsehoods until the brief gained its explanatory note (`46:41-48`, `48:264-287`,
`00_brief.md:148-153`). A member file (`46`) sat uncommitted while four files built on it, found
and recovered under the standing remedy at commit `7a3bddd` (`50:96-123`). Two negative
evidence claims ("unpriced", "no such arm exists") were refuted by one command each, the second
against an arm in my own probe directory (`50` section 5, `51` section 1), making the panel's
lesson twice-paid: a negative claim about evidence is a claim about a place. And the unit caught
one tautological check, one could-not-fail harness, and one sampled law inside its own evidence,
each named by the file that shipped it or the file after (`45`/`46`, `51` twice). The mode held:
nothing was settled, the option space shrank from the bottom, and the disagreements that remain
are located rather than diffuse.

## 11. Coverage, bounded honestly

**Read end to end:** `44`, `45`, `46`, `47`, `48`, `49`, `50`, `51`, `52`, `INTENTS.md`,
`00_brief.md`, `RULES.md`.

**Read at the cited passages, by opening the lines:** `15`, `16`, `17` as listed in section 1;
`OPTIONS.md` lines 690 through 1078; `45_probes/p1` and `47_probes/p1` headers; the four `.err`
files (counted, commands stated); `00_brief.md:143-153` as it now reads.

**Verified with commands:** the refusal counts and their domains (section 5); `49`'s commit
ordering; the panel tree's clean state.

**Not done, and what that leaves unverified.** I re-ran no probe: `52`'s byte-for-byte rerun of
`50_probes/` and `51`'s reproduction of `17`'s t2 are relied on as reported. I did not read `02`
through `14`, `18` through `43`, `SETTLED.md`, `DROPLIST.md` beyond the unit's own greps of it,
`seed/` beyond the passages members quote, or the closed predecessor panel; where this file
touches `10`, `35` or `43`, it relies on `44`'s and `47`'s accounts and says so at each use.
`45_probes/p3` and `p6` remain unaudited by anyone including me. The rung table in sections 3 and
4 is my derivation from the member files; `48:561-564` instructed exactly that, and a checker
should still re-derive it rather than inherit mine, because this file is now the most convenient
thing in the unit to compress from, which is precisely the condition under which drift starts.
