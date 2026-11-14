# The Concept And The Designs, Part By Part

Part of the second stack audit; `AUDIT.md` beside this file is the entry and holds the summary, the
ledger of what was read, and what became of each finding of the audit of 13 November. This file
judges the direction and each major design. The code is read as evidence of whether a design can be
built and never as the design. `NUMBERS.md` holds the benches, the spikes and the estimates,
`BENCH-CATALOGUE.md` the benches to write, and `DOWNSTREAM.md` the fit to consumers.

Paths and lines are at `dev` tip `d0161800`. A registry row is cited by its slug and by the line of
its `id` in the file. Every part ends in one of four marks, in words: good as it is; easy to get
wrong but can be right, with what right requires; going wrong; back to the drawing board. A number
called an ad-hoc spike is from `mock/research/sketches/202611140930_stack-audit-2-value-tier/`, one
shared x86_64 virtual machine, and is not a bench result.

## The concept

### A format is a domain and a representable set; an operation is an exact step and an adaptation

`ruling::the_format_spine_is_canon` (`mock/registry/ruling.toml:1479`, ratified).

A day ago this was a claim with one operation behind it at the value tier. It now has six. The
value's sum, difference, negation, product, quotient and conversion are each defined as the format's
own function on the slot, and the contract is an equality with it on every member
(`mock/crates/arvo-numeral/DESIGN.md.tmpl:47-53`). The two standards op asked for as "one of the
first things to write" (op-answers-36 on the kaski channel, 13 November) are declared from the
fundamentals alone: MATLAB `fi` and `fimath`, and five IEEE 754 interchange formats
(`README.md:21`, landed as `085033a4`). That the canon could carry this much implementation in one
night, with no row of the spine amended to do it, is the strongest evidence yet that the factoring
is right.

It is also the part of arvo with no equal that I know of in what a consumer can get elsewhere: the
rounding mode and the overflow policy are coordinates of the type, six rounding names with the
ambiguous one retired (`ruling.toml:1724`), and a verdict on which laws survive an adaptation that
is computed and not asserted.

One clause inside it is still not earned. Floats are said to be points of the same shape
(`README.md:29`). The five IEEE formats are declared, and nothing computes over one: the source
carries tickets saying no arithmetic runs through both sides of the IEEE parity
(`arvo_no_arithmetic_runs_through_both_sides_of_the_ieee_754_p`), nothing puts a value onto an
interchange format (`arvo_nothing_puts_a_value_onto_an_ieee_754_interchange_forma`), the infinities
and the signed zero have no member (`arvo_the_ieee_754_infinities_and_signed_zero_have_no_member`),
and sum, product, quotient, conversion and the member type each refuse a quantum indexed by
magnitude (four tickets covering the five; `grep -rn "FIXME(" mock/crates/arvo-format/src`). The honest state is that
the grid of a float is expressed and the float is not. Whether a signed zero, two infinities and
the values that are not numbers fit one affine predicate over a domain is the test, and it has not
been run.

Mark, for everything with a constant quantum: good as it is. Mark for floats: easy to get wrong but
can be right. Right requires one operation admitted over a magnitude-indexed quantum at the format
tier, with the special values given members or refused by name, before a float value or anything
above it is designed around the clause.

### Behaviour is stated over the declared signature, and the container is a separate question

`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` (`ruling.toml:1503`)
and `ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`
(`ruling.toml:1578`), both ratified.

It holds, and the night's code is a harder test of it than the first audit had. The host door
shows the rule surviving contact with host integers: a 13-bit unsigned value gives its index as a
`u16`, a `u32` or an `i64`, whichever the range fits, takes one from any of ten host types, and
neither door names the carrier (`mock/crates/arvo-numeral/DESIGN.md.tmpl:256-274`,
`mock/crates/arvo-numeral/src/host.rs:88-174`). The crossing between two signatures over one format
moves no bits and is a type-level act only (`num.rs:191-199`). In the ad-hoc spike the ring
operations below the carrier's width time the same as the hand loop that masks or clamps to 13 bits
in a `u16`, so stating behaviour over the declared width costs nothing over stating it over the
container where the container is wider.

Mark: good as it is.

### Never a run-time check, one lowered path, and what that means at the edge of a program

