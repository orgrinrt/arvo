# The Numbers On Record, Why They Are What They Are, And What To Expect

Part of the second stack audit; `AUDIT.md` beside this file is the entry. This file takes the
committed bench corpus, the timing that sits in sketches, and the two ad-hoc spikes the audits ran,
says what each measured against what it is cited for, names the mechanism where one can be shown and
says not known where it cannot, and then estimates where the designs will stand against hand-written
assembly, plain Rust and comparable libraries.

Three rules. A committed result is named by its file. A number an audit produced itself is an ad-hoc
spike and is called that every time: the first audit's is
`mock/research/sketches/202611131300_stack-audit-value-yardsticks/`, this one's is
`mock/research/sketches/202611140930_stack-audit-2-value-tier/`. An estimate is reasoning with its
assumptions beside it and is never written as a measurement. Paths and lines are at `dev` tip
`d0161800`. Marks are the four used throughout, in words.

## The corpus has not moved, and the first audit's account of it holds

No file under `mock/benches/` has changed since before the first audit
(`git diff --stat 7e924d0c..HEAD -- mock/benches` is empty). So the first audit's reading of it
(`mock/research/202611131215_stack-audit/NUMBERS.md`) is a claim about the same bytes, and I tested
it as one. I reopened the result files behind nine of its figures and every one matched:

| Claim of the first audit | Committed file | Matches |
|---|---|---|
| Saturating fold at length 4096: plain 41713 ns, hand NEON 362 and 255 | `mock/benches/satfold-length-l1_n12000_findings.md` | yes |
| Const gate true 1438 ns, plain 38741 | `mock/benches/satfold-const-gate_n10000_findings.md` | yes |
| Clamp once 190 ns against clamp each step 9789 | `mock/benches/warm-clamp-arity-w13_n130080_findings.md` | yes |
| Pairwise dense 54132 ns, best packed 112916, wide dense 256830 | `mock/benches/bitpack-contend-best_n41943044_findings.md` | yes |
| Xeon: 32-bit dense 1.19 ms, packed 1.05 ms | `mock/research/202611130200_bench-harness-at-head/results-x86_64-xeon/bitpack-carrier-width_n4194304_findings.md` | yes |
| Hosts are an Apple M1 and a Xeon at 2.10 GHz | the two `meta.json` files beside those | yes |

The five facts it says the whole corpus shares also hold, checked against the registry's own rows:
one host and no recorded profile (`probe::no_bench_artifact_records_its_build_profile`,
`mock/registry/probe.toml:452`), most regions older than the validation gate (`probe.toml:893`),
the argmin never run on arms that disagree (`probe.toml:1119`), and the corpus never validating a
shipped type (`probe.toml:1309`). The last is still the fact that matters most: no arm under
`mock/benches/variants/` calls a type that is in `mock/crates/` today. The night added six
operations to the value and no bench.

I did not rerun any bench. I did not read the 254 result sets one by one; beyond the nine above I
read the headline line of every family the first audit did not cover, which is the next table. A
reader was sent over the corpus and did not report before this file was written, so nothing here
rests on it.

Mark for the corpus as an instrument: going wrong, fixable in place, as on 13 November. Mark as
evidence about transforms and machines: easy to get wrong but can be right, family by family.

## Family by family

### Families the first audit judged

Its marks stand; I add what the night's code and this audit's spike say to each.

Packing against dense carriers. Packed is about half again slower than 16-bit dense on the M1 and
17 to 38 percent faster on the Xeon, same source. The M1 mechanism the first audit gave (a flat
penalty across cache sizes is the shift and mask, not memory) is an argument from scaling with no
counter behind it, and the Xeon mechanism is not known: host, instruction set and profile differ at
once. This audit's spike adds one datum from a third machine state: on the Xeon at SSE2 a 16-bit
wrapping sum that masks per step is 0.38 ns a member without LTO and 0.18 with it, so the dense
baseline on that host swings by a factor of two on the profile alone. That is enough to turn the
sign of the family by itself, and the M1 files record no profile. Mark: easy to get wrong but can
be right; right requires three hosts, the profile in every file, and the pairwise dense arm as the
baseline.

The weak dense baseline (`contend-best`). Good as it is, as a correction. Still not carried into the
registry rows that price packing against the old arm.

Index-driven and random access, and writes. Unchanged: random access is measured only inside the
first-level cache; packed writes are 18 to 20 times dense. Going wrong as a basis for the footprint
objective, fixable in place.

