# Arvo's next steps, rebuilt on a reading of the canon

This is the fourth part of job `arvo-canon-check`. It takes what `CHECK.md` found
stale and what `ANSWERS.md` found answered, and lays arvo's next work out as
steps a helper can take one at a time. Two plans were written before it,
`briefs/fixup/arvo-plan-2.md` and `briefs/fixup/map-arvo.md` on the channel, by
seats that say themselves they did not read the registry whole. Both are tested
here against the reading, and most of the second plan's steps survive the test.
What changes is what comes first, what is no longer waiting on op, and three
steps neither plan has.

What this job read in full and what it read as an index is in `LEDGER.md`, file
by file. The plan was first written with the ruling, obligation and law files
read in full and the two large proposal files, the questions, the retirements
and the probes read as an index. Those five and the dimension file have been
read in full since, row by row. That reading reordered nothing and removed no
step. What it changed is marked where it lands: the strategy subject of step 2,
a conflict with the bit crate's design in step 5, the section on the
coordinator's questions, and under most steps a short list of rows a brief for
that step has to carry, because each is a route the panel already measured and
closed or a question the step would otherwise answer by accident.

The plan stands on four things, and a step that cannot name one of them under it
is not in it. The ratified rows, which are defended. Op's words of 13 November,
recorded as `ruling::a_shipped_overflow_default_is_a_generic_overridable_per_case`
and `ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open`. The
obligation namespace, which is the demand side stated in the consumers' own
terms. And what the tree at `2c8e5343` actually ships, which was run rather than
read where a step leans on it.

One rule holds across every step: nothing parallel and nothing standing in. The
oracle for every operation is the one `arvo-format` already ships, an exact
result adapted once, and a value-tier operation is equal to it or it is wrong. No
step writes a second arithmetic, a second rounding or a private carrier table,
and where a family from the old tree returns, its bodies are ported onto the
contracts and the capacity the earlier steps land, never beside them.

Op chose the order of the first moves himself on 13 November: among claiming
first and then reworking the branch in one round, contracts first, a short panel,
and landing as built, he answered "Claim, then rework in one round
(Recommended)". `OPS-WORDS.md` quotes the option whole. The rework has landed,
as `9dfd5339`, on three of the four things the option names: the carrier table
is read from `arvo-placement`, the debug abort is its own axis, and the
admission is an open check. The fourth, aliases keyed on strategy defaulting to
`Warm`, waits on what `Warm` names, as his answer on `Warm` for integers in the
same round allows. The claim he put first has not been made: the value
definition is still a one-expert proposal, which is ticket `9e02318643e1` and
step 2 here, and it is the first thing owed. Contracts-first as an ordering was
put to him and not taken, and this plan does not reopen it: the contract surface
comes directly after the operations it is a contract of, with its sketch running
beside them because a sketch overhauls no crate.

## What the reading changes

Nothing in the plan waits on op now. The second plan carried two questions as
his, the overflow default and the public name, and built N3 and N17 around
waiting for them. He has answered both: the first in the words the new ruling
row carries, the second with "Decide at the port". `cargo mock
unasked-questions` reports "Nothing is waiting on him."

The overflow default is a step rather than a blank. His answer was not one of
the four options, so the second plan's N3, which leaves the aliases without a
default "until his answer", is planning for an answer that came in a different
shape. A default ships, as a generic carrying a default, overridable per case.
The sketch at `mock/research/sketches/202611131030_overflow-wrapper-rebinding/`
settles what that takes: the re-binding of a signature's overflow coordinate
compiles and is admitted, MATLAB's default pair, `Nearest` and `Saturate`, is
admitted by the value tier today, and a value crosses between two sibling
signatures through three public doors, `Num::adapted(Exact::on_grid(value.slot()))`.
So nothing is missing for the mechanism, and step 4 is its design: the default,
the spelling of the override, and whether the crossing gets a name.

The contract surface is demanded twice and placed piecemeal.
`obligation::the_surface_expressible_as_contracts_before_anything_implements_it`
and `obligation::composition_contracts_above_the_numeral` ask for it. The live
rows that say the two old contracts crates do not return re-home what they
carried, the bit contracts with the masks over `arvo-bits` and compare, `min`,
`max` and `constant` on the value crate, and one clause leaves `Abs`, `Recip`
and `Sqrt` with a consumer for the first slice, against his words that the
arithmetic contracts on the standards are arvo's own. Hilavitkutin at `c42d707`
still imports the bit contracts through the old masks crate: `BitAccess` on 14
`use` lines, `BitLogic` on 6 and `BitSequence` on 3, all from `arvo_bitmask`,
and `FromConstant` on 3 from `arvo::traits`. `Additive` and `Identity`, on 64
and 65 `use` lines, come from `arvo::strategy` and the facade, and are the
identity markers the value crate already carries, so they are no evidence about
the contracts crates. The counts are `tools/count-old-arvo-names` beside this
file, run at that revision, with its output committed next to it.

The standards reach the value tier already. The second plan puts standards
through values at N15, behind floats. The MATLAB half needs nothing that is not
landed, so it moves up beside the default, and only the IEEE 754 half waits.

