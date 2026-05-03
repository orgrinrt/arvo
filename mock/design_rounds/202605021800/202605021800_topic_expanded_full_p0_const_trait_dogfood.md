# Round 202605021800 — full P0 const-trait + dogfood foundations (expanded scope)

**Date:** 2026-05-02
**Phase:** TOPIC
**Scope:** arvo-strategy / arvo-storage / arvo-bits-contracts / arvo (facade)
**Source topics:** post-202605021600 audit findings (Expert A architectural dogfooding + Expert B const-trait completeness, both 2026-05-02). User redirect 2026-05-02: fold all P0 substrate work originally scoped across multiple rounds into Round 1 ("no deferrals or kicking the bucket; all P0 in first steps; prefer early to late").

## Supersedes

This topic supersedes (does NOT replace — that file stays frozen as audit trail) the prior topic file `202605021800_topic_substrate_const_trait_foundation.md` and the deprecated changelist `202605021800_changelist.doc.deprecated.md`. The prior topic narrowed Round 1 to lifts + bridges only and explicitly deferred MultiContainer BitPrim / IBitPrim parity / signed Bits BitLogic / numeric-contracts impl const / UWidenFrom const to Round 2. User feedback: "all P0 in the first steps like they should... no deferrals or kicking the bucket. Meaning the Multicontainer BitPrim here, but others in similar grey area, prefer early to late."

This topic expands Round 1 to cover every P0-marked finding from both audits except the Bits&lt;const N: Width&gt; lift (which has its own coherent scope and biggest blast radius — earns Round 2).

## Decisions to record

All decisions from the prior topic file stand:

- **Decision 1** (sealed marker trait const lifts): Signedness, BitsContainerFor, OverflowPolicy/ContainerWidth/StorageLayout/HasAxes, CrossStrategyOp, Ieee, IntegerLike/FractionLike/BitPresentation/FloatLike/BoolLike, AsBool → `pub const trait` + `impl const`.
- **Decision 2** (substrate bridges): `ConstEq`, `ConstOrd`, `ConstDefault` in `arvo-strategy`, with per-primitive-wrapper coverage. `ConstFrom`/`ConstTryFrom`/`ConstDeref`/`ConstAsRef`/`ConstTry`/`ConstHash` deferred to Rounds 4-5 (no in-substrate const-context use-case until then).
- **Decision 3** (UPrimConst → Identity collapse): delete UPrimConst/IPrimConst; introduce `pub const trait SignedIdentity: Identity { const NEG_ONE: Self; }`; rewire arith.rs body references.
- **Decision 4** (USize / Cap canonical): impl const Bounded + Identity; extend `impl_bounded_identity_*!` to platform-pointer-width primitives.

In addition, the following are now in-scope for Round 1:

### Decision 5: MultiContainer gains const BitPrim + Bounded + Identity impls

Per Expert A Finding 2 / Expert B Finding 4. `MultiContainer<HiT, LoT>` ships at the storage level since round 202604280500, but currently exposes no bit-level surface. `Bits<256, Hot, Unsigned>` is constructible but no const-callable bit ops resolve on it. The 129..=255-bit band — explicitly the substrate's primary downstream workload (loimu-style large-entity-count bitpacked column-store) — is dead substrate.

Concrete:

- `impl<HiT: [const] BitPrim, LoT: [const] BitPrim> const BitPrim for MultiContainer<HiT, LoT>` in `arvo-strategy/src/multi_container.rs` (or in `arvo-bits-contracts` if orphan rules require — pick at SRC-PLAN time).
- `WIDTH = HiT::WIDTH + LoT::WIDTH` (USize-arithmetic; depends on Decision 4's USize Bounded).
- `count_ones(self) = hi.count_ones() + lo.count_ones()` (USize-additive; tracks Round 3's BitPrim::WIDTH-typed lift).
- `trailing_zeros(self) = if lo.is_zero() { LoT::WIDTH + hi.trailing_zeros() } else { lo.trailing_zeros() }` — uses `is_zero` substrate bridge (already shipped in round 202605021600).
- `leading_zeros(self) = if hi.is_zero() { HiT::WIDTH + lo.leading_zeros() } else { hi.leading_zeros() }`.
- `bitor`, `bitand`, `bitnot`, `bitxor`: element-wise on hi+lo halves.
- `with_bit_set(idx)`, `with_bit_cleared(idx)`, `with_bit_toggled(idx)`, `get_bit(idx)`: select half by `idx < LoT::WIDTH`.
- `is_zero(self) = hi.is_zero() && lo.is_zero()`.
- `clear_lowest_set_bit(self)`: route through lo first, fallback hi.
- `impl<HiT: [const] Bounded, LoT: [const] Bounded> const Bounded for MultiContainer<HiT, LoT>`: `MIN = MultiContainer { hi: HiT::MIN, lo: LoT::MIN }`; `MAX = MultiContainer { hi: HiT::MAX, lo: LoT::MAX }`.
- `impl<HiT: [const] Identity, LoT: [const] Identity> const Identity for MultiContainer<HiT, LoT>`: `ZERO = MultiContainer { hi: HiT::ZERO, lo: LoT::ZERO }`; `ONE = MultiContainer { hi: HiT::ZERO, lo: LoT::ONE }`.

Naming clash resolution (Expert A Finding 15): the existing sealed-marker `arvo_strategy::multi_container::BitPrim` (used as a sealing-trait predicate on MultiContainer halves) renames to `MultiContainerHalf` to free the `BitPrim` name for its canonical role at `arvo_bits_contracts::BitPrim`. The arvo-storage re-export at `lib.rs:23` switches accordingly.

### Decision 6: IBitPrim gains parity with BitPrim

Per Expert B Finding 5. `IBitPrim` currently has count_ones / trailing_zeros / leading_zeros / get_bit / with_bit_set / with_bit_cleared / with_bit_toggled. Missing vs BitPrim: is_zero, bitor, bitand, bitnot, bitxor, clear_lowest_set_bit.

Concrete:

- Extend `pub const trait IBitPrim` declaration in `arvo-bits-contracts/src/lib.rs:210-241` with the six missing methods, with default-method bodies routing through the `$uty` reinterpretation pattern already established for shifts (lines 380-410 in the macro impls).
- `is_zero(self) = (self as $uty) == 0`. (Const-stable on concrete primitives via existing `BitPrim::is_zero` pattern.)
- `bitor/bitand/bitxor/bitnot`: `((self as $uty) <op> (other as $uty)) as $ity`.
- `clear_lowest_set_bit(self) = ((self as $uty) & ((self as $uty).wrapping_sub(1))) as $ity`.
- Add to the `impl_bit_prim_i!` macro at lines 355-411.

This unblocks Decision 7 (signed Bits BitLogic) and gives consumer code in algorithm crates const-callable `is_zero` on signed weights.

### Decision 7: BitAccess / BitSequence / BitLogic blanket impls generalise to `Bits<N, S, Sign>`

Per Expert B Finding 3. Current blanket impls bind on `Bits<N, S>` (which is `Bits<N, S, Unsigned>` per the Sign default). `IFixed<I, F, S>` wraps `Bits<{1+I+F}, S, Signed>`, so signed-axis Bits has zero const-callable bit-op surface.

Concrete:

- Generalise the blanket signature in `arvo-bits-contracts/src/bits_impl.rs:14-94` from `impl<const N: u16, S: Strategy> const HasBitWidth for Bits<N, S>` to `impl<const N: u16, S: Strategy, Sign: Signedness> const HasBitWidth for Bits<N, S, Sign>` (Signedness now const per Decision 1).
- Route the bound from `S: [const] UContainerFor<N>` to `S: [const] BitsContainerFor<N, Sign>` (BitsContainerFor now const per Decision 1).
- Cycle-avoidance: collapse the BitPrim-or-IBitPrim selection through a single sealed bridge predicate. Two viable approaches:
  - (a) New sealed bridge trait `pub const trait BitsBitPrim<Sign> { type T: ...; }` that selects `BitPrim` or `IBitPrim` based on `Sign`. Single-predicate `where <S as BitsContainerFor<N, Sign>>::T: [const] BitsBitPrim<Sign>`.
  - (b) Two impl blocks per consumer trait: one for `Sign = Unsigned`, one for `Sign = Signed`, each with the appropriate single-predicate bound. Avoids cycle through specialisation-shaped distinct impl blocks.
  - SRC-PLAN time picks one. Default: (a) if it compiles cleanly; (b) as fallback.
- Same shape for `BitAccess` (lines 21+), `BitSequence` (lines 44+), `BitLogic` (lines 73+).
- Once these blankets generalise, `Bits<N, S, Signed>` and consequently `IFixed<I, F, S>` gain const-callable bit ops.

### Decision 8: Numeric-contracts trait impls lift to `impl const`

Per Expert B Finding 13. Trait declarations are already `pub const trait` (Sqrt, Recip, Abs, FromConstant, TotalOrd, Predicate). Their impls in `arvo/src/traits.rs` are plain `impl X for Y` despite pure bodies. Massive surface: hundreds of impls across (strategy, I, F, container) cells.

Concrete:

- Add `const` to every `impl X for Y` block in `arvo/src/traits.rs` (Sqrt, Recip, Abs, FromConstant, TotalOrd impls and their macro callers).
- Free fns `sqrt_f32`, `sqrt_f64`, `abs_f32`, `abs_f64` lift to `pub const fn`. Verify `f32::from_bits` / `f32::to_bits` / `u*::isqrt` are const-stable on rustc 1.96.0-nightly; gate via `feature(const_int_sqrt)` if needed.
- The TotalOrd impls call `cmp` / `partial_cmp` — route through `ConstOrd::const_cmp` (Decision 2) for in-substrate const-context callers.

### Decision 9: Predicate family gains impls

Per Expert B Finding 14. `Predicate`, `IsZero`, `IsPositive`, `IsNonZero`, `IsNonNegative`, `IsZeroOrPositive` are declared `pub const trait` in arvo-numeric-contracts but never impl'd anywhere. Stranded substrate.

Concrete:

- Per-(I,F,S) `impl const Predicate for UFixed<I, F, S>` blanket: `Predicate::test(self)` checks the value against context-specific zero via `<Self as Identity>::ZERO` and `ConstEq::const_eq`.
- Same shape for `IFixed<I, F, S>` and `Bits<N, S, Sign>`.
- The five named-predicate impls (IsZero, IsPositive, IsNonZero, IsNonNegative, IsZeroOrPositive) delegate to Predicate::test plus an additional sign / nonzero check.

### Decision 10: UWidenFrom / UNarrowFrom / IWidenFrom / INarrowFrom → const trait

Per Expert B Finding 10. Strategy bridges are `pub trait` with all-pure bodies (`v as $dst_ty` casts and bounded comparison branches with `Outcome::Ok`/`Err`). Lift to `pub const trait` and `impl const` at all four macro-emitted impl families.

Concrete:

- `arvo-strategy/src/widen.rs:27-57`: declarations to `pub const trait UWidenFrom<Src: [const] UContainerFor<N> + HasAxes, const N: u16>: [const] UContainerFor<N> + HasAxes { fn u_widen(v: Src::T) -> Self::T; }` and three siblings.
- `arvo-strategy/src/widen.rs:64-119, 122-286`: every macro-emitted `impl ... for ...` lifts to `impl const ... for ...`.
- Verify `notko::Outcome::Ok` / `Outcome::Err` constructors are const-callable inside `impl const` (per Expert B Finding 22).

### Decision 11: UFixed / IFixed Clone / PartialEq → ConstEq

Per Expert B Finding 25. Hand-rolled `impl Clone for UFixed/IFixed` (body `*self`) and `impl PartialEq for UFixed/IFixed` (body `self.to_raw() == other.to_raw()`) are pure but stdlib non-const.

Concrete:

- Stdlib `Clone` and `PartialEq` impls stay (boundary coverage).
- Add `impl const ConstEq for UFixed<I, F, S>` and `impl const ConstEq for IFixed<I, F, S>` routing through `<Bits as ConstEq>::const_eq` on the inner.

### Decision 12: Float Identity dogfood

Per Expert A Finding 10. `Ieee` declares own `ZERO`/`ONE`; FastFloat / StrictFloat don't blanket Identity through inner.

Concrete:

- `impl const Identity for f32 { const ZERO: Self = 0.0; const ONE: Self = 1.0; }` and same for f64.
- Drop `Ieee::ZERO` / `Ieee::ONE`; supertrait-bound `Ieee: Identity` (cascade with Decision 1 lifting Ieee to const trait).
- `impl<F: [const] Ieee + [const] Identity> const Identity for FastFloat<F> { const ZERO = Self(<F as Identity>::ZERO); const ONE = Self(<F as Identity>::ONE); }` and same for StrictFloat.

### Decision 13: Out of scope (deferred to Rounds 2-5 within PR #42)

Explicitly NOT in this round, even though some are P0:

- `Bits<const N: u16, ...>` → `Bits<const N: Width, ...>` lift (Expert A F5) — Round 2 (#312). Biggest blast radius single change in the substrate; deserves coherent dedicated round. P0 but separable.
- MetaCarrier resolution (Expert A F18) — Round 2 (#312). Cascades from Bits<const N: Width> lift.
- LOGICAL_WIDTH typed arithmetic via IBits + FBits → Width (Expert A F13) — Round 2 (#312). Cascades.
- Mask&lt;W&gt; over Bits / BitMatrix&lt;W,N&gt; / mask-contracts impl (Expert A F1, F7, F12; Expert B F17, F32) — Round 3 (#313). P1.
- BitPrim::WIDTH typed (Expert A F8) — Round 3 (#313). P1.
- ConstFrom / ConstTryFrom / ConstDeref / ConstAsRef bridges (Expert B F6, F8, F24) — Round 4 (#314). P1.
- Algorithm crate USize counter sweep (Expert A F6) — Round 4 (#314). P1.
- mask_low_bits substrate helper (Expert A F9) — Round 4 (#314). P1.
- Smoke test coverage (Expert B F26-F30, F35-F36) — Round 5 (#315). P2.
- ConstHash / ConstHasher (Expert B F40) — Round 5 (#315). P1 but deferred to bundle with smoke tests.
- ConstTry / ConstControlFlow on Bool (Expert B F21) — Round 5 (#315). P2.
- Bitfield ConstEq / ConstDefault (Expert B F38) — Round 5 (#315). P1 cascade after Round 1's bridges land.
- Binpack Maybe sentinel (Expert A F14) — Round 5 (#315). P2.

Per user instruction "prefer early to late": each P0 deferral above has a concrete dependency reason (Bits<const N: Width> needs its own coherent scope; mask-contracts impl waits on Mask<W> generalisation which itself waits on Round 1's MultiContainer BitPrim being shipped; ConstHash bridges need substrate-bridge pattern stabilised first).

### Decision 14: Round closure criterion

This round closes when:

1. All decisions 1-12 land in source.
2. `cargo check` clean across the substrate workspace.
3. `cargo test` clean.
4. Lint gates clean at `--commit` and `--push` severity.
5. No new lint:allow markers added (this round's work resolves several existing tracked-256 markers).
6. PR #42 description updated to reflect Round 1 (expanded) scope.

## Cited Source Material

Audit reports:
- `mock/research/audits/2026_05_02_expert_a_architectural_dogfooding.md`
- `mock/research/audits/2026_05_02_expert_b_const_trait_completeness.md`

Workspace rules invoked:
- `arvo-toolbox-not-policer.md` — substrate ships tools, never policies
- `arvo-always-optimal-internals.md` — public API rules; internals do whatever's optimal
- `arvo-compile-time-last.md` — runtime first, compile time last (this round expands runtime/consumer-ergonomics surface)
- `no-bare-primitives.md`
- `use-the-stack-not-reinvent.md`

Prior frozen topic (audit trail):
- `202605021800_topic_substrate_const_trait_foundation.md`

Deprecated changelist (audit trail):
- `202605021800_changelist.doc.deprecated.md`

Closed predecessor rounds on PR #42 branch:
- 202605021200 (USize/Cap const-arith + Layer C sweep)
- 202605021400 (UArith/BitPrim const-trait machinery)
- 202605021600 (Bounded/Identity bridges + Mask cleanup)
