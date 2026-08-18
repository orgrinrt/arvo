# 161. The canon candidate for the primitive

**Topic five's canon candidate**, covering both sittings on the question: the first (`109` through
`114`) and the ninth unit that reopened it (`154` through `160`). Topic five is the only topic in
this panel with no ledger; `AGREEMENTS.md` has a section for every other topic, and the panel's
central noun has been reachable only by someone who already knew which member files to open. This
file is that ledger and the candidate built on it.

**What this is and is not.** It is input to the canon, not canon in miniature. Nothing moves to
`mock/canon/`, which does not exist. Op ratifies, at the end, once; per I12 an opinion given before
the experts converge is an ack, and this file offers rather than settles. It is a full standalone
writeup, never a delta: where a clause is carried unchanged its content is written out anyway.

**Order.** The agreement ledger comes first and in full, before the statement, because a compression
that leads with its conclusion has already decided what to drop. Then what is contested, then what
is closed or retired including what must no longer be cited, then the statement, then what the topic
did not settle, then what only op decides, then the live options in their own pass, then the anchor
accounting, generated rather than written.

---

## 0. The two gates

### 0.1 Canon gate: passed

Checked against `INTENTS.md` read in full including its normative "How to read an entry" section,
and against `RULES.md` read in full. The assignment is licensed: I14 (IN FORCE, `INTENTS.md:268-297`)
presupposes a determinate account of what a primitive is, I11 (`INTENTS.md:190-197`) makes the
contracts above the base the library's purpose, and op's `87` establishes that the canon is written
at the end from consolidations read alongside their members, which is what makes a topic with no
ledger a gap rather than a style choice. Nothing below touches the RATIFIED rung; I13
(`INTENTS.md:214`) is the working method throughout and argued with nowhere. The three questions
reserved for op (section 6) are carried open, conditionally, clause by clause.

### 0.2 Test gate: passed, at 123 across 13, and it is the eleventh count

Thirteen `-shared` crates, run crate by crate at `--release`, `bitpack-write-contend-shared`
serialised and otherwise untouched per the standing instruction:

```
12 crates: 9+12+6+5+3+6+1+3+11+7+15+30 = 108
bitpack-write-contend-shared (serial)  =  15
total                                  = 123, all passing
```

`161_probes/run_test_gate.sh`, output `161_probes/gate_release.out`, with the script's own negative
control (a crate whose invocation produces no parseable pass count prints `MISSING OR ZERO`) firing
on the nonexistent-crate line. Bodies were not re-read this dispatch; the surface here is the
panel's own files, and the three mechanical body scans this ledger leans on are `154`'s (all 123),
`155`'s (`warm-container-shared`'s fifteen in full) and `157`'s (`bitpack-write-contend-shared`'s
fifteen in full), named so the reliance is countable.

---

## 1. The agreement ledger

Every claim carries who established it, in which sitting, at which rung, and the **form** of each
instance's independence, because the rung is carried by its instruments. Two forms count:
**derived-before-reading** (blind phase one, or a probe committed before the source was opened) and
**separately-built-instrument** (a different construction reaching the same answer). Everything else
is inheritance, and inheritance is stated as such. Per `159` F159-1, a count of agreeing bench
crates is **not** a count of instances in this corpus: three of the four packed-end crates share one
dependency family and five of six `SplitMix64` declaration bodies hash identically with whitespace
stripped, so vocabulary counts over the bench tree are one instance until the dependency graph has
been checked.

Rungs: **RATIFIED** (nothing here), **TWO+ INSTANCES** (independent derivations or separately built
instruments, form stated), **CONVERGED** (attacked, replied, author agreed; the reply round's
characteristic rung), **ONE EXPERT** (asserted once, possibly verified but not independently
derived). Predicates are carried at the establishing file and not restated in full here; the ledger
names the anchor that holds each predicate, per the never-widen-in-place rule.

### 1.1 The denotation

