//! The strategy picks the container. `UFixed<13, 3, Warm>`, three written
//! parameters, container derived from (strategy, widths, sign).
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
use core::marker::PhantomData;

// ---------------------------------------------------------------- sign axis
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

// -------------------------------------------------- the ladder the hw has
#[derive(Clone, Copy)]
pub struct Wide<const BYTES: usize, A>([u8; BYTES], PhantomData<A>);
#[derive(Clone, Copy)]
pub struct A1;
#[derive(Clone, Copy)]
pub struct A16;

pub trait Project<const TAG: usize, G: Sign, const BYTES: usize, S> {
    type T: Copy;
}
pub struct Picker;
pub struct Hot;
pub struct Cold;
pub struct Warm;
pub struct Precise;

macro_rules! native {
    ($s:ty, [$($ut:literal => $u:ty),+], [$($it:literal => $i:ty),+]) => {
        $(impl<const B: usize> Project<$ut, Unsigned, B, $s> for Picker { type T = $u; })+
        $(impl<const B: usize> Project<$it, Signed,   B, $s> for Picker { type T = $i; })+
    };
}
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

pub const fn tag_min(w: u32) -> usize {
    if w <= 8 {
        0
    } else if w <= 16 {
        1
    } else if w <= 32 {
        2
    } else if w <= 64 {
        3
    } else if w <= 128 {
        4
    } else {
        5
    }
}
pub const fn tag_headroom(w: u32) -> usize {
    if w <= 8 {
        0
    } else if w <= 16 {
        1
    } else if w <= 32 {
        2
    } else if w <= 64 {
        3
    } else {
        5
    }
}
pub const fn bytes_for(w: u32) -> usize {
    (w as usize).div_ceil(8)
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

/// The container level: derived, never declared as an axis.
pub trait Store<const I: u32, const F: u32, G: Sign> {
    type T: Copy;
}
macro_rules! store {
    ($s:ty) => {
        impl<const I: u32, const F: u32, G: Sign> Store<I, F, G> for $s
        where
            Picker: Project<
                { <Rung<I, F, G, $s> as Tagged>::TAG },
                G,
                { <Rung<I, F, G, $s> as Tagged>::BYTES },
                $s,
            >,
        {
            type T = <Picker as Project<
                { <Rung<I, F, G, $s> as Tagged>::TAG },
                G,
                { <Rung<I, F, G, $s> as Tagged>::BYTES },
                $s,
            >>::T;
        }
    };
}
store!(Hot);
store!(Cold);
store!(Warm);
store!(Precise);

// ------------------------------------------------------------- the numeral
pub struct Fixed<const I: u32, const F: u32, G: Sign, S: Store<I, F, G>> {
    raw: <S as Store<I, F, G>>::T,
    _m: PhantomData<G>,
}
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, Unsigned, S>;
pub type IFixed<const I: u32, const F: u32, S> = Fixed<I, F, Signed, S>;

// ----------------------------------- the mathematical coordinates, as reads
pub trait Format {
    const PRECISION: u32; // significand digits: I + F, sign-free (D69)
    const EXPONENT: i32; // the quantum: -F
    const INTEGER_DIGITS: u32;
    const FRACTION_DIGITS: u32;
    const SIGNED: bool;
    const STORED_WIDTH: u32; // derived on the physical side (D69)
}
impl<const I: u32, const F: u32, G: Sign, S: Store<I, F, G>> Format for Fixed<I, F, G, S> {
    const PRECISION: u32 = I + F;
    const EXPONENT: i32 = -(F as i32);
    const INTEGER_DIGITS: u32 = I;
    const FRACTION_DIGITS: u32 = F;
    const SIGNED: bool = G::EXTRA == 1;
    const STORED_WIDTH: u32 = G::EXTRA + I + F;
}

// ---------------- the consumer surface: three parameters, no container name
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
pub fn ladder() {
    let _: <Warm as Store<13, 3, Unsigned>>::T = 0u32; // 16 digits + headroom
    let _: <Hot as Store<13, 3, Unsigned>>::T = 0u16; // 16 digits, min aligned
    let _: <Hot as Store<3, 0, Unsigned>>::T = 0u8;
    let _: <Cold as Store<0, 8, Unsigned>>::T = 0u8;
    let _: <Hot as Store<12, 3, Signed>>::T = 0i16; // 1 + 12 + 3 = 16
    let _: <Warm as Store<12, 3, Signed>>::T = 0i32;
    let _: <Warm as Store<40, 30, Unsigned>>::T = Wide::<9, A1>([0; 9], PhantomData);
    let _: <Hot as Store<200, 100, Unsigned>>::T = Wide::<38, A16>([0; 38], PhantomData);
}

// the coordinates read correctly, and precision is sign-free
const _: () = assert!(<UFixed<13, 3, Warm> as Format>::PRECISION == 16);
const _: () = assert!(<UFixed<8, 8, Warm> as Format>::PRECISION == 16);
const _: () = assert!(<IFixed<12, 3, Warm> as Format>::PRECISION == 15);
const _: () = assert!(<IFixed<12, 3, Warm> as Format>::STORED_WIDTH == 16);
const _: () = assert!(<UFixed<13, 3, Warm> as Format>::EXPONENT == -3);
const _: () = assert!(<UFixed<40, 30, Precise> as Format>::PRECISION == 70);
