## arvo-strategy   (files read: 18)

- purpose: "The strategy-marker and container-projection layer. Defines what "strategy" means in arvo: a compile-time tag that picks the underlying primitive container for a given bit width. No fielded types beyond the four ZST markers. `#![no_std]`, no alloc." (mock/crates/arvo-strategy/DESIGN.md.tmpl:3-6)
- README: "Strategy markers and the type-only container-projection trait." (README.md.tmpl:3)
- depends on: Cargo.toml not in the batch list. DESIGN.md.tmpl:8: "Depends on `notko` and `arvo-transparent`. Does NOT depend on `arvo-storage`." Sources `use notko::Outcome;` (const_convert.rs:15, widen.rs:14) and `use arvo_transparent::{ConstAsRef, ConstDeref, Transparent};` (widebits.rs:47). No cargo features visible in the listed files. tests/never_panic_contract.rs:21 adds `#![feature(macro_metavar_expr_concat)]`.
- attributes: lib.rs:1 `#![no_std]`; lib.rs:2 `#![feature(adt_const_params)]`; lib.rs:3 `#![feature(const_trait_impl)]`; lib.rs:11 `#![feature(generic_const_exprs)]` (comment lib.rs:4: "WATCH-tier unstable feature, soundness-vetted in the stack sweep (task #626)."); lib.rs:20 `#![feature(const_unsigned_bigint_helpers)]` (comment lib.rs:12: "ALLOWED-tier unstable feature, soundness-vetted 2026-06-23 and recorded in the"); lib.rs:21 `#![allow(incomplete_features)]`. Private mods: arith, arith_impls, arith_macros, axes, const_convert, container, cross_strategy, euclid, identity, ieee, widebits, widen (lib.rs:58-69).

### Public items

lib.rs
- `pub mod width;` (lib.rs:70) : no doc
- `pub use widebits::{Align, WideBits, A1, A16, A32, A64};` (lib.rs:72)
- `pub use width::{bytes_for, tag, width, width_le_64, width_u16, width_u8, Width};` (lib.rs:73)
- `pub use arith::{IArith, ISaturating, UArith, USaturating};` (lib.rs:75)
- `pub use axes::{Bitpacked, ContainerWidth, Dense, DoubleLogical, HasAxes, Min, OverflowPolicy, Saturating, StorageLayout, Wrapping};` (lib.rs:76-79)
- `pub use const_convert::{ConstFrom, ConstTryFrom};` (lib.rs:80)
- `pub use container::{BitsContainerFor, Picker, Project};` (lib.rs:81)
- `pub use cross_strategy::CrossStrategyOp;` (lib.rs:82)
- `pub use euclid::{ScalarEuclidRaw, UScalarEuclidRaw};` (lib.rs:83)
- `pub use identity::{tag_one_representable, Additive, Bounded, Identity, Multiplicative, OneRepresentable, SignedIdentity};` (lib.rs:84-86)
- `pub use ieee::{FromU8Ieee, Ieee};` (lib.rs:88)
- `pub use widen::{INarrowFrom, IWidenFrom, UNarrowFrom, UWidenFrom};` (lib.rs:89)
- `pub const trait Strategy: sealed::Sealed + Copy + Clone + Default + 'static` (lib.rs:95) : "Strategy marker trait."
  - `#[cfg(debug_assertions)] const NAME: &'static str;` (lib.rs:102) : "Human-readable name of the strategy."
  - `const RANK: u16;` (lib.rs:107) : "Conservativeness rank. Higher is more conservative. Used by"
- `pub struct Hot;` (lib.rs:116) : "Optimised for L1 density and operation throughput." (derives ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)
- `pub struct Warm;` (lib.rs:126) : "Store big, operate fast. The development-friendly default."
- `pub struct Cold;` (lib.rs:133) : "Store small, operate carefully. Optimised for storage density."
- `pub struct Precise;` (lib.rs:141) : "Store exactly, operate exactly. Correctness above all."
- `pub trait Signedness: sealed::Sealed + Copy + Clone + Default + 'static {}` (lib.rs:183) : "Sign-axis marker trait. Sealed; consumers cannot add new variants."
- `pub struct Unsigned;` (lib.rs:187) : "Unsigned bit pattern. Default `Sign` on `Bits<N, S, Sign>`."
- `pub struct Signed;` (lib.rs:191) : "Signed bit pattern. Used by `IFixed<I, F, S>` internally."
- `pub const trait Resolve<Other: Strategy>: Strategy` (lib.rs:210) : "Resolve the more conservative of two strategies."
  - `type Out: Strategy;` (lib.rs:212) : "The resolved strategy: more conservative of `Self` and `Other`."
- Strategy impls: `const impl Strategy for Hot/Warm/Cold/Precise` with RANK 0/1/2/3 (lib.rs:148-167). `impl_resolve!` for the 16 ordered pairs of the four strategies (lib.rs:215-245).

arith.rs
- `pub const trait UArith<const N: u16>: [const] BitsContainerFor<N, Unsigned> + HasAxes` (arith.rs:42) : "Unsigned arithmetic dispatch for `(strategy, N)`."
  - `fn u_add(a: <Self as BitsContainerFor<N, Unsigned>>::T, b: <Self as BitsContainerFor<N, Unsigned>>::T) -> <Self as BitsContainerFor<N, Unsigned>>::T;` (arith.rs:46) : "Strategy-specific `+`."
  - `fn u_sub(a, b) -> T` same shape (arith.rs:51) : "Strategy-specific `-`."
  - `fn u_mul(a, b) -> T` (arith.rs:56) : "Strategy-specific `*`."
  - `fn u_div(a, b) -> T` (arith.rs:64) : "Strategy-specific `/`. On a zero divisor the wrapping strategies"
  - `fn u_mul_fixed<const FRAC: u16>(a, b) -> T` (arith.rs:74) : "Fixed-point `*` for values scaled by `2^FRAC`: the product of two"
  - `fn u_div_fixed<const FRAC: u16>(a, b) -> T` (arith.rs:85) : "Fixed-point `/` for values scaled by `2^FRAC`: the quotient of two"
- `pub const trait IArith<const N: u16>: [const] BitsContainerFor<N, Signed> + HasAxes` (arith.rs:95) : "Signed arithmetic dispatch for `(strategy, N)`."
  - `fn i_add(a, b) -> T` (arith.rs:97) : "Strategy-specific `+`."
  - `fn i_sub(a, b) -> T` (arith.rs:102) : "Strategy-specific `-`."
  - `fn i_mul(a, b) -> T` (arith.rs:107) : "Strategy-specific `*`."
  - `fn i_div(a, b) -> T` (arith.rs:115) : "Strategy-specific `/`. On a zero divisor the wrapping strategies"
  - `fn i_mul_fixed<const FRAC: u16>(a, b) -> T` (arith.rs:120) : "Fixed-point `*` for values scaled by `2^FRAC`: see `UArith::u_mul_fixed`."
  - `fn i_div_fixed<const FRAC: u16>(a, b) -> T` (arith.rs:126) : "Fixed-point `/` for values scaled by `2^FRAC`: see `UArith::u_div_fixed`."
- `pub const trait USaturating: Sized` (arith.rs:138) : "Helper trait: yields the MAX value of an unsigned container type."
  - `fn saturating_max() -> Self;` (arith.rs:140) : "`T::MAX` for this container."
- `pub const trait ISaturating: Sized` (arith.rs:144) : "Signed counterpart of `USaturating`."
  - `fn saturating_max() -> Self;` (arith.rs:146) : "`T::MAX` for this container."
- `impl_saturating!(unsigned: u8, u16, u32, u64, u128);` (arith.rs:164); `impl_saturating!(signed: i8, i16, i32, i64, i128);` (arith.rs:165)

arith_impls.rs: no public items. Invocations: `impl_u_arith_wrapping_widen!` Hot and Cold at u8 (1..=8), u16 (9..=16), u32 (17..=32), u64 (33..=64); `impl_u_arith_wrapping_widen256!` Hot and Cold at 65..=128; `impl_u_arith_wrapping!` Warm at 1..=64; `impl_u_arith_saturating!` Precise at 1..=64; signed mirrors with `impl_i_arith_*` (arith_impls.rs:15-172).

arith_macros.rs: no public items (`pub(crate)` only): macros impl_u_arith_wrapping, impl_u_arith_saturating, impl_i_arith_wrapping, impl_i_arith_saturating, impl_u_arith_wrapping_widen, impl_i_arith_wrapping_widen, impl_u_arith_wrapping_widen256, impl_i_arith_wrapping_widen256 (arith_macros.rs:533-540); `pub(crate) const fn umul256(a: u128, b: u128) -> (u128, u128)` (:367), `pub(crate) const fn shr256_lo(lo: u128, hi: u128, frac: u32) -> u128` (:374), `pub(crate) const fn u_mul_fixed_128(a: u128, b: u128, frac: u32) -> u128` (:392), `pub(crate) const fn i_mul_fixed_128(a: i128, b: i128, frac: u32) -> i128` (:402).

axes.rs
- `pub const trait OverflowPolicy: sealed::Sealed + Copy + Clone + Default + 'static` (axes.rs:41) : "How arithmetic handles overflow."
  - `const DISCRIMINANT: u16;` (axes.rs:43) : "Stable per-marker discriminant for compile-time projection."
- `pub struct Wrapping;` (axes.rs:51) : "Overflow policy: arithmetic wraps modulo container width."
- `pub struct Saturating;` (axes.rs:58) : "Overflow policy: arithmetic saturates at logical min/max."
- `pub const trait ContainerWidth: sealed::Sealed + Copy + Clone + Default + 'static` (axes.rs:77) : "Container width relative to logical width."
  - `const DISCRIMINANT: u16;` (axes.rs:79) : "Stable per-marker discriminant for compile-time projection."
