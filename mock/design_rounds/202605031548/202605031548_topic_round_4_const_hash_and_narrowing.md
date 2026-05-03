**Date:** 2026-05-03
**Phase:** TOPIC
**Scope:** Round 4 / #314 expanded
**Source topics:** senior PR #42 review findings + #314 original scope + user-flagged discipline violation on per-N `hash_const` inherents

# Round 4 — ConstHash trait lift + bounded-generic narrowing + bridge family + container dedup

## Why this round exists

Round 4 was originally scoped for `ConstFrom` / `ConstTryFrom` / `ConstDeref` / `ConstAsRef` substrate bridges plus an algorithm-crate `USize` sweep and a `mask_low_bits` helper. Senior review of PR #42 expanded the scope after finding three structural issues in the merged code that the cascade introduced or surfaced:

1. `hash_const` per-N inherent methods on `Fnv1a<N>` and `XxHash3<N>` were appended as the bench-shim workaround for the cross-crate `.hash()` failure. The user flagged this as discipline violation: trait identity must be focused, not bolted onto every per-N implementor as inherents. Hash-as-const-construction belongs in a focused contract (`ConstHash<N>`), not as a per-N inherent that lives next to every algorithm.
2. The `impl_fnv1a!` and `impl_xxhash3!` macros each paste 64 byte-identical `Hasher<N>` impl bodies modulo a single primitive name (`u8` / `u16` / `u32` / `u64`). Pre-cascade this was the only shape the trait solver accepted because the container was opaque at trait level. Post-cascade, `<Hot as BitsContainerFor<N, Unsigned>>::T` resolves at trait-solving time, so a single bounded-generic `impl<const N: u16> Hasher<N> for Fnv1a<N>` can replace 128 hand-written impl bodies. The narrowing step is the remaining width-dispatched piece; lift it to a `NarrowFromU64<N>` const trait blanket-implemented per primitive plus per `WideBits<BYTES, A>`.
3. The 40 native + 8 wide `Project` impls in `container.rs` are hand-written. Hot-block and Cold-block bodies are byte-identical modulo the Strategy marker; same for Warm and Precise. `macro_rules! impl_native_bucket` cuts the impl count to 4 invocations (one per Strategy) without losing per-Strategy explicitness.

Two further concerns surfaced that aren't macro/trait-shape but belong in the same round to land alongside:

4. `Project` is unsealed and `Picker` is `pub`. A downstream crate can write `impl Project<6, ...> for Picker` and collide if the substrate ever adds a sixth bucket. Seal `Project` and demote `Picker` to `pub(crate)`.
5. The cross-crate `.hash()` failure recorded as a #316 apply-time finding is a real consumer-visible defect. Bench shims switched to `hash_const`, but the trait-solver chain that fails (`HasherExt` blanket impl over `Hasher<N>` over `Fnv1a<N>` with the const-impl `BitsContainerFor` chain underneath) leaves consumers with two parallel APIs where one silently fails cross-crate. Resolve by either making `ConstHash<N>` the prominent cross-crate ergonomic API or removing `HasherExt` until the rustc trait solver can navigate the chain.

The original Round 4 scope (bridges + algorithm USize sweep + `mask_low_bits`) lands in the same round because the bridges are a co-shape with `ConstHash<N>` (all of them are non-const-stdlib mirrors carried as substrate const traits) and the algorithm sweep is independent enough to ride on whichever round closes the substrate work.

## Current source state (relevant)

`mock/crates/arvo-strategy/src/container.rs` (288 lines):

- `BitsContainerFor<const N: u16, Sign: Signedness>` user-facing const trait, 4 impls (one per Strategy).
- `Project<const TAG: usize, Sign: Signedness, const BYTES: usize, S: Strategy>` helper trait, **unsealed**, 48 hand-written impls (40 native, 8 wide).
- `Picker` ZST is `pub` (line 152).
- Native impls cluster as 4 strategies x 5/4 buckets x 2 sign axes; same shape modulo `Strategy` marker. Hot/Cold buckets are byte-identical, same for Warm/Precise.

