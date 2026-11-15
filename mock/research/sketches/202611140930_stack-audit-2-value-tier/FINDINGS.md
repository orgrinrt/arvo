# The value tier beside hand-written code, the morning after it gained its operations

Outcome: WORKS as a reading, and it put sizes on four things. Everything in this directory is an
ad-hoc spike written for the second stack audit at `mock/research/202611140845_stack-audit/`. It is
not a bench, it did not run in the bench harness, and no number below is a measurement in the
harness's sense.

The tree it ran against is `dev` at `d0161800`. The first audit's spike
(`mock/research/sketches/202611131300_stack-audit-value-yardsticks/`) ran before the value had a
difference, a product, a quotient, a conversion or a door for a host integer, and estimated the
product by reasoning. All five exist now, so this spike times them.

## How it ran

One shared virtual machine: Intel Xeon at 2.10 GHz, four cores, x86_64 Linux, the toolchain in
`rust-toolchain.toml`, default target features, which is SSE2. Every arm is a kernel over columns of
4096, marked `#[inline(never)]`, in the one binary crate (`src/main.rs`). A value arm calls
`arvo_numeral::Num`; the hand arm beside it is what a programmer writes on the host integer for the
same declared signature. The two are asserted equal on the whole input before anything is timed, so
a hand arm that computed something else would stop the run. Each line is the median of 2001 runs
(201 for the slow arms, 21 for the slow chain) timed with `std::time::Instant`, with the fifth
percentile beside it, no pinning, no counters and no noise gate.

Three profiles, one committed run each under `outputs/`. `release` is what a consumer gets with no
profile line of its own: no link-time optimisation, sixteen codegen units. `fat` is fat LTO and one
unit. `checked` is `release` with debug assertions and overflow checks on, so the value's default
`Abort` check is live while the optimiser still runs.

The machine was doing other work during the committed runs, and it shows: a first run of the same
binary, not kept, read 20 to 30 percent lower on the slow rows and the same on the fast ones. So the
slow rows are good to a factor of about 1.4 and no better. Every statement below is about gaps far
larger than that, except where it says two arms are level.

The inputs keep the fixed-point products inside the range (Q16.16 members within plus or minus 64),
so the saturating arms never saturate in those rows. The saturating integer product over 32 bits
does leave the range on most pairs.

## One: the host door is the hand code only when the whole program is optimised together

Per element, 13 bits unsigned, from a `u16`:

| Arm | no LTO | fat LTO |
|---|---|---|
| ingest, saturating, `from_index` | 2.283 ns | 0.059 ns |
| ingest, saturating, `adapted` | 13.011 ns | 0.069 ns |
| ingest, saturating, by hand | 0.051 ns | 0.059 ns |
| ingest, wrapping, `from_index` | 1.873 ns | 0.036 ns |
| ingest, wrapping, `adapted` | 44.243 ns | 21.178 ns |
| ingest, wrapping, by hand | 0.050 ns | 0.036 ns |
| egress, `index::<u16>()` | 0.024 ns | 0.023 ns |
| egress, by hand | 0.024 ns | 0.023 ns |

`from_index` is the door `dev` gained overnight and it is the right door: under fat LTO it is the
hand code, saturating and wrapping. Without LTO it is 35 to 45 times the hand loop, and the reason
is two missing attributes. `outputs/asm-ingest_sat_from_index-release-no-lto-x86_64.s` is the
kernel's loop: a load, an indirect call through the GOT to `arvo_numeral::host::unsigned`, and then
the compare and select inline. `unsigned` and `signed` (`mock/crates/arvo-numeral/src/host.rs:49-63`)
are private, not generic and not marked `#[inline]`, so they are compiled once into the numeral
crate and a consumer without LTO calls one per element, which also keeps the loop from vectorising.
Every other function on the path is generic or marked. The egress door has no such call and is the
hand copy in both profiles.

The old route, `adapted` over an `Exact`, is still there and still what the first audit said: 13 ns
saturating and 44 ns wrapping without LTO, and the wrapping one stays at 21 ns under fat LTO.

## Two: the ring operations at an integer quantum are the host's, with one exception

Elementwise, no LTO (the fat column agrees to within the noise above):

| Arm | value | hand |
|---|---|---|
| add, 13 bits unsigned, saturating | 0.067 ns | 0.067 ns |
| sub, 13 bits unsigned, saturating | 0.052 ns | 0.052 ns |
| mul, 32 bits signed, wrapping | 0.227 ns | 0.226 ns |
| mul, 32 bits signed, saturating | 0.576 ns | 0.707 ns |
| mul, 13 bits unsigned, saturating | 0.951 ns | 0.236 ns |
| max fold, 32 bits signed | 0.193 ns | 0.193 ns |

The exception is the saturating product below the carrier's width, four times the hand arm. The
hand arm widens both factors to 32 bits, multiplies and clamps, which vectorises; the rung asks the
16-bit `overflowing_mul` and branches on its flag (`mock/crates/arvo-numeral/src/rung.rs:222-229`),
which does not. That is a reading of the two sources and of the ratio; the kernel's assembly was
not read. The same shape at the carrier's own width (32 in 32) is level with `saturating_mul`.

