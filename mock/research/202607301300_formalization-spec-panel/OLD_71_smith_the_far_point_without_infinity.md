# 71. The far point without infinity: the open cell stress-tested, and closed by making it not a cell

Julius O. Smith III, file 71. I wrote file 24 (the multiplicative half) and file 43 (division), both
about what happens when a result leaves the set it is supposed to land in, which is this dispatch's
question at the boundary of a float numeral that has no infinity to leave toward.

**What I read.** `68_consolidation_seven.md` in full, `70_wronski_the_presets_re_derived.md` in full,
`70b_op_checkpoint_seventeen.md` in full, all three required. One `ls` of the panel directory: files
`00` through `70b`, nothing after `70b`. Targeted reads at the coordinates my derivation needed:
`202607301100_topic.the-formalization-talk.md:1655-1690` for the D71 intent sentence and table, read
directly rather than through a paraphrase; `58_consolidation_five.md` section 1.14 for the grade
mechanism's actual shape (the value-carried grade, the IEEE flag-word convergence), since `63` and
`68` both carry that section as "unchanged" without restating it. I did not re-read my own files 24
and 43; where I use the multiplicative half below I use it as `68:325-333` carries it, which I checked
against that consolidation's own table-diff discipline rather than against my memory of writing it.

**Gates.** Canon gate, reproduced fresh from the repo root: `grep -rln "Adjustment\|Bias\|Numeral"
mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both exit 1, empty.
Test gate: `cargo test --offline --workspace` from `mock/`, summed per binary in this session, 658
passed, 0 failed, 9 ignored, matching the consolidation's own report (`68:64-65`). Toolchain
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, confirmed by running
`rustc --version` in this session.

**What is compiled, what is reasoned, what is external.** Sections 2, 3 and 5 lean on four probes in
`71_probes/`, compiled and run fresh this session, outcomes reproduced verbatim in
`71_probes/OUTCOMES.md`; each claim so grounded is tagged compiled. The E4M3 silicon facts (both
overflow conversion modes existing, which deployments use which) are checked this session against
the OCP OFP8 specification and vendor documentation through secondary reads, tagged physical, with
one primary-source read still owed and named in section 9. Everything else is reasoning from op's
ratified intent statements and the review's settled shapes, tagged as such. I take the instinct
under test from `70b`'s own prose ("clamp to the largest finite representable magnitude",
`70b:34-35`), which is the authority regardless of any option numbering that preceded it.

Per the standing method (`70b:42-50`): no shipped doc comment is cited as design justification
anywhere below, and every claim was checked against the deletion test before it entered.

## 1. The space is not two options, and the third one has silicon behind it

The dispatch hands me two answers: clamp to the largest finite magnitude (op's instinct), or a
well-formedness condition refusing the numeral under `Warm`/`Cold` at declaration. Before stressing
either, the honest enumeration of the whole resolution space for this cell, because the review's own
posture demands it and because one member of the space is deployed in silicon and neither given
option names it:

1. **Clamp to the largest finite magnitude** (op's instinct, `70b:30-38`).
2. **Well-formedness refusal at declaration** (the given alternative).
3. **Overflow to NaN**, available exactly where `Specials = NanOnly`. This is not hypothetical: the
   OCP ecosystem explicitly permits two overflow policies for E4M3 conversion, saturation to the
   maximum finite magnitude and overflow to NaN, and both ship. NVIDIA's conversion instructions
   produce NaN on overflow by default and saturate only under the `satfinite` modifier; the
   saturating mode is what the OFP8 specification names "saturating conversions, in which infinite
   results are converted to the maximum-magnitude finite value of the same sign"; the NaN mode is
   what JAX/TPU deploys, the saturating mode what other implementations deploy. *Physical, secondary
   reads this session, primary read owed (section 9).*
4. **Operation-level refusal**, which is `Precise`'s row and is excluded by the table's own
   construction (every pair differs in at least two cells; more precisely, adopting it would erase
   the load-bearing `OverRange` difference between `Warm` and `Precise`).
5. **Wrap**, `Hot`'s fixed-point boundary, which no float intent supports and which no float
   silicon implements; named only so the enumeration is closed.

Options 4 and 5 are dead on arrival for the stated reasons. The live space is 1, 2 and 3, and the
rest of this file stresses all three.

## 2. The derivation that makes op's instinct not a patch: the far point is the supremum of the ordered representable set

Op's checkpoint states the instinct as a graceful degradation: "without an infinity to reach, the far
point *is* the largest representable magnitude" (`70b:35-36`). Stress-testing it, I find something
stronger than survival: the instinct is the bounded instance of a single rule the review has already
ratified twice without naming it once, and stating the rule closes the cell by making it not a cell.

**The rule.** *The far point of a numeral on the overflow side is the supremum of its representable
values in the total order. `OverRange` under `Warm` and `Cold` resolves to the far point.* Three
instances:

- An IEEE-shaped numeral (`Specials ∈ {InfOnly, IeeeSpecials}`): the ordered representable set
  contains the signed infinities, the supremum on the overflow side is the infinity, and the rule is
  file 70's far-direction reading exactly (`70:277-292`).
- A no-infinity float numeral (`NoSpecials`, `NanOnly`): the ordered set is bounded above, its
  supremum is the largest finite magnitude, it is a member of the set, and the rule is op's clamp.
- A fixed-point numeral: the ordered set is bounded above, and the rule is the **ratified**
  fixed-point `Warm`/`Cold` clamp cell (`70b:8-10`), which file 11 already expressed in exactly this
  shape: "clamping above the range is simply `TowardNegative`" because at a bounded top there is one
  candidate (`11:195-196`, settled shape).

NaN needs no exclusion clause, and this is the detail that makes the rule clean rather than merely
convenient: the supremum is taken over the *ordered* values, and NaN is not in the order. The
`NanOnly`/`NoSpecials` agreement is a theorem of the definition, not a case in it. Compiled: the
projection is total over the whole four-member `Specials` product through one blanket impl, no
`HasInfinity` gate, no refusal anywhere, const-callable, zero feature gates
(`71_probes/probe_1_far_point_total.rs`, all four members asserted at compile time). This is the
structural difference from file 70's probe 1, which expressed the far-direction reading as a gated
bound that refuses two members (`70:294-303`); under the supremum rule the gate was never needed,
because the two "refused" members were never outside the rule, only outside the infinity.

**The boundary comes with the rule, and it answers file 70's flagged-forward tie question as a
corollary.** Where does in-range rounding stop and `OverRange` begin? On the extended grid, the
format's own step pattern continued one hypothetical step past the maximum: values within half a
top-binade ulp of the maximum round to it as ordinary in-range quantisation, and only a value whose
extended-grid rounding lands past the maximum is an `OverRange` event. The tie at exactly half an
ulp resolves by the ordinary even rule, and here the two number families genuinely diverge, for a
reason that is pure bookkeeping until it isn't:

- Every IEEE format's maximum finite has an all-ones significand, odd, so the extended-grid tie
  rounds *up*, off the finite set, and overflow at the tie delivers the infinity. This derives
  IEEE 754's own overflow behaviour (the standard's threshold formula includes the tie) from
  ordinary ties-to-even on the extended grid, with no directional constant, which is the answer to
  the question file 70 left open at `70:498-506`: the standard favours the infinity at the tie, and
  the *mechanism* is parity, not a special case.
- E4M3's maximum finite is `1.110 x 2^8 = 448`, stored mantissa `110`, **even**, because the
  all-ones slot in the top binade is the NaN. So the extended-grid tie at 464 rounds *down*, to 448,
  and everything in `(448, 464]` is ordinary in-range rounding, not an overflow event at all.
  Compiled: `q(464) = 448`, `q(456) = 448`, `q(464.0001)` is the first out-of-range value
  (`71_probes/probe_2_e4m3_model.rs`, assertions, not printout).

One construction, both standards' behaviours derived, and the no-infinity format's boundary falls
out with the correct, slightly surprising parity. I flag the generalisation honestly: the parity
argument is per-format (it reads the maximum finite's stored significand), so it is derived
generally, not sampled, but any future no-infinity format whose NaN encoding sits elsewhere gets
whatever parity its own maximum carries, and the spec sentence in section 9 states the rule, not the
E4M3 outcome.

**The construction is already the review's own.** The extended-grid-one-step-past reading is the
identical shape file 70 used for the far-direction reading at the top (`70:288-292`) and the review
used for `Abrupt`-underflow's hole at the bottom (`68:198-199`). I am reusing a validated
construction at its third site, not inventing a fourth.

*Grounded on: ratified (`70b:8-10`, `70b:30-38`), settled shapes (`11:195-196`, `70:277-292`),
compiled (`71_probes/probe_1`, `probe_2` assertions), reasoned (the supremum statement, the parity
derivation).*

## 3. What each answer does to a real computation, measured, not argued

The dispatch names three computations. I built them, value-exact, over the real E4M3 grid, under all
three live resolutions (`71_probes/probe_2_e4m3_model.rs`, outcomes verbatim in `OUTCOMES.md`).
True values: 448, 208, 1024.

| computation | true | saturate | NaN-mode | refuse |
|---|---:|---:|---|---|
| `(448 + 448) - 448`, the come-back sum | 448 | **0** | NaN | refused at op 1 |
| `(416 * 2) / 4`, product through a division | 208 | 112 | NaN | refused at op 1 |
| sum of sixteen elements of 64, the column fold | 1024 | 448 | NaN | refused at op 8 |

**The come-back sum is saturation's real cost and I will not smooth it over.** The saturated answer
is 0 against a true 448: a silent, in-range, full-scale error, the worst single number this file
contains. Any evaluation of op's instinct that does not put this number on the table first is not a
stress test. Three things then bear on how much it weighs:

First, **the ratified fixed-point table already accepted exactly this behaviour.** Fixed-point
`Warm`/`Cold` clamp (`70b:8-10`, ratified), and a clamped fixed-point come-back sum produces the
identical silent wrongness, in a set with the identical topology: a bounded collection of finite
rationals, evenly spaced there, log-spaced here. If the silent come-back error were disqualifying,
it would disqualify a cell op has already ratified. Consistency runs toward the instinct, not away
from it.

Second, **the alternatives do not recover the true value, they only fail louder.** NaN-mode and the
infinity reading both destroy the entire remainder of the computation (every downstream row of the
table is NaN); refusal aborts it. In my own field's terms, which the dispatch asked for: saturation
is clipping, an audible, proportionate, bounded artifact, and it is what every fixed-point DSP and
every ADC in existence does at full scale; a NaN or an infinity in an audio buffer is not an
artifact, it is the destroyed take. The column fold's row shows the difference at its clearest:
saturate delivers 448 of a true 1024, wrong but bounded below the truth, ordered correctly against
every other column entry, usable by a consumer that reads relative magnitude; NaN-mode delivers
nothing. Which differences are audible and which are bookkeeping: the clip is audible and the
computation survives it; the poisoned fold is the difference between a number and no number.

Third, **the design's own multiplicative half dissolves the middle row.** `mul_full` computes the
exact product into the sized-up result numeral (`68:325-333`), so `(a * b) / c` written through the
design's own surface never overflows at the intermediate; the cell fires only when a consumer
stores back into the narrow numeral. The stress case that looks most like "physics computed at the
wrong width" is precisely the case the design has already removed. What remains exposed is the
additive come-back, which no width short of the accumulator's true range removes, and which the
grade must therefore witness (section 5).

*Grounded on: compiled (`71_probes/probe_2`), ratified (`70b:8-10`), settled shapes (`68:325-333`),
reasoned (the field framing).*

## 4. What each answer does to the laws

**The associativity objection proves too much, and should be retired before someone spends a file on
it.** A saturating addition is not associative; neither is an infinity-producing one (the come-back
sum bracketed the other way gives the infinity instead of NaN, or NaN through `inf - inf`); neither,
decisively, is plain in-range float addition, which the quantiser's rounding already made
non-associative before any range event fires. No candidate for this cell preserves associativity
because the operation lost it in-range. The law story for this cell is not "which option keeps the
algebra" but "which option keeps the properties the design actually keys on", and there the
candidates genuinely separate:

- **Totality on the ordered values.** Saturating quantisation is a total function into the ordered
  representable set, and weakly monotone: compiled over a 2401-point sweep, adjacent-pair check,
  which suffices for monotonicity (`71_probes/probe_2`). NaN-mode loses the order at the boundary
  (`q(460) = 448`, `q(470) = NaN`, unordered against everything, compiled). Refusal loses totality
  of the function itself, which is `Precise`'s licence and nobody else's.
- **NaN production sites.** Saturation manufactures no NaN from a range event, ever. The IEEE path
  manufactures NaN from `inf - inf` and `inf / inf`, both reachable from pure overflow; NaN-mode
  manufactures NaN from overflow directly. A saturating `NanOnly` numeral therefore has strictly
  fewer NaN production sites than an IEEE numeral, and the design carries a live shipped defect
  whose mechanism is exactly unmanaged NaN reaching a value-keyed operation (`68:876-882`, entry 7).
  Fewer manufactured NaNs is not a slogan here; it is one fewer instance of the defect class the
  layer-keying rule was written against.
- **Monotonicity is what downstream machinery leans on.** `TotalOrd`, the comparisons, any
  argmax-shaped consumer over a column: all read the order. Saturation preserves it end to end;
  the far point sits at the top of the order whether it is `inf` or 448.

*Grounded on: compiled (`71_probes/probe_2`), tree ground (`68:876-882`, entry 7, cited as a defect
record, not as design meaning), reasoned.*

## 5. What each answer does to the grade, and the mechanism I propose

The review has already made the decision that governs this section, in a different corner: **the
witness rides the grade, not a per-thread flag word**, because the value-carried grade is
deterministic under a pluggable executor and because the IEEE flag mechanism is not even reachable
from the pinned toolchain (`58` section 1.14: no `fetestexcept`, no FPCR access in
`core::arch::aarch64`, `_mm_setcsr` deprecated). Overflow is already one of the grade's five
generators, in the design's own IEEE flag-word convergence.

What the infinity reading buys that saturation lacks is a *datum-level* witness: an `inf` in the
output says "this value overflowed" forever, in-band. What saturation loses, then, is not the
witness carrier the design chose (the grade) but the one it explicitly declined to make load-bearing
(the datum). That asymmetry is the strongest structural argument for op's instinct that I found:
**the design is unusually well placed to saturate, because it alone among float systems has a
first-class witness channel that is not the datum.** IEEE overflows to infinity partly because the
flag word is a per-thread afterthought nobody reads; arvo's grade is a type the fold publishes.

The concrete proposal, compiled in shape (`71_probes/probe_3_witness_bound_and_join.rs`):

- The far-point projection of section 2 publishes a **kind**, `Absorbing` (the far point is an
  infinity; reaching it is self-witnessing in the datum) or `Finite` (the far point is finite;
  reaching it is silent in the datum). Total over the `Specials` product, const-callable, probe 1.
- The kind joins through a fold with **silence dominating**: a fold's published grade records
  `Finite` the moment any operand numeral's far point is finite, so "somewhere in this computation,
  out-of-range resolves silently" is a static, type-level fact of the fold. The join's four laws are
  checked in const context over the whole two-element carrier, all eight associativity triples, not
  a sample (probe 3).
- A consumer that *needs* the in-band witness states the need as a bound (`AbsorbingFarPoint`), and
  the bound refuses a finite-far-point numeral at the exact call site, `E0277`, both no-infinity
  members exercised (probe 3b, expected-fail, reproduced in `OUTCOMES.md`).

Whether the kind is a parameter of the existing overflow generator or a sixth generator is a call
for the grade machinery's own keeper, not mine; I state only that it is expressible under the
permitted feature set with no gates at all, and that it is honest: the static grade says "may
resolve silently", which is the true strength of the claim, since a static grade never said "did"
for any generator.

**One caution the review should carry.** The static kind does not replace a runtime witness; under
saturation there is none, anywhere, and the come-back sum's 0 arrives with a clean datum and a grade
that said only "this could happen". That is the honest residue of op's instinct and it should be in
the spec's text, not smoothed: a consumer who needs to know *whether* a particular fold saturated,
rather than whether it could, needs either an absorbing numeral or `Precise`. The fixed-point table
already lives with the identical residue.

*Grounded on: settled shapes (`58` section 1.14), compiled (`71_probes/probe_1`, `probe_3`,
`probe_3b`), reasoned (the carrier asymmetry argument).*

## 6. The well-formedness alternative, stressed and found to forbid the design's own witness

The alternative on the table refuses a no-infinity numeral under `Warm`/`Cold` at declaration. Its
best case first, honestly: the design has real precedent for declaration-time refusal, the crossing
contract's `Crosses` obligation refuses at the format declaration site (`68:250-274`), and
declaration-time is the design's preferred binding time for exactly this kind of fact. If the
pairing were *incoherent*, this would be the right shape.

It is not incoherent, and the refusal forbids the flagship case. E4M3 is the design's own deployed
witness for `Specials` being a product (`68:236-239`), and E4M3's deployment profile, the smallest
viable storage, computed rarely and widely, read often, packed in columns, is `Cold`'s intent
sentence read back verbatim ("stores as small as possible... seldom computed or used", `68b`, quoted
at `70:120-123`). The well-formedness rule would make the one real no-infinity format in silicon
unusable under the one preset whose identity matches its deployment. That is not a corner case
sacrificed for cleanliness; it is the centre of the cell's real usage, refused.

It also polices. The workspace rule is explicit: "Refusals to expose a primitive because we think
the consumer is misusing it" is on the ban list, and the correct shape is "diagnostic, not
directive" (`arvo-toolbox-not-policer.md`, "What we do NOT provide" and "Warn but never police").
A `Warm` or `Cold` E4M3 is a legitimate consumer choice with a defined, deployed, silicon-backed
meaning. The consumer-side opt-in bound of section 5 is the same well-formedness condition with the
polarity corrected: the consumer that needs the infinity states it and is refused what cannot
provide it; the consumer that does not need it is not refused on the substrate's behalf. Probe 3b is
that refusal compiling, with the diagnostic naming the bound and listing the types that satisfy it.

And a last structural point: the brief's own framing said refusal at the *operation* turns `Warm`
into `Precise`. Declaration-time refusal does not literally collide with the two-cell rule, it
shrinks `Warm`'s domain instead, but the surviving table should still be checked, and under the
supremum rule it holds: on a no-infinity numeral, `Warm` and `Precise` still differ in `OverRange`
(far point against `Refuse`), `StoredWidth` and `Door`; `Cold` and `Precise` differ in `OverRange`
and `StoredWidth`; `Warm` and `Cold` differ in `Layout` and `Door`. Every pair differs in at least
two cells with no well-formedness carve-out needed. *Reasoned, from `70`'s ratified rows plus
section 2's cell.*

## 7. The NaN-on-overflow option, given its honest day and then declined

Because `Warm`'s ratified intent is "behaves the way the plain machine float behaves" (`68b:62-67`
as quoted at `70:106-113`), and because for E4M3 the plain machine behaviour is *deployment-
configured*, NVIDIA's default conversion overflows to NaN and saturates only under `satfinite`,
option 3 has a real claim on `Warm`'s intent that the dispatch's framing did not surface. I decline
it for the preset table anyway, on four grounds:

1. **It cannot close the cell alone.** `NoSpecials` has no NaN either. A NaN-on-overflow answer
   splits the cell by `Specials` member and reintroduces the non-uniformity the supremum rule just
   removed; the far-point rule covers both members with one sentence.
2. **It surrenders the order.** Compiled: `q(460) = 448`, `q(470) = NaN`, and the fold's remaining
   rows are all NaN (probe 2). NaN is outside the total order by construction, and the design's
   entry-7 defect is the standing exhibit of what leaks when NaN reaches value-keyed machinery.
3. **It manufactures the poisoning member from a range event.** Worse than the infinity reading,
   which at least yields a value that is ordered against every finite and preserves sign.
4. **"What the hardware does" is here a mode, not a constant.** The OCP ecosystem ships both
   policies; which one a deployment gets is a configuration fact. The design already has exactly one
   place for facts of that kind: the environment parameter on the hardware door and its receipt
   (`70:213-231`, file 59's `HostFloat<E: FloatEnv>` machinery). If the review ever wants
   NaN-on-overflow, it is a `FloatEnv` fact on the door, not a `Resolution` constant on the preset,
   and the declared `Quantisation` should then match the deployed mode, which is file 70's own
   flagged-forward honesty condition (`70:487-496`) applied to this cell.

*Grounded on: physical (OCP/vendor mode split, secondary reads, primary owed), compiled (probe 2),
ratified (`68b:62-67`), settled shapes (`70:213-231`).*

## 8. Where the cell actually binds: mostly Cold, and that is the preset the rule fits best

One asymmetry the dispatch's framing folds flat, worth separating because it changes where the
weight falls. `Warm`'s door is `HostFloat<E>` with the refusal posture: a numeral the target's
silicon does not implement refuses to build (`70:324-331`). No current compile target of this
toolchain implements E4M3 arithmetic in scalar silicon, so `Warm` plus a no-infinity numeral is
today a compile-time refusal on door grounds before the `OverRange` cell is ever consulted; the cell
binds `Warm` only on future targets whose silicon takes the numeral, where section 7's mode-honesty
condition governs. `Cold`'s door is `Quantised` everywhere (`70:350-367`), so `Cold` plus E4M3 is
buildable today, on every target, and is the pairing the format's whole deployment profile points
at. The open cell is, in practice, **Cold's cell**, and it is `Cold`'s software quantiser that will
implement the supremum rule directly, where it costs one compare against a boundary the quantiser
already computes. The rule's practical home is the preset whose identity matches the format's; that
is the right way around.

*Reasoned, from `70`'s ratified door assignments.*

## 9. What is compile-time, what is runtime, what stays open, and the spec text

**Binding times.** The far-point projection, its kind, and the consumer bound are all types resolved
at monomorphisation, const-callable, no `dyn`, no `TypeId`, no feature gates of any kind in any
probe (all four compiled with a bare `--edition 2024` on the pinned nightly, `aarch64-apple-darwin`,
`-O` on the runnable ones, no other flags). The saturating resolution itself is one compare in the
software quantiser's classification step at runtime under the `Quantised` door, and is the
hardware's own `satfinite` behaviour under a hardware door whose environment deploys that mode.
Probe 2 is a value-exact model on the host; it makes no performance claim and none is needed, since
no candidate resolution differs in cost by more than the compare it already implies.

**Open, stated rather than resolved.**

1. The IEEE 754-2019 §4.3.1 overflow threshold sentence (the "at least" that includes the tie) is
   cited here from memory corroborated by secondary sources; the parity derivation in section 2 does
   not depend on it, but the corollary claim that the standard's own text agrees owes the same
   primary-source read the panel gave the E4M3 witness (`68b:36` precedent). Cheap, and named.
2. The OCP mode-split facts (NVIDIA default NaN, `satfinite` saturating, JAX/TPU on NaN, others on
   saturation) are secondary-read this session; the section 7 rejection stands on grounds 1 through
   3 regardless, but ground 4's specifics should get the primary read before the consolidation
   carries them as physical.
3. Whether the far-point kind is a parameter of the existing overflow grade generator or a sixth
   generator: the grade machinery's call, both expressible, probe 3 shows the join either way.
4. Whether `OverRange`'s constructor vocabulary states the rule as a new named member (`TowardFar`)
   or as the existing directional member over the ordered set with the supremum semantics: a
   notation call, not a semantic one; both name the same function.

**The sentences the next consolidation could take, in the provenance form.**

*The far point of a numeral on the overflow side is the supremum of its representable values in the
total order; members of `Specials` participate in the order exactly when they are ordered, so
infinities do and NaN does not. Under `Warm` and `Cold`, `OverRange` and `UnderRange` resolve to the
far point of the corresponding side. Where the ordered set is unbounded-by-infinity the far point is
that infinity (file 70's cell); where it is bounded the far point is the largest finite magnitude
and the resolution saturates (this file's cell, op's instinct, and the ratified fixed-point clamp as
the third instance of the same rule). An out-of-range event begins past the extended-grid rounding
boundary, half a top-binade ulp beyond the maximum, with the tie resolved by the ordinary even rule
on the extended grid; for every IEEE format this delivers the standard's own overflow behaviour, and
for a format whose top slot is NaN the tie falls to the finite side by the same rule. The far-point
kind (absorbing or finite) is published through the grade and joins with silence dominating; a
consumer needing the in-band overflow witness states it as a bound and is refused a finite-far-point
numeral at the call site. No declaration-time restriction pairs a preset with `Specials`.*
*(Grounded: ratified `70b:30-38`, `70b:8-10`; settled shapes `11:195-196`, `70:277-292`; compiled
`71_probes/` all four; physical OCP E4M3, secondary, primary owed.)*

The cell, as posed, does not lock to an answer; it dissolves into a rule of which both ratified
tables were already instances. That is my evaluation of op's instinct: correct, and more correct
than its own graceful-degradation framing, because nothing degrades. The lock is op's.

## 10. The table-diff self-check and verification

The section 3 table was checked cell by cell against `71_probes/OUTCOMES.md`'s verbatim output, and
the section 6 pair-difference check against file 70's ratified rows plus section 2's cell rather
than against any earlier table. Canon gate, test gate, and toolchain reproduced fresh at the top of
this document. All four probes compiled and run this session; outcomes, including the expected-fail
E0277 pair with both no-infinity members exercised, reproduced verbatim in `71_probes/OUTCOMES.md`.
No claim in this file rests on a shipped doc comment, and every tree citation above points at a
defect record or a panel file, never at shipped prose read as design meaning.
