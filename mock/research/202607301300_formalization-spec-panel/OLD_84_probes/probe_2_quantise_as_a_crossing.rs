//! probe 2: quantise as a crossing into a fixed-exponent numeral, with the preset's
//! own range row doing the work, and no new axis anywhere.
//!
//! The claim under test: `quantize` needs no fallibility mechanism the design does not
//! already have. Lift the quantum to type position, name the target numeral
//! `At<N, Q>` (radix and precision of `N`, exponent fixed at `Q`), and the failure is
//! `OverRange` on that target. Probe 1 established that the refusal predicate is
//! identical to the design's own out-of-range predicate on `At<N, Q>`; this probe
//! builds the operation on top of that and checks four things:
//!
//!   1. One body serves all four presets, with the resolution acting as a handler and
//!      the carrier as evidence of where the handler returns. This is the shape
//!      `05_probes/a_handler.rs` compiled and `70:196-203` folded into
//!      `Quantisation::Fallibility<T>`; nothing here is new mechanism.
//!   2. Every quantity the operation needs is an associated const of the target type,
//!      so the whole plan is settled in const position per `82:481-498`.
//!   3. The result under `Precise` refuses on exactly probe 1's cells, checked over
//!      the whole 16,000-cell (x, q) matrix rather than a sample.
//!   4. A consumer that needs the standard's own behaviour states it as a bound
//!      (`ConformingQuantise`) and is refused at the declaration site by a preset
//!      whose range row does not refuse. The refusal is compile-time; nothing about
//!      conformance is checked at run time.
//!
//! Zero feature gates, `no_std`. The exponent is a type (the design's own sealed
//! `Exponent` grammar, modelled here by four ZST instances rather than rebuilt over
//! `Pos`, since the grammar is not this probe's subject). Values are exact integers
//! scaled by `10^2`, the model's finest quantum, which is the same arithmetic the
//! tower's type-level rationals would do at this model.

#![no_std]
#![forbid(unsafe_code)]

use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// The numeral, the exponent, and the target of a quantise.
// ---------------------------------------------------------------------------

pub trait Numeral {
    const RADIX: i128;
    const PRECISION: u32;
    /// Finest exponent in the numeral's range; the scaling domain for this model.
    const MIN_EXPONENT: i32;
}

/// radix 10, p = 3, exponents -2 ..= 1: file 80's model.
pub struct Dec3;
impl Numeral for Dec3 {
    const RADIX: i128 = 10;
    const PRECISION: u32 = 3;
    const MIN_EXPONENT: i32 = -2;
}

/// The exponent is a type, per the spine rule's second firing (`78:262-263`).
pub trait Exponent {
    const Q: i32;
}
pub struct Em2;
pub struct Em1;
pub struct E0;
pub struct Ep1;
impl Exponent for Em2 {
    const Q: i32 = -2;
}
impl Exponent for Em1 {
    const Q: i32 = -1;
}
impl Exponent for E0 {
    const Q: i32 = 0;
}
impl Exponent for Ep1 {
    const Q: i32 = 1;
}

const fn ipow(base: i128, k: u32) -> i128 {
    let mut acc = 1i128;
    let mut i = 0;
    while i < k {
        acc *= base;
        i += 1;
    }
    acc
}

/// `At<N, Q>`: the numeral with `N`'s radix and precision and the exponent fixed at
/// `Q`. This is an ordinary member of the design's own vocabulary, a fixed-point
/// numeral, and it is what `quantize` targets. Every const below is a function of the
/// two type parameters alone, so all of it is settled at compile time.
pub struct At<N, Q>(PhantomData<(N, Q)>);

impl<N: Numeral, Q: Exponent> At<N, Q> {
    /// One unit in the last place of the target, in the scaling domain.
    pub const ULP: i128 = ipow(N::RADIX, (Q::Q - N::MIN_EXPONENT) as u32);
    /// Mantissa modulus, `r^p`.
    pub const MODULUS: i128 = ipow(N::RADIX, N::PRECISION);
    /// Largest representable mantissa, `r^p - 1`.
    pub const MAX_MANTISSA: i128 = Self::MODULUS - 1;
    /// The far point: the target's largest representable magnitude, in the scaling
    /// domain. Per `78:275-286` this is the supremum of the target's ordered values.
    pub const FAR_POINT: i128 = Self::MAX_MANTISSA * Self::ULP;
}

// ---------------------------------------------------------------------------
// The carrier and the resolution, unchanged from 05_probes/a_handler.rs.
// A resolution is a handler; the carrier is the evidence of where it returns.
// ---------------------------------------------------------------------------

