# 253. The exposure list is not the canon's to choose. It is fixed by the identity clause

Daan Leijen, seat 253, on
`question::what_the_admission_contract_asks_a_candidate_to_expose` (Q29),
`decider = panel`, `answered` empty, topic `the_number_system`.

The short form, before the argument for it. **None of the three recorded options
is the answer, and the question as posed presumes something the ratified canon
already settles.** An admission contract does not get to pick a list of things to
ask for. Whatever identifies the candidate must be exposed, or two candidates
cannot be told apart and admission has nothing to be over; and nothing that is
not part of that identity may be asked of the candidate, because a candidate is
the equivalence class the identity clause defines. The canon has ratified what
identity is. The list follows.

**A candidate exposes its ambient domain and a presentation of its representable
set, each carrying the obligation that makes it denote. It exposes no verdict, no
law inventory and no reduction.** Every verdict the three options ask for is
computed from those two, and computing it costs nothing at run time while asking
for it costs a declaration nothing can constrain.

That is one instance. Under
`ruling::two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`
a second independent reading is owed before anything here binds. Section 13's
last paragraph says what a second reader should attack first.

## 0. Gates

**Canon gate: aligned.** Checked against `mockspace.toml:31`, which declares
`canon_paths = ["mock/registry/*.toml"]`. The row I was sent on carries
`decider = "panel"` and no `answered`, and
`ruling::the_panel_finishes_the_canon_without_him` is `rung = "ratified"`,
`ratified_by = "op"`, and says every remaining canon question is the panel's. So
deriving an answer is the work the row calls for. I found no ratified text
forbidding it and no reservation covering it. **What the canon does reserve, I
stop at and say so**: the count and spelling of the ambient's coordinates, in
section 5.3, which is Q33's and which
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`
sets the precedent for leaving open.

**Test gate: run, and the suite is real.** `cargo test --workspace` from `mock/`
on my tree: 111 passed and 1 ignored, 8 passed, 13 passed and 1 ignored, 21
passed and 1 ignored, 10 passed, plus 4 doctests and 5 `compile_fail` doctests.
Nothing failing.

I read the bodies of the tests in the surface I touch rather than their names:
`crates/arvo-format/src/tests/obligations.rs` in full, and the slot and quantum
sections of `tests/the_inventory.rs`. **They are not decorative and I have
nothing to delete.** Every obligation is asserted against a construction that
compiles and is wrong (`obligations.rs:49`, `:63`, `:75`, `:84`, `:92`), every
one has a positive control that would catch a law stuck at `false`
(`the_law_admits_every_*_this_crate_ships`), and each contract carries a
separation test asserting the verdict differs between a shipped instance and a
wrong one, which is the one assertion a law stuck at either constant fails.
`the_law_rejects_a_step_law_that_runs_off_the_exponent` even carries an internal
control saying its construction passes the *other* condition, so the two
conditions are two.

**One test-gate observation travels with this and it is about placement rather
than quality.** `tests/the_inventory.rs:304`,
`the_widest_admitted_width_is_where_the_count_stops_fitting`, pins a bound
derived from `i64::MAX` as a property of the admitted set, and its own comment
says "If somebody widens the impl set without this being true, this fails." The
test is sound and it does what it says. Section 6 argues that the property it
pins does not belong in the admission contract at all, which is a finding about
the contract and not about the test.

## 1. My brief's premises, checked before anything else

Two claims in my brief are checkable in seconds and one of them is false.

**False: "`242` states explicitly that its reading of admission should not be
merged with `241`'s."** It does not.
`253_probes/the_briefs_premise/` greps `241`, `242` and `243` for every phrasing
that claim could take, with a positive control:

```
  merge                    241=0    242=0    243=0
  merged                   241=0    242=0    243=0
  merging                  241=0    242=0    243=0
  combine                  241=0    242=0    243=0
  combined                 241=0    242=0    243=0
  synthes                  241=0    242=0    243=0
  should not be read       241=0    242=0    243=0
  cannot be read as one    241=0    242=0    243=0
  must not be              241=0    242=0    243=0
  ---- positive control
  resolution               241=18   242=0    243=12
  tier                     241=13   242=29   243=17
  admission                241=44   242=23   243=14
  coordinate               241=67   242=36   243=21
