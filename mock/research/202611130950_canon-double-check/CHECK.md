# What the canon says that is no longer true, and what it never settled

This is the second part of job `arvo-canon-check`: the canon read at
`origin/dev` tip `2c8e5343`, checked for being current and for being consistent
with itself. `LEDGER.md` beside it says what was read and how deep, which is the
only thing that bounds the absence claims made here.

The registry is in good order. Nothing here is a ratified row found wrong on its
subject. What is found is narrower and ordinary for a canon under construction:
notes and gap fields that describe a tree the work has since moved past, a schema
field that half the ratified rows do not carry, a set of design passages that
were not brought along when the value crate landed, and one clause of a
one-expert proposal that op's own words of 13 November speak to. Most of the
repairs are a sentence each.

Two disciplines govern what follows. Rows are named by namespace and slug and
never by line, because the registry gains rows constantly and a line keeps
resolving while naming something else. Source is named by `file:line`, which is
the permitted form for a tree that moves under a git sha. Where a finding says
the record is silent, that is a claim about the registry, all fifteen files of
which have been read in full over two passes as `LEDGER.md` records, and not
about the panel corpora, which were not read. Sections 1.1 to 1.9 and 2 to 6
were written on the first pass, when five files had been read as an index.
Section 1.10 is what the second pass added, and it corrects section 5 in one
place.

Section 7 says which of the proposed edits were applied here and which
were only written down. The rule this job works under is that nothing may be
applied that changes a `ratified` row or any row's standing, so the ones that
touch a ratified row's own text are proposals and stay proposals.

## 1. Rows that are stale against what has landed since

### 1.1 The `ruling.toml` header says four edge fields are unset, and three of them are set

The file's opening comment says that `answers`, `obligation`, `ratifies` and
`declines` are unset throughout, and that which question each row would answer is
recorded in a panel file instead. At `2c8e5343` the file carries 16 `answers`, 6
`obligation` and 9 `ratifies`. Only `declines` is still empty. The sentence is
the one a reader uses to decide whether to trust the edges, so it is worth more
than its size.

### 1.2 Eighteen ratified rulings carry no `ratified_by`

The schema's own words for that field are "Required on a `ratified` row and
meaningless elsewhere", and the namespace declares it `required = false`, so
nothing refuses a row that omits it. Of 39 ratified rulings, 18 omit it:
`an_ack_is_not_a_ratification`, `the_strategy_intents_are_not_clear_cut`,
`the_d_numbered_decisions_are_dead`, `nothing_settles_during_the_breadth_pass`,
`a_unit_ends_in_agreement_or_it_has_not_ended`,
`arvo_is_a_library_and_the_value_composes_on_top`,
`arvo_takes_no_stance_on_how_many_cores_it_runs_on`,
`adaptation_is_conditional_on_proof_and_on_soundness`,
`each_preset_names_a_stated_intent`,
`warms_objective_is_the_intuitive_best_choice`,
`hot_may_sacrifice_soundness_for_a_proven_meaningful_gain`,
`cold_is_for_cold_paths_and_cold_storage`, `cold_is_not_obliged_to_be_slow`,
`the_strategies_weigh_measurements_differently`,
`cold_is_not_to_be_deprioritised`, `the_work_is_predicated_arms_composed`,
`never_a_runtime_check_and_one_lowered_path` and
`the_overflow_panic_is_permitted_and_bounded`.

Every one of them carries a `ratification` text saying op selected it in one of
the ratification rounds, so the route is known and only the field is missing. The
value is `op` on all eighteen. The field is the first thing a later reader checks
when a ratified row turns out wrong, which is the schema's own argument for it,
so a missing value costs exactly when it matters most.

### 1.3 The standards obligation's gap is answered on both of its halves

`obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`
says in its `gap`: "no row says whether the set is those, more, or a core with
the rest open. The parity suite is also unbuilt: it is a mandate with no
artifact."

`ruling::the_standards_bound_starts_at_two_and_reserves_the_rest` speaks to the
first half in his own words, as direction on a `stated` row and not yet as
canon: MATLAB `fi` and `fimath` and IEEE 754 are the two,
SystemC `sc_fixed`, Vitis `ap_fixed` and Algorithmic C `ac_fixed` are reserved
rather than excluded. The second half is half true. The MATLAB parity suite is
built: `mock/crates/arvo-format/tests/matlab_fi_parity.rs` holds ten arms whose
expected numbers come from the MathWorks reference page rather than from arvo,
and `mock/crates/arvo-format/tests/the_ties_away_alias.rs` sweeps the Round
alias. The IEEE 754 half is unbuilt, and no operation admits a floating format,
so that half of the mandate still has no artifact.

### 1.4 The platform-width ruling's region says 64 is refused, and 64 is shipped

`ruling::a_platform_width_type_is_a_target_bound_member_of_the_format_family`
states its region with "`W = 64` refused by the shipped `Slots` ladder and not by
this ruling". Commit `d425aff5` carried the slot index to a signed 128-bit
integer for exactly this reason, and the ladder now implements widths 1 through
64: `mock/crates/arvo-format/src/slots.rs:111` says so in its own words and
`:221` asserts it, with `points::USize` and `points::ISize` at the host's pointer
width in `mock/crates/arvo-format/src/lib.rs:248` and `:253`.

The region is inside a ratified row, so this is a proposal and not an edit. The
repair is to restate that cell rather than to delete it: what was refused by the
ladder of the day is now established, and a region that records both is worth
more than one that records neither.

### 1.5 The numeric-door ruling says a design correction is owed, and it has landed

`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`
says in its note that `arvo-format/DESIGN.md.tmpl:235` states the two-type bound
and that the design tier changes before the code under it does. At `2c8e5343`
line 235 of that file is a list of open questions, and under the heading "The
two primitives, and why they are here" the design now says "They are not the
bound on what the door carries. A count of types is not the bound at all." The
position rule the note described as one seat's proposal is there too, at line
404, "Named here as open rather than adopted". So the correction the note asks
for has happened, and the design goes exactly as far as the ruling does.

