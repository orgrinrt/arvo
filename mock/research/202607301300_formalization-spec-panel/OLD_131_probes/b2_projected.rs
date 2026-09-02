//! The strategy picks the container. `UFixed<13, 3, Warm>`.
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
use core::marker::PhantomData;

pub trait Sign: Copy {
    const EXTRA: u32;
}
#[derive(Clone, Copy)]
pub struct Unsigned;
#[derive(Clone, Copy)]
pub struct Signed;
impl Sign for Unsigned {
    const EXTRA: u32 = 0;
}
impl Sign for Signed {
    const EXTRA: u32 = 1;
}

#[derive(Clone, Copy)]
pub struct Wide<const BYTES: usize, A>([u8; BYTES], PhantomData<A>);
#[derive(Clone, Copy)]
pub struct A1;
#[derive(Clone, Copy)]
pub struct A16;

/// Rung of the ladder the hardware actually has. Six: five native, one wide.
pub trait Project<const TAG: usize, G: Sign, const BYTES: usize, S> {
    type T: Copy;
}
pub struct Picker;

macro_rules! native {
    ($s:ty, [$($ut:literal => $u:ty),+], [$($it:literal => $i:ty),+]) => {
        $(impl<const B: usize> Project<$ut, Unsigned, B, $s> for Picker { type T = $u; })+
        $(impl<const B: usize> Project<$it, Signed,   B, $s> for Picker { type T = $i; })+
    };
}
pub struct Hot;
pub struct Cold;
pub struct Warm;
pub struct Precise;
native!(Hot,     [0=>u8,1=>u16,2=>u32,3=>u64,4=>u128], [0=>i8,1=>i16,2=>i32,3=>i64,4=>i128]);
native!(Cold,    [0=>u8,1=>u16,2=>u32,3=>u64,4=>u128], [0=>i8,1=>i16,2=>i32,3=>i64,4=>i128]);
native!(Warm,    [0=>u16,1=>u32,2=>u64,3=>u128],       [0=>i16,1=>i32,2=>i64,3=>i128]);
native!(Precise, [0=>u16,1=>u32,2=>u64,3=>u128],       [0=>i16,1=>i32,2=>i64,3=>i128]);
impl<G: Sign, const B: usize> Project<5, G, B, Hot> for Picker {
    type T = Wide<B, A16>;
}
impl<G: Sign, const B: usize> Project<5, G, B, Cold> for Picker {
    type T = Wide<B, A1>;
}
impl<G: Sign, const B: usize> Project<5, G, B, Warm> for Picker {
    type T = Wide<B, A1>;
}
impl<G: Sign, const B: usize> Project<5, G, B, Precise> for Picker {
    type T = Wide<B, A1>;
}

pub const fn tag_min(n: u32) -> usize {
    if n <= 8 {
        0
    } else if n <= 16 {
        1
    } else if n <= 32 {
        2
    } else if n <= 64 {
        3
    } else if n <= 128 {
        4
    } else {
        5
    }
}
pub const fn tag_headroom(n: u32) -> usize {
    if n <= 8 {
        0
    } else if n <= 16 {
        1
    } else if n <= 32 {
        2
    } else if n <= 64 {
        3
    } else {
        5
    }
}
pub const fn bytes_for(n: u32) -> usize {
    (n as usize).div_ceil(8)
}

pub struct Rung<const I: u32, const F: u32, G: Sign, S>(PhantomData<(G, S)>);
pub trait Tagged {
    type const TAG: usize;
    type const BYTES: usize;
}
macro_rules! tagged {
    ($s:ty, $f:ident) => {
        impl<const I: u32, const F: u32, G: Sign> Tagged for Rung<I, F, G, $s> {
            type const TAG: usize = const { $f(G::EXTRA + I + F) };
            type const BYTES: usize = const { bytes_for(G::EXTRA + I + F) };
        }
    };
}
tagged!(Hot, tag_min);
tagged!(Cold, tag_min);
tagged!(Warm, tag_headroom);
tagged!(Precise, tag_headroom);

/// The container level is derived, never declared as an axis (110:3251).
pub trait Lowering: Sized {
    type Store<const I: u32, const F: u32, G: Sign>: Copy;
}
macro_rules! lower {
    ($s:ty) => {
        impl Lowering for $s {
            type Store<const I: u32, const F: u32, G: Sign> = <Picker as Project<
                { <Rung<I, F, G, $s> as Tagged>::TAG },
                G,
                { <Rung<I, F, G, $s> as Tagged>::BYTES },
                $s,
            >>::T;
        }
    };
}
lower!(Hot);
lower!(Cold);
lower!(Warm);
lower!(Precise);

pub struct Fixed<const I: u32, const F: u32, G: Sign, S: Lowering> {
    raw: S::Store<I, F, G>,
    _m: PhantomData<G>,
}
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, Unsigned, S>;
pub type IFixed<const I: u32, const F: u32, S> = Fixed<I, F, Signed, S>;

pub fn consumer(
    _a: UFixed<13, 3, Warm>,
    _b: UFixed<8, 8, Warm>,
    _c: UFixed<40, 30, Precise>,
    _d: UFixed<3, 0, Hot>,
    _e: UFixed<0, 8, Cold>,
    _f: IFixed<12, 3, Warm>,
    _g: UFixed<200, 100, Hot>,
) {
}

// the projection lands where the hardware says it does
const fn same<T>() {}
pub fn ladder(_: UFixed<13, 3, Warm>) {
    let _: <Warm as Lowering>::Store<13, 3, Unsigned> = 0u32; // 16 bits + headroom -> u32
    let _: <Hot as Lowering>::Store<13, 3, Unsigned> = 0u16; // 16 bits, min aligned -> u16
    let _: <Hot as Lowering>::Store<3, 0, Unsigned> = 0u8;
    let _: <Hot as Lowering>::Store<12, 3, Signed> = 0i16; // 1+12+3 = 16
    let _: <Warm as Lowering>::Store<40, 30, Unsigned> = Wide::<9, A1>([0; 9], PhantomData);
    same::<u8>();
}
