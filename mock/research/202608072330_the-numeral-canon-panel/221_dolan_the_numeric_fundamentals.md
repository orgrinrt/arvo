# 221. The numeric fundamentals, derived

Seat 221. One half of a blind pair on the unanswered rows of
`question.toml` under `the_number_system`, `the_format`, `rounding`,
`overflow_policy`, `the_primitive` and `the_container_premise`.

## 0. The two gates, first

**Canon gate: passed, with one finding that is itself a canon-alignment
finding rather than a reason to refuse.** Checked against
`mock/registry/*.toml`, which `mockspace.toml` declares as `canon_paths`.
Nothing in the assignment asks for work the canon forbids:
`ruling::the_panel_finishes_the_canon_without_him` is ratified by op and puts
every remaining question on the panel, and
`ruling::two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`
is the mechanism. Deriving these is licensed.

The finding is that **three of my thirty-seven questions are already answered
by a proposition op ratified**, and they are sitting in the queue because the
edge that says so is one nothing reads. That is section 2, and it retires
three rows before any derivation starts.

**Test gate: run, and the suite is real in the surface I touch.** `cargo test
--workspace` in `mock/`, 152 tests across 20 files, all green. Green is the
weakest signal available, so I read bodies rather than counts, in the four
files that bear on what I write:
`a_settled_question_does_not_sit_in_the_queue.rs`,
`a_probe_reads_the_tree_it_sits_in.rs`,
`no_prose_cites_a_living_ledger_by_line.rs` and
`every_predicate_names_a_declared_axis.rs`.

None of them is decorative. Every one carries planted inputs in both
directions, every ceiling arm names the population it was measured over and
says which way it may move, and two of them carry, in their own doc comments,
the defect the first cut of that same arm had. The classifier note in
`no_prose_cites_a_living_ledger_by_line.rs` is the best thing in the four:
it records that partitioning by filename put a probe's own `FINDINGS.md` on
the repairable side, and that the seat which reported the class made the same
error in its own instrument the same day. That is a suite written by people
who expect to be wrong.

What it does not cover is section 2. `a_settled_question_does_not_sit_in_the_queue.rs`
exists precisely to stop a settled question reading as open, and it reads one
of the two namespaces that settle questions. That is a gap in a real test
rather than a fake one, and the difference matters: the arm is honest about
what it measures and nobody noticed what it does not reach.

## 1. What I read, when, and where my blindness is thin

Committed my probes at `472629c4` before writing a word of this file. I have
not opened any numbered member file of this panel and have not run `git log`.

What I read, all of it registry or harness:

- `mock/registry/*.toml` in full for `ruling`, `dimension`, `obligation`,
  `strategy`, and in full for the rows of `proposal`, `proposal-the-later-topics`,
  `law`, `law-the-later-topics`, `retirement` and `probe` that my questions
  name or that a slug I followed named.
- `mock/checks/src/corpus.rs` and five files under `mock/checks/tests/`.
- `mock/Cargo.toml`, `mockspace.toml`.

**Two places where my blindness is thin and I would rather say so than have it
inferred.**

**The registry is a shared premise, not an independent input.** My pair is
reading the same `proposal.toml` I am. Anywhere the two of us agree with a row
that is already there, that is two readers of one page, which
`expert-dispatch-defends-the-canon.md` calls shared drift rather than
corroboration. I mark every answer below with which it is.

**Seat 210 is my own persona.** The six `the_container_premise` rows are
`210_dolan`'s, and I read them before deriving. So on that question I am not a
second instance in any sense that counts, and I say so in section 6 rather
than quietly agreeing and letting the standing rise.

## 2. Three of my questions are answered, ratified, and in the queue anyway

**`mock/checks/tests/a_settled_question_does_not_sit_in_the_queue.rs:79-89`.**
`answered_by()` iterates `reg.of("ruling")` and builds the reverse index from
`ruling.answers`. That is the whole of the reverse edge anything in this
repository computes.

`proposal.answers` exists, 122 proposal rows use it, and the literal string
`"answers"` occurs exactly twice in `mock/checks`, `mock/lints` and
`mock/tools` put together, both of them in that one function and its
neighbour. `"ratifies"` occurs once. Nothing walks
`ruling.ratifies -> proposal -> proposal.answers`, which is the path a
proposition op ratified takes to reach a question.

`221_probes/p1_the_unread_answer_edge.rs` measures it, with four controls,
all of which fired. The one that matters is C4: a planted registry where a
proposal answers a question and no ruling does must be missed by the shipped
rule and caught by the wider one. If the shipped rule caught it there would be
no gap and the probe would count nothing.

```
questions declared                                    98
settled under RULE_A (what the shipped check sees)    14
reading as open under RULE_A                          84
of those, named by a `proposal.answers` edge          31
of those, backed by a RATIFIED proposition             3
```

The three:

| question | proposal | ratified through |
|---|---|---|
| `adaptation_in_identity_or_realisation` | `a_format_is_identified_by_its_ambient_domain_and_its_representable_set` | `ruling::the_format_spine_is_canon` |
| `is_the_number_system_inventory_open` | `the_concept_is_closed_and_the_inventory_is_open` | `ruling::the_format_spine_is_canon` |
| `which_width_coordinates_a_consumer_writes` | `membership_of_the_representable_set_is_one_affine_predicate` | `ruling::the_format_spine_is_canon` |

All three are in my assignment. **`ruling::the_format_spine_is_canon` is
`ratified_by = "both"`**: op stamped it and the expert convergence was already
there. Its `ratifies` list names four propositions, and three of those four
carry an `answers` edge into a question that is still sitting open.

Referential integrity on the unread edge is clean, 0 of 122 naming an
undeclared question, which is worth saying because it means the edge was
maintained carefully by people who had no way to know nothing reads it.

### Four notions of settled, and no question satisfies all four

The gap is wider than one unread field, and the shape of it is worth the four
lines it takes to say. A question in this registry can be marked settled in
four ways, and they are independent.

| criterion | questions |
|---|---|
| an `answered` field | 7 |
| a settled phrase in `note` or `bound` | 7 |
| an incoming `ruling.answers` edge | 12 |
| an incoming `proposal.answers` edge | 31 |

**Union 45. Intersection 0.** No question in the registry is marked settled by
all four. Worse, the proposal edge is disjoint from every one of the other
three: `proposal minus answered`, `proposal minus phrase` and `proposal minus
ruling` are all 31, so **every question the proposal edge settles is settled by
nothing else at all.** The 14 the shipped rule sees and the 31 it does not are
two populations with no overlap, which is why the shipped check reads clean
while a third of the settled corpus is invisible to it.

This also fixes what my thirty-seven means.
`question::what_then_validate_requires` is in my assignment because it carries
no `answered` field, and its `note` opens "Recorded as answered at `28` batch
one", which is one of the two phrases the shipped check matches. **So it is
settled under the shipped rule and unsettled under the brief's rule, and both
are correct about their own criterion.** That is not a defect in the brief. It
is the four notions again, met from the dispatch side.

**What this is worth.** The check's own module doc says the consequence is not
cosmetic: every roster of what op owes is built by reading `decider`, and a
question he answered goes back into the queue. It then repairs one namespace
and the same failure it documents is live in the other. Under
`ruling::the_panel_finishes_the_canon_without_him` there is nobody to send
them back to, so the cost has changed shape rather than gone away: a seat
dispatched on one of these three spends its budget deriving an answer that op
has already blessed, and if it derives a different one, the panel now has an
unratified answer sitting beside a ratified one with nothing saying which is
which.

