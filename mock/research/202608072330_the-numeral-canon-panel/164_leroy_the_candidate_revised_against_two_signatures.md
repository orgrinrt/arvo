# 164. The candidate revised against two signatures

Revision of `161` against `162` and `163`, neither of which signed cleanly. `161` stays as landed,
because both signatures cite it by line; **where this file amends or replaces a clause or entry,
this file governs, and everywhere else `161` governs unchanged.**

**Which clauses of `161` stand.** Of the statement's thirteen clauses: **1, 3, 5, 7, 8, 11, 12, 13
stand** as written. **2, 6, 9 are amended** (conditionality markers; clause 6's first sentence is
replaced by its marked form). **4 is amended** by one phrase (a type-check repair to its class
sentence; its marker was already present). **10 is amended** in one subordinate phrase. Of the
ledger: **L15's rung is corrected, L4 and L21 are amended in one phrase each, L20 and L23 gain
their signatures' additions, L31 is new**. Of the retirements: **R10 and R16 are amended, R17 is
new**. Of the corpus findings: **C6 and C7 are new**. Sections 2, 5, 7 and 8 of `161` gain the
amendments below; everything not named stands.

**Every dissent was reproduced before it was accepted**, per the standing instruction, and the
reproduction caught three things neither signature stated (section 1.4).

---

## 0. The two gates

**Canon gate: passed.** Checked against `INTENTS.md` and `RULES.md` read in full across this arc.
Revising a candidate against signatures is what `113` asks for. Nothing below touches the RATIFIED
rung, and the questions reserved for op are not closed: the container premise, Q65's marker, and
X1 through X4 all remain open below, with X3's *location* moved on `163`'s argument and its
disposition still not adjudicated.

**Test gate: passed, at 123 across 13, the thirteenth count.** Crate by crate at `--release`,
`bitpack-write-contend-shared` serialised and untouched; `164_probes/run_test_gate.sh`, output
`164_probes/gate_release.out`, negative control armed and firing on the control line.

---

## 1. The dissents, reproduced

### 1.1 `162`'s instruments

**p1, the premise-reach model.** Rerun bit for bit against the committed output: REPRODUCES.
Four controls pass, 32 primitives under container-internal against 64 under container-observable,
every identical-`(V,R)` pair split by the observation. This corroborates `157` F157-4 on a second
construction, as `162` offered.

**p2, the `cfg`-in-`const fn` verification.** Rerun via its own `run.sh`, both builds: REPRODUCES,
including the emitted-body extraction (base `cmp; csel`, alt `and`), the stable control, and the
no-branch-on-build check. L20's rung now rests on `162`'s own instrument rather than on `157`'s
premise, exactly as its amendment states.

**The L15 grep.** Rerun: `grep -n "I15" 109_bellard_the_primitive_derived_cold.md` returns lines
310, 320, 452, 454, 656, matching `162`'s report, and the five occurrences are where it says they
are (naming-as-validator twice, the I18 section twice, alternative C once). `109` section 11
grounds its criterion's scope in I13 and nowhere derives the modal's necessity. The refusal is
accepted; see section 4.1.

### 1.2 `163`'s instruments

**p1, clause 9's satisfiability.** Rerun bit for bit: REPRODUCES. On the footprint-internal branch
the carrier pair (two markers, one value set, two carriers per the shipped rule at
`warm-container-shared/src/lib.rs:5-11`) comes out `directions=2, witness=no` and clause 9's
refusal branch fires; on the observable branch it separates. All three controls pass, including
G2, which is a second, separately written instrument for F160-1's refinement branch.

**p2, the offset probe.** Run arm: REPRODUCES bit for bit. A sole occupant at offset 3 of a `u16`
is two bytes, `Sized`, referenceable, round-tripping over all 8192 values; S-8's positional
condition rejects it and sole occupancy accepts it. The control refusal fires with the identical
message; see 1.4 for the one deviation.

