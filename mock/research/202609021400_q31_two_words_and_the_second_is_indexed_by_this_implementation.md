# Q31 takes two words, and the canon has already been writing both of them

Seat q31a. **Verdict: option two.** Two words, because one word contradicts three canon
sentences that are already ratified or in force, and a vocabulary choice that falsifies a
ratified row loses to it by provenance rather than by argument. The row's stated price for
option two is wrong in shape and is corrected below; its stated price for option one is stale
and names a constraint set that has since been put in force.

## 0. What this seat's blindness covers, stated by surface rather than by the word

- **Blind against** `research/q31-*` other than my own branch. I ran no `git log --all`, no
  `git branch -a`, and opened no worktree under `.worktrees/` other than `q31a`. My branch
  was cut from `origin/dev` at `26f65faf`, before any parallel seat's work existed on it.
- **Not blind against** `mock/registry/*.toml`, `mock/crates/`, `mock/lints/`, `mock/tools/`,
  or `mock/research/`, all of which I read freely, including
  `mock/research/202608072330_the-numeral-canon-panel/74_giesen_consolidation_the_number_system_concept.md`,
  `73`, `68`, `241`, `246`, and the two files the brief named on `dev`.
- **Order, because it bears on what counts as independent.** I read the ratified spine, the
  `dimension` namespace and `arvo-format/src/ambient.rs` and reached the verdict before
  opening `241` or `246`. I had read `74`'s N10 before that, which is the source of
  `proposal::membership_and_hosting_are_two_questions`, so **my agreement with that row is
  not an independent arrival** and is not counted as one anywhere below. What I claim as my
  own is the warrant, not the conclusion: that row argues from cost, and section 2 argues
  from ratified text, which is a different tier of reason for the same sentence.

## 0a. The two gates

**Canon gate: aligned.** The row is in the `question` namespace, which is open by definition;
its `decider` is `panel`; it carries no `answered`. Answering it is the work the row exists
for. Checked against `mock/registry/question.toml`, `ruling.toml` and `topic.toml`, and
against `canon_paths` in `mockspace.toml`. One thing I did **not** find and looked for: any
ruling row on `topic = "the_number_system"` above `rung = "open"`. There are two and both are
deferrals. What settles this question reaches the topic sideways, through
`ruling::the_format_spine_is_canon`, whose own `topic` is `the_format` and which ratifies a
proposal whose topic is `the_number_system`. A reader filtering rulings by topic misses it,
which is how I nearly missed it.

**Test gate: passed, with one gap named and closed below.** The whole suite runs green: 104,
13, 8, 5 and 4 across the workspace, plus 9 doctests of which 5 are `compile_fail`. I read the
bodies, not the names, of every test in the surface this answer touches: all twelve in
`mock/crates/arvo-format/src/tests/obligations.rs` and the admission tests in
`the_inventory.rs`. They are real tests. Each wrong construction is kept permanently, each has
a control admitting the shipped set, and each obligation additionally carries a
`_separates_the_two_constructions_rather_than_answering_one_way` arm that a law stuck at
`true` or at `false` would fail. That is better than most suites I am handed.

**The gap, and it sits exactly on the clause this question turns on.** Section 6.2 measures it
by mutation and section 6.3 supplies the closer. It is one clause of five in one obligation,
the suite is not decorative, and it is not in the path of this answer, so I proceeded rather
than refusing.

## 1. What the question actually asks, because "one word or two" understates it

`question::one_word_or_two_for_is_a_number_system` reads as a vocabulary question and its own
options do not. Option one is "folding the residue clauses **into the concept**", which is a
structural merge and not a rename. So the question is: does the canon carry one predicate or
two, and only afterwards what to call the second.

That matters for who decides. The structural half is settled by canon and is answered here.
The residue, what the second word is spelled, is a naming call, and `topic::naming` is the
canon's own file for "where the name is a call somebody has to make rather than something a
measurement settles". I name candidates in section 5 and pick none.