Worth noting beside it: the note cites a living design document by line, which
this repo's own `no-line-citation-into-a-living-ledger` family of lints forbids
in new text. The stale citation is what that rule exists to prevent, and here it
is, in the canon, pointing at a line whose subject changed under it.

### 1.6 Three law gaps say an axis is undeclared, and the axis is declared

`law::additive_associativity_under_saturation`,
`law::distributivity_of_multiplication_over_addition` and
`law::coherence_of_a_reduction_onto_its_induced_operation` each say the cell they
cannot place is on neither list "because the declared operand window is not a
declared axis". `dimension::operand_window` exists, and its own note names those
three rows as the reason it was declared. The same holds for
`law::quantise_then_reduce_commutes`, whose gap says "no
`dimension` row declares an axis for the ambient domain" against
`dimension::ambient_domain`, and for
`law::the_fused_and_the_stepwise_multiply_add_denote_one_function`
against `dimension::accumulator_width`.

Two of the three cells already have a home. `law.toml` carries three rows this
section first missed, `additive_associativity_recovered_under_declared_nonnegative_operand_window`,
`multiplicative_associativity_recovered_under_declared_nonnegative_operand_window`
and `distributivity_recovered_under_declared_nonnegative_operand_window`, each
holding its cell with `operand_window: declared non-negative` on its `holds`
list. So the repair for the additive and the distributive gap is a sentence
saying the axis is declared and naming the row that holds the cell, and nothing
moves between lists. The coherence cell has no row yet, and the two later-topics
regions can now be written from their sources and have not been. An earlier
version of this section proposed moving each cell from `gap` into `holds`, which
would have duplicated two rows that exist and written three regions this job
did not derive.

### 1.7 Two registry files carry comments that count their own rows

`dimension.toml` opens with "Fifteen of the sixteen rows below" and later "The
four below were not in the sweep"; the file holds 25 rows and 10 follow that
marker. `topic.toml` says "The six below were added after the first port" and
seven follow. The repo has a lint named `no-registry-comment-counts-its-own-rows`
and it passes on both, so either its reach does not cover a count written in
words or it only reads a comment attached to a row. The repair in both files is
to drop the number rather than to correct it, since a corrected number goes stale
the same way, and the lint's reach is a ticket of its own.

### 1.8 One ruling's note rests on a lint and a crate layout that are both gone

`ruling::the_trait_contract_structure_is_a_primary_paradigm` argues in its note
against promoting itself to `in_force` on the ground that "`mockspace.toml:135`
declares an `arvo-bits-traits-only` lint and the crate vocabulary carries three
contracts crates". Neither survives. The only mention of that lint in the tree is
`mock/lints/a_lint_guards_a_crate_that_exists.rs:29`, which cites it as the
cautionary case of a lint gated on a crate that no longer existed, and there is
no contracts crate. The argument the note declines is therefore no longer
available to a later reader on the grounds given, which is worth saying since the
note's whole purpose is to record why the row was not promoted.

### 1.9 `mockspace.toml` still describes the removed crate tree

The `layers` list and the `[crate_colors]` table name `arvo-transparent`,
`arvo-storage`, `arvo-bitmask`, `arvo-spectral` and their siblings, all deleted
on purpose. This is configuration rather than canon and no lint gate depends on
it, and it is also the file `canon_paths` is declared in, so a reader who opens
it to find the canon's address meets a description of a tree that is gone. The
repair is to list the five crates that exist. This job does not touch severities
or structural lists in that file, so it is named here and left.

### 1.10 What the full read of the six remaining files found