**The repair is one function and it belongs in `mock/checks/tests/`, not
here.** Extend `answered_by()` to read `proposal.answers` as a second, weaker
edge, and to follow `ruling.ratifies` into it as a third, equal to the first.
It cannot be written this round: the phase is `TOPIC`, so a `.rs` edit under
`mock/checks` is refused by the write guard, and the registry is a
`canon_paths` tree. So the instrument is committed as a probe and the test is
owed. **Naming it precisely so nobody has to rediscover the shape:**
`mock/checks/tests/every_answer_edge_is_read.rs`, three arms, hard on the
ratified-backed set and a falling ceiling on the wider 31.

The corrected question count, since the brief asked me to say if I read a
different one: **thirty-seven** unanswered rows across my six topics, counting
rows with no `answered` field, which is what the brief specified and what I
measured. Under the wider rule of section 2, three of the thirty-seven are
answered and thirty-four are genuinely open.

## 3. The three ratified answers, transcribed

Not derivations. These are already canon and I am reporting where they are.

### Q18 `adaptation_in_identity_or_realisation`

**Answer: to its realisation. Option 2.**
`proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set`,
ratified through `ruling::the_format_spine_is_canon`, says it in those words:
"Adaptation choice and encoding are realisation, observable in computed values
and in pattern-level properties respectively, and not part of identity."

The question's third option, that the format unit's two-layer split dissolves
it, is a description of why the answer is what it is rather than a rival to
it. The two-layer split is the ratified spine: identity is the pair, the
adaptation is the derived slot, and the strategy selects a member of the slot
per operation. There is nothing left over.

The question's own note worries the residue may be naming, on the ground that
arvo's types carry the strategy either way. That worry survives and is a
separate question. **A type parameter is not an identity coordinate**, which
`proposal::membership_in_the_type_and_identity_are_two_criteria` (two experts)
states directly: what must be const-available decides membership in the type,
what is preserved up to denotation-preserving isomorphism decides identity,
and neither answers the other's question. So the strategy sitting in the type
is a membership fact and says nothing about identity. **These two rows
together close the question and its note, and neither cites the other.**

*Normative, no region, per the registry's own convention that a definition
carries none.*

### Q20 `is_the_number_system_inventory_open`

**Answer: the concept is closed and the inventory is open. Option 2, with the
amendment that "open" attaches to the inventory and not to the concept.**
`proposal::the_concept_is_closed_and_the_inventory_is_open`, ratified through
the same ruling.

The question's option 1 offers "closed, which is checkable and lets a canon
enumerate what it covers" and its option 2 offers "open, at the cost of the
canon being unable to say what it covers". **Both costs are avoided by the
ratified answer and the question's framing is what generates them.** A closed
concept says exactly what the canon covers, which is the first option's stated
benefit; an open inventory means a new instance joins by supplying the
concept's obligations rather than by amending the canon, which is the second
option's stated benefit. The question's own note says both cold derivations
reached it and neither closed it, and asks for "whether the concept has a
membership test that does not enumerate". It does, and it is the admission
contract of Q29.

*Normative, no region.*

### Q2 `which_width_coordinates_a_consumer_writes`, the definitional half

**The definitional half is ratified and it is option 4.**
`proposal::membership_of_the_representable_set_is_one_affine_predicate`:
"Membership is one predicate over one parameterisation: an affine slot
function, a quantum per magnitude and a phase, of which integers, fixed point,
scaled integers and floats are points."

So the object is not a width pair. It is a point of a three-coordinate
parameterisation, and a width pair is what that point is called in one slice
of it. Q2's fourth option says exactly this and the register carries it "as a
reading rather than a rival"; the ratification makes it the definition.

**Three coordinates, not two**, and the third is doing work. The same row: "a
nonzero phase decides whether the identity adaptation ever occurs and whether
the set carries an additive identity at all", and
`proposal::a_nonzero_phase_leaves_the_representable_set_without_an_additive_identity`
carries the consequence. **A consumer surface with two coordinates cannot name
a phase**, so a surface answer that stops at a pair has silently answered a
second question, which is whether a nonzero phase is declarable at all. I do
not think that is what anybody intended, and section 4 is where the surface
half goes.

*Normative, no region.*

## 4. The format and the coordinate surface

### Q2 continued, the surface half: total and fraction

**Answer: total and fraction, plus phase. Option 1 for the pair, extended.**

The question hands the surface choice no decider and its options price the two
pairs against each other on familiarity. There is a bound on it that is not
familiarity, and it is op's:
`obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`,
which he stated as a bound on a different question and which
`ruling::the_standards_bound_starts_at_two_and_reserves_the_rest` scopes to
MATLAB `fi`/`fimath` and IEEE 754 with the other three reserved.

That obligation is an adequacy test rather than a preference: a convention
that cannot be written as a first-class alias over the primitives is a gap in
the primitives. So the question is which surface pair the two named
conventions are aliases over, and both of them answer it the same way and
neither answers it with integer width.

- **MATLAB `fi`** is parameterised by `WordLength` and `FractionLength`. That
  is total and fraction. `numerictype` carries no integer-length property; the
  integer part is derived.
- **IEEE 754** interchange formats are parameterised by `k`, the total width
  in bits, with `p` the precision and `w` the exponent width satisfying
  `k = w + p`. Total first again, and the standard's own conformance clause is
  stated over "encoded in exactly `k` bits".

Under the second option a consumer writes integer width and every alias has to
convert. The conversion is free in one direction and **not total in the
other**: `I = W - F - sign` is a natural for every declarable format, and
`W = I + F + sign` requires `I` to range over the negatives as soon as
`F > W - sign`, which is the entire fractional-only band that both standards
declare routinely. `fi(v, 1, 8, 12)` is an ordinary MATLAB declaration with
integer width `-5`. **So the second option's cost is not "every reflective
surface must choose which pair it shows". It is a signed ladder in a
coordinate that has no negative values in the object being described**, and
that is the cross-cutting residue Q9's own note says is "unresolved for every
arrangement under a design keyed on integer-and-fraction width and dissolved
only under the total-and-fraction keying".

The third option, both pairs with each surface declaring which it speaks, is
the one to argue against hardest because it looks free. It is not: it makes
the negative-integer-width corner permanent rather than dissolving it, since
the `(I, F)` surface still has to name `fi(v, 1, 8, 12)`.

*Confidence: this is mine, one instance. The MATLAB and IEEE parameterisations
are documentary facts a second reader should check rather than take from me;
`210_probes/p4_standards_alias_adequacy.rs` is an instrument already built
against both surfaces and is where a check would go.*

*Normative, no region.*

### Q3 `mixed_numeral_addition`

**Answer: it exists and the result numeral is derived. Option 2.**

The row's own `bound` states the test: both conventions op named carry an
addition across differing formats, so option 1 has to be reached with the
adequacy test satisfied or not reached at all. It cannot be.

**Option 1 is refuted outright.** MATLAB `fimath` defines `SumMode` over two
`fi` operands of differing `FractionLength` and IEEE 754 clause 5.4.1 defines
`formatOf` operations whose operands need not share a format. An arvo with no
addition taking values from two numerals cannot express either as an alias
unless the alias converts first, which is option 3 and not option 1.

