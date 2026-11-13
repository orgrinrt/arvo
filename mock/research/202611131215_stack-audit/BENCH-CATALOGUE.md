# Benches still to write

This is the catalogue the stack audit owes ahead of its report: the measurements arvo does not have and
would decide something with. It is written from the first pass of the audit, at `dev` tip `326b52f5`,
and the report beside it, `AUDIT.md`, carries the reasoning for each gap.

The starting point is one finding about the corpus as it stands. `mock/benches/` holds 254 result
regions, every one from one Apple M1, 253 of them from a dirty tree, none recording its build profile,
none with instruction or cycle counters, and none measuring a type that `mock/crates/` ships today.
Each arm re-implements its transform inside a `variants/*-shared` crate. So the benches below are not
additions to a working picture of the current crates. For the value tier they are the first
measurements there will be.

Three rules hold for every row, because the existing corpus shows what their absence costs. Every bench
runs on at least two hosts, one aarch64 and one x86_64, since the one bench that has been rerun on a
second host, `bitpack-carrier-width`, changed which arm is fastest at five of six sizes. Every dense
baseline is the strongest dense arm somebody can write, with its disassembly committed, since the
`d16` baseline the packing question was argued against turned out 58 to 62 percent slower than
`d16-padal` on the same rows. And every result records its profile and is run at two of them, fat LTO
with one codegen unit and the plain release default, since `arvo-format` carries no `#[inline]` and the
one disassembly on record of a value's sum without constant dispatch keys found 24 calls through the
GOT.

Sizes are named by where the working set sits: L1 is about 16 KiB of payload, L2 about 1 MiB, DRAM
64 MiB or more, and scalar means one value carried through a loop of a million steps with the result
kept live. A hand-written assembly arm is a yardstick and is called that in each row.

## The value tier, which nothing measures yet

| Bench | Scenario | Arms | Sizes | What it could decide |
|---|---|---|---|---|
| value-add-against-the-host | elementwise sum of two columns and a fold of one, through `Num` | `Num` under wrap and under saturate; the host primitive's `wrapping_add` and `saturating_add`; `core::num::Wrapping` and `Saturating`; hand NEON and AVX2 as the yardstick | widths 8, 13, 16, 24, 32, 60, 64, signed and unsigned; L1, L2, DRAM | whether one lowered path holds in time as well as in the disassembly, and what the adaptation onto a width below the carrier costs once the loop vectorises or fails to |
| value-across-profiles | the same kernel at three profiles | fat LTO one unit; thin LTO; release default with sixteen units; each with and without `#[inline]` on the `arvo-format` accessors | width 13 and 64; L1 | whether a consumer who does not turn LTO on gets the same code, which decides ticket `arvo_format_mark_the_const_accessors_inline_so_consumers_fol` |
| wrap-against-saturate-through-the-value | a fold and an elementwise map | wrap, saturate, each signed and unsigned, with the debug check `Quiet` | widths 8, 13, 16, 32, 64; L1 and L2; reduction lengths 8, 64, 4096 | the pricing `ruling::wrap_or_clamp_stays_open_and_both_get_priced` asks for, on the shipped type and not on a copy |
| the-debug-check-in-a-dev-build | the same fold at `opt-level` 0 and 1 with debug assertions on | `Abort`, `Quiet`, the host primitive with overflow checks | width 13 and 32; L1 | whether a game's development build can afford `Abort` as the default, which is the build a kaski developer runs all day |
| value-order-and-search | sort and binary search of a column | `Num` by its `cmp`; the host primitive; a key extracted once to the host | 16 and 64 bit; 1 thousand, 1 million | what refusing `Ord` costs a consumer who has to sort, and whether the door a sort needs is missing |
| value-hash-in-a-table | insert and probe of an open-addressed table keyed by a value | `Num`'s `Hash`, which hashes the 128-bit slot index; a hash of the carrier; the host primitive; under a multiply hasher and under SipHash | 64 thousand and 4 million keys | whether the slot-index hash is kept or replaced by the carrier's bytes |
| bits-against-the-word | set, clear, test, union, intersection, population count and iteration of set bits | `Bits<N>`; a bare `u64`; `bitvec` and `fixedbitset` as the frameworks a Rust user would reach for; hand `tzcnt` and `blsr` as the yardstick | N of 28, 32, 64; scalar and L1 | whether each operation lowers to one instruction, which `obligation::set_operations_over_a_fixed_size_bit_set` states as the demand |
| bits-carrier-width | a column of 28-bit identifiers scanned, compared and probed | `Bits<28>` over its 64-bit word; the same value in a 32-bit word; packed at 28 bits | 64 thousand, 4 million, 64 million | whether `Bits<N>` should take its container from the placement ladder, which the on-disk 32-bit obligation of hilavitkutin already implies |

## Fixed point, which is kaski's lane and has no bench at all