```

**The opposite is nearer the truth.** `243` is seat 242's own attack file, and its
section 6 withdraws its Q22 discriminator in favour of `241`'s, calls `241`'s "a
strictly stronger argument for the same conclusion", and endorses `241`'s
coordinate finding outright.

**This is the third refutation of the same brief sentence.** `244` section 0
refuted it and `245` re-ran it independently with the same controls and got the
same zeros. It has now been handed to a fourth seat unchanged. I did not inherit
either refutation: my run is its own script with its own control set, and it
agrees. **The dispatcher should strike the sentence rather than let a fifth seat
spend budget on it.** What is true near it is `242` section 9's observation that
the admission subject does not close until the tier count does, which is a claim
about `question::are_the_level_hierarchies_the_same_cut` and about nothing else.

**True: the row is unanswered.** `cargo mock query
'question::what_the_admission_contract_asks_a_candidate_to_expose'` returns no
`answered` field and no `bound`, three options, `decider = panel`.

## 2. What governs, quoted from the rows rather than remembered

Four ratified sentences carry this whole subject. I quote them because the
argument below is an entailment from them and a paraphrase would hide where it
turns.

**`ruling::the_format_spine_is_canon`**, `rung = "ratified"`,
`ratified_by = "both"`, ratifies four propositions. Three of them matter here,
and I take the wording from the proposal rows rather than from the ruling's
summary, because that is where the load-bearing clauses live.

**I1, `proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set`:**

> A format is identified by its ambient domain and its representable set. The
> representable set is a constant of the type: a value set that depends on other
> data is not a format but storage. **Adaptation choice and encoding are
> realisation, observable in computed values and in pattern-level properties
> respectively, and not part of identity.**

**I2, `proposal::membership_of_the_representable_set_is_one_affine_predicate`:**

> Membership is one predicate over one parameterisation: an affine slot function,
> a quantum per magnitude and a phase, of which integers, fixed point, scaled
> integers and floats are points.

**I3, `proposal::arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`:**

> Arithmetic on a format is an exact operation in an ambient domain composed with
> a named, total adaptation onto the representable set. **The adaptation is a
> first-class object with its own laws.**

**I4, `proposal::the_concept_is_closed_and_the_inventory_is_open`:**

> The canon defines once what a number system is and what admission requires; the
> set of admitted instances is open, and **a new one earns admission by supplying
> the concept's obligations** rather than by amending the canon.

and its `because`, which is the sentence section 6 turns on:

> A closed inventory would put every new numeral through a canon amendment, and
> an open concept would make admission mean nothing. Closing the concept and
> opening the inventory is what makes admission a check rather than a
> negotiation.

## 3. The derivation, in four steps

The question asks what the contract *asks a candidate to expose*, and offers
three lists. I claim the list is not a choice.

**Step one. Admission is over candidates, and I4 says a candidate earns it by
supplying the concept's obligations.** So the contract's content is fixed by what
the concept is, and the concept is I1 to I3. The canon does not select a list; it
reads one off.

**Step two. I1 says identity is the pair (ambient domain, representable set). Both
halves must be supplied, or the candidate is not identified.** A candidate that
does not determine its own ambient does not determine its own identity, and a
thing that does not determine its identity is not one candidate but a family of
them. Admission over a family is admission over an object the concept has no
sentence about.

**Step three. I2 says the representable set is supplied as one predicate over one
parameterisation.** So the set is exposed intensionally, by fixing coordinates,
never by enumeration. That is what makes the inventory open in I4's sense: a new
instance joins by writing coordinates, and there is nothing to add to a list.

**Step four. I1 excludes adaptation choice and encoding from identity, by name, in
ratified text. I3 makes the adaptation a first-class object with its own laws.**
So a law verdict is a property of *the adaptation*, and the adaptation is not the
candidate. Two candidates differing only in adaptation are one candidate under I1.
There is therefore no place on a candidate to hang "its selected reduction" or
"that reduction's verdicts": the object being admitted does not have one.

**The list that comes out: the ambient domain, and a presentation of the
representable set. Nothing else.** Everything the three options add is either a
property of a different object or a fact derivable from these two.

### 3.1 I attacked this and it held, and here is the instrument

An entailment from ratified text is worth exactly as much as the completeness of
the reading behind it, so I built the instrument that would refute it:
**`253_probes/what_a_canon_sentence_reads_of_a_candidate/`.**

The refutation would be a governing sentence whose subject is a candidate and
which reads a reduction, an adaptation, a verdict, an encoding or a law
inventory. The census is exhaustive over the governing set, stated so the scope
is checkable: every `ruling` at `rung = "ratified"` (32) and every `proposal` at
`standing = "two_experts"` (23), fifty-five rows. The pattern is deliberately
broad, five words matched against the `says` field, and it returns eleven hits.

**Zero of eleven refutes.** The classification is in
`253_probes/what_a_canon_sentence_reads_of_a_candidate/classification.md`, row by
row over the complete list rather than a sample. Every hit has as its subject an
operation, a reduction, a law, an accumulator, or the concept itself. Three are
worth naming here:

- **`proposal::absorption_decides_associativity_of_a_clamped_reduction`** and
  **`ruling::the_additive_and_absorption_verdicts_are_canon`** are verdict
  sentences whose subject is *a reduction's induced operation*, not a candidate.
- **`proposal::inside_a_fragment_with_a_complete_test_set_the_verdict_is_computed_at_the_shipped_width`**,
  `standing = "two_experts"`, says in its own words that "a law's verdict is
  **computed** at the shipped width". A two-expert row already says verdicts are
  computed rather than declared.
- **I2** mentions the identity adaptation, and it does so as a *consequence* of
  the phase coordinate: "a nonzero phase decides whether the identity adaptation
  ever occurs". That is my claim in miniature. A coordinate is exposed and an
  adaptation fact is derived from it.

**The control fires.** A planted row carrying exactly the refuting sentence is
extracted and flagged by the same pipeline, so the zero is a fact about the
corpus rather than about the grep.

**Two defects in my first version of this instrument, disclosed rather than
quietly fixed, because both are the kind a census hides.**

- **It lost one row.** Paragraph-mode `awk` over a TOML file loses any row whose
  `quote` block contains a blank line, because the half carrying `rung` then
  carries no `id`. One ratified row went that way,
  `warms_objective_is_the_intuitive_best_choice`, and the census read 54 where
  the registry holds 55. **The count control caught it**, cross-checking against
  a plain `grep` and against `cargo mock query`, and the extractor is now a state
  machine over `[[table]]` headers. The lost row matches none of the five words,
  so the conclusion never moved. **The completeness claim did, and completeness
  is the whole of what this instrument is for.**
- **It matched on the wrong field**, emitting `id<TAB>says` and grepping the
  line, so one row was reported as a hit on its own slug. The pattern now matches
  `says` alone and the count falls from twelve to eleven with no genuine hit
  lost.

## 4. What each recorded option costs, and what each compiles to

The options are refused, and each for a reason that is not the one the row
records.

### 4.1 Option 1: the standing list prefixed with the reduction's two law verdicts

The row already records its sufficiency as refuted, twice over, by the
self-ambient construction. **That refutation is real and it is not the strongest
thing wrong with the option.** Two further ones:

**It asks the candidate for a property of a different object.** Step four. A
verdict is the adaptation's, and I1 puts the adaptation outside identity.

**And a verdict a candidate declares is a declaration nothing can constrain.** I
measured this rather than asserting it, in
`253_probes/declared_against_derived/`. One algebra, saturating addition on the
window `[-2, 1]`, which is not associative, with the first witness printed:
`(-2 + -2) + 1 = -1` against `-2 + (-2 + 1) = -2`. Two candidates over that one
algebra, differing only in the verdict they declare:

```
  Declared<true>  : ASSOCIATIVE = true  over saturating add on [-2, 1]
  Declared<false> : ASSOCIATIVE = false over saturating add on [-2, 1]
  both compile, and the algebra is the same type in both
