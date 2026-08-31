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

## 9. Reconciliation

Appended after the phase-one commit; see the commit ordering on this branch for the record that everything above was written blind.
