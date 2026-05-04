**Date:** 2026-05-04
**Phase:** TOPIC
**Scope:** arvo (bitfield macro, MaskOps blanket, smoke test crates)
**Source topics:** Round 1 expanded P0 deferral list (Expert B F26-F30, F35, F36, F38); Round 3 src CL deferrals (MaskOps concrete impls, bitfield! mask routing)

# Round 5 Topic 3: mechanical cascades. Bitfield ConstEq/ConstDefault, MaskOps blanket impl, bitfield! macro routing, const-context smoke tests

This topic covers the mechanical residue of Round 5: applications of bridges that already shipped (F38), the chassis MaskOps cascade deferred from Round 3, the bitfield macro routing through `BitPrim::mask_low`, and const-context smoke test coverage spanning the audit-specified gaps plus this round's new surface.

## Background

Round 1's expanded P0 deferral list moved several items to Round 5 because they cascaded after bridges that needed Round 1's substrate work to land first. With Round 1's `ConstEq` and `ConstDefault` bridges shipped, plus Round 3's `Mask<W>` chassis and `BitPrim::mask_low(USize) -> Self`, the cascade work is now mechanical.

Expert B's audit findings F26-F30 and F35-F36 named const-context smoke tests that confirm each bridge / each new const-callable surface is actually callable in const context (not just declared). Without these tests, regressions to non-const callability would not be caught at CI time. The tests are pure additions: new test files exercising existing surface in const expressions.

## Decisions

### Decision 1: F38, bitfield ConstEq + ConstDefault impls

Add `impl const ConstEq` and `impl const ConstDefault` for bitfield-emitted types. The `bitfield!` declarative macro emits manual `PartialEq` / `Hash` / `Debug` impls (per Expert B F38, lines 162-181 of the macro emission). Round 5 cascades to add the const-callable counterparts:

- `impl const ConstEq for <BitfieldType>` with body delegating to bit-pattern equality on the inner storage.
- `impl const ConstDefault for <BitfieldType>` with body returning the all-zeros bit pattern.

Both impls land inside the `bitfield!` macro emission. Manual `PartialEq` / `Hash` / `Debug` stay non-const (cascade-removal is out of scope; F38 is about adding the const surface, not removing the std-trait one).

### Decision 2: MaskOps blanket impl on `Mask<W>` for any W

Refinement on the audit's literal text. Round 3 deleted `Mask64` / `Mask256` and shipped the generic `Mask<W>` chassis. Round 5 ships:

```rust
impl<W> const MaskOps<{<W as BitPrim>::LOGICAL_WIDTH}> for Mask<W>
where
    W: [const] BitPrim + ...,
{
    // Bodies delegate to inherent methods on Mask<W>.
    fn empty() -> Self { Self::empty() }
    fn full() -> Self { Self::full() }
    fn insert(self, bit: USize) -> Self { self.insert(bit) }
    // ... (other MaskOps methods)
}
```

A single blanket impl covers every concrete `Mask<W>` instantiation. No per-W concrete impls (no `impl const MaskOps<{Width::W64}> for Mask<Bits<64, Hot, Unsigned>>` etc.) ship in Round 5.

`min_specialization` is NOT enabled for this round. If a future bench shows a concrete W beats the blanket (e.g., `W = QWord<Hot>` admits a SIMD shortcut the generic body cannot express), a concrete `impl` lands then with `min_specialization` enabled at that point, per `arvo-always-optimal-internals.md`'s naive-baseline-first principle.

### Decision 3: bitfield! macro routing through BitPrim::mask_low

The `bitfield!` macro currently emits hardcoded `(1u64 << N) - 1` patterns for slot masks (per Expert A F9 + the F9 Round 4 fix that landed `BitPrim::mask_low(USize) -> Self`). Round 5 sweeps the macro emission to call `<S as BitPrim>::mask_low(USize::from(N))` instead of the hardcoded shift.

