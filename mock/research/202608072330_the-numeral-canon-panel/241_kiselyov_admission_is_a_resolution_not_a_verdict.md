# 241. Admission is a resolution, not a verdict

Seat 241. Cold open on the admission subject of `topic = "the_number_system"`.

## Status

Stages 1 and 2, each committed before anything under `mock/research/` was opened. The
staging is deliberate: the pre-read commits are the only checkable evidence that the
derivation was blind, and a stall takes everything uncommitted with it.

## 0. The brief's cheap claims, checked before anything else

Three claims in the dispatch are checkable in seconds and two of them are false. Neither
is fatal to the subject, but one of them changes what I am permitted to answer.

**The brief says the six are open questions. One of them is ratified canon.**
`question::is_the_number_system_inventory_open` (Q20) carries an `answered` field, and it
resolves upward: it points at `ruling::the_format_spine_is_canon`, which is
`rung = "ratified"`, `ratified_by = "both"`, and whose `ratifies` list contains
`proposal::the_concept_is_closed_and_the_inventory_is_open`. That proposal's own `topic`
is `the_number_system`, not `the_format`. So the inventory question is not open, has not
been open since that ruling landed, and was settled by a ratification carrying the lead
designer's stamp as well as expert convergence.

I do not answer Q20. Under the provenance ladder a ratified ruling is defended rather
than weighed, and re-deriving it would produce a second unratified opinion sitting beside
a stamped one, which is how a settled thing gets reopened by accretion. What I do below
instead is *use* it, because it is the strongest floor the subject has.

**The brief says the `bound` field frequently carries a constraint that has already
closed part of the question. Not for these.** Zero of the six carry a `bound` field at
all:

```
is_the_number_system_inventory_open                     bound=0 answered=1
is_admission_a_predicate_or_a_location                  bound=0 answered=0
is_number_system_broad_enough_for_non_magnitude         bound=0 answered=0
are_set_valued_carriers_admitted                        bound=0 answered=0
one_word_or_two_for_is_a_number_system                  bound=0 answered=0
what_the_admission_contract_asks_a_candidate_to_expose  bound=0 answered=0
```

The claim is true of the topic as a whole, where four rows carry one, and false of every
row assigned to me. The instruction to read the bound is good advice generally and had
nothing to find here.

**The sixth question is misnamed in the brief.** The brief calls it "the one asking what
the admission contract asks a candidate to supply". The row is
`what_the_admission_contract_asks_a_candidate_to_expose`, and it asks what a candidate is
asked to *expose*. The difference is not cosmetic and I take it as load-bearing below:
supply is what a candidate hands over, expose is what it makes visible about itself, and
the whole third option of that row turns on the distinction, since a consumer-supplied
ambient domain is a thing supplied *to* the candidate rather than exposed *by* it.

## 1. The floor, quoted rather than remembered

Four ratified sentences bear on this subject, and I take them as given.

**R1, `ruling::the_format_spine_is_canon`, ratified, `ratified_by = "both"`.** A format is
identified by its ambient domain and its representable set; that set is a constant of the
type; membership in it is one affine predicate over one parameterisation, of which
integers, fixed point, scaled integers and floats are points; arithmetic on a format is an
exact operation in the ambient domain composed with a named total adaptation onto the set;
and the concept is closed while the inventory of admitted instances is open.

**R2, `ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`,
ratified, `ratified_by = "experts"`.** In its own words: "the design ships an admission
rule rather than an operation list: an operation is admitted exactly when it is a function
of the declared signature, and where two realisations of one name disagree, the signature
is missing a coordinate." Its `promotion` field says of itself that "the admission rule is
the format spine's closed-concept-open-inventory shape one tier down, which is what makes
it a derivation rather than a new decision."

**R3, `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`,
ratified, `ratified_by = "experts"`.** What crosses the numeric boundary is "the coordinate
set of the ratified parameterisation, spelled in types the stack owns," and the
parameterisation has ten coordinates. How many types that is, it explicitly does not say.

