# 209. McSherry: which of the twenty-nine questions are actually his

Twenty-nine `question` rows named op as the decider. Thirteen do.

Six of the twenty-nine already have a ruling answering them, in his own words,
recorded in the registry, and were sitting in the queue anyway. Eleven were never
his: they are maths, soundness, or a question whose shape he has already refused
in writing, and each of those now names the panel and carries the constraint it
is answered inside. One question that is genuinely his existed nowhere in the
namespace at all, and it is the one bounding the two questions four topics defer
to.

The number that matters is not that eleven moved. It is that **nothing anywhere
in the repository read the fields that carry any of this**, so the queue could
not have been right and was not. That is fixed, with an instrument rather than a
document, because a document goes stale and this one already had.

## Method

The canon gate passed, checked against `mock/registry/*.toml` as declared by
`canon_paths` in `mockspace.toml`, and against the schema's own definitions of
`decider` and `bound` in that same file. Nothing in the assigned work conflicts
with it. The dispatch's own premise, twenty-nine rows at `decider = "op"`,
checks out: `question.where(decider=op).count()` returns 29 of 94.

The test gate ran before anything else. `cargo mock` green with all lints
passing, `cargo test -p arvo-checks` at 106 passed, 0 failed, 0 ignored across
sixteen test files, which is the count as I found it. I read the bodies rather than the names, and found no
tautology, no sampled law and no test asserting only that a call returned. The
suite is real. Two things about it are worth saying anyway, and both are gaps
rather than defects: no arm anywhere reads the fields this dispatch is about, and
`what_one_field_obliges_another_to_carry.rs` at 742 lines is well past the
workspace's 500-line smell, carrying what reads as several concerns.

The classification test is the schema's, quoted from `mockspace.toml`: `op` where
it is his intent and nobody else's, `measurement` where the harness settles it,
`panel` where it is derivable and wants argument, `coordinator` where it was
answered in his stead. Per row I asked whether it is a taste call, a naming call
or a statement of what he wants, which are his, or a derivation, a soundness
question, a maths question or a consequence of something already ratified, which
are not.

Searching before concluding anything was unanswered: the registry in the words an
answer would use, `.data/op-responses/` in the workspace repository, and the
`ruling` namespace's `answers` edges. The op-responses corpus was where the round
this dispatch descends from turned up, along with a ruling batch from the same
day. Every search that returned zero got a positive control on the same
instrument in the same run before I believed it.

## What nothing was reading, which is the finding under the finding

A `question` row carries three fields that between them say who owes an answer:
`decider`, `bound`, and the incoming edge from a `ruling`'s `answers`. At the
commit this seat branched from, **no check, no lint and no tool named any of
them**.

The probe is committed at
`209_probes/which_registry_fields_the_tooling_reads.sh`, with its output beside
it. It searches the quoted string literal a check would use, over `mock/checks`,
`mock/lints` and `mock/tools`, at that commit and at HEAD:

```
FIELD          BEFORE   AFTER
decider        0        11
bound          0        6
unblocks       0        6
answers        0        2

--- control: fields already read at the base ---
rung           25       28
keywords       1        1
provenance     1        1
```

The control is the second block and it is why the first means anything. An
instrument returning zero for everything is broken rather than informative, so
fields known to be read are searched by the same grep over the same trees in the
same run.

Two corrections to my own working, both caught by the probe rather than by
reading:

- I first measured the bare word and reported `answers` at six incidental
  matches. With the pattern a check actually uses it is zero. `bound` scores nine
  at the same commit from "bounded" and "boundary" in comments, which is a
  measurement of English.
- The first run of the probe walked the working tree, which by then contained the
  two artifacts this dispatch added, both of which read all three fields. It
  therefore measured the repair instead of what was repaired. It now measures the
  base commit directly, which needs no exclusion list to keep in step.