- `pub struct Min;` (axes.rs:88) : "Container width: minimum byte-aligned width that fits the logical"
- `pub struct DoubleLogical;` (axes.rs:95) : "Container width: 2x the logical bit width (one bucket up)."
- `pub const trait StorageLayout: sealed::Sealed + Copy + Clone + Default + 'static` (axes.rs:114) : "How multiple values pack in column storage."
  - `const DISCRIMINANT: u16;` (axes.rs:116) : "Stable per-marker discriminant for compile-time projection."
- `pub struct Dense;` (axes.rs:124) : "Storage layout: each value occupies a full container slot."
- `pub struct Bitpacked;` (axes.rs:133) : "Storage layout: values pack into shared container slots, one bit"
- `pub const trait HasAxes` (axes.rs:159) : "Project a `Strategy` marker onto its three axis components."
  - `type Overflow: OverflowPolicy;` (axes.rs:161) : "Overflow policy of this strategy."
  - `type Width: ContainerWidth;` (axes.rs:163) : "Container width of this strategy."
  - `type Layout: StorageLayout;` (axes.rs:165) : "Storage layout of this strategy."
- HasAxes impls: Hot (Wrapping, Min, Dense), Warm (Wrapping, DoubleLogical, Dense), Cold (Wrapping, Min, Bitpacked), Precise (Saturating, DoubleLogical, Dense) (axes.rs:168-190). DISCRIMINANT: Wrapping 0, Saturating 1, Min 0, DoubleLogical 1, Dense 0, Bitpacked 1.

const_convert.rs
- `pub const trait ConstFrom<T>: Sized` (const_convert.rs:22) : "Const-callable conversion from `T` into `Self`."
  - `fn const_from(value: T) -> Self;` (const_convert.rs:24) : "Convert `value` into `Self`."
- `pub const trait ConstTryFrom<T>: Sized` (const_convert.rs:32) : "Const-callable fallible conversion from `T` into `Self`."
  - `type Error;` (const_convert.rs:34) : "Conversion failure carrier."
  - `fn const_try_from(value: T) -> Outcome<Self, Self::Error>;` (const_convert.rs:37) : "Try to convert `value` into `Self`. On failure, returns the"

container.rs
- `pub const fn tag_hot_cold(n: u16) -> usize` (container.rs:60) : no doc
- `pub const fn tag_warm_precise(n: u16) -> usize` (container.rs:78) : no doc
- `pub const fn bytes_for_u16(n: u16) -> usize` (container.rs:94) : no doc
- `pub const trait BitsContainerFor<const N: u16, Sign: Signedness>: Strategy` (container.rs:114) : "Sign-aware container dispatch: `(strategy, logical_bits, sign) -> type`."
  - `type T: Copy + Clone + PartialEq + Eq + Default + core::hash::Hash + core::fmt::Debug + 'static;` (container.rs:118) : "Concrete storage type for this `(strategy, bit-width, sign)`"
- `pub trait Project<const TAG: usize, Sign: Signedness, const BYTES: usize, S: Strategy>: crate::sealed::Sealed` (container.rs:137) : "Bucket-keyed projection helper. Per-Strategy impls below cover"
  - `type T: Copy + Clone + PartialEq + Eq + Default + core::hash::Hash + core::fmt::Debug + 'static;` (container.rs:141) : "Concrete storage type for this projection slot."
- `pub struct Picker;` (container.rs:152) : "Single dispatch site for `Project` impls. Zero-sized."
- Impls: `impl_native_bucket!` Hot and Cold [0=>u8,1=>u16,2=>u32,3=>u64,4=>u128] / [i8..i128]; Warm and Precise [0=>u16,1=>u32,2=>u64,3=>u128] / [i16..i128] (container.rs:191-218). Wide bucket `Project<5, Sign, BYTES, Hot>` = `WideBits<BYTES, A16>`; Cold, Warm, Precise = `WideBits<BYTES, A1>` (container.rs:230-244). `const impl BitsContainerFor<N, Sign>` for Hot, Cold (tag_hot_cold) and Warm, Precise (tag_warm_precise) (container.rs:254-280).

cross_strategy.rs
- `pub const trait CrossStrategyOp<S1: Strategy, S2: Strategy> {}` (cross_strategy.rs:49) : "Marker trait that fires a compile-time diagnostic at cross-strategy"
- Impls for `()` at (Hot,Hot), (Warm,Warm), (Cold,Cold), (Precise,Precise) (cross_strategy.rs:51-54).

euclid.rs
- `pub const trait ScalarEuclidRaw<const N: u16>: [const] BitsContainerFor<N, Signed>` (euclid.rs:29) : "Inner per-width signed euclidean-division contract."
  - `fn div_euclid_scalar(a: <Self as BitsContainerFor<N, Signed>>::T, n: usize) -> <Self as BitsContainerFor<N, Signed>>::T;` (euclid.rs:32) : "Euclidean quotient of the raw container by an integer count (ULP granularity)."
  - `fn rem_euclid_scalar(a, n: usize) -> T` (euclid.rs:37) : "Euclidean remainder of the raw container by an integer count (non-negative, < n)."
  - `fn even_split_parts(a, n: usize) -> (T, T, usize)` (euclid.rs:42) : "`(base, base + 1 ULP, remainder as a share count)` for an even split into `n` parts."
  - `fn div_euclid_whole<const FRAC: u16>(a, b) -> T` (euclid.rs:51) : "Whole-unit euclidean quotient by a fixed-point divisor (`FRAC` fractional bits), floored to units."
  - `fn rem_euclid_whole<const FRAC: u16>(a, b) -> T` (euclid.rs:56) : "Whole-unit euclidean remainder by a fixed-point divisor (the fixed-point leftover)."
- `pub const trait UScalarEuclidRaw<const N: u16>: [const] BitsContainerFor<N, Unsigned>` (euclid.rs:133) : "Inner per-width unsigned euclidean-division contract."
  - same five methods: `div_euclid_scalar` (euclid.rs:138) : "Euclidean quotient of the raw container by an integer count (ULP granularity)."; `rem_euclid_scalar` (:143) : "Euclidean remainder of the raw container by an integer count."; `even_split_parts` (:148) : "`(base, base + 1 ULP, remainder as a share count)` for an even split into `n` parts."; `div_euclid_whole<const FRAC: u16>` (:157) : "Whole-unit euclidean quotient by a fixed-point divisor (`FRAC` fractional bits), floored to units."; `rem_euclid_whole<const FRAC: u16>` (:162) : "Whole-unit euclidean remainder by a fixed-point divisor (the fixed-point leftover)."
- Impls: `impl_scalar_euclid_i!` / `impl_scalar_euclid_u!` for Hot and Cold (i8/u8 1..=8, i16/u16 9..=16, i32/u32 17..=32, i64/u64 33..=64, i128/u128 65..=128) and Warm and Precise (i16/u16 1..=8, i32/u32 9..=16, i64/u64 17..=32, i128/u128 33..=64) (euclid.rs:92-232).

identity.rs
- `pub const trait Bounded: Sized` (identity.rs:20) : "Per-type bottom/top const surface."
  - `const MIN: Self;` (identity.rs:22) : "The minimum representable value of this type."
  - `const MAX: Self;` (identity.rs:24) : "The maximum representable value of this type."
- `pub struct Additive;` (identity.rs:28) : "The operation whose identity is addition."
- `pub struct Multiplicative;` (identity.rs:31) : "The operation whose identity is multiplication."
- `pub const trait Identity<Op>: Sized` (identity.rs:51) : "The identity element of `Op` in this type."
  - `const IDENTITY: Self;` (identity.rs:53) : "The identity element of `Op` in this type."
- `pub const fn tag_one_representable(int_bits: u16) -> usize` (identity.rs:70) : "Compresses the integer-bit count into the two-element space "one is"
- `pub trait OneRepresentable<const TAG: usize>: crate::sealed::Sealed {}` (identity.rs:89) : "Witness that one is representable at a given tag."; `impl OneRepresentable<0> for Picker {}` (identity.rs:91)
- `pub const trait SignedIdentity: Sized` (identity.rs:135) : "Per-signed-type negative-one const surface."
  - `const NEG_ONE: Self;` (identity.rs:137) : "The additive inverse of the multiplicative identity (signed -1)."
- Impls: `impl_bounded_identity!(0, 1, u8, u16, u32, u64, u128, usize);` (:110); `impl_bounded_identity!(0, 1, i8, i16, i32, i64, i128, isize);` (:111); `impl_bounded_identity!(0.0, 1.0, f32, f64);` (:120) giving Bounded, Identity<Additive>, Identity<Multiplicative`>; `impl_signed_identity!(-1, i8, i16, i32, i64, i128, isize);` (:148); `impl_signed_identity!(-1.0, f32, f64);` (:154).

ieee.rs
- `pub const trait Ieee: sealed::Sealed + Copy + Default + PartialEq + PartialOrd + [const] Identity<Additive> + [const] Identity<Multiplicative> + [const] Bounded + 'static` (ieee.rs:42) : "IEEE float width marker. Sealed: implementable only for `f32`"
  - `const WIDTH: Width;` (ieee.rs:54) : "Width of this IEEE type in bits."
- `pub const trait FromU8Ieee: Ieee` (ieee.rs:66) : "Lossless `u8 -> IEEE float` bridge."
  - `fn from_u8_ieee(n: u8) -> Self;` (ieee.rs:68) : "Convert the given `u8` into this IEEE float type."