The second pass read `proposal.toml`, `proposal-the-later-topics.toml`,
`question.toml`, `probe.toml`, `retirement.toml` and `dimension.toml` row by row.
It found no ratified row wrong on its subject and nothing that moves a step of
`PLAN.md` out of order. It found six kinds of sentence that the tree has moved
past, each searched for across the whole registry after the first instance, with
TOML line continuations joined first, since a phrase split over a `\` line break
is invisible to a plain grep and two instances were missed that way on the first
try.

A note or gap saying an axis is undeclared. The search for the phrasings the
registry uses for it returns 26 sentences. Twelve are not instances: three in
the dimension file's own account of itself, two file headers, three probe rows
reporting a census or a gate, two law rows that were written with the axis in
hand, one ruling and one `answered` using the word for something else. The
other fourteen sit on twelve rows. Two law rows were corrected earlier on this
branch, five name an
axis that is still undeclared (the range's geometry twice, an encoding axis, the
assignment set, and the intermediate fraction width of a staged narrowing), and
one, `proposal::fusing_a_multiply_add_is_free_exactly_at_translation_equivariance`,
names "the domain the cell reaches", which may or may not be
`dimension::ambient_domain` and is left for a seat that has read its source.
Three were stale and are amended:
`proposal::the_realisation_map_has_an_algebraic_character` and
`proposal::the_realisation_map_factors_into_quantisation_and_range_policy`
against `dimension::ambient_domain`, and
`proposal::a_law_is_inherited_where_the_realisation_map_is_a_congruence_for_every_nesting_it_contains`
against `dimension::operand_window`. The twelfth is stale and ratified:
`proposal::at_shared_occupancy_no_per_element_footprint_observation_exists`
says "no dimension row declares one" of occupancy, `dimension::occupancy`
declares it, and `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`
ratifies the row, so it is written down in section 7 and not touched. Its
predicate does not name the axis either, and `dimension::occupancy` says of
itself that a claim not naming it holds nowhere occupancy is defined.

A note saying no `probe` row names its instrument. Four rows and one block
comment in `proposal.toml` said it, and for each a probe row now cites the very
file the note names: `97_probes/p2` is
`probe::one_congruence_property_predicts_every_law_verdict`, `07_probes/p4` is
`probe::narrowing_composes_where_the_modes_direction_switches_at_coarser_grid_points`,
`47_probes/p1` and its siblings are
`probe::a_single_richer_output_suffices_only_as_a_type`, and `80_probes/p1a` and
`p1b` are `probe::a_declared_law_marker_compiles_clean_when_false`. The last is
the plainest: the row's own `evidence` field cites that probe while its note says
"`probe.toml` does not exist". A sixth instance is a law row,
`law::fusing_a_multiply_add_preserves_the_answer_under_unsigned`,
whose gap names three instruments and says no `probe` row exists for them, where
`probe::fusion_under_unsigned_over_six_rounding_modes`,
`probe::equivariance_is_domain_restricted` and
`probe::the_false_unsigned_fusion_clause_and_its_repair` now name one each. All
six are amended, by a sentence naming the probe row, and no `evidence` edge is
drawn, since drawing one restates what the row claims. A seventh sentence of the
kind, in the note of
`ruling::a_composed_expressions_region_is_never_inherited_from_its_parts`, is a
dated account of what that ruling found and is not stale.

A count of the declared dimensions. `proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery`
and `question::can_a_claim_about_the_canons_own_structure_carry_a_region` both
say twenty-one, and the second lists them. There are twenty-five. The four added
since are `occupancy`, `association`, `leaf_aliasing` and `phase`, none of which
ranges over the canon's own contents, so the argument both notes make holds and
each gains a sentence saying so. The other seven lines carrying the word in the
registry count something else.

The shipped identity predicate. One proposal, one question and two probes say
the shipped `has_additive_identity` implements the superseded phase claim, and
`retirement::a_nonzero_phase_removes_the_additive_identity` says the claim still
stands in the crate's design, rustdoc and a test. The crate was corrected:
`arvo-format/src/format.rs:358` asks the cancellation question at each admitted
magnitude, and `arvo-format/src/tests/the_identity.rs:187` holds it equal to an
enumeration at a whole-step biased format, beside a half-step one a line above.
All five rows gain a sentence. What has
not been corrected is the canon: no `ruling` carries `corrects` for the phase
clause of `proposal::membership_of_the_representable_set_is_one_affine_predicate`
(the six `corrects` edges in `ruling.toml` name other rows), so
`question::how_the_stamped_spine_rows_phase_clause_gets_corrected` is open and
the canon is now the one tier that still states the clause. That question's
`bound` also cited the retirement under a slug no retirement has, the superseded
proposal's, and no lint refused it. The slug is corrected and the lint's reach
is ticket `5b32c44d42c6`. The same ticket carries a second shape the review of
this branch found, a registry file's stem written as the namespace, which this
document, the two beside it and one row note carried in sixteen distinct
citations with the lints green. They are rewritten to the namespaces the rows
are in.

A probe note that says sound under a field that says otherwise. Eight probe rows
stand at `uncontrolled`. Three of them,
`probe::chain_the_third_definition_is_not_observation_bounded`,
`probe::no_dependent_survives_the_rounding_units_defects` and
`probe::chain_accuracy_needs_an_intermediate_wider_than_the_operand_type`, open
their note with "Marked sound". Each stood at `sound` until `796fecfa`, the
triage that moved nine instruments onto the value, and the note was left as it
was. Each gains a sentence saying the field governs. No standing changes.

Notes owed and things said of a row that are no longer so. The `answered` of
`question::did_the_admitted_obligations_answer_the_exposure_question_in_the_design`
says `question::one_word_or_two_for_is_a_number_system` "stays open and is owed
a note recording that its first option has already shipped", and the note was
never written. It is now. `probe::the_axis_gap_is_an_open_row_no_seat_of_the_sitting_cited`
establishes that a question row carries no `answered`, and the row has one
since. `retirement::r132_truncation_as_a_mode_name` leaves the naming call "in
op's queue", and `ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names`
is him making it. The header of `question.toml` says no answer is recorded in
the file, and nineteen rows carry an `answered`. Each gains a sentence.

One correction to section 5 below. It speaks of two strategy definitions. The
registry holds five, and a question row is already about three of them:
`question::which_successor_carries_the_strategy_shape_ratification`, decider
`panel`, records that op ratified the lineage of
`ruling::a_strategy_is_a_preset_naming_a_point_with_a_weighting_in_it` and sent
the ratification to its successor, and asks which of
`proposal::a_strategy_is_a_pair_of_an_observable_assignment_and_a_weighting`,
`proposal::a_strategy_is_a_declared_semantics_together_with_a_weighting_over_the_arms_that_realise_it`
and `proposal::a_strategy_is_a_consumer_supplied_weighting_over_measurements_never_over_implementations`
that successor is, or whether it is a fourth nobody has written. All three stand
at one expert, and the second supersedes the first. The two section 5 names are
the two at two experts. So what a strategy is has a blessing waiting on an
identification, and the seats of `PLAN.md` step 2 take the question row and all
five texts, where the first version of the plan gave them two.

## 2. Ratified rows that disagree with each other

None. One pair reads as a contradiction on its `says` fields alone, and the
record resolves it in a field a reader of the first row never reaches.

`ruling::never_a_runtime_check_and_one_lowered_path` (ratified, his I15) says:
"Never any runtime checks, ever: invalids are caught at compile time and unused
paths are cleared when lowered. Unless there is a justifiable reason that is
blessed by a lead designer."

`ruling::the_overflow_panic_is_permitted_and_bounded` (ratified, his I18) says:
"Panic is the go-to for what cannot be caught at compile time through the
typestate, on debug builds, in every strategy rather than only where a native
primitive is being imitated."

A debug-build panic on overflow is a runtime check by any ordinary reading of the
phrase, so the second row describes the thing the first forbids. The second row
answers that itself. Its `because` reads: "The build bound is why it does not
undermine the never-a-runtime-check intent: a shipped program still contains no
runtime validation." Its `note` records that the option he took was the one
worded "the intent bends here, a debug-only panic being the one acceptable
exception", and his `quote` opens "Option 3 but strategy bound". So the reading
that governs is written down, in his name: the first row is about what a shipped
program contains, and the debug panic is the exception he took.

What is missing is one direction of the pointer. The first row names neither the
second row nor the build bound, so a reader who lands on I15 alone gets the
absolute reading, and the generated agent surface repeats it in that form ("no
runtime checks, ever"). The repair is a `note` on the first row naming the
second, which is additive and changes neither claim, and it is proposed in
section 7 because the row is ratified.

An earlier draft of this section, attached to this job's first `working` status,
called the pair unreconciled and listed it as owed by op. That was wrong, and the
error was reading `says` against `says` without the `because` under it.

The other near candidates resolve the same way on reading.
`ruling::each_preset_names_a_stated_intent` and
`ruling::the_strategy_intents_are_not_clear_cut` are both ratified and say
opposite-sounding things about how crisp a preset's intent is, and the second
carries a `supersedes` edge to the first, which is the mechanism working.
`ruling::accuracy_is_never_bought_with_performance` and
`ruling::hot_may_sacrifice_soundness_for_a_proven_meaningful_gain` point in
opposite directions on the same trade and are scoped to different strategies by
their own text.

One more pair sits across two tiers, a ratified row and a stated one, and the
later row names the earlier. `ruling::the_overflow_panic_is_permitted_and_bounded`
makes the panic the go-to on debug builds "in every strategy", and
`ruling::the_debug_abort_is_configured_by_the_consumer`, his words of
13 November at `stated`, makes whether a debug build aborts under a declared
overflow policy the consumer's to configure. The stated row's `because` cites
the ratified one and says why he was asked: a consumer that saturates or wraps
by meaning has a debug build that aborts on the meaning. The ratified row asks
to be read loosely in its own `says`, so the later row narrows it and does not
contradict it, and `Num<S, C>` with `Abort` as the default is the tree's reading
of both.

## 3. Design and principles passages against a ratified row

`mock/PRINCIPLES.md.tmpl` is the file that did not come along when the value
crate landed. The root `mock/DESIGN.md.tmpl` was updated in the same commit and
now says "Five crates"; `PRINCIPLES.md.tmpl:5` still opens "Four crates", and the
two root documents of one tree disagree in their first sentences.

The rest, each with what it says and what governs it.

`PRINCIPLES.md.tmpl:328` says "Default strategy is `Warm` unless called out, and
every semantic alias carries a default for its strategy parameter". No alias in
the tree has a strategy parameter: `mock/crates/arvo-numeral/src/alias.rs` gives
`Uint<BITS, A, C>` and `Int<BITS, A, C>` where `A` is what names an adaptation
and has no default. Here the principle is the oracle and the tree is what is
behind it. On 13 November op chose, for arvo's next steps, the option whose
rework names "aliases keyed on strategy defaulting to `Warm`", and in the same
round, on `Warm` for integers, the option under which "the slice ships `Warm`
unclaimed meanwhile and aliases name their adaptation"; both are quoted whole in
`OPS-WORDS.md`. The tree is at the second of those and not yet at the first, and
`alias.rs` says so in its own header: reading the parameter through a
projection "is what lets a strategy name it later, with `Warm` as its default".
So the sentence stands, and what is owed is the alias keyed on strategy once
`ruling::wrap_or_clamp_stays_open_and_both_get_priced` is closed, with
`ruling::a_shipped_overflow_default_is_a_generic_overridable_per_case` now
beside it. No ratified row carries the alias rule; his chosen option does.

The same option orders the claim before the rework: "claim first (promote the
datum and value definition by a cold second read and the gate), then rework the
same branch in one round". The value crate landed as `9dfd5339` with
`proposal::a_value_is_the_sole_occupancy_placement_of_a_declared_signature`
still at `one_expert` and no ruling ratifying it, so the claim he put first has
not been made. That is ticket `9e02318643e1`.

An earlier version of this section called the sentence at line 328 stale because
the tree has no such alias, which reasons from source against the design he
chose.

`PRINCIPLES.md.tmpl:295` says the bare-primitive discipline "is enforced by the
`arvo-types-only` lint (Task #50, shipping via #103)" and that "Until the lint
lands, the rule holds by convention and review". The lint lands: `cargo mock
lints` reports `arvo-types-only` at `error`, repository scope, from the pack
default. The sentence is stale, and it also carries two task numbers into a
document that renders to the public `docs/` tree, which the workspace's
documentation rule puts in the forbidden set for that tier.

`PRINCIPLES.md.tmpl:71` says "What arvo offers today is the format layer's own
vocabulary, `Width` and `Bool` among it; the fixed-point and exact-width
primitives are being designed and are not there to reach for yet". The exact-width
container is `arvo_bits::Bits<N>` and the fixed-point format point is
`arvo_format::points::UFixed`, both in the tree, and the same file at line 104
onwards describes `Num` and its aliases correctly. The file contradicts itself
about what exists.

`mock/DESIGN.md.tmpl:22` states as principle five that "Every trait whose methods
are sensibly callable in const context is `pub const trait` (RFC 3762 line)". No
crate in the tree declares a `const trait` and none enables a feature gate:
`grep -rE 'const trait|#!\[feature|~const' mock/crates --include=*.rs` returns
nothing, which is what `obligation::the_unstable_machinery_does_not_reach_a_consumer`
asks for and what the value crate's own design explains. The principle describes
the removed tree, where `pub const trait` was everywhere, and it is the clearest
case in these documents of a sentence that survived the architecture it was
written for.

`mock/DESIGN.md.tmpl:3` opens "Numeric primitives + analysis algorithms with Hot
/ Warm / Cold / Precise strategy markers". No analysis algorithm is in the tree,
and `ruling::the_strategy_set_is_not_closed_at_four` holds the four open, so the
tagline names a set the canon explicitly leaves unsettled. The same file's crate
table still calls addition "the one admitted operation" `arvo-format` ships,
against negation, subtraction, the product, division and conversion landed in
`7ec50792` and `2c8e5343`.

The generated agent surface says it too. Every file under `.claude/` and
`.github/` carries the banner "the tree holds the bottom three crates and nothing
above them", which was true before `9dfd5339` and is not now. It is generated
from `mock/agent/`, so the repair is one template and a regeneration, and it is
worth doing before the next helper reads it as the shape of the tree.

## 4. Questions the record already answers, and the ones genuinely open

Of the 110 question rows, 62 carry no `answers` edge from any ruling or proposal.
That count is not a measure of how much is unsettled: several of the 62 are
answered by ratified rows that simply never gained the edge. Those are the
cheapest promotions in the registry, because the work is an edge rather than an
argument.

### Answered by ratified rows, with the derivation

`question::mixed_numeral_addition` asks whether there is an addition that takes
values from two different numerals.
`ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`
is ratified and says an operation is admitted exactly when it is a function of
the declared signature. An addition over two signatures is a function of two, so
it is not an admitted operation of either: it is a conversion onto one signature
composed with that signature's own addition. `arvo-format`'s design already draws
this conclusion in its own words. The edge is owed, not the answer.

`question::are_set_valued_carriers_admitted` asks whether set-valued carriers are
admitted into the number-system concept.
`ruling::a_format_datum_is_one_point_and_a_saturating_end_is_one_point` says a
datum of a format is one point, and
`ruling::a_format_concept_closed_over_v_cross_v_to_v_cannot_state_a_composite_claim`
says stored-pair rationals, intervals and error-carrying values are not format
instances but consume the concept and owe their own laws. Both are ratified.
Together they answer it: scoped out of the concept, admitted above it as
compositions.

`question::is_the_derived_numeral_required_to_be_tightest` asks whether the
derived numeral must be the tightest honest answer.
`ruling::the_work_is_predicated_arms_composed` and
`ruling::there_is_no_universal_answer_take_the_win_and_gate_it` are both his and
both ratified, and they give the shape directly: soundness is required of every
arm, tightness is an arm with a predicate on it. `arvo-format`'s design states it
in those words already.

`question::does_a_named_policy_selection_survive_above_the_numeral` is answered by
`ruling::configuration_is_not_composition_and_a_predicate_never_transports_for_free`
and `ruling::a_composed_expressions_region_is_never_inherited_from_its_parts`,
both ratified: nothing about a part's declared policy transports to a composite
for free, so a selection among named policies does not survive upward of its own
accord, and a composite that wants one declares it.

`question::which_of_the_six_a_retired_word_names_on_each_row` asks which of the
six rounding names a retired word names on the rows that still use it.
`ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names` is
ratified by op and closes the vocabulary at `toward_zero`, `floor`, `ceil`,
`half_up`, `half_even` and `stochastic`. A predicate naming `truncate` names
`toward_zero` under that ruling and nothing else is available. The related
`question::is_the_rounding_vocabulary_complete_at_six`, raised because three
predicates sweep a mode called "away from zero", is answered by
`ruling::half_up_denotes_ties_toward_positive_infinity`, which is ratified and
says in its own text that ties-away is reached as an alias, composed rather than
named: so a predicate sweeping it states a composition and the vocabulary stays
at six.

`question::which_tie_direction_an_unqualified_nearest_names` is only half
answered. The vocabulary ruling forbids a bare `nearest` as a value, so each such
predicate has to name one of the six. Which one it names is a fact about the
instrument that produced it, recoverable from the probe row behind each predicate
and not derivable from the canon. The repair is per row and mechanical, and the
question as written can close once they are done.

### Genuinely open, with who decides

`question::does_warm_wrap_or_clamp` is the one everything downstream waits on,
and its decider is the panel while
`ruling::wrap_or_clamp_stays_open_and_both_get_priced` holds it open in his name.
His words of 13 November, quoted whole in `OPS-WORDS.md`, change its shape rather
than settle it: he says it "shouldn't be called one way", that a default ships
and is easy to override per case, and that the default may follow one of the
parity standards. That is direction on the question and this job records it as a
`stated` ruling row, applied below. What it leaves for the panel is narrower than
before: which standard the default follows, and what a per-case override is
spelled as.

The eight questions whose decider is `measurement` carry no answer and no
`answered` field, and none of them is blocked on anything but running the
instrument: `does_the_rounding_variance_form_hold_at_a_second_fraction_width`,
`does_the_position_keyed_members_monotonicity_failure_rate_differ_from_the_independent_members`
and their six siblings. The bench harness runs, which
`mock/research/202611130200_bench-harness-at-head/FINDINGS.md` records, so these
are work rather than questions.

The three whose decider is `coordinator` are about the panel's own process
(`what_stops_two_files_by_one_persona_reading_as_two_experts`,
`does_a_cold_brief_withhold_rows_the_proposal_author_wrote`,
`is_the_non_terminating_contention_crate_a_defect`). They are unanswered and they
gate how every other answer gets promoted, which makes them cheaper to settle
first than their subject matter suggests.

All 13 of the questions whose decider is `op` carry either an answering ruling or
an `answered` field. Nothing the registry reserves for him is sitting unanswered
in it. Section 8 says what that leaves owed by him, which is nothing now.

## 5. One-expert proposals against a ratified row or against his words

### The five live refusals that say an old crate does not return

`proposal-the-value-stance.toml` holds seven refusal rows of this shape. Two are
already superseded inside the file: `the_old_comb_crate_is_gone` by
`the_old_comb_crate_answers_to_a_stated_need`, and
`the_old_transparent_crate_is_gone` by
`the_old_transparent_crates_work_is_each_value_types_own_doors`. Five are live:
`the_old_bit_contracts_crate_is_gone`, `the_old_mask_contracts_crate_is_gone`,
`the_old_numeric_contracts_crate_is_gone`, `the_old_refit_crate_is_gone` and
`the_old_storage_crate_is_gone`. All are at `one_expert`, so none is canon.

His words of 13 November: "I want them to return, in function/feature, but the
shape the contracts and traits and default impls etc take are up for a whole
rewrite and also the taxonomy."

Read against each row, all five agree with him on the function. Each says a
crate does not return as a crate, and each `instead` re-homes what the crate
carried: `the_old_storage_crate_is_gone` onto `arvo-bits`, value aliases and
the capacity crate, `the_old_mask_contracts_crate_is_gone` into the masks crate
of slice two, `the_old_refit_crate_is_gone` onto `convert` and `cast`,
`the_old_bit_contracts_crate_is_gone` onto `arvo-bits` and `convert`, with its
`because` saying `BitAccess`, `BitLogic` and `BitSequence` "return with the masks
over `arvo-bits`", and `the_old_numeric_contracts_crate_is_gone` onto the value
crate's compare, `min`, `max` and `constant` doors and a refined divisor. What
each fixes beyond that is a taxonomy, which crate and which name, and he has
opened the taxonomy to a whole rewrite, so no row's home is binding.

One clause meets the end of his sentence. `the_old_numeric_contracts_crate_is_gone`
puts `Abs`, `Recip` and `Sqrt` "with the elementary functions, which stay in
kaski in slice one", and his sentence ends "However, definitely arvo terrain to
write the actual arithmetic contracts on the standard too". Those three are
arithmetic contracts, and for the first slice the row leaves them with a
consumer. The row does not send arvo's arithmetic contracts to a consumer as a
whole, and the clause is scoped to one slice, so this is one clause to restate
and not a row against his direction.

An earlier version of this section called the bit-contracts row and the
numeric-contracts row "against him as written", on the ground that the first
re-homed nothing and the second sent the arithmetic contracts to kaski. Both
readings were wrong against the rows' own `instead` fields, and the proposal in
section 7 to supersede the bit-contracts row went with them.

This job proposes no edit to their standing, which is forbidden to it, and adds
his words as a `stated` ruling row instead, which is what makes the five readable
correctly by the next seat.

### The two strategy definitions that are each at two experts and disagree

Section 1.10 corrects this subsection in one respect: the registry holds three
more definitions at one expert, and
`question::which_successor_carries_the_strategy_shape_ratification` is already
about them. What follows is true of the two it names.

`proposal::a_strategy_is_an_assignment_and_a_weighting` says a strategy is a
pair: an assignment on observable policy axes that fixes the denoted answer, and
a weighting over cost coordinates that ranges over realisations of it.
`proposal::a_strategy_is_a_preference_over_measurements_resolved_as_a_compile_time_argmin`
says a strategy is a preference over measurements resolved as a compile-time
argmin, and that "the container, the codegen choice and the overflow rule are
effects of that one cause rather than three components of a marker".

Both stand at `two_experts`, which the registry's own vocabulary calls the
strongest standing a proposition reaches before a ruling. They disagree on where
the overflow rule sits: a declared component of the strategy in the first, an
effect of the weighting in the second. That difference decides whether an alias
can carry a strategy and get an adaptation from it, which is the open end of
every alias in `arvo-numeral`, so it is not an academic disagreement.

Neither is canon and the coordinator's gate has promoted neither, which is the
mechanism working rather than failing. What is missing is that neither row names
the other, so a reader meeting one has no way to know the other exists. The
repair is a `note` on each, additive and proposed below.
`ruling::the_strategy_is_what_makes_an_answer_correct` favours the first reading,
and saying so is the panel's to do rather than this job's. The second reading is not
without support: `proposal::no_cost_model_may_move_an_answer`
states its half, and is itself a proposal.

### One more worth naming

`proposal::the_first_slice_answers_to_hilavitkutin_and_kaski`
measures a slice's completeness by two consumers. It is superseded inside its own
file by `no_row_measures_a_slices_completeness`, and
`ruling::arvo_is_a_library_and_the_value_composes_on_top` is ratified and names
"pretty much every single repo and project I have, downstream". The supersession
is in place, so this is noted for a reader who meets the earlier row first.

## 6. The canon tool

arvo has no canon query tool. `cargo mock query` reads the registry generically,
`cargo mock links` walks a row's edges, and five project tools answer narrower
questions (`awaiting-a-ruling`, `unasked-questions`, `rulings-with-no-verbatim`,
`rounding-vocabulary`, `the-positions`, `ask`, `coverage`, `near-copies`). What
does not exist is the `canon find` and `canon related` surface the workspace
rules name, which every brief tells an agent to use before it greps and which
this job had to work around by reading the files.

The workaround cost is measurable and it is the argument for the tool. Answering
"which rows speak to the overflow default" took a full read of
`ruling.toml` because no command ranks rows by subject, and the question-state
table this job built (which ruling or proposal answers each question) is a
fifteen-line script that nothing in the repo offers. That script is the shape of
the missing tool: it is `coverage` for the question namespace, where today
`cargo mock coverage question` reports every question as uncovered because the
command is built for obligations.

The tools are being upstreamed to `orgrinrt/mockspace-extra-lints` and nothing
is to be copied into arvo, so this is recorded rather than built. Two things are
worth upstreaming in the order given: a question-coverage report, which is
mechanical and catches the cheapest promotions in the registry; and a subject
search over rows, which is what every brief already assumes exists.

## 7. The edits, exactly

The job's rule is that nothing is applied that changes a `ratified` row or any
row's standing. What follows is in three groups: what the first branch applied,
what the second applies, and what stays written down for whoever may apply it.

### Applied on the first branch, landed on `dev` as `326b52f5`

One new `ruling` row carrying his 13 November words on the overflow default,
`a_shipped_overflow_default_is_a_generic_overridable_per_case`, at
`rung = "stated"`, topic `overflow_policy`. It carries no `answers` edge,
because it narrows `question::does_warm_wrap_or_clamp` without settling it, and
an edge would make a live question read as answered.

One new `ruling` row carrying his 13 November words on the old families,
`every_family_of_the_old_arvo_returns_and_the_shape_is_open`, at
`rung = "stated"`, topic `arvo_identity`.

Three header comments: the paragraph in `ruling.toml` about unset edge fields
rewritten to say which carry edges now, and the counts dropped from the two
comments in `dimension.toml` and the one in `topic.toml`.

### Applied on the second branch, after the review

On the two rows above. `kind` moves from `intent` to `ruling` on both, since the
schema keeps `intent` for a want stated with no question in front of it and both
answer a question put with options. The quote of the first gets back the
typographic apostrophe of "it’s", which the first branch had straightened while
saying it kept it. The `says` of the first keeps his hedge, "it might make more
sense", where it had been written as a settled statement, and the `says` of the
second loses a gloss of the taxonomy that was this job's reading and not his
words. The note of the first states the sketch's corrected result, and the note
of the second the corrected reading of the five refusals in section 5. Whether
`ruling` is the kind for the first, which narrows the question put to him and
carries no `answers` edge, is a call the reviewer of this branch left to a
second reader, and the row keeps the kind until one has read it.

On `obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`,
the second paragraph of the `gap` is replaced: on scope it names his direction,
`ruling::the_standards_bound_starts_at_two_and_reserves_the_rest`, as a `stated`
row and not a ratified one, the MATLAB
half of the parity suite is built and its two files are named, and the IEEE 754
half is unbuilt and waits on a float value.

On `ruling::the_trait_contract_structure_is_a_primary_paradigm`, a `stated`
row, the note is amended: the lint and the contracts crates it argued from are
gone, so nothing in the tree enforces the row today.

On `law::additive_associativity_under_saturation` and
`law::distributivity_of_multiplication_over_addition`, the `gap` says the axis
is declared and names the row that holds the cell. On
`law::coherence_of_a_reduction_onto_its_induced_operation`, the `gap` says the
axis is declared and no row holds the cell yet. On
`law::quantise_then_reduce_commutes` and
`law::the_fused_and_the_stepwise_multiply_add_denote_one_function`,
the `gap` says the axes are declared and the region has not been written from
its source. No `holds` or `fails` list changes.

On `proposal::a_strategy_is_an_assignment_and_a_weighting` and
`proposal::a_strategy_is_a_preference_over_measurements_resolved_as_a_compile_time_argmin`,
a paragraph in the `note` of each naming the other, the axis they disagree on,
and `probe::the_rationalisable_sections_on_the_committed_carrier_table`, which
counts the gap between the two definitions. Neither standing changes.

### Applied on the second branch, after the full read

Every one is a sentence added to a `note`, a `gap`, a `why`, a `replacement` or
a comment, on a row no ruling ratifies. No `says`, predicate, `holds`, `fails`,
`evidence` or standing changes, and section 1.10 gives the reason for each.

That an axis is declared now: the note and the gap of
`proposal::the_realisation_map_has_an_algebraic_character`,
the note of
`proposal::the_realisation_map_factors_into_quantisation_and_range_policy`,
and the note of
`proposal::a_law_is_inherited_where_the_realisation_map_is_a_congruence_for_every_nesting_it_contains`.

That a `probe` row names the instrument now: the notes of that same row, of
`proposal::staged_narrowing_disagrees_with_direct_narrowing_under_round_to_nearest_even`,
of `proposal::an_output_of_a_derivation_is_a_fact_a_downstream_site_cannot_recover`
and of `proposal::a_law_stated_as_an_author_written_marker_is_checked_by_nothing`,
the block comment above the first three, and the gap of
`law::fusing_a_multiply_add_preserves_the_answer_under_unsigned`.

That four axes were declared after the count of twenty-one: the notes of
`proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery` and
`question::can_a_claim_about_the_canons_own_structure_carry_a_region`.

That the crate's identity predicate was corrected and the canon's clause was
not: the note of
`proposal::the_additive_identity_is_decided_by_the_phase_being_a_whole_multiple_of_the_quantum`,
the note of `question::how_the_stamped_spine_rows_phase_clause_gets_corrected`,
a new `note` on each of `probe::the_identity_predicate_swept_against_an_enumeration`
and `probe::the_shipped_crate_admits_a_whole_step_phase`, and the `replacement`
of `retirement::a_nonzero_phase_removes_the_additive_identity`. In the `bound`
of the same question, the retirement's slug is corrected.

That the field governs: the notes of the three `uncontrolled` probe rows 1.10
names.

The rest: the owed paragraph on the note of
`question::one_word_or_two_for_is_a_number_system`, a sentence on the note of
`probe::the_axis_gap_is_an_open_row_no_seat_of_the_sitting_cited`, a sentence on
the `why` of `retirement::r132_truncation_as_a_mode_name`, and a paragraph in
the header of `question.toml` saying rows may carry `answered` and `bound`.

### Applied on the second branch, after its own review

The review of the second branch found that the notes amended after the full read
had been repaired by adding sentences that would go stale in their turn, each
saying an axis or a `probe` row exists "now" and that the edge or the wider row
"has not been" written. Twenty-one rows and one file comment are reworded so each note says what
its row carries and does not carry, with no word that depends on when it is
read, and the work those notes leave undone is ticket `2cda83cec7ff`. No
`says`, no predicate, no edge, no `rung` and no standing changes in them, and
none of the rows is ratified.

Three more come from the same review. The `gap` of
`obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`
had called the scope settled by a `stated` row, which
`ruling::an_ack_is_not_a_ratification` does not allow, and it now names that row
as his direction. The note of
`ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open` had said
all five refusals agree with him on the function, and it now says four do and
names the one clause of the fifth that conflicts, with the proposal that
restates it. And the note of
`proposal::a_strategy_is_a_preference_over_measurements_resolved_as_a_compile_time_argmin`
cited a row under its file's stem, which is corrected to `proposal::`.

### Written down and not applied

The edits to ratified rows were one patch of twenty-one hunks, changing no
`says`, no `rung` and no standing. Op was shown them on 13 November and asked
"Do the eighteen get `ratified_by = "op"` and the three notes their
corrections?", with the options [All of it] [The eighteen, not the notes]
[Some are not mine] [Leave them]. He answered "The eighteen, not the notes".
So the patch is now two files beside this one.

`ratified-rows.patch` holds the eighteen hunks that add `ratified_by = "op"` to
the rows of 1.2, which is what he took. It is still not applied: this seat's
permission check refused the edit a second time, after his answer, and that is
ticket `c9e9da26606f` with the refusal quoted in the status that carries this
branch. `git apply` takes the file on top of this branch, and the lint refusing
a ratified ruling with no `ratified_by` lands behind it.

`ratified-notes.proposal.patch` holds the three note rewrites he did not take.
They stay a proposal and nobody applies them on this answer.

One amends the note of
`ruling::a_platform_width_type_is_a_target_bound_member_of_the_format_family`:
the region clause reads that `W = 64` was refused by the ladder as it shipped
when the row was ratified and is established since `d425aff5`, and a closing
paragraph says the two paragraphs on the 62-bit ceiling describe the
representation of that day.

One amends the note of
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`
to say the design correction it owed has landed, citing the design's heading
in place of a line.