`ruling::never_a_runtime_check_and_one_lowered_path` (`ruling.toml:980`, ratified),
`ruling::the_overflow_panic_is_permitted_and_bounded` (`ruling.toml:1088`, ratified),
`ruling::the_debug_abort_is_configured_by_the_consumer` (`ruling.toml:1912`, stated), and, new since
the first audit, `ruling::the_ingest_door_waits_on_a_known_use` (`ruling.toml:1992`, stated).

Inside a program this is right and is built right. Every operation is total, an invalid
construction is a failed constant at the consumer's line, and the debug abort is a type parameter
that asks a verdict outside the total operation (`DESIGN.md.tmpl:198-236` of the numeral crate).

At the edge, the first pass of this audit said the design had moved the wrong way and that a stated
row worked against a stated goal. The second pass takes most of that back; what follows is the
corrected reading.

The first audit said the pattern for untrusted input is "ask the verdict, then build the value",
and that the verdict for a host integer was missing. A direct verdict now exists, `holds_index`, at
64 bits and as cheap as it can be said (`host.rs:127-140`). It is private, on purpose. The design
says "no function says whether a
host integer names a member: that would be a membership verdict on a datum from outside, which
`ruling::the_ingest_door_waits_on_a_known_use` leaves unblessed until a use for it is known"
(`DESIGN.md.tmpl:267-270`).

Read alone, that sentence says arvo has nothing for input. It has, by composing the two public
doors. `from_index` completes by the declared policy and `index` gives the index back and never
truncates (`host.rs:88`, `:147`), so `Num::from_index(h).index::<H>() == h` is true exactly when
`h` names a member, under either policy, wherever `H` holds the range. The value must name the `Quiet` check, since under `Abort`
a debug build aborts first (`host.rs:89-96`). A second public route is the format's `contains` on a
slot (`mock/crates/arvo-format/src/format.rs:160`). Neither restates a bound the type knows, and
arvo checks nothing: the consumer asks and branches. The second pass added the round trip to the
ad-hoc spike to test this: it agrees with a hand range test on every input under both policies, it
times the same as the hand test under fat link-time optimisation, and in a default release build
it carries the door's call, 26 to 32 times the hand test.

So op's doubt in the row's own quote, "I am not sure it's actually ever needed if the typestate
works as intended and designed" (`ruling.toml:1998`), is borne out for ingest further than my first
pass allowed. A daemon that reads a length from a socket, an application that parses a quantity, a
save file kaski loads: each can reject in arvo's vocabulary today. What is wrong is smaller and
still real. The pattern is written nowhere, the one sentence an author finds says no function gives
the verdict, and op has since confirmed the consumers who will look for it: "Yes, general numerics
too" (op-answers-26, 13 November), "I've answered already" (op-answers-36).

That the consumer asks and branches is also what
`ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up` (`ruling.toml:1019`) describes.

The divisor is one step further in, and there the row does bite. A value cannot be divided by a
value today (`DESIGN.md.tmpl:330-335`). The design says the membership type for the non-zero slots
"is the format's to ship, and the value takes it when it lands". It landed two commits before the
value's operations did: `Member<G>` with `Member::adapted`, total by the applied map, and
`Divisor::member` from a member of a range that leaves the zero out
(`mock/crates/arvo-format/src/member.rs:99-114`, `division.rs:172-188`, commit `b687fc01`). That is
a typestate answer to division by zero with no verdict in it: the run-time slot is completed into a
zero-free range by a declared policy, so what a zero divisor becomes is written in the type. It is
op's "Explicit per call" (op-answers-6) to the letter, and it is good. The value does not take it
yet, so the design's sentence is stale by a day, and through `adapted` it would pay the wide step.

A zero-free range lies on one side of zero, so a divisor of either sign has no door at all, and the
source asks the question itself: "Whether signed division is the known use the runtime membership
door waits on is the open question" (the ticket at `division.rs:121-125`). kaski's carrier is signed
and divides by run-time values (`DOWNSTREAM.md`). So a known use is on record in the tree's own
words, for the one case typestate does not reach, and it has not been put to op.

Mark: easy to get wrong but can be right. Right requires: one paragraph in the numeral design naming
the pattern for untrusted input (the round trip, or the private verdict published if the round trip
is not a compare in a default build); the value taking the member divisor on carrier arithmetic;
and the signed run-time divisor put to op as the known use the row waits on. Wrong looks like
"never a run-time check" continuing to read as "arvo has nothing for input".

