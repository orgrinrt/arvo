---
hypothesis: Pattern C from sketch 07 can ship as the production projection trait, replacing UContainerFor + IContainerFor.
outcome: FAILS WITH design gap — sketch 07's Strategy-erased native bucket impls don't match production arvo's Strategy-aware bucket boundaries.
status: blocks #316 src CL apply; deprecates the unification claim from doc CL D3 + D5; redesign required before next attempt.
date: 2026-05-03
---

# Sketch 08: Strategy-aware native bucket gap (apply-time finding)

## What was tried

During SRC apply of #316, after creating `Width` (66cb4ce) and
`WideBits` + `AlignedWideBits16` (db047bb), the next step was to
replace `arvo-strategy/src/container.rs` with sketch 07's Pattern C
dispatch:

```rust
pub const trait BitsContainerFor<const N: u16, Sign: Signedness>: Strategy {
    type T: Copy + 'static;
}

pub trait Project<const TAG: usize, Sign: Signedness, const BYTES: usize, S: Strategy> {
    type T: Copy + 'static;
}

// 10 native impls (Strategy-erased; sketch 07's design):
impl<const BYTES: usize, S: Strategy> Project<0, Unsigned, BYTES, S> for Picker { type T = u8; }
// ... etc

// 8 wide impls (Strategy-aware: Hot=Aligned, others=plain).
// 4 BitsContainerFor impls (one per Strategy).
```

This matches sketch 07's validated shape verbatim.

## What broke

The downstream `widen.rs` macros emit per-N impls:

```rust
impl const UWidenFrom<Hot, 1> for Warm {
    fn u_widen(v: <Hot as BitsContainerFor<1, Unsigned>>::T)
        -> <Warm as BitsContainerFor<1, Unsigned>>::T { v as ... }
}
```

The trait expects (per sketch 07): `Warm at N=1 -> u8` (bucket 0,
Strategy-erased -> picks u8 unconditionally).

The existing arvo design says: `Warm at N=1 -> u16` (bucket 0,
Strategy-aware: Warm uses 2x logical -> u16, not u8).

The macro expansion `v as u16` for `Warm` source then mismatches
against the projection that resolves to `u8` (per sketch 07's
Strategy-erased rule). E0308 mismatched types fires.

## The underlying design gap

Sketch 07's bucket function and Project impls are Strategy-erased
across the 5 native buckets:

- bucket 0 (N=1..=8): all four strategies project to u8.
- bucket 1 (N=9..=16): all four project to u16.
- bucket 2: all four project to u32.
- bucket 3: all four project to u64.
- bucket 4: all four project to u128.

This was **a sketch-level simplification**, not the production
semantics. The production `UContainerFor` table in
`arvo-strategy/src/container.rs` (existing baseline) enforces:

- **Hot / Cold**: minimum aligned per bucket. `1..=8 -> u8`,
  `9..=16 -> u16`, `17..=32 -> u32`, `33..=64 -> u64`,
  `65..=128 -> u128`.
- **Warm / Precise**: 2x logical per bucket. `1..=8 -> u16`,
  `9..=16 -> u32`, `17..=32 -> u64`, `33..=64 -> u128`. **No native
  impl for 65..=128** (no u256). Consumers attempting
  `Uint<100, Warm>` get a compile-time error pointing at the
  `#[diagnostic::on_unimplemented]` note.

Sketch 07's Pattern C cannot express this without:

- A Strategy-aware `tag` function (different bucket boundaries for
  Hot/Cold vs Warm/Precise), OR
