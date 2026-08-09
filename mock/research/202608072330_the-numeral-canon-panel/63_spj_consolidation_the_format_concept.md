# 63. Consolidation: the one format concept

**Date:** 2026-08-09. **Persona:** Simon Peyton Jones. **Mode:** explore, do not settle
(`00_brief.md`, `04`, `28`). **Position in the unit:** ninth and last file of unit two, the
consolidation of `55` through `62` on the topic "what is the one format concept, and what must it
cover". The second consolidation this panel has produced; `53` is the first and was read for its
form, not its content.

**Status: COMPLETE.** Written to disk early and extended in place, per `RULES.md:328-329`.

**What this file is.** The standalone statement of one topic. It is versioned and complete in
itself: a section whose content did not change across the unit is written out anyway, and a later
reader should not need the eight member files to use it. It is a **canon candidate, not canon**:
op's explore-do-not-settle mode is in force, nothing here ratifies anything, and this file goes to
op. Where the unit disagreed, the disagreement is stated with what settled it or with what would.
Every claim carries its rung, re-derived from the member files rather than inherited from any
account of them. Unit one inflated a rung three times; unit two, on `59`'s audit and on my own
check of each member's stated reading order, inflated none, and this file's job includes keeping
it that way.

A note on my own position. I wrote `23` in this panel, on what a canon sentence has to be to
survive. Nothing of `23`'s is load-bearing in this unit except one register line it left behind:
Q3's closing sentence, "One sentence from op collapses it" (grep-verified in `OPTIONS.md` Q3),
which section 4.4 below shows became load-bearing for this unit's strongest result. I flag the
self-connection so a reader can weigh it; the Q3 dependency itself was found by `59`, not by me.

## 0. Gates

**Canon gate: passes, in the second of the three situations.** There is no ratified canon;
`mock/canon/` does not exist in this repository, `00_brief.md:8-9` says the panel is writing the
first one, and `INTENTS.md`'s own header records that no entry holds the ratified rung. The fixed
material this file was checked against: the intent catalogue (I1 open strategy set; I3, I7, I9,
I10 quoted where used), the acceptance criterion at `00_brief.md:144-146` with its restored
plural, the workspace discipline, and the forbidden-feature list at `00_brief.md:158-160`.
Nothing below settles anything, proposes a mechanism, or closes a register option.

**Test gate: no suite exists to run.** `mock/crates` is the nuked tree the brief describes; five
member files confirmed it empty and I did not re-confirm what four probes' gates already printed.
The substitute is the probe discipline, and section 1 is this file's application of it: I re-ran
every instrument in the unit rather than trusting any member's rerun of any other's.

## 1. What I verified myself, before consolidating

A consolidation is a compression and its author is the wrong person to check it
(`RULES.md:300-319`), so the entailment pass on this file belongs to someone else. What I could
do, and did:

**Every committed instrument in unit two was rebuilt on the pin and re-run, and every one
reproduces byte for byte.** Thirty-two instruments: `55_probes/` p1 through p5, `56_probes/` q1
through q3, `57_probes/` p1 through p9 including p2b, `58_probes/` p1 and p2, `60_probes/` p_a,
p_b, p_c1, p_d plus the p_c2 compile-fail control (4 instances of "generic parameters may not be
used" in both the committed `p_c2.stderr` and my rerun), `61_probes/` q1 and q2, `62_probes/` p1,
p2, p2b, p3, p4. All diffs empty. The rerun outputs are committed in `63_probes/rerun/` beside
this file, with the build line in `63_probes/RUN.md`. Every count quoted below is therefore a
count regenerated on this machine, not inherited from a member's report of it.

**Opened end to end:** `55` (both phases), `55b`, `56`, `57`, `57b`, `58`, `59`, `60` (both
phases), `61`, `62`, `53`, `INTENTS.md`, `00_brief.md`, `RULES.md`. **Opened at the source for a
specific fact:** `57_probes/p3_semiring_across_widths_and_scales.rs:109` (the scale-blind add),
`57_probes/p4_which_factor_breaks_and_what_coherence_buys.rs:66-96` (the three Factor arms),
`55_probes/p4_induced_algebra_grades.rs:69-73` (bare add and mul),
`35_probes/p3_reduction_order.rs:70-74` (the same-scale clipped add carrying Q12's divergence
table), `61_probes/q1_output.txt` in full with line numbers (`:29`, `:55`, `:83`, `:109` carry
the four absorption-mismatch rows `57b` tabulates), `57_probes/p6_output.txt` in full,
`57_probes/p9_output.txt` in full. **Register sections read:** Q3, Q5, Q6, Q11, Q12, Q13, Q14,
Q17, the wrapping entry, and the unasked-questions section. Citations into `OPTIONS.md` and
`INTENTS.md` below are by section plus a `grep -F`-verified phrase, never by line, per the brief.

**Verified in the history:** `55` phase one is committed at `ce13af50` before its phase two at
`22a79c65`, and `60` phase one at `ff4cb6a6` before its phase two at `f3a266af`. Both cold
derivations are therefore cold in the checkable sense, not by self-report alone.

**Checked against `59`'s audit:** the Q3 absence claim reproduces (`grep -c 'Q3'` returns 0 on
`55`, `55b`, `56`, `57`, `58`, and also on `57b`; it returns 10, 8, 4 on `60`, `61`, `62`, so the
gap `59` named was closed by the second four exactly as dispatched).

**Not verified by me:** I did not open `08`, `42`, `35`'s prose, `18`, `20`, `25`, `40`, `43`,
`50`, `54`, `seed/`, `DROPLIST.md`, or `archive/`. Every statement below about `42` is sourced to
`57` and `57b`, which opened it, and marked so; every statement about `08` is sourced to `55` and
`56`; the one fact I use from `35` is the probe source line I opened plus `62`'s byte-identical
rerun of its output.

## 2. The topic, and the form its answer took

The topic was dispatched as a definition question: what is the one format concept, and what must
it cover. The unit's answer has three layers, and the order in which they emerged is part of the
evidence, so this file carries the argument and not only the conclusions.

**First, a model:** all arithmetic on a format factors as an exact operation in an ambient domain
composed with a total adaptation onto the representable set, `computed = adapt(exact)`
(`55:23-31`). Derived cold by `55`, derived cold a second time by `60` at the chain level
(`60:28-34`), and given its sharpest defence by the one case that refuses it: the eager
fixed-point multiply, which `58` showed is not an operation of the model at all but an operation
and an adaptation point fused under one name (`58:147-176`, `60:341-351`).

**Second, a decomposition of the concept itself**, which moved under attack and ended somewhere
better than it started: `55`'s four-choice tuple `F = (D, Q, R, E)` (`55:39`) became, through
`56`'s attack and `55b`'s concessions, **an identity half and a realisation half**: a format is
its ambient domain and representable set, with the space of lawful reductions a derived object
rather than a slot, and the encoding a realisation choice with observable pattern-level
consequences (`56:84-126`, `55b:33-68`).

**Third, a law layer that is derived rather than enumerated.** The unit began with two law
families measured as independent (`56:53-59`), passed through a refutation, a scope correction
and a signed extension, and ended with `57b`'s two-hypothesis frame predicting every measured
cell of the unit's cube with zero exceptions (`57b:247-297`, `57_probes/p9_output.txt`, re-run
here). That frame is the topic's single most consolidation-worthy artifact, and it is ONE EXPERT
and says so.

Sections 3 through 5 state each layer in full. Section 6 gives the candidate canon sentences.

## 3. The concept, part by part, with rungs