And the registry and the design documents get their repairs first, because every
later brief cites them. A helper briefed against a root document that says four
crates, describes a lint as not yet landed and states a `pub const trait`
principle in a tree with no feature gate is briefed against a tree that is gone.

## The steps

Each is one helper, one branch, one mockspace round where it touches a crate's
design or source. The proving checks common to every arithmetic step are not
repeated below: each operation equal to `arvo-format`'s oracle, exhaustive at
widths 1 to 8 through the value; one mutant per operation with its red output
committed; a compile-fail twin for each refusal; and the outside consumer crate
under `arvo-numeral/tests/consumer`, which builds with no feature line, reaching
every new public item.

| Step | Branch | Overhauls | Needs | Tier |
|---|---|---|---|---|
| 0 | `work/registry-repairs` | none; registry rows, one lint, one tool | nothing | sonnet medium |
| 1 | `work/principles-catch-up` | none; root documents and `mock/agent/` templates | nothing | sonnet high |
| 2 | `work/claim-the-value-rows` | none; registry only | nothing | opus medium per seat |
| 3 | `work/numeral-ring` | `arvo-numeral` | nothing | opus high |
| 4 | `work/overflow-default-and-override` | `arvo-numeral` | 3 | opus high |
| 5 | `work/numeral-contracts` | a contracts crate, new; adds impls to `arvo-numeral`, and holds the impls for `Bits<N>` unless the round reopens the bit crate's design | 3, sketch beside it | fable low to shape, opus medium to build |
| 6 | `work/standards-through-values` | none; adds to `arvo-numeral` | 4, 5 | sonnet high |
| 7 | `work/numeral-division` | `arvo-numeral` | 3 | opus high |
| 8 | `work/numeral-lanes` | none; adds to `arvo-numeral` | 3, 5 | opus high |
| 9 | `work/capacity-count` | a capacity crate, new | nothing for the row and sketch | opus medium |
| 10 | `work/bit-sets` | a bit-set crate, new | 5, 9 | sonnet high |
| 11 to 15 | one branch each | one crate each, new | 5, 9, 10 | sonnet high; opus medium for spectral |
| 16 | `work/composition-contracts` | registry first, then the contracts crate | 5, 8 | panel, then opus high |
| 17 | `work/float-values` | `arvo-numeral`, adds to `arvo-format` | 2, 5 | opus high |
| 18 | `work/shared-pair` | none; adds to `arvo-numeral` | 3, panel claim | opus high |
| 19 | `work/value-benches` | none; `mock/benches` | 3, 8 | opus medium |
| 20 | `work/port-hilavitkutin`, in hilavitkutin | hilavitkutin's imports | 3 to 10 at least | sonnet high after an opus read |

Steps 3, 4 and 7 each overhaul `arvo-numeral` and run one after another. Steps
0, 1, 2 and 9 touch nothing the others hold and can run beside them from the
start.

### 0. The registry repairs

Most of this step is done, on the branch that corrected this plan and on
`work/registry-repairs` after it, and `CHECK.md` section 7 lists both: the gaps
and notes on rows that are not ratified, the corrections to the two rows this
job added, and the stale sentences the full read found in the six files it
covered, which section 1.10 counts by class. Five things are left.

The edits to ratified rows. Op answered "The eighteen, not the notes" on 13
November, so `ratified-rows.patch` beside this file is the eighteen
`ratified_by = "op"` lines and nothing else, and the three notes stay a
proposal in `ratified-notes.proposal.patch`. The patch changes no `says`, no
`rung` and no standing, and it wants a seat whose permissions allow editing a
ratified row's fields, ticket `c9e9da26606f`, since this one was refused twice. With it goes a lint in
`mock/lints/` refusing a `ratified` ruling that names no `ratified_by`, which
the schema calls required and declares optional. The lint cannot land before
the patch, since it would be red on eighteen rows.

A tool, not a lint, for the other finding: one that lists every row whose `gap`
or `note` says an axis is undeclared, beside the `dimension` ids, for a reader
to check. Across the registry twenty-six sentences match the phrase and
fourteen of them are the claim, on twelve rows: five name an axis that is still
undeclared, six named one that has been declared since and are amended or
written down, and one cannot be told from the row. Telling those apart is a
judgement, which is what a tool reports and a lint cannot gate on. The tool
joins a TOML string's line continuations before it matches, since a search that
did not missed two of the fourteen. And
`no-registry-comment-counts-its-own-rows` passed on three comments that counted
their rows in words, so its reach is checked and widened.

A lint for the citation the full read found pointing into nothing, ticket
`5b32c44d42c6`: a `bound` named a slug under `retirement::` that no retirement
row carries, and `cargo mock check` was green with it in place. Which lint
should have refused it is not known yet, so the work starts by planting the
wrong slug back in a `bound`, a `note` and an `answered` and seeing which of the
three a lint refuses, and it widens the prose-citation lint where none does.

