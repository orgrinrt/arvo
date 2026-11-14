# The value tier beside hand-written code, the morning after it gained its operations

Outcome: WORKS as a reading, and it put sizes on three things. Everything in this directory is an
ad-hoc spike written for the second stack audit at `mock/research/202611140845_stack-audit/`. It is
not a bench, it did not run in the bench harness, and no number below is a measurement in the
harness's sense.

The tree it ran against is `dev` at `d0161800`. The first audit's spike
(`mock/research/sketches/202611131300_stack-audit-value-yardsticks/`) ran before the value had a
difference, a product, a quotient, a conversion or a door for a host integer, and estimated the
product by reasoning. All five exist now, so this spike times them.

## How it ran

One shared virtual machine: Intel Xeon at 2.10 GHz, x86_64 Linux, the toolchain in
`rust-toolchain.toml`, default target features, which is SSE2. Every arm is a kernel over columns of
4096, marked `#[inline(never)]`, in the one binary crate (`src/main.rs`). A value arm calls
`arvo_numeral::Num`; the hand arm beside it is what a programmer writes on the host integer for the
same declared signature. The two are asserted equal on the whole input before anything is timed, so
a hand arm that computed something else would stop the run. Each line is the median of 2001 runs
(201 for the slow arms, 21 for the slow chain) timed with `std::time::Instant`, with the fifth
percentile beside it, no pinning, no counters and no noise gate.

Two profiles. `release` is what a consumer gets with no profile line of its own: no link-time
optimisation, sixteen codegen units. `fat` is fat LTO and one unit. `outputs/` holds both runs.

The inputs keep the products inside the range (Q16.16 members within plus or minus 64), so the
saturating arms never saturate in the product rows. The saturating integer product over 32 bits
does leave the range on most pairs.

## One: the host door is the hand code only when the whole program is optimised together

Per element, 13 bits unsigned, from a `u16`:

| Arm | no LTO | fat LTO |
|---|---|---|
| ingest, saturating, `from_index` | 1.627 ns | 0.061 ns |
| ingest, saturating, `adapted` | 10.339 ns | 0.061 ns |
| ingest, saturating, by hand | 0.051 ns | 0.061 ns |
| ingest, wrapping, `from_index` | 1.306 ns | 0.037 ns |
| ingest, wrapping, `adapted` | 34.470 ns | 20.304 ns |
| ingest, wrapping, by hand | 0.037 ns | 0.039 ns |
| egress, `index::<u16>()` | 0.051 ns | 0.022 ns |
| egress, by hand | 0.023 ns | 0.021 ns |

`from_index` is the door `dev` gained overnight and it is the right door: under fat LTO it is the
hand code, saturating and wrapping. Without LTO it is 25 to 35 times the hand loop, and the reason
is one line. `outputs/asm-ingest_sat_from_index-release-no-lto-x86_64.s` is the kernel's loop: a
load, an indirect call through the GOT to `arvo_numeral::host::unsigned`, and then the compare and
select inline. `unsigned` and `signed` (`mock/crates/arvo-numeral/src/host.rs:49-63`) are private,
not generic and not marked `#[inline]`, so they are compiled once into the numeral crate and a
consumer without LTO calls them per element, which also keeps the loop from vectorising. Every
other function on the path is generic or marked. The egress door has no such call and is within a
load of the hand copy in both profiles.

The old route, `adapted` over an `Exact`, is still there and still what the first audit said: 10 ns
saturating and 34 ns wrapping without LTO, and the wrapping one stays at 20 ns under fat LTO.

## Two: the ring operations at an integer quantum are the host's, with one exception

Elementwise, no LTO (the fat column agrees to within noise in `outputs/run-fat-lto.txt`):

| Arm | value | hand |
|---|---|---|
| add, 13 bits unsigned, saturating | 0.072 ns | 0.072 ns |
| sub, 13 bits unsigned, saturating | 0.049 ns | 0.069 ns |
| mul, 32 bits signed, wrapping | 0.220 ns | 0.219 ns |
| mul, 32 bits signed, saturating | 0.555 ns | 0.682 ns |
| mul, 13 bits unsigned, saturating | 0.922 ns | 0.228 ns |
| max fold, 32 bits signed | 0.183 ns | 0.179 ns |

The exception is the saturating product below the carrier's width, four times the hand arm. The
hand arm widens both factors to 32 bits, multiplies and clamps, which vectorises; the rung asks the
16-bit `overflowing_mul` and branches on its flag (`mock/crates/arvo-numeral/src/rung.rs:222-229`),
which does not. That is a reading of the two sources and of the ratio; the kernel's assembly was
not read. The same shape at the carrier's own width (32 in 32) is level with `saturating_mul`.

The folds repeat what the first spike found, now for the shipped `add`:

| Arm | no LTO | fat LTO |
|---|---|---|
| wrapping sum, 13 bits, value | 0.382 ns | 0.108 ns |
| wrapping sum, by hand, mask at each step | 0.382 ns | 0.108 ns |
| wrapping sum, by hand, mask once | 0.034 ns | 0.039 ns |
| saturating sum, 13 bits, value | 0.414 ns | 0.413 ns |
| saturating sum, by hand | 0.403 ns | 0.400 ns |

The value is exactly the per-step hand loop, and the per-step hand loop is 3 to 11 times the loop
that masks once.

## Three: the fixed-point product, the quotient and the conversion run the format's wide step

Q16.16 in a 32-bit carrier, per element, no LTO (fat LTO does not change these):

| Arm | value | hand | ratio |
|---|---|---|---|
| product, half to even, saturating | 272 ns | 1.02 ns | about 270 |
| product, floor, wrapping | 253 ns | 0.81 ns | about 310 |
| quotient by the constant three, half to even | 224 ns | 1.83 ns | about 120 |
| conversion to Q8.8, half to even | 211 ns | 2.42 ns | about 90 |
| the product as a dependent chain, per step | 260 ns | 4.42 ns | about 60 |
| `f32` product, for scale | | 0.104 ns | |

`Num::mul` at a fractional quantum, `Num::div` and `Num::convert` each call the format's function on
slots (`mock/crates/arvo-numeral/src/num.rs:396-403`, `ratio.rs:58-63`, `ratio.rs:112-117`), each
under a `FIXME` that names the cause: the format's exact step is formed four 64-bit limbs wide. The
first audit estimated this route, by reasoning, at 10 to 50 times the hand product. It is 60 to 300
times on this host. The cost does not depend on the mode or the policy, which is what one expects of
a step that computes the exact position first, and link-time optimisation does not touch it, so it
is the arithmetic and not the calls.

A quarter of a microsecond per product is about four thousand products per millisecond on one core.

## What none of this establishes

Anything on aarch64. Any ordering finer than the gaps named: the fifth percentile and the median
differ by up to 15 percent on the slow rows. What the fixed-point arms cost when products do leave
the range. Whether the quotient's hand arm is the best hand arm, since it uses the host's 64-bit
euclidean division. The debug build, which was not run. And nothing here is a reason to think the
hand arms are what the value will reach by itself: they are what it has to reach, and the product
row says how far away it is today.
