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
`ruling::the_panel_finishes_the_canon_without_him` (`ruling.toml:1626`), which
says every remaining canon question is the panel's, that a question filed as op's
is now derived from what he has already said, and that nothing is parked awaiting
him. Deriving answers to open question rows is exactly what that licenses. Two
things in the brief's own framing I did have to test and both held: the registry
is the canon by `canon_paths` in `mockspace.toml:32`, and the questions were
ported without answers by that file's own stated policy, which
`question.toml:17` records in the header.

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
`predicate.rs:19` states outright that the values side of a predicate is not
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
`rung = "ratified"`, `ratified_by = "experts"` at `ruling.toml:1544` in the tree I
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
(`ruling.toml:1185`) carries op's verbatim answer, names the row in its `answers`
edge, and the question's own `note` says "Recorded as answered at `28` batch one".
So the field's absence marks rows the schema's newer answer mechanism has not
reached rather than rows nobody has answered. Reading the count as thirty-seven
open questions overstates it, and by more than one.

## 3. Four structural findings, which change what several of the questions are

These come first because three of them move questions out of my answer list
before any derivation starts, and the fourth changes the shape of eight more.

### 3.1 Three questions are closed at the governing tier and the queue cannot see it

`ruling::the_format_spine_is_canon` (`ruling.toml:1436`) is ratified, marked
`ratified_by = "both"`, and carries a `ratifies` list of four propositions. Three
of those four carry `answers` edges, and every one of the three lands in my
assignment:

| ratified proposition | line | closes |
|---|---|---|
| `a_format_is_identified_by_its_ambient_domain_and_its_representable_set` | `proposal.toml:65` | `adaptation_in_identity_or_realisation` (Q18) |
| `membership_of_the_representable_set_is_one_affine_predicate` | `proposal.toml:82` | `which_width_coordinates_a_consumer_writes` (Q2) |
| `the_concept_is_closed_and_the_inventory_is_open` | `proposal.toml:576` | `is_the_number_system_inventory_open` (Q20) |

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
(`proposal.toml:1579`) answers `does_narrowing_compose` at `standing =
"three_or_more"` with an exact predicate and a named instrument. That is not a
ratified proposition, so it does not close the question the way the three above
do; what it does is make the question's binary option set false, which is section
6.1.

### 3.2 A live self-contradiction in the registry, about the general form of a rounding law

`proposal.toml:1603` names the instrument
`07_probes/p4_composition_and_forced_adjoint.py` and line 1604 says of it, in
the same sentence, "and no `probe` row names it, so the general form is not
writable and what is written is the one instance whose instrument is
registered". The preceding line says, in bold, "**The general rule behind it is
in the corpus and cannot be written here.**"

`probe.toml:982` is a row with `id =
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
(`ruling.toml:909`) carries op's words: "we don't need to settle for one
universal solution, it's the anti-pattern I've already named. Case by case ...
Take the win where it applies, gate it out from where it does not. No single
one-fits-all solutions, it's impossible." The ratified
`ruling::the_work_is_predicated_arms_composed` (`ruling.toml:847`) and
`ruling::arms_over_regions_are_the_fundamental_heart` (`ruling.toml:1459`) say
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
at `proposal.toml:256`.

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
`ruling::validate_means_all_three_readings` (`ruling.toml:1185`) carries op's
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
walked and no `dimension` row declares it; the same gap is already recorded at
`proposal.toml:1616` about a staged narrowing's intermediate width, so this is a
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
(`ruling.toml:892`) is the sentence that makes it decisive rather than a
curiosity: under it, an axis available at const time is a predicate, and under
`ruling::never_a_runtime_check_and_one_lowered_path` (`ruling.toml:949`) reading
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
third domain as "signed symmetric range" (`proposal.toml:153` and
`proposal.toml:1220`) and does not say which construction that is. Two answer to
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
