# 226. Lattner: the derivation's outputs, and the operation set

Seat 226. Two questions, both `decider = "panel"`, both carrying no `answered`
field when I read them, both verified against the committed registry rather than
remembered:

- `question::container_derivation_output_count`, topic `the_realisation_map`.
- `question::which_operation_set_the_design_ships`, topic `the_strategy_object`.

Both answers below are dissolutions rather than selections from the recorded
option lists, and in both cases the option list is three answers to a question
that has two regions. That is the same shape the floor ruling reached on its own
question, and I did not set out to copy it; I set out to pick option three on the
first question and could not make it survive the occupancy split.

## 0. The two gates

**Canon gate: passed.** Checked against
`ruling::the_panel_finishes_the_canon_without_him`, which puts every remaining
canon question with the panel; against `question::which_operation_set_the_design_ships`'s
own `bound`, which records the question as having been put to op and returned and
says in terms that the chain ending here is workable end to end without him; and
against `ruling::the_container_derivation_needs_fresh_eyes`, whose standing call
is fresh eyes on the existing attempt. Nothing I was asked to do is work the
canon forbids, and neither question is one the canon holds open and reserves.

One adjacent deferral is open and I stayed off it:
`ruling::the_carrier_question_waits_on_the_contention_measurement`, `rung = "open"`,
defers *which carrier the packing claim is about* pending a contention run. That
is `question::which_carrier_the_packing_claim_is_about`, `decider = "measurement"`,
and it is not either of mine. Where my answer below touches the number of
placement objectives I say the count is not mine and name what closes it.

**Test gate: passed for the surface I touch, with one finding I report below
rather than refuse over.** `cargo mock check` reports the lint pipeline green
under strict, 689 rows across 10 namespaces, schema check passed. The lint pack's
own suite is 439 tests, all passing, and I read bodies rather than counting:
`a_region_agrees_with_the_sentence_kind.rs` in full, plus the shapes of
`canon_rows.rs`, `canon_citations.rs`, `panel_corpus.rs` and
`canon_lint_testkit.rs`, which are the surface a research file and a predicate
pass through. They are real tests. `a_region_agrees_with_the_sentence_kind.rs`
checks both directions of its rule, distinguishes an absent field from a blank
one across four spellings of blank, asserts on `finding_kind` rather than on a
count, and carries a silent-pair case so it cannot be reporting the rule instead
of a breach of it. I found no tautological test, no test without an assertion,
and every registry lint carries at least one `control_`-prefixed case.

**And `timeout` is not on macOS.** My first attempt to run the suite was
`timeout 900 cargo mock check`, which printed `command not found` and returned an
exit status I read as zero through a pipe. That is exactly the failure
`never-discard-stderr-on-a-check.md` names, it took thirty seconds to catch, and
it would have let me report a suite I never ran.

## 1. Blindness, and one leak I did not expect

I read the registry only, and `.claude/rules/`, before committing. I did not open
any numbered member file, did not run `git log`, and did not read `OPTIONS.md` or
any consolidation.

**My derivation and all six probes were committed before I read anything else.**
Commits `f0c7563e`, `b90f320a` and the four after them carry the probe directory
and its outputs; the deliverable was written after.

**The leak, verbatim.** The coordinator asked me mid-flight to fetch and merge
`research/canon-registry` to read op's `227`. The merge summary printed the names
of the files it brought in, and two of them were:

```
 .../225_probes/probe5_selector_or_key_is_one_map.rs
 .../225_probes/probe6_admission_separates_assignments.rs
```

Those two filenames are conclusion-shaped sentences and they are, near enough,
the two conclusions I had already reached and committed: that the selector and
the key readings are one map, and that the operation set is settled by an
admission rule. **I had committed both before the merge**, which is checkable
rather than asserted, so the contamination is bounded to nothing on those two
points. What it does cost is that I can no longer serve as a blind second
instance on the *framing*: I now know another seat reached something with those
names, and I cannot unknow it while writing section 3.

I did not open either file. Reporting it rather than smoothing it over, per the
brief.

**Then I read `227`**, op's own file, which the coordinator made required
reading. Section 3.0 says what it changed and what it did not.

---

# Question one: how many outputs the container derivation has

## 2.1 The recorded options, and why none of them can be right as put

The registry records three:

1. One output, the carrier alone, which occupies more memory than the strategy
   promises because the map from width and strategy to carrier is not injective.
2. Two outputs, the carrier and the stride, answering a per-value question and a
   per-aggregate question.
3. One richer output that is a type with named projections, the pair wearing one
   name rather than a rival to it.

