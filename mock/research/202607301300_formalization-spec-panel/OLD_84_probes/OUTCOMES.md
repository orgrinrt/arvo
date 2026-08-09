# 84_probes outcomes

Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, host `aarch64-apple-darwin`, resolved from
the repo's `rust-toolchain.toml`, every command run **inside the tree** (the identical command run
outside resolves to stable `1.94.0`, which files 73, 75 and 82 established as standing practice to
check, and which caught one wrong result in this session: see probe 5b). Edition 2024 throughout.
The bench harness was **not run**; its orchestrator overwrites committed artifacts (`81:38-44`), and
nothing below is a runtime timing claim. Every claim is either a compile, a run with asserted counts,
or a disassembly.

## probe_1_refusal_is_overrange.rs

`rustc --edition 2024 -O`, built and **run, exit 0**. Zero warnings. Exhaustive over file 80's own
model (r = 10, p = 3, quantum exponents -2..=1, |D| = 4000, 16,000,000 operand pairs), every count
asserted, plus a negative control that must differ.

```
per-(x,q) cells examined: 16000
A naive-digit-count refusals: 5679000
B above-far-point         : 5679000
C extended-grid overflow  : 5679000
A != B cells: 0
A != C cells: 0
B != C cells: 0
cells in the half-ulp window (max, boundary): 0
cells strictly inside the last-ulp gap: 0
negative control (nonzero mantissa): 15984 cells, differs as required
OK: quantize's failure is OverRange on the fixed-exponent numeral F_q
```

Establishes: file 80's 5,679,000 reproduces exactly through an independently written construction,
**and** the refusal predicate is cell-for-cell identical to the design's own out-of-range predicate
(`78:288-293`, the extended-grid boundary at half a top ulp with the tie resolved by even) on the
numeral `At<N, Q>` whose exponent is fixed at the requested quantum. The last two counts establish
that the agreement is a theorem rather than a model coincidence: the quotient `value(x) / r^q` is
never strictly between `r^p - 1` and `r^p`, so the half-ulp window where a rounding-boundary reading
could differ from a digit-count reading is unreachable for `quantize` at any operand pair.

## probe_2_quantise_as_a_crossing.rs, probe_2_run.rs

`rustc --edition 2024 --crate-type lib`, **compiles clean, exit 0**, zero feature gates. Every plan
const (`ULP`, `MODULUS`, `MAX_MANTISSA`, `FAR_POINT`) is an associated const of the target type and
is asserted in const position, so the compile is part of the check. Three const-position cases pin
the standard's three behaviours: exact (no event), inexact (value rounded and delivered), and the
hard failure (mantissa needs `p + 1` digits).

Runner, **run, exit 0**, whole matrix, counts asserted:

```
Precise sweep: cells=16000 refusals=5679 disagreements=0
Warm sweep: cells=16000 clamped=5679 wrong=0
Hot sweep: cells=16000 wrapped=5679 wrong=0
size_of Total = 16, size_of Fallible = 32
```

Establishes: one arithmetic body serves all four preset rows with the resolution acting as a handler
(the shape `05_probes/a_handler.rs` compiled, folded into `Quantisation::Fallibility<T>` at
`70:196-203`); the refusing tier refuses on exactly probe 1's cells; the two total tiers are total
and land on their own ratified answers (far point under `Warm`/`Cold`, reduce-modulo under `Hot`) on
exactly the same cells. Three rows are exercised, not a sample of one.

## probe_2b_conformance_refused.rs

**Expected-fail. Must not compile.** `rustc --edition 2024 --crate-type lib` refuses with `E0277`:

```
error[E0277]: the trait bound `Warm: ConformingQuantise` is not satisfied
    |     conforming_quantise::<Dec3, E0, Warm>(vx)
    |                                     ^^^^ unsatisfied trait bound
help: the trait `ConformingQuantise` is not implemented for `Warm`
help: the trait `ConformingQuantise` is implemented for `Precise`
note: required by a bound in `conforming_quantise`
```

Establishes: a consumer needing the standard's own behaviour states it as a bound and is refused at
the declaration site, with rustc's own diagnostic naming the remedy. Nothing about conformance is
checked at run time, carried in a value, or published in a grade a consumer must read.

## probe_3_quantum_binding_time.rs

`rustc --edition 2024 -O --crate-type lib --emit=asm`, **compiles clean**, with a const-position
assertion that the typed and dynamic shapes compute the same function over the model's whole
mantissa range. Instruction counts read from the emitted assembly per function:

| shape | instructions | hardware divisions | branches |
|---|---:|---:|---:|
| `typed_standalone` (quantum is a type) | 20 | 0 | 2 |
| `dynamic_standalone` (quantum is a datum) | 66 | 1 | 14 |
| `typed_loop`, 64 elements | 43 | 0 | 1 |
| `dynamic_loop`, 64 elements | 237 | 2 | 30 |

The typed loop **vectorises to NEON 2-wide** with no branch but the back-edge: the division by the
quantum is strength-reduced to `smulh` plus shifts against a magic constant, the ties-to-even rule
becomes `cmgt`/`cmeq`/`and`/`orr` on vectors, and the clamp becomes `cmgt`/`bif`. The dynamic loop
keeps an `sdiv` and a divide-by-zero check (`cbz x12, LBB0_49`) inside the per-element body, because
the quantum's scale is not known.

## probe_4_where_a_partial_operation_pays.rs, probe_4_run.rs

`rustc --edition 2024 -O --crate-type lib --emit=asm`, **compiles clean**, with const-position checks
that the three homes agree on admissible operands, that the layout claims hold, and that a bottom
sorting below every value is silently discarded by a total-order maximum.

