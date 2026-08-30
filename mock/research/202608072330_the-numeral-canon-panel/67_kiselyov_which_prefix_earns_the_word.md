# 67. Which prefix earns the word: the number-system concept is a telescope, not a tuple

**Author lens:** Kiselyov. Typed object languages, the syntax and semantics split, interpretations as
folds, what a type must carry for a property to be decidable at the type checker.
**Position:** third file of the number-systems unit's first four, after the two blind cold
derivations `65` and `66`. `68` landed while this file was being written and is read and answered in
sections 6 and 8; it did not exist when my probes were designed, and where it reaches my ground
first I say so.
**Probes:** `67_probes/`, five instruments, committed with this file at `f449353b` and `95e4078a`,
outputs and refusal transcripts beside them. Pinned `nightly-2026-05-28`, zero feature gates, no
`dyn`, no `TypeId`.

**The assigned question.** What is a number system as a canon-level concept, which ones must the
canon account for, and, specifically, whether a system's selected adaptation belongs to its identity
or to its realisation, where `65`, `63` and `66` hold three different positions and two of the three
say the difference is nominal.

**My answer to that last question in one line, before the argument.** The difference is nominal for
code and substantive for the canon, which is the artifact being written, and the reason is that the
concept is a **dependent sequence of choices** rather than a tuple of independent ones. Where you cut
that sequence decides what every universally quantified canon sentence is quantified over, and this
panel has now recorded **three separate instances** of getting exactly that quantifier wrong.

## 0. Gates, and what I read

**Canon gate: passes, situation two.** `mock/canon/` does not exist, `mock/crates/` is empty by the
declared mutation order, and this panel is writing the first canon. Nothing binds but op's intents.
I re-read `INTENTS.md` in full before writing. Nothing below settles anything.

**Test gate: no suite exists.** `mock/crates/` holds nothing, so there is nothing to run. The
substitute is the probe discipline. I applied it to my own five instruments (each has a stated
hypothesis, each has an output or a refusal transcript on disk, one carries a prediction its own
output refutes and keeps both) and, where I depend on another member's evidence, I say which and how
far. I did not re-run `65`'s or `66`'s probes; `68` did, thoroughly, and section 8 takes its findings
rather than duplicating the work.

**Read end to end:** `INTENTS.md`, `RULES.md`, `00_brief.md`, `65` (both phases), `66` (both
phases), `63`, `64`, `68`, `OPTIONS.md` in full including the Q18 through Q28 section added while I
was reading, `DROPLIST.md` in full.
**Opened at the source to check a specific claim:** `63` section 8 in full (for section 8's first
attack), `65:511-513` and `66:516-524` (the same attack), `66:60-68` (the crate-table use),
`00_brief.md:144-163` (the acceptance criterion and what the brief calls fixed).
**Not read:** `01` through `62` except as reached through `63`, `64`, `OPTIONS.md` and `DROPLIST.md`;
`seed/`; `archive/`; `35` beyond the grep-located lines quoted in section 7. Every statement here
about `16`, `35`, `42`, `51`, `55` through `62` is sourced to `63`, `OPTIONS.md` or `DROPLIST.md` and
inherits their errors if any. I built no bench and nothing here is priced.

**One thing to report outside the question, per the standing instruction.** The brief's list of what
is fixed includes "the workspace discipline: `#![no_std]`, no `alloc`, no `dyn`, no `TypeId`, sizes
const" (`00_brief.md:155-156`). **None of that appears in `INTENTS.md`**, which is the catalogue of
op's own words and which says of itself that anything he has said and that is not quoted there is
missing rather than excluded. The no-`dyn` constraint is what makes monomorphisation the dispatch,
which is what makes erasure mean anything, which is what `65`, `66`, `68` and this file all build on.
It is currently asserted in two agent-authored places (the brief, and the generated
`.claude/CLAUDE.md` that `65` just showed was stale in other respects) and quoted from op in neither.
That is not a refusal to proceed: the constraint is almost certainly real and it is the workspace's
standing discipline. It is a gap in the catalogue, and the remedy is the one `INTENTS.md` names for
itself, which is to find op's words and add them, or to mark the constraint as workspace discipline
rather than as arvo intent so a later reader does not cite it as the latter.

## 1. The located disagreement, and why it is not nominal

`65` phase one folds the correctness relation into the system's identity: wrap and saturate over one
window are two systems. `63`'s C2 puts adaptation choice outside identity: a format is (D, Q), the
reduction space is derived, and a strategy selects a member per operation. `66` phase one hit the
same fork blind and carried it open.

Then both cold derivations, on reading the panel, converged on the same reconciliation. `65` phase
two: "the difference is **mostly nominal and worth one sentence of care rather than a fight**", and
proposes to restate its own definition as a *view* over the unit's decomposition. `66` phase two: the
fork "**dissolves rather than needing to be picked**", because both readings are true at two
different named layers.

**Weigh that convergence carefully, because it is the weakest kind this panel recognises.** Phase one
of each file was blind, and their phase-one agreement (that whatever sits at the top of the hierarchy
is identified by its laws) is two instruments over one premise set, which is the discount `65` itself
applies. Their **phase-two** agreement is not that. Both read `63` before writing it, both took
`63`'s vocabulary, and both then reported that the disagreement they had with `63` had dissolved.
Two files agreeing after reading the thing they were disagreeing with is a read, not corroboration,
and the panel's own rule says so in as many words. So the "it is nominal" verdict currently sits at
ONE EXPERT twice over, not at TWO.