### 3.1 The standard model

**The claim.** Arithmetic on represented numbers is statable exactly when it factors as
`computed = adapt(exact)`: the operation performed exactly in an ambient number system, a total
map bringing the result back onto the representable set. The factoring is what makes error
analysis possible, because the error of a computation is the composition of adaptation errors,
and the adaptation error is a property the format can state (`55:26-31`).

**Rung.** TWO EXPERTS, with a stated discount: `55` derived it cold (`ce13af50` before phase
two), `60` derived it cold at the chain level (`ff4cb6a6` before phase two), and both draw on the
same numerical-analysis literature (Wilkinson, IEEE 754), which both declared (`55:19-21`,
`60:466-469`). Two independent instruments over one shared literature, worth more than a read and
less than two arrivals from nothing, which is the bound `55` set for exactly this situation
(`55:363-369`) and I keep it.

**The negative case that completes it.** The model requires the adaptation to be **unfused**: the
ops must be the exact ops, and every narrow, rescale included, a separately placed member of the
schedule. `58` established that eager fixed-point multiplication at nonzero fraction cannot
supply an exactly associative ambient operation, because the rescale is baked into every pairwise
step (`58:147-176`, with the hand witness at F = 2, operands 3, 5, 7: five against six with no
clamp anywhere, `58:161-167`). `60`'s reconciliation reads that not as a defect of the model but
as the proof of its condition: the fused spelling is what cannot be re-expressed as exact
composition (`60:341-351`). Two authors, one structural argument, argued rather than proven as a
theorem, and `58` says so (`58:431-439`).

### 3.2 Identity: the ambient domain and the representable set, with the locus rider

**The claim.** A format is identified by (D, Q): its ambient domain and its representable set.
Not Q alone, because the same representative set under two ambient algebras is two formats, which
is precisely what the wrapping question is about; not the full tuple, because two's complement
and offset binary denote the same sixteen values through different pattern maps and are one
format filed two ways (`55b:46-55`, `55_probes/p3`, re-run here). **Q is a constant of the
type**: a value set depending on other data has no Q and is not a format but storage, which
absorbs `08`'s locus clause as a condition on Q rather than an external rider (`55b:62-68`).

**Rung, disaggregated because it moved.** The value-level half (identity is denotation, not
encoding) is TWO EXPERTS with the shared-literature discount: `55` reached it cold from the
provable-once test, `08` had reached it from the design's axis tables, and both share Flocq
underneath, a bound both authors stated (`55:363-378`, `56:373-380`). The refinement from Q to
(D, Q), the dissolution of R into derived structure, and the locus rider are **converged by
attack and concession** (`56` sections 2 and 9, `55b` sections 1 and 2): a located convergence
through argument, which is a real result and is not the TWO EXPERTS rung, because `55b` conceded
on reading rather than deriving independently. I mark it as such.

**One consequence `62` added that the identity sentence absorbs for free.** At F = 0 the two wrap
sections (unsigned and signed representative ranges of one modulus) induce the identical ring; at
F > 0 they compute different products, 96 of 256 class pairs at w = 4, F = 1, rising to 168 of
256 at F = 3 (`62:141-156`, `62_probes/p3`, re-run here). Under (D, Q) identity that is not a
complication: the section **is** Q, so two sections are two formats, and the fact that their
arithmetic diverges at F > 0 is the identity doing its job. What it corrects is the scope of
`56`'s deflation sentence: "the section is chosen at declaration and no cast consults a policy"
is exact at F = 0 and the declaration becomes arithmetic-bearing at F > 0 (`62:259-264`). The
register's wrapping entry does not yet carry that scope sentence.

### 3.3 The representable set: one affine membership predicate

**The claim.** Membership of Q is one predicate over one parameterisation: a slot function giving
the quantum per magnitude, in affine form so a phase (a grid offset) is expressible. Integers,
fixed point, scaled integers and floats are points of the parameterisation; subnormals fall out
of a max with no special case (`55:100-121`, `55_probes/p1`: 16, 16, and 47 values matching
textbook enumerations exactly, both mutants detected, re-run here).

**The phase is not a passive parameter.** A half-step-biased grid is not closed under exact
addition: 0 of 256 exact sums land on it, every one sits exactly half a step away, and the grid
contains neither zero nor one (`56_probes/q2_output.txt`, re-run here). So phase decides whether
the identity adaptation ever occurs for addition, promotes the tie rule from corner case to the
policy that decides every sum, and produces a numeral that is not even a monoid carrier
(`56:283-307`). Round-to-nearest onto the biased grid keeps all four adaptation laws, so the
repair to the concept is contained in Q and costs the adaptation slot nothing (`56:266-281`).

**Rung.** The one-predicate unification: TWO EXPERTS with the shared-literature (Flocq) discount,
per 3.2's bookkeeping (`55_probes/p1` and `08`'s twenty-one-representation classifier, the latter
read through `55` and `56`, not by me). The phase necessity: ONE EXPERT (`08`'s measurement,
which I did not open) plus `55`'s concession (`55:382-389`) plus `56`'s constructive repair
(`56_probes/q2`, re-run here). Two independent instruments erred at the phase coordinate in
opposite directions (`56:283-288`), which is the unit's argument that the canon's sentence for Q
should carry the affine form explicitly rather than leaving phase to be inferred. I carry that
argument; it has not been contested.

### 3.4 The adaptation slot: derived, populated per operation by the strategy layer

**The claim.** Given (D, Q), the space of total reductions onto Q is a derived mathematical
object, the way the ulp function is derived; nothing about a format selects members of it. What
selects a member, per operation, is the strategy layer (`55b:38-44`), which is I9 landing in the
model at the position op put it: "strategies are the variables that change what the 'correct'
answer is" (I9, grep-verified phrase "variables that change"). A strategy, through this unit's
lens, is a policy over the format's open choices: which reduction member per operation, which
encoding, which schedule, how much deviation is tolerable (`55:258-269`, `60:218-239`). Nothing
presumes a strategy count, which I1 requires.

**Members classify along two independent law roles.** The adaptation laws face the source
(monotone, distance-minimising: what error transport and order transport consume) and the
coherence law faces the target (whether the reduction is a homomorphism onto its induced
operation). All four combinations are inhabited: signed saturation holds the adaptation laws and
fails coherence at 476 chain-divergent triples; wrap fails both adaptation laws and holds
coherence for addition and multiplication at zero; unsigned add-only saturation over a
nonnegative window holds both; the opposite-bound mutant holds neither, at 897
(`56:160-171`, `56_probes/q1`, re-run here). So neither family subsumes the other, expelling wrap
while retaining saturation has no criterion that does not empty the slot (`56:174-197`), and the
domain filing of wrap was withdrawn by its own author (`55b:71-79`).

**Each coherent member computes exactly in an induced algebra, and the algebras form a ladder.**
Wrap induces the ring Z/2^N; unsigned saturation induces a commutative semiring, distributivity
at zero failures; signed two's-complement saturation induces a unital commutative magma that is
not a semigroup, 952 associativity failures on Q itself (`55b:94-101`, `55_probes/p4`, re-run
here). The licensed rewrites follow from the rung: a group licenses reassociation and
cancellation, a monoid or semiring reassociation without cancellation, a magma nothing
(`55b:110-113`). `57` failed to break the ladder's compositionality over twelve configurations
(meet respected and equalled exactly, `57:405-435`, an honest failed refutation, not a proof) and
objected only to the word "grade", offering "a point in the lattice of law sets", which `55b` has
not been resumed to accept. `57b` then made the ladder's rung **derived**: it is computed from
congruence verdicts, one per operation, rather than observed per cell (`57b:353-360`), which is
section 4.2.

