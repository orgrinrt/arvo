# Sketch: HList-style heterogeneous N-ary container

**Date**: 2026-05-03T14:00Z
**Status**: SKETCH 01 COMPLETE — DESIGN DECISION REQUIRED
**Outcome**: Heterogeneous HList does **not** deliver optimal-fit under `repr(C)`. The original design rationale collapses; design needs rethink.
**Tracks**: task #317 (foundational redesign sketches), task #316 (the redesign itself).

## Critical finding from Sketch 01

Sketch `01_hlist_basic.rs` ran on rustc 1.96.0-nightly, edition 2024. Real layout sizes:

| Shape | Logical bits | Physical bytes | Cons content | Trailing pad |
|---|---|---|---|---|
| `u128` | 128 | 16 | n/a | 0 |
| `Cons<u128, u64>` | 192 | 32 | 24 | 8 |
| `Cons<u128, u128>` | 256 | 32 | 32 | 0 |
| `Cons<u128, Cons<u64, u8>>` | 200 | 32 | 25 | 7 |
| `Cons<u8, Cons<u64, u128>>` | 200 | 48 | 25 | 23 |
| `Cons<u128, Cons<u128, u128>>` | 384 | 48 | 48 | 0 |

**The padding rule.** Rust requires `total_size % alignment == 0`. Struct alignment = `max(field alignments)`. Heterogeneous Cons of `(u128, u64)` has alignment 16; content is 24 bytes; total is rounded up to 32. This is fundamental Rust layout, not a workaround surface — `repr(C)` honors it, default repr also honors it, field reordering doesn't change it.

**Consequence.** `Cons<u128, u64>` (192 logical bits) uses the **same 32 bytes** as `Cons<u128, u128>` (256 logical bits). Heterogeneous offers **zero storage benefit** over homogeneous in the typical case. The "u128, u64, u32 to fit 224 bits exactly" intuition is wrong — the 224-bit shape pads to whatever the largest-aligned element demands.

**Where heterogeneous DOES help.** Only when all elements have equal alignment. `Cons<u64, Cons<u64, u64>>` (192 bits) = 24 bytes content, alignment 8, total 24. So 3-of-u64 fits exactly. But the same 192 bits in `[u64; 3]` = 24 bytes also exact. Heterogeneous gives nothing that homogeneous doesn't already give — it just lets you mix primitives, with no fitting advantage.

**Where heterogeneous can NEVER help.** Mixing primitives of different alignments (u128 + u64, u64 + u32, u32 + u16) always forces trailing-pad to the highest alignment. The "smallest exact fit" promise is impossible without `repr(packed)`.

## What this means for the design decision

The design conversation went:
> "Homogeneous loses optimal fit between primitive boundaries (e.g., 129..=192 with `u64+u128` = 192 bits)."

That's wrong under repr(C). `MultiContainer<u64, u128>` is 32 bytes (256 bits stored) for 192 logical bits — same waste as `MultiContainer<u128, u128>`. The current substrate's existing layout has the same waste already. The heterogeneous design saves nothing.

So the practical choice between heterogeneous and homogeneous reduces to:

| Approach | Storage (typical) | Type complexity | Trait-solver risk | Arity expressibility |
|---|---|---|---|---|
| Heterogeneous HList (repr C) | aligned-padded; same as homogeneous | High — recursive Cons types | High — recursive trait impls | Unbounded |
| Homogeneous `[T; N]` | aligned-padded; same as heterogeneous | Low — uniform array | Low — direct loop | Unbounded |
| Heterogeneous repr(packed) | True byte-exact | High + unaligned-access codegen cost + can't ref fields | High | Unbounded |
| Per-arity macro structs (`MC1<T0>`..`MC32<T0..T31>`) | Aligned-padded | Medium — N concrete types | None — flat impls | Bounded by macro emission |

Homogeneous `[T; N]` wins on every axis except the fictional optimal-fit advantage that turned out not to exist. The user's preference for heterogeneous was based on the belief that it preserved exact-bit storage; this sketch shows it does not.

## Recommendation

Pivot to **homogeneous `[T; N]`-backed MultiContainer**:

```rust
#[repr(C)]
pub struct MultiContainer<T: BitPrim, const COUNT: usize> {
    parts: [T; COUNT],
}
```