One adds a paragraph to the note of
`ruling::never_a_runtime_check_and_one_lowered_path` naming
`ruling::the_overflow_panic_is_permitted_and_bounded` and the build bound its
`because` states.

The `proposal` row this section used to carry as written down is applied now,
as `proposal::the_old_numeric_contracts_return_as_arvos_own_and_the_crate_does_not`,
at `one_expert`, on the model of `the_old_comb_crate_answers_to_a_stated_need`
and with a `supersedes` edge onto `the_old_numeric_contracts_crate_is_gone`. It
says that the crate does not return and what it carried does, with the
contracts for absolute value, reciprocal and square root as arvo's own, written
on the standards, and where the elementary functions behind them are first
implemented left undecided. It rests on
`ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open` and on the
one clause section 5 names, and it touches no ratified row. A second reader
has not seen it.

From the full read, four more things are written down for seats that may do
them. The note of
`proposal::at_shared_occupancy_no_per_element_footprint_observation_exists`
wants the sentence that `dimension::occupancy` is declared, and its predicate
wants a second reading on whether it should name that axis, the row being
ratified. The `evidence` edges the amended notes say their rows do not carry are a panel
seat's to draw, and so are the rows and regions those notes say are not written:
the general rule on which rounding modes compose across nested grids, the
measured half of the derivation-output finding, the licence claims and the
obstruction result over the ambient domain, the commutation region, the clause
over the accumulator width and the operation set, and the saturating cell over a
nonnegative window. Ticket `2cda83cec7ff` lists each with the row it sits on. And the phase clause of the stamped
spine row waits on `question::how_the_stamped_spine_rows_phase_clause_gets_corrected`,
whose first option is a `ruling` carrying `corrects` on two agreements.