- Impls for f32 (WIDTH = 32) and f64 (WIDTH = 64) (ieee.rs:57-63, 71-85).

widebits.rs
- `pub trait Align: Copy` (widebits.rs:54) : "Alignment marker trait. Implementations are zero-sized types"
  - `const VALUE: usize;` (widebits.rs:56) : "The alignment in bytes."
- `pub struct A1;` (widebits.rs:62) `#[repr(C, align(1))]` : "Align-1 marker. Default for Warm / Cold / Precise wide storage."
- `pub struct A16;` (widebits.rs:71) `#[repr(C, align(16))]` : "Align-16 marker. Hot baseline (SSE2 / NEON)."
- `pub struct A32;` (widebits.rs:80) `#[repr(C, align(32))]` : "Align-32 marker. Hot AVX-2 tier (lands via #320 cfg gate)."
- `pub struct A64;` (widebits.rs:89) `#[repr(C, align(64))]` : "Align-64 marker. Hot AVX-512 tier (lands via #320 cfg gate)."
- `pub struct WideBits<const BYTES: usize, A: Align = A1>` (widebits.rs:103) `#[repr(C)]` : "Byte-sequence storage, parametric over alignment."
  - `pub bytes: [u8; BYTES]` (widebits.rs:109) : "Little-endian byte sequence. Bit 0 of the logical value is"
  - `pub const fn zero() -> Self` (widebits.rs:115) : "Zero value: all bytes zero."
  - `pub const fn from_bytes(bytes: [u8; BYTES]) -> Self` (widebits.rs:124) : "Construct from a byte sequence."
  - `pub const fn as_bytes(&self) -> &[u8; BYTES]` (widebits.rs:135) : "Borrow the underlying byte sequence."
- Impls: `const unsafe impl Transparent` (Inner = [u8; BYTES]) (:145); `Default` (:149); `Identity<Additive>` all-zero (:159); `Identity<Multiplicative>` bytes[0] = 1 (:166); `Bounded` MIN all-zero, MAX all 0xFF (:182); `ConstDeref` Target = [u8; BYTES] (:199); `ConstAsRef<[u8; BYTES]>` (:212).

widen.rs
- `pub const trait UWidenFrom<Src: BitsContainerFor<N, Unsigned> + HasAxes, const N: u16>: BitsContainerFor<N, Unsigned> + HasAxes` (widen.rs:27) : "Unsigned container widen bridge: `(Src, N) -> Dst::T`."
  - `fn u_widen(v: <Src as BitsContainerFor<N, Unsigned>>::T) -> <Self as BitsContainerFor<N, Unsigned>>::T;` (widen.rs:31) : "Widen an `Src::T` value into `Self::T`. Infallible by definition."
- `pub const trait IWidenFrom<Src: BitsContainerFor<N, Signed> + HasAxes, const N: u16>: BitsContainerFor<N, Signed> + HasAxes` (widen.rs:37) : "Signed container widen bridge."
  - `fn i_widen(v) -> <Self as BitsContainerFor<N, Signed>>::T;` (widen.rs:41) : "Widen an `Src::T` value into `Self::T`. Infallible by definition."
- `pub const trait UNarrowFrom<Src: BitsContainerFor<N, Unsigned> + HasAxes, const N: u16>: BitsContainerFor<N, Unsigned> + HasAxes` (widen.rs:48) : "Unsigned container narrow bridge with bounds check against the"
  - `fn u_try_narrow(v) -> Outcome<<Self as BitsContainerFor<N, Unsigned>>::T, ()>;` (widen.rs:52) : "Try to narrow `v` into `Self::T`. `Outcome::Err(())` when out of range."
- `pub const trait INarrowFrom<Src: BitsContainerFor<N, Signed> + HasAxes, const N: u16>: BitsContainerFor<N, Signed> + HasAxes` (widen.rs:58) : "Signed container narrow bridge with logical-range check."
  - `fn i_try_narrow(v) -> Outcome<<Self as BitsContainerFor<N, Signed>>::T, ()>;` (widen.rs:62) : "Try to narrow `v` into `Self::T`. `Outcome::Err(())` when out of range."
- Impls: widen Hot=>Warm, Hot=>Precise (u8=>u16 1..=8, u16=>u32 9..=16, u32=>u64 17..=32, u64=>u128 33..=64), Warm=>Precise (same-type casts); narrow Warm=>Hot, Precise=>Hot; signed mirrors (widen.rs:158-316). widen.rs:318: "// Cold widen/narrow impls deferred."

width.rs
- `pub struct Width(pub u16);` (width.rs:33) `#[repr(transparent)]` derives ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default, Hash : "Logical bit-width carrier."
  - `pub const ZERO: Self = Width(0);` (width.rs:37) : "Zero width."
  - `pub const ONE: Self = Width(1);` (width.rs:39) : "One bit."
  - `pub const W64: Self = Width(64);` (width.rs:41) : "64-bit width (FNV-1a-64 cap)."
  - `pub const W128: Self = Width(128);` (width.rs:43) : "128-bit native ladder cap."
  - `pub const fn raw(self) -> u16` (width.rs:47) : "Read the underlying u16 carrier."
- `pub const fn width(n: u16) -> Width` (width.rs:82) : "Construct a `Width` from a u16 literal."
- `pub const fn width_u8(n: u8) -> Width` (width.rs:92) : "Construct a `Width` from a u8 literal (widening)."
- `pub const fn width_u16(n: u16) -> Width` (width.rs:102) : "Construct a `Width` from a u16 literal (explicit form)."
- `pub const fn width_le_64(n: u8) -> Width` (width.rs:112) : "Construct a `Width` clamped to <= 64."
- `pub const fn tag(n: Width) -> usize` (width.rs:121) : "Bucket tag for Pattern C const-tag dispatch (sketch 07)."
- `pub const fn bytes_for(n: Width) -> usize` (width.rs:143) : "Round logical bit count up to whole bytes."
- Impls: `const impl Bounded for Width` (:52); `const impl<Op> Identity<Op> for Width where u16: [const] Identity<Op>` (:57); `const impl ConstFrom<u16> for Width` (:69).

