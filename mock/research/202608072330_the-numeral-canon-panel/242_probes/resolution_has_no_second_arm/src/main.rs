//! Seat 242 attacking seat 241. Does the resolution object have a second arm?
//!
//! `241_kiselyov_admission_is_a_resolution_not_a_verdict.md` section 2 defines
//! admission as returning "either a total assignment of the ratified coordinate
//! set, or the name of a coordinate the candidate failed to fix", and argues
//! that predicate and location are lossy projections of that sum.
//!
//! A sum type with an uninhabited second arm is its first arm. So the question
//! this probe asks is whether the second arm is inhabited at the tier 241 names,
//! and whether the diagnosis 241 uses to motivate the shape is reachable from it.
//!
//! **The cases that must fail, stated before the run.**
//!
//! 1. The four algebras must resolve to IDENTICAL assignments, or arm 1 is not
//!    reproducing 241's probe 1 and nothing here bears on its claim.
//! 2. The radix control must MOVE, or the instrument cannot see a coordinate
//!    change at all and every "identical" above is worthless.
//! 3. The `omit_a_coordinate` feature must REFUSE TO COMPILE, or the second arm
//!    is inhabited and 241's shape stands as written.
//!
//! Arm 3 is the whole finding. It is a separate feature because a compile
//! failure cannot share a build with a result.

use arvo_format::{
    contains, exponent_at, has_additive_identity, radix, slot_count, Ambient, Format, Quantum,
    Slots, Width,
};

/// The ten coordinates, resolved. This IS 241's "total assignment": everything a
/// candidate fixes, read back out through the crate's own observations.
#[derive(PartialEq, Eq, Debug)]
struct Resolved {
    radix: u32,
    signed: bool,
    exp_at_0: i32,
    exp_at_1: i32,
    magnitudes_ok: bool,
    min: i64,
    max: i64,
    width: u32,
    slots: i64,
    has_zero: bool,
    contains_origin: bool,
}

/// Resolution, as 241 defines it, over the ratified ten. Total by construction:
/// there is no second arm to return, because every coordinate is an associated
/// item the impl was obliged to fix in order to exist.
const fn resolve<F: Format>() -> Resolved {
    Resolved {
        radix: radix::<F>(),
        signed: <F::Ambient as Ambient>::SIGNED,
        exp_at_0: exponent_at::<F::Quantum>(0),
        exp_at_1: exponent_at::<F::Quantum>(1),
        magnitudes_ok: <F::Quantum as Quantum>::MAGNITUDES > 0,
        min: <F::Slots as Slots>::MIN,
        max: <F::Slots as Slots>::MAX,
        width: <F::Slots as Slots>::WIDTH.count(),
        slots: slot_count::<F::Slots>(),
        has_zero: has_additive_identity::<F>(),
        contains_origin: contains::<F>(0, 0),
    }
}

// --- one quantum and one slot range, shared, so only the ambient varies -------

pub struct Q6;
impl Quantum for Q6 {
    const BASE: i32 = -3;
    const SLOPE: i32 = 1;
    const MAGNITUDES: u32 = 4;
}
pub struct S6;
impl Slots for S6 {
    const MIN: i64 = -32;
    const MAX: i64 = 31;
    const WIDTH: Width = Width::bits(6);
}

/// Four ambient ALGEBRAS, following 241's probe 1: the rationals under plus and
/// times, the tropical semiring under min and plus, the two-element Boolean
/// algebra under and and or, and the interval algebra. Three of the four are not
/// about magnitude. The algebra lives in this comment, which is 241's point.
macro_rules! ambient {
    ($($name:ident),+) => { $( pub struct $name;
        impl Ambient for $name { const RADIX: u32 = 2; const SIGNED: bool = true; } )+ };
}
ambient!(Rationals, Tropical, Boolean, IntervalAlgebra);

/// The negative control: a declared coordinate genuinely moves.
pub struct DecimalControl;
impl Ambient for DecimalControl {
    const RADIX: u32 = 10;
    const SIGNED: bool = true;
}

macro_rules! fmt_over {
    ($f:ident, $a:ty) => {
        pub struct $f;
        impl Format for $f {
            type Ambient = $a;
            type Quantum = Q6;
            type Slots = S6;
            const PHASE_NUM: i64 = 0;
            const PHASE_DEN: i64 = 1;
        }
    };
}
fmt_over!(FRationals, Rationals);
fmt_over!(FTropical, Tropical);
fmt_over!(FBoolean, Boolean);
fmt_over!(FInterval, IntervalAlgebra);
fmt_over!(FControl, DecimalControl);

// --- the control that must refuse ---------------------------------------------

/// A candidate that leaves `PHASE_DEN` unfixed. If 241's second arm is
/// inhabited, this is what inhabits it: a candidate that resolved everything
/// except one coordinate, whose admission should return that coordinate's name.
#[cfg(feature = "omit_a_coordinate")]
pub struct Underdetermined;
#[cfg(feature = "omit_a_coordinate")]
impl Format for Underdetermined {
    type Ambient = Rationals;
    type Quantum = Q6;
    type Slots = S6;
    const PHASE_NUM: i64 = 0;
    // PHASE_DEN deliberately not supplied.
}

fn main() {
    let r = resolve::<FRationals>();
    let t = resolve::<FTropical>();
    let b = resolve::<FBoolean>();
    let i = resolve::<FInterval>();
    let c = resolve::<FControl>();

    println!("resolution of four DIFFERENT ambient algebras over one grid:");
    for (n, v) in [
        ("rationals (+,*)  ", &r),
        ("tropical (min,+) ", &t),
        ("boolean (and,or) ", &b),
        ("interval algebra ", &i),
    ] {
        println!("  {n}: {v:?}");
    }
    println!("  CONTROL radix 10 : {c:?}");

    // Case 1: all four must be identical, or this is not 241's finding.
    assert_eq!(r, t);
    assert_eq!(r, b);
    assert_eq!(r, i);
    // Case 2: the control must move, or the instrument is blind.
    assert_ne!(r, c);

    println!();
    println!("case 1 holds: four different algebras, one identical resolution.");
    println!("case 2 holds: the negative control moves, so the instrument sees a coordinate.");
    println!();
    println!("FINDING: every one of the four RESOLVED. None returned the name of a");
    println!("missing coordinate. The operation family is not among the ten, so a");
    println!("resolution over the ten cannot report it missing: the diagnosis 241");
    println!("cites to motivate the shape is not reachable from the shape.");
    println!();
    println!("Now build with --features omit_a_coordinate for the second arm.");
}