**R4, `ruling::the_panel_finishes_the_canon_without_him`, ratified, `ratified_by = "op"`.**
Silence in his corpus is not permission; a question his words do not reach is derived from
the intent inside its spirit and put through two independent agreements.

## 2. The central derivation: admission already has a ratified shape, and it is neither of the two the questions argue over

The six questions read as six independent disputes. They are not. Five of them are one
question asked five times, and the question is: **what kind of object does admission
return?**

R2 answers it, for operations, in ratified text, and the answer is not in any of the
option lists my questions offer. Read the second clause again: *where two realisations of
one name disagree, the signature is missing a coordinate.* That sentence describes what
happens when admission **fails**, and the failure is not a rejection. It is a **demand for
a coordinate**. The candidate is not thrown out of the set; the signature is found to be
underdetermined, and the repair is to add a coordinate until it is not.

So the ratified failure mode of admission, one tier down, is not `false`. It is the *name
of the coordinate that was not fixed*.

That is a third shape, and it subsumes both shapes `is_admission_a_predicate_or_a_location`
offers:

- **Admission is a resolution.** Given a candidate, it returns either a total assignment
  of the ratified coordinate set, or the name of a coordinate the candidate failed to fix.
- A **predicate** is that object composed with "did it succeed". It is a projection, and
  it is lossy in exactly the way the question's first option complains about: it "discards
  the coordinate a consumer needs".
- A **location** is that object composed with "which coordinate", defined only on the
  success branch. It is the other projection, and it is lossy in exactly the way the
  question's second option complains about: it has nothing to say about the candidate that
  did not resolve.

Both options are folds over one structure. The canon should state the structure and let a
predicate and a location be two interpretations of it, because that is the only version
where the two cannot drift: each is derived from the single statement rather than written
twice. Stating either projection as the canon sentence forces the other to be re-derived
by every consumer that needs it, and re-derived differently.

**This is not an import of a foreign idea.** It is what R2 already ratified, read at its
own word, and R2's own `promotion` says the shape descends one tier at a time. My subject
is one tier up from R2's operations. The same shape descending again is a derivation, not a
new decision, and it arrives with R2's ratification behind it rather than needing its own.

### What that costs, said here rather than left to be found

It costs an output type where a truth value would have done, which is exactly the cost the
question's second option names, and it does not dodge that. What it buys back is that the
cost is paid once: the two projections are free afterwards, and the failing case carries
information instead of discarding it.

It also commits the canon to *there being a coordinate set to be underdetermined about*.
R3 ratifies that there is one and refuses to say how many types it is spelled in. The
resolution reading needs the set, not the type count, so it composes with R3's reservation
rather than reopening it.

### The second reading, which I do not think wins but which the evidence does not kill

A resolution is the right object *only if the coordinate set is fixed before admission
runs*. If the ambient domain is itself a coordinate that a candidate may choose freely, and
the operation family with it, then "the signature is missing a coordinate" is not a
diagnosis but an infinite regress: any disagreement between two realisations can be
absorbed by inventing a coordinate, and admission never fails. On that reading admission
must be a predicate over a fixed schema after all, because the predicate is the only shape
that can say no.

What would distinguish them: whether the coordinate set is closed. R3 says the
parameterisation has ten coordinates and the door carries them, which reads as closed, but
R3 is a ruling about a door rather than about the concept, and it explicitly reserves the
type count. `question::is_the_ambient_operation_family_fixed` (Q33) is the row that decides
it, and Q33 is not in my six. **I therefore cannot close the resolution reading, and I do
not.** What I claim is the weaker and still useful thing: if the coordinate set is closed,
the resolution shape is forced by R2 and both of Q30's options are projections of it; and
Q30 cannot be answered without Q33, so the brief's cut of the subject is wrong at that
seam.

## 3. The refused shape, and why four of the six dissolve rather than resolve

Stage 2.

Before answering the remaining five I have to say what kind of questions they are,
because it changes what an answer may look like.

`ruling::there_is_no_universal_answer_take_the_win_and_gate_it` is `rung = "stated"`
and carries his own words:

> Again, we don't need to settle for one universal solution, it's the anti-pattern I've
> already named. Case by case. [...] Take the win where it applies, gate it out from where
> it does not. No single one-fits-all solutions, it's impossible

Its own `note` records that this was the third time in one sitting a question of that shape
had been put to him.

Four of my six are that shape exactly. Broad or narrow. Admitted or scoped out. Predicate
or location. One word or two. Each asks which single policy governs a whole category.

And there is a ratified precedent for what happens to such a question, in this registry, on
this panel, one tier down.
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` is
`rung = "ratified"`, `ratified_by = "experts"`, and its `says` opens: "The container premise
is malformed as a binary and both of its branches are false." Its `note` is explicit about
why: "The question's own option list was the shape op has refused three times, a single
policy to govern a whole category. It is closed by dissolution rather than by picking a
letter, which is what he has said the answer to that shape always is."

So the honest move on a question of this shape is not to pick a letter and not to decline.
It is to find the coordinate the binary is collapsing, and state the regions.
`ruling::the_option_set_is_not_a_boundary` licenses the shape of the answer; the two above
require it.

I do that below for four of them. One of them, Q29, is not of that shape and gets a direct
answer. One of them, Q20, is ratified and gets none.

## 4. Q21, whether "number system" is broad enough for things that are not magnitude

**It is not a question about breadth. It is a question about whether the ambient's
operation family is a coordinate, and the shipped answer is that it is not.**

R1 ratifies that a format is identified by its ambient domain and its representable set,
and that "two formats with the same representable set under different ambient algebras are
two formats". That sentence is only meaningful if two different ambient algebras are
distinguishable. They are not.

`241_probes/ambient_algebra_is_not_a_coordinate` declares four ambient domains from a
genuinely separate crate: the rationals under plus and times, the tropical semiring under
min and plus, the two-element Boolean algebra under and and or, and the interval algebra
over the rationals. Every one of them is a different algebra, and three of the four are not
about magnitude at all. Every observation the crate offers returns the identical tuple for
all four:

```
rationals (+,*) : (2, true, -3, true, true, true, -128, 127, true)
tropical (min,+): (2, true, -3, true, true, true, -128, 127, true)
boolean (and,or): (2, true, -3, true, true, true, -128, 127, true)
interval algebra: (2, true, -3, true, true, true, -128, 127, true)
CONTROL radix 10: (10, true, -3, true, true, true, -128, 127, true)
```

The negative control moves, so the instrument sees a declared coordinate change. The four
arms do not, so the algebra is not one. `Ambient` carries `RADIX: u32` and `SIGNED: bool`
and nothing else; the algebra lives in the doc comment, where "the rationals at radix two"
is prose and not a coordinate.

**Under a ratified rule this is a diagnosis rather than an observation.** R2 says: "where
two realisations of one name disagree, the signature is missing a coordinate." Four
realisations of one ambient disagree about every law anybody would want a verdict on. The
signature is missing the operation-family coordinate, and R2 is the ruling that says so.

So Q21 dissolves in a specific direction, and I state the region rather than the letter:

- **On the concept as ratified, the answer is broad, by construction.** R1's identity pair
  is ⟨ambient domain, representable set⟩ and neither names an order, a magnitude or a
  metric. Nothing in the ratified text excludes the Boolean algebra or a vector space over
  the two-element field. The narrow reading requires an extra clause R1 does not have, and
  writing one in now would be an amendment rather than a reading.
- **On the concept as shipped, the answer is vacuously broad**, which is worse than broad.
  Everything is admitted because nothing is distinguished, so the admission carries no
  information and no law verdict about the ambient means anything.
- **The useful question underneath is Q33**, `is_the_ambient_operation_family_fixed`, which
  is not in my six, and Q21's own `note` already says the two entries should be read as
  one. My cut says the same thing from the other side, and adds the measurement: the two
  cannot be separated because the discriminator Q21 would need does not exist in the
  signature.

**The second reading, which I do not think wins.** One could hold that the operation family
belongs to the *adaptation* rather than to the ambient, since R1 makes the adaptation a
first-class object with its own laws, and that the ambient is deliberately thin because its
job is only to say what values are drawn from where. On that reading the probe measures the
division of labour rather than a gap. What would distinguish them: whether any law verdict
the canon wants is a verdict about the ambient's own operation rather than about the
adaptation. R1's own factoring clause says arithmetic is "an exact operation in the ambient
domain composed with a named total adaptation", so there is an exact operation *in the
ambient* and its laws are the ambient's, which is what the probe finds no coordinate for.
I take the first reading, and I say plainly that this is one instrument and one argument.

## 5. Q22, whether set-valued carriers are admitted

**Neither. The answer is regional and the region is exact.**

R1 ratifies that membership is one affine predicate over one parameterisation: a value is
in the set when it is `phase + slot * quantum(magnitude)`. One slot index. An interval is a
pair. So the question is arithmetic rather than taste, and
`241_probes/set_valued_carriers_need_a_second_slot` does the arithmetic on a six-bit signed
grid:

```
slots in the range                      : 64
slots the shipped predicate admits      : 64
arm A, fixed-radius intervals nameable  : 64
arm B, general intervals on the grid    : 2080
unnameable by one affine slot           : 2016
CONTROL, radius 0: points on the grid   : 64
CONTROL gap (must be 0)                 : 0
```

The control forces the radius to zero so intervals collapse to points, where one slot must
name every one of them; the gap is zero, so the counting is sound and arm B's gap is about
set-valuedness rather than about the instrument.

**Fixed-radius intervals are admitted by construction, and no amendment is needed.** The
centre is the one affine slot; the radius is a constant of the type, which is precisely what
R1's identity clause demands ("the representable set is a constant of the type"). The ambient
is the interval algebra, the exact operation is the Minkowski one, and the adaptation onto the
grid is outward. That is R1's factoring clause satisfied verbatim, with nothing added.

**General set-valued carriers are refused, and refused by the canon's own admission rule
rather than by a preference.** One slot names 64 of the 2080 intervals the grid carries.
Admitting the other 2016 requires the affine predicate to become vector-valued, which is an
amendment to a ratified clause, and the ratified clause
`proposal::the_concept_is_closed_and_the_inventory_is_open` says an instance earns admission
"by supplying the concept's obligations rather than by amending the canon". A candidate whose
admission requires an amendment is not admitted, by the definition of admission that is
already stamped.

**What I owe here, because arm A proves less than it looks.** Arm A compiles, but probe 1
shows the crate accepts *any* ambient, so the crate accepting an interval ambient is not
evidence that the crate knows it is an interval. The mathematical argument is what carries
arm A; the crate carries only arm B's count, which is over slots, which the crate does model.
I state that rather than letting the compile stand in for the argument.

**And the constructive third route, which costs the canon nothing.** A certified value is a
value and a bound. Those are two formats, and a pair of formats composed at the tier above is
not a set-valued format at all. Every use case Q22's `note` names, "intervals and any future
error-tracking pair", is reachable that way with no clause touched. So the question's own
framing, that admitting generalises the concept and scoping out keeps it smaller, offers a
choice between two costs where the cheap answer is neither: admit the one-parameter family
that already fits, refuse the amendment, and put the general case one tier up where pairs
live anyway.

**The reading that would beat this, and what closed it.** One could hold that "one
parameterisation" in R1 never meant one *scalar*, and that a vector slot was always inside
it, in which case arm B is admitted and the 2016 have coordinates after all. My refusal
rested entirely on the scalar reading, and a reading of a spelling is a weak thing to rest a
refusal on. I went looking for something better and found it in a ratified count.

R3 says the door carries out "the coordinate set of the ratified parameterisation", and its
own reasoning counts **ten** associated constants and partitions them: six declared in types
a `u32` bit count cannot hold, of which three ship values it cannot hold.
`241_probes/the_ratified_ten_fixes_the_slot_arity` reconstructs that count from the shipped
traits, under two controls, and it comes out exactly:

```
coordinates                    : 10
  declared i64 or i32          : 6
  the rest (u32, bool, Width)  : 4