### Arms over regions, selected by a const predicate

`ruling::arms_over_regions_are_the_fundamental_heart` (`ruling.toml:1523`),
`ruling::the_work_is_predicated_arms_composed` (`ruling.toml:872`),
`ruling::there_is_no_exchange_rate_because_there_is_no_generalisation` (`ruling.toml:1703`), all
ratified.

The first audit said nothing in the tree picks between two implementations by a predicate over a
region. That no longer holds, and what replaced it is instructive in both directions.

`Num::mul` is an arm. Its predicate is a constant of the instantiation, whether the quantum's
exponent is zero (`num.rs:379`). Where it holds, the product is the rung's carrier arithmetic; where
it does not, the product is the format's function on slots (`num.rs:380-403`). The rung's sum is
another, on whether the declared width is below the carrier's (`rung.rs:91-100`). Selection costs
nothing: in the ad-hoc spike the integer arms time the same as hand code with the branch gone. So
the mechanism the concept rests on is shown working in shipped code, on x86, without a bench.

The same arm shows the hazard the concept has not yet priced. The two sides of that one predicate
differ by more than two orders of magnitude in the ad-hoc spike: an integer product is the host's,
0.23 ns, and a fixed-point product, which by hand is 1.1 ns, is about 350 ns. The numeral design
says the second arm is slow (`DESIGN.md.tmpl:312-317`). Nothing a consumer meets at the call site
does: not the type, not the alias's documentation (`alias.rs` has no word on it), not the build. `Fix<32, -16, _>` is offered as "what a consumer with a deterministic lane
writes" (`DESIGN.md.tmpl:471`) and lands on the slow side with no signal. A design made of many
small implementations selected silently needs, as part of the design, a way for the consumer to see
which implementation it got and what that one costs. The registry has the measurement side of this
as an open row: `probe::the_argmin_mechanism_has_never_run_on_arms_that_disagree`
(`mock/registry/probe.toml:1119`, standing uncontrolled).

Two further costs remain unpriced, as the first audit said: what a very large number of arms does
to build time, and how an arm that won on the machine that measured it is kept from being selected
on a machine where it loses. The second has its answer inside the concept, since the target is a
dimension a predicate may read, and no arm reads it yet. The committed benches say it matters: one
family turns its sign between the two hosts it has been run on (`NUMBERS.md`).

The shared principle "No parallel versioning tables, no parallel serialization paths, no fast path
alongside a slow path. One mechanism per concern" (`mock/PRINCIPLES.md.tmpl:233-237`) still reads as
though it forbade arms, and still does not say that an arm, of which one survives lowering, is not
its subject. With a fast and a slow arm now side by side in `Num::mul`, the sentence is worth
adding.

Mark: easy to get wrong but can be right. Right requires: every arm that ships has a bench of each
side through the shipped type before it ships; the consumer can ask which arm an instantiation
takes; and the first arm keyed on the target is written against a table from at least two hosts.

### The strategy axis

`ruling::the_strategy_set_is_not_closed_at_four` (`ruling.toml:539`, open),
`ruling::hot_may_sacrifice_soundness_for_a_proven_meaningful_gain` (`ruling.toml:665`, ratified),
`ruling::the_strategies_weigh_measurements_differently` (`ruling.toml:756`, ratified), and, new,
`ruling::a_value_alias_is_keyed_on_its_strategy_and_defaults_to_warm` (`ruling.toml:2006`, stated).

The intent passes, and the binding of a strategy to an adaptation is op's own and is defended: the
strategies "behave differently and have their own purpose, intent, that shapes what the answer is"
(`ruling.toml:676-679`).

As built, nothing has changed since the first audit. `Strategy` binds a name to an objective and an
adaptation (`mock/crates/arvo-strategy/src/lib.rs:42-56`). Three of the four presets bind the same
objective (`:95`, `:132`, `:150`), the objective changes nothing at sole occupancy
(`mock/crates/arvo-placement/src/lib.rs:217-229`), all four adaptations are carried defaults under
one ticket (`lib.rs:89`, `:108`, `:126`, `:144`), and the value crate does not depend on the
strategy crate (`mock/DESIGN.md.tmpl:47`). The night did the groundwork for the binding to arrive:
the alias reads its adaptation through a projection so a strategy can name it later without
breaking a consumer (`mock/crates/arvo-numeral/src/alias.rs:20-41`), and one coordinate can be
overridden at a site with `with_overflow` and `with_rounding` (`num.rs:201-224`), which is op's
"easy to override per case" (`ruling::a_shipped_overflow_default_is_a_generic_overridable_per_case`,
`ruling.toml:1928`) built as he worded it.

