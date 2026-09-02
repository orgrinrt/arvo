# 236. Seat A: which of the format topic's proposals are ratifiable today

Seat 236, derived independently. The question is per row: for each `proposal`
whose `topic` is `the_format`, does it follow from the ratified rulings and op's
stated ones, is its evidence committed and real, and does anything in the canon
contradict it.

**There are seven such rows. Three of them are already canon and were before
this dispatch was written, so the question does not apply to them. Of the four
that remain, none is ratifiable today, and each fails for a different reason
that I name and, where I could, attack rather than only report.**

## 0. The two gates, and what my evidence is

**Canon gate: passed.** Measured against `mock/registry/*.toml`, which
`mockspace.toml` declares as `canon_paths`. I read all
95 `ruling` rows in full, the seven `the_format` proposals in full, the fifteen
`the_format` retirements, the three `the_format` obligations, the two
`the_format` laws, the ten `the_format` questions, the `dimension` rows for
every axis those predicates name, and the three probe rows behind the two
measured proposals. Nothing in the dispatch asks for work the canon forbids.
The dispatch does rest on one false premise, which section 1 corrects.

**Test gate: passed with one infrastructure note.** `cargo mock test` runs nine
trees. Seven are green. The bench tree fails on four manifests, and all four
carry a `FIXME` in their own `Cargo.toml` saying they have not built since
2026-08-08, why, and that they are kept because the panel cites their committed
results. That is a marked state rather than a regression. The lints tree built
and ran 603 tests green on one invocation and failed to compile on two others
with `E0308`, from two mockspace checkouts, `a9268f6` and `b4e0c7a`, resolving
into one dependency graph. `mockspace.toml` pins `mockspace_branch = "dev"` and
says the launcher re-resolves the branch head periodically, so that is the pin
moving under a warm build cache rather than a defect in the registry work.
`cargo mock --lint-only --strict` is green over all 700 rows with five
pre-existing style warnings in the shipped crates.

**I read the bodies rather than the names.** The lints whose predicates decide
this question are `a_standing_is_reachable_from_what_it_cites`,
`an_imposition_rests_on_no_instrument` and `a_region_agrees_with_the_sentence_kind`,
and I read all three end to end. They are not decorative: each carries genuine
negative controls, each states in its own documentation what its unit tests
cannot ask, and the first two name the exact widening this file finds. Section 5
reports a hole in the second, found by reading it.

**What my evidence is, stated before any of it is used.** Two instruments,
committed beside this file with their raw output.

- `236_probes/p2_survey.sh` is a read over the committed registry. Every
  zero-returning check in it carries a positive control immediately after,
  because a zero from a pipeline is a claim about the pipeline until the
  pipeline has been shown able to return something else. Its control caught a
  real defect in itself on the first run: under a `#!/usr/bin/env nutshell`
  shebang `$0` is the interpreter rather than the script, so every path resolved
  under `~/.local/bin` and check 5 reported all eight artifacts absent including
  the seven that are present. Fixed to `BASH_SOURCE` and the fix is in the file
  with the reason.
- `236_probes/p1_phase_across_width_and_fraction.rs` is a sweep I built for
  section 4. Its four cases that must fail are stated in its header before the
  run and all four fire.

**Nothing here is a claim about what op wants.** He has left the canon work, and
`ruling::the_panel_finishes_the_canon_without_him` is ratified: every remaining
canon question is the panel's, derived from what he has already said. No
question in this file is routed to him.

## 1. The dispatch's premise is wrong by three rows, and I checked it first

The brief asks which of the format proposals are ratifiable, which presumes none
of them is. **Three of the seven have been canon since the ratification round at
`213`.**

`ruling::the_format_spine_is_canon` sits at `rung = "ratified"` with
`ratified_by = "both"`, and its `ratifies` list names four proposals:

> Four propositions become canon together and they are the format topic's spine.
> A format is identified by its ambient domain and its representable set, and
> that set is a constant of the type. Membership in it is one affine predicate
> over one parameterisation, of which integers, fixed point, scaled integers and
> floats are points. Arithmetic on a format is an exact operation in the ambient
> domain composed with a named total adaptation onto that set, and the adaptation
> is a first-class object with its own laws. The concept is closed and the
> inventory of admitted instances is open.

Three of the four are `the_format` rows. The fourth,
`proposal::the_concept_is_closed_and_the_inventory_is_open`, is filed under
`the_number_system`.

**The header of `proposal.toml` says the opposite and is stale.** It states that
a proposal becomes canon only when a ruling names it under `ratifies`, and then:
"Nothing does." Eleven rows are named under `ratifies` today, across four
ratified rulings, and `236_probes/p2_survey.sh` section 2 lists all eleven with
the ruling and rung that carries each. That same header already warns, two
paragraphs down, that "a header describing a state the file has passed is worse
than no header, because it is read as current by everyone who does not think to
test it". It is now describing one itself.

**This is a finding about the file rather than about the work.** The three rows
are canon whatever the header says, because the edge is what carries the
ratification and the edge resolves.

## 2. The bar I measured against

Two ratified rulings fix it and I quote both rather than paraphrase.

`ruling::two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`,
`rung = "ratified"`, `ratified_by = "op"`, in his words:

> So let's change the wording so that if two separate experts agree on it heavily
> and can reason and spell it out, that is a ratification for canon, though
> distinct from something I've blessed. But ratification never the less and
> should be treated as such

Its `note` bounds it in two directions that both matter here: it is "not
automatic promotion on a count of agreeing experts", the coordinator still
gating, and it is not permanent, a promoted proposition staying open to change.

`ruling::the_additive_and_absorption_verdicts_are_canon`, `rung = "ratified"`,
`ratified_by = "experts"`, states what the gate actually reads, in its
`promotion` field:

> Promoted on the gate rather than on the count. ... Neither prose reaches past
> its predicate, which is the thing this gate is for and which the third proposal
> read alongside them failed.

**So the gate is two things and both are necessary.** Two independent instances,
and prose that does not reach past its predicate. The second half has already
refused one row in this corpus:
`proposal::no_multiplicative_structure_survives_a_nonzero_fraction_width` carries
a `gate` field reading "Refused promotion, and it is the prose rather than the
finding", on a row standing at `three_or_more`. **A count does not buy past the
prose test, and that precedent is what decides row seven below.**

The third thing binding every row here is
`ruling::arms_over_regions_are_the_fundamental_heart`, ratified, `ratified_by =
"both"`, in op's words:

> Option 1 is exactly the paradigm. We fill the full space, but instead of one or
> a couple of general statements that are uniform across the dimensions, we have
> small arms and spans that work where they are optimal, and nowhere else. It's
> the fundamental heart within arvo.

## 3. The seven rows

`236_probes/p2_survey.out` section 1 is the table this reads from: id, standing,
sentence kind and the count of distinct files the row's `provenance` names. Every
one of the seven names exactly one file.

### 3.1 `arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`

**Out of scope: already canon**, ratified by `ruling::the_format_spine_is_canon`
as quoted in section 1. Nothing to decide.

### 3.2 `a_format_is_identified_by_its_ambient_domain_and_its_representable_set`

**Out of scope: already canon**, same ruling, same edge.

### 3.3 `membership_of_the_representable_set_is_one_affine_predicate`

**Out of scope: already canon**, same ruling, same edge.

### 3.4 `the_adaptation_slot_is_derived_and_a_strategy_selects_a_member_per_operation`

**Not ratifiable. Three blockers, and the second is the one that matters.**

**The count.** `standing = "one_expert"`. The bar in section 2 is two independent
instances and this row has one. Its own `note` is honest about which half has
what:

> The slot-is-derived claim is one author's. The two-family inhabitation beneath
> it is a separate measurement by a different author with an independent
> instrument, and it is carried as the law row this points at.