control A passed: the ruling's six reproduces.
control B passed: the ratified ten reconstructs.
```

The ten are `RADIX`, `SIGNED`, `BASE`, `SLOPE`, `MAGNITUDES`, `MIN`, `MAX`, `WIDTH`,
`PHASE_NUM`, `PHASE_DEN`. `ADMITTED` is excluded and the exclusion is the point: it is a
`const ADMITTED: ()`, it carries no value, and it is what every use site forces rather than
something a candidate chooses. The shipped code has already separated the coordinates from
the admission obligation, which is the resolution shape section 2 derives, sitting in the
tree unnamed.

Three of those ten are the slot axis, so the coordinate count is `3n + 7` in the slot arity
`n`. At `n = 1` that is ten. At `n = 2` it is thirteen and at `n = 3` sixteen.

**So the arity is fixed at one by a ratified count, not by a reading of a spelling.** A
vector-valued affine predicate makes R3's ten wrong, and R3 is ratified by two blind
instances with committed probes. That is a much stronger floor than the sentence I was
leaning on, and it means the Q22 refusal does not depend on how anybody reads "one
parameterisation".

**What survives as genuinely open**: whether the panel *should* amend the parameterisation
to a vector slot, which is a different question from whether the current one admits arm B.
It does not. Amending it would take R3's count with it, and R3 is the row the numeric door
is built on, so the amendment is not local to the membership clause. I do not propose it,
and I note that the compositional route in the paragraph above reaches the same use cases
without touching a ratified row.

**The first version of that probe was wrong and it is kept, with the defect named in its
header.** Its declaration regex matched impl bodies as well as trait declarations, so it
counted 22 and its summary asserted that the ten reconstructed. It did not, and nothing in
the run said so, because the summary was written before the number arrived. The two controls
now stand between the count and the claim and the claim prints only if both hold. I record
this because a probe whose printed conclusion is not what its run measured is the exact
defect the test gate names, and I wrote one.
refusal is wrong.

## 6. Q31, one word or two

**Two, and the second is not a second admission procedure. It is a predicate over the first
one's output.**

Under the resolution reading, being a number system is *fixing the coordinates*. That is
target-free: whether a candidate determines an ambient, a quantum law, a slot range and a
phase is a fact about the candidate and about nothing else.

Being hostable is a predicate over the *values* those coordinates take, against a
particular target's realisation ladder. `Slots::ADMITTED` in the shipped crate is exactly
this and nothing else: a width of 63 bits is a perfectly good coordinate assignment that
this stack cannot carry, because a slot count of `2^63` does not fit the signed 64-bit
integer a slot index is carried in. Its own `note` says the refusal fires at codegen rather
than at `cargo check`, which is a fact about a compilation, which is a fact about a target.

So the two words are not two passages of comparable weight. One is total on the coordinate
set; the other is a predicate on its values, indexed by target. That makes Q31's option 3,
"two words with the second scoped to a target", right about the shape and wrong about the
price it quotes: it charges "two admission passages plus a quantifier over compilations",
and the actual cost is one admission and one residue predicate, with the target index
falling out of the residue predicate rather than being added to it. A predicate over
target-dependent facts is target-indexed already.

Option 1, one word, is refuted by the row's own `note` and I have nothing to add: the canon
already says true things about systems arvo cannot host, since the bounded windows it admits
are defined as bounded windows of systems it cannot host.

**And this is the refused shape dissolving rather than a letter being picked.** The
question reads as "which vocabulary governs", and the answer is that two different kinds of
sentence are being asked to share one word: a total function and a predicate. Give each its
own name because they have different types, not because a vote went that way.

**The other reading**: that hosting is not one predicate but a family, one per target, and
that calling it one word hides a quantifier the canon will have to write out anyway. That is
option 3's real point and it may be right. What would distinguish them: whether any canon
sentence needs to quantify over targets rather than fixing one. I did not find one and I did
not sweep for one, so I do not claim there is none.

## 7. Q29, what admission asks a candidate to expose

This one is not the refused shape. It is a real three-way about where the ambient domain
comes from, and its first option is already refuted in its own text. I answer the remaining
two.

**The collapse the first option names is not a collapse, and R1 already says why.** The
worry is that "every system has a second declaration of itself naming its own computed
algebra as its ambient domain, satisfying the list verbatim while computing the identical
function". Take a format `F` with ambient `A` and a nontrivial adaptation, and its collapsed
twin `F'` whose ambient is the algebra `F` actually computes and whose adaptation is the
identity. R1 ratifies that a format is identified by its ambient domain and its representable
set. `A` and `A'` are different domains. So `F` and `F'` are **two formats**, by the ratified
identity clause, which compute the same function. Two things computing one function are not a
collapse; they are two things computing one function, and the canon already has the vocabulary
to say so.

