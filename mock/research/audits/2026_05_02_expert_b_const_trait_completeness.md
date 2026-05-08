Confirmed: smoke tests cover USize/Cap arith + Bounded/Identity on primitives, but NOT smoke tests for: Bits Identity/Bounded, Mask Bounded/Identity, BitPrim/IBitPrim, BitAccess/BitSequence/BitLogic, UFixed/IFixed Identity, FastFloat/StrictFloat ops, UArith/IArith, UContainerFor, BitsContainerFor, UWidenFrom/UNarrowFrom, Resolve, HasBitWidth.

I have enough. Let me deliver the comprehensive report now.

---

# Const-trait completeness audit: arvo

PR #42 branch: `feat/usize-const-arith`. The recent rounds (202605021200 / 202605021400 / 202605021600) lifted the highest-value surfaces: strategy markers, container projections, UArith/IArith, BitPrim, BitAccess/BitSequence/BitLogic on Bits, Bounded/Identity, USize/Cap inherent ops. arvo is now genuinely const-callable along the spine. But const-trait reach has stopped at the spine and not propagated outward to the trait declarations the consumer actually walks (Sign axis, marker traits, Widen/Narrow strategy bridges, numeric-contract impls, mask concretes, float wrappers, refit cross-domain). The audit below names every gap and the cascading work needed before PR merge.

---

### Finding 1: `Signedness` is not `pub const trait`

**Location:** `arvo-strategy/src/lib.rs:148`

**Gap:** `pub trait Signedness: sealed::Sealed + Copy + Clone + Default + 'static {}` is a sealed marker carried as a const-generic bound on `Bits<N, S, Sign>`. The trait itself has no methods so the body-purity test is trivially satisfied. It blocks `[const]` on every downstream `Sign:` bound (BitsContainerFor, Bits inherent fns, every blanket impl reaching for `Sign`).

**Concrete change:** `pub const trait Signedness: sealed::Sealed + Copy + Clone + Default + 'static {}`. Update the two `impl Signedness for Unsigned` / `for Signed` to `impl const Signedness for ...`.

**Cascade:** Every `Sign: Signedness` bound in arvo-storage/src/bits.rs (lines 51, 68, 74, 94, 106, 119, 130, 144), arvo-strategy/src/container.rs (lines 71, 83, 90), arvo-bits-contracts/src/widen.rs:23, /cross_domain.rs:22-122, and arvo-bits/src/lib.rs:34-67 should gain `[const]` once the trait is `pub const trait`. Without `[const]` propagation, `BitsContainerFor` remains stuck non-const (Finding 2).

**Bridge required?** No: straight lift.

**Priority:** P0. Foundational marker; blocks BitsContainerFor.

---

### Finding 2: `BitsContainerFor<N, Sign>` is not const trait

**Location:** `arvo-strategy/src/container.rs:71-95`

**Gap:** The trait carries an associated type `T` (no method bodies), and its two blanket impls dispatch through `S: UContainerFor<N>` / `IContainerFor<N>` (both already `pub const trait`). Body purity is trivial. Yet `pub trait BitsContainerFor<...>` is not const, so every downstream `S: BitsContainerFor<N, Sign>` bound on Bits, on the Bounded/Identity blankets, on cross-domain Narrow/Widen, cannot satisfy a `[const]` constraint. This is the blocker that makes the Bits-with-Signed half of arvo non-const-callable.

**Concrete change:** `pub const trait BitsContainerFor<const N: u16, Sign: Signedness>: Strategy { type T: Copy + ... ; }`. Both blanket impls (lines 83-95) → `impl<...> const BitsContainerFor<...>` with `S: [const] UContainerFor<N>` / `[const] IContainerFor<N>`.

**Cascade:** every consumer of `BitsContainerFor` gains a `[const]` projection. In particular: arvo-storage/src/bits.rs `from_raw`/`to_raw` already const fn; the `Deref` / `AsRef` impls (Finding 6); the unsafe `Transparent` impl on Bits already const. arvo-bits-contracts/src/cross_domain.rs:22-122 `impl<...> const Narrow / Widen` blocks rely implicitly on this. Currently those compile because the bound is checked under the looser non-const path; tightening to `[const]` is required for Sign-aware const propagation.

**Bridge required?** No.

**Priority:** P0. Foundational; blocks Bits-with-Sign const surface.

---

### Finding 3: `BitAccess` / `BitSequence` blanket impls only cover `Sign = Unsigned` (Bits with Signed is uncovered)

**Location:** `arvo-bits-contracts/src/bits_impl.rs:14-94`

**Gap:** `impl<const N, S> const HasBitWidth for Bits<N, S>` (line 14), `BitAccess` (line 21), `BitSequence` (line 44), and `BitLogic` (line 73) all bind on `Bits<N, S>`: i.e. `Bits<N, S, Unsigned>` (the default). `IFixed<I, F, S>` wraps `Bits<{1+I+F}, S, Signed>`, so Signed-axis Bits has no `BitAccess` / `BitSequence` / `HasBitWidth` impl whatsoever. Any const-context bit op on an `IFixed` value or an `Bits<N, S, Signed>` value compiles only because consumers don't yet exercise it in const fn bodies; the moment they do, `<Bits<_, _, Signed> as BitAccess>::bit(...)` fails to resolve. Round 305 lifted these to const but missed the Sign-axis split.

**Concrete change:** Generalise the blanket signature to `impl<const N: u16, S: Strategy, Sign: Signedness> const HasBitWidth for Bits<N, S, Sign> where S: [const] BitsContainerFor<N, Sign>, <S as BitsContainerFor<N, Sign>>::T: [const] BitPrim` (and analogous for `BitAccess` / `BitSequence`). For `BitLogic` over Hot, generalise similarly. Route the body through `BitsContainerFor::T` rather than `UContainerFor::T`. The `BitPrim` bridge as it stands binds only to unsigned primitives; signed primitives need the symmetric `IBitPrim`-shaped routing (Finding 5).

**Cascade:** Re-route every `<S as UContainerFor<N>>::T:` bound to `<S as BitsContainerFor<N, Sign>>::T:`. Consumers like arvo-bitmask `Mask<W>` indirectly depend on `BitAccess + BitSequence` on the word; if word is `Bits<64, Hot, Signed>` it now resolves.

**Bridge required?** Yes: needs `IBitPrim::is_zero` / signed Whole-word logic surface (Finding 5) plus a unified `BitPrim`-or-`IBitPrim` bound expressible in one impl block.

**Priority:** P0. Without this, half of arvo (everything signed) is permanently non-const-callable on bit ops.

---

### Finding 4: `MultiContainer<HiT, LoT>::new` is the only const fn: every other op is missing

**Location:** `arvo-strategy/src/multi_container.rs:78-89`

**Gap:** `MultiContainer<HiT, LoT>` ships with `new(hi, lo)` const fn, but no const-trait impls of `BitPrim`, no `Bounded`, no `Identity`, no widen/narrow surface. The N=129..=255 Bits cells dispatch to `MultiContainer<u64, u128>` and `MultiContainer<u128, u128>` (container.rs lines 162-186, 219-242, 311-334, 364-387), so a consumer who writes `let b: Bits<150, Hot, Unsigned> = Bits::from_raw(MultiContainer::new(0, 0));` compiles but cannot reach any bit op on it const- or runtime-wise. The whole-bit-op surface for 129..=255 is silently absent.