**Two halves at two standings inside one row is not two instances of the row.**

**The kind is wrong on the third sentence, and that is a silent widening.** The
row is `sentence_kind = "normative"`, which carries no region by design. Its
third sentence is "All four combinations occur", which is a claim an instrument
could return the other answer to, and one did: the row's own `law` edge points at
`law::coherence_of_a_reduction_onto_its_induced_operation`, whose `holds` and
`fails` both carry `total_width: W = 4` and `fraction_width: F = 0`, and whose
`witness` reads "Signed saturation holds the adaptation laws and fails coherence
at 476 chain-divergent triples". The registry's own header states the test: "A
claim that could be measured false is not `normative` however definitional its
grammar, and it carries the region it was established in or it is not here at
all." `an_imposition_rests_on_no_instrument`'s documentation names the cost
exactly: "Filing a measured claim as an imposition silently widens it from the
model width it was established at to everywhere, and it does that without
touching the predicate, which is where the widening would have shown up."

**So the row states at every width what was established at four bits.** Under the
notation that is the strongest possible overreach, and the check that exists for
it does not see this row. Section 5 says why.

**The granularity clause is ambiguous in the direction op has refused three
times.** "a strategy selects a member per operation" reads two ways. That
selection happens at each operation site rather than once per format, which is
compatible with everything. Or that the operation is the granularity of
selection, which fixes one policy over a whole category, and
`ruling::there_is_no_universal_answer_take_the_win_and_gate_it` is op refusing
exactly that shape, in terms:

> Take the win where it applies, gate it out from where it does not. No single
> one-fits-all solutions, it's impossible

Under `arms_over_regions_are_the_fundamental_heart` the granularity is whatever
region a const predicate names, which can be finer than an operation or coarser.
**I am not claiming the row means the forbidden reading. I am reporting that
nothing in it decides which, and a canon sentence whose scope two readers supply
differently is the instability I13's predicate discipline exists to remove.**

**What would land it.** Split the measured clause out of the row and give it the
law row's region. Get a second independent arrival on the derived-slot clause,
which is the cheap one, because the claim that the space of total maps from an
ambient domain onto a representable set is determined once that pair is fixed
follows from the ratified identity row by construction. Then say which
granularity the selection clause means.

### 3.5 `the_format_concept_carries_three_things_upward_and_compositions_owe_their_own_laws`

**Not ratifiable. One blocker on the count and one live disagreement it would
settle by accident.**

**The count, and the row says so itself.** `standing = "one_expert"`, and its
`note` reads: "One expert, uncontested rather than corroborated. Absence of
contest is not a second instance, and the consolidation does not claim it is."
**That is the row conceding the gate.** Seven files passing over a claim without
attacking it is not a second derivation, and the author of the row is the person
who wrote that sentence.

**Its obligation is unreached.** The row carries `obligation =
["composition_contracts_above_the_numeral"]`. `cargo mock obligation-coverage
composition_contracts_above_the_numeral` reports `tier: proposed`, with ten
proposals naming it and nothing ratified reaching it. So the row is the tenth
proposal pointing at an obligation no canon carries, which is not against it, but
it does mean nothing above it constrains what the three exported things have to
be.

**And it would settle a live disagreement without arguing it.** The row names
"the width algebra of exact results" as the first of three things a format hands
upward. `proposal::the_reachable_interval_is_the_true_grade_and_width_is_its_lax_abstraction`
is a live measured row in this same corpus reporting that the width rule
`g(W,V) = max(W,V) + 1` is not associative, that the reachable interval is what
composes exactly, and that the slack between them grows from zero to two bits
over fold lengths one to six. **Promoting the three-things row fixes width as the
exported quantity while a committed instrument says width is not the quantity
that composes.** Neither is canon, so this is not a contradiction with the canon.
It is two live rows disagreeing inside one topic, and ratifying one of them is
how a disagreement gets closed by filing rather than by argument.

