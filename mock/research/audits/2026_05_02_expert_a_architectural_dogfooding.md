# arvo Substrate Dogfooding Audit (Expert A, 2026-05-02)

Audit dispatched 2026-05-02 against branch `feat/usize-const-arith` (PR #42) at commit `daf9518`. Audit charge: find every place arvo fails to dogfood its own framework — every parallel hand-rolled width-specific type, every hardcoded primitive literal, every concrete impl that should be a generic blanket impl.

The substrate has built rich abstractions (`Bits<N, S, Sign>`, `MultiContainer<HiT, LoT>`, `Bounded`, `Identity`, `BitPrim`, `BitLogic`) but does not consistently consume them itself.

---

### Finding 1: `Mask256` is a hand-rolled parallel struct, not `Mask<W>`

**Location:** `arvo-bitmask/src/mask.rs:88-123`, plus the entire 256-specific surface in `ops.rs:174-456` and `dirty.rs:48-72`, and `BitMatrix256` in `matrix.rs:137-239`.

**Smell:** `Mask256` is `pub struct Mask256(pub [QWord<Hot>; 4])`, not `Mask<Bits<256, Hot, Unsigned>>`. Module docs at `mask.rs:11-15` justify it as "Rust arrays don't implement the arvo-bits traits", but `arvo-storage::Bits<256, Hot, Unsigned>` ALREADY exists with a `MultiContainer<HiT, LoT>` projection (round 202604280500), and `BitsContainerFor<256, Unsigned>` ALREADY routes through it. The substrate built the mechanism; the bitmask crate ignores it. Result: 280 lines of unrolled `a[0..3]` boilerplate that should be N lines of generic code, and `Mask4096` / `Mask1024` / `Mask128` are forced to be hand-authored or absent.

**Substrate generalisation:** Add `BitAccess + BitSequence + BitLogic + HasBitWidth` impls on `MultiContainer<HiT, LoT>` (or on `Bits<N, S, Sign>` where the container is `MultiContainer<...>`). Recursive composition: `bitor(a, b) = MultiContainer { hi: BitLogic::bitor(a.hi, b.hi), lo: BitLogic::bitor(a.lo, b.lo) }`. With those impls, `Mask256` collapses to `pub type Mask256 = Mask<Bits<256, Hot, Unsigned>>`.

**Priority:** P1. Depends on Finding 2.

---

### Finding 2: `BitLogic` / `BitAccess` / `BitSequence` impls on `Bits<N, S, Sign>` are not extended to `MultiContainer`-backed widths

**Location:** `arvo-strategy/src/multi_container.rs:78-89`, `arvo-bits-contracts/src/bits_impl.rs`, `arvo-bits-contracts/src/lib.rs:434-491`.

**Smell:** `MultiContainer<HiT, LoT>` ships at the storage level, but the L0.5 contracts (`BitAccess`, `BitSequence`, `BitLogic`) are only impl'd for `Bits<N, S, Sign>` when the container is a single bare primitive. `Bits<256, Hot>`, `Bits<200, Cold>`, etc. are constructible (via the dispatch table) but expose no bit-level surface.

**Substrate generalisation:** Implement `BitPrim` for `MultiContainer<HiT: BitPrim, LoT: BitPrim>`. `count_ones` = `hi.count_ones() + lo.count_ones()`. `trailing_zeros` = `if lo.is_zero() { LoT::WIDTH + hi.trailing_zeros() } else { lo.trailing_zeros() }`. `bitor/and/xor/not` map element-wise. `is_zero` = `hi.is_zero() && lo.is_zero()`.

**Priority:** P0. Foundational. Findings 1, 7, 9 all wait on this.

---

### Finding 3: `Bounded` and `UPrimConst` are duplicate trait surfaces over the same primitives

**Location:** `arvo-strategy/src/arith.rs:394-411` (Bounded/Identity), `arvo-strategy/src/arith.rs:461-507` (UPrimConst/IPrimConst).

**Smell:** Two parallel const-trait surfaces ship for the same answer. `<u8 as Bounded>::MAX = u8::MAX` and `<u8 as UPrimConst>::MAX = u8::MAX` are the same value reached two different ways. The `is_zero` body inside `arith.rs:120` reaches for `<<Self as UContainerFor<$bits>>::T as UPrimConst>::ZERO` rather than the equally valid `<<...>::T as Identity>::ZERO`.

**Substrate generalisation:** Collapse `UPrimConst` / `IPrimConst` into `Bounded + Identity` plus `pub const trait SignedIdentity: Identity { const NEG_ONE: Self; }` for the signed -1 case. Rewrite arith.rs internal references through `<T as Identity>::ZERO` / `<T as Bounded>::MAX`.

**Priority:** P0. Foundational hygiene.

---

### Finding 4: `USize` and `Cap` don't impl `Bounded` / `Identity`

**Location:** `arvo-storage/src/platform.rs:29-35, 158-161`.

**Smell:** USize and Cap each carry inherent `pub const ZERO / ONE / MAX` constants — exactly the pattern Mask was just refactored away from. They predate `Bounded` / `Identity` and don't yet route through them. `usize::MAX` is reached as a literal at platform.rs:34, not `<usize as Bounded>::MAX`.

**Substrate generalisation:** Implement `Bounded` and `Identity` for `USize` and `Cap`. Extend `impl_bounded_identity_u!` to include `usize`; `impl_bounded_identity_i!` to include `isize`.

**Priority:** P0.

---

### Finding 5: `Bits<const N: u16, ...>` uses bare `u16` for the const generic; should be `Width`

**Location:** `arvo-storage/src/bits.rs:51`, `arvo-bits-contracts/src/lib.rs` (entire file), `arvo-strategy/src/container.rs:26, 46, 71`, `arvo-strategy/src/arith.rs:40, 55`.

**Smell:** The const generic on `Bits` is bare `u16`. The `Width` newtype was created precisely for this position. Per `no-bare-primitives.md` definition-site exception 4 (ergonomic helper-fn parameters), the `width(n: u16) -> Width` helper exists; the substrate intends `Bits<{ width(12) }, Hot>` calls to be the discipline. But the Bits struct itself takes `const N: u16` rather than `const N: Width`, so the discipline ends at the struct boundary.

Secondary sub-finding: `MetaCarrier(pub u16)` itself wraps a bare `u16` because of a const-eval cycle. The cycle existed because `Bits<{...}, ...>::T` projected through `BitsContainerFor<9, Unsigned>` which projected through `UContainerFor<9>` which projected back to `MetaCarrier` for the carrier. With the lift to `Bits<const N: Width, ...>`, the cycle resolves.

**Substrate generalisation:** Lift `Bits<const N: u16, ...>` to `Bits<const N: Width, ...>`. `Width` already has `ConstParamTy` derive. `UContainerFor`, `IContainerFor`, `BitsContainerFor`, `UArith`, `IArith`, `UBitContainer`, `IBitContainer`, `Narrow::narrow_to::<const N>`, all switch from `const N: u16` to `const N: Width`.

**Blast radius:** Wide. Every const-generic position naming `N: u16` (six trait declarations, ~12 macro invocations, every test using `Bits<N, S>` at a literal N).

**Priority:** P0. The whole Width / MetaCarrier infrastructure exists for exactly this lift.

---

### Finding 6: Algorithm crates (graph / sparse / spectral / comb / topo) use bare `let mut i = 0usize;` loop counters

**Location:** `arvo-graph/src/topo.rs:42, 51-66, 110`, `arvo-graph/src/spanning.rs:84, 96, 131-132`, `arvo-graph/src/components.rs:29, 42`, `arvo-graph/src/rank.rs:104`, `arvo-graph/src/path.rs:45`, `arvo-graph/src/waist.rs:36`, `arvo-spectral/src/fiedler.rs:94, 106, 115, 119, 122, 134, 140, 148, 154`, `arvo-spectral/src/partition.rs:52, 110, 147, 169`, `arvo-sparse/src/block.rs:34, 47`.

**Smell:** L2/L3 algorithm crates bottom out at bare `let mut i = 0usize; while i < cap_size(N) { ...; i += 1; }` for every loop. The substrate just shipped `USize::ZERO + USize::ONE + const Add/Sub/Mul + Ord` (round 202605021200). Algorithm crates predate that round and were never migrated.

**Substrate generalisation:** Sweep `let mut i = 0usize; while i < cap_size(N)` to `let mut i = USize::ZERO; let limit = USize(cap_size(N)); while i < limit`. All increments `i += 1` become `i = i + USize::ONE`.

**Priority:** P1.

---

### Finding 7: `BitMatrix64<N>` and `BitMatrix256<N>` are parallel structs differing only in row width

**Location:** `arvo-bitmask/src/matrix.rs:33-135` (BitMatrix64), `arvo-bitmask/src/matrix.rs:142-239` (BitMatrix256). `dirty.rs:27-72`.

**Smell:** `BitMatrix64<const N: Cap>` has `rows: [Mask64; cap_size(N)]`; `BitMatrix256<const N: Cap>` has `rows: [Mask256; cap_size(N)]`. Every method body reads identically at both widths. Only the row's `Mask` width differs. Generic-blanket-impl smell.

**Substrate generalisation:** `pub struct BitMatrix<W, const N: Cap> where [(); cap_size(N)]: { rows: [Mask<W>; cap_size(N)] }` once Finding 1's generic Mask<W> lands. `pub type BitMatrix64<const N: Cap> = BitMatrix<QWord<Hot>, N>;` etc.

**Priority:** P1. Depends on Finding 1.

---

### Finding 8: `BitPrim::WIDTH: u16` and `IBitPrim::WIDTH: u16` are bare-primitive associated consts; should be `USize`

**Location:** `arvo-bits-contracts/src/lib.rs:148-202` (BitPrim), `arvo-bits-contracts/src/lib.rs:210-241` (IBitPrim).

**Smell:** `BitPrim` declares `const WIDTH: u16`. `HasBitWidth` declares `const WIDTH: USize`. Two trait surfaces for the same axis at different types. `BitPrim::count_ones(self) -> u32`, `trailing_zeros -> u32`, `leading_zeros -> u32`, `get_bit(self, idx: u32)` etc. all use bare u32. All have lint:allow tracked-256 markers.

**Substrate generalisation:** Change `BitPrim::WIDTH: u16` to `BitPrim::WIDTH: USize`. Change `count_ones`/`trailing_zeros`/`leading_zeros` return types from `u32` to `USize`.

**Priority:** P1.

---

### Finding 9: `arvo-hash` and `arvo/src/bitfield.rs` reach for hardcoded `u64::MAX` and `1u64 << ...`

**Location:** `arvo-hash/src/fnv1a.rs:94, 114`, `arvo-hash/src/xxhash3.rs:99, 120`, `arvo/src/bitfield.rs:219-256`.

**Smell:** Each callsite is the same pattern: build a mask of `N` low bits as a u64. Each hardcodes `u64::MAX` for the saturated case and `1u64 << $n` for the rest. The substrate has `BitPrim::ONE` / `Bounded::MAX` per primitive.

**Substrate generalisation:** Add `BitPrim::mask_low(n: USize) -> Self` substrate helper. Body: `if n.0 == 0 { T::ZERO } else if n.0 >= T::WIDTH.0 { <T as Bounded>::MAX } else { (T::ONE << n.0) - T::ONE }`.

**Priority:** P1. Independent.

---

### Finding 10: `Float::Ieee` declares `ZERO` / `ONE` directly; `FastFloat` and `StrictFloat` don't impl `Identity` / `Bounded`

**Location:** `arvo/src/float.rs:26-45, 53-94`.

**Smell:** `Ieee` is the float-side counterpart of `BitPrim` but declares its own `const ZERO: Self; const ONE: Self;` instead of supertrait-bounding `Identity`. `FastFloat<F>` and `StrictFloat<F>` don't blanket-impl `Identity` / `Bounded` through the inner.

**Substrate generalisation:** (a) Implement `Identity` for `f32` and `f64`. (b) Drop `Ieee::ZERO` / `Ieee::ONE`; supertrait-bound `Ieee: Identity`. (c) Blanket `impl<F: Ieee> const Identity for FastFloat<F>` and same for StrictFloat.

**Priority:** P1.

---

### Finding 11: `Mask<W>::empty()` and `Default::default()` are non-const-routed; could be `Identity::ZERO`

**Location:** `arvo-bitmask/src/mask.rs:46-50, 71-79, 92-97, 118-123`.

**Substrate generalisation:** Make `Mask<W>::empty()` route through `Identity::ZERO`.

**Priority:** P2. Cleanup once Finding 3 lands.

---

### Finding 12: `Mask<const W>` const trait from `arvo-mask-contracts` is unused

**Location:** `arvo-mask-contracts/src/lib.rs`, `arvo-bitmask/src/mask.rs`.

**Substrate generalisation:** Implement `Mask<const W: Width>` on Mask64 / Mask256.

**Priority:** P1. Depends on Finding 5.

---

### Finding 13: `LOGICAL_WIDTH` / `logical_width()` constants reach for raw `as usize` casts

**Location:** `arvo/src/ufixed.rs:75-78, 131`, `arvo/src/ifixed.rs:81-84, 132`, `arvo/src/aliases.rs:73, 87`.

**Substrate generalisation:** Implement const `Add` for `IBits + FBits -> Width` and `From<Width> for USize`. Then `LOGICAL_WIDTH: USize = USize::from(...)`.

**Priority:** P1. Depends on Finding 5.

---

### Finding 14: `arvo-comb/src/binpack.rs` uses `USize(0)` as both "first bin" and "doesn't fit" sentinel

**Location:** `arvo-comb/src/binpack.rs:13, 45, 121-122`.

**Substrate generalisation:** Change return type from `(USize, Array<USize, N>)` to `(USize, Array<Maybe<USize>, N>)`.

**Priority:** P2.

---

### Finding 15: `BitPrim` is duplicated between `arvo-strategy` and `arvo-bits-contracts`

**Location:** `arvo-strategy/src/multi_container.rs:29-64`, `arvo-bits-contracts/src/lib.rs:148-203`.

**Smell:** Two unrelated traits sharing the name `BitPrim`. The strategy version is a sealing-marker for MultiContainer halves. The bits-contracts version is the rich const-bridge with `WIDTH`, `ZERO`, `ONE`, `count_ones`, etc.

**Substrate generalisation:** Rename strategy-side `BitPrim` to `MultiContainerHalf`; let bits-contracts BitPrim be the canonical const-bridge surface (or make it a supertrait of strategy-side).

**Priority:** P1.

---

### Finding 16: `Mask::empty()` impl bound is `W: ... + Default` but `Default` is non-const

**Location:** `arvo-bitmask/src/mask.rs:42-50, 71-79`.

**Substrate generalisation:** Add `pub const fn zero() -> Self where W: [const] Identity` constructor; rewrite `empty()` body to route through Identity.

**Priority:** P2.

---

### Finding 17: `arvo/src/aliases.rs` writes `(N - 1) as u16` casts in const-generic alias body

**Location:** `arvo/src/aliases.rs:87`.

**Substrate generalisation:** Subsumed by Findings 5 + 13.

**Priority:** P1.

---

### Finding 18: `MetaCarrier` exists as workaround for a const-eval cycle that Finding 5 resolves

**Location:** `arvo-storage/src/meta_bits.rs:30-48`.

**Substrate generalisation:** Subsumed by Finding 5. Once Finding 5 lands, change `pub struct MetaCarrier(pub u16);` to `pub type MetaCarrier = Bits<{Width::W9}, Hot, Unsigned>;` or delete entirely.

**Priority:** P1.

---

## Summary of structural patterns

The 18 findings cluster into five recurring shapes:

- **Eight findings cluster around `Mask` / `BitMatrix` not generalising over `Bits`** (Findings 1, 2, 7, 9, 11, 12, 16, partial 6).
- **Five findings cluster around bare `u8` / `u16` / `usize` / `u32` const-generic positions and primitive-typed associated consts** (Findings 5, 8, 13, 17, 18).
- **Three findings cluster around parallel-trait or parallel-const surfaces for the same axis** (Findings 3, 10, 15).
- **Two findings cluster around inherent constants/methods that should route through canonical traits** (Findings 4, 11).
- **One finding around domain-level abstraction skip** (Finding 14).

Dominant fix sequence:

- **P0 (foundational, must land first):** Findings 2, 3, 4, 5.
- **P1 (cascades after P0):** Findings 1, 7, 8, 9, 10, 12, 13, 15, 17, 18.
- **P2 (cleanup):** Findings 6, 11, 14, 16.