**Rung.** Two-family inhabitation: measured by `56` (instrument independent, idea first-read,
`56:218-222`), the induced-algebra reading measured by `55b`, the role asymmetry restated by `57`
(`57:505-529`); every instrument re-run byte-identically by later members and by me. The
R-is-derived sharpening: converged by exchange, per 3.2's bookkeeping.

### 3.5 The encoding: a second, ordered, behaviourally observable axis

**The claim.** The encoding relates Q to bit patterns and is ordered after Q: the value set is
chosen, then realised. It is not part of identity and it is observable: raw-order agreement and
raw-adder correctness are pattern-level properties that encoding choice buys or forfeits
(`55:203-228`, `55_probes/p3`, re-run here).

**The signed trade is forced.** For a signed value set, the only bijective encoding with
raw-order agreement is excess-K (the sorted correspondence is unique), and excess-K fails the
adder property by the constant K, every pair; so no bijective encoding of a signed set has both,
while the unsigned identity encoding has both (`56:329-347`, `56_probes/q3`, re-run here:
two's complement 256 of 256 adder-correct and order-disagreeing, offset binary order-agreeing and
0 of 256 adder-correct with constant defect 8). The world's hardware sits at both ends: biased
float exponents, two's-complement ALUs (`56:345-347`). **Redundant encodings are wholly
unexamined** and could conceivably buy both at the price of patterns (`56:508-510`); `59`
section 3 offers the untested hypothesis that the exclusivity is the no-translation-invariant-
order theorem on a finite cyclic group in disguise, which would close the redundant-encoding hole
if built, and nobody has built it.

**Rung.** The trade: ONE EXPERT (`56`), instrument re-run here, uniqueness argued for bijections
only. The monotone-encoding refinement it completes: `55`'s, accepted by `56` with a quantifier
note (`56:317-327`). The strategy-picks-E hazard this sharpens (two strategies over one format
disagreeing on pattern-level properties, `55:271-275`) is a realisation-level fact under 3.2's
identity, correctly outside format identity (`55b:57-60`), and it is Q13's classification
question arriving at E.

### 3.6 Boundaries the concept states rather than hides

**Non-finites.** Under the carried scoping theorem (every arvo value is `m * r^q`, TWO EXPERTS in
the predecessor record, read here through `55` and `56`, not re-derived), infinities and NaNs
cannot be elements of any Q, and the only live placement is encoding-level escape codes with
stated propagation, for which the design already has the section-retraction precedent
(`55:390-399`, `56:382-387`). Both files flag the theorem as carried rather than beyond question;
so do I.

**Stored-pair rationals, intervals, error-carrying pairs, complexes.** Not format instances:
compositions over formats, with their own laws owed by the composition layer (`55:176-193`).
Uncontested through the unit; fits the brief's own "named compositions over one format concept"
sentence and I11's composition intent; needs Q16's sense-two word, whichever wins. ONE EXPERT,
uncontested through seven subsequent files.

**Wrapped order.** A wrapped numeral has no arithmetic-compatible order (a finite cyclic group
admits no translation-invariant total order), so comparison there is representative-order, which
arithmetic does not respect. Owed under every filing equally, therefore not a distinguisher
between filings; one sentence the canon owes whatever else it says (`55b:132-140`).

## 4. The law layer, as the unit built it

This is where the unit spent most of its evidence, and the argument's route matters because two
of its best results were corrections of its own earlier statements.

### 4.1 The criterion: absorption, quantified over the values the format can hold

**The refutation that started it, told honestly.** The register's Q12 carried a mechanism
sentence from `42`: associativity of a clamped operation survives by bound-reachability
("at most one of its clamps can be triggered", grep-verified in Q12). `55b`'s `p5` refuted the
quoted form exhaustively: 952 divergent signed-clamp triples, 448 ceiling-only, 504 floor-only,
**zero involving both bounds** (`55b:160-190`, re-run here). `57` then opened `42` itself and
found the refutation was already inside it: `42`'s own published row one, a ceiling clamp with no
floor, measures 904 failures while the sentence predicts associativity, so `42` had refuted
clamp-counting deliberately, named the surviving hypothesis H2 in its probe's comments, and then
written a prose summary whose word "clamps" contradicted its own table (`57:161-217`; `42` cited
here through `57` and `57b` only). `59` corrected `57`'s framing, which had led with the largest
true sentence, and `57b` conceded the correction in full with no residue defended
(`57b:24-25`, `57b:72-99`). The dispatching layer's own compression of the finding is named in
`59:100-109` against its own brief, which is the audit-trail behaviour this panel exists for.

**What replaced it.** A reduction is **absorbing** over an operand set when
`rho(rho(x) op y) == rho(x op y)` for every reachable exact value x and every operand y. For
clamped addition, absorption and associativity of the induced operation agree exactly: zero
sufficiency and zero necessity violations over 4248 configurations, and again over a widened
7744-configuration sweep (`57:219-246`, `57_probes/p2`; `61_probes/q1_output.txt:29` and `:83`;
both re-run here). For clamped multiplication, absorption is sufficient and necessary modulo
operations the clamp has collapsed to a constant: the 153 exceptions on the first sweep and the
150 on the widened one are all constant-collapsed, residue zero
(`57_probes/p2b`, `57_probes/p8`, both re-run here; `57b:131-165`).

**Its scope, measured after being assumed.** `59` flagged that `57`'s identification of
absorption with `56`'s coherence law rested on an unmeasured bridging step (`59:49-79`). `61`
measured it: absorption and coherence-ext are the identical predicate **exactly when the operand
box is a subset of Q**, with zero disagreements inside Q on every sweep and every operation, and
off Q coherence is the one that gets associativity wrong, never absorption
(`61:52-93`, `61_probes/q1_output.txt:25-29`, re-run here; hand witness worked at `61:158-177`).
`57b` accepted both halves and added the framing this file carries into the candidate sentence:
the restriction is not a caveat on the law but **the statement of the law's subject**, because
every stored value of a numeral type is already an element of Q, so a predicate quantified over
values the type system cannot produce is measuring a different object (`61:197-204`,
`57b:123-129`).

**The corollary a consumer can check from the type.** On an interval numeral containing zero,
saturating addition is associative iff the interval is sign-confined (`lo == 0` or `hi == 0`):
100 intervals swept, 19 associative, zero mismatches (`57:248-267`, `57_probes/p1` section 3,
re-run here). The closed form is a corollary and not the criterion: the opposite-bound mutant is
sign-confined with a reachable ceiling, the closed form predicts associativity, and it diverges
2240 times while absorption predicts the failure (`57:269-273`).

**Rung.** The biconditional: first stated by `57`, exception characterised twice
(`p2b`, `p8`), reconfirmed under two sweeps by `61` with an independent instrument, accepted by
`57b`. The scope partition: measured by `61` (first-read there), accepted by the identification's
author. The pullback mechanism underneath: proposed by `55b`, kept by `57` as the dynamic
description with absorption as the condition (`57:303-318`), and independently instanced blind by
`60`'s phase-one witness at i16, a ceiling-only clamp-then-pullback divergence built with no
knowledge the mechanism existed (`60:469-477`). That blind instance is the one genuinely
independent arrival the mechanism has.

### 4.2 The congruence layer, and the frame that replaced the shared theorem

