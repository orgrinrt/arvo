# Agreements: what this panel actually holds, as opposed to what it has explored

This file is an index, not a canon and not a canon candidate. It reads the four finished
consolidations of this panel (the format concept, the number-system concept, derived algebraic
laws, the strategy axis) alongside every one of op's own files, and separates what is genuinely
settled or converged from what merely looks settled because a consolidation stated it once. Per
op's own `87`, none of the material below is canon: nothing moves into `mock/canon/` until every
topic is done and the canon is written fresh from all the consolidations in one act, which op then
ratifies. A consolidation is "the topic's best available compression and has no standing beyond
that" (`87` section 1, quoted at `90` and `74`'s own opening sections). This file inherits that
posture. It resolves no disagreement, adds no analysis, and picks no side.

Ranking follows the brief: op's own words; multi-instance agreements the source consolidation
itself calls independent; single-expert claims a consolidation carried forward as if settled;
contested or located disagreements; and explicitly closed or retired material. Where a claim
carries no stated predicate, this file says so rather than inventing one, per
`every-finding-carries-its-predicate.md` and per op's own instruction at `INTENTS.md` I13 that an
unmeasured dimension is not written into a predicate at all.

## 0. What was read to build this

The four consolidations in full: `63_spj_consolidation_the_format_concept.md`,
`74_giesen_consolidation_the_number_system_concept.md`,
`90_giesen_consolidation_derived_algebraic_laws.md`, and
`106_giesen_consolidation_the_strategy_axis.md` including its sections 16, 17 and 18 (the repairs
applied after `107`'s and `108`'s independent checks and attack). Every file matching `ls | grep
'_op_'` in this directory: `01`, `04`, `28`, `32`, `34`, `36`, `37`, `38`, `39`, `83`, `85`, `87`,
`88`, `95`, `104`, `105`, `113`. `INTENTS.md` in full, including its "How to read an entry"
section.

