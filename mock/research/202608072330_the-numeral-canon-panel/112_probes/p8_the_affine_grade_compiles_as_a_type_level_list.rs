// p8. The affine grade, carried in the typestate, compiled.
//
// p7 establishes that an affine grade recovers every licence the corner rule
// loses to correlation, and argues it is const-shaped because the coefficient
// vector's length is (leaves + non-constant multiplications), both static
// properties of a term.  That is an argument and not a compile, and this panel
// has been burned by a claim of expressibility nobody checked.
//
// The obvious spelling puts the vector in an associated const array whose
// length is another associated const, which needs arithmetic in type position
// and therefore `generic_const_exprs`, which is FORBIDDEN.  The workspace's
// standing reflex is that a refused bound is a trait nobody has named yet, so
// the coefficient vector is a type-level list instead: one cell per noise
// symbol, in a fixed order, with the coefficient as a const argument and the
// derived quantities as associated consts computed in their own bodies, where
// arbitrary const expressions are legal.
//
// PREDICTIONS, RECORDED BEFORE COMPILING
// --------------------------------------
// P1. It compiles with NO `#![feature(...)]` line at all.
// P2. Addition of two grades is a type-level pointwise sum and its radius
//     comes out exactly right, so the two occurrences of one leaf cancel and
//     `(x + y) - y` carries `y`'s coefficient at zero.
// P3. The discharge test is an associated const boolean, so it gates an arm
//     through a `const { }` block and the refused arm is absent from the
//     emitted code.
// P4. The numbers agree with p7's model: at x <= 15, y <= 15 over a container
//     of 0..=15, the corner rule refuses `(x + y) - y` and the affine rule
//     licenses it.

use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// The coefficient vector as a type-level list, one cell per noise symbol.
// ---------------------------------------------------------------------------

trait Coeffs {
    /// Sum of the absolute coefficients: the form's radius.
    const RADIUS: i32;
}

struct Nil;
struct Cons<const C: i32, T>(PhantomData<T>);

impl Coeffs for Nil {
    const RADIUS: i32 = 0;
}
impl<const C: i32, T: Coeffs> Coeffs for Cons<C, T> {
    const RADIUS: i32 = {
        let a = if C < 0 { -C } else { C };
        a + T::RADIUS
    };
}

/// Pointwise sum of two coefficient vectors of the same shape.
trait AddC<Other> {
    type Out: Coeffs;
}
impl AddC<Nil> for Nil {
    type Out = Nil;
}
impl<const A: i32, const B: i32, S, T> AddC<Cons<B, T>> for Cons<A, S>
where
    S: AddC<T>,
{
    type Out = Cons<{ A + B }, <S as AddC<T>>::Out>;
}

/// Pointwise difference.
trait SubC<Other> {
    type Out: Coeffs;
}
impl SubC<Nil> for Nil {
    type Out = Nil;
}
impl<const A: i32, const B: i32, S, T> SubC<Cons<B, T>> for Cons<A, S>
where
    S: SubC<T>,
{
    type Out = Cons<{ A - B }, <S as SubC<T>>::Out>;
}

// ---------------------------------------------------------------------------
// A grade: a centre and a coefficient vector.
// ---------------------------------------------------------------------------

trait Grade {
    const CENTRE: i32;
    const RADIUS: i32;
    const LO: i32 = Self::CENTRE - Self::RADIUS;
    const HI: i32 = Self::CENTRE + Self::RADIUS;
}

struct Aff<const CENTRE: i32, C>(PhantomData<C>);

impl<const CENTRE: i32, C: Coeffs> Grade for Aff<CENTRE, C> {
    const CENTRE: i32 = CENTRE;
    const RADIUS: i32 = C::RADIUS;
}

/// The grade of a sum: centres add and coefficient vectors add pointwise.
struct GAdd<A, B>(PhantomData<(A, B)>);

impl<const CA: i32, const CB: i32, A, B> Grade for GAdd<Aff<CA, A>, Aff<CB, B>>
where
    A: Coeffs + AddC<B>,
    B: Coeffs,
{
    const CENTRE: i32 = CA + CB;
    const RADIUS: i32 = <A as AddC<B>>::Out::RADIUS;
}

/// The grade of a difference.
struct GSub<A, B>(PhantomData<(A, B)>);