pub trait Carrier: Copy {
    fn from_output(v: i128) -> Self;
}

/// The infallible tier: notko's `Just<T>` in the real design.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Total(pub i128);
impl Carrier for Total {
    fn from_output(v: i128) -> Self {
        Total(v)
    }
}

/// The refusing tier: notko's `Outcome<T, OutOfRange>` in the real design.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Fallible {
    Ok(i128),
    OutOfRange,
}
impl Carrier for Fallible {
    fn from_output(v: i128) -> Self {
        Fallible::Ok(v)
    }
}

/// A `Resolution` in the design's own vocabulary. `over` is the handler: it receives
/// the exact mantissa the result would need and the target's own consts, and returns
/// into whichever carrier its tier is. The arithmetic body never names a refusal
/// constructor, which is why one body serves every tier.
pub trait Resolution {
    type Out: Carrier;
    fn over(exact_mantissa: i128, max_mantissa: i128, modulus: i128) -> Self::Out;
}

pub struct ReduceModulo;
pub struct Clamp;
pub struct Refuse;

impl Resolution for ReduceModulo {
    type Out = Total;
    fn over(exact: i128, _max: i128, modulus: i128) -> Total {
        Total(exact.rem_euclid(modulus))
    }
}
impl Resolution for Clamp {
    type Out = Total;
    fn over(_exact: i128, max: i128, _modulus: i128) -> Total {
        Total(max)
    }
}
impl Resolution for Refuse {
    type Out = Fallible;
    fn over(_exact: i128, _max: i128, _modulus: i128) -> Fallible {
        Fallible::OutOfRange
    }
}

// ---------------------------------------------------------------------------
// The presets, exactly the ratified fixed-point rows (`78:409-414`). The target of a
// quantise is a fixed-point numeral, so the fixed-point table governs, not the float
// table, whichever kind `N` itself is.
// ---------------------------------------------------------------------------

pub trait Preset {
    type OverRange: Resolution;
}
pub struct Hot;
pub struct Warm;
pub struct Cold;
pub struct Precise;

impl Preset for Hot {
    type OverRange = ReduceModulo;
}
impl Preset for Warm {
    type OverRange = Clamp;
}
impl Preset for Cold {
    type OverRange = Clamp;
}
impl Preset for Precise {
    type OverRange = Refuse;
}

// ---------------------------------------------------------------------------
// The operation. One body, four presets, no branch on which preset it has.
// ---------------------------------------------------------------------------

/// Round-half-even of `vx / ulp`. This is the design's own quantiser step, applied to
/// the target numeral rather than to a new mechanism.
const fn round_half_even(vx: i128, ulp: i128) -> i128 {
    let quo = vx.div_euclid(ulp);
    let rem = vx.rem_euclid(ulp);
    let twice = 2 * rem;
    if twice > ulp || (twice == ulp && quo % 2 != 0) {
        quo + 1
    } else {
        quo
    }
}

/// `quantise::<N, Q, S>(x)`: the value of `x` at the quantum `Q`, under preset `S`.
///
/// The body performs the quantiser's own round-first-classify-second step and hands
/// an out-of-range classification to the resolution. It never constructs a refusal,
/// so it carries no `FromResidual`-shaped bound and instantiates at every tier.
pub fn quantise<N: Numeral, Q: Exponent, S: Preset>(vx: i128) -> <S::OverRange as Resolution>::Out {
    let m = round_half_even(vx, At::<N, Q>::ULP);
    if m <= At::<N, Q>::MAX_MANTISSA {
        <<S::OverRange as Resolution>::Out as Carrier>::from_output(m)
    } else {
        <S::OverRange as Resolution>::over(m, At::<N, Q>::MAX_MANTISSA, At::<N, Q>::MODULUS)
    }
}

// ---------------------------------------------------------------------------
// The const-callable form: the same kernel, called in const position with the
// target's associated consts, which is what monomorphisation produces anyway.
// ---------------------------------------------------------------------------

/// Result of the kernel: the mantissa, plus whether it left the target's range.
const fn quantise_kernel(vx: i128, ulp: i128, max_mantissa: i128) -> (i128, bool) {
    let m = round_half_even(vx, ulp);
    (m, m > max_mantissa)
}

// 1.23 at quantum 10^-2 is mantissa 123, in range. Scaled by 10^2: vx = 123.
const IN_RANGE: (i128, bool) =
    quantise_kernel(123, At::<Dec3, Em2>::ULP, At::<Dec3, Em2>::MAX_MANTISSA);
