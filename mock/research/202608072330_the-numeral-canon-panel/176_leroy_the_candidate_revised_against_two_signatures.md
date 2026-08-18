# 176. The chain candidate revised against two signatures

Revision of `173` against `174` and `175`. `173` stays as landed, because both signatures cite it
by line; **where this file amends or replaces a clause or entry, this file governs, and everywhere
else `173` governs unchanged.**

**Which clauses of `173` stand.** Of the statement's twelve clauses: **6, 7, 8, 9, 10, 11, 12
stand** as written. **1, 2, 4, 5 are amended.** **3 is amended by replacing one sentence**, the one
`175` refuses, and the rest of the clause stands. **The legend is amended twice**: it gains the
`[argument]` kind and it states the marking convention. Of the ledger: **L1, L2 and L3 are
amended; one R-entry and one C-note are new; O-5's reading is corrected.** The A6/B7 artifact
repair is applied in `173_probes/` itself with numbers verified unchanged, because a generated
artifact's provenance labels are the instrument's, not the candidate's.

**Every dissent was reproduced before it was accepted** (section 1), and the reproduction caught
two things neither signature stated, one of them in this file's own first harness.

---

## 0. The two gates

**Canon gate: passed.** Nothing below closes anything reserved: the container premise, Q65, X1
through X4, the canon-form question coupled to `156` item 2, and whether the observability
principle becomes an arvo intent all stay op's. The B3 repair adopted in section 2 changes which
clause governs the schedule choice; it does not touch any reserved question, and `175` section 0
states the same of the refusal itself.

**Test gate: passed, at 123 across 13**, crate by crate at `--release`,
`bitpack-write-contend-shared` serialised (15 passed; this run is the **fourth** confirmation that
it terminates, after `174`'s first from that seat, `175`'s, and `172`'s).
`176_probes/run_test_gate.sh`, output `176_probes/gate_release.out`, control firing.

---

## 1. The dissents, reproduced

`176_probes/reproduce_dissents.sh`, committed with its output, with a must-differ control (two
unrelated probe outputs diffed against each other) firing:

- **`174` r1** (finiteness against the deferral theorem): REPRODUCES, including the 663
  unrealisable-deferral cells, the 17 cells where no realisable placement attains `pi(exact)`, and
  the worst shortfall of 160.
- **`174` r2** (the profile moves the definedness domain): REPRODUCES, 256/256 agreeing at `off`
  against 53/256 at `on` with 203 diverging and zero value disagreements.
- **`175` marks** (the trailing convention is forced): REPRODUCES; clauses 4 and 5 each end with a
  mark that has no successor sentence, so a leading convention leaves two marks marking nothing.
- **`175` clause23** (the boundary-function family): REPRODUCES; three of four chains carry more
  than one boundary function, the witness four, two placements differing at the boundary on 30 of
  256 inputs.
- **`175` partial3** (the `(x*x)/x` definedness construction): REPRODUCES at both profiles, with
  the 1-in-4096 split, full value agreement, and the value-only check certifying the pair.
- **`175` options** (the census): REPRODUCES; O-171-1 appears zero times in `173`.
- **A6/B7** (the stale provenance strings): CONFIRMED PRESENT at reproduction time; repaired in
  section 6.
- **A3/B4** (the disclaimer): confirmed, 1 occurrence in `60` and 0 in `173`, under the
  whitespace layer.

**Two things the reproductions caught that the signatures had not stated.**

**One, in this file's own first harness.** The first version of the reproduction script reported
`175` partial3 as DIFFERS against a byte-identical binary output, because it omitted the runner's
own `##########` header lines and then emitted one trailing blank the committed file does not
have. That is the **harness class** (`171` 3.1) biting the reviser inside the script built to
check the signatures, caught by reading the diff rather than the verdict; the corrected script and
the note are committed. A reproduction that fails for the reproducer's own reason must not be read
as the artifact failing.

**Two, the shape of B3's witnesses.** `175`'s chains with boundary-function families are exactly
chains where clause 6's deletion licences refuse: `*3 >>1 *5` under a rounding resolution has no
algebra licence (rounding does not commute with the shift) and no range licence (intermediates
off the grid). Under the wrap resolution, where the algebra licence holds, `175`'s own identity
column and `168` 4.1's degeneracy table agree the family has **one** member at every depth. **So
the definite description in clause 2 denotes precisely where a deletion licence holds or an exact
grade is declared, and the contradiction lives only where the licences already say no.** That is
why the unit never tripped over it, and it is what makes `175`'s R2 compose with clause 6 rather
than merely replace a pointer: under the repaired clause 2, the licences become the proof
obligations that a placement meets an exact grade. Neither signature stated this composition, and
it is what section 2 builds into the repaired text.

---

## 2. The refusal, and the repair: the declared grade becomes the invariant

`175` B3 is accepted in full: clause 3's "placed under clause 2" does not denote on exactly the
edges it governs, because clause 2's invariant is a definite description ("the stretch's boundary
function") and a free schedule yields a family. Of the three replacements offered, **R2 is
adopted**, for `175`'s stated reasons (one sentence, removes B2 and B3 together, explains why
clause 4 sits where it does) and for the composition found in reproduction: the grades fix the
referent, and the licences discharge the exact grades.