```

**The case that must fail for an exposed verdict does not fail.** The field
admits both truth values over one algebra, and no arrangement of the trait makes
it not. That is `the-test-gate.md`'s "declarations nothing constrains" in its
purest form: ask what value would make it fail, and the answer is none. It is the
same defect seat 237 and seat 242 found at `PHASE_DEN`, one tier up, and the
crate has since fixed that one.

**The derived form does not have the defect and does not cost anything.** The same
probe computes the verdict from the algebra with a `const fn`, forced into a
`const` item so the run cannot be mistaken for a runtime computation. It answers
`false` on the saturating window and `true` on addition modulo four, so it is not
stuck at one answer, and it contradicts `Declared<true>`, which nothing in the
declared form can. The two cases that must fail for the derived form are each
behind their own feature so one cannot mask the other: pinning the lying
declaration against the derivation refuses at compile time with `E0080`, and
pinning the honest one builds.

**What it compiles to, since that is the part a design has to know.** A declared
verdict compiles to one bit on a trait, checked by nothing. A derived verdict
compiles to the same one bit, produced by const evaluation and checked by the
algebra. The declared form is not cheaper; it is the same cost with the guarantee
removed. Under `ruling::never_a_runtime_check_and_one_lowered_path`, ratified,
the derived form is the only one of the two that can be believed without a
runtime check, and there is nothing to trade.

**And a candidate-level bit cannot carry a verdict's predicate anyway.** The
registry's own verdicts are quantified over four to nine axes.
`law::rounding_retraction_is_the_identity` **holds** at `fraction_width: F = 0`
and **fails** at `F in 1..=W`, both at `operation = mul`, both under two rounding
modes. `law::coherence_of_a_reduction_onto_its_induced_operation` holds unsigned
under wrap and saturate and fails signed under saturate. A constant on a trait has
room for none of those indices, so an exposed verdict is not merely unchecked, it
is the wrong *shape* for the thing it names.

### 4.2 Option 2: the same, plus the ambient's law inventory, plus a retraction verdict

Everything in 4.1 applies, and two costs the option does not state.

**The added item is unconstrained by construction, and worse than the one it was
added to fix.** A candidate that declares both an ambient domain and that
ambient's law inventory has made two declarations of one fact, with nothing tying
them. The whole point of naming the ambient is that its laws follow from the
name; restating them creates exactly the drift the option was invented to catch.
The option's own diagnosis is right, that a verdict conjoined with nothing is
worth nothing, and its remedy adds a second unconstrained declaration beside the
first.

**And it asks for an object that is not finite.**
`proposal::a_law_is_inherited_where_the_realisation_map_is_a_congruence_for_every_nesting_it_contains`,
`sentence_kind = "theorem"`, `standing = "one_expert"`, says in its own words that
"the retraction table has one entry per ordered operation pair and is finite,
while **the space of identities is infinite at every arity**, so deciding the
finite table decides the infinite family". A law inventory is the infinite thing.
The finite thing that decides it is a table over the *named adaptation*, which is
derived from the identity and the adaptation and is not the candidate's to hand
over.

**And a law inventory is measured to carry strictly less than what it is a
function of.** `110` F3, a different topic and a different persona: a law set is a
function of the algebra, is not injective on it, and "40 algebras collapse to 7
law sets. So it carries strictly less information than the primitive and cannot
reconstruct it." Willsey measured that over 48 configurations. It is about
primitives rather than about candidates and it is the same relation: the
inventory is downstream of the thing, lossy, and therefore useless as an
independent declaration and redundant as a dependent one.

**The third verdict, for the retraction, is gesturing at the right object and
bolting it to the wrong list.** Retraction is a property of a reduction on a
format, and `law::rounding_retraction_is_the_identity` records it holding at
`F = 0` and failing above, per operation and per rounding mode. That is a
predicate over a region, and the option asks for it as a bit.

The row says the collapse option 2 buys against is "harmless rather than
forbidden". Section 7 says what I think the collapse actually is, and it is
neither.

### 4.3 Option 3: a consumer-supplied ambient, the candidate exposing its set and its reduction

**Option 3 has the list exactly inverted.** It drops the half I1 requires and
keeps the half I1 excludes.

- The ambient is **half of the identity** under I1, so a candidate that does not
  supply it does not determine what it is. The option states this cost in its own
  words, as "a candidate not determining its own identity until a frame is
  chosen", and files it as a price. Under a ratified identity clause it is not a
  price, it is the object failing to be a candidate.
- The reduction is the one thing I1 names as **not part of identity**. It is the
  single item in the option's two-item list that has no business there.

Seat 241 reached the first half of this from the same clause and did not reach the
second. **I do not count myself as a second instance for the first half**: I read
`241` before deriving, on the brief's instruction, so my agreement there is
inheritance rather than an arrival. The inversion reading is mine.

## 5. The positive answer, stated with what it does and does not fix

**The admission contract asks a candidate for two things.**

1. **Its ambient domain.** Half of identity under I1.
2. **A presentation of its representable set**, as a total assignment of the
   ratified parameterisation's coordinates, since I2 makes membership one
   predicate over one parameterisation.

**And it asks, of those two and of nothing else, that they denote.** That is not a
third item. It is a condition on the first two, and section 6 gives the criterion
for which conditions qualify.

**It asks for no verdict, no law inventory and no reduction.** Every one of those
is computed from (1) and (2) together with an adaptation the consumer or the
strategy layer selects, per
`proposal::the_adaptation_slot_is_derived_and_a_strategy_selects_a_member_per_operation`:
"Given the identity, the space of total reductions onto the representable set is
derived, not chosen; a strategy selects a member per operation."

### 5.1 What happens to the "necessary direction" the row says survives

The row's `note` says "The necessary direction of the first option is untouched
and survives." Under this answer it survives and **stops being a test**. If every
verdict is computed from the declaration, then every admitted system satisfies its
own computed verdicts by construction. That is a theorem about the derivation, not
a condition a candidate could fail. **It should be recorded as entailed rather
than as the surviving half of an admission test**, because a reader who keeps it
on a list of things to check will look for a check that cannot exist.

### 5.2 The shipped design already does this, which is buildability and not correctness

`arvo-format/src/adapt.rs:63` declares `DeclaredSignature` with two associated
types, `Format` and `Adaptation`, and `adapt.rs:72` names the pair
`Signature<F: Format, A: Adaptation>`. So the shipped surface pairs a format with
an adaptation to make a signature, and the format alone carries no adaptation.
`adapt.rs:121`'s `Operation` carries the rounding and overflow selection per
operation, which is the strategy selecting a member per operation.

**Shipped code is presumed wrong where it conflicts with the canon and it is not
evidence that this answer is right.** What it is evidence of is that the answer is
buildable, which under
`ruling::a_thing_that_constrains_the_work_and_cannot_be_designed_away_is_canon`
is the bar an intent has to clear before it may be written down.

### 5.3 What this answer does not settle, and stops at deliberately

**How many coordinates the ambient domain takes, and how they are spelled.** The
answer says the ambient is exposed. Whether that is two constants, or a domain
with an operation family, is
`question::is_the_ambient_operation_family_fixed` (Q33), and I do not touch it.
**The precedent for stopping here is exact**:
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`
ratifies that the door carries the coordinate set and says of itself, in bold in
its own `says`, "**How many types that is, this ruling does not say**".