**What the worry is actually about is which object the law verdicts are about.** `F` reports
that associativity holds, truly, of `A`. `F'` reports that it fails, truly, of `A'`. A
consumer wants to know about the computation, so a verdict about the ambient alone answers
the wrong question in `F`'s case. R1 fixes this too, in the clause the question does not
use: "the adaptation is a first-class object with its own laws". The verdict a consumer needs
is the adaptation's, and it is already ratified as available.

So: **option 2, and for a stronger reason than option 2 gives.** It offers the collapse as
"harmless rather than forbidden", which concedes that something odd happens and then tolerates
it. Nothing odd happens. The two declarations are two points of the ratified parameterisation
and are distinguished by it, and the only thing that ever looked like a collapse was reading
a verdict about the ambient as a verdict about the computation.

**Option 3 costs more than it looks.** A consumer-supplied ambient makes a candidate not
determine its own identity until a frame is chosen, which the option itself states. Under R1
the ambient is *half of the identity*. A thing that does not determine its own identity is not
a candidate for admission; it is a family of candidates. That is a coherent object and the
canon may want one, but it is not what admission is over, and calling it admission would put a
quantifier inside the identity clause.

**And the brief's own misnaming turned out to be the hinge.** The brief calls this question
"what the admission contract asks a candidate to supply"; the row says *expose*. Options 1
and 2 are about what a candidate exposes about itself. Option 3 is about what is supplied to
it. They are not three points on one axis, and the row's word is the correct one.

