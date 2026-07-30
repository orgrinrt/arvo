// Probe D for panel file 10. What the one-definition pipeline costs in
// emitted code, against the union's hand-shaped arithmetic.
//
// The predictable objection to making phi the executed function is that a
// reference semantics in the hot path costs performance, which for arvo
// would be disqualifying (`arvo-always-optimal-internals.md`). This probe
// instantiates the pipeline with the range bounds as constants, the way a
// real composition's bounds arrive from the type, and exposes concrete
// symbols for inspection. No timing is claimed; the instruction sequence
// is the artifact, per `bench-and-sketch-discipline.md`.
//
// rustc +nightly-2026-05-28 --edition 2024 -C opt-level=3 --emit asm ...
#![feature(const_trait_impl)]
#![allow(dead_code)]

#[derive(Copy, Clone)]
pub enum Rec<T: Copy> {
    At(T),
    Refused,
}

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
}

#[derive(Copy, Clone)]
pub struct P16(pub u16);
const impl Payload for P16 {
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
        P16(w as u16)
    }
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

pub struct TowardNegative;
const impl Resolve for TowardNegative {
    fn phi<P: [const] Payload>(x: P::Wide, min: P, max: P) -> Rec<P> {
        if P::wgt(x, max.widen()) {
            Rec::At(max)
        } else if P::wlt(x, min.widen()) {
            Rec::At(min)
        } else {
            Rec::At(P::narrow(x))
        }
    }
}

pub const trait CarrierC<T: Copy>: Copy {
    fn from_output(v: T) -> Self;
    fn refused() -> Self;
    fn observe(self) -> Rec<T>;
}

#[derive(Copy, Clone)]
pub struct Total<T: Copy>(T);
const impl<T: Copy> CarrierC<T> for Total<T> {
    fn from_output(v: T) -> Self {
        Total(v)
    }
    fn refused() -> Self {
        panic!("total carrier reached a refusal")
    }
    fn observe(self) -> Rec<T> {
        Rec::At(self.0)
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

// bounds as constants, the way a composition's type supplies them.
// [0, 4095]: a 12-bit numeral, power-of-two span, so the wrap should
// lower to a mask if LLVM sees through the reference body.
#[unsafe(no_mangle)]
pub fn add_wrap_12bit(a: P16, b: P16) -> Total<P16> {
    pipeline_add::<ReduceModulo, P16, Total<P16>>(a, b, P16(0), P16(4095))
}

#[unsafe(no_mangle)]
pub fn add_clamp_12bit(a: P16, b: P16) -> Total<P16> {
    pipeline_add::<TowardNegative, P16, Total<P16>>(a, b, P16(0), P16(4095))
}

// the hand-written baselines a maintainer would write directly.
#[unsafe(no_mangle)]
pub fn baseline_wrap(a: u16, b: u16) -> u16 {
    (a as u32 + b as u32) as u16 & 0x0fff
}

#[unsafe(no_mangle)]
pub fn baseline_clamp(a: u16, b: u16) -> u16 {
    let s = a as u32 + b as u32;
    if s > 4095 {
        4095
    } else {
        s as u16
    }
}

fn main() {}