```
size_of i64          = 8
size_of bottom (i64) = 8
size_of Refusing     = 16
size_of Nz           = 8
column of 64: plain=512B carrier=1024B
```

| loop, 64 elements | instructions | divisions | branches |
|---|---:|---:|---:|
| `bottom_loop` (value home) | 18 | 1 | 3 |
| `carrier_loop` (result-type home) | 21 | 1 | 3 |
| `total_loop` (declaration home) | 19 | 1 | 2 |
| `admit_column` (the relocated check) | 10 | 0 | 2 |

**The finding that corrects the natural expectation**: at a dividing operation the three homes cost
almost the same in instructions, because the division dominates. The refusing carrier's real price is
**layout**, a 2x column footprint, which is the axis arvo exists for. And `total_loop` still carries a
`cbz` plus a `panic_const_div_by_zero` landing pad and the whole unwind apparatus, because a
`repr(transparent)` newtype over `i64` carries no validity range: the type system knows the divisor
is nonzero and the optimiser does not.

## probe_5_the_proof_the_optimiser_can_see.rs

`rustc --edition 2024 -O --crate-type lib --emit=asm`, **compiles clean**, zero feature gates, sizes
asserted in const position.

| loop, 64 elements | instructions | divisions | branches | panic references |
|---|---:|---:|---:|---:|
| `newtype_loop` (`Nz`, proof in the type system only) | 19 | 1 | 2 | 2 |
| `niche_loop` (`core::num::NonZeroI64`) | 16 | 4 (unrolled 4x) | 1 | 0 |
| `checked_loop` (bare `i64`, check written out) | 13 | 1 | 2 | 0 |

`niche_loop` unrolls four ways, keeps only the loop back-edge, and emits no check and no landing pad.
Layout, asserted: `size_of::<Option<NonZeroI64>>() == 8` against `size_of::<Option<i64>>() == 16` and
`size_of::<Option<Nz>>() == 16`.

## probe_5b_pattern_type_reference.rs

**Reference measurement only, not adoptable, not a proposal.** `rustc --edition 2024 -O --crate-type
lib --emit=asm` compiles with `#![feature(pattern_types, pattern_type_macro)]`, and rustc's
`internal_features` lint fires: "the feature `pattern_types` is internal to the compiler or standard
library ... using it is strongly discouraged".

```
pattern_loop: instr=16 sdiv=0 udiv=4 branch=1 panic=0
size_of Pos64 = 8, size_of Option<Pos64> = 8, size_of Result<Pos64, ()> = 8
```

Identical to `NonZeroI64`'s shape, with one thing more: the range `1..` tells LLVM the divisor is
positive as well as nonzero, so it emits `udiv` rather than `sdiv`. The design's `Domain: SignDomain`
member is exactly that kind of statement.

**Two toolchain facts worth pinning so nobody re-derives them.** First, `core`'s niche mechanism on
this pin is `pattern_type!` in `core/src/num/niche_types.rs`, whose module attribute reads
`#![unstable(feature = "temporary_niche_types", issue = "none", reason = "for core, alloc, and std
internals until pattern types are further along")]`. Second, the older attribute route is closed
outright: `#[rustc_layout_scalar_valid_range_start(1)]` is rejected on this pin even under
`#![feature(rustc_attrs)]`, with "attributes starting with `rustc` are reserved for use by the
`rustc` compiler" and "cannot find attribute ... in this scope". Any memory of that attribute as the
mechanism is stale.

**Vetting outcome**, run per `unstable-features.md`'s own procedure and its std-internal carve-out:
the carve-out's first step is "check whether a stable or public wrapper suffices", and one does, so
**arvo does not adopt this feature**. Probe 6 is that wrapper's general form.

## probe_6_the_refusing_carrier_for_free.rs, probe_6_run.rs

`rustc --edition 2024 -O`, **compiles clean and runs, exit 0**, zero feature gates, stable mechanism
only. Round-trip exactness asserted in const position over the **whole** 65,535-value domain rather
than a sample, plus the assertion that the one spent pattern is refused rather than aliased.

```
Biased=2 Option<Biased>=2 Result<Biased,()>=2
Plain=2 Option<Plain>=4
column of 64: biased-refusing=128B plain-refusing=256B
```

| sum over 64 elements | instructions | branches | vector ops |
|---|---:|---:|---:|
| `biased_sum` | 31 | 0 | 25 |
| `plain_sum` | 22 | 0 | 16 |

Establishes: storing the datum biased by one in a `core::num::NonZero` inherits the stable niche, so
the infallible and refusing tiers are the same width and a refusing column is half what it would
otherwise be. The debias is nine extra instructions across 64 elements, fully inside the vector
pipeline (`add.8h` against a broadcast `-1`), with no branch and no loss of vectorisation.

## The oracle run (not a Rust probe)

CPython's `decimal` module, an implementation of the General Decimal Arithmetic specification that
IEEE 754-2019's decimal formats align with, used as an oracle for one premise check at `prec = 3`:

```
quantize(1.23,  0.01) -> 1.23   flags: {}
quantize(1.234, 0.01) -> 1.23   flags: {Inexact, Rounded}
quantize(1234,  1)    -> InvalidOperation
quantize(999,   1)    -> 999    flags: {}
quantize(1.23, 1) = 1   quantize(1.23, 1.0) = 1.2   with Decimal('1') == Decimal('1.0')
```

Establishes: `quantize` **does** change the value, and signals inexact rather than failing, when the
target quantum is coarser; the hard failure is a separate event; and file 80's datum dependence
reproduces on a real conforming implementation.