**Concrete change:** Either (a) add `impl<HiT: BitPrim, LoT: BitPrim> const BitPrim for MultiContainer<HiT, LoT>` with multi-precision implementations of count_ones/trailing_zeros/leading_zeros/get_bit/with_bit_*/bitor/bitand/bitnot/bitxor/clear_lowest_set_bit/is_zero (each unrolled across `hi` and `lo`), plus `impl const Bounded` and `impl const Identity`. Or (b) split the bit-prim surface into a separate `MultiBitPrim` trait that the multi-half code provides, and let `BitsContainerFor` dispatch carrier-trait selection between `BitPrim` (single-half) and `MultiBitPrim` (multi-half). Option (a) is the simpler path and lets every existing `BitAccess` / `BitSequence` blanket impl flow through unchanged.

**Cascade:** The 129..=255 Bits cells gain `BitAccess` / `BitSequence` / `BitLogic` const-callable. `Bounded::MIN`/`MAX` and `Identity::ZERO`/`ONE` flow through to Bits<150, ..> etc.

**Bridge required?** No: direct impl on the local type.

**Priority:** P0. The 129..=255 band is a documented public surface ("primitive-wide-domain Cold-storage workloads") with currently zero callable bit-op surface.

---

### Finding 5: `IBitPrim` is missing the `is_zero` / `bitor` / `bitand` / `bitnot` / `bitxor` / `clear_lowest_set_bit` symmetry that `BitPrim` carries

**Location:** `arvo-bits-contracts/src/lib.rs:210-241`

**Gap:** `IBitPrim` has `count_ones` / `trailing_zeros` / `leading_zeros` / `get_bit` / `with_bit_set` / `with_bit_cleared` / `with_bit_toggled`. `BitPrim` has all of these PLUS `bitor` / `bitand` / `bitnot` / `bitxor` / `clear_lowest_set_bit` / `is_zero`. The asymmetry means signed Bits cannot route `BitLogic` through a unified blanket (Finding 3) and cannot reach the const-equality bridge that enables `Bool(self == 0)` on signed paths.

**Concrete change:** Extend `pub const trait IBitPrim` with the six missing methods, mirroring the `BitPrim` impl bodies but routing through the `$uty` reinterpretation already used for shifts (lines 380-410). `is_zero` body: `(self as $uty) == 0`. `bitor`/`bitand`/`bitnot`/`bitxor` on signed: `((self as $uty) <op> (other as $uty)) as $ity`. `clear_lowest_set_bit`: `((self as $uty) & ((self as $uty).wrapping_sub(1))) as $ity`. Add to the `impl_bit_prim_i!` macro at lines 355-411.

**Cascade:** Finding 3 unblocked once IBitPrim has the same surface as BitPrim. The signed `BitLogic for Bits<N, Hot, Signed>` impl can land. Consumer code in algorithm crates (arvo-graph, arvo-sparse) calling `is_zero` on signed weights becomes const-callable.

**Bridge required?** No: extension of an existing const trait.

**Priority:** P0. Blocks Finding 3.

---

### Finding 6: `Deref` / `AsRef` impls on Bits, USize, Bool, IBits, FBits, Width, MetaCarrier are not const

**Location:** `arvo-storage/src/bits.rs:119-139` (Bits Deref/AsRef), `arvo-storage/src/platform.rs:37-43, 233-239` (USize, Bool Deref), `arvo-storage/src/meta_bits.rs:135-144` (meta-bits wrapper Deref/AsRef)

**Gap:** Every `Deref::deref` / `AsRef::as_ref` body is a one-line `&self.0` / `&self.0.0`. Pure. None is `impl const`. `core::ops::Deref` and `core::convert::AsRef` are NOT const-stable in stdlib, so direct `impl const Deref` is rustc-blocked. arvo must ship its own bridge.

**Concrete change:** Define `pub const trait ConstDeref { type Target; fn const_deref(&self) -> &Self::Target; }` and `pub const trait ConstAsRef<T> { fn const_as_ref(&self) -> &T; }` in `arvo-transparent` (the L0 root). Keep the non-const `Deref` / `AsRef` impls for stdlib trait coverage; ADD parallel `impl const ConstDeref for ...` / `impl const ConstAsRef<...> for ...` impls on every wrapper. Consumers that need const-context dereferencing route through the bridge.

**Cascade:** Bits, USize, Bool, IBits, FBits, Width, MetaCarrier, BitMatrix64/256, NodeId, Mask64/Mask256, UFixed/IFixed (which currently have no Deref but should via the same bridge to `Bits`).

**Bridge required?** Yes: `ConstDeref` / `ConstAsRef`. Place: `arvo-transparent` (no orphan issues since Deref/AsRef are stdlib).

**Priority:** P1. Cascades through every wrapper; not foundational but pervasive.

---

### Finding 7: `Default::default` impls on Bits/USize/Bool/Mask/Mask256/UFixed/IFixed/BitMatrix64/256/NodeId are not const

**Location:** Multiple. `arvo-bitmask/src/mask.rs:71-79, 118-123` (Mask Default, Mask256 Default), `arvo-bitmask/src/matrix.rs:127-138, 231-242` (BitMatrix Default), `arvo/src/ufixed.rs:115-123`, `arvo/src/ifixed.rs:116-124`, `arvo/src/bitfield.rs:159` (derived). Bits is `#[derive(Default)]` (arvo-storage/src/bits.rs:49). The derived/manual impls all have pure bodies.

**Gap:** `core::default::Default::default` is not const-stable in stdlib. arvo needs a `ConstDefault` bridge. Currently `Mask::empty()` (mask.rs:48) is not even const fn because its body calls non-const `W::default()`.

**Concrete change:** Define `pub const trait ConstDefault { fn const_default() -> Self; }` in `arvo-strategy` (sibling to Bounded/Identity). Implement on every primitive wrapper; the body for ZSTs / numeric types is `Self::ZERO` (via Identity) or hand-coded `Self::from_raw(0)`. Convert `Mask::empty`, `Mask256::empty`, `BitMatrix64::empty`, `BitMatrix256::empty`, `UFixed::default`, `IFixed::default` to delegate through `<Self as ConstDefault>::const_default()`. Lift the inherent `empty()` fns to `pub const fn`.

**Cascade:** Every `BitMatrix::empty` / `Mask::empty` / `Mask256::empty` / wrapper `default` body becomes const-callable. The `#[derive(Default)]` on Bits stays for stdlib coverage but pairs with a hand-written `impl const ConstDefault for Bits<...>` body that routes through Identity::ZERO.

**Bridge required?** Yes: `ConstDefault`. Place: `arvo-strategy` (it sits next to Bounded/Identity which are conceptually identical kinds of typed-const surface).

**Priority:** P1.

---

### Finding 8: `From` / `Into` / `TryFrom` impls across arvo are not const

**Location:** `arvo-storage/src/platform.rs:279-291` (`From<bool> for Bool`, `From<Bool> for bool`), `arvo-storage/src/meta_bits.rs:146-164` (`From<u8>`/`From<u16>`/`From<$W>` for u8/u16 in `meta_bits_wrapper!`), `arvo/src/ufixed.rs:232-295` (UFixed strategy conversions), `arvo/src/ifixed.rs:214-275` (IFixed strategy conversions)

**Gap:** `core::convert::From` / `Into` / `TryFrom` are not const-stable in stdlib. Every `From::from` / `TryFrom::try_from` body in arvo is a one-line constructor (e.g. `Bool(b)`, `Self::from_raw(...)`). Pure. Currently rustc-blocked from `impl const From`.

