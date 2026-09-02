# 235. Kiselyov: which of the sixteen obligations the ratified canon can carry a design for

Seat 235. The question is the one op's exhaustiveness bar poses, asked per
obligation rather than in aggregate: for each of the sixteen rows in
`mock/registry/obligation.toml`, does the **ratified** canon settle what that
obligation would have to be, closely enough that a design for it can be derived
rather than invented beside it. Then a ranking of whatever survives, by how much
it unblocks and how low it sits in the build order.

**The answer is none of the sixteen, and I show it below row by row.** The
rejection list is the deliverable here, not the residue of one.

## 0. The two gates, and what this file's evidence is

**Canon gate: passed.** Measured against `mock/registry/*.toml`, which
`mockspace.toml:31` declares as `canon_paths`. I read all 95 `ruling` rows and
the full text of all 31 at `rung = "ratified"`, all 16 `obligation` rows, all 104
`question` rows by id, topic and answered-state, every `obligation` and
`precondition_for` edge in the repository, the four `ratifies` lists on ratified
rulings together with the proposals they name, the coverage tool with its suite,
the three shipped crates, and the design rounds that produced them. Nothing in
the dispatch asks for work the canon forbids.

**Test gate: the tool's suite is real and has one hole, named in section 7.** It
is not decorative, it carries genuine negative controls, and its fixture never
plants the field whose absence is the defect.

**What my evidence is, stated before any of it is used.** Every check is a read
over committed text. `235_probes/survey.sh` is the script and
`235_probes/survey.out` is its raw output, both committed beside this file. Each
zero-returning check carries a positive control immediately after it, because a
zero from a pipeline is a claim about the pipeline until the pipeline has been
shown able to return something else.

**Nothing here runs the coverage tool.** This dispatch was read-only in a
worktree belonging to another seat and was told not to invoke cargo, so where
this file says the tool reports something, that is the dispatcher's quoted run
plus my own reading of `mock/tools/obligation-coverage/src/lib.rs`. The claims in
section 7 about what the tool *would* report are derived from its source and from
walking the same registry edges it walks, and they are written as that rather
than as a run. Somebody should execute it after the repair and confirm the
numbers move the way section 7 predicts.

## 1. Three corrections to the dispatch's own facts

Stated first so nothing below reasons under a false premise.

**"104 open `question` rows" is wrong.** 104 rows, 13 carrying an `answered`
field, so 91 open (`survey.out` section 4). It matters because three of the
thirteen bear on obligations and two of them hand work explicitly to a design
round rather than reserving it.

**The tool's tiers do not measure what the question asks, and they are wrong in
both directions.** Section 7.

**The framing presumes a canon further along than the canon says it is.**
`ruling::the_canon_must_support_a_full_design_and_impl` reads: "Before the canon
goes to him for review it has to be exhaustive enough that a full design, and
then a full implementation of everything, can be done from it." That is a bar for
a future moment. A verdict of zero is the expected reading of an unfinished canon
rather than an indictment of it, and the tool says so itself at `lib.rs:25-26`: "An
obligation nothing answers is not a defect: it is the state of unfinished work."
I report zero and I do not report it as a failure.

## 2. Verdict over all sixteen