`T` is a base primitive (u8 / u16 / u32 / u64 / u128). `COUNT` is the multiplier. Tier projection picks `(T, COUNT)` per N range, e.g.:
- `N ≤ 8`: bare u8 (no MC wrapper)
- `N ≤ 16`: bare u16
- `N ≤ 32`: bare u32
- `N ≤ 64`: bare u64
- `N ≤ 128`: bare u128
- `N ≤ 256`: `MultiContainer<u128, 2>` (32 bytes — same as heterogeneous)
- `N ≤ 384`: `MultiContainer<u128, 3>` (48 bytes — same as heterogeneous)
- ... etc, no architectural cap

Single-impl projection via `feature(generic_const_exprs)` becomes:

```rust
pub const fn parts_count(n: u16) -> usize {
    let n = n as usize;
    if n <= 128 { 1 } else { (n + 127) / 128 }
}

impl<const N: Width, Sign> const BitsContainerFor<N, Sign> for Hot
where
    [(); parts_count(N.raw())]:,
    // sign axis projection
{
    type T = MultiContainer<u128, { parts_count(N.raw()) }>;
}
```

For N ≤ 64 the table still picks bare primitives (small-N fastpath). The MultiContainer covers 65+ in unified shape.

`BitPrim` impl over `MultiContainer<T, COUNT>` is one impl with a const-loop (LLVM unrolls for known COUNT):

```rust
impl<T: [const] BitPrim, const COUNT: usize> const BitPrim for MultiContainer<T, COUNT> {
    const WIDTH: USize = USize(<T as BitPrim>::WIDTH.raw() * COUNT);
    fn count_ones(self) -> USize {
        let mut sum = 0;
        let mut i = 0;
        while i < COUNT {
            sum += <T as BitPrim>::count_ones(self.parts[i]).raw();
            i += 1;
        }
        USize(sum)
    }
    // trailing_zeros, leading_zeros, get_bit, etc. — all const loops
}
```

Sketch 02 (next file in this directory) will implement this and verify.

## Sketches that this round will run

- `01_hlist_basic.rs` — DONE. Layout finding documented above.
- `02_homogeneous_multicontainer.rs` — TODO. Build `MultiContainer<T, const COUNT>` + const BitPrim impl. Verify count_ones / trailing_zeros etc. compile and produce correct values.
- `03_single_impl_projection.rs` — TODO. `BitsContainerFor<const N: Width, Sign>` single-impl via generic_const_exprs. Verify trait-solver doesn't cycle.
- `04_bits_with_homogeneous_mc.rs` — TODO. End-to-end Bits<const N: Width, S, Sign> from N=7 through N=1024.

## Decision required

The user's earlier call was heterogeneous N-ary specifically for optimal-fit. That rationale doesn't hold. Two paths:

  - **(a) Homogeneous pivot.** Drop heterogeneous; ship homogeneous `[T; N]` MultiContainer. Same storage cost, much cleaner. (My recommendation.)
  - **(b) Heterogeneous via repr(packed).** Pay unaligned-access cost for true byte-exact storage. Requires consumer-side discipline (no field refs, only field reads via `read_unaligned`). Substantial codegen impact.
  - **(c) Hybrid.** Default `Hot`/`Warm` use homogeneous (aligned, fast). `Cold` strategy already does manual bitpacking at column level — leave it; the repr(packed) alternative is unnecessary for `Cold`. So in practice we just go homogeneous everywhere for the `Bits<N>` container itself.

Path (a) and (c) are functionally equivalent — the substrate's `Cold` strategy doesn't need a repr(packed) container because Cold is column-bitpacked at a level above the per-Bits container.

## Notes for next agent / next session

If user picks (a) or (c): proceed with homogeneous `[T; N]` MultiContainer for sketches 02-04.
If user picks (b): rewrite Sketch 01 with `repr(packed)` and re-run layout assertions. Note that sketch 02-04 will need significant unaligned-access trait/method machinery.

Either way, this sketch's layout finding is the audit-trail record. The heterogeneous-vs-homogeneous question is decided by Rust layout rules, not by preference.

## Hypothesis

`MultiContainer<HiT, LoT>` (binary-nested) is the lazy choice. The substrate's `arvo-toolbox-not-policer` rule says we provide tools; capping at 256 bits is a policy. The audit (2026-05-03) confirmed: the cap exists because the per-N projection table stops at 255. Architecturally there's no reason to cap.

The right shape is a heterogeneous N-ary container. Each "part" can be a different primitive (`u128`, `u64`, `u32`, `u16`, `u8`) so optimal-fit holds for any logical width — `Bits<200, _, _>` projects to `u128 + u64 + u8 = 200 bits exactly`, no waste.

