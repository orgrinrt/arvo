**Date:** 2026-05-03
**Phase:** TOPIC
**Scope:** Round 3 — Mask\<W\> over Bits + BitMatrix\<W, N\> + mask-contracts impl + dogfood sweep
**Source topics:** task #313, audit findings 1, 7, 9, 10, 11, 12 from `mock/research/audits/2026_05_02_expert_a_architectural_dogfooding.md`

# Round 202605031748 — Round 3: mask + bitmatrix collapse, dogfood sweep

## Why this round exists

The 2026-05-02 expert-A architectural-dogfooding audit identified a cluster of substrate-internal types that hand-roll their own width-specific surface instead of riding the generic primitives the substrate ships. The most visible offender is `Mask256`: a `pub struct Mask256(pub [QWord<Hot>; 4])` with 280 lines of unrolled `a[0..3]` boilerplate, justified at the time by "Rust arrays don't implement the arvo-bits traits". That justification is mooted: Round 2 (#311) shipped `BitPrim` impls on `MultiContainer<HiT, LoT>`, and the `Bits<N, S, Sign>` blanket impls of `HasBitWidth + BitAccess + BitSequence + BitLogic` already route through `BitsContainerFor<N, Sign>` for any N up to 256. The substrate built the mechanism; the bitmask crate ignored it.

Round 3 closes the loop: the generic `Mask<W>` chassis already exists in `arvo-bitmask/src/mask.rs:34`. The work is to retire the parallel `Mask256` struct, retire the parallel `BitMatrix64` / `BitMatrix256` structs, implement the abstract `arvo_mask_contracts::Mask<W>` trait so the trait declaration is no longer dead, and apply low-cost dogfood callouts on adjacent types (`FastFloat` / `StrictFloat` Identity / Bounded routing, `Mask<W>::empty` via `Identity::ZERO`).

The round runs on top of #316 / #311 / #312 substrate work which all merged in the prior chain. No substrate prerequisites remain.

## Findings in scope

From the architectural-dogfooding audit, in priority order:

**P1 core** (the primary scope):

1. **F1**: `Mask256` parallel struct → collapse onto `Mask<W>` chassis with `W = Bits<256, Hot, Unsigned>`.
2. **F7**: `BitMatrix64<N>` / `BitMatrix256<N>` parallel structs → collapse onto `BitMatrix<W, const N: Cap>` chassis.
3. **F12**: `arvo_mask_contracts::Mask<const W>` trait is declared but unimpl'd → ship per-shipping-mask impls. Renamed in this round (see Decision C).

**P1 dogfood**:

4. **F9**: `arvo-hash` and `arvo/src/bitfield.rs` hardcode `u64::MAX` and `1u64 << ...` patterns → add `BitPrim::mask_low(n: USize) -> Self` substrate method, route call sites through it.
5. **F10**: `Ieee` declares `ZERO` / `ONE` directly; `FastFloat<F>` / `StrictFloat<F>` don't blanket-impl `Identity` / `Bounded` → impl Identity for f32/f64, supertrait-bound Ieee: Identity, blanket Identity for FastFloat/StrictFloat.

**P2 cleanup** (rolls in cleanly):

6. **F11**: `Mask<W>::empty()` non-const-routed → route through `Identity::ZERO`.

The audit's other findings are out of scope for Round 3:
- F2, F8 already shipped in Round 2 (#311 / #312).
- F3, F4 shipped earlier (round 202605021600).
- F5 shipped in #316.
- F6 (algorithm-crate `let mut i = 0usize` sweep) shipped in Round 4 (#314).
- F13-F16 are downstream of Round 3 work and are tracked under Round 5 (#315) or in BACKLOG.

## Substrate state confirmed

Pre-round verification of substrate readiness:

- `arvo_bits_contracts::bits_impl` blanket-impls `HasBitWidth + BitAccess + BitSequence + BitLogic` on `Bits<N, S, Sign>` via the `BitsBitPrim<Sign>` bridge over `BitsContainerFor<N, Sign>::T`. `Bits<256, Hot, Unsigned>` resolves through `MultiContainer<u128, u128>` and reaches the bit-level surface in const context. Verified by reading `mock/crates/arvo-bits-contracts/src/bits_impl.rs:20-115`.
- `BitPrim` and `IBitPrim` already declare `WIDTH: USize` (no `u16` left in those positions). Audit F8 is already mooted.
- `mask_low_bits(n: u16) -> u64` ships in `arvo-strategy::width` from Round 4 (#314). Round 3's F9 work is to replace it with the per-primitive `BitPrim::mask_low(n: USize) -> Self` and update call sites; the free fn becomes redundant and is deleted per `no-legacy-shims-pre-1.0`.

## Decisions captured

### Decision A: Round scope

**Core + dogfood sweep.** Findings F1, F7, F12 are the core; F9, F10, F11 land as dogfood callouts in the same round. Single round on a single branch with one PR targeting `dev`.

### Decision B: Mask chassis shape

**Type-generic over W.** The existing `Mask<W: BitSequence + BitAccess + Copy + Default>` chassis stays. After Round 3, `W` can be `Bits<64, Hot, Unsigned>` for the 64-bit case or `Bits<256, Hot, Unsigned>` for the 256-bit case (or any other width up to 256 once a consumer needs it). No structural change to the chassis declaration; the work is removing the parallel `Mask256` struct and updating consumers.

The const-generic alternative (`Mask<const N: Width, S: Strategy = Hot, Sign: Signedness = Unsigned>`) was rejected: it adds where-clause complexity at every Mask use site for no win the type-generic chassis does not already deliver. The W-type bound captures the same axes (the Bits-shape carries N, S, Sign internally) without forcing every signature to repeat them.

### Decision C: Mask alias names

**Delete `Mask64` and `Mask256` aliases.** Per `no-legacy-shims-pre-1.0`, names that exist only as parallel handles for the canonical shape do not survive into shipping APIs. Consumers reach for `Mask<Bits<64, Hot, Unsigned>>` or `Mask<Bits<256, Hot, Unsigned>>` directly. Every current call site (Mask64-typed locals in arvo-bitmask, BitMatrix internal use, NodeId sites if any, propagate_dirty_64 / propagate_dirty_256, SetBitsIter64 / SetBitsIter256) gets renamed.

This is the more aggressive of the three options offered. The trade-off: longer call-site spellings, in exchange for one canonical name per shape and zero parallel-name confusion. Pre-1.0 timing makes the cost of the longer spellings acceptable; once consumers form, they can introduce their own domain aliases.

The audit recommended keeping `pub type Mask64 = Mask<...>` aliases. The user's call here overrides the audit on a discretionary point; the audit's structural recommendation (the collapse) stands.

### Decision D: Contracts trait rename

**`arvo_mask_contracts::Mask<const W: u16>` renames to `MaskOps<const W: Width>`.** Two reasons:

1. **Disambiguation from the struct.** After the collapse, `arvo_bitmask::Mask<W>` (the chassis struct) and `arvo_mask_contracts::Mask<const W>` (the abstract trait surface) both exist with the name `Mask`, distinguished only by crate. Different shapes (type-generic vs const-generic), different roles (concrete chassis vs abstract operation set). The same-name overlap reads as a clash; renaming the trait to `MaskOps` reads as "operations on a mask of width W" and matches the trait's body (`set` / `clear` / `test` / `union` / `intersection` / `difference` / `complement` / etc.).

2. **`const W: u16` lifts to `const W: Width`.** Per audit Finding 5 follow-through (#312 already lifted Bits-side; mask-contracts had been pending). The Width-typed const-generic is the correct substrate position.

The trait declaration moves to `pub const trait MaskOps<const W: Width>: Sized + Copy { ... }`. The rename is a substrate API surface change visible to any consumer naming `arvo_mask_contracts::Mask`; per audit verification, no current workspace consumer references the trait by name (Finding 12 itself states the trait is unused). The blast radius is internal-only.

### Decision E: BitMatrix shape

**`pub struct BitMatrix<W, const N: Cap> where [(); cap_size(N)]: { rows: [Mask<W>; cap_size(N)] }`** with the same `W: BitSequence + BitAccess + Copy + Default` bound that `Mask<W>` carries. `BitMatrix64<N>` and `BitMatrix256<N>` aliases are deleted (parallel decision to C; both alias families go together).

Methods compose: `BitMatrix<W, N>::set(row, col)` calls `self.rows[row].set(col)` which dispatches through `Mask<W>::set` which dispatches through `<W as BitAccess>::with_bit_set`. Same shape for clear, test, union/intersection/difference/complement on whole matrices, propagate_dirty.

The `dirty.rs` `propagate_dirty_64` / `propagate_dirty_256` parallel functions collapse to a single generic `propagate_dirty<W, const N: Cap>(...)` that takes the matrix + a fold callback.

### Decision F: BitPrim::mask_low method

**Add `fn mask_low(n: USize) -> Self` to `BitPrim` and `IBitPrim` with default impl bodies.** Body for `BitPrim` (unsigned):

```text
if n.0 == 0 {
    <Self as Identity>::ZERO
} else if n.0 >= <Self as BitPrim>::WIDTH.0 {
    <Self as Bounded>::MAX
} else {
    // (Self::ONE << n.0) - Self::ONE, routed through const-stable bridge
}
```

Body for `IBitPrim` (signed): same shape, using `IBitPrim::ONE` and the signed path through `$uty` reinterpretation (the existing macro pattern at `arvo-bits-contracts/src/lib.rs:380-410`).

Call sites:
- `arvo-hash/src/fnv1a.rs:94, 114` switch to `<<S as BitsContainerFor<N, Unsigned>>::T as BitPrim>::mask_low(USize(N as usize))`.
- `arvo-hash/src/xxhash3.rs:99, 120` same pattern.
- `arvo/src/bitfield.rs:219-256` switches to the `BitPrim::mask_low` form.
- `arvo-strategy::width::mask_low_bits(n: u16) -> u64` is deleted.

### Decision G: Float Identity / Bounded supertrait

**Three sub-changes:**

1. Add `impl const Identity for f32` and `impl const Identity for f64` in `arvo-bits-contracts` (where the float Bounded impls already live).
2. Lift `Ieee` to `pub const trait Ieee: Identity + Bounded { ... }`. Drop `Ieee::ZERO` and `Ieee::ONE` declarations; consumers reach `<F as Identity>::ZERO`.
3. `impl<F: Ieee> const Identity for FastFloat<F>` and `impl<F: Ieee> const Identity for StrictFloat<F>`. Same for Bounded.

The `Ieee::ZERO` / `Ieee::ONE` direct-declaration is replaced with the supertrait bound; consumers reading `<F as Ieee>::ZERO` get a deprecated path. Per `no-legacy-shims-pre-1.0`, no compatibility shim ships; the consumer rewrites the spelling.

### Decision H: Mask\<W\>::empty via Identity::ZERO

**Route `Mask<W>::empty() -> Self` through `<Self as Identity>::ZERO`** (the const-trait Identity impl already lands on `Mask<W>` per round 202605021600). The hand-coded `W::default()` body retires; the same-named inherent fn delegates: `pub const fn empty() -> Self { <Self as Identity>::ZERO }`. `Default::default()` impl stays for stdlib coverage but its body becomes `<Self as Identity>::ZERO` too.

`Mask::full()` and `Mask::FULL` similarly route through `<Self as Bounded>::MAX`.

### Decision I: Branch and PR shape

**Single round, single branch (`feat/mask-bits-generic-and-bitmatrix`), single PR targeting `dev`.** Round opens TOPIC → DOC → SRC → CLOSE. After close, the PR opens, senior reviewer dispatches, fixes apply, merge.

## Out of scope

- Round 5 (#315) const-context smoke tests + binpack Maybe sentinel + bitfield ConstEq/ConstDefault. Separate round.
- Mask4096 / Mask1024 / Mask128 ergonomic aliases (post-1.0 if needed).
- Audit findings 13-16 (LOGICAL_WIDTH typed lift, BitsBitPrim collapse, etc.). Tracked in BACKLOG.

## Verification before lock

Pre-DOC-CL:

- All decisions captured.
- Substrate state confirmed (#311, #312, #316 all merged, no missing prerequisites).
- Audit cross-references checked (no findings in scope are blocked on missing infrastructure).

Pre-SRC-CL:

- Doc CL covers every shipped surface change: Mask chassis docs unchanged, MaskOps trait declaration changed, BitMatrix declaration changed, BitPrim::mask_low method added, Ieee supertrait lifted, FastFloat/StrictFloat Identity blanket added.
- Backlog promotions / removals captured in arvo-bitmask BACKLOG, arvo-mask-contracts BACKLOG, arvo BACKLOG.

Pre-close:

- `cargo +nightly check --workspace` clean.
- `cargo +nightly test --workspace` clean.
- `cargo mock` lint pipeline clean at commit and push severity.
- Every `## CHANGE:` block in the src CL verifies (per `cl-claim-sketch-discipline`).
