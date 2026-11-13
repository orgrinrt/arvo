# What the canon says that is no longer true, and what it never settled

This is the second part of job `arvo-canon-check`: the canon read whole at
`origin/dev` tip `2c8e5343`, checked for being current and for being consistent
with itself. `LEDGER.md` beside it says what was read and how deep, which is the
only thing that bounds the absence claims made here.

The registry is in good order. Nothing here is a ratified row found wrong on its
subject. What is found is narrower and ordinary for a canon under construction:
notes and gap fields that describe a tree the work has since moved past, a schema
field that half the ratified rows do not carry, a set of design passages that
were not brought along when the value crate landed, and a handful of one-expert
proposals that op's own words of 13 November now speak against. Most of the
repairs are a sentence each.

Two disciplines govern what follows. Rows are named by namespace and slug and
never by line, because the registry gains rows constantly and a line keeps
resolving while naming something else. Source is named by `file:line`, which is
the permitted form for a tree that moves under a git sha. Where a finding says
the record is silent, that is a claim about the registry, which was read whole,
and not about the panel corpora, which were not.

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

`ruling::the_standards_bound_starts_at_two_and_reserves_the_rest` answers the
first half in his own words: MATLAB `fi` and `fimath` and IEEE 754 are the two,
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
line 235 of that file is a list of open questions, and the design states the
position rule the note described as a proposal at line 404: "a bare primitive
appears only in the constructor and the accessor of a primitive this crate
defines". The correction the note asks for has happened.

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
`law-the-later-topics::quantise_then_reduce_commutes`, whose gap says "no
`dimension` row declares an axis for the ambient domain" against
`dimension::ambient_domain`, and for
`law-the-later-topics::the_fused_and_the_stepwise_multiply_add_denote_one_function`
against `dimension::accumulator_width`.

These are the most valuable repairs in this section, because each moves a
measured cell out of a gap and into a `holds` list, which is where a later arm
can gate on it. The spelling the dimension row's grammar admits is
`operand window = declared non-negative`.

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

## 3. Design and principles passages against a ratified row

`mock/PRINCIPLES.md.tmpl` is the file that did not come along when the value
crate landed. The root `mock/DESIGN.md.tmpl` was updated in the same commit and
now says "Five crates"; `PRINCIPLES.md.tmpl:5` still opens "Four crates", and the
two root documents of one tree disagree in their first sentences.

The rest, each with what it says and what governs it.

`PRINCIPLES.md.tmpl:328` says "Default strategy is `Warm` unless called out, and
every semantic alias carries a default for its strategy parameter". No alias in
the tree has a strategy parameter at all: `mock/crates/arvo-numeral/src/alias.rs`
gives `Uint<BITS, A, C>` and `Int<BITS, A, C>` where `A` is what names an
adaptation and has no default, and the crate's own design says why, citing
`ruling::wrap_or_clamp_stays_open_and_both_get_priced`. This one matters more
than the others because `arvo-numeral`'s design leans on this sentence as its
authority for the default it intends to add later, so a design derives from a
passage of another design that the tree contradicts, and no ratified row carries
the alias rule at all.

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

Read against each row, the picture is not uniform and the distinction is the
useful part. A row that says a crate does not return as a crate, and whose
`instead` re-homes the function elsewhere, agrees with him: the function returns
and only the taxonomy differs, and he has opened the taxonomy. That covers
`the_old_storage_crate_is_gone`, `the_old_mask_contracts_crate_is_gone` and
`the_old_refit_crate_is_gone`, whose `instead` fields put conversion and bit
access on crates that exist.

Two are against him as written. `the_old_bit_contracts_crate_is_gone` says
flatly "does not return", and the function it carried, the traits a consumer
bounds generic code on, is the one
`obligation::the_surface_expressible_as_contracts_before_anything_implements_it`
demands and `ruling::arvo_is_a_library_and_the_value_composes_on_top` names as
the selling point. `the_old_numeric_contracts_crate_is_gone` is the sharpest
case: its `instead` sends absolute value, reciprocal and square root to kaski for
the first slice, and his sentence ends "However, definitely arvo terrain to write
the actual arithmetic contracts on the standard too". A row that re-homes arvo's
arithmetic contracts into a consumer is against his direction on exactly the
subject he named.

This job proposes no edit to their standing, which is forbidden to it, and adds
his words as a `stated` ruling row instead, which is what makes the five readable
correctly by the next seat.

