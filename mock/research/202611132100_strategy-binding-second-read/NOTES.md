# The strategy binding in the root documents, a second read

The root documents describe `arvo-strategy` as binding a name to a placement
objective and an adaptation selection. This note asks, sentence by sentence,
whether that wording stays within the canon, and in particular whether any
sentence presents the two-part binding as what a strategy is rather than as
what the crate holds today. It is one independent reader's answer, written
without the earlier reader's verdict, its ticket, or the history of the
sentences.

The canon here is the typed rows under `mock/registry/*.toml`, declared as
`canon_paths` in `mockspace.toml`. Every row cited below was read with
`cargo mock query`, and the strategy namespace from its declaration in
`mockspace.toml` and its rows in `mock/registry/strategy.toml`. The shipped
tree is treated as claims under test.

## The canon gate

The question is aligned with the canon and answerable from it. One correction
to the brief: `ruling::a_strategy_is_a_preset_naming_a_point_with_a_weighting_in_it`
carries `topic = "the_strategy_object"`, not `the_strategy_axis`. It was read
with the rest regardless, and it is the row that decides the question.

## What the canon says a strategy is

`ruling::a_strategy_is_a_preset_naming_a_point_with_a_weighting_in_it`, rung
`stated`: a strategy is mostly a preset naming a point in a space of
independent axes, with a little of being nothing but a weighting over
measurements mixed in, "Both, and not either". Its note calls it a starting
shape rather than a settled answer.

`ruling::the_strategies_weigh_measurements_differently`, ratified: every
strategy is decided by measurement and they weigh measurements differently.

`ruling::the_strategy_set_is_not_closed_at_four`, rung `open`: the number, the
names and the decomposition are open. Under the canon's own reading of `open`,
the decomposition is a reserved call and may not be filled inside a design or
a document.

`ruling::which_half_of_the_pair_i9_attaches_to_is_not_his`, a stated deferral:
whether a strategy is a pair, and which half the correctness intent attaches
to, is implementation detail for the experts to converge on. Its note records
that the decomposition into a pair is the panel's invention.
`question::is_i9_about_a_strategy_or_one_component` stays live on the panel.

`ruling::the_strategy_intents_are_not_clear_cut`, ratified, and
`ruling::what_is_ratifiable_is_the_intent_not_the_concretes`, stated: the
per-strategy statements are not to be written as settled, and only the intent
is ratifiable.

The two-part shape exists in the registry only as proposals.
`proposal::a_strategy_is_an_assignment_and_a_weighting` (two experts, topic
`the_strategy_object`) defines a strategy as a policy assignment plus a
weighting over cost coordinates, and
`proposal::a_strategy_is_a_preference_over_measurements_resolved_as_a_compile_time_argmin`
(two experts, topic `the_strategy_axis`) refuses components and calls the
overflow rule an effect. Neither is canon, and neither names a placement
objective. The phrase "placement objective" has zero hits across
`mock/registry/` and `mockspace.toml`; the same search over
`mock/crates/arvo-strategy/` returns four, which is the control that the
search itself works. The code holds the pair as `trait Strategy` with two
associated types, `Objective` and `Adaptation`, at
`mock/crates/arvo-strategy/src/lib.rs:41`. That is a fact about the
implementation and says nothing about its correctness.

## Per site

`mock/DESIGN.md.tmpl` line 44, the crate table: "The binding from a name to a
placement objective and an adaptation selection. Four presets ship as instances
of an open concept rather than as members of an enumeration." Within the
canon. The column is "What it ships", so the sentence reads as crate content,
and the open-concept clause matches `ruling::the_strategy_set_is_not_closed_at_four`.

`mock/DESIGN.md.tmpl` lines 49 to 50: "a new strategy by implementing
`Strategy`". Not decidable from the canon, and leaning outside it. It equates
being a strategy with implementing today's trait, whose two associated types
are the binding, so by implication it makes the binding the definition and
closes the decomposition the open row reserves. Better carried as "a new preset
by implementing the `Strategy` contract the crate has today".

