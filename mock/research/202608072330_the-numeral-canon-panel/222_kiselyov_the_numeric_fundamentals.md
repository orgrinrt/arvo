# 222. The numeric fundamentals: what the ratified canon already decides, and what an instrument had to decide

Seat 222. One half of a blind pair on the open questions of `the_number_system`,
`the_format`, `rounding`, `overflow_policy`, `the_primitive` and
`the_container_premise`.

The short version is that the canon decides considerably more of this than the
question rows suggest, and that most of what it does not decide is decided by a
computation somebody could have run in an afternoon. Of the thirty-seven rows I
was sent at, four are already closed at the governing tier and render as open
because nothing follows the edge that closes them; three more are settled by an
instrument I built here; eight are malformed in a way the canon itself names, so
the useful act is retiring them rather than answering them; and one is not
answerable from the canon at all, because the sentence whose reading it
disambiguates was never admitted to the canon. The rest are derivations of
varying strength and each says which it is.

I want to be exact about what my agreement is worth before any of it. Where a
`one_expert` proposal already answers a question, I read that proposal before I
derived anything, because `mock/registry/*.toml` is the canon and the brief made
it required reading. So my agreeing with such a row is confirmation and not
corroboration, and it does not move it to two experts. I mark those as
confirmations and say so at each one. What I offer as independent are the
entailments from ratified text, which are applications rather than derivations
and do not need a second instance, and the four probes, which are instruments
nobody had built.

## The two gates

**The canon gate passes.** I checked the assigned work against
`mock/registry/ruling.toml` and specifically against
`ruling::the_panel_finishes_the_canon_without_him` (`ruling::the_panel_finishes_the_canon_without_him`), which
says every remaining canon question is the panel's, that a question filed as op's
is now derived from what he has already said, and that nothing is parked awaiting
him. Deriving answers to open question rows is exactly what that licenses. Two
things in the brief's own framing I did have to test and both held: the registry
is the canon by `canon_paths` in `mockspace.toml:31`, and the questions were
ported without answers by that file's own stated policy, which
`question.toml`'s own header comment records, in the paragraph beginning "No answer is recorded here".

