//! p04. The five candidate surfaces, side by side, all compiling in one crate,
//! so that what a consumer types is measured off real text rather than recalled.
//!
//! Each block spells the SAME three things:
//!   tier 2 alias:      a 5-bit unsigned handle, and a 13.3 fixed-point coord
//!   tier 3 explicit:   the coord written directly at a signature
//!   tier 2 derived:    the type of a product, named rather than inferred
//!
//! Marked with `// SITE:` on each line a consumer actually types, so the
//! character counts in `count.sh` are produced by a command over this file
//! rather than by hand.
//!
//! No `#![feature]`, no `-Z` flag.
//!
//! rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib \
//!       --emit=metadata -o out/p04.meta p04_five_spellings.rs
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

include!("ladder.rs");

pub struct Hot;
pub struct Warm;
pub struct Arvo;
pub type Sum<A, B> = <A as Add<B>>::O;
pub type Cont<W> = <W as Container>::C;

#[repr(transparent)]
pub struct Fixed<WI, WF, S>
where
    WI: Add<WF>,
    Sum<WI, WF>: Container,
{
    raw: Cont<Sum<WI, WF>>,
    _m: PhantomData<(WI, WF, S)>,
}

pub type T5 = D1<D0<D1<Term>>>;
pub type T32 = D0<D0<D0<D0<D0<D1<Term>>>>>>;

// =============================================================================
// C0. Const surface, const keying. The design as it stands (`11` p12 shape).
// =============================================================================
pub mod c0 {
    use super::*;
    pub struct Idx<const N: u32>;
    pub trait ToNat<M> {
        type N;
    }
    macro_rules! d { ($($n:literal => $t:ty),* $(,)?) => { $( impl ToNat<Arvo> for Idx<$n> { type N = $t; } )* } }
    d! { 0 => T0, 3 => T3, 5 => T5, 13 => T13, 16 => T16, 26 => T26, 6 => T6, 32 => T32 }

    #[repr(transparent)]
    pub struct CFixed<const I: u32, const F: u32, S, M = Arvo>
    where
        Idx<I>: ToNat<M>,
        Idx<F>: ToNat<M>,
        <Idx<I> as ToNat<M>>::N: Add<<Idx<F> as ToNat<M>>::N>,
        Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>: Container,
    {
        raw: Cont<Sum<<Idx<I> as ToNat<M>>::N, <Idx<F> as ToNat<M>>::N>>,
        _m: PhantomData<(S, M)>,
    }
    pub type UInt<const N: u32> = CFixed<N, 0, Warm>;
    pub type UFixed<const I: u32, const F: u32, S> = CFixed<I, F, S>;

    pub type StrHandle = UInt<5>; // SITE-C0-alias
    pub type Coord = UFixed<13, 3, Hot>; // SITE-C0-alias
    pub fn explicit(x: UFixed<13, 3, Hot>) -> UFixed<13, 3, Hot> {
        x
    } // SITE-C0-explicit
    pub type Product = UFixed<26, 6, Hot>; // SITE-C0-derived
}

// =============================================================================
// C1. Raw nat surface. b03's shape with no alias layer at all.
// =============================================================================
pub mod c1 {
    use super::*;
    pub type StrHandle = Fixed<D1<D0<D1<Term>>>, Term, Warm>; // SITE-C1-alias
    pub type Coord = Fixed<D1<D0<D1<D1<Term>>>>, D1<D1<Term>>, Hot>; // SITE-C1-alias
    pub fn explicit(
        x: Fixed<D1<D0<D1<D1<Term>>>>, D1<D1<Term>>, Hot>,
    ) -> Fixed<D1<D0<D1<D1<Term>>>>, D1<D1<Term>>, Hot> {
        x
    } // SITE-C1-explicit
    pub type Product = Fixed<D0<D1<D0<D1<D1<Term>>>>>, D0<D1<D1<Term>>>, Hot>; // SITE-C1-derived
}