### Stated behaviour
- DESIGN.md.tmpl:36-38: "`Hot` selects the narrowest container per bit width. 1..=8 bits use `u8`, 9..=16 use `u16`, 17..=32 use `u32`, 33..=64 use `u64`,"
- DESIGN.md.tmpl:43-47: "`Warm` selects a containers matching Hot's footprint up to native primitives. The earlier "2x-logical safety" Warm cap collapsed to "same projection as Hot" once arvo's bit-width range exceeded single native primitives."
- DESIGN.md.tmpl:51-52: "`Cold` mirrors Warm's container shape; bitpacking is an access-path concern at the column-store layer above the per-Bits container, not a per-value container shape."
- DESIGN.md.tmpl:56-57: "`Precise` mirrors Warm's container shape; saturating arithmetic distinguishes the strategy at the op layer."
- DESIGN.md.tmpl:60-63: "`Precise` saturation clamps at the logical bound, the `i_MIN..i_MAX` (or `0..u_MAX`) of the logical width `N`, not at the wider container bound."
- DESIGN.md.tmpl:69-71: "`Hot` and `Cold` do not. They wrap at the **container** width, not the logical width, so a result may hold a raw above the logical maximum: a multiply at logical 12 in a 16-bit container can return 57408 against a logical maximum of 4095."
- DESIGN.md.tmpl:75-78: "It is a contract `Precise` keeps. Choosing a never-panic strategy is choosing this, and a consumer that needs the raw to stay inside the logical range either picks `Precise` or masks on output."
- DESIGN.md.tmpl:79-85: "A zero divisor has a per-strategy answer, and the strategies differ. `Hot`, `Warm` and `Cold` return the numerator unchanged: wrapping arithmetic has no defined answer for division by zero, and propagating the numerator is the cheapest fallback that does not panic. `Precise` clamps to the logical bound on the side the quotient is heading toward, which is the maximum for a non-negative numerator and the minimum for a negative one."
- DESIGN.md.tmpl:92-97: "At the container minimum the signed multiply returns that minimum directly, rather than reaching it by negating its own magnitude."
- DESIGN.md.tmpl:99-103: "The fixed-point multiply forms a `2N`-bit intermediate before the `>> F` rescale (the product of two `N`-bit values needs `2N` bits)." and "The rescale uses an arithmetic shift, so negative results floor toward minus infinity."
- DESIGN.md.tmpl:111-113: "The divide truncates toward zero, the natural integer-`/` behaviour, a documented asymmetry with the multiply's floor."
- DESIGN.md.tmpl:115-118: "The 128-bit boundary captures the rustc primitive ladder: u8/u16/u32/ u64/u128 each have stable Rust support with stdlib ops that lower as well as the toolchain currently does for that width."
- DESIGN.md.tmpl:156-161 (table): Hot Wrapping/Min/Dense; Warm Wrapping/Min/Dense; Cold Wrapping/Min/Bitpacked; Precise Saturating/Min/Dense. axes.rs:153-158 (table): Hot Wrapping/Min/Dense; Warm Wrapping/DoubleLogical/Dense; Cold Wrapping/Min/Bitpacked; Precise Saturating/DoubleLogical/Dense.
- DESIGN.md.tmpl:171-175: "This collapses all four strategies to `Min` container width: with the 128-bit native boundary, the previous "2x logical" distinction at the storage layer disappears."
- DESIGN.md.tmpl:177-182: "Each axis surface carries a `const DISCRIMINANT: u8` projection." (code: `const DISCRIMINANT: u16;` axes.rs:43)
- lib.rs:32-33: "Four strategies determine container width, arithmetic semantics, and cross-width rules for `UFixed` / `IFixed`:" with table lib.rs:35-40: Hot "min aligned" "wrapping"; Warm "2x logical" "wrapping (safe 1-op)"; Cold "min (bitpacked)" "widen-op-narrow"; Precise "2x logical" "saturating".
- lib.rs:48-50: "`UFixed<_, _, Warm>` with `I + F > 64` is a compile-time error pointing at Hot or Cold via the `#[diagnostic::on_unimplemented]` note on `BitsContainerFor`."
- lib.rs:110-114 (Hot doc): "Container is the minimum byte-aligned standard width that fits `I + F` bits. Arithmetic is wrapping. Single instruction per op; LLVM vectorises freely."
- lib.rs:120-124 (Warm doc): "Container is 2x the logical bit width. A single `add` / `sub` / `mul` of two values within their logical range cannot overflow the container. Bounded to `I + F <= 32` per doc CL D2: no u128 container is available, so Warm is forbidden at logical widths beyond 32 bits rather than degrading silently."
- lib.rs:130-131 (Cold doc): "Minimum container, bitpacked for sub-byte values. Arithmetic widens to 2x before operating, narrows back on store."
- lib.rs:137-139 (Precise doc): "Container is 2x the logical width (same physical layout as Warm). Arithmetic is saturating: overflow clamps to logical min/max rather than wrapping."
- arith.rs:9-17: "- `Hot`: wrapping (single-op convention; overflow wraps)." "- `Warm`: wrapping on the 2x container (safe for a single op)." "- `Cold`: saturating bound at the 2x-widened equivalent; in this L0 round we use the container's own wrapping ops since Cold and Hot share container widths." "- `Precise`: saturating at the container level."
- arith.rs:19-24: "Division by zero: Hot/Warm/Cold return the numerator unchanged (wrapping math has no identity for zero, so propagating `a` is the cheapest defined fallback that does not panic)."
- arith_macros.rs:54: "// Precise saturates at the LOGICAL bound, not the container bound (design topic"
- arith_macros.rs:156-158: "// `wrapping_div`, not `/`. Signed MIN / -1 has no representable answer and // `/` panics on it, which a never-panic strategy must not do."
- arith_macros.rs:234-238: "`saturating_div`, matching the sibling `i_div` above. `/` panics on signed MIN / -1, and `wrapping_div` answers it with the minimum, which the clamp below reads as already inside the bound and passes through."
- arith_macros.rs:469-471: "// CATALOGUE (tracked, task #5): correct only when `a << FRAC` fits the u128 container; the // 65..=128 case needs 256/128 long division (no `carrying_div` intrinsic)."
- arith_macros.rs:366-367: "TODO(perf, task #4 / research 202606231500): `carrying_mul` is portable"
- euclid.rs:16-19: "Bodies are strategy-independent (euclidean division by a count >= 1 shrinks magnitude, so the quotient stays inside the logical bound the numerator satisfied; no Precise-style clamp needed). Div-by-zero returns the numerator (the `i_div` convention)."
- identity.rs:41-46: "**Absence is a statement.** An identity element must be a value of the type it is an identity for, so where an operation has no identity in the type there is no impl, and naming `IDENTITY` for it fails to resolve rather than returning something plausible."
- ieee.rs:3: "`Ieee` carries the bit-width of an IEEE 754 float type plus a" (ieee.rs:30-31) "IEEE float width marker. Sealed: implementable only for `f32` and `f64`."
- Named external standards: IEEE 754 (ieee.rs:3). No MATLAB `fi`, no Q format mention in the listed files.
- widebits.rs:34-39: "Per audit L2: when `BYTES` is not a multiple of `A::VALUE`, the struct's physical size is padded to `round_up(BYTES, A::VALUE)` but the trailing pad bytes after `[u8; BYTES]` are NOT part of the safe field surface."
- widebits.rs:41-42: "Byte-ordering convention (per audit C1): bit 0 is the LSB of byte 0."
- DESIGN.md.tmpl:565 and cross_strategy.rs:8-11: "Per the `arvo-toolbox-not-policer` workspace rule, the substrate cannot refuse the op (the consumer's choice stands), but it can warn at compile time when the resolution shifts overflow policy, container width, or storage layout."
- Consumer bound/select: lib.rs:20-22 of cross_strategy.rs: "code that bounds on `where (S1, S2): CrossStrategyOp<S1, S2>` produces a compile-time warning naming the shift"; DESIGN.md.tmpl:530-536: consumers bound on `BitsContainerFor<N, Sign>` per strategy; `where T: [const] HasAxes` for const-context callers (DESIGN.md.tmpl:186-187).
- tests/never_panic_contract.rs:1: "`Hot` and `Cold` return a value for every input. They never panic."

### Backlog and known gaps
No BACKLOG.md.tmpl or SHAME.md.tmpl in this crate's listed files. DESIGN.md.tmpl:683: "Round-by-round history is recorded in `mock/research/arvo-strategy-history.md` (Tier 2)." Headings of DESIGN.md.tmpl: "# arvo-strategy. Design", "## Strategy markers", "## Strategy trait", "## Strategy axis decomposition", "## Sealed marker traits and `pub const trait` discipline", "## Const-trait bridges (`ConstEq`, `ConstOrd`, `ConstDefault`)", "## ConstFrom and ConstTryFrom bridges", "## Canonical typed-const surfaces: `Bounded`, `Identity`, `SignedIdentity`", "## Width newtype", "## Storage primitives (cross-references)", "## BitsContainerFor: unified container projection", "## mask_low_bits helper", "## Strategy widen / narrow as `pub const trait`", "## Macro-generated impls: `impl_bounded_identity_*!` extension", "## Resolve", "## Const-trait arithmetic surface", "## Scalar euclidean raw contract", "## Ieee".

### Tests
- tests/never_panic_contract.rs: 6 `#[test]` attributes (2 inside `macro_rules!` bodies, 4 plain: `signed_fixed_multiply_at_the_container_minimum`, `precise_division_saturates_upward_at_the_container_minimum`, `wrapping_division_wraps_where_precise_saturates`, `zero_divisor_answers_per_strategy`); the `across_widths!` invocations (hot wide, cold wide, warm narrow, precise narrow) expand the macro bodies into 36 further test functions.
- tests/resolve_const_projection.rs: 1 (`resolve_const_projection_runtime_parity`).
- No `#[test]` in src files.

## arvo-tensor   (files read: 16)

- purpose: "Fixed-shape, type-level-capacity tensor wrappers. Every container binds a `Cap` const generic to a homogeneous value type; every slot is materialised at compile time; no heap. `#![no_std]`, no alloc, stack-only." (mock/crates/arvo-tensor/DESIGN.md.tmpl:3-6)
- README: "L2 fixed-shape tensor wrappers with type-level capacity:" (README.md.tmpl:3)
- depends on: Cargo.toml:7-8 `[dependencies] arvo.workspace = true`; Cargo.toml:10-11 `[features] default = []` (no other features). Package keys `version.workspace`, `edition.workspace`, `license.workspace` (Cargo.toml:3-5). DESIGN.md.tmpl:8-14: "The crate now depends on `arvo-storage` (for `USize` and the strategy markers used in trait bounds), `arvo-bits-contracts` (for the `HasBitWidth` and related trait surface used by bit-bearing tensor consumers), and `arvo-numeric-contracts` (for the `Predicate` family applied in filter / select operations)." README.md.tmpl:7: "Depends on arvo L0 only." Sources use `arvo::{Cap, USize}`.
- attributes: lib.rs:20 `#![no_std]`; lib.rs:21 `#![feature(const_trait_impl)]` (comment lib.rs:22-24: "`const_trait_impl` (WATCH-tier ALLOWED in the unstable-feature sweep; already in use across arvo-bits / arvo-bitmask / arvo-storage / the contract crates) gates the `ConstCapacity` const trait, the const-callable sibling of `Capacity`.").

### Public items

lib.rs
- `pub mod array;` (lib.rs:33), `pub mod cap;` (:34), `pub mod capacity;` (:35), `pub mod enumerator;` (:36), `pub mod matrix;` (:37) : no doc
- `pub use array::Array;` (:39); `pub use cap::{cap, cap_size};` (:40); `pub use capacity::{Capacity, ConstCapacity, Dim};` (:41); `pub use enumerator::Enumerator;` (:42); `pub use matrix::Matrix;` (:43)

array.rs
- `pub struct Array<T, C: Capacity>` (array.rs:16) `#[repr(transparent)]` : "Rank-1 fixed-size tensor." (field `data: C::Array<T>` private)
  - `pub const fn new(data: C::Array<T>) -> Self` (array.rs:23) : "Construct from a pre-built backing array."
  - `pub fn from_fn<F>(f: F) -> Self where F: FnMut(USize) -> T` (array.rs:29) : "Construct by invoking `f(i)` for every slot."
  - `pub fn get(&self, i: USize) -> &T` (array.rs:38) : "Read the slot at `i`."
  - `pub fn set(&mut self, i: USize, v: T)` (array.rs:45) : "Write `v` to slot `i`."
  - `pub const fn len() -> USize` (array.rs:52) : "Slot count. Matches the capacity."
  - `pub fn as_slice(&self) -> &[T]` (array.rs:58) : "Inner slice view."
  - `pub fn as_mut_slice(&mut self) -> &mut [T]` (array.rs:64) : "Mutable inner slice view."
  - in `impl<T: Copy, C: Capacity> Array<T, C>`: `pub fn filled(v: T) -> Self` (array.rs:72) : "Build an Array populated with `v` in every slot."
  - trait impls: `Copy` where `C::Array<T>: Copy` (:77); `Clone` (:79); `IntoIterator for &'a Array<T, C>` Item = `&'a T` (:89); `IntoIterator for &'a mut Array<T, C>` Item = `&'a mut T` (:99)