**What that absence costs is exactly the shape of this dispatch.** The
`question` namespace is defined in the schema as "Something the canon has not
settled". A question a ruling answers is settled. The row stays as audit trail,
which is right, and `decider = op` stays on it, which is also right, because he
is who settled it. But the rendered `docs/QUESTION.md` is a nine-column table and
the incoming `answers` edge is not one of the columns, so an answered question
renders as open, reads as open in every field it carries, and counts as open in
any roster built by reading `decider`. Six were in that state.

## What I closed: six were already answered

Each has a `ruling` naming it in `answers`, in his own words.

| Question | Answered by | Rung |
|---|---|---|
| `what_then_validate_requires` | `validate_means_all_three_readings` | stated |
| `the_cross_repo_strategy_name_collision` | `notko_renames_and_strategy_is_arvos_name` | stated |
| `which_reading_of_i3_at_a_non_native_width` | `the_imitation_is_ergonomics_not_an_arithmetic_boundary` | stated |
| `does_the_native_imitation_cover_the_debug_panic` | `the_overflow_panic_is_permitted_and_bounded` | ratified |
| `may_the_canon_carry_an_unpredicated_proposition` | `a_thing_that_constrains_the_work_and_cannot_be_designed_away_is_canon` | stated |
| `is_the_rounding_candidates_pairing_section_canon_or_design` | same ruling | stated |

**A `stated` ruling settles the question even though its own wording is not yet
ratified**, and the schema draws that distinction itself. `ruling.answers` is
described as "Questions this settles", unconditionally; `proposal.answers`, the
neighbouring field, is described as "Questions this would settle if it were
stamped. **It does not settle them while it is a proposal.**" The asymmetry is
deliberate. Ratifying a ruling's wording is what `awaiting-a-ruling` is for and is
a separate mechanism from whether he decided the thing.

**Four of the six said so in their own prose and two did not.** The four use a
construction they arrived at independently: "Recorded as answered at `28` batch
one", "Recorded as answered by op on 2026-08-14", "Recorded as closed by op on
2026-08-14", "Recorded as answered on 2026-08-14". The two that did not are
`may_the_canon_carry_an_unpredicated_proposition` and
`is_the_rounding_candidates_pairing_section_canon_or_design`, both from the round
recorded at `206`, both answered by the one test he gave in place of six options,
and both reading as fully open. They would have gone into the next batch. That is
precisely the round trip the `bound` field was added to stop, and it restarted the
same day the field landed, in the same round that produced it.

Both now carry the settlement record. Nothing else about them changed.

## What I moved: eleven were never his

Each row now names the panel and carries a `bound` stating the constraint it is
answered inside, with where that constraint already exists. Grouped by how hard
the ground under them is, which is the order a second reader should attack them
in, back to front.

### Four rest on a refusal he has already given verbatim

`mixed_numeral_addition`, `one_numeral_family_or_several`,
`inclusion_order_singleton_amendment`,
`is_a_profitability_predicate_resolved_pessimistically_when_not_const_available`.

The first three are mathematics: whether an operation exists in the operation
set, whether a set of shapes under an inclusion order has unique least upper
bounds, and whether an order's predicate is stated over denotation or over
declaration. His refusal on the operation set covers the class and says so:

> This bears on the prior answer and isn't mine to call, sine I'm not an expert
> or a domain specialist on maths. This is why you and the panel exist.

`one_numeral_family_or_several` carries a second reason on top. The row records
that his instinct is one family and that he said explicitly not to act on it,
because acting on an instinct is how the previous panel locked itself into a
shape that did not fit. An instinct withheld on purpose is not an answer waiting
to be collected.

The fourth is soundness, which is the other bound he named on the container
premise, and the row's own distinction is the derivation: a correctness predicate
must be const, a profitability predicate only wants to be, and where it is not,
resolving it pessimistically is slow and never wrong. Leaving the case unstated
is the one option soundness does not admit.

### Four are answered in shape by a ruling that already exists

`is_root_check_cost_stated_within_a_behaviour_or_across_a_design`,
`do_arvos_consumers_want_value_keying_or_position_keying`,
`is_the_firewall_carried_unpredicated`, `are_open_domain_dimensions_stated_as_open`.