**L1. A primitive is a value set together with one realisation map taking an exact result back into
it, over a declared operation set; identity is that structure up to denotation-preserving
isomorphism; a law is read off it and never declared.** The composed clause is `112:904-906`, one
expert's wording, and it survived the ninth unit's attack at both ends (`157` section 2.1: the
packed end does not touch it, because it is a statement about denotation and F6 is a statement
about representability). `155` phase two defers to it; `160` clause 1 carries it unchanged. The
parts have their own, stronger, provenance below. **Rung: ONE EXPERT wording over multi-instance
parts, attacked once and standing.**

**L2. The realisation map is one map with two regions: rounding between grid points, completion
outside the range.** Third instance at `110` section 2, blind, after `63` C1 (itself from two cold
arrivals, `55` and `60`); and a fourth, separately built instrument on the licence side at `112`
F112-4: a magnitude bound switches off the completion and not the rounding, a grid bound the
reverse, which is what two regions of one map predict and two independent axes do not. **Rung:
TWO+ INSTANCES (three derivations, one cross-side instrument).**

**L3. The value set and the realisation vary independently, in both directions.** `109` P1, blind:
same value set realised twice (per-element `u16` against a dense 13-bit stream, identical answers,
different footprints), and same bits read twice (`I=8,F=0` against `I=4,F=4`, 64714 of 65536
products differ). Direction A has a second blind instance: `154` F5 reached the same separation
from the shipped `bitpack-footprint-shared` buffer, different instrument, derived before reading
`109`, and claimed the TWO EXPERTS rung for exactly this claim (`154` P2.2). **Rung: TWO+
INSTANCES on direction A; ONE EXPERT on direction B.**

**L4. The realisation is not identity-bearing; every pure code assignment is presentation.** `110`
P1 quotients four encodings including a structureless bijection, blind; `63` C2 states it earlier
from a different witness pair; `109` conceded its own contrary reading in phase two, on `110`'s
reason (admitting the encoding into identity makes every representational choice semantic and
forbids rewriting). Bounded by `109` P8's discriminator, tested rather than argued: an encoding is
observable through an operation exactly when that operation is defined on the representation rather
than the denotation, so `63`'s observability and `110`'s quotient are both right at their own
levels. **Rung: CONVERGED, with the reconciling instrument at `109` section 16.4.**

**L5. Identity is decided by denotation-preserving isomorphism relative to a declared signature,
and that relation, alone among the three sameness relations, is a congruence under composition.**
`110` F1 (the criterion reproduces the axis classification without being given it) and F13 (0
congruence failures over 71 merged pairs, against 131 and 17 for the two weaker relations, with the
encoding control showing it is not surviving by strictness), blind. `109` section 10's
three-relation lattice (nominal, representational, denotational, each licensing a different
operation: assignment, memory reinterpretation, rewriting) composes with it rather than competing;
`110` conceded the lattice framing and kept the congruence result, and `111` 8.3 adds the fourth
property (the lattice is one decidable relation at the top, one true relation at the bottom, and an
adequacy obligation between them). **Rung: CONVERGED; the congruence measurement is ONE EXPERT with
its controls.**

**L6. The signature is part of the definition: how many primitives there are is not well posed
until the operation set is fixed.** `110` F4 (84 primitives under `{add}`, 186 under `{add,mul}`,
same 288 configurations) and `109` section 8 (a primitive is not definable one at a time; the unit
is a family closed under the operations), **both derived blind in parallel, different routes,
neither having read the other** (`110` 16.3, `109` 16.3). `111` 5.1 adds that the direction is by
construction (a term that separated still separates) and only the magnitude is measured. **Rung:
TWO+ INSTANCES, derived-before-reading.**

**L7. The identity a signature induces is determined by the reach of its terms into the realisation
map's domain; it is monotone, saturates when the reach is the whole domain, and a full literal
reaches saturation at depth one. The theorem holds over signatures whose members are functions of
the value set and the realisation map, and a container observation is outside that class and splits
every class it touches.** The saturation measurement is `111` 5.2 (165 classes under `{literal}`,
identical partition with everything added, 148 for the richest operation-only signature); the reach
restatement is `157` S-10; the premise is `157` F157-4 (32 to 64 classes under a container
observation, every class split); the design consequence (a design that can write a literal needs no
closed operation set for stable identity, and arvo cannot avoid a constructor per I3) is `111` 5.3.
**Rung: the bound is an argument (proof), the magnitudes ONE EXPERT each; the premise's measurement
ONE EXPERT; the whole is conditional on `156` item 1 and is carried conditionally.**

