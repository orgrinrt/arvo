**Date:** 2026-05-04
**Phase:** TOPIC
**Scope:** notko (cross-repo) and arvo-storage
**Source topics:** Round 1 expanded P0 deferral list (Expert B F21 ConstTry/ConstControlFlow); user-introduced const-feature gate constraint

# Round 5 Topic 1: ConstTry / ConstFromResidual bridge + notko `const` cargo feature

This topic captures two interlocked decisions: where the substrate's const-callable Try / FromResidual bridge lives, and how the const-trait machinery in notko gets gated behind an opt-out cargo feature.

## Background

Expert B's audit Finding 21 named that `core::ops::Try` and `core::ops::FromResidual` are non-const, so `?` on `Bool`, `Just<T>`, `Maybe<T>`, `Outcome<T, E>` cannot be used in const fn. The `?` operator's desugaring is hardcoded to the core trait names; a substrate-defined `ConstTry` does NOT make `?` work in const context. It does enable explicit `match x.const_branch() { ... }` on the same logic substrate code already routes through `Try`.

As of nightly 2026-05, `core::ops::Try` is not `pub const trait` and there is no `const_try` feature gate in flight. Waiting on core's lift is unlikely to bear fruit; the substrate ships its own bridge.

The const-trait machinery itself (`feature(const_trait_impl)`, `feature(adt_const_params)`, etc.) is unstable. notko is shared infrastructure across many independent frameworks (not only arvo). Consumers of notko who do not need the const-trait surface should be able to opt out so unstable features do not leak into their build.

## Decisions

### Decision 1: ConstTry / ConstFromResidual lives in notko

The bridge declaration lives in notko as `pub const trait ConstTry` plus `pub const trait ConstFromResidual<R = ...>`. Per `arvo-bridge-home-rule.md`, the lowest layer where the consumer types are reachable hosts the trait. notko's `Just<T>`, `Maybe<T>`, `Outcome<T, E>` plus arvo-storage's `Bool` are all consumers; notko is the lowest reachable layer.

Implementation cascade:

1. notko adds `ConstTry` + `ConstFromResidual` trait declarations.
2. notko adds `impl const ConstTry for Just<T>`, `impl const ConstTry for Maybe<T>`, `impl const ConstTry for Outcome<T, E>` plus matching ConstFromResidual impls.
3. arvo-storage adds `impl const ConstTry for Bool` plus matching ConstFromResidual impl.

Cross-repo rounds: notko PR ships the trait + notko-side impls; arvo PR ships the Bool impl. The notko PR lands first; arvo Round 5 source CL depends on the notko PR being merged.

### Decision 2: Mirror core's two-trait shape

`ConstTry` mirrors `core::ops::Try`: associated `Output` and `Residual`, methods `branch(self) -> ControlFlow<Self::Residual, Self::Output>` and `from_output(output: Self::Output) -> Self`.

`ConstFromResidual<R = <Self as ConstTry>::Residual>` mirrors `core::ops::FromResidual`: method `from_residual(residual: R) -> Self`.

Reuses `core::ops::ControlFlow` directly. ControlFlow's enum constructors are stable-const, so no notko mirror is needed.

Two-trait split (rather than combined `ConstTry`) makes later removal cleaner if core ever lifts Try to const trait: drop the bridge, keep the impls under the new core surface.

### Decision 3: notko `const` cargo feature gates const-trait surface, default on

Add cargo feature `const` to notko. Default features include `const`. With `--no-default-features` or `default-features = false`, the const-trait machinery is excluded.

```toml
[features]
default = ["const"]
const = []
```

Lib root:

```rust
#![cfg_attr(feature = "const", feature(const_trait_impl))]
#![cfg_attr(feature = "const", feature(adt_const_params))]
```

Trait declarations use `cfg_attr(feature = "const", const_trait)`:

```rust
#[cfg_attr(feature = "const", const_trait)]
pub trait ConstTry { ... }
```

Impl blocks duplicate (cfg-gated keyword position cannot be cfg-attr'd):

```rust
#[cfg(feature = "const")]
impl const ConstTry for Just<T> { ... }

#[cfg(not(feature = "const"))]
impl ConstTry for Just<T> { ... }
```

### Decision 4: Gate scope is notko-only; arvo stays ungated

arvo unconditionally requires const traits (substrate identity per `arvo-always-optimal-internals.md`); the gate would be vacuous noise. Existing Rounds 1-4 const-trait surface in arvo stays as-is. Only notko gets the feature.

### Decision 5: Mechanism is dual impl blocks

Notko's surface affected by the gate is small (3 ConstTry impls + 3 ConstFromResidual impls = 6 dual blocks plus 2 trait declarations). Hand-writing dual blocks is straightforward and transparent. A `cfg_const_impl!` declarative macro adds learning-cost and footgun surface for marginal payoff at this volume.

If notko's const-trait surface grows substantially in later rounds, revisit and introduce a macro then.

## Out of scope

- Lifting `core::ops::Try` to `pub const trait Try` in core. We're not core maintainers.
- Removing the bridge once core lifts. Track in BACKLOG; the bridge is explicitly transitional but the lift is unlikely.
- Const-trait gate retrofit to arvo or other workspace crates. arvo, hilavitkutin, vehje stay unconditionally const-trait-using; the gate is a notko-shared-infrastructure concern.

## Cross-references

- `mock/research/audits/2026_05_02_expert_b_const_trait_completeness.md` Finding 21 (the audit motivation).
- `.claude/rules/arvo-bridge-home-rule.md` (placement decision rule).
- `.claude/rules/arvo-always-optimal-internals.md` (why arvo is ungated).
- notko `src/just.rs`, `src/maybe.rs`, `src/outcome.rs` (existing non-const Try impls to mirror).
- arvo-storage `src/platform.rs:278-294` (existing non-const Try impl on Bool).