| # | obligation | tool tier | verdict | what decides it |
|---|---|---|---|---|
| 1 | `an_exact_width_container_a_consumer_can_alias_and_pin` | met | not settled | reserved at `question::arbitrary_width_demands_in_the_canon` and `question::the_width_surface_crossing` |
| 2 | `every_standard_convention_expressible_as_an_alias_over_the_primitives` | proposed | not settled | ratified by proxy, but the ratified evidence excludes signed and every fraction width; scope only at `stated` |
| 3 | `composition_contracts_above_the_numeral` | proposed | not settled | named by a ratified intent, defined by nothing; seven open `the_chain` questions |
| 4 | `a_platform_sized_unsigned_integer_at_an_api_position` | nothing | not settled | reserved at `question::what_a_platform_width_type_is`; unrecorded tension with the in_force constraints |
| 5 | `a_build_flag_that_changes_float_semantics` | nothing | not settled | reserved at `question::which_axes_a_build_arm_may_move` |
| 6 | `the_surface_expressible_as_contracts_before_anything_implements_it` | nothing | not settled | only `stated`; reserved at `question::what_the_admission_contract_asks_a_candidate_to_expose` |
| 7 | `a_primitive_for_every_position_a_bare_number_would_take` | nothing | not settled | `in_force` only, and the demand is over an unenumerated set of positions |
| 8 | `the_unstable_machinery_does_not_reach_a_consumer` | nothing | not settled | canon-silent; no ruling, no question, unmeasured |
| 9 | `a_content_hash` | nothing | not settled | canon-silent; no ratified `says` mentions hashing |
| 10 | `debug_output_from_every_numeral_shape` | nothing | not settled | canon-silent; no ratified `says` mentions rendering |
| 11 | `set_operations_over_a_fixed_size_bit_set` | nothing | not settled | canon-silent; no topic, no ruling, no question |
| 12 | `ordering_a_directed_acyclic_graph` | route-closed | not settled | canon-silent; a gate was retired, not an algorithm |
| 13 | `a_sparse_adjacency_a_plan_can_be_built_on` | nothing | not settled | canon-silent |
| 14 | `a_spectral_partition_of_a_dependency_graph` | route-closed | not settled | canon-silent |
| 15 | `a_cost_dynamic_program` | route-closed | not settled | canon-silent |
| 16 | `the_algebra_is_legible_enough_to_adopt_without_adopting_half_of_it` | nothing | not settled, and not this kind of thing | no implementable object; the row says its own instrument does not exist |

## 3. No survivors, so here are the three that come closest and what stops each

I ranked before rejecting, so the near misses stay visible instead of vanishing
into a flat "none".

### 3.1 `an_exact_width_container_a_consumer_can_alias_and_pin`

The only one where the ratified canon carries a definitional spine deep enough
that most of a design falls out of it.

`ruling::the_format_spine_is_canon` ratifies four propositions and says: "A
format is identified by its ambient domain and its representable set, and that
set is a constant of the type. Membership in it is one affine predicate over one
parameterisation, of which integers, fixed point, scaled integers and floats are
points. Arithmetic on a format is an exact operation in the ambient domain
composed with a named total adaptation onto that set."
`proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set`
adds the line that decides several rejections below: "a value set that depends on
other data is not a format but storage."

`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`
gives the width statement: "every operation the design declares is a function of
the declared width and never of the machine carrier."

`ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`
gives the carrier and the admission test: "one output at sole occupancy, the
carrier; three at shared, the carrier, the access width and the stride ... an
operation is admitted exactly when it is a function of the declared signature."

Together those settle what a 28-bit unsigned thing is, what its carrier is, and
whether a widening conversion is an admitted operation. That is the majority of a
design, and it is why this ranks first.

**What stops it is the half the obligation actually asks for.** The need is "A
container of an exactly declared bit width that a consumer can name, alias under
its own name". Naming and aliasing is the width surface, and the width surface is
reserved twice over.

`question::arbitrary_width_demands_in_the_canon` (Q28), open,
`decider = "panel"`, asks "Should the canon speak of arbitrary const-width
demands at all?" over "Yes, the canon speaking of arbitrary const-width demands"
and "No, leaving width families to design." That is fatal on its own: a design
round cannot decide whether its own subject belongs to the canon or to a design
round, and until it answers a designer does not know whether he is deriving or
inventing.

`question::the_width_surface_crossing` (Q9), open, `decider = "panel"`, asks
"What should the crossing be between a consumer's written width literal and the
type system's representation of it?" with seven options, all live, its own note
saying: "The container-derivation mechanism itself is established and is not what
is open; this is the upstream question of how a written const becomes the
type-level natural the derivation operates on." Picking one of seven inside a
design round is invention beside the canon by definition.

Two further open rows narrow the margin without deciding it.
`question::what_a_datum_stands_for` (Q4) is open on the `the_primitive` topic,
which carries no ruling at any rung, and asks what one datum denotes with a point
reading and a set reading among its options.
`question::is_the_derived_numeral_required_to_be_tightest` is open and bears on
the width a conversion's result declares.