// =============================================================================
// C2. Nat surface with a shipped alias layer. arvo ships N0..N64 as NAMES.
// =============================================================================
pub mod c2 {
    use super::*;
    pub type N0 = Term;
    pub type N3 = T3;
    pub type N5 = T5;
    pub type N6 = T6;
    pub type N13 = T13;
    pub type N26 = T26;
    pub type UInt<W> = Fixed<W, N0, Warm>;
    pub type UFixed<WI, WF, S> = Fixed<WI, WF, S>;

    pub type StrHandle = UInt<N5>; // SITE-C2-alias
    pub type Coord = UFixed<N13, N3, Hot>; // SITE-C2-alias
    pub fn explicit(x: UFixed<N13, N3, Hot>) -> UFixed<N13, N3, Hot> {
        x
    } // SITE-C2-explicit
    pub type Product = UFixed<N26, N6, Hot>; // SITE-C2-derived
}

// =============================================================================
// C3. Nat surface, widths minted at a DECLARATION SITE by a macro.
// The macro call is not at the alias site. It is one line, once, per program.
// =============================================================================
pub mod c3 {
    use super::*;
    macro_rules! widths {
        ($($name:ident = $t:ty),* $(,)?) => { $( pub type $name = $t; )* };
    }
    widths! { W0 = T0, W3 = T3, W5 = T5, W6 = T6, W13 = T13, W26 = T26 } // SITE-C3-decl
    pub type UInt<W> = Fixed<W, W0, Warm>;
    pub type UFixed<WI, WF, S> = Fixed<WI, WF, S>;

    pub type StrHandle = UInt<W5>; // SITE-C3-alias
    pub type Coord = UFixed<W13, W3, Hot>; // SITE-C3-alias
    pub fn explicit(x: UFixed<W13, W3, Hot>) -> UFixed<W13, W3, Hot> {
        x
    } // SITE-C3-explicit
    pub type Product = UFixed<W26, W6, Hot>; // SITE-C3-derived
}

// =============================================================================
// C4. The hybrid: const at the door, nat underneath. p03's shape.
// =============================================================================
pub mod c4 {
    use super::*;
    pub struct Idx<const N: u32>;
    pub trait ToNat<M> {
        type N;
    }
    macro_rules! d { ($($n:literal => $t:ty),* $(,)?) => { $( impl ToNat<Arvo> for Idx<$n> { type N = $t; } )* } }
    d! { 0 => T0, 3 => T3, 5 => T5, 13 => T13, 26 => T26, 6 => T6 }
    pub type NatOf<const N: u32> = <Idx<N> as ToNat<Arvo>>::N;
    pub type UInt<const N: u32> = Fixed<NatOf<N>, T0, Warm>;
    pub type UFixed<const I: u32, const F: u32, S> = Fixed<NatOf<I>, NatOf<F>, S>;

    pub type StrHandle = UInt<5>; // SITE-C4-alias
    pub type Coord = UFixed<13, 3, Hot>; // SITE-C4-alias
    pub fn explicit(x: UFixed<13, 3, Hot>) -> UFixed<13, 3, Hot> {
        x
    } // SITE-C4-explicit
    pub type Product = UFixed<26, 6, Hot>; // SITE-C4-derived
}

// --- all five agree on the layout, which is the point of measuring them ------
const _: () = {
    assert!(core::mem::size_of::<c0::Coord>() == 2);
    assert!(core::mem::size_of::<c1::Coord>() == 2);
    assert!(core::mem::size_of::<c2::Coord>() == 2);
    assert!(core::mem::size_of::<c3::Coord>() == 2);
    assert!(core::mem::size_of::<c4::Coord>() == 2);
    assert!(core::mem::size_of::<c0::StrHandle>() == 1);
    assert!(core::mem::size_of::<c1::StrHandle>() == 1);
    assert!(core::mem::size_of::<c2::StrHandle>() == 1);
    assert!(core::mem::size_of::<c3::StrHandle>() == 1);
    assert!(core::mem::size_of::<c4::StrHandle>() == 1);
};

// --- and C1..C4 are the SAME TYPE, since all four key on the nat -------------
pub fn c1_is_c2(x: c1::Coord) -> c2::Coord {
    x
}
pub fn c2_is_c3(x: c2::Coord) -> c3::Coord {
    x
}
pub fn c3_is_c4(x: c3::Coord) -> c4::Coord {
    x
}