On `Hot` I part from the first audit in the mark and keep its finding. The first audit marked the
shipped `Hot` as going wrong because it names a wrapping, toward-zero adaptation with no gain
measured. The cell is marked in source as carried and unproven, with a ticket
(`lib.rs:89-92`), no value reads it, and no consumer can observe it. A placeholder that says it is
one is not going wrong. What the finding becomes is a statement about what must be true before the
cell is claimed, and here the two ad-hoc spikes have something to say, on hand-written code, one
x86 host:

- Wrapping against saturation is a real gain in some regions and none in others. A 32-bit product
  is 0.23 ns wrapping against 0.58 to 0.71 saturating; a 13-bit sum elementwise is level; a
  wrapping fold that may defer its mask is ten times a saturating fold, and one that may not is
  level with it. So the overflow half of `Hot` has a provable gain to point at only per region,
  which is what the arms concept predicts and what a single cell cannot say.
- Toward zero is not the cheap rounding. In the first audit's spike the dependent chain cost 1.5 ns
  a step for floor, 1.7 for half up, 3.2 for toward zero and 3.1 for half to even
  (`mock/research/sketches/202611131300_stack-audit-value-yardsticks/FINDINGS.md`). On a signed
  domain toward zero needs the sign and the remainder, floor needs neither. If those figures hold
  under a real bench, the rounding half of `Hot` as carried is dominated: floor is faster, and half
  up is nearly as fast and far less biased.

Neither is a measurement. Both say the same thing: the cells of the presets cannot be filled by
argument, and the bench that fills them is step 19 of 20 in the plan
(`mock/research/202611130950_canon-double-check/PLAN.md:125`) while the step that needs them is
step 4 (`PLAN.md:114`).

Mark: easy to get wrong but can be right. Right requires the value-tier bench of wrap against
saturate and of the five deterministic modes, on two hosts, before any preset's cell loses its
ticket; and the threshold inside "meaningful", which the row's own `gap` says is unset
(`ruling.toml:684`), set once the first table exists.

### Parity with the standards

`ruling::the_standards_bound_starts_at_two_and_reserves_the_rest` (`ruling.toml:1561`, ratified this
week), `ruling::the_standard_is_parity_in_output_not_in_the_internals` (`ruling.toml:1603`, stated),
`obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`
(`mock/registry/obligation.toml:196`).

Using the standards as an adequacy test of the primitives is right, and it has now paid twice: the
MATLAB file found a rounding method with no mode among the six, and the IEEE work found the same
for `roundTiesToAway` (`README.md:21`). Both are answered by saying how a caller writes it, which is
what an open inventory over a closed concept should do.

What the parity suites check, where their expected values come from, and what they do not reach is
in the section on `arvo-format` below. In one sentence: rounding onto the binary float grids is
checked against the host's own floats, which is the executable reference the first audit asked for;
the decimal and MATLAB sides are checked against rules written by hand from the documents; and no
arithmetic runs through either standard yet. The plan's step 6, the standards through the value,
is not started.

Mark: easy to get wrong but can be right. Right requires the arithmetic through both sides once an
operation admits a float, and for MATLAB one small table produced by the real tool and committed
with how it was made.

## The crates

### `arvo-numeral`: the value

A transparent newtype over the machine integer the placement derives, every operation a `const fn`
defined as the format's operation on the slot (`mock/crates/arvo-numeral/src/num.rs:56-57`).

This is where most of the night landed, and most of it is good. Taking the first audit's six
findings in turn:

1. The doors. Fixed in design and almost fixed in code. `from_index` and `index` are stated over
   the slot index, take and give any of ten host integers, never name the carrier, and compute at 64
   bits with the format's exact step off the path (`host.rs:1-11`, `:88-125`, `:147-174`). Under
   fat link-time optimisation `from_index` is the hand clamp or mask in the ad-hoc spike. Without
   it, it is 27 to 45 times the hand loop across two runs, for one reason: `signed` and `unsigned` at
   `host.rs:49-63` are private, not generic and carry no `#[inline]`, so a consumer's loop calls
   into the numeral crate once per element. The assembly is committed with the spike. Two
   attributes should close it (not tried, since this audit changes no crate), and the gap is known in general: the lint that enforces inlining covers
   `arvo-format` alone and carries a ticket for the other four crates.