### The two strategy definitions that are each at two experts and disagree

`proposal-the-later-topics::a_strategy_is_an_assignment_and_a_weighting` says a strategy is a
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
without support: `proposal-the-later-topics::no_cost_model_may_move_an_answer`
states its half, and is itself a proposal.

### One more worth naming

`proposal-the-value-stance::the_first_slice_answers_to_hilavitkutin_and_kaski`
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

Applied on this branch, all additive or comment-only, none touching a ratified
row or any standing.

One new `ruling` row carrying his 13 November words on the overflow default,
`a_shipped_overflow_default_is_a_generic_overridable_per_case`, `kind = "intent"`
at `rung = "stated"`, topic `overflow_policy`. It carries no `answers` edge,
because it narrows `question::does_warm_wrap_or_clamp` without settling it, and
an edge would make a live question read as answered.

One new `ruling` row carrying his 13 November words on the old families,
`every_family_of_the_old_arvo_returns_and_the_shape_is_open`, `kind = "intent"`
at `rung = "stated"`, topic `arvo_identity`.

Three header comments: the paragraph in `ruling.toml` about unset edge fields
rewritten to say which carry edges now, and the counts dropped from the two
comments in `dimension.toml` and the one in `topic.toml`.

Proposed and not applied, in the registry's own shapes.

On each of the eighteen ratified rulings in 1.2, add `ratified_by = "op"`.
Forbidden to this job because it edits a ratified row; mechanical for whoever may.

On `obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`,
replace the `gap` with one that says the scope is settled by
`ruling::the_standards_bound_starts_at_two_and_reserves_the_rest`, that the MATLAB
parity suite is built and names its file, and that the IEEE 754 half is unbuilt
and blocked on a float value.

On `ruling::a_platform_width_type_is_a_target_bound_member_of_the_format_family`,
amend the note's region clause to read that `W = 64` was refused by the ladder of
the day and is established since `d425aff5`, keeping both.

On `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`,
amend the note to say the design correction landed, and replace the line citation
with a heading citation.

On `law::additive_associativity_under_saturation`,
`law::distributivity_of_multiplication_over_addition` and
`law::coherence_of_a_reduction_onto_its_induced_operation`, move the
declared-non-negative cell from `gap` into `holds`, spelled
`operand window = declared non-negative`, as `dimension::operand_window`'s own
note already specifies. Likewise `quantise_then_reduce_commutes` against
`dimension::ambient_domain` and
`the_fused_and_the_stepwise_multiply_add_denote_one_function` against
`dimension::accumulator_width`.

On `ruling::never_a_runtime_check_and_one_lowered_path`, a `note` naming
`ruling::the_overflow_panic_is_permitted_and_bounded` and the build bound its
`because` states, so the absolute reading is not what a reader of this row alone
takes away.

On `ruling::the_trait_contract_structure_is_a_primary_paradigm`, amend the note:
the lint and the contracts crates it argues from are gone, so the reason it gives
for not promoting the row no longer exists.

On `proposal-the-later-topics::a_strategy_is_an_assignment_and_a_weighting` and
`proposal::a_strategy_is_a_preference_over_measurements_resolved_as_a_compile_time_argmin`,
a `note` on each naming the other and the axis they disagree on.

Two new `proposal` rows in `proposal-the-value-stance.toml`, on the model of
`the_old_comb_crate_answers_to_a_stated_need`, each with a `supersedes` edge.
One superseding `the_old_bit_contracts_crate_is_gone`: what that crate carried,
the contracts a consumer bounds bit-level generic code on, returns, and which
crate holds them under which name is open. One superseding
`the_old_numeric_contracts_crate_is_gone`: the arithmetic contracts return as
arvo's own, and none is re-homed in a consumer. Both rest on
`ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open` and on
hilavitkutin's own imports of those names, counted in `PLAN.md`. They are
proposed rather than applied because a `supersedes` edge moves the superseded
row's standing in everything but the field.

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
supplies both halves of a default today, and the sketch shows the value tier
admits it. The standing question it sits under,
`question::does_warm_wrap_or_clamp`, has `panel` as its decider.

What happens to the five live refusals is not a choice either. He answered that
the families return in function and feature, and the two rows that say otherwise
are superseded on that answer by whoever may write the superseding rows, which
section 7 proposes.

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