**Not settled. The definition is ratified, the surface is reserved, and the
obligation is about the surface.**

### 3.2 `every_standard_convention_expressible_as_an_alias_over_the_primitives`

Second, and the one the tool gets wrong in the generous direction.

`proposal::every_operation_arvo_declares_is_a_function_of_the_declared_width`
carries `obligation = ["every_standard_convention_expressible_as_an_alias_over_the_primitives"]`,
and it appears in the `ratifies` list of
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`.
So a ratified ruling ratifies a proposal naming this obligation, which is a
stronger tier than the tool reports. Both edges are shown resolved in
`survey.out` section 3, which is where the file and line for each sit; they are
not cited here, because a registry line citation names a different row a week
later.

It still does not settle it, for three separate reasons.

**The scope of the demand sits at `stated`.**
`ruling::the_standards_bound_starts_at_two_and_reserves_the_rest`: "The parity
suite is built against MATLAB `fi`/`fimath` and IEEE 754. The other conventions
the corpus carries ... are reserved rather than excluded."

**The parity suite has no artifact.** The obligation's own `gap`: "The parity
suite is also unbuilt: it is a mandate with no artifact."

**The ratified evidence does not reach the formats the standards need.** The
ratified proposal's own `predicate` reads `"fraction_width: 0"` and
`"signedness: unsigned"` (`survey.out` section 3), and its note says: "The signed
half of the predicate is unmeasured and therefore absent: every sweep here is
unsigned. So is every fraction width above zero, which is the limit worth reading
twice, because a fractional format is where arvo's float side lives and nothing
here reaches it." MATLAB `fi` is a fixed-point convention and IEEE 754 a floating
one. Both live entirely outside the region the ratified claim was established
over, and under `ruling::the_work_is_predicated_arms_composed` an absent
dimension is not a hedge, so the ratified claim says nothing at all about either
of the two conventions op named.

**Not settled. Best provenance of the sixteen, worst coverage-to-demand ratio.**

### 3.3 `composition_contracts_above_the_numeral`

Third, on strength of intent and nothing else.

`ruling::arvo_is_a_library_and_the_value_composes_on_top` (I11) says: "The
selling point is the algorithm crates that every downstream repository uses, and
the contracts for things that compose into units bigger than numerals alone."
`ruling::accuracy_is_never_bought_with_performance` constrains what such a
contract may trade. Neither says what a composition contract is.

Ten proposals name the obligation, eight at `one_expert` and two at
`two_experts`, and none appears in any ratified `ratifies` list. The foundation
is open in seven places at once: `question::which_sense_composition_carries`,
`question::which_chain_carrier_ships`,
`question::what_a_numeral_guarantees_to_a_fold`,
`question::two_shapes_of_aggregate_composition`,
`question::chain_or_region_between_observations`,
`question::reduction_order_or_associativity`, and
`question::chains_and_ops_as_two_things_or_one_phrase`. The last is recorded as
op's own, in `strategy::precise`: "Op's own wording also carries a second
unresolved reading, whether `chains and ops` is two things or one phrase, and
that is his to settle."

**Not settled. A ratified statement that arvo owes the thing, with no ratified
statement of what the thing is, is exactly the case that does not count.**

## 4. The rejections, with their grounds

### 4.1 Reserved by an open question

**`a_platform_sized_unsigned_integer_at_an_api_position`.**
`question::what_a_platform_width_type_is` (Q26), open, `decider = "panel"`, asks
"What kind of thing is a platform-width type?" over four options, one of which is
"A different kind of thing the format concept need not account for." So it is
open whether the canon's central concept even ranges over this. Second,
independent block: the ratified
`proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set`
says a value set depending on other data "is not a format but storage", and the
canon has no `storage` concept anywhere, so a design would have to invent one.
Third, raised in section 8: the need reads "An unsigned integer of the platform's
size" while `ruling::the_operating_constraints_are_intents_and_rules` lists "no
platform dependency" among constraints that "are not to be questioned", and no
row records the tension.

**`a_build_flag_that_changes_float_semantics`.**
`question::which_axes_a_build_arm_may_move` (Q13), open, `decider = "panel"`,
four options from "Any axis, including overflow policy and intermediate
precision" to "Unobservable axes only". Float semantics is the observable axis
the fork is about. The obligation's own `gap` records both that the mechanism
rather than the need is in doubt, and a conflict with `a_content_hash`: "a hash
stable across machines and a float regime that changes with a build flag cannot
both hold over the same values, and nothing in the registry says so."
`ruling::never_a_runtime_check_and_one_lowered_path` is adjacent and does not
decide it.

**`the_surface_expressible_as_contracts_before_anything_implements_it`.** The
only row is `ruling::the_trait_contract_structure_is_a_primary_paradigm` at
`stated`, whose own note disclaims its reach: "An argument for `in_force` exists
and I have not taken it ... That enforces where traits may live rather than that
contracts are the paradigm, which is a narrower thing than he stated." The
obligation's `gap` agrees: "no row anywhere establishes that the surface can be
stated that way." And `question::what_the_admission_contract_asks_a_candidate_to_expose`
(Q29) is open over three options on what a contract must ask a candidate to
expose. Nothing ratified touches it.

### 4.2 Direction rather than canon, over an unenumerated demand

**`a_primitive_for_every_position_a_bare_number_would_take`.**
`ruling::the_operating_constraints_are_intents_and_rules` carries it at
`in_force`: public API positions use "the stack's own primitives rather than bare
integers, floats, bool or usize". That binds as direction and is not canon.
Independently of tier it cannot be designed to as worded, because the demand is
over a set nobody has written down. Its own `gap`: "It is not satisfied by
counting the primitives that exist: it is satisfied by the positions, and nothing
has enumerated those."

### 4.3 Canon-silent, and silence is not permission

Each checked three ways. No ratified `says` mentions the subject: a walk over all
31 ratified rows for `hash`, `graph`, `topolog`, `bitmask`, `bit set`, `Debug`,
`fmt`, `sparse`, `spectral`, `dynamic program` returns zero, with a positive
control on the same walk returning a hit for `declared width` (`survey.out`
section 5). No `question` row's keywords carry the subject. And `topic.toml` has
no topic for any of them, so no panel unit has ever run on one.

**`the_unstable_machinery_does_not_reach_a_consumer`.** Its own row states the
position better than I can: "Nothing in the registry is about this, and it is not
implied by anything that is ... Whether a `generic_const_exprs` bound in a public
signature can be hidden from a consumer at all is exactly the open question, and
it is unmeasured." Note what this costs the other fifteen: it constrains the
implementation of every arm the canon proposes, so its absence is not a gap
beside the others but underneath them.

**`a_content_hash`.** No ratified row mentions hashing. Its `gap`: "What the
population actually is, and therefore what width is enough, is not established
anywhere ... the row is quoted and still under-specified, and those are different
states."

**`debug_output_from_every_numeral_shape`.** No ratified row says what a numeral
renders as. The tree is nearest here of any of the sixteen and still nowhere:
`Width` and `Bool` carry `#[derive(Debug)]`, which is the host's rendering of a
host integer, while the need is "written into a caller-supplied fixed-size buffer
with no alloc and no std" over "every numeral, at every width and under every
strategy". There are no numerals in the tree to render.

