# How Well arvo Answers Those Who Build On It

Part of the second stack audit; `AUDIT.md` beside this file is the entry. This file takes each kind
of consumer: what it needs from arvo, what the intent and the design answer, and what is missing.
The consumers are kaski, the one known consumer; hilavitkutin, the layer between; and the two kinds
op asks to be assumed, a generic application and a server daemon. notko is last.

What is said here of another repository's source comes from a reader's notes unless it says I
opened the file; the reader worked from `Read` on paths, most of its searches in those repositories
were refused, and it says its kaski evidence is sampled. Paths into other repositories are at their
`dev` on 14 November 2026 and are given so a reader can open them. Ad-hoc spike figures are from
`mock/research/sketches/202611140930_stack-audit-2-value-tier/`, one shared x86_64 host.

One thing changed under this whole file since the first audit. The first audit ended on a question
of intent: the tree's documents put general numerics out of scope and op's brief assumed them in.
Op answered it the same afternoon. Asked "general numerics too", "stack only for now" or "stable
Rust first", he chose "Yes, general numerics too" (op-answers-26 on the kaski channel), and when
asked again, "I've answered already. Option 2, but can be ordered as 3" (op-answers-36), option 2
being generic applications and daemons with the everyday surface planned and a stable toolchain
considered, and option 3 the stack first. So the intent is settled: everyone, in the order stack
first. The sections below are judged against that.

## kaski