**The test gate passes and the suite is real.** `cargo test --manifest-path
mock/checks/Cargo.toml` runs 152 tests across nineteen files and all pass. I read
the bodies rather than the names in the surface I touch, which is
`every_predicate_names_a_declared_axis.rs` and
`a_settled_question_does_not_sit_in_the_queue.rs`. Both are genuine: every arm
runs against a planted input as well as against the committed canon, the planted
inputs are the failures the arm exists to catch, and the files say in their own
prose why an arm that has only seen a clean canon establishes nothing. The
declaration class the workspace's test gate warns about is absent here: no test
asserts a constant against the literal its own definition sets, and the predicate
checker's assertions reach through to the parsed rows rather than to another
declaration. I found one gap and it is already recorded rather than hidden:
`predicate.rs:17` states outright that the values side of a predicate is not
checked and gives the reason, and
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` names two live
violations that the slug-side-only arm cannot see. That is a known hole with a
stated reason, which is not the same thing as a decorative suite.

## Blindness, and what I read

I read, in this order and nothing else before committing this file: the four
`.claude/rules` directories generated into my worktree, `mockspace.toml`, every
file under `mock/registry/`, `mock/checks/src/` and `mock/checks/tests/`,
`mock/Cargo.toml`, the probe sources and outputs under
`mock/research/202608072330_the-numeral-canon-panel/*_probes/` that I name below,
and one `awk` range over `OPTIONS.md` covering the heading of Q57 alone.

I did not read any numbered member file, I did not run `git log`, and I did not
look for the parallel seat's branch. The one place my blindness came close to
the edge is that `OPTIONS.md` range, which is a living ledger rather than a
member deliverable; I took it because Q57's option set in the registry is
uninterpretable without it, and it turned out to be uninterpretable with it too,
which is the finding in section 9.

Two things the coordinator flagged as having landed on the trunk while I worked
turn out to have been in my tree already, so neither changes anything here.
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` is at
`rung = "ratified"`, `ratified_by = "experts"` at `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` in the tree I
cut, and I wrote every predicate below under it. The three questions closed
through `ruling::the_format_spine_is_canon` I found by walking the same two hops
myself, before the message arrived; that walk is section 3.1 and it is the first
thing I did after counting the rows.

## The count

Thirty-seven, matching the brief exactly: `the_number_system` fifteen,
`the_format` ten, `rounding` eight, `overflow_policy` two, `the_primitive` one,
`the_container_premise` one. Measured by walking `[[question]]` blocks in
`mock/registry/question.toml` and counting those with no `answered` key.

Three further rows carry no `answered` field and no `topic` at all
(`should_phase_collide_across_two_vocabularies`,
`the_cross_repo_strategy_name_collision`,
`is_the_non_terminating_contention_crate_a_defect`). They are outside my
assignment and I did not touch them, but a topicless question is invisible to any
roster built by topic, which is worth somebody's attention.

**What "no `answered` field" means is weaker than it reads**, and this matters for
how the thirty-seven should be understood. `what_then_validate_requires` (Q1) has
no `answered` field and is nonetheless closed: `ruling::validate_means_all_three_readings`
(`ruling::validate_means_all_three_readings`) carries op's verbatim answer, names the row in its `answers`
edge, and the question's own `note` says "Recorded as answered at `28` batch one".
So the field's absence marks rows the schema's newer answer mechanism has not
reached rather than rows nobody has answered. Reading the count as thirty-seven
open questions overstates it, and by more than one.

## 3. Four structural findings, which change what several of the questions are

These come first because three of them move questions out of my answer list
before any derivation starts, and the fourth changes the shape of eight more.

### 3.1 Three questions are closed at the governing tier and the queue cannot see it

`ruling::the_format_spine_is_canon` (`ruling::the_format_spine_is_canon`) is ratified, marked
`ratified_by = "both"`, and carries a `ratifies` list of four propositions. Three
of those four carry `answers` edges, and every one of the three lands in my
assignment:

| ratified proposition | line | closes |
|---|---|---|
| `a_format_is_identified_by_its_ambient_domain_and_its_representable_set` | `proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set` | `adaptation_in_identity_or_realisation` (Q18) |
| `membership_of_the_representable_set_is_one_affine_predicate` | `proposal::membership_of_the_representable_set_is_one_affine_predicate` | `which_width_coordinates_a_consumer_writes` (Q2) |
| `the_concept_is_closed_and_the_inventory_is_open` | `proposal::the_concept_is_closed_and_the_inventory_is_open` | `is_the_number_system_inventory_open` (Q20) |

**The mechanism that hides them is one line of the checker.**
`checks/tests/a_settled_question_does_not_sit_in_the_queue.rs:80` defines
`answered_by`, and its walk at line 82 is `for ruling in reg.of("ruling")`. It
reads `ruling.answers` and nothing else. A question closed by a ratified
*proposition* rather than by a ruling is therefore invisible to the arm whose
whole purpose is finding settled questions still in the queue, and the file's own
module doc says why that matters: "a question op has already answered renders as
open, reads as open, and counts as open in every roster built by reading
`decider`". It is right about the failure and wrong about the extent, because it
guards one of the two edges that produce it.

The fix is two lines in `answered_by`: walk `reg.of("proposal")` as well, and
report only where the answering proposition appears in some ruling's `ratifies`.
That last condition is what keeps a `one_expert` proposal from closing anything,
which it must not.

**A fourth row is in the same class and I am less sure of it**, so it is
separate. `staged_narrowing_disagrees_with_direct_narrowing_under_round_to_nearest_even`
(`proposal::staged_narrowing_disagrees_with_direct_narrowing_under_round_to_nearest_even`) answers `does_narrowing_compose` at `standing =
"three_or_more"` with an exact predicate and a named instrument. That is not a
ratified proposition, so it does not close the question the way the three above
do; what it does is make the question's binary option set false, which is section
6.1.

### 3.2 A live self-contradiction in the registry, about the general form of a rounding law

`proposal::staged_narrowing_disagrees_with_direct_narrowing_under_round_to_nearest_even` has a `note` naming the instrument
`07_probes/p4_composition_and_forced_adjoint.py` and says of it, in
the same sentence, "and no `probe` row names it, so the general form is not
writable and what is written is the one instance whose instrument is
registered". The preceding line says, in bold, "**The general rule behind it is
in the corpus and cannot be written here.**"

`probe::narrowing_composes_where_the_modes_direction_switches_at_coarser_grid_points` is a row with `id =
"narrowing_composes_where_the_modes_direction_switches_at_coarser_grid_points"`,
whose `lives` list at line 987 is exactly
`07_probes/p4_composition_and_forced_adjoint.py`, and whose `establishes` states
the general rule verbatim: narrowing composes exactly when the mode's direction
switches only at coarser-grid points. It carries `standing = "sound"` and a
control whose note is the best statement of method in this corpus.

So the blocker is gone and the sentence recording it is still there. The general
form is writable now, and section 6.1 writes it. I did not check which row is
older, because I may not run `git log` and because it does not matter: both are
in the committed canon at the same commit and they disagree.

### 3.3 One `answers` edge reaches past its own `says`

`membership_of_the_representable_set_is_one_affine_predicate` closes Q2, "which
pair of width coordinates does a consumer write at the surface?" Its `says`
establishes that membership is one affine predicate over a slot function, a
quantum and a phase, of which integers, fixed point, scaled integers and floats
are points. That is Q2's fourth option's *reading*, which the row's own question
describes as "the numeral is one object, a grid cut down to a reach, and the
width pair is what that object is called". It settles that the width pair is a
projection of one definition rather than a definition.

It does not say which projection a consumer writes, which is the question's first
three options and its literal text. So Q2 is closed at the definitional half and
open at the surface half, and the residue is a strictly narrower question than
the row as written. That is the same failure the ratification gate exists to
catch, in a field the gate does not read: the `promotion` note on
`the_additive_and_absorption_verdicts_are_canon` says the gate is that "neither
prose reaches past its predicate", and an `answers` edge is prose about scope
that nothing audits.

### 3.4 Eight of the thirty-seven ask which single policy governs a category

The canon names this shape and rejects it.
`ruling::there_is_no_universal_answer_take_the_win_and_gate_it`
(`ruling::there_is_no_universal_answer_take_the_win_and_gate_it`) carries op's words: "we don't need to settle for one
universal solution, it's the anti-pattern I've already named. Case by case ...
Take the win where it applies, gate it out from where it does not. No single
one-fits-all solutions, it's impossible." The ratified
`ruling::the_work_is_predicated_arms_composed` (`ruling::the_work_is_predicated_arms_composed`) and
`ruling::arms_over_regions_are_the_fundamental_heart` (`ruling::arms_over_regions_are_the_fundamental_heart`) say
the same thing positively.

The eight, and what each is really asking once the shape is removed:

| row | asks for one policy over | the answer the canon already gives |
|---|---|---|
| `does_warm_wrap_or_clamp` | a whole strategy's overflow behaviour | the mode is declared per site; section 5.1 |
| `do_arvos_consumers_want_value_keying_or_position_keying` | a whole consumer base | both, each gated; its own `bound` says so |
| `does_narrowing_compose` | a whole design | a predicate on the rounding mode; section 6.1 |
| `is_the_derived_numeral_required_to_be_tightest` | every derived numeral | soundness in the canon, tightness as arms; 7.3 |
| `is_the_cross_kind_join_closed_or_priced` | the whole shape space | priced and unclosed, arms per region; 7.4 |
| `does_the_position_keyed_dither_arm_ship` | whether an arm exists at all | it ships; 7.6 |
| `does_a_consumer_supplied_seed_surface_exist` | whether a surface exists at all | it exists; 7.7 |
| `whose_reduction_governs_a_lossy_crossing` | every lossy crossing | the target's, by the ratified factoring; 6.4 |

Three of those eight already say so in their own `bound` fields, which were
written after op returned them. That is the discipline working. What it has not
done is change the `options` arrays, so each still reads as a live fork to anyone
who reads the row rather than the bound, and the option arrays are what a
consolidation compresses. Rewriting an option set is not a port's call and is not
mine either, so I am naming them rather than editing them.

## 4. Already closed at the governing tier, with what the answer is

Four rows. I am not deriving these, I am reporting what the ratified text says and
which option it selects, because a question whose answer is already ratified wants
reading rather than answering. None carries a predicate: all four are normative,
and a normative sentence has no region because there is nothing an instrument
could refute. That is the same reading the number-system block states about itself
in the header comment above its own block in `proposal.toml`.

**Q18, `adaptation_in_identity_or_realisation`. Answer: realisation.** The ratified
`a_format_is_identified_by_its_ambient_domain_and_its_representable_set` says
"Adaptation choice and encoding are realisation, observable in computed values and
in pattern-level properties respectively, and not part of identity". The row's
`answers` edge names this question. Its option two is that answer, word for word.

One qualification travels with it and comes from a later row rather than from the
ratified one. `the_realisation_is_not_part_of_denotational_identity`
(`proposal-the-later-topics.toml`, `topic = "the_primitive"`) narrows the word
`identity` to denotational identity, and assigns the carrier to representational
sameness, which licenses memory reinterpretation and licenses nothing about
substitution inside a composite. Unqualified, the ratified sentence is ambiguous
between the two relations and the container premise makes the ambiguity bite.
Qualified, it is true on both branches of that premise. A reader taking Q18's
answer should take it qualified.

**Q2, `which_width_coordinates_a_consumer_writes`. Answer: the definitional half
only, and the surface half is a narrower question still open.** Per 3.3 above. The
ratified `membership_of_the_representable_set_is_one_affine_predicate` selects the
fourth option's reading: the numeral is one object, the width pair is what it is
called, and the question is what projection to show. Which projection is not
settled by anything I can find, and the honest residue is one sentence rather than
the four-option fork the row carries.

**Q20, `is_the_number_system_inventory_open`. Answer: open, with the concept
closed.** The ratified `the_concept_is_closed_and_the_inventory_is_open` is the
answer and names the question in its `answers` edge. Its option one, "closed,
which is checkable and lets a canon enumerate what it covers", is refused by
ratified text.

That refusal is load-bearing three sections below, so it is worth stating as a
rule rather than as a fact about this row: **any answer that would require a new
instance to be admitted by amending the canon contradicts a ratified sentence.**
The ratified clause says a new one "earns admission by supplying the concept's
obligations rather than by amending the canon", and I use that as a test twice, at
6.2 and 6.3.

**Q1, `what_then_validate_requires`. Answer: all three readings.**
`ruling::validate_means_all_three_readings` (`ruling::validate_means_all_three_readings`) carries op's
verbatim words, "Usage, Admissibility, Self-validation, All that makes sense", and
names the question in `answers`. It is `stated` rather than ratified, so it binds
as direction. Its challenge bar is his own phrase, "truly not worth it", and the
row's note is careful that his stated prior about robustness is not part of the
call. Nothing in my scope challenges any of the three.

## 5. Settled by an instrument, with the instrument committed

Four probes, in `222_probes/`, sources and outputs committed together with
`RUN.md`. Each prints its predictions before its result and each prints its
controls firing rather than asserting that they would. Two of them caught me
predicting wrongly, which is recorded in the probe source rather than tidied away,
because a control that has never fired has established nothing.

### 5.1 The rounding variance forms hold at every fraction width, in ulp units, and only there

`question::does_the_rounding_variance_form_hold_at_a_second_fraction_width`. The
row says the forms were "stated as algebraic in the fraction width from a check at
one fraction" and that one sweep at a second fraction is cheap and unrun.

**Answer: yes in units of the coarse quantum, at every fraction width, and no in
absolute units, where the forms carry a factor of `4^-Fc`.** The corpus does not
state which unit it means, and that is the defect rather than the arithmetic.
`a1_output.txt` Part 3 prints the same `45/32` at `Fc` of 0, 1, 3, 8, 16 and 31 in
ulp units, beside the absolute column moving from `45/32` to
`45/147573952589676412928`.

**And neither prior instrument stood on the fraction-width axis at all.**
`128_probes/r3_output.txt` and `130_probes/y1_output.txt` both report
`f(1-f) = 2/9`, which forces `f = 1/3` or `f = 2/3`. Neither is a binary rational,
so neither is the residue of any narrowing between two binary fixed-point grids.
`a1` Part 0 walks every `j/2^d` for `d` in 1 to 24 and finds no such residue, with
a positive control on the same walk finding `1/2` at all twenty-four widths, so the
zero is a fact about the residue set rather than about a loop that never matches.
This does not falsify the forms. It says the axis was never varied, which is
exactly what the question suspected.

Three routes compute the independent coupling's variance and none may assume the
answer: full enumeration of all `2^n` outcome vectors with exact rational weights,
the binomial closed form over `k`, and an integer-only second central moment.
The third is written the way it is on purpose: using additivity of variance across
independent summands to check a claim that additivity encodes would be circular,
so that route computes `(M2*T - M1^2) / T^2` from integer moments and never touches
the identity. Zero disagreements among the three over 300 cells, and zero
mismatches against both closed forms over 1657 further cells.

The controls fire: a deterministic nearest rounding gives variance 0 against forms
of 60 and 240; an antithetic pair at `n = 2` gives 12 against 30 and 60; a scheme
rounding up with probability `f^2` has mean `-15/2` rather than 0; and the mutated
predictor, which offers the comonotone form against independent data, disagrees on
182 of 182 cells, so the zeros above are the output of a comparison that can report
a difference.

```
holds for: fraction_width: F any: construction, the per-rounding error is defined
             in units of the quantum and the fraction width is what sets the
             quantum, so no fraction width can appear in the expression
             k*m - n*j that the whole result is about
           chain_length: chain length in 1..=10: exhaustive, every chain length in
             the band, at every attainable residue, at four values of the dropped-bit
             count
           rounding: rounding = stochastic
           radix: radix = 2
           threads: threads = 1
           toolchain: rustc = 1.98.0-nightly (57d06900f 2026-05-27)
           toolchain: edition = 2024
           build_profile: debug-assertions = off
evidence:  222_probes/a1_the_variance_forms_across_fraction_widths.rs, Part 3,
           which varies the coarse fraction width over six values and reports the
           ulp-unit variance identical at all six
```

**What the absences mean, said out loud because they are the sharp part.**
`total_width` and `overflow_policy` are unlisted, so under
`ruling::a_predicate_lists_only_what_holds` the forms hold in no situation where
either exists. That is correct and it is the useful half of the finding: the model
accumulates in an unbounded integer, so the moment an accumulator can overflow or
saturate, neither form is the variance. `signedness` is unlisted because the probe
is parameterised by a residue rather than by a value and never instantiated a
signed numeral. The dropped-bit count, `Ff - Fc`, is the axis the sweep actually
walked and no `dimension` row declares it; the same gap is already recorded in
`proposal::staged_narrowing_disagrees_with_direct_narrowing_under_round_to_nearest_even`'s `note`, about a staged narrowing's intermediate width, so this is a
second instance of one missing axis rather than a new complaint.

### 5.2 The two keyed rounding members do differ, and the direction is not uniform

`question::does_the_position_keyed_members_monotonicity_failure_rate_differ_from_the_independent_members`.
The row says two source files each hold one count, neither measured the other
member under the same construction, and what would decide it is a sweep with one
construction held fixed across both members, "which nobody has built".

**The comparability problem is the question, and it has a named fix.** One member
is deterministic and one is random, so a realised count for one and a realised
count for the other are not the same kind of number.
`129_probes/x1_output.txt` reports 7 violations over 40 consecutive pairs for the
position-keyed member: an exact count of a deterministic process at one phase. An
independent member has no such number, it has a distribution. The statistic that
puts them on one footing is **expected monotonicity violations per consecutive
pair**, with the deterministic member averaged over the dither's phase.

**Answer: they differ, position-keying is worse on 57 of the 60 cells where either
is nonzero, in a narrow band of 1.20 to 2.40 times the independent rate, and it is
exactly zero on the other 3, where the independent member is not.** That last part
is the result. A canon sentence saying one keying is less monotone than the other
would be false, because there is a region where the position-keyed member is
strictly better, and the region is nameable at compile time.

The mechanism is the joint and not the marginal. The golden-ratio additive
recurrence fixes `u_{i+1} - u_i mod m` to two values out of `m`; `a2_output.txt`
Part 2 prints 2 of 8, 2 of 16, 2 of 32 and 2 of 64 over 256 steps. An independent
member spreads over all `m` increments. Once the marginals are equalised the
entire difference is that diagonal.

**Equalising the marginals is where I got it wrong first, and the control caught
it.** The first phase model advanced the recurrence's index, `u_i(p) = key(i + p)`.
`a2_output.txt` Part 0 reports that model's phase-averaged marginal disagreeing
with `j/m` at 14 of 16 residues at `m = 16`, because sixteen successive terms of a
golden-ratio walk are equidistributed only asymptotically. Under that model the
Part 1 numbers were comparing a biased member against an unbiased one, which is
comparing two rounding schemes rather than two keyings. The model that works makes
phase a uniform offset on the threshold, `u_i(p) = (key(i) + p) mod m`, which is
exactly uniform per point and leaves the consecutive increment untouched. Both
models are still in the file and both still run, so the paragraph can be checked
rather than believed.

The three zero cells are `d = 3, delta = 5` at all three ramp lengths, and the
mechanism is arithmetic rather than luck. At `m = 8` with step 5 the residues run
`0, 5, 2, 7, 4, 1, 6, 3`, and the only pairs sharing a coarse cell need the key to
move from at most 1 up to at least 7, or from 0 up to at least 6. The recurrence's
increments at `m = 8` are 4 and 5, and neither bridges either gap. So the
position-keyed member is exactly monotone there and the independent member is not,
and both facts are computable from `d` and `delta` alone, which is a const
predicate an arm can be gated on.

The remaining controls fire: shared threshold reports exactly zero on all 114 rows,
which is the known-true case and the check that the violation detector works at
all; an adversarial alternating key reports 0.44 against 0.14, so the counter can
report a high rate; a decreasing key reports zero for a different structural reason
than the shared threshold, which separates a real zero from a sleeping arm; and at
`delta = m` every member is zero, because no consecutive pair shares a coarse cell.

```
holds for: access_pattern: access pattern = sequential
           rounding: rounding = stochastic
           radix: radix = 2
           threads: threads = 1
           toolchain: rustc = 1.98.0-nightly (57d06900f 2026-05-27)
           toolchain: edition = 2024
           build_profile: debug-assertions = off
```

The dropped-bit count ran over 2 to 6, the ramp step over `{1, 2, 3, 5, 7, m/2, m,
m+1}` and the ramp length over `{16, 41, 128}`, and none of the three is a declared
axis, so none of them appears above. `operation` is unlisted because the
narrowing is not a value in the declared operation vocabulary and `operation any`
is declared inadmissible by `dimension::operation`'s own grammar; the notation
cannot express this row's operation and I am not inventing a value to fill it.
`signedness` is unlisted: the ramp is non-negative throughout.

### 5.3 The footprint is observable, and it is observable at const time

`question::the_container_premise`. Three of its own options are listed and the
first is the live one: footprint is observable, under which the clause saying the
realisation is not part of identity is false as written.

**Answer: observable, through exactly one observation, and that observation is
const.** The first half is a third instance of what
`the_carrier_is_observable_through_the_ambient_layout_observation_alone` already
measured, on a construction of my own: `a3_output.txt` separates `N13U16` from
`N13U32` at 2 bytes against 4 and at alignment 2 against 4, while roundtrip,
wrapping addition, saturating addition, wrapping multiplication, saturating
multiplication and exclusive-or all report 0 differences over all 8192 values and
all 67108864 ordered pairs. Two newtypes over the same carrier are not separated,
so the observation reads the carrier rather than type identity, and a zero-sized
marker reports 0, so it reads real layout.

**The second half is not in any row I could find and is the part worth taking
forward.** `core::mem::size_of` is a const function, so the footprint is available
where a predicated arm is allowed to read. The probe compiles two `const` items
whose values came from `size_of`, so the claim is a compile result rather than an
argument, and `ruling::the_predicate_is_whatever_is_available_at_const_time`
(`ruling::the_predicate_is_whatever_is_available_at_const_time`) is the sentence that makes it decisive rather than a
curiosity: under it, an axis available at const time is a predicate, and under
`ruling::never_a_runtime_check_and_one_lowered_path` (`ruling::never_a_runtime_check_and_one_lowered_path`) reading
it costs nothing at runtime. So the container premise does not merely decide
whether a sentence about identity is true; it hands the design a gateable axis.

The probe uses `core::mem` rather than `std::mem` on purpose, because the question
is whether the observation reaches a `no_std` crate, and `std` would beg it.

**Two dead routes on the control, both kept and both still run, because between
them they say when a carrier is arithmetically observable at all.** The first
mutant was an addition wrapped at the carrier width: 0 of 67108864, because two
13-bit operands sum below `2^16` and no carrier in the pair can truncate an
addition. The second was a multiplication wrapped at the carrier width and then
masked to the declared width: also 0, and this one is the more instructive
failure, because the declared-width mask is a mask of the low bits and the carrier
mask is a wider mask of the same low bits, so the narrower absorbs the wider and
the truncation is unobservable however large the intermediate grows. The rule that
falls out is that the carrier is arithmetically observable only where the
intermediate can leave the narrower carrier **and** the projection reads magnitude
rather than low bits. The smallest such projection is saturation, and the working
mutant is a saturating multiply with a carrier-held intermediate: 8346078 of
67108864 pairs, 12.4 percent.

```
holds for: integer_width: I = 13
           fraction_width: F = 0
           signedness: signedness = unsigned
           container: container in {u16, u32}
           operation: operation in {encode, wrap_add, sat_add, wrap_mul, sat_mul, xor}
           arity: arity in {1, 2}
           alignment: alignment = aligned
           threads: threads = 1
           toolchain: rustc = 1.98.0-nightly (57d06900f 2026-05-27)
           toolchain: edition = 2024
           build_profile: debug-assertions = off
```

`alignment = aligned` is written rather than omitted for the same reason the
existing row writes it: this is a sole occupant of its own carrier allocation and
says nothing about a packed placement, where
`at_shared_occupancy_no_per_element_footprint_observation_exists` goes the other
way and I did not test it.

### 5.4 Precision does not count the sign digit, at radix two

`question::does_precision_count_the_sign_digit`. The register names this as one of
exactly two things genuinely undetermined, and its `bound`, written after
`ruling::the_panel_finishes_the_canon_without_him` returned it, already carries the
decision procedure: "Nothing decides this mathematically ... Answered inside the
arms paradigm: take the reading under which the three sign domains form a
structure a const predicate can gate on rather than one leaving a domain
incomparable."

That is not a preference, it is a computation, and nobody appears to have run it.
Build the three sign domains under each reading, order them by inclusion, and see
which gives a chain.

**Answer: the precision coordinate does not count the sign digit.** At radix two,
over precisions 1 to 6, reading B gives a chain at every cell, `unsigned` inside
`symmetric` inside `twos_complement`, 30 of 30. Reading A gives a chain at 0 of 30
and leaves `unsigned` incomparable with both signed domains at every cell. The
criterion the bound states selects reading B and selects nothing else.

Two further consequences the row's `unblocks` asks for, both computed:

Reading A manufactures a singleton at a legitimate precision. The symmetric domain
at precision one denotes exactly `{0}`, at radix 2, 3 and 10 alike. Reading B
produces no domain of fewer than two values at any precision at or above one, 0 of
30 against 5 of 30. That is a second, independent source of the sub-two-value
numeral `question::inclusion_order_singleton_amendment` exists to handle, and
choosing reading B removes it. **It does not remove the one that question is
actually about**, which its note locates "at the coarsest declared step in the
box", a grid with one point in a declared range rather than a sign domain at
precision one. So this narrows Q10's case set without closing it, and I say so at
7.5 rather than claiming more.

**The odd-radix half refuted my own prediction and the probe records that.** I
predicted the unsigned and symmetric domains would collapse to equal cardinality at
an odd radix under the sign-magnitude model. They do not: at `r = 3, P = 1` they
are 3 against 5, and the first version of the probe printed those numbers in the
same block as prose asserting the collapse. The collapse is real and belongs to
the **balanced** model, where the centred digit set has exactly `r^mag` members,
equal to unsigned's count at every odd radix and every precision I ran. It is a
cardinality coincidence and not a set equality, so a design keying anything on
cardinality alone would merge two domains there.

**The limit of the answer, which is the important part.** The corpus writes the
third domain as "signed symmetric range" (`proposal::the_laws_of_a_format_are_derived_from_two_hypotheses_rather_than_enumerated_per_policy` and
`proposal::the_law_frame_was_attacked_from_another_topic_and_held`) and does not say which construction that is. Two answer to
the phrase and they are different sets: sign-magnitude, which is two's complement
with the extra negative removed, and balanced radix, which is the centred digit
set. Under the balanced model **neither** reading gives a chain, at 0 of 15 and 0
of 12, because a centred domain never contains the unsigned one. So the answer
above is conditional on the symmetric domain being the sign-magnitude one.

At radix two that condition is not a condition at all: `(2^mag - 1)/2` is not an
integer for any `mag` at or above one, so the balanced model does not exist there,
and the probe skips those cells rather than rounding and calling it a result. The
design's radix is two everywhere the corpus measures. So the answer holds
unconditionally where the design lives and is undecided by this criterion at an
odd radix under a balanced reading.

All four detectors were run against planted inputs before anything rested on them:
the chain detector reports `None` for `([0,1],[1,2],[0,2])` and `Some` for a
nested triple, the singleton detector reads 1 for `[0,0]` and 2 for `[0,1]`, and
the inclusion test reports false in both directions for two overlapping intervals.
The interval shortcut was re-checked against explicit set inclusion at every
affordable cell, 0 disagreements.

```
holds for: radix: radix = 2
           signedness: signedness in {unsigned, twos_complement, symmetric}: exhaustive,
             the three sign domains the corpus names, which is the whole of the axis
             as the corpus states it
```

Two things about that block. The `signedness` grammar declares only `unsigned`,
`signed` and `any`, so the three-domain vocabulary the corpus uses in its own
predicates is **not expressible in the declared grammar**, and this row is
therefore written in a dialect the checker cannot read. That is a third instance of
one problem: the dimension row and the corpus disagree about the axis's value set.
And precision itself is not a declared axis; `integer_width`, `fraction_width` and
`total_width` are, and the question is about a coordinate none of them is.

I ran the sweep at radices 2, 3, 4, 5 and 10 and precisions 1 to 6, so a reader
wanting the wider claim can have `radix in {2, 3, 4, 5, 10}` for the
sign-magnitude model. I write `radix = 2` above because that is where the answer
is unconditional, and widening a predicate is a new claim in a new deliverable
rather than a widening in place.

## 6. Entailed by ratified canon

These are applications rather than derivations. Each is a consequence of a
sentence that is already ratified, so none needs a second instance and none is
mine to be corroborated on. Where an entailment agrees with an existing
`one_expert` proposal I say so and the agreement is worth nothing extra, because I
read that proposal first.

None of these carries a predicate. All of them are normative, and a consequence of
a stipulation is part of the stipulation.

### 6.1 Narrowing composes exactly where the mode's direction switches on the coarse grid

`question::does_narrowing_compose` asks whether the design wants narrowing to
compose, yes or no. Both options are false, and the canon holds the true sentence
in two places that do not know about each other.

`proposal::staged_narrowing_disagrees_with_direct_narrowing_under_round_to_nearest_even` measures the instance: staged narrowing disagrees with direct
narrowing under round to nearest even, first at `-247/16` on a nine-bit signed grid
staged from four fraction bits through two to zero, at `standing = "three_or_more"`
with three named witnesses. `probe::narrowing_composes_where_the_modes_direction_switches_at_coarser_grid_points` holds the general form: narrowing
composes exactly when the mode's direction switches only at coarser-grid points,
tested through two consequences of an adjunction rather than through its defining
biconditional, at zero failures on-grid and seven off-grid.

So the answer is a predicate on the rounding mode, which is what
`ruling::arms_over_regions_are_the_fundamental_heart` asks every answer to be:

- **composes**: the four directed modes, and `toward_zero`, whose only direction
  switch is at zero and zero is a point of every anchored grid;
- **does not compose**: `half_up` and `half_even`, which switch direction at every
  cell midpoint of the finer grid and no midpoint is a point of the coarser one.

Both arms exist, both are reachable, and the design owes a gate rather than a
choice. The row's second option, "the canon owes the sentence that narrowing twice
is not narrowing once", is owed only on the second arm and is already written at
that row's `because`.

The blocker recorded in that row's `note` is stale, per 3.2, and the general
form is writable today.

### 6.2 The ambient operation family is a parameter, and the concept is therefore broad

`question::is_the_ambient_operation_family_fixed` (Q33), read as one with
`question::is_number_system_broad_enough_for_non_magnitude` (Q21), which its own
note instructs.

**The argument is the amendment test from 4 above.** Ratified text says a format is
identified by its ambient domain and its representable set, so a candidate must
supply an ambient domain to have an identity at all. Ratified text also says the
inventory is open and a new instance "earns admission by supplying the concept's
obligations rather than by amending the canon". Fix the operation family at
addition and multiplication, and a min-plus system cannot supply an ambient domain
within the obligations, because its domain has no multiplication in the required
sense. It could then join only by amending the canon, which is precisely the
mechanism the ratified sentence excludes. So the family is a parameter.

**And min-plus is not a hypothetical here.** `obligation::a_cost_dynamic_program`
and `obligation::ordering_a_directed_acyclic_graph` name `hilavitkutin` as the
consumer, and `probe::monotonicity_and_absorption_are_two_properties_and_a_reserved_top_buys_one`
measures a min-plus shortest path across three overflow policies at unsigned widths
3 and 4. The design already computes in the tropical semiring on behalf of a named
consumer. Q33's first option concedes exactly this in its own cost clause, "the
named selling point computes in something the canon does not cover", and the
concession is fatal rather than a price.

Q21 follows without further argument. The two-element Boolean algebra and the
vector space over the two-element field are ambient domains over ordinary
representable sets, so they are members under a parametric family and unadmittable
under a fixed one. This agrees with `the_concepts_edge_is_not_an_order_and_wrapping_is_the_test`
(`proposal::the_concepts_edge_is_not_an_order_and_wrapping_is_the_test`) and I read that row first, so my agreement is confirmation.
What is new is the route: that row argues from the measured emptiness of the
order-shaped discriminator, and this one argues from the ratified admission
mechanism, so the two do not share a premise even though they share an author's
reading order.

**What the second option's cost list gets right.** Every law sentence acquires a
scope it did not need, and prefix equality becomes a relation somebody must
define. Those are real and they are the price of a ratified sentence rather than a
reason to reconsider it.

### 6.3 Set-valued carriers are outside the format concept, and a datum stands for a point

`question::are_set_valued_carriers_admitted` (Q22) and
`question::what_a_datum_stands_for` (Q4), which are the same question at two
depths.

Ratified `membership_of_the_representable_set_is_one_affine_predicate` says
membership is one predicate over one parameterisation, "an affine slot function, a
quantum per magnitude and a phase, of which integers, fixed point, scaled integers
and floats are points". An interval is a pair of grid points and is not a point of
that parameterisation. So a set-valued carrier is not a format, and Q22's answer is
scoped out **of the format concept**, which is not the same as scoped out of the
design. `the_format_concept_carries_three_things_upward_and_compositions_owe_their_own_laws`
(`proposal::the_format_concept_carries_three_things_upward_and_compositions_owe_their_own_laws`) already places them: intervals and error-carrying values are
compositions over formats that consume the width algebra, the named adaptation and
the exactness predicate, and owe their own laws. That row is one expert and I read
it first; the ratified clause is what does the excluding, and the row is what says
where the excluded thing goes.

Q4 falls out at three of its four options:

- **Option 4, a set admitted generally, is refused** by the same ratified clause.
- **Option 2, an absorbing top, is refused** by ratified
  `a_format_is_identified_by_its_ambient_domain_and_its_representable_set`, which
  puts adaptation choice in realisation and not in identity. A denotation that
  changes when the overflow member changes would make the adaptation part of what
  the datum means, which is the clause's own negation.
- **Option 1, a point, is what is left and is positively supported**: the
  representable set is a constant of the type and its elements are values, and
  ratified `arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`
  makes the adaptation a first-class object with its own laws, so saturation's
  absorbing behaviour is a property of the adaptation. Which is option 1's own
  wording, "a documented restriction on where a fold is sound rather than a
  denotation".
- **Option 3 is not a rival and should not be listed as one.** It says the
  denotation clause is a statement about the constructor wearing the grammar of a
  statement about every datum. That is a diagnosis of how the question became
  confusing and it is compatible with option 1 rather than an alternative to it.

### 6.4 The target's reduction governs a lossy crossing

`question::whose_reduction_governs_a_lossy_crossing` (Q36). No proposal answers it
and `an_order_is_named_exactly_where_a_crossing_is_lossy` records it as an open
gap.

Ratified `arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`
says arithmetic on a format is "an exact operation in an ambient domain composed
with a named, total adaptation onto the representable set". A crossing into a
target format is that shape with the exact operation being the identity: take the
source value exactly in a common ambient, then adapt onto the target's
representable set. The adaptation is onto the target's set, and by ratified
`the_adaptation_slot_is_derived_and_a_strategy_selects_a_member_per_operation`'s
parent clause the space of total reductions onto a set is derived from that set and
a strategy selects a member. So the member that governs is the target's.

**Option 2, and its stated cost is what the ratified factoring says an operation
into a format does.** The row's cost for option 2 is "a value can be adapted by a
policy its own system never selected", which is a description of every arithmetic
operation under the ratified factoring rather than a defect peculiar to crossings.
Option 1's cost, that "the target's declared policy does not govern values entering
it", is the one the factoring forbids.

One row cuts the other way and it loses on provenance.
`conversion_and_resolution_are_one_obligation_at_two_arities` (`proposal::conversion_and_resolution_are_one_obligation_at_two_arities`)
lists "which reduction governs the loss" as a third thing a crossing must name,
which reads as though it were free. It is `one_expert` and the factoring is
ratified. What survives of it is that the crossing must **say** which reduction
governs, and the answer to what it says is fixed.

### 6.5 The admission contract asks for the ambient domain's own law inventory

`question::what_the_admission_contract_asks_a_candidate_to_expose` (Q29). Option
one's sufficient direction is already refuted by
`an_exposure_test_over_reduction_verdicts_alone_is_satisfied_by_a_system_that_computes_nothing`
(`proposal::an_exposure_test_over_reduction_verdicts_alone_is_satisfied_by_a_system_that_computes_nothing`), a measured row with a control that asks whether the
collapsed verdict can be made to fail at all and finds it cannot.

Between the surviving two, ratified text decides. Option three makes admission
relative to a consumer-supplied ambient domain, so the candidate does not determine
its own identity until a frame is chosen. Ratified
`a_format_is_identified_by_its_ambient_domain_and_its_representable_set` says the
ambient domain is half of what identifies it. A candidate that does not fix its own
ambient domain does not have an identity, and a concept whose members lack
identities is not the concept that sentence ratifies. **Option two.**

### 6.6 A platform-width type is a target-indexed family of formats, which is a fifth option

`question::what_a_platform_width_type_is` (Q26). Its first option, storage rather
than format, is refused by ratified text read carefully. The ratified identity
clause says "a value set that depends on other data is not a format but storage".
A platform-width numeral's value set depends on the compilation target, which is
fixed before anything runs, so per compilation its representable set is a constant
of the type and the exclusion does not reach it.
`each_choice_in_the_sequence_has_an_owner_and_a_resolution_time`
(`proposal::each_choice_in_the_sequence_has_an_owner_and_a_resolution_time`) states the positive form, that it is "a target-indexed family
of formats whose exclusion grounds apply only to dependence that survives to
runtime", and I read that row first, so the positive half is a confirmation and the
refusal of option one is the entailment.

That answer is none of the four listed. It is not a degenerate instance, not an
orthogonal axis, and not a different kind of thing: it is a family of instances of
the same kind, indexed by a parameter resolved at compile time. The option set is
missing its answer, which is 8.3 below.

### 6.7 Wrapping is one member of one derived slot

`question::where_wrapping_lives`. Ratified
`arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation` says "the
adaptation is a first-class object with its own laws". One object, one slot,
members classified by which laws they satisfy. Wrap is a member of that slot and
not an axis value with an exception list, because an exception list is what appears
when a member is filed outside the structure that classifies it.

**Option one**, agreeing with `the_adaptation_slot_is_derived_and_a_strategy_selects_a_member_per_operation`
(`proposal::the_adaptation_slot_is_derived_and_a_strategy_selects_a_member_per_operation`), which I read first. The row's own `because` carries the part
I cannot improve on: expelling wrapping while retaining saturation "has no
criterion that does not empty the slot".

The obligation the row records as owed under every filing equally is still owed: a
wrapped numeral has no arithmetic-compatible order, so the canon owes one sentence
saying which order comparison means there. `law::existence_of_a_translation_invariant_total_order`
is the measurement behind it.

### 6.8 The overflow mode is declared, so the strategy does not fix it

`question::does_warm_wrap_or_clamp` (Q6) asks whether one strategy wraps or
clamps. The premise is that a strategy fixes the mode, and ratified text says the
declaration fixes it.

`ruling::the_overflow_panic_is_permitted_and_bounded` (`ruling::the_overflow_panic_is_permitted_and_bounded`), which
is ratified in op's own words and at his own request kept as loose as he stated
them, says that on a release build "what stands in its place is the guarantee plus
an explicit declaration of the mode, saturate or wrap or whatever applies to the
overflow or underflow at hand, which then lowers and behaves accordingly". The
mode is a declaration that lowers, and the axis of that ruling is the build rather
than the strategy: panic on debug, in every strategy, "even on hot"; declared mode
on release.

So the question dissolves, which is its own third option, and it dissolves along a
different seam than the one that option names. What is left of it is two smaller
things, and both have answers:

- **What the default declaration is where a consumer writes nothing.** Ratified
  `warms_objective_is_the_intuitive_best_choice` (`ruling::warms_objective_is_the_intuitive_best_choice`) says being a
  Rust crate makes Rust's way "the baseline for intuition", and a native Rust
  primitive panics under debug assertions and wraps without them. So the Warm
  default is panic on debug and wrap on release, and the ratified preset table's
  clamp cell is stale, which op has already said in
  `ruling::warm_behaves_as_a_native_rust_primitive_would`'s successor. **Derived**,
  from a ratified sentence about intuition plus a fact about the baseline it names.
- **Whether the mimicry survives measurement.** Ratified text supplies the escape:
  mimicry "does not make it absolutely required, if mimicking is consistently just
  worse choice". Whether it is worse is what op's own deferral
  `wrap_or_clamp_stays_open_and_both_get_priced` (`ruling::wrap_or_clamp_stays_open_and_both_get_priced`) sends to the
  bench, in his words "Option 1 but see previous answer too", with `instead`
  reading "It goes back to the bench: both readings are written and both are
  priced, and the measurement decides rather than he does". **That is not my
  question and it is not answerable without the harness.**

The standing caveat on the row's evidence survives all of this and should travel
with any future reading: the clamp-family arity crossover at `20` section 1.5 is
self-flagged as contaminated, written after reading a commit subject carrying its
own conclusion, and is owed an independent read that has not run.

## 7. Derived from the intent, inside its spirit, and marked as derived

Everything here is weaker than section 6. Each says what it rests on and what
would overturn it. Where it agrees with an existing `one_expert` row I say so and
the agreement adds nothing, because I read the row first.

### 7.1 Two words for membership and hosting, with hosting scoped to a target

`question::one_word_or_two_for_is_a_number_system` (Q31). The row's own note gives
the decisive test and answers it: whether the canon ever says something true of a
system arvo cannot host, "and it already does, since the bounded windows it admits
are defined as bounded windows of systems it cannot host". Ratified
`membership_of_the_representable_set_is_one_affine_predicate` makes every
representable set a bounded window over some ambient domain, and the unbounded
domains those windows are windows of are not hostable. So one word is refused.

Between two words and two words with the second scoped to a target, 6.6 decides
it. A platform-width numeral is a target-indexed family, so it is hostable at one
target and not at another, and an unscoped hosting predicate would have to call it
both. **Option three**, and the cost the row names, a quantifier over compilations,
is what the platform-width case needs anyway.

Agrees with `membership_and_hosting_are_two_questions` (`proposal::membership_and_hosting_are_two_questions`) on the
split, which I read first. The target-scoping half is where this goes further than
that row, and it goes there on 6.6 rather than on taste.

### 7.2 Admission is a location for membership and a predicate for hosting

`question::is_admission_a_predicate_or_a_location` (Q30). This is Q31's shape one
level down and it takes Q31's answer: two questions, two procedures. Membership
asks which coordinate of the dependent sequence a candidate fixes, which is a
location, because ratified text makes identity a pair of coordinates and locating
a candidate on them is what deciding membership is. Hosting asks what a value at
rest may carry on a named target, which is a predicate returning yes or no.
**Option three.**

The row's note is the supporting evidence and it is measured rather than argued:
most of the disputed cases, a Gray code, two's complement and a stride, are
coordinate choices rather than rejections. Agrees with
`admission_returns_a_coordinate_rather_than_a_verdict` (`proposal::admission_returns_a_coordinate_rather_than_a_verdict`),
read first.

### 7.3 Soundness is the canon sentence and tightness is an arm

`question::is_the_derived_numeral_required_to_be_tightest`.
`probe::the_design_closed_forms_are_not_the_tightest_numeral` measures the design's
natural closed forms tight on 1099 of 1296 cases for multiplication, 1175 of 1296
for addition and 751 of 1296 for the subtraction candidate. So a canon sentence
claiming tightness would be false today, and option two cannot be the canon
sentence as the design stands.

Option three, say nothing about tightness, is refused for a different reason: a
canon must be able to say which things are doable, and a derived numeral that is
not guaranteed to hold every value the operation can produce is not usable by
anything. Soundness is the minimum a canon can say and still be a canon.

**So option one is the canon sentence, and bestness is an arm wherever a tight
form is derivable**, which is what `ruling::arms_over_regions_are_the_fundamental_heart`
asks of every result that holds somewhere and not everywhere. The two admissions
option two needs, the origin shape and negative integer width, then belong to the
arm's predicate rather than to the canon, which is where an admission that only
some regions need should sit. Derived, and the derivation is the arms ruling
applied to a measured fraction.

### 7.4 The cross-kind join stays unclosed and priced

`question::is_the_cross_kind_join_closed_or_priced`.
`probe::closing_the_family_under_intersection_is_priced_and_does_not_reach_tapered_formats`
measures the closure at a 16 to 34 percent enlargement, every added shape a
segmented grid neither family names, and the closure not reaching tapered formats
at all. A closure that costs a third family and still leaves a named format class
outside is not a closure of anything a consumer wanted closed.

Closing the space is also the shape `ruling::the_work_is_predicated_arms_composed`
rejects by premise: it is one uniform structural answer bought so that every later
question has a cheap uniform answer. **Option two**, unclosed and priced, with the
join computed where a region admits one and named where it does not.

### 7.5 The singleton amendment is narrowed and not closed

`question::inclusion_order_singleton_amendment` (Q10). 5.4 removes one source of
sub-two-value numerals, the symmetric sign domain at precision one, by choosing the
precision reading that never produces it. It does not remove the source the
question is about, which its own note locates "at the coarsest declared step in
the box": a grid whose declared step is coarse enough that one point fits in the
range. Those are different constructions and I did not measure the second.

So Q10 stays open with a smaller case set, and its third option, that no consumer
reaches such a numeral, is now less plausible rather than more: 5.4 shows the case
arises from an ordinary convention choice rather than from a pathological shape,
which is weak evidence that it will arise again from another. The second read `03`
asked for has still not run and this is not it.

### 7.6 The position-keyed dither arm ships

`question::does_the_position_keyed_dither_arm_ship`. Recorded as answered by the
coordinator in op's stead and marked overturnable in a sentence, so this is the
second independent reading the two-expert rule asks for. I formed it before reading
the coordinator's reasoning, which the row states as the toolbox argument.

`ruling::there_is_no_universal_answer_take_the_win_and_gate_it` carries op's words:
"Take the win where it applies, gate it out from where it does not." The test for
an arm is therefore whether a region exists where it wins, and 5.2 measures one:
at `d = 3, delta = 5` the position-keyed member is exactly monotone on an
increasing ramp where the independent member is not, and the region is computable
from the dropped-bit count and the ramp step alone, so a const predicate can gate
it. Ratified `arms_over_regions_are_the_fundamental_heart` then says the arm ships,
because an arm with a nameable region is what the design is made of.

**It ships**, and I reach it on a measurement rather than on the toolbox rule, so
the two readings do not share a premise. Whether it ships as a default is a
different question and 7.8 does not answer it either.

### 7.7 A consumer-supplied seed surface exists

`question::does_a_consumer_supplied_seed_surface_exist`. Also a coordinator call in
op's stead, also marked overturnable, and this is my own reading formed from canon
text.

`ruling::the_operating_constraints_are_intents_and_rules` (`ruling::the_operating_constraints_are_intents_and_rules`) is
`in_force` and lists **no platform dependency** among constraints that "are not to
be questioned". Sourcing entropy is a platform dependency. So arvo cannot produce
randomness, and a stochastic member is either fed from outside or is not
stochastic.

`ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names` is ratified
by op and fixes the mode vocabulary at six names, one of which is `stochastic`.
A design with no seed surface has a mode named `stochastic` that is deterministic,
which makes one of six ratified names denote something it is not. **So the surface
exists**, and it exists because the in-force constraint and the ratified vocabulary
cannot both hold without it.

The second option's cost as the row states it, "picks determinism for every
consumer", understates what it costs: it does not pick determinism, it renames it.

### 7.8 Both keyings ship, each gated, and the question should not have been asked

`question::do_arvos_consumers_want_value_keying_or_position_keying`. The row's own
`bound` already says the shape is one op refused, quoting him at length, and names
the third option as the answer ratified `the_work_is_predicated_arms_composed`
already gives. I agree and have nothing to add to the reasoning.

What I can add is the predicate one of the two arms is gated on, which the row says
is what the panel owes. 5.2 supplies part of it: on a sequentially read increasing
ramp, position keying costs between 1.20 and 2.40 times the independent member's
monotonicity failure rate at every cell where either is nonzero, and costs nothing
at all in the cells where the key's increment cannot bridge the ramp's residue gap.
Both halves are computable at compile time from the dropped-bit count and the step.
That is a gate rather than a preference, and it is the shape the answer wanted.

### 7.9 There is no single default rounding position, and the bound that says otherwise over-reaches

`question::why_the_default_rounding_position_is_chosen`. The row's `bound` derives
its answer from ratified `warms_objective_is_the_intuitive_best_choice` and flags
itself as one of the three least certain moves in its pass, with the attack stated:
"the ruling quoted is about one strategy's objective, and reading it as governing a
crate-wide default is a widening."

**The attack lands.** Ratified `the_strategies_weigh_measurements_differently`
(I8) says the strategies measure different things and weigh different measurements
differently, and ratified `each_preset_names_a_stated_intent` (I2) gives four
different intents. A crate-wide default derived from one strategy's objective would
have the other three inherit a default chosen against a weighting they do not
share, which is what I8 says cannot be right. So the widening fails.

**Its failure selects the row's third option rather than returning the question.**
If the default cannot be crate-wide it is per concern, which is what
`ruling::the_work_is_predicated_arms_composed` predicts and what the row records
neither candidate having written down. Within the intuitive-compromise strategy the
bound's own reasoning then holds unchanged and picks familiarity as the baseline
with the measured escape; within a speed-first strategy
`ruling::throughput_wins_are_the_speed_first_strategys` picks differently; and the
row's sharp observation, that the IEEE default is the one mode not free under
either signedness, becomes a fact about which arms are reachable at each default
rather than an argument for one global choice.

So the question is answered without op, and the part the bound worried was his
turns out to be the part that dissolves.

### 7.10 The concept fixes coordinates and their order, and commits to no count of levels

`question::are_the_level_hierarchies_the_same_cut` (Q19). The question offers a
three-level cut and a five-level cut and asks whether one refines the other. Its
note names the instrument that would decide it, applying each cut's own change-test
to the other's levels, and the change-tests are not in the registry, so I could not
run it.

What ratified text does decide is narrower and enough. The ratified identity
clause commits to which coordinates exist and to what each decides: the ambient
domain and the representable set identify, adaptation choice and encoding are
realisation. It commits to no grouping of them and to no count. A level is a
grouping, so a count of levels is a claim about a partition rather than about the
concept, and two different partitions can both be faithful to the same coordinates.

So the answer is that the question has no answer of the shape it asks for, and the
concept commits to its choices and their order and to nothing about levels. This
agrees with `the_concept_commits_to_its_choices_and_to_no_count_of_levels`
(`proposal::the_concept_commits_to_its_choices_and_to_no_count_of_levels`), which I read first, so the conclusion is confirmed
rather than corroborated. The route is not shared: that row argues from two
instruments producing partitions that do not refine each other, and this argues
from what the ratified sentence commits to, which is a different premise reaching
the same place.

### 7.11 The role concept is closed and the role inventory is open

`question::is_the_role_set_closed` (Q23). Ratified text settles the same fork one
level up, at `the_concept_is_closed_and_the_inventory_is_open`, and the reasoning
transfers exactly: a closed inventory of roles would put every new role through a
canon amendment, and an open concept of a role would make the word mean nothing.

The concept is available and is already written:
`roles_derive_representations_and_a_realisation_variant_computes_nothing_new`
says roles "differ in who re-establishes the invariant", which is a definition
rather than a list. So the concept closes on that definition and the inventory of
roles satisfying it stays open, at storage, compute and interchange and whatever
else supplies it.

**The proposed fourth is refused as a category error rather than voted down.** The
same row says a chain's extent "is not a role at all but a schedule", and
`conversion_and_resolution_are_one_obligation_at_two_arities` independently makes
schedules a property of an expression rather than of a representation. A schedule
does not re-establish an invariant, so it cannot be a role under the definition
that closes the concept.

Derived by transferring a ratified sentence one level down, and the transfer is the
weak link: nothing ratified says the role axis is the same kind of thing as the
system concept. If somebody wants to attack this, that is where.

### 7.12 Interoperation is one obligation at two arities, which is none of the three words offered

`question::is_interoperation_conversion_or_resolution` (Q27). The option set is
three bare words with no elaboration, which the row's own note admits: "each is
named and none is described, which is as symmetric as the source allows".

Ratified `arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`
answers it directly and the answer is not among the three. Moving one value into a
declared system is an exact operation of arity one in a common ambient composed
with an adaptation onto the target's representable set. Combining several values
from disagreeing systems is the same sentence with the arity raised. The three
things each owes are identical, and 6.4 fixes the third of them.

So this is an entailment rather than a derivation, and it agrees with
`conversion_and_resolution_are_one_obligation_at_two_arities`, read first. The
useful output is not the agreement, it is that the option set cannot hold the
answer, which is 8.2.

### 7.13 The ownership key is adopted, and its stated cost is already paid

`question::the_ownership_key_as_a_structural_axis` (Q35). Ratified
`never_a_runtime_check_and_one_lowered_path` makes what resolves before runtime the
axis the whole design turns on: "invalids are caught at compile time and unused
paths are cleared when lowered". Resolution time is therefore already load-bearing,
and ownership is the other half of the same fact, since a choice cannot resolve
without something owning it.

Two of my own results need the key to be stateable at all. 6.6 makes a
platform-width numeral a family owned by the compilation and resolved at
monomorphisation, and 5.3 makes the footprint an axis resolved at const time. Both
are ownership-and-resolution-time claims and neither is expressible without the
key.

**Adopt.** The row's stated cost, that every per-value canon sentence acquires an
implicit whose-components rider, is real and is not a new cost: the canon already
requires every sentence about numerals to name the prefix it quantifies over, per
`every_canon_sentence_names_the_prefix_it_quantifies_over` (`proposal.toml`, the
`canon_form` row following the sequence). A rider that a discipline already demands
be explicit is not implicit.

Agrees with `each_choice_in_the_sequence_has_an_owner_and_a_resolution_time`
(`proposal::each_choice_in_the_sequence_has_an_owner_and_a_resolution_time`), read first.

### 7.14 Name all five crossing classes, and owe an order only where the crossing loses

`question::does_the_canon_name_crossing_classes` (Q37, first half) and
`question::when_is_an_order_owed_at_a_crossing` (Q37, second half).

**Five names.** The argument is the predicate discipline rather than taste. A
crossing's law obligation differs by which coordinate moves, and ratified text
makes some of those coordinates identity and others realisation, so a sentence
about "a crossing" that does not say which coordinate moved is quantifying over a
set whose members obey different laws. Under `ruling::a_predicate_lists_only_what_holds`
such a sentence claims nothing anybody can gate on. Naming the classes is what
makes the sentence writable, and the cost the option lists, five names plus a rule
that a composite names its intermediate, is the cost of being able to say anything.

**An order only where lossy.** Where a crossing loses nothing, composition commutes
with the endpoints, so an order names a choice with one outcome. Under
`ruling::arms_over_regions_are_the_fundamental_heart` that is an arm over an empty
region, which is not an arm. Where it loses, the two candidate orders give
different answers and something outside the typestate has to say which, and 6.4
says which. So the second option, and the third option's global order is refused
because 6.4 makes the answer depend on which endpoint is the target.

Both agree with `an_order_is_named_exactly_where_a_crossing_is_lossy`
(`proposal::an_order_is_named_exactly_where_a_crossing_is_lossy`) and `a_crossing_carries_two_relations_and_a_verdict_per_law_family`,
read first. What 6.4 adds is closing that row's own recorded gap.

### 7.15 Mixed-numeral addition exists, and its existence has a cost the register should carry

`question::mixed_numeral_addition` (Q3). Two independent things refuse the first
option, that no such addition exists.

Ratified `arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`
makes it expressible without any new machinery: lift both operands into a common
ambient domain, add exactly there, adapt onto a declared target representable set.
Nothing in that sentence requires the operands to have come from one format, and
7.12 is the same construction at arity two.

And `obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`
is an adequacy test rather than a preference: a convention that cannot be written
as an alias is a gap in the primitives. Both conventions op named carry an addition
across differing formats, so a design in which no such addition exists cannot write
either alias and fails the test.

Between the second and third options, `ruling::there_is_no_universal_answer_take_the_win_and_gate_it`
gives both: the explicit conversion is always available, and the inferred form
exists wherever the target numeral is derivable by a const predicate. That is a
composition rather than a winner, and it is what the arms ruling asks for.

**The cost, which I would rather state than let a consolidation discover.** The
row's note records that unit two's one unconditional result rests on this question
staying open, because "under the second option addition aligns scales, alignment is
a shift, and a shift is the coarsening that kills multiplication". My answer admits
the inferred form in a gated region, so that unconditional result becomes
conditional on the gate, and somebody has to restate it with the predicate the gate
supplies. I did not do that work and it is real.

### 7.16 The shape space is not a lattice, so no design may rely on a computed join

`question::one_numeral_family_or_several` (Q8). Op's instinct is one family and he
said explicitly not to act on it, so the instinct is not evidence and I have not
used it.

`probe::closing_the_family_under_intersection_is_priced_and_does_not_reach_tapered_formats`
measures the join of a fixed-point numeral and a float numeral as having two
incomparable minimal upper bounds and no least one, and no admission of more
fixed-point shapes repairing it, on two instruments by two authors where the second
attacked the first's scope. That is the whole structural content available, and it
settles what a design may rely on: not a computed unique join, at any point,
anywhere.

The question as posed asks for a universal structural verdict over the whole shape
space, which is the shape `ruling::the_work_is_predicated_arms_composed` rejects by
premise. What survives of it is the third option's content, reached from the
measurement rather than from a taste for totality: stop requiring a unique least
upper bound, return the set of minimal upper bounds, and let a stated rule pick
where a caller needs one. The row's own note records that the no-determiner cell is
empty and coherence says it must be, so the rule has few callers and each can name
its own.

**I did not find the fifth option**, a route to one family surviving all three
conditions. I looked for one for about as long as it was honest to, from the
step-set reading of the fourth option, and the step-set relation is nested for the
fixed-point family alone and is not nested across the float boundary, which is the
same incomparability in different words. So the fourth option looks to me like a
restatement of the measured obstruction rather than a way past it, and I am
reporting that rather than proposing it.

### 7.17 The width surface wants the alias to carry the const and the algebra to key on nats

`question::the_width_surface_crossing` (Q9). **This is the least certain thing in
this file** and I would rather it be attacked than adopted.

Three constraints narrow the seven options and none of them is a preference.

`obligation::a_primitive_for_every_position_a_bare_number_would_take` carries op's
own words, "No bare usize other than in const generics for smoother and more
ergonomic api, and even there, only when truly painful otherwise". The const
generic position is the excepted one, so an option that removes the const surface
entirely is not required by that constraint and an option that keeps it is not
forbidden. What the second clause does forbid is a spelling chosen when a less
painful one exists, and option two's hand-written binary digit tower is the painful
one by any reading.

`obligation::the_unstable_machinery_does_not_reach_a_consumer` says "A consumer
naming arvo's types needs no unstable compiler features, no nightly-only attributes
and no feature gates of its own, whatever arvo uses internally to build them", and
names `kolli` as having stated it as a property adoption rests on. Any arrangement
whose consumer-facing spelling needs an unstable feature is refused outright rather
than priced.

The row's own sharpest finding is that the ceiling belongs to neither the bridge
nor the const surface but to the price of crossing back, and that a table is the
only known implementation of either direction.

Those three select the fifth option: a consumer-facing type alias carries the const
parameter, the numeral is keyed on structural nats, and the bridge fires once at
the alias so the algebra never re-enters it. The const surface survives where op
excepted it, the consumer writes a literal rather than a tower, the unstable
machinery stays inside, and the crossing that is expensive happens once per named
type rather than once per operation.

**Why I am unsure, stated so a later seat starts from it.** The bar I reasoned
from was written about bare primitives at API positions, and reading it as
governing how a width literal becomes a type-level natural is a widening of exactly
the kind I rejected at 7.9. It may be one widening too far. The sixth option, where
the literal is what a consumer sees and the structural nat is a hidden projection,
satisfies every constraint I named equally well, and I separate it from the fifth
only on the crossing-back argument, which is the row's finding rather than mine and
which I did not re-measure. A seat that measures the crossing-back price on the two
arrangements would settle it, and nothing else I can think of would.

## 8. Option sets that cannot hold their own answers

A question retired as wrongly posed is as good an outcome as one answered, and
four of these are wrongly posed in a way that will survive being answered, because
the option array is what a consolidation compresses rather than the prose beneath
it.

**8.1 The eight at 3.4 ask for one policy over a category.** Three of them already
carry a `bound` saying so, and the option arrays still read as live forks. The
repair is not an answer, it is rewriting each option set as a region and a
predicate, which is what `ruling::there_is_no_universal_answer_take_the_win_and_gate_it`
asks and what `never-ask-which-single-rule-governs` in the workspace rules states
as a reflex. I have not rewritten them, because an option set is the source's and
editing one is not a reader's call.

**8.2 Q27's three options are three bare words and the answer is a fourth thing.**
`is_interoperation_conversion_or_resolution` offers "Conversion", "Resolution" and
"Neither", each named and none described, which the row's own note concedes. 7.12
shows ratified text gives an answer that is none of the three and that the third
option, "neither", is true only in the uselessly weak sense of not being the first
two. The row should carry the unification as an option or carry no options.

**8.3 Q26's option set is missing its answer.** `what_a_platform_width_type_is`
offers storage, a degenerate instance, an orthogonal axis, and a different kind of
thing. 6.6 shows the answer is a family of instances of the same kind indexed by a
parameter resolved at compile time, which is none of the four. The row's own note
already warns that the first option is "named once, in passing, in a file about a
different topic", which is a warning about weight rather than about completeness,
and the completeness problem is the larger one.

**8.4 Q4's third option is not a rival to its first.**
`what_a_datum_stands_for` lists "a constructor-level clause" alongside "a point",
"an absorbing top" and "a set". 6.3 shows the third is a diagnosis of why the
question reads ambiguously and is compatible with the first, so listing it as a
fourth alternative makes the space look wider than it is. Op explicitly refused to
bound this option set, which is a refusal to say what is admissible and not a
statement that everything listed is a rival.

## 9. What I could not settle, and where the wall is

**`question::what_the_double_rounding_mechanism_is` (Q57). I concede it, and the
concession is structural rather than a failure of effort: the question cannot be
answered from the canon, because the sentence whose reading it disambiguates was
never admitted to the canon.**

The row asks "which reading of 'the grid' does the double-rounding clause carry
once grids chain?", and offers two options described only by how many cells they
differ on, 32 to 94 in one comparison and 124 to 170 in the other. Neither option
says what either reading is.

What I tried, in order, with what closed each:

1. **The registry, in the words the answer would use.** Grepped `double rounding`,
   `staged`, `direct`, `narrowing`, `chained grids`, `the grid` and `reading`
   across all twelve registry files. The phrase appears in
   `law::double_rounding_is_innocuous_at_an_intermediate_width_between_f_and_2f`,
   which is the closed half and states that no intermediate width strictly between
   `F` and `2F` makes double rounding innocuous at any `F`. That is a different
   claim from the one in question and does not name either reading.
2. **The clause itself.** Grepped for the arms as the corpus paraphrases them:
   `defer.*root`, `range part`, `at every node`, `leaves the grid`. **One hit in
   the entire registry, and it is inside Q57's own `note`.** So the clause that the
   two readings are readings *of* has no row anywhere in the canon.
3. **The probe corpus**, which the brief licenses as greppable. Found
   `131_probes/v2_the_vocabulary_and_which_equality_double_rounding_threatens.py`,
   which is the instrument that located the problem, and read its header. It
   describes the clause as belonging to `122` section 4.6 and paraphrases it as an
   equality between two arms that both round at every node, one applying the range
   part at each node and the other deferring it to the root while applying the grid
   part at every node whose result leaves the grid. It does not state the two
   readings of "the grid" either, because at the time it was written the ambiguity
   had not been found.
4. **`167_probes/doubleround/doubleround.rs`**, which is a clean exhaustive
   instrument on the adjacent question and shows per-operation correct rounding not
   composing. Its case that must fail is present and correct. It is about
   staged-versus-direct narrowing, which the row's note already says is what is
   actually at risk, and not about the ambiguity.
5. **One `awk` range over `OPTIONS.md`** covering Q57's heading alone, which is the
   nearest thing to a source the brief permits me. It adds that "once grids chain,
   4.6's 'the grid' is genuinely ambiguous" and repeats the two cell counts. It
   still does not state either reading, and it points at `122` section 4.6, which
   is a numbered member file I may not open.

**The wall, stated exactly.** The two readings exist only inside an unratified
member file. The registry is the canon by `mockspace.toml:31`, and under the
provenance ladder a member file is agent output presumed wrong where it conflicts
with the canon. So there are two ways to answer Q57 and both are bad: read the
member file, which means resolving a canon question by reasoning from the tier the
canon-design-code chain declares dead, or guess the readings from their cell
counts, which is manufacturing an answer to fill the slot.

**What would move it, and it is one act rather than a research question.** Port
the clause. A `law` or `proposal` row stating the equality `122` section 4.6
claims, with its two arms written out, would make the ambiguity a question about a
row in the canon, and only then is a fourth construction worth building. Until
then a fourth construction has nothing to reproduce, which is why the three that
were tried were wrong: they were reproducing a paraphrase.

**What I am not doing, deliberately.** The row's `decider` is `measurement` and its
note invites a fourth construction starting from the three dead routes. I could
build one. It would be a construction over a sentence I reconstructed from a
paraphrase of a file I may not read, and it would produce a number, and the number
would enter the record. That is the shape
`conceding-is-an-answer-and-expert-code-is-a-spike` names: an artifact produced in
service of an answer, mistaken for the answer. So the honest output is this
paragraph and the port request above.

**One further thing about that row, which is not the concession.** Its two options
are not options. "The first reading, which differs from the second on 32 to 94
cells" and "The second reading, which differs from the first on 124 to 170 cells"
name nothing a reader can choose between, and the asymmetry of the two counts is
the only content either carries. Under
`record-the-options-a-decision-chose-among` an answer recorded against that set
would say a choice was made and not which, which is the failure that rule exists to
prevent. So the row needs restating whether or not anybody answers it.

## 10. The roster

All thirty-seven, with the tier each answer sits at. The tier is what matters for
what happens next: `ratified` needs nothing, `entailed` needs nothing because it is
an application of a ratified sentence, `measured` needs a second instrument if
somebody doubts it, `derived` needs a second independent expert before it goes
anywhere, and `confirmed` adds nothing to the row it agrees with because I read
that row first.

| question | topic | verdict | tier | section |
|---|---|---|---|---|
| `adaptation_in_identity_or_realisation` | number system | realisation, qualified to denotational identity | ratified | 4 |
| `are_the_level_hierarchies_the_same_cut` | number system | no count of levels is canon | confirmed | 7.10 |
| `is_the_number_system_inventory_open` | number system | open, concept closed | ratified | 4 |
| `is_number_system_broad_enough_for_non_magnitude` | number system | broad | entailed | 6.2 |
| `are_set_valued_carriers_admitted` | number system | outside the format concept, placed as compositions | entailed | 6.3 |
| `is_the_role_set_closed` | number system | concept closed, inventory open, chain extent refused | derived | 7.11 |
| `is_interoperation_conversion_or_resolution` | number system | none of the three; one obligation at two arities | entailed | 7.12 |
| `what_the_admission_contract_asks_a_candidate_to_expose` | number system | option two, with the ambient law inventory | entailed | 6.5 |
| `is_admission_a_predicate_or_a_location` | number system | location for membership, predicate for hosting | derived | 7.2 |
| `one_word_or_two_for_is_a_number_system` | number system | two words, hosting scoped to a target | derived | 7.1 |
| `is_the_ambient_operation_family_fixed` | number system | a parameter | entailed | 6.2 |
| `the_ownership_key_as_a_structural_axis` | number system | adopt | derived | 7.13 |
| `whose_reduction_governs_a_lossy_crossing` | number system | the target's | entailed | 6.4 |
| `does_the_canon_name_crossing_classes` | number system | name all five | derived | 7.14 |
| `when_is_an_order_owed_at_a_crossing` | number system | only where lossy | derived | 7.14 |
| `what_then_validate_requires` | format | all three readings | ratified | 4 |
| `which_width_coordinates_a_consumer_writes` | format | definitional half closed, surface half open | ratified | 4, 3.3 |
| `mixed_numeral_addition` | format | exists; both the inferred and explicit forms, gated | entailed | 7.15 |
| `one_numeral_family_or_several` | format | not a lattice; minimal upper bounds and a stated rule | derived | 7.16 |
| `the_width_surface_crossing` | format | the fifth option, and it is my least certain | derived | 7.17 |
| `inclusion_order_singleton_amendment` | format | narrowed, still open | partial | 7.5 |
| `is_the_derived_numeral_required_to_be_tightest` | format | soundness in canon, tightness as arms | derived | 7.3 |
| `does_precision_count_the_sign_digit` | format | it does not, at radix two | measured | 5.4 |
| `is_the_cross_kind_join_closed_or_priced` | format | unclosed and priced | derived | 7.4 |
| `what_a_platform_width_type_is` | format | a target-indexed family, a fifth option | entailed | 6.6 |
| `does_narrowing_compose` | rounding | a predicate on the mode; both arms exist | entailed | 6.1 |
| `what_the_double_rounding_mechanism_is` | rounding | conceded; the clause is not in the canon | conceded | 9 |
| `why_the_default_rounding_position_is_chosen` | rounding | no single default; per concern | derived | 7.9 |
| `does_the_position_keyed_dither_arm_ship` | rounding | it ships | derived | 7.6 |
| `does_a_consumer_supplied_seed_surface_exist` | rounding | it exists | derived | 7.7 |
| `does_the_rounding_variance_form_hold_at_a_second_fraction_width` | rounding | yes in ulp, no in absolute units | measured | 5.1 |
| `do_arvos_consumers_want_value_keying_or_position_keying` | rounding | both, gated, with part of the gate measured | derived | 7.8 |
| `does_the_position_keyed_members_monotonicity_failure_rate_differ_from_the_independent_members` | rounding | they differ, and not in one direction | measured | 5.2 |
| `does_warm_wrap_or_clamp` | overflow | the mode is declared; the premise fails | entailed | 6.8 |
| `where_wrapping_lives` | overflow | one derived slot | entailed | 6.7 |
| `what_a_datum_stands_for` | primitive | a point | entailed | 6.3 |
| `the_container_premise` | container premise | observable, and observable at const time | measured | 5.3 |

Three ratified, nine entailed, four measured, fourteen derived, one confirmed, one
partial, one conceded, and four option sets flagged for restatement in section 8.

## 11. The alternatives I found and did not take

Listing these is most of what a next seat can use, because a dispatch that found
four routes and shipped one has thrown away three quarters of what it learned.

**On the container premise, an occupancy axis rather than a const axis.** 5.3 makes
the footprint gateable through `size_of`. The other way to reach the same place is
to declare occupancy, sole against shared, as a `dimension` row, which
`at_shared_occupancy_no_per_element_footprint_observation_exists` says it wants and
does not create, filing the condition in its sentence instead. That route makes the
answer a region rather than a const read, and it is better if the shared-occupancy
half turns out to need arms of its own. I did not take it because declaring an axis
is not a reader's call and because the const route needed no new vocabulary. A seat
that builds a packed column and measures whether a per-element size observation
exists would settle which route is right, and that probe does not exist.

**On the variance forms, a distributional route rather than a moment route.** I
computed the second central moment three ways. A fourth route computes the whole
distribution of the scaled error and compares distributions rather than moments,
which would catch a coupling that matched both moments and differed in shape.
Nothing in the question needs it, and it would cost an exponential enumeration
where the moment route is polynomial, so I stopped. It is the route to take if
somebody wants a claim about the error's tail rather than its variance, which the
chain topic may eventually want.

**On the monotonicity comparison, a different dither.** I measured the golden-ratio
additive recurrence because that is the one `129_probes/x1` used and comparability
demanded the same construction. Every low-discrepancy sequence with a fixed
increment has the same two-value increment structure, so the result at 5.2
generalises to that whole family by the mechanism rather than by measurement. A
dither whose increment varies, a bit-reversal permutation or a scrambled sequence,
would not, and I did not run one. That is the obvious next arm and it is cheap: the
probe takes a key function as a parameter already.

**On the precision reading, a route through the affine parameterisation.** Instead
of building the three sign domains and ordering them, one could ask which reading
makes the ratified affine predicate's phase term behave, since a sign domain is a
choice of where the grid sits relative to zero. I tried this first and abandoned it
because the ratified clause fixes the parameterisation and not the convention for
counting its digits, so the phase term is silent about the question. That is a dead
route and it is dead for a reason worth knowing: the ratified spine does not reach
naming conventions at all.

**On Q8, the step-set reading.** The fourth option says two numerals are in one
family exactly when their admitted step sets are nested. I chased it and it
collapses into the same obstruction: step sets are nested within the fixed-point
family and are not nested across the float boundary, which is the incomparability
the closure probe already measured, restated in the vocabulary of steps. I report
it as a restatement rather than as a route, and I could be wrong about that, which
is why it is here rather than deleted.

## 12. What I noticed that nobody asked me about

Reported harshly where it deserves it, per the standing instruction.

**The settled-question checker guards one of the two edges that close a question,
and the file it lives in states the failure it half-guards.**
`checks/tests/a_settled_question_does_not_sit_in_the_queue.rs:82` walks rulings
only. Three questions in my scope alone are closed by ratified propositions and
render as open. The module doc at the top of that file is a careful, correct essay
about exactly this failure mode. Writing the essay and then guarding one edge is
worse than guarding neither, because the essay is what a reader trusts instead of
checking.

**The registry contradicts itself about whether a general rounding law is
writable**, between `proposal::staged_narrowing_disagrees_with_direct_narrowing_under_round_to_nearest_even`'s `note` and `probe::narrowing_composes_where_the_modes_direction_switches_at_coarser_grid_points`. One of them is stale
and no check can tell which, because nothing cross-reads a `note` against the rows
it describes. That is the same class as
`no_note_describes_a_row_it_is_not_on.rs`, which exists and does not reach this.

**Nothing audits an `answers` edge's scope.** 3.3 shows one reaching past its own
`says` by three of a question's four options. Every mechanism in this registry
audits predicates and regions; an edge is a claim about what a sentence settles and
it escapes all of them, which is the same shape as the prose-versus-predicate gap
the ratification gate's own note names.

**Four axes the corpus reasons about are not declared and cannot be written in a
predicate**, so every finding about them is silently unpredicated: the dropped-bit
count of a narrowing (mine at 5.1, and `proposal::staged_narrowing_disagrees_with_direct_narrowing_under_round_to_nearest_even` records the same gap
for a staged narrowing's intermediate width), occupancy (recorded by
`at_shared_occupancy_no_per_element_footprint_observation_exists` as wanted and not
created), precision (5.4), and the ramp geometry at 5.2. Under
`ruling::a_predicate_lists_only_what_holds` an undeclared axis converts the
notation's strongest negative statement into a shrug, which `predicate.rs:12` says
in its own words.

**`dimension::signedness` declares two values and the corpus writes three.**
`proposal::the_laws_of_a_format_are_derived_from_two_hypotheses_rather_than_enumerated_per_policy` and `proposal::the_law_frame_was_attacked_from_another_topic_and_held` both carry "three sign domains,
unsigned and signed two's complement and signed symmetric range" on the values
side, which the declared grammar does not admit. The values side is unchecked by
construction, so nothing catches it, and 5.4's answer had to be written in the same
inadmissible dialect to say anything at all.

**Three question rows carry no topic**, listed at the count in section 2. A
topicless question is invisible to any roster built by topic, which is how every
roster here is built.

## 13. Provenance of this file

Nothing here was ratified by anybody. Sections 4 and the `ratified` rows of the
roster report what ratified text already says and are as strong as that text.
Sections 6 and the `entailed` rows are applications of ratified sentences and stand
or fall on whether the entailment is valid, which is a thing to attack rather than
to count instances of. Sections 5 and the `measured` rows rest on four probes
committed in `222_probes/` with their outputs and a `RUN.md`, each with its
controls printed firing; they are one instrument each and one instance decides
nothing, so a second instrument on any of them is worth having and I have named
what it would be. Everything at `derived` is one expert's reading, which is mine,
and needs a second before it goes into a consolidation.

Where I agree with an existing `one_expert` proposal I read that proposal first,
and I have marked every such case `confirmed` rather than letting it read as a
second instance. Agreement reached by starting from somebody else's answer is
confirmation and it does not move a standing.

**One thing I did wrong and repaired, recorded because the repair is the point.**
The first draft of this file cited registry rows by line, about thirty times.
`checks/tests/no_line_citation_into_the_registry.rs` exists precisely for that, its
ceiling on member files is 45, and its assertion message says "Brief the next seat
to write slugs; do not raise this". I would have pushed it well past the ceiling.
Every citation here now names a row slug, and the two remaining line citations
point at `mockspace.toml`, which that check's own control establishes is not a
registry file.

The mechanism proved itself while I worked rather than in the abstract: five of my
line citations were already wrong when I checked them, because rows had moved under
me between reading and writing. A slug cannot go wrong that way, which is the whole
argument and it took an afternoon to feel.

---

# Appendix. Reply to 221

Written after reading `221_dolan_the_numeric_fundamentals.md` and its probes, merged
into my worktree from `origin/research/fundamentals-221`. Everything above this line
was committed before I read a word of it.

**One correction before anything else, because the coordinator's brief said the
file was on `research/canon-registry` and it is not.** That branch carries `219`
and `220` and no `221`. The file is on `origin/research/fundamentals-221`, which is
where I merged it from. Worth saying because a reader following the brief would
have fetched a branch, found nothing, and had to guess whether the seat had landed.

## A. What this appendix changes above, so nobody acts on the superseded half

**Section 6.4 is withdrawn.** I answered `whose_reduction_governs_a_lossy_crossing`
with "the target's" and called it an entailment of the ratified factoring. The
entailment does not reach that far and B.1 says exactly where it stops. The row is
unresolved and 221's concession on it is the right answer.

**Section 7.16 is superseded, not withdrawn.** My statement that the shape space is
not a lattice is true and hides the structure. 221 measured the structure and B.2
carries it.

**Section 7.5 is superseded.** I left `inclusion_order_singleton_amendment` partial.
It is answerable and 221 answers it; B.3 concedes the answer and C.1 attacks the
evidence offered for it.

**Section 3.3's residue on Q2 is closed.** I said the surface half was settled by
nothing I could find. It is settled, by a bound I had read and failed to apply, and
B.4 concedes it.

**Section 7.17's reasoning is replaced and its conclusion is now a composition.**
D.1 carries the measurement, which refutes 221's decider and mine.

Everything else above stands.

## B. Where 221 is right and I was not

### B.1 I over-reached on Q36, and the over-reach is one word

My 6.4 argued: ratified `arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`
makes a crossing an exact operation composed with "a named, total adaptation onto
the representable set", the set is the target's, so the target's reduction governs.

**The ratified sentence fixes the codomain and says nothing about the selector.**
All three of the row's options adapt onto the target's representable set; what they
disagree about is whose policy picks which member of the adaptation slot. The row
that names a selector is `proposal::the_adaptation_slot_is_derived_and_a_strategy_selects_a_member_per_operation`,
which says a strategy selects a member per operation, and it is `one_expert` and
does not say whose strategy. So there is no ratified text on the question I claimed
to have entailed an answer to.

221 declines to pick and gives the reason I should have weighed: MATLAB `fimath`
attaches `OverflowAction` and `RoundingMethod` to the `fimath` object rather than
to either operand's `numerictype`, which is the third option, a policy named at the
site. Under `obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`,
an adequacy test rather than a preference, a design in which the target's policy
always governs cannot write that convention as an alias without the alias carrying a
policy neither endpoint holds. **That is evidence against my answer and I had the
obligation in front of me at 7.15, where I used it, and did not carry it one section
further.**

**Concession in terms: 6.4 is withdrawn. What survives is one sentence, that the
codomain of a crossing is the target's representable set, which is an entailment and
is not what the question asks.** The question is the panel's and open.

### B.2 Q8 has structure and I reported its absence

I wrote that the shape space is not a lattice and that the step-set reading is the
same obstruction in different words. Both are true and neither is the useful thing.

`221_probes/p2` computes the order over 47 denotationally distinct points and gets
1003 unique joins of 1081, 1032 unique meets, and this:

```
every pair, bounds drawn from the whole space   pairs 1081  join fails 78  meet fails 49
constant pairs, bounds from the whole space     pairs  703  join fails 55  meet fails  0
constant pairs, bounds from the constant space  pairs  703  join fails  0  meet fails  0
```

**The constant-quantum family on its own is a lattice, and putting the float points
into the space breaks the join on 55 pairs that are both fixed-point.** Those pairs
did not change; their bounds did. So the join failure is a property of the space
rather than of the pair, which is the opposite of what "several families" would
predict, and the cost of one family is paid inside the kind that was already fine.

That is a real result and I did not have it. My section 7.16 said "no design may
rely on a computed unique join, at any point, anywhere", which is now visibly too
strong: inside one kind a design may rely on it entirely.

**On the step-set reading, our verdicts agree and its diagnosis is better than
mine.** I said nesting fails across the float boundary, which is the obstruction
restated. 221 says nesting of step sets is what inclusion already is on these
points, so the criterion is the order rather than a partition of it. That is why the
option cannot work at all rather than why it fails on these particular points, and
it is the sharper statement.

**One thing about its predicate that is right and worth repeating rather than
attacking.** It writes `signedness: signed: construction` and then says in the same
breath that it has not run the instrument a `construction` token obliges under
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`, so the entry
should be read as unmarked and the claim as holding at signed only. **That is the
warrant discipline working on the day it landed**, and its item 1 is the repair. Its
section 5 numbers hold at `signedness = signed` and nowhere else until that runs.

### B.3 The singleton amendment is answerable and I left it partial

My 7.5 narrowed `inclusion_order_singleton_amendment` and left it open, on the
ground that the sub-two-value numeral I found at 5.4 is a different construction
from the one the row is about. That part is right and I stand on it.

What I did not do is attack the order directly. 221 did, and its C5 arm reports that
removing the degenerate points makes 256 of 630 pairs inside one kind have no meet
at all. **That refutes the row's third option outright**, which says a numeral
carrying fewer than two values is not a case any consumer reaches: the order needs
the bottom whether or not anybody declares one, so the case is reached by the
algebra rather than by a consumer.

I reproduce both of its cells exactly in `222_probes/a6_the_two_repairs_separated.rs`,
0 and 256, which is the control that says my copy is its instrument.

**Concession: the row is answerable, option 3 is dead, and option 1 is the answer.**
I agree with the answer. C.1 is about the evidence rather than the conclusion.

### B.4 Q2's surface half was settled by a bound I had read

I wrote at 4 that the surface half is "not settled by anything I can find". It is
settled by `obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`,
which I quoted at 7.15 for a different row and did not apply here.

The argument is 221's and it is right. The obligation is an adequacy test, so the
question is which surface pair the two named conventions are aliases over, and both
answer total and fraction. **I checked the two documentary facts rather than taking
them, which makes this a second instance on them:** MATLAB's `fi` is parameterised
by `WordLength` and `FractionLength`, `numerictype` carries no integer-length
property and the integer part is derived from the other two and the signedness; IEEE
754 interchange formats are parameterised by `k` with `k = w + p`, and conformance is
stated over a value encoded in exactly `k` bits. Both correct.

And its `fi(v, 1, 8, 12)` example is the load-bearing part: word length 8, fraction
length 12, integer width `8 - 12 - 1 = -5`, and MATLAB accepts it as an ordinary
declaration. **So the integer-and-fraction surface has to name a coordinate that
takes negative values in an object that has no negative widths**, which is a sharper
statement of the cost than the row's own "every reflective surface must choose which
pair it shows".

### B.5 `overflow policy = panic` names something that is not an overflow policy

221's finding and I did not make it, which is worse than not having thought of it,
because I quoted the sentence it follows from. My 6.7 cites ratified
`arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation` for the
"one slot" answer, and the same sentence says the adaptation is **total**. Panic
diverges, so it is not a member of the slot, so a predicate span writing `overflow
policy = panic` is saying something about the build profile in the overflow
vocabulary. `dimension::build_profile` is the axis that says it, and ratified
`the_overflow_panic_is_permitted_and_bounded` reaches the same bound from the other
side.

I had the clause, used it for one consequence, and stopped. **Conceded, and it is
the cleanest example in either file of a ratified sentence carrying more than the
seat that quoted it took.**

### B.6 Q3's option 3 is refuted as worded, and the cost I flagged dissolves

My 7.15 answered "both, the inferred and the explicit forms, gated". 221 refutes
option 3 on its own sentence: it says "a consumer converts one operand into the
other's numeral first", and MATLAB's `SumMode = 'FullPrecision'` goes to a numeral
wider than both, computed from both, because the result needs a carry bit neither
operand has. **So the target of the conversion is not either operand, and what I
called the always-available explicit form is option 2 with the target written by
hand rather than inferred.** The composition I offered is still a composition; one
of its two arms was misnamed.

And the cost I flagged at the end of 7.15, that unit two's unconditional result
becomes conditional, dissolves under its observation that **alignment to a join is a
widening and widening loses nothing**. The coarsening happens on the way back down
to a declared output width, which is a separate declared act. That is better than my
"somebody has to restate it with the predicate the gate supplies", and it means
nobody has to.

## C. Where 221 is wrong

### C.1 Its evidence for the singleton amendment measures a repair it never varied

I agree with its answer, per B.3. What follows is about the evidence, and it is
decidable because its probe is committed and I ran it.

Its `catalogue_with(degenerates: bool)` makes admitting the bottom a parameter and
its C5 arm varies it. **The deduplication by denotation sits unconditionally in the
same function and nothing varies it.** Its file says "the deduplication is what
makes C1 pass", and its own `p2d` diagnosis of the 256 says the opposite: "every one
of them has an EMPTY intersection, zero have several maximal lower bounds", which is
the bottom being absent rather than duplicates being present.

`222_probes/a6_the_two_repairs_separated.rs` copies its catalogue machinery verbatim,
lines 63 to 270 of its core, changes one thing, and runs the two-by-two. **Both of
its cells reproduce exactly, 0 and 256, which is the control that says my copy is its
instrument:**

```
degenerate  dedup  points  pairs   fail | no bounds  annihilated  distinct      same
      true   true      47    703      0 |         0            0         0         0
      true  false      54    861     28 |         0           28         0         0
     false   true      45    630    256 |       256            0         0         0
     false  false      52    780    284 |       256           28         0         0
```

Three mechanisms, not two, and the third is the one nobody had named:

- **256 is the bottom being absent.** Every one of them has no lower bound at all.
- **28 is the copied helper.** `maximal` keeps `i` when no `j` has `c[i].vals` a
  subset of `c[j].vals`, and two indices denoting one set are each a subset of the
  other, so **both are dropped and the maximal set comes back empty**. Running its
  instrument without deduplication is therefore not running the same measurement; it
  is running one whose helper functions are undefined on the input.
- **Zero failures anywhere are several maximal lower bounds with distinct
  denotations**, which is the shape a genuine order failure would take.

So the number offered as evidence for deciding inclusion on denotation is a number
about admitting the bottom, and varying the other repair produces a fact about the
instrument rather than about the canon. **The answer survives and the argument for
it has to be the algebraic one:** without deduplication the relation is a preorder,
two declarations denoting one set are each below the other, neither is the meet, and
"unique least upper bound" is unique only up to an equivalence nobody named. That
argument needs no measurement and 221 did not make it.

My first version of that split got it wrong in the same family and is kept beside it
as `a6_v1_the_split_conflated_no_bounds_with_annihilated_bounds.txt`: it reported all
28 as empty intersections, which cannot be right with the bottom present, and the
four-column split is what the correction produced.

### C.2 Q26 is decided against 221 by a row it does not cite

221 answers "storage, not format, and the ratified spine says so in the words the
option uses", quoting: "a value set that depends on other data is not a format but
storage". My 6.6 answers a target-indexed family of formats.

**`proposal::each_choice_in_the_sequence_has_an_owner_and_a_resolution_time` speaks
to exactly this and says the exclusion does not reach it**: "a platform-width numeral
is a target-indexed family of formats whose exclusion grounds apply only to
dependence that survives to runtime". 221 cites that row nowhere in its file, and
its own discipline everywhere else in section 6 is to confirm the one-expert row that
answers a question.

The substance rather than the citation: **a type is a compile-time object, and within
one compilation a platform-width numeral's representable set is a constant of the
type.** Across compilations it is a different type with the same spelling. "Depends on
other data" in the ratified clause is a statement about a value set that varies with
data the program is holding, which is what makes it storage; a value set fixed before
the program exists is not that. Read 221's way, the clause classifies by the spelling
of a type rather than by what varies at run time.

**What its reading gets right and mine has to answer.** If a platform-width numeral
is a format, then the format concept contains a family whose member is chosen by
something outside the program, and every canon sentence about "a format" acquires a
per-compilation quantifier. That is a real cost and it is the cost
`the_ownership_key_as_a_structural_axis` is about, which is why 7.13 adopts the key.
**Under 221's reading the cost disappears and so does the ability to say anything
about `obligation::a_platform_sized_unsigned_integer_at_an_api_position`**, a live
consumer demand: calling the thing storage says what it is not and nothing about
which laws it has, and the consumer asked for a primitive rather than for a
classification.

### C.3 Two of its nine concessions were passable, and I passed them

221 concedes `does_the_rounding_variance_form_hold_at_a_second_fraction_width` with
"I could not run it without knowing what the variance forms are, and they live in a
numbered member file I may not open", and
`does_the_position_keyed_members_monotonicity_failure_rate_differ_from_the_independent_members`
with "the two members' definitions are in files I may not open". It then diagnoses
both as "a dispatch problem rather than a research one" and recommends sending a seat
with reading rights.

**Neither wall is where it thought.** The brief licenses the probe corpus as
greppable, and both objects are in it:

- The variance forms are in `128_probes/r3_output.txt` and `130_probes/y1_output.txt`,
  which print `n^2 f(1-f)` and `n f(1-f)` against enumerated values at nine chain
  lengths between them. My 5.1 is built on those two outputs and no member file.
- The two keying members are in `129_probes/x1_output.txt` and its script, which name
  the shared-threshold, independent-per-element and position-keyed golden-ratio
  variants and report the 7-of-40 count. My 5.2 is built on those and no member file.

So the recommendation to dispatch a seat with reading rights for those two rows is
answering a wall that is not there, and **both rows are measured now.** The
recommendation may still be right for Q57, where the wall is real and G says so.

**I do not think this reflects badly on 221 and it is worth saying why.** It searched
for the definitions and concluded they were behind a rule; I searched for the numbers
and found them printed. That is a difference in what we went looking for rather than
in diligence, and the lesson generalises: **a committed probe output is a source a
blind seat may read, and it frequently carries the thing the member file states.**

### C.4 It had the tool for Q22 and Q4 and did not use it

221 concedes `are_set_valued_carriers_admitted` and `what_a_datum_stands_for`, and
its finding about them is a good one: the two rows price the same admission
disjointly and neither cites the other.

Two sections earlier, at `is_the_cross_kind_join_closed_or_priced`, it argues that
ratified `membership_of_the_representable_set_is_one_affine_predicate` says membership
is "one affine predicate over one parameterisation, of which the named kinds are
points", and that a set whose membership is not one affine predicate is outside the
concept rather than a new inventory entry. **That argument settles Q22 verbatim**, and
my 6.3 is it: an interval is not a point of that parameterisation, so a set-valued
carrier is not a format, and `the_format_concept_carries_three_things_upward_and_compositions_owe_their_own_laws`
says where it goes instead.

**Its own caveat on that argument does not bite here**, which is why I can use it
where it could not use it on the join. It marks the reading as turning on whether "a
quantum per magnitude" means an arbitrary function of magnitude or a parameter of the
affine slot function. An interval carrier's elements are pairs rather than scalars, so
it is not a point of a parameterisation of scalar grids under either reading of the
quantum. **The Q22 use of the clause is unconditional where the closure use is
conditional.**
