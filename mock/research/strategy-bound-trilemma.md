# The Strategy-Bound Trilemma

> **Promoted 2026-04-28**: this analysis is now shipped as
> `mock/crates/arvo/DEEPDIVE_strategy-bound-trilemma.md.tmpl`. That
> template is the canonical source going forward. This research file
> remains as audit trail for the round 202604500000 investigation that
> produced the analysis; do not edit here, edit the DEEPDIVE template.

> **Status**: research note. Captured during round 202604500000 (foundations restructure + Fnv1a) source execution. Promote to a DEEPDIVE template (`mock/crates/arvo/DEEPDIVE_strategy-bound-trilemma.md.tmpl`) in a follow-up DOC-phase round.

This note formalises a structural constraint encountered during round 202604500000. It explains why arvo's `UFixed` / `IFixed` / `Bits` types carry a `where S: UContainerFor<{...}>` bound that consumers writing generic-over-bit-width code must restate, and why no creative reorganisation eliminates that restatement on current rustc without compromising substrate principles.

The constraint is not arvo-specific. Any Rust library that combines (a) static validity checking via trait bounds on const-expressions over generic params, (b) type-level layout determined by those bounds, and (c) generic-over-the-const-param consumer code hits it.

## The intent

Three properties define arvo's numeric substrate:

1. **Static validity check.** Combinations of `(strategy, bit-width)` that don't make sense fail at compile time. `Uint64<Warm>` produces a `UContainerFor`-bound error pointing the consumer at strategy choice, not a runtime panic or a silent wrap. Warm caps at `I + F <= 32`; above that, no `UContainerFor<N>` impl exists for `Warm`, and the type fails to compile. The static check IS the substrate's correctness story.

2. **Strategy-driven container width at the type level.** `Bits<8, Hot>` is one byte; `Bits<8, Warm>` is two; `Bits<8, Cold>` is bitpacked to N bits in column storage. The strategy marker chooses physical layout, not just arithmetic semantics. This is what gives Hot its L1 density, Cold its archival density, Warm its 2x safety on single ops. Lose this and Strategy collapses from a layout-determining marker to an arithmetic-flavour tag.

3. **Ergonomic call sites for consumers.** `let n: Uint8 = ...;` `type Coord = Fixed<13, 3, Warm>;` Without ergonomic call-site syntax the substrate becomes painful to consume.

The substrate's value comes from holding all three together. Drop any one and the design erodes.

## The constraint

On current rustc (`adt_const_params` + `generic_const_exprs` available, `implied_bounds` not), you can have any two of those three, not all three.

The mechanism: `UFixed<const I: IBits, const F: FBits, S: Strategy>`'s storage field is `<S as UContainerFor<{ ufixed_bits(I, F) }>>::T`. The field type is a projection through a trait bound that depends on a const-expression over the type's own generic parameters. For the type to be well-formed (Sized, layout-known), rustc must verify the bound. For consumer code that writes `Fixed<I, F, S>` while itself being generic over `<const I: u8, const F: u8, S: Strategy>`, rustc cannot verify the bound at the consumer's definition site without the consumer restating it.