**p3, the audit of `161`'s accounting instrument.** Rerun bit for bit: REPRODUCES. The loose
extractor finds the same nineteen anchors, the novel set is exactly the corrected `82` range, and
`count_anchors.py` reproduces its committed output. `161` section 8's instrument is now verified
by a second party and re-verified here, which closes the one thing `162` said its signature did
not cover.

**The retirement check.** `OPTIONS.md:2502` opened: it carries the literal sentence F157-5
measured false. R17 below.

### 1.3 What the reproductions accepted

Everything both signatures measured. Clause 2's extension moves, clause 6's truth value moves,
clause 9's admissibility moves; S-8 fails in a third direction; S-14's defect could not have been
caught by `157`'s own grid because the grid contained no refinement; L15's second instance does
not exist; the accounting instrument is sound.

### 1.4 What the reproductions caught that the signatures did not state

**One. `163`'s committed control error is line-stale against its committed source**, the same
rustfmt-shift class `160` found and repaired in its own p1: the committed `offset_control.err`
names line 53 while the committed, hook-formatted source holds `_OOB` at line 66. Rebuilt from the
committed source, the refusal fires with the identical message at line 66. Nothing in the finding
moves; the artifact is stale in the direction this panel already has a name for, it is `163`'s to
refresh, and the reproduction is the reason it is on the record. (My own first rebuild used the
wrong cfg name before reading the probe's header, which documents `--cfg oob`; noted because a
reproduction that fails for the reproducer's own reason must not be read as the artifact failing.)

**Two. The carrier pair's shared denotation is not a modelling assumption; the shipped suite
asserts it.** `163` section 9 names as its load-bearing risk that its model "assumes two markers
over one `(I, F)` can agree in value set and realisation map". The shipped corpus already asserts
exactly that agreement: `warm-container-shared`'s
`all_four_arms_agree_with_each_other_and_with_the_oracle_on_every_key`
(`warm-container-shared/src/lib.rs:1356`) requires every arm over every carrier rule to produce
the byte-identical result against an independent oracle on every declared key. So on the shipped
evidence the pair `163` constructs has identical denotation by test, and reading 2 (a strategy
separates them arithmetically) is, for this crate's swept keys, measured false rather than open.
The reading-2 escape remains open only for behaviour outside that suite's keys, which narrows
`163`'s risk paragraph in the direction unfavourable to comfort.

**Three. `163`'s reading 3 is not a new enormous position; it is `110` section 6's second branch
arriving through the certificate.** `110` declined to settle whether a strategy is identity-bearing
and stated both branches: if a strategy never changes a computed value, it "is a selector over
presentations and is not part of identity at all". Clause 9 refusing the carrier pair under
footprint-internal-and-no-X3 is that branch stated mechanically: two markers over one `(V, R)` are
one primitive, and the surface may not carry them as two spurious type names, though clause 10
still governs *when* elimination is licensed. Naming the lineage matters because it means nothing
in either sitting invented the position; the certificate made an existing fork's consequence
concrete, which is what a certificate is for.

---

## 2. The conditionality sweep, which is the class repair

Both signatures named instances: `162` found clauses 2 and 6, `163` found clause 9. A report names
an instance; the class is **any sentence in the candidate whose truth, extension or admissibility
the container premise moves**, and the fix is a pass over every clause and every ledger entry
asking that question once, so the count stops being incremented finding by finding.

The sweep, over all thirteen clauses and thirty ledger entries:

| unit | reads the premise? | what moves |
|---|---|---|
| clause 1 | no | stable on both branches |
| clause 2 | yes | **extension** (32 to 64 primitives; `162` p1) |
| clause 3 | no | stable |
| clause 4 | by design | it names the premise; marker already present |
| clause 5 | no | stable; L25's invariance argument is the reason |
| clause 6 | yes | **truth value** of its first sentence (`162`) |
| clause 7 | no | stable (a refinement pair shares carrier on both branches) |
| clause 8 | no | stable |
| clause 9 | yes | **admissibility**, jointly with X3 (`163` p1) |
| clause 10 | one phrase | the maximal-set characterisation ("...is the realisation map's whole domain") is the internal-branch reading; the aging claims themselves hold on both branches, since a witness term survives growth whatever the members are and a refinement pair shares its carrier |
| clause 11 | no | stable |
| clause 12 | no | stable |
| clause 13 | no | stable |
| L4 | one phrase | its first phrase ("the realisation is not identity-bearing") inherits clause 6's dependence; its **content** ("every pure code assignment is presentation") is stable, because an encoding maps values inside one container and no footprint observation can see it |
| L21 | one phrase | same phrase as clause 10 |
| L25 | no | it states the invariance and is the tool the sweep uses |
| all other L, C, R entries | no | checked singly; none reads the carrier |

So the honest count is: **three clauses conditional in full (2, 6, 9), one by design (4), and two
subordinate phrases (clause 10, mirrored at L21; L4's first phrase).** The two phrase-level
dependents are this revision's own catch; neither signature reached them, which is the expected
consequence of counting by instance.

`161` section 4's preamble sentence and section 9's "localised to one clause" are **replaced** by
the paragraph above. `160` section 5.2's sentence "the premise does not reach clauses 1 through 6"
is wrong on the same sweep and is noted here as superseded; `160` stays as landed.

### 2.1 The amended clause texts

**Clause 2 [AMENDED], the marker added and nothing else changed:**

> Its **identity** is that structure up to denotation-preserving isomorphism, relative to the
> declared operation set. **[Conditional on op's container decision: under footprint-internal the
> isomorphism quotients the carrier and this clause's extension is the coarser partition; under
> footprint-observable it must preserve the carrier and the extension refines, measured at 32
> against 64 on the swept grid.]** Of the three sameness relations, nominal, representational,
> denotational, each licensing a different operation (assignment, memory reinterpretation,
> rewriting), only the denotational one is a congruence under composition, which is why it and
> only it licenses substitution inside a composite.

**Clause 6 [REPLACED], per `163`'s refusal of the unmarked form:**

> **[Conditional on op's container decision.]** Under footprint-internal, the realisation is not
> part of identity; under footprint-observable, the carrier component of the lens is
> identity-bearing and the encoding remains presentation on both branches. On either branch the
> realisation is emphatically part of the surface: a consumer may ask for the storage-minimising
> placement, and denotational sameness is what licenses the substitution underneath that choice.
> An axis the realisation map does not read **must not** be a type parameter; an axis the arm
> selection reads **may** be one, because weakening repairs it and weakening is free. The cost of
> two names for one primitive is a property of where the spellings meet: nothing at a monomorphic
> site, one threaded parameter at a polymorphic signature, and no repair at a homogeneous
> container, which is why a spurious parameter's whole cost lands on the storage path this design
> protects.

No wording of the first sentence is true on both branches until op rules, which is `163`'s point;
the marked disjunction is the honest form, and it is not a choice of branch.

**Clause 4 [AMENDED], the one-phrase type-check repair, marker unchanged:**

> ...This holds over signatures whose operations are functions of the value set and the
> realisation map; an observation of the container **is not such a function**, it splits every
> class it touches, and **whether such an observation is in the design's operation set is op's
> open decision**, on which this clause's saturation is conditional.

The repair closes the loose reading `163` named, under which observations are categorially outside
operation sets and the conditional could never be violated.

**Clause 9 [AMENDED], the marker added, the mechanism unchanged:**

> ...A pair with neither, connected both ways and separated by nothing, is a spurious split and is
> refused. **[Conditional jointly on op's container decision and on X3: under footprint-internal,
> a pair of shipped instantiations differing only in the carrier a marker selected is refused by
> this clause unless a strategy separates them arithmetically; under footprint-observable the same
> pair is separated. Which outcome is correct is not decidable inside this topic.]** The
> obligation is **per pair of shipped instantiations**, not per axis, because an axis can be read
> at some instantiations and not at others. The axis classification, two directions spurious, one
> refinement, zero declared semantics, is this same obligation stated per axis.

**Clause 10 [AMENDED], one phrase:** "...which with a full literal is the realisation map's whole
domain **on the footprint-internal branch, and the maximal observation set on the other**; at the
shipped signature, inertness is a licence the resolver may take under a predicate, never a
reclassification of the axis." L21 carries the same phrase.

---

## 3. The escalation: X3 moves

`163`'s argument is accepted: a question that decides whether a clause of this statement can be
satisfied at all is not an inter-topic pointer. **X3 moves from section 2's contested list into
section 6's op-queue, coupled to `156` item 1**, in this form:

> **Op's queue, item coupled to `156` item 1: whether a strategy ever changes a computed value
> (X3), jointly with whether footprint is observable.** Clause 9's admissibility reads both: the
> carrier pair a shipped rule creates is refused under footprint-internal unless some strategy
> separates it arithmetically. Three readings exist (`163` section 3) and none is chosen here.
> The question's *disposition* was handed to topic eight and its ledger (`151`, checked at `152`)
> remains the place its content is argued; what moves here is only where the question lives in
> this candidate, because a clause's satisfiability now hangs on it. `110` section 6's warning
> travels with it: I5 licenses a cost function to change an answer, which is exactly where
> substitution stops being sound, so any such licence must be a named, scoped predicate.

And one narrowing from reproduction catch two: on the shipped suite's swept keys, the arms over
both carrier rules are byte-identical against an independent oracle, so reading 2 is measured
false **for that surface**; it remains open only off it. That is evidence toward how op may weigh
the readings and decides nothing.

---

## 4. Ledger amendments

### 4.1 L15 [AMENDED]: the rung is ONE EXPERT, on the refusing author's own showing

`162` refused the TWO+ INSTANCES rung on its own row; `163` concurred, verified at source and
saying so as verification; the grep reproduces here. The corrected entry reads: **CONVERGED on the
membership/identity split (both phase twos, independently stated); ONE EXPERT on the entailment
(`154`, blind, `154_probes/p1_saturation/sat.s:31-39`), with `109`'s criterion at the rung its own
file supports.** The entailment stands, is not weakened, and is asking for the second read the
rung exists to request. The restored probe anchor is `162`'s amendment 4, taken.

### 4.2 L20 [AMENDED]: the rung stands on its own evidence now

Per `162` section 3: the parenthetical becomes *one compiled instrument, one independent argument
since verified against its own instrument at `162_probes/p2_cfg_in_const_fn/`*. And the wording
class `162` named is adopted wherever this arc carries F159-2: the claim is that **neither build
branches on the build and each emits one path**, not "no runtime check", because the base build's
`cmp; csel` is the declared saturating arithmetic and is exactly what I15 permits.

### 4.3 L23 [AMENDED]: three failure directions, not two

F163-1 is carried: S-8's positional condition is under-strict on sharing at offset zero,
over-strict on padding, and over-strict on **sole occupancy at a nonzero offset**, the case
`160`'s probe did not instantiate and `163` compiled (13 bits at offset 3 of a `u16`, two bytes,
`Sized`, round-tripping over all 8192 values). Sole occupancy classifies all three correctly. The
entry also now carries what the unrepaired wording would have cost, in `163`'s own words: the
canon would have classified the first element of every packed column as an ordinary value, which
is the exact case I17 protects. Rung of the repaired condition: ONE EXPERT (`160`) **plus a
separately built instrument extending it (`163` p2)**.

### 4.4 L4 and L21 [AMENDED]: one phrase each

Per the sweep (section 2): L4's first phrase gains clause 6's marker and its content sentence is
explicitly unconditional; L21's maximal-set phrase gains clause 10's branch note.

### 4.5 L31 [NEW]: the three-senses finding gets a home outside the option that housed it

> **L31. The word "primitive" does three jobs in the governing material, and they pick out
> different sets: a substitution role (whatever stands where Rust would have a primitive), a
> generator (an irreducible element of a generating set), and a lowering survivor (what reaches
> one machine operation).** `154` section 1, blind, un-refuted through both sittings; the
> statement uses the word in the denotational sense throughout and clause 5 covers the lowering
> sense, which is a choice this entry makes visible. **Rung: ONE EXPERT.** Carried here so that
> O-E's eventual closure, either way, does not take the finding with it; both signatures asked
> for exactly this (`162` section 7, `163` section 8).

### 4.6 C6 and C7 [NEW]: two corpus findings the signatures established or restored

> **C6.** The criterion-tested-against-an-instrument-too-thin-to-reach-its-breaking-case class
> now has **six recorded instances** in this panel: `110` P4, `110` P8's first run, `111` section
> 9.4, `154` P4, `157`'s P5 first control, and `157`'s P1 grid containing no refinement while
> S-14 quantified over refinement-bearing pairs. Two of the six are one author's, in consecutive
> sections of one file, the second written in the paragraph that named the first (`163` section
> 2's own accounting). The repair is the one every instance used: build the case that must fail
> and watch it fail before the run that produces the number counts.

> **C7.** `157` F157-2's guard is restored to the record: two of the cold pair's five overlaps
> (O1, O2) are single instances wearing two hats, both being one-step inferences from shared
> ratified premises. No rung in this candidate rests on them; the guard exists so none ever does.
> (`163` section 7 named its absence the one genuine degradation; nothing is wrong today, and
> this entry is what keeps it that way.)

---

## 5. Retirement amendments

### 5.1 R10 [AMENDED]: name which half of the sentence died

Per `163` section 2's amendment: S-14's **witness-only completeness outcome** and S-16's gap
assertion are what R10 retires. The **per-pair scope in the same sentence** (`157:698-699`, "a
design owes a witness per pair of instantiations it ships, not per axis") **survived and is
carried** at L18 and clause 9. A reader retiring the sentence wholesale would retire the half that
is load-bearing for the surviving obligation.

### 5.2 R16 [AMENDED]: the reason is a category mismatch, not a missing discharge

Both signatures signed the retirement and both narrowed its reason; `163`'s form is sharper and is
adopted verbatim in substance: **adequacy contains no model-width result for a transfer argument
to be about.** Soundness is a factoring property of the code, in which no width appears;
completeness at a pair is discharged at the real width directly. A missing argument might arrive
tomorrow; a category mismatch does not. (`162`'s form, "asks for the wrong kind of evidence", is
the same amendment at lower resolution, and its separate note is also carried: F159-2 does not
bear on R16, being about soundness's enforceability, a different half of a different claim.)

### 5.3 R17 [NEW]: the register's literal sentence on rounding observability

> **R17. `OPTIONS.md:2502`'s sentence "rounding at `F = 0` is observable the moment anyone writes
> a non-grid literal" must no longer be cited as stated.** `157` F157-5 measured it false: four of
> six non-grid literals separate nothing (`157_probes/p1b_literal_ties.out`), because a tie under
> ties-to-even and truncation land on the same grid point; non-grid is necessary and not
> sufficient. The underlying existence claim survives on `111` F111-5, whose dense rational
> sample names its three exceptions, and the statement's clause 4 says "a full literal", which
> carries none of the error. The registered compression dropped both qualifiers, and this entry
> is the do-not-cite record F157-5 lacked in `161`.

---

## 6. Options amendments

**S-13 enters the options pass**, per `163` section 6, as the unresolved proposal with nothing for
a compressor to grip:

> **O-S13. Replace "declared operation set" with "declared observation set" in clauses 1, 2 and
> 4.** For: what decides identity is what can be observed rather than what can be computed;
> topic eight reached the same word from the other end (`156` item 1's "visibility under the
> maximal observation set"); and clause 4 already quantifies over a thing that is observed and
> not computed, which is the discriminator firing in the proposal's favour. Against: the rename
> is ONE EXPERT (`157` S-13, adopted by nobody), and adopting a vocabulary under an open premise
> risks baking the observable branch's framing into the neutral text. **Closed by**: op's
> container decision plus one vocabulary pass at the candidate-consolidation step; if footprint
> is internal, "operation set" is exact and S-13 lapses; if observable, S-13 or an equivalent is
> forced, because the operation-set vocabulary then excludes a member of its own class.

---

## 7. Anchors: restorations and the regenerated accounting

**Restored, per `162` amendment 4**: `154_probes/p1_saturation/sat.s:31-39` at L15 (section 4.1
above), `159:176-185` at X2's entry, `159:225-230` at L22, all three panel-internal or probe
anchors whose content had survived without addresses. **Not restored, with the reason**: `163`'s
four dropped anchors, on `163`'s own verdicts: two address a retired sentence, one was correctly
replaced by restoration from the establishing source, one addresses a count the candidate corrects.

**The accounting, regenerated rather than edited.** `164_probes/anchor_accounting/count_anchors.py`
extends `161`'s instrument with `161`, `162` and `163` as sources and this file as the candidate,
same two controls declared and firing; output at `accounting.out`, dropped list in the sibling
`dropped_anchors.txt`, never inlined here. The numbers pasted below are its output:

```
unique line anchors across the 16 sources : 215
unique probe paths across the 16 sources  : 111
line anchors in the candidate             : 7
probe paths in the candidate              : 5
candidate anchors that appear in a source : 6
candidate anchors novel to the candidate  : 1
    NOVEL warm-container-shared/src/lib.rs:1356
source anchors not carried (dropped)      : 209
probe paths carried                       : 2 of 111
```

**Reading the numbers for what this file is.** This is a delta revision under the coordinator's
own [STANDS]/[AMENDED]/[REPLACED] shape, so the near-total drop is the shape working: the ledger's
anchors live in `161`, which stands as landed and governs everywhere this file does not amend. What
the accounting establishes for a delta is the two things a delta can get wrong: the three
restorations `162` asked for are present (they are among the six carried anchors), and the novel
set contains exactly what it should. **The measured novel set has one member**,
`warm-container-shared/src/lib.rs:1356`, which is this file's own reproduction catch (section
1.4's second item), cited from the shipped source directly rather than through any panel file,
which is what novel means and is correct here. The dropped list is in the sibling
`164_probes/anchor_accounting/dropped_anchors.txt` and is not inlined.

**The wording class `163` caught is repaired in this file's own accounting**: the closing claim
is stated over the measured novel set, whatever its size, rather than asserting zero three lines
below a table reporting one.

---

## 8. Coverage, bounded honestly

**Read in full this dispatch:** `162`, `163`, both end to end; their probe sources and outputs as
named in section 1; `109` section 11 reopened at source; `OPTIONS.md:2502` opened;
`warm-container-shared/src/lib.rs:5-11` and the oracle test at `:1356` opened. Everything else
stands on the coverage `160` and `161` recorded.

**Reproduced rather than taken:** `162` p1, `162` p2 (both builds), `163` p1, `163` p2 (run and
control), `163` p3, and `162`'s L15 grep, each bit-for-bit against the committed output except
`163` p2's control, which reproduces with the line-number shift 1.4 records.

**Not verified:** `163`'s p4 and p5 sweeps (its own-entry anchor check and findings-survival
enumeration), taken on its report; the clauses resting on topics neither `160` nor `161` read
(`122`'s ledger, topic eight) remain one compression deep exactly as `161` section 9 states.

**Which sections would move if something here is wrong.** Section 1.4's second catch reads the
oracle test's key set as covering the carrier pair's agreement; if `ALL_KEYS` excludes
configurations where the rules disagree, the narrowing of reading 2 is weaker than stated and
X3's coupling stands unchanged. Section 2's sweep is a reading audit by one author; a fourth
dependent clause found later would extend the table, not contradict it, and the sweep's value is
that the next finder amends a table instead of a count.

**What this file settled.** The conditionality question, as a class: a table over every clause and
entry rather than a count that moved three times. The rung on L15. The home for the three-senses
finding and for F157-2's guard.

**What it moved.** X3, from contested pointer to op's queue, coupled to item 1. S-13, from zero
occurrences to an option with a discriminator. R16's reason, from missing evidence to category
mismatch.

**What it could not.** Choose a branch of the container premise or of X3; price C5; or supply the
second read L15's entailment is now explicitly requesting. All three are named where they live.