## 8. Q30, predicate or location, restated with the probes behind it

Section 2 derived that admission is a resolution and that both offered options are
projections of it. The probes sharpen the cost of getting this wrong.

`Slots::ADMITTED` is the shipped admission mechanism and it is a `const ()` of assertions:
a predicate, in the option-1 shape. Its failure mode is a const-evaluation error at codegen
carrying a message string. That is precisely the shape R2 refuses: R2's ratified failure mode
is "the signature is missing a coordinate", which names *which* coordinate disagreed, and a
`const ()` of five assertions can name a message but cannot return a coordinate to a caller.
The crate's own `is_admissible::<S>()` exists to work around this, and its doc says why: "the
law, returning a verdict rather than asserting one, so a construction that compiles and is
wrong can be reported on". A verdict is the predicate projection. There is no location
projection anywhere, so a consumer that wants to know which coordinate failed has to read the
message string.

**So the shipped code has independently rediscovered that one projection is not enough, and
has built the second one by hand as a separate function that must be kept in step with the
first.** `is_admissible` and `ADMITTED` restate the same five conditions in two places. That
is the drift the resolution shape removes: one statement, and the predicate and the location
are folds over it.

I do not claim the resolution shape is buildable in this crate today. `ADMITTED` is a const
item forced at use sites, and returning a coordinate name from const evaluation in a form a
diagnostic can print is a different mechanism, possibly one the crate's permitted feature set
refuses. **That is unmeasured and I did not attack it**, because the canon question is what
admission *is* rather than how it is spelled, and
`ruling::the_canon_does_not_police_what_shape_a_law_takes` says the spelling is not the
canon's to fix. The finding stands at the canon tier; the mechanism is a design question.