`proposal::an_output_of_a_derivation_is_a_fact_a_downstream_site_cannot_recover`
already dissolved the one-against-two half of that: any product is one thing, a
pair is a single element of a single set, anything with two projections is a
pair, so one and two are one mechanism at two levels of packaging. What it left
open, in its own words, is *how many of the derived facts must be available as
types rather than as values, and that number is at least two.*

So the live question is not a count of associated items. It is **which facts the
derivation owns, and what each is for.** And that question has two answers,
because the floor ratified an occupancy split and the fact set is different on
each side of it.

## 2.2 What the floor changes, which is the frame rather than a detail

`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`
carries four ratified rows, and three of them land here:

- `every_operation_arvo_declares_is_a_function_of_the_declared_width`. So **no
  fact the derivation produces may enter an answer.** The derivation is entirely
  on the cost side of the firewall
  `proposal::no_cost_model_may_move_an_answer` draws.
- `the_carrier_is_observable_through_the_ambient_layout_observation_alone`, with
  `alignment: aligned` written rather than omitted, because the claim is about a
  value that is the sole occupant of its own allocation.
- `at_shared_occupancy_no_per_element_footprint_observation_exists`. At a packed
  placement the element is reached through its carrier, no size-bounded contract
  ranges over it, and **what is observable there is the allocation's stride,
  which is a property of the column and not of the element.**

Put together: **the container derivation does not produce semantics. It produces
a placement.** Where the bits go and how they are reached, and nothing about what
they mean. That is worth stating as the frame because it is what makes the count
answerable at all: a placement has coordinates, and which coordinates exist
depends on whether the value has its allocation to itself.

## 2.3 The answer

**The derivation has one output at sole occupancy and three at shared occupancy,
and that is two arms rather than an inconsistency.**

| | facts the derivation owns | sort |
|---|---|---|
| sole occupancy | the carrier | type |
| shared occupancy | the carrier, the access width, the stride | type, type, contract |

At **sole occupancy** the value is the only logical occupant of its allocation.
Consecutive values are one carrier apart and one load reaches the whole of one,
so the stride and the access width are pure functions of the carrier. Under
`proposal::a_fact_is_carried_when_producing_it_applies_a_rule_the_strategy_owns`,
which is the two-expert clause, a pure function of what every site already holds
is **recomputed rather than carried**. One output. `226_probes/p1` confirms the
redundancy over widths 1 to 128 and its C2 control shows the check can see a
difference, by reporting a padded placement as differing at all 128 widths.

At **shared occupancy** neither is a function of the carrier, and that is the
load-bearing measurement.

## 2.4 The measurement, and it broke my first criterion

**`226_probes/p1` asked the wrong question and its own run said so.** I set out to
confirm what the registry already carries, that the native rung partition and the
access rung partition of widths 1 to 128 share no jump point. Two access rules
were computed rather than one, so the answer would not turn on which packing rule
a design ships: a *tight* rule using the offsets a packed column actually
produces, which are the multiples of `gcd(W, 8)`, and a *loose* rule assuming the
worst offset is 7 at every width.

The carrier ladder jumps at 9, 17, 33 and 65 and is monotone. The loose access
ladder jumps at 2, 10, 26, 58 and 122 and shares none of them. **The tight access
ladder shares all four**, plus twenty of its own.

So the claim as the register carries it is **true under one packing rule and
false under the other, and neither file that carries it says which rule it
assumed.** Reported rather than smoothed over. The separation survives under both
rules by different arguments, and the tight rule's is the stronger of the two:
the tight access ladder is **non-monotone**, returning `u16` at width 3 and `u8`
at width 4, while the carrier ladder is monotone, and a non-monotone function is
not a reparameterisation of a monotone one at any width.

**But jump counting was never the right instrument.** What
`a_fact_is_carried_when_producing_it_applies_a_rule_the_strategy_owns` needs
answered is whether a site holding the carrier can recompute the access width,
which is the question of whether access is a *function* of carrier:

```
access is a function of carrier  <=>  for all W1, W2:
    carrier(W1) == carrier(W2)  implies  access(W1) == access(W2)
```

`226_probes/p1b` answers it. **It is not, and neither is the converse**, under
both packing rules: 432 and 651 violating pairs forward, 416 and 644 back. The
first witness under the tight rule is widths 1 and 3, which share the carrier
`u8` and need loads of 8 and 16 bits.

Three controls, all firing. A rule defined equal to the carrier reports zero. A
constant rule reports zero forward and 5440 back, so the search can come back
empty and can come back full. A single planted violation at width 5 is found and
nothing else is.

**And `p1b`'s first run panicked at width 123**, where the tight rule needs a
136-bit window and the widest native type is 128 bits. That is not an error, it
is a fact I had not anticipated and now report: **the access ladder runs out of
native containers before the carrier ladder does.** A design that packs a 123-bit
declared width has a straddling element no single native load reaches, which is a
real constraint on the shared-occupancy arm and belongs in whatever design round
takes this.

