# The Concept, The Designs And The Way Of Working, Part By Part

Part of the stack audit; `AUDIT.md` beside this file is the entry. This file judges the direction and
each major design, with the code read as evidence that the design can be built and not as the thing
being judged. `NUMBERS.md` holds the benches and the estimates, and `DOWNSTREAM.md` the fit to
consumers.

Each part ends in one of the four answers the brief asks for (passes in intent and needs only
implementation; can be fixed in place; going wrong; back to the drawing board) and carries one of the
four marks in words. A registry row is cited by its slug. A path with a line is at `dev` tip
`326b52f5`. Where a statement rests on a reader's notes and I did not open the evidence myself, it
says so.

The order is the concept first, then the crates from the bottom up, then the root documents, then the
canon and the way the work is run.

## The concept

### A format is a domain and a representable set, and arithmetic is an exact step and a named adaptation

This is `ruling::the_format_spine_is_canon`, and the README states it in four paragraphs
(`README.md:29-35`). A format is the ambient domain and the set of values it can hold, membership is
one predicate with a slot, a quantum and a phase, and every operation is the exact operation in the
ambient domain followed by an adaptation that is a named pair of a rounding mode and an overflow
policy.

It holds. It is the factoring the hardware fixed-point world arrived at (a word length, a fraction
length, and a quantisation and overflow mode beside them) made general enough that integers, binary
fixed point and scaled integers at any radix are the same object, and it is what lets the tree say
which laws survive which adaptation as a computed verdict (`addition_is_associative`,
`mock/crates/arvo-format/src/addition.rs:363-383`) instead of as folklore. Six explicit rounding names
with the ambiguous word retired (`ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names`)
is a small decision that will save every consumer a bug. The code shows it can be built: addition,
negation, subtraction, the product, the quotient and conversion all exist at the format tier as
`const fn` against an exact oracle with an independent rational arithmetic as the test oracle
(`mock/crates/arvo-format/src/tests/big.rs`, `ratio.rs`; reader's notes).

One claim inside it is not yet earned. The README says floats fall out as points of the same shape
(`README.md:29`). A `Floating` format is declared (`mock/crates/arvo-format/src/lib.rs:220`), and no
operation admits it: every one of addition, negation, subtraction, product, quotient and conversion
refuses a quantum that depends on the magnitude, each with a ticketed marker
(`mock/crates/arvo-format/src/addition.rs:80`, `multiplication.rs:74`, `division.rs:184`,
`conversion.rs:74`). Whether IEEE 754's values that are not numbers on a grid, the signed zero, the
infinities and the payload-carrying non-numbers, fit one affine predicate is the test of the claim, and
nothing built says either way. I do not know the answer.

Answer: passes in intent and needs only implementation, for everything with a constant quantum. Mark:
good as it is. For floats the mark is easy to get wrong but can be right, and right requires one float
operation admitted at the format tier before anything above it is designed around the claim.

### Behaviour is stated over the declared signature, and where the bits sit is a separate question

`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` and
`ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`. A value of thirteen
bits wraps at thirteen bits whatever integer holds it, and the choice of that integer is a placement
that no operation reads.

It holds, and it is the part of arvo that most plainly beats what a consumer gets from the host
language. The placement design's one-sentence case for it is exact: a numeral whose behaviour is
stated over the container returns 8192 where a thirteen-bit wrap is 0
(`mock/crates/arvo-placement/DESIGN.md.tmpl:24-27`). The dependency direction enforces it without a
rule: the format crate cannot name a carrier because it does not depend on the crate that has one
(`mock/DESIGN.md.tmpl:28-32`).

Answer: passes in intent. Mark: good as it is.

### Never a run-time check, one lowered path, and a debug abort the consumer names

`ruling::never_a_runtime_check_and_one_lowered_path`, `ruling::the_overflow_panic_is_permitted_and_bounded`
and `ruling::the_debug_abort_is_configured_by_the_consumer`. Every operation is total, an invalid
construction is a failed constant at the consumer's line, and whether a debug build aborts on a result
that leaves the range is the second type parameter of the value.

The three rows read as if they pulled against each other and the built design shows they do not: the
abort asks a verdict outside the total operation, under a branch that is a constant false in a release
build and under `Quiet` (`mock/crates/arvo-numeral/src/num.rs:179-186`), and a type parameter keeps
one type one behaviour where a cargo feature would be unified across a build
(`mock/crates/arvo-numeral/DESIGN.md.tmpl:172-179`). That is the right vehicle and the reasoning for it
is the best paragraph in the numeral design.

What is easy to get wrong sits at the edge of a program. A scheduler whose sizes are all constants can
live with no run-time validation. A daemon reading a length off a socket cannot, and neither can
anything that parses a number. The design's answer to that is correct and is nowhere written as the
answer: the verdict is public (`fits_add`, `num.rs:208`; `slot_in_range`,
`mock/crates/arvo-format/src/slots.rs:314`; `panic_on_overflow`, `apply.rs:291`), so a consumer asks
it and branches itself, which is `ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up`.
Until the documents say "ask the verdict, then build the value" as the pattern, and until there is a
verdict a consumer can ask of a host integer at the carrier's width without going through 128-bit slot
indices, a reader will take "never a run-time check" to mean arvo has nothing for untrusted input.

Answer: passes in intent, with the edge pattern to be written down and given a cheap door. Mark: easy
to get wrong but can be right.

### Arms over regions, selected by a const predicate

`ruling::arms_over_regions_are_the_fundamental_heart`, `ruling::the_work_is_predicated_arms_composed`
and `ruling::there_is_no_exchange_rate_because_there_is_no_generalisation`. No universal answer; many
small implementations, each holding over a region it names and selected at compile time.

The committed evidence is on its side. The saturating-fold family shows a const gate costing nothing
and the arm behind it worth a factor of 27 to 160 where its law holds, and the carrier-width table
turning its sign between two hosts shows that a single answer per category would be wrong on one of
them (`NUMBERS.md`, the fold and the packing families).

It is also the part with the least built. The only arms in the tree are `derive_sole` and
`derive_shared`, and the first reads its objective only to discard it
(`mock/crates/arvo-placement/src/lib.rs:217-229`). No arm is keyed on a target, a width region or a
length. The mechanism that is to pick between arms that disagree has, by the registry's own row, never
run on arms that disagree (`probe::the_argmin_mechanism_has_never_run_on_arms_that_disagree`, standing
uncontrolled; reader's notes). And two costs of "a million small implementations" are not priced
anywhere: what it does to build time, and how an arm that was right on the machine that measured it is
kept from being selected on a machine where it is wrong. The second has an answer inside the concept,
since the target is a declared dimension a predicate may read, and no arm uses it yet.

One of the shared principles pulls against it in so many words. "No fast path alongside a slow path.
One mechanism per concern" (`mock/PRINCIPLES.md.tmpl:209-213`) comes from the fragment every repository
shares, and arvo's heart is many paths for one concern, selected statically. The two can be read
together, since the principle is about two mechanisms that both exist at run time, but the text does
not say so and a reviewer holding the principle will flag the arms.

Answer: passes in intent and needs implementation, of the first real arm most of all. Mark: easy to get
wrong but can be right. Right requires three things: the bench harness on more than one host before
any arm is selected by a timing, a statement of how a predicate reads the target, and the fold or the
deferred mask as the first arm above the value, because that is where the committed evidence already
is.

### The strategy axis

`ruling::the_strategy_set_is_not_closed_at_four` and its neighbours. A consumer selects what it
optimises for, the set is open, each named strategy is an intent and not a table, and all of them are
decided by measurement.

As an intent this passes. As built it is the weakest part of the tree, and the documents claim more
for it than exists. `Strategy` binds a name to a placement objective and an adaptation
(`mock/crates/arvo-strategy/src/lib.rs:41-55`). Three of the four presets bind the same objective
(`:91`, `:126`, `:143`), and that objective changes nothing at sole occupancy, which is the only
occupancy a value exists at. All four adaptations are placeholders marked as such (`:87`, `:105`,
`:122`, `:139`), with no ticket behind the markers, which is ticket `684a26a63155`. The value crate
does not depend on the strategy crate at all (`mock/DESIGN.md.tmpl:41`). So choosing `Hot` over
`Precise` today changes nothing any program can observe except which placeholder adaptation it gets.
Against that, the principles say the markers "are not decoration" and drive container width and
codegen (`mock/PRINCIPLES.md.tmpl:320-329`), and the root design opens on them
(`mock/DESIGN.md.tmpl:3-12`).

There is a question under the placeholders that the open rows circle without stating. A strategy that
selects the adaptation selects what the arithmetic means: `Hot` wrapping and `Precise` saturating
return different numbers from the same inputs, and the strategy design says so itself, calling the
result a different signature (`mock/crates/arvo-strategy/DESIGN.md.tmpl:153-156`). An objective, on
the other hand, is supposed to change cost and never the answer. Binding both under one name means a
consumer who changes its performance preference changes its results. Op's direction of 13 November on
the overflow default (`ruling::a_shipped_overflow_default_is_a_generic_overridable_per_case`: a default
that is a generic with a default, overridable per case, perhaps following a parity standard) reads
more naturally if the adaptation default is its own parameter and the strategy keeps to objectives.
Kaski is the live case: it wants the speed objective with half to even and saturation, which is one
preset's objective and another's adaptation. The open set rescues it, since kaski can write its own
binding, and then the four presets are examples and not the vocabulary the documents present them as.

The registry agrees that this is unsettled: by a reader's count the strategy axis and the strategy
object carry seventeen open questions between them, and the canon check found two proposals at two
experts each that disagree on what a strategy is
(`mock/research/202611130950_canon-double-check/CHECK.md`).

Answer: the intent passes; the binding as shipped goes back to the drawing board on one point, whether
a strategy names an adaptation at all, and the rest can be fixed in place. Mark: going wrong, in the
sense that the documents and the tree have drifted apart and the tree is the honest one.

### Parity with the standards

`obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`,
`ruling::the_standards_bound_starts_at_two_and_reserves_the_rest` and
`ruling::the_standard_is_parity_in_output_not_in_the_internals`. MATLAB `fi` and IEEE 754 first, as a
test of whether the primitives are adequate.

Using the standards as an adequacy test is right, and it has already paid once: the parity file is
where the tree learned that one MATLAB rounding method has no mode among the six (`README.md:21`).
`Fi`, `Ufi` and the `fimath` aliases exist (`mock/crates/arvo-format/src/standards.rs:58-98`). The IEEE
half is unbuilt, which follows from no operation admitting a float. And the suite is a reading of
documentation checked against arvo's own oracle, where op's words ask for the same computation run
through both.

A small thing that will confuse a consumer: the only signed fixed-point format with a name is the
MATLAB one. The native points are a signed integer, an unsigned fixed point, a biased one and a float
(`mock/crates/arvo-format/src/lib.rs:154-220`), so a consumer wanting plain signed fixed point reaches
for a vendor's vocabulary to get it.

Answer: passes in intent and needs implementation. Mark: easy to get wrong but can be right; right
requires an executable reference on the other side of the comparison at least once, even a small table
generated by the real tool and committed with how it was made.

## The crates

### `arvo-format`: the oracle

What it is: every format, every adaptation and every exact operation as `const fn` over 128-bit slot
indices and a four-limb wide integer, with obligations checked as constants.

As an oracle it is good. It is exhaustive at eight bits against an independent rational arithmetic,
the obligations refuse at the consumer's line, the inventory is open and tested as open from outside
the crate, and the three places the product is not yet carried are marked with tickets
(`mock/crates/arvo-format/src/multiplication.rs:74-95`).

Three things are going wrong around it and all three can be fixed in place.

The first is that the oracle is on a run-time path. `Num::adapted` calls `adapt` as it stands
(`mock/crates/arvo-numeral/src/num.rs:135`), so bringing an integer into a value runs 128-bit
rounding and completion, and no small accessor in the crate is marked inline, which the value-add
sketch already found and ticketed. `NUMBERS.md` has what that costs. The numeral design states the
right rule for the sum, that the carrier arithmetic is the mechanism and the oracle is the contract
(`mock/crates/arvo-numeral/DESIGN.md.tmpl:31-35`), and the rule wants stating for the crate as a whole:
nothing that runs calls the oracle, and every value operation is carrier arithmetic shown equal to it.
The product, the quotient and conversion are about to arrive at the value tier, and the cheapest way
to write each is to call the format's function. That would be correct and ten to fifty times slower
than the hand version by my reasoned estimate.

The second is the design document. It is 2122 lines, and a part of it tells the story of defects the
crate used to have where a specification would state what the crate does (a reader measured about
seven percent of the last 1200 lines; I read the first 900 and saw the same habit there). One document
for one crate cannot be held in the head at that length, the workspace's own rule for a design is that
it carries no history, and the source it governs is split into files under 500 lines while the design
is not.

The third is one red test. `the_inverse_of_three_round_trips_at_every_unsigned_width_from_one_to_63`
fails on `dev` with a subtraction overflow inside the test's own sample builder
(`mock/crates/arvo-format/src/multiplication/tests/the_inverse_round_trip.rs:37`), which is ticket
`eae41fda0fd6`. The other 297 tests of the crate pass. Two exhaustive sweeps take over a minute each in
a debug build, which the slow-test rule wants ticketed and ignored or made fast.

Answer: passes in intent; the three faults can be fixed in place. Mark: good as it is, as an oracle;
easy to get wrong as a dependency of anything that runs.

### `arvo-placement`: where the bits sit

The concept is good and stated above. As built it is a function from a signature to three widths and a
table from a slot family to one of four containers
(`mock/crates/arvo-placement/src/lib.rs:217-270`).

At sole occupancy this is complete and correct, and the trick of giving the derivation twice, once as
a width and once as a type, with a check that the two agree, is sound
(`mock/crates/arvo-placement/DESIGN.md.tmpl:89-103`).

At shared occupancy nothing holds a value. There is no column, so the footprint objective has no
effect on any program, and the design's own law that the access width is not a function of the carrier
cannot be exhibited by the one packing rule the crate ships, which the catalogued ignored test says
honestly (`mock/crates/arvo-placement/src/tests.rs:173`). The objective has two values, and the
committed benches say the worth of packing depends on the host, the access pattern and the column
size, none of which an objective of two values can express. That is not a defect of the crate if the
shared arm becomes several arms over those dimensions, which is what the concept says it should be.

Answer: passes in intent and needs implementation. Mark: good as it is at sole occupancy; easy to get
wrong but can be right at shared occupancy, where right requires the arms to be keyed on more than the
objective.

### `arvo-strategy`

Judged above under the strategy axis. Its laws section owes five tests
(`mock/crates/arvo-strategy/DESIGN.md.tmpl:143-162`); I did not check which of them exist.

### `arvo-numeral`: the value

`Num<S, C>` is a transparent newtype over the machine integer the placement derives, with every
operation a `const fn` defined as the format's operation on the slot.

The construction is sound and several of its decisions are better than they had to be. The admission
is a check and not a list, so a format of another crate is held on the terms the shipped ones are, and
the check is shown open from both sides by compile-fail pairs
(`mock/crates/arvo-numeral/DESIGN.md.tmpl:326-336`). The carrier cannot be named by a consumer. The
alias takes its adaptation through a projection so that a strategy default can arrive later without
breaking anybody (`:256-266`). The suite is exhaustive where it can be, has a planted mutant per
carrier width, and a consumer crate outside the workspace reaches every public item. The measured sum
is the host's sum. This is a crate a competent reviewer would pass.

Six things in it are going wrong or easy to get wrong. None needs the drawing board.

1. The doors. In is `constant` or `adapted`, out is `slot`, and for everything below pointer width
   that means a round trip through a 128-bit index and the oracle (`num.rs:108-157`). The design
   defends having no conversion from a host integer as a door the signature does not declare
   (`DESIGN.md.tmpl:227-229`). The semantics of that are right: an ingest must adapt, so it is
   `adapted`. The route is what costs, and a door that takes the host integer of the carrier's width
   and applies the declared adaptation as a mask or a clamp is the same function of the declared
   signature at a fraction of the price. `USize` and `ISize` already have bit-copy doors
   (`mock/crates/arvo-numeral/src/alias.rs:61-84`); the other widths have none.
2. `Hash` hashes the 128-bit slot index (`num.rs:296-300`), so a one-byte value feeds a hasher sixteen
   bytes. The reason given, that the hash must not depend on the carrier, is met as well by hashing
   the index at the declared width's own size.
3. No operator trait, no ordering trait. The stated reason for inherent functions is that a trait
   method cannot be called in a `const fn` (`DESIGN.md.tmpl:222-223`), which explains why the inherent
   ones exist and does not explain why `Add` is not also implemented by calling them. The stated
   reason for no `PartialOrd` is that its method answers in a bare `Option` (`:224-226`), while the
   principles name exactly that method as a permitted escape (`mock/PRINCIPLES.md.tmpl:254-258`) and
   `arvo-bits` derives both ordering traits (`mock/crates/arvo-bits/src/lib.rs:49`). The row behind it
   is `proposal::a_value_takes_no_ordering_trait_from_core` at one expert. A value that cannot be a key
   in anything sorted and cannot be passed where a bound says `Ord` will be wrapped by every generic
   consumer, which is the reinvention the stack exists to prevent.
4. Four cargo features on the shipped crate remove a carrier width's adaptation, for the mutant tests
   (`mock/crates/arvo-numeral/Cargo.toml:17-20`, read at `mock/crates/arvo-numeral/src/rung.rs:167-179`).
   A feature is unified across a build, which is this design's own argument against using one for the
   debug check. Any crate in a consumer's tree that turned one on would silently break arithmetic for
   all of them. The mutants belong behind something a dependent cannot switch.
5. The alias makes the consumer name the adaptation every time, `Int<8, Adapt<TowardZero, Wrap>>`
   (`DESIGN.md.tmpl:251-254`), because the default waits on a question that is open. That is honest,
   and until it closes every consumer writes its own alias, so the open question is costing ergonomics
   now.
6. Whether the container a width takes is fixed once a release ships is open
   (`DESIGN.md.tmpl:110-118`, under `question::what_a_datum_stands_for`). For anything that crosses a C
   boundary or is written to disk as its own bytes this is the question, and op's own row says
   everything ends up in a C boundary sooner or later.

What is not here is listed honestly (`DESIGN.md.tmpl:359-365`): subtraction, negation, the product,
lanes, the accumulator, a door for a value's bits.

Answer: passes in intent; the six can be fixed in place. Mark: good as it is for the construction and
the sum; going wrong at the doors, the hash and the mutant features; easy to get wrong but can be
right for the traits and the layout promise.

### `arvo-bits`

One type, `Bits<const N: u32>` over a `u64`, with masked construction, a raw door and a masked cast
(`mock/crates/arvo-bits/src/lib.rs:48-130`). The suite is small and has a real independent oracle and a
control on the oracle (reader's notes).

Two things. It always takes eight bytes, whatever `N` is, in a workspace that has a crate whose whole
subject is deriving the narrowest carrier; `Uint<13>` sits in sixteen bits and `Bits<13>` in
sixty-four. The obligation it answers prices the consumer's own use at a 28-bit value in memory and 32
bits on disk (`obligation::an_exact_width_container_a_consumer_can_alias_and_pin`), so the answer to
that obligation doubles the consumer's handle. And it has no set operation, so
`obligation::set_operations_over_a_fixed_size_bit_set`, the one obligation that names an instruction
count, is not started.

Answer: can be fixed in place, by deciding the carrier on purpose and saying why. Mark: good as it is
as a field in a register; going wrong as something a table holds many of.

## The root documents

`mock/PRINCIPLES.md.tmpl`, `mock/DESIGN.md.tmpl`, the README and the generated agent instructions
disagree with the tree and with each other, and the canon check of 13 November lists most of it. The
ones that would mislead a reader about direction:

| Where | What it says | What is true |
|---|---|---|
| `mock/PRINCIPLES.md.tmpl:5` | Four crates | Five (`mock/DESIGN.md.tmpl:28`) |
| `mock/DESIGN.md.tmpl:3-6` | Numeric primitives and analysis algorithms with four strategy markers | No algorithm ships, and the set is open |
| `mock/DESIGN.md.tmpl:21-24` | Every sensible trait is a const trait | No const trait is in the tree, and the crates enable no feature |
| `mock/DESIGN.md.tmpl:38` | Addition is the one admitted operation | The format tier has six |
| `mock/PRINCIPLES.md.tmpl:71-73` | The fixed-point and exact-width primitives are not there yet | Forty lines on, the same file describes `Num` |
| `mock/PRINCIPLES.md.tmpl:295-299` | The lint has not landed | It is in the lint set |
| `mock/PRINCIPLES.md.tmpl:320-329` | Strategy markers drive width and codegen, and default to `Warm` | They drive nothing a value does, and no alias has a default |
| The generated agent instructions | The tree holds the bottom three crates and no numeral | Five crates, one value type |

The README is the accurate one. It says what exists, what does not, and that this is the bottom of a
stack and not a usable library (`README.md:19-25`).

Answer: can be fixed in place, and is step two of the canon check's plan. Mark: going wrong, cheaply
repaired. It matters more than its size because the agent instructions are what every session working
in the tree reads first, and they describe a tree two crates smaller than the one in front of it.

## The canon and the way the work is run

This is where the audit has the most to say about direction, and it is said with the caveat that I
read the governing tier and the recent research myself and took the shape of the older research from
a reader.

The registry is a good idea carried a long way. Typed rows with rungs, op's words kept verbatim,
retirements that stop a refuted claim from being cited again, obligations enumerated from the
consumer's side, and probes with a standing that includes "uncontrolled" and "defective": this is more
honest about what is known than any design corpus I can compare it to, and the canon check found no
two ratified rows contradicting each other.

What the numbers say about the yield is harder. By a reader's count the registry holds 106 rulings of
which 39 are ratified, 110 questions of which about 85 have no answer, 159 proposals of which 124 rest
on one expert, 196 retirements and 116 probes. The design-round directory holds 184 rounds, 71 of them
in September and by their scope lines almost all edits to one crate's design document. The first round
is dated 19 April and the tree was cleared on 8 August. Against that, what a consumer can depend on
today is one value type with a sum and a comparison. The reader's sorting of the panel's later files
by title put most of them on the registry and the process and not on numerics.

Two things in the same record point the other way and deserve the same weight. The work of 12 and 13
November produced the value crate, and the product, the quotient and conversion at the format tier, in
two days, on top of the canon as it stands. So the canon is able to carry fast implementation. And
the canon check produced a twenty-step plan from the registry alone
(`mock/research/202611130950_canon-double-check/PLAN.md`), which is what op's bar for the canon asks
of it (`ruling::the_canon_must_support_a_full_design_and_impl`).

So the reading is that the concept did not need more settling than it had by September, and the months
between went to making the record of the concept consistent with itself. The eighty-five open
questions are mostly not blocking anything: the value crate was built across several of them by taking
a position and saying so (`mock/crates/arvo-numeral/DESIGN.md.tmpl:110-118`), and that is the pattern
that works.

Three specific things are going wrong.

The plan measures last. Its value benches are step 19 of 20 and the port of the one real consumer is
step 20. Every finding in this audit that came from running something (the doors, the per-step mask in
a fold, the latency of a rounding mode) was available on the day the value crate landed and would
change how steps 4 to 8 are written. Op's own row says once benches exist it is hard to deny what they
tell (`ruling::do_not_escalate_a_measurement_dispute`), and the strategies are all to be decided by
measurement. The measuring wants to be beside the first operation, not after the last.

The benches do not measure the tree. This is in `NUMBERS.md` and is repeated here because it is a
direction finding: seven months of bench work produced a corpus in which no arm touches a shipped
type, and a registry in which rows about cost rest on one machine.

The consumer is consulted through documents and not through code. The obligations were written by
reading hilavitkutin's design text. Its source says something its design text does not: it takes the
old arvo's platform integer apart with a field access and does host arithmetic about five hundred
times, under 286 lint escapes (`DOWNSTREAM.md`). That is the most important fact about what the
consumer needs, it is in no obligation row, and porting one hot module onto the new value would have
found the doors on the first afternoon.

Answer: the canon as an instrument passes; the order of work can be fixed in place and should be. No
course change in the concept. Mark: good as it is for the registry's honesty; going wrong for the
ratio of record to product and for measuring last.