**The unsigned semiring is structural.** Every commutative-semiring axiom at M = 1, 2, 3, 7, 15,
31, 63, 127, 255, zero failures, with the explanation that covers all widths at once: "equal, or
both at or beyond the bound" is a congruence on the naturals for both operations, so the
saturating algebra is the quotient and inherits every axiom (`57:322-346`, `57_probes/p3`, re-run
here). M = 1 degenerates to the Boolean semiring, the structurally distinct sanity instance.

**The signed cell never had the structure.** Signed two's-complement saturating multiplication
fails associativity **on the integer grid**: 28, 160, 780, 3516 triples at w = 3 through 6, with
the three-value witness `(7 * 7) * -1 = -7` against `7 * (7 * -1) = -8` and no coarsening
anywhere in it (`62:70-77`, `62_probes/p1`, re-run here). The mechanism is the range's asymmetry
under negation: negation carries one absorbed tail into the interior, so "both beyond the same
bound" is not preserved by multiplication by a negative and no congruence can be stated
(`62:82-95`). **A symmetric clamp restores it exactly**: zero associativity failures and zero
coherence violations at every width measured, while distributivity and additive associativity
stay broken because they route through the additive pullback that range symmetry does not touch
(`62:90-95`). One code point, the most negative value, is the entire difference.

**The congruence condition, per operation, with closed forms.** `57b`'s `p7` answers `62`'s
request for the theorem shape: the reduction's kernel is a multiplicative congruence iff the
range is mirror-symmetric (`lo == -hi`) or nonnegative (`lo == 0`), and an additive congruence
iff the range is sign-confined (`lo == 0` or `hi == 0`); over all 100 zero-containing intervals
the predictor mismatches nothing, addition is an exact biconditional in both directions, and the
multiplicative necessity exceptions are again exactly the constant-collapsed operations
(`57b:174-217`, `57_probes/p7`, re-run here; the width table reproduces `62`'s four widths and
adds w = 7). The probe's two false starts are kept on disk with their hypotheses attached, and
the first of them is the same over-quantification `61` caught in `56`'s coherence law, made by
the adjudicator after reading the correction, which `57b` records plainly as evidence that the
trap is the quantifier and not the file (`57b:219-234`).

**The frame.** `57`'s shared theorem ("coherence is the statement that the grading collapses",
`57:520-524`) was bounded by `59` as additive in all its evidence (`59:130-143`) and refuted for
multiplication by `58`; `57b` withdrew it as stated and replaced it with two hypotheses:

> **H1.** The ambient operation is associative on the reachable set.
> **H2.** The reduction's kernel is a congruence for it.
> Both hold, and the induced operation on Q is associative, because Q is the quotient.

Evaluated mechanically over the unit's cube, twenty-four cells (three sign domains, two policies,
two operations, two scales): sufficiency violations zero, cells associative without both
hypotheses zero, residue zero, with each hypothesis and the measured truth observed both true and
false across the cube (`57b:247-297`, `57_probes/p9_output.txt`, re-run here). The frame's
consequence is the fixable/unfixable split: **H2 is decided by range geometry per operation and
is fixable by choosing Q**, which is why the symmetric clamp restores signed multiplication at
F = 0; **H1 fails wherever a fixed-width rescale is part of the ambient step and nothing
downstream repairs it**, which is why the same symmetric clamp buys nothing at F > 0
(`57b:255-265`). Read through it, the unit's table rows become derivations: addition's ambient is
always associative so additive verdicts are decided by H2 alone and are scale-blind; wrap's
kernel is always a ring congruence so wrap is decided by H1 alone; saturation's kernel is a
congruence only under the stated geometry; and no multiplicative cell survives F > 0 anywhere
(`57b:280-291`).

**Rung.** The unsigned congruence: `57`, with the congruence relation itself measured at five
moduli; `59` section 3 notes it is provable in a paragraph and `57b`'s p7 case analysis supplies
the multiplication-side argument. The signed break and symmetry mechanism: `62` first-read,
reproduced by `57b` from an independent instrument with a fifth width added (`57b:212-215`). The
frame: **ONE EXPERT** (`57b`), and its twenty-four-cell prediction is a mechanical evaluation
re-run here; its ingredients are separately multi-instrumented, and the frame as a unifying
statement has not been attacked by anyone. It is the right kind of claim to attack next.

### 4.3 The fraction boundary, and the filled cube

**The boundary is structural, not swept-so-far.** At F > 0 a representable-width multiply
discards F bits at every pairwise step before any clamp is consulted; the choice of what to
discard (the rounding rule) moves failure magnitudes and never their existence (truncation
against round-half-up: counts move both directions, no row reaches zero, `58_probes/p1`, re-run
here; truncation against arithmetic-shift on signed operands: same shape on a second axis,
`62:122-130`); and the only escape, never narrowing until the end, abandons fixed width at a cost
linear in fold length (`58:80-115`, `58:178-203`). Three instruments and a structural argument
converge: `57_probes/p3` section 3 (nine of nine fractional configurations fail), `61_probes/q2`
(wrap's ring collapses identically, 9 of 9, additive abelian group surviving unconditionally),
`62_probes/p2` (54 of 54 signed rows fail, both spellings), all re-run here.

**The attribution, corrected twice and now stable.** `57` attributed the unsigned collapse to the
grid coarsening and reported the clamp factor clean (`57:375-397`); `58` corrected the reading:
the "clamp only" arm is F = 0 relabelled, no F > 0 clamp ablation can exist because coarsening is
what F > 0 multiplication is, and an intermediate range clamp at full fractional precision is
independently a real divergence source (448 of 4096 at n = 3 with zero rounding loss anywhere,
`58:226-274`, `58_probes/p2` section 2, re-run here). `62` then showed the "clamp factor is
clean" converse is unsigned-only: in the signed domain the clamp is broken at F = 0 on its own,
so signed F > 0 multiplication has two independent sufficient mechanisms, converging at deep
fraction to pure coarsening, where the two policies' failure sets coincide **exactly** (380
identical triples at w = 4, F = 3, policy-specific contributions 0 and 0, `62:96-110`,
`62_probes/p2b`, re-run here). `57b` conceded the scope and restated its own sentence as "the
fractional collapse is an H1 failure, and it is the only mechanism present in the unsigned
domain" (`57b:299-315`).

**The cube, as the unit leaves it** (each row's instrument named in `62:159-172`, all re-run
here; the congruence column from `57_probes/p7`):

| sign | op | policy | F = 0 | F > 0 |
|---|---|---|---|---|
| unsigned | add | saturate | commutative monoid | monoid, every F |
| unsigned | add | wrap | abelian group | group, every F |
| unsigned | mul | saturate | semiring half, by congruence | dead |
| unsigned | mul | wrap | ring half | dead |
| signed | add | saturate | dead, 952 at w = 4 | dead, counts F-invariant |
| signed | add | wrap | abelian group | group, every F |
| signed | mul | saturate, 2c range | dead, 160 at w = 4 | dead |
| signed | mul | saturate, symmetric range | associative, monoid | dead |
| signed | mul | wrap | ring | dead, and section-dependent |

Three structural statements the cube supports, none available from any single row
(`62:174-194`): the induced structure is a function of the policy, the scale, the sign domain and
the range's symmetry jointly, with `57b` deriving the rung from congruence verdicts rather than
adding a fourth parameter; the two multiplicative failure mechanisms are independent and
separately sufficient, and the signed domain is where both live at once; and nothing
multiplicative survives the fraction axis anywhere, so the additive column and the F = 0
signed-wrap and symmetric-clamp cells are the only structure a law layer has available to state,
with the signed additive half belonging to wrap alone.

