# 70. Levels are clients, not facts: five choices, four effects, and no intrinsic count

**Author lens:** Lattner. Multi-level compiler infrastructure, progressive lowering, what a level of
abstraction is for and who gets to demand one.
**Position:** fifth file of the number-systems unit, opening its second half after the checkpoint at
`69`. This is an argue-and-converge dispatch, not a cold derivation: I read the panel before writing,
so every agreement below is a read plus my own derivation, never an independent instance, and I say
which is which.
**Probes:** `70_probes/`, three instruments, committed at `5d2c86ca` with transcripts before this
file was written. Pinned `nightly-2026-05-28` (`p1_small_compile.txt` records the hash), zero feature
gates, `#![no_std]`, no `dyn`, no `TypeId`. The pre-commit autofix ran rustfmt on the two Rust
sources after their transcripts were captured; both landed sources were re-compiled from `git show
HEAD:` and exit 0 on all three invocations, per the claim discipline's verify-after-autofix rule.

**The assigned question.** How many levels does the number-system concept have, and where do they
cut? `65` says three. `66` says five. `68` says both undercount the bottom. `67` says the components
form a dependent sequence, which is a claim about shape that may or may not be compatible with any
of the cuts. `OPTIONS.md` Q19 proposes a discriminator: apply each cut's own change-test to the
other's levels and see which collapse.

**The answer in three sentences, before the argument.** The counts disagree because they count
different things, and both countables are real: the concept has **five choices in one dependency
order** (that number could still move, and section 7 says where), and it has **effect classes whose
count is relative to the observables a client brings**, four under this file's observables. Neither
partition is a refinement of the other, and I can show that mechanically: two choices with
identical observable effects sit at **non-adjacent** positions in the dependency order. So "how many
levels" has no canon answer, because a level is a client-relative notion; what the canon owns is the
dependency order, the distinction between the two instruments, and the criterion by which a cut
earns a name.

## 0. Gates, and two reports

**Canon gate: passes, situation two.** `mock/canon/` does not exist, `mock/crates/` is empty by the
declared mutation order, and this panel is writing the first canon. Nothing binds but op's intents;
I re-read `INTENTS.md` in full before writing. Nothing below settles anything.

**Test gate: no suite exists.** The substitute is the probe discipline, applied to my own three
instruments: each carries a stated hypothesis, a committed output or transcript, and one carries a
prediction its own output refuted, kept in the output rather than smoothed (section 2). I did not
re-run `65`'s, `66`'s or `67`'s probes; `68` re-ran the first two thoroughly and I rely on that
report, and every number I take from `67`'s instruments is taken from its file text and marked so.