**`set_operations_over_a_fixed_size_bit_set`.** Canon-silent, and blocked a
second way even if it were not: the need makes the lowering part of the demand,
"each lowering to one instruction", and the obligation's `why` says "a correct set
type that lowers to a loop does not meet this." No ratified row states anything
about an instruction count. What the canon does give is the method for such a
claim, in `ruling::there_is_no_exchange_rate_because_there_is_no_generalisation`:
"A win is taken however small the gain and however small the region it applies
to; the size of the region is the arm's predicate." A method is not an object.

### 4.4 The three route-closed ones, where the tier reads stronger than it is

**`ordering_a_directed_acyclic_graph`,
`a_spectral_partition_of_a_dependency_graph`, `a_cost_dynamic_program`.** All
three are route-closed by one row,
`retirement::dl_gate_algorithm_crates_on_addassoc`, whose `claim` is "Gating the
graph, combinatorial and spectral crates on an associativity fact by default" and
whose `kind` is `wrong`. What was retired is a gating mechanism. Nothing about
any of the three algorithms was tried or closed. So route-closed is worth
strictly less than its name suggests here, and the tool is right both to
distinguish it and to say it "is not the same as nobody having looked".

`ordering_a_directed_acyclic_graph` additionally carries the repository's only
`precondition_for` edge, from
`proposal::splitting_a_reduction_is_sound_in_three_of_the_four_sign_and_policy_cells`
at `standing = "one_expert"`. Per the tool's own doctrine that leaves it further
from met rather than nearer, and I agree.