## 2.5 What each fact is for, and what sort it takes

**The carrier.** The machine integer the value's bits sit inside for one access.
At sole occupancy it *is* the observable footprint, by the ratified layout row,
which is what makes it the thing `strategy::cold`'s intent ranges over. It is a
**type**, and that is settled rather than chosen: `probe::a_single_richer_output_suffices_only_as_a_type`
records the value-valued spelling refused six times across three syntactic
positions, each naming `generic_const_exprs`, which the constraints forbid.

**The access width.** How many bits one load must touch to reach an element at
its worst reachable offset. A **type**, for the same reason and by the same
refusal: a generic body has to name it as the type it loads. `p1b` is why it is a
separate fact rather than a projection of the carrier.

**The stride.** The bit distance from one element to the next. Not a type,
because nothing needs to name a type from it, so under
`proposal::a_carried_fact_takes_the_sort_its_consuming_site_uses_it_in` it is a
constant. **But not a bare constant**, and this is the one place the existing
clauses are not quite enough:
`proposal::a_fact_delivered_as_a_const_a_generic_body_loops_over_costs_the_reduction`
measured that a fact delivered as an associated const that a generic body loops
over serialises the reduction onto one accumulator and is worse code than a
hand-written twin at and above width 18, while the identical fact delivered as a
per-width trait contract matches the twin at every width tested. The stride is
exactly a loop bound. **So it is carried as a contract, not as a const**, and
that is a consequence of a row already in the register rather than a new claim.

## 2.6 The predicates

**F1. The carrier and the access width are two independent facts of the declared
width, at shared occupancy.**

```
total_width:     W in 1..=128: exhaustive, the whole span the native set {8,16,32,64,128} reaches
container:       the native set {8, 16, 32, 64, 128}
alignment:       aligned, straddling
signedness:      any: construction, the placement is computed from the declared bit count and a
                 sign bit is one of those bits rather than a coordinate beside them
fraction_width:  any: construction, the point's position moves no bit, so the placement reads the
                 total width and never the split
threads:         1
target_features: host aarch64-apple-darwin
toolchain:       nightly-2026-05-28
build_profile:   release
```

The two `construction` tokens oblige `evidence` to name an instrument that varied
those axes and found no movement, per
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`. That instrument
is `226_probes/p4`, built for this and nothing else: 16512 cells, total width 1 to
128 by fraction width 0 to W-1 by both signednesses, zero movement in the carrier
rule or either access rule, and a control rule that reads the sign and the
fraction moving in 2457 of them.

**The weakness of that warrant, stated rather than left to be found.** A
differential control over functions that syntactically ignore two of their three
arguments is weak evidence. What it certifies is that *these candidate placement
rules* do not read those axes, which is what the ruling obliges an instrument to
show, and it does not establish that no correct placement rule could. One family
plausibly would: an encoding that is neither two's complement, sign-magnitude nor
offset binary might want a placement the sign participates in. That is the hole in
`signedness: any` and it is named here rather than buried.

**`target_features` is written as the host rather than as `any`**, and
`threads: 1` rather than `any`, because I varied neither. Both are pure integer
computations that plainly cannot move with either, and under
`ruling::a_predicate_lists_only_what_holds` plainly-cannot is not a warrant. A
later seat that wants them widened runs the differential and writes its own row;
predicates are append-only and I am not widening these in place.

**F2. The two currying orders of the ladder resolve to one carrier.**

```
total_width: W in {3,4,8,9,13,16,17,32,33,64}: swept, the widths spanning every jump point of
             both partitions in the model
