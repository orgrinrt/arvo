# Findings: const-trait arithmetic on every arvo numeric primitive

**Date:** 2026-05-02
**Sketch:** `sketch.rs` (compiles clean on nightly with `feature(const_trait_impl)` + `feature(const_ops)`)
**Round:** 202605021200

## Validated facts

The substrate can land `impl const` on every `core::ops::*` trait used by arvo numerics. The sketch validates the full op matrix on USize, the Cap layered-composition shape, the UFixed-shaped const-generic case, the Mask bit-op case, and the load-bearing hilavitkutin Depth case. Every const expression in the sketch folds at compile time; the assert! statements all pass.

The full op matrix on USize that compiles at const time:

- `Add`, `Sub`, `Mul`, `Div`, `Rem`
- `Shl<USize>`, `Shr<USize>`
- `BitAnd`, `BitOr`, `BitXor`, `Not`

Both trait-method syntax (`x.add(y)`) and operator syntax (`x + y`) const-evaluate. The implementing crates can use either.

## Required nightly features

Two features are needed together at the implementing crate root:

- `feature(const_trait_impl)`. Already enabled in `arvo-transparent`. Allows `impl const Trait for T`.
- `feature(const_ops)`. NEW; not currently enabled in arvo. Required to use the standard library's `core::ops::*` traits as const traits at const-context call sites. Without it, every const-context call to `Add::add` etc. fails with `Add is not yet stable as a const trait`.

The error message rustc emits on the missing feature is the obvious one: `error: Add is not yet stable as a const trait. add #![feature(const_ops)] to the crate attributes to enable.` That establishes that the feature is what unlocks std-trait const usage; not gating consumer adoption is the design choice.

Both features ship together at every crate that authors `impl const` on a `core::ops::*` trait. Crates that only use the const-trait surface as consumers (call `x + y` in a const context) need the same feature on their crate root because operator desugaring at const time goes through the const trait method.

## Layered composition: the recommended pattern

Cap wraps USize. When Cap needs arithmetic, the body delegates through USize's const Add rather than reaching into the inner `usize`:

```rust
// Recommended: layered composition, no raw .0.0 access
impl const Add<Cap> for Cap {
    type Output = Cap;
    fn add(self, rhs: Cap) -> Cap {
        Cap(self.0.add(rhs.0))  // self.0 is USize, USize::add is const
    }
}

// NOT recommended: reaches through to the bare usize
impl const Add<Cap> for Cap {
    type Output = Cap;
    fn add(self, rhs: Cap) -> Cap {
        Cap(USize(self.0.0 + rhs.0.0))  // raw usize arith inside
    }
}
```

Both compile. The first stays inside the typed surface (USize::add is the load-bearing op; Cap's body composes through it). The second reaches to bare `usize` and wraps back, which is exactly the field-access shape this round eliminates.

Generalises: any wrapper-of-arvo-primitive that needs arithmetic should compose through the inner type's const ops, not through the inner-inner bare primitive. UFixed wraps Bits, IFixed wraps Bits, Cap wraps USize, Mask64 wraps u64 (the bottom-level wrapper; here raw u64 ops are the only available ops).

## Mixed-arity policy: validated

The negative test (commented out in the sketch): `const _: USize = USize(1) + 1usize;` does not compile. rustc rejects mixed `USize + bare-usize` because no `Add<usize>` impl exists. Good. Consumers must use `USize::ONE` / `USize::ZERO` / `USize(N)` as the const-time literal form.

The const constants `USize::ZERO`, `USize::ONE`, `USize::MAX` cover the common literal cases; consumer call sites that need specific literals construct via `USize(N)`.

## Per-primitive notes for the SRC apply

**USize**: fresh full-matrix surface. Every op listed above. Inherent constants: ZERO, ONE, MAX. PartialOrd/Ord need fresh impls; the `derive` PartialEq/Eq stays.

**Cap**: same surface as USize, body delegates through USize's const ops.

**UFixed<I, F, S>**: existing 4 ops impls (Add/Sub/Mul/Div) at `arvo/src/ufixed.rs:153,167,181,195`. Convert each to `impl const`. The existing bodies use the underlying primitive's arithmetic; once `feature(const_ops)` is on the arvo crate root, the conversion is mechanical (add `const` keyword to the trait-impl line). Add ZERO / ONE inherent associated constants per the const-generic shape (verify nightly accepts `pub const ZERO: UFixed<I, F, S> = ...;` on the generic struct; sketch's `UFixedToy` confirms it does).