## 2. One word contradicts the ratified canon, three times over

`ruling::the_format_spine_is_canon` carries `rung = "ratified"` and `ratified_by = "both"`. It
ratifies four propositions. Two of them are about a domain, and the domain is where the
argument is.

**2.1 A ratified sentence asserts an exact operation in a domain arvo cannot host.**

> `proposal::arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`:
> "Arithmetic on a format is an exact operation in an ambient domain composed with a named,
> total adaptation onto the representable set."

The exactness is the whole content of the factoring. An operation that is exact is exact
somewhere, and the somewhere is not the representable set, because the adaptation is what maps
onto that set afterwards. For binary fixed point the somewhere is the dyadic rationals. Arvo
cannot carry the rationals: `ruling::the_operating_constraints_are_intents_and_rules` is
`in_force` and forbids alloc and runtime growth, and an exact rational needs unbounded terms.

Under one word, "the rationals" is not a number system, and this ratified sentence then
quantifies over a non-system. It becomes either false or vacuous. **A vocabulary choice that
falsifies a ratified row is refused by provenance.**

**2.2 A ratified sentence puts that domain inside a format's identity.**

> `proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set`:
> "A format is identified by its ambient domain and its representable set."

So the unhostable thing is not incidental scenery the canon mentions in passing. It is half of
what a hostable thing **is**. One word cannot state the identity of a hostable format without
naming an unhostable domain, which is the row's own distinguisher in its sharpest available
form: the bounded windows arvo admits are defined as bounded windows of systems it cannot host.

**2.3 The canon's own predicate notation ranges over those domains, and a live row uses it.**

`dimension::ambient_domain`'s `grammar` field: "Values name a domain: the integers, **the
rationals, the reals**, a finite ring." And a committed predicate writes it, on a `measured`
row about the additive identity:

> `proposal::the_additive_identity_is_decided_by_the_phase_being_a_whole_multiple_of_the_quantum`,
> `predicate`: `"ambient_domain: ambient domain in {the rationals, the rationals at radix ten}"`

Under `every-finding-carries-its-predicate`, a listed dimension is a region the finding holds
in. So the canon carries a measured claim that **holds over the rationals**. Under one word
that predicate is unwritable, because its region would be a set of non-systems.