The folds repeat what the first spike found, now for the shipped `add`:

| Arm | no LTO | fat LTO |
|---|---|---|
| wrapping sum, 13 bits, value | 0.383 ns | 0.188 ns |
| wrapping sum, by hand, mask at each step | 0.383 ns | 0.178 ns |
| wrapping sum, by hand, mask once | 0.035 ns | 0.069 ns |
| saturating sum, 13 bits, value | 0.416 ns | 0.627 ns |
| saturating sum, by hand | 0.430 ns | 0.637 ns |

The value is exactly the per-step hand loop, and the per-step hand loop is 3 to 11 times the loop
that masks once.

## Three: the fixed-point product, the quotient and the conversion run the format's wide step

Q16.16 in a 32-bit carrier, per element, no LTO:

| Arm | value | hand | ratio |
|---|---|---|---|
| product, half to even, saturating | 354 ns | 1.06 ns | about 330 |
| product, floor, wrapping | 351 ns | 1.11 ns | about 320 |
| quotient by the constant three, half to even | 235 ns | 1.90 ns | about 120 |
| conversion to Q8.8, half to even | 239 ns | 2.50 ns | about 95 |
| the product as a dependent chain, per step | 270 ns | 4.57 ns | about 60 |
| `f32` product, for scale | | 0.176 ns | |

Under fat LTO the value column reads 361, 263, 282, 226 and 316 ns: the same within the noise, so
the cost is the arithmetic and not the calls.

`Num::mul` at a fractional quantum, `Num::div` and `Num::convert` each call the format's function on
slots (`mock/crates/arvo-numeral/src/num.rs:396-403`, `ratio.rs:58-63`, `ratio.rs:112-117`), each
under a `FIXME` that names the cause: the format's exact step is formed four 64-bit limbs wide. Read
in the source, one product is four-limb schoolbook multiplies with an overflow test at each
(`mock/crates/arvo-format/src/wide.rs:140-170`, called from `multiplication.rs:176-204`), a long
division of the four limbs (`wide.rs:231-260`), and then the remainder reduced to lowest terms by
Euclid's algorithm with two more divisions (`apply/position.rs:205-210`), before the mode looks at
it. The first audit estimated this route, by reasoning, at 10 to 50 times the hand product. It is
60 to 330 times on this host. The cost does not depend on the mode or the policy, which is what one
expects of a step that computes the exact position first.

A third of a microsecond per product is about three thousand products per millisecond on one core.

## Four: under the default check, a debug build runs the wide step for every product

`outputs/run-checked-debug-assertions.txt`, optimiser on, debug assertions on, inputs that never
leave the range:

| Arm | value under `Abort` | hand, host operator with overflow checks |
|---|---|---|
| add, 13 bits unsigned | 1.29 ns | 0.89 ns |
| mul, 32 bits signed, integer quantum | 218 ns | 1.05 ns |
| product, Q16.16 | 738 ns | |

The sum's verdict is a range test on a 128-bit index and costs about what the host's overflow flag
costs. The product's verdict is `fits_mul`, which calls the format's `product_position` and
`panic_on_overflow` (`num.rs:369-376`, `:409-412`) at every exponent, the integer ones included. So
the integer product, which is the host's product in a release build, is about two hundred times the
host's checked product in a build with debug assertions, and the fixed-point product pays the wide
step twice. `Abort` is the default check (`mock/crates/arvo-numeral/src/num.rs:57`).

This is with the optimiser on. A plain debug build was not run and will be slower still.

## Five, added in the audit's second pass: the two doors compose into a verdict

The audit's second pass claimed that a consumer can ask whether a host integer names a member with
the public surface alone, as `Num::from_index(h).index::<H>() == h` under the `Quiet` check. Three
arms were added to test it and the binary run again in two profiles
(`outputs/run-release-no-lto-pass-two.txt`, `outputs/run-fat-lto-pass-two.txt`). Both round trips
are asserted equal to the hand range test on the whole input, which holds members and non-members.

| Arm | no LTO | fat LTO |
|---|---|---|
| verdict, round trip, saturating | 1.683 ns | 0.074 ns |
| verdict, round trip, wrapping | 1.344 ns | 0.095 ns |
| verdict, hand range test | 0.052 ns | 0.108 ns |

So the composition is exact under both policies, is the hand test under fat LTO, and carries the
door's call without it, 26 to 32 times the hand test.

The second run's other rows repeat the first within the noise stated above, with one thing to note:
`from_index` without LTO read 1.39 ns here against 2.28 in the first committed run, so the door's
gap to the hand arm is 27 to 45 times across the two runs, and the audit quotes that range.

## What none of this establishes

Anything on aarch64. Any ordering finer than the gaps named. What the fixed-point arms cost when
products do leave the range. Whether the quotient's hand arm is the best hand arm, since it uses the
host's 64-bit euclidean division. And nothing here is a reason to think the hand arms are what the
value will reach by itself: they are what it has to reach, and they agreed with the format's oracle
on every one of the 4096 inputs of each row, which is a small sign that the fast arm can be held
equal to it.
