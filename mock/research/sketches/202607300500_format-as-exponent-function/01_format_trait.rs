// Probe 01: the format concept as a trait carrying an exponent function.
//
// Hypothesis: Flocq's `cexp beta fexp x = fexp (mag beta x)` applies fexp to a
// VALUE's magnitude exponent, so fexp never reaches type position, and the whole
// format concept compiles with zero feature gates.
//
// The type carries only PREC, EMIN and F, which are plain const parameters.
//
// Run: rustc --edition 2021 01_format_trait.rs -o /tmp/p01 && /tmp/p01

use core::marker::PhantomData;

// --- underflow policies, the axis FLX / FLT / FTZ actually differ on --------

pub trait Underflow {
    /// Canonical exponent for magnitude exponent `e`, given precision and the
    /// exponent floor. Each discipline decides what happens below the floor.
    fn fexp(e: i32, prec: i32, emin: i32) -> i32;
}

/// FLX: no floor at all. Precision is always exactly `prec`.
pub struct Unbounded;
/// FLT: subnormals. IEEE 754's choice.
pub struct Gradual;
/// FTZ: abrupt, flush to zero.
pub struct Flushed;

impl Underflow for Unbounded {
    #[inline]
    fn fexp(e: i32, prec: i32, _emin: i32) -> i32 {
        e - prec
    }
}

impl Underflow for Gradual {
    #[inline]
    fn fexp(e: i32, prec: i32, emin: i32) -> i32 {
        // FLT_exp e = max(e - prec, emin)
        let x = e - prec;
        if x < emin {
            emin
        } else {
            x
        }
    }
}

impl Underflow for Flushed {
    #[inline]
    fn fexp(e: i32, prec: i32, emin: i32) -> i32 {
        // FTZ_exp e = if e - prec < emin then emin + prec - 1 else e - prec
        let x = e - prec;
        if x < emin {
            emin + prec - 1
        } else {
            x
        }
    }
}

// --- the format concept ----------------------------------------------------

pub trait Format {
    /// Canonical exponent for a value whose magnitude exponent is `e`.
    fn fexp(e: i32) -> i32;
}

/// FIX: constant exponent. The radix point is nailed down and magnitude is
/// ignored entirely. `F` fractional bits means the exponent is always -F.
pub struct Fixed<const F: i32>;

/// FLX / FLT / FTZ, distinguished only by the underflow policy `U`.
pub struct Floating<const PREC: i32, const EMIN: i32, U>(PhantomData<U>);

impl<const F: i32> Format for Fixed<F> {
    #[inline]
    fn fexp(_e: i32) -> i32 {
        -F
    }
}

impl<const PREC: i32, const EMIN: i32, U: Underflow> Format for Floating<PREC, EMIN, U> {
    #[inline]
    fn fexp(e: i32) -> i32 {
        U::fexp(e, PREC, EMIN)
    }
}

// --- a consumer generic over the format, which is the real test ------------

/// Threads a format through its own generic code, the shape recorded as having
/// overflowed the well-formedness evaluator under the old const-expression form.
fn canonical_exponent<F: Format>(mag: i32) -> i32 {
    F::fexp(mag)
}

fn main() {
    // FIX ignores magnitude entirely.
    assert_eq!(canonical_exponent::<Fixed<16>>(0), -16);
    assert_eq!(canonical_exponent::<Fixed<16>>(100), -16);
    assert_eq!(canonical_exponent::<Fixed<16>>(-100), -16);

    // FLX: e - prec, unbounded below. binary32-ish precision, no floor.
    type Flx = Floating<24, -126, Unbounded>;
    assert_eq!(canonical_exponent::<Flx>(0), -24);
    assert_eq!(canonical_exponent::<Flx>(-1000), -1024);

    // FLT: max(e - prec, emin). IEEE binary32 parameters.
    type Flt = Floating<24, -149, Gradual>;
    assert_eq!(canonical_exponent::<Flt>(0), -24);
    assert_eq!(canonical_exponent::<Flt>(-200), -149); // clamped, subnormal region

    // FTZ: same until the floor, then emin + prec - 1.
    type Ftz = Floating<24, -149, Flushed>;
    assert_eq!(canonical_exponent::<Ftz>(0), -24);
    assert_eq!(canonical_exponent::<Ftz>(-200), -149 + 24 - 1);

    // The three underflow policies genuinely differ where it matters and agree
    // where they should, which is the point of separating the axis.
    assert_eq!(canonical_exponent::<Flx>(0), canonical_exponent::<Flt>(0));
    assert_ne!(
        canonical_exponent::<Flt>(-200),
        canonical_exponent::<Ftz>(-200)
    );

    println!("01 WORKS: format as exponent function, zero feature gates");
}