strategy:    Cold, Warm, Hot
container:   the native set {8, 16, 32, 64, 128}
toolchain:   nightly-2026-05-28
```

Evidence `226_probes/p2`, with `p2b` and `p2c` as the two files that must refuse
and do.

**A notation gap, reported rather than worked around.** F2 is a compile result:
nothing runs, so `build_profile` has no value to take, and `threads` and
`target_features` have nothing to range over. Under the discipline an absent axis
says the finding holds nowhere that axis exists, which reads as a much stronger
negative than I mean. The notation has three region-free spellings and none of
them is *compile-only*. `proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery`
already records the same gap from the other side, that all twenty-one declared
dimensions are numeric or machine axes and none can express a region over the
canon's own contents. This is a second instance of that gap on a different axis
and it wants an answer before more compile results are written.

**And no `dimension` row declares occupancy.** The whole of question one turns on
sole against shared, and the only adjacent declared axis is `alignment`, at a
different granularity. `proposal::at_shared_occupancy_no_per_element_footprint_observation_exists`
flagged this in its own note and said one seat should not make the call alone. I
agree, I am the second seat to hit it, and I say the same: **the arms above cannot
be gated on the axis they are actually keyed on, and that is a gap in the
dimension vocabulary rather than in the finding.**

## 2.7 The standing disagreement does not survive the dissolution

The question's note records a disagreement neither source file addresses:
**whether the strategy is an upstream selector the ladder never sees, or a key of
the ladder itself.**

**It does not survive, and it does not survive for the reason the floor supplies.**

Under the container premise as it stood, the disagreement was load-bearing. If
behaviour were stated over the container, then a strategy choosing the container
would be choosing the semantics, so the strategy had to be visible all the way
down the ladder and a design that hid it would be hiding an answer. The floor
ratified the opposite: every declared operation is a function of the declared
width, so **the strategy cannot reach any answer through the ladder at all.**
What is left is not a semantic question.

And what is left is not two functions. The two readings are

```
selector:  carrier = L_S(W)
key:       carrier = L(W, S)
```

which are one function curried two ways. They cannot disagree extensionally,
because `L_S(W)` and `L(W,S)` name the same value by construction. So the
disagreement is about **where the strategy is bound**, and binding time is
settled: under `ruling::never_a_runtime_check_and_one_lowered_path` and
`ruling::the_predicate_is_whatever_is_available_at_const_time`, the strategy is a
type parameter present at const time in either spelling and both monomorphise to
one lowered path.

`226_probes/p2` is that claim compiled. Both orders build under the pin with no
feature gate, under `#![no_std]`, no `alloc`, no `dyn`, no `TypeId`, and resolve
to the same carrier at all ten widths under all three strategies. The two tables
are written out separately by hand rather than generated from one macro
invocation, because generating both would make the agreement true by construction
and the probe would prove nothing. `p2b` is that file with one cell of the second
table changed and it refuses, naming the two types. `p2c` makes the
distinguishability arm false and refuses, which is what stops the agreement result
being vacuous over a table where every objective gave the same carrier.

**`p2b`'s own first draft edited both tables**, because the two lines are
textually identical, so the orders still agreed and it compiled. A control that
moves both sides of an equality is not a control. Recorded in the file's header
rather than quietly fixed.

**So: a spelling choice, and a design-round one.**
`ruling::the_canon_does_not_police_what_shape_a_law_takes` is the governing
posture, in op's words: if a law is a law it should be expressed so that it
actually works. The canon has nothing to say here and should say nothing.

## 2.8 What replaces it, because refuting is not enough

The two readings are not equally cheap, and the difference is real even though it
is not semantic. Keyed on the strategy, the ladder must be total over `(W, S)`
and its impl set carries the union of every objective's jump points. Selecting
among ladders, each carries only its own.

But **both readings name the wrong key**, and that is the constructive answer.
`proposal::the_named_strategies_are_points_in_a_product_and_the_flat_set_is_a_slice`
is a two-expert row: a strategy name is a binding rather than a member of a closed
set, and a fifth unnamed point costs one alias symbol and eight bytes. If the
ladder is keyed on the strategy *name*, adding a strategy extends the ladder and
is a design change, which contradicts that row. If it is keyed on the width alone,
the objective has nowhere to live.

**The ladder is keyed on the placement objective the strategy names, and the
strategy-to-objective map is the selector.** Three consequences, and all three are
things the canon already wants:

- Adding a strategy that binds an existing objective costs an alias and no ladder
  impls, which is what the two-expert row measured.
- The ladder's size is a function of the objective count rather than the strategy
  count, so `strategy::cold`'s intent surviving the set being four or seventeen or
  a billion is structural rather than a promise.
- It degenerates correctly. `proposal::a_coordinate_set_is_a_countable_ceiling_on_how_many_strategies_can_exist`
  finds that with one cost coordinate exactly one section is reachable, by algebra.
  With one placement objective there is one ladder and every strategy places
  identically, which is the same degeneracy arriving from the other side.

`p2` compiles this shape, not the two the register recorded: its order A is
`strategy -> objective` then `(width, objective) -> carrier`, and its
distinguishability arm shows the three objectives are genuinely three, coinciding
in two different patterns at widths 3 and 4 so the table is not one column wearing
three names.

**How many objectives there are is not mine.** Two or three are readable off the
strategy intents, `Cold` minimising and `Hot` weighing footprint below speed, but
`ruling::the_carrier_question_waits_on_the_contention_measurement` is open at
`rung = "open"` and the objective count is close enough to it that settling one
settles part of the other. What closes it is the contention run that deferral
names.

## 2.9 What I could not settle on question one

- **The occupancy axis.** Second seat to hit it, same conclusion, still not one
  seat's call.
- **The compile-only region.** Second instance of a known notation gap.
- **The objective count**, above.
- **Whether the register's shared-jump-point claim should be corrected or
  scoped.** I established it is model-dependent; which packing rule its source
  assumed is a fact about a file I did not open.

