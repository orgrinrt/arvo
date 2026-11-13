# How Well The Stack Answers Those Who Build On It

Part of the stack audit; `AUDIT.md` beside this file is the entry. This file takes the consumers one at
a time: what each needs from arvo, what arvo has for it today, what the plan has for it, and where the
need is not in the plan at all.

The consumers are the two that exist in source (hilavitkutin and kaski), the smaller repositories that
stand in for arvo with their own newtypes, and the two kinds op asks to be assumed: a generic
application and a server daemon. notko is taken last, as the thing arvo itself builds on.

Almost everything here about another repository's source is from a reader's notes
(`slice-downstream.md` in the session's scratch space, a sonnet reader, not committed). I opened the
kaski carrier file and the notko feature list myself and say so where it matters. A count from the
reader is the reader's count. Paths into other repositories are at whatever their `dev` was on 13
November 2026 and are given so that a reader can open them, not as pinned evidence.

Each consumer ends in one of the four answers and one of the four marks, as in `DESIGNS.md`.

## hilavitkutin

hilavitkutin is the first consumer by every row that names one, and nine of the seventeen obligations
are written from it. It still depends on the arvo that was cleared on 8 August, pinned by revision
(`mock/Cargo.toml:39-48` in that repository, reader's notes).

What its source does with that old arvo is the most useful single fact the audit found outside this
repository. By the reader's count the platform-sized unsigned type appears about 1400 times and is
built from a bare integer about 370 times; the field holding the bare integer is read about 550
times; and 286 lint escapes permit it. The hot loops read the bare integer out, do host arithmetic,
and wrap the result again (`src/dispatch/morsel.rs:89-103`, `scheduler/worker.rs:192-198`, reader's
notes). Capacity and dimension types appear in the hundreds, bit access in the tens, fixed point about
forty times, and the one fixed-point kernel, the moving average the adaptive scheduler is designed
around, is a stub (`providers/src/adapt_ema.rs:75-79`).

Three things follow.

The old arvo did not answer this consumer either. A numeric type that its heaviest user unwraps in
every hot loop was supplying a name and no arithmetic. The clearing of 8 August was right, and the
measure of the new tree is not whether it has the old families back but whether hilavitkutin's loops
can stay inside the value.

What they need to stay inside it is small and most of it is in the plan early. A morsel loop wants a
platform-sized count that adds, subtracts, multiplies by a small constant, divides rounding up,
compares, indexes a slice and iterates a range. The new value adds and compares. Subtraction, negation
and the product are step 3 of the canon check's plan and the quotient is step 7
(`mock/research/202611130950_canon-double-check/PLAN.md:82-101`). Indexing and ranges are in no step I
found: `USize` has a bit-copy door out (`mock/crates/arvo-numeral/src/alias.rs:61-84`), which is the
right door for indexing, and nothing makes a range of values iterable. Capacity is step 9, written
from hilavitkutin's own usage, and that step's own count of it (49 uses of `Capacity`, 17 of `Dim`,
`PLAN.md:270-276`) is much lower than the reader's count above, so one of the two is counting
something else and the number wants redoing before the row is written.

The obligations describe the algorithms hilavitkutin's design says it wants (a spectral partition, a
cost DP, sparse adjacency analyses) and those are large. By the source, none of them is what the
consumer is waiting for. The bit set is: `obligation::set_operations_over_a_fixed_size_bit_set` is the
only row with an instruction count in it, and the scheduler's access sets are the hottest numeric
thing in the engine by its own design. Nothing in the tree starts it; the plan has it at step 10,
behind the contracts and the capacity crate.

One number from that repository matters to the strategy question. Its committed night findings of 11
May, on one Apple M1, put a Q0.32 moving average 11 to 14 percent ahead of `f32`, wrapping about twice
as fast as saturating, and 64-bit fixed point 25 to 62 percent behind floats on shapes a fused
multiply-add serves (`NIGHT_FINDINGS_2026-05-11.md`, reader's notes; I did not open it). That is one
host and old code, and it agrees in direction with this audit's ad-hoc spike.

Answer: passes in intent and needs implementation, in a different order than planned. Mark: easy to
get wrong but can be right. Right requires porting one hot module, `morsel.rs` by preference, onto the
new value as soon as subtraction and the product exist, and treating every place the port has to
unwrap as a finding against arvo.

## kaski

kaski has no dependency on arvo. It carries its own fixed-point type, a 64-bit signed integer with a
const fraction width, at `kaski-engine-carrier/src/fixed.rs:40-54`, which I opened: saturating
addition and subtraction, a product through 128 bits that floors, square and cube roots, an
exponential, a logarithm and a power, construction from a decimal, and doors to and from the raw bits.
By the reader's count about 600 files mention it, the fraction width in use is 16, and the vector
crate beside it is `f32` and `f64` throughout with markers saying a stack type should replace them.

`obligation::a_deterministic_fixed_point_lane_with_a_declared_rounding` is kaski's row. Against it:

| kaski needs | arvo today | in the plan |
|---|---|---|
| Signed fixed point in 64 bits | Only as the MATLAB-named `Fi`; no value of it has been built or timed | Step 3, which adds a signed fixed-point point beside `UFixed` |
| Saturating add and subtract | Add, with `Saturate`; no subtract | Step 3 |
| A product with half to even | At the format tier, as an oracle | Step 3, exhaustive in all five deterministic modes at six bits |
| Division | At the format tier | Step 7 |
| Roots, exponential, logarithm, power | Nothing | Not in the twenty steps; the old families are to return under `ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open` |
| A door to raw bits for save files and hashes | None below pointer width | Named as not here (`mock/crates/arvo-numeral/DESIGN.md.tmpl:359-365`) |
| Ordering, so a value can be a key and be sorted | Refused by the shipped design | Step 5, where order is one of the contracts a consumer bounds on |
| Vectors of fixed point | Nothing | A higher crate's, per `ruling::arvo_is_a_library_and_the_value_composes_on_top` |

Two things in that table are more than a list of the unbuilt.

The first is a number. kaski's product floors today, its tenet and its obligation say half to even,
and this audit's ad-hoc spike puts a price on the difference for a Q16.16 product on one x86 host:
about the same as a floor when products are independent, and about twice a floor's latency when each
product feeds the next, 3.1 against 1.5 nanoseconds a step (`NUMBERS.md`). The same spike shows why
kaski wants it anyway: after 30000 steps of a decay, floor has drifted by 0.078 and half to even by a
millionth. A simulation that integrates state is the dependent case. So the mode kaski asks for costs
something real exactly where kaski would use it, and the saturating half-to-even product that the
`Precise` preset names is the slowest combination measured. This is not a reason to refuse the mode.
It is the first place the arms concept would earn its keep for a consumer: in the spike's one
sequence a half-up arm costs about what a floor costs, 1.7 nanoseconds a step against 1.5, and ends
0.000014 above the real value where floor ends 0.078 below it and half to even 0.000001 below. So half
up is within a hundredth of floor's price and a five-thousandth of its error, while being ten times
further out than half to even. Whether kaski's determinism needs half to even in particular, or any
declared mode that does not drift one way, is a question for kaski.

The second is a pull between rows. kaski's obligation speaks of half to even as the mode arvo takes by
default; the canon check's candidates for a shipped default are a floor with saturation or a wrap; and
`ruling::wrap_or_clamp_stays_open_and_both_get_priced` is open. Op's direction of 13 November, that a
shipped default is a generic a consumer overrides per case, dissolves it, as long as the default is
not also welded to a strategy name. `DESIGNS.md` takes that up under the strategy axis.

Answer: passes in intent and needs implementation, with the transcendental functions the largest
unplanned piece. Mark: easy to get wrong but can be right. Right requires the value-tier product to be
benched in a dependent chain before its design is locked, and kaski's own carrier used as the baseline
to beat, since it exists, is tested and is what kaski will keep if arvo is slower.

## The repositories that stand in for arvo

kirjo, lato and riimu, by the reader's notes, each carry a small module of integer newtypes named
after arvo's and marked as stand-ins. kolli and tarina are named in obligations
(`obligation::the_unstable_machinery_does_not_reach_a_consumer`,
`obligation::the_algebra_is_legible_to_a_proof_layer`).

What the stand-ins show is which part of arvo consumers reach for when they are not doing numerics: a
width-named unsigned integer, a platform-sized count, and nothing else. That is the cheapest part of
arvo to finish, it is mostly finished, and it is blocked for them by two things that have nothing to
do with arithmetic: the nightly compiler requirement below, and the alias that makes them write the
adaptation out.

Answer: passes in intent. Mark: good as it is, once the two blockers are dealt with.

## A generic application

Op asks that generic applications be assumed. No registry row names one: a case-insensitive grep over
`mock/registry/` for a server, a daemon or an application returns two hits, one about a mathematical
application and one about applying an instrument, and `mock/PRINCIPLES.md.tmpl:272-275` says
"downstream applications" once in passing. So this section is my reading of what such a consumer
meets, not a check against stated needs.

What it gets that it cannot get elsewhere is real. Scaled integers at radix ten with a declared
rounding mode and a declared overflow policy, with the laws that hold computed and not assumed, is
what money, quantities and anything audited want, and no crate I know of gives all three together
(the `fixed` crate is binary only; decimal crates are run-time scaled and allocate or carry a wide
mantissa; I state this from memory and did not check current versions). Exact-width integers whose
behaviour does not depend on the container are what a protocol or file format wants. A failed
constant at the consumer's line for an invalid format is a better error than any of them give.

What it meets first is this.

1. A nightly compiler. arvo's crates enable no unstable feature, and say so
   (`mock/crates/arvo-numeral/DESIGN.md.tmpl:37-38`), but `arvo-format` depends on notko with its
   `const` and `try_trait_v2` features (`mock/Cargo.toml:64`), and notko turns on five unstable
   features for them (`src/lib.rs:13-17` in that repository; I opened both). So the claim is true of
   arvo's text and false of arvo's build. `obligation::the_unstable_machinery_does_not_reach_a_consumer`
   is written about API surface; a toolchain requirement reaches the consumer harder than any API.
   Most applications will not take a nightly pin for a number type. The repository pins
   `nightly-2026-05-28` for its own build (`rust-toolchain.toml`). I could not turn this into a
   measurement: two hooks between them refuse every stable build in a workspace clone, one because a
   stable cargo ignores an unstable setting in the shared build directory and one because a build
   directory of its own is not allowed, which is ticket `39d402b85f3d`. So the claim that a consumer
   cannot build arvo on stable is reasoned from the feature list, not measured.
2. No way to print or parse a value. `Debug` exists; `Display`, `FromStr` and a decimal formatter do
   not, and `obligation::debug_output_without_an_allocator` covers only the first.
3. No bytes. Nothing turns a value into its little-endian bytes or back, and no serialisation trait is
   implemented or planned.
4. No ordering trait and no operators, as in `DESIGNS.md`. This one is planned: step 5 of the plan
   judges its three arms on whether a consumer with no feature line can write a function generic over
   a numeral that adds, orders and has an additive identity, which is the right test.
5. No default adaptation, so every type is written with four names.
6. No promise that the container stays the container across releases.

None of these is a flaw in the concept. All six are the distance between a foundation and a library,
and the README says that distance out loud: for general numerics this "probably isn't the right
thing" (`README.md:41`). The finding is that op's brief assumes generic applications as consumers and
the tree's own documents assume they are not, and one of the two wants changing on purpose.

Answer: passes in intent for the domain it names; for a generic application, can be fixed in place,
and the nightly requirement is the one item that needs a decision and not just work. Mark: easy to get
wrong but can be right. The decimal scaled integer is the part to finish first for this audience,
because it is where arvo is alone.

## A server daemon

Also assumed by op, also named nowhere in the tree. A daemon differs from an application in four ways
that touch a numeric foundation.

It reads numbers it does not trust. `DESIGNS.md` covers this under the run-time check: the verdict
functions exist, the pattern is not written down, and the cheap carrier-width verdict is missing. For
a daemon this is the first thing it does with every request, so the cost of the ingest door in
`NUMBERS.md`, eight to nine nanoseconds without link-time optimisation on the spike's host, is on its
hottest path. With the oracle off that path the estimate is a compare and a branch.

It counts things from many threads. There is no atomic value and no statement of how a value sits in
an atomic. A transparent newtype over a machine integer can be transmuted into the host's atomic of
the same width, which is sound only if the layout promise is made, which is the open question above.
A counter that saturates or wraps at a declared width under contention is a compare-and-swap loop a
consumer will get wrong; it belongs with the accumulator the numeral design names as not here.

It runs for a long time. Wrapping counters and saturating gauges with a declared width are exactly
what arvo's signatures say well. Durations and timestamps as scaled integers with a declared quantum
fit the format spine with no change. This is the good news for this consumer and nothing is built for
it.

It talks to C and to the network. The transparent layout is right for both, and
`ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up` shows op has this in mind. What is
missing is the sentence in a design that says a value of a given signature is its carrier at a C
boundary and which carrier that is on each target, with a test.

Answer: passes in intent and needs implementation; the untrusted-input pattern and the layout promise
can be fixed in place and are cheap. Mark: easy to get wrong but can be right. Wrong looks like a
daemon author reading "never a run-time check", concluding arvo is for trusted arithmetic only, and
keeping bare integers at the edge, which is where the bugs are.

## notko

arvo's own foundation. By the reader's notes it is at 0.0.1, uses bare primitives in its own source,
and needs nightly for its default features; I confirmed the feature list. arvo takes its outcome and
option types from it and nothing numeric.

The dependency is thin and the direction is right. Its one effect on arvo's consumers is the nightly
requirement, and that is arvo's choice of features and not notko's doing: notko's stable surface may
be enough for what the format crate uses, and I did not test whether `arvo-format` builds with the two
features off.

Answer: passes in intent. Mark: good as it is, with the feature choice the one thing to look at.

## What this adds up to

For its two real consumers arvo is going the right way and is early. Both need the same next things:
subtraction and the product on values, a cheap way in and out at the carrier's width, ordering, and a
bench of the value tier on more than one host. hilavitkutin needs the bit set after that and kaski the
transcendental functions.

For the consumers op asks to be assumed, arvo has a real thing to offer in decimal scaled integers and
exact widths with declared behaviour, and has not yet decided to offer it: the documents say general
numerics are out of scope, the build needs nightly, and the surface a non-specialist touches first
(printing, parsing, bytes, ordering, defaults) is unbuilt and mostly unplanned. That is the one place
in this audit where the intent itself, and not the implementation, has two readings, and it is op's to
say which.