**Option 3 is refuted too, and this is the part the register does not
carry.** Option 3 says "a consumer converts one operand into the other's
numeral first". MATLAB's `SumMode = 'FullPrecision'` does not go to either
operand's numeral. It goes to a numeral wider than both, computed from both,
and it is lossless. Converting into the narrower operand's numeral is lossy
and converting into the wider one's is not always sufficient, because the
result needs a carry bit neither operand has. **So the target of the
conversion is not either operand and option 3's own sentence is what refutes
it.** What is left is a derived result numeral, which is option 2.

**What the derived target is.** It is the join in the inclusion order,
extended by one position for the carry. That connects this row to Q8 and makes
Q8 load-bearing for it rather than the other way round: option 2 is only
writable if the join exists. Section 5 is where I measure whether it does, and
the answer is that it does inside one kind and does not always across kinds.

**The worry in the row's note dissolves.** The note records that unit two's
one unconditional result rests on this being open, because under option 2
addition aligns scales, alignment is a shift, and a shift is the coarsening
that kills multiplication. **Alignment to a join is a widening and widening
loses nothing.** The coarsening happens on the way back down to a declared
output width, which is a separate, declared act, and
`law::double_rounding_is_innocuous_at_an_intermediate_width_between_f_and_2f`
is about that act rather than about the join. So the result the note is
worried for survives, at the price of one sentence saying the narrowing is
where the loss is.

*Mine, one instance, resting on two documentary claims about MATLAB and IEEE
that a second reader should check.*

*Normative, no region.*

### Q9 `the_width_surface_crossing`

**Answer: the consumer writes a const generic. The structural natural, if one
exists at all, is a hidden projection. Option 6.**

The row flags itself as one of the three least certain moves in its pass and
names the attack: the ergonomics bar was stated about bare primitives at API
positions and may not reach a question about how a width literal becomes a
type-level natural. **The attack fails on the quote's own second clause.**

`obligation::a_primitive_for_every_position_a_bare_number_would_take` carries
op's words: "No bare usize other than in const generics for smoother and more
ergonomic api, and even there, only when truly painful otherwise." A width
literal in a numeral's type *is* a const generic position. So the bound does
not merely fail to forbid it, it names that position as the one place the bare
form belongs, and gives the reason: smoother and more ergonomic. **The
exception is not a grudging carve-out being tolerated; it is a statement about
where the bare form is better.**

The second clause bounds the exception to where the alternative is truly
painful otherwise. Option 2, a raw natural surface with the consumer spelling
the width as a hand-written binary digit tower, is the definition of painful
otherwise. Options 3 and 4, a shipped alias layer and a consumer-side
declaration macro, are two ways of hiding a tower, and both are answers to
pain the exception exists to avoid rather than reasons the exception does not
apply.

Between the remaining routes, `ruling::the_predicate_is_whatever_is_available_at_const_time`
decides it: "the above collapses to whatever is available at const time". An
arm's predicate is a const expression, and a const expression over a
type-level natural has to cross back out of the type level to be one. Option 5
keys the numeral on naturals with the bridge firing once at the alias, which
puts the algebra on the far side of that crossing from its own predicates.
Option 6 puts the literal in the type and demotes the natural to a projection
used where a trait needs one, which is the side the predicates are already on.

**What I cannot settle, and it is the real wall.** Two retirement rows,
`retirement::dl_width_arithmetic_as_a_const_generic` and
`retirement::dl_const_generic_width_comparison_in_a_where_clause`, retire the
two constructions option 6 needs most: arithmetic on const generic widths and
comparison of them in a where clause. I have not compiled anything against the
pinned nightly to see what is available now, and the row says every route was
compiled and is a real arm, so somebody has. **So the answer above is a
statement about which route the stated bar selects, and it is conditional on
the const surface being able to carry width arithmetic at all.** If it cannot,
option 7, declaring the output width and checking it is wide enough, is the
route that needs no width arithmetic, and it is the one to reach for next.
That is a compile question and it is cheap; I did not run it because the
retirement rows say it has been run and I could not read what they were run
against without opening files I may not open.

*Mine, one instance, conditional as stated.*

### `does_precision_count_the_sign_digit`

**Answer: it does not count it.**

The row's `bound` states the criterion itself: "take the reading under which
the three sign domains form a structure a const predicate can gate on rather
than one leaving a domain incomparable". Nothing had computed it.
`221_probes/p3_does_precision_count_the_sign_digit.rs` computes it, over
radix in `{2, 3, 4, 10}` and precision `1..=5`, on inclusion of denotations.

```
READING A  precision COUNTS the sign digit    chain at  0 of 20 points, broken at 20
    first break: r=2 P=1: NonNegative(0, 1) vs Asymmetric(-1, 0) are incomparable
READING B  precision does NOT count it        chain at 20 of 20 points, broken at  0
    no break
```

Not close. Under the reading that counts the sign digit the non-negative
domain is incomparable with both signed domains at every point measured,
because it spends its whole precision on magnitude while they spend one digit
of it on a sign, so one reaches further up and the other reaches below zero
and neither contains the other. Under the reading that does not count it the
three nest exactly: non-negative inside symmetric inside asymmetric, at every
radix and every precision.

A chain is what a const predicate can gate on. Three mutually incomparable
regions is three arms with no order between them, and
`ruling::arms_over_regions_are_the_fundamental_heart` wants arms over regions
that compose.

**The two consequences the row's `unblocks` names, measured in the same run.**

- **The symmetric domain at precision one denotes exactly `{0}` under reading
  A**, which is the zero-width numeral the row asks about. It is a real point
  of the space either way, and section 5 shows the degenerate points are
  load-bearing, so this is not an argument against reading A. It is only an
  observation that reading A puts a degenerate point at a precision a consumer
  would write by accident.
- **The odd-radix collapse does not happen.** At radix 3 a balanced point at
  three digits denotes `(-13, 13)`, the symmetric point denotes `(-26, 26)`
  and the asymmetric one `(-27, 26)`. The balanced point is a strict subset of
  both and equal to neither, so nothing collapses. What is true is weaker and
  more useful: **a balanced point carries no sign digit at all, so at an odd
  radix the reading question is vacuous for it.** There is nothing for
  precision to count.

**The repair the row asks for comes with it.** The row says the two
disagreeing rows of the record's own family table are repaired in the same
act, "because preserving the disagreement is what made this a question". The
row to keep is the one that does not count the sign digit.

*Predicate, and the first two entries are where a second reader should push.*

```
holds for: radix: radix in {2, 3, 4, 10}: swept
           precision: P in 1..=5 radix-r digits: swept
           signedness: signedness in {unsigned, signed}: exhaustive, the three
             sign domains are the whole of the sign axis the record's family
             table names
           fraction_width: 0
           ambient_domain: the integers
           threads: 1
           toolchain: rustc = nightly-2026-05-28, edition = 2021
           build_profile: opt level = 3
```

**`operation` is absent on purpose and the severity is intended**: this is a
claim about representable sets before anything computes on them, so it holds
in no situation where an operation exists, and lifting it there is a separate
job. **`precision` is not a declared `dimension` row and I have written it
anyway**, which is a violation I am reporting rather than hiding: the corpus
has `total_width` and `integer_width`, both declared in bits, and there is no
radix-general digit-count axis. Section 8 carries it.

### `is_the_cross_kind_join_closed_or_priced`

**Answer: priced, and the row is asking about the wrong operation.**