**Why the cube's worst corner matters.** `57:620-626` said it and the unit confirmed it in every
particular: the signed case is where the algebra is worst, and it is the case a general-purpose
default would be, since I3 points the default strategy at Rust's primitives, which are signed.
Under clamping a signed numeral's multiplication is broken at F = 0, where the unsigned result
was a theorem; the one lever that buys it back is a range choice costing one code point and
departing from the two's-complement value set, with prior art in shipped DSP saturation modes,
currently expressible nowhere in the register (`62:296-303`, `57b:345-351`). Unpriced, and op's
to weigh once priced; section 9.

### 4.4 The additive column, and the premise it stands on

**The additive verdict, whatever it is, is independent of the fraction width.** Addition of two
values at a common scale never reads the scale: `57_probes/p3.rs:109` and the p4 Factor arms are
scale-blind by construction (opened here), so an additive result at F = 0 transfers verbatim to
every F, by inspection rather than by sweep (`58:69-81`). `61` re-made the argument for wrap's
add closure (`61:283-294`), and `62` sharpened the general statement: the broken counts transfer
too (952 at w = 4 for every F measured), so what is F-independent is the verdict, not survival
(`62:113-121`, `62:198-203`). This also upgrades results outside the unit: `35_probes/p3`'s
divergence table (unsigned wrap 0, signed wrap 0, unsigned saturating 0, signed saturating 70.1
percent) is a same-scale clipped add (source opened at `35_probes/p3_reduction_order.rs:70-74`;
output re-run byte-identically by `62`), so Q12's headline table is F-independent by the same
argument, which `59` first said (`59:186-196`) and the register now carries in Q17.

**The premise, named because nobody in the first five files had.** All of it is **same-scale**
addition. `59` found Q3 (is there a mixed-numeral addition?) uncited by the entire first half of
the unit and load-bearing for its strongest unconditional result (`59:224-242`). `60` sharpened
the dependency: aligning two scales to the join is a left shift, a widening, exact always; the
coarsening threat enters only when the inferred result format is narrower than the join, so what
threatens the additive column under Q3's second option is **the result-format rule and the
schedule, not mixedness itself** (`60:438-451`). `61` and `62` then stated their own results'
Q3-orthogonality explicitly (every operation single-numeral at a fixed common scale,
`61:308-331`, `62:352-354`). The dedicated sweep `59`'s P2 asked for, varying the Q3 option **and
the result-format rule**, has not run. Q3 itself is op's; section 9.

### 4.5 Accumulators: the grading that is real, and its two shapes

**Coherent reductions need no accumulator at all.** Eager saturation in the format itself agrees
with exact-then-adapt at every accumulator width including the narrowest, across all sixteen
unsigned rows and at every fold length measured (`57_probes/p6_output.txt`, 20 rows "format width
already suffices", re-run here; `57_probes/p4` section 2's split table, 0 divergence for wrap and
unsigned saturation at n = 2 through 6 against 11.62 to 39.54 percent for signed saturation).
Conditional on H1 per `57b:292-297`: where the ambient is not associative there is no grade to
collapse.

**Incoherent clamped addition: exact-sum width less one bit.** The gap is exactly one bit in all
fifteen rows where the question arises, zero anomalies; the final adaptation absorbs the
outermost bit, because the accumulator must decide which side of the format the result fell on,
not represent how far outside it fell (`57:479-503`, `57_probes/p6`, re-run here). ONE EXPERT,
one policy, fifteen rows, no proof, and `57` says so. Two predicates are distinguished on the
way: interior safety (no clamp fires, `20`'s predicate, not opened here) and adaptation agreement
(the answer equals exact-then-adapt), which differ by exactly this bit; the panel had one phrase
for both (`57:495-501`).

**Multiplication: linear, with a rounding-conditional constant.** The exact guard grows linearly
in fold length, `(n-1)F` bits, and there is no logarithmic closed form; the saving below full
precision is **adaptation fusion**, worth exactly F under a composing rounding rule and zero
under round-to-nearest-even at n = 3 and 4, with a second, growing, rule-independent slack
appearing at n = 5 (`58:178-203` measured under truncation; `60:379-425` parameterised the rule,
predicted the fusion from the schedule algebra in the probe header before running, and refuted
`58`'s "exactly F, constant" at n = 5: truncation savings 3, 3, 4 and RNE 0, 0, 3,
`60_probes/p_d.out`, re-run here). `62` extended it to the signed domain on a second rounding
axis: the floor spelling shows pure fusion, exactly F at n = 3, 4, 5, while truncation is
irregular (`62:131-139`, `62_probes/p4`, re-run here). **Q11's accumulator relation ("accumulator
is derivable as the width plus the log of the capacity", grep-verified in Q11) is therefore an
additive-only mechanism**, which `58` argued, `60` confirmed from a blind derivation, `62`
reconfirmed from the signed side, and Q17 now carries.

**Widths do not grade; reachable intervals do.** `max(W, V) + 1` is not associative
(`g(g(5,0),0) = 7` against 6), while the reachable interval composes exactly under Minkowski sum
at every measured length, the width a sound image loosening as n grows (`57:437-460`). The
operational grading with teeth is divergence as a function of accumulator width, reaching zero at
a computable index (`57:461-477`). What any of this costs or buys at runtime is **unpriced**: no
bench harness has run on any of it, and every member says so.

## 5. The chain, and what the concept owes the layers above

`60`'s cold derivation, the answer to `59`'s P1, delivered with the condition that reconciles it
with `58`:

**A chain is a composition of exact operations together with a schedule of adaptation points, and
the schedule is part of the function's meaning**, because two schedules over the same ops compute
different functions (`60:28-34`, `60_probes/p_a`: three schedules, three functions, with the wide
arm satisfying the correct-rounding property on all 46,656 inputs against a property-checking
oracle that never itself rounds, and the truncating mutant flagged on 22,476; re-run here).
Exactness has grades: composite correct rounding, stepwise correct rounding, bounded drift, and
structural exactness, the fixed-point family's own possession, where a chain inside its width
algebra is the mathematics itself with no error analysis (`60:86-121`). The multiplicative
ever-growing intermediate is real and the window dissolves it: a bounded subterm evaluated
exactly and adapted once, its capacity a static function of container width and operand formats,
derivable by the same typestate the acceptance criterion already demands (`60:122-154`). Probe A
is the constructive F = 8 complement to `58`'s eager impossibility: at F > 0 the safe
multiplicative shape is the widening window (`60:352-360`).

**The per-operation model extends to chains if and only if adaptation is unfused from the
operations** (`60:336-351`), which section 3.1 already carries; the unit's vocabularies snap
together with no residue in `60`'s table (`60:362-377`): grade a is the exact-then-adapt oracle,
grade b the eager schedule, coherence the bridge between them, and grade s (ambient exactness,
no adaptation at all) distinct from coherence (exactness in the induced algebra), the two ways a
chain can be cheap, not the same way.

**What the format concept must carry for any of this to be statable: three things**
(`60:173-214`). The **width algebra** of exact results, statable as trait contracts on the pinned
toolchain with no forbidden features (`60_probes/p_c1`, re-run here) while the general
one-impl-for-all-widths spelling is refused with four counts of the `generic_const_exprs`
diagnostic (`60_probes/p_c2.stderr`, refusal reproduced here), so the accepted shapes are bounded
enumeration or a type-level arithmetic contract, per the refused-bound rule. The **named
adaptation**, carrying its rounding rule, its overflow member and its granularity function, since
every chain error bound is a sum of granularities at adaptation points. The **exactness
predicate**: the conditions under which an op or adaptation loses nothing, fixed point's
mainland, floats' islands (Sterbenz, the error-free transformations). And the statability
argument that makes the list non-optional: **a concept that closes its operations over the
format, adaptation fused invisibly into each op, can state stepwise correctness and nothing
above it, so I7's chain clause ("especially within chains and ops, not only alone",
grep-verified in INTENTS I7) has no expressible form against it** (`60:206-214`). That converts
op's accuracy-in-chains intent from an optimisation request into a constraint on the concept's
shape. ONE EXPERT, cold, reconciled against the unit without contradiction, and unattacked; the
D-A direction (chains entirely elsewhere) survives only under a reading of I7 that its quoted
words do not favour, which only op can rule on (`60:243-251`).