This removes the macro's reliance on hardcoded `u64` and lets the substrate's `BitPrim::mask_low` strategy-aware routing take over (the impl knows whether to use a single shift, a multi-word fold, or a SIMD path based on the container).

The macro emission stays a declarative `macro_rules!` (no proc macro change). Source CL touches `arvo/src/bitfield.rs` only.

### Decision 4: Const-context smoke tests, audit-specified plus this round's new surface

New test crates (or test files in existing crates) exercising:

**Audit-specified (Expert B):**

- F26: `Mask<Bits<W, Hot, Unsigned>>` const-context smoke. Replaces F26's literal Mask64 reference. Crates `arvo-bitmask/tests/mask_const_arith.rs`.
- F27: `Bits<N, S, Sign>` Identity / Bounded const access; const-context BitAccess / BitSequence / BitLogic. Crate `arvo-bits/tests/bits_const_arith.rs`.
- F28: UFixed / IFixed const-arith composition. Crate `arvo/tests/ufixed_const_arith.rs`, `arvo/tests/ifixed_const_arith.rs`.
- F29: FastFloat / StrictFloat const-arith. Crate `arvo/tests/float_const_arith.rs`.
- F30: BitPrim / IBitPrim const-context. Crate `arvo-bits-contracts/tests/bitprim_const_access.rs`.
- F35: MetaCarrier::as_bits const-context. Crate `arvo-storage/tests/metacarrier_const_arith.rs`.
- F36: Resolve<Other>::Out projection const access. Crate `arvo-strategy/tests/resolve_const_projection.rs`.

**Round 5 new surface:**

- `Slot<T: NonZeroable>` const-context construction + as_maybe. Crate `notko/tests/slot_const_arith.rs` (notko-side, lands in notko PR).
- `NUSize` const-context construction + as_maybe + unwrap_or. Crate `arvo-storage/tests/nusize_const_arith.rs`.
- `ConstTry` / `ConstFromResidual` on Just / Maybe / Outcome / Bool const-context branch + from_output + from_residual. Crate `notko/tests/consttry.rs` (notko-side) plus `arvo-storage/tests/bool_consttry.rs` (arvo-side).
- bitfield ConstEq + ConstDefault const-context. Crate `arvo/tests/bitfield_const_eq_default.rs`.
- MaskOps blanket impl const-context (any W). Crate `arvo-bitmask/tests/mask_ops_blanket_const.rs`.

Each test file exercises the surface in `const _: ... = { ... };` blocks; failure shows as compile error on the const evaluation. No runtime assertions beyond what's already required.

### Decision 5: bitfield ConstEq / ConstDefault test scope

The bitfield ConstEq + ConstDefault test exercises the const surface across at least three bitfield instantiations: a 1-bit slot, a multi-bit slot inside u64, and a multi-bit slot inside a wide container (>128 bits). Coverage matters because the macro emission has been touched twice this round (F38 const impls + Decision 3 mask_low routing).

## Out of scope

- Removing manual `PartialEq` / `Hash` / `Debug` from bitfield emission. These stay non-const; the const surface is additive.
- Concrete MaskOps impls per W. Defer to bench-justified follow-ups.
- `min_specialization` enablement. Defer until a concrete override is needed.
- Macro hygiene rework on `bitfield!`. Routing through `BitPrim::mask_low` is the only macro change this round.

## Cross-references

- `mock/research/audits/2026_05_02_expert_b_const_trait_completeness.md` Findings F26-F30, F35-F36, F38.
- `mock/research/audits/2026_05_02_expert_a_architectural_dogfooding.md` Finding F9 (mask_low_bits substrate helper, shipped Round 4).
- Round 3 src CL `mock/design_rounds/202605031748/202605031748_changelist.src.lock.md` (the MaskOps deferral).
- `.claude/rules/arvo-always-optimal-internals.md` (naive-baseline-first, why blanket precedes concrete).
- `arvo/src/bitfield.rs` (the macro emission, hardcoded mask sites).
- `arvo-bits-contracts` BitPrim::mask_low (the routing destination).