The rounding residue, which is the panel's and is listed here so it is not
lost: four question rows track predicates that still name a retired or
unqualified mode, `question::which_of_the_six_a_retired_word_names_on_each_row`,
`question::which_tie_direction_an_unqualified_nearest_names`,
`question::is_the_rounding_vocabulary_complete_at_six` and
`question::what_region_does_a_predicate_naming_no_mode_state`. Two of them close
by opening the instrument behind each row, which is a reading a helper can do
and hand to two seats, and `cargo mock rounding-vocabulary` lists the rows.

The red test of ticket `5a78e00a2c9e`, fixed in the test's own sample
construction, so the readiness check can say READY at all.

It is done when `cargo mock check` is green, the new lint is shown red on a row
without the field before the patch and green after, and the tool's output is
committed once with each of its rows read.

### 1. The root documents catch up

A doc round over `mock/PRINCIPLES.md.tmpl`, `mock/DESIGN.md.tmpl` and the banner
in `mock/agent/`, correcting the passages `CHECK.md` section 3 quotes: four
crates where there are five, the lint described as not yet landed, the
fixed-point primitives described as not there, principle five's `pub const
trait` in a tree with no feature gate, the tagline that closes the strategy set
at four, and the "bottom three crates" banner every generated agent file opens
with. Each change named and quoted in the changelist.

The alias-default sentence stays as it is. It states the design op chose, and
the tree is what is behind it. What this step may add beside it is one sentence
saying where the tree stands: the aliases name their adaptation until `Warm` is
claimed.

It is done when the generated `docs/` and `.claude/` agree with the tree, and the
`design-doc-source-mismatch` lint is green.

### 2. Claim the value rows

The claim op ordered first, and the second plan's P0, unchanged, because it was
right: two cold reads, one after the other, each from a worktree cut before the
other's commit, then the coordinator's gate, over the rows the value crate was
built on at one expert.
`proposal::a_value_is_the_sole_occupancy_placement_of_a_declared_signature`,
the admission row, `the_debug_check_is_a_parameter_the_consumer_names`, and the
three rows in `proposal-the-exact-operations.toml`.

One subject is added, and the full read widened it. Five proposals say what a
strategy is. Two stand at two experts and disagree,
`proposal::a_strategy_is_an_assignment_and_a_weighting` and
`proposal::a_strategy_is_a_preference_over_measurements_resolved_as_a_compile_time_argmin`.
Three stand at one expert and postdate the ruling whose lineage op ratified,
`proposal::a_strategy_is_a_pair_of_an_observable_assignment_and_a_weighting`,
`proposal::a_strategy_is_a_declared_semantics_together_with_a_weighting_over_the_arms_that_realise_it`,
which supersedes the pair, and
`proposal::a_strategy_is_a_consumer_supplied_weighting_over_measurements_never_over_implementations`.
The registry already asks which of the three his ratification of
`ruling::a_strategy_is_a_preset_naming_a_point_with_a_weighting_in_it` lands
on, or whether it is a fourth nobody has written, as
`question::which_successor_carries_the_strategy_shape_ratification`, and gives
it to the panel. So the subject goes to the same seats as that question and not
as a choice between two rows, because whether the overflow rule is a declared
component of a strategy or an effect of its weighting decides whether a
strategy can name an adaptation at all, and step 4 leans on the answer.

### 3. The ring operations through the value

The second plan's N1. `sub`, `neg` and `mul` on `Num` as inherent `const fn`,
each the format's own operation carried through the carrier, with the verdicts
`fits_sub`, `fits_neg` and `fits_mul` beside them under `DebugCheck`; a signed
fixed-point point beside `UFixed`; the multiplicative identity computed from the
format's predicate and refused as a failed constant where it is not a member.

Beyond the common checks: the product exhaustive for a 6-bit signed and unsigned
fixed-point signature in all five deterministic modes and both policies, and a
tie at a negative product in each mode, since `half_up` sending a tie toward
positive infinity whatever the sign is the place a carrier implementation goes
wrong first.

### 4. The shipped default and the per-case override

New, from his words and the sketch. Three things, in one round because they are
one feature.

The override: `WithOverflow<S, O>` and its rounding twin as aliases, so a
consumer writes the re-bound type by naming one coordinate. The sketch shows the
alias compiles with no feature gate, that its result is admitted, and that it is
refused where the admission refuses. Whether the spelling is the wrapper op
floated or the alias is the round's to settle against
`ruling::configuration_is_not_composition_and_a_predicate_never_transports_for_free`:
the admission bound does not come along for free, and a spelling that hides it
states a transport the canon refuses.

The crossing: a value moves between two signatures that share a format today, as
`Num::adapted(Exact::on_grid(value.slot()))`, through three public doors. So the
round does not build a door. It decides whether the composition gets a name of
its own, and if it does, the name is bounded to signatures sharing a format,
where the slot is on the target's grid and inside its range by construction. A
change of format is `convert`, which is step 7's. Whether the composition lowers
to a move is a codegen question the sketch does not answer, and it goes to step
19's benches.

The default: the aliases are keyed on a strategy and default to `Warm`, which is
the design he chose on 13 November, and they name their adaptation until `Warm`
is claimed. What `Warm` names for overflow is `question::does_warm_wrap_or_clamp`,
and it is the panel's, with both priced as
`ruling::wrap_or_clamp_stays_open_and_both_get_priced` asks and as his "Bring me
both priced" repeats. Two candidates are in scope. MATLAB's documented pair,
`Nearest` and `Saturate`, is the one complete convention and is admitted today.
Rust's own release behaviour, wrap, is the other, on the reading that it is what
the intuition of somebody writing Rust mimics.
`ruling::warms_objective_is_the_intuitive_best_choice` asks for mimicry where it
is not consistently the worse choice and names neither Rust nor wrap, so that
reading is this plan's and not the row's.

The step lands the mechanism and stops there. The aliases keep naming their
adaptation, and the one line that makes `Warm` select an overflow behaviour,
with the compile-fail test that pins what `Uint<8>` with no adaptation means,
lands only once the panel's answer to `question::does_warm_wrap_or_clamp` is a
ruling. The row that holds the question open is at `rung = "open"` and reserves
the call, so a default written in code before the answer would fill it from
below.

The rounding half of the default has a question of its own that the step must
not answer by picking. `question::why_the_default_rounding_position_is_chosen`
is the panel's and asks whether familiarity or what a mode licenses governs,
and `dimension::rounding` records under `moves` that the IEEE default is the
one mode that is not free under either signedness. MATLAB's `Nearest` is a
candidate under the first option and not a settled default.

It also records the gap `ANSWERS.md` part c found: the format tier's overflow
axis is open and `Sole` admits two policies. Whether a consumer's own policy is
admitted to the value tier by supplying its carrier arithmetic is a panel
question, listed below, and this step leaves the admission as it is.

Checks: the crossing exhaustive at widths 1 to 8 for every ordered pair of
admitted policies and shown to be the identity on the slot; the sketch's two
refused signatures as compile-fail tests in the crate; if a named crossing
lands, a use of it across two formats refused at build; the consumer crate
writing an override with no feature line.

### 5. The contracts a consumer bounds on

The step his second answer names as arvo's own ground, and the one both
obligations ask for. A sketch first, three arms as the second plan has them:
plain traits beside the inherent `const fn`s, `const trait` behind an opt-in
feature with the plain traits still present, and `const trait` alone. Each arm
is judged on one thing, which `obligation::the_unstable_machinery_does_not_reach_a_consumer`
fixes: a consumer with no feature line builds a function generic over a numeral
that adds, orders and has an additive identity.

Then the contracts, each a trait whose law is the oracle equality: sum,
difference, product, order, the identities, sign predicates, absolute value,
construction from a constant, the monotonicity of addition that the algorithm
families gate on, and the bit tier's width, access, logic and sequence. Reciprocal and square root are named as contracts with no instance
until division and floats. `Num` and `Bits<N>` are instances, and one outside
instance in the consumer crate shows the set open.

Where they live is taxonomy, which he opened. The obligation's own wording
decides more than it seems to: a consumer must be able to "build against the
shape while the implementation behind it is unfinished", which a contract
declared inside the implementing crate cannot give. So the recommendation to the
round is one contracts crate below the value crate, with each trait placed by
the lowest crate that reaches its types. The round may find better and says why.

One conflict with a landed design has to be settled inside the round and is not
the builder's to settle. `arvo-bits`'s design says `Bits<N>` depends on nothing
in this workspace, and declines the width accessor `arvo-format` could offer it
as a second name for something already in scope. So `Bits<N>` cannot implement
a contract declared in another workspace crate without that design changing,
and a width contract over it is close to the accessor that design declined. Two routes are open: the contracts
crate sits above `arvo-bits` and holds the impls for it, which the orphan rule
allows and which leaves the bit crate's design as it is, or the bit crate's
design takes a round of its own first. The first changes no landed design and
is the recommendation, and the table above is written for it.

Three rows bound the surface the contracts are declared over.
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`
answers `question::what_the_numeric_introduction_door_may_carry_out`: the door
carries the coordinate set of the ratified parameterisation in types the stack
owns, and the ruling says in so many words that it does not say how many types
that is. So a contract's associated items are typed with the stack's own types,
and the count of those types is the round's to derive and not to assume. `question::what_the_ingest_row_claims_and_what_turns_on_it`
is open and the panel's, and a construction contract that validates its input
would answer it by accident, so construction from a constant stays a compile-time
door. And the law suite takes its regions from the law rows and not from
intuition about rings: `retirement::dl_wraps_induced_ring_transfers_to_fractional_formats`,
`retirement::dl_the_wrap_section_is_a_relabelling_at_every_scale` and
`retirement::dl_signed_twos_complement_saturating_multiplication_induces_a_semigroup_at_some_width`
each retire a law a generic suite would otherwise assert over every instance.

