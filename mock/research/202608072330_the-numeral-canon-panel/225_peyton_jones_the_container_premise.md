# 225. Peyton Jones, seat 225: the container premise, derived cold

Phase one of a blind derivation. Everything above the reconciliation heading was
written before reading any prior work on this question, per the brief. The
reconciliation at the end says what was read afterwards and what moved.

**Gates.** The canon gate passed: the assigned work is the question the canon
itself carries open (`question::the_container_premise`, `decider = panel`,
bounded by op and explicitly the panel's under
`ruling::the_panel_finishes_the_canon_without_him`), so deriving it is licensed
work rather than drift. The test gate passed: the full `mock/` suite runs green,
152 tests across 19 binaries, and I read the bodies of the checks governing the
surface this file touches (`no_prose_cites_a_living_ledger_by_line`,
`a_probe_reads_the_tree_it_sits_in`, `a_panel_catalogue_is_readable`); they are
substantive, carry planted-fixture negative arms, and their own doc comments
record assertions being narrowed onto properties rather than spellings. Two of
their constraints bind this file: no line citations into living ledgers or the
registry, and no home-anchored paths in probes. Both are observed.

**What phase one read**, exactly: `mock/registry/topic.toml`,
`dimension.toml`, `ruling.toml`, `question.toml`, `obligation.toml`,
`mockspace.toml`, and the repository's generated `.claude/rules/`. Not read,
beyond the brief's own prohibitions: `proposal.toml`,
`proposal-the-later-topics.toml`, `law.toml`, `law-the-later-topics.toml`,
`probe.toml`, `retirement.toml`, `strategy.toml`, anything numbered under the
panel directory, and `git log`. The brief's allowed list read as exhaustive, so
I treated it that way; the registry files outside it stay closed until the
reconciliation.

**Two known leaks into the blindness, reported rather than smoothed over.**
First, the brief itself names the four forbidden proposal slugs, so I know
their titles; I have not opened them, and everything below is derived without
them, but a title is not nothing and the reconciliation checks whether the
titles steered me. Second, the `note` on
`ruling::the_observability_licence_is_an_intent_and_he_put_it_to_the_panel`, a
file I was allowed and required to read, states that seat 210 "established what
observability is, that it is induced by a signature rather than carried by a
representation". That is a one-sentence summary of prior work on an adjacent
question, sitting inside the canon. I flag it because part of my derivation
lands on the same shape; the derivation below reaches it from the machine facts
and the ratified rows, and the probes are mine, but the sentence was in my
inputs and I cannot un-read it.

---

## 0. The question, and the first move

`topic::the_container_premise`: is a declared numeral's behaviour stated over
its declared width, or over the container that carries it?
`question::the_container_premise` sharpens it through footprint: is the
footprint observable, *so that* behaviour is stated over the container?

So, let's see. The first move is to notice that the question welds two
different things with that "so that", and the weld is where it breaks.

- **The denotational half**: which values do the declared operations produce.
  Is the arithmetic a function of the declared width or of the carrier?
- **The observational half**: is the carrier visible at all, through footprint,
  alignment, bit pattern, boundary casts.

The "so that" is an inference: *if* the footprint is observable, *then* a
sound specification must cover it, *then* behaviour is stated over the
container. The inference assumes behaviour is one thing stated once, over one
width, for the whole numeral. The canon, I will argue, has already refused
exactly that shape of statement, and each of the two universal branches is
refuted by one of the two bounds op gave when he returned this question. The
fork does not need a ruling. It needs the quantifier the canon already owns.

## 1. What the canon already forces: the denotational half is closed

`ruling::the_format_spine_is_canon` is ratified, `ratified_by = "both"`, and it
is directly on point. Four propositions, of which two decide this half:

- "A format is identified by its ambient domain and its representable set, and
  that set is a constant of the type." The representable set is fixed by the
  declaration (one affine predicate, quantum and phase). **The container
  appears nowhere in the identity of a format.**
- "Arithmetic on a format is an exact operation in the ambient domain composed
  with a named total adaptation onto that set." The adaptation lands on the
  *representable set*, which is the declared set. **A container-dependent
  arithmetic result is incompatible with this factoring**: if the result of an
  operation depended on the carrier, the operation would not factor through an
  adaptation onto the declared set, and a ratified row would be false.

So for the operation set a format declares, the canon has already picked:
**arithmetic behaviour is stated over the declared width.** That is not my
derivation; it is a consequence of a row ratified by op and by expert
convergence together, and under the provenance ladder it is not reopenable
here. Any downstream clause wording arithmetic over the container contradicts
`ruling::the_format_spine_is_canon` and loses.

Two further rows lean the same way without being decisive alone:

- `ruling::the_imitation_is_ergonomics_not_an_arithmetic_boundary`: put the
  exact fork "declared width or container" for I3's imitation at a non-native
  width, op answered "Neither, it's ergonomics", and the row records that where
  the arithmetic boundaries land is the panel's to answer *from the width and
  the overflow policy*. Declared quantities. He conspicuously did not put the
  container into the inputs of the answer.
- `ruling::hot_may_sacrifice_soundness_for_a_proven_meaningful_gain` (I5,
  ratified) presupposes this baseline structurally, which is the neat part:
  "sacrifice soundness" is only a meaningful phrase if there is a reference
  semantics to deviate from. If behaviour were stated over the container, a
  container-wrap could never be unsound and I5's licence would license
  nothing. The one strategy allowed to leak the carrier into results is
  defined *as a priced deviation from the declared-width denotation*, which
  makes the declared-width denotation the statement and the leak an arm.