**IFixed<I, F, S>**: same as UFixed, at `arvo/src/ifixed.rs:142,156,170,184`. ZERO / ONE / MINUS_ONE inherent.

**FastFloat<F>, StrictFloat<F>**: existing core::ops impls per task #52. Convert to `impl const`. ZERO / ONE inherent constants.

**Bits<N, S, Sign>**: bit-op impls in `arvo-bits-contracts` and `arvo-bits`. Convert to const. The `BitLogic` / `BitSequence` / `BitAccess` traits in `arvo-bits-contracts` are already `pub const trait` per the round 202604271346 work (verify each sub-trait, convert any non-const stragglers).

**Mask64, Mask256, BitMatrix**: bit-ops only. Convert to const.

**IBits, FBits, Width, BitWidth**: thin newtypes over UFixed<7, 0, Hot> shapes. Once UFixed's const ops are wired, these inherit naturally. No fresh impls needed (verify during apply).

**Bitfield<...>** (round 202604200055-ish, task #127): audit during apply. If it has ops, convert.

**ContentHash**: typed alias on a Bits-shape. No ops; no work.

## Const-context concerns

One concern surfaced and resolved: `feature(const_ops)` is currently named that way and was previously called `feature(const_trait_impls_for_std)` or similar. The naming may shift before stabilisation. The substrate ships under nightly already (the workspace policy is "compile-time last; nightly is the contract"), so feature renames are tracked as they happen. Document the current name in the relevant DESIGN.md.tmpl with a footnote.

A second concern: the const-trait infrastructure has interactions with const-generic-position requirements. None of the sketch's stress instances exercise an arvo type *inside* a `const N: ArvoType` position other than `USize` and `Cap` already in `ConstParamTy`. Round 202604271346 already wired this path. No new const-generic-position changes required.

## Consumer gates

Once arvo lands the const surface, hilavitkutin's `Depth::D` const becomes:

```rust
// Before: USize(R::D.0 + 1) — field access on USize
// After:  R::D + USize::ONE  — composition through const Add
impl<H, R: Depth> Depth for (H, R) {
    const D: USize = R::D + USize::ONE;
}
```

Hilavitkutin's `hilavitkutin-api/src/lib.rs` adds `#![feature(const_ops)]` to the crate root, since the `+` desugar at the const-context call site routes through the const trait.

Ditto for any other consumer that hits the same shape (vehje and viola once they author plan-stage code).

## Sweep targets in arvo source

The grep in the topic addendum surfaced ~50 sites in arvo source that use `.0` field-access on USize / Cap, primarily in `arvo-bitmask/src/ops.rs` (bit-position arithmetic) and `arvo-comb/src/{dp,greedy}.rs` (loop counters). Every site has a mechanical migration to the const-trait surface; the SRC CL enumerates each file and the per-line rewrite.

## Stress instance scale

Per the workspace process recommendation surfaced in PR #46's review: every type-level mechanism gets a stress instance at 3× the largest realistic consumer scale. Here: USize covers consumers up to `usize::MAX` by definition, so no scale stress is meaningful; the stress is operator coverage (every op in the matrix at const time). UFixed-shaped const-generics covered at W=7 (representative of the meta-bit newtypes) and W=64 (top of single-container range). The matrix exhaustiveness is the load-bearing validation, not type-instance scale.

## Out-of-scope items deferred

- Lint rule for `.0` access on arvo primitives outside the implementing crate. Filed as follow-up in the original topic; ships in a future round once #300 is closed.
- Hilavitkutin-side migration of `Depth::D` and any other const-context arithmetic in hilavitkutin source. Not part of the arvo round; ships as a one-line update on PR #46 once arvo merges.
- Clause / vehje / viola adoption. Each lands in its own follow-up round when consumer authors hit a const-arith case.