**And the dependency runs one way only.** Seat 241 argued that Q30 cannot be
answered without Q33. Q29 is not like that. The *content* of the exposure list is
fixed without Q33; what Q33 decides is whether anything in the shipped surface
can *supply* item 1. Section 8 measures that it cannot.

## 6. Which obligations belong in the contract, and the criterion is derivable

Seat 242 proposed three well-formedness obligations as a fourth option to Q29.
Seat 246 section 5.2 measured that `Slots::ADMITTED` is not one kind of condition
but two, three well-formedness against two hosting, and wrote "where the third
belongs is open". Seat 247 recorded the disagreement as its O3 and did not resolve
it. **I resolve it, and neither of the two framings is right.**

**They are not a fourth item and they are not a third question. They are
conditions on the two items already there, and the criterion for inclusion is
derivable from I4.**

**A condition belongs in the admission contract exactly when the ratified
sentences do not apply to the declaration without it.** `MAGNITUDES >= 1` and
`PHASE_DEN != 0` and `RADIX >= 2` qualify: at zero magnitudes there is no set for
I1 to identify the format by, and at a zero denominator the phase names no
position so I2's affine predicate has nothing to be a predicate about. I4's own
words are "supplying the concept's obligations": the obligation is the concept's,
stated once, and the candidate supplies what it demands. That is why these are
conditions rather than declarations.

**A hosting condition fails that test, and it fails a second one that is worse.**

`Slots::ADMITTED` at `slots.rs:220` asserts `Self::WIDTH.count() <= 62`, with the
stated reason "the count of slots is 2^width and 2^63 does not fit a signed 64-bit
integer". That is a fact about `Slot` being an `i64` in this crate. Put it in the
admission contract and I4 breaks:

- **The inventory stops being open in I4's sense.** A 63-bit declaration is not
  admitted, and admitting it later would mean changing what admission requires,
  which is I4's own definition of a canon amendment. So a new instance would join
  by amending the canon, which is the state I4 exists to prevent.
- **And the amendment would be forced from below.** The reason to change the bound
  would be arvo widening `Slot`, an implementation change. A canon whose
  obligations move when a carrier moves is not closed, and the ordering the whole
  chain rests on runs the other way.
- I4's `because` names the same thing directly: "A closed inventory would put
  every new numeral through a canon amendment [...] Closing the concept and
  opening the inventory is what makes admission a check rather than a
  negotiation."

**So: well-formedness conditions are the concept's and belong in the contract.
Hosting conditions are this implementation's and do not.** That is a split with a
criterion rather than a third word, and it answers 246's open placement question
without introducing one.

### 6.1 Measured, and the bound is not even the right bound

`253_probes/hosting_in_the_contract/` builds an outside candidate declaring a
width of 63 bits over the index window `[0, 255]`, and reports the five conditions
of the shipped contract one at a time so the cause is named rather than
summarised:

```
Wide63  (the candidate): MIN=0 MAX=255 WIDTH=63
  MIN <= MAX                         : true
  WIDTH >= 1                         : true
  WIDTH <= 62   (carrier capacity)   : false
  span < i64::MAX                    : true
  span < 2^WIDTH                     : true
  is_admissible                      : false
  conditions failed                  : 1

Narrow62 (the control): MIN=0 MAX=255 WIDTH=62
  ... conditions failed              : 0
```

**One condition of five, and it is the carrier one.** The arm that forces
`ADMITTED` is a separate compilation, because a codegen refusal aborts the build
and cannot share a binary with the arm that prints. It fails, as it must:

```
error[E0080]: evaluation panicked: declared width is wider than a slot index
carries; the count of slots is 2^width and 2^63 does not fit a signed 64-bit
integer
  ... evaluation of `<Wide63 as arvo_format::Slots>::ADMITTED` failed here
  ... in arvo-format/src/slots.rs:219
```

**And the control for that arm builds**, the same forcing on the 62-bit candidate,
exit 0. Without it the refusal would say nothing about which condition refused.

**Now the part I did not expect.** The bound's stated hazard is a count of
`2^width`. The count this crate computes is not that. `slots.rs:318`:

```rust
SlotCount::of(S::MAX.index() - S::MIN.index() + 1)
```

The probe prints it: `slot_count(Narrow62) = SlotCount(256)`. **Nothing in either
crate computes `2^WIDTH` into an `i64`.** I grepped every use of `WIDTH`,
`declared_slot_width` and `.count()` across `arvo-format` and `arvo-placement`
outside the tests: the only `2^WIDTH` anywhere is `1i128 << Self::WIDTH.count()`
at `slots.rs:234` and `:254`, which is in `i128` and is safe to a width of 126.