This is the widest step in the plan, every family from step 10 on is written
against it, and an expert seat reads the shape before the build is briefed.

Checks: a law suite generic over each contract, run against `Num` and against a
deliberately wrong outside instance that must fail it; the disassembly of a
generic sum against the concrete one, committed as an ad hoc reading and called
that.

### 6. The standards through the value, MATLAB first

The arithmetic contracts on a standard, which the registry is silent on and his
words ask for. Each standard's convention as a named instance of step 5's
contracts over step 4's default mechanism: MATLAB's `fi` with its `fimath`
defaults as value aliases, and the existing parity suite's ten arms run through
the value and through the contracts as well as through the slot arithmetic, one
computation by three routes with one expected number from the MathWorks page.

The IEEE 754 half joins after step 17 and is marked where it is missing, which
`ruling::the_standard_is_parity_in_output_not_in_the_internals` calls an honest
hole in a standard rather than a defect in the framework.

Which standards are owed is settled: `question::which_standards_the_alias_obligation_covers`
carries his answer, the two named as the core and the other three the corpus
carries reserved and not excluded. `probe::container_premise_standards_alias_adequacy`
is the instrument that already judged an encoding against both, and its arms
are the shape the value-tier parity arms follow.

### 7 and 8. Division, conversion, lanes and the accumulator