The row's two options are to close the shape space under intersection at the
cost of a third family of segmented numerals, or to leave it unclosed and
priced. Section 5 measures which operation the kind boundary actually breaks,
and it is the meet: 47 of the 49 pairs with no unique greatest lower bound
have one point of each kind, while 55 of the 78 pairs with no unique least
upper bound have both points in one kind. **Intersection is the meet, so
option 1 is aimed at the right operation and the row's title is not.**

On the substance, closure is refused by ratified canon rather than by cost.
`proposal::membership_of_the_representable_set_is_one_affine_predicate` is
ratified and says membership is **one** affine predicate over **one**
parameterisation, of which the named kinds are **points**. The intersection of
a constant-quantum point and an exponential-quantum point has a quantum that
is the pointwise maximum of a constant and an exponential, which is neither,
and is not an affine slot function of the index. So the closure produces sets
whose membership is not one affine predicate, and admitting them is not
extending the inventory, it is amending the concept. **`ruling::the_format_spine_is_canon`
ratifies the concept as closed.**

**This is a reading of ratified text and I mark it as one.** The reading turns
on whether "a quantum per magnitude" means an arbitrary function of magnitude
or a parameter of the affine slot function. If it means an arbitrary function,
the space is closed under intersection already and this question dissolves in
the other direction. The reading I take is that if the quantum were arbitrary,
the sentence naming integers, fixed point, scaled integers and floats as
"points" of the parameterisation would be vacuous, since every set would be
one. **A second reader should attack that sentence rather than my conclusion**,
and if it goes the other way, then 125 of the 1081 intersections my probe
found outside its catalogue are inside the concept after all and the price is
zero.

*Mine, one instance, resting on a reading of a ratified sentence.*

### `is_the_derived_numeral_required_to_be_tightest`

**Answer: the canon states soundness. Tightness is an arm with a predicate,
not a canon claim. Neither option as posed.**

The row's own note says the sum-of-widths product form is not tight and wastes
exactly one bit on a characterised minority of pairs, so a canon sentence
claiming tightness would be false as the design stands. That refutes option 2
as a canon-wide claim and it does not refute the tight form as an arm.

`ruling::there_is_no_exchange_rate_because_there_is_no_generalisation` is
ratified by op and settles the shape: "Even small wins are wins worth
pursuing, however small, and however small a set they apply to. This is the
patchwork approach where we don't even try to generalise, we operate on const
predicates that choose the most optimal path always." **One bit of declared
width, on a characterised region, in a library whose reason for existing is
exact widths, is a win with a predicate already attached to it.** The row's
note says the two admissions the tight form needs are measured to be disjoint
regions of one formula's codomain rather than two independent repairs, which
is the predicate written down.

So: the canon carries soundness, because it is true everywhere and is what a
consumer relies on. The tight form ships as an arm gated on the region where
it is computable, and it does not appear in a canon sentence, because a canon
sentence claiming it would be the universal
`ruling::there_is_no_universal_answer_take_the_win_and_gate_it` refuses. **The
question's third option, say nothing about tightness, loses the arm and is the
worst of the three.**

*Mine, one instance. The shape is forced by two ratified rulings; which region
the arm is gated on is the row's own measurement and I did not re-derive it.*

*Normative, no region.*

### Q26 `what_a_platform_width_type_is`

**Answer: storage, not format, and the ratified spine says so in the words the
option uses.**

`proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set`,
ratified: "The representable set is a constant of the type: a value set that
depends on other data is not a format but storage."

A platform-width type's value set depends on the compilation target, which is
other data. So it is storage. The row weights option 1 as "named once, in
passing, in a file about a different topic", which is right about where the
option came from and wrong about its standing now: the sentence that decides
it is a ratified clause of the spine, and the option's own phrasing is a
transcription of that clause.

**Option 2, a degenerate instance of the shape family, is refuted by the same
clause** rather than merely outranked. A degenerate instance would still have
to have a representable set that is a constant of the type, and this one does
not.

*Confirmation of an option already on the page, from ratified text. One
instance, and cheap.*

## 5. One family or several, measured

`question::one_numeral_family_or_several` is deferred twice by op, at
`ruling::the_family_question_wants_the_comparison_first` and
`ruling::his_instinct_on_one_family_is_not_to_be_acted_on`, both at `open`.
Under `ruling::the_panel_finishes_the_canon_without_him` they are the panel's.
His instinct is one family and he said explicitly not to act on it, so it is
not in what follows.

`221_probes/p2_joins_exist_meets_do_not.rs` computes the inclusion order on
denotations over a catalogue of 47 denotationally distinct points: 38
constant-quantum, which is integers and fixed point and scaled integers, and 9
exponential-quantum, which is floats with subnormals. Five controls, all
fired, and one of them is the arm that makes the whole thing mean something.

### The first run voided, and the failure was the finding

C1 required the meet to be unique inside the constant-quantum family alone. It
failed, at 256 of 630 pairs, and the run is committed as
`p2_v1_c1_failed_meets_inside_the_constant_family.out`.
`p2d_diagnose_c1.rs` split the 256 and **every one of them has an empty
intersection and zero of them have several maximal lower bounds**. The
catalogue was filtering out the degenerate points, so the order had no bottom
and two grids at disagreeing phases had nowhere to meet.

**That is `question::inclusion_order_singleton_amendment` answered by the
algebra rather than by taste.** The row's third option is to leave the
predicate alone on the argument that a numeral carrying fewer than two values
is not a case any consumer reaches. **Refuted:** without the degenerate
points, 256 of 630 pairs inside one kind have no meet at all, so meet is not a
total operation and the constant-quantum family is not a meet-semilattice.
C5 in the repaired probe measures exactly this and reports 256 as required.
**A consumer never writing a zero-value numeral is beside the point; the order
needs the bottom whether or not anybody declares it.**

The row's first two options are then the live ones, and the same run decides
between them. The repair is to admit the degenerate points as points of the
parameterisation, which is a statement about what the shape space contains.
The row's first option amends the order's predicate to decide inclusion on
denotation rather than on declaration, which is a statement about what the
order is. **The probe computes the order on denotations throughout and
deduplicates the catalogue by denotation, 47 distinct points out of more
declarations than that, and the deduplication is what makes C1 pass.** So
option 1 is what the instrument had to do to get an answer at all, and I take
it: **decide inclusion on denotation.** The row's own note says the instrument
that first got it wrong held exactly one numeral carrying fewer than two
values, so the predicate was never offered the case that breaks it. Mine held
none until it was told, and broke.

### The measurement

```
MEASUREMENT over all 1081 unordered pairs
  join    unique least upper bound      1003
  join    several minimal upper bounds    78
  meet    unique greatest lower bound   1032
  meet    several maximal lower bounds    49
  meet    no lower bound at all            0

  of the 78 pairs with no unique JOIN:      of the 49 with no unique MEET:
    both points of one kind    55             both points of one kind     2
    one of each kind           23             one of each kind           47
    phases agree               36             phases agree               49
    phases differ              42             phases differ               0
```

And the arm that says what those numbers are about:

```
  the same constant-quantum pairs are measured twice, and the only thing that
  changes is whether the exponential-quantum points are in the space at all.
  every pair, bounds drawn from the whole space         pairs  1081  join fails   78  meet fails   49
  constant pairs, bounds from the whole space           pairs   703  join fails   55  meet fails    0
  constant pairs, bounds from the constant space only   pairs   703  join fails    0  meet fails    0
```

### What that says, and it is not what the question expects