And the measured fact, mine, committed at `225_probes/`:

**Probe 1** (`probe1_declared_vs_container_wrap.rs`, output
`probe1_out.txt`): declared-width wrap and container wrap are different
arithmetics almost everywhere. For unsigned multiplication under wrap in a
`u32` carrier, the two disagree at every declared width from 2 to 31
(exhaustive over all operand pairs for `W in 2..=10`, deterministic 40,000-pair
sample per width for `W in 11..=31`), agreeing only at `W = 1` and at
`W = C`. The control arm (`W = C = 32`) reports zero disagreements over
200,000 pairs, and the negative-control arm, which asserts agreement at
`W = 13`, fails as required with witness `a=2, b=4096`: declared-wrap `0`,
container-wrap `8192`.

> holds for: W in 2..=31: swept, exhaustive at W in 2..=10 over all pairs;
> C = 32; signedness = unsigned; operation = mul; arity = 2;
> overflow policy = wrap; F = 0; rounding = exact; threads = 1;
> rustc = 1.98.0-nightly (57d06900f), edition = default

**Probe 2** (`probe2_double_rounding_through_a_wider_intermediate.rs`, output
`probe2_out.txt`) does the same at the rounding axis, because a carrier
holding more fraction bits than the declaration invites rounding onto the
carrier's grid first. Quantising half-up through a wider intermediate diverges
from quantising direct in **every one of the 83 cells** with
`F_dst < F_mid < F_src <= 8`, exhaustively per cell; the exact-intermediate
control reports zero witnesses everywhere; the negative control at
`(4, 3, 2)` fails as required with witness `x = 1/16` (direct `0/4`, via
eighths `1/4`). This is the fixed-point form of the x87 double-rounding
history: carrier-stated rounding is observably a different arithmetic, which
is why IEEE 754 states behaviour over the declared format and why extended
intermediates violated it.

> holds for: F_src in 3..=8: exhaustive; F_mid in 1..F_src; F_dst in 0..F_mid;
> signedness = unsigned; rounding = half_up; operation = quantise; arity = 1;
> threads = 1; rustc = 1.98.0-nightly (57d06900f), edition = default

## 2. Op's own bounds each kill one branch, and that is the whole fork

He returned the question with two constraints: soundness, and
`obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`,
MATLAB `fi`/`fimath` and IEEE 754 named first-class
(`ruling::the_standards_bound_starts_at_two_and_reserves_the_rest`).

**The standards bound refutes the container branch.** MATLAB `fi` semantics
are declared-width semantics by definition: a `fi` object with word length 13
under wrap wraps at 2^13, whatever MATLAB stores it in, and the parity suite
the obligation demands asserts equal answers. Probe 1's arm C is the
counterexample generator: at `W = 13` in a `u16`-or-wider carrier, a
container-stated numeral returns `8192` where `fi` documents `0`, for the very
first witness, and for 67,017,387 of the 2^26 operand pairs. A convention that
cannot be written as an alias is, in the obligation's own words, a gap in the
primitives; a container-stated primitive cannot carry the `fi` alias, so the
container branch fails the adequacy test outright. Probe 2 extends the same
refutation to rounding, where IEEE 754's whole history says carrier-grid
rounding is the bug, not the behaviour.

**The same bound, read to its end, also refutes the pure declared-width
branch**, by which I mean the branch worded as "the realisation is not part of
identity, the carrier is unobservable, footprint is internal". Because IEEE 754
is not only an arithmetic standard: its interchange formats pin the footprint
and the bit positions. A binary32 alias owes `to_bits`/`from_bits` over a
pinned 32-bit occupancy with sign, exponent and fraction at fixed offsets. An
alias over a primitive whose carrier is officially unobservable cannot state
that contract at all. And the canon already carries the same demand twice more,
from the demand side:

- `obligation::an_exact_width_container_a_consumer_can_alias_and_pin`: a
  consumer's 28-bit hash converts at an rkyv boundary as "a single masked
  cast", the cost being part of the need. A masked cast at a serialisation
  boundary is a contract *about the carrier*. Unstateable if the carrier is
  outside behaviour.
- `ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up`:
  everything here ends up in a C ABI, and the consumer defines APIs in arvo's
  shapes. A C ABI is a layout contract. A numeral whose layout is officially
  not part of its behaviour cannot sit in one.

And the machine fact, which no design can legislate away, committed as
**probe 3** (`probe3_footprint_is_signature_relative.rs`, output
`probe3_out.txt`): for any numeral that is a standalone Rust type, the
footprint is const-observable through the ambient layout signature.
`size_of::<Declared13InU16>()` is `2` and `size_of::<Declared13InU32>()` is
`4`, both at const time, and nothing a library writes removes that signature.
Two carriers for one declaration are two observably different types, full
stop. The must-fail companion
(`probe3_must_fail_one_primitive_two_carriers.rs`, stderr committed) is the
strongest form available: the claim "one primitive, two carriers, footprint
inside the contract" is asserted at const time and **does not compile**,
E0080, the const evaluation itself refusing the sentence. Per the workspace's
own signal ordering, that is better evidence than a wrong value: the claim has
no expressible form.

> holds for: W = 13; container in {u16, u32}; rustc = 1.98.0-nightly
> (57d06900f), edition = 2024. Compile-time facts; no threads axis is listed
> because the claims hold only where threads do not exist, which for a
> compile-time fact is exactly right under the omission rule.

So: **soundness plus the standards bound refute the container branch, and the
standards bound plus two standing canon rows plus the machine refute the
unobservable-carrier reading of the declared-width branch.** Each of op's two
constraints, applied concretely, eliminates one option of the binary. The fork
as posed has no true side, and that is a result, not an impasse.