The first two ask which single answer governs a whole category when two are true
in different regions. `ruling::the_work_is_predicated_arms_composed`, the only
ratified entry in the registry, rejects that shape by premise: arms with const
predicates, each optimising one nameable region and applying nowhere else. The
root-check licence is free within the behaviour and not free across the design,
which is two regions with two answers rather than one price to state. The keying
question is the same shape and lands closer still: his refusal at `88`, filed as
`ruling::there_is_no_universal_answer_take_the_win_and_gate_it`, was given on a
question in the same I11 territory, shaped as whether consumers would write a
particular declaration so the design could decide whether to build the surface at
all. His note there is that it was the third of that shape in one sitting.

The second two were both waiting on whether a canon may carry a proposition with
no predicate. That is answered, by the test he gave instead of an option:

> This is something you should decide based on the workflow and soundness and
> validity of the canon and work. If it's a real thing that constrains the work
> and is needed to know, it's canon

A sentence's form was never what decided it, so applying the test to the firewall
proposition and to the open domain dimensions is a derivation rather than a
second ruling. The firewall row names its own precondition and the precondition
is met; the domain-dimensions row records the coordinator folding it into the
parent as the same question one level down.

### Three are the ones to attack first if this pass is wrong

`arithmetic_column_one_axis_or_two`, `why_the_default_rounding_position_is_chosen`,
`the_width_surface_crossing`.

Each rests on a statement of his read slightly wider than the sentence it came
from, and each bound says so in its own text so a later reader does not have to
find it.

- **The axis count** rests on `ruling::the_strategy_intents_are_not_clear_cut`
  and on his "though don't have to be; Open for discussion and exploration!" at
  `36`. The attack: the presets are his intent, and if the axes are what the
  presets are defined over, naming the axes names the presets.
- **The rounding default** rests on
  `ruling::warms_objective_is_the_intuitive_best_choice`, that mimicry is dropped
  where it is consistently the worse choice, plus the measurement that the IEEE
  default is the one mode not free under either signedness. The attack: that
  ruling is about one strategy's objective and reading it as governing a
  crate-wide default is a widening.
- **The width surface crossing** rests on his own quote in
  `obligation::a_primitive_for_every_position_a_bare_number_would_take`: "No bare
  usize other than in const generics for smoother and more ergonomic api, and
  even there, only when truly painful otherwise." That is twice bounded and both
  bounds are on this question. The attack: what a consumer writes at every call
  site is the most visible taste surface arvo has, and a bar stated about bare
  primitives at API positions may not reach how a width literal becomes a
  type-level natural.

## What I added: one question of his that existed nowhere

`which_standards_the_alias_obligation_covers`.

The standards bound is now one of the two constraints the container premise and
the operation set were handed back under, and four topics defer to those two. Its
own scope is unstated. `obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`
records that in its `gap` field: MATLAB `fi`/`fimath`, IEEE 754, SystemC
`sc_fixed`, Vitis `ap_fixed` and Algorithmic C `ac_fixed` all appear in the
corpus, he named two, and no row says whether the set is those, more, or a core
with the rest open.

**A gap is not a question and nothing builds a roster from one.** Every list of
what is owed here comes from `question` rows and their `decider`, so a scope
question upstream of the two largest questions in the namespace was invisible to
the queue. Established rather than assumed, at the commit this seat branched from: MATLAB appeared in `question.toml`
on one line, inside a quoted bound, while SystemC and `ap_fixed` appeared nowhere in it, against
a control of `rounding` at 41 lines, `strategy` at 94 and `predicate` at 78.

It is his rather than the panel's, and the split is clean. How far a demand
reaches is the demand's author's to say. Whether any given convention is
expressible over the primitives is the adequacy test doing its job and is the
panel's to establish. The parity suite named in the same gap is not asked,
because building it is work rather than a decision, and it cannot start until
this is answered.

