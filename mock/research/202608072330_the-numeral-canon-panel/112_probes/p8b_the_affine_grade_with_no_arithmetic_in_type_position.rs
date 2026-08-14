// p8b. The repair of p8: the affine grade with no arithmetic in type position.
//
// p8 reached for a type-level list, which is the right move, and then put the
// coefficient in a CONST ARGUMENT, so `Cons<{ A + B }, ..>` needed
// `generic_const_exprs`.  Half-applying the reflex lands on the forbidden
// feature.
//
// The repair: a coefficient is a TYPE carrying an associated const, never a
// const argument, so every arithmetic operation happens inside an impl body
// where arbitrary const expressions are legal.  `109` P5 already does exactly
// this for a scalar bound; this is the same construction for a vector.
//
// PREDICTIONS, RECORDED BEFORE COMPILING
// --------------------------------------
// P1. It compiles with NO `#![feature(...)]` line at all, no `dyn`, no
//     `TypeId`.
// P2. The two occurrences of one leaf cancel IN THE TYPE, so `(x + y) - y`
//     carries y's coefficient at zero and its radius is x's alone.
// P3. The corner rule refuses the same term, so the two rules disagree here,
//     matching p7's model, which reports corner 16/256 against affine 136/256.
// P4. The discharge test gates an arm through `const { }` and the refused arm
//     is absent from the emitted code, so the licence costs nothing at
//     runtime.
// P5. The whole vector composes to depth, so a term of several operations
//     resolves without any of the intermediate types being written by hand.

use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// A coefficient is a type carrying a value.  Arithmetic lives in impl bodies.
// ---------------------------------------------------------------------------

trait Coeff {
    const V: i32;
}

struct Lit<const N: i32>;
impl<const N: i32> Coeff for Lit<N> {
    const V: i32 = N;
}

struct CSum<A, B>(PhantomData<(A, B)>);
impl<A: Coeff, B: Coeff> Coeff for CSum<A, B> {
    const V: i32 = A::V + B::V;
}

struct CDiff<A, B>(PhantomData<(A, B)>);
impl<A: Coeff, B: Coeff> Coeff for CDiff<A, B> {
    const V: i32 = A::V - B::V;
}

// ---------------------------------------------------------------------------
// The coefficient vector, one cell per noise symbol, in a fixed order.
// ---------------------------------------------------------------------------

trait Coeffs {
    /// Sum of the absolute coefficients: the form's radius.
    const RADIUS: i32;
    /// How many cells, so a reader can see the vector's length is static.
    const LEN: usize;
}

struct Nil;
struct Cons<H, T>(PhantomData<(H, T)>);

impl Coeffs for Nil {
    const RADIUS: i32 = 0;
    const LEN: usize = 0;
}

impl<H: Coeff, T: Coeffs> Coeffs for Cons<H, T> {
    const RADIUS: i32 = {
        let v = H::V;
        (if v < 0 { -v } else { v }) + T::RADIUS
    };
    const LEN: usize = 1 + T::LEN;
}

/// Pointwise sum of two vectors of the same shape.
trait AddC<Other> {
    type Out: Coeffs;
}
impl AddC<Nil> for Nil {
    type Out = Nil;
}
impl<H1, T1, H2, T2> AddC<Cons<H2, T2>> for Cons<H1, T1>
where
    H1: Coeff,
    H2: Coeff,
    T1: AddC<T2>,
{
    type Out = Cons<CSum<H1, H2>, <T1 as AddC<T2>>::Out>;
}

/// Pointwise difference.
trait SubC<Other> {
    type Out: Coeffs;
}
impl SubC<Nil> for Nil {
    type Out = Nil;
}
impl<H1, T1, H2, T2> SubC<Cons<H2, T2>> for Cons<H1, T1>
where
    H1: Coeff,
    H2: Coeff,
    T1: SubC<T2>,
{
    type Out = Cons<CDiff<H1, H2>, <T1 as SubC<T2>>::Out>;
}

// ---------------------------------------------------------------------------
// A grade: a centre (itself a coefficient type) and a coefficient vector.
// ---------------------------------------------------------------------------

trait Grade {
    type Centre: Coeff;
    type Vec: Coeffs;
    const LO: i32 = <Self::Centre as Coeff>::V - <Self::Vec as Coeffs>::RADIUS;
    const HI: i32 = <Self::Centre as Coeff>::V + <Self::Vec as Coeffs>::RADIUS;
}

struct Aff<C, V>(PhantomData<(C, V)>);
impl<C: Coeff, V: Coeffs> Grade for Aff<C, V> {
    type Centre = C;
    type Vec = V;
}

/// The grade of a sum.  Centres add, vectors add pointwise, and the result is
/// itself a grade, so terms compose to any depth without a hand-written type.
struct GAdd<A, B>(PhantomData<(A, B)>);
impl<A, B> Grade for GAdd<A, B>
where
    A: Grade,
    B: Grade,
    A::Vec: AddC<B::Vec>,
{
    type Centre = CSum<A::Centre, B::Centre>;
    type Vec = <A::Vec as AddC<B::Vec>>::Out;
}

/// The grade of a difference.
struct GSub<A, B>(PhantomData<(A, B)>);
impl<A, B> Grade for GSub<A, B>
where
    A: Grade,
    B: Grade,
    A::Vec: SubC<B::Vec>,
{
    type Centre = CDiff<A::Centre, B::Centre>;
    type Vec = <A::Vec as SubC<B::Vec>>::Out;
}

// ---------------------------------------------------------------------------
// The container and the discharge test.
// ---------------------------------------------------------------------------