const _: () = assert!(IN_RANGE.0 == 123 && !IN_RANGE.1);

// 1234 at quantum 10^0 needs mantissa 1234, four digits: out of range.
// 1234 scaled by 10^2 is 123400.
const OUT_OF_RANGE: (i128, bool) =
    quantise_kernel(123_400, At::<Dec3, E0>::ULP, At::<Dec3, E0>::MAX_MANTISSA);
const _: () = assert!(OUT_OF_RANGE.0 == 1234 && OUT_OF_RANGE.1);

// 1.234 at quantum 10^-2 rounds to 1.23 and stays in range: the standard's inexact
// case, which is not a failure at all. 1.234 is not a datum of this model (its
// finest quantum is 10^-2), so this is stated at the next model down by scaling:
// 12345 scaled units = 123.45, quantised at 10^0 gives mantissa 123, ties-to-even.
const INEXACT: (i128, bool) =
    quantise_kernel(12_345, At::<Dec3, E0>::ULP, At::<Dec3, E0>::MAX_MANTISSA);
const _: () = assert!(INEXACT.0 == 123 && !INEXACT.1);

// The plan consts themselves, pinned so a change in the grammar is loud.
const _: () = assert!(At::<Dec3, Em2>::ULP == 1);
const _: () = assert!(At::<Dec3, Em1>::ULP == 10);
const _: () = assert!(At::<Dec3, E0>::ULP == 100);
const _: () = assert!(At::<Dec3, Ep1>::ULP == 1000);
const _: () = assert!(At::<Dec3, E0>::MAX_MANTISSA == 999);
const _: () = assert!(At::<Dec3, E0>::FAR_POINT == 99_900);
const _: () = assert!(At::<Dec3, Ep1>::FAR_POINT == 999_000);

// ---------------------------------------------------------------------------
// Conformance is a declaration-site fact, not a runtime one.
// ---------------------------------------------------------------------------

/// The standard's own `quantize` signals invalid operation and delivers NaN when the
/// result would need more than `p` digits (`80:118-126` reading clause 5.3). In this
/// design that is a refusing range row. A numeral and preset pair conforms exactly
/// when its `OverRange` refuses, and a consumer who needs conformance says so as a
/// bound. Implemented for `Precise` and for nothing else.
pub trait ConformingQuantise: Preset {}
impl ConformingQuantise for Precise {}

pub fn conforming_quantise<N: Numeral, Q: Exponent, S: ConformingQuantise>(
    vx: i128,
) -> <S::OverRange as Resolution>::Out {
    quantise::<N, Q, S>(vx)
}

/// Positive control: `Precise` satisfies the bound.
pub fn conforming_call_site(vx: i128) -> <Refuse as Resolution>::Out {
    conforming_quantise::<Dec3, E0, Precise>(vx)
}

// Negative control lives in probe_2b, which must not compile.

// ---------------------------------------------------------------------------
// The whole-matrix check, exported so probe_2_run can drive it without a std dep
// here. Kept in this file so the checked surface and the built surface are one.
// ---------------------------------------------------------------------------

/// Exhaustive agreement between `quantise::<_, _, Precise>`'s refusal and probe 1's
/// predicate, over every (mantissa, exponent, quantum) cell of the model.
/// Returns (cells, refusals, disagreements).
pub fn sweep() -> (u32, u32, u32) {
    let mut cells = 0u32;
    let mut refusals = 0u32;
    let mut disagreements = 0u32;

    let mut ei = 0usize;
    while ei < 4 {
        let mut m = 0i128;
        while m < 1000 {
            let vx = m * ipow(10, ei as u32);
            let mut qi = 0usize;
            while qi < 4 {
                cells += 1;
                let (ulp, max, refused) = match qi {
                    0 => (
                        At::<Dec3, Em2>::ULP,
                        At::<Dec3, Em2>::MAX_MANTISSA,
                        matches!(quantise::<Dec3, Em2, Precise>(vx), Fallible::OutOfRange),
                    ),
                    1 => (
                        At::<Dec3, Em1>::ULP,
                        At::<Dec3, Em1>::MAX_MANTISSA,
                        matches!(quantise::<Dec3, Em1, Precise>(vx), Fallible::OutOfRange),
                    ),
                    2 => (
                        At::<Dec3, E0>::ULP,
                        At::<Dec3, E0>::MAX_MANTISSA,
                        matches!(quantise::<Dec3, E0, Precise>(vx), Fallible::OutOfRange),
                    ),
                    _ => (
                        At::<Dec3, Ep1>::ULP,
                        At::<Dec3, Ep1>::MAX_MANTISSA,
                        matches!(quantise::<Dec3, Ep1, Precise>(vx), Fallible::OutOfRange),
                    ),
                };
                // probe 1's predicate A, recomputed here independently.
                let oracle = round_half_even(vx, ulp) > max;
                if refused != oracle {
                    disagreements += 1;
                }
                if refused {
                    refusals += 1;
                }
                qi += 1;
            }
            m += 1;
        }
        ei += 1;
    }
    (cells, refusals, disagreements)
}

