# Sketch findings: the format concept as an exponent function

**Date:** 2026-07-30
**Outcome:** **WORKS.** Zero feature gates for the plain-trait form, one allowed gate
(`const_trait_impl`, WATCH tier) for the const-callable form. The risk the proposal flagged as
gating everything downstream does not exist.
**Unblocks:** `202607300300_topic.the-formalization-proposal.md` and D50 of `202607300400`.

## Hypothesis

The proposal rests on Flocq's `generic_format β fexp`, where fixed point and floating point differ
only in the exponent function. The proposal named its own largest risk: `fexp` computing a canonical
exponent at compile time is exactly the `generic_const_exprs` shape D1 of `202607282100` forbids the
gate for, and if it cannot be expressed gate-free the proposal changes shape.

Re-reading the definition before probing anything showed the risk was misidentified.
`cexp β fexp x = fexp (mag β x)` applies `fexp` to `mag β x`, the magnitude exponent **of a value**.
The type carries only `PREC`, `EMIN` and `F`, which are plain const parameters. So `fexp` lives in
value position and never reaches type position at all.

The probes test that, and then test where the type-position risk actually is.

## What was tried, in order

Run from inside the repository so the pinned toolchain applies.

**`01_format_trait.rs`. Compiles and runs, no feature gates.** The `Format` trait carries
`fn fexp(e: i32) -> i32`. `Fixed<F>` returns `-F` and ignores magnitude entirely. `Floating<PREC,
EMIN, U>` defers to an `Underflow` policy. All four Flocq formats are expressed and their values
asserted, including that `FLT` and `FTZ` genuinely diverge below the floor:

```
Fixed<16>              at any magnitude -> -16
Floating<24,-126,Unbounded> at -1000    -> -1024   (FLX, no floor)
Floating<24,-149,Gradual>   at -200     -> -149    (FLT, clamped)
Floating<24,-149,Flushed>   at -200     -> -126    (FTZ, emin + prec - 1)
```

A function generic over `F: Format` threading the bound through its own generic code compiles, which
is the shape recorded as having overflowed the well-formedness evaluator under the old form.

**`02_const_trait.rs`. Compiles and const-evaluates, one allowed gate.** The same shape as
`pub const trait`, which is what arvo's contract surface uses throughout, so the format can sit under
`FromConstant` and the identity machinery. Every value above is computed in a `const` item rather than
at runtime, including one through a `const fn canonical<F: [const] Format>` generic over the format.

Note on syntax: the pinned nightly wants `pub const trait Foo` and `U: [const] Underflow`. The older
`#[const_trait]` attribute form is gone on this toolchain and fails with "cannot find attribute
`const_trait` in this scope".

**`03_composition_width.rs`. Compiles and runs, one allowed gate.** This is where the real
type-position risk lives, and it is not `fexp`. A fixed-point format with `I` integer and `F`
fractional bits needs a container of `I + F` bits, and `Bits<{ I + F }, S>` is the forbidden shape,
which is what ships inside `UFixed` today.

The cure is the one D2 of `202607282100` already settled: the format carries its total width as an
**associated type** (`type Total: Width`) rather than computing it in type position, and the container
falls out of the existing projection. A six-axis composition `Num<Fmt, Sign, Round, Over, Grow>`
resolves to a concrete container, with `size_of` asserted at 2 bytes for a 16-bit fixed format and 4
for a binary32-shaped one. Nothing computes in a type argument anywhere.

## What this establishes

- The proposal's stated gating risk was wrong, and in the safe direction. `fexp` is value-position by
  construction.
- The format concept, the underflow axis, and the six-axis composition all express with no forbidden
  feature and no new mechanism.
- The one genuine type-position hazard is the container width, it is pre-existing rather than
  introduced by this proposal, and the cure already decided for it applies unchanged.
- `Unbounded` (FLX) is expressible and gives a format IEEE 754 does not standardise, which is one of
  the places the proposal claimed to go beyond the standards.

## What it does not establish

The probes use `i32` for exponents and bare primitives throughout, which
`no-bare-primitives.md` forbids in shipping source. That is deliberate for a sketch testing shape
rather than surface; the real declarations take arvo types, and whether the meta-newtypes compose
cleanly at these positions is a separate question this does not answer.

Nothing here measures anything. Whether the per-width impl table costs acceptable compile time is a
bench question per `bench-and-sketch-discipline.md` and belongs in `mock/benches/`.