**`a_sparse_adjacency_a_plan_can_be_built_on`** is canon-silent with an extra
consumer-side hole its own row names: "which fine decomposition the consumer
wants when the graph is not square, since Dulmage-Mendelsohn is named without the
bipartite structure it decomposes being described anywhere."

### 4.5 Not this kind of thing at all

**`the_algebra_is_legible_enough_to_adopt_without_adopting_half_of_it`.** The
dispatch asks which obligation could be taken "through a full design round to a
landed implementation". This one has no implementable object. Its own `gap`:
"Legibility has no instrument here and this row does not supply one ... Worth
saying plainly that this row cannot be closed by an argument that the two layers
are correctly stated: they are, and the consumer read them anyway." The remedy it
names is a second consumer reading the shape cold and reporting what it found.
That is a dispatch, not a design round. I reject it on category, and I would
reject it the same way if the canon were finished.

## 5. The ranking, over what the canon poses instead

Nothing survives, so the useful ordering is over the reserved questions. Both
criteria the dispatch names live at `stated` rather than `ratified`, which is
worth saying since they are the instrument for this half of the answer:
`ruling::the_next_unit_is_chosen_bottom_up`, "The next topic is whatever is
natural and settles the most downstream questions at once, unblocking further
design and exploration. The build order is strictly ground up", and
`ruling::the_selection_criterion_is_what_serves_the_rest_of_arvo`, "Among
candidate shapes, the one to converge to is the one that serves all other parts
of arvo best. Not the one best in isolation."

1. **`question::arbitrary_width_demands_in_the_canon` (Q28).** Lowest in the
   order, because it decides whether the next item is a canon question or a
   design decision. Answered one way it hands the width surface to a design round
   and unblocks obligation 1 outright; answered the other it tells the canon what
   it owes. Five design rounds already cite it, more than any other question row
   (`survey.out` section 6).
2. **`question::the_width_surface_crossing` (Q9).** The entry gate for every
   numeral-shaped obligation, so the single largest unblock among the four that
   are not canon-silent: 1, 2, 4 and, through the primitives, 7. Seven compiled
   arms already sit in the corpus waiting to be argued.
3. **`question::what_a_datum_stands_for` (Q4).** Topic `the_primitive` has no
   ruling at any rung and exactly one open question, which is this. Everything
   above the format spine ranges over data, so the spine currently sits on a
   concept nothing has stated.
4. **`question::what_a_platform_width_type_is` (Q26).** Narrower, unblocking only
   obligation 4, and downstream of Q28, which decides whether the width family
   belongs to the canon at all.
5. **The `the_chain` cluster.** Seven questions unblocking obligation 3.
   Correctly last under a bottom-up order, since a chain sits above the numeral
   and the numeral is not built.

The seven canon-silent obligations are not on this list because there is nothing
to answer. What they need first is a topic. `topic.toml` holds twenty topics and
none is about the algorithm surface, hashing, rendering or set operations, while
five of the sixteen obligations are filed under `arvo_identity` for want of
anywhere better.

## 6. What I would tell whoever starts next

Do not take any of the sixteen through a design round yet. Take Q28 through a
panel unit: it is one question, it is the lowest thing in the order, and its
answer either hands obligation 1 to a design round immediately or tells the canon
what it owes. Then Q9, which has seven compiled arms already sitting in the
corpus.