So the hosting condition in the admission contract is not merely misplaced. **It
is stated over the wrong quantity**: it bounds a coordinate on behalf of a count
that the fourth and fifth assertions already bound directly, and in doing so it
refuses a candidate whose actual count is 256. The honest form of the same
protection is the span bound, which is already there and which `Wide63` passes.

I state one limit on this. The crate's own `admit_widths!` macro at
`slots.rs:275` computes `1i64 << $w` for the impls it generates, so the bound is
correct **about this crate's own `Signed<N>` and `Unsigned<N>`**, whose ranges are
the full `2^N` window. It is wrong as a condition on an arbitrary implementor,
which is what an admission contract is.

## 7. The self-collapse, disposed of

Option 1's refutation is
`proposal::an_exposure_test_over_reduction_verdicts_alone_is_satisfied_by_a_system_that_computes_nothing`,
`sentence_kind = "measured"`, with a committed instrument and a control showing
the collapsed verdict cannot be made to fail. It is right, and under this answer
it stops being a problem about admission.

Seat 241's disposal was that `F` and `F'` are two formats by I1's identity clause,
so nothing collapses. Seat 244 section 1.5 answered that this holds at the concept
tier and fails at the shipped tier, where two ambients are not distinguishable.
**Both are correct and neither is the disposal, because both argue about whether
the two declarations are one object.**

**Under an exposure list that carries no verdicts, the collapse cannot be a lie.
It can only be a mis-addressed question.** A candidate declaring its own computed
algebra as its ambient has a closed ambient operation, so by
`proposal::a_closed_ambient_operation_makes_the_reduction_the_identity` its
reduction is the identity and its derived verdicts are trivially true, **and truly
so**. Nothing was asserted, so nothing was false. The consumer who reads
"associativity holds" as a statement about the rationals asked about one ambient
and read an answer about another.

**So the repair is on the verdict's side, not on the contract's, and this
workspace already has it.** A verdict names the ambient it is quantified over, in
the same way `ruling::a_predicate_lists_only_what_holds` and the arms discipline
of `ruling::the_work_is_predicated_arms_composed` require a finding to name its
region. The ambient is a region. `dimension::ambient_domain` exists and is
declared. A verdict without it is a finding without its predicate, and this panel
already refuses those.

**The cost of that reading, said plainly.** It moves work from the admission
contract to the law layer: every verdict sentence grows one axis. That is a real
cost and it is one the notation already pays for eight other axes, and the
alternative, option 2, pays it by adding an unconstrained declaration that would
have to be trusted.

### 7.1 A second instance for the refutation, and its region is empty

`253_probes/declared_against_derived/` seconds the *conclusion* of
`an_exposure_test_over_reduction_verdicts_alone...`, that an exposure test over
verdicts alone establishes nothing, by a different instrument on a different
object: a compiled Rust trait rather than a swept declaration model, and an
unconstrained declaration rather than a self-ambient one.

**It does not second the construction, and the two dimensions intersect nowhere.**
That row's predicate is `total_width: W = 4`, `fraction_width: F = 0`,
`signedness in {unsigned, signed}`, `overflow policy in {wrap, saturate}`,
`operation = add`, `arity = 3`. Mine has no width at all: a four-element carrier,
one operation, arity 3. **So we converge on an argument and not on a region**, and
under the intersection rule that is what it may be cited as and no more.

## 8. Two things the shipped surface cannot supply, measured

Both are outside the question and both bear on it, so they are reported rather
than filed.

### 8.1 One of the two ambient coordinates is not about the domain

`ambient.rs:70` declares `Ambient` with exactly two items, `RADIX` and `SIGNED`.
Seats 241 and 243 each measured that four genuinely different algebras over one
grid produce one identical assignment. I reach the same conclusion from a fourth
direction, by reading what each coordinate is about rather than by comparing
tuples:

- **`SIGNED` is about the domain.** `ambient.rs:79`'s own doc says so, and
  `UnsignedBinaryRationals` at `ambient.rs:189` is a genuinely different domain
  from `BinaryRationals`, the non-negative rationals against the rationals.
- **`RADIX` is not.** `ambient.rs:179` calls `BinaryRationals` "The rationals at
  radix two" and `ambient.rs:196` calls `DecimalRationals` "The rationals at radix
  ten". Both are the rationals. The exact operation in the rationals is the same
  in any base, and `RADIX` is read in exactly two functional places in the crate,
  its own obligation at `ambient.rs:176` and the running product in
  `cancelling_slot` at `format.rs:334`. Both are about the grid's geometry, which
  is the representable set, not the domain.

**So the shipped `Ambient` carries one bit about the domain and one notation
parameter of the set.** Item 1 of the exposure list cannot be supplied by it. That
is Q33's, and it is what makes Q33 load-bearing for whether this answer is
*implementable* rather than for whether it is *right*.

### 8.2 The exposed tuple is not a normal form for the identity it presents

Nobody has raised this map, though four seats have raised the same shape at four
other maps, which section 13 sets out. It is the finding I would most want
attacked.

I1 identifies a format by (ambient domain, representable set). The candidate
exposes a coordinate tuple, which *presents* the set. **Two different tuples can
present one set, so the exposure is strictly finer than the identity it carries,
and I1 does not say what to do about it**, because I1 locates the redundancy in
encoding and adaptation and this redundancy is inside the identity coordinates
themselves.

`253_probes/the_exposure_is_not_a_normal_form/` enumerates the denoted set of each
candidate from its own exposed coordinates, through the crate's own accessors,
as exact rationals over `i128` reduced to lowest terms, so two spellings of one
value compare equal:

```
PAIR 1: the quantum law spelled two ways
  ConstantStep          Quantum = Constant<0>     SLOPE=0
  IndexedAtOneMagnitude Quantum = Indexed<0, 1>   SLOPE=1
  tuples equal      : false
  denoted sets equal: true      (16 values, -8 to 7, both)

PAIR 2: the phase spelled two ways
  ConstantStep     PHASE=0/1
  PhaseZeroOverTwo PHASE=0/2
  tuples equal      : false
  denoted sets equal: true