`mock/crates/arvo-hash/src/algo.rs` (75 lines):

- `Hasher<const N: u16>` trait with `update` / `finalize`.
- `HasherExt<const N: u16>: Hasher<N>` blanket-extension trait carrying the one-shot `hash(self, bytes: &[u8])` form.
- `fnv1a_64(bytes: &[u8]) -> u64` free const fn (algorithm-fixed `u64` state).

`mock/crates/arvo-hash/src/fnv1a.rs` (136 lines):

- `Fnv1a<const N: u16>` with single `state: u64` field.
- `OFFSET_BASIS` / `PRIME` constants per the FNV spec.
- `impl_fnv1a!` macro paste: 64 `Hasher<N> for Fnv1a<N>` impl blocks plus 64 `hash_const` inherent fn definitions, partitioned by primitive width (u8 for N=1..=8, u16 for 9..=16, u32 for 17..=32, u64 for 33..=64). Each impl block: `update` is the same FNV round; `finalize` differs only in the primitive cast at the narrowing step.

`mock/crates/arvo-hash/src/xxhash3.rs` (139 lines):

- `XxHash3<const N: u16>` with `buffer: [u8; 256]` + `pos: usize` streaming buffer.
- `xxhash3_64` free const fn.
- `impl_xxhash3!` macro paste: same 64-impl shape as Fnv1a. The streaming `update` + `finalize` are the same byte-stream-hash narrowing pattern.

The narrowing step in both algorithms is the only width-dispatched piece. Today it is hand-written per primitive: `Bits::<$n, Hot>::from_raw((raw & mask) as $ty)` where `$ty` is the primitive name pasted by the macro. This is exactly the per-N inherent pattern the cascade unified everywhere else.

## Proposed structure

The proposed structure for execution (subject to user direction):