**What would land it.** A second independent derivation, and the width against
reachable-interval question resolved on its own terms first. The second is the
larger of the two and it is a real question rather than a formality.

### 3.6 `a_nonzero_phase_leaves_the_representable_set_without_an_additive_identity`

**Not ratifiable as written. The finding is real, its instrument is committed and
sound, and its stated region is wrong in three places. I fixed two of them and
name the third.**

**The count.** `standing = "one_expert"`.

**The rounding entry names no member of the ratified vocabulary.**
`ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names`, ratified
by op, fixes the vocabulary at `toward_zero`, `floor`, `ceil`, `half_up`,
`half_even`, `stochastic`. This row's predicate writes `rounding = nearest,
against a phase-zero mutant`. `cargo mock rounding-vocabulary` reports it under
"Names a distinction it does not make", with the reason: "`nearest` does not say
which way a tie goes, and the two ways are separate members of the six."
`question::which_tie_direction_an_unqualified_nearest_names` is open, its
`decider` is the panel, and its `note` names this row as one of the two it is
about. **A row whose predicate names a mode the canon does not carry has nothing
to gate on, which is the whole purpose of a predicate.**

**The fraction width is not what the instrument ran.** `56_probes/q2_affine_membership.rs`
declares `SCALE: i64 = 32`, `STEP: i64 = 8` commented "1/4 scaled", and `BIAS:
i64 = 4` commented "1/8 scaled". So the quantum is `2^-2` and the phase is
`2^-3`. `dimension::fraction_width`'s own grammar says "`F = 0` names the
exact-integer case". **The instrument ran at a quantum of one quarter, and the
row's predicate claims the exact-integer case.** That is not a narrowing, which
would be safe; it is a different cell.

**What the instrument does establish, and it is sound.** `probe::a_half_step_biased_grid_is_not_closed_under_addition`
sits at `standing = "sound"` and its control is real: "The affine membership
predicate is written once and instantiated at two phases, and each instantiation
is compared exhaustively against a direct enumeration that never mentions the
predicate." I opened the source and the output. `q2_output.txt` reads "exact sums
of hub points landing on the hub grid: 0 of 256", "every exact sum sits exactly
half a step from the hub grid (tie): true", "hub grid contains zero: false
contains one: false", "mutant predicate (bias dropped) detected: true", "mutant
rounder (phase-zero target) fails retraction on hub: true". Both files are
committed at `00fe52db`.

**So the blockers are in the region rather than in the finding, and a region is
something an instrument can fix.** Section 4 is that instrument.

**What would land it**, beyond the second arrival: the corrected region in
section 4, and a `dimension` row for phase, which does not exist.

### 3.7 `raw_order_agreement_holds_for_monotone_encodings_not_only_unsigned`

**Not ratifiable. Its prose reaches past its predicate, and the registry already
holds the precedent that refuses that.**

**The count.** `standing = "one_expert"`, on one probe at one width.

**The prose test.** The row's `says` claims raw-compare order agreement "holds
for exactly the monotone encodings of a value set, not only for plain unsigned".
Its `predicate` is `total_width: W = 4` and `signedness: signedness = signed`.
**A claim about exactly which encodings of any value set have a property, on
evidence from four bits and four named encodings, is a universal standing on a
single cell.** The gate in section 2 refuses that shape by name, and it has
already refused it once, on
`proposal::no_multiplicative_structure_survives_a_nonzero_fraction_width`, whose
`gate` field reads:

> The statement claims the result holds "for any policy, sign domain, range or
> rescale spelling", and the predicate carries `total_width: W in 3..=7`. A
> measured band of five widths is not any range, and under the predicate
> discipline the predicate is what holds, so the sentence reaches past its own
> evidence.

**That row stood at `three_or_more` and was still refused. This one stands at
one.**

**The row concedes it.** Its own `note`: "One probe, one width; offered as a
refinement to a wider claim rather than as its own general theorem. The general
statement (raw-order agreement holds for exactly the monotone encodings) is
stated as the reading that explains the one measured instance, not itself
independently swept."

**What would land it, and there are two routes rather than one.**

The first is to narrow the statement to what was measured: at four bits over a
signed set, offset binary is bijective onto the same value set as two's
complement and its raw compare agrees with value order, so raw-compare
sortability lives on the encoding axis rather than the value axis. That is a
small honest claim, its instrument is committed
(`55_probes/p3_encoding_is_a_separate_axis.rs` and its output, present at
`e1b863bc`), and its control is a mutant offset at `K = 7` that is detected.

**The second is better and is available today: the general half is not a
measured claim at all.** Raw compare gives value order exactly when the pattern
map is order-preserving, and order-preserving is what monotone means. Nothing
could return the other answer, so under the registry's own test the general
sentence is imposed rather than established, carries no region, and belongs at
`sentence_kind = "definition"` or `"normative"` with the measured instance kept
separately at its own predicate. **Splitting the row that way loses nothing and
removes the defect, and it is one edit, on their row rather than mine.**

**And the substantive content is not at risk either way**, because
`law::no_bijective_signed_encoding_has_both_raw_order_and_raw_adder_correctness`
already carries the interesting half at `signedness = signed`, `total_width: W
any`, on two probes in a different file by a different persona, with a uniqueness
argument in its `note` rather than only a sweep: a monotone bijection between two
finite totally ordered sets is unique, the probe constructs it and confirms it is
offset binary, and offset binary fails the adder property by the constant `K`.

## 4. What I built, and the region it earns

`236_probes/p1_phase_across_width_and_fraction.rs`, with `p1_output.txt` beside
it. It exists because
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` is ratified and
says what a width-free claim owes:

> a `construction` entry obliges the row's `evidence` to name an instrument that
> varied that axis and found no movement.

**No instrument in the corpus varies width or fraction width on the phase
finding.** `q2` runs one grid. So no warrant on either axis was available, and
that is what this probe supplies.

**The model.** Arithmetic is exact in integer units of `Q/6`, where `Q` is the
quantum, so phases of zero, one sixth, one third and one half are all integers
and no rational type and no rounding of the model itself enters. A grid point of
slot `k` at phase `p/6` sits at `6k + p`. The quantum enters only as `Q = 2^-F`
and the width only as the slot count `N = 2^W`, and both are swept rather than
argued away.

**The four cases that had to fail, stated in the file's header before the run.**

- `C1`. At phase zero the closure claim must be false. **It is**: sums do land on
  the grid, ten of them at `W = 2` and thirty-six at `W = 3`.
- `C2`. At a one-third phase the closure claim must still hold and the half-step
  distance claim must be false. **Both**: no sum lands on the grid at any cell,
  and `every_sum_half_step` reads false at every cell. **This is the control the
  original instrument could not run**, because it swept one phase, and it is what
  separates "no sum lands on the grid", which any nonzero phase gives, from
  "every sum sits exactly half a step away", which only the half phase gives.
- `C3`. A rounder targeting the phase-zero grid must fail retraction on the
  biased grid. **It does, at every cell**, reproducing `q2`'s control rather than
  trusting it.
- `C4`. The monotonicity arm must be able to report false. A deliberately
  non-monotone rounder is run through the same arm and **is caught at every
  cell**.

**Result one, the closure half.** Over `W in 2..=8` crossed with `F in 0..=6`, at
a half-quantum phase: no exact sum of two grid points lands on the grid anywhere,
every sum sits exactly half a quantum from the lattice, and the grid contains
neither zero nor one. Forty-nine cells, all three verdicts uniform.

**Result two, the tie rule, which answers the open question for this row.** At
the half phase every exact sum is a tie, so this is the cell where a tie rule is
maximally observable rather than one where it never fires. Under `half_up` and
under `half_even` the four adaptation laws, total, retraction, monotone and
distance minimising, all hold at every cell of the same cube. **So the
adaptation-law half of the finding does not read the tie rule.**

