// Probe A for panel file 10 (Leroy, what is actually certified).
//
// The repair 09 section 4 named and did not build: ONE definition of each
// resolution's semantics (`phi`), which is simultaneously
//   (1) the function the const-eval witness checks,
//   (2) the function the executed arithmetic calls, and
//   (3) generic over the payload, so the model instantiation the checker
//       runs and the real-width instantiation the consumer runs are two
//       monomorphisations of the same body, not two authored texts.
//
// Plus the obligation 09 found unstated, now stated and checked: a
// semantic-preservation equation in Kulisch's form,
//
//     observe(op'(a, b)) == phi(a + b)      (Kleene equality)
//
// checked exhaustively at compile time over a small model payload, at the
// door of the executed pipeline, so a program whose executed arithmetic
// disagrees with its verified semantics does not compile.
//
// rustc +nightly-2026-05-28 --edition 2024 a_one_definition.rs -o a_repro && ./a_repro
#![feature(const_trait_impl)]
#![allow(dead_code)]

// ===========================================================================
// 1. the payload abstraction: what a width is, to phi.
//
// This is the width-dependent trusted base, isolated: widening, wide
// arithmetic, comparisons. In real arvo this is the existing per-width
// impl table on Bits<N, S>; here two instances, the checker's 3-bit model
// and a 16-bit "real" payload.
// ===========================================================================

pub const trait Payload: Copy {
    /// Wide enough to hold any exact sum of two payload values.
    type Wide: Copy;
    fn widen(self) -> Self::Wide;
    fn wadd(a: Self::Wide, b: Self::Wide) -> Self::Wide;
    fn wsub(a: Self::Wide, b: Self::Wide) -> Self::Wide;
    fn wgt(a: Self::Wide, b: Self::Wide) -> bool;
    fn wlt(a: Self::Wide, b: Self::Wide) -> bool;
    fn wrem_euclid(a: Self::Wide, m: Self::Wide) -> Self::Wide;
    fn wone() -> Self::Wide;
    /// Caller guarantees the wide value is in the payload's range.
    fn narrow(w: Self::Wide) -> Self;
    /// For the checker's equality only.
    fn to_model(self) -> i64;
}

/// The checker's model: a 3-bit unsigned payload.
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

/// The consumer's payload: 16 bits.
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
    fn to_model(self) -> i64 {
        self.0 as i64
    }
}

// ===========================================================================
// 2. THE one definition. Generic over the payload: the checker's
// instantiation and the runtime's instantiation are the same text.
// ===========================================================================

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

pub struct TowardNegative; // clamp above, clamp below
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

pub struct Refuse;
const impl Resolve for Refuse {
    fn phi<P: [const] Payload>(x: P::Wide, min: P, max: P) -> Rec<P> {
        if P::wgt(x, max.widen()) || P::wlt(x, min.widen()) {
            Rec::Refused
        } else {
            Rec::At(P::narrow(x))
        }
    }
}

// ===========================================================================
// 3. carriers. Two structural decisions carried over from the panel, made
// load-bearing here:
//   - `refused()` takes NO payload argument. A carrier cannot substitute a
//     value on the refusal path, because it has no value to substitute:
//     with only `T: Copy` in scope and no T parameter, no T is
//     constructible (see probe C for the compile error the union's
//     clamping delivery now produces).
//   - `observe` is the only door out, and it is the observation the
//     preservation check is stated against. Fields are private.
// ===========================================================================

pub const trait CarrierC<T: Copy>: Copy {
    fn from_output(v: T) -> Self;
    fn refused() -> Self;
    fn observe(self) -> Rec<T>;
}

/// Total carrier: for compositions whose phi never refuses. Its `refused`
/// is unreachable; the panic is the runtime guard on the width-uniformity
/// argument (the small-model check proves never-refuses at the model
/// width; the transfer to the real width is an argument, and this line is
/// where that argument would fail loudly instead of silently).
#[derive(Copy, Clone)]
pub struct Total<T: Copy>(T);
const impl<T: Copy> CarrierC<T> for Total<T> {
    fn from_output(v: T) -> Self {
        Total(v)
    }
    fn refused() -> Self {
        panic!("total carrier reached a refusal: width-uniformity argument violated")
    }
    fn observe(self) -> Rec<T> {
        Rec::At(self.0)
    }
}