I attacked it and it does not hold. Here is the attack.

**The nominal case is real at the type checker, and I grant it in full.** Every arvo numeral carries
its strategy in its type, so a Rust type names a full assignment across every coordinate whichever
vocabulary you use. Nothing a compiler does distinguishes the cuts. `p1` and `p5` both demonstrate
this from the inside: a law contract bounded on the (identity, adaptation) pair behaves identically
whether you call that pair "a system" or "a format plus a selected member". So for **code**, the two
files are right and the dispute is naming.

**The substantive case is about what a canon sentence quantifies over, and it is not nominal at
all.** A canon states intents in the form "for every X, P". Cutting the concept at a different depth
changes what X ranges over, and therefore changes the truth value of P without changing a word of P.

`p1_telescope.rs` makes this mechanical rather than rhetorical. Take one ambient domain and one
representable set, the signed 4-bit window. Two adaptations over it. A law contract naming
associativity of the induced addition. Then:

- Quantified over **(identity, adaptation, encoding)**: the contract is decided, and it is decided
  the same way for both encodings over that pair. `t3_law_does_not_read_the_encoding` compiles with
  two's complement and with excess-K and the same bound accepts both.
- Quantified over **(identity, adaptation)**: decided. `reassociating_fold` accepts the wrapping
  member.
- Quantified over **identity alone**: **not decided**, and the demonstration is a compile failure.
  `p1_neg_b.stderr` carries `error[E0277]: the trait bound Saturate<Id<RingZ, S4>>: AddAssociates is
  not satisfied` at the same ambient domain, the same representable set, the same encoding and the
  same container as the accepted case. Only the adaptation moved.

So a canon sentence beginning "for every format" means something different under `63`'s cut than
under `65`'s, and one of the two meanings is false for the law layer. That is not a naming question.
It is the question of whether the canon's default quantifier is the one its law sentences need.

**And the panel has already paid for this three times.** `61` found `56`'s coherence law stated
without the restriction to the representable set, and `57b` accepted the correction with the framing
that the restriction "is not a caveat on the law but the statement of the law's subject" (`63`
section 4.1). `57b`'s own `p7` then failed twice by, in `63`'s words, "the same over-quantification
`61` caught in `56`'s coherence law, made by the adjudicator after reading the correction". And
`42`'s clamp-counting sentence was refuted for the same reason: it quantified over a class its own
published table contradicted (`63` section 4.1). Three instances, two of them by authors who had just
read the correction to the first.

**A concept whose sentences keep acquiring the wrong quantifier does not have a quantifier problem.
It has a shape problem**, and section 2 says what the shape is.

## 2. The shape: a telescope, not a tuple

`55` proposed the concept as a four-choice tuple, `F = (D, Q, R, E)`. `56` attacked it. `55b`
conceded that R is **derived** rather than chosen. `56` separately established that E is **ordered
after Q**: the value set is chosen, then realised. `63` section 8 records the surviving methodological
observation, that every later result "states cleanly in the split form and awkwardly in the tuple",
and files it as one reader's observation rather than a settlement.

Both of those discoveries are the same discovery, and nobody has said what it is. **A tuple is a
product: its components are independently choosable, and any combination is a member. What the unit
found is that these components are not independently choosable, because each one ranges over a set
that the earlier ones determine.** That structure has a name in type theory and it is not a product.
It is a **telescope**: a dependent sequence, where each later component's type mentions the earlier
components.

Written in the notation a canon may carry:

    Format  :=  (D : Ambient) x (Q : Reach(D)) x (rho : Reduce(D, Q)) x (E : Encode(Q)) x (C : Hold(E))

The parenthesised arguments are the whole content. `Reduce(D, Q)` is `63`'s "the space of total
reductions onto Q is a derived mathematical object" stated as a dependency rather than as a remark.
`Encode(Q)` is `56`'s ordering stated the same way. `Hold(E)` is the container. And the arrow of
dependency runs one way only, which is why the tuple form kept reading awkwardly: it offered a
symmetry the object does not have.

`p1_telescope.rs` establishes that this dependency is **enforceable and not merely describable**, on
the pin, gate-free. Each component after the first carries an associated type naming what it is over.
The completed term names only its **last** component and recovers every earlier one by projection
(`IdentityOf`, `AmbientOf`, `ReachOf`), which is the telescope's chain made into the type's shape.
Attaching a component declared over one identity to a term at another is refused:
`p1_neg_a.stderr` carries `error[E0271]: type mismatch resolving <TwosComplement<Id<RingZ, U4>> as
Encoding>::Of == Id<RingZ, S4>`. And the whole apparatus erases: four `const` assertions that the
completed term has the container's size, discharged at compile time.

Three consequences, and the third is the one that answers the assignment.

**First, "how many outputs does the derivation have" is a question about the chain, not about an
arity.** `47` reached the same place from the other end and the register carries its sentence: "the
derivation's result must make available, as types, every fact a lowering site cannot recompute from a
const", with the note that once the single output is a type with named projections it **is** the pair
wearing one name. That is the telescope's completed term described from the outside. Two derivations
from opposite directions, and I did not read `47` (I have it through `OPTIONS.md`'s account, which I
say rather than imply).