Not read: the numbered expert files (`55` through `103` and everything between and around them),
`OPTIONS.md`, `DROPLIST.md`, `RULES.md`, `PERSONA_CALLS.md`, `00_brief.md`, `107` and `108`
themselves (only their effect on `106` as recorded in `106`'s own sections 16 through 18), and the
`seed/` and `archive/` material. Where a consolidation's claim rests on one of those and I cannot
verify it, I say so at the claim rather than silently inheriting it.

## 1. Op's own words, rung by rung

This is the material every other rung is measured against, not an agreement among experts. Markers
follow `INTENTS.md`'s own convention: **RATIFIED** (converged and blessed, one entry only),
**IN FORCE** (enforced by the workspace and repo's own lints, independent of convergence),
**STATED** (his direction, an ack rather than a ruling), **OPEN** (explicitly not settled).
Standing instruction, his own, governing every STATED entry: *"You should not write these as clear
cut and settled. The intent is clear I think, but nothing about them is absolute otherwise"*
(`INTENTS.md` I8 note, restated at `38`).

### 1.1 The correction that governs how every other op statement is read

`01` section 0, before any design content: an opinion he gives before the experts converge is an
**ack**, meaning the direction checks out, not a ratification. Ratification is the last step,
reached only when the experts agree nothing is open and bring him a converged thing. *"Until that
time, my word is only thing that ratifies shit... we are wasting time taking my opinions as
anything other than 'yeah checks out, direction is good' acks."* This is why almost nothing below
carries the RATIFIED marker even though op said all of it.

### 1.2 The intent catalogue (`INTENTS.md`), by entry

- **I1 (OPEN, demoted from a carried RATIFIED at `39`).** The strategy set is not closed at
  exactly four. `Hot`, `Cold`, `Warm`, `Precise` are "a prior attempt at the intent, not the
  intent" (`39`). Number, names and decomposition are all open.
- **I2 (STATED; enumeration OPEN under I1).** Each preset names a stated intent, not a derived
  rule. The four one-line descriptions carried from the prior panel (`Hot` fastest, `Cold`
  smallest, `Precise` most precise, `Warm` the intuitive compromise) are RATIFIED on the prior
  panel's record; their status as an exhaustive list of four is not.
- **I3 (STATED, amended at `104`).** `Warm` behaves as native Rust primitives would. Amended
  2026-08-14: this is **about ergonomics**, not about where arithmetic boundaries land. *"Neither,
  it's ergonomics."* The declared-width-versus-container fork the panel built under I3 is a
  question I3 does not range over.
- **I4 (STATED).** `Warm`'s objective is the intuitive best choice for most every use case;
  imitation serves that objective and is dropped where it is consistently the worse choice.
- **I5 (STATED).** `Hot` may sacrifice soundness; that is its explicit purpose, bounded by a
  **provable, meaningful** gain, never lost for nothing.
- **I6 (STATED).** `Cold` is for cold paths and cold storage: it aggressively bitpacks, and the
  coldness of the path is what licenses non-efficient choices, not merely permits them.
- **I7 (STATED).** `Precise` is accurate across chains and ops, "not only alone," at whatever
  performance cost makes sense.
- **I8 (STATED, corrected at `88`).** All strategies are decided by measurement; they measure
  different things and weigh those measurements differently. Op, 2026-08-13: everything after
  "for the most part, they probably agree" was musing on the spot and is **not** part of the
  intent; whether the weightings actually agree is an ordinary empirical question, not an intent
  he owes an answer to.
- **I9 (STATED).** The strategy is what makes an answer correct: strategies "are the variables
  that change what the 'correct' answer is for what we choose as the path." Whether this names the
  whole strategy pair or only its policy half is explicitly **not his to rule on** (`104` section
  3): *"I think the intent is clear and this is impl detail that already had answer: optimal and
  converged to by experts (plural, iterative)."* This is returned to the panel, not settled.
- **I10 (STATED).** arvo takes no stance on core count. It runs at threads = 1, 2, or any finite n,
  and adapts wherever adaptation is proven to improve performance without sacrificing soundness.
- **I11 (STATED).** arvo is a library, not a program; its selling point is the algorithm crates
  downstream (hilavitkutin, vehje, and others) consume, and the composition contracts for units
  bigger than a numeral, not the numeral alone.
- **I12 (STATED).** An opinion given before the experts converge is an ack, not a ratification
  (restated from `01` section 0; governs every other STATED entry).
- **I13 (RATIFIED, the panel's only entry at this rung).** The work is predicated arms composed;
  a universal solution is rejected because the strategies make one impossible by premise. *"We are
  not writing a generalization, rather a bunch of arms with const predicates that optimize each
  little 'sometimes'... thus 'everywhere' is optimal by construction as a composition of those
  sometimes."* Op further specified, as his instruction rather than as a second ratification: an
  unmeasured or unknown dimension is **not written into the predicate**, it is assumed not true
  until proven true (no "unsure" entries). And, per `83`: the admissible category for a predicate
  is **whatever is available at const time**, which reaches const functions and const data outside
  the typestate, with the typestate usable but not exclusive inside such an expression. He did not
  settle whether a genuinely non-const condition may gate anything.
- **I14 (IN FORCE).** `#![no_std]`, no `alloc`/`Vec`/`String`/`Box`; const sizes, no runtime
  growth; monomorphisation as dispatch, no `dyn`/`TypeId`/`std::any`; no platform dependency; the
  stack's own primitives at public API positions. Op, `85` section 3: these "are very explicitly
  also arvo intents and rules... not to be questioned," and nothing built on them "needs redoing,"
  correcting a panel-wide hedge that they were unratified ground.
- **I15 (STATED).** Never any runtime checks, ever. *"We catch invalids on compile time, and
  unused paths we clear out when lowered. Period."* Branching is const-time and erased by
  monomorphisation "as much as possible"; the checks themselves are never runtime, without
  exception in that half of the sentence. This closes the runtime column of the panel's
  validation grid and refuses `68`'s ingest-boundary argument and `80`'s value-gated arm as design
  options (per `85`).
- **I16 (STATED).** The canon does not police what shape a law takes. *"If a law is a law, it
  should be expressed so that it actually works, be it typestate or const expressions or
  whatever... The law is defined as makes sense and is applicable in each situation on a case by
  case basis."* The requirement on a law's expression is functional (it must reach one lowered
  path), not structural.
- **I17 (STATED).** `Cold`'s intent, that the storage-minimising path is not deprioritised,
  survives the strategy set being reshaped, resized or renamed. *"The intent governs, BUT in
  intent alone, not in any arbitrary amount that made sense back when it was written... Whether the
  strategies are four or seventeen or a billion is besides the point of the intent."*
- **I18 (STATED, "a rule of thumb, not an absolute").** A native-primitive-style overflow panic is
  permitted, and I15 bends for it, bounded by build (dev/debug only, never in a release artifact)
  and by concern (imitation, never cost). Op corrected his own first wording mid-answer; the second
  statement, *"It's the intent inferrable,"* governs over either statement's literal marker names.

### 1.3 Op's process-governance words (not design intents, but binding on how the panel is read)

These are not in `INTENTS.md` because they govern the panel's conduct rather than arvo's design,
but they are as much his own words as anything above and several change how the design material
must be weighed.

- **`87`: the canon is written once, at the end, from every consolidation together, and op
  ratifies that single act.** Nothing moves into `mock/canon/` before then. A consolidation "is
  input, not canon in miniature." This is why nothing in this file, including the material at
  section 1.2, is more than a candidate until that act happens. `87` also states the next-unit
  criterion (strictly bottom-up, whatever settles the most downstream at once) and that `Cold`'s
  intent (I17) survives strategy-set reshaping.
- **`95`: a unit has to end in agreement, or it has not ended.** Attack remains vital and is not
  demoted, but it is the middle of the work, never its end; a refuting expert owes "several
  solutions" back to the refuted party, and a located disagreement stated precisely, where genuine
  convergence fails, is a result. This governs how sections 2 through 5 below read: convergence
  earned by resumed argument outranks convergence manufactured by a compressor.
- **`113`: steer the experts, make them build together, and do not stop at the refutation.** The
  refuted party must be resumed to answer rather than replaced, and the canon candidate is
  established together by the experts who built it. `112` (uncommitted, in flight as of this
  writing) is the first unit dispatched under this instruction and is outside this file's scope.
- **`88` section 4: there is never a universal answer, and asking for one is the anti-pattern I13
  already names.** *"Take the win where it applies, gate it out from where it does not. No single
  one-fits-all solutions, it's impossible."* Stated three separate times in one sitting against
  three different category-wide forks the coordinator built (`85`, `83`, `88`), and it recurs as a
  finding inside `106` (see section 4 below).
- **`105`: `Strategy` is arvo's name; notko renames its own profile-tier concept**, because op has
  already ruled the two are "not one mechanism" but share "synergy, nothing more." This settles
  which side moves and nothing about notko's replacement name.
- **`04` / `28`: the panel's standing mode is explore, not settle**, extended from one night to
  roughly its first hundred files, and none of the eight questions `28` records is a lock; several
  of them (the family question, the coordinate-naming question) remain genuinely open to whatever
  extent the consolidations below have not since closed them by measurement.

## 2. The format concept (`63`)

Written 2026-08-09, before I13, I15, I16, I17 or I18 existed, so its candidate sentences (C1
through C10) were never checked against them. Its own account of its rungs is unusually precise
and is carried here largely verbatim.

### 2.1 Multi-instance, per the consolidation's own account

- **C1, the standard model** (`computed = adapt(exact)`): **TWO EXPERTS with a stated
  shared-literature discount.** `55` and `60` each derived it cold (git-verified: phase one
  committed before phase two in both cases), and both draw on the same numerical-analysis
  literature (Wilkinson, IEEE 754), which both files declared themselves. The consolidation is
  explicit that this discount is "worth more than a read and less than two arrivals from nothing."
  Completed by a negative case: eager fixed-point multiply at nonzero fraction cannot supply an
  associative ambient operation, a structural argument from two authors, not a proven theorem.
- **C2, identity (D, Q)**: the value-level half (identity is denotation, not encoding) is **TWO
  EXPERTS with the same discount** (`55` and `08`, both resting on Flocq, `08` read through `55`
  and `56` rather than directly by the consolidator). The refinement from Q alone to (D, Q), and
  the dissolution of the reduction into derived structure, is **"converged by attack and
  concession,"** explicitly marked as weaker than TWO EXPERTS because the conceding file (`55b`)
  read rather than independently derived.
- **C3, the representable set** (one affine membership predicate, phase included explicitly): the
  one-predicate unification is **TWO EXPERTS with the shared-literature discount**; the necessity
  of the phase term is **ONE EXPERT plus a concession plus a constructive repair** ("two
  independent instruments erred at the phase coordinate in opposite directions," which the
  consolidation offers as the argument for stating phase explicitly, not as a TWO EXPERTS rung).
- **C5, the absorption criterion**: stated first by `57`, its exception characterised twice by
  independent probes, then **reconfirmed under two wider sweeps by `61` with an independent
  instrument**, and accepted without residue by `57b`. The consolidation does not label this
  TWO EXPERTS explicitly but treats it as the strongest law-layer result in the unit.
- **The scale-independence of the additive column** (Q17's headline table, also feeding `90` and
  `106`): supported by three separately built instruments converging on the same figure
  (`57_probes/p3`, `61`'s wrap-closure argument, `62`'s generalisation), with the pre-existing
  `35_probes/p3` figure (70.1% divergence, unsigned wrap 0, signed wrap 0, unsigned saturating 0)
  folded in as a fourth, independently generated months earlier.

### 2.2 Single-expert claims carried as if settled, flagged

- **C6, the law frame** (H1/H2: ambient associativity plus a congruence kernel jointly decide
  associativity of the induced operation): explicitly marked **"provisionally passes"** by the
  consolidation itself. *"The frame itself is ONE EXPERT and unattacked... nobody has tried to
  break the frame."* The consolidation names this as the single thing it most wants attacked next.
  (It was subsequently attacked from the number-system topic and survived; see section 6 below.)
- **C9, the chain concept** (a chain is exact operations plus a schedule of adaptation points; the
  schedule is part of the function's meaning): **ONE EXPERT, cold, reconciled without
  contradiction, and unattacked** within this unit. Three directions for where "the chain's home"
  lives (closed ops elsewhere, the three-carrier concept C9 states, or a first-class typed chain
  object) are all left live; the consolidation explicitly restored the third after its own first
  draft dropped it, catching itself only via the independent entailment check.
- **The one-bit accumulator constant and the fusion-savings constant** (C8): both stated as
  measured, not proven, single-instrument results, and the consolidation deliberately writes C8's
  candidate sentence to survive without the exact constant, "which is how a canon should hold a
  constant that is measured and not proven."

### 2.3 Contested or located, per the consolidation

- **The wrap-order sentence** ("a wrapped numeral has no arithmetic-compatible order"): "owed
  under every filing equally," so it does not distinguish anything, but its exact wording depends
  on comparison vocabulary no unit had yet built.
- **The D-A/D-B/D-C fork on where a chain "lives"** (closed operators, the three-carrier concept,
  or a typed chain object): none killed. D-A survives only under a reading of I7 op declined to
  rule on; D-C is unattacked and carries real, named costs (a second vocabulary, type sizes growing
  with expression size, drift toward a computation-graph framing).
- **`56` was never resumed** on three separate open questions from three different later files, so
  this unit's own convergence is incomplete by its own account, independent of anything settled
  since.

### 2.4 Explicitly closed inside the unit

The four-choice-tuple framing of a format (conceded in favour of the identity-and-realisation
split). Wrap filed as an "adaptation with a permanent exception list" (withdrawn by its own
proposer). The clamp-counting mechanism from an earlier panel file (`42`), which the consolidation
found had already refuted itself and been misread three times over before being corrected.

## 3. The number-system concept (`74`)

Written after `63`, at the same explore-not-settle rung: nothing here holds RATIFIED and the
consolidation states plainly that the highest rung available to anything in the file is TWO
EXPERTS, because every claim resting on an intent inherits that intent's STATED status.

### 3.1 Multi-instance, per the consolidation's own account

- **The number-numeral distinction, laws-as-identity, gate-free expressibility of the pipeline**:
  **two blind instruments over one shared premise set** (`65` and `66`, both phase one), discounted
  accordingly, plus the format unit's already-converged text carried through their reconciliation.
- **The crate-table cross-check's evidential worth is zero**, because it describes the removed
  crate tree: attacked by `65` phase two, independently seconded by two further files. The
  consolidation calls this "the closest thing in the unit to a settled correction."
- **The strategy-selects-the-correctness-relation claim**: went from zero probe instances anywhere
  in the panel to one, inside this unit (`67`'s fifth instrument, built after reading a checkpoint
  naming the gap). Recorded honestly as one instance, not multiple.

### 3.2 Single-expert claims flagged, and the "nominal" read-twice trap

- **The consolidation names its own worst instance of overcounted independence**: two files' "the
  difference is nominal" reconciliation of a located question (where the selected adaptation
  lives) was read by both after reading `63`, so it is "a read twice over, not TWO EXPERTS," and
  the file that attacked it (`67`) has never been answered by a defender of the nominal reading.
  This stands as a located disagreement (D1 below), not a settled point, despite reading as
  settled in the register.
- **Most of the fifteen candidate canon sentences (N1 through N23) sit at ONE EXPERT**, each
  explicitly marked so, several flagged by their own author as "the sentence most wanting attack."
  N6 (the two-depths sentence, what a crossing means versus what it costs) is singled out by the
  unit's own files as the one everything else is scoped by and therefore the one most in need of a
  second, independent derivation.
- **The three-role model** (storage, compute, interchange): ONE EXPERT. A second file's claim of
  independent corroboration is a miscitation (it attributes to `63` a sentence that is verbatim
  another unit-two file's), corrected in this consolidation's own section 7.

### 3.3 Contested or located, per the consolidation (five named disagreements)

- **D1**: where a system's selected adaptation lives, identity or realisation or dissolved by a
  two-layer split. Genuinely unresolved; the "nominal" reading above was never independently
  defended against attack.
- **D2**: a word collision. Two units use "format" for different prefixes of the same dependent
  sequence, and the panel separately carries two load-bearing uses of "crossing." Not a canon
  candidate until resolved; explicitly op's naming call.
- **D3**: whether the "role" set (storage, compute, interchange, ...) is homogeneous. Turns on
  whether any role may widen the representable set before the encoding, which is a live,
  unresolved register entry.
- **D4**: whether the ambient domain's operation family (addition, multiplication) is fixed or a
  parameter. The consolidation calls this "the unit's largest open fork." Both readings are
  coherent; neither is free.
- **D6**: whose reduction governs a lossy crossing, source's, target's, or named at the crossing
  site. Three coherent positions, none forced by anything measured, explicitly op's.

### 3.4 Explicitly closed or corrected inside the unit

The standing admission test's **sufficient** direction is refuted (a self-declaring system that
names its own computed algebra as its ambient domain passes the test while computing nothing); the
**necessary** direction survives untouched. The order-or-magnitude boundary for "number system" is
measured empty over every total order at two widths (finite groups admit no translation-invariant
total order), which forces wrapping and GF(2)^n into the same bucket regardless of which reading of
"number system" is chosen. Twelve corrections to specific member-file claims are itemised in the
consolidation's own section 7, including one miscited line count into `RULES.md` and one figure
(952) attributed to the wrong operation, a mistake `90` later independently confirms and corrects
again (see cross-topic section below).

## 4. Derived algebraic laws (`90`)

The first consolidation written after I13, I15, I16 and I17 all existed, and the first to be
checked against them explicitly at the gate.

### 4.1 Multi-instance, per the consolidation's own account

- **R1, "a law is a fact about an operation composed under a fixed arithmetic semantics, never
  about a type or an operation alone": the unit's only TWO EXPERTS claim.** `76` and `77` derived
  it cold, in parallel, blind to each other, from I9, in two different formal traditions. A third
  file's attempt to count an earlier same-author instance as further corroboration was corrected
  down to "two independent instances plus one earlier same-author instance at lesser weight."
- **The reproduction chain never broke.** Every later member re-ran an earlier member's committed
  instrument and every rerun matched to the digit, across five separate re-run events spanning six
  member files. This is a genuine multi-instance result about the panel's own instrument
  discipline, not about arvo, and the consolidation treats it as a finding in its own right (R13).
- **The band-transfer defeat, in both fragments.** Two independently constructed families
  (`84`'s ring-fragment threshold family, `86`'s saturating-fragment family) both place a law's
  truth-value flip exactly at the shipped width, where no model-width band, at any guard setting,
  on any host, could have disagreed. Two fragments, two authors, the same structural shape.

### 4.2 Single-expert claims flagged, with the unit's own corrections chain as the evidence

The consolidation states its own corrections chain (section 4) as a first-class result: seven
sequential corrections, each landing exactly at the point the corrected file had itself flagged as
least certain. This is offered as a **methodological** finding (the least-certain-item flag is the
real attack surface), not as a claim that any individual corrected sentence is now multi-instance.
Individually flagged single-expert claims that stand as R-numbered results without a second
derivation: R4 (the const-eval frontier depends on domain size, per-tuple encoding, and procedure,
three separately discovered factors that were never stated together before this unit), R7 (sign
uniformity of a declared operand window is exact for signed saturating fold reassociation, with its
**sufficiency** direction later made proof-shaped by a fourth file but its **necessity** direction
remaining one file's measurement, "corroborated but not re-proved" in the consolidation's own
words), and R8 (the closure criterion for lifting a trajectory condition into a declaration),
explicitly marked low-confidence by the consolidation for a reason unrelated to its content: an
earlier draft of this very file misattributed which of R8's supporting instances shared an author,
an error the consolidation records itself as having made and repaired.

### 4.3 Contested or located

- **Q38's route (c), the model-band cross-check, is corrected rather than merely disputed**: the
  consolidation states plainly that it is "strictly dominated inside any fragment with a test-set
  theorem," which is a result, not an open fork, but the class of laws with **no** such fragment
  (general signed laws with both clamps reachable) remains genuinely unresolved, with one route
  closed by a non-constant tail and the expensive alternative named and unbuilt.
- **Q41, whether the strategies are partially ordered by how many chain-level laws they honour**:
  offered explicitly as a falsifiable candidate by one file, and the consolidation notes plainly
  that **no later member of this unit engaged it at all**. Restored into the register after this
  consolidation's own first draft dropped it (caught by the independent check that followed).

### 4.4 Explicitly closed or killed inside the unit

The model-band mechanism as a general verdict carrier (killed by construction in both fragments,
though its narrower use validating a checker's own implementation survives). The register sentence
"at a shipped width the compiler produces only negative verdicts" (killed as a universal;
positive verdicts at width 64 are reachable inside a fragment with a theorem). Value-gated arms
(killed twice: measured worse than either static arm, then closed on principle by I15). The natural
"my exact result stays in range" declaration as a licence (killed by measurement: sound-looking,
wrong on half the domain). The panel-wide hedge that I14's constraints were unratified (killed by
op at `85`, with the explicit statement that nothing built on them needs redoing).

## 5. The strategy axis (`106`, with its own sections 16 to 18 as later, superseding repairs)

**This topic's material needs the heaviest caveat of the four.** The consolidation's own section 18
records that after it was written, a further check (`107`) and an attack (`108`) found that **five
of the eight clauses in section 1's central definition, "a strategy is a pair," needed repair**,
including one clause the consolidation calls "false, and it is the one that mattered" (component
two was defined as ranging over "the arms that produce the answer the first component fixed," which
section 8 of the same file shows would make an accuracy coordinate measure a constant, directly
contradicting the claim that the mechanism serves I5 and I7). `106` states explicitly: *"Whoever
writes the canon takes it from there [`108` section 7] rather than from here."* `108` itself was
outside what this file was told to read. **So the strategy-pair definition below is presented as
`106` states it, with `106`'s own note that it is superseded, not as this topic's settled output.**

### 5.1 Three or more independent instances, per the consolidation's own account

- **The selection erases at compile time.** Four separate instruments across three files: a
  const-fn argmin compared against a hand-written arm (identical after label normalisation), three
  probes reading entry bodies and finding a single tail branch with zero conditionals, a compiled
  comparison where the assembler emitted a symbol alias for two encodings, and a committed harness
  run where the two paths' timing intervals overlap. The consolidation calls this "the single
  best-supported claim in the unit and it is the one the whole mechanism rests on."
- **The rationalisability counts** (which arms a non-negative or strictly positive weighting can
  select on a committed cost table): **three independent implementations from three different
  geometries** (extreme-ray enumeration of a pointed cone, interval arithmetic on the weight
  simplex, polygon clipping written without opening either predecessor), converging on the same
  counts. The consolidation states this explicitly clears the panel's own three-instance bar.
- **Multiplicative associativity and distributivity hold at F = 0 and fail at F > 0, for unsigned
  types, with the exact predicate stated.** And, separately and independently re-measured inside
  this very unit: **at signed saturating, F = 0, two independently written models both measure the
  law failing** (47.72% of triples at one width, 34.52% at another), which the consolidation is
  explicit is the *third* time this exact qualifier has been lost from the sentence in this panel's
  own history (first at an earlier panel file, again at a member file inside this unit, and again
  in the standing workspace rule `arvo-always-optimal-internals.md`, which was corrected during
  this unit precisely because the unqualified form was a live licence to emit a wrong rewrite).
- **The corpus's test suite is 123 tests across 13 crates.** Five independent counts across four
  member files plus the consolidation's own rerun, with the consolidation itself finding and
  correcting a sixth, wrong count produced by its own first invocation (a `tail -4` extraction that
  silently read the doc-test result line instead of the unit-test line, reporting zero tests at
  exit code 0). This is recorded as a result about the corpus's own instrumentation, separate from
  the count itself.

### 5.2 Two experts, each deriving before reading the other

- **A strategy is a preference over measurements, resolved as a compile-time argmin over candidate
  arms, with the container, codegen choice, and overflow rule as effects of one cause rather than
  three components of a marker.** Two files, blind, in parallel, from different premises (one from
  a partial order on cost vectors, the other from two named intents read together).
- **Chain accuracy cannot be served by an operator closed over its operand type; the required
  intermediate width grows linearly in chain length.** Two files, two different parameter settings,
  blind.
- **The named strategies are points in a product, and the flat named set is a slice through it.**
  Two files, two disjoint arguments (one from the resolution side by counting, one from the
  component side by measuring the cost of an unnamed point).
- **175 of 254 committed bench regions were produced before the harness's cross-variant validation
  even existed.** One file's measurement, independently re-derived by the consolidation itself from
  the raw commit metadata, with an exact match and one honestly-stated bookkeeping reconciliation
  (24 distinct commit-hash strings versus 23 distinct commits, because one commit appears both
  clean and dirty in the corpus).

### 5.3 Single-expert claims flagged, with the "pair" caveat repeated

**The strategy pair itself, as originally stated in section 1, is single-expert, arrived at
seventh in the unit's sequence, and was never attacked before the consolidation shipped.** The
consolidation is explicit about this and about what followed: five of its eight component clauses
were subsequently found to need repair (section 18), one of them (the range of the weighting
component) false in a way that directly undercut the claim that the mechanism serves op's accuracy
intents.

Other single-expert claims explicitly flagged as needing a second read by the consolidation's own
account: the polarity distinction between "observable" and "unobservable" coordinates (the
consolidation corrects the panel's own register, which had been treating this as a two-instance
result; the file usually cited as the second instance states of itself, in its own words, that it
"did **not** derive it independently"); the strict-positivity requirement for the
no-dominated-arm guarantee, alongside a fourth, cheaper option (unique-argmin) that a later file
showed buys the same guarantee without forbidding a zero weight; and twenty of 254 committed
regions found not to be answer-pinned, a claim that itself corrects an even larger single-file
overclaim (that every committed region was answer-pinned), discussed in the cross-topic section
below because of how it was reached.

### 5.4 Contested or located

- **The generate-a-table-versus-check-a-table fork.** The consolidation calls this "the unit's
  largest unclosed item": named by the checkpoint as the first thing later members should attack,
  attacked around its edges by two later files, and never attacked at its core by anyone. It
  remains open.
- **Which object the word "strategy" names**, the pair, the policy half alone, or a "named
  binding" combining one point from each. Op explicitly declined to rule on this and returned it
  to the panel with a decision procedure (convergence among experts). The consolidation offers its
  own ONE EXPERT attempt at that procedure, explicitly marked as a contribution rather than a
  compression, and section 18 separately notes the consolidation's own claim of "three instances"
  for the underlying two-level structure overcounts: it is TWO EXPERTS at most.
- **Whether the rationalisability constraint has any content once the "pair" object exists.** Two
  files read op's own words on this two different, both-coherent ways, and neither has been tested
  against the other.

### 5.5 Explicitly closed inside the unit

The four historical responses to cross-strategy resolution, shown to be correct answers to three
different questions plus one wrong one rather than four competing designs. A total silent join
over the observable axes as the resolution mechanism (refuted: no such join exists on the
measured data, and the operation must report a conflict instead). A divergence coordinate for I3
(closed twice, once by measurement and once by op directly at `104`). A value-level accuracy
lattice over any finite marker set (closed on arithmetic grounds: the bound does not survive
multiplication at any set size). Incrementing a const generic to carry chain depth (closed by the
forbidden-feature list; replaced by a compiled trait-based alternative).

## 6. Cross-topic agreements

These are the claims two or more of the four independently-dispatched topics converge on without
either citing the other as its source, which is exactly the class each consolidation warns is
easiest to lose because no single topic's compressor has both halves in view.

**The F = 0 qualifier, and the general lesson that a predicate must carry every dimension it
depends on, recurs across all four topics as both content and method.** `63` (format concept)
establishes the cube showing unsigned multiplicative structure holding at F = 0 and dying at
F > 0, with the signed cell "worst" and broken even at F = 0 under clamping. `90` (derived laws)
restates the same boundary as R1's dimension list and treats "a frontier number without all three
factors it depends on" as a defect class in its own right (R4). `106` (strategy axis) independently
re-measures the signed-F=0 failure with two new instruments and states explicitly that this is the
**third** time in the panel's own history the qualifier has been dropped from the sentence,
including once inside the standing workspace rule `arvo-always-optimal-internals.md`, which was
corrected during the strategy-axis unit specifically because of this. No consolidation cites this
as a cross-topic convergence explicitly; it is visible only by reading all four together.

**I13's "arms with const predicates, reject the universal" shape is independently the organising
principle of the two later topics.** `90`'s gate section names I13 as the one RATIFIED entry and
structures its entire finding set (R1 through R13) as region-scoped arms. `106` states plainly that
op rejected a category-wide-policy question shape "four separate times" across the panel (`85`,
`83`, `88` twice), three of those four falling inside the derived-laws and strategy-axis topics,
and section 6 of `106` composes its own "four arms with disjoint predicates" recommendation
explicitly in that form. `63`, written before I13 existed, arrives at a structurally identical
shape independently: its C5 (the absorption criterion) and its sign-confinement corollary are
explicitly carried "as a corollary, never as the criterion," which is the same region-scoped
posture stated before the rule that would later require it.

**The chain / multiplicative-accuracy finding recurs, without contradiction, across three
topics.** `63` (format concept, section 5, one cold derivation): a chain is exact operations plus a
schedule of adaptation points, and closing operations over the format so adaptation fuses invisibly
into each one makes op's chain-accuracy intent (I7) unstatable. `90` (derived laws, R11): chain
laws split by whether a lifting theorem exists; multiplicative chains need width growing linearly
in fold length, with no logarithmic closed form, and the schedule kind is what I7 is stated over.
`106` (strategy axis, section 11, its own independent blind cold pair): the same linear-growth
result, reached independently at different width parameters by the two files that opened the
topic, later sharpened into "chain length is a region dimension" and "the crossing point where one
accuracy arm overtakes another is itself decided by a weighting." All three land on the same
structural conclusion, computed independently three separate times, and none of the three cites the
prior two.

**`63`'s law frame (C6, marked "provisionally passes" and explicitly named as the thing the unit
most wants attacked) is attacked from a different topic and survives.** `74` (number-system
concept) built a dedicated attack on it "from a direction it was not built to face" (a
self-collapsed system that names its own computed algebra as its ambient domain) and the frame
held; `74` states this plainly as a failed refutation rather than a proof. `90` then cites this
same frame's boundary condition (H1, ambient associativity) becoming a const predicate over a
declaration, extending rather than re-testing it. This is not a contradiction: it is the panel's
clearest instance of a single-expert claim from one topic being independently pressure-tested by a
different topic and coming out intact, which is stronger evidence than either topic alone states.

**The strategy axis is independently identified, by three different topics, as the shared
placeholder every one of them terminates on.** `63`'s C4 (the adaptation slot) names "whatever a
strategy resolves to" as an open dimension. `74`'s crossing and pipeline sections defer the same
question. `90`'s dimension list for a law's region ends in the identical phrase, "whatever named
axis a strategy resolves to." Op's own `87` names the strategy axis as the next unit specifically
because three topics had independently reached the same dead end (`87` section 3, read alongside
`90`'s account of the same reasoning). This is the strongest cross-topic agreement in the panel
because it is stated by op himself as the reason for the unit ordering, not merely observed by a
consolidator after the fact.

**The 952-triples figure was independently mis-attributed to the wrong operation twice, in two
different topics, and corrected twice.** `74` (number-system concept, section 7) corrects one
member file's attribution of 952 to signed saturating **multiplication**; the true owner is
addition, measured directly at 4096 triples. `106` (strategy axis) cites the corrected figure
(952, addition) as a load-bearing count in its own rationalisability work and separately notes,
in its own section 6.3, that this exact number is "the whole 72-against-9 gap" reducing to a single
tie between two arms. No contradiction: this is one number, correctly re-derived and re-used across
two topics after being wrong in between, and it is worth flagging because a reader encountering it
in only one consolidation would not see that it was ever wrong.

## 7. Contradictions between consolidations

None found among the four consolidations' own candidate canon text or stated findings. The closest
thing to a contradiction is internal to one topic rather than between topics: `106` section 4's
claim that "component two ranges over the arms that produce the answer the first component fixed"
directly conflicts with `106` section 8's own finding that a fidelity coordinate is entirely
absent from the corpus, which `106` section 18 later resolves by repairing the first claim rather
than the second. This is recorded in section 5.3 above rather than here, because it is a
consolidation correcting itself under its own later sections, not two consolidations disagreeing.

Two topics' independent measurements of related-but-distinct quantities are close enough in shape
to invite conflation and are worth distinguishing explicitly: `63`'s signed-saturating
**multiplication associativity** failure counts (28, 160, 780, 3516 triples at widths 3 through 6)
and `106`'s signed-saturating **distributivity** failure counts (47.72% at width 7, 34.52% at width
6) are two different laws, measured by two different instrument sets, at two different points in
the panel's timeline. Neither supersedes the other and neither was checked against the other by
either consolidation, because they are not the same claim.

## 8. Coverage, stated honestly

I read the four consolidations end to end, including `106`'s own later repair sections (16
through 18), and all seventeen `_op_` files in this directory plus `INTENTS.md` in full. I did not
open any numbered member file, any probe directory, `OPTIONS.md`, `DROPLIST.md`, `RULES.md`, or
`00_brief.md`, per the brief that dispatched this file, so every claim above about what a member
file measured, argued, or conceded is inherited from its consolidation's account of it and carries
whatever error that account carries. Where a consolidation itself flagged uncertainty about its own
compression (each of the four does, in a dedicated closing section), I have carried that flag
forward rather than resolving it.