## 9. What I settle, what I move, what I could not

**Settled, as a derivation from ratified text:**

- Admission is a resolution over the ratified coordinate set, and the predicate and the
  location are two folds over it. R2 ratifies the shape one tier down and its own
  `promotion` says the shape descends a tier at a time.
- Q29 takes option 2, and the collapse is not a collapse: `F` and `F'` are two formats under
  R1's identity clause, and the verdict a consumer needs is the adaptation's, which R1
  already makes first-class.
- Q31 takes two words, with the second a target-indexed predicate over the first's output
  rather than a second admission passage.
- Fixed-radius set-valued carriers are admitted by construction; general ones are refused by
  the ratified concept-closed clause, because admitting them requires an amendment and the
  clause defines admission as joining without one.
- The ambient's operation family is not a coordinate in the shipped crate, measured over four
  algebras with a moving control, and R2 is the ratified rule that calls that a missing
  coordinate.

**Moved rather than settled:**

- Q21 cannot be answered separately from Q33, and Q33 is not in my six. My cut of the subject
  is therefore five questions and a dependency, not six.
- Q20 is not open. It is ratified by `ruling::the_format_spine_is_canon` and I decline to
  re-derive it.

**Settled after going back at what stage 2 had conceded:**

- **The slot arity is fixed at one by a ratified count**, so the Q22 refusal does not rest
  on a reading of "one parameterisation" after all. The ten coordinates R3 counts are
  `3n + 7` at slot arity `n`, which is ten only at one. Reconstructed under two controls.
- **The ten coordinates are separated from the admission obligation in the shipped code
  already**, `ADMITTED` being a `const ()` outside the ten, which is the resolution shape
  sitting in the tree without a name on it.

**Could not:**

- Whether the resolution shape is expressible in this crate's permitted feature set. A
  `const ()` aborts; returning a coordinate name from const evaluation in a form a
  diagnostic can print is a different mechanism and may be refused by the crate's feature
  set. **Not attacked, and stated as not attacked**, because the canon question is what
  admission is rather than how it is spelled, and
  `ruling::the_canon_does_not_police_what_shape_a_law_takes` says the spelling is not the
  canon's.
- Whether the panel should amend the parameterisation to a vector slot. That is a live
  design question I decline to open, and the compositional route reaches the same use cases
  without it.

## 10. Predicates

Per `dimension`, and absent axes hold nowhere.

- **The ambient algebra is not a coordinate of the shipped format concept.**
  `radix: radix in {2, 10}`, `ambient domain: ambient domain in {rationals under plus and
  times, tropical min-plus, two-element Boolean, interval over the rationals}`,
  `total_width: W = 6`, `fraction_width: F = 3`, `signedness: signedness = signed`,
  `toolchain: rustc = nightly-2026-05-28, edition = 2024`, `threads: threads = 1`.
  Evidence: `241_probes/ambient_algebra_is_not_a_coordinate`, four arms and one control that
  moves.
- **One affine slot names a one-parameter family of intervals and no more.**
  `total_width: W = 6`, `signedness: signedness = signed`, `fraction_width: F = 4`,
  `radix: radix = 2`, `toolchain: rustc = nightly-2026-05-28, edition = 2024`,
  `threads: threads = 1`. Counted exhaustively over the declared slot range with the shipped
  predicate, control at radius zero returning a gap of zero.
  Evidence: `241_probes/set_valued_carriers_need_a_second_slot`.
- **The general count is `n(n+1)/2` intervals against `n` slots**, which is width-free:
  `total_width: W any: construction, the count is a function of the slot cardinality alone
  and no step of the argument reads a width`, `radix: radix any: construction, the argument
  counts slot indices and never their denotations`, `threads: threads = 1`.
  Evidence: the same probe at `W = 6`, plus the closed form, which is why the axis is marked
  `construction` rather than swept.
- **The shipped admission mechanism is a predicate and its second projection is hand-written
  beside it.** `crate: crate = arvo-format`, `toolchain: rustc = nightly-2026-05-28,
  edition = 2024`. Read from `mock/crates/arvo-format/src/slots.rs`, `ADMITTED` against