---

# Question two: which operation set the design ships

## 3.0 What op's `227` changed for me, which is less than it changed for others

I read `227` after committing every probe above and after drafting sections 2.1
to 2.9. It corrects a reading of the standards bound: parity is in output, not in
the internals, and a standard may ship with marked holes because what has to ship
is a framework that is sound, expressible, mathematically accurate and exhaustive.

**My derivation did not go through the inventory reading and does not change.** I
never treated the standards bound as fixing arvo's operation list; I reached an
admission rule from
`every_operation_arvo_declares_is_a_function_of_the_declared_width` and from
`the_concept_is_closed_and_the_inventory_is_open`, and `p3` was written and
committed before `227` existed. Saying so plainly because the brief asked, and
because a derivation that reaches a right answer by a route he has now closed is
worth recording as exactly that. Mine did not take that route, and I would rather
say so than claim credit for surviving a correction I was never exposed to.

**What `227` does add is one level of structure I did not have**, and it is the
better statement: he separates *expressibility*, where the framework may have no
holes, from *inventory*, where an operation may be a marked `FIXME`. My admission
rule is the expressibility criterion. Which admitted operations ship now is a
design ledger. I take the reframing as right and say why in 3.4.

## 3.1 The floor already answered what this question was blocking

`question::which_operation_set_the_design_ships` was load-bearing because
`proposal::observability_is_relative_to_a_declared_signature` makes footprint
observability *a theorem schema whose parameter is the operation set*, with no
truth value until the signature is named. Naming the set was the route to a truth
value, and thence to the primitive count and to four of the five things the
question's `unblocks` field claims.

**The floor got the truth value without naming the set**, and it got it by finding
an observation that is not in arvo's operation set at all:
`the_carrier_is_observable_through_the_ambient_layout_observation_alone` turns on
the host language's layout observation on a sized type, which arvo does not own
and cannot withhold.

So **the footprint answer is independent of which operation set arvo ships.**
Adding or removing an arvo operation cannot move it, because the observation that
decides it is not arvo's to add or remove. That cuts two of the five things
`unblocks` names outright: whether footprint is observable, and whether the count
of primitives is container-relative, which is now a function of occupancy rather
than of the operation set, and is answered yes at sole and no at shared.

## 3.2 The count is constant in the operation set, and it is a theorem

`retirement::r146_the_count_is_not_a_property_of_the_design` retired the claim
that the count is not a property of the design, replacing it with *the
shape-to-count function, which is monotone in the observation set and saturates.*
The floor's own `says` sharpens that: **identity saturates at the declared
signature set.**

The argument, and it is short. Every declared operation is a function of the
declared width. Two realisations of one declaration share a declared width.
Therefore every declared operation gives them the same answer. Therefore **no
declared operation, present or future, can split them.** The shape-to-count
function is not merely monotone-and-saturating in the operation set; it is
**constant** in it. It saturated before the first operation was named.

**And here I retire my own probe's positive arm.** `226_probes/p3` sweeps widths
3 to 10 exhaustively over the whole value domain and every admissible carrier, and
reports the class count as 1 after every prefix of `[encode, add, mul, xor, fma]`.
**That result is true by construction and proves nothing.** The declared-width
operations in the model ignore the carrier parameter by definition, so of course
they cannot split by it. It is setup that helps, it is the failure `the-test-gate`
names, and I nearly shipped it as a measurement. The claim is analytic and needs
an argument rather than a sweep, and the sweep restates the argument's premise.

What `p3` actually contributes is its two controls, and those are real. A
footprint observation added to the same set splits to **exactly** the admissible
carrier count, 5 at widths 3 to 8 and 4 at 9 and 10, which pins the counter's
ceiling as well as its floor. And the second control found something nobody asked
for, which is section 3.5.

## 3.3 The answer: the design ships an admission rule, not a list

**All three recorded options are answers to "which extension", and the answer is
an intension.**

> An operation is admitted exactly when it is a function of the declared
> signature. Where two candidate realisations of one operation name disagree on
> any input, the name is ambiguous: it denotes two operations, and the coordinate
> distinguishing them is missing from the declared signature. The repair is to add
> that coordinate to the declaration, at which point both realisations are
> admitted as distinct declared operations.

**This is the format spine's own shape one level down.**
`ruling::the_format_spine_is_canon` ratifies that *the concept is closed and the
inventory of admitted instances is open*, for formats. The same sentence for
operations is not an analogy; it is the same mechanism applied to the next tier,
and reusing it costs the canon one sentence rather than a new machine.