**Second, the mutation order the workspace already runs on is the same shape.** A later component may
be re-chosen freely; an earlier one invalidates everything after it. That is not a cute analogy, it
is the same dependency relation, and it is why the panel's instinct that "encoding is realisation"
felt obviously right without anyone being able to say what made it later.

**Third, identity is a prefix, and which prefix earns which word is the whole dispute.** Section 3.

## 3. Which prefix earns the word, and how to decide it without arguing

Under the telescope, "is the adaptation part of identity" is not a question about the nature of
number systems. It is the question **at which index the equality relation is taken**. Two terms agree
at prefix k when they agree on the first k components. Every prefix induces an equality, and each is
a different relation on the same objects.

That reframing does two things. It explains why both cold derivations concluded the fork was nominal
(every prefix equality is definable, so no cut is *wrong*), and it says exactly why that conclusion is
insufficient (a canon needs a word for each prefix that some consumer's question is asked at, and
prefixes with no consumer need no word).

So the question becomes a **caller census**, which is an instrument this panel has already used to
good effect: `06` classified every site where a numeral appears that the consumer did not spell, and
found the lattice's meet and join had no located caller after two independent looks. The same
question here, from the panel's own record:

- **Prefix 2, (D, Q).** Has callers. `63`'s C2 says it "decides format equality, which Q10 needs and
  which the four-tuple left unstated", and Q10 is op's own open question about whether the inclusion
  order identifies shapes denoting the same value set. A conversion predicate asks at this prefix.
- **Prefix 3, plus the selected reduction.** Has callers. Every law bound an algorithm crate places
  is asked here, demonstrated at `p1_neg_b` and `p5_neg`. `63`'s own cube is a table of prefix-3
  facts.
- **Prefix 4, plus the encoding.** Has callers, and they are pattern-level: `56`'s raw-order and
  raw-adder properties are exactly the questions asked at this prefix and at no earlier one.
- **Prefix 5, plus the container.** Has callers, and section 6 argues with `68` about what they are.
- **Prefix 1, the ambient domain alone.** Has a caller nobody has named. Section 7.

**So the canon needs at least four words, not one, and the dispute between `65` and `63` is which of
two prefixes gets to be called "the number system".** Both cuts are internally coherent; what is not
coherent is having one word and four consumers.

And there is a fifth thing to name that is not a prefix at all: the completed term. That is what a
type is, and op's criterion is about producing one.

**Which gives a reading of the acceptance criterion I have not seen stated.** "Have the typestate
derive the matching container and numeral representations, then validate, and erase" is, under this
shape, one sentence: **the consumer supplies a prefix, the typestate completes the telescope,
validation checks that the completion satisfies the constraints the prefix imposed, and erasure
discards the completion's evidence while keeping its consequences.** Derivation is completion from a
prefix. That is a permanent statement, it mentions no mechanism, and it makes op's plural fall out
rather than needing to be accommodated: a completion may branch, and the branches are `65`'s roles.

It also says something about Q2 (which coordinates a consumer writes) that Q2 does not currently say:
**which prefix the consumer supplies is a design choice, and different consumers may supply different
prefixes.** So the derivation is not one function but a family indexed by the supplied prefix, which
is a sharper statement of what Q2 is choosing between than "which pair of numbers".

## 4. Interoperation: three crossings, and the panel has a word for one of them

`66` asks whether interoperation is conversion or resolution, and files them as two separable
questions. That split is right and it is not deep enough. Under the telescope, **a crossing is
identified by the smallest index at which the two terms disagree**, and the index decides what the
crossing can preserve.

`p2_three_crossings.py` measures all three exhaustively at the 4-bit model width, asking two questions
of each and never conflating them: does the map preserve values or patterns, and does it commute with
the operations.

**X1, restrategise.** Disagreement first at index 3. Same ambient domain, same representable set,
same encoding, same container; only the selected reduction moves. The value map is the identity, so
16 of 16 values are preserved, bijectively, by construction. The operations are **not** preserved:
wrap and saturate agree on 192 of 256 operand pairs for addition on the signed window and 111 of 256
for multiplication, and on the unsigned window 136 of 256 and 80 of 256.

**X2, widen.** Disagreement first at index 2. The value map is the inclusion, injective, 16 of 16.
The operations are preserved on 192 of 256 pairs for addition and 101 of 256 for multiplication, and
under both policies identically, because agreement is exactly the sub-box where the narrow numeral's
adaptation never fired.

**X3, reinterpret.** Disagreement at index 1. Same container, same bit patterns, different ambient
operation family. Every pattern is carried, 16 of 16, and nothing else is: wrapping addition agrees
with xor on 108 of 256 pairs and with min on 1 of 256.

**The result is uniform and it is the finding.** Every crossing is total and preserves values or
patterns at 100%. **No crossing preserves operations at 100%.** The two properties are independent,
and a vocabulary with one word for "conversion" is naming the first and is silent about the second.

That silence has a cost, and it falls on the crossing that looks free. X1 changes nothing a consumer
can see in a stored value and changes what every operation computes. `65` section 9 reached the
consequence from the other direction and stated it well: a strategy change can change which
algorithms are willing to compile against the type, and the canon should say so out loud so nobody
files it later as a usability regression. `p5` is that sentence made mechanical, and section 5 gives
the numbers behind which algorithms.