And before either, repair the coverage tool, because the number it prints is the
number that gets relayed to op as the answer to his own exhaustiveness bar, and
it has already been relayed wrong once by its predecessor. `lib.rs:10-12` records
that: "its figures were relayed upward three times, twice to op, one of them
wrong." The successor is a better instrument reporting a different wrong number
for a different reason.

## 7. The tool: what its tiers establish and what they do not

Established by reading `mock/tools/obligation-coverage/src/lib.rs` and its suite,
and by walking the registry edges it walks. Not by running it; see section 0.

It is a careful instrument with a clean doctrine and three of its four design
decisions are right. Its headline number is not a claim about the canon.

**It never reads a rung.** `EDGES` at `lib.rs:96-100` maps a bare namespace to a
tier, `("ruling", Reach::Met)` at `lib.rs:97`, and the walk at `lib.rs:136-143`
iterates `reg.rows_in(ns)` with no filter between the iteration and
`entry.0 = entry.0.min(tier)`. The string `rung` occurs zero times in `lib.rs`
and zero times in `tests.rs`, against 102 occurrences in `ruling.toml` as the
positive control (`survey.out` section 1). So `Met` means "some row in the
`ruling` namespace carries an `obligation` edge naming this", across all four
rungs.

**Consequence, and it is the whole headline.** Exactly one ruling in the
repository carries an `obligation` edge:
`ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up`, and it sits
at `rung = "stated"` (`survey.out` section 2, which walks every such edge and
prints the rung of the row each belongs to). That is the entirety of `met 1`. The tool's doc comment at `lib.rs:35-37` glosses the tier as "A ruling
names it and op has been in the loop, which is the only tier that is an answer."
Op was in the loop on that row and the loop's output was a refusal. The row's own
note quotes him: "I need elaboration on the Ingest vs C ABI. That explanation does
not explain what this is, so I can't bless it or comment on it in any manner."
There is a second ruling recording the refusal as a statement in its own right,
`ruling::he_declined_to_bless_the_ingest_row_as_written`, and a question opened to
unblock it, `question::what_the_ingest_row_claims_and_what_turns_on_it`, whose
`unblocks` reads: "Promotion of the row. It sits at `stated` carrying his words,
reads as ready, and is the one row he was asked about and refused, with nothing on
it saying so until now."

So the single `met` in a report whose purpose is to measure whether the canon can
carry a design is produced by the one row op explicitly declined to bless. That is
not a small miscalibration. It is the worst available row producing the best
available tier.

**And it under-reports in the other direction, through ratification by proxy.** A
ratified ruling naming a proposal in its `ratifies` list does not inherit that
proposal's `obligation` edge, so
`every_standard_convention_expressible_as_an_alias_over_the_primitives` is
reported as `proposed` while a ratified ruling ratifies the proposal that names
it. On a population of sixteen the tiers are wrong in both directions at once,
which means the numbers cannot be read even as a lower bound.

**The suite pins the defect rather than catching it.** 20 tests
(`grep -c '^#\[test\]'`, `survey.out` section 9). The fixture `with()` at
`tests.rs:59-72` plants a row carrying an `obligation` field and nothing else, so
every ruling any test ever sees has no rung at all. `tests.rs:75`, named
`a_ruling_meets_an_obligation_and_a_proposal_only_proposes_it`, asserts
`Reach::Met` for that rungless row. Not a tautological test and not a bad one on
its own terms: the assertion is real and the suite carries genuine negative
controls, `control_a_met_obligation_carrying_a_precondition_is_not_in_that_list`
at `tests.rs:263` among them. It is the setup-that-helps class exactly. Every
input is one the implementation handles, so the path that breaks, a `stated`
ruling, is never entered, and the one fundamental property the tool asserts about
itself in prose has no test anywhere.