- 5 native buckets x 2 Sign x 4 Strategy = 40 native Project impls
  (instead of sketch 07's Strategy-erased 10) plus the
  Warm/Precise-at-65..=128 case routing to wide bucket, OR
- Direct per-Strategy `BitsContainerFor` impls without the helper
  trait, mirroring the existing per-N table but unified in N.

## Why sketch 07 didn't catch this

Sketch 07 was a feasibility check for the trait-solver mechanism
(does Pattern C compile? does it avoid E0119? does it project the
expected types for canonical (N, Sign, S) triples?). It tested
N=7/8/13/16 etc. with Hot and Warm and got identical native
container types for both because the sketch's `tag` function was
Strategy-erased.

The production arvo's bucket boundaries are not Strategy-erased
because the four strategies make different precision/throughput
tradeoffs (Hot wraps in min container; Warm has overflow headroom
in 2x; Precise saturates in 2x; Cold bitpacks). The bucket boundary
**is the design**, not an incidental simplification.

The audit (2026-05-03) flagged H2 ("Pattern C as the projection
mechanism") as a sketchable claim and sketch 07 confirmed the
mechanism. But H2 was about the dispatch trick (TAG-keyed Project
impls dodge E0119), not about whether Strategy-erased native
buckets fit production semantics. The latter was never asked.

## Outcome of the apply attempt

Reverted `container.rs`, `arith.rs`, `widen.rs`, `lib.rs` to
baseline at commit `2e295f0` (preserving the WideBits +
AlignedWideBits16 relocation to arvo-strategy and the Width newtype
in arvo-strategy::width — both validated additive steps).

Production state at HEAD `2e295f0`:

- `Width` is reachable as `arvo_strategy::Width` (also re-exported
  via arvo-storage). Used by future projection redesigns.
- `WideBits<const BYTES: usize>` and `AlignedWideBits16<const BYTES: usize>`
  exist in arvo-strategy and re-export through arvo-storage. Used
  by future wide-bucket dispatch.
- `UContainerFor` + `IContainerFor` + their per-N tables are intact.
  Production unchanged.
- `MultiContainer<HiT, LoT>` + `MultiContainerHalf` are intact.
  Production unchanged.

## Required redesign before re-attempting unification

The unified `BitsContainerFor` design needs one of:

1. **Strategy-aware tag function**: `tag_for_strategy(N, S) -> usize`
   const fn. Returns 0..=4 for native, 5 for wide. Boundary varies
   per Strategy. Project impls then key on this Strategy-specific
   tag.

2. **Direct per-Strategy impls without Project helper**: one
   `BitsContainerFor` impl per Strategy with a Strategy-specific
   tag function (`tag_hot`, `tag_warm`, etc.) inlined into the
   where-clause. Loses the "single helper trait" elegance but
   matches production semantics directly.

3. **Strategy-erased buckets at the storage level only**: keep
   sketch 07's Pattern C, but re-introduce a separate
   `LogicalToContainer` trait that does the 2x-logical mapping for
   Warm/Precise. UFixed/IFixed bind on both. This adds a layer of
   indirection that reframes the question (the strategy-aware
   bucket choice happens elsewhere).

4. **Drop the unification entirely**: ship Pattern C only for the
   wide bucket (N > 128, where all four Strategies project to
   either WideBits or AlignedWideBits16 cleanly), and keep the
   per-N table for native. Net win is the WideBits / AlignedWideBits16
   replacement of MultiContainer (storage simplification at the
   wide bucket); no per-N table cleanup.

Option 4 is the most conservative and maps cleanly to what the
audit + corrective topic actually validated. Sketch 07 proved
Pattern C works at the wide bucket; the native bucket cleanup was
the over-claim.

## Recommended scope reduction for the next round attempt

- **In scope**: introduce `WideBits` / `AlignedWideBits16` as the
  storage shape for N > 128. Replace `MultiContainer<u64, u128>` /
  `MultiContainer<u128, u128>` references in the per-N table with
  `WideBits<bytes_for(N)>` (Warm/Cold/Precise at high N) or
  `AlignedWideBits16<bytes_for(N)>` (Hot at high N). Add BitPrim
  impls on the new wide types. Delete `MultiContainer` +
  `MultiContainerHalf` once references are gone.

- **Out of scope (for this round)**: unify `UContainerFor` +
  `IContainerFor` into a single `BitsContainerFor` trait. The
  sketch-level simplification doesn't fit production. Keep the
  unified trait as a long-term goal; the redesign options above
  are tracked as future work.

- **Out of scope (for this round)**: lift `const N: u16` to
  `const N: Width`. The Width newtype is now reachable from
  arvo-strategy and ready for future use. The lift cascades through
  every consumer where-clause and is a separable concern.

This reduced scope ships the storage-shape simplification (the
audit-validated win) without claiming the projection-trait
unification (the apply-invalidated claim).

## Discipline lesson

Sketches answer specific feasibility questions. They do not validate
unrelated assumptions. The doc CL claimed both "Pattern C works"
(true, sketch 07) and "Pattern C unifies the projection without
loss" (untested by sketch 07; false at apply). Future sketches
should explicitly enumerate the production-semantics constraints
they DO validate vs. those they do NOT. The audit's PROCEED-WITH-
CHANGES verdict + corrective topic missed this gap because the
sketch's framing was internally consistent — the gap only surfaced
against the production code's existing per-Strategy bucket
boundaries.

Per `cl-claim-sketch-discipline.md`: this finding deprecates the
locked doc CL's D3 + D5 claims (delete UContainerFor/IContainerFor;
single per-Strategy impl via Pattern C). The src CL needs revision
to reflect the reduced scope (option 4 above).

## References

- Sketch 07: `07_native_or_wide_projection.rs` (validates Pattern C
  mechanism at the wide bucket; over-claims for native).
- Doc CL: `mock/design_rounds/202605031400_changelist.doc.lock.md`
  (D3, D5 — deprecated by this finding).
- Existing baseline: `arvo-strategy/src/container.rs` HEAD before
  attempted apply (the per-N table that captures the
  Strategy-aware bucket boundaries).
- Apply-attempt commits: container.rs rewrite + cascade reverted at
  HEAD `2e295f0`. WideBits/AlignedWideBits16 relocation kept.