## 3. The statement that survives, and it is already the canon's shape

What remains is the only statement consistent with all of the above, and it is
the arms shape (`ruling::arms_over_regions_are_the_fundamental_heart`,
ratified) applied to the design's own semantics:

**A declared numeral's behaviour is stated per declared signature, not once
over one width.**

- **Arithmetic signatures are stated over the declared width.** Forced by
  `ruling::the_format_spine_is_canon`. No arithmetic observation distinguishes
  two realisations of one declaration, except where an arm takes the I5
  licence, in which case the deviation is itself declared, priced, and gated
  (`S = Hot`, provable meaningful gain), which is a predicate on the arm and
  not a premise of the design.
- **Layout signatures are stated over the container, and exist exactly where
  declared.** A standalone numeral cannot avoid offering the ambient layout
  observation (probe 3, fact 1); a numeral at shared occupancy cannot offer a
  per-element one, because a packed 13-bit element is not a place, and probe
  3's fact 3 shows there is not even a value such an observation could return
  (five 13-bit elements occupy 9 bytes; 9/5 is not whole). "Is the footprint
  observable" is therefore not a property of the numeral. It is a property of
  the signature set the numeral's kind declares, and the design *chooses* that
  set per kind: pinned where a standard or a boundary demands it (binary32
  interchange, `Bits<28>` at rkyv, C ABI positions), absent where the
  realisation owns the space (Cold's dense stream, ratified as its intent in
  `ruling::cold_is_for_cold_paths_and_cold_storage`).
- **Between observations, the carrier is free.** This is already canon at the
  `stated` rung:
  `ruling::the_observability_licence_is_an_intent_and_he_put_it_to_the_panel`,
  everything inside an unobserved stretch is arvo's to choose. That licence is
  only coherent under the per-signature statement; under either universal
  branch it is respectively redundant or false. The canon has, in other words,
  already been paying for this framing in one topic while this topic held it
  open.

One more strut, cheap but load-bearing under `I14`/monomorphisation
(`ruling::the_operating_constraints_are_intents_and_rules`,
`ruling::never_a_runtime_check_and_one_lowered_path`): **the container is not
an independent quantity.** With sizes const at the type level and
monomorphisation as the only dispatch, the carrier is a const function of the
declaration and the strategy, chosen by the realisation map (the prior design
even spelled it as an associated type, `S::Container<I, F>`; dead tier, cited
as evidence of why the topic arose and nothing more). So "behaviour stated
over the container" was never a different function space, since the container
is itself derived from declared quantities. It is a *weaker contract over the
same functions*: it promises less about the same monomorphisations. The fork
was never denotational; it was always about what the contract quantifies over,
and contracts here quantify over declared signatures.

## 4. What this does to the clauses the topic says are blocked

The blocking claim was: no wording of several downstream clauses is true on
both branches. The resolution is not a wording true on both branches; it is
that the branches were the wrong quantifier, and the clauses become writable
once they quantify per signature.

- **"The realisation is not part of identity"** is false as written and also
  not to be inverted. The correct clause: *the realisation enters identity
  exactly through the layout signatures the numeral's kind declares.* Where a
  layout signature is declared (standalone repr, pinned interchange, ABI
  position), two realisations that differ under it are two primitives, and
  probe 3's must-fail shows the contrary claim cannot even be compiled. Where
  none is declared (shared occupancy), realisations differing only in layout
  are one primitive, and the licence clause makes the interior free.
- **"Identity saturates at the literal"** becomes: *identity saturates at the
  declared signature set.* Two declared numerals are the same primitive iff no
  declared signature separates them. The question row's measured aside, that
  admitting a container observation splits every class 32 to 64, is exactly
  what this predicts: adding a signature refines the quotient. The split is
  earned where the signature is declared and spurious where it is not, which
  turns the count from an alarming fact into a table of which kinds declare
  the observation.
- **"Two markers over one value set and one realisation map must not be two
  types"** becomes a per-region check instead of a premise: the split is
  licensed iff some declared signature separates the markers. `I9`
  (`ruling::the_strategy_is_what_makes_an_answer_correct`) supplies the
  separating signature wherever two strategies' weightings actually select
  arms with different results; where two markers provably never separate under
  any declared signature, the split is spurious *there*, and saying so is an
  arm-level finding with a predicate, not a canon fork.

## 5. The question's own shape was the anti-pattern

Said plainly, since the brief asks whether the question is malformed: **as a
binary over the whole design, yes.** "Is behaviour stated over the declared
width or the container" asks which single policy governs a whole category,
which is the exact shape op has refused three times in one sitting
(`ruling::there_is_no_universal_answer_take_the_win_and_gate_it`), and the
workspace rule about it warns that a registered binary carries its shape into
whatever consolidates it. The topic itself is real and was right to block:
the downstream clauses genuinely could not be worded, and the panel was right
not to write them until the quantifier existed. What was malformed is the
option list, whose first option is one universal branch, whose third
contradicts a shipped rule, and whose second is filed by the row itself as a
pointer elsewhere. None of the three is the answer; the answer is the
quantifier.

## 6. Findings, each with its predicate

1. **Arithmetic behaviour is stated over the declared width** (imposed;
   derived from `ruling::the_format_spine_is_canon`, ratified). Holds
   wherever the format spine holds, which is the format concept itself; the
   carve-out is an arm taking the I5 licence, `S = Hot`, priced and provable,
   per `ruling::hot_may_sacrifice_soundness_for_a_proven_meaningful_gain`.
