# Sketches: arvo Round 4 const-hash and bounded-generic narrowing

**Date**: 2026-05-03T15:48Z
**Status**: BOTH SKETCHES PASS WITH TWO FINDINGS APPLIED. READY FOR DOC CL.
**Tracks**: task #314 (Round 4 expanded scope).

## What this directory exists to validate

Round 4 introduces two new const traits (`NarrowFromU64<N, S, Sign>` and `ConstHash<N, S, Sign>`) plus a sealed-`Project` + `Picker` visibility change. The cross-crate `.hash()` failure recorded as a #316 apply-time finding showed that this category of trait-solver chain (const-impl + Pattern C blanket impl) is fragile. These sketches validate that the new shape resolves correctly before the doc CL locks.

Per the workspace `cl-claim-sketch-discipline.md` rule.

## Sketches

| Sketch | Outcome | Findings |
|---|---|---|
| `01_narrow_from_u64_orphan.rs` | **WORKS** | `NarrowFromU64<N, S, Sign>` blanket impls per native primitive (u8/u16/u32/u64/u128 + signed siblings) keyed on the `(S, Sign)` pair compile without E0119 conflicts. WideBits<BYTES, A> wide-bucket impl coexists. The `(S, Sign)` constraint partitions the impls so primitive Self types carry only the `(S, Sign)` combinations where they are the dispatched container under BitsContainerFor; rustc accepts all 25 native impls + 1 wide impl in one crate. |
| `02_const_hash_cross_crate/` | **WORKS** | `ConstHash<N, Hot, Unsigned>` invoked from a downstream crate resolves through the full chain: `<Fnv1a<N> as ConstHash<...>>::hash_const` → `<Hot as BitsContainerFor<N, Unsigned>>::T as [const] NarrowFromU64<N, Hot, Unsigned>>::narrow_u64` → `<Picker as Project<{tag(N)}, Sign, {bytes_for(N)}, Hot>>::T`. Both compile-time const-context calls and runtime calls produce correct hash values. The `HasherExt` cross-crate failure from #316 does NOT recur because the Pattern C dispatch is explicit at the trait-arg position. |

## Findings recorded during sketching

### Finding 1: `[const]` host-effect bound required

The supertrait constraint on `ConstHash` must use the `[const]` syntax (not the bare trait name) for cross-crate dispatch in const context to satisfy the const-trait host-effect:

```rust
// WRONG (cross-crate const dispatch rejected with E0277):
pub const trait ConstHash<const N: u16, S: Strategy, Sign: Signedness>: Sized
where
    S: BitsContainerFor<N, Sign>,
    <S as BitsContainerFor<N, Sign>>::T: NarrowFromU64<N, S, Sign>,
{ ... }

// RIGHT:
pub const trait ConstHash<const N: u16, S: Strategy, Sign: Signedness>: Sized
where
    S: BitsContainerFor<N, Sign>,
    <S as BitsContainerFor<N, Sign>>::T: [const] NarrowFromU64<N, S, Sign>,
{ ... }
```

The same applies on the per-algorithm impl's where clause. Doc CL captures this in the `ConstHash` declaration and impl-skeleton sections.

### Finding 2: `Picker` cannot be `pub(crate)`; seal `Project` instead

The senior reviewer's "demote `Picker` to `pub(crate)`" recommendation cannot land as written. `Picker` appears in the public associated type body of every `BitsContainerFor` impl:

```rust
type T = <Picker as Project<{ tag_hot_cold(N) }, Sign, { bytes_for_u16(N) }, Hot>>::T;
```

A `pub(crate) Picker` triggers `E0446: private type Picker in public interface`. The actual gate against downstream impls is **sealing `Project`** (adding a sealed supertrait bound). `Picker` itself stays `pub` because it carries no useful operations — the only useful thing about `Picker` is which `Project` impls exist for it, and those are sealed. A consumer can name `Picker` but cannot extend the `Project` cascade.

Doc CL captures this: seal `Project`, keep `Picker` public, document the visibility rationale.

### Finding 3 (corollary): `[const]` host-effect propagation pattern

When a const trait carries a where-clause bound on an associated type produced by another const trait, the inner trait must be referenced with `[const]` even if the outer trait is `pub const trait`. The substrate's existing const-trait definitions should be audited for this pattern; `BitsContainerFor` itself does not currently carry such cross-trait associated-type bounds, so the only round-4 surfaces affected are `ConstHash` and any future const trait that bounds on `<X as BitsContainerFor<...>>::T: SomeConstTrait`.

## How to reproduce

Sketch 01 (single-file rustc):

```bash
cd mock/research/sketches/202605031548_round_4_const_hash_and_narrowing
rustc +nightly --edition 2024 -o /tmp/sketch_01 01_narrow_from_u64_orphan.rs
/tmp/sketch_01
```

Sketch 02 (multi-crate workspace):

```bash
cd mock/research/sketches/202605031548_round_4_const_hash_and_narrowing/02_const_hash_cross_crate
cargo +nightly run --release
```

Both should print their final-line "WORKS" announcement.

## Architecture validated

1. **`NarrowFromU64` orphan + coherence**: per-primitive blanket impls keyed on `(S, Sign)` compile without overlap. Generic `WideBits<BYTES, A>` wide-bucket impl coexists.
2. **`ConstHash` cross-crate dispatch**: the trait-solver chain ConstHash → NarrowFromU64 → BitsContainerFor → Project resolves at both compile time and runtime in a downstream crate. Replaces the per-N `hash_const` inherent pattern.
3. **`[const]` host-effect bound**: the bound on `<S as BitsContainerFor<N, Sign>>::T: NarrowFromU64<N, S, Sign>` requires the `[const]` qualifier syntax for cross-crate const dispatch.
4. **Sealed `Project`**: prevents downstream `Project` impls; `Picker` stays public for visibility-of-public-associated-type reasons.
5. **`HasherExt` failure does not recur**: the new shape is structurally different (Pattern C dispatch through trait-arg position rather than blanket-impl-over-blanket-impl chain).

## What this means for the doc CL

The doc CL captures:

- `ConstHash<N, S, Sign>` with `[const]` host-effect bound on the supertrait constraint (Finding 1).
- `NarrowFromU64<N, S, Sign>` with `(S, Sign)`-partitioned per-primitive blanket impls + WideBits wide-bucket impl.
- Sealing of `Project`; Picker remains `pub` (Finding 2).
- Bridge family (ConstFrom / ConstTryFrom / ConstDeref / ConstAsRef) follows the same `[const]` host-effect pattern wherever bounds project through const-trait associated types (Finding 3 corollary).
- Container.rs `Project` macro deduplication; trait shape unchanged.
- `mask_low_bits` helper next to `bytes_for_u16` in `arvo-strategy/src/width.rs`.
- Algorithm-crate USize sweep (mechanical).

Round 4 is unblocked.