The container fork and the clamp family. The finding inside them is that adapting at every step is
where the time goes: a factor of 51 between clamping each step and clamping once at 13 bits. This
audit's spike reproduces the shape through the shipped type on another host: the value's wrapping
fold at 13 bits is exactly the hand per-step loop, and that loop is 3 to 11 times the loop that
masks once. So the families' central fact is now also a fact about `Num::add` as shipped. Mark:
easy to get wrong but can be right; the families are still cited for which container, which they
do not measure.

The saturating fold. Good as it is, and still the best bench in the tree: validated, controlled on
both sides, mechanism stated (a dependent chain of saturating adds at about four cycles against
lanes under the clamp-once law). Still M1 only with an aarch64-only hand arm. The value's `add`
in a loop is its slowest arm.

The quantiser. Easy to get wrong but can be right: both arms are kept from inlining, so the 16.5
times is a lower bound on the gap to a vectorised float loop. This audit's spike has the fixed-point
analogue: a hand Q16.16 product is 1.06 ns against 0.176 for `f32`, six times, at SSE2.

The wide rung, the hashes, the graph benches. As the first audit left them: a sound retirement, and
result files for a tree that is gone.

### Families the first audit did not name

Read at headline depth only: the title and the fastest-arm line of each findings file. I give what
the title claims and what the headline shows, and no mechanism, since I did not open the arms.

| Family | Headline on record | What it can carry |
|---|---|---|
| `warm-affine-density-w13` | deferred or kernel arm 317 to 400 ns, 95 to 97 percent under baseline, at every density (`mock/benches/warm-affine-density-w13_n130401_findings.md` to `_n130416`) | The title asks the right question: how much of the deferred form's gain is the optimiser collapsing an affine chain. Not read far enough to say what it answers |
| `precise-widening-theorem-l1` | 87 to 97 percent under baseline at widths 8 to 32; 0.4 percent at 60 (`..._n80501` to `_n640501`) | That knowing saturation cannot occur is worth an order of magnitude where there is headroom and nothing where there is none. Consistent with the clamp family |
| `warm-elementwise-width-l1`, `precise-elementwise-width-l1` | native or kernel arm 47 to 88 percent under baseline; 2 percent at 60 bits wrapping | With no loop-carried value the container still matters by a factor of two to eight. This is the one family that speaks to an elementwise transform |
| `bitpack-footprint-*` at seven million | dense and its copy within 0.6 percent (`mock/benches/bitpack-footprint-headtohead_n7000000_findings.md`) | The headline is the control pair; the packed row was not read |
| `decimal-quantiser-radix-sweep` | radix two 40 to 46 percent faster than decimal32 | A decimal quantiser costs under twice a binary one in that harness. Bears on decimal scaled integers; not read further |
| `wide-rung-density-w200` | arms within 1 to 4 percent | Agrees with the retirement |

Mark for this group: not judged. The honest entry is that five families are on record that neither
audit has read whole.

### The timing in sketches

The first audit found two sketches with a number. A reader of this audit swept the 61 sketch
directories and reports the same two plus the audits' own spikes, and that
`202611130342_value-add-disassembly` builds again. Its finding, the sum equal to the host's, is
confirmed twice over by the spikes. The other sketches are type-level feasibility builds whose
outcome is that they compile, and they claim no time.

Mark: good as it is. Reading assembly is the right instrument for the claim, and this audit used it
once more for the host door.

## The two ad-hoc spikes

Both ran on one shared x86_64 virtual machine at SSE2, medians of repeated runs with no pinning and
no counters. This audit's ran while the machine was busy and its slow rows are good to a factor of
about 1.4. Everything below is from `FINDINGS.md` and `outputs/` in the second sketch directory
unless it says the first.

What the numbers are and why:

| Result (ad-hoc spike) | Mechanism | How well shown |
|---|---|---|
| `from_index` 2.28 ns saturating without LTO (1.39 in a second run), 0.059 with; hand 0.051 to 0.059 | `signed` and `unsigned` at `mock/crates/arvo-numeral/src/host.rs:49-63` are not generic and not marked inline, so a consumer without LTO calls into the numeral crate per element and the loop cannot vectorise | Shown: the committed assembly has the call through the GOT inside the loop |
| `adapted` over an `Exact`: 13 ns saturating, 44 ns wrapping; wrapping stays 21 ns under fat LTO | The format's completion on a 128-bit slot index; for wrap, euclidean remainders at 128 bits | Reasoned from source, as in the first audit; binary not read |
| add, sub, max, 32-bit wrapping product equal to hand | The rung's arm is the host operator plus a mask or select (`rung.rs:91-100`) | Shown by equal times and, for add, the earlier disassembly sketch |
| Saturating product at 13 bits in 16: 0.95 ns against 0.24 by hand | The rung calls the 16-bit `overflowing_mul` and branches on the flag (`rung.rs:222-229`); the hand arm widens to 32 bits and clamps, which vectorises | Reasoned from the two sources and the ratio; assembly not read |
| Saturating product at 32 in 32: 0.58 against 0.71 by hand | Not known. The value is faster than my hand arm, so my hand arm is not the best one | Not shown |
| Wrapping fold at 13 bits: value 0.383, hand per step 0.383, hand mask once 0.035 | A mask in the loop-carried chain stops the reduction from widening into lanes; deferring it frees the vectoriser | Shown by the three arms; this is the M1 clamp family's fact on x86 |
| The same under fat LTO: 0.188, 0.178, 0.069 | Not known why one unit halves the per-step loop and doubles the mask-once loop. Different unrolling is the likely cause | Not shown |
| Q16.16 product 354 ns, quotient 235, conversion 239; by hand 1.1, 1.9, 2.5; LTO changes nothing | The format's exact step: four-limb schoolbook product (`mock/crates/arvo-format/src/wide.rs:140-170`), long division (`wide.rs:231-260`), remainder reduced by Euclid (`apply/position.rs:205-210`) | Reasoned from source and from the cost being the same in both profiles, both modes and both policies; no profile of where the 350 ns goes was taken, so the split between the three is not known |
| Under `Abort` with debug assertions: add 1.29 against 0.89; integer product 218 against 1.05; Q16.16 product 738 | `fits_mul` calls the format's `product_position` at every exponent (`num.rs:369-376`, `:409-412`); the sum's verdict is a range test at 128 bits | Reasoned from source; the doubling of the fixed-point row fits the step running twice |
| First spike: chain per step floor 1.5 ns, half up 1.7, toward zero 3.2, half to even 3.1 | Floor is a shift; half up an add and a shift; the other two need the sign or the tie on the dependent path | Reasoned; hand code only |

What neither spike shows: anything on aarch64 or wasm32, anything with a noise gate, any 64-bit
fixed-point carrier, products that leave the range, or a plain debug build without the optimiser.

## Estimates

Each row says what is compared, the estimate, and what it rests on. Spike means an ad-hoc spike
above, on one x86 host. Reasoned means no number stands behind it but the ones named. The yardstick
in every row is the best code a person would write by hand for the same declared signature, in
assembly where that differs from what the compiler gives.

### Against hand-written assembly and hand-written Rust

| What | Estimate | Rests on |
|---|---|---|
| Sum, difference, negation, order at any width up to 64 | Ratio 1.0 elementwise | Spike; disassembly sketch |
| Integer product, wrapping, or saturating at the carrier's width | Ratio 1.0 | Spike |
| Integer product, saturating, below the carrier's width | 4 times slow today; 1.0 once the rung widens and clamps | Spike for today; reasoned for after |
| Host integer in, `from_index` | 1.0 under fat LTO; 27 to 45 times slow in a default release build across two runs; 1.0 in every profile once two functions are marked inline | Spike for the first two; the third is reasoned from the assembly and is the change I am most sure of |
| Host integer out, `index` | 1.0 | Spike |
| Rejecting an untrusted host integer by the round trip through both doors, against a hand range test | 1.0 under fat LTO; 26 to 32 times slow in a default release build until the door is inline | Spike, second-pass rows: 0.074 and 0.095 ns against 0.108 with fat LTO, 1.68 and 1.34 against 0.052 without. Asserted equal to the hand test on every input |
| A fold below the carrier's width, against assembly that masks or clamps once | 3 to 11 times slow at SSE2 as shipped; 27 to 160 times slow against hand vector code under the clamp-once law on the M1 table; 1.0 once the fold is an arm that may use the law | Spike; `satfold-length-l1_n12000`. The value has no fold, so a consumer's loop over `add` is the slow side |
| Fixed-point product, quotient by a constant, conversion | 60 to 330 times slow today at Q16.16 in 32 bits | Spike |
| The same once written on the carrier | 1.0 for floor; for half to even 1.0 as throughput and about twice floor's latency on a dependent chain | Reasoned: the hand arms in the spike are that code and agreed with the oracle on every input; first spike for the latency |
| Fixed point at 64 bits, fraction 16, against kaski's own carrier | Slower than the 32-bit figure today, so above 150 times; 1.0 after, since kaski's carrier is a 128-bit multiply and a shift and the arm would be the same instructions | Reasoned; not timed at 64 bits by anyone |
| A debug build under the default check | Sum about 1.5 times the host's checked sum; integer product about 200 times; fixed-point product twice its release cost | Spike, optimiser on. Without the optimiser: not estimated |
| `Hash` | Up to twice the bytes hashed for 64 bits, sixteen times for a byte | Reasoned: the 128-bit index is hashed (`num.rs:435-439`) |
| `Bits<N>` in a table | Two to eight times the footprint for 32 bits or fewer | Reasoned: one carrier, `u64` (`mock/crates/arvo-bits/src/lib.rs:48-50`) |
| A packed column read in order | M1: 2 to 3.5 times the best dense loop. x86: not estimated; one table says packed leads against a baseline nobody has attacked | `bitpack-contend-best_n41943044`; the Xeon rerun |