**L8. The law set is not a component of a primitive. It is a lossy projection of the algebra, a
false declaration compiles and is load-bearing, and the exclusion is analytic besides.** The
best-supported claim in the topic, on three genuinely different instruments plus an argument:
`110` TEST 2 (40 algebras collapse to 7 law sets, one law set shared by 8 algebras: the projection
cannot reconstruct the primitive), blind; `109` P2 (a false `ADD_ASSOCIATIVE = true` compiles, and
a rewrite gated on it changes 952 of 4096 answers, witness `(-8, -8, 1)`), blind; `90` R1/R3,
earlier, at TWO EXPERTS from `76` and `77`; and `111` 2.3's analytic point that a function of the
tables cannot vary while the tables are held fixed, which needs no sweep. The **residue** all
parties kept: a law set read as a *demand* is a predicate over the configuration space (`110` TEST
4: demanding `distrib_add` selects 12 of 48 configurations, all at `F = 0`), a surface a consumer
may use and never a field a consumer may set. **Rung: TWO+ INSTANCES, three instruments plus an
argument. The 0-of-48 count is dead and is in section 3.**

**L9. Closure is prior to every law.** `109` P2's own correction (one-sided saturation over a
signed set is not a completion at all, 36 of 256 pairs escape, so asking whether it associates is
asking about a function of a different type) and `110`'s interval construction returning `(1, 0)`
over a wrapping base, two instances from two directions, stated jointly at `109` 16.7. **Rung:
TWO+ INSTANCES.**

### 1.2 The refinement, location half only

The propagation half (rules, exactness, intersection, the structural predicate, the discharge
arms) belongs to the realisation-map topic, whose authoritative ledger is `122`'s per
`AGREEMENTS.md` section 9, checked by `123`, closed by `124`. It is pointed at, not restated,
because restating a ledger this file's author did not read would be a fresh compression with no
checker.

**L10. The four-part working assumption had the law set and the refinement exactly backwards: it
lists the coordinate that fails the freedom test and omits the one that passes it.** `111` 9.1,
resting on `82` F6 (a declared operand window separates verdicts with every other coordinate held
fixed); `111` and `82` are one persona and `111` says so, so this is **ONE EXPERT** and is recorded
at that rung in Q52 as well. The connection is load-bearing for the statement's shape and nothing
contradicts it anywhere in either sitting.

**L11. The refinement is neither a coordinate of the primitive nor a member of the declared
semantics. It is a grade over a fixed primitive and a fixed observable assignment: a declared
restriction on where an operation's arguments lie, ordered by weakening, transformed rather than
preserved by operations, read only by the arm selection.** `112` sections 1 and 3 concluded it
(the restricted carrier is not closed under the operations, so there is no algebra there for a
coordinate to be a coordinate of, which converts `111` F111-8's falsified hypothesis into the
proof; and a discharged declaration changes no denotation, only which arms are available). `111`
section 18 then confirmed it by the criterion `108:825` states, with r1: 1753 declaration pairs
change the selected arm, zero change an answer, and the moved-observable-axis control reports
differences in the tens of thousands; `111` withdrew its own lean the other way. `157` F157-9
rebuilt the zero on a second instrument at widths 3 to 32 and relocated what it rests on: the zero
measures arm-licence soundness, and the classification follows from the definition of a refinement,
so the analytic half carries no width. `114` section 12 records a third independent reason. **Rung:
CONVERGED, with the author of the contrary lean agreeing, on three reasons and two instruments.**