2. **Declared-width wrap and container wrap are different arithmetics**
   (measured, probe 1). holds for: W in 2..=31, C = 32,
   signedness = unsigned, operation = mul, arity = 2, overflow policy = wrap,
   F = 0, rounding = exact, threads = 1, rustc = 1.98.0-nightly (57d06900f).
   Exhaustive at W in 2..=10; sampled at 40,000 pairs per width above. They
   agree at W = 1 and at W = C, both measured.
3. **Rounding through a wider intermediate diverges from direct rounding**
   (measured, probe 2). holds for: F_src in 3..=8 exhaustive,
   F_dst < F_mid < F_src, signedness = unsigned, rounding = half_up,
   operation = quantise, arity = 1, threads = 1, same toolchain. All 83
   cells carry witnesses; the exact-intermediate control carries none.
4. **A standalone type's footprint is const-observable and two carriers are
   two types** (measured at compile time, probe 3 and its must-fail
   companion). holds for: W = 13, container in {u16, u32},
   rustc = 1.98.0-nightly (57d06900f), edition = 2024. No threads axis: the
   claim holds only where threads do not exist, which is what a compile-time
   claim means under the omission rule.
5. **At shared occupancy no per-element footprint value exists** (probe 3,
   fact 3, arithmetic). holds for: W = 13, elements = 5,
   container = dense bit stream. The general divisibility statement (any W, k
   with 8 not dividing kW) is arithmetic and I state it as argued, not swept.
6. **The fork dissolves into per-signature statement** (argued; sections 2
   and 3). Its ground is findings 1 to 5 plus the cited ratified rows and
   obligations. It is a claim about what the canon forces, not a measurement,
   and it is the part of this file that most wants a second independent read.

## 7. What would settle what remains

The canon question closes with the quantifier; what remains is not this
topic's to hold open:

- **Which layout signatures each numeral kind declares** is a projection
  choice over a fixed semantics, exactly the shape the register already
  records for the width-coordinate question (its answer moved the surface
  choice to the design tier). The bound on it is not free choice: a kind
  standing at a standards alias or an ABI/serialisation boundary must declare
  the pinned layout signature (the obligations force that much), and a
  shared-occupancy kind cannot declare a per-element one (probe 3). Between
  those walls it is design, not canon.
- **The spurious-split check per marker pair** is a measurement: for each
  pair of markers over one value set and one realisation map, does any
  declared signature separate them anywhere? That is a sweep with a predicate
  per pair, and it retires the third option of the question row cell by cell
  rather than by ruling.

## 8. Reported outside the question, as the brief requires

