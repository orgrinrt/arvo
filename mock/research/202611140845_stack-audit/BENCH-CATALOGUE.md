# Benches Still To Write

Part of the second stack audit; `AUDIT.md` beside this file is the entry. Each entry gives the
scenario, the arms, the sizes, and what the result could decide. An entry that could decide nothing
is not here.

The first audit wrote a catalogue too (`mock/research/202611131215_stack-audit/BENCH-CATALOGUE.md`).
I read its headings and not its body, so I do not judge its entries one by one. What I can say with
evidence is that none of them was written: no file under `mock/benches/` has changed since
(`git diff --stat 7e924d0c..HEAD -- mock/benches` is empty). Its five groups (the value tier, fixed
point, placement asked again, the returning families, what is left out) are the right groups, and
this catalogue keeps them. What is different a day later is that the value has operations, so the
first two groups can be written today against shipped types, and two ad-hoc spikes say where the
interesting rows are.

## Rules every entry assumes

These are conditions for any of the results to carry weight, taken from what the corpus lacks
(`NUMBERS.md`).

- Every arm calls a type in `mock/crates/` or is the hand yardstick beside one. No arm is a copy of
  a transform.
- The hand arm is asserted equal to the value arm on the whole input before timing, and the value
  arm is the one held to the format's oracle by the suite.
- Three hosts at least: one aarch64, one x86_64, and wasm32, which kaski ships to. The profile is
  recorded in every result. Two profiles per host: the default release a consumer gets, and fat
  LTO. The spike found a factor of 40 between them on one door.
- Each kernel's assembly is committed beside its result, so a mechanism can be read and not guessed.
- Sizes: one that fits the first-level cache (4096 members), one that fits the second (262144), one
  past the last (16 million or more), unless the entry says otherwise.

## One: the value tier as shipped

These can be written now. Each should land in the same pull request as the operation it times; for
the operations already landed, first.

| Bench | Scenario | Arms | What it could decide |
|---|---|---|---|
| ring-elementwise | add, sub, neg, mul over two columns into a third, at 8 in 8, 13 in 16, 32 in 32, 60 in 64, signed and unsigned, wrapping and saturating | `Num`; host operator with mask or clamp; `core::num::Wrapping` and `Saturating` at full widths | Whether "the value is the host's arithmetic" holds off the one host tried. Finds the next case like the 13-bit saturating product, 4 times slow in the spike |
| ring-fold | sum and product folded over one column, same signatures | `Num::add` in a loop; hand per step; hand adapt once; hand lanes under the clamp-once law | The size of what a fold arm would buy through the shipped type, per host. Decides whether the accumulator the numeral design lists as not here comes before or after the contracts crate |
| host-door | `from_index` and `index` over a column, from `u8`, `u16`, `i32`, `u64` into 13 in 16 and 32 in 32, both policies | `Num`; hand clamp or mask; `as` cast; the old `adapted` route | Whether the door is free in a default release build. Today it is not. This bench is the regression guard for the inline lint's extension to the other four crates |
| order | min, max, compare-and-select, sort of a column of values by `cmp` | `Num`; host | Whether an inherent `cmp` costs anything in a sort, before the decision on `Ord` |
| debug-check | every ring operation under `Abort`, `Unchecked`, and the host's overflow checks, in a build with debug assertions, optimiser on and off | as named | What the default check costs a developer. The spike says 200 times on an integer product. Decides whether `fits_mul` needs its own carrier arm and whether `Abort` can stay the default |
| arm-selection | one function generic over the signature, instantiated at an integer quantum and a fractional one | the generic; each side written directly | That selection by a const predicate is free through the shipped type on every host, which the M1 gate bench shows only for a copy |
| hash-and-key | values as keys in a hash map and a sorted vector, 8, 16, 32, 64 bits | `Num` with its `Hash`; host integer; `Num` hashed by its carrier | Whether hashing the 128-bit index is visible. Decides the first audit's finding two |

## Two: fixed point, kaski's lane

The first row is the most valuable bench in this file. It is the one that would have caught the
largest finding of this audit on the day the product landed.