**L12. A refinement is transformed by an operation rather than preserved by it, so it decorates
derivation nodes and composition is a derivation rather than an invariant.** `111` 9.3 (the
propagated bound predicts the completion merge boundary exactly, both directions, F111-9), with
the region bounded by `112` F112-6/F112-7 to the term shapes swept. Further development is topic
six's and is at `122`. **Rung: CONVERGED as to the shape; the exactness region is `112`'s
correction, accepted by `111` section 19 onward.**

**L13. Weakening is free and tightening is a compile-time refusal naming the instantiation, at
both ends of the declared range.** Three compiled instances on separately built constructions:
`111` F111-12 (dense carried range: three widenings alias `_plain_identity`, tightening `E0080` at
`widen::<Lit<200>, Lit<100>>`), `112` F112-11 (independent construction, same shape), `160` F160-3
(bitpacked column with no element type anywhere in it: weakening changes no bit and no address,
tightening `E0080` at `weaken_ref::<200, 100>`), which closes the packed region I17 protects.
**Rung: TWO+ INSTANCES, three instruments.**

**L14. A discharged refinement licenses substituting one arm for another on a term, never
identifying two primitives, and never reinterpreting bits.** `112` F112-3 (two assignments of an
observable axis compute the same answers under a discharged bound and remain two primitives;
`types merge` false in every row) and F112-5 (eight of 16 bit patterns denote differently under
unsigned against signed while zero of 16 arithmetic results differ on the discharged extent).
**Rung: ONE EXPERT with controls, uncontested through both sittings.**

### 1.3 Adequacy

**L15. The type is whatever must be const-available to decide validity or select a lowering, and
membership in the type is a different question from identity of the primitive; a design needs both
criteria and they do not compete.** `109` section 11 (membership: const-availability, inheriting
I13's "whatever is available at const time" scope from op's own instruction) and `110` section 3
(identity), with the split stated independently by both phase twos (`109` 16.3, `110` 16.3).
`154` section 2 strengthened the membership criterion's ground blind: I15 **entails** saturation,
because a parameter left runtime forces the `cmp` the intent forbids in as many words
(`154_probes/p1_saturation/sat.s`), so the "must" in the criterion is compelled rather than chosen.
**Rung: CONVERGED on the split; TWO+ INSTANCES on the entailment's ground (`154` blind, `109`'s
criterion blind, different routes).**

**L16. Adequacy is the obligation between them: the type owes the denotation soundness (never one
name for two denotations) and completeness (never two names for one).** Named at `111` 8.1-8.2 as
the obligation "nobody's yet", the other two layers being pointless without it. `157` 3.4 then
found that `112:934-937`'s classification rule **is** this obligation stated per axis, one file
after `111` wrote it was nobody's, with neither noticing: two directions of coercion mean a
completeness violation if carried, zero mean a soundness violation if dropped. `158` section 2
reworked the identification independently before rereading `157`'s statement and reports it comes
out the same both times. **Rung: ONE EXPERT (the identification), with an independent reasoned
check; the halves themselves are `111`'s wording, uncontested.**

**L17. The two halves are different kinds of obligation. Soundness is free by functionality where
the denotation factors through the carried parameters; completeness is per pair, discharged by one
witness at any width, and only refutation needs exhaustion.** `157` sections 3.2-3.3: the
certificate compiles at every width 1 to 64 inside a const item, the spurious-axis control fails
to compile, and the witness scheme agrees with exhaustive denotational identity on 1128 of 1128
pairs where exhaustion is affordable (F157-6, F157-7). `158` rebuilt both the certificate and its
control from committed source, byte-for-byte, which is verification rather than independent
derivation and is stated as such. **Rung: ONE EXPERT, verified at source by a second.**

**L18. The obligation is per pair of shipped instantiations, not per axis, because an axis can be
read at some instantiations and not at others.** `157` 3.6/F157-11 (44 pairs collapse on a grid
where every axis has a per-axis witness, all differing in exactly one axis; zero multi-axis
cancellations), which is a second, separately built instrument for `111` F111-6's joint-fact
finding (truncate and floor unobservable at unsigned saturating, separable at wrapping). **Rung:
TWO+ INSTANCES on the joint-fact claim; the per-pair reformulation is ONE EXPERT.**

