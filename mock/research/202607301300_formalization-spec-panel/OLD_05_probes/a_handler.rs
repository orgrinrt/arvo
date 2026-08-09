// PROBE A: the resolution IS the handler. One generic arithmetic body, two
// instantiations, no ConstFromResidual bound anywhere.
//
// 02's f2_refusal.rs showed that a body which constructs the refusal ITSELF
// needs `Q::Fallibility<T>: ConstFromResidual<Outcome<Infallible, E>>`, that a
// total composition correctly fails that bound, and concluded "arvo cannot
// have one generic add over all compositions". The conclusion holds only for
// bodies that construct the refusal. Move the construction onto the
// resolution and the body never names a refusal constructor.
//
// OUTCOME: WORKS. Compiled and run under nightly-2026-05-28.
//   A: total=1000 refusing_err=true refusing_ok=true
#![allow(dead_code)]

use notko::{ConstTry, Just, Outcome};

#[derive(Clone, Copy, Debug)]
pub struct OutOfRange;

/// A handler for the out-of-range operation. `Carrier` is the answer type
/// this handler returns into; `over` is the handler body.
pub trait OverRangeRule {
    type Carrier<T: Copy>: ConstTry<Output = T>;
    fn over<T: Copy>(max: T) -> Self::Carrier<T>;
}

pub struct TowardNegative;
impl OverRangeRule for TowardNegative {
    type Carrier<T: Copy> = Just<T>;
    fn over<T: Copy>(max: T) -> Just<T> {
        Just::new(max)
    }
}

pub struct ReduceModulo;
impl OverRangeRule for ReduceModulo {
    type Carrier<T: Copy> = Just<T>;
    fn over<T: Copy>(max: T) -> Just<T> {
        Just::new(max)
    }
}

pub struct Refuse;
impl OverRangeRule for Refuse {
    type Carrier<T: Copy> = Outcome<T, OutOfRange>;
    fn over<T: Copy>(_max: T) -> Outcome<T, OutOfRange> {
        Outcome::Err(OutOfRange)
    }
}

// ONE body. No ConstFromResidual. No branch on which resolution this is.
fn add<R: OverRangeRule>(a: u16, b: u16, max: u16) -> R::Carrier<u16> {
    match a.checked_add(b) {
        Some(v) if v <= max => <R::Carrier<u16> as ConstTry>::from_output(v),
        _ => R::over(max),
    }
}

fn main() {
    let t: Just<u16> = add::<TowardNegative>(60000, 60000, 1000);
    let r: Outcome<u16, OutOfRange> = add::<Refuse>(60000, 60000, 1000);
    let ok: Outcome<u16, OutOfRange> = add::<Refuse>(3, 4, 1000);
    println!(
        "A: total={} refusing_err={} refusing_ok={}",
        t.get(),
        r.is_err(),
        ok.is_ok()
    );
}