#[derive(Copy, Clone)]
pub enum Fallible<T: Copy> {
    Ok(T),
    Refused,
}
const impl<T: Copy> CarrierC<T> for Fallible<T> {
    fn from_output(v: T) -> Self {
        Fallible::Ok(v)
    }
    fn refused() -> Self {
        Fallible::Refused
    }
    fn observe(self) -> Rec<T> {
        match self {
            Fallible::Ok(v) => Rec::At(v),
            Fallible::Refused => Rec::Refused,
        }
    }
}

/// 05's absorbing bottom, private fields, observe as the only exit.
#[derive(Copy, Clone)]
pub struct Poison<T: Copy> {
    v: T,
    bottom: bool,
}
const impl<T: Copy> CarrierC<T> for Poison<T> {
    fn from_output(v: T) -> Self {
        Poison { v, bottom: false }
    }
    fn refused() -> Self {
        Poison {
            v: unsafe { core::mem::zeroed() }, // unobservable: observe() discards it
            bottom: true,
        }
    }
    fn observe(self) -> Rec<T> {
        if self.bottom {
            Rec::Refused
        } else {
            Rec::At(self.v)
        }
    }
}

// ===========================================================================
// 4. the executed pipeline. ONE body: exact intermediate, then phi, then
// the carrier embedding. This is the function the consumer's `a + b`
// monomorphises, and the function the preservation check runs on the
// model payload. There is no second definition for them to disagree
// across.
// ===========================================================================

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

// ===========================================================================
// 5. the preservation obligation, stated and discharged.
//
//    for all a, b in the model domain:
//        observe(pipeline_add(a, b))  ==_Kleene  phi(widen(a) + widen(b))
//
// This is Kulisch's defining equation of a machine operation,
// a op' b = round(a op b), checked by exhaustion at the model width. It
// is the equation whose absence 09 demonstrated: in the union, nothing
// stated it, so nothing could fail when the executed path clamped where
// phi wrapped.
// ===========================================================================

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

// The door: forced eagerly here, and in real arvo forced inside the one
// generic entry point per 07's two-site discipline.
const _: () = assert!(preserved::<ReduceModulo, Fallible<M3>>());
const _: () = assert!(preserved::<TowardNegative, Fallible<M3>>());
const _: () = assert!(preserved::<Refuse, Fallible<M3>>());
const _: () = assert!(preserved::<Refuse, Poison<M3>>());
const _: () = assert!(preserved::<ReduceModulo, Total<M3>>());

// ===========================================================================
// 6. run it at the real width. Same body, second monomorphisation.
// ===========================================================================

fn main() {
    // 09's reproduction case: 5 + 4 = 9 over [0, 7]. In the union, Hot
    // (ReduceModulo) returned the hardcoded clamp, 7. Here there is no
    // second definition: the executed path IS phi.
    let wrapped: Fallible<P16> =
        pipeline_add::<ReduceModulo, P16, Fallible<P16>>(P16(5), P16(4), P16(0), P16(7));
    match wrapped.observe() {
        Rec::At(v) => println!(
            "add under ReduceModulo, 5+4 over [0,7]: {}  (phi's wrap)",
            v.0
        ),
        Rec::Refused => println!("add under ReduceModulo: refused (WRONG)"),
    }

    let clamped: Fallible<P16> =
        pipeline_add::<TowardNegative, P16, Fallible<P16>>(P16(5), P16(4), P16(0), P16(7));
    match clamped.observe() {
        Rec::At(v) => println!(
            "add under TowardNegative, 5+4 over [0,7]: {}  (phi's clamp)",
            v.0
        ),
        Rec::Refused => println!("add under TowardNegative: refused (WRONG)"),
    }

    let refused: Fallible<P16> =
        pipeline_add::<Refuse, P16, Fallible<P16>>(P16(5), P16(4), P16(0), P16(7));
    match refused.observe() {
        Rec::At(v) => println!("add under Refuse: {} (WRONG)", v.0),
        Rec::Refused => println!("add under Refuse, 5+4 over [0,7]: refused  (phi's refusal)"),
    }

    // and in range, all three agree:
    let ok: Fallible<P16> =
        pipeline_add::<Refuse, P16, Fallible<P16>>(P16(3), P16(4), P16(0), P16(7));
    match ok.observe() {
        Rec::At(v) => println!("add under Refuse, 3+4 in range: {}", v.0),
        Rec::Refused => println!("in-range refused (WRONG)"),
    }
}
