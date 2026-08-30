// q15. Two things the composition in section 4.2 could have broken, checked
// rather than assumed.
//
// c1  13:232-278 reports arrangement A has no ceiling: the operation algebra
//     never consults the bridge, so multiplying past the table's last row is
//     fine. That was measured with the SUM-OF-WIDTHS product rule. This
//     composition uses the TIGHT rule from q02, which has an extra comparison
//     against N1 in it, and a comparison could in principle reach the bridge.
//     Does the ceiling stay absent.
//
// c2  12:362-397 found that an undeclared width written at the alias-definition
//     site produces NO ERROR AT ALL, because a Rust type alias does not check
//     its bounds, and the error lands at the first use spanning a name the
//     consumer never wrote. That is a defect of the const door, attributed by
//     12 to the design as it stands. Does the (W,F) keying change it, and does
//     the diagnostic tag help at that site.
//
// Toolchain: rustc 1.98.0-nightly (57d06900f 2026-05-27), pin nightly-2026-05-28.
// Features: none. Edition 2024.
// Build: rustc +nightly-2026-05-28 --edition 2024 --crate-type lib \
//          q15_ceiling_and_alias_site.rs --out-dir build       (c1 alone: exit 0)
//        rustc +nightly-2026-05-28 --edition 2024 --cfg c2 --crate-type lib \
//          q15_ceiling_and_alias_site.rs --out-dir build       (c2: the defect)

#![no_std]
#![allow(dead_code)]

include!("q07_body_inc.rs");
include!("q09_shape_inc.rs");

pub struct L<const K: u32>;
pub trait Lit {
    type N;
}
// The whole bridge. Six rows. 999 is deliberately NOT here.
macro_rules! lits { ($($k:literal => $t:ty),* $(,)?) => { $(
    impl Lit for L<$k> { type N = $t; }
)* } }
lits!(0 => N0, 3 => N3, 5 => N5, 13 => N13, 16 => N16, 40 => N40);
pub type NatOf<const K: u32> = <L<K> as Lit>::N;

pub struct Tag<const I: u32, const F: u32>;
pub struct Anon;
pub struct Numeral<W, F, T, Sn, S>(core::marker::PhantomData<(W, F, T, Sn, S)>);

pub type UFixed<const I: u32, const F: u32, Sn, S> =
    Numeral<Sum<NatOf<I>, NatOf<F>>, NatOf<F>, Tag<I, F>, Sn, S>;

pub trait Derived2 {
    type Container;
    type Stride;
    type W;
    type F;
}
impl<W, F, T, Sn: Signedness, S> Derived2 for Numeral<W, F, T, Sn, S>
where
    S: Realise<Sn, W>,
{
    type Container = <S as Realise<Sn, W>>::Container;
    type Stride = <S as Realise<Sn, W>>::Stride;
    type W = W;
    type F = F;
}

// the tight product, lifted onto numerals
pub trait Mul<R> {
    type Out;
}
impl<W1, F1, T1, W2, F2, T2, Sn, S> Mul<Numeral<W2, F2, T2, Sn, S>> for Numeral<W1, F1, T1, Sn, S>
where
    Shape<W1, F1>: ProdS<Shape<W2, F2>>,
    Prod<Shape<W1, F1>, Shape<W2, F2>>: Parts,
{
    type Out = Numeral<
        WOf<Prod<Shape<W1, F1>, Shape<W2, F2>>>,
        FOf<Prod<Shape<W1, F1>, Shape<W2, F2>>>,
        Anon,
        Sn,
        S,
    >;
}
pub type P<A, B> = <A as Mul<B>>::Out;

// ---- c1: multiply four times past the table's last row (40) ----------------
type Money = UFixed<13, 3, Unsigned, Warm>; // W = 16
type M2 = P<Money, Money>; // W = 32
type M4 = P<M2, M2>; // W = 64
type M8 = P<M4, M4>; // W = 128
type M16 = P<M8, M8>; // W = 256

fn c1()
where
    M2: Derived2<W = N32, F = N6, Container = u32>,
    M4: Derived2<W = N64, F = N12, Container = u64>,
    M8: Derived2<W = N128, F = N24, Container = u128>,
    M16: Derived2<W = N256, F = N48>,
{
}

// ---- c2: an undeclared width at the alias-definition site -------------------
// 999 has no bridge row. Under 12's finding this alias DEFINITION is accepted in
// silence and the error surfaces at the first use.
#[cfg(c2)]
type Undeclared = UFixed<999, 3, Unsigned, Warm>;

#[cfg(c2)]
fn c2_first_use(_: Undeclared) {}