const CLO: i32 = 0;
const CHI: i32 = 15;

trait Discharges {
    const OK: bool;
}
impl<G: Grade> Discharges for G {
    const OK: bool = G::LO >= CLO && G::HI <= CHI;
}

// ---------------------------------------------------------------------------
// Two symbols, (x, y), in a fixed order.  A leaf declared 0..=b has centre
// b/2 and coefficient b/2 on its own symbol and zero on every other.
// ---------------------------------------------------------------------------

type Gx = Aff<Lit<7>, Cons<Lit<7>, Cons<Lit<0>, Nil>>>;
type Gy = Aff<Lit<7>, Cons<Lit<0>, Cons<Lit<7>, Nil>>>;

type GSum = GAdd<Gx, Gy>; //  x + y
type GDiff = GSub<GSum, Gy>; //  (x + y) - y, composed rather than hand-written
type GDeep = GSub<GAdd<GAdd<Gx, Gy>, Gx>, GAdd<Gx, Gy>>; //  ((x+y)+x) - (x+y)

// The interval rule for the same terms, computed the same way it is in p7.
const XLO: i32 = 0;
const XHI: i32 = 14;
const YLO: i32 = 0;
const YHI: i32 = 14;
const CORNER_SUM_LO: i32 = XLO + YLO;
const CORNER_SUM_HI: i32 = XHI + YHI;
const CORNER_DIFF_LO: i32 = CORNER_SUM_LO - YHI;
const CORNER_DIFF_HI: i32 = CORNER_SUM_HI - YLO;

// ---------------------------------------------------------------------------
// Arms gated on the associated const.
// ---------------------------------------------------------------------------

#[inline(never)]
#[no_mangle]
pub fn affine_gated_diff(a: u8, b: u8, c: u8) -> u8 {
    if const { <GDiff as Discharges>::OK } {
        (a + b) - c
    } else {
        a.saturating_add(b).saturating_sub(c)
    }
}

#[inline(never)]
#[no_mangle]
pub fn corner_gated_diff(a: u8, b: u8, c: u8) -> u8 {
    if const { CORNER_DIFF_LO >= CLO && CORNER_DIFF_HI <= CHI } {
        (a + b) - c
    } else {
        a.saturating_add(b).saturating_sub(c)
    }
}

#[inline(never)]
#[no_mangle]
pub fn general_diff(a: u8, b: u8, c: u8) -> u8 {
    a.saturating_add(b).saturating_sub(c)
}

#[inline(never)]
#[no_mangle]
pub fn bare_diff(a: u8, b: u8, c: u8) -> u8 {
    (a + b) - c
}

fn main() {
    println!("p8b. The affine grade with no arithmetic in type position");
    println!("{}", "=".repeat(70));
    println!();
    println!("  container: [{}, {}]", CLO, CHI);
    println!();
    println!(
        "  Gx      -> [{:>3}, {:>3}]   vector length {}",
        <Gx as Grade>::LO,
        <Gx as Grade>::HI,
        <<Gx as Grade>::Vec as Coeffs>::LEN
    );
    println!(
        "  Gy      -> [{:>3}, {:>3}]   vector length {}",
        <Gy as Grade>::LO,
        <Gy as Grade>::HI,
        <<Gy as Grade>::Vec as Coeffs>::LEN
    );
    println!(
        "  x + y   -> [{:>3}, {:>3}]   discharges {}",
        <GSum as Grade>::LO,
        <GSum as Grade>::HI,
        <GSum as Discharges>::OK
    );
    println!();
    println!("  the correlated term, (x + y) - y, COMPOSED rather than written out:");
    println!(
        "    affine -> [{:>3}, {:>3}]   discharges {}   radius {}",
        <GDiff as Grade>::LO,
        <GDiff as Grade>::HI,
        <GDiff as Discharges>::OK,
        <<GDiff as Grade>::Vec as Coeffs>::RADIUS
    );
    println!(
        "    corner -> [{:>3}, {:>3}]   discharges {}",
        CORNER_DIFF_LO,
        CORNER_DIFF_HI,
        CORNER_DIFF_LO >= CLO && CORNER_DIFF_HI <= CHI
    );
    println!();
    println!("  a deeper composition, ((x + y) + x) - (x + y):");
    println!(
        "    affine -> [{:>3}, {:>3}]   discharges {}   vector length {}",
        <GDeep as Grade>::LO,
        <GDeep as Grade>::HI,
        <GDeep as Discharges>::OK,
        <<GDeep as Grade>::Vec as Coeffs>::LEN
    );
    println!(
        "    (the exact value of that term is x, so [0, 14] is the tight answer)"
    );

    println!();
    println!("  BEHAVIOUR, on the declared term with c = b:");
    let mut differ = 0usize;
    let mut n = 0usize;
    for a in 0u8..=14 {
        for b in 0u8..=14 {
            n += 1;
            let want = a as i32 + b as i32 - b as i32;
            if affine_gated_diff(a, b, b) as i32 != want {
                differ += 1;
            }
        }
    }
    println!(
        "    the affine-gated arm differs from exact on {}/{} tuples",
        differ, n
    );
    let mut cdiff = 0usize;
    for a in 0u8..=14 {
        for b in 0u8..=14 {
            let want = a as i32 + b as i32 - b as i32;
            if corner_gated_diff(a, b, b) as i32 != want {
                cdiff += 1;
            }
        }
    }
    println!(
        "    the corner-gated arm differs from exact on {}/{} tuples, because it",
        cdiff, n
    );
    println!("    refused the licence and took the saturating path");
}