I could not independently verify any rung classification against the underlying probes or member
files; every "TWO EXPERTS," "ONE EXPERT," or "independent instrument" characterisation above is the
consolidation's own stated rung, not a rung I derived. Where a consolidation's own later material
(`106`'s sections 16 through 18) corrected an earlier claim in the same file, I represented the
corrected version and noted the correction; I did not attempt to adjudicate between `106` and the
`107`/`108` material I was told not to open beyond what `106` itself quotes from them.

I did not attempt to resolve, rank, or add analysis to any disagreement listed in section 3 through
7. Every disagreement above is stated because a consolidation stated it as open, not because I
judged it open.

---

## 9. The realisation map (topic six, files `114` through `124`)

Appended after sections 2 through 8 were written, so the numbering is chronological rather than
topical. This section postdates section 6's cross-topic pass, which therefore does not cover it.

**This is a pointer, not a compression.** The ledger for this topic already exists, was written by a
member who took part in the argument rather than by a harvester, and has been through an independent
check. Restating it here would be a fresh compression with no checker, which is the failure
`a-compression-is-checked-by-someone-else.md` names. So:

- **The authoritative ledger is `122`'s**, which supersedes `119`'s clause by clause and says at its top
  which clauses of `119` stand. `119` is kept as landed because both partial signatures cite it by line.
- **`123` is the independent entailment check** on `122`. It found no severe defect, reproduced the
  anchor diff independently, and confirmed the withdrawn `F118-5` has no orphaned dependents.
- **`124` closes the one item `123` left open**: `F118-8` does not reach the confounded ambient range,
  because `sweep_arms` never calls the two functions that carry it.

**What the topic settled, at the rungs its own ledger assigns.** The overflow policy selects which
licence family is available, and this is structural rather than incidental: no realisation map onto a
finite value set is both a ring homomorphism and monotone except a constant one, since a finite additive
group has no infinite ascending chain. The discharge check and its condition set both follow the map's
character, so the design is arms with disjoint const predicates rather than one rule.

**What it corrected in itself, which is the part worth carrying forward.** Two predicates were wrong
rather than merely narrow: one admitted a counterexample, and one contradicted a shipped test green in
all fifteen gate counts. A finding was withdrawn after its author found its own control unmatched. A
ledger entry was revised downward by the member it flattered. **The domain's closure under negation, and
the ambient span, were missing from every predicate in the sitting**, which is a class rather than two
instances, and nine of eleven predicates were amended for it.

**What it did not settle**, and a later unit inherits: no transfer argument to real widths for anything
except the finiteness theorem, which needs none; nothing at non-uniform value sets; whether consumer
terms are trees or DAGs; clauses 4.8 and 4.9 not re-swept for the domain dimension, disclosed in
`122`'s own body text; and the one result with a consumer attached, the accumulator width collapse under
the shipped guard, which is unpriced and which `117` explains cannot currently be priced honestly.

---

## 10. The rounding axis (topic seven, files `125` through `138`)

A pointer, on the same reasoning as section 9: the ledger exists, was written by a member who took part
in the argument, and has been through three signatures and an independent check. Re-compressing it here
would be a fresh compression with no checker.

- **The authoritative ledger is `136`'s**, restored by `138`, which lists the lines it supersedes. `132`
  is kept as landed because the three signatures cite it by line.
- **`137` is the independent check.** Its verdict is that the compression is sound and that every
  correction `136` made to `132` is right. What it found wrong was one construction and four bookkeeping
  defects, all repaired in `138` after being reproduced at source rather than accepted.

**What the topic settled.** Rounding is not a second overflow axis: the homomorphic class is empty for
deterministic modes off-grid, unconditionally and at every domain including one-signed, and monotonicity
is free for all of them, so the property pair that decides character for the overflow policy degenerates
here. Rounding has its own obstruction, from divisibility rather than from closure under negation, which
is why it has no one-signed refuge.

**The fork is not where either cold derivation put it.** Both placed it at the deterministic-stochastic
boundary; that was refuted by construction. It is one axis, correlation in the coupling against
per-realisation monotonicity, admitting two independent keyings, value and position, which coincide only
where value and position move together. Within a cell the realisation-monotone unbiased law is unique.
Across cells the coupling is the whole design space, and it is priced: summed-error variance `n²f(1-f)`
comonotone against `nf(1-f)` independent.

**And the entropy constraint decides the arms.** I14's no-platform-dependency clause, not I15, means arvo
cannot source a draw and a const seed makes a member deterministic. The position-keyed dither escapes the
disjunction entirely, and it compiles `#![no_std]` with its findings as `const assert!` items.

**What it did not settle.** The double-rounding mechanism, open after three wrong constructions, with
`122` 4.6 shown not to be at risk and staged-versus-direct narrowing shown to be what is; the four
predicates carrying no domain dimension, named as obligations rather than filled; and whether the
position-keyed arm's guarantees survive a keying axis that is not one-dimensional, which is unmeasured.

**Two provenance notes worth carrying.** The blind-convergence union is **six**, reconciled rather than
merged, and the entry a merge would have lost is the topic's headline answer. And two figures in this
topic came from report messages rather than artifacts, both relayed by the coordinator; the committed
work was correct in both cases and both were caught by members reading the files.

---

## 11. The strategy object (topic eight, files `139` through `152`)

A pointer, on the same reasoning as sections 9 and 10.

- **The authoritative ledger is `151`'s**, which supersedes `146` clause by clause and says which stand.
  `146` is kept as landed because four signatures cite it by line.
- **`152` is the independent check.** Verdict: sound. It derived the topic's central counterexample by
  hand from the rounding-mode definitions **before opening any probe**, then reran the probe and got
  byte-identical output, which is independent arrival rather than confirmation.

**What the topic settled.** A strategy is a two-component object and both halves were measured for the
first time. The count question has no answer of the kind it was asked in: `shape -> count` is a
well-defined function, monotone in the observation set, and it saturates, so the axis-only property is
visibility under the maximal observation set. That set is the operation set the design ships, so **the
table waits on a decision rather than on evidence**, which is a question for op.

The storage-minimising concern is a **weighting with zero policy content**, reached blind by both cold
derivations, then rescoped: it has policy content at the accumulator in exactly one cell, signed
saturating, confirmed on three instruments.

On the weighting half, measured last and changing the picture: the Pareto claim strengthens to an exact
rational certificate over the whole simplex; the portability worry **inverts**, because a weighting
travels by construction while an arm does not; and the single corpus instance offered for the whole
design was **withdrawn by its own author as noise** once the noise gate was made pairwise, leaving zero
established instances corpus-wide.

**What it corrected in itself.** A clause in the first candidate was **false**, found independently by
two signers and reproduced on a third instrument in a third language: the unsigned half of the fusion
arm was claimed for all six rounding positions on a closure argument covering only the reduction
relocation, and nearest-half-even, the IEEE default, fails there. The repair is one clause, equivariance
on the domain the cell reaches, five modes against one under unsigned.

**Two provenance findings that reach beyond this topic.** The blindness evidence is weaker than it
looks: the commit ordering runs the wrong way for one member, and **both cold derivations read the same
auto-loaded workspace rules, one of which states a mechanism they both used**, so wherever that
mechanism does the work they are one instance. And the intersection instrument intersected dimension
**names** rather than values, so a dimension the instances partition vanished while appearing present.

**What it did not settle.** The firewall proposition ships **unpredicated**, and both signers agree that
is right rather than a gap: a predicate records where a claim was established and the firewall is
imposed, so the notation would invert it. What is predicable is its enforceability condition and its
violation predicate, and the candidate names those. The count's table waits on op. And the equivariance
result reaches the closed rounding topic; two members independently say it does **not** require
reopening, and one adds that a proposed option for that topic would introduce an error into it.

---

## 12. The primitive (topic five, files `109` through `114`, reopened as the ninth unit, `153` through `165`)

A pointer, on the same reasoning as sections 9 through 11.

**This section is why the ninth unit existed.** Topic five opened the question of what a primitive is,
produced two cold derivations, an attack, and an offered statement at `112:904-945`, and then moved to
the refinement half without ever compressing what it had. Every other topic had a section here and this
one did not, which is the same absence stated from the other side. Under `87` the canon is written from
the ledgers read alongside their members, so a topic without one is a topic whose findings are reachable
only by someone who already knows to look.

- **The authoritative ledger is `164`'s**, which supersedes `161` clause by clause and states at its top
  which clauses of `161` still govern. `161` is kept as landed because both signatures cite it by line.
- **`165` is the independent check**, by a member that took no part in the unit. Verdict: sound. It
  rebuilt every load-bearing repair independently rather than reading them, including the offset probe
  bit for bit and the container-premise sweep clause by clause across all thirteen.
- **Two repairs were applied after that check** and are recorded in `164` itself: R17's count, and R18.

**What the topic settled.** A primitive is a value set with one realisation map over a declared
operation set, with identity up to denotation-preserving isomorphism and a law read off it rather than
declared. **Types are the degenerate case of lenses**, which is the synthesis the cold pair could not
reach and which arrived in the reply round: it gives one vocabulary across the whole declared range
without the cost the single-vocabulary option had carried. The discriminator is **sole occupancy of the
carrier allocation**, padding permitted and sharing not, correct in three failure directions.

**Adequacy is settled and was the obligation `111` named as nobody's.** It is two obligations and only
one is hard: soundness is free by functionality, and completeness is a conjunction of inequalities, each
discharged by one witness, so it is checkable at real width and only refutation needs exhaustion. Taken
as first written it rejected every refinement parameter; the repair is **completeness up to weakening**,
a three-outcome per-pair certificate that compiles with the spurious case failing to build.

**What it corrected in itself.** A rung was refused by the member it flattered. The claim that the
container premise was localised to one clause was wrong three times over, reaching three clauses, then
four, then two further subordinate phrases that only a systematic sweep found. A do-not-cite entry
carried a wrong figure through seven files. And the six-instance class is now recorded: a criterion
tested against an instrument too thin to reach the case that breaks it, twice in consecutive sections of
one file.

**What it did not settle**, and it blocks the statement rather than sitting beside it: **the container
premise is op's**, and one clause is refused as written because no wording is true on both branches
until he rules. X3 is relocated into that queue rather than resolved, because a question deciding
whether a clause can be satisfied at all is not a cross-reference. Q65's marker question is his too.
Every magnitude in both sittings remains **unpriced**.

---

## 13. The chain and the composite (topic ten, files `166` through `178`)

A pointer, on the same reasoning as sections 9 through 12.

- **The authoritative ledger is `176`'s**, superseded in seven places by `178`, which lists exactly what
  it supersedes. `173` is kept as landed because both signatures cite it by line.
- **`177` is the independent check**, by a member that took no part in the unit. Verdict: sound.
- **`178` is the restoration pass**, each repair restored from its establishing source.

**What the topic settled.** The unit is delimited by **observation rather than by the operator**: a
maximal stretch whose intermediates are not observable, with a single operation as the length-one case.
Composition owes obligations no per-operation surface can answer. There are **two independent licences**
for deleting an interior resolution, range and algebra, neither subsuming the other, and the algebraic
one is a **conjunction over every step**. Deferral is **pointwise optimal** wherever the boundary
resolution is a nearest-point projection.

**The result that shapes the canon rather than the design.** The unit's central claim splits by
provenance and the candidate marks every sentence with its kind. **(P), the partition, is a theorem**,
derivable without the observability rule, by contextual equivalence plus I14's ban on `dyn`, `TypeId`
and `core::any`, with its one empirical premise measured. **(L), the licence, is normative and
underivable**: three members tried to derive it from op's stated intents and all three failed for stated
reasons. It has two exact bounds, a build-profile bound that **converges with I18 from a direction I18
was never derived from**, and the definedness bound, which quantifies it over the boundary function
*with its definedness domain*.

**Two sweeps became theorems**: the deferral optimum, whose proof makes the tie rule irrelevant and
idempotence a consequence, and the no-threshold double-rounding claim, constructive at `F any`.

**What it corrected in itself.** A rung claim was made, over-generalised to three files, and cut back to
two by measurement. An argument was marked as a measurement, on `60`'s own central result, with `60`'s
disclaimer carried nowhere. A clause was refused on a real contradiction and repaired by composition
rather than patch. A biconditional was claimed and one direction fails. **Nine instrument defects**, two
naming new classes: **scope rather than mechanism**, where no control can catch it because a control
tests whether an instrument measures what it points at and not whether it points at the whole claim; and
**the harness rather than the instrument**. Members refuted **eleven of their own hypotheses** and kept
every run.

**A coordinator failure is recorded at `166` section 6**: the curated reading list omitted `60` and `43`,
both on this unit's question, and the compression that had preceded it dropped exactly the material that
would have made the omission survivable. The two failures were not independent.

**What it did not settle**, and all of it is op's: whether the observability principle becomes an arvo
intent, which is what (L) rests on and a workspace rule is not a ratification; which accuracy target I7
names; which chain carrier ships; two vocabulary calls; and the canon-form question, coupled to `156`
item 2 as one decision. **Every magnitude is unpriced** except what `OPTIONS.md` Q42 already carries.
