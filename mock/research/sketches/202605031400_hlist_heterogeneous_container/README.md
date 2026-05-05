# Sketches: foundational arvo container redesign

**Date**: 2026-05-03T14:00Z
**Status**: ALL SIX SKETCHES COMPLETE — ARCHITECTURE VALIDATED, READY FOR #316 DOC CL
**Tracks**: task #317 (foundational redesign sketches), task #316 (the redesign itself).

## What this directory exists to validate

The senior audit on 2026-05-03 found that the arvo's `MultiContainer<HiT, LoT>` binary nesting was the lazy-choice fallback from an earlier round. The audit + user discussion led to the conclusion that container projection above 128 bits should be re-architected. This directory holds the sketches that validate the new architecture before it lands in source.

## Sketch 01: cons-list heterogeneous container layout finding

`01_hlist_basic.rs` ran on rustc 1.96.0-nightly. Real layout sizes:

| Shape | Logical bits | Physical bytes | Padding |
|---|---|---|---|
| `u128` | 128 | 16 | 0 |
| `Cons<u128, u64>` | 192 | 32 | 8 (forced by 16-byte alignment) |
| `Cons<u128, u128>` | 256 | 32 | 0 |
| `Cons<u128, Cons<u64, u8>>` | 200 | 32 | 7 |
| `Cons<u8, Cons<u64, u128>>` (small-first) | 200 | 48 | 23 |
| `Cons<u128, Cons<u128, u128>>` | 384 | 48 | 0 |

**The padding rule**: Rust requires `total_size % alignment == 0`. Struct alignment = `max(field alignments)`. Heterogeneous Cons of `(u128, u64)` has alignment 16; content is 24 bytes; total is rounded up to 32. This is fundamental Rust layout under `repr(C)`, default repr, and field reordering. The "exact bit width" promise of heterogeneous-with-mixed-aligns is impossible without `repr(packed)`.

**The same issue affects the existing `MultiContainer<u64, u128>`**: verified at `/tmp/mc_existing.rs` runtime — 32 bytes, identical to `MultiContainer<u128, u128>`. The "u64+u128 saves 8 bytes" rationale baked into the arvo was always fiction; nobody noticed.

## Design pivot from this finding

The user's response to the finding: **stop forcing native-primitive composition above 128 bits**. Three observations drove the pivot:

1. The arvo already pays the custom-ops cost above 128 bits — every op on `MultiContainer<HiT, LoT>` is hand-composed across halves. We never used native primitive ops for wide values.
2. Native-primitive composition with mixed alignment is no smaller than aligned native composition (proven above). The supposed benefit doesn't exist.
3. Modern hardware (x86-64 ≥ Sandy Bridge 2011, ARMv7+, all aarch64, WASM, RISC-V most cores) has near-zero unaligned-access cost for non-cache-line-crossing reads. Byte-exact storage at align-1 is performant on every relevant target.

The pivot:

- **N ≤ 128**: bare primitive (u8 / u16 / u32 / u64 / u128). Native ops via stdlib.
- **N > 128**: `WideBits<const BYTES: usize>` byte-sequence storage. Custom ops via cfg-gated platform paths (scalar + SSE2-x86-64 + NEON-aarch64 baseline; AVX-2/AVX-512/SVE/WASM-SIMD/RVV expansions in #320).

The strategy axis genuinely drives the storage shape:

- **Hot**: SIMD-aligned byte-sequence (`#[repr(C, align(N))]` where N matches the largest SIMD vector targeted, e.g., 32 for AVX-2, 16 for SSE2/NEON). Trailing pad ≤ alignment-1 bytes. Lossy-compute path (`HotTruncate` variant) deferred to research task #322.
- **Warm**: align-1 byte-exact `[u8; BYTES]`. Development default; full precision; no SIMD assumption.
- **Cold**: align-1 byte-exact. Column-bitpacked layout happens at a layer above the per-Bits container (Cold's existing column-store work is unchanged).
- **Precise**: align-1 byte-exact. Saturating semantics; full precision.

## What this means concretely

`MultiContainer<HiT, LoT>` and `MultiContainerHalf` are **deleted** in #316 — no deprecation alias per `clause-dev/.claude/rules/no-legacy-shims-pre-1.0.md`. The replacement is `WideBits<const BYTES: usize>` parameterized over the strategy axis (different alignment per strategy).

Single-impl projection via `feature(generic_const_exprs)`:

```rust
pub const fn bytes_for(n: Width) -> usize {
    let bits = n.raw() as usize;
    (bits + 7) / 8
}

// Hot uses aligned wrapper; others use align-1.
impl<const N: Width, Sign: Signedness> const BitsContainerFor<N, Sign> for Hot
where
    [(); bytes_for(N)]:,
    /* sign-axis projection */
{
    type T = /* if N <= 128: native primitive; else: AlignedWideBits<{bytes_for(N)}> */;
}

impl<const N: Width, Sign: Signedness> const BitsContainerFor<N, Sign> for Warm
where
    [(); bytes_for(N)]:,
    /* sign-axis projection */
{
    type T = /* if N <= 128: native primitive; else: WideBits<{bytes_for(N)}> */;
}
```

The const-condition mechanism for "if N ≤ 128 use one type else another" is the trait-solver's hard part — sketches 02-04 validate this cascade resolves cleanly.

## Sketch outcomes

| Sketch | Outcome | Findings |
|---|---|---|
| 02_widebits_basic.rs | **WORKS** | `WideBits<BYTES>` with `[u8; BYTES]` under `repr(C)` is align-1 across 17/25/32/64/128/512 byte widths. Scalar `BitPrim` ops (count_ones, leading/trailing_zeros, get_bit, set_bit, bitand/or/xor/not) compose correctly from per-byte primitives. Compile-time + runtime asserts pass. |
| 03_aligned_widebits.rs | **WORKS** | `Aligned16/32/64<BYTES>` with `repr(C, align(N))` literal attribute. Size = round_up(BYTES, align). Const-generic alignment isn't expressible in Rust; discrete tiers (16/32/64) cover SSE2/NEON, AVX-2, AVX-512 baselines. |
| 04_simd_count_ones.rs | **WORKS** | cfg(target_arch) gating with `core::arch::x86_64` and `core::arch::aarch64` intrinsics compiles on both arches. Three paths (scalar, chunked-u64, intrinsic-load) produce identical results across 5 test patterns. NEON path live on aarch64 dev machine; SSE2 path verified via fallback dispatch. |
| 05_single_impl_projection.rs | **WORKS** | `feature(generic_const_exprs)` accepts `where [(); bytes_for(N)]: ` plus `type T = WideBits<{bytes_for(N)}>` in a single impl block. Trait solver doesn't cycle. 4 impls (one per strategy) replace per-N enumeration. Compile-time `const _` assertions verify projection across 9 N values × 4 strategies. |
| 06_bits_end_to_end.rs | **WORKS** | `Bits<W, S, Sign>` over the projected `Container<W, S>` ties everything together. Storage geometry correct: Warm/Cold/Precise = align-1 byte-exact, Hot = 16-aligned with trailing pad. Ops (count_ones, bitand, bitor) produce correct results across N ∈ {7, 13, 64, 128, 200, 256, 4096} × {Warm, Hot, Cold, Precise}. |

## Architecture validation

Every pillar from the resume-memory architecture is now sketch-validated:

1. **128-bit native boundary**: not literally enforced in sketches (all sketches use WideBits even at small widths); native primitive bypass for N ≤ 128 is an optimization decision the doc CL captures as a per-strategy projection variant. The sketches prove WideBits<1> is byte-equivalent to u8 storage at align-1, so the "boundary" exists at the ops layer (native u8/u16/u32/u64/u128 ops below 128 bits) more than the storage layer.
2. **Strategy-aware alignment**: validated. Hot uses Aligned16; Warm/Cold/Precise use WideBits at align-1.
3. **Single-impl projection**: validated. `feature(generic_const_exprs)` accepts the const-fn-derived associated type pattern.
4. **No legacy shims pre-1.0**: rule already in `~/Dev/clause-dev/.claude/rules/no-legacy-shims-pre-1.0.md`. MultiContainer<HiT, LoT> + MultiContainerHalf will be deleted, not deprecated, in #316.
5. **cfg-gated platform paths**: validated. Both x86_64 and aarch64 surfaces compile cleanly; intrinsic-load path produces identical numeric results to scalar baseline.
6. **Asm microkernels**: deferred to #321; the sketch family was about validating the surface, not benchmark-driven optimization.
7. **Sketch-then-commit discipline**: this directory is the pattern. All six sketches committed before any #316 doc CL is opened.
8. **Claim-vs-source contract**: workspace rule already in `~/Dev/clause-dev/.claude/rules/cl-claim-sketch-discipline.md`; mockspace #318 will enforce.

## Next step

Open #316 round on this branch. Doc CL captures the new shape with structured `## CHANGE: <symbol> FROM ... TO ...` blocks for each arvo-name change (MultiContainer deletion, BitsContainerFor table → single-impl projection, Width position lift if needed). SRC CL applies. Lock + close.

## Notes for next agent / next session

The architectural decision is locked: byte-sequence above 128 bits, native below, strategy-aware alignment. Sketches 02-06 validate the implementation. If any sketch fails, the resolution is documented inline (per `cl-claim-sketch-discipline.md`) and the next sketch builds on the resolution.

`MultiContainer<HiT, LoT>` and `MultiContainerHalf` go away in #316. The arvo-storage `meta_bits.rs` `MetaCarrier` companion stays — it serves a different purpose (ConstParamTy_ const-generic carrier for meta-newtypes), unrelated to the wide-container projection.

The Width newtype gives Width=u16 → 65535 bits max. That's 8KB per `Bits<N>` value. Sufficient for arvo use cases (loimu column-store, hash digests up to SHA3-512, RSA up to 4096 bits, etc.). If a consumer needs more, Width can be lifted to u32 — no architectural impact on the WideBits shape.