**What the crossings need from each other, stated at intent level.** A crossing declares which of the
two it preserves. Preserving values is a statement about the carrier and is checkable from the two
representable sets. Preserving operations is a statement about the two induced algebras and is
checkable from the two law inventories. **A crossing that preserves both is a homomorphism of
algebras; one that preserves only values is a bijection of carriers and transports nothing.** The
panel's `ConstFrom` / `ConstTryFrom` shape (`66` names it, from a dead-tree design document, correctly
marked as evidence rather than adoption) answers the first and is silent on the second, which is fine
as a mechanism and is not an answer to the question `66` asked.

I have no good name for the value-preserving, operation-destroying crossing and I decline to coin one.
Naming it badly now is worse than carrying the gap, and the gap is stateable: the canon needs to
distinguish carrier identity from algebra compatibility, and X1 is the case that has the first and
lacks the second.

## 5. The two law families are two consumer classes, and one cell serves both

`63` section 3.4 records that a reduction carries two independent law families, the adaptation laws
facing the source and the coherence law facing the target, with all four combinations inhabited so
neither subsumes the other. That is the panel's, measured, and I did not derive it.

**What nobody has said is who needs which.** That is answerable, it connects two units, and it
predicts a measurement the panel already has from a completely different direction.

The consumers op names as arvo's selling point (I11: "our main selling point are the algo crates")
are, in `35`'s words as the register carries them, semiring computations, "and the ones the algorithm
layer holds are mostly tropical": min as the additive operation, the numeral's own addition as the
multiplicative one. So the graph crates' arithmetic is min and plus, not plus and times.

**The derivation, which I wrote into `p4`'s header before running it so the probe could refute it.**
`min` is closed on the representable set, so it needs no adaptation at all and inherits its ambient
laws unconditionally; `57b`'s H1 and H2 are vacuous for it. The tropical structure needs addition to
distribute over min. In the integers, `min(a,b) + c` equals `min(a+c, b+c)` exactly, by translation
invariance of the order, so distributivity holds for all operands **exactly when the reduction
commutes with binary min on the reachable set, which is exactly when it is monotone there**. And
monotonicity is the first of `56`'s adaptation laws.

`p4_two_law_families_two_consumers.py` checks it exhaustively at 4 bits over two windows and three
reductions, including `56`'s opposite-bound mutant as the neither-cell control. The biconditional
holds in **6 of 6 cells**, with both sides observed true and false across the cube so it is not
holding by accident: monotone in 2 cells, non-monotone in 4, distributive in exactly the monotone
two.

The cross-table is the point:

| window | reduction | tropical consumer | reassociating fold | both |
|---|---|---|---|---|
| unsigned [0,15] | wrap | no | yes | no |
| unsigned [0,15] | saturate | yes | yes | **yes** |
| unsigned [0,15] | opposite-bound mutant | no | no | no |
| signed [-8,7] | wrap | no | yes | no |
| signed [-8,7] | saturate | yes | no | no |
| signed [-8,7] | opposite-bound mutant | no | no | no |

**One cell of six serves both consumers, and it is unsigned saturation over a nonnegative window.**
That is `63` section 3.4's both-cell arriving from the consumer side, and it is also, exactly, the
workload shape of nonnegative shortest path. `35`'s independently measured table says the same thing
in algorithm terms: saturating min-plus folds diverge at 0, wrapping ones at 45.4% and 48.9% at two
widths.

Three things follow, and they are the contribution.