- `.claude/rules/implementation.md` in this repository presents the dead
  design tier in binding present tense to every agent that loads it: a
  concrete `UFixed<const I, const F, S>` with `S::Container<I, F>`, a
  four-strategy container table, and a MUST clause ("it MUST carry
  `S: Strategy`"), while the canon these rows descend from is being rewritten
  and the chain rule holds the designs dead for canon purposes. That is the
  reattachment hazard the chain rule warns about, wearing the authority of a
  generated rule file. During a canon panel that file is a standing invitation
  to reason from a dead tier, and it cost nothing this time only because the
  table happens to *illustrate* the carrier-is-derived point rather than
  decide anything. It should not be able to cost anything next time either:
  the template it generates from should either carry a "prior design, dead for
  canon work" banner or not render while the canon arc is open.
- `question::the_container_premise`'s option list is a category binary of the
  shape `ruling::there_is_no_universal_answer_take_the_win_and_gate_it`
  refuses, recorded rather than voiced, which the workspace's own rule says is
  the worse form because consolidations inherit it. Whoever ports this
  question's closure should close the *shape*, not pick a letter.

---

## 9. Reconciliation, appended after the phase-one commit

Everything above this heading was committed at `0b8e969c` and pushed before any
of the following was opened. Nothing above it has been edited since; the branch
history is the record. What phase two read, in order: the four proposal rows
named in the brief, all seat 210's; the two `r210_*` retirement rows; the panel
files `210_dolan_the_container_premise_is_a_theorem_over_a_signature.md`,
`221_dolan_the_numeric_fundamentals.md` (its container-premise section),
`222_kiselyov_the_numeric_fundamentals.md` (its 5.3 and section 11),
`223_checkpoint_the_topic_layering.md`; the `probe` rows 210's proposals cite,
whose committed artifacts I confirmed exist under `210_probes/`; and
`git log` on this branch's ancestry.

**The blindness, honestly restated.** Beyond the two leaks declared up front,
nothing reached me before the commit. The four slugs in the brief are, read
together, close to a table of contents for my sections 1 to 3, so the honest
claim is: independent in route and instruments, signposted in destination. The
route is where the independence lives, and my route was built from the ratified
rows plus probes 1 to 3 before any prior file was opened.

### Against the four proposals, one at a time

- `observability_is_relative_to_a_declared_signature` (seat 210, definition,
  one expert). My section 3's quantifier is the same definition reached from
  the other end: 210 derives it from the candidate's own clauses 2 and 4, which
  I never read; I derived it from the machine facts and the ratified spine.
  Convergent, and by different instruments. What mine adds: the choice of
  signature set is not free. The standards bound and the obligations force
  layout signatures to be *declared* for some kinds (interchange, ABI, the
  masked-cast boundary), and shared occupancy forbids a per-element one, so the
  schema's parameter ranges over a constrained set with two hard walls.
- `every_operation_arvo_declares_is_a_function_of_the_declared_width` (seat
  210, measured, one expert). My finding 1 and probe 1 are the second
  independent instance. Per the intersection discipline: the two instruments
  overlap at `W = 13, F = 0, signedness = unsigned, operation = mul,
  overflow policy = wrap, container = u16/u32-class, threads = 1`, and the
  convergence claim is over that region. Each instance extends it alone
  elsewhere: 210's carries `operation in {encode, add, mul, xor}`,
  `overflow policy in {wrap, sat}`, `chain length in {1, 4}` and the twelve
  standards widths; mine carries `W in 2..=31` for the wrap-mul divergence,
  exhaustive at `W in 2..=10`. Cite 210 for the operation and policy axes and
  mine for the width span; neither alone carries the union. And 210's own note
  flags that nothing there reaches a fraction width above zero: **probe 2 is
  the first instrument on this question that varies fraction grids**, and it
  establishes the projection obligation at the rounding axis: an intermediate
  on a finer grid, unprojected, is arithmetically observable, witnesses in all
  83 cells with `F_dst < F_mid < F_src <= 8` under half-up. That is the
  fractional edge of 210's "where the projection is omitted the carrier
  becomes arithmetically observable", measured rather than inherited.
- `the_carrier_is_observable_through_the_ambient_layout_observation_alone`
  (seat 210, measured, one expert). Probe 3's facts 1 and 2 are the second
  instance of the layout half, same shape, different types, and the must-fail
  companion adds the direction 210's part A does not carry: the contrary
  claim, one primitive with two carriers and footprint in the contract, is not
  merely false but **uncompilable**, E0080 at const evaluation, stderr
  committed. 210 shows the separation compiles; mine shows the anti-claim
  cannot.
- `at_shared_occupancy_no_per_element_footprint_observation_exists` (seat 210,
  argument, one expert, its own note saying "a probe that builds the shared
  placement and shows no per-element size observation exists would raise it",
  and 222 section 11 repeating that the probe does not exist). **It exists
  now: probe 4, built in this phase for exactly that gap.** A packed column of
  five 13-bit elements, 9 bytes, elements 1 through 4 straddling byte
  boundaries, roundtripping exactly (the control that the placement is real),
  and the negative arm showing the only reachable size observation returns 16
  bits, the extraction target, never the 13-bit placement. So the row's
  argument-tier claim now has an instrument, which under its own note raises
  it, and the second instance is not the same persona.

### The one live disagreement in the corpus, and where I land

222's summary table carries the container premise as "observable, and
observable at const time", and its section 11 opens a fork: declare an
occupancy dimension row, or rest on the const `size_of` read. Probe 4 bears on
it directly: **the const read exists for the column and not for the element**,
so at shared occupancy a per-element predicate cannot be built from `size_of`
at all. The const-availability result (222's 5.3, real and useful) holds at
sole occupancy only, and its table line is unpredicated on exactly the axis
that moves it. Whichever route the panel picks, the sole-against-shared
condition has to be carried somewhere explicit; the const read alone does not
carry it. That is evidence for the occupancy-axis route, or for the condition
staying in the sentences as 210 chose, and against reading 5.3 as
occupancy-free. Declaring the axis is a two-reader call per the vocabulary
rule and I am one reader; I flag rather than declare, and note that
`alignment` does not substitute (probe 4's elements are straddling, and the
distinction it needed was occupancy, not alignment).

### What the prior work has that mine lacked, credited

The clause-level surgery is 210's and it is the better half of the joint
result: clause 6 repaired by one word ("denotational"), clause 9's witness
quantifier widened to admit nullary observations with the spurious-pair
control surviving, clause 4's conditional discharged, and the circular
dependency between this question and the operation-set question located and
retired (`r210_the_container_premise_is_upstream_of_the_operation_set_question`).
I never saw the candidate's clauses and could not have done any of that. The
32-to-64 dissolution is also 210's in full; my phase-one section 4 predicted
its shape ("adding a signature refines the quotient") without knowing it was
already established. And 222's const-availability observation, that `size_of`
being const makes the footprint a gateable axis under
`ruling::the_predicate_is_whatever_is_available_at_const_time`, is a
consequence I did not draw.

### What mine adds beyond the prior work, so the next seat starts from the list

1. **The refutation symmetry** (section 2): each of op's two returned bounds
   kills exactly one universal branch, so the fork was closed by its own bound
   the day he returned it. 210 reaches both halves; the symmetry as a
   one-sentence closure of the question row's option list is new, and it is
   the sentence I would put in the consolidation.
2. **Probe 2, the fraction-grid instrument**, the first on this question at
   `F > 0` in the quantisation sense, closing the gap 210's own note names.
3. **The must-fail compile form** of the layout half.
4. **Probe 4, the packed-column instrument** both prior files name as missing.
5. **The I5 strut**: Hot's licence to sacrifice soundness presupposes the
   declared-width denotation as the reference semantics, so the strategy
   axis's own ratified wording already commits the design to the
   declared-width statement of value observations. Not in 210, 221 or 222.
6. **The carrier-is-derived strut**: under const sizes and monomorphisation
   the carrier is a const function of declaration and strategy, so the
   container branch was never a different function space, only a weaker
   contract over the same one. Also not in the prior files.

### Standing, stated for the coordinator's gate

Four claims now hold two independent instances each, with the second instance
not seat 210's persona, which is what 221 said was owed: the signature-relative
definition, the declared-width statement of declared value and encoding
observations (convergent over the named intersection, extended per-axis by the
instance that varied that axis), the ambient-layout observability at sole
occupancy, and the shared-occupancy absence of a per-element footprint (now
instrumented from both sides). The independence caveat is the signposting
stated at the top of this section, and the coordinator should weigh it; the
committed route evidence is what makes the caveat bounded rather than fatal.

---

# Second dispatch: the two questions standing on the settled floor

Appended on the same branch after merging `origin/research/canon-registry`,
which now carries `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`
(ratified, experts), both fundamentals seats and the lint pack that replaced
`mock/checks`. **Gates.** Canon gate: both assigned questions are open canon
rows, `decider = "panel"`, so deriving them is the licensed work. Test gate:
the checks crate is gone by design (its arms are lints under `mock/lints/` and
tools under `mock/tools/`, gating at commit); the five tools' suites run green,
84 tests, and the lint pack ran on every commit of this branch. What this
dispatch read: the two question rows; every `proposal` row on
`the_realisation_map` and `the_strategy_object`; the `r210` retirements; the
register's "The derivation's outputs" section; `156` items 1 and 2; `146`
sections 5.4, 6.1 and 8; `151` section 10; `150` F150-7; `162`'s and `47`'s
claims as carried by the rows citing them; and the canon-form rows. Per the
brief, I did not go looking for the parallel seat and have read nothing of it.

## 10. `question::container_derivation_output_count`

**The count is not a canon constant, and the canon already carries the thing
that is: the criterion. Applied under the dissolution, the criterion makes the
count a computed consequence per declaration, and it comes out 1, 2 or 3
strategy-owned facts by region.** The three options, one output, two outputs,
one richer output, all quantify over the whole category, which is the shape op
has refused three times; this row is the third registered instance of it on
this panel and it dissolves the same way the premise did.

### 10.1 The criterion is settled and it decides every candidate fact

`proposal::a_fact_is_carried_when_producing_it_applies_a_rule_the_strategy_owns`
stands at two experts, flanked by the site clause
(`a_lowering_site_holds_the_numerals_full_type`) and the sort clause
(`a_carried_fact_takes_the_sort_its_consuming_site_uses_it_in`). Run every
candidate fact the corpus has named through it:

| fact | strategy-owned to produce? | verdict |
|---|---|---|
| carrier | yes: the rung rule is the strategy's | **carried, as a type** (generic bodies need the type; the value-valued spelling is compiled-refused six times, `47`) |
| stride at sole occupancy | no: `size_of(carrier)`, a language primitive over a fact already carried | **recomputed** |
| placement at shared occupancy | yes: the packing rule is the strategy's, and the extent is not recoverable from the carrier | **carried** |
| load type for one packed element | no: `floor((W+6)/8)+1`, a pure function of the declaration | **recomputed** (the corpus already says it is "neither of the two outputs") |
| alignment | no: `align_of(carrier)` | **recomputed** (the corpus: alignment "rides on the carrier") |
| compute carrier for a chain | yes, exactly where the strategy's declared semantics reaches chains | **carried there** (I7's Precise; the pigeonhole result makes the widening forced by semantics) |

The occupancy split in the second and third rows is measured rather than
argued: probe 4 already showed the packed extent (9 bytes for five 13-bit
elements) is not `k * size_of(carrier)` (10), and probe 5 arm B counts 80 of
128 cells over `W in 1..=16, k in 1..=8` where the shared extent differs from
`k` carriers, with arm D the must-fail case at the exact cell. Both prior
files' evidence fits the split: the wide-rung case (payload 25, stride 32 at
`W = 200`) is sole occupancy with stride equal to the carrier's size, and the
general keying claim both files carry, that stride follows the carrier's size,
is exactly the sole-occupancy half. **So the per-aggregate answer is an
independent fact precisely at shared occupancy, which is the same boundary the
ratified dissolution drew for the footprint observation.** The derivation's
output set inherits the occupancy arm structure of the floor it stands on, and
that is not a coincidence: a fact is independently carried exactly where a
signature class has a strategy-owned decision in it, and the classes are the
ruling's classes (value, aggregate, chain).