`does_the_canon_speak_of_unhostable_systems.sh` runs all of this through `cargo mock query`,
the harness's own reader of `canon_paths`, with a negative control (`the surreals`, `the
hyperreals`, `the p-adics`: 0 hits each) and a positive one (`the rationals`, `the reals`: 2
each), so the zeroes in it are facts about the registry rather than about the grep.

**2.4 The three are one argument and it does not need the hosting half.**

Worth saying because `proposal::membership_and_hosting_are_two_questions` is `one_expert` and
its own `note` discounts the hosting half as "a different author's and conditional on an open
question of op's". Nothing above touches the hosting half. It runs entirely off the ratified
concept clauses plus one `in_force` constraint row, so it stands whatever happens to the open
question that row is conditional on.

## 3. Option one is refused, and its stated price is stale

Two costs are written into option one. One is right and one is no longer true.

**Right, and now stronger than the row states it.** "Makes the concept exclude unbounded exact
rationals as a matter of mathematics, which is false." Correct, and section 2 sharpens it: the
problem is not that the excluded sentence would be false in mathematics, it is that the
excluded sentence is **already ratified in this registry**. That moves the objection from a
matter of taste about redefining a term of art to a conflict inside the canon.

**Stale.** "Makes every hosting clause conditional on **unratified constraints** inside a
sentence about arithmetic." The constraints are `ruling::the_operating_constraints_are_intents_and_rules`,
`rung = "in_force"`, in op's own words "already in place, enforced by the mockspace lints and
the workspace and repo rules, and they are **not to be questioned**." That row's `note` says
exactly what happened: "The panel had spent five files hedging that arguments resting on these
constraints stood on unratified ground... nothing built on the constraints needed redoing,
only the hedge attached to it."

Q31's option one is one more copy of that hedge, still standing, still being read. **It is
priced against a premise op corrected.** Option one is refused anyway, on section 2's grounds,
but a live row should not carry a dead reason: whoever fixes this row deletes that clause and
leaves the rest.

## 4. Option three is refused, and it is measured rather than argued

Option three scopes the second word to a target, "at the same cost plus a quantifier over
compilations". A quantifier is worth writing when the thing quantified over varies. Two
independent measurements say it does not.

`is_the_hosting_predicate_target_indexed.sh` sweeps every non-test source file in all three
crates, for pointer width, target architecture, target OS, target endianness, target features,
any `cfg(target`, any `cfg(feature`, `f16`, `f128` and pointer-sized `size_of`. **Zero.**
Every hosting bound is carried by `Radix(u32)`, `Exponent(i32)`, `MagnitudeCount(u32)`,
`Slot(i64)` and `Width(u32)`, all fixed-width. With the declared coordinates held fixed, no
assertion in any `ADMITTED` block can change truth value between two targets.

Two controls, both required to pass before the zero is reported: a planted file carrying each
pattern must be found, and the crates' own test files must yield the one hit known to be there.
Both fire.

**And the first version of this probe reported four hits, which is why the two columns exist.**
A broad `#[cfg` pattern matched four `#[cfg(test)]` attributes. A test-inclusion gate is
conditional compilation and is not a target index. The probe now reports both columns and
prints every ANY hit, so a reader can see what the broad number was made of rather than
trusting my split.

This is a replication rather than a citation. `246`'s
`hosting_is_not_target_indexed.sh` reached the same zero over `arvo-format` alone with a
narrower pattern set; mine widens the pattern and the scope. **Two instruments, and I did not
build mine from its.** Neither is a claim about what the stack should do, only about what it
does.

**Separately, the canon already has the general mechanism option three would duplicate.**
`dimension::target_features` is a declared axis of the predicate notation, so any claim,
hosting claims included, already scopes to a target the same way every other claim does.
Option three proposes a bespoke target index in the *vocabulary* for one predicate, where the
*notation* carries one uniformly for all of them. That is four analyses of one structure
waiting to be one, and the canon has already factored it.

## 5. What option two actually costs, which is not what the row says

Option two's stated price is "two admission passages and a reader told which is which at every
use." I do not think that is the shape, and the correction makes option two cheaper rather
than defending it.

**The two sentences are not two passages of comparable weight, because they have different
types.** The first is total: every candidate gets located somewhere on the chain of choices,
and nothing is refused, which is what `question::is_admission_a_predicate_or_a_location`
option two is circling from the other side. The second is a partial predicate over the values
those coordinates take, and it refuses. So the cost is one procedure plus one gate, not two
procedures. A reader told "this locates, that refuses" needs telling once, in the definition,
not "at every use".

**What the second word is indexed by, which the row leaves unstated in option two and gets
wrong in option three: this implementation.** Section 4 measured it. `Slots::ADMITTED` refuses
a 63-bit range because `2^63` does not fit the `i64` **this crate** counts slots in. Widen
`Slot` to `i128` and the refusal moves, on every target at once. That is what
`proposal::membership_and_hosting_are_two_questions` already says in its own words, "whether
**this implementation** can carry one", and it is the word to keep.

**Candidate spellings, and I pick none, because `topic::naming` says the pick is not a
measurement's to make.** For the first: membership, or location, or being a number system. For
the second: hosting, carriage, residue, or bearing. My only substantive input is that whatever
the second is called, it should not be `admission`, because `admission` is the word the
ratified `the_concept_is_closed_and_the_inventory_is_open` already spends on the first: "what
admission requires... a new one earns admission by supplying **the concept's** obligations".

## 6. The design tier, and where I part from the two files on `dev`

Both `202609020400_...` and `202609021238_...` conclude that the design tier has already
answered Q31 in favour of option one, inside `arvo-format`. **Half right, and the half that is
wrong changes what should be done about it.**

**6.1 What is right, and I reproduced it independently.** `Slots::ADMITTED` and
`Quantum::ADMITTED` weld hosting bounds into the same const as concept clauses, with nothing
separating them. Classified by exhibiting the refused candidate rather than by matching text:

| where | what it refuses | behind the refusal |
|---|---|---|
| `ambient.rs` `RADIX >= 2` | radix 1 and 0 | nothing. Every magnitude names the same value |
| `format.rs` phase denominator | a zero denominator | nothing. It names no position on the grid |
| `quantum.rs` magnitudes >= 1 | a law over no magnitudes | nothing. It parameterises the empty set |
| `quantum.rs` `reach_is_representable` | a reach outside `i32` | a step law over 2^40 magnitudes, which is a number system |
| `slots.rs` `MIN <= MAX` | an inverted range | nothing |
| `slots.rs` `WIDTH >= 1` | zero bits | nothing |
| `slots.rs` `WIDTH <= 62` | a 63-bit range | the 63-bit two's complement integers |
| `slots.rs` span `< i64::MAX` | a span of 2^63 | a system with 2^63 values |
| `slots.rs` span `< 1 << WIDTH` | a width that cannot address its range | nothing. Two declared coordinates disagreeing |

Six refuse candidates that denote no values at all. Three refuse candidates any reader can
name. `refused_candidates.rs` runs the two clearest through the shipped verdict functions with
their controls: radix 1 refused and radix 2 admitted, 63 bits refused and 62 bits admitted.

**My first classifier got two of these cells wrong and it is committed as the audit trail.**
It matched on assertion text, which put `reach_is_representable`'s `i32` bound one level out of
reach and read `slots.rs`'s overflow-safe `i128` casts as a bound. The two errors cancelled in
the total, 6 and 3 for the wrong reasons, which is exactly why a count alone would have been
believed. Withdrawn, superseded, and the reason written into its own output file rather than
deleted.

**6.2 What is wrong: the crate has not chosen one word. It has chosen no word, and then been
inconsistent.** Under option one the concept excludes what arvo cannot host. `Ambient::ADMITTED`
carries **one** clause, radix at least two, and nothing about hosting at all, and the crate
then instantiates it with `BinaryRationals`, `UnsignedBinaryRationals` and `DecimalRationals`,
whose domain is the rationals. `is_admissible_ambient::<BinaryRationals>()` returns true, and
`obligations.rs:173` pins it as a test. **Under option one that construction would be
inadmissible. It is admitted, and the admission is asserted permanently.**

So the same crate admits an unhostable domain at one obligation and refuses a hostable system
at another, calling both `ADMITTED`. That is not a side being picked. It is what a design tier
does when the tier above it has no word for a distinction the design keeps running into: it
gets the distinction right where the case is obvious and loses it where the case is subtle,
and nothing in the source can tell the two apart afterwards.

**Which changes the remedy.** The `dev` files propose reverting `slots.rs:220` and `:229` or
getting a ruling that puts a hosting bound inside admission. Neither is right on this reading.
The hosting bounds are correct and load-bearing: `SpanTooWide`'s own comment records that
before the obligation was strengthened, `slot_count` panicked under `overflow-checks` and
returned `-9223372036854775808` without it. Deleting them reintroduces a measured defect. What
is wrong is that they share a name with the concept clauses. **Answer Q31 with two words and
the fix is a second const, not a revert.**

**6.3 A test-gate finding, and it lands on this exact clause.** `WIDTH <= 62` is the crate's
single clearest hosting condition. **Delete it and all 134 tests stay green.**
`does_any_test_isolate_the_62_bit_clause.sh` mutates the const and the verdict together, runs
the whole suite, and gets zero failures. The construction the suite points that clause at,
`the_inventory.rs`'s `RogueRange`, has `MIN = 4611686018427387904` and
`MAX = -4611686018427387905`, so it is **inverted as well as 63 bits wide**, and the first
clause refuses it whether or not the third exists. Its assertion message says "an inverted
range was admitted", which is honest about what it caught and is why nobody noticed what it
does not.

The control is in the same probe: deleting `MIN <= MAX` turns two things red immediately,
including the `compile_fail` doctest at `slots.rs:166`. So the method fires and the zero is a
fact about the suite.

**The closer, written and shown to fire**, because naming a gap is half a finding.
`the_isolating_construction.rs` declares `MIN = 0`, `MAX = 255`, `WIDTH = 63`, which holds
every clause but the width bound: the crate already ships `WiderThanItsSpan` at `MIN = 0`, `MAX = 3`, `WIDTH = 13` and asserts it
admissible, so a width far wider than its span is a deliberately admitted shape and this
construction differs from an admitted one in exactly one coordinate. It returns false against the crate and true against the mutant, exit 0 and
exit 1. It belongs in `the_inventory.rs` beside `SpanTooWide` and it is one paste. It is in the
probe directory rather than in the crate because this seat is in TOPIC phase and may not edit
crate source.

## 7. Findings outside the question, per the standing instruction

**7.1 `topic::the_number_system` writes the conflation into the taxonomy.** Its `what` field
reads "What counts as a number system **arvo can carry**, and what a candidate has to expose
to be admitted as one." That is option one, phrased, in the file every reader of this registry
starts from. `topic` rows carry no `rung`, so this is untiered taxonomy rather than a canon
claim, which is the only reason it is a finding and not a contradiction. It still frames the
question for anybody who reads the topic list before reading the rows, and this seat nearly
began from it. **When Q31 is answered, that sentence is edited in the same act**, or the answer
is contradicted by its own filing cabinet.

**7.2 `question::is_admission_a_predicate_or_a_location` cannot be answered before this one,
and its option three says so in its own text**: "A location for membership and a predicate for
hosting, **which is the shape Q31's split falls into**". So Q30's third option is Q31's answer
wearing Q30's clothes. Answering Q31 for two words does not settle Q30, because Q30 still has
to say whether the *first* word returns a coordinate or a boolean. But it does remove Q30's
third option from the ballot as an independent choice: it is now a consequence, not a rival.
Whoever takes Q30 next should be told that.

**7.3 An open trait whose obligation refuses more than the canon does.**
`arvo-format/src/slots.rs:110-113` tells an outside implementor the trait is open and that what
they owe is `ADMITTED`, and `arvo-format/src/slots.rs:117-122` says the same thing again in a
`#[diagnostic::on_unimplemented]` note, which is the message a stranger actually reads when
their impl is refused. `arvo-format/DESIGN.md.tmpl:599-601` cites
`proposal::the_concept_is_closed_and_the_inventory_is_open` as the licence for that openness.
The ratified row says a new instance joins "by supplying **the concept's** obligations". A
62-bit ceiling is not one of the concept's obligations under any reading, and it is exported to
every implementor at build time. The `dev` files found this and they are right that it is a
real defect. My only disagreement is section 6.2's, about what fixes it.

**7.4 Nothing unlicensed in the mechanism itself.** The verdict-plus-const pairing, the
permanently-kept wrong constructions, the `compile_fail` doctests and the separation arms are
all straightforwardly what the workspace's own test discipline asks for, and better executed
than most. Saying so is a result and I would rather say it than leave the section one-sided.

## 8. Predicates

**F1. One word contradicts the ratified canon.**
*Holds for: the registry at `26f65faf`; `ruling::the_format_spine_is_canon` at rung ratified;
`ruling::the_operating_constraints_are_intents_and_rules` at rung in_force; ambient domain in
{the rationals, the reals}; target features any, by construction, the argument reads no target;
threads any, by construction, it is a reading of committed text; toolchain any, by
construction, same; container any; radix any; signedness any.*

**F2. No hosting refusal in the shipped crates varies by target.**
*Holds for: the tree at `26f65faf`; the three crates `arvo-format`, `arvo-placement`,
`arvo-strategy`; every non-test `.rs` file in them, 13 files; the pattern set named in
`is_the_hosting_predicate_target_indexed.sh`; target features any; toolchain =
rustc 1.98.0-nightly (57d06900f); threads = 1; build profile = dev, debug-assertions on.*

**F3. Nine `ADMITTED` assertions split six concept to three hosting.**
*Holds for: the tree at `26f65faf`; the four `ADMITTED` blocks in `arvo-format`; classified by
exhibiting the refused candidate, two of the nine run through the shipped verdict functions and
seven classified by reading; threads = 1; target features any; toolchain = rustc
1.98.0-nightly (57d06900f); build profile = dev.*
Weaker than F1 and F2 and marked so: seven of the nine rest on my reading of what a refused
candidate denotes, which is a judgement. The two that do not are the two the probe runs.

**F4. Deleting `Slots::ADMITTED`'s `WIDTH <= 62` clause leaves the suite green.**
*Holds for: the tree at `26f65faf`; `cargo test -p arvo-format`, 134 tests including doctests;
the mutation named in `does_any_test_isolate_the_62_bit_clause.sh`; toolchain = rustc
1.98.0-nightly (57d06900f); build profile = dev, debug-assertions on; threads = 1; target
features any.*

**F5. The construction at `MIN = 0`, `MAX = 255`, `WIDTH = 63` separates the crate from that
mutant.**
*Same region as F4.*

## 9. What I did not settle, and what would close each

- **What the second word is spelled.** A naming call and `topic::naming` says so. Candidates in
  section 5; op's, or a panel's with op's stamp.
- **Whether the first word returns a coordinate or a boolean.** That is Q30 and this answer does
  not reach it. What it does is retire Q30's third option as a rival, per 7.2.
- **Whether hosting *should* become target-indexed later.** F2 says it is not today. Whether the
  stack ever intends a target-dependent numeric surface is a design question about a tree that
  currently has none, and it is not Q31's to answer. If the answer is ever yes, section 4's
  second argument still stands: the predicate notation already carries `target_features` and the
  vocabulary should not grow a second mechanism for it.
- **Whether `Quantum::ADMITTED`'s `reach_is_representable` clause is isolated by any test.** I
  measured `Slots`'s width clause and not this one, and it is the same shape: a hosting bound
  inside an obligation, with `ReachesPastTheExponent` as its named construction. That
  construction sets `SLOPE = i32::MAX` with `MAGNITUDES = 4`, which fails only the reach clause,
  so I expect it **is** isolated, and expecting is not measuring. The mutation is four lines and
  the next seat in that surface should run it rather than take my expectation.

## 10. Coverage, and what to distrust

I read every `[[question]]` row on `topic = "the_number_system"`, both `[[ruling]]` rows on it,
all four proposals `ruling::the_format_spine_is_canon` ratifies, `dimension::ambient_domain`,
`topic.toml` entire, `arvo-format/src/{ambient,slots,quantum,format}.rs`, all twelve tests in
`obligations.rs` and the admission tests in `the_inventory.rs`, sections 3.4, 5 and the Q20 to
Q31 register block of `74`, section 5 of `68`, and the Q31 passages of `241` and `246`.

**Distrust first:** F3's seven read cells, per its own marking. Then my reading of `68` section
5's residue clauses, which I took from one file and did not cross-check against `66`, which it
credits. Then section 7.2's claim about Q30, which is an inference from that row's own option
text and not a measurement.

**What I carried forward unchanged, and from whom.** `proposal::membership_and_hosting_are_two_questions`
(from `74`'s N10, `73`'s), whose conclusion I agree with and whose warrant I replaced rather
than seconded, per section 0. `246`'s measurement that the hosting predicate is
implementation-indexed rather than target-indexed, replicated independently here at wider scope
rather than cited. `241`'s observation that the two words are of different types, which I
reached separately and which section 5 states in my own terms. `68` section 5's three residue
clauses, unchanged and uncontested.
