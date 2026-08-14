// p8c. The repair of p8b: the discharge check recurses over every node.
//
// p8b compiled the affine grade correctly and wired a ROOT-ONLY discharge test
// to it.  `p7`'s model rule checks every node, and `p7c` builds a hand witness
// where a root-only test licenses an arm that computes the wrong answer.  So
// p8b's gate was not the predicate whose numbers `p7` reports.
//
// The repair is one trait: `AllOk` recurses the check over the grade's own
// structure, which is available because a composed grade is a composed TYPE.
// That is the property that makes the per-node discipline expressible at all,
// and it is worth stating: the term's shape is in the type, so the check that
// has to visit every node can visit every node.
//
// PREDICTIONS, RECORDED BEFORE COMPILING
// --------------------------------------
// P1. Compiles with no feature gate, as p8b did.
// P2. At x, y <= 14 the per-node check REFUSES `(x + y) - y`, where p8b's
//     root-only check licensed it, because the intermediate `x + y` reaches 28
//     and the declared range stops at 15.
// P3. At x, y <= 7 the per-node check LICENSES it and the corner rule still
//     refuses, because the corner root's lower bound is -7.  That is the
//     affine advantage surviving the per-node discipline, which is what `p7`
//     measures and what p8b failed to demonstrate.
// P4. The licensed arm is a bare add and subtract in the emitted code, and the
//     refused arm is absent.

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

/// The per-node check.  A composed grade is a composed type, so the structure
/// the check has to walk is the structure the type already has.
trait AllOk {
    const OK: bool;
}
impl<C: Coeff, V: Coeffs> AllOk for Aff<C, V> {
    const OK: bool = <Aff<C, V> as Discharges>::OK;
}
impl<A, B> AllOk for GAdd<A, B>
where
    A: Grade + AllOk,
    B: Grade + AllOk,
    A::Vec: AddC<B::Vec>,
{
    const OK: bool = A::OK && B::OK && <GAdd<A, B> as Discharges>::OK;
}
impl<A, B> AllOk for GSub<A, B>
where
    A: Grade + AllOk,
    B: Grade + AllOk,
    A::Vec: SubC<B::Vec>,
{
    const OK: bool = A::OK && B::OK && <GSub<A, B> as Discharges>::OK;
}

// ---------------------------------------------------------------------------
// Two symbols, (x, y), in a fixed order.  A leaf declared 0..=b has centre
// b/2 and coefficient b/2 on its own symbol and zero on every other.
// ---------------------------------------------------------------------------

// wide: x, y in 0..=14
type Gx = Aff<Lit<7>, Cons<Lit<7>, Cons<Lit<0>, Nil>>>;
type Gy = Aff<Lit<7>, Cons<Lit<0>, Cons<Lit<7>, Nil>>>;

// narrow: x, y in 0..=6
type Nx = Aff<Lit<3>, Cons<Lit<3>, Cons<Lit<0>, Nil>>>;
type Ny = Aff<Lit<3>, Cons<Lit<0>, Cons<Lit<3>, Nil>>>;

type GSum = GAdd<Gx, Gy>; //  x + y            at the wide declaration
type GDiff = GSub<GSum, Gy>; //  (x + y) - y   at the wide declaration
type NSum = GAdd<Nx, Ny>; //  x + y            at the narrow declaration
type NDiff = GSub<NSum, Ny>; //  (x + y) - y   at the narrow declaration
type GDeep = GSub<GAdd<GAdd<Nx, Ny>, Nx>, GAdd<Nx, Ny>>; //  ((x+y)+x) - (x+y)

// The interval rule for the same terms, computed the same way it is in p7.
const XLO: i32 = 0;
const XHI: i32 = 14;
const YLO: i32 = 0;
const YHI: i32 = 14;
const CORNER_SUM_LO: i32 = XLO + YLO;
const CORNER_SUM_HI: i32 = XHI + YHI;
const CORNER_DIFF_LO: i32 = CORNER_SUM_LO - YHI;
const CORNER_DIFF_HI: i32 = CORNER_SUM_HI - YLO;

// the same at the narrow declaration
const NCORNER_SUM_LO: i32 = 0;
const NCORNER_SUM_HI: i32 = 6 + 6;
const NCORNER_DIFF_LO: i32 = NCORNER_SUM_LO - 6;
const NCORNER_DIFF_HI: i32 = NCORNER_SUM_HI - 0;

// ---------------------------------------------------------------------------
// Arms gated on the associated const.
// ---------------------------------------------------------------------------