> holds for the measured half: W in 1..=16, k in 1..=8, container = dense bit
> stream against minimum rung, signedness = unsigned, threads = 1, toolchain
> in `225_probes/toolchain.txt`. The classification itself is the criterion
> applied, argued, with each row's ground cited above.

### 10.2 The arity was packaging, and packaging is not canon

`proposal::an_output_of_a_derivation_is_a_fact_a_downstream_site_cannot_recover`
has this right and the compiled evidence behind it: any product is one thing,
one richer type-valued output with named projections is the pair wearing one
name, and the value-valued spelling is refused at the forbidden-feature wall
while the type-valued one compiles gate-free. Under
`the-canon-is-intent-not-implementation` the canon states which facts are
carried and in which sort each is reachable; how many associated items spell
that is design tier. **What the canon owes is one sentence: every carried fact
is reachable as a type from the declaration without forbidden features**, and
`47`'s committed compilations establish that as doable, which is the only
doability the intent needs.

### 10.3 The standing disagreement does not survive the dissolution

The register's fork: is the strategy an upstream selector the ladder never
sees (`10`), or a key of the ladder itself (`15`)? Both compile, and the
register says neither file addresses the other. **Under the ratified identity
clause this is two factorings of one map, and no declared signature separates
them.** Probe 5 makes the shape of that claim exact: a keyed spelling and a
selector-into-blind-ladder spelling of one model rule agree pointwise over all
2048 cells, the negative control with one exceptional cell is caught at its
first witness, so the sweep distinguishes assignments while the factorings are
extensionally one. What is genuinely forced is only the map's domain: it
includes the strategy, because the assignment differs by strategy somewhere
(the packing and padding evidence in both files). What is genuinely free is
the interior: whether a ladder factors through strategy parameters is
unobservable through every declared signature, hence unstatable as canon under
the dissolution. **The residue, what a diagnostic can say, is real and is
design tier**, governed by the usage half of
`ruling::validate_means_all_three_readings`, and a design is free to pick the
spelling whose refusals read better. So the disagreement should be retired as
dissolved rather than argued to a winner: a question about which factoring the
design "is" has no canon content once identity saturates at the declared
signature set.