The second plan's N4 and N5, unchanged in substance. Division over a divisor
that cannot be zero by type, the remainder pair and `convert` between two
admitted signatures, all equal to the oracle `2c8e5343` landed. Then kaski's
lane, `obligation::a_deterministic_fixed_point_lane_with_declared_rounding`: a
fixed-point alias under a named rounding with half to even on the product, and
the exact accumulator beside it, built from a value, exact under addition and
subtraction, back to a value only through the signature's adaptation.

One correction to N5. It designs the accumulator as "the first chain carrier"
ahead of `question::which_chain_carrier_ships`. It is one carrier, and step 16 is
where what a chain guarantees gets stated; the accumulator's design says it is an
instance and does not close the question.

The accumulator's brief carries what the panel measured about folds, since each
is a wall the builder would otherwise find again. A fold cannot take a widening
operation, `probe::a_widening_operation_cannot_be_folded`, and an accumulator
whose type grows per iteration is retired as unpayable,
`retirement::dl_growing_an_accumulator_type_per_iteration`. So the shape is a
closed operation with an accumulator determined separately,
`proposal::a_fold_needs_a_closed_operation_and_a_separately_determined_accumulator`,
at one expert. A width written by hand compiles at every length and is right at
one, `probe::a_runtime_length_fold_compiles_at_every_length_and_computes_at_one`,
which is the mutant this step's test must kill, and
`probe::the_accumulator_width_is_the_exact_sum_width_less_one_bit` gives the
width a signed saturating fold needs at the lengths it measured.

### 9. Capacity

The second plan's N6. No obligation row asks for it, which that plan noticed and
this reading confirms, and hilavitkutin names it more than anything but `USize`:
by whole-word count `Capacity` 563 times, `Dim` 474, `ConstCapacity` 75 and
`Cap` 38, with 72 paths through the old tensor crate. A whole-word count
over-reads a name as short as `Cap`, and the script that produced these is
beside this file for anyone who wants the sites. So the row comes first, in
hilavitkutin's own terms, then a sketch of the spellings, then the crate.

The sketch starts from three retirements and does not repeat them. The naive
spelling, the shared carrier answering directly for the backing array, was
refused four ways and is `retirement::dl_capacity_unification_naive_spelling`.
The earlier feasibility probe never reached the associated array type, which
`retirement::dl_feasibility_probe_compiled_the_load_bearing_path` records, so
each arm of the new sketch has to size a real array. And a count checked at one
construction door is bypassed by the trait-method route above rank zero,
`retirement::dl_capacity_checked_at_the_one_construction_door`. A static length
as a lever on what lanes cost is `retirement::r173_rk_the_static_length_lever`,
unestablished in either direction, and is cited by nobody as a reason for a
spelling.

Checks: the obligation row quotes use sites the counting script finds again at
the revision it names; the sketch commits an outcome for each spelling, judged
on whether the outside consumer sizes a container with no feature line; then the
common checks on the crate, with a count that is not a constant refused at
build.

### 10 to 15. Bit sets and the algorithm families

The second plan's N9 to N14: the bit-set contract with `Bits<N>` as its one-word
instance, then content hash, DAG ordering, sparse structure, spectral partition
and the cost programs, one crate per obligation, bodies ported from `1b2796e9`
onto the contracts and the capacity, each with the old tree's bench family rerun
on the harness. The spectral crate is the one with numerics in it and takes the
higher tier.

Checks, the same four for each crate. The contract's law suite runs against the
crate's instance and against one deliberately wrong instance that must fail it.
The property the obligation names is asserted over every input of a small size,
every graph on up to five nodes for the ordering and every word for the bit
set, and by known answers where a published one exists, as for the hash. The old
bench family's committed numbers at `1b2796e9` are the comparison for the rerun,
and a difference either way is written as a finding. And the obligation's own
`gap` is re-read at the end and either closed or restated.

