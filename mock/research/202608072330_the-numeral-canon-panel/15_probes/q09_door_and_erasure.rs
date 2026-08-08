// q09. The whole thing end to end, and the one question the (W,F) keying turns
// on: the consumer writes an INTEGER width and a fraction width, the machinery
// needs a TOTAL width, and the sum has to happen somewhere.
//
// It cannot happen in a const argument. `Numeral<{I + F}, ...>` is arithmetic in
// const-argument position, which needs generic_const_exprs, which is forbidden
// (unstable-features.md, op 2026-07-28). The move this probe tests is the one
// a-refused-bound-wants-a-trait-not-a-feature.md describes: do not put the
// expression in the const position, put the derivation behind a contract. The
// door maps each literal to a nat, and the ADDITION happens in the nat algebra,
// which is ordinary trait resolution.
//
//     pub type UFixed<const I: u32, const F: u32, Sn, S> =
//         Numeral<Sum<NatOf<I>, NatOf<F>>, NatOf<F>, Sn, S>;
//
// Toolchain: rustc 1.98.0-nightly (57d06900f 2026-05-27), pin nightly-2026-05-28.
// Features: none. No -Z flags. Default solver. Edition 2024.
//
// Build: rustc +nightly-2026-05-28 --edition 2024 -O q09_door_and_erasure.rs \
//          --out-dir build && ./build/q09_door_and_erasure

#![allow(dead_code)]

include!("q07_body_inc.rs");
include!("q09_shape_inc.rs");

// ------------------------------------------------------------- the door -----
// One impl per literal a program writes. This is the bridge, and it is the
// table every candidate in the second stretch has. Its domain is the literals
// in the source text, per 11, 12 and 13 arriving at that separately.
pub struct L<const K: u32>;
pub trait Lit {
    type N;
}
include!("q09_lit_inc.rs");
pub type NatOf<const K: u32> = <L<K> as Lit>::N;

// --------------------------------------------------------- the numeral ------
// Keyed on (total width, fraction width). The integer width is a derived view,
// never stored, which is exactly why the negative-integer-width case costs
// nothing: see q01 and q06.
pub struct Numeral<W, F, Sn, S>(core::marker::PhantomData<(W, F, Sn, S)>);

pub type UFixed<const I: u32, const F: u32, Sn, S> =
    Numeral<Sum<NatOf<I>, NatOf<F>>, NatOf<F>, Sn, S>;
pub type UInt<const I: u32, Sn, S> = Numeral<NatOf<I>, N0, Sn, S>;

// The whole derivation named once, so the bound avalanche is arvo's cost paid
// in one impl and never reaches a consumer. 13:214-231 is the same repair for a
// two-input map; this is it for three.
pub trait Derived {
    type Container;
    type Stride;
    type Width;
    type Frac;
}
impl<W, F, Sn: Signedness, S> Derived for Numeral<W, F, Sn, S>
where
    S: Realise<Sn, W>,
{
    type Container = <S as Realise<Sn, W>>::Container;
    type Stride = <S as Realise<Sn, W>>::Stride;
    type Width = W;
    type Frac = F;
}

pub type Cont<T> = <T as Derived>::Container;
pub type Strd<T> = <T as Derived>::Stride;

// --------------------------------------------------------- what it derives --
type Money = UFixed<13, 3, Unsigned, Hot>;
type MoneyCold = UFixed<13, 3, Unsigned, Cold>;
type MoneySigned = UFixed<13, 3, Signed, Hot>;
type StrHandle = UInt<5, Unsigned, Warm>;
type Big = UFixed<200, 40, Unsigned, Warm>;
type BigHot = UFixed<200, 40, Unsigned, Hot>;
type BigCold = UFixed<200, 40, Unsigned, Cold>;

// the negative-integer-width case, spelled as the ONLY thing that can spell it:
// the output of a product. There is no consumer syntax for it and none is
// needed, because a consumer never names an operation's output type.
type Half = UFixed<0, 1, Unsigned, Warm>; // W = 1, F = 1
type HalfSqShape = Prod<Shape<N1, N1>, Shape<N1, N1>>;
type HalfSq = Numeral<WOf<HalfSqShape>, FOf<HalfSqShape>, Unsigned, Warm>;

// static assertions, checked by the compiler rather than by the printout below
fn _static()
where
    Money: Derived<Container = u16, Stride = N16, Width = N16, Frac = N3>,
    MoneyCold: Derived<Container = u16, Stride = N16, Width = N16, Frac = N3>,
    MoneySigned: Derived<Container = i16, Stride = N16, Width = N16, Frac = N3>,
    StrHandle: Derived<Container = u8, Stride = N8, Width = N5, Frac = N0>,
    Big: Derived<Width = N240, Frac = N40>,
    BigCold: Derived<Stride = N240>,
    Big: Derived<Stride = N240>,
    // and the case the whole file exists for: I = -1, W = 1, F = 2
    HalfSq: Derived<Container = u8, Width = N1, Frac = N2>,
{
}

fn size<T>() -> usize {
    core::mem::size_of::<T>()
}
fn align<T>() -> usize {
    core::mem::align_of::<T>()
}

fn main() {
    macro_rules! row {
        ($name:expr, $t:ty) => {
            println!(
                "{:<26} container size={:<4} align={:<3}",
                $name,
                size::<Cont<$t>>(),
                align::<Cont<$t>>()
            );
        };
    }
    row!("UFixed<13,3,U,Hot>", Money);
    row!("UFixed<13,3,U,Cold>", MoneyCold);
    row!("UFixed<13,3,S,Hot>", MoneySigned);
    row!("UInt<5,U,Warm>", StrHandle);
    row!("UFixed<200,40,U,Warm>", Big);
    row!("UFixed<200,40,U,Hot>", BigHot);
    row!("UFixed<200,40,U,Cold>", BigCold);
    row!("Half^2 (W=1,F=2,I=-1)", HalfSq);
    println!();
    println!("the wide rung, alignment by strategy:");
    println!(
        "  Hot  align={}  Warm align={}  Cold align={}",
        align::<Cont<BigHot>>(),
        align::<Cont<Big>>(),
        align::<Cont<BigCold>>()
    );
}