cap.rs
- `pub const fn cap_size(c: Cap) -> usize` (cap.rs:23) : "Unwrap `Cap` to `usize` for array sizing in"
- `pub const fn cap(n: usize) -> Cap` (cap.rs:36) : "Build a `Cap` from a `usize`, the inverse of `cap_size`."

capacity.rs
- `pub trait Capacity` (capacity.rs:19) : "A fixed storage capacity expressed as a type."
  - `type Array<T>: AsRef<[T]> + AsMut<[T]>;` (capacity.rs:21) : "The backing array for `T` at this capacity. `[T; N]` for `Dim<N>`."
  - `const CAP: Cap;` (capacity.rs:24) : "The typed capacity. `Dim<N>` maps to `cap(N)`."
  - `fn filled<T: Copy>(v: T) -> Self::Array<T>;` (capacity.rs:27) : "Build the backing array with `v` in every slot."
  - `fn from_fn<T, F: FnMut(USize) -> T>(f: F) -> Self::Array<T>;` (capacity.rs:34) : "Build the backing array by invoking `f(i)` for each slot index."
- `pub struct Dim<const N: usize>;` (capacity.rs:44) : "A capacity of exactly `N` slots."
  - `impl<const N: usize> Capacity for Dim<N>`: `type Array<T> = [T; N]`, `const CAP: Cap = cap(N)` (capacity.rs:46-59)
- `pub const trait ConstCapacity` (capacity.rs:85) : "The const-callable capacity surface, the const analog of [`Capacity`]."
  - `type Array<T: Copy>: Copy;` (capacity.rs:87) : "The backing array for `T` at this capacity. `[T; N]` for `Dim<N>`."
  - `const CAP: Cap;` (capacity.rs:90) : "The typed capacity. `Dim<N>` maps to `cap(N)`."
  - `fn filled<T: Copy>(v: T) -> Self::Array<T>;` (capacity.rs:93) : "Build the backing array with `v` in every slot."
  - `fn get<T: Copy>(a: &Self::Array<T>, i: USize) -> T;` (capacity.rs:96) : "Read the slot at `i` by value."
  - `fn set<T: Copy>(a: &mut Self::Array<T>, i: USize, v: T);` (capacity.rs:99) : "Write `v` to the slot at `i`."
  - `fn slice<T: Copy>(a: &Self::Array<T>) -> &[T];` (capacity.rs:107) : "View the backing array as a slice in const context."
  - `fn slice_mut<T: Copy>(a: &mut Self::Array<T>) -> &mut [T];` (capacity.rs:112) : "View the backing array as a mutable slice in const context."
  - `impl<const N: usize> const ConstCapacity for Dim<N>` (capacity.rs:115-143)

enumerator.rs
- `pub trait Enumerator` (enumerator.rs:11) : "Parallel of `core::iter::Iterator::enumerate` that yields `USize`"
  - `type Item;` (enumerator.rs:12) : no doc
  - `fn enumerated(self) -> impl Iterator<Item = (USize, Self::Item)>;` (enumerator.rs:14) : "Yield `(index: USize, item)` pairs."
  - blanket `impl<T: IntoIterator> Enumerator for T` (enumerator.rs:17)

matrix.rs
- `pub struct Matrix<W: Copy, C: Capacity>` (matrix.rs:17) `#[repr(transparent)]` : "Dense `N × N` matrix over element type `W`." (private field `data: C::Array<C::Array<W>>`)
  - `pub const fn new(data: C::Array<C::Array<W>>) -> Self` (matrix.rs:25) : "Construct from a pre-built backing array-of-arrays."
  - `pub fn from_fn<F>(mut f: F) -> Self where F: FnMut(USize, USize) -> W` (matrix.rs:31) : "Construct by invoking `f(row, col)` for every cell."
  - `pub fn get(&self, i: USize, j: USize) -> W` (matrix.rs:41) : "Read the value at `(i, j)`."
  - `pub fn set(&mut self, i: USize, j: USize, v: W)` (matrix.rs:49) : "Write `v` to cell `(i, j)`."
  - `pub fn diagonal(&self) -> Array<W, C>` (matrix.rs:57) : "Extract the diagonal as an `Array<W, C>`."
  - in `impl ... where C::Array<W>: Copy`: `pub fn filled(v: W) -> Self` (matrix.rs:68) : "Build a Matrix populated with `v` in every cell."
  - trait impls: `Copy` where `C::Array<C::Array<W>>: Copy` (:73); `Clone` (:75)

### Stated behaviour
- Overflow, wrapping, saturating, rounding, strategy markers (`Hot`, `Warm`, `Cold`, `Precise`): not stated in the listed files of this crate; the crate does not use them in its sources. DESIGN.md.tmpl:8-9 mentions "the strategy markers used in trait bounds".
- Debug checks: array.rs:39 `debug_assert!(i.0 < cap_size(C::CAP), "Array::get: index out of range");` ; array.rs:46 `debug_assert!(i.0 < cap_size(C::CAP), "Array::set: index out of range");` ; matrix.rs:42-43 `debug_assert!(i.0 < cap_size(C::CAP), "Matrix::get: row index out of range");` and `debug_assert!(j.0 < cap_size(C::CAP), "Matrix::get: column index out of range");` ; matrix.rs:50-51 the `Matrix::set` equivalents.
- Named external standards: none in the listed files.
- DESIGN.md.tmpl:39-40: "Nightly `generic_const_exprs` requires a raw `usize` in const-generic array-length position (language grammar constraint)."
- DESIGN.md.tmpl:62-69: "Fixed-capacity storage where the capacity is a TYPE, not a `Cap` const generic."
- DESIGN.md.tmpl:114-118: "`Array`, `Matrix`, `Csr`, and `BitMatrix` now bind a `C: Capacity` type parameter in place of a `Cap` const generic, storing `C::Array<T>` rather than `[T; cap_size(N)]`."
- DESIGN.md.tmpl:129-132: "`Capacity` itself cannot be a `const trait`. Its `from_fn` takes a `FnMut(USize) -> T` closure and calls `core::array::from_fn` in its body, neither const-callable, and `impl const Trait` must provide a const body for every method."
- DESIGN.md.tmpl:203-206: "`get` / `set` take `USize`. The `.0` unwrap used for internal array indexing lives inside the method body, not at the call site."
- DESIGN.md.tmpl:281-291 "What this crate does NOT contain": "- Arithmetic operations on tensors (Add, Mul, Sqrt), consumer concern." "- Heap-backed growable shapes: no. Sizes are const."
- DESIGN.md.tmpl:283-285 consumer bounds: "arvo algorithm crates supply trait bounds (Add + TotalOrd + FromConstant + Copy) and compose with arvo-tensor types freely."
- Consumer expectation: capacity bound `C: Capacity` (DESIGN.md.tmpl:89 "consumers bind `C: Capacity` with no const param"); const consumers bind `C: [const] ConstCapacity` (DESIGN.md.tmpl:125).
- DESIGN.md.tmpl:6 vs lib.rs:3: doc text "`Array<T, N>` is rank-1, `Matrix<W, N>` is rank-2 N×N."