### 2.1 Clause 2 [AMENDED]

> **2. [normative]** Within a maximal unbound stretch, the design may select any realisation
> whose boundary behaviour **meets the stretch's declared grade** on its definedness domain. The
> exact grades fix the boundary function uniquely, so the earlier form of this clause is recovered
> there: composite correct rounding owes `pi` of the exact value, structural exactness owes the
> exact value, stepwise owes the fully-eager function; the bounded-drift grade admits the family
> of placements within its bound. The licence rests on **two normative premises, both named**: the
> observability-perimeter principle, which bounds obligations **above** (nothing beyond the
> observation surface is owed), and the **declaration itself**, which bounds them **below** (the
> declared grade is owed). It holds in a shipped artifact and fails in a development build, which
> is I18's build bound arriving from an independent direction, **and the profile does not merely
> add an observation channel: it changes which operations are partial**, so the same pair of
> realisations can share a definedness domain at one profile and not at the other, which is why
> the I18 convergence is a convergence rather than a coincidence. Where a stretch contains an
> operation that is partial **at the profile in force**, agreement on definedness is part of the
> declared grade at every profile, because partiality is a binding-free observation channel that
> no assertion flag governs.

This resolves `175` B2 (the unnamed lower bound is now the declaration, named as the second
normative premise), `174` A2 (the partial-operation set moves with the profile, stated once as a
shift in the hypothesis rather than as one more channel), and B3's other half. The two-premise
finding traces where `175` B7 traces it: `167`'s original sentence carried three claims, `171`
quoted it whole and named two, and the third, that the boundary value is owed, is what the
declaration now carries explicitly.

### 2.2 Clause 3's sentence [REPLACED]

The refused sentence, "an adaptation point on a bound edge is forced; on an unbound edge it is
free and placed under clause 2", is replaced by:

> An adaptation point on a bound edge is forced and part of the meaning. On an unbound edge its
> placement is the design's, **constrained by the declared grade (clause 4) and priced by clause
> 5's optimum; where the declared grade is exact, clause 6's licences are the proof that a
> placement meets it.**

The rest of clause 3, including the five obligations and the schedule-is-meaning sentence, stands
as written; both signatures sign it.

---

## 3. The legend and clause 4: A3/B4 adopted without the hedge

**The legend [AMENDED]** gains a fifth kind and states its convention:

> Kinds: **[theorem]**, **[measured]**, **[enumeration]**, **[argument]** (an established claim
> whose support is reasoning about expressibility or structure, with no sweep and no measurement,
> per C-X2's two-predicate practice), **[normative]**. **A clause's opening mark gives its primary
> kind; a trailing mark attaches to the sentence immediately before it.** The convention is stated
> because `175` showed it is forced by the text (two clauses end in marks with no successor
> sentence) and `174` showed a reader must otherwise reconstruct it.

**Clause 4 [AMENDED]** in one mark and one carried disclaimer: the statability sentence's mark
becomes **[argument]**, and the sentence now carries `60`'s own qualifier beside it, quoted: "That
is a statability argument, not a benchmark" (`60:210`). The no-threshold sentence's trailing
`[theorem]` stands; both signatures sign it. `174`'s A3 is thereby adopted in `175` B4's
unconditional form, and the kind-flattening class it names, an argument promoted to a measurement,
joins the warned direction (a sweep promoted to a theorem) in C-X2's note.

---

## 4. Clauses 1 and 5: the profile hypothesis and the finite carrier

**Clause 1 [AMENDED]** per A1/B1, carrying L1's qualification into the statement:

> **1. [theorem, with one measured premise and an enumeration bound]** A program divides uniquely
> into **maximal stretches** of operations none of whose intermediate values is bound outside the
> stretch. Two realisations of a stretch that induce the same boundary function, **on the same
> definedness domain, read at the build profile in force**, are contextually indistinguishable.
> The measured premise is the coincidence of the binding and distinguishing perimeters at
> `debug-assertions = off`; at `on`, binding-free channels exist and the equivalence classes are
> finer, **and whether two realisations share a definedness domain is itself a function of the
> profile**, since an operation total at one profile may be partial at the other. The boundary is
> the act of binding, not the operator or the spelling; a single operation is the length-one
> stretch; the stored-intermediate pipeline is the case where every edge is a boundary.

`174`'s r2 motivates the wording exactly: its probe built to show the clause false at `on` showed
it **vacuous** for its pair instead, because the pair no longer shared a definedness domain, which
is the hypothesis doing its work invisibly; the amendment makes it visible.

**Clause 5 [AMENDED]** per A4, both signatures signing the theorem itself:

> **5. [theorem]** Where the boundary resolution is any fixed selection from the nearest-point
> correspondence onto the representable set, **deferring every interior resolution to the boundary
> is pointwise optimal**, the theorem ranging over realisations of a chain of total steps **over
> exact values**: every placement's output lies in the representable set because the boundary
> resolution fires last, and the deferred output attains the minimum distance by definition. The
> tie rule is irrelevant and idempotence is a consequence. **Under I14 containers are finite and
> the deferred realisation is frequently not computable in the carrier; there the theorem's value
> is a lower bound that a finite carrier sometimes cannot attain by any placement** [measured: an
> optimum exists in all 663 unrealisable-deferral cells swept, is not in general subset-minimal,
> and falls short of `pi(exact)` in 17 of them, worst shortfall 160 of 256], **and the gap is
> governed by clause 7's window and clause 9's band.** The property belongs to the resolution, not
> to chains: a non-nearest boundary projection is beaten, and measurably. **[measured]**

---

## 5. The rung history: A5 and B5 adopted, with the discount that argues against its maker

**L3's end state [AMENDED]** to B5's three-numbers-three-claims form, which subsumes A5's wording
fix:

> **End state.** **(P) as a definition: two instances**, and they are genuinely two by
> failure-independence, since the single faults that kill each are disjoint (the rule being
> inapplicable kills `170`'s and not `171`'s; step 2 being covertly normative kills `171`'s and
> not `170`'s). **(P) as rule-free-derivable: one instance** (`171`'s route, second-read by
> `172`). **(L): zero.** The claim the canon's clause 1 rests on is the second, and there is
> exactly one instance of it. **And the discount neither count captures**: both instances are
> *definitions*, agreed by two members of one model family on one premise set, which is the
> weakest kind of two available, because no failure-independence argument detects a shared
> framing; the stated failure modes are exactly the ones the two authors thought to name. So O-4,
> the cold dispatch with the rule removed, is the only instrument in the register that tests the
> framing rather than the reasoning, and it discharges O-171-1 and O-171-4 together.

The discount sentence is `175` 5.3's, stated against its own maker's interest, and it is carried
here explicitly because it is the clause a compression will drop. Two provenance notes travel with
the entry: `174`'s A5 correction is adopted in substance (no bare "two instances" survives without
its claim attached), and `174`'s locating note stands: the three-way phrasing `170` slid into was
**inherited, not invented**, entering from `AGREEMENTS.md` section 6's multi-route framing and
`168` pass three's C3 before `170` reused it, which is the shared-drift mechanism named at its
entry point.

---

## 6. The register repairs

**R-o [NEW]. O-171-1 is closed and its closure is in the wrong place to be found.** Closed by
`172` section 10's second read, which is the decider `171` named for it; recorded only inside
L3's rung history, where an option-set diff will not find it (`175` B6, census at
`175_probes/options/`: zero occurrences in `173`). This entry is that record, **with the
qualifier that makes O-4's value visible**: the closer shares the premise set whose influence the
option exists to test, so the closure is a second read rather than independent evidence, and O-4
discharges O-171-1 and O-171-4 at once.