PAIR 3 (control): Constant<0> against Constant<1>
  denoted sets equal: false     (-8..7 against -16..14)

PAIR 4 (the comparator's own control): a candidate against itself
  tuples equal      : true    denoted sets equal: true
```

**Two independent sources of non-injectivity, both inside the identity
coordinates, with a control that moves in both directions.** At one magnitude the
slope is reached by nothing, so it is free; and a zero phase has one value and
infinitely many spellings.

**A third observation from the same probe, and it is about a doc rather than a
design.** `format.rs:250`'s `contains` is documented as "the affine predicate,
evaluated" and its body is
`magnitude_in_range::<F::Quantum>(magnitude).and(slot_in_range::<F::Slots>(slot))`.
It reads the two index ranges and consults neither the exponent, nor the phase,
nor the radix. Measured over the union of two index rectangles, 84 points: it
agrees on 84 of 84 for the two candidates of PAIR 3, whose denoted sets provably
differ, and differs on 12 of 84 for a control with a different slot range. So it
is the range check over the predicate's *domain*, and the affine map is elsewhere.
That is a defensible design and the doc sentence overstates it.

## 9. Findings, each with its predicate

Per `ruling::a_predicate_lists_only_what_holds`. An axis listed with a value holds
only there; an absent axis holds nowhere. `dimension.toml` carries no axis able to
hold a claim about the canon or about a source tree, which seats 240, 242 and 246
each reported; my registry and source claims are predicated on the tree instead,
which is outside the declared grammar, and I say so rather than smuggle it. The
tree is `fb6c843f` throughout, plus this seat's own probe directory, which adds
files and changes no measured surface.

- **A slot range that is coherent as a declaration is refused by the shipped
  admission contract on a capacity bound of this crate's own carrier, and the
  bound names a quantity nothing computes.** Established by construction, one
  failing condition of five, with a building control one bit narrower and a
  building control on the forcing arm.
  `toolchain: rustc = 1.98.0-nightly (57d06900f 2026-05-27), edition = 2024`,
  `build_profile: debug, debug-assertions = on`, `threads: threads = 1`, tree at
  `fb6c843f`. Evidence: `253_probes/hosting_in_the_contract/`.

- **A law verdict a candidate declares admits both truth values over one algebra;
  the same verdict derived from that algebra admits one and separates two
  algebras.** Established by construction, four arms, with the lying pin refusing
  at `E0080` and the honest pin building.
  `ambient_domain: ambient domain in {saturating add on the four-element window
  [-2, 1], addition modulo four}`, `operation: operation = add`,
  `arity: arity = 3`,
  `toolchain: rustc = 1.98.0-nightly (57d06900f 2026-05-27), edition = 2024`,
  `build_profile: debug, debug-assertions = on`, `threads: threads = 1`.
  Evidence: `253_probes/declared_against_derived/`.

- **Two distinct coordinate tuples over one ambient type denote one representable
  set, by two independent mechanisms.** Established by construction over the
  denoted values as exact rationals, with a control pair whose sets differ and a
  self-comparison control.
  `radix: radix = 2`, `ambient_domain: ambient domain = the rationals`,
  `total_width: W = 4`, `signedness: signedness = signed`,
  `phase: phase in {0/1, 0/2}`,
  `toolchain: rustc = 1.98.0-nightly (57d06900f 2026-05-27), edition = 2024`,
  `build_profile: debug, debug-assertions = on`, `threads: threads = 1`.
  Evidence: `253_probes/the_exposure_is_not_a_normal_form/`.

- **The shipped membership function agrees on every point for two candidates whose
  denoted sets differ.** Same predicate as above, over 84 points of the union of
  two index rectangles, with a control differing on 12 of 84.
  Evidence: the same probe.

- **No governing sentence has a candidate as its subject while reading a
  reduction, an adaptation, a verdict, an encoding or a law inventory.** Registry
  claim, exhaustive over 55 rows, 32 ratified rulings and 23 two-expert
  proposals, with a planted-row control that is flagged. Tree at `fb6c843f`,
  `threads: threads = 1`. Evidence:
  `253_probes/what_a_canon_sentence_reads_of_a_candidate/`.

- **Neither `241` nor `242` nor `243` contains any phrasing of the claim that
  `242`'s reading must not be merged with `241`'s.** Absence claim over ten
  phrasings with four positive controls that all fire. Tree at `fb6c843f`,
  `threads: threads = 1`. Evidence: `253_probes/the_briefs_premise/`.

- **The four format contracts declare nine coordinates on this tree, not ten.**
  Registry-and-source claim, extracted from the trait bodies, with a
  planted-const control that moves the count. Tree at `fb6c843f`,
  `threads: threads = 1`. Evidence:
  `253_probes/the_coordinate_count_on_this_tree/`. Second instance; seat 247 is
  the first, by reading rather than by extraction.

- **`RADIX` is read in exactly two functional places in `arvo-format`, its own
  obligation and the grid's running product, and in neither as part of an ambient
  operation.** Source claim, tree at `fb6c843f`, `threads: threads = 1`.

Every one of these is a compile-time or registry fact, so `threads = 1` is the
correct and only region for all of them.

## 10. What I settled, what I moved, and what I could not

**Settled, as an entailment from ratified text with the refuting instrument
built and returning nothing:**

- The exposure list is not the canon's to choose. It is fixed by I1's identity
  clause and I4's obligation clause, and it is the ambient domain and a
  presentation of the representable set.
- All three recorded options are refused, and each for a reason the row does not
  record. Option 3's list is inverted rather than merely costly.
- No verdict, no law inventory and no reduction is asked of a candidate.
- The necessary direction of option 1 survives and is entailed rather than
  checkable, so it should not be recorded as a surviving test.

**Settled, and this is the part I would want a second reader to attack first:**

- The well-formedness obligations are conditions on the two items rather than a
  fourth item or a third question, and the criterion for admitting a condition is
  that the ratified sentences do not apply to the declaration without it. This
  answers `247`'s O3, which it left open, and `246`'s section 5.2, which
  explicitly declined to place its third kind.
- Hosting conditions are excluded by I4, because an obligation that moves when a
  carrier moves makes the inventory implementation-relative and forces a canon
  amendment from below.

**Moved rather than settled:**

- Whether this answer is *supplied* by anything today depends on Q33, because one
  of the two ambient coordinates is a notation parameter rather than a domain
  one. The content of the list does not.
- The self-collapse moves from the admission contract to the law layer's
  predicate discipline. That relocates the problem to somewhere with an existing
  mechanism and does not by itself write the sentence.

**Could not:**

- **Whether a normal form for the presentation exists in general.** I have two
  redundancies with witnesses and an obvious normalisation for each of them
  (reduce the phase to lowest terms; the slope is free at one magnitude). I did
  not establish that normalising those two is enough, and I can see at least one
  further collapse I did not measure: a radix of four at exponent one and a radix
  of two at exponent two give the same step. Whether that is a third redundancy or
  two formats under I1 depends on whether the radix is part of the ambient's
  identity, which is section 8.1's finding and is Q33's.
- **Whether any consumer needs the exposure to be a normal form.** Seat 242 named
  the same wall on a different question: the downstream real-workload reference is
  not in this clone and a consumer survey is a different dispatch. I did not clone
  it either.
- **Whether the derived-verdict shape is expressible for arvo's real verdicts.**
  My probe derives associativity over a four-element Cayley table at const time,
  which is a demonstration and not a claim about a fixed-point format at width 32.
  Seat 241 named the same limit from the other side and declined to attack it, for
  the same reason: `ruling::the_canon_does_not_police_what_shape_a_law_takes` puts
  the spelling outside the canon question. **I say plainly that I did not attack
  it**, and that a design round will have to, because "derived rather than
  declared" is only cheap if the derivation is const-evaluable at real widths.

## 11. Options I open, each with what would close it

**O1. The canon owes a sentence saying that identity is over the denoted pair and
the exposed tuple is a presentation of it.** Without it, two presentations of one
format are two formats to every consumer that compares exposures, and one format
to every consumer that compares sets, and the canon does not say which is right.
**Closer:** a second independent derivation of section 8.2's non-injectivity, plus
a decision on whether the canon states the presentation/denotation distinction or
requires a normal form. The two have different costs and both are real: stating
the distinction costs one sentence and leaves set equality to be decided per use;
requiring a normal form costs a normalisation nobody has shown exists in general.

**O2. Whether the hosting conditions currently in `Slots::ADMITTED` move, and
where to.** Section 6 establishes they do not belong in the admission contract.
It does not say where they go. The natural home is the hosting predicate of
`question::one_word_or_two_for_is_a_number_system`'s second word, and
`proposal::membership_and_hosting_are_two_questions` is the standing row for that
split. **Closer:** a second reading agreeing that I4 excludes them, and then a
design round with somewhere to put them. **This one has a cost if it is done
badly**: removing them from `ADMITTED` without a home means a 63-bit declaration
compiles and then overflows somewhere downstream, which is worse than refusing it.
The order is the home first, then the removal.

**O3. Whether `RADIX` belongs on `Ambient` at all.** Section 8.1 measures that it
is a notation parameter of the representable set and that both shipped ambients
carrying it are the same domain. **Closer:** Q33, which decides what an ambient
domain is made of. I do not open a proposal to move it, because moving a
coordinate between contracts is exactly the amendment I4 makes expensive, and
because the count and spelling are R3's reserved question.

**O4. Whether `contains`'s doc sentence should be narrowed.** It says "the affine
predicate, evaluated" and it evaluates the range check. **Closer:** somebody
deciding whether the design intends a value-membership function at all. If it
does, that function does not exist; if it does not, the doc is one clause too
strong. Small, and it is the kind of thing that becomes load-bearing when a
consumer writes a compatibility check against it.

## 12. What I carry forward unchanged, and from whom

Thirteen, counted.

1. **The ratified spine as the floor**, from `ruling::the_format_spine_is_canon`
   through `213`, unchanged and quoted from the proposal rows rather than the
   summary.
2. **`244`'s C1 reading, that admission's return type has no second arm**, from
   `244` section 1.3 on `243`'s `E0046` evidence. My answer is about what goes
   *in*, and it composes with a total map rather than a sum either way.
3. **`243`'s section 5 placement of R2's second clause as a canon-revision
   trigger rather than a branch of a function**, which I use in section 7 without
   restating.
4. **`243`'s measurement that four ambient algebras resolve to one assignment**,
   and `241`'s independently. Section 8.1 is a fourth route to the same place and
   I do not count it as a fresh instance of their measurement.
5. **`244`'s L1 table**, that five of six questions carry a standing answer row
   and this one carries two. Re-derived independently by `246` with a different
   instrument; I did not re-run it.
6. **`245`'s and `246`'s finding that neither cold seat seconds
   `proposal::the_concepts_edge_is_not_an_order_and_wrapping_is_the_test`.** Not
   my question and I did not re-run it.
7. **`246` section 5.2's census of `Slots::ADMITTED` into three well-formedness
   and two hosting conditions.** I did not re-run its classifier. My section 6
   supplies the criterion its section 9 left open and my section 6.1 measures one
   of its two hosting conditions independently.
8. **`246` section 5.1's finding that the shipped hosting predicate is
   implementation-indexed rather than target-indexed.** It is what makes section
   6's "this implementation's" the right word.
9. **`247`'s finding that the four contracts declare nine coordinates on the
   current tree while R3 speaks of ten.** Section 9 records my own extraction as a
   second instance.
10. **`242`'s three derived obligations**, `RADIX >= 2`, `MAGNITUDES >= 1`,
    `PHASE_DEN != 0`, which are shipped now. I reclassify them rather than
    re-derive them, and the reclassification is section 6.
11. **`245`'s and `244`'s refutation of the brief's merge premise.** I re-ran it
    with my own controls and agree; the carry-forward is that this is now the
    third refutation rather than the first.
12. **`110` F3's measurement that a law set is a lossy, non-injective function of
    the algebra**, 40 algebras collapsing to 7 law sets over 48 configurations,
    which section 4.2 uses against option 2's added item. Willsey's, not re-run.
13. **`161` L26's statement that a non-injective naming is dangerous because a
    name is an existence claim**, which is section 8.2's shape one tier over.
    Leroy's, carried as prior art rather than as an instance of my claim.

**And one thing I explicitly do not carry as an instance.** I read `241` before
deriving, on the brief's instruction, so wherever my conclusion agrees with
`241`'s, that agreement is inheritance and not an arrival. It applies to option
3's first half, to Q29's disposal of the collapse at the concept tier, and to the
general shape that admission returns a coordinate assignment. **A promotion of
`proposal::admission_returns_a_coordinate_rather_than_a_verdict` may not count me
as an instance.**

## 13. Coverage: what I read, and what I did not

**Read in full:** `241`, `242`, `243`. **Read by section, headings first then the
sections bearing on admission:** `244` (gates, 0, 1, 2 headings, 3, 4), `245`
(all but its paths section), `246` (0, 1, 5, 6, 7), `247` (6, 7 opening), `250`
(0, 1, 2 opening). **Not read:** `248` and `249` in full, whose subject is what
the `standing` field counts, which is a registry-semantics question rather than
this one; `250` sections 3 to 10, which are Q21 and Q22; `73` and `74`, which I
know only through the quotations in `244`, `245` and `246`, the same limitation
those files name against themselves.

**Read only as grep context, six to twenty-four lines around a match, which is
enough to classify a claim and not enough to carry one:** `08`, `110`, `155`,
`161`, `226`, `16` and `03`, all reached by the section 13 search and all cited
only for what that context says. **Not read at all: the other roughly 435 files
in this directory.**

**Read from the registry, by query and raw:**
`question::what_the_admission_contract_asks_a_candidate_to_expose` and its five
neighbours on the topic, every `ruling` row's `id`/`rung`/`topic`/`says` as a
table, and in full `the_format_spine_is_canon`,
`the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`,
`behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`,
`a_thing_that_constrains_the_work_and_cannot_be_designed_away_is_canon`,
`two_experts_converging_is_a_ratification_and_the_coordinator_holds_the_gate`,
`an_expert_asks_its_peers_before_it_asks_op`,
`the_panel_finishes_the_canon_without_him`,
`never_a_runtime_check_and_one_lowered_path`,
`the_predicate_is_whatever_is_available_at_const_time`; the four ratified format
proposals in full; `the_adaptation_slot_is_derived_and_a_strategy_selects_a_member_per_operation`,
`the_laws_of_a_format_are_derived_from_two_hypotheses_rather_than_enumerated_per_policy`,
`a_law_is_inherited_where_the_realisation_map_is_a_congruence_for_every_nesting_it_contains`,
`a_system_exposes_its_ambient_laws_its_set_and_its_reductions_verdicts`,
`an_exposure_test_over_reduction_verdicts_alone_is_satisfied_by_a_system_that_computes_nothing`,
`a_closed_ambient_operation_makes_the_reduction_the_identity`,
`membership_and_hosting_are_two_questions`,
`admission_returns_a_coordinate_rather_than_a_verdict`;
`law::rounding_retraction_is_the_identity` and
`law::coherence_of_a_reduction_onto_its_induced_operation`; the `id` list of
`obligation.toml` and of `dimension.toml`; and the `says` field of all 55 rows in
the census set.

**Read from source:** `arvo-format/src/{ambient,quantum,slots,format,adapt,width}.rs`
in the parts bearing on the contracts, `apply.rs` and `rounding.rs` and
`overflow.rs` by their public items only, `tests/obligations.rs` in full,
`tests/the_inventory.rs` by its slot and quantum sections.

**Not read:** `arvo-placement` and `arvo-strategy` beyond one grep;
`mock/DESIGN.md.tmpl` and the crate design templates, deliberately, because a
canon question is not settled from the design tier; the `archive/` canon
candidate, which is demoted and which `244`'s L4 already reports on; the
`retirement` namespace, which I did not open and which may bear on the
presentation/denotation vocabulary in O1; the `probe` namespace beyond one row.

**The search behind section 8.2, and it did not return nothing.** I grepped the
whole directory for `normal form`, `same set`, `denote the same`, `non-injective`
and `not injective`, excluding my own files, with a positive control at 64 files
for `representable set`. It returns thirty-odd files, and reading the context of
each hit outside my own topic gives this:

- **`161` L26**: "Naming is [...] dangerous exactly when it is non-injective [...]
  two names for one primitive is a missed merge." Same shape, one tier over, about
  primitives rather than about format coordinates. Leroy.
- **`08` section 5's table**: decimal cohorts and carry-save marked "not
  injective". That is the **datum map**, which I1 puts outside identity, and it is
  the map `08` is classifying. Knuth.
- **`110` F3**: a law set is "a function of the algebra [...] not injective [...]
  40 algebras collapse to 7 law sets. So it carries strictly less information than
  the primitive and cannot reconstruct it." Willsey, measured, and section 4.2
  now cites it.
- **`226` and `16`**: the map from width and strategy to carrier is not injective,
  1024 declarations behind 10 carriers. Lattner and Aaltonen.
- **`243` section 3**: four ambient algebras resolving to one assignment. Seat
  242.

**None of those is the map in section 8.2**, which is the format coordinate tuple
onto the denoted set. So the claim is new and the *shape* is not: this panel has
now measured a declaration-to-denotation map failing injectivity at five
different maps, by five personas, and nobody has said that out loud. Section 11's
O1 is the local repair; whether the recurrence is worth a sentence of its own is
somebody else's dispatch.

**A grep for a phrase is weak evidence of absence and I did not read all 442
files**, so section 8.2 is offered as new with that caveat. Evidence for this
paragraph: `253_probes/the_briefs_premise/output.txt`, second half.

**What a reader should distrust most in this file.** Section 6's criterion for
which conditions belong in the admission contract. It is an entailment from I4
that nobody has attacked, it is the thing the rest of section 6 hangs on, and it
was written by one reader in one sitting. The measurement under it, section 6.1,
is solid and separate; the criterion is the part that wants a second reading.