Rust does not have variadic generics. The standard rust-native pattern for type-level lists is HList: `Cons<H, T>` recursively + `Nil` base case. This sketch tests whether HList composes cleanly with the substrate's existing const-trait machinery (`BitPrim`, `ConstParamTy`, `Transparent`, `repr(transparent)` chains).

## What we want to verify

1. **Layout**: `Cons<H, T>` where H, T are themselves `BitPrim` types lays out predictably under `repr(C)`. Total size is `sizeof(H) + sizeof(T)` plus alignment padding. The padding question matters — packed layout via `repr(packed)` might be needed, with the soundness implications that brings.

2. **BitPrim composition**: a single recursive impl of `BitPrim` for `Cons<H, T>` (with `H: BitPrim, T: BitPrim`) covers count_ones, trailing_zeros, leading_zeros, get_bit, with_bit_set, etc. Base case is `Nil` (zero bits, all-zeros operations).

3. **Const-callability**: every method is `impl const` and the recursion terminates via `Nil` impl. No methods break const-fn-ness.

4. **Trait-solver cycle behavior**: when `Cons<H, T>` is used inside a generic `Bits<const N: Width, S, Sign>` projection, does the trait solver accept the recursive Cons traversal? This is the load-bearing question. MetaCarrier was created exactly to dodge a similar cycle.

5. **Optimal projection**: a const-fn or trait that, given `N: Width`, picks the smallest HList shape covering N bits. E.g., `optimal::<200>` = `Cons<u128, Cons<u64, Cons<u8, Nil>>>`.

## Sketch files

This directory will contain:

- `01_hlist_basic.rs` — Cons/Nil declarations + repr(C) layout + size assertions.
- `02_hlist_bitprim.rs` — recursive const BitPrim impl + Nil base case + sample composition tests.
- `03_optimal_projection.rs` — const-fn picking optimal HList shape for a given Width.
- `04_bits_with_hlist.rs` — Bits<const N: Width, S, Sign> projecting through HList.

Each sketch file is a standalone Rust file that compiles (or fails) under rustc 1.96.0-nightly with the substrate's feature gates.

## Fallback if HList trips the trait-solver cycle

Per-arity macro-generated structs:

```rust
pub struct MultiContainer1<T0> { p0: T0 }
pub struct MultiContainer2<T0, T1> { p0: T0, p1: T1 }
// ... up to MultiContainer16<T0..T15>
```

Each `MultiContainerK` has its own non-recursive `impl const BitPrim`. The projection table picks `(T0, T1, ..., Tk)` per N range. Caps at K_MAX × u128 = 16 × 128 = 2048 bits with K=16, or 32 × 128 = 4096 bits with K=32. The cap is a macro-emission knob.

This loses: clean type-level recursion, single-impl projection.
This gains: zero trait-solver cycle risk, predictable compile time.

## Decision matrix

| Sketch outcome | Action |
|---|---|
| All four sketches compile | Land HList redesign in #316. Single-impl projection, no per-N table, no cap. |
| Sketch 01-02 compile, 03-04 cycle | Use HList for the container shape but keep a per-N projection table that emits `Cons<...>` types per range. Loses single-impl but keeps optimal-fit + arbitrary cap. |
| Sketch 01-02 cycle | Fall back to per-arity macro-generated structs (`MultiContainer1..MultiContainerN`). Cap = N × u128. |
| Layout fails (padding) | `repr(packed)` chain or rethink. Possibly fall back to homogeneous `[u128; N]` for the bulk + small explicit tail. |

## Notes for next agent

The substrate already uses `feature(generic_const_exprs)` in places (e.g., `ufixed_bits(I, F)` const-fn projection at `arvo/src/strategy.rs:30-40`). The HList approach pushes that machinery harder. Any cycle that emerges here probably emerges in similar shape elsewhere; documenting the cycle precisely is itself useful.

The MetaCarrier resolution from round 202604280806 is the canonical workaround pattern: introduce a layout-equivalent companion type that bypasses the trait-solver cycle while preserving the underlying layout. If HList trips, the analogous workaround would be a `HListCarrier<const SHAPE: ShapeDescriptor>` newtype that the trait solver sees as a single concrete type — but that defeats heterogeneity. So if HList trips, the fallback is the per-arity structs, not a MetaCarrier-style workaround.