One bound on that dissolution, stated so it is not overread: it holds for the
statement of the map, not for every family of assignments. A selector
vocabulary that is deliberately small (a crossover point and nothing else)
expresses a strict subfamily of the assignments a full keying expresses, and
legislating such a vocabulary into the canon would be exactly the
generalisation `ruling::there_is_no_exchange_rate_because_there_is_no_generalisation`
rejects. The canon states the domain and the carried facts; it does not state
a parameter vocabulary.

### 10.4 What remains open, and it is small

Whether the chain-class fact is the third projection of one derivation or the
first projection of a chain-level one is packaging again. What is not
packaging: `proposal::a_fact_delivered_as_a_const_a_generic_body_loops_over_costs_the_reduction`
shows delivery form is load-bearing at the consuming site (the W = 18
collapse), so the design owes each carried fact a delivery-form decision, and
the canon owes only the criterion it already has (the sort clause). The
proposal's own gap, "how many facts must be types is at least two and not
settled", resolves under 10.1 to: at least the carrier everywhere, plus the
placement at shared occupancy, plus the compute carrier where the strategy
reaches chains, each as a type where a generic body consumes it. I find
nothing in the canon that forces more, and nothing that permits less.

## 11. `question::which_operation_set_the_design_ships`

**The canon forces the shape of the answer, a named core with the inventory
declared open under an admission contract, and it forces the floor of the
core. What it does not force is membership above the floor, and that residue
has a checkable oracle rather than a judgement in it.**

### 11.1 What the dissolution already closed in this row

Three of the row's own load-bearing clauses are stale, and saying so is part
of the answer:

- **"It also decides whether footprint is observable": dead.** The ratified
  ruling settles footprint observability independently of any operation-set
  choice: the layout observation is the host's, outside every set arvo could
  declare, and every operation arvo declares is a function of the declared
  width and never the carrier. No admissible choice here can make footprint
  observable or unobservable.
- **"Whether the count of primitives is container-relative": dead the same
  way.** Identity saturates at the declared signature set and no declared
  operation reads the carrier, so the count under arvo's maximal declared
  observation set is container-free at every admissible answer. The 32-to-64
  split was the ambient observation's doing, and the ambient observation is
  not arvo's to ship or withhold.
- **The `bound` field still carries, verbatim, the sentence
  `retirement::r210_the_container_premise_is_upstream_of_the_operation_set_question`
  retired** ("It is downstream of `the_container_premise` ..."). A live row
  restating a retired claim is the exact class the lint pack polices, and this
  one appears to sit outside its reach. Reported below.

What survives is the real question: which declared-width operations exist,
which decides which strategy-object axes are visible (F150-7: the intermediate
axis is visible only through the fused operations, at the swept cell).

### 11.2 The shape is forced by three precedents and one theorem

- **Option 1, close the set now, contradicts a stated ruling.** Op reserved
  `sc_fixed`, `ap_fixed` and `ac_fixed` "so it can't close them out"
  (`ruling::the_standards_bound_starts_at_two_and_reserves_the_rest`). Each
  reserved convention carries operations of its own, so a closed set closes
  them out, which is the thing the reservation exists to prevent.
- **Option 2, wholly open, fails the full-design bar needlessly.** Under
  `ruling::the_canon_must_support_a_full_design_and_impl` the canon has to be
  exhaustive enough to design from, and a shape-to-count table unwritable by
  construction leaves the strategy object undesignable. Needlessly, because
  the topic's own measurement makes a core-relative table sound: visibility is
  monotone in the observation set (F150-7, 24 triples, zero violations), so a
  table computed at a named core and predicated `operation in <core>` is never
  falsified by a later admission; admission refines it, splitting cells, and
  the predicate discipline already knows how to say that.
- **Option 3 is the pattern the canon already ratified twice.** The format
  spine's fourth clause, ratified by both: the concept is closed and the
  inventory is open. Op's own standards answer: name two, reserve the rest.
  The strategy set: open on his word. A named operation core with open
  admission is the same sentence at the operation tier, and I can find no row
  pulling the other way.

### 11.3 The floor of the core is derivable, and it discharges F150-7's worry

`obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`
is an adequacy test: every operation the two named standards declare must be
writable over the primitives. So the core contains at least the union of
MATLAB `fi`/`fimath`'s and IEEE 754's declared operation sets, and that union
is read off documents rather than decided. One membership matters structurally:
**IEEE 754 requires the fused multiply-add**, so the fused operation is in the
floor by obligation, and the cell F150-7 warns about, a design shipping the
unfused four and reporting the intermediate axis dead, is unreachable for any
standards-adequate core. The worry the row carries forward is discharged by an
obligation the registry already holds; nobody had put the two rows next to
each other. Beside the standards union, the canon's own rows already quantify
over operations that are therefore in the floor on pain of those rows meaning
nothing: the six rounding modes (a quantise per mode), the declared overflow
policies, the encodings (`bin`, `storedInteger`, interchange read and write),
and the law rows' `add`, `sub`, `mul`.

### 11.4 The open half carries an admission obligation, measured