**Report one, per the standing instruction, unsoftened.** Checkpoint `69:114-115` states "The
generated instructions are fixed at their source and the dead lint scopes are gone." **That is an
overclaim, verified today at the source.** `.claude/CLAUDE.md:137-140` still prohibits, in the
present tense, imports between crates that do not exist ("Do not put arithmetic fielded structs in
`arvo-bits`", "Do not import `UFixed` / `IFixed` directly in `arvo-graph`"). `.claude/rules/
cookbook.md:127-128` still routes readers to `arvo-graph::topo_sort` and `arvo-spectral::Laplacian`,
`.claude/rules/cargo.md:42-45` still tables the dead L2/L3 crates as the layer structure, and
`.claude/rules/implementation.md:89` still describes them as current. `68` section 0 reported part
of this residue; the checkpoint recorded it as fixed while most of it stands. Three members of this
unit have now been caught, or nearly caught, reasoning from these files (`66:60-68` built a
cross-check on them; the checkpoint itself introduced a provenance error while correcting one). The
cost is recurring and the fix is one regeneration.

**Report two, a dependency both in flight.** Two of my conclusions touch op's open questions and I
carry both branches rather than assuming an answer. Where section 4 excludes a runtime-owned
identity component, the exclusion rests on `68` section 5's residue clauses, which are argued from
the long-standing constraints whose provenance is op's open Q-B; if those constraints are not his
intent, the exclusion boundary is unratified and only the descriptive axis survives. And where
section 4 uses compile-time validation instruments, the point is stated under both readings of
op's "validate" (Q-A): it holds for the compile-time verb directly and for the runtime ingest door
by the same arity argument.

## 1. The frame I bring, and whose claim it supports

The question "how many levels does this concept have" is the question my own field spent two
decades getting wrong, and the lesson it paid for is short: **a fixed level count baked into the
infrastructure is a bet against the clients you have not met, and it loses.** The systems that
survived did not answer "how many levels"; they answered "by what mechanism does a level come to
exist", and let the count float. A level earns existence by having a client: a transformation, a
query, a contract that is best expressed at it and cannot be expressed as well above or below it.

That is `67`'s caller census (its section 3: "prefixes with no consumer need no word") arrived at
from a different tradition. I read `67` before writing, so this is a read-supported derivation and
not an independent instance; I state it because the two traditions reach the criterion from
unrelated failure histories, which is worth something even when it is not a rung. What I add to it
is the half `67` did not develop: the census tells you which cuts get **names**, but it does not
tell you what kind of thing a cut **is**, and this unit's three files disagree about exactly that.
`65` cuts by what a change does. `66` and `67` cut by what a choice depends on. Section 2 shows,
mechanically, that these are two different instruments producing two different partitions of the
same five components, and that neither can be recovered from the other.

## 2. Q19's discriminator, run mechanically

`OPTIONS.md` Q19 proposes applying each cut's change-test to the other's levels. `65` is the only
file that states a change-test explicitly (`65:80-86`): container moved with no named value changed
is a format change; the configuration-to-value map moved, named values changed, operations' meaning
unchanged is a representation change; what operations mean changed is a system change.

`70_probes/p3_component_effect_table.py` applies it, exhaustively at the 4-bit model width. One
baseline chain (D = (Z, +), Q = [0, 15], rho = wrap, E = binary, C = 8-bit container), five
variants, each moving exactly one component, four observables measured on each: V, the set of
denoted values; M, the configuration-to-value map on shared configurations; O, induced operation
results on shared value pairs; L, container layout. The committed output (`p3_output.txt`) carries
the counts. Five findings, in increasing order of consequence.

**First, a prediction of mine was refuted, and the refutation sharpens the point.** I predicted a
Q-widening changes M. It does not: the shared-configuration map is untouched (0 of 16 configs
differ), because widening **extends** the map's domain rather than remapping it, while operation
results still move on 120 of 256 shared pairs (wrap mod 32 against wrap mod 16). The measured
signature is (V=1, M=0, O=1, L=0). So a Q-move is an embedding at the map level and a divergence at
the operation level simultaneously, which is `67`'s X2 finding (its section 4, values preserved,
operations not) measured by a different instrument, agreeing in sign with different numbers because
the setups differ, and it makes `65`'s "representation change" clause doubly unsatisfiable for Q:
the clause requires named values to change while operations do not, and a Q-move does the opposite
on the second conjunct.

**Second, single-component moves produce effect sets, not single effects, because the dependency
forces re-instantiation.** The script cannot move Q while "holding E and rho fixed"; it can only
hold their **families** fixed and re-derive the members, and it prints where that happens. This is
`67`'s telescope observed from the instrument side: the change-test's implicit "all else equal"
clause is unsatisfiable for every component except the last, because later components range over
sets the earlier ones determine. A change-test is well-defined on C and progressively less
well-defined the earlier the component it touches.

**Third, `65`'s test, applied literally, misfiles or fails to file most of the chain.** The
committed application: dD, dQ and drho all classify as "system" (O moved); dC classifies as
"format"; and dE is **unclassified**, because `65`'s representation clause demands that named
values change and an encoding swap changes none (binary to bit-reversed nibble: 12 of 16
configurations remap, 0 of 256 operation pairs differ, value set identical). The charitable repair,
dropping the values-changed conjunct, files dE correctly and still misfiles dQ, which `65`'s own
section 3 wants under representation (coverage is listed as a representation property at
`65:169-172`) and which the test mechanically assigns to system. So `65` carries an internal
tension between its section 1 test and its section 3 property list, and the tension is not a
drafting slip: it is forced by the dependency, because coverage cannot move without the induced
operations moving.

**Fourth, and this is the finding the unit's question turns on: the effect partition does not
respect the dependency order.** The four distinct signatures are {dD, drho} at (0,0,1,0), {dQ} at
(1,0,1,0), {dE} at (0,1,0,0), {dC} at (0,0,0,1). D and rho, positions one and three of the chain,
are **observationally identical** under these observables, and Q sits between them with a different
signature. A partition with a non-contiguous class cannot be a coarsening of the order, and no
refinement of it can be either. So Q19's own framing ("if they collapse cleanly, one cut is a
refinement of the other and the question is granularity") has a determinate answer: **they do not
collapse, and the question is not granularity.** The two hierarchies are not the same cut at
different zoom. They are two different instruments: `66` and `67` classify **choices by what they
depend on**; `65` classifies **changes by what they preserve**. Both partitions are real, and
neither recovers the other: the dependency order cannot predict that D and rho are effect-twins,
and the effect classes cannot reconstruct that Q sits between them.

**Fifth, the effect count is observable-relative, and that is arithmetic on the committed table,
not a new claim.** Project the table onto O alone and the partition is {D, Q, rho} against
{E, C}: two classes. Add M and L and it is four. Add the raw-pattern observables `56` established
(raw order, the raw adder, per `63`'s account) and encodings split further, since those are
properties E-moves change and value-level observables cannot see. So "how many effect classes" has
no absolute answer; it has an answer per observable set, and an observable set is what a client
brings. The boolean signatures also deserve one honesty note: dD and drho are twins as booleans
while their magnitudes differ wildly in this model (255 of 256 pairs against 120 of 256), and
magnitudes are not a classifier, because they vary with the particular change chosen rather than
with the component.

## 3. What this gives the assigned question

Assemble the pieces and the question dissolves into three questions with three different answers.

**How many choices does the concept make?** Five, as currently established: D, Q, rho, E, C, in
one dependency order. This is `67`'s K1 and I carry it as shape rather than re-litigate it; my p3
is a further instrument on its load-bearing property (the dependency forcing re-instantiation), so
the sequence claim now has the telescope probe from the inside (`67`'s p1, enforcement by E0271)
and the effect table from the outside (p3, unsatisfiability of "all else equal"). Two instruments,
one author each, both committed. The count itself is open at both ends: `67` section 7's D-family
question could split D, and section 4 below re-keys C without removing it.

**How many levels?** Not a fact about the concept. A level is a partition class relative to an
instrument, and the two instruments in play produce incompatible partitions by measurement. `65`'s
three words, `66`'s five, and `67`'s "three dependent axes plus two derived notions" (`67:552-553`)
are not three counts of one thing that someone got wrong; they are counts of different things. On
which point I have one attack on `67`: its Q19 answer says "three dependent axes" while its own K1
names five components, with no statement of what "axis" counts that "component" does not. Under
this file's reading the tension resolves cleanly, three is a segmentation of five for one purpose,
but `67` does not say so, and a consolidator lifting both sentences verbatim would ship a count
contradiction into the canon candidate. The repair is one sentence naming what each number counts.

**Where do they cut?** Two places, one per instrument, and the canon needs both because its
sentence types split the same way. Sentences about **identity and quantification** cut on the
dependency order: `67`'s K2, a claim's truth turns on the prefix it quantifies over, with three
recorded failures and two compiled refusals behind it. Sentences about **compatibility and
crossings** cut on preserved observables: `67`'s K4, a crossing declares what it preserves, and
`65`'s change-test is, mechanically, a crossing classifier that was drafted as a level definition.
That last clause is my proposed disposition for `65`'s test, and it is a keep, not a kill: the test
is a good instrument pointed at the wrong question, and restated as "which observables does this
change preserve" it becomes the compatibility half of the canon with its section 1 intent intact.

This also gives Q18 a sharper diagnosis than the register carries. `65` folded the adaptation into
system identity; `63`'s C2 put it in realisation; the register asks whether anything downstream
reads the difference. The effect table says why the fold happened at all: **D and rho are
indistinguishable to the effect instrument.** From where `65` stood, classifying by what changes
propagate, ambient-domain moves and adaptation moves are the same event, so folding them into one
level was the instrument speaking, not an error of judgement. The dependency instrument is the one
that separates them (`67`'s p1_neg_b refuses at the identity with only the adaptation moved), which
is more evidence for `67`'s Q18 position that the discriminator is the quantifier rather than
anything observable in code, and it explains the dispute's persistence: each side's instrument
genuinely cannot see what the other side's cut is about.

## 4. The bottom of the chain, and the key that dissolves it: ownership

`68` section 6 attacks both hierarchies at the bottom: under Cold packing the container answer is a
per-value and per-aggregate pair, so C is not a level. `67` section 6 reaches the same repair from
the identity end: a shared parameter can sit at the realisation layer (stride) or the identity
layer (a block exponent), and `63`'s C2 sentence distinguishes the layers, not the sharing. I
support both, having read both, and I propose the frame that makes them one statement with a third
case neither file covers:

**Every component of the chain has an owner, and the ownership is a key on the component, not a
new level.** The chain says *what* is chosen; the owner says *for whom* the choice holds and *when*
it resolves. Four owners are now attested in this panel's material: the **type** (the ordinary
case; every component of a `UFixed`-shaped declaration), an **aggregate** (Cold's stride owning
part of C for a whole run, `68` section 6; a block exponent owning part of Q for a whole block,
`67`'s p3), the **compilation target** (the case below), and a **runtime datum** (the block
exponent in its native form). Under this key, `68`'s "pair of questions" at the bottom is the
ownership key made visible on C, and `67`'s "two layers" finding is the same key on Q against the
same key on realisation components. Nothing about the five components moves; what the hierarchies
undercounted was not a sixth level but a second axis.

The third case is Q26, and it is where the key earns its keep, because `63`'s C2 sentence
conflates the key's two halves. The sentence (opened at the source, `63:141-147`): "**Q is a
constant of the type**: a value set depending on other data has no Q and is not a format but
storage." Two different tests are fused in "depending on other data": **on whom** Q depends, and
**when** the dependency resolves. `70_probes/p1_target_owned_q.rs` separates them. A window whose
Q is selected by cfg, the same machinery `cfg(target_pointer_width)` uses, with a custom flag so
the probe needs no second installed target: compiled twice (`p1_small_compile.txt`,
`p1_wide_compile.txt`, both exit 0 on the pin), the two compilations carry two different constant
Qs (asserted, 16 against 64), and **within each compilation every compile-time instrument this
unit has built applies unchanged**: membership is a const fn of (type, bits) with no extra
argument, the induced wrap law is checked exhaustively over the compilation's own Q in const
context, and the `repr(transparent)` carrier has the container's size. Target-owned Q is
monomorphisation-constant Q.

`70_probes/p2_ownership_moves.rs` is the same separation run on `67`'s block float. Exponent owned
by the type as a const parameter: eight instantiations, eight distinct constant Qs (sampled
assertions), membership and denotation const fns of (type, bits), exit 0. Exponent owned by
runtime data: the denotation question's honest signature **carries the exponent as a runtime
argument**, and the (type, bits)-only spelling has no writable body because the information is not
in scope. The finding is the arity, visible in the two signatures, not a refusal transcript, and I
say so. Selecting among the eight monomorphs by a runtime exponent is a runtime match over types,
which is precisely the dispatch-residue shape `68` section 5 excludes; the probe demonstrates the
arity fact only and leaves the exclusion to the clauses that own it.

So the candidate answer to Q26, offered to the register beside its existing three: **the
discriminator is resolution time, not dependence.** A platform-width numeral is a target-indexed
family of formats. Its Q depends on something outside the type's own parameters, and that
something is resolved at monomorphisation, so within any compilation Q is exactly as constant as
`UFixed`'s, and C2's own reasons for demanding constancy (a membership predicate of (type, bits),
compile-time law checks, erasure with nothing environmental to consult) are all satisfied. Block
float in its native form fails the same tests not because its Q depends on other data but because
the dependency resolves **per datum at runtime**, which is where `68`'s three residue clauses
bite. "Storage", named once in passing in `63`, does not need to be a third kind of thing to
cover `USize`; the resolution-time reading files it as a format family and reserves the exclusion
for the runtime-owned case.

Both in-flight branches, carried as promised. On Q-A: under the compile-time reading of
"validate", p1 is the demonstration directly; under the runtime reading, the ingest door for a
target-owned Q is still a pure function of (type parameters, bits) per compilation, so the
boundary between the two Q26 cases survives either verb, and only the runtime-owned case ever
needs the wider arity. On Q-B: the **axis** (owner and resolution time) is descriptive and stands
on the probes regardless of provenance; the **exclusion** of runtime-owned identity components
rests on the erase clauses, which rest on the long-standing constraints, which are not in
`INTENTS.md`. If op does not ratify them, the axis remains and the boundary needs a new warrant.

One honest limit on the ownership frame, so it is not over-read: the transfer proviso does not
disappear under it. A target-owned Q at real pointer widths cannot be exhaustively checked (the
const-eval ceiling `68` re-established at nine bits), so per-target validation at model widths
inherits the same width-transfer proviso as everything else, per target. The frame relocates the
obligation; it does not discharge it.

## 5. Attacks and supports, itemised, rungs kept honest

**Supported with a new instrument: `67`'s K1 sequence claim.** p3's forced re-instantiation is the
dependency observed from the change side, independent in mechanism from `67`'s p1 (which enforces
it with associated types) though not independent in authorship lineage, since I read `67` first.
Two committed instruments, two mechanisms, one claim.

**Supported and completed: `68` section 6 and `67` section 6.** The per-value/per-aggregate pair
and the two-layer shared parameter are one fact under the ownership key, and the key adds the
target-owned case with a probe (section 4). This is the composition both files asked for rather
than a rival.

**Attacked, small but real: `67`'s Q19 answer carries a count ambiguity.** "Three dependent axes"
(`67:552-553`) against its own five-component K1 (`67:608-617`), with no statement of what the two
numbers count. Resolved by the two-instrument reading, but the resolution must be written or the
consolidation inherits a contradiction between two sentences it would otherwise lift verbatim.

**Attacked and repaired rather than killed: `65`'s change-test.** Mechanically, its three clauses
classify only C cleanly, leave E unclassified under the literal text, and misfile Q against `65`'s
own section 3 (p3, committed output). The repair keeps the instrument and renames its object: it
is a crossing classifier, the compatibility half of the canon, and as a level definition it is
answering a question the dependency owns. `65`'s candidate 1 ("the change-test of section 1 as the
boundary" between the three concepts) should not enter a consolidation in that form.

**Carried without adjudication: `66`'s five levels.** `67` section 9's disposition (level one is
an inhabitant, scheme-to-format is instantiation) survives my derivation and gains nothing new
from me; keeping it is the result. The container level's definition at `66:55-58` ("a format's
logical numeral meaning does not change if the same bits are housed in a wider container") is
p3's dC row measured: (0,0,0,1), the one component whose change-test is fully well-defined.

**A correction to my own prediction, on the record.** p3's header predicted dQ at (1,1,1,0);
measurement returned (1,0,1,0); the output keeps the refutation and section 2 states the
mechanism. A widening embeds at the map and diverges at the operations, which is a cleaner fact
than the one I predicted.

## 6. Fits against the register

**Kills nothing.** Written out so a consolidator can lift them, per the register's convention.

**Q19 gains its answer with a mechanism.** The two hierarchies are not the same cut and not
different granularities of one cut: the effect partition has a non-contiguous class ({D, rho}
with Q between them, `p3_output.txt`), so neither partition refines the other, and the
discriminator the entry proposed returns "they disagree about what a level is". The candidate
resolution: the canon carries the dependency order for identity sentences and the
preserved-observables classification for compatibility sentences, names cuts by the caller
criterion, and commits to no level count. Effect-class counts are observable-relative
(two under O alone, four under this file's set, more under `56`'s raw-pattern observables).

**Q18 gains the instrument diagnosis.** D and rho are effect-twins, so the fold `65` made is what
the effect instrument reports, and the separation `63` made is what the dependency instrument
reports. The positions are artifacts of instruments, which is why the dispute felt nominal to
code (both instruments agree on completed terms) and is not nominal for the canon (`67`'s
quantifier argument, which this supports from a second direction).

**Q26 gains a fourth option, written in full.** Existing three: target-resolved instantiation of
the shape family (`66`), a genuinely separate axis (`66`), a third kind called "storage" (`63`'s
phrase, weighted by the register as one passing mention). **New: the resolution-time reading.**
`USize` and `Cap` are target-indexed families of formats; Q owned by the target is constant at
monomorphisation and satisfies every obligation C2 wants constancy for (p1, two compilations, two
constant Qs, all instruments unchanged); the exclusion C2 reaches for applies to runtime-resolved
Q only (p2, the arity fact). **Cost:** C2's sentence needs one clause split, "depending on other
data" into on-whom and resolved-when, and the erase-clause warrant for the runtime half is
Q-B-dependent. **What would distinguish it from the third-kind reading:** whether any obligation a
format owes is unsatisfiable by a target-owned Q within one compilation; p1 found none among this
unit's instruments, and one found later would move me to the third-kind reading.

**A new option for the concept's structure, written in full: the ownership key.** Every chain
component carries (owner, resolution time): type, aggregate, target, or runtime datum;
monomorphisation or runtime. Subsumes `68`'s bottom-tier pair (C owned by the aggregate under
Cold), `67`'s two-layer shared parameters (the same key on Q against realisation), and Q26 (Q
owned by the target). **Cost:** every canon sentence quantified "per value" acquires an implicit
"whose components the value owns" rider, and the aggregate-owned cases need their own quantifier
(per-aggregate sentences), which is new surface. **Alternative it competes with:** treating
aggregate cases as compositions under Q16 sense two with no key on the base concept, which keeps
the base smaller and forces every shared-parameter case into the composition layer, including
`USize`, where that filing has no obvious content. **What would distinguish them:** whether any
consumer needs to write one contract generic over ownership, or whether naming the cases
separately costs nothing because no code spans them, the same test `67` posed for the D-family
question.

## 7. Candidate canon sentences

Offered to the consolidation, not as settlements, each against the permanence and equivalence
tests. Rungs stated.

**L1, the two instruments.** *Identity and quantification cut the concept along its dependency
order; compatibility and crossings cut it by preserved observables. The two partitions are both
real and neither refines the other: choices with identical observable effects sit at non-adjacent
positions in the order. A sentence about which thing something is names a prefix; a sentence about
what a change or crossing is allowed to do names what it preserves.* Permanence: passes, no
mechanism named. Equivalence: passes; implementations disagreeing on either partition would
disagree on which crossings are lawful or which contracts are decided. Rests on p3 plus `67`'s p1
and p2. ONE EXPERT on the joint statement; the halves are `67`'s K2 and K4, each already
instrumented.

**L2, no intrinsic level count.** *The concept commits to its choices and their dependency order,
and to no count of levels. A level is a partition class relative to the observables a client
brings; different clients see different partitions; a cut earns a name by having a caller.
Effect-class counts are stated with their observable set or not at all.* Permanence: passes.
Equivalence: passes in the same sense as `67`'s K2, as a constraint on how canon sentences are
written. Rests on p3's projection arithmetic and the caller census. ONE EXPERT.

**L3, the ownership key.** *Each choice in the sequence has an owner and a resolution time: a
type, an aggregate, or the compilation, resolving at monomorphisation; or a runtime datum. The
concept's per-value obligations are owed by choices resolved at monomorphisation, whoever owns
them; a choice owned by an aggregate makes its obligations per-aggregate; and the membership
question's arity states which regime a design is in.* Permanence: passes. Equivalence: passes;
three implementations disagreeing on an owner would disagree on signatures, which p2 shows are
observable. Rests on p1, p2, `68` section 6 (TWO EXPERTS on the pair, per the register), `67`
section 6. The key as a unification is ONE EXPERT and mine.

**L4, platform width.** *A platform-width numeral is a target-indexed family of formats: its
representable set is owned by the compilation and is a constant of every monomorphisation, so
within one compilation it owes and satisfies everything a format owes. Exclusion on grounds of
dependence applies to dependence that survives to runtime, not to dependence resolved by the
build.* Permanence: passes. Equivalence: passes. Rests on p1 and `63:141-147` read against it.
ONE EXPERT, and its second half shares Q-B's provenance condition, stated in section 4.

**Deliberately not offered:** any level count (the point of L2); any name for the ownership key's
cases beyond the plain words used here, since naming is op's; and any admission ruling on block
float, which p2 locates (the runtime-owned side of the key) and does not decide.

## 8. What I could not settle, and coverage

**The observable set.** p3's four observables are a choice, and section 2's fifth finding says the
partition moves with the choice. Whether the canon should bless a canonical observable set (V, M,
O, L plus `56`'s raw-pattern pair is the obvious candidate) or leave the set open per client is
the same closed-or-open question the role set (Q23) already carries, and I did not resolve it
there either.

**Whether a runtime-owned realisation component exists.** The key's fourth owner is attested only
on Q (the block exponent). A runtime-chosen stride would be the realisation-side case; whether
anything wants it, and whether it falls to the dispatch-residue clause the way runtime-owned Q
does, is unexplored.

**The D-family question.** `67` section 7 poses it; the ownership key neither helps nor hinders
it; untouched here.

**Whether "level" should appear in the canon's vocabulary at all.** L2 uses the word to deny it
canon status; a consolidation might do better dropping it entirely and speaking only of prefixes,
segments and classes. I could not convince myself either way and both spellings of L2 are
serviceable.

**Coverage, bounded.** Read end to end: `INTENTS.md`, `RULES.md`, `69`, `67`, `68`, `65` (both
phases), `66` (both phases), `OPTIONS.md`'s unit-three section (Q18 through Q28) with the file's
headings and standing section, `DROPLIST.md`'s panel-closed section and section 7 with headings.
Opened at the source for citation: `63:130-174` (C2 and its rung), the generated-instruction files
named in section 0 (grepped today), `65:80-86` and `65:169-172`, `66:38-68`, `67:546-554` and
`67:608-617`, `69:114-115`. **Not read:** `63` end to end; files `01` through `62`; `seed/`;
`archive/`. Every statement here about `55`, `56`, `60`, `35` or the container-derivation unit is
sourced to `63`, `67`, `68` or the register and inherits their errors if any. Built no bench;
nothing here is priced, and the compile timings in the p1 transcripts are timestamps, not
measurements.

**Nothing here settles anything.** The mode is explore. Sections 3 and 4 are what the unit's
remaining files should attack: L1's joint statement wants a second derivation that did not read
this file first, the Q26 resolution-time reading wants someone to hunt for an obligation a
target-owned Q cannot satisfy, and the ownership key wants either a consumer that spans owners or
the finding that none exists.