The first version of this section also proposed a row superseding
`the_old_bit_contracts_crate_is_gone`. That row re-homes what its crate carried,
so nothing in his answer speaks against it, and the proposal is withdrawn.

## 8. What is owed by op

Nothing now, and one thing later.

The first draft of this file listed three. Each was checked again against his
own words before being handed to him, and none survives as his.

The runtime-check pair is reconciled in the record, as section 2 now says.

Which parity standard a shipped default follows is the work's to find. His words
are "perhaps following one of the parity standards like matlab or whatever would
be the sensible default to use", which states the intent, a default that follows
a parity standard, and hands the choice of which to whoever can tell what is
sensible. The panel has what it needs: MATLAB is the only standard in scope that
supplies both halves of a default today, `Nearest` and `Saturate`, and the sketch
shows the value tier admits that pair. The standing question it sits under,
`question::does_warm_wrap_or_clamp`, has `panel` as its decider.

What happens to the five live refusals is not a choice either. He answered that
the families return in function and feature, all five rows re-home the function
their crate carried, and the one clause that leaves three arithmetic contracts
with a consumer is restated by a one-expert proposal row, as section 7 says,
and is a second reader's to confirm.

The one thing that is his is the public name, and he has already said when: on
13 November, among a thin crate named `arvo` that re-exports, no facade with
consumers importing the inner crates, and deciding at the port with the import
list in hand, he answered "Decide at the port". `PLAN.md` carries it on the step
that ports hilavitkutin, with the import list counted.

The searches behind "nothing now". `cargo mock unasked-questions` is this repo's
own tool for the question and reports: "13 of 110 questions name op, and 13 of
those a ruling has already answered. Nothing is waiting on him. Every question
that names him has an answering ruling." Its control is the count itself, which
`grep -c 'decider = "op"' mock/registry/question.toml` reproduces at 13.
`cargo mock awaiting-a-ruling` lists the 61 rows that are his direction recorded
as an ack, the two added here among them, which is the other half: direction of
his that no ruling has yet stamped is the panel's to carry, by
`ruling::the_panel_finishes_the_canon_without_him`.