impl<const CA: i32, const CB: i32, A, B> Grade for GSub<Aff<CA, A>, Aff<CB, B>>
where
    A: Coeffs + SubC<B>,
    B: Coeffs,
{
    const CENTRE: i32 = CA - CB;
    const RADIUS: i32 = <A as SubC<B>>::Out::RADIUS;
}

// ---------------------------------------------------------------------------
// The container, and the discharge test as an associated const.
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
// The two symbols this program uses, in a fixed order: (x, y).
// A leaf declared 0..=b is centre b/2 with that leaf's coefficient at b/2.
// ---------------------------------------------------------------------------

// x in 0..=14, y in 0..=14 (even so the centre is an integer; the model is
// exact rationals, and a shipping design would carry a scale)
type Gx = Aff<7, Cons<7, Cons<0, Nil>>>;
type Gy = Aff<7, Cons<0, Cons<7, Nil>>>;

// (x + y)
type GSum = GAdd<Gx, Gy>;
// (x + y) - y, with the y coefficients cancelling.  Written out rather than
// composed through GAdd's output type because GAdd reports a radius and not a
// vector; a shipping design would return the vector, and the point being made
// here is that the cancellation is expressible, which the next type shows.
type GxPlusY = Aff<14, Cons<7, Cons<7, Nil>>>;
type GDiff = GSub<GxPlusY, Gy>;

// The interval rule's answer for the same term, for the comparison.
const CORNER_LO: i32 = 0 + 0 - 14;
const CORNER_HI: i32 = 14 + 14 - 0;

// ---------------------------------------------------------------------------
// The arm, gated on the associated const.
// ---------------------------------------------------------------------------

#[inline(never)]
#[no_mangle]
pub fn affine_gated_add(a: u8, b: u8) -> u8 {
    if const { <GSum as Discharges>::OK } {
        a + b
    } else {
        a.saturating_add(b)
    }
}

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
pub fn general_diff(a: u8, b: u8, c: u8) -> u8 {
    a.saturating_add(b).saturating_sub(c)
}

fn main() {
    println!("p8. The affine grade compiles as a type-level list");
    println!("{}", "=".repeat(70));
    println!();
    println!("  container: [{}, {}]", CLO, CHI);
    println!();
    println!(
        "  Gx      centre {:>4} radius {:>4} -> [{}, {}]",
        <Gx as Grade>::CENTRE,
        <Gx as Grade>::RADIUS,
        <Gx as Grade>::LO,
        <Gx as Grade>::HI
    );
    println!(
        "  Gy      centre {:>4} radius {:>4} -> [{}, {}]",
        <Gy as Grade>::CENTRE,
        <Gy as Grade>::RADIUS,
        <Gy as Grade>::LO,
        <Gy as Grade>::HI
    );
    println!(
        "  x + y   centre {:>4} radius {:>4} -> [{}, {}]   discharges: {}",
        <GSum as Grade>::CENTRE,
        <GSum as Grade>::RADIUS,
        <GSum as Grade>::LO,
        <GSum as Grade>::HI,
        <GSum as Discharges>::OK
    );
    println!();
    println!("  the correlated term, (x + y) - y:");
    println!(
        "    affine  centre {:>4} radius {:>4} -> [{}, {}]   discharges: {}",
        <GDiff as Grade>::CENTRE,
        <GDiff as Grade>::RADIUS,
        <GDiff as Grade>::LO,
        <GDiff as Grade>::HI,
        <GDiff as Discharges>::OK
    );
    println!(
        "    corner                             -> [{}, {}]   discharges: {}",
        CORNER_LO,
        CORNER_HI,
        CORNER_LO >= CLO && CORNER_HI <= CHI
    );
    println!();
    println!("  so the affine rule licenses what the corner rule refuses, and the");
    println!("  cancellation happened in the type rather than at runtime.");

    // behaviour, checked rather than asserted
    let mut differ = 0usize;
    let mut n = 0usize;
    for a in 0u8..=14 {
        for b in 0u8..=14 {
            for c in 0u8..=14 {
                if (a as i32 + b as i32 - c as i32) < 0 || (a as i32 + b as i32 - c as i32) > 15 {
                    continue;
                }
                if c != b {
                    continue; // the declared term is (x + y) - y
                }
                n += 1;
                if affine_gated_diff(a, b, c) != (a as i32 + b as i32 - c as i32) as u8 {
                    differ += 1;
                }
            }
        }
    }
    println!();
    println!(
        "  on the declared term, the gated arm differs from exact on {}/{} tuples",
        differ, n
    );
}
