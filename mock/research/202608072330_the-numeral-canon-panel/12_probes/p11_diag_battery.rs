//! p11. The diagnostic battery. Five keyings, one mistake each, side by side, so
//! that what a consumer READS is compared rather than asserted.
//!
//! The mistake is the most ordinary one available: return a 26.6 numeral where a
//! 13.3 was declared. It is the error a tier-two consumer makes on their first
//! day and again every week after.
//!
//!   K0  const keying                       (C0, the design as it stands)
//!   K1  binary nat keying                  (C4, p03's hybrid)
//!   K2  binary nat, consts in front        (p06's recovery)
//!   K3  decimal nat keying                 (p09 + p10)
//!   K4  decimal nat, consts in front       (p06's recovery, base ten)
//!
//! THIS FILE IS EXPECTED TO FAIL TO COMPILE. Five E0308s are the result and
//! their text is the measurement. Captured in out/p11.log.
//!
//! rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib \
//!       --emit=metadata -o out/p11.meta p11_diag_battery.rs 2> out/p11.log
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

pub struct Hot;
pub struct Arvo;

// --- K0: const keying ---------------------------------------------------------
pub mod k0 {
    use super::*;
    pub struct Fixed<const I: u32, const F: u32, S>(PhantomData<S>);
    pub type Coord = Fixed<13, 3, Hot>;
    pub type Product = Fixed<26, 6, Hot>;
    pub fn mistake(x: Product) -> Coord {
        x
    }
}

// --- K1: binary nat keying ----------------------------------------------------
pub mod k1 {
    use super::*;
    pub struct Term;
    pub struct D0<T>(PhantomData<T>);
    pub struct D1<T>(PhantomData<T>);
    pub struct Fixed<WI, WF, S>(PhantomData<(WI, WF, S)>);
    pub type B13 = D1<D0<D1<D1<Term>>>>;
    pub type B3 = D1<D1<Term>>;
    pub type B26 = D0<D1<D0<D1<D1<Term>>>>>;
    pub type B6 = D0<D1<D1<Term>>>;
    pub type Coord = Fixed<B13, B3, Hot>;
    pub type Product = Fixed<B26, B6, Hot>;
    pub fn mistake(x: Product) -> Coord {
        x
    }
}

// --- K2: binary nat, consts in front and nats defaulted off them --------------
pub mod k2 {
    pub use super::k1::{Term, D0, D1};
    use super::*;
    pub struct Idx<const N: u32>;
    pub trait ToNat {
        type N;
    }
    macro_rules! d { ($($n:literal => $t:ty),* $(,)?) => { $( impl ToNat for Idx<$n> { type N = $t; } )* } }
    d! { 3 => k1::B3, 6 => k1::B6, 13 => k1::B13, 26 => k1::B26 }
    pub type NatOf<const N: u32> = <Idx<N> as ToNat>::N;
    pub struct Fixed<const I: u32, const F: u32, S, WI = NatOf<I>, WF = NatOf<F>>(
        PhantomData<(S, WI, WF)>,
    );
    pub type Coord = Fixed<13, 3, Hot>;
    pub type Product = Fixed<26, 6, Hot>;
    pub fn mistake(x: Product) -> Coord {
        x
    }
}

// --- K3: decimal nat keying ---------------------------------------------------
pub mod k3 {
    use super::*;
    pub struct E;
    pub struct T<D, R>(PhantomData<(D, R)>);
    pub struct N1;
    pub struct N2;
    pub struct N3;
    pub struct N6;
    pub struct Fixed<WI, WF, S>(PhantomData<(WI, WF, S)>);
    pub type Dec13 = T<N3, T<N1, E>>;
    pub type Dec3 = T<N3, E>;
    pub type Dec26 = T<N6, T<N2, E>>;
    pub type Dec6 = T<N6, E>;
    pub type Coord = Fixed<Dec13, Dec3, Hot>;
    pub type Product = Fixed<Dec26, Dec6, Hot>;
    pub fn mistake(x: Product) -> Coord {
        x
    }
}

// --- K4: decimal nat, consts in front ----------------------------------------
pub mod k4 {
    pub use super::k3::{E, N1, N2, N3, N6, T};
    use super::*;
    pub struct Idx<const N: u32>;
    pub trait ToNat {
        type N;
    }
    macro_rules! d { ($($n:literal => $t:ty),* $(,)?) => { $( impl ToNat for Idx<$n> { type N = $t; } )* } }
    d! { 3 => k3::Dec3, 6 => k3::Dec6, 13 => k3::Dec13, 26 => k3::Dec26 }
    pub type NatOf<const N: u32> = <Idx<N> as ToNat>::N;
    pub struct Fixed<const I: u32, const F: u32, S, WI = NatOf<I>, WF = NatOf<F>>(
        PhantomData<(S, WI, WF)>,
    );
    pub type Coord = Fixed<13, 3, Hot>;
    pub type Product = Fixed<26, 6, Hot>;
    pub fn mistake(x: Product) -> Coord {
        x
    }
}