**The mechanism behind `35`'s headline is the reduction's monotonicity, not a property of the
algorithm.** `35` and the droplist attribute the interior-wrapping failure to monotonicity already
(the droplist's entry names 560 of 2176 monotonicity failures), so this is not new in kind. What is
new is that it is **derivable from `56`'s law families** rather than measured per algorithm: the
tropical consumer needs the adaptation laws, and every reduction that has them supports min-plus, at
every width, without a sweep.

**The two law families are not two technical properties of equal standing. They are the requirements
of two consumer classes that the panel has been treating as one.** Coherence is what a fold that
splits, threads or reassociates needs. The adaptation laws are what order transport and error
transport need, and the tropical algorithms are the load-bearing consumer of order transport. `63`
says the families "face" the source and the target; this says who is standing on each side.

**And the default the imitation intent points at is in a cell that serves one or the other and never
both.** `63` section 4.3 already establishes that the signed cell is where the algebra is worst and
that a general-purpose default under I3 lands there. This sharpens the cost: at the signed window,
wrapping serves the fold consumer and breaks the tropical one, saturating serves the tropical one and
breaks the fold consumer, and no cell serves both. That is a consequence of stated intents meeting
measurements, it is unpriced, and it is not mine to resolve.

**Bounds, stated.** One model width, exhaustive within it. Addition only under the tropical reading
(the tropical product is the numeral's addition, which is the table's fourth column). Nothing at
nonzero fraction width. And one oddity kept on the record rather than smoothed away: the tropical
violation count is **1360 in all four non-monotone cells**, across two different windows and two
different reductions. A repeated number is the shape of a counting bug, so I wrote `p4b_recount.py`
from scratch rather than editing `p4`; it reproduces 1360 in all four, and the underlying commuting
failures counted unweighted differ (240, 450, 464, 562), so the reductions are genuinely different
functions and the collision is in the weighted count alone. I did not explain it and I claim nothing
from it.

## 6. Shared parameters over a run, and the level that is not a level

`68` section 6 attacks both hierarchies at the bottom: both are value-centric, and the container tier
fails on the panel's own strongest aggregate result, because a packed element has no per-value
container to occupy a level. I read that after building `p3` and I think it is right, it reaches my
ground first, and it is the better statement of half of what I had. What follows is the other half,
which `68` does not cover and which arrives at the same repair from the identity side rather than the
realisation side.

`63` section 3.6 files stored-pair rationals, intervals and error-carrying pairs as compositions over
formats. Every one of those is a **point composition**: one datum is a tuple of numerals travelling
together. There is a second shape, and the panel has one instance of it shipped and unnamed and
another excluded by a sentence that does not distinguish them.

`p3_shared_parameter.py` applies `63`'s own identity condition mechanically. The condition is C2's:
"the representable set is a constant of the type: a value set that depends on other data is not a
format but storage."

- **Block floating point**, a run of 4-bit mantissas sharing one 3-bit exponent: **8 distinct
  representable sets** over the element type, one per block exponent, ranges from [0, 15/8] to
  [0, 240]. Q is not a constant. **Fails the test.**
- **A packed run**, 4-bit elements at strides 4, 5, 8, 16: **1 distinct representable set**. The
  shared parameter moves the bit offsets (at stride 5 the first eight elements sit at offsets
  0, 5, 2, 7, 4, 1, 6, 3) and moves no value. **Passes the test.**
- **A self-contained float** of the same arithmetic, exponent per element rather than shared: one
  representable set of 72 values. **Passes.** This is the control, and it is what shows the test
  separates *shared* from *per-element* rather than separating "has a scale factor" from "does not":
  the block and the self-contained float encode the same arithmetic and land on opposite sides.

**So arvo already ships one shared-parameter aggregate and excludes another with the same sentence,
and the sentence does not distinguish the two shapes. It distinguishes which layer the shared
parameter sits at.** `Cold`'s stride is a shared parameter at the realisation layer; block floating
point's exponent is a shared parameter at the identity layer. The register's own words for the first
are that `Cold` "is not a container choice with a field attached; it is a statement about how a run
of values composes, and this is why it has no standalone value form at all", reached independently by
two members.

That is the same finding `68` reaches, from the other end. `68` says the container tier is not a
level because the storage answer is two questions keyed differently. I say the element's *identity*
is not self-contained either when the run carries part of it. **Both are the observation that the
telescope's components can belong to an aggregate rather than to a value**, and it can happen at
either end of the chain.

Which gives the repair, and it composes with `68`'s rather than competing: **a canon that names
compositions needs two words, not one.** A point composition is a datum made of numerals and owes its
own laws. A shared-parameter aggregate is many data plus a parameter that participates in each one,
and what it owes depends on the layer: at realisation it owes a layout discipline and nothing
arithmetic, at identity it owes the arithmetic the shared parameter takes part in. Q16 currently
carries two senses of "composition" (a numeral kind bound to a strategy, and an aggregate over
numerals) and this is a third distinction inside its second sense, which `43` reached from yet another
direction with its "capacity static, length dynamic" boundary.

Whether block floating point should ever be admitted is not my call and I do not make it. What `p3`
establishes is that the panel's current sentence excludes it for a reason that also excludes nothing
about `Cold`, and that the two are the same structural shape at two different depths.

## 7. What the canon must account for: one addition to the kernel, from the callers

`65` derives a kernel of six items, each from a quoted intent, and an open ring of admissible
extras. I re-derived the kernel independently before reading section 5 of `65` and reached a subset
of it, so I have nothing to add by that route and say so: keeping `65`'s kernel is the result.

One thing is missing from it, and it comes from the caller census rather than from the intents read
alone.

**The ambient domain's operation family is a choice, and the canon has never said whether it is
fixed.** `63`'s C2 identifies a format by (D, Q) where D is the ambient domain, and the entire law
layer, every absorption result, every congruence verdict, the whole H1/H2 frame, the twenty-four-cell
cube, is about one D: the integers with plus and times. Nothing in the panel asks whether one Q can
carry two operation families. `p2`'s X3 says it can: the same 4-bit patterns under (Z, +) and under
GF(2)^4 and under (Z, min) are three ambient domains over one carrier, agreeing on every pattern and
on almost no operation.

And the caller is not hypothetical. `35` measured the tropical semiring as the algorithm layer's
actual arithmetic. `65` section 9 names it and files it as an algorithm's law bound. Under the
brief's own founding sentence, which places "number systems and derived algebraic laws underneath"
the format concept, the tropical semiring is not a law bound on an existing system; **it is a second
ambient domain over the same representable set**, and it is the one arvo's named selling point
computes in.

Section 5 shows the consequence is not merely taxonomic. The law layer's per-operation instruments
cannot see the tropical failure, because that failure is in a **two-operation law**, distributivity of
the numeral's addition over min, where one operation is closed on Q and the other is not. Absorption
is a per-operation predicate. The congruence verdicts are per-operation. The cube's distributivity
column is for plus over times. **Nothing in the panel's law layer is quantified over a pair of
operations from two different families**, and the one law the graph crates actually need is exactly
that shape.

So the kernel gains an item, and I state it as a question rather than an answer because the mode is
explore: **is D's operation family fixed at (+, x), or is it a parameter?** If fixed, the tropical
algorithms compute in something arvo's concept does not describe, and the algorithm crates carry the
description. If a parameter, then one representable set carries several ambient domains, prefix-1
equality becomes a relation somebody has to define, and the law layer's quantifiers change again.
Both are coherent. Neither is free.

**And one observation that costs nothing under either answer.** Operations **closed** on the
representable set need no adaptation at all and inherit their ambient laws unconditionally. `min`,
`max` and the order comparisons are such. So the adaptation apparatus, and everything the unit built
on it, is about the non-closed operations specifically, and a canon sentence saying so bounds the law
layer's own subject correctly. `p4` measures the closure directly: min needs 0 adaptations over Q
squared in all six cells.

## 8. Attacks, with citations, and one support

**`66` miscites `63` and inflates a rung on the strength of it.** `66:520-523` writes: "the panel's
own `63` section 8 independently states the same shape more sharply still: 'canonical at storage and
interchange, a tool at compute, with the normalisation law validated.'" It then concludes: "Two
independent sources landing on a three-way split this file only found two of the three roles for is a
real refinement to adopt, not merely note."

Both halves are wrong, and one command shows it. `grep -n "canonical at storage" *.md` in the panel
directory returns exactly two hits: `66:521`, the quotation, and **`65:511`**, which is `65`'s own
candidate 7. `63` section 8 is "Disagreements and unanswered resumptions" and contains no redundancy
material at all; `grep -c -i "interchange" 63_spj_consolidation_the_format_concept.md` returns **0**,
so the sentence's own vocabulary appears nowhere in the document it is attributed to.

So the "two independent sources" are one source cited twice: `65`, quoted once by name in the
preceding sentence and once more as `63`. The three-role model is ONE EXPERT and `66` adopted it as
corroborated. This is the panel's most-recorded failure mode arriving in a file that is otherwise
careful, and it is the fourth or fifth instance depending on how one counts `63` section 12's ledger.
The remedy is one word: `66`'s paragraph should say `65` where it says `63`, and drop the "two
independent sources" sentence. Nothing else in `66` moves.

**`66`'s use of the generated crate table as a cross-check: `65` attacked it, and I second it, having
reached the same conclusion before reading `65`'s phase two.** `66:60-68` uses `Bits<N, S>` in
`arvo-storage` and `UFixed` layered over it to claim its five-level hierarchy "predicts the layering
the existing crate table already describes", calling that "a genuine cross-check, not a coincidence
manufactured to fit". The crate table describes the nuked tier. `mock/crates/` is empty on purpose,
the canon-design-code chain says an agent consulting a dead dependent tier during canon work "is
reattaching a tier that had to be detached for the edit to be permitted", and a hierarchy predicting a
dead design's layering is corroborated by nothing. That the generated instructions were on the
permitted reading list explains the reach and does not license the use. `68` section 7 seconds it too,
so this now has three readers and is the closest thing in the unit to a settled correction. `66`'s
hierarchy does not need it and stands on op's two nouns without it.

**A drafting hazard in `65`, smaller and worth naming because a consolidator will hit it.** `65`'s
candidate 4 reads "the format does not determine the system: one container hosts many systems, and
the canon types the system, not the box." That sentence is true in `65`'s vocabulary, where "format"
means a representation pinned to a container. Read in `63`'s vocabulary, where a format is (D, Q), it
says something else entirely and something `63`'s C2 already covers under "encoding is realisation".
`63` was careful about this and flags its own vocabulary as working names put to op as nothing; `65`
offers twelve candidate sentences with no such flag. **Two units now use "format" for different
prefixes of the same chain**, which is a live collision of exactly the shape `24` found for "phase"
and the register still carries as open. A candidate sentence that changes meaning between units is not
a canon candidate yet, whatever its content.

**`68`'s finding that "validate" is two verbs: I support it and it reaches my ground.** Its diagnosis
is that `65`'s validate is compile-time per type and `66`'s is a runtime predicate per datum, and that
neither reconciliation noticed because each checked the other's hierarchy and not the other's verbs.
Under the telescope this is a clean statement rather than an awkward one: **the compile-time verb
checks the completion, and the runtime verb checks membership of a datum whose completion nobody
witnessed.** They are different acts on different objects, and `68`'s boundary keying (a door at
ingest, nothing between doors) is the same structure section 4's crossings have, since an ingest is
precisely a crossing whose source term is unknown.

