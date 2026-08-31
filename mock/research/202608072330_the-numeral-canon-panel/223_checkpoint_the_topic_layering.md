# 223. Checkpoint: what rests on what, so bottom-up is mechanical

Op set a standing order that the panel fills its unanswered, unsettled and missing parts from the
foundations upward, so the fundamentals reach design and implementation soonest and each layer above
gets worked in parallel as it settles. His words are captured at
`.data/op-responses/202608311926_arvo-panel-bottom-up.md` in the workspace repository.

**The order cannot be followed, because the layering it refers to does not exist anywhere.** The
`topic` namespace carries `id`, `what`, `unit` and `keywords`, and nothing else. Twenty topics, zero
dependency edges. `unit` looks like an ordering and is not: the namespace's own header says a unit
name records when something was argued, which is a fact about the panel rather than about arvo, and
the units are numbered two, three, four, five, eight in the order the panel happened to reach them.

So "work bottom-up" is currently a judgement the coordinator re-makes from scratch at each dispatch,
which is the shape that drifts without anybody noticing, because each individual call looks
reasonable and nothing compares two of them.

**This file derives the layering and is one expert's reading.** It is written so the next seat can
attack a stated edge set rather than re-derive one.

## What this is derived from, and what it is not

Read off the `what` sentence of each of the twenty `topic` rows, and nothing else. Not from the
panel's own file order, not from the units, not from any proposal or ruling, and not from the shipped
tree, which is empty by design while the canon is written.

**The test applied at each edge**: can topic B's subject be stated at all without a decision belonging
to topic A? Not whether B is more interesting than A, and not whether the panel argued A first.

## Three groups, and only one of them is a stack

**The numeric stack**, which is what the order is about and where dispatch order matters.

**The frame**, which every layer sits inside and which is not above or below anything: the constraints
arvo is built under and does not relitigate, what arvo is and is for, what it assumes about the
machine, and when a thing is decided. These bound the stack rather than resting on it.

**The canon's own machinery**, which is about the canon rather than about arvo: what a canon sentence
may be, how a finding states its region, what things are called, and how the panel is run. A topic
here can be settled at any time and blocks nothing in the stack, with one exception noted below.

Filing all twenty into one order is the error this split exists to prevent, and it is the one a reader
in a hurry makes, because every topic is a row in one file and they look alike.

## The stack, bottom first

**`the_container_premise` is the floor and it is blocking.** Its own `what` says so: whether a
declared numeral's behaviour is stated over its declared width or over the container carrying it, and
that no wording of several downstream clauses is true on both branches. A topic that changes the truth
of sentences beneath it is not a peer of those sentences.

**`the_format` rests on it.** A format is the widths a numeral declares, its signedness, and how the
declared format relates to the container underneath. The last clause is the container premise; the
format cannot be stated until the premise picks a branch.

**`rounding` and `overflow_policy` rest on the format**, and on nothing else in the stack. Rounding is
an axis over a fraction width and overflow is what happens at the edge of a declared range; both need
the format to have said what a fraction width and a range are, and neither needs the other.

**`the_number_system` rests on the format.** What a candidate has to expose to be admitted needs the
vocabulary a format supplies for saying what it exposes.

**`the_realisation_map` rests on the format and the container premise.** It is how a declared numeral
reaches machine storage and machine operations, which is the declared-against-container question in
its operational form.

**`the_primitive` rests on the realisation map and the number system.** Its own `what` spells the
dependency out: a value set with a realisation map over a declared operation set. The value set comes
from the number system and the map from the realisation map, so this row cannot be stated before
either.

**`the_strategy_axis` rests on rounding, overflow policy and the realisation map.** A strategy is what
a consumer selects along, and what it selects between is exactly the freedom those three leave. An
axis over an empty set of choices is not an axis, so the choices are named first.

**`the_strategy_object` rests on the strategy axis.** It asks how many components the strategy has and
what each ranges over, which presumes the strategy is a thing.

**`algebraic_laws` rests on the primitive, rounding and overflow policy.** A law's verdict is over an
arithmetic, and the arithmetic is not determined until the primitive and the two edge behaviours are.

**`the_chain` rests on the algebraic laws and the primitive.** Composition beyond a single operation
is stated in terms of what the single operations do.

**`validation` rests on the format and on binding time.** What validating a numeral means, and who
does it, needs the shape being validated and needs the answer to when it happens, and the second is a
frame topic rather than a stack one.

## The one edge that crosses out of the machinery group

**`the_predicate_notation` gates every finding in the stack**, because every finding states its region
in it. It is not above or below any topic; it is the language the others are written in. It is nearly
settled: the warrant marker landed today as
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`, and two rows remain open.

**Those two are worth closing ahead of the stack work** rather than in topic order, on the grounds that
a notation change after the fact rewrites every predicate written under the old one, and predicates
are append-only so the rewrite is not available.

## What this says about dispatch order

The floor is `the_container_premise`, then `the_format`, and everything else in the stack waits on
those two whatever its open-question count says. **A count is not a depth**, and `the_strategy_axis`
carrying fifteen open questions against `the_container_premise`'s one is exactly the trap: fifteen
answers derived before the floor is settled are fifteen answers that may not survive it.

The pair currently deriving in parallel has `the_format`, `the_number_system`, `rounding`,
`overflow_policy`, `the_primitive` and `the_container_premise`, which is the floor plus the layer
resting directly on it. That was dispatched before this derivation existed, and it happens to be right.
Saying that plainly rather than presenting the derivation as having chosen it.

## What would refute this

**An edge is wrong if topic B's open questions can be answered without touching A.** That is checkable
against the question rows rather than arguable: take an edge, read B's unanswered questions, and see
whether any of them turns on A's subject. The edges most likely to fail that test are
`the_number_system` on `the_format`, which may be closer to independent than stated here, and
`validation` on `binding_time`, which may be the only stack topic genuinely resting on a frame topic
and may instead belong to the machinery group entirely.

**And the group split is the load-bearing claim, not the edges.** If the frame topics turn out to sit
in the stack rather than around it, the order changes for most of the rows below them.

## Its standing

**One expert, and it is the coordinator's own derivation, which is the tier most likely to be wrong.**
It is recorded so dispatch has something stated to follow and something specific to attack, not
because it is settled. It is filed as `proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery`.