2. `Hash` still hashes the 128-bit slot index (`num.rs:435-439`). Holds, unchanged.
3. No operator trait and no ordering trait. Holds, and the design now states it at more length
   (`DESIGN.md.tmpl:360-372`). The reason given for no `Ord`, that core's `partial_cmp` answers in a
   bare `Option`, is still contradicted by the principles, which name exactly that method as a
   permitted escape (`mock/PRINCIPLES.md.tmpl:278-281`). The inherent `cmp`, `lt`, `min` and `max`
   are there and are the host's in the spike (`order.rs:14-82`). The contracts crate that would let
   a consumer write a function generic over a numeral is step 5 of the plan and not started.
4. The mutant features. Fixed. They are compiler configuration flags a dependent cannot switch
   (`mock/crates/arvo-numeral/Cargo.toml:26-37`).
5. No default adaptation. Holds (`alias.rs:6-13`), by a row that is open until both readings are
   priced (`ruling::wrap_or_clamp_stays_open_and_both_get_priced`, `ruling.toml:1074`).
6. The layout promise. Holds as an open question the crate takes a position under
   (`DESIGN.md.tmpl:136-144`).

What is new and good: the difference, the negation and the integer product as carrier arithmetic
with the unsigned saturating cases handled where the adaptation would get them wrong
(`rung.rs:134-153`, `:184-191`); the order; the crossing between sibling signatures; identities
computed from the format's own predicate; the value as the count of a type-level list behind an
opt-in feature (`DESIGN.md.tmpl:400-431`). The suite holds each operation against the format's
oracle, exhaustively at widths 1 to 8, with planted mutants per operation. In the ad-hoc spike
every value arm agreed with its hand arm on every input before timing, which is a small independent
check that the equality holds at 13 and 32 bits too.

What is new and going wrong is one thing, and it is the largest finding of this audit.

The fixed-point product, the quotient and the conversion call the format's exact step at run time.
`Num::mul` at a non-zero exponent is `arvo_format::mul` on two slots (`num.rs:396-403`), `Num::div`
is `arvo_format::div` (`ratio.rs:58-63`), `Num::convert` is `arvo_format::convert`
(`ratio.rs:112-117`). The step forms the product in four 64-bit limbs with a schoolbook multiply
(`mock/crates/arvo-format/src/wide.rs:140-170`), divides it by long division (`wide.rs:231-260`),
and reduces the remainder to lowest terms with Euclid's algorithm and two more divisions
(`mock/crates/arvo-format/src/apply/position.rs:205-210`), all to decide a rounding that for a
binary quantum is a test of the low bits. In the ad-hoc spike a Q16.16 product costs about 350 ns
against 1.1 ns by hand, the quotient by a constant 235 ns against 1.9, the conversion to Q8.8
239 ns against 2.5, and link-time optimisation changes none of it. The first audit predicted this
route would be 10 to 50 times slow if taken. It was taken, and it is 60 to 330 times.

The crate knows. Each call carries a ticket, `arvo_numeral_s_fixed_point_product_goes_through_the_128_bit`,
and the design says the step "is slow for it" and that the cure is the format's to ship
(`DESIGN.md.tmpl:312-317`). So this is a known debt and not a blind spot. Three things make it a
finding all the same.

First, it is the operation the one known consumer is waiting for.
`obligation::a_deterministic_fixed_point_lane_with_declared_rounding` (`obligation.toml:209`) is
kaski's row, and the product is the centre of it.

Second, the reason the design gives for not writing the fast arm does not hold up against the
crate's own contract. The reason: "The rounding has one home, the format's applied map, and a second one
written here to be quick would be a second arithmetic kept equal to the first by tests alone"
(`DESIGN.md.tmpl:309-310`). The contract, 260 lines earlier: "The carrier arithmetic is the
mechanism that meets that equality fast, and it is free to be anything that does"
(`DESIGN.md.tmpl:52-53`). The sum, the difference, the negation and the integer product are each
already a second arithmetic kept equal to the first by exhaustive tests and planted mutants, and
that is the crate's best idea. The design does draw a line: those four only complete, and it is
rounding that "has one home". But the rung's completion is already a second completion beside the
format's, and the cure the design names, an adaptation the format ships at the container's width
(`DESIGN.md.tmpl:314-316`), is itself a second rounding beside the wide one, kept equal to it by
tests. So the reason given decides which crate holds the fast rounding and not whether there is
one. The rounding of a binary fixed-point product is a widening multiply, a shift and a test of
the remainder bits, checkable against the oracle exhaustively at 8 bits in all five deterministic
modes, in either crate.