**And one thing `68` names as empty that I filled after reading it.** `68` section 7 reports that "the
strategy is a parameter of the correctness relation" carries **zero probe instances**, correctly notes
that `65`'s two impls are keyed by role and `66`'s strategy selects the encoding, and calls the
missing probe "the cheapest constructive item the unit's second half could build", specifying it as
"one probe where a strategy parameter selects among reduction members over one fixed (D, Q), refusing
an algorithm bound when the selected member loses the law". `p5_strategy_selects_the_member.rs` is
that probe, built after reading `68`. A `Strategy<I>` trait selects a `Reduction` over the identity it
is applied at; `Fast` takes wrapping, `Guarded` takes saturating; over the fixed signed window the
reassociating fold accepts `Fast` and refuses `Guarded` with `error[E0277]: the trait bound
Saturate<Id<RingZ, S4>>: AddAssociates is not satisfied` (`p5_neg.stderr`), while the sequential fold
accepts both, so the refusal is about the law and not about the term.

It adds one thing `68` did not ask for, and that addition is why the probe is an argument rather than
a demonstration: **the same strategy is accepted at one identity and refused at another.** `Guarded`
compiles at the unsigned window and is refused at the signed one; at the signed window `Fast` compiles
and `Guarded` does not. If the law tracked the strategy, a canon could attach it to the strategy. If
it tracked the identity, to the format. It tracks the **pair**, which is prefix 3, which is section 1's
claim standing on its own instrument. The law rows the probe implements are `p4`'s measured verdicts
rather than assumptions, and I say so in the probe's header so nobody cites the impls as facts.