**The repair is small and I would not rename the tier.** Split `Met` by rung: a
`ratified` ruling is an answer, an `in_force` or `stated` one is direction and
wants a tier of its own between `Met` and `Proposed`. Follow the `ratifies` edge
so a ratified ruling confers its tier on the proposals it ratifies. Then plant a
`stated` ruling in the fixture and assert it does not reach `Met`, which is the
test that would have caught this and whose absence is why nobody did.

**What the tool gets right, said because it is most of it.** Keeping
preconditions out of the tier arithmetic, and saying why at `lib.rs:43-47`, is
correct and is the non-obvious call. Distinguishing route-closed from nothing is
correct and load-bearing for three of the sixteen. Refusing a pass line, at
`lib.rs:26-28`, is correct: "gating on a count would invent a deadline nobody set,
and an invented threshold is worse than no gate because people defend numbers."

## 8. Unlicensed, or wrong, and outside what I was asked

### 8.1 The shipped tree answered an open canon question inside code

`question::the_width_surface_crossing` is open with seven live options and
`decider = "panel"`. `arvo-format` has picked option one.

```
mock/crates/arvo-format/src/slots.rs:140  pub struct Signed<const BITS: u32>;
mock/crates/arvo-format/src/slots.rs:143  pub struct Unsigned<const BITS: u32>;
mock/crates/arvo-format/src/slots.rs:175  admit_widths!(1, 2, 3, ... 62);
```

That is the option's own wording: "Keyed on const generics directly with a
per-width bridge, one row per written width." The bridge is 62 macro-generated
impl pairs. All four shipped formats are keyed the same way, at `lib.rs:70`,
`lib.rs:87`, `lib.rs:105` and `lib.rs:123`.

Citations of that question row across `mock/design_rounds` and `mock/crates`:
zero, against nine other question rows those same trees do cite, five of them
`arbitrary_width_demands_in_the_canon` (`survey.out` section 6, with the control).
So the pick was made and nothing records that a pick was made.

**The same tree knows better one crate over.**
`mock/crates/arvo-strategy/src/lib.rs:25-27`: "Which overflow mode each preset
names is not written here. That question is open in the registry, and filling it
inside a design is how an open question gets closed where nobody can see it
happen." The discipline, stated correctly, by the same author, in the same round
family, and not applied to the larger question one crate down.

### 8.2 A locked changelist justifies a derived call with a claim its own file disproves

`mock/design_rounds/202608311902/202608311902_changelist.src.lock.md:103`:

> an associated const must have a primitive type; the language gives no alternative.

False, and falsified by the file it licenses, nine lines apart inside one trait.
`mock/crates/arvo-format/src/slots.rs:54` and `:57` declare `const MIN: i64;` and
`const MAX: i64;`, and `slots.rs:63` declares `const WIDTH: Width;` in the same
trait. `mock/crates/arvo-placement/src/lib.rs:53` does the same with
`const BITS: Width;`. An associated const may have any type. Only a const
*generic parameter* is restricted to primitives, and that position is the one
`obligation::a_primitive_for_every_position_a_bare_number_would_take` explicitly
excepts: "A const generic parameter is excepted." So the derived call took cover
behind the one restriction that is real to license a set of positions where it is
not.

### 8.3 The same changelist states a totality its own crate contradicts

`202608311902_changelist.src.lock.md:104-105`:

> `arvo-format`'s value positions carry `Width` and `Bool` rather than host integers.

Nineteen public `const fn` in that crate return a bare host primitive. Two are in
`width.rs`, each documented there as "The unwrap door, declared as one", which is
honest. Seventeen are outside `width.rs`, where no door is declared, and three
functions in the whole crate return the stack's own `Bool` (`survey.out` section
8b). Nine of the seventeen return bare `bool` while `Bool` is defined in the same
crate: `format.rs:54`, `format.rs:65`, `slots.rs:130`, `slots.rs:183`,
`quantum.rs:72`, `quantum.rs:81`, `apply.rs:80`, `apply.rs:86`, `apply.rs:254`,
`apply.rs:260`. Four return bare `i64`, `i32` or `u32`: `apply.rs:229`,
`slots.rs:194`, `format.rs:71`, `format.rs:88`.