**And it is the firewall in a second voice.**
`proposal::no_cost_model_may_move_an_answer` says no cost model may move an
answer. The admission rule says an operation whose answer moves under a cost
choice is not one operation, it is two, and both get declared. Same claim,
prohibition on one side and admission criterion on the other. Two topics reaching
one statement from opposite ends is worth naming as convergence rather than
leaving as a coincidence.

**Why an intension is the right shape here and not merely a dodge.**
`ruling::arvo_is_a_library_and_the_value_composes_on_top` says the selling point
is the algorithm crates every downstream repository uses. A library whose
operation set is a closed list forecloses every algorithm crate that has not been
written. One that ships an admission rule admits them, and the rule is what a
downstream author checks their operation against.
`ruling::the_selection_criterion_is_what_serves_the_rest_of_arvo` says to pick the
shape that serves the rest of arvo rather than the one best in isolation, and this
is that shape.

## 3.4 The five-operations bite is the mechanism, not a counterexample

The question's own note carries the sharpest objection available: *at unsigned, a
design shipping four of the five operations would report the intermediate axis
dead and be wrong.*

That is exactly right and it does not break the rule; it is the rule firing.

The four are `encode, add, mul, xor` and the fifth is the multiply-add. With four,
neither the fused nor the stepwise form is in the set, so nothing can distinguish
them and `accumulator_width` looks dead. Add the multiply-add and the axis is
live, because `law::the_fused_and_the_stepwise_multiply_add_denote_one_function`
reports the two forms denoting different functions at signed saturating.

**But that law row already says what to do about it**, and it says it in the
admission rule's words: *where they do not, they are two denotations rather than
two lowerings of one, and the intermediate axis already names the difference: a
consumer wanting the fused answer declares the exact-intermediate position.*

So `multiply_add` as a bare name is **not** a function of the declared signature
and the rule refuses it. `multiply_add` with the intermediate position declared
**is**, and the rule admits it, at both positions, as two declared operations. A
design shipping four of five is not wrong because it is short an operation; it is
wrong because it left a coordinate out of the signature. **The repair is always to
widen the declaration, never to let the realisation decide**, and that is the
firewall again.

## 3.5 Op's open question: is the fused result reachable without an fma?

`227` leaves this open explicitly and says it is the seats' to establish. It is
establishable, and the answer is a region rather than a yes or a no.

**`226_probes/p6`.** Over every triple at total widths 5, 6 and 7, every fraction
width, both signednesses and both range policies, comparing `adapt(a*b + c)`
against `adapt(a*b) + c`. Where the range policy is applied is a modelling choice
and not a detail, so both forms of the composition are run: placing into the
declared format once at the end, and placing at both steps, which is what a real
stepwise implementation does.

| | reaches the fused result |
|---|---|
| unsigned, wrap or saturate | floor, ceil, toward zero, away from zero, half up; half even at F = 0 only |
| signed, wrap | floor, ceil, half up; toward zero, away from zero, half even at F = 0 only |
| signed, saturate | none, at any fraction width including zero |

**So the fused result is reachable by composing multiply and add in most of the
space, and unreachable at signed saturating.** Op's "second half may still hold"
holds, and holds far more narrowly than the inventory argument claimed: not *the
fma is unreachable*, but *the fma is unreachable in one cell*.

**Two mechanisms, and both are structural rather than empirical.**

The first came out of a control that failed. I planted a supposedly
non-equivariant adaptation, `3 * r >= den`, and it agreed everywhere. The reason
is the general statement: translating by a grid point adds a multiple of the
denominator to the numerator, which leaves the residue untouched, so **any
adaptation whose decision reads only the residue is translation-equivariant by
construction.** A rule that cannot commute has to read the quotient. Half even
reads it, via `q % 2`; toward zero and away from zero read the sign of the
numerator, which is constant under unsigned and is not under signed. **That one
sentence accounts for the entire measured pattern including its signedness
dependence**, and it explains why the two registry law rows differ exactly where
they do rather than merely recording that they differ.

The second is the isolated cell. **Wrap composes because wrapping is a ring
homomorphism**, so applying it once at the end and twice along the way give the
same answer, which the run confirms cell for cell. **Saturation is not a
homomorphism**, and at unsigned it is harmless anyway because a product of
non-negative values never meets the lower clamp. At signed saturating the double
clamp is what breaks it, and it breaks it at F = 0 too, which is the tell that the
mechanism is the range policy rather than the rounding.

**This reproduces two registry rows digit for digit at W = 6 and extends both to
W = 5 and W = 7**, which neither reaches, and it supplies the mechanism that
`law::the_fused_and_the_stepwise_multiply_add_denote_one_function`'s own `gap`
records as missing: *no witness, the failing cell is established by a count of
differing triples rather than by a named instance.* The named instance is now the
whole cell, and the reason is the double clamp.