## 9. Fits against the register, and what it should gain

**Kills nothing.** No live option anywhere is closed by this file. Written out in full so a
consolidator can lift them, per the register's own convention and because unit two's consolidation
lost a live option and unit one's lost another.

**Q18 (does the selected adaptation belong to identity or realisation) gains the discriminator it
currently lacks.** The entry says "whether anything downstream reads the difference" is what would
distinguish the three positions, and records that both cold derivations think type identity agrees
under all three so the residue may be naming. **It is not naming, and the discriminator is the
quantifier of a canon sentence rather than a fact about code.** Under the telescope, identity is a
prefix, every prefix induces an equality, and a sentence's truth value changes with the prefix it
quantifies over: `p1_neg_b` refuses at prefix 2 and `p1` accepts at prefix 3 with every other
coordinate held fixed. The panel has three recorded instances of a law sentence acquiring the wrong
quantifier (`56`'s coherence law caught by `61`; `57b`'s `p7` twice; `42`'s clamp-counting sentence),
which is evidence the shape rather than the authors is at fault. So the entry's resolution is not to
pick a cut but to **name every prefix that has a caller**, and the caller census is section 3.

**Q19 (are the two hierarchies the same cut) gains an answer with a mechanism.** They are the same cut
above the bottom, as `65`'s reconciliation table already maps. What separates them is that `66`'s five
levels are not five levels: level 1 is an inhabitant rather than a coarser description, and the
scheme-to-format step is **instantiation** rather than a further level, so `66`'s "format" is a point
in the product of the other levels, which is what a Rust type names and is worth naming for that
reason. Net: **three dependent axes plus two derived notions**, the completed term (a full assignment,
what a type is) and the family (a partial assignment, what a parametrised type is). And the bottom is
not a level under either count, which is `68` section 6's attack and `p3`'s shared-parameter result
arriving at the same repair from opposite ends.

**Q21 (is "number system" broad enough to include things not about magnitude) gains a second
instance.** `65` takes the broad reading, `66` carries it open, `63` takes no position. I derived the
broad reading independently from the telescope, before reading `65` section 1: prefix 1 is an ambient
domain, an ambient domain is a carrier plus an operation family, and nothing in that requires an
order. `p2`'s X3 instantiates it, with GF(2)^4 and (Z, min) as two further ambient domains over one
carrier, measured against (Z, +). That is a second instance and not a third, and one of the two is
still `65`'s.

**Q27 (is interoperation conversion, resolution, or neither) gains the shape of the answer.** There
are **three** crossings, not two, identified by the smallest telescope index at which the terms
disagree, and each preserves values or patterns totally while none preserves operations, measured
exhaustively in `p2`. The panel's vocabulary names the value-preserving property and has no word for
the operation-preserving one. The crossing with no name is the one at index 3, which changes nothing a
consumer can see at rest and changes what every operation computes.

**A new option, written out in full for the register: is the ambient domain's operation family
fixed?** Two readings. **Fixed at (+, x)**: the concept describes one arithmetic, the law layer's
entire evidence base is correctly scoped as it stands, and the tropical semiring the algorithm crates
compute in is described by the algorithm crates rather than by arvo's concept. Cost: I11's named
selling point computes in something the canon does not cover, and the two-operation law those
algorithms need (addition distributing over min) is quantified over a pair of operations from two
families, which no instrument in the panel measures. **A parameter**: one representable set carries
several ambient domains, prefix-1 equality becomes a relation someone must define, `65`'s K5 (Boolean
and GF(2) structures) and the tropical semiring become ordinary members rather than special cases, and
`p2`'s X3 becomes a crossing the concept can talk about. Cost: every law sentence in the panel's
existing evidence acquires a scope it did not previously need, and the reduction space `Reduce(D, Q)`
now varies with an axis nobody has enumerated. **What would distinguish them:** whether any consumer
needs to write a single generic algorithm over both families, or whether naming two families
separately costs nothing because no code spans them.