`question::which_tie_direction_an_unqualified_nearest_names` offers three
options and its `note` names a fourth it says neither reaches: "An instrument
sweeping a domain that produces no tie has measured something true of both rules
and named one, so the honest repair there is the wider statement rather than a
pick." **For this row the wider statement is right and the note's reason is not
the reason.** Every sum here is a tie, so the rule fires constantly, and the four
laws still hold under both. The repair is the wider statement because the laws
are tie-rule-independent, not because no tie occurred.

**The corrected region, appended here rather than written onto their row**, per
the rule that a predicate is never widened in place and the correction goes in
the later deliverable.

For the closure half:

```
total_width: W in 2..=8: swept, one grid per cell of the W by F cube
fraction_width: F in 0..=6: swept, quantum 2^-F
signedness: signedness = unsigned
operation: operation = add
arity: arity = 2
```

For the adaptation-law half, the same, plus:

```
rounding: rounding in {half_up, half_even}: exhaustive, both tie rules over every
          integer in the window at every cell
```

**Three things this does not claim, said here rather than left to be found.**

- **`stochastic` is the sixth ratified mode and the probe did not run it**, so it
  is absent from the rounding entry, which under the notation says the arm holds
  at no stochastic rounding at all. That is the honest reading and it is not a
  hedge: a rounder that is not a function of its input cannot be handed to a
  retraction check as written.
- **`threads` and `target_features` are absent**, so under I13 nothing here holds
  where either exists. The probe is a single-threaded exact-integer computation
  and I did not vary either axis, so writing `any` on them would be the widening
  this discipline exists to stop.
- **The phase itself has no axis to be written on.** `dimension` declares
  twenty-four axes and phase is not among them, so the coordinate the entire
  finding turns on cannot appear in any predicate in this registry. `W` and `F`
  are swept above; `phase = half a quantum` is stated in prose here because there
  is nowhere else to put it. Section 5 carries that as a finding.

**And the width claim stays a sweep rather than becoming a proof.** The argument
that the result is width-free is available and short, since a sum of two points
at `6k + 3` and `6m + 3` is `6(k+m) + 6`, congruent to zero rather than to three,
at any `k` and `m`. **I have not written `construction` on the axis anyway**,
because the ratified marker ruling makes that token a claim about what cannot
enter the argument, and one seat's derivation of a closed form is the thing that
gate is there to distrust. `swept` over a named cube is what the run earns.

## 5. Three findings outside the question, unsoftened

**5.1 Three of the four rows the format spine ratified cannot be checked.**
`arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`,
`a_format_is_identified_by_its_ambient_domain_and_its_representable_set` and
`membership_of_the_representable_set_is_one_affine_predicate` each set `standing
= "two_experts"` and each names exactly one file in `provenance`,
`63_spj_consolidation_the_format_concept`, twice at two line offsets. That is
precisely the state `a-standing-is-reachable-from-what-it-cites` refuses, in its
own words: "One file has one author, so a reader cannot reach the second arrival
from this row."

**The gate is green because the ceiling grandfathers them.** The lint carries
`const CEILING: usize = 29` with the instruction to lower it as topics are
second-read and never raise it, and its own comment says why the population
matters: "Under the rule that two agreeing experts ratify, these are exactly the
rows eligible for promotion, and not one of them names a second source."
**Three of them are no longer eligible for promotion. They are canon.**

Measured independently: `236_probes/p2_survey.out` section 3 reproduces the
population at exactly 29 rows, matching the shipped lint's ceiling, and its
control reports 90 `one_expert` rows that the check must never name and 6
multi-arrival rows citing two or more files that it must not name either.
**Two instruments agreeing on 29 is why I state the number rather than the
predicate.**

This is not a claim that the three are wrong. It is a claim that a reader cannot
reach their second arrival, on three rows that now govern.

