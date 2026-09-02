# The admission obligations are Q1's, and the open question the design answered is Q31, not Q29

Seat q29a. Worktree `.worktrees/q29a`, branch `research/q29-q29a`, cut from `origin/dev` at
`7fed7b5932f31e77c8b288bd8aabbe93ec3b40aa`.

## Verdict in one line

**Both readings the brief offers are wrong, and the framing is wrong with them.** The nine `ADMITTED`
assertions are not an answer to `question::Q29`, because Q29 asks what a candidate **exposes** and not
one of the nine names anything to expose. They are the Admissibility reading of
`question::what_then_validate_requires` (Q1), which op answered in full, so the work is licensed and
Q29 is untouched. What the design did reach into is a **different** open row:
`question::one_word_or_two_for_is_a_number_system` (Q31), `decider = "panel"`, unanswered, which asks
whether the canon uses one word or two for "is a number system" and "can arvo carry it". The design
uses one word for both, over a set of nine conditions that is demonstrably a mix of the two kinds, and
it does so by citing a row from the wrong topic.

---

## 0. Gates

### The test gate

Ran the whole suite before the assigned work. `cargo test --workspace` from `mock/`: 165 passing
across five binaries, 0 failing, 2 ignored, both carrying `catalogue:` reasons
(`arvo-placement/src/tests.rs:164`, `arvo-format/tests/matlab_fi_parity.rs:278`), which is the
sanctioned shape for a known gap rather than a lie about coverage.

Read the bodies, not the names, of the surface this question touches:
`arvo-format/src/tests/the_inventory.rs` in full, `arvo-format/src/tests/obligations.rs`,
`arvo-placement/src/tests/the_open_inventory.rs`.