**L19. Completeness is owed up to weakening, and a witness-only certificate cannot tell a
refinement from a spurious pair.** `160` sections 3.1-3.2: S-14's completeness clause as written
rejects every refinement parameter (a refinement pair shares the realisation map, so no input
separates it, at any width, ever; measured at r1's zero); the repaired obligation is per-pair
witness **or** weakening in exactly one direction, with the neither case refusing to compile
(F160-1, `160_probes/p1_two_branch_certificate/`, all three pair kinds classified, the conflation
demonstrated, the refusal `E0080`). **Rung: ONE EXPERT, compiled, unattacked as yet; it repairs
CONVERGED text and is what section 3 retires that text to.**

**L20. Soundness is not enforceable by a signature, nor by anything that inspects one build; the
residual obligation is a restriction on what the realisation-map call path may read, which is a
lint's shape.** `157` F157-13 (two builds of one source, one `cfg` apart, one type name denoting
saturation and wrapping, both controls stable, and the certificate itself flipping between builds),
rebuilt byte-for-byte by `158`; `159` F159-2 adds the independent second argument from the intent
side (every single build satisfies I15 completely, so the property I15 buys cannot see the hazard,
which is a relation between builds). `109:649-651`'s target-independence clause is the same
constraint with two qualifiers dropped (`157` S-21), and the attribution is `109`'s, corrected at
`159` section 2 after the ninth unit's own brief misattributed it. **Rung: TWO+ INSTANCES (one
compiled instrument, one independent argument).**

**L21. The classification's three verdicts age differently under signature growth: declared
semantics and refinement are stable, spurious is provisional, and eliminating a spurious axis from
the surface is licensed only at the largest signature the design will ever admit; at the shipped
signature a two-direction verdict is a licence to gate under a predicate, per `108:827`, not a
reclassification.** `160` 4.1, assembled from `110` F5/F6 (definitional against reachability
degeneracy), `111` section 6 (the two are one notion at two extents, collapsing under constant
injection, 144 to 0), and the witness-monotonicity argument of `157` S-17. **Rung: ONE EXPERT as
an assembled statement; each part is CONVERGED or better in its own file.**

### 1.4 The lens

**L22. At the packed end a primitive has no standalone `Sized` form; the packed element exists
only as a position in a carrier.** `154` F6, blind, with the compile refusal committed
(`154_probes/p2_fibre/`, "a packed 13-bit element does not occupy 13 bits as a standalone value",
and the nearest expressible standalone form 8x the logical width). The supporting instruments are
**three, not five**: the `Carrier` bound (`warm-container-shared/src/lib.rs:187`, `Copy + 'static`
forcing `Sized`, which is a proof that `155`'s instrument could not have refuted the claim
whatever it found), `bitpack-shared` (no bitpack dependency), and the `bitpack-plan-shared` family
counted as one (`159` F159-1: three of four depend on it, five of six declaration bodies hash
identically). Widened by `159` section 5 to `W any where W mod 8 != 0` on the size-in-bytes
argument, which is a proof and carries no width sweep. **Rung: TWO+ INSTANCES (one blind
derivation, one structural argument, plus corroborating vocabulary counted honestly).**