**O-5 [AMENDED reading].** Its four named channels are `171`'s untested candidates and **the four
are not the residue**: partiality was in neither `171`'s tested six nor its named four, was found
by `172` from the licence side, and was confirmed by `175` from a construction `172` did not use
(`(x*x)/x`, split at 1 input in 4096, value-only check certifying the pair, both profiles). A
channel outside both lists having existed once, the enumeration bound on (P) is an enumeration
bound, not a perimeter.

**C-X1 [EXTENDED].** The two signatures refuted **seven** of their own hypotheses between them and
kept every run (`174`: three; `175`: four, two caught by its own controls), and `175`'s
control-placement lesson joins the record: **a control evaluated where the phenomenon is absent
returns zero and reads as a refutation** (its C-A identity-that-was-a-clamp and C-D
wrong-chain runs, both kept). This file adds one harness instance of its own (section 1).

**A6/B7 [REPAIRED, in the artifact].** The accounting script's docstring and the sibling file's
header both carried ninth-unit provenance ("161", "thirteen source files"). Repaired at
`173_probes/anchor_accounting/` and regenerated; **the diff of the regenerated output against the
committed output is empty**, so no number moved, which is what makes this a provenance repair on a
generated artifact rather than a change to anything the candidate asserts.

---

## 7. Anchors and the accounting for this file

**The accounting, regenerated rather than edited.** `176_probes/anchor_accounting/` extends the
instrument with `174` and `175` as sources and this file as the candidate, controls declared and
firing; output at `accounting.out`, dropped list in the sibling `dropped_anchors.txt`, never
inlined here. The numbers below are its output:

```
ACCOUNTING-BLOCK
```

**Reading them for what this file is**: a delta revision under the [STANDS]/[AMENDED]/[REPLACED]
shape, so the ledger's anchors live in `173`, which governs everywhere this file does not amend.
What the accounting establishes for a delta is that the anchors this file does carry resolve, and
that the novel set contains only what it should.

---

## 8. Coverage, bounded honestly

**Read in full this dispatch:** `174` and `175`, end to end; their probe sources and outputs as
named in section 1; `60:210` reopened for the disclaimer; `173` at every amended site;
`173_probes/anchor_accounting/` for the A6 repair.

**Reproduced rather than taken:** all six probe instruments and both grep-shaped findings,
per section 1, each bit-for-bit after this file's own harness defect was repaired and recorded.

**Not verified:** `174`'s r1 interior details beyond its committed output (its `pi` is the clamp;
its own coverage note says a grid-shaped `S` would move the shortfall count and not the
existence); `175`'s 5.3 discount is an argument about model families and premise sets, adopted as
the argument it is, testable only by O-4.

**Which amendments would move if something here is wrong.** Section 2's repair rests on `175`'s
family measurement and on the composition found in reproduction; if a reading of "the stretch's
boundary function" as "whichever function the design has fixed" were intended, B3 collapses to a
wording request, but that reading makes clause 2 circular, as `175` says, and the repaired form
is strictly clearer under either reading. Section 4's clause 5 amendment quotes `174` r1's
numbers, reproduced here. Section 5 carries a discount that no instrument reaches; it is marked
as an argument and O-4 is its test.

**What this file settled.** The refusal, by adopting R2 with the licence-discharge composition
made explicit; the legend, by adding the kind and the convention the marks already forced; the
rung history, in the three-numbers-three-claims form with the model-family discount attached; the
register, by giving O-171-1 a findable closure record and O-5 its honest reading; and the
artifact, by repairing A6 with numbers verified unchanged.

**What it moved.** The lower bound of the licence from an unnamed premise to the declaration
itself; the profile from a channel-list item to a hypothesis-shifting fact stated once in each of
clauses 1 and 2.

**What it could not.** Close O-4, which after this revision is the register's single most valuable
dispatch, testing the framing rather than the reasoning and discharging two options at once; or
anything on op's list, which is unchanged.

**Citations, checked by opening them under all four layers.** `176_probes/citecheck.out` for the
`file:line` anchors with both wrong-citation controls firing, and `176_probes/quotecheck/` for the
verbatim quotations under whitespace, markup and case normalisation with both planted controls
behaving; the layer report is in the output, and a zero on a layer is a quoting-style fact.