**The suite passes the gate and I say so with the reason rather than the count.** Every obligation arm
carries a live control (`the_law_admits_every_range_this_crate_ships`,
`the_law_admits_every_quantum_law_this_crate_ships`), a separation arm asserting the verdict is not
stuck at one value (`the_law_separates_the_two_constructions_rather_than_answering_one_way`), and the
wrong constructions are kept permanently in the file rather than deleted. `the_inventory.rs:193-195` names
and repairs a prior tautology in its own comment ("a constant against the literal its own definition
set"), which is the class `the-test-gate.md` calls not-a-test, caught by the authors rather than by me.
`every_admitted_width_has_a_coherent_range` asserts over all 62 admitted widths rather than a sample.
I found nothing to delete.

One gap worth naming rather than a defect: **the two-directional half of Q1's Admissibility reading is
asserted only over the shipped set plus two foreign constructions.** Op's own answer at
`research/202608072330_the-numeral-canon-panel/28_op_answers_two.md:90` says the reading "owes a
two-directional admissibility sweep". `the_law_admits_every_range_this_crate_ships` covers "does not
refuse ones it could" for 124 shipped ranges and 2 foreign ones. That is a sample of the admissible
region, not a sweep of it.

### The canon gate

Aligned, and the alignment is the finding rather than a formality. Checked against
`mock/registry/*.toml` as declared in `mockspace.toml:32` (`canon_paths = ["mock/registry/*.toml"]`).
Nothing about producing this reading conflicts with a ratified row. What the reading itself reports is
in section 5.

---

## 1. Breaking the brief first

Every "shipped state" claim checked. **All of them hold.** Detail, because a brief whose facts survive
is worth saying so precisely:

| Claim | Verdict | Evidence |
|---|---|---|
| `pub trait Ambient` in `ambient.rs` | holds | `arvo-format/src/ambient.rs:70` |
| `const ADMITTED` at `ambient.rs:159` | holds | `arvo-format/src/ambient.rs:159` |
| three further at `quantum.rs:317`, `slots.rs:210`, `format.rs:227` | holds | exact lines |
| nine assertions between the four | holds | 1 + 2 + 5 + 1 = 9, counted below |
| the proposal cited at five doc-comment sites | holds | `ambient.rs:89`, `quantum.rs:226`, `slots.rs:141`, `arvo-format/src/tests/the_inventory.rs:8`, `arvo-placement/src/tests/the_open_inventory.rs:8` |
| cited inside none of the four `ADMITTED` docs | **partly wrong, and in the brief's favour** | it is cited inside three of the four, in the doc comment attached to `ADMITTED` itself: `ambient.rs:89` sits in the doc block ending at the `const ADMITTED` on line 159, likewise `quantum.rs:226`→`317` and `slots.rs:141`→`210`. What is true is the stronger claim the brief makes next |
| `format.rs` does not cite it at all | holds | `grep -rn the_concept_is_closed_and_the_inventory_is_open mock/crates/` returns eight hits, none in `format.rs` |
| reaches canon through `ruling::the_format_spine_is_canon` | holds | `ruling.toml:1445`, `rung = "ratified"`, `ratified_by = "both"` |
| `question::Q29` open with three options | holds | `question.toml:688`, no `answered` field, `decider = "panel"` |

**The one correction is a wording slip, not a false premise**, and it cuts against the brief's own
framing rather than for it: the proposal *is* cited in three of the four `ADMITTED` doc blocks, which
makes the misattribution I report in section 5 more direct than the brief supposed, not less. I
proceeded.

Two further checks the brief did not make, both of which matter downstream:

- **`format.rs:227`'s `ADMITTED` cites nothing at all.** No registry row, no licence. It is the one
  obligation with no stated provenance.
- **Q29 is cited nowhere in the tree.** `grep -rn "Q29\|what_the_admission_contract_asks" mock/crates/
  mock/design_rounds/` returns zero. The design never claimed to be answering it.

**Predicate.** Holds at tree revision `7fed7b5` and registry revision `7fed7b5`; over
`mock/crates/arvo-format`, `mock/crates/arvo-placement` and `mock/registry/*.toml`; over the full text
of each named file. Does not hold over any other revision, any other crate, or `mock/design_rounds/`
beyond the single grep named.

---

## 2. The nine assertions, enumerated, because the argument turns on what they are

| # | Site | Assertion | Message names |
|---|---|---|---|
| 1 | `ambient.rs:160` | `RADIX.is_positional()`, i.e. `RADIX >= 2` | "not a positional notation" |
| 2 | `quantum.rs:318` | `ranges_over_a_magnitude(MAGNITUDES)` | "describes no values, so it is not a step law" |
| 3 | `quantum.rs:322` | `reach_is_representable(BASE, SLOPE, MAGNITUDES)` | "past what an **exponent carries**" |
| 4 | `slots.rs:211` | `MIN.index() <= MAX.index()` | "inverted ... admits nothing" |
| 5 | `slots.rs:215` | `WIDTH.count() >= 1` | "admits no values and is not a slot range" |
| 6 | `slots.rs:219` | `WIDTH.count() <= 62` | "wider than a **slot index carries** ... `2^63` does not fit a **signed 64-bit integer**" |
| 7 | `slots.rs:228` | `MAX - MIN < i64::MAX` | "more indices than **a count can carry**" |
| 8 | `slots.rs:232` | `MAX - MIN < 2^WIDTH` | "the declared width does not cover the range" |
| 9 | `format.rs:228` | `PHASE.denotes()`, i.e. denominator `!= 0` | "names no position on the grid" |

**Not one of the nine names a thing a candidate must expose.** Every one is a condition on a coordinate
already declared. That is the whole of the refutation of reading (a), and it is mechanical rather than
interpretive.

**Predicate.** Holds at tree revision `7fed7b5`; over the four `const ADMITTED` blocks in
`mock/crates/arvo-format/src/`; over every assertion in each. Does not hold over `ADMITTED_WIDTHS`
(`slots.rs:293`), which is a macro-generated inventory and not an obligation, nor over any other crate.

---

## 3. Reading (a) is refuted: Q29 asks for a different kind of sentence

`question::Q29` (`question.toml:688-700`), verbatim:

> asks = "What does the admission contract ask a candidate number system to expose?"

and its three options, each an exposure list and nothing else:

> "The standing list prefixed with the reduction's two law verdicts ..."
> "The same, plus the ambient domain's own law inventory, plus a third verdict for the retraction ..."
> "Admission relative to a consumer-supplied ambient domain, the candidate exposing only its
> representable set and its reduction ..."

Three claims follow, each checkable.

**3.1 The option space contains no conditions, only exposures.** Every option enumerates things a
candidate hands over: a standing list, law verdicts, a law inventory, a representable set, a reduction.
None constrains a value. The nine assertions constrain values and enumerate nothing. **A sentence
answering Q29 and a sentence in an `ADMITTED` block are different kinds of sentence**, and no reading
of Q29 makes "a radix below two is not a positional notation" one of its three answers or a fourth.

**3.2 The tree's exposure shape matches none of the three options, so it is not a fourth answer
either.** What `Format` exposes is `type Ambient`, `type Quantum`, `type Slots` and `const PHASE`
(`format.rs:151-171`), reaching nine required consts and three required associated types. **There is no
reduction anywhere in it.** `Adaptation` (`adapt.rs:27`) is the reduction, it is a separate trait, and
`Format` neither names it nor requires it. Option 3 asks for "its representable set and its reduction";
`Format` gives the representable set and the ambient domain and no reduction. Options 1 and 2 ask for
law verdicts; `Format` exposes no verdict.

**3.3 The `ADMITTED` blocks stop exactly where Q29's subject matter starts.** Q29's candidate is a
*number system*. `proposal::the_numeral_concept_is_a_dependent_sequence_of_choices` (`proposal.toml`,
topic `the_number_system`) says:

> "an ambient domain, a representable set over it, a reduction from the space that pair derives, an
> encoding of the set, a container for the encoding"

The four `ADMITTED` blocks sit on coordinates one and two of that sequence and nowhere else.
`Adaptation` (coordinate three) carries no `ADMITTED`. Neither does `DeclaredSignature`
(`adapt.rs:63`), `Operation` (`adapt.rs:121`), `Rounding` (`rounding.rs:23`) or `Overflow`
(`overflow.rs:26`). **Q29 asks about the coordinate the tree's obligations do not reach.**

And coordinates one and two are precisely what `ruling::the_format_spine_is_canon` already fixed, via
`proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set`:

> "A format is identified by its ambient domain and its representable set."

with `proposal::membership_of_the_representable_set_is_one_affine_predicate` fixing the
parameterisation:

> "an affine slot function, a quantum per magnitude and a phase"

**So the ten-ish coordinate exposure list the design does carry is not an addition to the canon; it is
the ratified affine parameterisation spelled out at the design tier, which is the design tier's job.**

**Predicate.** Holds at registry revision `7fed7b5` and tree revision `7fed7b5`; over
`question::what_the_admission_contract_asks_a_candidate_to_expose` as written, over the four `const
ADMITTED` blocks, and over the nine public traits in `arvo-format/src/`. Does not hold if Q29's option
list is amended to carry a condition rather than an exposure, and does not hold over any revision in
which `Adaptation` grows an `ADMITTED`.

**3.4 A ratified ruling already says what the exposure list is, and it is not Q29's answer.**
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`,
`rung = "ratified"`, `ratified_by = "experts"`:

> "What the door carries out is **the coordinate set of the ratified parameterisation**, spelled in
> types the stack owns."

**So the ten coordinates are canon-derived and already ratified as a coordinate set**, descending from
`membership_of_the_representable_set_is_one_affine_predicate` through
`ruling::the_format_spine_is_canon`. They are not an unlicensed exposure list, and they are not a
fourth answer to Q29. That row was ratified after the crate was written and about this exact surface,
which is the strongest independent confirmation available that the design's coordinate list is where
it belongs.

### The one thing that could move 3.3, named rather than hidden

`proposal::the_numeral_concept_is_a_dependent_sequence_of_choices`'s own note, at `proposal.toml:288`:

> "The word `format` collides here: two units use it for different prefixes of this same sequence, and
> the collision is unresolved and is op's naming call."

So the canon records that whether the format topic's object and the number-system topic's object are
the same thing is **unresolved and op's**. My 3.3 does not depend on resolving it: whatever the two are
called, a Q29 candidate is asked for a reduction and verdicts, and the tree's obligations reach neither.
**But if the collision resolves toward one object, then 3.2 becomes live**, and the ten-coordinate
exposure list in `the_inventory.rs:108` would be a fourth answer to Q29 written in the design tier.
That is a conditional finding and it is op's to close, not mine.

---

## 4. Reading (b) is wrong about the remedy: the third condition is Q1, and it is already answered

The brief's (b) says the obligations are "a third kind of condition, standing beside membership and
hosting", so "no open row asks about them" and "they want a registry row of their own".

**They have one, it is older than the crate, and op answered it.**
`question::what_then_validate_requires`, key `Q1`, `decider = "op"`, topic `the_format`
(`question.toml`), option one verbatim:

> "Admissibility: the typestate refuses declarations it cannot serve and does not refuse ones it could,
> which owes a two-directional sweep and for which the panel holds no evidence."

That is the nine assertions, described in advance, in the canon, at the right tier and the right topic.
`topic::validation` (`topic.toml`) exists for exactly it:

> what = "What validating a numeral means and who does it: whether a shape is admissible, whether a use
> is correct, and whether a value checks itself."
> unit = "raised as Q1 and never separated from the format topic"

And op answered it, verbatim, at
`mock/research/202608072330_the-numeral-canon-panel/28_op_answers_two.md:82`:

> Usage, Admissibility, Self-validation, All that makes sense. Again, since this is the new panel, we
> learnt from last night, this isn't a strict call, so it can be challenged if truly not worth it, but
> my instinct is that the more robust it is, the better it'll serve us.

**All three readings, conjunctively, with a named challenge route.** So building the Admissibility
reading is not drift. It is the design tier doing what op asked for, and the `ADMITTED` blocks are its
first realisation.

**So (b) is right that the conditions are a third kind of thing and wrong that nothing asks about them.**
The remedy is not a new row. It is recording the answer to the row that exists.

### The canon defect that made (b) look plausible, and that caused the misattribution

**`question::what_then_validate_requires` carries no `answered` field.** Verified mechanically: of 106
question rows, 14 carry `answered` and Q1 is not among them. Its `note` says only "Recorded as answered
at `28` batch one", which points at a panel file rather than carrying the answer.

**So the canon does not record op's answer to an op-decided question he answered.** A reader grepping
the registry for the licence to write an admissibility check finds Q1 looking open, and reaches for the
nearest ratified sentence with the word "admission" in it. Which is exactly what happened: the three
`ADMITTED` blocks that cite anything cite `proposal::the_concept_is_closed_and_the_inventory_is_open`,
a `topic::the_number_system` row, to license a `topic::the_format` mechanism. **Q1 is cited nowhere in
`mock/crates/`.** The design is operating under a licence it does not know it has, using one it does not
have.

**Predicate.** Holds at registry revision `7fed7b5`, over `mock/registry/question.toml` in full and
`mock/research/202608072330_the-numeral-canon-panel/28_op_answers_two.md`; over the 106 question rows
counted. Does not hold if `28`'s transcription of op's words is itself wrong, which I did not
independently verify against `.data/op-responses/` because that lives outside this worktree.

---

## 5. What the design actually did answer, and it is Q31

`question::one_word_or_two_for_is_a_number_system`, key `Q31`, `decider = "panel"`, topic
`the_number_system`, **open, no `answered` field**:

> asks = "Does the canon use one word or two for \"is a number system\" and \"can arvo carry it\"?"

with option one:

> "One word, folding the residue clauses into the concept, which makes the concept exclude unbounded
> exact rationals as a matter of mathematics, which is false, and makes every hosting clause
> conditional on unratified constraints inside a sentence about arithmetic."

**The design uses one word.** `ADMITTED`, `is_admissible`, `is_admissible_ambient`,
`is_admissible_quantum`, `is_admissible_format`, and in prose "the concept's obligations". One
vocabulary, one const per contract, one verdict function per contract.

**And the nine conditions under that one word are two kinds, which the assertion messages themselves
say.** Split by the mechanical test *does the condition name a carrier width of this crate's own
types*:

- **Six are about the concept**: #1 radix below two is not positional; #2 a law over no magnitudes
  describes no values; #4 an inverted range admits nothing; #5 zero bits admits no values; #8 the
  declared width does not cover the range; #9 a zero denominator names no position. None mentions a
  carrier.
- **Three are about what this implementation can carry**: #3 names "what an **exponent** carries", and
  `Exponent` is `i32` (`quantum.rs:40`); #6 names "what a **slot index** carries" and "a signed 64-bit
  integer", and `Slot` is `i64` (`slots.rs:41`); #7 names "what **a count** can carry", same carrier.

A `Slots` impl with `WIDTH = 63` is a perfectly good slot range as a concept. This crate cannot carry
it, because its own index is `i64`. The design refuses it under the same word, from the same const,
with the same citation, as a radix of one.

`proposal::membership_and_hosting_are_two_questions` (topic `the_number_system`) says that is two
things:

> "Whether something is a number system and whether this implementation can carry one are different
> questions. The first is about structure and is answered by locating the candidate on the chain of
> choices; the second is about residue at runtime and is answered by what a value at rest may carry.
> Neither enumerates, both are open, and **a system the implementation cannot host is still a system**."

**I am not calling that a refutation, and the reason is provenance.** That row carries `standing =
"one_expert"`, the weakest tier, and Q1 outranks it: op decided Q1, and Q1's Admissibility reading is
about "the typestate", which is entitled to refuse what it cannot serve regardless of what the concept
admits. **So folding the three carrier bounds into `ADMITTED` is licensed.** What is not licensed is
naming the result "the concept's obligations", because that imports the concept-level word for a
typestate-level check, which is Q31 option one chosen in the design tier while Q31 is open with
`decider = "panel"`.

**And the repo's own standing instruction names this act specifically.** `.claude/CLAUDE.md`, generated
from `mock/agent/`, deriving from `ruling::the_panel_finishes_the_canon_without_him`
(`rung = "ratified"`, `ratified_by = "op"`, whose own words are "the canon should be solvable and fully
fillable without me from now on"):

> "Where the canon explicitly holds a question open and reserves the call, that is not silence and it
> may not be filled inside a design."

**Q31 is explicitly open and explicitly reserved** (`decider = "panel"`). That is not silence and the
design filled it. I mark the sentence as agent-authored instruction text rather than a registry row,
because it is: the ruling it derives from says "silence in his corpus is still not permission" and
"Nothing is parked awaiting him", and the design-may-not-fill clause is the generated instruction's own
extension of it. **It is the right extension and it is still one tier down from the row**, so I rest
section 5 on Q31's own `decider` field rather than on this sentence.

**This is the drift, and it is small, real, and cheap to fix.** Not a mechanism to delete. A
vocabulary the canon reserved to a panel and the design spent.

**Predicate.** Holds at tree revision `7fed7b5` and registry revision `7fed7b5`; over the nine
assertions enumerated in section 2 and their message text; over `Exponent`, `Slot` and `SlotCount` as
declared at `quantum.rs:40`, `slots.rs:41`. The six/three split holds under the stated test and no
other; a different test could cut #5 or #8 the other way, and I do not claim it would not. Does not
hold if `Slot` or `Exponent` change carrier, at which point #3, #6 and #7 change value and possibly
kind.

---

## 6. One retraction and two things the canon does not license

**6.1 The ten is ratified and correct. The one-plus-nine split under it is not, and I had this backwards
until the canon corrected me.**

I first wrote this entry claiming "ten coordinates" matched nothing countable. **That was wrong, and the
row that refutes it is ratified.**
`ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`
(`rung = "ratified"`, `ratified_by = "experts"`) says:

> "`Width` is a count of bits over `u32`, and three of **the ten associated constants** carry values it
> cannot hold at all"

and its `promotion` names "a **ten-coordinate** door" as a built existence proof. **Ten is the canon's
own figure for the coordinate set**, and it reaches ten because the phase is two coordinates: that same
row names `Format::PHASE_DEN` as one of the constants it counted. So `the_inventory.rs:108` is quoting
a ratified count, and `the_whole_contract_is_supplied_from_outside_and_every_coordinate_reads_back`
duly asserts the phase's numerator and denominator separately. **I retract the claim and record the
retraction rather than deleting it**, because the failure is the one this deliverable is about: a count
checked against the tree and not against the canon.

What survives is narrower and still real. **The current tree carries nine associated constants, not
ten**, because `PHASE_NUM` and `PHASE_DEN` were merged into a single `PHASE: Phase` (`format.rs:171`)
after that ruling was written. The coordinate set is still ten values; the const count is now nine.
Under that, `the_open_inventory.rs:24`'s split, "the one coordinate `Format` itself demands" against
"the nine underneath it", is off by one in each half: `Format` demands two of the ten coordinates and
eight sit underneath it, or one const and eight. **It reads as ten by adding one to each side of the
right split**, which is the shape a number gets when it is carried rather than recounted.

**Neither is a defect worth a round on its own.** It is worth naming because the eventual repair is a
sentence saying the phase is two coordinates in one const, and without that sentence the next reader
recounts and gets nine, exactly as I did.

**6.2 A totality claim about the obligation set that is false.**
`obligations.rs:11`: "Four contracts here are open, so four carry an obligation." `arvo-format` has
nine public traits. `Rounding` declares itself closed (`rounding.rs:20-22`). That leaves **eight open
contracts and four obligations**. `Overflow` says of itself at `overflow.rs:24`:

> "Open: an implementor outside this crate is a policy this crate does not know about, and that is the
> intended shape."

and carries no obligation. `Adaptation`, `DeclaredSignature` and `Operation` are open and carry none.
The sentence quantifies over "contracts here" and is wrong by four.

**6.3 An unguarded coordinate of exactly the class the four obligations exist for.**
`Operation::ARITY` (`adapt.rs:126`) is an `Arity`, and `Arity::of` (`adapt.rs:97`) is
`pub const fn of(operands: u32) -> Self { Self(operands) }` with no guard. **`Arity::of(0)` is
constructible**, so an `Operation` declaring zero operands compiles and nothing refuses it. That is the
same shape as `MagnitudeCount::of(0)`, which #2 exists to refuse, one contract over and unguarded.

**Predicate.** All three hold at tree revision `7fed7b5` and registry revision `7fed7b5`; over `mock/crates/arvo-format/src/` in full.
6.1 holds over `the_inventory.rs`, `the_open_inventory.rs` and `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`, and its retracted half held nowhere. 6.2 holds over the nine `pub trait` declarations in that
directory as counted. 6.3 holds over `Arity` as declared and over every construction path into it
reachable from outside the crate, which is `of`, `UNARY` and `BINARY` alone. None holds over any other
crate or revision.

---

## 7. What the canon decides and what it does not

**Decided, and cited:**

- The nine assertions are not an answer to Q29. Q29 asks for an exposure list; none of the nine
  exposes anything, and the tree's exposure shape matches none of Q29's three options and carries no
  reduction. `question.toml:688`, `arvo-format/src/adapt.rs:27`, section 2.
- The obligations are licensed. Q1's Admissibility reading names them in advance, op answered Q1
  selecting all three readings, and `topic::validation` exists for exactly this.
  `28_op_answers_two.md:82`.
- The format's identity and parameterisation are ratified, and `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon` says outright that the coordinate set is what the door carries, so the exposure list the design does carry
  is derived rather than invented. `ruling::the_format_spine_is_canon`.

**Not decided, and it is op's:**

- Whether the format topic's object and the number-system topic's object are one thing.
  `proposal.toml:288` records the collision as unresolved and as his naming call. **If they are one,
  section 3.2's conditional fires and the design's coordinate list becomes a fourth Q29 answer written
  below the canon.**
- Q31 itself, which is a panel's. The design has already spent it.

**Not decided by the canon, decided by provenance:**

- Whether the three carrier bounds belong under the same word as the six concept conditions. Q1
  (op) licenses folding them in; `membership_and_hosting_are_two_questions` (one expert) says they are
  two questions. Q1 wins on provenance, so the mechanism stands and the **word** is the defect.

---

## 8. What would fix it, cheapest first

Each is a registry act, not a code act, and none touches the mechanism.

1. **Record op's Q1 answer in the row.** Add `answered` to
   `question::what_then_validate_requires` carrying his verbatim words and the fact that he selected
   all three readings with a named challenge route. That is the licence the crate is operating under
   and it currently exists only in a panel file.
2. **Recite the obligations against Q1 rather than against the number-system row.** Three doc comments,
   `ambient.rs:89`, `quantum.rs:226`, `slots.rs:141`, and the two design passages at
   `arvo-format/DESIGN.md.tmpl:387` and `:601`. `format.rs:227` cites nothing and should cite it too.
3. **Stop calling the check "the concept's obligations"** until Q31 closes. The mechanism is a
   typestate admissibility check over declared coordinates; naming it that costs nothing and spends no
   open question.
4. **Split the three carrier bounds visibly**, in the message or in a second predicate, so a reader can
   tell a refusal about the concept from a refusal about this crate's index width. Not required by any
   ratified row. It is what makes Q31 answerable either way later instead of pre-answered.
5. **6.1, 6.2 and 6.3 are independent** and each is a few lines.

## 9. What I did not reach

- `.data/op-responses/`, which is outside this worktree, so op's Q1 words are taken from panel file
  `28` alone. One instance, not three. If `28` transcribed him wrongly, section 4 falls.
- The panel files behind Q29 and Q31 (`73`, `74`, `OPTIONS`). I read the rows and the format-topic
  provenance, not the arguments that produced the number-system options. A reading of `73` and `74`
  could move section 3.3's conditional.
- No search beyond the repository was needed and none was run.