**The design consequence, which is the actual answer to the question.**

> The fused multiply-add is not a required member of the inventory. It is a
> required **arm** at signed saturating, and everywhere else it is a free lowering
> of a composition arvo already has.

That is `ruling::arms_over_regions_are_the_fundamental_heart` exactly: a small arm
holding where it is optimal and nowhere else, with a const predicate over
signedness, overflow policy and rounding mode, all three available at const time.
And it is `227`'s marked-hole shape without needing a hole: ship the composition,
ship the one arm, and the framework is exhaustive because it can express both.

**Predicate for F3, the reachability finding.**

```
total_width:     W in 5..=7: exhaustive, every (a, b, c) triple at each width
fraction_width:  F in 0..=W-1: exhaustive
signedness:      unsigned, signed
overflow_policy: wrap, sat
rounding:        floor, ceil, toward_zero, away_from_zero, half_up, half_even
operation:       multiply-add
arity:           3
chain_length:    2
threads:         1
target_features: host aarch64-apple-darwin
toolchain:       nightly-2026-05-28
build_profile:   release
```

`stochastic` is absent from the rounding line because I did not run it, and it is
the one of the six ratified names whose decision is not a function of the residue
and the quotient at all. **I expect it to fail and I did not measure it**, so it
is absent, which under the discipline says the finding does not hold there.

## 3.6 A defect found on the way, and it needs a deliberate call

**Three live predicate entries spell a rounding mode with a word a ratified
ruling retired**, and the ambiguity is load-bearing rather than cosmetic.

`ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names` is
`rung = "ratified"`, `ratified_by = "op"`, and fixes the vocabulary as
`toward_zero, floor, ceil, half_up, half_even, stochastic`. These carry
`rounding: truncate toward zero`, which is none of the six:

- `law::the_fused_and_the_stepwise_multiply_add_denote_one_function`, in both its
  `holds` and its `fails` block.
- `proposal::where_fusion_changes_the_answer_it_is_not_a_lowering`.

**Under either reading of that phrase, the row conflicts with something**, which
is what makes it a call rather than a typo:

- Read as `toward_zero`: the row's `holds` block claims agreement at signed, wrap,
  toward zero, and its sibling
  `law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping` puts
  toward zero in its `fails` block at the same signedness, policy and width. `p6`
  agrees with the sibling. **The two rows contradict each other.**
- Read as `floor`, which is what bit truncation by an arithmetic shift gives: the
  row's `fails` block claims floor fails at signed saturating, and `p6` finds
  floor **reaching** the fused result there under one placement while failing
  under two. Consistent only under the two-placement reading, which the row does
  not state.

`retirement::r146_fractional_shift_as_an_arithmetic_shift_right` already retired
the claim that the fractional shift be spelled as an arithmetic shift right,
*because the swap is measured to change up to 44.53 percent of multiply answers on
signed shapes*. So the workspace has already paid for this exact ambiguity once.
**Not mine to resolve**: the two readings give different answers and picking one
edits a live law row's region. Reported with the rows named.

---

# 4. Two small wins, and neither was what I went looking for

**W1. An unprojected intermediate is observable exactly where the operation's
growth above the declared width exceeds the headroom the narrowest admissible
carrier already has.**

`226_probes/p3`'s second control. An unprojected add grows by one bit and splits
the classes only at width 8, where the headroom is zero. An unprojected multiply
grows by W bits and splits at 5, 6, 7, 8, 9 and 10 and not at 3 or 4. Sixteen
cells, both outcomes present in each arm, the biconditional held in every one.

The control's first form checked only that the add splits, and it did not below
width 8. **The reason is the finding rather than a broken control**, and it is a
real win: the projection back to the declared width, which
`every_operation_arvo_declares_is_a_function_of_the_declared_width`'s own note
calls an obligation with a stated trigger, **may be elided for free in a nameable
region.** `headroom = carrier_bits(W) - W` and `growth` are both const, so this is
an arm.

**W2. That arm is wider than the obvious closed form, and the wider one is free.**

`226_probes/p5`. A chain of k additions accumulated without projecting between
steps and projected once at the end agrees across carriers up to some k. Two
routes to the boundary, because a closed form checked against itself is not
checked. The form `ceil(log2(k+1)) <= headroom` is **sound and conservative**: over
widths 3 to 16 and chain lengths 1 to 4096 it disagrees with the exact condition
in 5 cells and is unsound in none. The exact condition, `(k+1) * (2^W - 1) < 2^C`,
allows 35 free additions at width 3 where the form allows 31, and 16 at width 4
where the form allows 15. Both are const-computable, so the wider one costs
nothing to prefer.