`mock/DESIGN.md.tmpl` lines 74 to 75 and line 90 ("Choosing between placements
needs `arvo-strategy`", "binds a name to a choice between them"): within. Both
describe what a consumer depends on and what the crate binds. They are about
the crate.

`mock/PRINCIPLES.md.tmpl` lines 11 to 12, "`arvo-strategy` depends on both and
binds a name to a placement objective and an adaptation selection": within. The
subject is the crate.

`mock/PRINCIPLES.md.tmpl` lines 347 to 350, "A strategy is not decoration. As
`arvo-strategy` has it today, it binds a name to a placement objective ... and
to an adaptation selection": within. The scoping clause "as `arvo-strategy`
has it today" is exactly what keeps it a claim about the crate. Line 353's
default of `Warm` rests on `ruling::each_preset_names_a_stated_intent`, which
calls Warm the compromise that suits most default cases. Line 371, the crate
binds a name to a choice between placements, is within for the same reason as
the DESIGN dependency sentences.

`mock/WORKFLOW.md.tmpl` lines 13 to 16: within, about the crate.

`mock/WORKFLOW.md.tmpl` lines 28 to 32, "The strategy is a marker of its own
in `arvo-strategy` ... and each binds a placement objective and an adaptation
at the type level": within, narrowly. "Each" ranges over the four shipped
markers, so it describes the presets the crate ships. The one-phrase
descriptions (speed first, accuracy, footprint) are near the stated intents and
do not read as settled concretes. It would be safer with the same "as the
crate has it today" clause PRINCIPLES carries, since a reader can take "the
strategy is a marker" as a definition.

`mock/agent/rules/implementation.md.tmpl` lines 14 to 17, the heading "A
strategy is a binding, and its presets are not a table" and "A strategy maps a
name onto a placement objective and an adaptation selection. It names no
implementation and owns none, and the ladder it points into is keyed on the
objective rather than on the name." Outside the canon. It is unscoped and
definitional: it says what a strategy is, not what the crate holds. The canon's
definition row is a preset naming a point in a space of axes with a weighting
in it, the decomposition is reserved by an `open` row, the pair is the panel's
invention by a stated deferral, and the only pair shapes in the registry are
proposals whose components are not these two. The rest of that section (no
behaviour table keyed on a preset name, the open set, overflow left open) is
within and correctly cited.

## Is a row owed

No. A row making the two-part binding the definition would fill
`ruling::the_strategy_set_is_not_closed_at_four`, which reserves the
decomposition, and it would narrow
`ruling::a_strategy_is_a_preset_naming_a_point_with_a_weighting_in_it` to two
named axes with no weighting in it, against "Both, and not either". Nothing
stated or ratified names a placement objective as a component, and the two
proposals that do propose pairs disagree with each other and with the crate's
pair. The canon already says what a strategy is, so what is owed is wording.

The outside site, `implementation.md.tmpl`, should carry instead: "As
`arvo-strategy` has it today, a strategy binds a name to a placement objective
and an adaptation selection. What a strategy is in the canon is
`ruling::a_strategy_is_a_preset_naming_a_point_with_a_weighting_in_it`, and its
decomposition is open under `ruling::the_strategy_set_is_not_closed_at_four`."
The heading would follow, for example "Today's strategy is a binding, and its
presets are not a table". The leaning site at `DESIGN.md.tmpl` lines 49 to 50
should say "a new preset by implementing the `Strategy` contract".

## Confidence and what would move it

High on `implementation.md.tmpl` being outside, around 0.85. Medium-high on the
DESIGN line 49 reading, since it depends on whether "a new strategy" is read
as an instance of the trait or as a new concept. Within verdicts on the crate
sentences are high. The single fact that would change the answer is a stated
or ratified row, or op's own words, naming a placement objective and an
adaptation as the components of a strategy; with that, the definitional
wording would be within and the scoping clauses would be redundant.
