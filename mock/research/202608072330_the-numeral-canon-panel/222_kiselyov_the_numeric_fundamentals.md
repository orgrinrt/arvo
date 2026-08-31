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

`proposal.toml:1579` measures the instance: staged narrowing disagrees with direct
narrowing under round to nearest even, first at `-247/16` on a nine-bit signed grid
staged from four fraction bits through two to zero, at `standing = "three_or_more"`
with three named witnesses. `probe.toml:982` holds the general form: narrowing
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
`proposal.toml:1585`.

The blocker recorded at `proposal.toml:1604` is stale, per 3.2, and the general
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
(`proposal.toml:471`) and I read that row first, so my agreement is confirmation.
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
(`proposal.toml:241`) already places them: intervals and error-carrying values are
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
`conversion_and_resolution_are_one_obligation_at_two_arities` (`proposal.toml:411`)
lists "which reduction governs the loss" as a third thing a crossing must name,
which reads as though it were free. It is `one_expert` and the factoring is
ratified. What survives of it is that the crossing must **say** which reduction
governs, and the answer to what it says is fixed.

### 6.5 The admission contract asks for the ambient domain's own law inventory

`question::what_the_admission_contract_asks_a_candidate_to_expose` (Q29). Option
one's sufficient direction is already refuted by
`an_exposure_test_over_reduction_verdicts_alone_is_satisfied_by_a_system_that_computes_nothing`
(`proposal.toml:1490`), a measured row with a control that asks whether the
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
(`proposal.toml:521`) states the positive form, that it is "a target-indexed family
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
not an axis value with an exception list, because an exception list is what you get
when a member is filed outside the structure that classifies it.

**Option one**, agreeing with `the_adaptation_slot_is_derived_and_a_strategy_selects_a_member_per_operation`
(`proposal.toml:98`), which I read first. The row's own `because` carries the part
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

`ruling::the_overflow_panic_is_permitted_and_bounded` (`ruling.toml:1052`), which
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
  `warms_objective_is_the_intuitive_best_choice` (`ruling.toml:606`) says being a
  Rust crate makes Rust's way "the baseline for intuition", and a native Rust
  primitive panics under debug assertions and wraps without them. So the Warm
  default is panic on debug and wrap on release, and the ratified preset table's
  clamp cell is stale, which op has already said in
  `ruling::warm_behaves_as_a_native_rust_primitive_would`'s successor. **Derived**,
  from a ratified sentence about intuition plus a fact about the baseline it names.
- **Whether the mimicry survives measurement.** Ratified text supplies the escape:
  mimicry "does not make it absolutely required, if mimicking is consistently just
  worse choice". Whether it is worse is what op's own deferral
  `wrap_or_clamp_stays_open_and_both_get_priced` (`ruling.toml:1038`) sends to the
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

Agrees with `membership_and_hosting_are_two_questions` (`proposal.toml:422`) on the
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
`admission_returns_a_coordinate_rather_than_a_verdict` (`proposal.toml:479` block),
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

`ruling::the_operating_constraints_are_intents_and_rules` (`ruling.toml:935`) is
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
(`proposal.toml:512` block), which I read first, so the conclusion is confirmed
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
(`proposal.toml:521`), read first.

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
(`proposal.toml:393` block) and `a_crossing_carries_two_relations_and_a_verdict_per_law_family`,
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
satisfies every constraint I named just as well, and I separate it from the fifth
only on the crossing-back argument, which is the row's finding rather than mine and
which I did not re-measure. A seat that measures the crossing-back price on the two
arrangements would settle it, and nothing else I can think of would.