What gates these crates on a numeral is the place the old tree was wrong and
the panel measured it. Gating on an associativity fact admits the one preset
whose recurrences return wrong answers and refuses two that compute correctly,
`retirement::dl_gate_algorithm_crates_on_addassoc`, whose replacement is a
monotonicity marker over addition. The instruments are
`probe::a_dag_dynamic_programme_returns_wrong_answers_on_in_range_instances`,
`probe::monotonicity_and_absorption_are_two_properties_and_a_reserved_top_buys_one`
and `probe::the_top_absorbs_under_saturation_and_never_under_wrapping`, with
`proposal::a_min_plus_computation_needs_monotonicity_as_well_as_an_absorbing_top`
at one expert over them. A numeral wrapping inside with a reserved absorbing
top is retired, `retirement::dl_interior_wrapping_with_a_reserved_absorbing_top`.
Splitting a reduction is sound in three of the four sign-and-policy cells and
not under signed saturation,
`probe::which_sign_and_policy_cells_survive_reassociation` and
`proposal::splitting_a_reduction_is_sound_in_three_of_the_four_sign_and_policy_cells`,
which any crate that reduces in lanes has to state as a bound.
`retirement::dl_the_precise_exile_as_the_problem` and
`retirement::dl_total_ord_makes_no_output_law_expressible` close two framings a
port from the old bodies would bring back with them. So steps 12, 14 and 15 are
written against step 5's monotonicity contract and each of their wrong
instances is one of the measured cells.

### 16. The composition contracts

Neither earlier plan has this, and it is half of what op says arvo is for.
`obligation::composition_contracts_above_the_numeral` is one slug carrying four
demands, by its own `gap`, and nothing in N1 to N17 answers any of them.

Panel work first: the obligation split into the demands two independent readings
already found, each its own row, which is a registry change for seats that may
make it. Then the contracts for what a chain guarantees, with step 8's
accumulator as the first instance and the ratified rows on composed regions as
the law they are checked against.

Four open questions sit under this step and all four are the panel's, so the
split is read against them and the contracts close none by being written.
`question::which_chain_carrier_ships` has four options and one of them is a
composition of carriers. `question::what_a_numeral_guarantees_to_a_fold` asks
whether the numeral names its structure, its accumulator relation, both or
neither. `question::reduction_order_or_associativity` asks whether the order is
specified or associativity required, which is the same fact the lane crates of
steps 10 to 15 meet. And
`question::does_a_named_policy_selection_survive_above_the_numeral` decides,
by its own `unblocks`, what a composition contract may carry.

Checks: the split agreed by two cold reads, each from a worktree cut before the
other's commit; every contract naming the law row it is checked against; the
accumulator passing the chain suite, and one chain that breaks the stated
guarantee refused at build with its compile-fail twin.

### 17 to 19. Floats, shared placement, benches

The second plan's N8, N7 and N16. The float arm closes the IEEE 754 half of step
6 and answers `obligation::a_build_flag_that_changes_float_semantics`, strict
against fast as a parameter of the value in the shape `DebugCheck` has. Shared
placement waits on the panel's claim about values at a shared placement. The
benches run wrap against saturate through the value, which is the pricing
`ruling::wrap_or_clamp_stays_open_and_both_get_priced` asks for and what the
panel needs to name step 4's default.

Checks. For the floats, parity arms in the shape of the MATLAB suite, each
expected number taken from the standard's own tables and none from the
implementation, run under both settings of the flag, with one arm that must
differ between them. For shared placement, whatever the claimed row states, as a
test that fails when the row is broken. For the benches, the harness under
`mock/benches/` and nothing beside it: one workload shared by both policies,
calibrated repetition, and the CSV and findings committed, with the crossing of
step 4 as one of the variants.

The bench step inherits four findings about the harness itself, and a number
from it is worth nothing until each is met. The tree was built with no profile
declared anywhere, `probe::the_bench_tree_was_built_at_the_undocumented_profile`,
a row standing at `uncontrolled`, and no committed artifact records the profile
it was built at, `probe::no_bench_artifact_records_its_build_profile`. No
committed run cross-checked that its timed arms compute the same answer,
`probe::bench_corpus_never_validates_its_own_arms`, and 175 of 254 committed
regions predate the validation gate,
`probe::most_committed_bench_regions_predate_the_validation_gate`. So the step
declares its profile, records it in the artifact, and runs with validation on,
before any wrap-against-saturate figure is read.
`retirement::r161_r8_the_retirement_of_the_107_second_figure` is the case of
two correct numbers at two profiles read as a conflict.

Shared placement inherits two. A packed-access gather written as a loop loses
its instruction shape at width 18,
`probe::a_packed_access_gather_written_as_a_loop_collapses_at_width_18`, and
the claim that the shipped suite shows two markers sharing a realisation map is
retired, `retirement::r210_the_shipped_suite_asserts_two_markers_share_a_realisation_map`:
what that suite shows is the container invisible to arithmetic under one fixed
semantics. `arvo-placement`'s design also records that its access ladder runs
out of native containers at width 123, and leaves that as a bound on the
derivation under `question::arbitrary_width_demands_in_the_canon`, so the step
states the widths it covers and does not design around the bound.