| Bench | Scenario | Arms | Sizes | What it could decide |
|---|---|---|---|---|
| fixed-product-by-rounding-mode | product of two fixed-point values, scalar chain and column | each of `floor`, `toward_zero`, `half_up`, `half_even` under wrap and under saturate; the host widening multiply and shift; the `fixed` crate; `f32` and `f64` multiply; hand `smull` and `imul` with a rounding shift as the yardstick | Q16.16 in 32 bits, Q32.32 in 64 bits, Q8.8 in 16 bits; scalar chain, L1, L2 | what half to even costs over floor on the product, which is the price of kaski's ratified tenet, and whether saturation or the rounding is the larger part |
| fixed-decay-drift | a decay applied a hundred million times, compared against an exact rational | the same four modes | Q16.16 and Q32.32; factors near one | the accuracy side of the same tenet: how far each mode drifts, measured, so that the cost above is weighed against something |
| fixed-lanes | the product and the sum over lanes | 4, 8 and 16 lanes through the value; `core::simd`; the `wide` crate; hand NEON `sqrdmulh` and AVX2 `pmulhrsw` as the yardstick, noting where their rounding is not the declared one | Q16.16 and Q1.15; L1 and L2 | whether the declared rounding survives vectorisation, since the instructions a hand writer reaches for round half up and not half to even |
| the-exact-accumulator | a sum of N products adapted once against adapted at every step | exact accumulator then one adaptation; saturate at every step; wrap at every step; the lane fold of the `satfold` family | N of 8, 64, 4096, 1 million; Q16.16 | the design of the accumulator in the plan's step 8, and whether the saturating fold's 23 percent from the bounds proof alone carries to a signed fixed-point sum |
| division-by-a-typed-divisor | quotient and remainder | a constant divisor checked at build; a runtime divisor of a format with no zero slot; the host `/`; `checked_div`; `NonZero` | 16, 32, 64 bit; scalar chain and L1 | what the type-level nonzero divisor buys, and whether the constant arm reaches the reciprocal multiply the compiler gives a literal |
| conversion-between-signatures | narrowing and widening a column with a declared rounding | `convert`; shifts and `as` casts; `fixed`'s conversions | Q8.8 to Q16.16 and back, 32 to 13 bits; L1 and L2 | the cost of a conversion as an explicit adapted operation, which every mixed-signature expression pays under the admission rule |
| decimal-scaled-integers | sums, products and a rounded division at radix ten | a radix-ten format under `half_even`; `rust_decimal`; a hand 128-bit scaled integer | two and four decimal places in 64 bits; scalar and L1 | whether the radix axis of the format serves money and units for an application or a daemon, which nothing has asked of it |

## Placement and footprint, to be asked again properly

| Bench | Scenario | Arms | Sizes | What it could decide |
|---|---|---|---|---|
| carrier-width-on-two-hosts | the existing carrier sweep, rebuilt | the five carriers; the strongest dense arm per host; a packed arm with a vector unpack per host | the six sizes on record plus 64 million; one, two, four and eight threads | the carrier question `ruling::the_carrier_question_waits_on_the_contention_measurement` holds open, with counters, on a clean tree and at a recorded profile |
| packed-access-that-is-not-sequential | random, strided and page-cold reads of a packed column | dense at each carrier; packed with scalar and with windowed decode | 1 million and 64 million; strides 1, 7, 64, 4096; cold cache | the half of the Cold intent the corpus never touched: `question::which_carrier_the_packing_claim_is_about` records random and strided access as unmeasured |
| footprint-per-entity | a struct of mixed-width fields in a table | each field at its sole carrier; a shared placement under both objectives | 16 fields of 1 to 60 bits; 1 million rows | whether the `Footprint` and `Access` objectives differ in anything a consumer can observe, which at sole occupancy they do not |
| the-corpus-again | the 254 regions, rerun | as committed | as committed | which of the conclusions the registry cites survive a clean tree, a recorded profile, the validation gate and a second host; 175 regions predate the gate |

## The families that return, and the consumers beyond kaski

| Bench | Scenario | Arms | Sizes | What it could decide |
|---|---|---|---|---|
| content-hash | a stable hash of a byte string at a named width | the returned family at 28, 64 and 128 bits; `xxh3`, `wyhash`, `blake3` and FNV-1a as the frameworks | 16 bytes to 1 MiB | which construction the family ships, once stability across machines is the gate and speed the second column |
| dag-and-sparse | topological order, CSR build and reachability of a plan | the returned families; `petgraph`; hand arrays | 16, 256, 4096 nodes at densities a scheduler meets | whether the const-sized form costs anything against a heap graph at plan sizes, and the first comparison these families will have had: the old benches had one arm |
| capacity-typed-indexing | indexed reads and writes of a fixed array | a typed index proven in range; a bounds-checked slice; `get_unchecked` as the yardstick | 64, 4096, 1 million elements; sequential and random | whether the capacity typestate removes the bounds check in practice, which is the gain it is sold on |
| counters-and-rates | saturating counters, durations and unit conversion in a request loop | `Num` under saturate; the host primitive; `std::time::Duration` arithmetic as the framework | 64-bit; scalar chain | whether a daemon's ordinary arithmetic, counters that must not wrap and nanoseconds times a constant, costs anything through the value |
| formatting-without-allocation | a value written to a fixed buffer | the debug-output family once it returns; `itoa`; `core::fmt` | 8 to 64 bits, decimal and hexadecimal | `obligation::debug_output_from_every_numeral_shape`, against what a daemon's log line already uses |
| build-cost | compile time and binary size of a crate naming many signatures | 10, 100 and 1000 distinct signatures; the same code over host primitives | one consumer crate | what the const obligations and the monomorphisation cost a build, measured so that compile time coming last is a known price and not an unknown one |

## What is not here, and why

No bench is listed for the float arm, the composition contracts or stochastic rounding through a
value. None of the three has a design to measure, and a bench written first would fix a shape the
canon has not chosen. Each gets its rows when its design round names what it ships.

The order to write them in is the first two tables before anything else. The plan in
`mock/research/202611130950_canon-double-check/PLAN.md` puts value benches at step 19 of 20. The
audit's finding is that this is late: the default the panel has to name in step 4 waits on the wrap
against saturate pricing, and the claim the whole design rests on, that a declared signature costs
nothing over the machine's own arithmetic, has one ad hoc disassembly under it and no timing.