**The constant-quantum family on its own is a lattice.** 703 pairs, unique
join and unique meet on every one.

**Putting the exponential-quantum points into the same space breaks the join
on 55 pairs that are both fixed-point.** Those pairs did not change. Their
bounds did: a float point can sit between two fixed points and be incomparable
with the fixed point that used to be the join, so a pair with one least upper
bound acquires two minimal ones. **The cost of one family is not paid at the
family boundary. It is paid inside the kind that was already fine.**

**The meet failure is where the kind boundary is.** 47 of 49, and all 49 with
phases agreeing, so phase is not doing it. A constant point and an exponential
point have maximal lower bounds that are one of each kind and incomparable,
because the largest common sub-grid is segmented and no point of either kind
denotes it. 125 of the 1081 intersections fall outside the catalogue entirely.

So the answer to Q8 is not one of its five options as written, and the useful
thing I can say is a correction to the question:

- **Within one kind, one family.** Joins and meets both exist and are unique,
  over the whole catalogue, once the degenerate points are admitted. Nothing
  needs a tie-break and nothing needs closure.
- **Across kinds, the meet is what fails**, and it fails because the
  intersection is segmented. That is `is_the_cross_kind_join_closed_or_priced`, and closure is what would fix it,
  and the ratified affine-predicate clause is what refuses closure.
- **The join failure is a property of the space rather than of the pair**, so
  it is not evidence for several families. It is evidence that a space
  containing both kinds needs a stated rule for picking among minimal upper
  bounds, which is Q8's third option, **and that the rule is needed for pairs
  inside one kind and not only across kinds.** That is the part nobody has
  written down and it is the most useful thing in this section.
- **Q8's fourth option, that two numerals are in one family exactly when their
  step sets are nested, does not survive.** Nesting of step sets is what
  inclusion already is on these points, so the criterion is the order rather
  than a partition of it, and it puts every pair in the catalogue that is
  comparable into one family and says nothing about the incomparable ones,
  which is all of the interesting ones.

**Predicate**, and the absences are deliberate.

```
holds for: radix: 2
           fraction_width: F in 0..=4: swept, steps 2^0 down to 2^-4
           signedness: signed: construction, every point in the catalogue is
             symmetric about zero, so no unsigned point was measured
           ambient_domain: the rationals
           shape kinds: constant quantum and exponential quantum: exhaustive,
             all 1081 unordered pairs of the 47 denotationally distinct points
           threads: 1
           toolchain: rustc = nightly-2026-05-28, edition = 2021
           build_profile: opt level = 3
```

**`operation`, `container`, `overflow_policy`, `rounding` and `strategy` are
all absent and I mean the severity.** This is a claim about an order on
representable sets and nothing computes anywhere in it. To use it about actual
arvo types somebody has to establish that the order on declared representable
sets is invariant under the container, and
`proposal::a_law_verdict_is_invariant_under_change_of_encoding_and_container`
is the row that would do it. It is at one expert.

**`signedness: signed` carries a `construction` token and therefore owes an
instrument that varied the axis and found no movement**, per
`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`. **I have not
run one, so read that entry as unmarked and the claim as holding at signed
only.** The honest repair is a second catalogue of non-negative points, and it
is cheap; I ran out of budget before it and it is the first thing on the list
in section 9.

## 6. The number system, the container premise, and where I am only confirming

`the_number_system` has eleven rows in my set beyond Q20, and every one of
them has a one-expert proposal answering it that I read before deriving. So on
these I am a second reader of a page rather than a second instance, and saying
otherwise would raise a standing on nothing. **What follows is what I checked
and what I would attack, marked as confirmation throughout.**

**Q19 `are_the_level_hierarchies_the_same_cut`.** Confirming
`proposal::the_concept_commits_to_its_choices_and_to_no_count_of_levels`:
neither cut refines the other, so neither is the partition and a count of
levels would be a count of one of them. **The independent leg I can add is
that this is the same shape as I1's refusal to presume a strategy count**,
which the row's own note observes without citing, and
`ruling::the_strategy_set_is_not_closed_at_four` is that refusal in the
registry. Two topics reaching the same refusal without citing each other is
worth more than either alone.

**Q21 `is_number_system_broad_enough_for_non_magnitude`.** Confirming
`proposal::the_concepts_edge_is_not_an_order_and_wrapping_is_the_test`:
broad. **An independent leg that does not come from that row:** the canon
already carries wrapping as a first-class member in two places. `dimension::overflow_policy`
declares `wrap` as a value of a declared axis, and
`proposal::arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`,
ratified, makes the adaptation a named total object with its own laws, of
which wrapping is one. A narrow reading that excludes non-magnitude structures
excludes a member the ratified canon already carries, and it does so through a
discriminator the corpus measured empty.

**Q23 `is_the_role_set_closed`.** Not settled by me and I would leave it open.
The row leans closed at storage, compute and interchange with chain-extent as
a possible fourth. **The reason to hold it open is structural rather than
cautious:** `ruling::the_strategy_set_is_not_closed_at_four` is op's, the
level count is refused by Q19's answer, and the inventory is open by Q20's.
**Three counts refused in one concept is a pattern, and a fourth closed count
sitting among them wants a reason rather than a default.** I do not have one.

**Q27 `is_interoperation_conversion_or_resolution`.** Confirming
`proposal::conversion_and_resolution_are_one_obligation_at_two_arities`, which
answers "neither" by making the fork a difference of arity. **The leg I can
add is Q3**: the derived result numeral of a mixed addition is the join, and
the join of three operands is the join of the joins, so the two-operand and
n-operand cases genuinely are one obligation and the arity is the only thing
that moves. That is the row's claim reached from the format side.

**Q29 `what_the_admission_contract_asks_a_candidate_to_expose`.** Confirming
option 2, `proposal::a_system_exposes_its_ambient_laws_its_set_and_its_reductions_verdicts`,
whose refutation of option 1 is carried as a measured row with its own control
and is the strongest evidence in the topic. Option 3, admission relative to a
consumer-supplied ambient domain, is the one I would keep alive: it makes the
collapse unstateable rather than caught, which is a real property and not
obviously worse than catching it. The row says what would distinguish them and
that the proposed distinguishing case was measured not to be one. **I have
nothing to add and would not close option 3.**

**Q30 `is_admission_a_predicate_or_a_location`.** Confirming
`proposal::admission_returns_a_coordinate_rather_than_a_verdict`. **The leg I
can add:** Q26 above is an instance of it. "What kind of thing is a
platform-width type" was posed as an admission question with four options and
the answer is a coordinate, storage rather than format, which is exactly the
shape the row predicts. That is one more instance of "every dispute about
admission dissolved into a dispute about an address", found in a different
topic.

**Q31 `one_word_or_two_for_is_a_number_system`.** Confirming
`proposal::membership_and_hosting_are_two_questions`, two words. The row's own
distinguisher is already satisfied and the row says so: the canon already says
something true of a system it cannot host, since the bounded windows it admits
are defined as bounded windows of systems it cannot host. **That is a closed
argument and I would promote it rather than re-derive it.** Between the second
and third options, the third scopes hosting to a target and composes with
target-indexed families; `dimension::toolchain` and `dimension::target_features`
are both declared axes, so the target index has somewhere to live and the
third option costs a quantifier the notation already carries. **I lean third
and it is a lean, not a finding.**

