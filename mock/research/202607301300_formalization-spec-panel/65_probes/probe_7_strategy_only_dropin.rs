//! Probe 7. Can `arvo-strategy`'s gate come off WITHOUT touching the facade?
//!
//! The 2026-07-28 sketch's shape takes the width as a TYPE
//! (`BitsContainerFor<Wid<13>, Unsigned>`), which changes the trait's own
//! signature and so propagates to `Bits` and to the facade. This probe asks
//! whether the signature can stay exactly as shipped
//! (`BitsContainerFor<const N: Width, Sign>`) with `Wid<N>` used only INSIDE
//! the impl, where `N` is a standalone argument and needs no feature.
//!
//! If yes, the two gates are independent problems with independent prices,
//! and the cheap one is genuinely cheap.
#![no_std]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
use core::marker::ConstParamTy;

#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Width(pub u16);

pub trait Signedness {}
pub struct Unsigned;
impl Signedness for Unsigned {}
pub struct Signed;
impl Signedness for Signed {}
pub trait Strategy {}
pub struct Hot;
impl Strategy for Hot {}
pub struct Warm;
impl Strategy for Warm {}

// -- the bucket vocabulary, as the sketch has it ---------------------------
pub struct B8;
pub struct B16;
pub struct B32;
pub struct B64;
pub struct B128;
pub struct BWide<const BYTES: usize>;
pub trait Bucket {}
impl Bucket for B8 {}
impl Bucket for B16 {}
impl Bucket for B32 {}
impl Bucket for B64 {}
impl Bucket for B128 {}
impl<const BYTES: usize> Bucket for BWide<BYTES> {}

pub trait Family {}
pub struct HotCold;
impl Family for HotCold {}
pub struct WarmPrecise;
impl Family for WarmPrecise {}

// Width-as-typestate, INTERNAL to the crate. Never appears in a signature.
pub struct Wid<const N: Width>;
pub trait WidthFor<F: Family> {
    type Bkt: Bucket;
}
macro_rules! widths {
    ($fam:ty, $( $n:literal => $bkt:ty ),* $(,)?) => {
        $( impl WidthFor<$fam> for Wid<{ Width($n) }> { type Bkt = $bkt; } )*
    };
}
widths!(HotCold, 1 => B8, 8 => B8, 9 => B16, 16 => B16, 17 => B32, 32 => B32,
                 33 => B64, 64 => B64, 65 => B128, 128 => B128);
widths!(WarmPrecise, 1 => B16, 8 => B16, 9 => B32, 16 => B32, 17 => B64, 32 => B64,
                     33 => B128, 64 => B128);
// wide bucket carries its own byte count, one row per width as the sketch says
impl WidthFor<HotCold> for Wid<{ Width(129) }> {
    type Bkt = BWide<17>;
}
impl WidthFor<WarmPrecise> for Wid<{ Width(65) }> {
    type Bkt = BWide<9>;
}

pub trait Project<B: Bucket, Sign: Signedness, S: Strategy> {
    type T: Copy;
}
pub struct Picker;
macro_rules! project {
    ($s:ty, $( $b:ty => ($u:ty, $i:ty) ),* $(,)?) => { $(
        impl Project<$b, Unsigned, $s> for Picker { type T = $u; }
        impl Project<$b, Signed,   $s> for Picker { type T = $i; }
    )* };
}
project!(Hot, B8 => (u8, i8), B16 => (u16, i16), B32 => (u32, i32),
              B64 => (u64, i64), B128 => (u128, i128));
project!(Warm, B16 => (u16, i16), B32 => (u32, i32),
               B64 => (u64, i64), B128 => (u128, i128));
pub struct WideStore<const BYTES: usize>([u8; BYTES]);
impl<const BYTES: usize> Clone for WideStore<BYTES> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const BYTES: usize> Copy for WideStore<BYTES> {}
impl<const BYTES: usize, Sign: Signedness> Project<BWide<BYTES>, Sign, Hot> for Picker {
    type T = WideStore<BYTES>;
}
impl<const BYTES: usize, Sign: Signedness> Project<BWide<BYTES>, Sign, Warm> for Picker {
    type T = WideStore<BYTES>;
}

// -- THE SHIPPED SIGNATURE, UNCHANGED --------------------------------------
// `arvo-strategy/src/container.rs:114`, except `u16` -> `Width` as the crate's
// own doc (`width.rs:6-8`) already says it should be.
pub trait BitsContainerFor<const N: Width, Sign: Signedness>: Strategy {
    type T: Copy;
}

impl<const N: Width, Sign: Signedness> BitsContainerFor<N, Sign> for Hot
where
    Wid<N>: WidthFor<HotCold>,
    Picker: Project<<Wid<N> as WidthFor<HotCold>>::Bkt, Sign, Hot>,
{
    type T = <Picker as Project<<Wid<N> as WidthFor<HotCold>>::Bkt, Sign, Hot>>::T;
}

impl<const N: Width, Sign: Signedness> BitsContainerFor<N, Sign> for Warm
where
    Wid<N>: WidthFor<WarmPrecise>,
    Picker: Project<<Wid<N> as WidthFor<WarmPrecise>>::Bkt, Sign, Warm>,
{
    type T = <Picker as Project<<Wid<N> as WidthFor<WarmPrecise>>::Bkt, Sign, Warm>>::T;
}

// -- `Bits`, signature unchanged from arvo-storage/src/bits.rs:57 ----------
#[repr(transparent)]
pub struct Bits<const N: Width, S: Strategy = Hot, Sign: Signedness = Unsigned>(
    <S as BitsContainerFor<N, Sign>>::T,
)
where
    S: BitsContainerFor<N, Sign>;

// -- resolution checks, not just parsing ----------------------------------
const _: () = {
    let _: <Hot as BitsContainerFor<{ Width(13) }, Unsigned>>::T = 0u16;
    let _: <Hot as BitsContainerFor<{ Width(64) }, Signed>>::T = 0i64;
    let _: <Warm as BitsContainerFor<{ Width(13) }, Unsigned>>::T = 0u32;
    let _: <Warm as BitsContainerFor<{ Width(32) }, Signed>>::T = 0i64;
    let _: <Hot as BitsContainerFor<{ Width(129) }, Unsigned>>::T = WideStore::<17>([0u8; 17]);
};
// the caller-threads-its-own-generic case
pub fn threaded<const N: Width, S: Strategy, Sign: Signedness>(_b: Bits<N, S, Sign>)
where
    S: BitsContainerFor<N, Sign>,
{
}
const _: () = {
    let _ = threaded::<{ Width(13) }, Hot, Unsigned>;
};
