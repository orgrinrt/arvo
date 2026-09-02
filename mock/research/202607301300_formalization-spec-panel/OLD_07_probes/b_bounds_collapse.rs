//! PROBE B: the carrier-join bounds collapse into one blanket impl, and
//! every arithmetic body bounds on ONE name.
//!
//! 05 section 1 measured the honest cost of the computed carrier join, a
//! five-line `where` clause per arithmetic function, and named without
//! compiling the repair, a blanket extension trait. This compiles it, with
//! the addition that makes it actually work: the join and the lifts are not
//! only computed once, they are CONSUMED once, because the resolution
//! operations become methods of the aggregate. The bounds live at the
//! blanket impl; every arithmetic body sees `Q: QuantExt` and nothing else.
//!
//! Two structural points the panel file discusses:
//!   1. the grade is a two-point join monoid (No <= Yes, join = or) and the
//!      lift is stated once per grade PAIR, not once per (rule, T) pair,
//!      which is what keeps the bounds finite;
//!   2. this is 05's evidence-passing shape one level up: the aggregate
//!      impl captures the evidence, the operation calls through it.
#![allow(dead_code)]

// --- a minimal carrier pair, standing in for notko's Just / Outcome --------

pub trait Carrier<T: Copy> {
    fn from_output(v: T) -> Self;
}
#[derive(Debug)]
pub struct Total<T>(pub T);
impl<T: Copy> Carrier<T> for Total<T> {
    fn from_output(v: T) -> Self {
        Total(v)
    }
}
#[derive(Debug)]
pub enum Fallible<T> {
    Ok(T),
    Refused,
}
impl<T: Copy> Carrier<T> for Fallible<T> {
    fn from_output(v: T) -> Self {
        Fallible::Ok(v)
    }
}

// --- the grade: a two-point join monoid ------------------------------------

pub struct No;
pub struct Yes;

pub trait Or<B> {
    type Out;
}
impl Or<No> for No {
    type Out = No;
}
impl Or<Yes> for No {
    type Out = Yes;
}
impl<B> Or<B> for Yes {
    type Out = Yes;
}

/// Grade -> carrier: the interpretation of a grade.
pub trait CarrierOf {
    type C<T: Copy>: Carrier<T>;
}
impl CarrierOf for No {
    type C<T: Copy> = Total<T>;
}
impl CarrierOf for Yes {
    type C<T: Copy> = Fallible<T>;
}

/// Grade subsumption, stated once per grade pair rather than per (rule, T)
/// pair. Three impls cover the whole lattice.
pub trait LiftGrade<G: CarrierOf>: CarrierOf {
    fn lift<T: Copy>(x: Self::C<T>) -> G::C<T>;
}
impl LiftGrade<No> for No {
    fn lift<T: Copy>(x: Total<T>) -> Total<T> {
        x
    }
}
impl LiftGrade<Yes> for No {
    fn lift<T: Copy>(x: Total<T>) -> Fallible<T> {
        Fallible::Ok(x.0)
    }
}
impl LiftGrade<Yes> for Yes {
    fn lift<T: Copy>(x: Fallible<T>) -> Fallible<T> {
        x
    }
}

// --- range rules as handlers (05 probe A's shape, graded) ------------------

pub trait RangeRule {
    type Grade: CarrierOf;
    /// what this rule returns at an out-of-range point, in its own grade's
    /// carrier; the aggregate lifts it into the composition's carrier.
    fn resolve<T: Copy>(nearest: T) -> <Self::Grade as CarrierOf>::C<T>;
}

pub struct ClampRule;
impl RangeRule for ClampRule {
    type Grade = No;
    fn resolve<T: Copy>(nearest: T) -> Total<T> {
        Total(nearest)
    }
}
pub struct RefuseRule;
impl RangeRule for RefuseRule {
    type Grade = Yes;
    fn resolve<T: Copy>(_nearest: T) -> Fallible<T> {
        Fallible::Refused
    }
}

// --- the quantisation, spec-shaped -----------------------------------------

pub trait Quantisation {
    type Over: RangeRule;
    type Under: RangeRule;
}

pub struct Saturating;
impl Quantisation for Saturating {
    type Over = ClampRule;
    type Under = ClampRule;
}
pub struct PreciseQ;
impl Quantisation for PreciseQ {
    type Over = RefuseRule;
    type Under = RefuseRule;
}
pub struct MixedHi;
impl Quantisation for MixedHi {
    type Over = RefuseRule;
    type Under = ClampRule;
}

pub type OverGrade<Q> = <<Q as Quantisation>::Over as RangeRule>::Grade;
pub type UnderGrade<Q> = <<Q as Quantisation>::Under as RangeRule>::Grade;
pub type JoinOf<Q> = <OverGrade<Q> as Or<UnderGrade<Q>>>::Out;

// --- THE AGGREGATE. Every join and lift bound lives here, once. ------------

pub trait QuantExt {
    type Answer<T: Copy>: Carrier<T>;
    fn ok<T: Copy>(v: T) -> Self::Answer<T>;
    fn over<T: Copy>(max: T) -> Self::Answer<T>;
    fn under<T: Copy>(min: T) -> Self::Answer<T>;
}

impl<Q: Quantisation> QuantExt for Q
where
    OverGrade<Q>: Or<UnderGrade<Q>> + LiftGrade<JoinOf<Q>>,
    UnderGrade<Q>: LiftGrade<JoinOf<Q>>,
    JoinOf<Q>: CarrierOf,
{
    type Answer<T: Copy> = <JoinOf<Q> as CarrierOf>::C<T>;
    fn ok<T: Copy>(v: T) -> Self::Answer<T> {
        <Self::Answer<T> as Carrier<T>>::from_output(v)
    }
    fn over<T: Copy>(max: T) -> Self::Answer<T> {
        <OverGrade<Q> as LiftGrade<JoinOf<Q>>>::lift(<Q::Over as RangeRule>::resolve(max))
    }
    fn under<T: Copy>(min: T) -> Self::Answer<T> {
        <UnderGrade<Q> as LiftGrade<JoinOf<Q>>>::lift(<Q::Under as RangeRule>::resolve(min))
    }
}

// --- the arithmetic. ONE bound, no join, no lift, no projection. -----------

pub fn add<Q: QuantExt>(a: u16, b: u16, max: u16) -> Q::Answer<u16> {
    match a.checked_add(b) {
        Some(v) if v <= max => Q::ok(v),
        _ => Q::over(max),
    }
}

pub fn sub<Q: QuantExt>(a: u16, b: u16, min: u16) -> Q::Answer<u16> {
    match a.checked_sub(b) {
        Some(v) if v >= min => Q::ok(v),
        _ => Q::under(min),
    }
}

fn main() {
    let s: Total<u16> = add::<Saturating>(90, 40, 100);
    let p: Fallible<u16> = add::<PreciseQ>(90, 40, 100);
    let hi: Fallible<u16> = add::<MixedHi>(90, 40, 100);
    let lo: Fallible<u16> = sub::<MixedHi>(10, 40, 0);
    println!(
        "B: sat={:?} precise={:?} mixed_hi={:?} mixed_lo={:?}",
        s.0, p, hi, lo
    );
}