**Q33 `is_the_ambient_operation_family_fixed`.** Not settled and I think it is
the hardest row in the topic. Both options are priced in the row and the
distinguisher is whether any consumer needs one generic algorithm spanning
both families. **What I can add is that `dimension::ambient_domain` is a
declared axis whose own note says it is "the single largest blocker in the
corpus's unwritable predicates, sole blocker of four spans and present in
eleven".** A concept that fixes the operation family makes that axis a
constant, and eleven spans currently need it to vary. That is not proof that
it must be a parameter, because those spans may be about ambient domains
rather than about operation families, but it is the cheapest place to look and
nobody has looked there. **Concession: I could not settle this one.**

**Q35 `the_ownership_key_as_a_structural_axis`.** Not settled. The row's
distinguisher is whether any consumer writes one contract generic over
ownership. **`obligation::an_exact_width_container_a_consumer_can_alias_and_pin`
is a consumer statement about a value crossing a serialisation boundary where
the on-disk width is pinned by a format rather than chosen, which is a
shared-parameter case with a real owner**, so there is at least one instance
in the demand side. One instance does not decide a structural axis. **Concession.**

**Q36 `whose_reduction_governs_a_lossy_crossing`.** Not settled by me and I
will say why rather than pick. The row records the observable difference as
226 of 256 source values in the measured cell, so this is not a naming
question. `proposal::an_order_is_named_exactly_where_a_crossing_is_lossy`
carries a `gap` saying whose order it is remains a located disagreement with
three coherent positions, none forced by anything measured. **The one thing I
can rule out is the row's own framing that it is "explicitly op's", which
`ruling::the_panel_finishes_the_canon_without_him` retires.** It is the
panel's and it is unresolved. **Concession, and the thing that would move it
is the standards test again: MATLAB `fimath` attaches `OverflowAction` and
`RoundingMethod` to the fimath object rather than to either operand's
numerictype, which is the third option, a policy named at the site. That is
one convention and one instance, and I did not build the instrument.**

**Q37 `does_the_canon_name_crossing_classes`.** Confirming the row's own
distinguisher, which it says is already satisfied: at least two canon
sentences quantify over one class and not another, so the classes exist
whether or not they are named, and option 2 respells them everywhere. Between
naming five and naming two, **naming two cannot state the order problem and
the row says so**, so it is five. Confirmation.

**`when_is_an_order_owed_at_a_crossing`, the second row under key Q37.** Confirming
`proposal::an_order_is_named_exactly_where_a_crossing_is_lossy`, option 2. The
argument is closed in the row: composition of lossless crossings commutes with
the endpoints so nothing is owed, composition of lossy ones does not and the
two candidate orders give different answers. Option 1 is ceremony with no
content and option 3 is wrong at every lossy site under one of the two
readings. **The row's conservative test, asking whether the target's set
contains the source's, is the right one and it is decidable on the
representable sets**, which section 5's order is exactly the machinery for.

### The container premise, and why I am not a second instance on it

`question::the_container_premise` is answered by six rows, all of them seat
210's, and **seat 210 is this persona.** I read them before deriving. So
whatever I say about it is confirmation from the same head, and under the
two-expert rule it must not raise the standing.

What I can do without pretending otherwise is check the shape against the
canon, which I did. `proposal::observability_is_relative_to_a_declared_signature`
files the question as a theorem schema over the operation set with no truth
value until the signature is named, and `retirement::r210_the_container_premise_is_upstream_of_the_operation_set_question`
retires the sentence that made it upstream of the operation-set question.
Both are consistent with the ratified spine: a format is identified by ambient
domain and representable set, adaptation and encoding are realisation, and
what a signature may observe is a question about the operation set. **I find
no conflict with ratified canon, and that is the whole of what a second read
by the same persona is worth.**

**A genuinely independent second read is owed and it should not be Dolan.**

### Q4 `what_a_datum_stands_for`, and Q22 beside it

**Not settled, and I want to say why the option set is the problem rather than
the answer.** Op refused to bound this option set, so what is listed is what
has been written rather than what is admissible.

The live sub-fork the row names, soundness against bestness, is the same fork
as `is_the_derived_numeral_required_to_be_tightest` and I answered it there: the canon states soundness, bestness is an arm.
**That composes with all four readings, as the row says, so it does not decide
between them.**

What I can add is that option 4, sets admitted generally, and Q22, are the
same question asked in two topics.
`question::are_set_valued_carriers_admitted` asks whether set-valued carriers
are admitted into the number-system concept and Q4's fourth option asks
whether a datum may denote a set. **The costs the two rows list are disjoint
and neither cites the other**: Q4 prices it at the value-level total order,
multiplicative associativity and the additive inverse, and Q22 prices it at
making the carrier a set of sets. Both are true and a decision made from
either list alone is made on half the price. **That is a finding about the
register rather than an answer, and it is the useful thing I have on this
row.**

`law::existence_of_a_translation_invariant_total_order` is the row that makes
Q4's first listed cost checkable, and it is already about wrapping rather than
about intervals, so somebody would have to run it again over an interval
carrier. Cheap and not run. **Concession on both rows.**

## 7. Rounding and overflow

### Q6 `does_warm_wrap_or_clamp`

**Answer: the question dissolves, and not for the reason its third option
gives.**

`ruling::the_overflow_panic_is_permitted_and_bounded` is ratified by op and
says what stands in the panic's place on a release build: "the guarantee plus
an explicit declaration of the mode, saturate or wrap or whatever applies to
the overflow or underflow at hand, which then lowers and behaves accordingly."

**The mode is declared.** So a strategy does not define an overflow mode; at
most it states a default over an axis that is declared separately, and
`dimension::overflow_policy` is that axis, declared in the canon with `wrap`,
`saturate`, `clamp` and `panic` among its values.

Add `ruling::there_is_no_universal_answer_take_the_win_and_gate_it`, which
refuses a question asking which single policy governs a whole category, and
`ruling::wrap_or_clamp_stays_open_and_both_get_priced`, where op chose "explore
both and price both" over either answer and said the measurement decides. **Both
arms ship. The question of which one a preset names is a defaults question and
not an arithmetic one**, and `ruling::warms_objective_is_the_intuitive_best_choice`
decides defaults: convention is the baseline for intuition and is dropped where
it is consistently the worse choice.

**This is option 3's substance reached without Q5.** The row's third option
routes the dissolution through Q5's two-axis answer, which is open. The route
above does not need Q5: it needs I18, which is ratified, plus the declared
axis, plus the arms ruling. **So Q6 is answerable now and its own option set
makes it look blocked.**

**The row's second finding survives and matters more than the answer.** It
records that the committed wrapping bench family measures a different strategy
than the one it names under option 1, and that `20` section 1.5's clamp-family
arity crossover is self-flagged as contaminated. Neither is repaired by
dissolving the question. **A bench arm named for a strategy, on an axis the
strategy does not define, is misnamed whichever answer wins**, and the honest
repair is to name the bench arms by the overflow policy they implement, which
is what they vary.

*Mine, one instance, from three ratified rows.*

*Normative, no region.*

### `where_wrapping_lives`

**Answer: one slot, members classified by law role. Option 1, and ratified
canon says it.**

`proposal::arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`,
ratified through `ruling::the_format_spine_is_canon`: "Arithmetic on a format
is an exact operation in an ambient domain composed with a named, total
adaptation onto the representable set. The adaptation is a first-class object
with its own laws."