## What stays his: thirteen, and why

The instrument reports them, so this list is a reading rather than a record:
`cargo mock unasked-questions`.

**What he meant by his own words**, which nobody can settle from outside:
`which_sense_composition_carries`,
`which_accuracy_target_the_accuracy_intent_names`,
`chains_and_ops_as_two_things_or_one_phrase`,
`which_closed_panel_statements_the_absolute_framing_carve_out_reaches`.

**Naming and taste**: `the_rounding_mode_vocabulary`,
`chain_or_region_between_observations`, `does_precision_count_the_sign_digit`.

**Whether something becomes his intent, or amends one**:
`does_the_observability_principle_become_an_intent`,
`what_a_proof_marker_is_against_a_measurement`, which would amend the ratified
predicate notation, and `the_exchange_rate_a_preference_yields_at`.

**Scope and guarantee**: `which_chain_carrier_ships`,
`which_units_a_weighting_is_expressed_in`, and the new
`which_standards_the_alias_obligation_covers`.

Two of those deserve their reasoning stated, because I nearly moved both.

`the_exchange_rate_a_preference_yields_at` asks what rate a strategy's preference
yields at, and the shape is the one he has refused three times. I moved it and
moved it back. `ruling::hot_may_sacrifice_soundness_for_a_proven_meaningful_gain`
carries a `gap` field naming exactly this as unset: "What counts as a meaningful
gain is unset. A real but negligible gain does not buy a soundness loss, and no
number for it exists." It sets a threshold on his own word, inside a soundness
trade, and the ruling he ratified the same week attaches the escape to a lead
designer's blessing rather than to a rate. **What the bound does change is how it
gets asked**: not what the exchange rate is, which is the refused shape, but what
the floor is below which a soundness loss is never bought.

`does_precision_count_the_sign_digit` is a definitional convention and reads like
taste, and the standards bound genuinely bears on it without settling it. IEEE
754 counts significand digits and excludes the sign; MATLAB's `fi` word length
includes it. The two conventions differ, so the alias obligation is an adequacy
test on whichever arvo picks rather than a derivation of which. It stays his, and
whoever asks should name both conventions when they do.

## The four to put to him next

Ordered by what each unblocks, which is the only ordering the dispatch asked for.
The instrument's ordering is alphabetical within its two bands, because every one
of the thirteen has zero incoming references, and that emptiness is its own small
finding: no proposal anywhere answers any question he owes.

**1. `which_closed_panel_statements_the_absolute_framing_carve_out_reaches`.**
The only one whose answer can shrink the rest of the queue. Nine statements of
his in the closed panel are either lost material or correctly absent, and the
`ruling` namespace is either missing rows or is not. Rows that appear would bound
other open questions. It is also pure recall about what he framed as absolute and
what he meant by "other files", so nobody else can touch it.

**2. `which_standards_the_alias_obligation_covers`.** The standards bound governs
the container premise and the operation set, four topics defer to those, and an
adequacy test whose scope is unnamed can neither refute a candidate nor clear
one. Three options, one sentence to answer. It also unblocks the parity suite,
which is a mandate with no artifact and cannot start before it.

**3. `does_the_observability_principle_become_an_intent`.** Underivable by
construction: three members tried to derive the licence clause from his stated
intents and all three failed for stated reasons. What it actually rests on is a
workspace rule, and a workspace rule is not a ratification. Grounds the tenth
unit's licence clause, that everything inside an unobserved stretch is arvo's to
choose.

**4. `does_precision_count_the_sign_digit`.** Binary, one word, and it moves four
things: what the sign domain moves, whether the three sign domains at equal
precision form a chain or leave one incomparable, whether the symmetric domain at
precision one denotes exactly the zero set, and whether two of the three collapse
at an odd radix. All four consequences are computed under both readings, so
nothing is blocked on the derivation, only on the choice. Ask it with both
standard conventions named.