kaski has no dependency on arvo (its `mock/Cargo.toml:387-576` and lock, reader's notes). It
carries its own fixed point, `Fixed<const FRAC: u32>(i64)` in
`kaski-engine-carrier/src/fixed.rs:54`, with the fraction at 16 in general use, 30 for a wide lane
and 48 for one subsystem. The carrier's own comment says it becomes an arvo numeral when a value
type exists, with the field private so the swap stays in the file (`fixed.rs:36-39`). kaski's
principles say it uses a stack member's primitive where one exists (`docs/PRINCIPLES.md:28-31`), and
op: "We have arvo used for kaski too if we have it already at hand for hila. No proxies for the
fundamentals" (op-answers-29).

A value type now exists. So the question for this consumer has become concrete: can `fixed.rs` be
replaced by `Fix<64, -16, _>` today, and what happens if it is.

| kaski has or wants | arvo at `d0161800` | Verdict |
|---|---|---|
| Signed fixed point in 64 bits, fraction 16, 30, 48 | `Fix<BITS, FRAC, A, C>` (`mock/crates/arvo-numeral/src/alias.rs:57-62`) | There |
| Saturating add and subtract (`fixed.rs:159-169`) | `add`, `sub` under `Saturate`, the host's own in the ad-hoc spike | There, and as fast |
| A product; floors today (`fixed.rs:171-176`), half to even wanted | `mul` rounds by the declared mode, through the format's wide step | There, correct, and about 300 times the hand product in the ad-hoc spike at 32 bits |
| Division by a run-time value; zero gives zero silently today (`fixed.rs:72`, `:178`) | `div` takes a `Divisor` that is a constant; no door from a value | Not there |
| `from_ratio`, `scale` | `div` by a constant, `convert` | Partly, same cost as the product |
| Integer roots, `exp`, `ln`, `pow` on integers (`fixed.rs:188-241`) | Nothing, and in no step of the plan | Not there |
| Parse from decimal text (`fixed.rs:84-138`) | Nothing | Not there |
| Raw bits out and in "for a codec or a hash" (`fixed.rs:147-157`) | `index::<i64>()` and `from_index`; the first is exact, the second completes silently | There for trusted bits; no verdict for a save file |
| `Ord`, `Hash`, serde derives (`fixed.rs:41-53`) | Inherent `cmp`; `Hash` over a 128-bit index; no `Ord`, no serde | Half there |
| A bit set of 256 tags (`kaski-engine-contracts/src/tags.rs:35-40`) | `Bits<N>` to 64 bits, no set operation | Not there |
| Vectors over a scalar trait (`kaski-vector/src/scalar.rs:66-68`) | Nothing; no contracts crate | Not there |
| The same bits on native and in the browser | Integer arithmetic on the declared width, no float, no host-dependent step (`mock/crates/arvo-numeral/DESIGN.md.tmpl:471-475`) | There by construction |

Four things in that table are judgements and not a list.

The rounding question resolved itself the right way round. kaski's ratified tenet now says every
precision-losing fixed-point operation uses "the one arvo takes by default", so that decays and
shares "drift in no direction" (the capture of 13 November under `.data/op-responses/`, reader's
notes), and op declined to pick arvo's default by fiat: it is "a generic with default perhaps
following one of the parity standards" and "easy to override per case"
(`ruling::a_shipped_overflow_default_is_a_generic_overridable_per_case`,
`mock/registry/ruling.toml:1928`). The design answers that exactly: the mode is a coordinate of the
signature, `Fix<64, -16, Adapt<HalfEven, Saturate>>` says it in one line, and a site overrides one
coordinate with `with_rounding` (`mock/crates/arvo-numeral/src/num.rs:214-224`). This is the fit
between intent and design at its best. What is missing is only the default itself, which waits on a
measurement nobody has scheduled before step 19.

The product is where the fit is worst, and it is the centre of kaski's obligation
(`obligation::a_deterministic_fixed_point_lane_with_declared_rounding`,
`mock/registry/obligation.toml:209`). kaski's simulation ticks 30 times a second and gives the world
at most six tenths of a 16.67 ms frame (`kaski-settings/settings.toml:44-61`, reader's notes), about
10 ms. At the ad-hoc spike's third of a microsecond per product through the format's step, that
whole budget is about thirty thousand products, where kaski's own carrier, a 128-bit multiply and a
shift, does the same budget in millions. That is reasoning from a 32-bit spike to a 64-bit carrier
I did not time, and the wide step's cost does not obviously depend on the carrier. kaski cannot
adopt `Fix` for its lattice or its decays until the product is carrier arithmetic. Nothing in the
design prevents that; `DESIGNS.md` says what is in the way.

Division by a run-time value is the second gap, and it is a design gap and not only an unbuilt
one. Op chose "Explicit per call" for the zero divisor (op-answers-6): a divisor type that cannot
be zero, or a named operation that says what zero gives. arvo's `Divisor` is the first of those and
is right. What does not exist is the way to get one from a value at run time, since that is a
verdict on a datum, and the row that holds it back is the same one that holds back the ingest
verdict (`DESIGNS.md`, on run-time checks).

The transcendental functions are the largest piece nobody has planned. kaski has integer-only
`exp`, `ln`, `pow` and roots that give the same bits on every machine, and "if it's a numerics
thing, it's arvo" (op-answers-30). They return under
`ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open` (`ruling.toml:1942`) and appear
in none of the plan's twenty steps.

Mark: easy to get wrong but can be right. The intent and the design answer this consumer better
than anything else it could pick up: declared rounding, declared overflow, identical bits across
targets, per-site override. Right requires, in this order: the fixed-point product, quotient and
conversion as carrier arithmetic; kaski's own carrier used as the baseline in the first value
bench, since it exists, is tested, and is what kaski keeps if arvo is slower; a divisor from a
run-time value; `Ord` and a bytes door; and the integer transcendentals given a step.

## hilavitkutin

Still pinned to the arvo that was cleared on 8 August, by revision, for ten crates (its
`mock/Cargo.toml:45-54`, reader's notes). Nothing of the new value has been tried in it.

The first audit's most useful fact stands and was checked again by this audit's reader in the
source: the hot loops take the old platform integer apart and do host arithmetic. `morsel.rs:49`
builds `USize(self.start.0 + self.len.0)`; the loop at `:96-99` is `while i.0 < end.0`,
`i = USize(i.0 + 1)`; `scheduler/run.rs:236-263` and `worker.rs:199-205` do the same under lint
escapes. The old type supplied a name and no arithmetic.

Against what a morsel loop needs, the new value now has nearly all of it, which it did not a day
ago:

| The loop needs | arvo at `d0161800` |
|---|---|
| A platform-sized count that adds, subtracts, multiplies | `USize` with `add`, `sub`, `mul`, the host's in the ad-hoc spike |
| Compare, minimum, maximum | `cmp`, `lt`, `min`, `max` (`order.rs:14-82`) |
| To index a slice | `host()` (`alias.rs:79-90`), a bit copy |
| Divide rounding up | Not directly: `div` takes a constant divisor and rounds by the declared mode |
| To iterate a range of values | Nothing |
| A bit set with and, not, test | Nothing (`obligation::set_operations_over_a_fixed_size_bit_set`, `obligation.toml:25`) |
| Capacity as a type-level size | Nothing; step 9 |
| A moving average in unsigned fixed point | `UFix` exists; its product has the cost above |

So for the first time a port of `morsel.rs` is possible to attempt, and would be the cheapest test
there is of whether the loop can stay inside the value. Two things would be found in the first
hour: there is no range to iterate, so the loop counter leaves the value anyway, and every `USize`
must name its adaptation, since no default exists.

The bit set remains the piece this consumer waits on most. The scheduler's access sets are its
hottest numeric object by its own design (`run.rs:176-254`), the obligation is the only one that
names an instruction count, and it is step 10, behind a contracts crate and a capacity crate.

hilavitkutin's own audit of 13 November, which this audit's reader read whole, marks the fit to
kaski and to generic applications and daemons as going wrong for reasons that are hilavitkutin's
and not arvo's (failure policy, growth, ingestion). Whether its findings were addressed overnight
is for the sibling audit; the reader could not list that repository's log.

Mark: easy to get wrong but can be right. Right requires one hot module ported now, `morsel.rs` by
preference, with every place the port has to unwrap filed against arvo; and the bit set moved ahead
of the algorithm families.

## A generic application

In scope by op's answer above. The README still says otherwise (`README.md:41`).

What it gets that it cannot get elsewhere is unchanged from the first audit and is real: scaled
integers at any radix with declared rounding and overflow and computed laws, which is what money
and audited quantities want; exact widths whose behaviour does not depend on the container; an
invalid format refused at the consumer's line. The decimal half of that is built at the format tier
and admitted at the value tier (a scaled integer at radix ten passes `Sole`,
`mock/crates/arvo-numeral/DESIGN.md.tmpl:89-92`), with the product at the cost above.

What it meets first:

1. A nightly compiler. Still true of the build and still not of the text. arvo's crates enable no
   unstable feature by default, but the workspace takes notko with `const` and `try_trait_v2`
   (`mock/Cargo.toml:64`), which turn on five unstable features in notko, and arvo's source uses
   none of what they gate: only `notko::Maybe` (reader's grep). notko alone builds on stable from
   1.85 with its defaults off. So the stable path op asked to have "considered" looks like one
   line in a manifest for the bottom four crates. Nobody has tried it; neither did I, since the
   workspace hooks refuse a stable build (the first audit's ticket `39d402b85f3d`).
2. No way to print or parse. `Debug` prints the slot. No `Display`, no `FromStr`, no decimal
   formatter. For a decimal scaled integer this is the feature.
3. No bytes, no serde.
4. No operator traits and no `Ord`, so no `a + b`, no sorting, no use as a key in an ordered map.
5. No default adaptation, so the shortest spelling of an integer is `Int<32, Adapt<TowardZero, Wrap>>`.
6. No verdict on untrusted input (`DESIGNS.md`).
7. No value past 64 bits (`mock/crates/arvo-numeral/src/admit.rs:55-58`), so no 128-bit money.

None of the seven is a flaw in the concept. The second, fourth and fifth are the distance between
a foundation and a library and are what op called "the everyday surface". The plan's step 5, the
contracts a consumer bounds on, covers the fourth; the rest are in no step.

Mark: easy to get wrong but can be right. Right requires the scope sentence changed, the notko
feature line tried without the two features, and the everyday surface given steps in the plan now
that it is in scope: `Display` and parsing for decimal first, because that is where arvo is alone.

## A server daemon

Also in scope. It differs from an application in four ways that touch arvo.

It reads numbers it does not trust, on its hottest path. This is the run-time check finding in
`DESIGNS.md`, and it is sharper for a daemon than for anyone: the verdict exists, is private, and
the row that keeps it private waits on a known use that this consumer is. The door itself is now
cheap in the right build and 35 to 45 times a hand clamp in a default release build, by two
missing attributes (ad-hoc spike).

It counts from many threads. Nothing is said of a value in an atomic. A transparent newtype over a
machine integer is sound to hold in the host's atomic of the same size only if the layout is
promised, which is open under `question::what_a_datum_stands_for` (`mock/registry/question.toml:98`).
A counter that saturates at a declared width under contention is a compare-and-swap loop a
consumer will get wrong, and belongs with the accumulator the design lists as not here
(`DESIGN.md.tmpl:537-538`).

It runs for a long time. Wrapping counters, saturating gauges, durations and timestamps as scaled
integers with a declared quantum are what the format spine says best. None is built as a named
thing, and none needs a new concept.

It talks to C and to the network. The transparent layout is right for both. What is missing is one
sentence in a design saying which container a signature takes on each target and that it is a
promise, with a test.

Mark: easy to get wrong but can be right. Right requires the public verdict, the layout promise
answered, and the door's two attributes. Wrong looks like a daemon author reading "never a run-time
check", keeping bare integers at the edge, and wrapping into a value only what has already been
validated by hand.

## notko

arvo takes `Maybe` from it and nothing else (reader's grep; `mock/crates/arvo-format/src/apply.rs:33`
is one use I saw). The dependency is thin and the direction is right. Its one effect on arvo's
consumers is the nightly requirement, which is arvo's choice of feature line and not notko's
doing.

Mark: good as it is, with the feature line to try.

## What this adds up to

For the stack arvo is going the right way, and after last night it is no longer early in the way
it was: the operations kaski and hilavitkutin need are named, built and proved equal to the
oracle. What separates both from adopting it is not design. It is that the fixed-point half runs
on the oracle, that nothing has been timed through a shipped type but by two ad-hoc spikes, and
that neither consumer's code has been put on the value to see where it has to step outside.

For the consumers op has now confirmed, the foundation is the right one and the surface they touch
first is unbuilt and mostly unplanned. That is an ordering he chose. The plan should say it.
