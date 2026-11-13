# The value's doors, its fold, and a hand-written fixed-point product

Outcome: WORKS as a reading, and it found a cost. Everything in this directory is an ad-hoc spike
written for the stack audit at `mock/research/202611131215_stack-audit/`. It is not a bench, it did not
run in the bench harness, and no number below is a measurement in the harness's sense.

The audit owes estimates of how the designs will perform against hand-written code, and three of them
could be checked cheaply instead of argued. The first is what a value costs to bring in from a host
integer that is only known at run time, since `Num`'s public doors are `constant`, `adapted` and
`slot`. The second is what a fold and an elementwise sum of values cost beside the same loops over the
host integer. The third is what each rounding mode costs on a fixed-point product, written by hand,
because the value has no product yet and the hand version is what it will be read against.

It ran once, on one shared virtual machine: Intel Xeon at 2.10 GHz, x86_64 Linux, the toolchain in
`rust-toolchain.toml`, default target features, which is SSE2 although the host offers AVX-512. Each
line is the median of 2001 runs timed with `std::time::Instant`, with no pinning, no counters and no
noise gate. Two profiles: `release` with fat LTO and one codegen unit, and `plain`, the same without
LTO and with sixteen units. `outputs/` holds both runs and the assembly of the library half, which
`cargo rustc --release --lib -- --emit asm` wrote; a library build is not link-time optimised, so that
file is the code a consumer without LTO gets.

## The doors are a chain of calls unless the consumer turns LTO on

`outputs/asm-release-lib-x86_64.s` shows each ingest as five calls through the GOT into `arvo-format`:
`Slot::at`, `Exact::on_grid`, `round_slot`, `complete_slot` and `Slot::index`. `egress_u13`, which is
`slot().index()` narrowed to sixteen bits, is two: `Slot::at` and `Slot::index`. The file has seventeen
calls in all and every one is in a door. The folds and the elementwise sum have none.

The two runs put a size on it, as an ad-hoc spike. Per element of a 4096-element column:

| Arm | fat LTO | no LTO |
|---|---|---|
| ingest, 13 bits saturating, through `adapted` | 0.046 ns | 9.278 ns |
| the same by hand, a compare and a select | 0.040 ns | 1.202 ns |
| ingest, 13 bits wrapping, through `adapted` | 0.260 ns | 8.674 ns |
| the same by hand, a mask | 0.034 ns | 1.201 ns |
| egress through `slot` | 0.027 ns | 3.300 ns |

The hand arms are slower without LTO too, because each is an exported function the loop in the binary
calls once per element. The comparison within a column is the one to read: without LTO a value's ingest
is seven to eight times the call it is compared with, and under fat LTO the saturating ingest is the
hand code while the wrapping ingest is still seven times the mask. Why the wrapping one stays behind
was not read in the binary's assembly. The source's completion under `Wrap` is three euclidean
remainders of 128-bit integers (`arvo-format/src/apply.rs`, the `Policy::Wrap` arm of `complete_slot`),
and the likeliest reading is that they do not reduce to a mask of a sixteen-bit value. That is an
inference and is marked as one.

What this does not show is that the doors are slow by design. `Num`'s sum goes through none of this. It
shows that the only way to bring a run-time integer into a value below pointer width today is a route
through the 128-bit oracle, and that the route is free only where the whole program is optimised
together.

## The sum is the host's sum, with or without LTO

Under both profiles the fold and the elementwise sum through the value time the same as the hand loop
to within two percent, at every arm: saturating at 13 bits, wrapping at 13 bits, wrapping at the full
16 bits, and the elementwise saturating sum. The assembly has no call in any of them. This agrees with
`mock/research/sketches/202611130342_value-add-disassembly/`, and extends it from one sum to a loop.

One thing in the numbers is worth carrying to a real bench. The wrapping fold at 13 bits costs three
times the wrapping fold at 16 bits under fat LTO (0.098 ns against 0.031) and ten times without it
(0.350 against 0.034), in the hand loop exactly as in the value. Masking the accumulator at every step
is what the format's sum means, and a loop written that way does not vectorise as well as one that
masks once at the end. Wrapping is a ring homomorphism, so masking once is the same answer. The value's
`add` cannot know it is inside a fold, which is a first, concrete case of what the composition
contracts above the numeral are for.

## The rounding mode costs little in throughput and doubles the latency of a chain

A Q16.16 product in a 32-bit carrier, over two columns of 4096, per element, fat LTO:

| Arm | ns per element |
|---|---|
| `floor`, wrap | 0.760 |
| `toward_zero`, wrap | 0.727 |
| `half_up`, wrap | 0.819 |
| `half_even`, wrap | 0.807 |
| `floor`, saturate | 0.542 |
| `half_even`, saturate | 0.923 |
| `f32` multiply | 0.086 |

The four modes are within thirteen percent of each other as throughput. All of them are about nine
times an `f32` multiply, and that gap is the target's and not the rounding's: SSE2 has no 64-bit
vector multiply that keeps the high half, so the loop stays scalar while the float loop vectorises.
`floor` with saturation reading faster than `floor` with wrap is not explained here and is the kind of
result a spike on a shared machine should not be trusted on.

As a loop-carried chain, one value decayed 100000 times, the picture differs: `floor` 1.5 ns a step,
`half_up` 1.7, `toward_zero` 3.2, `half_even` 3.1, and `half_even` with saturation 3.8 to 4.5. The tie
test and the sign test sit on the dependency chain, so a mode that needs one about doubles the latency
of a step.

The same decay shows what the latency buys. After 30000 steps of a factor just under one, starting
from 1000, the real-valued answer is 64.138673. `floor` and `toward_zero` both give 64.060745, low by
0.12 percent and falling further behind with every step. `half_up` gives 64.138687 and `half_even`
64.138672. So the bias the half-to-even tenet exists to remove is real and large against the cost of
removing it, and at this factor `half_up` removes nearly all of it at half the latency.

## What none of this establishes

Any of it on aarch64, where the vector multiply and the saturating instructions differ and the product
table is expected to look different. Any ordering finer than the gaps named. Anything about the
value's own product, which does not exist. Each of these is a row in
`mock/research/202611131215_stack-audit/BENCH-CATALOGUE.md`.