Wrapping is a total map onto the representable set. So is saturation. Both are
members of the slot the ratified sentence names, and neither is an exception
to anything. `proposal::the_adaptation_slot_is_derived_and_a_strategy_selects_a_member_per_operation`
carries the classification by two law roles and the observation that all four
combinations occur, and its own argument against expelling wrapping is that
there is no criterion that does not empty the slot.

**Option 2's cost is real and is why option 1 is not merely tidier.** The row
records that a permanent exception list costs both ways, because
chain-exactness sentences carry a saturation exception in the mirror
direction. Under option 1 there is no exception in either direction: a generic
sentence over the slot quantifies over the family that carries it.

**The apparent conflict with `dimension::overflow_policy` is not one.** That
row declares an axis of the predicate notation whose values name which member
of the slot is selected. An axis over a slot's members and a slot are
different objects and both statements are true. Worth saying because a reader
meeting both could reasonably think the canon had answered this question
twice, differently.

**And one thing in that dimension row does not survive the ratified spine.**
Its grammar lists `panic` among the named values seen in the corpus. **Panic is
not a total adaptation onto the representable set; it diverges.** So under the
ratified factoring it is not a member of the slot at all, and
`ruling::the_overflow_panic_is_permitted_and_bounded` agrees from the other
side: the panic is a debug-build behaviour and what stands in its place on
release is a declared mode. `dimension::build_profile`'s own `moves` field
reaches the same bound from a third direction. **So `overflow policy = panic`
is a predicate span naming something that is not an overflow policy**, and a
row carrying it is saying "the build was a debug build" in the wrong
vocabulary. Section 8.

*Confirmation of a one-expert row, plus one leg from ratified text that the
row does not use, plus one finding against a canon row.*

*Normative, no region.*

### `does_narrowing_compose`

**Answer: it composes for the directed modes and for toward-zero, and does not
for the round-to-nearest modes. Both arms ship and the canon states the
predicate rather than a preference. Neither option as posed.**

The row asks "does the design want narrowing to compose" and offers yes, which
constrains which rounding modes a numeral may carry, or no, which owes a canon
sentence saying narrowing twice is not narrowing once. **That is a question
about which single policy governs a category, which
`ruling::there_is_no_universal_answer_take_the_win_and_gate_it` refuses in op's
own words**, and `.claude/rules/never-ask-which-single-rule-governs.md` names
the shape.

The measurement is already in the registry and is stronger than either option.
`proposal::staged_narrowing_disagrees_with_direct_narrowing_under_round_to_nearest_even`
is at three or more instances and carries the characterisation: a mode
composes across nested grids exactly when its direction switches only at points
of the coarser grid, tested by moving a pivot on and off the coarse grid at
zero failures on-grid and seven off. Round to nearest switches at every cell
midpoint of the finer grid and none of those is a point of the coarser one.
The directed modes never switch direction; toward-zero switches only at zero,
which is a point of every anchored grid.

**So composition is a const predicate over `dimension::rounding`, and the
ratified vocabulary of `ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names`
is exactly the domain it ranges over.** Of the six ratified names,
`toward_zero`, `floor` and `ceil` compose; `half_up` and `half_even` do not;
`stochastic` is not a function and the question does not apply to it as
stated. **That is the canon sentence, and it is shorter than either option's.**

The row's option 2 says the canon owes the sentence that narrowing twice is
not narrowing once. It owes a narrower one: **narrowing twice is not narrowing
once at `rounding in {half_up, half_even}`**, which is a statement somebody can
gate on and the unqualified one is not.

*Mine as a derivation; the measurement under it is the corpus's and is at
three instances.*

*Predicate, transcribed from the row that established it rather than re-run:*

```
holds for: total_width: W = 9
           fraction_width: F = 4
           signedness: signedness = signed
           rounding: rounding = round to nearest even
           radix: radix = 2
           threads: threads = 1
```

**The general characterisation is not writable as a predicate** and the row
says why: its instrument is committed and no `probe` row names it, so the
general form has no registered evidence. **That is the wall on this row and it
is one probe row away from being writable.**

### `why_the_default_rounding_position_is_chosen`

**Answer: there is no single default. Option 3, which the row says neither
candidate wrote down.**

The row's `bound` argues the fork is decided by
`ruling::warms_objective_is_the_intuitive_best_choice`, and flags itself as
one of the three least certain moves in its pass, with the attack being that
the ruling is about one strategy's objective and reading it as a crate-wide
default is a widening. **The attack is right and the widening is not needed**,
because a second ratified ruling reaches the answer without it.

`ruling::there_is_no_exchange_rate_because_there_is_no_generalisation` is
ratified by op and its own words are "no attempt to generalise, we operate on
const predicates that choose the most optimal path always, from a million small
different impls for different situations". A single crate-wide rounding default
is a generalisation over a category. **The third option is what the ratified
arms intent produces and the row already says so.**

The row's other observation then becomes the useful part rather than the
argument: the IEEE default is the one mode that is not free under either
signedness, so a design defaulting to it has every relocation unavailable at
the default. **Under option 3 that is not a cost to accept, it is a predicate:
the concern that wants relocations carries a directed mode, the concern that
wants familiarity carries `half_even`, and neither is the crate's default
because the crate does not have one.**

*Mine, one instance, from one ratified ruling. It agrees with the row's
conclusion by a route the row's own attack does not reach.*

*Normative, no region.*

### `does_the_position_keyed_dither_arm_ship`

**Answer: it ships, and the reason is ratified rather than the toolbox
argument the coordinator used.**

The row records the coordinator answering in op's stead as derivable, with the
call marked overturnable and the reasoning being the toolbox rule. **The call
is right and the reasoning is weaker than what is available.**
`ruling::there_is_no_exchange_rate_because_there_is_no_generalisation`, op's
own words, ratified: "Even small wins are wins worth pursuing, however small,
and however small a set they apply to." An arm that wins in a region ships;
the size of the region is its predicate and not an argument against it.

**What the toolbox reasoning gets wrong is the direction of the burden.** It
argues that not knowing the consumer's access pattern is a reason to ship a
knob. Under the ratified intent, not knowing is not the reason at all: the arm
ships because a region exists where it is optimal, and
`ruling::there_is_no_universal_answer_take_the_win_and_gate_it` says whether
some consumer somewhere writes the declaration is not a precondition for
building the arm. **Same answer, and the ratified route survives an argument
about consumers that the toolbox route does not.**

*Confirmation of a coordinator call, from ratified text the call did not use.*

### `does_a_consumer_supplied_seed_surface_exist`

**Answer: it exists, and the ratified rounding vocabulary forces it.**

`ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names` is
ratified by op and fixes the vocabulary as `toward_zero`, `floor`, `ceil`,
`half_up`, `half_even`, `stochastic`. **`stochastic` is in the canon by op's
own stamp.**

A stochastic mode needs a source of randomness. Three places it can come from
and only three. Inside arvo, which is refused:
`ruling::never_a_runtime_check_and_one_lowered_path` is ratified and arvo has
no ambient entropy to reach for in the environments it targets. From the
value, which is value keying and is a different member, already named
separately in the register as one of the two keyings. **Or across the
boundary, from the consumer, which is a seed surface.** There is no fourth.

So the surface exists, and it exists because op ratified a vocabulary
containing a mode that cannot be realised without it. **The row's second
option, that it does not exist and determinism is picked for every consumer,
is available only by making `stochastic` a value-keyed mode and nothing else,
which is a narrowing of a ratified vocabulary rather than a design choice.**