The relevant rustc feature that would erase this restatement is `feature(implied_bounds)` (RFC 2089, tracking issue #44491). It lets a struct's where-clause apply implicitly when the struct is named in a consumer's signature. The feature has been open since September 2017 with open design questions on crate-local vs module-local scope and type-inference interactions; not on a near-term stabilisation track. Adopting it would be on the same risk class as `feature(fundamental)` (explicitly "not intended to be stabilized as-is" per its tracking issue) — too uncertain to commit to.

## Why the alternatives compromise principle

Three alternative architectures eliminate the friction. Each costs one of the three properties. Each was traced and rejected during round 202604500000 src execution.

### Alternative A: drop static validity (uniform storage)

Make UFixed's storage uniform `u64`, store width info as a phantom tag, validate at construction only. Eliminates the bound on the struct definition; consumer-generic code names the type freely.

Costs: every `Bits<8, Hot>` value pays 8 bytes instead of 1. Cold's bit-density at the type level evaporates. The substrate's "L1-density Hot vs archival Cold" story collapses to a runtime-checked variant tag. Hot becomes "u64 with hot semantics" instead of "the smallest aligned container that fits". This is the regression we explicitly reject; arvo's framing as a fixed-point-first stack rests on strategy-driven layout being real, not nominal.

### Alternative B: drop strategy-driven layout (per-container types)

Mirror the `fixed` crate's approach: `pub struct UFixedI8<const F: u8>(i8);` `UFixedI16<...>(i16);` etc. The integer container is part of the type's name; no projection needed; bounds are simple `where Self: Sized`.

Costs: combinatorial explosion of types; generic-over-strategy code becomes generic-over-container-type instead, which is awkward; the whole `UContainerFor` table that maps `(S, BITS)` to a container disappears, taking with it the codegen optimisation hooks (LLVM passes that fire on `Bits<N, Hot>` would have to be reimplemented per-container). The strategy marker degrades from a layout director to an arithmetic flag, which the substrate explicitly rejects (Strategy IS layout per `PRINCIPLES.md`).

### Alternative C: keep both, accept friction

The current design. Generic-over-width consumer code restates `where S: UContainerFor<{ ufixed_bits(I, F) }>`. Monomorphic consumer code (the dominant case) is unaffected. The friction is contained: it lives in algorithm code internal to arvo (graph / sparse / spectral / comb generic-over-width paths) and in occasional consumer code that genuinely needs to be generic over bit width. The substrate principles hold; the cost is a verbose where-clause at one specific kind of call site.

This is the chosen path.

## Creative unsafe options that don't help

During round 202604500000 src execution, the following angles were traced and rejected. Each fails for the same root reason: the field type IS the projection, and `unsafe` lets us reinterpret memory we already have but not elide the typecheck that determines what type the memory should be.

1. **Sealed unsafe marker trait with always-true blanket impl** (`unsafe trait BoundProof<S, I, F>`): the trait bound on the struct still expands to the same const-expr in the where clause, just one trait deeper.
2. **`MaybeUninit<projection-typed field>`**: same projection, just less initialised. The Sized/layout query still hits the bound.
3. **Union of all container types** (u8/u16/u32/u64): would work, but max-size = u64 violates Cold's exact-bit-density principle. Reduces to alternative A.
4. **`[u8; size_of_container::<S, I, F>()]` field**: array length is a const-expr, still depends on generic params, same propagation issue.
5. **Phantom-only struct, storage moves off-type**: breaks per-value semantics — `let x: UFixed<I, F, S> = ...` requires storage on the type.
6. **Specialisation-based fallback**: `feature(specialization)` is famously incomplete + has known unsoundness in non-trivial uses. Worse than `implied_bounds`.
7. **Const-eval guard via `[(); 1 / valid()]:`** (the Fnv1a trick): same const-expr propagation issue when used inside generic-over-width caller code.

The pattern across all of these: the bound failure is INTRINSIC to "the field's type depends on a const-expr derived from generic params". `unsafe` is not a workaround for this class of problem.

## How to work within the constraint

Three layers of access, each with its own pattern. The friction is concentrated in layer 3 by design.

### Layer 1: end-user code

End users (vehje, viola consumers, downstream Clause) write canonical-width aliases (`Uint8`, `Int16`, `Bits<28, Hot>`) and domain aliases their crate defines locally. They never see the bound. Their imports reach `arvo::{Uint8, Int16, USize, Bits, ...}` and that's the surface.

Code at this layer is monomorphic — every type is fully specified at the use site. The bound is verified once at the alias definition's instantiation, never at the consumer's signature.

### Layer 2: domain-author code

A consumer crate defining its own domain alias for an ad-hoc width. Pattern:

```rust
// In a consumer crate.
use arvo::{Fixed, Signed};

pub type Angle = Fixed<9, 7, Warm>;
pub type Coord = Signed<15, 16, Warm>;
pub type Hash = arvo::Bits<28, arvo::Hot>;
```

`Fixed<I, F, S>` and `Signed<I, F, S>` are bare-`u8`-const-generic aliases that forward to the `IBits` / `FBits`-wrapped form internally. The alias body is the only place `{ ibits(...) }` machinery appears; consumer call sites never see it. Like layer 1, code here is monomorphic — `Fixed<13, 3, Warm>` substitutes concrete values.

### Layer 3: arvo-internal generic-over-width code

The friction's home. Code inside arvo's algorithm crates (`arvo-graph`, `arvo-sparse`, `arvo-spectral`, `arvo-comb`) that's generic over numeric trait bounds occasionally needs to be generic over bit width specifically. When that happens, the explicit bound is restated:

```rust
fn process<const I: IBits, const F: FBits, S: Strategy>(x: UFixed<I, F, S>)
where
    S: UContainerFor<{ ufixed_bits(I, F) }>,
{ /* ... */ }
```

This is verbose. It's also internal: no consumer reads it; it lives in arvo crate sources. The verbosity is the price of the static guarantee.

## The discipline that keeps friction contained

Generic-over-bit-width APIs at consumer-facing surfaces are the smell that drags layer-3 friction into layer 1 / layer 2. The discipline:

- **Public consumer APIs use trait-bound abstractions, not bit-width-generic abstractions.** Take `T: BitPresentation + Add + Mul` not `<const I: IBits, const F: FBits, S>`. Algorithms generic over numeric trait bounds (the existing `arvo-graph` / `arvo-sparse` pattern) are clean. Algorithms generic over the const-generic bit width are the smell.
- **Domain types are aliases, not generic over width.** `pub type Coord = Fixed<13, 3, Warm>` is right. `pub type Coord<const I: u8, const F: u8> = Fixed<I, F, Warm>` is the smell — it pushes the bound up to every Coord use site.
- **When a consumer is tempted to write `<const I: u8, const F: u8>`, redesign as `<T: SomeNumericBound>`.** The trait-first cookbook (`.claude/rules/cookbook.md`) is the existing workspace-level guidance; this constraint reinforces why it matters beyond aesthetics.

If discipline is followed, the friction never manifests at consumer call sites. Layers 1 and 2 stay clean; layer 3 absorbs the verbosity in the substrate's own implementation.

## Watch-item: implied_bounds

If `feature(implied_bounds)` reaches the same stability/usability level as the features arvo already uses (`adt_const_params`, `generic_const_exprs`, `try_trait_v2`, `macro_metavar_expr_concat`), a follow-up round flips on the feature flag and the friction in layer 3 dissolves: every where-clause on a struct definition becomes implicit at use sites, and generic-over-width consumer code can name `Fixed<I, F, S>` without restating the bound.

Until then: the design is correct on principle, the friction is real but contained, and the rule is "trait-first, alias-first, never bit-width-generic at the consumer surface".

## Sources

- `mock/design_rounds/202604500000_topic.foundations-restructure-final.md` (round driving this analysis)
- `mock/design_rounds/202604500000_changelist.src.md` "Ergonomic surface for arvo primitive newtypes" section (the surrounding investigation)
- Rust tracking issue #44491 (`implied_bounds`)
- Rust tracking issue #29635 (`fundamental` — explicitly "not intended to be stabilized as-is", referenced for comparison)
- Rust tracking issue #112792 (`lazy_type_alias` — referenced for comparison)