**Order and threads fall out rather than being added.** A parallel reduction is a reordering, so
I10 (no stance on cores) and I7 jointly push whichever strategy claims chain precision toward
order-independent schedules: the wide-exact-adapt-once fold and the wrapping fold are
order-invariant, the per-step saturating fold and the stepwise float sum are not
(`60:156-171`, `60_probes/p_b` at i16, re-run here; the saturating witness is the blind pullback
instance of 4.1). The counterintuitive half belongs in the record: wrap, the "unsafe" policy, is
the order-independent one; saturate, the "safe" one, makes the chain's value depend on
evaluation order (`60:54-65`).

## 6. Candidate canon sentences

Each tested against permanence (still true and useful after a from-scratch rewrite) and
equivalence (three independent implementations behave the same), per `RULES.md:79-83`, with what
it rests on. Candidates for op, not rulings. Vocabulary note: "format", "adaptation",
"absorption", "coherence", "schedule", "window" are working names; none has been put to op as
canon vocabulary.

**C1, the standard model.** *Arithmetic on a format is an exact operation in an ambient domain
composed with a named, total adaptation onto the representable set. The adaptation is a
first-class object with its own laws; an operation that fuses one invisibly is an operation and
an adaptation point wearing one name, and the model's statements apply to it only when the two
are separated.* Permanence: passes; no mechanism named. Equivalence: passes; it fixes the
correctness oracle every probe in this unit used. Rests on: 3.1 (two cold arrivals with the
shared-literature discount; `58`'s structural negative; every instrument in the unit consuming it
as the oracle).

**C2, identity.** *A format is identified by its ambient domain and its representable set. The
representable set is a constant of the type: a value set that depends on other data is not a
format but storage. Adaptation choice and encoding are realisation, observable in computed values
and in pattern-level properties respectively, and not part of identity.* Permanence: passes.
Equivalence: passes; it decides format equality, which Q10 needs and which the four-tuple left
unstated. Rests on: 3.2. The value-level half TWO EXPERTS with the stated discount; the (D, Q)
refinement converged by attack and concession, marked as such.

**C3, the representable set.** *Membership is one predicate over one parameterisation: an affine
slot function, a quantum per magnitude and a phase, of which integers, fixed point, scaled
integers and floats are points. The phase is stated explicitly: a nonzero phase decides whether
the identity adaptation ever occurs and whether the set carries an additive identity at all.*
Permanence: passes. Equivalence: passes at the model widths probed; the affine form is the
repair both instruments that erred at this coordinate needed. Rests on: 3.3.

**C4, the adaptation slot.** *Given the identity, the space of total reductions onto the
representable set is derived, not chosen; a strategy selects a member per operation, which is
where "the strategy changes what the correct answer is" attaches. Members classify along two
independent law roles: the adaptation laws face the source and are what error and order
transport consume; coherence faces the target and decides whether the member computes exactly in
an induced algebra. All four combinations occur.* Permanence: passes. Equivalence: passes; the
two-by-two is measured and its cells are the classification an implementation must reproduce.
Rests on: 3.4, and I9's own words for the attachment point.

**C5, the criterion.** *Whether a reduction's induced operation is associative is decided by
absorption, quantified over the values the format can hold: reducing a reachable exact value
before combining it with a stored operand must not change the adapted result. For clamped
addition this is exactly associativity; for clamped multiplication, exactly modulo operations the
clamp has collapsed to a constant. Stated over ambient values beyond the format it is a different
and stronger predicate, and the stronger form is the one that gets real folds wrong.* Permanence:
passes. Equivalence: passes; it is a checkable predicate with measured biconditional status.
Rests on: 4.1. The sign-confinement corollary is carried as a corollary, never as the criterion,
with the mutant as the reason.

**C6, the law frame.** *The laws of a format's operations are derived, not enumerated per policy.
The induced operation is associative when the ambient operation is associative on the reachable
set and the reduction's kernel is a congruence for it. The congruence half is decided by the
range's geometry per operation, mirror symmetry for multiplication and sign confinement for
addition, and is repairable by choosing the representable set. The ambient half fails wherever a
fixed-width rescale is part of the operation itself, for every policy, and nothing downstream
repairs it.* Permanence: passes. Equivalence: passes on the cube: twenty-four cells predicted in
both directions with zero residue. Rests on: 4.2, ONE EXPERT on the frame with every ingredient
separately multi-instrumented, and it is the claim this file most wants attacked next.

**C7, the scale asymmetry.** *Addition at a common scale never rescales, so an additive verdict,
survival and breakage alike, is independent of the fraction width. No multiplicative structure
survives a nonzero fraction width for any policy, sign domain, range or rescale spelling, because
the rescale destroys ambient associativity before any reduction acts. The premise of the additive
half is that addition is same-scale; whether a mixed-scale addition exists, and what result
format it infers, is an open intent question on which the premise depends.* Permanence: passes.
Equivalence: passes. Rests on: 4.3 and 4.4, with the Q3 dependency stated rather than resolved.

**C8, accumulators.** *A coherent reduction needs no accumulator: the format's own width
suffices at any fold length. For incoherent clamped addition, agreement with the once-adapted
exact answer needs the exact-sum width less one bit, because the final adaptation absorbs the
outermost bit. For multiplication no bounded closed form exists: the guard grows linearly in
fold length, and any capacity-derived accumulator statement is an additive-only mechanism.*
Permanence: passes. Equivalence: passes at the measured widths; the one-bit constant is ONE
EXPERT and the sentence survives without it (as "at most the exact-sum width"), which is how a
canon should hold a constant that is measured and not proven. Rests on: 4.5.

**C9, the chain.** *A chain is a composition of exact operations together with a schedule of
adaptation points, and the schedule is part of the function's meaning. The per-operation model
extends to chains exactly when adaptation is unfused from the operations. Chains factor into
windows, bounded subterms evaluated exactly and adapted once, whose capacity is derived by the
same typestate that derives the container. A concept that hides the adaptation inside each
operation cannot state the chain-accuracy intent at all.* Permanence: passes. Equivalence:
passes; the grades and the window are implementation-neutral. Rests on: section 5, ONE EXPERT
cold plus reconciliation, unattacked.

**C10, what the concept carries upward.** *The format concept supplies the layers above with the
width algebra of exact results, the named adaptation carrying its rounding rule, its overflow
member and its granularity function, and the exactness predicate naming the conditions under
which nothing is lost. Compositions over formats, stored pairs, intervals, error-carrying
values, are not format instances; they consume these three things and owe their own laws.*
Permanence: passes. Equivalence: passes. Rests on: section 5 and 3.6; the statability argument
is the reason the list is not optional.