| Bench | Scenario | Arms | What it could decide |
|---|---|---|---|
| fixed-product | elementwise product and a dependent decay chain, Q16.16 in 32 and fraction 16, 30, 48 in 64, each of the five deterministic modes, wrapping and saturating, products in range and out | `Fix` as shipped; a hand widening multiply and shift per mode; kaski's `Fixed` product as it stands; the `fixed` crate; `f32` and `f64` for scale | Whether kaski can adopt `Fix`. What each rounding mode costs as throughput and as latency, which is the table the strategy presets need before any cell loses its ticket. Whether toward zero is dominated, as the first spike suggests |
| fixed-quotient | quotient by a constant divisor and by a run-time one once that door exists; `convert` between fractions 16, 30, 48 and to Q8.8 | `Fix`; hand; kaski's `div` and `scale` | The same for the other two operations on the wide step |
| fixed-drift | a decay and a share-out run for a million steps per mode | the five modes by hand and through `Fix` | Accuracy against cost per mode: the other half of the default. Not a timing alone; it reports the error at the end. kaski's tenet asks for no drift |
| fixed-frame | one of kaski's lattice steps, taken from its source read-only, on its carrier and on `Fix` | kaski's `Fixed`; `Fix` | Whether the whole of a real kernel stays inside the value, and what it costs against a 10 ms budget. This is a port as much as a bench |
| decimal-scaled | sum, product and rescale of a decimal scaled integer, two and four places, 64 bits | arvo at radix ten; hand `i64` with a constant power of ten; `rust_decimal` | Whether the part of arvo with no equal elsewhere is usable for money at speed |
| fixed-wasm | fixed-product and fixed-frame on wasm32 | the same | wasm has no 64 by 64 to 128 multiply in one instruction. Decides whether the 64-bit lane needs a different arm on that target, which is the arms concept's first real case of a predicate on the target |

## Three: placement and footprint

Unchanged in substance from the first audit, because nothing here has moved.

| Bench | Scenario | Arms | What it could decide |
|---|---|---|---|
| packed-random | random reads by index into a column of 13-bit members, 64 thousand to 256 million members | dense 16; dense pairwise; packed per member; packed in blocks | The one region where packing is supposed to pay, never measured past the first-level cache. Decides whether the footprint objective has a use |
| packed-write | single-thread and partitioned parallel writes | dense; packed with split points on the packing period; packed in blocks | Whether a shared placement can be written at all in parallel, and at what cost |
| packed-ordered-hosts | the carrier-width family rerun with the pairwise arm, three hosts, both profiles | as named | Which of host, instruction set and profile turned the sign between the M1 and the Xeon |
| bits-in-a-table | a table of a million rows each with a 13-bit and a 3-bit field | `Bits<N>` as shipped on `u64`; the narrowest host integer; packed | Whether `Bits` wants a derived carrier. Decides the bits crate's mark |
| bitset-ops | and, or, not, test, count over sets of 64, 256 and 4096 | whatever `arvo-bits` gains; hand `u64` words; `fixedbitset` | The obligation that names an instruction count, and hilavitkutin's scheduler |

## Four: the edge and the consumers

| Bench | Scenario | Arms | What it could decide |
|---|---|---|---|
| ingest-verdict | a column of untrusted host integers, some out of range; count the bad and build the good | a public verdict then `from_index`, once one exists; hand range test; `TryFrom` | That the verdict costs what a hand test costs. Needs the row on the ingest door read again first (`DESIGNS.md`) |
| morsel-loop | hilavitkutin's morsel loop on the new `USize` | its present `.0` arithmetic; the value | Whether the hot loop can stay in the value. A port more than a bench, and the cheapest test of fit there is |
| atomic-counter | a saturating counter at a declared width under four and sixteen threads | hand compare-and-swap; whatever arvo offers | A daemon's first need. Blocked on the layout promise |
| build-time | a crate instantiating 10, 100 and 1000 distinct signatures | clean and incremental build, release and debug | What the admission's const evaluation and many arms cost to compile. The arms concept has never been priced on this axis |
| stable-toolchain | the bottom four crates built on stable with notko's default features | builds or does not | Not a timing. Decides whether the nightly requirement is one line in a manifest |

## What is left out, and why

A bench of the format tier at run time. The oracle is not meant to be fast and timing it would
invite tuning the wrong thing. Its cost shows up through `fixed-product` and goes away with the
carrier arm.

Floats. Nothing computes over one. A float bench is worth writing the day an operation admits a
magnitude-indexed quantum.

The hash families, the graph families and the transcendental functions. They return under their own
rounds and each should bring its bench with it.

## Order

If only three are written: `fixed-product`, `host-door`, `debug-check`. Each guards a finding of
this audit that an ad-hoc spike found in an afternoon, and each would run in seconds.