**The row's own worry survives and is a different question.** It says
randomness cannot be sourced inside arvo so either it crosses the boundary or
the member is deterministic. Both, and they are two members rather than a
fork: the value-keyed member is deterministic and needs no seed, the
position-keyed and independent members need one. **That is `do_arvos_consumers_want_value_keying_or_position_keying`'s answer too.**

*Mine, one instance. This is the answer I am most confident in and the least
proud of, because it is one grep away and nobody ran it.*

*Normative, no region.*

### `do_arvos_consumers_want_value_keying_or_position_keying`

**Answer: both, gated. Option 3, and the row's own `bound` already says so.**

The row's `bound` field carries op's refusal verbatim and states that the third
option is not a compromise between the first two but the answer
`ruling::the_work_is_predicated_arms_composed` already gives, and that what the
panel owes is the predicate each keying is gated on rather than a winner.

**So the row is answered and what is open is the predicate.** I can name part
of it from Q46: **value keying is the deterministic member and needs no
boundary crossing; position keying and the independent member need a seed
surface.** That is a predicate over something a consumer can gate on, namely
whether a seed is available at all, and it is not one of the access-pattern
considerations the row's discussion is about.

**The rest of the predicate is unmeasured and that is the wall.** The
divergence between the members is measured; which regions each wins in is not,
and the monotonicity-rate row is the sweep that would say. **Concession on the rest.**

### The three measurement rows

**Q57 `what_the_double_rounding_mechanism_is`. Concession, and I did not
attack it.** The row records three constructions tried and all three wrong,
committed rather than repaired away, with the clause first suspected measured
not to be at risk and staged-versus-direct narrowing shown to be what is.
`law::double_rounding_is_innocuous_at_an_intermediate_width_between_f_and_2f`
closes the half that says no intermediate width between `F` and `2F` helps at
any `F`. **What is left is a fourth construction and I did not have the budget
to build one after p2 and p3. Saying so is more useful than a fourth wrong
one.** The starting point is the three dead routes, which is why they are
committed.

**`does_the_rounding_variance_form_hold_at_a_second_fraction_width`.
Concession, and it is the cheapest open row in my set.** The row says the
source states the forms are algebraic in the fraction width on the strength of
one checked fraction, and that one sweep at a second fraction is cheap and
nobody has run it. **I could not run it without knowing what the variance forms
are, and they live in a numbered member file I may not open.** That is the
whole of the wall and it is a dispatch problem rather than a research one: a
seat that may read `132` runs this in an hour. **Until then the forms hold at
one fraction width and `fraction_width any` is not writable**, which under
`ruling::a_predicate_lists_only_what_holds` is not a hedge but the current
state of the claim.

**`does_the_position_keyed_members_monotonicity_failure_rate_differ_from_the_independent_members`.
Concession, same wall, same cost.** Two source files hold one count each,
neither measured the other member under the same construction, and the sweep
that would settle it needs one construction and one input shape held fixed
across both members. **The two members' definitions are in files I may not
open.** A seat that can read them builds this in an afternoon, and it should
build it in the same act as Q48 because it is the same harness.

## 8. Things the canon does not license, reported whether or not they are mine

**`dimension::overflow_policy` lists `panic` as a named value of the axis.**
Under `proposal::arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`,
ratified, an adaptation is total; panic diverges and is not one. Under
`ruling::the_overflow_panic_is_permitted_and_bounded`, ratified, the panic is a
debug-build behaviour and what replaces it on release is a declared mode. So a
predicate span reading `overflow policy = panic` is saying something about the
build profile in the overflow vocabulary, and `dimension::build_profile` is the
axis that says it. **The dimension set is append-only and a row may not be
deleted or renamed, so the repair is a note on the row rather than an edit to
the grammar.** I have not counted how many spans write it.

**`precision` is not a declared axis and the corpus needs one.**
`total_width` and `integer_width` are both declared in bits, and
`dimension::radix` is declared separately, so a claim at radix 10 has no
width axis it can honestly use. Q11 is such a claim and I wrote `precision`
anyway, in a file rather than a row, and flagged it. **Adding a dimension row
adds nothing backwards and nukes nothing**, per that file's own header, so this
is a cheap repair blocked only by the two-reading rule.

**The `answers` edge is maintained by hand and read by nothing.** Section 2.
Beyond the three ratified rows, twenty-eight more questions carry an edge that
renders as no edge. The cost is not hypothetical: it is a seat's whole budget
per row, and this seat spent part of its own on three of them.

**`question::is_the_cross_kind_join_closed_or_priced` is named for the join and
is about the meet.** Section 5's measurement. A row whose title names the wrong
operation gets cited by its title.

**`proposal::the_carrier_is_observable_through_the_ambient_layout_observation_alone`
writes `alignment: aligned` and its neighbour writes `alignment: aligned,
straddling`, and both note that the axis they want is occupancy, sole against
shared, which no dimension row declares.** Both rows say so themselves and
neither is hiding it. Named here because it is the second missing axis in this
report and two is a pattern: **the dimension set is being extended by whoever
trips over the gap, and nobody is extending it by reading the corpus.** The
file's own header says a census exists and ranks the undeclared keys by how
many spans each blocks. **Nobody has worked that ranking.**

## 9. What I would attack next, in order

1. **The unsigned half of p2.** One catalogue of non-negative points, same
   probe, same controls. It is the difference between `signedness: signed` and
   a claim about the order at all, and it is under an hour. Until it runs, my
   section 5 numbers hold at signed only.
2. **`every_answer_edge_is_read.rs` in `mock/checks/tests/`.** Named in
   section 2, blocked on the phase rather than on anything hard. Three arms.
   It converts a probe into a gate and it stops the class recurring.
3. **The two rounding measurement rows together**, by a seat that may read `132`. One harness, two
   rows, and both are described in their own rows as cheap and unrun.
4. **The compile question under Q9.** Whether width arithmetic and width
   comparison in a where clause are available on the pinned nightly today.
   Two retirement rows say they were not; the retirements are old enough that
   re-running them is cheaper than reasoning about them, and the answer
   decides between option 6 and option 7.
5. **The undeclared-axis ranking.** `dimension.toml` says the census exists and
   ranks the keys by how many spans each blocks. Working it would close
   `precision`, `occupancy`, and whatever else is above them, and it is the one
   piece of work here that unblocks other people's rows rather than mine.

## 10. What I did not answer, listed plainly

Nine of thirty-seven, and I would rather name them than pad the count.

`what_then_validate_requires` (Q1, which is settled under the shipped rule and
open under the brief's, per section 2, and is a filing question rather than a
research one), `what_a_datum_stands_for` (Q4),
`are_set_valued_carriers_admitted` (Q22), `is_the_role_set_closed` (Q23),
`is_the_ambient_operation_family_fixed` (Q33),
`the_ownership_key_as_a_structural_axis` (Q35),
`whose_reduction_governs_a_lossy_crossing` (Q36),
`what_the_double_rounding_mechanism_is` (Q57), and the two rounding
measurement rows, which are one wall counted twice.

`the_container_premise` is answered in the registry and I am not a second
instance on it, which is a tenth row I have deliberately not counted as mine.

The pattern in the nine is worth naming: **six of them are blocked on an
instrument nobody has built, and three of the six are blocked on a member file
a blind seat may not open.** That is a dispatch cost rather than a research
one, and the fix is to send one seat at the two rounding measurement rows and Q57 together, with
reading rights, rather than three blind seats at them separately.