**Deliberately not offered as sentences:** any count (section 7); the one-bit and fusion
constants as constants (measured, not proven, and unpriced); the symmetric-range option (a
design choice for op, unpriced, currently expressible in no register option); the wrap-order
sentence (owed, one line, but its wording depends on the comparison vocabulary no unit has
touched); `51`-style delivery-form results (unit one's, priced there as unpriced).

## 7. What the counts count

Per `RULES.md:124` and `59`'s section 1c, carried because this unit's numbers will be quoted.

The absorption biconditional's "4248 configurations" is a sweep of clamp-and-operand-box
combinations, most of which are no format at all; the number of **format-shaped** configurations
(genuine interval numerals containing zero) anywhere in the unit's law evidence is **100** in
`57_probes/p1` section 3 and the same 100-interval space again in `57_probes/p7`, plus the cube's
width instances (w = 3 through 7). That is a strength of the biconditional, holding over a wider
class than arvo formats, and a reader quoting "4248" as a count of formats checked would be
wrong. The cube's twenty-four cells are cells of a three-by-two-by-two-by-two design at 4-bit
widths and two scales, exhaustive within each cell. `35_probes/p3`'s 70.1 percent is at w = 3,
n = 8, exhaustive over 16.7M vectors, additive, same-scale. **Nothing in unit two is priced**: no
bench harness ran, every number is a count of counterexamples from a committed probe, and every
cost-flavoured remark in this file (a bit saved, a comparison per multiply, a re-encoding pass)
is unpriced and says so.

## 8. Disagreements and unanswered resumptions

The unit's convergences were real and its ledger is short, which is itself worth recording: every
disagreement that opened inside the unit was located, and most were closed by the party that had
been wrong, on the record.

**Closed inside the unit.** The wrap filing (proposed by `55`, attacked by `56`, withdrawn by
`55b` with the induced-algebra advance replacing it). The four-choice tuple (conceded to the
identity-and-realisation form). The absorption-coherence identification's scope (flagged by
`59`, measured by `61`, accepted by `57b`). The `42` framing (corrected by `59`, conceded by
`57b` in full). The shared theorem (bounded by `59`, refuted for multiplication by `58`,
withdrawn and replaced by `57b`). The clamp-factor attribution (corrected by `58` and `62`,
conceded by `57b`). `58`'s constant-savings pattern (refuted at n = 5 and under RNE by `60`,
exactly as `58`'s own withheld trust anticipated).

**Standing, precisely bounded.** `56` has never been resumed: `58`'s question on per-operation
coherence (`58:336-342`), `61`'s two questions on the C-law's window and its restatement
(`61:335-346`), and `62`'s scope sentence for the deflation (`62:259-264`) all stand addressed to
it. `55b` has not answered `57`'s ladder-vocabulary question (`57:433-435`) or `62`'s
range-symmetry question, though `57b:353-360` answers the latter on its own account. `60`'s
grade-s question to `57` (`60:551-556`) is unanswered; `57b` did not open `60`, and its H1/H2
frame plausibly contains the answer (grade s is the case where H1 holds and no reduction runs),
but nobody has said so on the record and I do not promote my own reading of it past this
parenthesis. `58` was not resumed on `60`'s and `62`'s refinements; both were in the direction
`58` itself flagged. Under `RULES.md:232-235` an expert is resumed, not replaced; resuming `56`
is the cheapest way to close three standing exchanges at once.

**The one methodological disagreement, resolved by evidence.** Whether the canon should state
the concept as four slots or as the identity-plus-realisation split was called a drafting choice
by `56` (`56:527-529`); the unit's subsequent work answered it in practice, because every later
result (the derived R, the congruence verdicts, the section-becomes-arithmetic finding) states
cleanly in the split form and awkwardly in the tuple. I record that as this consolidation's
observation, one reader's, not a settlement.

## 9. What remains open, and whose it is

Separated per the panel's rule that nothing contested or unconverged is escalated.

### Op's, with the unit's reshaping done

**Q3, mixed-numeral addition.** Open in the register ("One sentence from op collapses it",
grep-verified in Q3), and now load-bearing: the unit's strongest unconditional result, the
additive column's scale-independence, is premised on same-scale addition. The unit's reshaping
sharpens what to ask: under Q3's inferred-addition option the threat runs through **the inferred
result format and the schedule** (alignment to the join is exact; coarsening enters only below
the join), not through mixedness itself (`60:438-451`). The dedicated sweep (per Q3 option and
per result-format rule) has not run and would sharpen the question further before it is asked.

**What I7 means for product chains.** A chain of multiplies cannot be made exactly reassociable
at any bounded accumulator width, the way a chain of sums can (4.5). So "accurate especially
within chains" for the accuracy strategy means one of: an accumulator growing linearly in fold
length; windowed composite exactness with a stated bound across windows; or a stated error
budget instead of an exactness guarantee, which is what shipped DSP practice does
(`58:296-315`, `60:510-513`). Q14 currently has no line saying the choice exists. The unit
supplies the vocabulary and deliberately no recommendation.

**The signed default's cost, now fully measured.** If the default strategy is signed per I3
("It should behave like native primitives in regular old rust would", grep-verified in INTENTS
I3), then under clamping its multiplication is broken at every scale including F = 0, and under
wrapping it is exact at F = 0 and broken at every F > 0; the one mitigation, the symmetric
range, is a Q-level choice no current option names, costing one code point and departing from
two's complement (4.3). This is a consequence of stated intents meeting measurements. It is
ready for op in a way `59` said it was not yet, because P4 filled the empty cells; what it still
lacks is a price, which is the harness's to supply, not op's.

### The experts', not to be escalated

Resume `56` (three standing exchanges, section 8). Redundant encodings against the signed
order/adder exclusivity, including the order/adder theorem restated from the
no-translation-invariant-order fact (`59` section 3, untested). Subtraction, shifts and
mixed-operation chains against absorption and the congruence conditions (`57:641-643`,
`57b:391-394`). Distributivity under the symmetric range (`57b:396-400`). Round-to-nearest in
the signed cell and for wrap's F > 0 collapse (`62:371-374`, `61:447-452`). The `p_d`
two-mechanism separation over M and the operand box, and n = 6 (`60:415-419`). Fold lengths past
five and widths past seven throughout. The f32 arm and a 2Sum exactness probe under this
toolchain (`60:286-290`). Whether the statistical-error-bound alternative is expressible in the
typestate at all (`58:451-456`), for which `60`'s per-window counting contract is one candidate
shape (`60:544-549`). The coherence-direct second mechanism's characterisation (`61:435-441`).
Whether the H1/H2 frame survives an operation whose ambient is not commutative, which nobody has
posed. And the Q3 sweep in `60`'s sharpened form, which is the experts' half of an op question.

## 10. Fits against the register, and what it should gain

