// Probe B for panel file 10. The preservation door biting.
//
// Parametricity forecloses one class of carrier lie (a carrier cannot
// fabricate a payload value; probe C). It does NOT foreclose branch lies:
// a carrier can drop the value and claim a refusal, or in general route
// the two Rec branches wrongly, because both bodies are writable with
// only `T: Copy` in scope. This probe writes exactly that lying carrier
// and shows the preservation equation refuses it at compile time.
//
// EXPECTED OUTCOME: does not compile. E0080, evaluation panicked, at the
// `preserved` assert, naming the composition being checked.
//
// rustc +nightly-2026-05-28 --edition 2024 b_lying_carrier_caught.rs
#![feature(const_trait_impl)]
#![allow(dead_code)]

pub const trait Payload: Copy {
    type Wide: Copy;
    fn widen(self) -> Self::Wide;
    fn wadd(a: Self::Wide, b: Self::Wide) -> Self::Wide;
    fn wsub(a: Self::Wide, b: Self::Wide) -> Self::Wide;
    fn wgt(a: Self::Wide, b: Self::Wide) -> bool;
    fn wlt(a: Self::Wide, b: Self::Wide) -> bool;
    fn wrem_euclid(a: Self::Wide, m: Self::Wide) -> Self::Wide;
    fn wone() -> Self::Wide;
    fn narrow(w: Self::Wide) -> Self;
    fn to_model(self) -> i64;
}

#[derive(Copy, Clone)]
pub struct M3(pub u8);
const impl Payload for M3 {
    type Wide = i64;
    fn widen(self) -> i64 {
        self.0 as i64
    }
    fn wadd(a: i64, b: i64) -> i64 {
        a + b
    }
    fn wsub(a: i64, b: i64) -> i64 {
        a - b
    }
    fn wgt(a: i64, b: i64) -> bool {
        a > b
    }
    fn wlt(a: i64, b: i64) -> bool {
        a < b
    }
    fn wrem_euclid(a: i64, m: i64) -> i64 {
        a.rem_euclid(m)
    }
    fn wone() -> i64 {
        1
    }
    fn narrow(w: i64) -> Self {
        M3(w as u8)
    }
    fn to_model(self) -> i64 {
        self.0 as i64
    }
}

#[derive(Copy, Clone)]
pub enum Rec<T: Copy> {
    At(T),
    Refused,
}

pub const trait Resolve {
    fn phi<P: [const] Payload>(x: P::Wide, min: P, max: P) -> Rec<P>;
}

pub struct ReduceModulo;
const impl Resolve for ReduceModulo {
    fn phi<P: [const] Payload>(x: P::Wide, min: P, max: P) -> Rec<P> {
        let lo = min.widen();
        let hi = max.widen();
        let span = P::wadd(P::wsub(hi, lo), P::wone());
        Rec::At(P::narrow(P::wadd(P::wrem_euclid(P::wsub(x, lo), span), lo)))
    }
}

pub const trait CarrierC<T: Copy>: Copy {
    fn from_output(v: T) -> Self;
    fn refused() -> Self;
    fn observe(self) -> Rec<T>;
}

/// The lie. Writable under parametricity: no value is fabricated, one is
/// dropped. `from_output` claims a refusal for a value phi recovered.
/// In the union's terms: a Deliver impl whose behaviour differs from the
/// witnessed semantics, compiled silently there, refused here.
#[derive(Copy, Clone)]
pub enum Lying<T: Copy> {
    Ok(T),
    Refused,
}
const impl<T: Copy> CarrierC<T> for Lying<T> {
    fn from_output(_v: T) -> Self {
        Lying::Refused // drops the recovered value, claims refusal
    }
    fn refused() -> Self {
        Lying::Refused
    }
    fn observe(self) -> Rec<T> {
        match self {
            Lying::Ok(v) => Rec::At(v),
            Lying::Refused => Rec::Refused,
        }
    }
}

pub const fn pipeline_add<R, P, C>(a: P, b: P, min: P, max: P) -> C
where
    R: [const] Resolve,
    P: [const] Payload,
    C: [const] CarrierC<P>,
{
    let exact = P::wadd(a.widen(), b.widen());
    match R::phi::<P>(exact, min, max) {
        Rec::At(v) => C::from_output(v),
        Rec::Refused => C::refused(),
    }
}

pub const fn preserved<R: [const] Resolve, C: [const] CarrierC<M3>>() -> bool {
    let min = M3(0);
    let max = M3(7);
    let mut a = 0u8;
    while a <= 7 {
        let mut b = 0u8;
        while b <= 7 {
            let exact = M3(a).widen() + M3(b).widen();
            let spec = R::phi::<M3>(exact, min, max);
            let got = pipeline_add::<R, M3, C>(M3(a), M3(b), min, max).observe();
            let eq = match (spec, got) {
                (Rec::At(x), Rec::At(y)) => x.to_model() == y.to_model(),
                (Rec::Refused, Rec::Refused) => true,
                _ => false,
            };
            if !eq {
                return false;
            }
            b += 1;
        }
        a += 1;
    }
    true
}

const _: () = assert!(
    preserved::<ReduceModulo, Lying<M3>>(),
    "executed arithmetic disagrees with its verified recovery map"
);

fn main() {}