**The next four after those**, so the batch after this one does not need
rebuilding: `which_accuracy_target_the_accuracy_intent_names` and
`chains_and_ops_as_two_things_or_one_phrase` asked together, since they are two
unresolved readings of one sentence of his; then `which_chain_carrier_ships`; then
`which_sense_composition_carries`.

## What I built, and what it is for

**`mock/tools/unasked-questions`**, a project tool beside `awaiting-a-ruling`.
That one reports the `ruling` side, things he said that nobody has run past him.
This reports the `question` side: rows naming him that no ruling answers,
ordered, with the settled ones counted and excluded rather than hidden, and any
row carrying a `bound` while still naming him reported at the top as the filing
error the schema calls it.

It reads the engine's typed reverse edges rather than parsing `answers` itself,
which is what keeps a `proposal` out of the reckoning: a proposal carries the same
field and settles nothing while it is a proposal, so a namespace-blind read would
swallow exactly the questions most likely to look answered and not be. Fifteen
tests, including that case, a namespace merely sharing the prefix, and the
positive control where nothing names him at all. Without that last one every
other assertion is satisfied by a tool that reports everything forever.

Not a lint, for the same reason its sibling is not. There is no failing case: a
question waiting on him is the ordinary resting state of one nobody has asked yet.

**`mock/checks/tests/a_settled_question_does_not_sit_in_the_queue.rs`**, three
arms over the committed canon, each with planted inputs both ways. A ruling may
not answer a question that does not exist. A `decider = op` row carrying a
`bound` is the schema's own stated filing error. A question a ruling settles has
to say so.

It was committed red, on the third arm, on the two rows named above, and went
green when they were repaired.

**The arm was wrong twice before that and the planted inputs caught both, not the
canon.** The first cut matched the bare word "answered" in any field, and `id` is
a field, so a planted row named `answered_twice` answered its own question: a test
asserting a value against itself with two hops in between. The same looseness read
"a further item folds in here rather than being answered on its own" as a
settlement record, which is a row saying the opposite, and it swallowed one of the
two real findings. Both are regression controls now. The bare words appear on 54
lines of `question.toml` in ordinary prose; the phrase the five rows actually
settled on does not.

## What this is not

**One expert's read.** Eleven `decider` moves is a substantial set of calls about
what the canon permits, and the workspace rule is that no such call is made on one
expert's word: two must independently agree, each grounding it in quoted canon,
the second forming its own reading before seeing the first. **I am the first. A
second is owed before anything is built on the moved rows**, and the three marked
uncertain are where it should start.

**Not a claim that the eleven are now answered.** Moving a row says who derives
it, not what the derivation is. Every one still needs the work, and the bound is
the constraint that work runs inside rather than a hint at the result.

**Not a claim that thirteen is the floor.** It is what the schema's own test gives
against the corpus as it stands. A ruling landing tomorrow moves it, which is why
the number lives in a tool rather than in this file.

## Unlicensed mechanisms and defects found outside the question asked

- **`what_one_field_obliges_another_to_carry.rs` is 742 lines**, against the
  workspace's 500-line limit, and reads as several concerns in one file. Its
  content is sound and its tests are real; the file wants splitting along its
  seams.
- **The pre-commit autofix cannot format the tool crates.** Each declares its own
  `[workspace]`, so the fmt pass over the mock workspace never reaches them and
  reports `auto_fmt: skipped, rustfmt failed`. `awaiting-a-ruling` sits at 24
  lines of drift from this. Neither crate is broken and nothing says the pass is
  not covering them. Pointing the autofix at `mock/tools/*` closes it.
- **`docs/QUESTION.md` renders nine columns and none of them is the incoming
  `answers` edge.** That is the mechanism by which a settled question reads as
  open, and the prose repair in two rows is a workaround rather than a fix. The
  durable fix is the renderer carrying the edge, which is mockspace's business
  rather than this repository's.
- **`ruling.answers` had no referential integrity check before this dispatch.** A
  ruling could name a question that does not exist and nothing would report it.
  The canon is clean of that today; it was clean by luck.