**5.2 `an-imposition-rests-on-no-instrument` has a hole and eight rows are in
it.** The lint reads `evidence` and nothing else: its `check_repo` takes
`sentence_kind`, returns unless the kind is `normative` or `definition`, then
`list(ctx.registry, q, "evidence")` and returns if that is empty. **A `normative`
row carrying a `law` edge instead of an `evidence` edge reaches a measured claim
through a different field and the check never sees it.**

Eight rows are in that state, listed in `236_probes/p2_survey.out` section 4.
Two are `the_format` rows and one of those two is canon:
`arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation` is
`normative`, carries `law = ["multiplicative_associativity_under_wrapping"]`, and
carries no `evidence`. The other is
`the_adaptation_slot_is_derived_and_a_strategy_selects_a_member_per_operation`,
section 3.4 above. The remaining six span four other topics.

**The control on that count is the lint's own predicate**, which my script runs
alongside: imposed rows carrying `evidence` come back at zero, against a green
gate, so the script agrees with the shipped lint where the shipped lint looks and
disagrees only where it does not look.

**Whether every one of the eight is a real widening is per row and I have not
read all eight.** What is not per row is that the check cannot see any of them.

**5.3 There is no `phase` axis, and the format spine turns on one.** The ratified
`membership_of_the_representable_set_is_one_affine_predicate` says membership is
"an affine slot function, a quantum per magnitude and a phase, of which integers,
fixed point, scaled integers and floats are points", and adds "The phase is
stated explicitly: a nonzero phase decides whether the identity adaptation ever
occurs and whether the set carries an additive identity at all." **A coordinate
the canon calls out by name as deciding whether a numeral has an additive
identity has no `dimension` row, so no predicate anywhere in this registry can
state which phase a finding holds at.** Section 3.6's row is about phase and
cannot say so; my own corrected region in section 4 has to say it in prose.

`the_axis_set_is_append_only` makes adding one a deliberate, checked act rather
than an edit, which is right. Somebody should make it.

## 6. The answer, in one line per row

| row | verdict |
|---|---|
| `arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation` | out of scope, already canon by `ruling::the_format_spine_is_canon` |
| `a_format_is_identified_by_its_ambient_domain_and_its_representable_set` | out of scope, already canon by the same ruling |
| `membership_of_the_representable_set_is_one_affine_predicate` | out of scope, already canon by the same ruling |
| `the_adaptation_slot_is_derived_and_a_strategy_selects_a_member_per_operation` | not ratifiable: one expert, a measured clause filed as imposed and thereby widened from `W = 4`, and an ambiguous granularity clause |
| `the_format_concept_carries_three_things_upward_and_compositions_owe_their_own_laws` | not ratifiable: one expert and uncontested by its own admission, and it would close the live width against reachable-interval disagreement by filing |
| `a_nonzero_phase_leaves_the_representable_set_without_an_additive_identity` | not ratifiable: one expert, a rounding value outside the ratified six, and `F = 0` where its instrument ran at a quarter quantum. Region corrected and widened in section 4 |
| `raw_order_agreement_holds_for_monotone_encodings_not_only_unsigned` | not ratifiable: one expert, and its prose reaches past `W = 4` to all encodings of any value set, which the gate has refused before on a stronger row |

**The predicate on this file's own findings.** Everything in sections 1, 3 and 5
holds over the registry at `origin/dev` at `0cac9beb`, which is the tree this
seat was cut from, and over no other tree, because the registry gains rows
constantly and every count here is a count over that commit. Section 4's regions
are stated on their own findings and are the only claims here about arvo rather
than about its canon.

**What I could not do.** I am one seat and every verdict above is one instance.
Under the same bar I am measuring these rows against, none of my four rejections
is itself ratified, and the two that turn on judgement rather than on a quoted
rule, the granularity clause in 3.4 and the settle-by-filing argument in 3.5, are
the two most worth attacking.