- **The ratified ten reconstructs from the shipped traits, partitioned six and four, and is
  `3n + 7` in the slot arity.** `crate: crate = arvo-format`,
  `toolchain: rustc = nightly-2026-05-28, edition = 2024`,
  `total_width: W any: construction, the count is over trait declarations and no step reads
  a width`, `radix: radix any: construction, no step of the count reads a radix`,
  `threads: threads = 1`. Two controls, both required to pass before the finding prints.
  Evidence: `241_probes/the_ratified_ten_fixes_the_slot_arity`.
  `is_admissible`. Stated as a reading of shipped source rather than a measurement.

Nothing above claims a region at more than one thread, because nothing above was run at more
than one.

## 11. The test gate

The suite runs and is green: 82 tests across the three crates, 81 passing and one ignored,
plus a `trybuild` arm. I read the bodies in the surface I rely on, `arvo-format`'s
`src/tests.rs`, and one of them is weak enough to name.

`the_format_inventory_admits_a_member_this_crate_does_not_know_about` is the only test
asserting the ratified open-inventory clause. Its `Ternary` takes `DecimalRationals`,
`Constant<-1>` and `Signed<3>`, every one of them the crate's own type, and writes only
`PHASE_NUM = 0` and `PHASE_DEN = 1`, which are the trivial values. So the "member this crate
does not know about" is a struct with a different name over the crate's own coordinates, and
it is called `Ternary` while sitting at radix 10.

**It is not tautological and I do not call it that**: its assertions can fail, and the
`radix::<Ternary>() == 10` arm would catch a real regression in coordinate routing. What it
does not do is test the clause in its name. An open inventory means an *outside* crate can
supply coordinates this one does not have, and every coordinate here is one this one has.

`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`
already carries this finding in its `note`, from seat 238, so I am confirming rather than
finding it. What I add is that **the property it fails to test is true**, which nothing had
shown: both my probes are separate crates outside the workspace, each declaring its own
`Ambient` impls and its own `Format` impls, and both compile and run. The open inventory is
real for `Ambient` and `Format`. The test is narrow; the clause is not false.

The repair is one file: a probe-shaped test crate outside the workspace that implements all
of `Ambient`, `Quantum` and `Slots` itself, including `ADMITTED`. Both of mine implement
`Ambient` and `Format` and borrow `Quantum` and `Slots`, so neither is that test either, and
I say so rather than offering them as one.

## Paths opened during the blind phase

Registry, all through `cargo mock query` except where noted:

- `mockspace.toml`
- `mock/registry/question.toml` (query, plus `sed` and `grep` on the raw file)
- `mock/registry/ruling.toml` (query, plus `sed` on the raw file)
- `mock/registry/proposal.toml` (query)
- `mock/registry/dimension.toml` (query)

Source, read directly:

- `mock/crates/arvo-format/src/format.rs`
- `mock/crates/arvo-format/src/ambient.rs`
- `mock/crates/arvo-format/src/quantum.rs`
- `mock/crates/arvo-format/src/slots.rs` (lines 1 to 150)
- `mock/crates/arvo-format/src/tests.rs` (function list, and lines 370 to 412)
- `mock/crates/arvo-placement/src/tests.rs` (the ignored test's attribute only, via grep)
- `mock/crates/arvo-format/Cargo.toml`, `mock/Cargo.toml`

Also: `.claude/rules/` as a listing, generated by `cargo mock` in this worktree. `cargo test
--workspace` run twice.

**Nothing under `mock/research/` opened, and no `git log` beyond my own commits.** The two
probe crates I wrote live under `mock/research/.../241_probes/` and are mine.

The one ignored test is an honest catalogue-red in the correct form:
`the_carrier_is_not_a_function_of_the_access_width` carries
`#[ignore = "catalogue: ... Red until the second packing rule lands."]`. That is the
discipline working rather than a gap, and I record it so a later reader does not have to
re-establish it.