/// The same sweep under `Warm`, checking totality: every cell delivers a value, and
/// every refused cell delivers exactly the far point's mantissa.
pub fn sweep_warm() -> (u32, u32, u32) {
    let mut cells = 0u32;
    let mut clamped = 0u32;
    let mut wrong = 0u32;

    let mut ei = 0usize;
    while ei < 4 {
        let mut m = 0i128;
        while m < 1000 {
            let vx = m * ipow(10, ei as u32);
            let mut qi = 0usize;
            while qi < 4 {
                cells += 1;
                let (Total(got), ulp, max) = match qi {
                    0 => (
                        quantise::<Dec3, Em2, Warm>(vx),
                        At::<Dec3, Em2>::ULP,
                        At::<Dec3, Em2>::MAX_MANTISSA,
                    ),
                    1 => (
                        quantise::<Dec3, Em1, Warm>(vx),
                        At::<Dec3, Em1>::ULP,
                        At::<Dec3, Em1>::MAX_MANTISSA,
                    ),
                    2 => (
                        quantise::<Dec3, E0, Warm>(vx),
                        At::<Dec3, E0>::ULP,
                        At::<Dec3, E0>::MAX_MANTISSA,
                    ),
                    _ => (
                        quantise::<Dec3, Ep1, Warm>(vx),
                        At::<Dec3, Ep1>::ULP,
                        At::<Dec3, Ep1>::MAX_MANTISSA,
                    ),
                };
                let exact = round_half_even(vx, ulp);
                if exact > max {
                    clamped += 1;
                    if got != max {
                        wrong += 1;
                    }
                } else if got != exact {
                    wrong += 1;
                }
                qi += 1;
            }
            m += 1;
        }
        ei += 1;
    }
    (cells, clamped, wrong)
}

/// The same sweep under `Hot`, so all four preset rows are exercised rather than
/// three. `Hot` reduces modulo `r^p`, which is the fixed-point table's own row, and
/// the result is always a representable mantissa.
pub fn sweep_hot() -> (u32, u32, u32) {
    let mut cells = 0u32;
    let mut wrapped = 0u32;
    let mut wrong = 0u32;

    let mut ei = 0usize;
    while ei < 4 {
        let mut m = 0i128;
        while m < 1000 {
            let vx = m * ipow(10, ei as u32);
            let mut qi = 0usize;
            while qi < 4 {
                cells += 1;
                let (Total(got), ulp, max, modulus) = match qi {
                    0 => (
                        quantise::<Dec3, Em2, Hot>(vx),
                        At::<Dec3, Em2>::ULP,
                        At::<Dec3, Em2>::MAX_MANTISSA,
                        At::<Dec3, Em2>::MODULUS,
                    ),
                    1 => (
                        quantise::<Dec3, Em1, Hot>(vx),
                        At::<Dec3, Em1>::ULP,
                        At::<Dec3, Em1>::MAX_MANTISSA,
                        At::<Dec3, Em1>::MODULUS,
                    ),
                    2 => (
                        quantise::<Dec3, E0, Hot>(vx),
                        At::<Dec3, E0>::ULP,
                        At::<Dec3, E0>::MAX_MANTISSA,
                        At::<Dec3, E0>::MODULUS,
                    ),
                    _ => (
                        quantise::<Dec3, Ep1, Hot>(vx),
                        At::<Dec3, Ep1>::ULP,
                        At::<Dec3, Ep1>::MAX_MANTISSA,
                        At::<Dec3, Ep1>::MODULUS,
                    ),
                };
                let exact = round_half_even(vx, ulp);
                if exact > max {
                    wrapped += 1;
                    if got != exact.rem_euclid(modulus) {
                        wrong += 1;
                    }
                } else if got != exact {
                    wrong += 1;
                }
                // Whatever the row does, the answer is always a representable
                // mantissa of the target: the operation is closed.
                if got < 0 || got > max {
                    wrong += 1;
                }
                qi += 1;
            }
            m += 1;
        }
        ei += 1;
    }
    (cells, wrapped, wrong)
}
