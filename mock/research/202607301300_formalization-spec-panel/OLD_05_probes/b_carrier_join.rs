// PROBE B: two range positions, possibly different rules, one generic body.
// The composition's carrier is the JOIN of its handlers' carriers; each
// handler returns its own carrier and the body lifts. This is the row-
// subsumption move from effect systems: a computation using a subset of the
// effects embeds into the larger row, and the embedding is the identity
// wherever the rows coincide.
//
// OUTCOME: WORKS under nightly-2026-05-28.
//   B: sat=100 precise_err=true mixed_hi_err=true mixed_lo=true
// The Mixed case (refuse above, clamp below) is the one that matters: it
// carries Outcome, refuses above, and returns Ok(clamped) below.
//
// COST TO NOTE: the where-clause on `add` runs to five lines. That is the
// honest price of computing the carrier rather than declaring it, and it is
// the thing to weigh against the declared `Fallibility<T>` the spec has.
#![allow(dead_code)]

use notko::{ConstTry, Just, Outcome};

#[derive(Clone, Copy, Debug)]
pub struct OutOfRange;

pub trait Truth {}
pub struct Yes;
impl Truth for Yes {}
pub struct No;
impl Truth for No {}

pub trait Or<R> {
    type Out: Truth;
}
impl Or<Yes> for Yes {
    type Out = Yes;
}
impl Or<No> for Yes {
    type Out = Yes;
}
impl Or<Yes> for No {
    type Out = Yes;
}
impl Or<No> for No {
    type Out = No;
}

/// Projection from "can this composition refuse" to the answer type.
pub trait CarrierOf<T: Copy> {
    type C: ConstTry<Output = T>;
}
impl<T: Copy> CarrierOf<T> for No {
    type C = Just<T>;
}
impl<T: Copy> CarrierOf<T> for Yes {
    type C = Outcome<T, OutOfRange>;
}

/// Embedding of a smaller carrier into a larger one. Identity where equal.
pub trait Lift<Into> {
    fn lift(self) -> Into;
}
impl<T: Copy> Lift<Just<T>> for Just<T> {
    fn lift(self) -> Just<T> {
        self
    }
}
impl<T: Copy> Lift<Outcome<T, OutOfRange>> for Outcome<T, OutOfRange> {
    fn lift(self) -> Outcome<T, OutOfRange> {
        self
    }
}
impl<T: Copy> Lift<Outcome<T, OutOfRange>> for Just<T> {
    fn lift(self) -> Outcome<T, OutOfRange> {
        Outcome::Ok(self.unwrap())
    }
}

pub trait RangeRule {
    type CanRefuse: Truth;
    type Carrier<T: Copy>: ConstTry<Output = T>;
    fn resolve<T: Copy>(bound: T) -> Self::Carrier<T>;
}

pub struct TowardNegative;
impl RangeRule for TowardNegative {
    type CanRefuse = No;
    type Carrier<T: Copy> = Just<T>;
    fn resolve<T: Copy>(bound: T) -> Just<T> {
        Just::new(bound)
    }
}
pub struct TowardPositive;
impl RangeRule for TowardPositive {
    type CanRefuse = No;
    type Carrier<T: Copy> = Just<T>;
    fn resolve<T: Copy>(bound: T) -> Just<T> {
        Just::new(bound)
    }
}
pub struct Refuse;
impl RangeRule for Refuse {
    type CanRefuse = Yes;
    type Carrier<T: Copy> = Outcome<T, OutOfRange>;
    fn resolve<T: Copy>(_bound: T) -> Outcome<T, OutOfRange> {
        Outcome::Err(OutOfRange)
    }
}

pub trait Quantisation {
    type Over: RangeRule;
    type Under: RangeRule;
}

pub type Answer<Q, T> = <<<<Q as Quantisation>::Over as RangeRule>::CanRefuse as Or<
    <<Q as Quantisation>::Under as RangeRule>::CanRefuse,
>>::Out as CarrierOf<T>>::C;

fn add<Q, T>(a: i32, b: i32, lo: T, hi: T, to_t: fn(i32) -> T, lo_i: i32, hi_i: i32) -> Answer<Q, T>
where
    T: Copy,
    Q: Quantisation,
    <Q::Over as RangeRule>::CanRefuse: Or<<Q::Under as RangeRule>::CanRefuse>,
    <<Q::Over as RangeRule>::CanRefuse as Or<<Q::Under as RangeRule>::CanRefuse>>::Out:
        CarrierOf<T>,
    <Q::Over as RangeRule>::Carrier<T>: Lift<Answer<Q, T>>,
    <Q::Under as RangeRule>::Carrier<T>: Lift<Answer<Q, T>>,
{
    let exact = a + b;
    if exact > hi_i {
        <Q::Over as RangeRule>::resolve(hi).lift()
    } else if exact < lo_i {
        <Q::Under as RangeRule>::resolve(lo).lift()
    } else {
        <Answer<Q, T> as ConstTry>::from_output(to_t(exact))
    }
}

pub struct Saturating;
impl Quantisation for Saturating {
    type Over = TowardNegative;
    type Under = TowardPositive;
}
pub struct Precise;
impl Quantisation for Precise {
    type Over = Refuse;
    type Under = Refuse;
}
pub struct Mixed;
impl Quantisation for Mixed {
    type Over = Refuse;
    type Under = TowardPositive;
}

fn main() {
    let s: Just<i16> = add::<Saturating, i16>(30000, 30000, -100, 100, |v| v as i16, -100, 100);
    let p: Outcome<i16, OutOfRange> =
        add::<Precise, i16>(30000, 30000, -100, 100, |v| v as i16, -100, 100);
    let m_hi: Outcome<i16, OutOfRange> =
        add::<Mixed, i16>(30000, 30000, -100, 100, |v| v as i16, -100, 100);
    let m_lo: Outcome<i16, OutOfRange> =
        add::<Mixed, i16>(-30000, -30000, -100, 100, |v| v as i16, -100, 100);
    println!(
        "B: sat={} precise_err={} mixed_hi_err={} mixed_lo={}",
        s.get(),
        p.is_err(),
        m_hi.is_err(),
        m_lo.is_ok()
    );
}