### Backlog and known gaps
BACKLOG.md.tmpl headings (no SHAME.md.tmpl in this crate's list):
- "# arvo-tensor, backlog" : "> **Type-surface reminder**: new entries use `arvo::USize` /" (BACKLOG.md.tmpl:3)
- "## Higher-rank shapes" : "Future rank-N containers as consumer demand surfaces:" (BACKLOG.md.tmpl:10)
- "## core::ops / Index impls" : "Adding `impl Index<USize> for Array<T, N>` and `impl Index<(USize, USize)>" (BACKLOG.md.tmpl:24)
- "## ArrayLen trait (generalised cap_size)" : "`cap_size(c: Cap) -> usize` is a single-purpose bridge." (BACKLOG.md.tmpl:31)
- "## Tensor trait surface" : "If a second rank-specific container surfaces (e.g. Grid) that shares" (BACKLOG.md.tmpl:40)
- "## Consumer migration" : "Each downstream sweep round (arvo-graph, arvo-sparse, arvo-spectral)" (BACKLOG.md.tmpl:48)

### Tests
- src/cap.rs: 1 (`cap_roundtrips_with_cap_size`, in `mod tests`)
- tests/array.rs: 9
- tests/capacity.rs: 4
- tests/const_capacity.rs: 4
- tests/container_capacity.rs: 2
- tests/enumerator.rs: 2
- tests/matrix.rs: 3
- Test files: array.rs, capacity.rs, const_capacity.rs, container_capacity.rs, enumerator.rs, matrix.rs

## arvo-transparent   (files read: 5)

- purpose: "The transparent-wrapper layer. Defines the trait every `repr(transparent)` arvo type implements to expose its inner container in const context, and the free function that consumes it. No fielded types. `#![no_std]`, no alloc." (mock/crates/arvo-transparent/DESIGN.md.tmpl:3-6)
- README: "The `Transparent` const unsafe trait and `NumericPrimitive` marker." (README.md.tmpl:3)
- depends on: Cargo.toml:7-8 `[dependencies] notko.workspace = true`; Cargo.toml:10-11 `[features] default = []`. DESIGN.md.tmpl:8: "Depends on `notko` only." src/lib.rs names only `core::mem::transmute_copy` (lib.rs:38).
- attributes: lib.rs:1 `#![no_std]`; lib.rs:2 `#![feature(const_trait_impl)]`; lib.rs:3 `#![allow(incomplete_features)]`.

### Public items

lib.rs
- `pub trait NumericPrimitive: Copy + Sized + 'static {}` (lib.rs:41) : "Marker for native primitives that arvo wrappers can transmute to."
  - impls for `u8`, `u16`, `u32`, `u64`, `usize`, `i8`, `i16`, `i32`, `i64`, `isize`, `bool` (lib.rs:43-53)
- `pub const unsafe trait Transparent: Copy + Sized` (lib.rs:67) : "Soundness contract for `repr(transparent)` arvo wrappers."
  - `type Inner: Copy;` (lib.rs:69) : "The single non-ZST field's type."
  - `fn raw(self) -> Self::Inner` (lib.rs:73, default body `unsafe { transmute_copy::<Self, Self::Inner>(&self) }`) : "Read the inner value via layout-equivalent transmute."
- `pub const trait ConstDeref` (lib.rs:91) : "Const-callable mirror of `core::ops::Deref`."
  - `type Target: ?Sized;` (lib.rs:93) : "Deref target."
  - `fn const_deref(&self) -> &Self::Target;` (lib.rs:95) : "Const-callable deref."
- `pub const trait ConstAsRef<T: ?Sized>` (lib.rs:102) : "Const-callable mirror of `core::convert::AsRef<T>`."
  - `fn const_as_ref(&self) -> &T;` (lib.rs:104) : "Const-callable as-ref."
- `pub const fn raw<W, T>(w: W) -> T where W: [const] Transparent<Inner = T> + Copy, T: NumericPrimitive` (lib.rs:119) : "Free-fn form of [`Transparent::raw`] for prefix-style call sites."

### Stated behaviour
- Overflow, wrapping, saturating, rounding, strategy markers, named external standards: not stated in the listed files.
- lib.rs:30-36: "[`Transparent`] is `unsafe`: implementors guarantee `#[repr(transparent)]` over the declared `Inner`."
- lib.rs:35-36: "Implementing `Transparent` for a non-transparent type is undefined behaviour."
- lib.rs:26: "All three collapse to the same `transmute_copy` at codegen."
- lib.rs:64-66: "Per round 202604271346 D-12, this trait is `pub const unsafe`, so the `raw()` method is callable in const context. Impl blocks are `unsafe impl const Transparent for X`."
- DESIGN.md.tmpl:19-22: "`pub const unsafe trait`: the `unsafe` reflects the `repr(transparent)` soundness obligation (the implementor must guarantee the layout is identical to `Inner`). The `const` reflects that the default `raw()` body uses `transmute_copy` and is callable in const context."
- DESIGN.md.tmpl:41-45: "Clean marker. Identifies the bottom-out primitives that arvo wrappers ultimately transmute to: `u8`, `u16`, `u32`, `u64`, `usize`, `bool`, `i8`, `i16`, `i32`, `i64`, `isize`. No methods."
- DESIGN.md.tmpl:47-49: "Per D-11, this returns to its clean marker-only role. No `from_u64_masked`, `to_u64`, or per-size method on this trait."
- DESIGN.md.tmpl:79: "The transparent-newtype invariant means `const_deref` and `const_as_ref` lower to a no-op pointer reinterpretation at codegen."
- DESIGN.md.tmpl:81: "Round 4 (#314) ships the trait declarations."
- Consumer bound: lib.rs:115-117: "The `[const]` bound on `Transparent` requires call sites to provide a `const`-callable impl (every arvo wrapper carries `unsafe impl const Transparent` per the macro DRY)." DESIGN.md.tmpl:79: "consumers (notably arvo-storage's `Bits` and arvo's `UFixed` / `IFixed`) impl them through forward composition."

### Backlog and known gaps
- "# arvo-transparent. Backlog" : "(none currently)" (BACKLOG.md.tmpl:3)
- "## Removed entries" : "(none)" (BACKLOG.md.tmpl:7)
- No SHAME.md.tmpl in this crate's list.

### Tests
- No `#[test]` functions and no test files in this crate's list.

## arvo   (files read: 59)

- purpose: "The downstream facade of the arvo numeric stack." (README.md.tmpl:3); "**arvo is a fixed-point arithmetic stack.** Standalone `#![no_std]`," (DESIGN.md.tmpl:3)
- depends on: Cargo.toml dependencies, all `.workspace = true`: notko, arvo-transparent, arvo-strategy, arvo-storage, arvo-bits-contracts, arvo-numeric-contracts, arvo-bits. Dev-dependency: `trybuild = "1"`. Features: `default = []`. build.rs:10 emits `println!("cargo::rustc-check-cfg=cfg(arvo_fast_math)");` (cfg, not a cargo feature). Cargo.toml description: "Numeric and analysis primitives with strategy tags dictating speed, precision and layout per call site. no_std, no alloc, no platform deps." (Cargo.toml:6)
- attributes (lib.rs:14-27): `#![no_std]`, `#![feature(adt_const_params)]`, `#![feature(const_trait_impl)]`, `#![feature(const_ops)]`, `#![feature(generic_const_exprs)]`, `#![feature(macro_metavar_expr_concat)]`, `#![allow(incomplete_features)]`.

### Public items
lib.rs modules: `pub mod aliases;` (29), `pub mod bitfield;` (30), `pub mod float;` (32), `pub mod ifixed;` (33), `pub mod ifixed_impl;` (34), `pub mod markers;` (36), `pub mod predicate;` (37), `pub mod prim;` (38), `pub mod strategy;` (39), `pub mod traits;` (40), `pub mod transparent;` (41), `pub mod ufixed;` (42), `pub mod ufixed_impl;` (43). Not public: `pub(crate) mod fixed_scale;` (31), `mod layout_assertions;` (35).

lib.rs re-exports (lib.rs:45-70), verbatim:
- `pub use aliases::{Fixed, Int, Signed, Uint};`
- `pub use arvo_bits::{ Bit, BitsRefitCtor, Byte, DWord, Narrow, Narrowed, Nibble, QWord, Widen, Widened, Word, };`
- `pub use arvo_numeric_contracts::{ IsNonNegative, IsNonZero, IsPositive, IsZero, IsZeroOrPositive, Predicate, };`
- `pub use arvo_storage::{ fbits, ibits, width, AsBool, Bits, Bool, Cap, FBits, IBits, NUSize, USize, Width, };`
- `pub use float::{FastFloat, Float, StrictFloat};`, `pub use ifixed::IFixed;`, `pub use markers::{BitPresentation, BoolLike, FloatLike, FractionLike, IntegerLike};`, `pub use notko::{Just, Maybe, Outcome};`, `pub use predicate::{Pred, Pred2, Pred3};`, `pub use prim::{BitPrim, IBitContainer, IBitPrim, UBitContainer};`
- `pub use strategy::{ tag_one_representable, width_le_64, Additive, Bounded, Cold, ConstBitEq, ConstDefault, ConstEq, ConstOrd, ConstOrdering, ConstPartialEq, FromU8Ieee, Hot, Identity, Ieee, Multiplicative, OneRepresentable, Picker, Precise, SignedIdentity, Signedness, Strategy, Unsigned, Warm, };`
- `pub use traits::{ Abs, EuclidDiv, EvenShares, EvenSplittable, FromConstant, Recip, ScalarEuclid, Sqrt, TotalOrd, };`, `pub use transparent::{raw, NumericPrimitive, Transparent};`, `pub use ufixed::UFixed;`
- strategy.rs:19-27 `pub use arvo_storage::{...}` and `pub use arvo_strategy::{...}` lists; prim.rs:14 `pub use arvo_bits_contracts::{BitPrim, IBitContainer, IBitPrim, UBitContainer};`; transparent.rs:8 `pub use arvo_transparent::{raw, NumericPrimitive, Transparent};`; traits/mod.rs:29 `pub use arvo_numeric_contracts::{Abs, FromConstant, Recip, Sqrt, TotalOrd};`, traits/mod.rs:30 `pub use euclid::{EuclidDiv, EvenShares, EvenSplittable, ScalarEuclid};`; traits/from_constant.rs:10 `pub use arvo_numeric_contracts::FromConstant;`; traits/total_ord.rs:12 `pub use arvo_numeric_contracts::TotalOrd;`; traits/sqrt.rs:9 `pub use arvo_numeric_contracts::Sqrt;`.

aliases.rs:
- `pub type Fixed<const I: u16, const F: u16, S = Warm> = UFixed<{ ibits(I) }, { fbits(F) }, S>;` (aliases.rs:45) : "```ignore"
- `pub type Signed<const I: u16, const F: u16, S = Warm> = IFixed<{ ibits(I) }, { fbits(F) }, S>;` (aliases.rs:57) : "```ignore"
- `pub type Uint<const N: u16, S = Warm> = UFixed<{ ibits(N) }, { fbits(0u16) }, S>;` (aliases.rs:71) : "```ignore"
- `pub type Int<const N: u16, S = Warm> = IFixed<{ ibits(N - 1) }, { fbits(0u16) }, S>;` (aliases.rs:85) : "```ignore"

markers.rs:
- `pub const trait IntegerLike: Copy {}` (markers.rs:23) : "UFixed / IFixed with `F = 0`. No fractional part."
- `pub const trait FractionLike: Copy {}` (markers.rs:28) : "UFixed / IFixed with `F > 0`."
- `pub const trait BitPresentation: Copy` (markers.rs:36) : "Every UFixed and IFixed implements this. `LOGICAL_WIDTH` is the"
  - `const LOGICAL_WIDTH: USize;` (markers.rs:41) : "Logical bit width of this type."
- `pub const fn logical_width_unsigned(i: IBits, f: FBits) -> USize` (markers.rs:48) : "Compose `LOGICAL_WIDTH` for `UFixed<I, F, S>`. Round 202605030400"
- `pub const fn logical_width_signed(i: IBits, f: FBits) -> USize` (markers.rs:55) : "Compose `LOGICAL_WIDTH` for `IFixed<I, F, S>` (1 sign bit + I + F)."
- `pub const trait FloatLike: Copy {}` (markers.rs:65) : "`FastFloat` and `StrictFloat`. These do not carry a `Strategy`"
- `pub const trait BoolLike: Copy` (markers.rs:72) : "Currently `Bool`. `pack()` converts a control-flow `Bool` into"
  - `type Packed: [const] BitPresentation;` (markers.rs:78)
  - `fn pack(self) -> Self::Packed;` (markers.rs:81)

strategy.rs (plus the re-export lists above):
- `pub const fn ufixed_bits(i: IBits, f: FBits) -> u16` (strategy.rs:39) : "`UFixed` logical width is `I + F`. The trait bound uses this to"
- `pub const fn ifixed_bits(i: IBits, f: FBits) -> u16` (strategy.rs:48) : "`IFixed` reserves one bit for the sign; logical width is `1 + I + F`."
- `pub const fn is_fractional(f: FBits) -> usize` (strategy.rs:57) : "Used to express "F has a fractional component" in const-generic" (first doc line at strategy.rs:~52 is above; the quoted line is the doc line immediately before `#[inline(always)]`)
- `pub const fn width_le_64(n: arvo_storage::Width) -> bool` (strategy.rs:72) : "Predicate, not an accessor; an accessor of `Width` to its inner"

predicate.rs:
- `pub trait Pred<A>: Fn(&A) -> Bool` (predicate.rs:16) : "Unary predicate. Equivalent to `impl Fn(&A) -> Bool`."
  - `fn test(&self, a: &A) -> Bool` (predicate.rs:19)
- `pub trait Pred2<A, B>: Fn(&A, &B) -> Bool` (predicate.rs:26) : "Binary predicate. Equivalent to `impl Fn(&A, &B) -> Bool`."
  - `fn test(&self, a: &A, b: &B) -> Bool` (predicate.rs:29)
- `pub trait Pred3<A, B, C>: Fn(&A, &B, &C) -> Bool` (predicate.rs:36) : "Ternary predicate. Equivalent to `impl Fn(&A, &B, &C) -> Bool`."
  - `fn test(&self, a: &A, b: &B, c: &C) -> Bool` (predicate.rs:39)

float.rs:
- `pub struct FastFloat<F: Ieee>(pub F);` (float.rs:32) : "Operations on `FastFloat<F>` may be reassociated, may use"
  - `pub const fn new(f: F) -> Self` (float.rs:46) : "Wrap an IEEE float in fast-math semantics."
  - `pub const fn into_inner(self) -> F` (float.rs:52)
- `pub struct StrictFloat<F: Ieee>(pub F);` (float.rs:41) : "Bit-exact, NaN-propagating, order-preserving. Used where" (the doc line above it, float.rs:~38, reads "Strict IEEE 754 float wrapper.")
  - `pub const fn new(f: F) -> Self` (float.rs:60)
  - `pub const fn into_inner(self) -> F` (float.rs:66)
- `pub type Float<F = f32> = FastFloat<F>;` (float.rs:220, `#[cfg(arvo_fast_math)]`) : "`FastFloat<F>` when the `arvo_fast_math` cfg is active;"
- `pub type Float<F = f32> = StrictFloat<F>;` (float.rs:227, otherwise) : "`StrictFloat<F>` outside the `arvo_fast_math` cfg. The default"
- core::ops Add, Sub, Mul, Div, Neg impls for both wrappers by `macro_rules! float_binop_impl` (float.rs:179) and `macro_rules! float_neg_impl` (float.rs:191), private macros.

ifixed.rs:
- `pub struct IFixed<const I: IBits, const F: FBits, S: Strategy = Warm>( Bits<{ ifixed_bits(I, F) }, S, Signed>, ) where S: BitsContainerFor<{ ifixed_bits(I, F) }, Signed>;` (ifixed.rs:43) : "`I` = integer bits (magnitude), `F` = fractional bits, `S` ="
  - `pub const fn from_raw(bits: <S as BitsContainerFor<{ ifixed_bits(I, F) }, Signed>>::T) -> Self` (ifixed.rs:234)
  - `pub const fn to_raw(self) -> <S as BitsContainerFor<{ ifixed_bits(I, F) }, Signed>>::T` (ifixed.rs:240)
  - `pub const fn logical_width() -> USize` (ifixed.rs:246)
- Trait impls on `IFixed` in ifixed.rs: Identity<Additive>, Identity<Multiplicative> (gated by `Picker: OneRepresentable<{ tag_one_representable(I.raw()) }>`), SignedIdentity, Bounded, ConstPartialEq, ConstEq, ConstBitEq, ConstOrd, ConstDefault, Copy, Clone, PartialEq, Eq, Default, Debug, BitPresentation, IntegerLike, FractionLike, Add, Sub, Mul, Div, From and TryFrom between strategies.

ufixed.rs:
- `pub struct UFixed<const I: IBits, const F: FBits, S: Strategy = crate::strategy::Warm>( Bits<{ ufixed_bits(I, F) }, S>, ) where S: BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>;` (ufixed.rs:35) : "`I` = integer bits, `F` = fractional bits, `S` = strategy"
  - `pub const fn from_raw(` (ufixed.rs:188), `pub const fn to_raw(self) -> <S as BitsContainerFor<{ ufixed_bits(I, F) }, Unsigned>>::T` (ufixed.rs:196), `pub const fn logical_width() -> USize` (ufixed.rs:202)
- Same trait-impl family as IFixed, without SignedIdentity.

ifixed_impl.rs, ufixed_impl.rs: no `pub` items; impls of `HasBitWidth`, `BitAccess`, `BitSequence` for `IFixed` and `UFixed` (ufixed_impl.rs:16, 23, 60), and `BitLogic` for `UFixed<I, F, Hot>` (ufixed_impl.rs:111). Module doc ufixed_impl.rs:1: "`HasBitWidth` / `BitAccess` / `BitSequence` impls on `UFixed<I, F, S>`."

bitfield.rs (`#[macro_export]`):
- `macro_rules! __bitfield_container_ty` (bitfield.rs:61) : "Internal helper: yields the concrete container primitive type for" (`#[doc(hidden)]`)
- `macro_rules! bitfield` (bitfield.rs:275) : "Generates a bitfield struct wrapping `Bits<N, S>` with"
- `macro_rules! __bitfield_impl` (bitfield.rs:323) : "Internal expansion shared between the default and explicit-strategy" (`#[doc(hidden)]`)
- generated inherent items: `pub const BITS: u16 = $n;` (389), `pub const fn new() -> Self` (392), `pub const fn from_bits(raw: $crate::Bits<$n, $strategy>) -> Self` (398), `pub const fn to_bits(self) -> $crate::Bits<$n, $strategy>` (404), `pub const ${concat($field, _MASK)}: $crate::Bits<$n, $strategy>` (410), `pub const fn $field(self) -> $crate::Bits<$field_bits, $strategy>` (423), `pub const fn ${concat(with_, $field)}(` (436).

traits/ :
- `pub trait ScalarEuclid: Sized` (euclid.rs:32) : "`div_euclid_scalar` / `rem_euclid_scalar` divide at ULP granularity:"
  - `fn div_euclid_scalar(self, n: USize) -> Self;`, `fn rem_euclid_scalar(self, n: USize) -> Self;`
- `pub trait EuclidDiv: Sized` (euclid.rs:43) : "The quotient is floored to whole integer units (the coarser,"
  - `fn div_euclid(self, rhs: Self) -> Self;`, `fn rem_euclid(self, rhs: Self) -> Self;`
- `pub struct EvenShares<T>` (euclid.rs:55) : "Yields `base + 1 ULP` for the first `r` shares (the remainder"
  - `type Item = T;` and `fn next(&mut self) -> Option<T>` (Iterator impl)
- `pub trait EvenSplittable: Sized` (euclid.rs:83) : "Split a value into `n` shares that sum to it exactly."
  - `fn split_evenly(self, n: USize) -> EvenShares<Self>;`
- `pub const fn abs_f32(x: f32) -> f32` (abs.rs:169) : no doc
- `pub const fn abs_f64(x: f64) -> f64` (abs.rs:174) : no doc
- `pub const fn sqrt_f32(x: f32) -> f32` (sqrt.rs:75) : no doc
- `pub const fn sqrt_f64(x: f64) -> f64` (sqrt.rs:92) : no doc
- Impl-by-macro (private macros): `impl_from_constant_ufixed!`/`impl_from_constant_ufixed_fractional!`/`impl_from_constant_ifixed!`/`impl_from_constant_ifixed_fractional!` (from_constant.rs:20, 34, 118, 131) generate `FromConstant` for `UFixed`/`IFixed` per (strategy, I); `impl_sqrt_ufixed_concrete!` (sqrt.rs:21) generates `Sqrt` for `UFixed<{ ibits($i) }, { FBits::ZERO }, $strategy>` at Hot 1..=64, Cold 1..=64, Warm 1..=32, Precise 1..=64; `const impl` of `Sqrt`, `TotalOrd`, `Recip`, `Abs`, `FromConstant` for `FastFloat<f32|f64>` and `StrictFloat<f32|f64>`; `const impl TotalOrd for UFixed<I, F, S>` and `IFixed<I, F, S>` (total_ord.rs:60, 71).
- Private helpers: `const fn total_cmp_f32`, `total_cmp_f64`, `const_ordering_to_core` (total_ord.rs:29, 46, 87).

### Stated behaviour
- "**arvo is a fixed-point arithmetic stack.** Standalone `#![no_std]`, no alloc, const-generic sizing." (DESIGN.md.tmpl:3-4)
- "The `Int<const N: u16, S>` / `Uint<const N: u16, S>` semantic convention: **N is total storage bit count.**" (DESIGN.md.tmpl:55-56)
- "The cap on UFixed / IFixed (and so on `Uint<N, S>` / `Int<N, S>`) is `N <= 128`" (DESIGN.md.tmpl:62-63)
- "**Bound:** `I + F ≤ 32`. Warm's value proposition is the 2x container safety guarantee on single ops;" (DESIGN.md.tmpl:857-858)
- "guarantee. Warm is a compile-time error at `I + F > 32`, pick Hot (wrapping, accepts overflow risk) or Precise (saturating, carries correctness) instead." (DESIGN.md.tmpl:860-862, line breaks removed between 861 and 862 only in layout)
- Container width table (DESIGN.md.tmpl:934-943): "| 1-8 | u8 | u16 | u8 (bitpacked at access) | u16 |", "| 33-64 | u64 | u128 | u64 (bitpacked at access) | u128 (saturating) |".
- Strategy comparison (DESIGN.md.tmpl:962-976): "| Add/Sub | wrapping | wrapping (safe 1-op) | widen-op-narrow | saturating |", "| Multiply | wrapping | wrapping (safe 1-op) | widen-op-narrow | widening + saturate |", "| Div-by-zero | numerator | numerator | numerator | max |", "| Chain safety | overflows | overflows on long chains | overflows on long chains | clamps always |", "| Precision loss | yes (wrapping + narrow) | no (single ops) | no (single ops) | never |".
- Float wrappers (DESIGN.md.tmpl:1093-1103): "`FastFloat`: wraps f32/f64 with LLVM fast-math intrinsics enabled. Trades IEEE 754 compliance for speed (reassociation, reciprocal approximation, no NaN propagation)." and "`StrictFloat`: wraps f32/f64 with strict IEEE 754 semantics. No fast-math. For validation, comparison, and contexts where bit-exact reproducibility matters." Named standard: IEEE 754. No MATLAB `fi` or Q format named in DESIGN.md.tmpl (grep for "MATLAB" and "Q format" returned no lines).
- "`StrictFloat::sqrt` is currently a 7-ULP Newton-Raphson approximation in no_std (arvo has no libm dependency)." (DESIGN.md.tmpl:1169-1170)
- "# WARNING: 7-ULP Newton-Raphson approximation" (sqrt.rs:60); "but not correctly-rounded" (sqrt.rs:64).
- Wrapping division: "returns the numerator unchanged when the divisor is zero." (BACKLOG.md.tmpl:427)
- tests/strategy_semantics.rs:3-7: "- Hot / Warm: wrapping (single-op container overflow wraps modulo container width)." and "- Precise: saturating (clamps to container MAX / MIN on overflow, clamps on div-by-zero)."
- tests/fixed_point_div.rs:3-4: "Truncates toward zero (the natural integer-`/` rounding, a documented asymmetry with the multiply's floor toward minus infinity)."
- tests/fixed_point_mul.rs:175: "The rescale is an arithmetic right shift, so negative results floor toward minus infinity (not toward zero)."
- tests/fixed_point_mul.rs:259-260: "Precise saturates at the LOGICAL bound, not the container bound".
- tests/strategy_wrapping_div_zero.rs:3-6: "Pins the post-review fix (F1) semantic: Hot / Warm / Cold `u_div` and `i_div` MUST NOT panic when `b == 0`."
- tests/ifixed_ops.rs:44 and 52: "Hot uses wrapping_abs: i8::MIN.wrapping_abs() = i8::MIN." and "Precise uses saturating_abs".
- Identity gating: tests/identity_laws.rs:19-21: "Shapes with no integer bit have no multiplicative identity at all; that refusal is a compile error"; tests/ui/no_signed_identity_on_unsigned.rs:1: "`UFixed` has no `SignedIdentity`, at any width or strategy."
- Debug-assertion or overflow-check statements: none found in the arvo docs read beyond those above.
- Consumer selection: DESIGN.md.tmpl:857-863 says to pick Hot or Precise instead of Warm above `I + F > 32`; DESIGN.md.tmpl:1169-1174: "Consumer code that needs correctly-rounded sqrt should either link libm at its own layer or wait for arvo BACKLOG item to ship."
- Other docs in the crate: DEEPDIVE_newtype-deref.md.tmpl (opens "arvo wraps several Rust primitives in newtypes to enforce semantic meaning and satisfy the arvo-types-only lint:", line 5) and DEEPDIVE_strategy-bound-trilemma.md.tmpl (title "# arvo. Deep Dive: The Strategy-Bound Trilemma", line 1).

### Backlog and known gaps
BACKLOG.md.tmpl headings, each with the first line beneath it (BACKLOG.md.tmpl line given):
- "# arvo. backlog" : "> **Type-surface reminder**: new entries use `arvo::USize` /" (3)
- "## Debug on UFixed and IFixed (shipped in round 202605250730)" : "Task #428 was a rolling sweep adding Debug derives across the arvo numeric type family." (10)
- "## Closeout, deferred bridges" : "Round 1 of the post-202605021600 core-fundamentals work" (14)
- "## LogicalBounds trait" : "Today `<UFixed<I, F, S> as Bounded>::MIN/MAX` projects through the" (31)
- "## NBits<N>: niche-filled Bits variant" : "Maybe of NBits niche-optimises to size_of of Bits, giving "optional" (46)
- "## Const-trait family forward-looking entries" : "The const-traits-and-arvo-as-facade restructure parks several" (58)
- "## Substrate-completion forward-looking entries" : "Audit-driven follow-up chain Round A. Several items deferred to the" (74)
- "## IFixed reshape forward-looking entries" : "The original Round B forward-looking entries pointed at MultiContainer" (136)
- "## Cycle-workaround forward-looking entries" : "Audit-driven follow-up chain Round C. Pass 2 carrier-widening (via" (174)
- "## Currently forward-looking entries" (line 204) : "Audit-driven follow-up chain Round D-aux. Layout-soundness gates for" (206)
- "## Currently forward-looking entries" (line 228) : "Audit-driven follow-up chain Round D consolidated. Seven passes ship" (230)
- "## Q-D completion" : "The std-parallel canonical aliases `Uint8` / `Uint16` / `Uint32` /" (262)
- "## MaskOps trait impls (SHIPPED)" : "The `arvo-mask-contracts::Mask<const W: u8>` const trait was declared" (271)
- "## Cold widen / narrow impls (deferred)" : "This removed the `_ColdReserved` workaround in" (286)
- "## ConstParamTy on UFixed / IFixed" : "UFixed and IFixed do NOT derive ConstParamTy." (308)
- "## Semantic domain aliases (researched widths)" : "Shipped historically by downstream consumers; dropped from arvo L0" (337)
- "## Deferred newtypes" : "Decided to live in consuming layers, not L0:" (361)
- "## Cross-width and cross-strategy arithmetic" : "The L0 implementation ships same-strategy same-width arithmetic only." (376)
- "## Numeric trait impls deferred beyond L0" : "- `Sqrt` / `Recip` for fractional `UFixed` / `IFixed` need" (386)
- "## Benchmark round" : "Exhaustive comparison of arvo types vs raw `f32` / `f64` / `u32` /" (394)
- "## Post-review notes" : "Items surfaced during a post-shipping review of arvo; kept" (403)
- No SHAME.md.tmpl in this crate's list. In-source markers: tests/fixed_point_div.rs:105 "// Catalogue red (tracked task #5)" with `#[ignore = "catalogue: >64-bit-logical fixed-point divide needs 256/128 long division; tracked #5"]` (tests/fixed_point_div.rs:111); tests/cross_width.rs:3 "Cross-width arithmetic is deferred (see ufixed.rs / ifixed.rs TODOs)".

### Tests
Embedded `#[test]` in src: traits/euclid.rs 11. No other src file has `#[test]`.
Integration tests (`#[test]` count per file, tests/):
- bitfield_const_eq_default.rs 2
- cross_width.rs 18
- fixed_point_div.rs 13 (one of them `#[ignore]`)
- fixed_point_mul.rs 25
- float_const_arith.rs 3
- float_ops.rs 20
- identity_laws.rs 4 literal `#[test]` attributes in the file (3 of them inside `macro_rules!` bodies: `both_identities!`, `neg_one_law!`, `neg_one_cancels_one!`, `additive_only!`, each expanded per shape and strategy; the grep count is 4)
- ifixed_const_arith.rs 3
- ifixed_ops.rs 9
- logical_width_const_probe.rs 2
- no_multiplicative_identity.rs 1 (a `trybuild` driver over tests/ui/*.rs)
- numeric_contracts_const_probe.rs 2
- predicate_family_const_probe.rs 2
- strategy_semantics.rs 12
- strategy_wrapping_div_zero.rs 6
- ufixed_const_arith.rs 3
- ufixed_ops.rs 8
- warm_bound_positive.rs 2
tests/ui (compile-fail inputs, no `#[test]`): no_multiplicative_identity.rs, no_multiplicative_identity_signed_cold.rs, no_multiplicative_identity_signed_hot.rs, no_multiplicative_identity_signed_precise.rs, no_multiplicative_identity_signed_warm.rs, no_multiplicative_identity_unsigned_cold.rs, no_multiplicative_identity_unsigned_precise.rs, no_multiplicative_identity_unsigned_warm.rs, no_signed_identity_on_unsigned.rs.
Total `#[test]` functions grep count over the crate: 146 (the `identity_laws.rs` count of 4 is the number of attribute lines, not the expanded count).

<!-- END -->