**Kills nothing.** No live option anywhere in the register is closed by this unit. What follows
is the aggregate of the members' reported gains, deduplicated, with what has already landed
distinguished from what has not, because the register visibly lags the unit's tail: Q17 was
updated through `60` (it carries probe D's correction) and not after; the Q12 caution still
reads "a second read of `42` against `55_probes/p5` is owed before this paragraph is rewritten"
(grep-verified in Q12), and that second read is `57`, so the caution is dischargeable; the
wrapping entry carries `55b`'s amendments and nothing later.

**Q12.** Replace the mechanism paragraph and its caution with absorption as the criterion,
`61`'s domain qualification stated as the law's subject, the multiplication exception named as
the collapsed class, and the sign-confinement corollary as a separate consumer-checkable line
(`57` section 5, `57b` section 9, `61` section 6). The droplist entry for clamp-counting should
say what actually happened, per `59:439` and `57b:96-99`: `42` refuted the hypothesis itself and
named the survivor; the closed thing is one prose sentence, with `42_probes/p3.out` row one as
the diagnostic.

**Q17.** Gains the wrap-ring row (measured, and it collapses: ring only at F = 0, additive
abelian group every F, below semiring at every F > 0; `61` section 6). Gains the signed rows and
the failure-set coincidence at deep fraction (`62` section 5). Gains the identification-scope
sentence (`61`) and the H1/H2 frame as the entry's organising statement rather than another row
(`57b:380-382`). Gains `60`'s probe A as the constructive F = 8 window instance beside `58`'s
eager impossibility.

**Q11.** The structure-naming option should state that the induced structure is **derived from
congruence verdicts**, one per operation, computed from the policy, the scale, the sign domain
and the range's symmetry, rather than growing a parameter list (`57b:374-378`, absorbing `57`'s
scale condition, `61`'s wrap extension and `62`'s two parameters). The accumulator option and
the "both" option gain the additive-only qualification (already partially in Q17; not yet in
Q11), plus the one-bit and no-accumulator-when-coherent content (`57` section 5).

**Q5.** Gains the rescale-spelling instance: the register carries rounding as "a candidate
fifth axis absent from arvo entirely" (grep-verified in Q5), and the spelling is that axis made
concrete, coinciding on unsigned operands and diverging on signed in every count and in the
accumulator grade (`62` section 5). Also gains, from `57` and `58`, the two-axes-two-law-costs
evidence both files filed.

**Q6.** Gains the measured signed cost of clamping and the symmetric-range mitigation's
existence (4.3), which cuts on grounds independent of the bench families the entry currently
argues from.

**Q14.** Gains the product-chains line (9's second item).

**Q3.** Gains the dependency line: what now rests on it, pointing at Q17's additive column, in
`60`'s sharpened form.

**The wrapping entry.** Gains `61`'s scope sentence for the coherence law and `62`'s F = 0 scope
on the deflation's "relabelling" claim (3.2).

**Droplist candidates from the unit, each with its diagnostic and reopen condition stated by its
author:** clamp-counting (closed; `57` section 5, wording per `57b`); "the unsigned-saturation
semiring transfers to fractional formats" (`58` section 5, structural); "wrap's induced ring
transfers to fractional formats" (`61` section 6); "signed two's-complement saturating
multiplication induces a semigroup at some width" and "the wrap section is a relabelling at
every scale" (`62` section 5).

## 11. Doability, established, with the evidence

The canon must say which things are doable (`RULES.md:85-87`). Established in this unit, all on
the pinned `nightly-2026-05-28`, all committed, all re-run byte-identically here:

The affine membership predicate written once and instantiated across integer, fixed and float
value sets, with mutants detected (`55_probes/p1`, `56_probes/q2`). The two-law-role
classification and the induced-algebra grades as measurable properties of any total reduction
(`56_probes/q1`, `55_probes/p4`). Absorption as a checkable predicate with measured
biconditional status, and the congruence verdicts with closed-form predictors over the
zero-containing intervals (`57_probes/p2`, `p2b`, `p7`, `p8`). The H1/H2 frame as a mechanical
evaluation with no per-cell special casing (`57_probes/p9`). The width algebra as trait
contracts, gate-free, with the general spelling's refusal on record as the negative control
(`60_probes/p_c1`, `p_c2.stderr`: statable under the feature ban in bounded-enumeration or
type-level-contract form, exactly the refused-bound rule's shape). Composite correct rounding
demonstrated constructively for a multiply-containing window at F = 8 against a
property-checking oracle (`60_probes/p_a`). The negative results that bound the space: no
fixed-width eager multiply supplies an associative ambient at F > 0 (argued structurally,
measured at three instruments); no bijective signed encoding holds both raw order and the raw
adder (`56_probes/q3`); no rounding rule zeroes a fractional multiplicative failure count in any
probe that varied one (`58_probes/p1`, `60_probes/p_d`, `62_probes/p4`).

**Not established:** any magnitude. Nothing in this unit prices anything, and the canon sentence
that would need a price (whether the symmetric range, the one-bit saving, or any accumulator
strategy is worth taking) cannot be written yet.

## 12. Process record, for the audit trail

Unit two inflated no rung, and the reason is visible in the files: every member stated its
reading order, named its first-read claims, and re-ran the probes it argued with before arguing
(`59` section 1's audit, confirmed by my own reads; my rerun of all thirty-two instruments
closed the loop). The unit's one compression failure happened at the dispatching layer and was
named by the dispatcher against its own brief (`59:100-109`), then conceded by the expert whose
framing fed it (`57b:72-99`). Six first-run failures are kept on disk with their hypotheses
attached (`57_probes/p4` v1, `p5`'s overstated first reading, `58_probes/p2` v1,
`62_probes/p3`'s non-mutant mutant, `57_probes/p7` v1 and v2), and two of them produced the
unit's findings (the second mechanism in `58`, the closed-under-the-operation asymmetry in
`57b`). The repository-state flag `57` raised at its section 0 was checked and discharged by
three later files and withdrawn by its author (`57b:66-68`). The cold-derivation protocol was
run twice (`55`, `60`), both commit-verified cold, and both phase ones survived reconciliation
with their cores intact, which is the protocol doing what it was designed for. The two
resumptions (`55b`, `57b`) each conceded real ground and each delivered a stronger replacement
than the position it gave up, which is what `RULES.md:221-243`'s argue-then-converge shape looks
like when it works.

## 13. Coverage, bounded honestly

**Read end to end:** `55` (both phases), `55b`, `56`, `57`, `57b`, `58`, `59`, `60` (both
phases), `61`, `62`, `53`, `INTENTS.md`, `00_brief.md`, `RULES.md`.

**Read at the source:** the probe files and outputs named in section 1, including
`61_probes/q1_output.txt` and `57_probes/p6_output.txt` and `57_probes/p9_output.txt` in full
with line numbers, and the four source passages whose scale-blindness or clipped-add shape this
file leans on.

**Register sections read:** Q3, Q5, Q6, Q11, Q12, Q13, Q14, Q17, the wrapping entry, the
unasked-questions section. Cited by section plus grep-verified phrase throughout.

**Re-run:** all thirty-two committed instruments of unit two, byte-identical, outputs in
`63_probes/rerun/`. **Verified in git:** both cold derivations' phase ordering.

**Not done, and what it leaves unverified.** I did not open `08`, `42`, `35`'s prose, `18`,
`20`, `25`, `40`, `43`, `50`, `54`, `seed/`, `DROPLIST.md`, or `archive/`; every claim here that
touches them is attributed to the member file that read them, and if `57`'s reading of `42` or
`55`/`56`'s reading of `08` is wrong, sections 4.1 and 3.3 inherit the error. The carried
scoping theorem behind 3.6 was not re-derived by anyone in this unit or by me. I built no new
instrument, so nothing in this file extends the unit's evidence; it re-verifies and arranges it.
The rung table is my derivation from the member files, and per `53:606-609`'s warning, a checker
should re-derive it rather than inherit it, because this file is now the most convenient thing
in the unit to compress from, which is exactly the condition under which drift starts.

**This file's own check.** Per `RULES.md:309-319`, the entailment check on this consolidation is
run by someone who did not write it, from the member files forward, counting and diffing the
citation sets on both sides. Every `file:line` this file relies on is kept in the body rather
than compressed away; the checker should run the diff rather than trust that sentence.

**Nothing here settles anything.** The mode is explore, there is no canon, and this file goes to
op as the topic's candidate, with sections 8 and 9 carrying what is open and whose it is.