Third, nobody had timed it. A debt of a factor of 300 on the consumer's central operation was
shipped with a ticket and without a number, because the plan puts the first value bench at step 19.

One more cost sits behind the same door and is not ticketed. The default check is `Abort`, and
under it a debug build asks a verdict before each operation. For the product the verdict is
`fits_mul`, which calls the format's `product_position` and `panic_on_overflow` on every call, at
an integer quantum as well (`num.rs:369-376`, `:409-412`). So in a debug build under the default
check, every integer multiply runs the four-limb step. Debug builds are where a game is developed.
The ad-hoc spike's figure for this is in `NUMBERS.md`.

Mark: good as it is for the construction, the admission, the ring at an integer quantum, the
order, the crossing and the doors' design. Going wrong for the fixed-point product, the quotient
and the conversion on the format's step, fixable in place: right requires the adaptation at the
container's width, written as an arm beside the oracle and held equal to it by the suite, before
anything is built on `Fix`. Easy to get wrong but can be right for the traits, the hash, the default
and the layout promise, as on 13 November.

### `arvo-format`: the oracle

Every format, every adaptation and every exact operation as `const fn`, with the exact position
held as a floor in four 64-bit limbs and a remainder, and obligations checked as constants. I read
`apply.rs`, `multiplication.rs`, the division's doors and the wide integer's division and common
divisor myself; the design document and the suite I have through a reader's section-by-section
notes, and say so where a statement rests on them.