`mockspace.toml:1408`, `arvo-format = ["numeric"]`, exempts the crate from the
bare-primitive lints, so this is not a lint breach and I am not calling it one.
It is a false sentence in a locked changelist describing a tree that does not look
like that, and it is the sentence a later round will read to learn what the
convention is. The crate's own module doc at `width.rs:14-16` makes the same claim
in the same direction: "Two types and no more ... the point of the door is that it
stays narrow." The door is not narrow. `Width::is_none` returns `Bool` and
`slots::is_admissible` returns `bool`, both public, both in the crate that exists
to define what a truth value is in this stack.

### 8.4 A ratified ruling claims a two-expert tier over the union of two disjoint instruments

**I am the first read on this and a second is outstanding.** Nobody downstream
should take this section for a pair. Under
`ruling::two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`
and `ruling::an_expert_asks_its_peers_before_it_asks_op`, a second independent
reading is owed before anything is done about it, and that reading should be
formed from the ruling and the rule directly rather than from this file.

`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`,
`because`:

> the deciding fact is that the two instances carry disjoint axes and agree where they overlap.

and `note`:

> So the two agree over the union of what each covered and neither is a second instance on the other's axes, which is the honest shape rather than a defect.

The workspace rule the two-expert tier comes from says the opposite, under "Two
agreeing instances agree about the intersection of their dimensions, not the
union":

> the tier is over their intersection, which is sometimes empty, meaning there is no convergence about it at all.

Two instruments with disjoint axes have an empty intersection. Agreement over the
union is not something two agreeing instances can establish, and the ruling's own
words say the axes are disjoint. So the tier is asserted over a region neither
instance measured, on a promotion whose same note declares a blindness
contamination.

Scope of the consequence: `mock/crates/arvo-format/DESIGN.md.tmpl:6` and
`mock/crates/arvo-placement/DESIGN.md.tmpl:4` both name this ruling as their
governing canon, and its own note says "This unblocks the floor of the stack."
Both shipped crates stand on it.

**And the ruling's `says` carries no predicate at all** while one of the four
proposals it ratifies is predicated at `"fraction_width: 0"` and
`"signedness: unsigned"`. `ruling::the_work_is_predicated_arms_composed` requires
the opposite: "Every finding must be predicated, including universal ones, so that
where it holds is said exactly rather than assumed." A ratified ruling stating an
unpredicated universal, promoted from a measured proposal whose region excludes
signed values and every fractional format, breaks a ratified ruling in the same
tier. The proposal's own author flagged the exact axis as the one that matters:
"a fractional format is where arvo's float side lives and nothing here reaches
it." The ruling carries no such sentence.

### 8.5 `question.toml`'s header is false about thirteen of its own rows

The header of `mock/registry/question.toml`, before any row:

> No answer is recorded here, including for the rows whose source records one. Where a question was answered, `note` says that it was and where, and never which way.

Thirteen rows carry a full `answered` field stating which way, several at length,
listed in `survey.out` section 4.

Not a filing quibble. A reader trusting the header treats all 104 rows as
reserved, and two of the thirteen do the opposite of reserving:
`question::which_width_coordinates_a_consumer_writes` hands a decision explicitly
to a design round, "It belongs to the design round for the format, not here, and
it is recorded as such rather than left sitting in the canon's queue looking
unsettled." The dispatch that produced this file made exactly that error, and it
is the header's fault rather than the dispatcher's.

### 8.6 A conflict between an obligation and an in_force constraint that nothing records

`obligation::a_platform_sized_unsigned_integer_at_an_api_position` needs "An
unsigned integer of the platform's size".
`ruling::the_operating_constraints_are_intents_and_rules` lists "no platform
dependency" among constraints that "are already in place ... and they are not to
be questioned."

The obligation file records one conflict of exactly this shape, between
`a_build_flag_that_changes_float_semantics` and `a_content_hash`, and says so on
the row. It records nothing about this one, and Q26's option list does not mention
the constraint either. Whether it resolves as a per-target family of formats or as
something the format concept need not account for, the conflict belongs on the
row, because as things stand a designer reading only the obligation gets a demand
the operating constraints forbid.