Openness is not free, and the cost has a precise shape. **Admitting an
operation that separates two assignments previously equal under the core turns
a resolver-free interior choice into an answer-moving one retroactively.**
Probe 6 builds the smallest real instance: signed saturating fixed point at
`W = 4, F = 1`, two assignments differing only in the fused intermediate. Under
the core `{add, sub, mul}` the exact-intermediate behaviour is unreachable
(all nine depth-two composites differ from it somewhere, 760 to 3818 of 4096
triples each), the stepwise behaviour is core-expressible (mul-then-add equals
it on all 4096, the control), so the two assignments are one denotation there;
admit the fused operation and they separate on 760 of 4096 triples, first
witness `(-8, -8, -8)`, exact `7` against stepwise `-1`. So the admission
contract the open half owes is: **an admitted operation that separates
existing assignments forces the separating axis into the declared policy at
admission, or pins a named default**, else one type name carries two
semantics, which is the exact shape
`obligation::a_build_flag_that_changes_float_semantics`' gap already condemns
for the fast-math flag.

> holds for: W = 4, F = 1, signedness = signed, overflow policy = saturate,
> rounding = floor, operation in {add, sub, mul, madd}, arity in {2, 3},
> chain length in {1, 2}, exhaustive over the representable set, threads = 1,
> toolchain in `225_probes/toolchain.txt`. The general monotonicity of
> visibility is F150-7's and is cited, not re-established here.

### 11.5 The firewall coupling resolves, and the firewall gains a parent

`151` says the unpredicated-proposition question and this one are one
decision. The first half is answered
(`ruling::a_thing_that_constrains_the_work_and_cannot_be_designed_away_is_canon`),
so the coupling now runs one way: the firewall enters the canon on op's test as
an imposed sentence with its predicated enforceability condition beside it,
exactly as `146` 6.1 stages it, and its quantifier, the denotation of an
assignment, is read at the declared core and refines monotonically with
admission. Worth saying because nobody has: **the firewall is the cost-axis
instance of the ratified dissolution.** "No cost model may move an answer;
every difference traces to the declared policy" and "every operation arvo
declares is a function of the declared width and never of the carrier" are one
principle, behaviour is a function of declared inputs only, instantiated at
the cost axis and the carrier axis. The ratified ruling is therefore the
parent the firewall lacked, and Q62's unpredicated form is consistent with how
the parent itself is stated: a normative sentence whose measured consequences
carry the predicates.

### 11.6 What the canon does not force, and what would settle it

Membership above the floor: whether division, square root, remainder and the
comparison predicates beyond the standards' lists ship in the core or arrive by
admission. The canon is silent and the silence is not permission; what settles
it is mechanical, an enumeration of the two named standards' operation lists
against the parity-suite mandate, which is a reading of two documents with a
checkable oracle at the end, not a judgement. Until that enumeration is done
the honest canon sentence names the floor as "the union of the two named
standards' declared operations" intensionally, which is exact without the
list, and the list lands as evidence when the parity suite is built.

## 12. Findings, each with its predicate

1. **The output count is a consequence of the settled criterion, per
   declaration class** (argued; section 10.1's table with each row's cited
   ground). Carried facts: the carrier everywhere; the placement exactly at
   shared occupancy; the compute carrier exactly where the strategy's declared
   semantics reaches chains. The options' shared premise, one count for the
   category, is the refused shape.
2. **The shared-occupancy placement is not a function of the carrier**
   (measured, probes 4 and 5 arms B and D). holds for: W in 1..=16,
   k in 1..=8, container = dense bit stream against minimum rung,
   signedness = unsigned, threads = 1. 80 of 128 cells differ; must-fail arm
   at W = 13, k = 5 shows 9 against 10.
3. **Two factorings of one realisation map are separated by no declared
   signature, and the sweep that decides factoring-versus-assignment exists**
   (measured on the model, probe 5 arms A and C). holds for: the model rule,
   S in {hot, warm, cold, precise}, W in 1..=256, signedness in
   {unsigned, signed}, threads = 1. The claim about the real design's two
   spellings is not established here; the instrument shape is, with both
   controls firing. The selector-versus-key disagreement is thereby dissolved
   as canon and survives as design-tier diagnostics.
4. **Admitting the fused operation separates assignments that are one
   denotation under the unfused core** (measured, probe 6). Predicate in
   section 11.4. Consequence: the admission contract of 11.4, argued from the
   measurement plus the fast-math gap's own condemnation of one-name-two-
   semantics.
5. **The operation-set answer's shape is option 3 and its floor is the
   standards union** (argued from ratified and stated rows named in 11.2 and
   11.3; the fused operation's membership in the floor is IEEE 754's own
   requirement read through the alias obligation).

## 13. Reported outside the questions, as the standing instruction requires

- **`question::which_operation_set_the_design_ships` carries, in its `bound`,
  the verbatim sentence `r210` retired as wrong**, and two stale stakes in its
  `unblocks` (the footprint clause and the container-relative count, both now
  settled by the ratified ruling). A live row restating a retired claim is the
  class `mock/lints/a_live_row_restates_a_retired_claim.rs` exists to catch;
  whether the lint's reach excludes `bound` fields or the match is too narrow,
  the row is standing in the state the lint names, and either the row or the
  lint wants fixing.
- **`question::container_derivation_output_count`'s option list is the
  category-binary shape a third time**, after the premise row and the
  operation-set row. Three registered instances in one topic family is a
  pattern for whoever ports closures: the register keeps writing "which single
  answer governs the category" over questions whose canon answer is a
  criterion plus regions.