As an oracle it is better than it was a day ago. The night added the wrapping product past the slot
index, the floor carried in four limbs so that no product is refused for where it lands, a member
type, a divisor taken from a member, and the two standards (`ccb4c239`, `280e3a2a`, `b687fc01`,
`085033a4`). The test oracle is a separate big integer on a pair of host `u128`s, with a different
representation and a bit-serial division, importing nothing from the crate
(`mock/crates/arvo-format/src/tests/big.rs:6-17`, reader's notes). The one red test of the first
audit is fixed by the reader's reading of it: the sample builder saturates below the top
(`mock/crates/arvo-format/src/multiplication/tests/the_inverse_round_trip.rs:37`), and the test now
runs to width 64. I could not run the suite, since the shared build directory ran out of disk during
this audit.

The parity suites, which the first audit judged as a reading of documentation checked against
arvo's own oracle, are now two different things and deserve separate marks.

The IEEE 754 binary suites have an executable reference on the other side, which is what the first
audit asked for. binary32 and binary64 are compared with the host's own conversions at test time,
about eight and nine thousand seeded inputs per rounding direction, with planted wrong modes that
must be reported (`mock/crates/arvo-format/tests/ieee754_binary_parity.rs:9-15`, `:215-241`,
`:322-335`, reader's notes). What they check is one thing: an exact value rounded onto the float's
grid. No sum, product or quotient runs through an interchange format, and a test asserts exactly
that as eleven refusals (`ieee754_binary_parity.rs:488-511`). The decimal suites and the MATLAB
suites compare against rules written by hand from the standard's text and the vendor's
documentation (`mock/crates/arvo-format/tests/matlab_fi_parity.rs:8-11`,
`matlab_fimath_parity.rs:17-24`), and no output of the real tool is stored.

Three things around the oracle, in the first audit's order.

It is on a run-time path. This is the finding under `arvo-numeral` above, seen from the other
side. The format's design does not discuss being called at run time at all: by the reader's search
the word oracle appears in it only for test oracles, and the phrase the value crate uses for the
cure, an adaptation at the container's width, does not appear in it. So the debt is recorded in the
crate that owes nothing (`mock/crates/arvo-numeral/src/num.rs:397-401`) and absent from the design
of the crate that is to pay it. There is also a reason to think the cure does not belong in this
crate as designed: `arvo-format` cannot name a carrier, by the dependency direction that the root
design calls the point (`mock/DESIGN.md.tmpl:34-38`). An adaptation "at the container's width" is
either stated over a declared width small enough to fit a machine word, which the format can say,
or it is carrier arithmetic, which it cannot. The numeral design's one sentence on the cure picks
the first: the format ships the adaptation and the numeral calls it "with the machine's widening
product" (`mock/crates/arvo-numeral/DESIGN.md.tmpl:314-316`). That is a workable split. It is
written in the design of the crate that will not build it, it is in no step of the plan (a search
of `mock/research/202611130950_canon-double-check/PLAN.md` for the ticket or the phrase finds
nothing), and it does not say up to what declared width it applies.

The design document is 2424 lines, up from 2122 (`git diff --stat 7e924d0c..HEAD` on the file: 475
insertions, 173 deletions). The first audit said part of it narrates defects the crate used to
have. This audit's reader sampled 250 lines and searched the whole for the words that mark such
narration and puts it at one to three percent, about ten places, so I withdraw that half of the
finding. The length stands: one document for one crate, five times the limit the repository
enforces on a source file, and the part that grew fastest is the standards, which could be a page
of their own as the numeral's proof now is.

Inlining is now enforced here by a lint, `every-function-the-format-ships-is-inline`, which found
209 functions unmarked when it was written (design round `202611132043-aun1`, reader's notes). The
lint covers this crate only, and its own header carries a ticket for the other four
(`mock/lints/every_function_the_format_ships_is_inline.rs:33-34`). The host door's two unmarked
functions in `arvo-numeral` are that ticket's first measured case.

Mark: good as it is, as an oracle and as the home of the standards' declarations. Going wrong as
the arithmetic a value runs on, fixable in place, with the question of which crate owns the fast
adaptation to answer first. Easy to get wrong but can be right for the length of its design; right
is the standards and the open items moved to pages of their own.

### `arvo-placement`: where the bits sit

Unchanged since the first audit in source and in judgement. At sole occupancy it is complete: a
function from a signature to a container, given once as a width and once as a type, with a check
that the two agree which the value crate asserts for itself and does not trust
(`mock/crates/arvo-placement/DESIGN.md.tmpl:89-103`, `mock/crates/arvo-numeral/src/admit.rs:83-88`).

At shared occupancy nothing holds a value, so the footprint objective has no effect on any
program, and the design's law that the access width is not a function of the carrier is carried as
a catalogued red test because the one packing rule shipped cannot exhibit it
(`mock/crates/arvo-placement/src/lib.rs:21-25`). The objective has two values
(`lib.rs:97-102`). The committed benches say the worth of packing depends on the host, the access
pattern, the column size and whether the column is written (`NUMBERS.md`), which two values cannot
express. That is consistent with the concept if the shared arm becomes several arms keyed on those
dimensions.

Mark: good as it is at sole occupancy. Easy to get wrong but can be right at shared occupancy; right
requires the arms keyed on more than the objective, and the packed column benched for random access
past the cache and for writes before the design is fixed.

### `arvo-strategy`

Judged under the strategy axis above. One small thing in its design: the surface section names the
trait's first item as `OBJECTIVE`, "which is the `arvo_placement::Objective`"
(`mock/crates/arvo-strategy/DESIGN.md.tmpl:166-167`), where the source has an associated type bound
by `ObjectiveKind` (`mock/crates/arvo-strategy/src/lib.rs:49`) and the design itself explains forty
lines of the placement design why it must be a type.

### `arvo-bits`

Unchanged. One type over a `u64` whatever the width (`mock/crates/arvo-bits/src/lib.rs:48-50`),
with masked construction, a raw door and a masked cast, and no set operation, so
`obligation::set_operations_over_a_fixed_size_bit_set` (`obligation.toml:25`), the one obligation
that names an instruction count and the one hilavitkutin's scheduler leans on hardest, is not
started. The plan has it at step 10 (`PLAN.md:120`).

Mark: good as it is as a field in a register. Going wrong as something a table holds many of, since
a 13-bit field takes 64 bits beside a crate whose subject is deriving the narrowest container;
fixable in place by choosing the carrier on purpose.

## The root documents

The first audit marked these going wrong. They were repaired the same day (`f939933f`): the
principles say five crates and describe `Num` (`mock/PRINCIPLES.md.tmpl:5-35`, `:117-133`), the
strategy section says no value takes a strategy yet (`:328-335`, `:347-358`), and the agent
instructions open on five crates (`mock/agent/MAIN.md.tmpl:10-13`).

What remains is small, with one exception.

| Where | What it says | What is true |
|---|---|---|
| `README.md:39` | The known consumers are hilavitkutin and vehje, "and they're what drive which surfaces eventually exist" | kaski has an obligation row of its own; op: "Yes, general numerics too" (op-answers-26) |
| `README.md:41` | For general-purpose numerics "today, this probably isn't the right thing" | True of today. Nothing beside it says general numerics is intended |
| `mock/PRINCIPLES.md.tmpl:16-18` | The value's sum and comparison | Six operations and the order |
| `mock/PRINCIPLES.md.tmpl:126-128` | Aliases `Uint`, `Int`, `USize`, `ISize` | `Fix` and `UFix` as well (`alias.rs:57-66`) |
| `mock/DESIGN.md.tmpl:99-100` | A facade "would sit below all of them" | `mock/PRINCIPLES.md.tmpl:27-28` says above |
| `mock/DESIGN.md.tmpl:65` | `Additive` is the instance of `Identity` | `Multiplicative` too (`identity.rs:39-41`) |

The exception is the first two rows taken together. My first two passes said the README states
the opposite of op's answer. Read again in the third, line 41 is a statement about today and is
true, and the README's account of the present state (`README.md:19-25`) is current and candid. What
is out of date is line 39: who drives the surfaces. A stranger reading the two lines concludes
that general numerics is not intended, and op has said twice that it is.

Mark: good as it is, with one sentence of intent to add.

## The canon and the way the work is run

Counted by a reader over `mock/registry/` at `d0161800` and spot-checked by me: 110 rulings (48
ratified, 56 stated, 5 open, 1 in force), 173 proposals of which 137 rest on one expert, 110
questions of which 19 carry an answer, 196 retirements, 120 probes (109 sound, 8 uncontrolled, 2
defective, 1 withdrawn), 25 dimensions, 26 laws, 20 topics, 17 obligations.

The registry remains good as an instrument and I have nothing to add to the first audit's praise of
it. Two things changed in a day and both are for the better. Ratification got a recorded route for
what the written history shows to be op's
(`ruling::what_the_written_history_shows_to_be_his_is_ratified_by_default`, `ruling.toml:1960`), and
each ratified row now names who ratified it. And the ratio the first audit complained of, record to
product, turned: one night produced the difference, the negation, the product, the quotient, the
conversion, the order, the host doors, the fixed-point aliases, the count and two standards, on the
canon as it stood.

The first audit's three process findings, now:

The plan measures last. Holds, and the night proved what it costs. Value benches are still step 19
and the port step 20 (`PLAN.md:125-126`). The plan's own table lists the benches as needing steps 3
and 8 only, so nothing but the order keeps them late. No file under `mock/benches/` has changed since before
the first audit (`git diff --stat 7e924d0c..HEAD -- mock/benches` is empty). Two things landed
overnight that fifteen minutes of timing would have caught on the day: the host door's missing
inline, and the size of the fixed-point product's debt. Both were found here by an ad-hoc spike of
about three hundred lines. The value is new enough that every operation added to it is cheapest to
measure on the day it is written.

The benches do not measure the tree. Holds; see `NUMBERS.md`.

The consumer is consulted through documents and not through code. Holds. hilavitkutin still pins
the old arvo and nothing of the new value has been tried in one of its loops (`DOWNSTREAM.md`).

One new observation. The review trail for the night is itself large. By a reader's count the
design-round directory holds 225 rounds, 57 of them dated 12 to 14 November, every one closed; and
the research directory holds some twenty review directories for the night's branches, most with two
or three passes and most opening on a verdict of do not merge that the next round answered
(`mock/research/2026111320*` to `2026111406*`). I did not judge them one by one. The reviews found
real things and the code is better for them. None of them ran the code for time: a search of
`mock/research/` for a figure in nanoseconds or a timer finds no file in a review of the night's
branches. The two findings above passed through all of them, which says the review asks whether the code matches the
canon and nobody is asked whether it is fast.

Mark: good as it is for the registry as an instrument and for the canon's ability to carry
implementation. Going wrong for the order of work, fixable in place: right requires a value-tier
bench in the same pull request as each operation, and one hot module of a real consumer ported
before the contracts crate is designed.
