// The "input to the range coordinate" reading, made concrete: can the two endpoints be
// DERIVED from radix, precision and sign domain, rather than declared beside them, with
// no forbidden gate?
//
// This matters because the two readings differ in what a consumer writes.  If the range
// is derived, the sign domain is a presentation parameter and the numeral has one range
// per (precision, domain).  If the range is declared, the sign domain is redundant with
// it and can disagree with it, which is a perimeter hole rather than a coordinate.
//
// EXPECTED: compiles gate-free.  The derivation is arithmetic on associated consts, and
// no expression sits in type position, so no const-generic expression feature is
// reached for.
//
// rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib p5_domain_derives_range.rs

#![no_std]

use core::marker::PhantomData;

/// The three declared sign domains (110:915), as types rather than as a value.
pub trait SignDomain {
    /// The count of codes the domain places below zero, per code count `c`.
    fn low(c: i128) -> i128;
    /// The largest representable multiple of the quantum, per code count `c`.
    fn high(c: i128) -> i128;
}

pub struct NonNegative;
pub struct Symmetric;
pub struct AsymmetricLow;

impl SignDomain for NonNegative {
    fn low(_c: i128) -> i128 {
        0
    }
    fn high(c: i128) -> i128 {
        c - 1
    }
}
impl SignDomain for Symmetric {
    fn low(c: i128) -> i128 {
        -((c - 1) / 2)
    }
    fn high(c: i128) -> i128 {
        (c - 1) / 2
    }
}
impl SignDomain for AsymmetricLow {
    fn low(c: i128) -> i128 {
        -(c / 2)
    }
    fn high(c: i128) -> i128 {
        c - c / 2 - 1
    }
}

/// The numeral's declared members.  The range is not among them: it is read off.
pub trait Numeral {
    const RADIX: u32;
    const PRECISION: u32;
    const EXPONENT: i32;
    type Domain: SignDomain;

    /// Code count. Derived, so no consumer writes it and none can contradict it.
    fn codes() -> i128 {
        (Self::RADIX as i128).pow(Self::PRECISION)
    }
    /// The two endpoints, in units of the quantum. Derived from the domain.
    fn lo() -> i128 {
        <Self::Domain as SignDomain>::low(Self::codes())
    }
    fn hi() -> i128 {
        <Self::Domain as SignDomain>::high(Self::codes())
    }
}

pub struct Fixed<const R: u32, const P: u32, const E: i32, D>(PhantomData<D>);

impl<const R: u32, const P: u32, const E: i32, D: SignDomain> Numeral for Fixed<R, P, E, D> {
    const RADIX: u32 = R;
    const PRECISION: u32 = P;
    const EXPONENT: i32 = E;
    type Domain = D;
}

/// The four-condition order, with the two endpoint conditions reading DERIVED values.
/// Both operands are anchored at zero here, so the phase condition is discharged and
/// what remains is the grid clause and the two endpoint clauses.
pub fn includes<A: Numeral, B: Numeral>() -> bool {
    if A::RADIX != B::RADIX {
        return false; // out of scope for this probe: one radix at a time
    }
    let finer = B::EXPONENT <= A::EXPONENT;
    let scale = (A::RADIX as i128).pow((A::EXPONENT - B::EXPONENT).unsigned_abs());
    finer && B::lo() <= A::lo() * scale && A::hi() * scale <= B::hi()
}

pub type U4 = Fixed<2, 4, 0, NonNegative>;
pub type S4 = Fixed<2, 4, 0, Symmetric>;
pub type A4 = Fixed<2, 4, 0, AsymmetricLow>;
pub type S5 = Fixed<2, 5, 0, Symmetric>;

/// The relations the sign-domain question turns on, as compile-time facts.
pub const SYM_IN_ASYM: bool = true;
pub fn checks() -> [bool; 6] {
    [
        includes::<S4, A4>(), // Symmetric sits inside AsymmetricLow at equal precision
        includes::<A4, S4>(), // and not the other way
        includes::<A4, S5>(), // AsymmetricLow sits inside Symmetric one precision up
        includes::<S5, A4>(), // and not the other way
        includes::<U4, A4>(), // NonNegative and AsymmetricLow are incomparable
        includes::<A4, U4>(),
    ]
}
