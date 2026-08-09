//! The strategy picks the container. UFixed<13, 3, Warm>: three written
//! parameters, container projected from (strategy, widths, sign).
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
use core::marker::PhantomData;

// ---- sign axis -----------------------------------------------------------
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

// ---- the hardware ladder: five native rungs plus the wide rung -----------
pub trait Project<const TAG: usize, G: Sign> {
    type T: Copy;
}
pub struct Picker;
pub struct Wide<const BYTES: usize>([u8; BYTES]);
impl<const BYTES: usize> Clone for Wide<BYTES> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const BYTES: usize> Copy for Wide<BYTES> {}

impl Project<0, Unsigned> for Picker {
    type T = u8;
}
impl Project<1, Unsigned> for Picker {
    type T = u16;
}
impl Project<2, Unsigned> for Picker {
    type T = u32;
}
impl Project<3, Unsigned> for Picker {
    type T = u64;
}
impl Project<4, Unsigned> for Picker {
    type T = u128;
}
impl Project<0, Signed> for Picker {
    type T = i8;
}
impl Project<1, Signed> for Picker {
    type T = i16;
}
impl Project<2, Signed> for Picker {
    type T = i32;
}
impl Project<3, Signed> for Picker {
    type T = i64;
}
impl Project<4, Signed> for Picker {
    type T = i128;
}
// rung 5 is wide; keyed on the byte count, itself a projection
pub trait WideOf<const BYTES: usize> {
    type T: Copy;
}
impl<const BYTES: usize> WideOf<BYTES> for Picker {
    type T = Wide<BYTES>;
}

// ---- the two policies over the ladder -----------------------------------
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

// ---- strategies ----------------------------------------------------------
pub struct Hot;
pub struct Cold;
pub struct Warm;
pub struct Precise;

pub struct Rung<const I: u32, const F: u32, G: Sign, S>(PhantomData<(G, S)>);
pub trait Tagged {
    type const TAG: usize;
}
impl<const I: u32, const F: u32, G: Sign> Tagged for Rung<I, F, G, Hot> {
    type const TAG: usize = const { tag_min(G::EXTRA + I + F) };
}
impl<const I: u32, const F: u32, G: Sign> Tagged for Rung<I, F, G, Cold> {
    type const TAG: usize = const { tag_min(G::EXTRA + I + F) };
}
impl<const I: u32, const F: u32, G: Sign> Tagged for Rung<I, F, G, Warm> {
    type const TAG: usize = const { tag_headroom(G::EXTRA + I + F) };
}
impl<const I: u32, const F: u32, G: Sign> Tagged for Rung<I, F, G, Precise> {
    type const TAG: usize = const { tag_headroom(G::EXTRA + I + F) };
}

/// The container level is derived, never declared as an axis (110:3251).
#[diagnostic::on_unimplemented(
    message = "strategy `{Self}` has no container for a {I}+{F} digit numeral",
    note = "Hot and Cold cover 1..=128 physical bits over the native ladder u8/u16/u32/u64/u128. Warm and Precise take one rung of headroom, so they cover 1..=64 and have no native rung at 65..=128: name Hot or Cold there. Above 128 physical bits every strategy lands on the wide rung."
)]
pub trait Lowering {
    type Store<const I: u32, const F: u32, G: Sign>: Copy;
}

macro_rules! lower {
    ($s:ty) => {
        impl Lowering for $s {
            type Store<const I: u32, const F: u32, G: Sign>
                = <Picker as Project<{ <Rung<I, F, G, $s> as Tagged>::TAG }, G>>::T
            where
                Picker: Project<{ <Rung<I, F, G, $s> as Tagged>::TAG }, G>;
        }
    };
}
lower!(Hot);
lower!(Cold);
lower!(Warm);
lower!(Precise);

// ---- the numeral ---------------------------------------------------------
pub struct Fixed<const I: u32, const F: u32, G: Sign, S: Lowering> {
    raw: S::Store<I, F, G>,
    _m: PhantomData<G>,
}
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, Unsigned, S>;
pub type IFixed<const I: u32, const F: u32, S> = Fixed<I, F, Signed, S>;

// ---- what a consumer writes ---------------------------------------------
pub fn consumer(
    _a: UFixed<13, 3, Warm>,
    _b: UFixed<8, 8, Warm>,
    _c: UFixed<40, 30, Precise>,
    _d: UFixed<3, 0, Hot>,
    _e: UFixed<0, 8, Cold>,
    _f: IFixed<12, 3, Warm>,
) {
}