Controls: the same form with one bit of slack it does not have disagrees in 313
cells, every one unsound, so the direction that matters is visible. And both
outcomes must appear at every width with headroom, with only the splitting outcome
where headroom is zero. **That second control's first form demanded both outcomes
everywhere and failed on correct data**, because at zero headroom a two-term sum
already wraps the carrier and no chain is ever free. The criterion was wrong
rather than the data, and it is two claims now rather than an exemption.

Predicate for both, and it is narrow:

```
total_width:     W in 3..=10 (W1), W in 3..=16 (W2): exhaustive
fraction_width:  0
signedness:      unsigned
overflow_policy: wrap
container:       the native set, every member >= W
operation:       add, mul (W1); add (W2)
arity:           2 (W1); chain_length 1..=4096 (W2)
threads:         1
target_features: host aarch64-apple-darwin
toolchain:       nightly-2026-05-28
build_profile:   release
```

**Signed and fractional are absent and that is a real limit**, not a formality:
the elision argument is about range, and a signed range is not symmetric. Whoever
extends this should expect the boundary to move.

# 5. Findings outside my two questions

**5.1 Five lints have no tests and scan a tree that does not exist.**
`mock/lints/no_std_enforcer.rs`, `no_alloc_enforcer.rs`, `no_dynamic_dispatch.rs`,
`no_runtime_grow.rs` and `arvo_bits_traits_only.rs` are `CrateLint`s over
`mock/crates/`, which was deleted when the canon work began. None carries a
`#[cfg(test)] mod tests` at all, against 439 tests across the registry lints. They
are the enforcers of `ruling::the_operating_constraints_are_intents_and_rules`,
which is the frame everything I wrote above is bounded by, and they will fire again
the day a crate returns, untested, having drifted through however many toolchain
changes.

**Not a fabricated green.** `cargo mock check` reports `! build no workspace
members yet` and `! tests no workspace members yet` plainly rather than counting
them as passing, which is the honest thing and is why this is a report rather than
a refusal. But the first crate written from this canon will be gated by five
untested lints, and writing their tests now costs nothing and cannot be done later
without the crates to test them against.

**5.2 The `dimension` vocabulary has no occupancy axis and no compile-time axis.**
Sections 2.6 and 2.2. The first blocks gating the two arms question one produces.
The second means every compile-only finding in this panel is written in a notation
that says it holds nowhere.

**5.3 The register's shared-jump-point claim is model-dependent.** Section 2.4.
True under one packing rule, false under the other, and neither carrier says which
it assumed.

**5.4 The retired rounding word, three live entries.** Section 3.6.

# 6. What I did not take, so the next seat starts from a list

- **Option three on question one, one richer type with named projections.** Not
  refuted and not adopted: it is the packaging of whichever arm you are in, and
  packaging is design. If a design round wants it, the shape is a trait with
  `type Carrier`, `type Access` and a stride contract, and the sole-occupancy arm
  simply leaves the last two as the carrier's own consequences.
- **Keying the ladder structurally rather than on a width marker.**
  `probe::the_structural_width_ladder_compiles_gate_free_on_the_pin` records a
  source that does it and compiles gate-free. I enumerated in `p2` because the
  claim under test was curry equivalence, which does not need totality. A seat
  attacking the ladder's totality should start from that source rather than from
  mine, and should note that probe row's own `standing = "uncontrolled"`.
- **Pricing the two currying orders by impl count.** Real, measurable, and I did
  not measure it, because compile-time cost has no axis in the notation and an
  unpriceable number is worse than none. It is a bench question if anything.
- **Extending the reachability sweep to `stochastic`.** Named as absent in 3.5. I
  expect it to fail and expectation is not a predicate.
- **Extending W1 and W2 to signed and fractional.** Section 4.
- **The access ladder running out above width 121.** Found in 2.4, not pursued. A
  design that packs those widths needs a two-load path and nothing in the register
  mentions one.

# 7. What a next seat should attack in what I wrote

- **The occupancy split is the whole of question one's answer and rests on one
  ratified row that is itself argued rather than measured.**
  `at_shared_occupancy_no_per_element_footprint_observation_exists` says in its own
  note that nothing instantiates a packed column and that a probe building the
  shared placement would raise it. The floor's promotion record says seat 225 built
  one. I did not open it. **Someone should check that my two arms match what that
  instrument actually did**, because if the shared-occupancy arm has a per-element
  observation after all, my count is wrong on that side.
- **The `signedness: any` warrant in F1**, per the weakness stated in 2.6.
- **The admission rule against an operation nobody has thought of.** I tested it
  against the multiply-add, which is the one the register already knew was hard.
  A rule that only handles the known case is not a rule.
- **Whether `p6`'s two-placement model is the right stepwise form.** It is what I
  believe a real implementation does and I did not establish that from any row.
  If the design places only once, the signed-saturating cell reopens.