### Against std Rust and no_std Rust

arvo is no_std with no allocation, and for everything above the distinction changes nothing: the
host operators, `wrapping_add`, `saturating_add`, `overflowing_mul` and the `Wrapping` and
`Saturating` newtypes are all in core and compile to the same instructions with or without std.
So "plain Rust" is one yardstick here, and it is the hand column of the spikes. Reasoned, with two
places where std does differ and arvo has nothing to compare yet:

- Run-time detection of vector features lives in std. A no_std arm keyed on the target must be
  chosen at compile time, which is what the arms concept says anyway. Where a consumer ships one
  binary for many x86 machines, hand code on std can pick AVX2 at run time and arvo as designed
  cannot. Size of the effect: the M1 fold table suggests a factor of 2 to 6 between lane counts.
- Transcendental functions: std has them on floats through the platform's library, core does not.
  kaski's integer `exp`, `ln` and `pow` are the comparable thing and arvo has none. Not estimated.

### Against existing libraries

Nothing of arvo's has been timed beside any of these. Each statement about a library is from
memory of its documentation and was not checked in this audit; treat it as an assumption.

| Library | Why it is comparable | Estimate, reasoned |
|---|---|---|
| kaski's `Fixed<FRAC>(i64)` | The code the one known consumer runs today and keeps if arvo is slower | arvo level on add and subtract; two orders slower on the product today; level after the carrier arm. arvo adds declared rounding and the override per site |
| `fixed` | Binary fixed point in a primitive carrier with the fraction in the type; wrapping, saturating and checked spellings | Its product is a widening multiply and shift, so it stands where the hand column stands: arvo 60 to 330 times slower today on a product, level after. arvo has what it lacks (rounding as a coordinate, widths below the carrier, other radixes, computed laws); it has what arvo lacks (traits, formatting, parsing, transcendentals) |
| `core::num::Wrapping`, `Saturating` | What a Rust programmer reaches for first | Level at the carrier's width. Below it they cannot say the width at all |
| `ux`, `arbitrary-int` | Integers of odd width in a wider carrier, masked per operation | Level on arithmetic. Their constructor from a host integer is a mask or compare in every profile; arvo's is that only under LTO today |
| `rust_decimal` | Decimal with a 96-bit mantissa and a run-time scale; what money code uses | A decimal scaled integer in arvo has its scale in the type and a 64-bit carrier, so a sum should be one add against a scale alignment and a multi-word add: expected several times faster. Its product today runs the wide step and is expected slower than `rust_decimal`'s. Neither timed |
| `bitvec`, `fixedbitset` | What the bit-set obligation will be read against | Nothing to estimate: `Bits<N>` has no set operation. The obligation's own yardstick is one instruction per operation at 64 bits or fewer (`mock/registry/obligation.toml:25`) |
| Columnar bit-packing codecs (Arrow's, the `bitpacking` crate) | Packed columns decoded in blocks with vector code | They decode a block at a time and never offer random writes. The committed packed arms decode per member. Expect the block form to close most of the M1 gap on ordered reads; this is a reason the shared placement should bench a block arm |
| `rustc_apfloat`, Berkeley SoftFloat | Software IEEE 754 with every rounding direction | A parity reference, not a speed one. The binary suites already use the host's floats; these would give the directed modes and the decimal side something executable to stand against |
| MATLAB `fi` | A parity target | Not a speed comparison. No output of the tool is stored |

## What the numbers add up to

Three sentences carry it. Where the value's operation is carrier arithmetic it is the hand code,
on the one host tried, and that now covers the whole integer ring, the order and one door. Where it
is not, it is two orders of magnitude away, and the three operations in that state are the ones the
known consumer needs. And every figure that says either thing is an ad-hoc spike, because the
committed corpus measures no shipped type and the plan does not reach a value bench until step 19
(`mock/research/202611130950_canon-double-check/PLAN.md:125`).