**Concrete change:** Define `pub const trait ConstFrom<T> { fn const_from(t: T) -> Self; }` and `pub const trait ConstTryFrom<T, E> { fn const_try_from(t: T) -> Outcome<Self, E>; }` in `arvo-strategy` (since they live above `arvo-storage`'s wrapper layer). Mirror every existing `From`/`TryFrom` impl with `impl const ConstFrom for ...`. Keep stdlib From for boundary compatibility; add the const counterpart for in-arvo const-context consumption.

**Cascade:** UFixed↔UFixed strategy widening conversions (Hot→Warm, Hot→Precise, Warm→Precise) become const-callable. Same for IFixed. `Bool` ↔ `bool`. The meta-bits wrappers' `From<u8>` / `From<u16>` / `From<$W> for u16` / `for u8`. The `TryFrom` narrowing edges in UFixed/IFixed become const via `Outcome` (already const-friendly via notko).

**Bridge required?** Yes: `ConstFrom<T>`, `ConstTryFrom<T, E>`. Place: `arvo-strategy` or `arvo-transparent` depending on the layering need.

**Priority:** P1.

---

### Finding 9: `PartialEq::eq` / `Ord::cmp` / `PartialOrd::partial_cmp` impls are not const

**Location:** `arvo-storage/src/platform.rs:133-145, 203-214` (USize, Cap PartialOrd/Ord), `arvo/src/ufixed.rs:100-108`, `arvo/src/ifixed.rs:101-109`, `arvo/src/bitfield.rs:170-175` (derived bitfield PartialEq), `arvo/src/traits.rs:34-80` (TotalOrd impls call `cmp` / `partial_cmp`).

**Gap:** `core::cmp::PartialEq::eq`, `Ord::cmp`, `PartialOrd::partial_cmp` are non-const-stable. Bodies pure. `BitPrim::is_zero` (Finding 5 sibling) is the arvo's pattern: a const-trait method with body `self == 0` (const-stable on bare primitives only). To extend the pattern to compound equality between two arvo wrappers, bridge traits are needed. Currently `UFixed::eq` (ufixed.rs:104) cannot be called from const fn even though its body is `self.to_raw() == other.to_raw()` (const-stable on the primitive `==`).

**Concrete change:** Define `pub const trait ConstEq { fn const_eq(&self, other: &Self) -> Bool; fn const_ne(&self, other: &Self) -> Bool { Bool(!self.const_eq(other).0) } }` and `pub const trait ConstOrd: ConstEq { fn const_cmp(&self, other: &Self) -> Ordering; fn const_lt(&self, other: &Self) -> Bool; fn const_le(&self, other: &Self) -> Bool; fn const_gt(&self, other: &Self) -> Bool; fn const_ge(&self, other: &Self) -> Bool; }` in `arvo-strategy`. (Ordering itself is `core::cmp::Ordering` which already has const constructor literals in nightly; if not, ship `pub enum ConstOrdering { Less, Equal, Greater }`.) Implement on USize, Cap, Bool, Bits, UFixed, IFixed, FastFloat, StrictFloat, Mask64, Mask256, NodeId, MetaCarrier, IBits, FBits, Width.

**Cascade:** TotalOrd impls (traits.rs:34-80) become `impl const TotalOrd for ...`. UArith/IArith div-by-zero check (`b == ZERO`) becomes generic-callable through `<T as ConstEq>::const_eq` rather than the per-primitive `BitPrim::is_zero` (which it currently uses, but only because the body sees the concrete primitive type: generic bodies need the trait dispatch). Sorts and predicates become const.

**Bridge required?** Yes: `ConstEq`, `ConstOrd`. Place: `arvo-strategy`.

**Priority:** P0 for ConstEq (UArith body composition), P1 for ConstOrd.

---

### Finding 10: `UWidenFrom` / `UNarrowFrom` / `IWidenFrom` / `INarrowFrom` strategy bridges are not const trait

**Location:** `arvo-strategy/src/widen.rs:27-57` (trait declarations), `arvo-strategy/src/widen.rs:64-119, 122-286` (impls)

**Gap:** All four traits are `pub trait`. All impls are `impl ... for ...`. Bodies are `v as $dst_ty` (cast: const-stable) and bounded comparison branches with `let max_u128 = (1u128 << $bits) - 1; if (v as u128) > max_u128 { Outcome::Err(()) } else { Outcome::Ok(v as $dst_ty) }`: entirely const-stable. The non-const surface blocks `From<UFixed<I, F, Hot>> for UFixed<I, F, Warm>` from being const (Finding 8 cascades).

**Concrete change:** `pub const trait UWidenFrom<Src: [const] UContainerFor<N> + HasAxes, const N: u16>: [const] UContainerFor<N> + HasAxes { fn u_widen(v: Src::T) -> Self::T; }`. Same for `IWidenFrom` / `UNarrowFrom` / `INarrowFrom`. Add `impl const` to all macro-emitted impls (4 macros: `impl_u_widen!`, `impl_i_widen!`, `impl_u_narrow!`, `impl_i_narrow!`). The `Outcome::Ok` / `Outcome::Err` constructor is already const (from notko).

**Cascade:** Strategy conversion `From` / `TryFrom` impls in ufixed.rs:232-295, ifixed.rs:214-275 can become `impl const ConstFrom` / `impl const ConstTryFrom` (Finding 8). Cross-Bits widen/narrow (cross_domain.rs already partially const) gets a fully const-callable upstream chain.

**Bridge required?** No: straight lift; `HasAxes` (axes.rs:159-190) is a non-const trait but has only associated types, so `[const]` propagation is unnecessary for it. (See Finding 11 for HasAxes.)

**Priority:** P0. Strategy widening is a hot consumer surface for const-eval composition.

---

### Finding 11: `HasAxes` / `OverflowPolicy` / `ContainerWidth` / `StorageLayout` are not const trait

**Location:** `arvo-strategy/src/axes.rs:41-44, 77-80, 114-117, 159-166`

**Gap:** Four sibling marker traits, each carrying associated types and const discriminants. Bodies trivial. None const. `HasAxes` is a supertrait bound on `UArith` / `IArith` / `UWidenFrom` etc., so any `[const]` propagation through those bounds halts at HasAxes.

**Concrete change:** Lift all four to `pub const trait`. `pub const trait OverflowPolicy: sealed::Sealed + Copy + Clone + Default + 'static { const DISCRIMINANT: u16; }`. Same for `ContainerWidth`, `StorageLayout`. `pub const trait HasAxes { type Overflow: [const] OverflowPolicy; type Width: [const] ContainerWidth; type Layout: [const] StorageLayout; }`. The four marker-impl blocks (Wrapping/Saturating, Min/DoubleLogical, Dense/Bitpacked) lift to `impl const`. The four strategy `impl HasAxes for Hot/Warm/Cold/Precise` (lines 168-190) lift to `impl const HasAxes for ...`.

**Cascade:** UArith/IArith/USaturating/ISaturating bounds, UWidenFrom/INarrowFrom bounds, every `Self: HasAxes` projection in trait declarations.

**Bridge required?** No.

**Priority:** P1. Cascades but doesn't itself block other findings.

---

### Finding 12: `Resolve<Other>` is `pub const trait` but only same-pair impls; cross-strategy resolution ships only same-pair `CrossStrategyOp` implementations

**Location:** `arvo-strategy/src/cross_strategy.rs:49-54`

**Gap:** `pub trait CrossStrategyOp<S1, S2> {}` is NOT `pub const trait`. (The header at line 49 lacks the const keyword.) Resolve<S1, S2> is declared const at lib.rs:175 but CrossStrategyOp at cross_strategy.rs:49 is plain. Marker trait, no body, but blocks `[const]` on cross-strategy diagnostic-bound positions.

**Concrete change:** `pub const trait CrossStrategyOp<S1: [const] Strategy, S2: [const] Strategy> {}`. Lift each `impl CrossStrategyOp<...> for () {}` (lines 51-54) to `impl const CrossStrategyOp<...> for ()`.

**Cascade:** Future cross-strategy op-site bound positions become `[const] CrossStrategyOp<S1, S2>`-callable.

**Bridge required?** No.

**Priority:** P2.

---

### Finding 13: Numeric-contract trait IMPLS are not `impl const` (despite trait declarations being `pub const trait`)

**Location:** `arvo/src/traits.rs:34-664` (Sqrt, Recip, Abs, FromConstant, TotalOrd impls), `arvo-numeric-contracts/src/lib.rs:28-116` (declarations)

**Gap:** Every numeric-contract trait (`Abs`, `Recip`, `Sqrt`, `TotalOrd`, `FromConstant`, `Predicate`, `IsZero`, `IsPositive`, `IsNonZero`, `IsNonNegative`, `IsZeroOrPositive`) IS declared `pub const trait` in arvo-numeric-contracts. But every `impl Sqrt`, `impl Abs`, `impl FromConstant`, `impl TotalOrd`, `impl Recip` in `arvo/src/traits.rs` (and its predicate.rs sibling, if predicate is impl'd anywhere) is plain `impl X for Y`, NOT `impl const X for Y`. This is a massive surface: hundreds of impls (every (strategy, I, F, container) cell in the four impl_sqrt/abs/from_constant/from_constant_fractional macros plus the four float impls per type). Bodies are pure (`Self::from_raw(self.to_raw().isqrt())`, `Self::from_raw(<$ctype>::wrapping_abs(...))`, sign-bit-clear bit twiddles). The trait-declaration was promoted const but the impls were left behind.

**Concrete change:** Add `const` to every `impl X for Y` block in arvo/src/traits.rs lines 34, 44, 54, 61, 68, 75, plus every macro-generated impl (lines 90-104, 264-318, 438-462, 549-560 and their callers at 105-122, 174-204, 208-238, 242-252, 267-389 (i32/i64/i128 abs cells), 416-430, 466-664). Cascade: `f32::isqrt` is not const-stable and is not used here (the float sqrt path uses `f32::from_bits` / `f32::to_bits` via `sqrt_f32`/`sqrt_f64` free functions, which need to become `pub const fn`). `u*::isqrt` IS const-stable on nightly per `feature(const_int_sqrt)`: verify and gate accordingly. `wrapping_abs` / `saturating_abs` on `i8..i128` are const-stable. Once free functions and macro-generated impls are const, the trait surface becomes consumer-callable in const fn.

**Cascade:** Every algorithm crate (arvo-graph/sparse/comb/spectral) that bounds on `T: Sqrt`/`Abs`/`FromConstant`/`TotalOrd` becomes const-callable through those bounds (with `[const]` annotations on the trait positions). Consumers use `T::from_constant::<{USize(1)}>()` in const fn bodies.

**Bridge required?** No: straight lift on every impl block. `sqrt_f32` / `sqrt_f64` / `abs_f32` / `abs_f64` (lines 139-172, 391-398) lift to `pub const fn`.

**Priority:** P0. The numeric-contract trait family is the primary consumer-facing arithmetic abstraction; leaving its impls non-const while declarations are const is the most visible inconsistency in the substrate.

---

### Finding 14: `Predicate` / `IsZero` / `IsPositive` / `IsNonZero` / `IsNonNegative` / `IsZeroOrPositive` have NO impls anywhere

**Location:** `arvo-numeric-contracts/src/lib.rs:91-116` (declarations), search across all arvo crates

**Gap:** The Predicate family is declared (`pub const trait Predicate`, with 5 named-predicate const supertraits) but never impl'd. No `impl const Predicate for UFixed<...>` / `IFixed<...>` / `Bits<...>`. The const-trait-completeness audit caught this: a const-trait surface that ships zero impls is dead substrate that consumers cannot reach.

**Concrete change:** Add per-(I,F,S) `impl const Predicate for UFixed<I, F, S>` / `IFixed<I, F, S>` / `Bits<N, S, Sign>` blanket impls. `Predicate::test(self)` on UFixed checks the value against context-specific zero (`<Self as Identity>::ZERO`); requires `ConstEq` (Finding 9) once that bridge is in. The five named-predicate impls (`IsZero`, etc.) are sub-trait impls that delegate to `Predicate::test` plus an additional sign / nonzero check.

**Cascade:** Algorithm crates can finally use `T: Predicate` bounds (e.g. arvo-graph topological sort guarding zero edge weight, arvo-spectral power iteration guarding zero residual).

**Bridge required?** Yes: depends on `ConstEq` (Finding 9) plus `Identity` (already const).

**Priority:** P1. Stranded surface that needs to be reached.

---

### Finding 15: `Ieee` is `pub trait`, not `pub const trait`; `FastFloat` / `StrictFloat` arithmetic impls are const but `Ieee::WIDTH` / `ZERO` / `ONE` projection is uncallable in const generic bounds

**Location:** `arvo/src/float.rs:26-45`

**Gap:** `pub trait Ieee: ...` carries `const WIDTH: u16; const ZERO: Self; const ONE: Self;`. Trait body has no methods so `pub const trait Ieee` is trivially body-pure. `impl Ieee for f32` / `impl Ieee for f64` are plain `impl`. `FastFloat<F>` / `StrictFloat<F>` core::ops impls (lines 102-136) ARE `impl const`, BUT they bound `F: Ieee + [const] core::ops::$op<Output = F>`. `[const] Ieee` cannot be expressed because `Ieee` itself isn't const. The current macro spell happens to compile because the actual Ieee bound is non-const: but the `[const] core::ops::$op` half forces F to be const-arithmetic-callable, which couples to a non-const trait sibling.

**Concrete change:** `pub const trait Ieee: sealed::Sealed + Copy + Default + ... + 'static { const WIDTH: u16; const ZERO: Self; const ONE: Self; }`. Lift `impl Ieee for f32` / `for f64` to `impl const Ieee for ...`. The `FastFloat` / `StrictFloat` core::ops impl macro `float_binop_impl!` updates `F: Ieee` to `F: [const] Ieee`. Same for `float_neg_impl!`. The `FromU8Ieee` (traits.rs:647-663) sibling trait should likewise be lifted to `pub const trait FromU8Ieee` and `impl const FromU8Ieee for f32` / `for f64`.

**Cascade:** `FastFloat::new` / `StrictFloat::new` / `into_inner` (already const fn). `Float<F>` cfg-resolved alias becomes fully const-arithmetic. Float TotalOrd/Sqrt/Recip/Abs (Finding 13) gain const-callable IEEE projections.

**Bridge required?** No: Ieee is a arvo-owned trait.

**Priority:** P1.

---

### Finding 16: Marker traits `IntegerLike` / `FractionLike` / `BitPresentation` / `FloatLike` / `BoolLike` are not `pub const trait`

**Location:** `arvo/src/markers.rs:23, 28, 36, 49, 56`

**Gap:** Each is a `pub trait` (no `const`). `BoolLike` carries a method `pack(self) -> Self::Packed`. `BitPresentation` carries `const LOGICAL_WIDTH: USize`. The `pack` body (line 86-92) is pure: `UFixed::<...>::from_raw(self.0 as u8)`. None const.

**Concrete change:** `pub const trait IntegerLike: Copy {}`, same for FractionLike, FloatLike. `pub const trait BitPresentation: Copy { const LOGICAL_WIDTH: USize; }`. `pub const trait BoolLike: Copy { type Packed: [const] BitPresentation; fn pack(self) -> Self::Packed; }`. Lift the BoolLike-for-Bool impl (line 82) to `impl const BoolLike for Bool`.

**Cascade:** Consumers bounding on `T: IntegerLike` / `T: BoolLike` in const contexts can compose. UFixed/IFixed BitPresentation impls (ufixed.rs:127, ifixed.rs:128) lift to `impl const BitPresentation`. Float impls (`impl FloatLike for FastFloat<F>` / `for StrictFloat<F>` at float.rs:93, 94) lift to `impl const FloatLike`.

**Bridge required?** No.

**Priority:** P1.

---

### Finding 17: `arvo_mask_contracts::Mask<const W>` is `pub const trait` but has NO impls (Mask64 / Mask256 don't implement it)

**Location:** `arvo-mask-contracts/src/lib.rs:30-55` (trait), `arvo-bitmask/src/mask.rs` (concretes), `arvo-bitmask/src/ops.rs` (inherent ops)

**Gap:** `pub const trait Mask<const W: u16>` declares 11 const fns. `Mask64` and `Mask256` are concrete types in `arvo-bitmask` providing the same surface as INHERENT methods (mask.rs:42, ops.rs:26-111, ops.rs:176-357). Neither implements `arvo_mask_contracts::Mask<W>`. The const-trait surface in arvo-mask-contracts is stranded; consumers can use the inherent methods on the concrete types but cannot bound generically on `T: Mask<64>` / `T: Mask<256>`. Round 305/306 lifted `BitLogic` impls but didn't implement the abstract `Mask<W>` trait.

**Concrete change:** Add `impl const arvo_mask_contracts::Mask<64> for Mask64 { fn empty() -> Self { ... } fn full() -> Self { ... } ... fn mask_for_width(n: USize) -> Self { ... } }` plus `impl const arvo_mask_contracts::Mask<256> for Mask256 { ... }`. Each method body delegates to the existing inherent method (which becomes const fn per Finding 18). `mask_for_width(n)` is a new method (not currently inherent on Mask64/Mask256); add as `pub const fn mask_for_width(n: USize) -> Self { Self::from_word(QWord::<Hot>::from_raw((1u64 << n.0) - 1)) }` plus 256 unrolling. Same for the BitMatrix family if it should also impl Mask (likely no: different shape).

**Cascade:** Consumers of arvo-mask-contracts can bound on `T: Mask<64>` and reach all 11 methods generically and const-callable. The trait is no longer dead.

**Bridge required?** No.

**Priority:** P0. Stranded const-trait surface; either delete or implement.

---

### Finding 18: Mask64 / Mask256 inherent methods are not const fn

**Location:** `arvo-bitmask/src/ops.rs:26-111` (Mask64), `arvo-bitmask/src/ops.rs:176-357` (Mask256)

**Gap:** `union`, `intersection`, `difference`, `complement`, `is_empty`, `intersects`, `contains`, `count`, `lowest_set`, `highest_set`, `iter_set_bits` (Mask64); same set for Mask256. Bodies route through `BitLogic::bitor` etc. which ARE const-trait. The `pub fn` should be `pub const fn`. Likewise `Mask::empty` / `Mask256::empty` (mask.rs:48, 95) should be `pub const fn` once Default is bridged.

**Concrete change:** Add `const` to every `pub fn` listed. `iter_set_bits` returns a struct iterator; the iterator impl `next` body cannot be const (Iterator trait not const) but the constructor can be `pub const fn`.

**Cascade:** Consumer code (graph/sparse/spectral algorithms) that calls `mask.union(other)` or `mask.count()` in const fn bodies becomes const-callable.

**Bridge required?** Depends on Finding 7 (ConstDefault) for `empty()`.

**Priority:** P1.

---

### Finding 19: BitMatrix64 / BitMatrix256 inherent methods are not const fn

**Location:** `arvo-bitmask/src/matrix.rs:41-148`, `arvo-bitmask/src/matrix.rs:151-249`

**Gap:** `empty`, `edge`, `set_edge`, `clear_edge`, `successors`, `predecessors`, `transitive_closure` are all `pub fn` not `pub const fn`. Bodies route through Mask64/Mask256 ops (Finding 18) and BitAccess methods. Once Findings 17/18 land, every body is const-callable.

**Concrete change:** Promote all to `pub const fn`. The mutable accessors (`set_edge`, `clear_edge`, `transitive_closure`) work in const fn since rust supports `&mut self` const fn since 1.83.

**Cascade:** Const construction of edge matrices in algorithm-crate code paths.

**Bridge required?** Depends on Findings 17/18.

**Priority:** P2.

---

### Finding 20: `arvo-bitmask::dirty::propagate_dirty_64` / `propagate_dirty_256` are not const fn

**Location:** `arvo-bitmask/src/dirty.rs:27, 53`

**Gap:** Pure functions. Bodies compose Mask64/Mask256 BitLogic ops and BitMatrix successors lookup. Should be `pub const fn` once Findings 17-19 land.

**Concrete change:** Add `const` to both signatures.

**Bridge required?** No.

**Priority:** P2.

---

### Finding 21: `Try` / `FromResidual` impls on Bool are not const

**Location:** `arvo-storage/src/platform.rs:241-261`

**Gap:** `Bool` impls `Try` (allowing `?` on Bool) and `FromResidual<Infallible>`. Bodies are pure constructors. `core::ops::Try` and `FromResidual` are not const-stable in stdlib.

**Concrete change:** Currently rustc-blocked. arvo already has `notko::Outcome` for the Result-shape const path. For Bool's `?` ergonomics in const contexts, define a `pub const trait ConstTry { type Output; type Residual; fn const_branch(self) -> ConstControlFlow<Self::Residual, Self::Output>; ... }` mirror in arvo-storage or notko, with `pub const enum ConstControlFlow<R, O> { Continue(O), Break(R) }`. Implement on `Bool` with const bodies.

**Cascade:** Consumers using `?` on Bool in const fn (currently impossible) gain that surface.

**Bridge required?** Yes: `ConstTry` / `ConstControlFlow`. Place: notko (or arvo-storage if scoped to arvo).

**Priority:** P2.

---

### Finding 22: `notko::Outcome::Ok` / `Err` constructors used in const widen narrow paths: verify they're const

**Location:** `arvo-strategy/src/widen.rs:96, 115`

**Gap:** `Outcome::Ok(...)` / `Outcome::Err(())` constructors are reached in const-eligible bodies but the impl macros currently emit non-const impls (Finding 10). Once Finding 10 is lifted to `impl const`, the Outcome constructors must themselves be const-callable. They likely are (notko ships Outcome as a plain enum, enum constructors are always const), but verify.

**Concrete change:** Verify `notko::Outcome::Ok` / `Err` resolves cleanly inside `impl const`. If notko's Outcome is `pub enum Outcome<T, E> { Ok(T), Err(E) }`, no work needed.

**Bridge required?** No.

**Priority:** P0: verification only, but blocks Finding 10's lift.

---

### Finding 23: `MetaCarrier::as_bits` uses `core::mem::transmute`: const-stable, but verify the `MetaCarrier` Hash / Default / PartialEq derives don't block downstream `[const]` bounds

**Location:** `arvo-storage/src/meta_bits.rs:46-69`

**Gap:** `MetaCarrier` derives `Default`, `Hash`, `Debug`, `PartialEq`, `Eq`, `Copy`, `Clone`, `ConstParamTy`. `Hash::hash` / `Debug::fmt` are non-const stdlib trait impls; this becomes a problem only when consumers reach for `[const]` projections through MetaCarrier-bounded code. With ConstDefault / ConstEq / ConstHash bridges (Findings 7, 9, plus a `ConstHash` not yet noted), the manual non-derive impls land alongside.

**Concrete change:** Implement `impl const ConstDefault for MetaCarrier { fn const_default() -> Self { Self(0) } }`, `impl const ConstEq for MetaCarrier { ... }`. Drop reliance on `#[derive(Default, PartialEq)]` for const purposes (keep them for stdlib trait coverage).

**Bridge required?** Yes: covered by Findings 7 and 9.

**Priority:** P2.

---

### Finding 24: `meta_bits_wrapper!` macro `Deref` / `AsRef` / `From` impls are non-const

**Location:** `arvo-storage/src/meta_bits.rs:135-164`

**Gap:** Each macro-emitted wrapper (IBits, FBits, Width) gets non-const Deref, AsRef, From impls. Pure bodies. Same story as Finding 6 / Finding 8.

**Concrete change:** Add parallel `impl const ConstDeref for $W` / `impl const ConstAsRef<u16> for $W` / `impl const ConstFrom<u8> for $W` / `impl const ConstFrom<u16> for $W` / `impl const ConstFrom<$W> for u16` etc. inside the macro emission. The non-const stdlib impls stay for boundary coverage.

**Bridge required?** Yes: covered by Findings 6 and 8.

**Priority:** P2.

---

### Finding 25: UFixed / IFixed `Clone` and `PartialEq` impls are not const

**Location:** `arvo/src/ufixed.rs:90-108`, `arvo/src/ifixed.rs:91-109`

**Gap:** Hand-rolled `impl Clone for UFixed<...> { fn clone(&self) -> Self { *self } }`, `impl PartialEq for UFixed<...> { fn eq(&self, other: &Self) -> bool { self.to_raw() == other.to_raw() } }`. Pure bodies. `*self` is const-stable for Copy types, `self.to_raw() == other.to_raw()` is const-stable on bare primitives. Both blocked by stdlib non-const Clone/PartialEq.

**Concrete change:** Add `impl const ConstEq for UFixed<I, F, S>` / `impl const ConstEq for IFixed<I, F, S>` (Finding 9 cascade). For `Clone`, the substrate already has `Copy` available (line 85, 86); `ConstClone` is rarely useful since Copy already gives const-callable copying.

**Cascade:** ConstEq (Finding 9).

**Priority:** P1.

---

### Finding 26: Mask64 const-context smoke test is missing

**Location:** No `arvo-bitmask/tests/mask64_const_arith.rs` or similar

**Gap:** `usize_const_arith.rs` covers USize/Cap and primitives. There is no const-context smoke test demonstrating that `Mask::EMPTY` / `Mask::FULL` / `BitOr` / `BitAnd` / `BitXor` / `Not` are callable from `const _: Mask64 = Mask64::EMPTY | Mask64::FULL;` etc. Mask64 BitAnd/BitOr/BitXor/Not impls landed const in round 202605021600 but have no const-eval verification.

**Concrete change:** Add `arvo-bitmask/tests/mask64_const_arith.rs` with `const _: Mask64 = <Mask64 as Bounded>::MIN; const _: Mask64 = <Mask64 as Identity>::ZERO; const _: Mask64 = Mask64::from_word(QWord::<Hot>::from_raw(0)); const _: Mask64 = !Mask64::from_word(...) ; const _: Mask64 = Mask64::from_word(...) | Mask64::from_word(...);` etc. Same for Mask256. Add test fn that asserts the const values match runtime values.

**Bridge required?** No.

**Priority:** P2.

---

### Finding 27: Bits Identity / Bounded const-context smoke test is missing

**Location:** `arvo-bits/tests/` has `bits.rs`, `bit_access.rs`, `bit_logic.rs`, `bit_sequence.rs`, `bit_width.rs`, `bitfield.rs`, `aliases.rs`: none of which include const-eval smoke

**Gap:** Round 305 lifted Bits BitAccess/BitSequence/BitLogic blanket impls to `impl const`, and round 306 added Identity blanket on Bits. No `const _: Bits<8, Hot> = <Bits<8, Hot> as Identity>::ZERO;` smoke. The const-trait surface is unverified.

**Concrete change:** Add `arvo-bits/tests/bits_const_arith.rs` exercising: Bits Identity / Bounded const access; const-context BitAccess::bit / with_bit_set; const-context BitSequence::is_zero / count_ones; const-context BitLogic::bitor / bitand / bitnot / bitxor / clear_lowest_set_bit. Both Sign=Unsigned (current default) and (post-Finding 3) Sign=Signed.

**Bridge required?** No.

**Priority:** P1.

---

### Finding 28: UFixed / IFixed const-arith composition smoke test is missing

**Location:** No `arvo/tests/ufixed_const_arith.rs` (only `ufixed_ops.rs` etc., presumably runtime).

**Gap:** UFixed/IFixed const Add/Sub/Mul/Div impls landed in round 202605021400. UFixed/IFixed `Identity::ZERO` / `ONE` blankets landed in round 202605021600. No smoke test exercising `const _: UFixed<{ ibits(8) }, { fbits(0) }, Hot> = UFixed::ZERO + UFixed::ONE;` or `const _: IFixed<...> = IFixed::ZERO * IFixed::ONE;`. arvo's most consumer-visible surface has zero const-context proof.

**Concrete change:** `arvo/tests/ufixed_const_arith.rs` and `ifixed_const_arith.rs` exercising:
1. UFixed Identity ZERO/ONE per strategy.
2. UFixed Bounded MIN/MAX per strategy.
3. UFixed const Add/Sub/Mul/Div across all four strategies at representative bit widths.
4. UFixed strategy conversion `From` / `TryFrom` const-callable (post Finding 8 / 10).
5. UFixed numeric-contracts (Sqrt, Abs, FromConstant, TotalOrd) const-callable (post Finding 13).
6. Same for IFixed.

**Bridge required?** No.

**Priority:** P1.

---

### Finding 29: FastFloat / StrictFloat const-arith smoke test is missing

**Location:** No `arvo/tests/float_const_arith.rs`. `float_ops.rs` is presumably runtime.

**Gap:** FastFloat / StrictFloat core::ops impls are `impl const` but unverified in const context.

**Concrete change:** Add smoke test exercising `const _: FastFloat<f32> = FastFloat::new(1.0) + FastFloat::new(2.0);` etc. `f32::from_bits` / `to_bits` are const-stable; `+` on f32 in const fn requires checking nightly status. If currently const-blocked at the f32 level, gate the test or document the boundary.

**Bridge required?** No.

**Priority:** P2.

---

### Finding 30: BitPrim / IBitPrim const-context smoke test is missing

**Location:** No tests directly under arvo-bits-contracts/tests/.

**Gap:** BitPrim was lifted to `pub const trait` and `is_zero` const bridge added. No `const _: bool = <u8 as BitPrim>::is_zero(0); const _: u32 = <u64 as BitPrim>::count_ones(0xFF);` etc.

**Concrete change:** Add `arvo-bits-contracts/tests/bit_prim_const.rs`. Cover all const bridge methods on u8/u16/u32/u64/u128 and i8/i16/i32/i64/i128.

**Priority:** P2.

---

### Finding 31: `width_le_64` and other const-fn predicates take bare types as input

**Location:** `arvo/src/strategy.rs:53-65`

**Gap:** `pub const fn width_le_64(n: arvo_storage::Width) -> bool`: returns bare bool. `pub const fn is_fractional(f: FBits) -> usize`: returns bare usize. Both have lint:allow on tracking #256. The bare-bool return signature is stuck because const-generic where-clause guards demand bare bool, but this leaks the bare primitive into the consumer's surface. `Bool` should be the typed return; until const-generic guards accept arvo wrappers, this stays.

**Concrete change:** Document the constraint via a stronger `#[diagnostic::on_unimplemented]` hint on the const-eval guards (Fnv1a, etc.) so consumers know why bare bool appears here, and confirm the lint:allow is genuinely tracked. Optionally ship a `Bool::as_bool_const(self) -> bool` const-stable bridge so consumer code can stay typed and unwrap only at the where-clause boundary.

**Priority:** P2: doc + bridge surface.

---

### Finding 32: `Mask::full()` / `Mask::mask_for_width(n)` (substrate-mask-contracts surface) have no implementation, so consumers cannot reach them generically

**Location:** `arvo-mask-contracts/src/lib.rs:34, 54`

**Gap:** `Mask` declares `fn full()` and `fn mask_for_width(n: USize)`. Mask64 / Mask256 inherent surface has neither; only `empty()`. There are MIN/MAX via Bounded (which can substitute for full = MAX), but `mask_for_width(n)` has no realization. Consumers narrowing Bits to N bits via `Narrow<T>` rely on a mask path; the trait declares it but no impl exists.

**Concrete change:** Add inherent `pub const fn full() -> Self` on Mask64/Mask256 (delegating to `<Self as Bounded>::MAX`). Add `pub const fn mask_for_width(n: USize) -> Self` with body `if n.0 >= 64 { Self::FULL } else { Self::from_word(QWord::<Hot>::from_raw((1u64 << n.0) - 1)) }` for Mask64; Mask256 unrolls per-word. Then implement `arvo_mask_contracts::Mask<W>` per Finding 17, delegating each method to the inherent surface.

**Bridge required?** No.

**Priority:** P1.

---

### Finding 33: `arvo_storage::AsBool::as_bool` is non-const trait method

**Location:** `arvo-storage/src/platform.rs:267-277`

**Gap:** `pub trait AsBool` not const. `impl AsBool for Bool` not const. Body trivial.

**Concrete change:** `pub const trait AsBool { fn as_bool(&self) -> bool; }`, `impl const AsBool for Bool { fn as_bool(&self) -> bool { self.0 } }`. The bare-bool return matches a documented escape-hatch position.

**Bridge required?** No.

**Priority:** P1.

---

### Finding 34: `Bool::pack` (BoolLike impl) is non-const

**Location:** `arvo/src/markers.rs:82-93`

**Gap:** `BoolLike::pack` body: `UFixed::<...>::from_raw(self.0 as u8)`. Pure. Currently non-const because `BoolLike` itself isn't const trait (Finding 16). Once Finding 16 lifts BoolLike, the impl becomes `impl const BoolLike for Bool`.

**Concrete change:** Cascade of Finding 16.

**Priority:** P1.

---

### Finding 35: `MetaCarrier::as_bits` and the macro-emitted `as_bits` accessors are const fn: but the underlying `core::mem::transmute` ABI assertions are checked at compile time only

**Location:** `arvo-storage/src/meta_bits.rs:65-69, 121-125`

**Gap:** `pub const fn as_bits(self) -> Bits<9, Hot, Unsigned>` is good. The body uses `core::mem::transmute` which is const-stable. But there's no `const _: Bits<9, Hot, Unsigned> = MetaCarrier::from_raw(0).as_bits();` smoke test asserting the layout-equivalence holds in const eval.

**Concrete change:** `arvo-storage/tests/meta_bits_const.rs` with `const _: Bits<9, Hot, Unsigned> = MetaCarrier::from_raw(123).as_bits(); const _: u16 = MetaCarrier::from_raw(123).to_raw();` plus IBits/FBits/Width as_bits parallel tests.

**Priority:** P2.

---

### Finding 36: `Resolve<Other>::Out` projection is not exercised in const context anywhere

**Location:** `arvo-strategy/src/lib.rs:175-210`

**Gap:** `Resolve` is `pub const trait` with same-pair impls only. No const-context smoke. Cross-strategy resolution (`<Hot as Resolve<Warm>>::Out` = Warm) compiles at type level; need a `const _: u16 = <<Hot as Resolve<Warm>>::Out as Strategy>::RANK;` smoke test demonstrating const-eval projection.

**Concrete change:** `arvo-strategy/tests/resolve_const.rs` with the four same-pair and 12 cross-pair `const _: u16 = <<S1 as Resolve<S2>>::Out as Strategy>::RANK;` assertions.

**Priority:** P2.

---

### Finding 37: `Strategy::RANK` is `const u16`; `Strategy::NAME` is gated on `cfg(debug_assertions)`: const access works only in debug builds

**Location:** `arvo-strategy/src/lib.rs:60-73`

**Gap:** `Strategy::NAME` is `#[cfg(debug_assertions)]`. A `const _: &'static str = <Hot as Strategy>::NAME;` works in debug but fails in release, breaking const eval consistency. Either drop the cfg gate (small `.rodata` cost) or document explicitly that NAME is debug-only and unreachable in const context for release.

**Concrete change:** Document the cfg gate via doc comment update, or drop the gate (4 strings × ~5 bytes is trivial). The `Strategy::RANK` part is fine.

**Priority:** P2.

---

### Finding 38: Bitfield `Default` / `PartialEq` / `Hash` / `Debug` derives are non-const but the macro emits non-const manual `PartialEq` / `Hash` / `Debug` (lines 162-181)

**Location:** `arvo/src/bitfield.rs:159-181`

**Gap:** `#[derive(Copy, Clone, Default)]` plus hand-written `PartialEq`, `Eq`, `Hash`, `Debug`. None const. Pure bodies. The const-callability of consumer-defined `bitfield! { struct Foo: 32 { ... } }` is therefore limited: `Foo::new()` is const fn (line 199), `Foo::from_bits` / `to_bits` const fn, per-field accessors / setters const fn: but `Foo::default()`, `Foo::eq`, `Foo::hash` are not.

**Concrete change:** Replace the `#[derive(Default)]` with manual `impl const ConstDefault for $name { fn const_default() -> Self { Self::new() } }`. Replace manual `impl PartialEq` with `impl const ConstEq for $name { fn const_eq(&self, other: &Self) -> Bool { Bool(self.0 == other.0) } }`. Hash / Debug are stdlib non-const; provide `impl Hash` / `Debug` as before for stdlib coverage but document the const-ineligibility.

**Cascade:** ConstDefault (Finding 7), ConstEq (Finding 9).

**Priority:** P1.

---

### Finding 39: `IFixed`'s default Strategy is `Warm` but `UFixed`'s default is also `Warm`, while Bits's default is `Hot`. Inconsistency means default const-context construction varies by type.

**Location:** `arvo/src/ufixed.rs:32` (default Warm), `arvo/src/ifixed.rs:41` (default Warm), `arvo-storage/src/bits.rs:51` (default Hot)

**Gap:** Not directly a const-trait gap, but a core-fundamentals gap. Inconsistent defaults mean `const _: Bits<8> = ...;` uses Hot, `const _: UFixed<{ibits(4)},{fbits(4)}> = ...;` uses Warm. Const expression composition across types becomes subtly cross-strategy without consumer awareness. This is a design-smell signal: likely intentional but worth flagging during the substrate-foundations push.

**Concrete change:** Document explicitly in DESIGN.md (or per-type doc) that Bits's Hot default reflects bit-pattern-storage semantics while UFixed/IFixed Warm reflects arithmetic-correctness ergonomics. Confirm via the user that this asymmetry is intended; if not, unify.

**Priority:** P2: design clarity, not const-trait completeness.

---

### Finding 40: `Hash::hash` impls are nowhere const; substrate has no `ConstHash` bridge

**Location:** Throughout: `Bits` derives Hash (arvo-storage/src/bits.rs:49), MetaCarrier derives Hash (meta_bits.rs:46), bitfield macro emits manual Hash (bitfield.rs:177). Plus arvo-hash crate (Hasher / Fnv1a) likely has non-const trait impls.

**Gap:** `core::hash::Hash::hash` is not const-stable. No bridge. Consumers cannot hash arvo values in const context. The `arvo-hash` crate (with `Fnv1a`, `Hasher<const N: Width>`, `ContentHash`) likely has the same gap; it's a arvo-shipped hash family that should be const-callable for compile-time content addressing.

**Concrete change:** Define `pub const trait ConstHash<H: [const] ConstHasher>: { fn const_hash(&self, hasher: &mut H); }` and `pub const trait ConstHasher { fn const_write(&mut self, bytes: &[u8]); fn const_finish(&self) -> u64; }` in arvo-hash. Implement on Fnv1a (FNV-1a is purely const-friendly: per-byte xor + multiply). Bridge ConstHash for every primitive wrapper type. This unlocks compile-time content addressing: a documented use case.

**Bridge required?** Yes: `ConstHash` / `ConstHasher`. Place: arvo-hash.

**Priority:** P1. Audit arvo-hash separately as part of the same gap-closure round.

---

## Summary of missing substrate const-bridges

The audit identifies six distinct bridges the substrate needs to define before PR #42 merge:

1. **`ConstEq`** (and `ConstOrd` as supertrait): `arvo-strategy`. Replaces non-const `PartialEq` / `Eq` / `PartialOrd` / `Ord` for in-arvo generic-context use. Required by Findings 9, 13, 14, 25, 38.
2. **`ConstDefault`**: `arvo-strategy`. Replaces non-const `Default::default` for typed-zero const construction. Required by Findings 7, 18, 38.
3. **`ConstFrom<T>` / `ConstTryFrom<T, E>`**: `arvo-strategy`. Replaces non-const `From::from` / `TryFrom::try_from` for in-arvo conversions. Required by Findings 8, 24.
4. **`ConstDeref` / `ConstAsRef<T>`**: `arvo-transparent`. Replaces non-const `Deref::deref` / `AsRef::as_ref`. Required by Findings 6, 24.
5. **`ConstTry` / `ConstControlFlow<R, O>`**: notko or arvo-storage. Replaces non-const `core::ops::Try` / `FromResidual` for `?`-on-Bool in const context. Required by Finding 21.
6. **`ConstHash` / `ConstHasher`**: arvo-hash. Replaces non-const `core::hash::Hash::hash` / `Hasher::write`. Required by Finding 40.

Lifting the eleven existing arvo traits / sub-substrates (Signedness, BitsContainerFor, IBitPrim parity, HasAxes / OverflowPolicy / ContainerWidth / StorageLayout, CrossStrategyOp, Ieee, marker traits, AsBool, BoolLike, BitPresentation, FastFloat/StrictFloat IEEE bound) is straight-lift work: no new bridges needed.

## Summary of cycle hazards

Round 202605021600 fixed Identity blankets on UFixed/IFixed via single-predicate routing through the inner Bits. Active cycle hazards I identified that may not yet be resolved:

1. **`BitsContainerFor` blanket impls** (`arvo-strategy/src/container.rs:83-95`): once both blankets become `impl const BitsContainerFor`, the trait solver needs to verify both `S: [const] UContainerFor<N>` AND `<S as UContainerFor<N>>::T: ConstParamTy_` etc. Some chains carry multiple where-clause predicates that may collide. Resolve by combining into a single sealing-trait predicate per impl.

2. **`UBitContainer<BITS>` / `IBitContainer<BITS>` blanket impls** (`arvo-bits-contracts/src/lib.rs:473-490, 505-522`): already use the `sealed::UBridge<BITS>` / `IBridge<BITS>` collapse pattern correctly; not a current hazard but verify under `[const]` propagation.

3. **`BitAccess` / `BitSequence` blanket impls extended to Sign axis** (Finding 3): when generalising from `Bits<N, S>` to `Bits<N, S, Sign>`, the where clauses gain an additional Sign-projection predicate. Use a unified `BitsContainerFor<N, Sign>` dispatch (which collapses U / I selection) plus a unified `BitOrIBitPrim` bound (a new sealed bridge picking BitPrim or IBitPrim based on Sign). The naive form with two separate predicates `where S: [const] UContainerFor<N>, S: [const] IContainerFor<N>` IF `Sign = Unsigned` || `Sign = Signed` is the cycle hazard to avoid; route through a single predicate `S: [const] BitsContainerFor<N, Sign>` instead.

4. **Cross-strategy Resolve projection in cross-strategy ops** (future work tracked in BACKLOG, but worth flagging here): when `Resolve<S1, S2>::Out` appears in an impl block alongside `S1: [const] UContainerFor<{ufixed_bits(I, F)}>` and `<<S1 as Resolve<S2>>::Out as UContainerFor<{ufixed_bits(I, F)}>>::T: BitPrim`, the resulting two-or-more anonymous const-expr predicates form the prototypical cycle. When this work lands, use the sealed-bridge collapse pattern.

## Const-context test coverage gap

Current const-context test surface: `arvo-storage/tests/usize_const_arith.rs` covering USize / Cap inherent ops + Bounded / Identity on bare primitives. Everything else is either runtime-only or absent.

Missing const-context smoke tests, per substrate trait:

| Crate | Const-context test missing for |
|---|---|
| arvo-storage | Bits Identity / Bounded across (N, S, Sign) cells; MetaCarrier as_bits / from_raw; meta-bits wrappers (IBits/FBits/Width) helper-fn const projections; Bool TRUE/FALSE access in const |
| arvo-bits-contracts | BitPrim per-primitive const access (count_ones, trailing_zeros, leading_zeros, get_bit, with_bit_set, with_bit_cleared, with_bit_toggled, bitor, bitand, bitnot, bitxor, clear_lowest_set_bit, is_zero); IBitPrim parity once Finding 5 lands; UBitContainer / IBitContainer const projections; Narrow / Widen cross-primitive |
| arvo-bits | Bits BitAccess / BitSequence / BitLogic blanket impls in const context across (N, S, Sign) cells; BitsRefitCtor const constructors |
| arvo-bitmask | Mask64 BitAnd / BitOr / BitXor / Not const-callable in const expr; Mask256 ditto; Mask Bounded MIN/MAX, Identity ZERO/ONE; once Finding 17 lands, the `arvo_mask_contracts::Mask<W>` impl const access |
| arvo-strategy | Resolve cross-pair projection const access; UArith / IArith per-(Strategy, BITS) const-callable; UWidenFrom / UNarrowFrom (Finding 10) |
| arvo (facade) | UFixed Identity ZERO/ONE; UFixed const Add/Sub/Mul/Div per strategy; UFixed strategy conversions (Finding 8 / 10); IFixed parity; FastFloat / StrictFloat Add/Sub/Mul/Div/Neg const; numeric-contract impls Sqrt/Abs/FromConstant/Recip/TotalOrd const callable post-Finding-13 |
| arvo-hash | Fnv1a const-context access; Hasher::write / finish in const fn; ContentHash const-construction |

Recommended landing pattern: one `tests/<surface>_const_arith.rs` per crate, each covering every const-trait method declared in that crate. Each test starts with `const _: ... = ...;` declarations forcing const-eval evaluation at test-build time, plus a `#[test] fn const_evaluations_match_runtime()` that asserts the const value equals the same expression evaluated at runtime.

---

End of audit. Total findings: 40. P0 findings (foundational, blocking): 1, 2, 3, 4, 5, 9, 10, 13, 17, 22. P1 findings (cascade after P0): 6, 7, 8, 11, 14, 15, 16, 18, 25, 27, 28, 32, 33, 34, 38, 40. P2 (consumer / call-site): 12, 19, 20, 21, 23, 24, 26, 29, 30, 31, 35, 36, 37, 39.