**L23. Types are the degenerate case of lenses: a primitive's realisation is always a lens, a
placement `(carrier, offset, width)`, and the lens degenerates to an ordinary value exactly where
its focus is the sole logical occupant of its carrier allocation, padding permitted, sharing not.**
The synthesis is `157` S-8 (`157:358-362`), offered to `154` at its concession that no single
account covered both ends; `159` adopted it outright ("this **is** the synthesis I could not
find", withdrawing O-B's cost clause, `159:169-175`). The degeneracy condition as S-8 worded it
(position const-zero in one machine word) is insufficient, admitting the first element of a packed
column; the sole-occupancy repair is `160` section 2 with its compiled instances and the
out-of-carrier refusal (F160-2, `160_probes/p2_lens_degeneracy/`). **Rung: CONVERGED on the lens
formulation (offered, adopted by the party that had conceded); ONE EXPERT on the repaired
condition.**

**L24. Whether a placement has a standalone name is a property of the target's addressing, never
of the primitive, so the canon states the reason and not the arity.** `157` S-6, accepted by
`159` section 4; it is the permanence test applied to the element-against-column question, and it
is what keeps L22 out of the canon text while keeping its consequence in. **Rung: CONVERGED.**

**L25. The lens formulation is invariant under the container premise: the premise decides whether
the carrier is part of identity, not whether the realisation is a lens, so clauses about the
lens's form can be compressed now while saturation stays conditional.** `160` 2.3, argument.
**Rung: ONE EXPERT.**

### 1.5 Naming, cost, composition, chains

**L26. Naming is interesting exactly when it is partial, and dangerous exactly when it is
non-injective: a name is an existence claim, the naming function is the validator under I15, and
two names for one primitive is a missed merge.** The partial half is `109` section 6 (P5b's
compile refusal naming the exact composition that overflows the container); the injective half is
`110` section 5 (the `E0308` wall); the two halves stated as complementary at `110` 16.4. The
cost of the missed merge is **three-armed by where the spellings meet**: nothing at a monomorphic
site, one threaded parameter at a polymorphic signature, no repair at a homogeneous container.
`111` section 7 found `110`'s internal contradiction, `112` section 7 located the resolution,
`110` R3 compiled both repairs and withdrew F8 as stated; `112` F112-9 is the independent compiled
instrument. The storage boundary being the one armless case is what makes a spurious parameter's
cost land on the path I17 protects (`112`'s sharpening, taken by `110` R3). **Rung: CONVERGED,
with the author of the broken claim compiling the repair.**

**L27. An axis the realisation map does not read must not be a type parameter; an axis the arm
selection reads may be one, because weakening repairs it and weakening is free.** `110` F9 (with
the grid-step reparameterisation as the constructive form, F9's rule), sharpened by `112` (the
scope correction `110` accepted: arm-selection-read axes are parameters with a repair) and `111`
9.5 (the spurious-against-refinement asymmetry, compiled both directions). **Rung: CONVERGED.**

**L28. Configuration is not composition. Choosing format, system and strategy fills in a record;
composition is a construction taking an algebra to an algebra; and a composite is a primitive
under the same definition, so the canon needs one concept.** `110` section 4 and F10, blind;
`154` section 7's "terms compose, treatments interpret" is a second blind arrival in the later
sitting, from a different instrument set (footprint and container-relativity probes), classified
by `157` 1.3 as a genuine convergence with the shared bench corpus named. The fibration frame
`154` offered beyond it is `154`'s own proposal and stays at one expert. **Rung: TWO+ INSTANCES
on the distinction and the closure; ONE EXPERT on any particular frame for it.**

**L29. A construction on primitives carries two things of its own: a predicate on its base and a
transformer for its base's refinements; equality transports through a construction for free, and a
predicate never does.** `110` F12 (interval requires monotonicity, 16 of 16, found by failing),
`112` F112-13/F112-14 (borrowing the base's rule is unsound on 26 of 81 for complex; the smallest
sound transformer is per construction and joint with the base's signedness), and `110` R6's p11
(congruence transports 4 of 4 with the sabotage control caught 0 of 4; lifting unsound in 10 of 48
rule firings), which also reproduced `112`'s three figures on a separately built instrument.
**Rung: TWO+ INSTANCES.**

**L30. Chain accuracy is a fact about the operator typing, not about any component of the operand
type: an operator closed over its operand type forces the per-step quantisation, and the moment
the result may be a different primitive the chain story falls out of the typing.** `109` section 8
(deferred never worse on any of 3200 chains, lengths 1 to 8, truncation), the third instance after
`93` P6 and `94` probe E per `106` section 11, and untouched by three consecutive attackers per
`111` section 26. Bounded honestly: truncation only, and `101`'s crossing result (bias accumulates
linearly, unbiased error as a random walk, crossing at `k = 4`) supersedes any per-arm scalar
reading; the rounding-to-nearest attack `109` named against itself remains unrun in this topic.
**Rung: TWO+ INSTANCES (three), with the named untested region.**

### 1.6 About the corpus rather than about arvo, kept because the next unit inherits them

**C1.** Not one of the 82 findings in `110`/`111`/`112`/`114` carries `W any` (`157` F157-10),
**bounded** by `159` F159-4: the generalisation to "topic five" is false because `109` spells its
width universals `I any, F any` (`109:156`), which the instrument's patterns cannot see. The
four-file count stands; the sweeping sentence does not. This is Q65's substance.

**C2.** The workspace rule set is not uniform across members; 23 of 75 rule files carry `paths:`
frontmatter and none of those was in `157`'s context (`157` F157-1), corroborating `154` P2.0 and
falsifying `110:113`'s "auto-loads into every agent context" as stated. Contamination declarations
resting on that sentence are unreliable in both directions.

**C3.** "N bench crates agree" is worth much less than N anywhere in this panel (`159` F159-1's
generalisation), because the bench corpus descends from one template.

**C4.** Every timing carries its build profile; the 107-second retirement was itself retired when
`154` measured 109.08s debug against 3.78s release, a factor of 29, on one host in one session
(Q52's correction). No bare wall-clock figure is a measurement of anything.

**C5.** The certificate's compile-time cost is **unpriced** (`157` Q157-E), and the word is used
deliberately: nothing on `mock/benches/` measures a const-evaluation budget, and the harness arms
are already named (no certificate, per-axis, per-pair, per-pair-with-direction-count).

---

## 2. What is contested, each with what would decide it

**X1. O-A against O-B: one vocabulary or two across the declared range.** Not a disagreement
between experts; a fork op's premise decides. Under footprint-observable, a packed 13-bit column
and a dense 13-bit value differ in identity, not merely realisation, and O-A's two vocabularies is
the honest shape; under footprint-internal, S-8's single lens vocabulary is right. `159` F159-3
mapped the split exactly and declined to argue either branch, which is the correct posture and is
carried. **Decided by `156` item 1.**

**X2. O-C's residue: the boundary shape at the wall.** `159` narrowed `154`'s concession from "the
sentence says too little" to "the sentence says enough about the wall and not enough about the
shape at the wall": a column type with index accessors and a borrowed packed reference differ at
the boundary in ways a consumer observes (storability, aliasing, lifetime). `160` 5.1 locates why
no probe closes it: the equivalence test quantifies over behaviour at the consumer boundary, and
the lens formulation deliberately says nothing about that boundary because saying it would be the
concrete spelling the canon may not carry. **Decided by S-5's unrun discriminator: hand the
statement's lens clauses to two designers, ask each for the consumer-facing shape of a 13-bit
packed column and a 47-bit dense value, and compare behaviour.** A designer question, not a probe.

**X3. Whether a strategy ever changes a computed value.** Raised inside this topic by `110`
section 6, which declined to settle it and stated the measurable discriminator (pick a strategy
pair, ask whether any operation at any width computes a different value) and the warning worth
carrying: I5 licenses a cost function to change an answer, which is exactly the configuration
where substitution-based optimisation stops being sound, so the licence has to be declared and
scoped as a named predicate. The question's disposition belongs to topic eight's ledger (`151`,
checked at `152`), which this candidate does not compress; recorded here as an inter-topic pointer
so the warning is not lost with the fork.

**X4. Where the next dispatch on refinement propagation goes.** `111` section 26 records the
located disagreement (annihilation case against pricing a two-endpoint declaration at construction
sites) and `114` section 12 records the three-way composition on disjoin/select/intersect. Topic
six's, at `122`'s ledger; pointed at, not adjudicated here.

---

## 3. Closed or retired, including what must no longer be cited

Each entry names the thing, the reason, and where the correction lives. A corrected count is not a
refutation: in every entry below where a number died, the conclusion it supported survives on
stronger ground, and `159` section 6's warning is carried verbatim: a corrected count that reads
as a refutation is how a true finding gets retired.

**R1. The 0-of-48 law-set freedom count.** Vacuous by construction: the key returned the swept
axes, so the collision test never fired, and a mutation making the law set genuinely free did not
move the verdict. Withdrawn by its author (`110` R0) after verifying the mechanism itself. **Do
not cite.** The conclusion (L8) stands on three other instruments.

**R2. The five-instrument count for F6.** Three, per `159` F159-1: one dependency family is one
instance. **Do not cite the five.** The settlement of `154` against `155` stands regardless, on
the `Copy + 'static` structural argument, which needs no count.

**R3. F157-10's generalisation to "topic five states nothing about any width".** False of topic
five, true of the four files counted; `109` spells width universals in a spelling the instrument
did not enumerate (`159` F159-4). Cite the four-file count only.

**R4. `154` F11, F12, F13** (the 127-primitive index count, the container-relative degenerate
set, and the declaration criterion). Withdrawn by their author in `154` P2.1: the collapse was a
reachability degeneracy at the thinnest possible signature, the fourth instance of
criterion-tested-against-a-signature-too-thin. The distinction that refutes them (L21's machinery)
is the thing to cite instead.

**R5. `109`'s "the operation has to be a type".** The const-eval wall is real
(`function pointer calls are not allowed in constant functions`, recorded verbatim); the
conclusion drawn from it is false: three other carriers work, one already shipped in
`satfold-shared` (Q52, from `111`). Cite the wall, not the conclusion.

**R6. `110` F8 as stated ("no repair").** Withdrawn by its author and restated with its region:
right about type equality, wrong about consumers; the cost is three-armed by boundary (L26).

**R7. `155` section 5 requirement 1's "for the value, not the container".** Withdrawn in full by
its author (`158` 1.2), on the ground that its own cited instrument excludes, by its bound, the
case in dispute. S-1's lens-one-level-down is the replacement, adopted.

**R8. The retirement of the 107-second figure.** Itself retired (Q52): the figure was a correct
debug measurement, the three refuting measurements were release, and the two were never in
conflict. Cite neither number without its profile.

**R9. The number-system convergence as two instances.** `109` made a category claim, `110` made a
cut, `109` conceded; ONE EXPERT (Q52). The cut itself (radix identity-bearing at `F > 0`,
encoding is presentation) stands at that rung inside L2/L4.

**R10. S-14's completeness clause as written, and S-16's gap assertion as written.** Superseded
by L19's three-outcome form at one expert: as written they reject every refinement parameter, and
the conflation is compiled at `160_probes/p1_two_branch_certificate/`. The soundness half of S-14
is untouched and carried.

**R11. S-8's degeneracy condition as worded.** Superseded by sole occupancy (L23). The lens
formulation itself is untouched.

**R12. `110:113`'s "this workspace auto-loads `arvo-always-optimal-internals.md` into every agent
context".** False as stated (C2). Independence accounting that rests on it is unreliable in both
directions; `155`'s "one shared exposure" sentence is conceded on exactly this ground (`158`
section 5).

**R13. O-B's cost clause** (`Bool` and `USize` become one-element columns). Withdrawn by its
author on adopting S-8 (`159` section 4): under the lens the native end never mentions the lens,
so the cost does not arise.

**R14. O-D** (the deliberately over-counting index). Closed by the container premise **on either
branch** (`159` F159-3): under observable the index counts correctly and O-D dissolves; under
internal it is the live description of exactly that axis and is subsumed by L21's licence
machinery. No further evidence is needed and it is not carried as an option.

**R15. The four-part working assumption itself** (a primitive as a named composition of a format,
a number system, a law set and a strategy). Superseded by the statement in section 4. Per element:
format is two things, one identity-bearing and one selectable (L3, L4); number system is a
category error resolved into the radix cut (R9); the law set is L8's exclusion; the strategy is a
selector whose request-versus-resolution placement was settled in `106` section 1's pair (topic
four's ledger, pointed at), with `109`'s own recommendation to carry the marker withdrawn in its
phase two on that ground.