#[inline(never)]
#[no_mangle]
pub fn affine_gated_diff(a: u8, b: u8, c: u8) -> u8 {
    if const { <NDiff as AllOk>::OK } {
        (a + b) - c
    } else {
        a.saturating_add(b).saturating_sub(c)
    }
}

#[inline(never)]
#[no_mangle]
pub fn corner_gated_diff(a: u8, b: u8, c: u8) -> u8 {
    if const {
        NCORNER_SUM_LO >= CLO
            && NCORNER_SUM_HI <= CHI
            && NCORNER_DIFF_LO >= CLO
            && NCORNER_DIFF_HI <= CHI
    } {
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
    println!("p8c. The per-node discharge check, recursing over the grade type");
    println!("{}", "=".repeat(70));
    println!();
    println!("  container: [{}, {}]", CLO, CHI);
    println!();
    println!("  WIDE declaration, x and y in 0..=14");
    println!(
        "    x + y            -> [{:>3}, {:>3}]   root-only {}   per-node {}",
        <GSum as Grade>::LO,
        <GSum as Grade>::HI,
        <GSum as Discharges>::OK,
        <GSum as AllOk>::OK
    );
    println!(
        "    (x + y) - y      -> [{:>3}, {:>3}]   root-only {}   per-node {}",
        <GDiff as Grade>::LO,
        <GDiff as Grade>::HI,
        <GDiff as Discharges>::OK,
        <GDiff as AllOk>::OK
    );
    println!(
        "    corner, same term-> [{:>3}, {:>3}]   licenses  {}",
        CORNER_DIFF_LO,
        CORNER_DIFF_HI,
        CORNER_SUM_LO >= CLO
            && CORNER_SUM_HI <= CHI
            && CORNER_DIFF_LO >= CLO
            && CORNER_DIFF_HI <= CHI
    );
    println!();
    println!("    p8b licensed this term. The per-node check refuses it, correctly,");
    println!("    because the intermediate x + y reaches 28 and the range stops at 15.");
    println!();
    println!("  NARROW declaration, x and y in 0..=6");
    println!(
        "    x + y            -> [{:>3}, {:>3}]   per-node {}",
        <NSum as Grade>::LO,
        <NSum as Grade>::HI,
        <NSum as AllOk>::OK
    );
    println!(
        "    (x + y) - y      -> [{:>3}, {:>3}]   per-node {}   radius {}",
        <NDiff as Grade>::LO,
        <NDiff as Grade>::HI,
        <NDiff as AllOk>::OK,
        <<NDiff as Grade>::Vec as Coeffs>::RADIUS
    );
    println!(
        "    corner, same term-> [{:>3}, {:>3}]   licenses  {}",
        NCORNER_DIFF_LO,
        NCORNER_DIFF_HI,
        NCORNER_SUM_LO >= CLO
            && NCORNER_SUM_HI <= CHI
            && NCORNER_DIFF_LO >= CLO
            && NCORNER_DIFF_HI <= CHI
    );
    println!();
    println!("    the affine advantage survives the per-node discipline: the affine");
    println!("    root cancels y and lands in [0, 6]; the corner root keeps both y");
    println!("    occurrences and lands in [-6, 12], whose lower bound is outside.");
    println!();
    println!("  a deeper composition at the narrow declaration, ((x+y)+x) - (x+y):");
    println!(
        "    affine           -> [{:>3}, {:>3}]   per-node {}   vector length {}",
        <GDeep as Grade>::LO,
        <GDeep as Grade>::HI,
        <GDeep as AllOk>::OK,
        <<GDeep as Grade>::Vec as Coeffs>::LEN
    );
    println!("    (the exact value of that term is x, so [0, 6] is the tight answer)");

    println!();
    println!("  BEHAVIOUR at the narrow declaration, on the declared term with c = b:");
    let mut adiff = 0usize;
    let mut cdiff = 0usize;
    let mut n = 0usize;
    for a in 0u8..=6 {
        for b in 0u8..=6 {
            n += 1;
            let want = a as i32;
            if affine_gated_diff(a, b, b) as i32 != want {
                adiff += 1;
            }
            if corner_gated_diff(a, b, b) as i32 != want {
                cdiff += 1;
            }
        }
    }
    println!("    affine-gated arm differs from exact on {}/{}", adiff, n);
    println!("    corner-gated arm differs from exact on {}/{}", cdiff, n);
    println!("    (both are 0: the corner arm is correct too, it is just slower,");
    println!("     because it refused the licence and took the saturating path)");
}