### 20. The port, and the one question that is his

Hilavitkutin's pinned imports replaced family by family. It opens by bringing op
the public-name question he deferred to this moment, with the import list at
hilavitkutin `c42d707`: `arvo::` 427 occurrences, `arvo_tensor::` 72,
`arvo_bitmask::` 24, `arvo_bits::` 13, `arvo_hash::` 5, `arvo_graph::` 5,
`arvo_sparse::` 4, and 2 each for `arvo_spectral::` and `arvo_refit::`.

Three retirements are about this migration and the port's opening read takes
them as given. `retirement::dl_facade_migration_route_x` and
`retirement::dl_facade_migration_route_y` are two routes refused by the
compiler, and `retirement::dl_the_facade_fix_touches_every_consumer` records
that the real public break was the call sites naming the three-parameter fixed
types directly, twenty-one of them when it was measured, with the rest of what
a consumer writes unaffected. That count was taken on the old tree and is
recounted at the port.

Checks: hilavitkutin's own suite green after each family, and the counting
script at zero for each old path once its family is through.

## Where each old family returns

His words are that every family returns in function and feature. This is the
whole list, sixteen crates at `1b2796e9`, against the step that returns it.

| Old crate | What it was | Returns in | In what shape |
|---|---|---|---|
| `arvo` | the facade and its numerals | 3, 4, 20 | value aliases; the public name decided at the port |
| `arvo-storage` | the bare-primitive layer | landed, 9 | `arvo-bits` and the value aliases, the carrier derivation in `arvo-placement`, and the capacity crate |
| `arvo-transparent` | the transparency trait | landed, 4 | the doors each value type declares; a named sibling crossing if step 4 wants one |
| `arvo-strategy` | the markers and container projection | landed, 2, 4 | presets over a declared signature; cells claimed in 2 |
| `arvo-bits-contracts` | the bit-level traits | 5, 10 | the three traits return with the masks over `arvo-bits`; where they are declared is step 5 |
| `arvo-bits` | the exact-width container | landed | `Bits<N>`, an instance of 5's contracts |
| `arvo-mask-contracts` | the one mask trait | 10 | the bit-set contract, in the masks crate |
| `arvo-bitmask` | masks and bit matrices | 10 | a bit-set crate over `arvo-bits` and capacity |
| `arvo-numeric-contracts` | the arithmetic traits | landed, 5, 6 | compare, `min`, `max` and `constant` on the value crate; the contracts in 5, the standards as instances in 6 |
| `arvo-refit` | narrowing and widening | landed, 7 | `cast` on `Bits<N>`, and `convert` between admitted signatures |
| `arvo-tensor` | capacity, arrays, matrices | 9 | the capacity crate, then its containers |
| `arvo-hash` | the content hash | 11 | one crate on `obligation::a_content_hash` |
| `arvo-graph` | DAG algorithms | 12 | one crate on `obligation::ordering_a_directed_acyclic_graph` |
| `arvo-sparse` | sparse structure | 13 | one crate on `obligation::a_sparse_adjacency_a_plan_can_be_built_on` |
| `arvo-spectral` | spectral partition | 14 | one crate on `obligation::a_spectral_partition_of_a_dependency_graph` |
| `arvo-comb` | cost programs | 15 | one crate on `obligation::a_cost_dynamic_program` |

Crate names for everything new are open, as the record decides none of them.

## Where the two earlier plans are wrong

### `arvo-plan-2.md`

It is a good plan and most of its steps are carried above almost as written.
Six things in it do not survive the reading.

It cites the registry by line throughout, `ruling.toml:1038` and its like, 78
times. This repo's lints refuse that form because the files gain
rows constantly, and this job's own two rows have already moved what follows
them. Every one of those citations has to be re-found by subject before it can be
trusted, which is the cost the rule exists to prevent.

It treats the overflow default as waiting on op and asks him the four-option
question again. He answered on 13 November and took none of the four, so N3's
"no default until his answer" and section 4's Q1 are both overtaken.

N15 names "the MATLAB `fi` and IEEE 754 aliases of `arvo-format/src/standards.rs`".
That file holds MATLAB alone. `grep -rl IEEE mock/crates/*/src/` returns nothing,
and the control is the same grep for `Fi`, which finds the file. The IEEE half is
unbuilt, not waiting to be carried through.

It has no step for composition contracts, which an obligation demands in op's
name, and none for the override mechanism.

It orders MATLAB parity behind floats, which it does not need.

And by its own section 5 it rests on `arvo-format`'s design unread and on no test
run. The first matters less than it might, since the root design it used instead
is accurate on the format crate. The second hid a red: the suite at `2c8e5343`
fails one test, in the test's own sample construction, ticket `5a78e00a2c9e`.

### `map-arvo.md`