1. `arvo-hash` lift: introduce `ConstHash<const N: u16>` const trait with `fn hash_const(bytes: &[u8]) -> Bits<N, Hot>`. `Fnv1a<N>` and `XxHash3<N>` impl this once each, generic over N.
2. `arvo-bits-contracts` (or `arvo-strategy`): introduce `NarrowFromU64<const N: u16>` const trait with `fn narrow_u64(raw: u64) -> Self` (`Self` is the dispatched container). Blanket-impl per native primitive plus `WideBits<BYTES, A>`. Position decision: `arvo-bits-contracts` is the natural home (it already hosts narrowing primitives), but the bridge-home rule says it lives in the lowest layer where its return type is reachable; that's `arvo-storage` for the primitive narrowing and `arvo-bits-contracts` for `WideBits`. Topic decision needed.
3. `arvo-hash` rewrite: delete `impl_fnv1a!` and `impl_xxhash3!` macros. Replace with single bounded-generic `impl<const N: u16> Hasher<N> for Fnv1a<N>` and same for XxHash3. The narrow step uses `<Hot as BitsContainerFor<N, Unsigned>>::T as NarrowFromU64<N>>::narrow_u64(raw)`. Per-N `hash_const` inherents disappear; consumers reach for `<Fnv1a<N> as ConstHash<N>>::hash_const(bytes)` (or the `ConstHash` trait in scope).
4. `arvo-strategy/container.rs`: introduce `macro_rules! impl_native_bucket` that takes (Strategy, Unsigned-list, Signed-list) and emits 8 Project impls per invocation. Reduces 40 native impls to 4 invocations.
5. Seal `Project`: introduce `mod sealed { pub trait Sealed {} }` and add the bound. Demote `Picker` to `pub(crate)`. Re-exports through `arvo-strategy/src/lib.rs` adjust as needed.
6. Delete `HasherExt` (the cross-crate `.hash()` chain that doesn't resolve). Consumer-facing one-shot ergonomics ride on `ConstHash<N>::hash_const`. The streaming `Hasher<N>` trait stays for incremental hashing.
7. Bridge family: `ConstFrom<T>` / `ConstTryFrom<T>` / `ConstDeref` / `ConstAsRef<T>` substrate const traits. Placement per bridge-home rule:
   - `ConstFrom<T>` / `ConstTryFrom<T>` return `Self` / `Outcome<Self, ...>`. `Outcome` lives in `notko`. `arvo-strategy` reaches `notko`. → `arvo-strategy`.
   - `ConstDeref` / `ConstAsRef<T>` return `&Self::Target`. They go in `arvo-transparent` (the typed-unwrap-door layer; reachable by everything above).
8. Algorithm-crate `USize` sweep: `arvo-graph`, `arvo-sparse`, `arvo-comb`, `arvo-spectral` — replace remaining bare `usize` in const-generic positions with `arvo::USize` where the call site is consumer-facing.
9. `mask_low_bits<const N: u16>(raw: u64) -> u64` const fn helper, hosted next to `bytes_for_u16` in `arvo-strategy/src/width.rs`. Centralises the `if N == 64 { u64::MAX } else { (1u64 << N) - 1 }` pattern that today appears 4 times in the hash macros.

## Concerns / open questions

A. **Round scoping.** The expanded scope is larger than typical small-rounds-convention work. Three plausible shapes: (i) single consolidated round on this branch (8-9 sub-passes); (ii) split into hash subsystem round + bridge family round (chained on the same branch, single PR); (iii) split into two rounds on two branches and two PRs.

B. **`NarrowFromU64<N>` placement.** The trait projects `u64 → Bits<N, Hot>` (or `Bits<N, S, Sign>`?). Today's hash flow is `Hot`-only. If we make `NarrowFromU64` generic over `S` and `Sign`, the bridge-home rule places it at `arvo-bits-contracts`. If we keep it `Hot` + `Unsigned` (matches current hash usage), `arvo-storage` works. Decision needed.

C. **`ConstHash<N>` width genericism.** Current per-N hash returns `Bits<N, Hot>`. Make `ConstHash<N>` Hot-only (matches today), or generic over Strategy? Hot-only ships now; Strategy-generic is a follow-up. Decision needed.

D. **Cross-crate trait-solver risk.** The `HasherExt` failure was caused by the const-impl + Pattern C blanket-impl chain. `ConstHash<N>` adds another blanket-impl-over-Pattern-C link. Test cross-crate before merging by sketch in `mock/research/sketches/` per the cl-claim-sketch-discipline rule.

## What the doc CL has to do

The doc CL covers:

- `arvo-hash/DESIGN.md.tmpl`: replace per-N inherent prose with `ConstHash<N>` trait section. Update `Hasher<N>` section to note the bounded-generic impl. Delete `HasherExt` references.
- `arvo-strategy/DESIGN.md.tmpl`: container.rs structural prose update (macro deduplication mentioned, sealing noted). New `NarrowFromU64<N>` section (placement TBD). New `mask_low_bits` helper section.
- `arvo-strategy/BACKLOG.md.tmpl` + `arvo/BACKLOG.md.tmpl`: graduate `ConstHash`, `ConstFrom`, `ConstTryFrom`, `ConstDeref`, `ConstAsRef` from BACKLOG to DESIGN. Strike the Round 5 (#315) parenthetical on items moving to Round 4.
- `arvo-bits-contracts/DESIGN.md.tmpl` (if `NarrowFromU64` lands there): new section.
- `arvo-transparent/DESIGN.md.tmpl` (if `ConstDeref`/`ConstAsRef` land there): new section.
- Delete the FNV1a / XxHash3 per-N inherent prose chunks; the bounded-generic impl shape replaces them.

## What the src CL has to do

Per-crate file-and-fn-level changes following the structured `## CHANGE:` grammar from `cl-claim-sketch-discipline.md`. Not enumerated here; the src CL handles the per-claim record.

## Decisions captured below

### Decision A: Round shape

**Single consolidated round on `feat/const-hash-and-narrowing`.** All 9 sub-passes land under one timestamped round. One src CL with 9 sub-section claims. One PR.

Rationale: the sub-passes share an implementation theme (substrate hardening + hash subsystem rewrite + bridge family). The hash subsystem rewrite touches container.rs hardening (passes 3-5) and the bridge family extension overlaps with `ConstHash<N>` shape decisions. Splitting would require backflow between the two halves. The branch-pr-flow workspace rule explicitly endorses chained rounds on a single branch when the implementation reveals that scope.

### Decision B: `NarrowFromU64` parameterisation and placement

**`NarrowFromU64<const N: u16, S: Strategy, Sign: Signedness>` lives in `arvo-bits-contracts`.** Returns the dispatched container `<S as BitsContainerFor<N, Sign>>::T`.

Trait declaration carries the `S: BitsContainerFor<N, Sign>` bound. Blanket impls per native primitive (`u8` / `u16` / `u32` / `u64` / `u128` / their signed siblings) keyed on the matching `(S, Sign)` pairs, plus `WideBits<BYTES, A>` for the wide bucket per Strategy.

Rationale: `arvo-bits-contracts` is the natural home for narrowing primitives (already hosts `Narrow<T>` and the `Narrowed<N, T>` alias). Strategy + Sign generality matches the substrate's strategy-aware identity. `arvo-storage` reaches `BitsContainerFor` (via `arvo-strategy`) but `arvo-bits-contracts` reaches one layer up where the narrowing-trait family already lives, keeping the bridges clustered.

Implementation note: the per-(S, Sign) blanket impls land hand-written on the native primitives (~40 entries). `WideBits<BYTES, A>` impl is single-shot generic over `BYTES` and `A`. The macro deduplication option for these blanket impls is a future-round concern; this round ships the canonical surface.

### Decision C: `ConstHash<N>` parameterisation

**`ConstHash<N, S, Sign>` mirrors NarrowFromU64.** Returns `Bits<N, S, Sign>`.

Carries `S: BitsContainerFor<N, Sign>` plus `<S as BitsContainerFor<N, Sign>>::T: NarrowFromU64<N, S, Sign>` as supertraits. Single bounded-generic impl per algorithm. `Fnv1a<N>` impl reaches for `fnv1a_64` then `narrow_u64`; `XxHash3<N>` does the same with `xxhash3_64`.

Rationale: matches the strategy-aware substrate identity end-to-end. A consumer hashing into a Cold container (compact storage of pass fingerprints in clause-codegen) or signed bits (specialised use case) gets the typed result without an extra cast at the call site. Mirrors decision B's choice to keep narrowing fully generic.

### Decision D (discipline, not a vote): sketches required before doc CL locks

Per the workspace `cl-claim-sketch-discipline.md` rule, this round opens with sketches in `mock/research/sketches/` for the trait-solver-fragile parts before the doc CL locks:

1. **NarrowFromU64 blanket-impl orphan check.** Confirm rustc accepts the per-primitive blanket impls without E0119 conflicts.
2. **ConstHash cross-crate dispatch.** Confirm the trait-solver chain (consumer -> `ConstHash<N, S, Sign>` -> `NarrowFromU64<N, S, Sign>` -> `BitsContainerFor<N, Sign>` -> `Project<TAG, Sign, BYTES, S>`) resolves from a downstream crate. The `HasherExt` cross-crate failure recorded in #316's apply-time findings is direct evidence that this category of chain is fragile.
3. **Sealed `Project` migration.** Confirm sealing `Project` and demoting `Picker` to `pub(crate)` does not break the `BitsContainerFor` impls in arvo-strategy itself (same crate; sealed bound is satisfied internally).

If sketches surface a problem rustc can't navigate, the doc CL adjusts before locking. Sketches stay forever as audit trail.
