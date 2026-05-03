//! Sketch 02 provider: BitsContainerFor + Project + NarrowFromU64 + ConstHash.
//!
//! Mirrors the substrate's Pattern C dispatch + the round 4 trait additions.
//! The consumer crate calls into ConstHash::hash_const through method
//! dispatch to validate the trait-solver chain crosses a crate boundary.

#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features, dead_code)]
#![no_std]

use core::marker::ConstParamTy;

mod sealed { pub trait Sealed {} }

#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Hot;
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Warm;
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Cold;
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Precise;

pub trait Strategy: sealed::Sealed + Copy + Clone + Default + 'static {}
impl sealed::Sealed for Hot {} impl Strategy for Hot {}
impl sealed::Sealed for Warm {} impl Strategy for Warm {}
impl sealed::Sealed for Cold {} impl Strategy for Cold {}
impl sealed::Sealed for Precise {} impl Strategy for Precise {}

#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Unsigned;
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct Signed;

pub trait Signedness: sealed::Sealed + Copy + Clone + Default + 'static {}
impl sealed::Sealed for Unsigned {} impl Signedness for Unsigned {}
impl sealed::Sealed for Signed {} impl Signedness for Signed {}

// ---------------------------------------------------------------------------
// Pattern C dispatch (same shape as arvo-strategy/src/container.rs).
// ---------------------------------------------------------------------------

pub const fn tag_hot_cold(n: u16) -> usize {
    let n = n as usize;
    if n <= 8 { 0 }
    else if n <= 16 { 1 }
    else if n <= 32 { 2 }
    else if n <= 64 { 3 }
    else if n <= 128 { 4 }
    else { 5 }
}

pub const fn bytes_for_u16(n: u16) -> usize {
    (n as usize).div_ceil(8)
}

pub trait Project<const TAG: usize, Sign: Signedness, const BYTES: usize, S: Strategy>: sealed::Sealed {
    type T: Copy + Clone + 'static;
}

// Picker stays pub because it appears in the public associated type of
// `BitsContainerFor`. Sealing `Project` is what prevents downstream
// consumers from adding impls; `Picker`-the-type being public is fine
// because it carries no useful operations of its own.
pub struct Picker;
impl sealed::Sealed for Picker {}

impl<const BYTES: usize> Project<0, Unsigned, BYTES, Hot> for Picker { type T = u8; }
impl<const BYTES: usize> Project<1, Unsigned, BYTES, Hot> for Picker { type T = u16; }
impl<const BYTES: usize> Project<2, Unsigned, BYTES, Hot> for Picker { type T = u32; }
impl<const BYTES: usize> Project<3, Unsigned, BYTES, Hot> for Picker { type T = u64; }
impl<const BYTES: usize> Project<4, Unsigned, BYTES, Hot> for Picker { type T = u128; }

pub const trait BitsContainerFor<const N: u16, Sign: Signedness>: Strategy {
    type T: Copy + Clone + 'static;
}

impl<const N: u16, Sign: Signedness> const BitsContainerFor<N, Sign> for Hot
where
    Picker: Project<{ tag_hot_cold(N) }, Sign, { bytes_for_u16(N) }, Hot>,
{
    type T = <Picker as Project<{ tag_hot_cold(N) }, Sign, { bytes_for_u16(N) }, Hot>>::T;
}

// ---------------------------------------------------------------------------
// NarrowFromU64.
// ---------------------------------------------------------------------------

pub const trait NarrowFromU64<const N: u16, S: Strategy, Sign: Signedness>: Sized {
    fn narrow_u64(raw: u64) -> Self;
}

const fn mask_low(n: u16) -> u64 {
    if n >= 64 { u64::MAX } else { (1u64 << n) - 1 }
}

impl<const N: u16> const NarrowFromU64<N, Hot, Unsigned> for u8 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u8 }
}
impl<const N: u16> const NarrowFromU64<N, Hot, Unsigned> for u16 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u16 }
}
impl<const N: u16> const NarrowFromU64<N, Hot, Unsigned> for u32 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u32 }
}
impl<const N: u16> const NarrowFromU64<N, Hot, Unsigned> for u64 {
    fn narrow_u64(raw: u64) -> Self { raw & mask_low(N) }
}
impl<const N: u16> const NarrowFromU64<N, Hot, Unsigned> for u128 {
    fn narrow_u64(raw: u64) -> Self { (raw & mask_low(N)) as u128 }
}

// ---------------------------------------------------------------------------
// Bits storage (transparent over the dispatched container).
// ---------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Bits<const N: u16, S: Strategy = Hot, Sign: Signedness = Unsigned>
where
    S: BitsContainerFor<N, Sign>,
{
    raw: <S as BitsContainerFor<N, Sign>>::T,
}

impl<const N: u16, S: Strategy, Sign: Signedness> Bits<N, S, Sign>
where
    S: BitsContainerFor<N, Sign>,
{
    pub const fn from_raw(raw: <S as BitsContainerFor<N, Sign>>::T) -> Self {
        Self { raw }
    }
    pub const fn raw(self) -> <S as BitsContainerFor<N, Sign>>::T { self.raw }
}

// ---------------------------------------------------------------------------
// ConstHash.
// ---------------------------------------------------------------------------

pub const trait ConstHash<const N: u16, S: Strategy, Sign: Signedness>: Sized
where
    S: BitsContainerFor<N, Sign>,
    <S as BitsContainerFor<N, Sign>>::T: [const] NarrowFromU64<N, S, Sign>,
{
    fn hash_const(bytes: &[u8]) -> Bits<N, S, Sign>;
}

// FNV-1a-64 free const fn.
pub const fn fnv1a_64(bytes: &[u8]) -> u64 {
    const OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x100_0000_01b3;
    let mut hash: u64 = OFFSET_BASIS;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(PRIME);
        i += 1;
    }
    hash
}

pub struct Fnv1a<const N: u16>
where
    Hot: BitsContainerFor<N, Unsigned>;

impl<const N: u16> const ConstHash<N, Hot, Unsigned> for Fnv1a<N>
where
    Hot: BitsContainerFor<N, Unsigned>,
    <Hot as BitsContainerFor<N, Unsigned>>::T: [const] NarrowFromU64<N, Hot, Unsigned>,
{
    fn hash_const(bytes: &[u8]) -> Bits<N, Hot, Unsigned> {
        let raw = fnv1a_64(bytes);
        let narrowed = <<Hot as BitsContainerFor<N, Unsigned>>::T as NarrowFromU64<N, Hot, Unsigned>>::narrow_u64(raw);
        Bits::from_raw(narrowed)
    }
}