**A new option, written out in full: two shapes of composition, not one.** Q16's sense two (an
aggregate over numerals) contains two structurally different things. **Point compositions**: a datum
is a tuple of numerals travelling together, which is `63` section 3.6's filing of stored-pair
rationals, intervals and error-carrying pairs, and which owes its own laws at the composition layer.
**Shared-parameter aggregates**: many data plus one parameter participating in each datum, of which
arvo already ships one at the realisation layer (`Cold`'s stride, which leaves the representable set
constant) and excludes one at the identity layer (block floating point, whose block exponent makes the
element's representable set vary, 8 distinct sets over 8 exponents in `p3`). **What would distinguish
the treatment:** whether the shared parameter participates in the arithmetic. If it does not, the
aggregate is a layout fact and the element remains a format. If it does, the element has no
representable set of its own and `63`'s C2 excludes it, correctly, but the concept then has no home
for it, because "storage" is named once in passing in a file about a different topic and covers the
stride case rather than this one.

**Q1 gains nothing from me that `68` did not already give it**, and I say so rather than padding.

## 10. Candidate canon sentences

Each offered to the consolidation, not as a settlement, each tested against permanence (still true and
useful after a from-scratch rewrite in another language in another decade) and equivalence (three
independent implementations behave the same). Rungs stated honestly.

**K1, the shape.** *The numeral concept is a dependent sequence of choices, not a tuple of independent
ones: the ambient domain, then a representable set over it, then a reduction from the space its
identity derives, then an encoding of that set, then a container for that encoding. Each choice ranges
over a set the earlier choices determine, and the dependency runs one way.* Permanence: passes, no
mechanism named. Equivalence: passes, since three implementations that disagreed on the dependency
would disagree on which combinations exist. Rests on: section 2 and `p1`, plus `55b`'s concession that
the reduction space is derived and `56`'s finding that the encoding is ordered after the value set,
which are the two dependencies stated one at a time by their own authors. ONE EXPERT on the shape as a
whole.

**K2, the quantifier.** *Every canon sentence about numerals names the prefix of that sequence it
quantifies over. A property decided by the identity, a property decided by the identity together with
its selected reduction, and a property decided by the encoding are three different claims, and stating
one at the wrong depth is how a true sentence becomes false.* Permanence: passes. Equivalence: passes.
Rests on: section 1, `p1_neg_b`, `p5_neg`, and the panel's three recorded quantifier failures
(`61` on `56`; `57b`'s `p7`, twice; `42`'s clamp-counting sentence, all through `63` section 4.1).
This is the sentence I would most want attacked, because it constrains how every other sentence is
written.

**K3, derivation is completion.** *The consumer supplies a prefix and the typestate completes the
sequence; validation checks that the completion satisfies what the prefix demanded; erasure discards
the completion's evidence and keeps its consequences. A completion may branch, and the branches are
what op's plural names.* Permanence: passes. Equivalence: passes. Rests on: section 3, `p1`'s
projections and size assertions, and `68`'s split of validate into two verbs, which this sentence must
be read with rather than against.

**K4, crossings.** *Two numerals are crossed at the smallest index where they differ, and a crossing
declares what it preserves. Preserving values is a statement about carriers and is decided by the
representable sets. Preserving operations is a statement about induced algebras and is decided by the
law inventories. A crossing may have the first and lack the second, and that crossing is the one that
looks free.* Permanence: passes. Equivalence: passes. Rests on: section 4 and `p2`, exhaustive at one
model width. ONE EXPERT.

**K5, closure bounds the adaptation layer's subject.** *An operation closed on the representable set
needs no adaptation and inherits its ambient laws unconditionally. The adaptation apparatus, and every
law derived through it, is about the operations that can leave the set.* Permanence: passes.
Equivalence: passes. Rests on: `p4`'s closure measurement and the derivation in its header. Small, and
it is what makes the tropical additive operation free.

**K6, the two law families have two consumer classes.** *The adaptation laws are what order transport
consumes, and the algorithms whose additive operation is the order are their consumer. Coherence is
what a reduction that may be split or reassociated consumes. The two are independent, all four
combinations occur, and a numeral serves one class, the other, both, or neither.* Permanence: passes.
Equivalence: passes; the classification is measured. Rests on: `63` section 3.4 for the families,
section 5 and `p4` for the consumer mapping and the monotonicity biconditional, `35` through the
register for the independent algorithm-side measurement. The families are the panel's; the consumer
mapping is ONE EXPERT and mine.

**Deliberately not offered as sentences:** any name for the crossing at index 3, because coining one
badly is worse than carrying the gap; any statement about which prefix should be called "the number
system", because that is a naming call for op and section 3 says what it depends on; any magnitude,
because nothing here is a bench and nothing is priced; and any admission or exclusion of block
floating point, because `p3` establishes where the current sentence files it and not what should
happen next.

## 11. What I could not settle

**Whether the ambient operation family is fixed.** Section 7 poses it and I decline to answer it. It
interacts with `65`'s K5, with the whole law layer's scope, and with I11's named consumers, and I do
not think one file's evidence should decide it.

**A name for the value-preserving, operation-destroying crossing.** I looked for prior art and the
honest answer is that the mathematical vocabulary calls it a bijection of underlying sets, which is
accurate and useless as a canon word because it says what the thing lacks. I could not find one and I
am not confident a good one exists; someone with a stronger feel for the project's naming register
should try.

**Whether the telescope's dependency should be visible to a consumer.** `p1` and `p5` enforce it with
associated types, which works and is one spelling among many, and per the probe discipline the
spelling is scaffolding rather than a proposal. Whether a consumer should ever see the chain, or only
its completed term, is a surface question adjacent to Q2 and Q9 that I did not attack.

**Whether the 1360 collision in `p4` means anything.** Two independently written scripts reproduce it
across four cells whose unweighted failure counts differ. I did not explain it.

**Transfer past the model width.** Everything exhaustive here is at 4 bits, and `68` section 2.4 has
now established inside this panel that the ceiling is forced by the toolchain rather than chosen, and
that uniformity of construction does not by itself carry the transfer. My results inherit that
proviso in full.

**Nothing here settles anything.** The mode is explore. Sections 1, 5, 7 and 9 are what I would most
want the unit's second half to attack, and section 8's first item is a correction `66` should make in
one word.
