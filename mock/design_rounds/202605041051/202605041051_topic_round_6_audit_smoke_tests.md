**Date:** 2026-05-04
**Phase:** TOPIC
**Scope:** arvo-bitmask, arvo-bits, arvo, arvo-bits-contracts, arvo-storage, arvo-strategy
**Source topics:** Round 1 expanded P0 deferral list (Expert B audit findings F26-F30, F35-F36); Round 5 deferred follow-up (#324)

# Round 6 Topic 1: audit-specified const-context smoke tests

This topic covers the const-context smoke tests Expert B's 2026-05-02 const-trait completeness audit specified as Findings F26 through F30 plus F35 and F36. Round 5 (#315) explicitly deferred them as confirmation-only against substrate already exercised by real consumer code. This round closes that gap.

The tests are pure additions. Each file exercises existing const-callable surface inside `const _: () = { ... };` blocks, so a regression to non-const callability surfaces as a compile error at CI time. No source change; no design change; no API surface change. The discipline gain is: regressions at the const boundary do not slip past current test coverage.

## Context: why deferred from Round 5

Round 5's PR body explicitly named the deferral: "the audit-specified F26-F30 / F35-F36 set was scoped out as confirmation-only against substrate already exercised by real consumer code (tracked as #324)". The reasoning at the time: Round 5 was already heavyweight (NUSize, Bool ConstTry, MaskOps reshape, binpack rewrite, bitfield macro changes, four new-surface smoke tests), and adding seven more files would have widened the round's blast radius without protecting any substantively new surface. The audit findings stay valid; the deferral was just timing.

## Decisions

### Decision 1: F26 — Mask const-context smoke

**Test file:** `mock/crates/arvo-bitmask/tests/mask_const_arith.rs`

Exercises `Mask<Bits<W, Hot, Unsigned>>` for representative widths (8, 32, 64, 256) inside `const _: () = { ... };` blocks. Surface to verify const-callable: `Mask::empty`, `Mask::full`, `Mask::set`, `Mask::clear`, `Mask::test`, `Mask::count`, `Mask::union`, `Mask::intersection`, `Mask::difference`, `Mask::complement`. The blanket `MaskOps` impl from Round 5 lifts each of these into const-trait surface; the tests confirm that lift compiles in const context.

### Decision 2: F27 — Bits const-context smoke

**Test file:** `mock/crates/arvo-bits/tests/bits_const_arith.rs`

Exercises `Bits<N, S, Sign>` for N values that hit each container bucket (8, 16, 32, 64, 128, 256). Surface: `Identity::ZERO` and `Bounded::MAX` const access; `BitAccess::bit`, `BitAccess::with_bit_set`, `BitAccess::with_bit_cleared` const-context; `BitSequence::is_zero`, `BitSequence::count_ones`, `BitSequence::trailing_zeros`, `BitSequence::leading_zeros` const-context; `BitLogic::bitand`, `BitLogic::bitor`, `BitLogic::bitxor`, `BitLogic::bitnot` const-context. These already shipped impl const in Round 1; tests confirm the const lift across the container-dispatch matrix.

### Decision 3: F28 — UFixed / IFixed const-arith composition

**Test files:** `mock/crates/arvo/tests/ufixed_const_arith.rs`, `mock/crates/arvo/tests/ifixed_const_arith.rs`

Exercises `UFixed<I, F, S>` and `IFixed<I, F, S>` const composition: addition, subtraction, multiplication, division (with non-zero divisor), absolute value, identity / bounded constants, ZERO / ONE / MINUS_ONE inherent constants. Both files use representative width tuples covering Hot / Warm / Cold strategies. Each test asserts the result of a composition matches a hand-computed expected value, all inside `const _: () = { ... };` blocks.

### Decision 4: F29 — FastFloat / StrictFloat const-arith

**Test file:** `mock/crates/arvo/tests/float_const_arith.rs`

Exercises `FastFloat<F>` and `StrictFloat<F>` for F = 32, 64. Surface: addition, subtraction, multiplication, division const-context; reciprocal, sqrt const-context (where supported); identity / bounded const access. The semantic is "math-imprecise LLVM intrinsics route through fast where allowed, strict otherwise"; the test confirms both paths are const-callable.

### Decision 5: F30 — BitPrim / IBitPrim const-context

**Test file:** `mock/crates/arvo-bits-contracts/tests/bitprim_const_access.rs`

Exercises the `BitPrim` / `IBitPrim` const trait surfaces directly on the bare-primitive container types (u8, u16, u32, u64, u128 for unsigned; signed counterparts for IBitPrim). Surface: `BitPrim::WIDTH`, `BitPrim::mask_low`, `BitPrim::set_bit`, `BitPrim::clear_bit`, `BitPrim::has_bit` (and IBitPrim equivalents) const-context. Confirms the L0.5 contract layer is fully const-callable.

### Decision 6: F35 — MetaCarrier::as_bits const-context

**Test file:** `mock/crates/arvo-storage/tests/metacarrier_const_arith.rs`

Exercises `MetaCarrier::as_bits()` projection in const context. The carrier is the type-level mechanism Round 2 introduced for `Bits<const N: Width>` lift. The test confirms the projection compiles inside `const fn` bodies and `const _: () = { ... };` blocks.

### Decision 7: F36 — Resolve<Other>::Out projection const access

**Test file:** `mock/crates/arvo-strategy/tests/resolve_const_projection.rs`

Exercises `Resolve<Other>::Out` const-context access across the four strategy markers (Hot, Warm, Cold, Precise) and pairwise resolution outcomes. The cross-strategy resolution is the type-level operator Round 2 introduced for binary-op compatibility; the test confirms the projection is const-callable.

## Sketches needed

None. Each test exercises substrate already exercised by real consumer code. The audit findings explicitly framed these as confirmation-only; if the surface compiles in real consumer code today, the const-context test will compile too. The cl-claim-sketch-discipline rule's sketch-first criterion ("trait-solver-cycle risk, generic-const-expr risk, repr(transparent) layout risk, or any other 'does rustc actually accept this' question") is not triggered here.

## Lock criterion

This topic locks (frozen) when:

1. All seven decisions above have a test-file path and a surface enumeration.
2. No surface enumerated above is unimplemented (the substrate already exercises it through real consumer code per the audit).
3. The doc CL is opened referencing this topic.

## Out of scope

- New const-callable surface (none introduced; tests confirm existing surface).
- BACKLOG entries beyond the audit findings.
- Test infrastructure changes (use existing `cargo test` shape).
- Doc tmpl edits (the tests are confirmation-only; they don't claim new substrate).
- Notko-side tests (separate notko round if any are needed; audit findings F26-F30, F35-F36 are arvo-only).

## Cross-references

- `mock/research/audits/2026_05_02_expert_b_const_trait_completeness.md` (Findings F26-F30, F35-F36).
- `mock/design_rounds/202605040602/202605040602_changelist.src.lock.md` (Round 5 deferral notice).
- `.claude/rules/cl-claim-sketch-discipline.md` (no sketches needed per criterion).
- `.claude/rules/arvo-always-optimal-internals.md` (naive-baseline-first, why these tests confirm rather than introduce).
- Task #324 (this round closes it).