It was written at `989e02f9` against the branch `work/numeral-integers`, before
the rework op asked for. Its four criticisms of the value crate were right then
and each is fixed at `2c8e5343`, checked in source: admission is an open check
over what a signature declares where it was a sealed list
(`arvo-numeral/src/admit.rs`, the blanket impl of `Sole`); the carrier is read
from the placement and asserted by `carrier_agrees` where it was a transcribed
table; the `Wrapping` and `Saturating` marker pair is gone in favour of the
`DebugCheck` axis; and the platform-width aliases take their adaptation as a
parameter where they fixed one. Anything briefed from its section 3 today is
briefed against a crate that no longer has those shapes.

Its section 6 lists as unestablished whether the standards conventions produce
signatures `Num` admits. They do, and the sketch's fifth arm is the evidence.

Its section 4, on canon the designs state and the registry lacks, was not
re-derived here and is neither confirmed nor refuted by this reading.

## The questions

### Op's

One, and not now: arvo's public name, at step 20, as he said. Options, as the
second plan framed them and with nothing in the record since to change them: a
crate named `arvo` returns and owns the defaulted aliases; no such crate, and
consumers name the inner crates; a crate named `arvo` that only re-exports; the
value crate itself is renamed `arvo`. The record: one proposal at one expert,
`proposal::the_old_facade_returns_as_value_aliases_and_no_facade`,
and his "Decide at the port".

That the list is this short is a finding and it was checked rather than assumed.
`cargo mock unasked-questions`: "13 of 110 questions name op, and 13 of those a
ruling has already answered. Nothing is waiting on him."
`ruling::the_panel_finishes_the_canon_without_him` gives every canon question to
the panel, and each candidate this job first drafted as his turned out answered
by his own words, as `CHECK.md` section 8 records.

### The panel's

Listed apart, each with the step it holds and what two cold reads would have to
settle for it to close.

Which cell `Warm` names for overflow, and with it which adaptation the shipped
default names, MATLAB's `Nearest` and `Saturate` or Rust's wrap. It holds the
one line in step 4. Each read would have to say, from step 19's committed
findings and the two rulings and without the other's text, which cell it names
and over which formats, and the two would have to name the same cell over the
same region.

What a strategy is, which the registry holds as
`question::which_successor_carries_the_strategy_shape_ratification` over three
one-expert proposals, with two older definitions at two experts beside them, as
step 2 lists. It holds whether a preset can name an adaptation, in step 4. Each
read would have to name one of the five or state a sixth, say which overflow
rule follows from it, say whether
`probe::the_rationalisable_sections_on_the_committed_carrier_table` separates
the candidates, and the two would have to name the same row for the
ratification to land on.

Whether the default rounding position is chosen for familiarity or for what it
licenses, `question::why_the_default_rounding_position_is_chosen`. It holds the
rounding half of step 4's one line. Each read would have to take one of the
row's three options from his stated rule in its `bound`, and say what that
makes of MATLAB's `Nearest` as a shipped default.

Whether a consumer's own overflow policy is admitted to the value tier. It holds
nothing built today and decides whether "composable from scratch" reaches past
the format tier. Each read would have to say whether `Sole` admitting two
policies is the intent or only what is built, and if a third may enter, what it
supplies: its completion on the carrier and the equality to the oracle that
makes it one arithmetic and not a second.

How `obligation::composition_contracts_above_the_numeral` splits. It holds step
16. Each read lists the demands it finds in the row's `gap` and quote, and the
two would have to agree on how many rows there are and where each ends.

`question::which_chain_carrier_ships`. It does not hold step 8 and is closed by
step 16. Each read would have to state what a chain guarantees in a form the
accumulator and one other candidate can both be checked against.

Values at a shared placement. It holds step 18's build. Each read would have to
say what sole occupancy means once two values share a placement, and whether the
pair is one admitted thing or two.

The float arm's admission, for which no row exists. It holds step 17. Each read
would have to say what a float signature declares and which of the four things
`Sole` asserts carry over, range and quantum being the two that cannot as they
stand.

The capacity spelling, after its obligation row. It holds step 9's crate and
everything from 10 on that sizes a container. Each read picks among the
sketch's arms against
`obligation::the_unstable_machinery_does_not_reach_a_consumer`, and the two
would have to pick the same arm.

### The coordinator's

The first version of this section named three questions and had two of them
wrong. The registry gives the coordinator three, by `decider`:
`question::is_the_non_terminating_contention_crate_a_defect`,
`question::does_the_position_keyed_dither_arm_ship` and
`question::does_a_consumer_supplied_seed_surface_exist`. Each row's `note` says
the coordinator answered it in op's stead as derivable, that the answer is not
written in the row, and that the call carries no authority. None holds a step
here. The first bears on step 19, since it is about the bench harness, and the
other two on the rounding arms nothing in this plan builds.

The two process questions that gate how every answer is promoted are the
panel's and not the coordinator's:
`question::what_stops_two_files_by_one_persona_reading_as_two_experts` and
`question::does_a_cold_brief_withhold_rows_the_proposal_author_wrote`. Step 2
runs under both, and until they close its briefs take the careful side of
each: two seats of two personas, and a cold brief that says which rows the
proposal's author wrote.
