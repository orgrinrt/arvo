//! Even division into shares, and the remainder it leaves.
//!
//! Split out of `traits.rs`, which carried all seven trait families in
//! one file well past the size limit.

use crate::fixed_scale::{frac, FracShift};
use crate::ifixed::IFixed;
use crate::strategy::{ifixed_bits, ufixed_bits, ScalarEuclidRaw, Strategy, UScalarEuclidRaw};
use crate::ufixed::UFixed;
use arvo_storage::{FBits, IBits, USize};

// Only the test modules below name these, and `use super::*` carries them in.
// At file scope without the gate the lib build warns them unused; repeated
// inside each module they collide with that same glob.
#[cfg(test)]
use crate::strategy::Hot;
#[cfg(test)]
use arvo_storage::{fbits, ibits};

// --- ScalarEuclid / EuclidDiv / EvenSplittable (round 202606232230) -----
//
// Small separate single-method-family traits, blanket-impl'd over the
// fixed-point types via the inner `ScalarEuclidRaw` / `UScalarEuclidRaw`
// contract (the array-`Capacity`-contract pattern). The raw container
// arithmetic stays inside the contract; this surface is raw-free.

/// Euclidean division of a fixed-point value by an integer count.
///
/// `div_euclid_scalar` / `rem_euclid_scalar` divide at ULP granularity:
/// the base share and its remainder for an as-equal-as-possible split
/// into `n` parts. Contrast `EuclidDiv`, which floors to whole units.
pub trait ScalarEuclid: Sized {
    /// Euclidean quotient by an integer count (ULP granularity).
    fn div_euclid_scalar(self, n: USize) -> Self;
    /// Euclidean remainder by an integer count (the leftover ULPs).
    fn rem_euclid_scalar(self, n: USize) -> Self;
}

/// std-parity euclidean division by a `Self` divisor.
///
/// The quotient is floored to whole integer units (the coarser,
/// conserving variant, distinct from `ScalarEuclid`'s ULP split).
pub trait EuclidDiv: Sized {
    /// Whole-unit euclidean quotient by a fixed-point divisor.
    fn div_euclid(self, rhs: Self) -> Self;
    /// Euclidean remainder by a fixed-point divisor (the fixed leftover).
    fn rem_euclid(self, rhs: Self) -> Self;
}

/// Lazy iterator over `n` exact-conserving even shares of a value.
///
/// Yields `base + 1 ULP` for the first `r` shares (the remainder
/// distributed by index) then `base`, for `n` shares total summing to
/// the original. No allocation.
pub struct EvenShares<T> {
    base: T,
    base_plus: T,
    r: usize, // lint:allow(no-bare-numeric) reason: internal share-count cursor; tracked: #256
    n: usize, // lint:allow(no-bare-numeric) reason: internal share-count cursor; tracked: #256
    i: usize, // lint:allow(no-bare-numeric) reason: internal share-count cursor; tracked: #256
}

impl<T: Copy> Iterator for EvenShares<T> {
    type Item = T;
    // `rustfmt::skip` keeps the allow on its line: the lint reads the line the
    // violation is on, and the formatter otherwise moves the comment below it.
    #[rustfmt::skip]
    fn next(&mut self) -> Option<T> { // lint:allow(no-bare-option) reason: core::iter::Iterator::next trait-method signature returns Option<Self::Item>; tracked: #115
        if self.i >= self.n {
            return None;
        }
        let v = if self.i < self.r {
            self.base_plus
        } else {
            self.base
        };
        self.i += 1;
        Some(v)
    }
}

/// Split a value into `n` shares that sum to it exactly.
pub trait EvenSplittable: Sized {
    /// A no-alloc iterator of `n` exact-conserving shares.
    fn split_evenly(self, n: USize) -> EvenShares<Self>;
}

impl<const I: IBits, const F: FBits, S: Strategy> ScalarEuclid for IFixed<I, F, S>
where
    S: ScalarEuclidRaw<{ ifixed_bits(I, F) }>,
{
    #[inline]
    fn div_euclid_scalar(self, n: USize) -> Self {
        Self::from_raw(
            <S as ScalarEuclidRaw<{ ifixed_bits(I, F) }>>::div_euclid_scalar(self.to_raw(), n.0),
        )
    }
    #[inline]
    fn rem_euclid_scalar(self, n: USize) -> Self {
        Self::from_raw(
            <S as ScalarEuclidRaw<{ ifixed_bits(I, F) }>>::rem_euclid_scalar(self.to_raw(), n.0),
        )
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> EuclidDiv for IFixed<I, F, S>
where
    S: ScalarEuclidRaw<{ ifixed_bits(I, F) }>,
    (): FracShift<{ frac(F) }>,
{
    #[inline]
    fn div_euclid(self, rhs: Self) -> Self {
        Self::from_raw(
            <S as ScalarEuclidRaw<{ ifixed_bits(I, F) }>>::div_euclid_whole::<{ frac(F) }>(
                self.to_raw(),
                rhs.to_raw(),
            ),
        )
    }
    #[inline]
    fn rem_euclid(self, rhs: Self) -> Self {
        Self::from_raw(
            <S as ScalarEuclidRaw<{ ifixed_bits(I, F) }>>::rem_euclid_whole::<{ frac(F) }>(
                self.to_raw(),
                rhs.to_raw(),
            ),
        )
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> EvenSplittable for IFixed<I, F, S>
where
    S: ScalarEuclidRaw<{ ifixed_bits(I, F) }>,
{
    #[inline]
    fn split_evenly(self, n: USize) -> EvenShares<Self> {
        let (base, base_plus, r) =
            <S as ScalarEuclidRaw<{ ifixed_bits(I, F) }>>::even_split_parts(self.to_raw(), n.0);
        EvenShares {
            base: Self::from_raw(base),
            base_plus: Self::from_raw(base_plus),
            r,
            n: n.0,
            i: 0,
        }
    }
}

#[cfg(test)]
mod euclid_split_tests {
    use super::*;
    use crate::ifixed::IFixed;

    // A non-power-of-two-width signed fixed-point: 13 integer bits, 16
    // fractional, Hot. Logical width 30 -> i32 container.
    type Q = IFixed<{ ibits(13) }, { fbits(16) }, Hot>;

    // `rustfmt::skip` keeps the allow on its line: the lint reads the line the
    // violation is on, and the formatter otherwise moves the comment below it.
    #[rustfmt::skip]
    fn q(raw: i32) -> Q { // lint:allow(no-bare-numeric) reason: test raw constructor over the i32 container; tracked: #256
        Q::from_raw(raw)
    }

    fn sum(shares: &[Q]) -> Q {
        let mut acc = q(0);
        let mut k = 0; // lint:allow(no-bare-numeric) reason: test loop cursor; tracked: #256
        while k < shares.len() {
            acc = acc + shares[k];
            k += 1;
        }
        acc
    }

    #[test]
    fn split_evenly_conserves_indivisible() {
        // 100 raw over 3: 34 + 33 + 33 == 100 exactly.
        let shares: [Q; 3] = {
            let mut it = q(100).split_evenly(USize(3));
            [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
        };
        assert_eq!(it_none(q(100).split_evenly(USize(3))), 3);
        assert_eq!(sum(&shares).to_raw(), 100);
        assert_eq!(shares[0].to_raw(), 34);
        assert_eq!(shares[1].to_raw(), 33);
        assert_eq!(shares[2].to_raw(), 33);
    }

    // `rustfmt::skip` keeps the allow on its line: the lint reads the line the
    // violation is on, and the formatter otherwise moves the comment below it.
    #[rustfmt::skip]
    fn it_none(mut it: EvenShares<Q>) -> usize { // lint:allow(no-bare-numeric) reason: test count of yielded shares; tracked: #256
        let mut c = 0; // lint:allow(no-bare-numeric) reason: test counter; tracked: #256
        while it.next().is_some() {
            c += 1;
        }
        c
    }

    #[test]
    fn split_evenly_deterministic() {
        let a = it_collect(q(67).split_evenly(USize(5)));
        let b = it_collect(q(67).split_evenly(USize(5)));
        assert_eq!(a, b);
        // 67 over 5: 14,14,13,13,13 -> sum 67.
        assert_eq!(a.iter().map(|s| s.to_raw()).sum::<i32>(), 67);
    }

    fn it_collect(mut it: EvenShares<Q>) -> [Q; 5] {
        [
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
        ]
    }

    #[test]
    fn split_evenly_conserves_negative() {
        // a fixed-point value can be negative; euclidean split still conserves.
        let shares = it_collect3(q(-100).split_evenly(USize(3)));
        assert_eq!(shares.iter().map(|s| s.to_raw()).sum::<i32>(), -100);
    }

    fn it_collect3(mut it: EvenShares<Q>) -> [Q; 3] {
        [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
    }

    #[test]
    fn scalar_euclid_base_remainder_identity() {
        // base*n + remainder == original (ULP granularity).
        let base = q(100).div_euclid_scalar(USize(3));
        let rem = q(100).rem_euclid_scalar(USize(3));
        assert_eq!(base.to_raw(), 33);
        assert_eq!(rem.to_raw(), 1);
    }

    #[test]
    fn euclid_div_whole_unit() {
        // whole-unit divisor: 100.0 / 3.0 -> quotient floored to 33.0 (whole units),
        // remainder 1.0. raw: one unit == 1 << 16.
        let one = 1i32 << 16; // lint:allow(no-bare-numeric) reason: test raw one-unit constant; tracked: #256
        let a = q(100 * one); // lint:allow(no-bare-numeric) reason: test raw value; tracked: #256
        let b = q(3 * one); // lint:allow(no-bare-numeric) reason: test raw value; tracked: #256
        assert_eq!(a.div_euclid(b).to_raw(), 33 * one); // 33.0 whole units
        assert_eq!(a.rem_euclid(b).to_raw(), one); // 1.0 leftover
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> ScalarEuclid for UFixed<I, F, S>
where
    S: UScalarEuclidRaw<{ ufixed_bits(I, F) }>,
{
    #[inline]
    fn div_euclid_scalar(self, n: USize) -> Self {
        Self::from_raw(
            <S as UScalarEuclidRaw<{ ufixed_bits(I, F) }>>::div_euclid_scalar(self.to_raw(), n.0),
        )
    }
    #[inline]
    fn rem_euclid_scalar(self, n: USize) -> Self {
        Self::from_raw(
            <S as UScalarEuclidRaw<{ ufixed_bits(I, F) }>>::rem_euclid_scalar(self.to_raw(), n.0),
        )
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> EuclidDiv for UFixed<I, F, S>
where
    S: UScalarEuclidRaw<{ ufixed_bits(I, F) }>,
    (): FracShift<{ frac(F) }>,
{
    #[inline]
    fn div_euclid(self, rhs: Self) -> Self {
        Self::from_raw(
            <S as UScalarEuclidRaw<{ ufixed_bits(I, F) }>>::div_euclid_whole::<{ frac(F) }>(
                self.to_raw(),
                rhs.to_raw(),
            ),
        )
    }
    #[inline]
    fn rem_euclid(self, rhs: Self) -> Self {
        Self::from_raw(
            <S as UScalarEuclidRaw<{ ufixed_bits(I, F) }>>::rem_euclid_whole::<{ frac(F) }>(
                self.to_raw(),
                rhs.to_raw(),
            ),
        )
    }
}

impl<const I: IBits, const F: FBits, S: Strategy> EvenSplittable for UFixed<I, F, S>
where
    S: UScalarEuclidRaw<{ ufixed_bits(I, F) }>,
{
    #[inline]
    fn split_evenly(self, n: USize) -> EvenShares<Self> {
        let (base, base_plus, r) =
            <S as UScalarEuclidRaw<{ ufixed_bits(I, F) }>>::even_split_parts(self.to_raw(), n.0);
        EvenShares {
            base: Self::from_raw(base),
            base_plus: Self::from_raw(base_plus),
            r,
            n: n.0,
            i: 0,
        }
    }
}

#[cfg(test)]
mod ueuclid_split_tests {
    use super::*;
    use crate::ufixed::UFixed;

    // 13 integer bits, 16 fractional, Hot. Logical width 29 -> u32 container.
    type U = UFixed<{ ibits(13) }, { fbits(16) }, Hot>;

    // `rustfmt::skip` keeps the allow on its line: the lint reads the line the
    // violation is on, and the formatter otherwise moves the comment below it.
    #[rustfmt::skip]
    fn u(raw: u32) -> U { // lint:allow(no-bare-numeric) reason: test raw constructor over the u32 container; tracked: #256
        U::from_raw(raw)
    }

    #[test]
    fn usplit_conserves_indivisible() {
        let mut it = u(100).split_evenly(USize(3));
        let a = it.next().unwrap();
        let b = it.next().unwrap();
        let c = it.next().unwrap();
        assert!(it.next().is_none());
        assert_eq!(a.to_raw() + b.to_raw() + c.to_raw(), 100);
        assert_eq!(a.to_raw(), 34); // first share carries the remainder ULP
        assert_eq!(b.to_raw(), 33);
        assert_eq!(c.to_raw(), 33);
    }

    #[test]
    fn uscalar_euclid_identity() {
        assert_eq!(u(100).div_euclid_scalar(USize(3)).to_raw(), 33);
        assert_eq!(u(100).rem_euclid_scalar(USize(3)).to_raw(), 1);
    }

    #[test]
    fn ueuclid_div_whole_unit() {
        let one = 1u32 << 16; // lint:allow(no-bare-numeric) reason: test raw one-unit constant; tracked: #256
        let a = u(100 * one); // lint:allow(no-bare-numeric) reason: test raw value; tracked: #256
        let b = u(3 * one); // lint:allow(no-bare-numeric) reason: test raw value; tracked: #256
        assert_eq!(a.div_euclid(b).to_raw(), 33 * one);
        assert_eq!(a.rem_euclid(b).to_raw(), one);
    }
}

#[cfg(test)]
mod euclid_overflow_tests {
    use super::*;
    use crate::ifixed::IFixed;
    use crate::strategy::Hot;
    use crate::ufixed::UFixed;
    use arvo_storage::{fbits, ibits};

    // Width-8 containers: IFixed<7,0> -> i8 (logical 1+7+0=8), UFixed<8,0> -> u8.
    type Q8 = IFixed<{ ibits(7) }, { fbits(0) }, Hot>;
    type U8 = UFixed<{ ibits(8) }, { fbits(0) }, Hot>;
    type Q = IFixed<{ ibits(13) }, { fbits(16) }, Hot>;

    #[test]
    fn split_one_at_signed_max_does_not_overflow() {
        // base == a == i8::MAX; the old eager `base + 1` panicked in debug. A 1-way split is the value.
        let mut it = Q8::from_raw(i8::MAX).split_evenly(USize(1)); // lint:allow(no-bare-numeric) reason: test raw MAX over the i8 container; tracked: #256
        assert_eq!(it.next().unwrap().to_raw(), i8::MAX);
        assert!(it.next().is_none());
    }

    #[test]
    fn split_one_at_unsigned_max_does_not_overflow() {
        let mut it = U8::from_raw(u8::MAX).split_evenly(USize(1)); // lint:allow(no-bare-numeric) reason: test raw MAX over the u8 container; tracked: #256
        assert_eq!(it.next().unwrap().to_raw(), u8::MAX);
        assert!(it.next().is_none());
    }

    #[test]
    fn euclid_div_subunit_divisor_saturates() {
        // 100.0 div_euclid (1 ULP): whole quotient ~6.5e6, shifted back overflows i32 -> saturates, no panic.
        let one = 1i32 << 16; // lint:allow(no-bare-numeric) reason: test raw one-unit constant; tracked: #256
        let a = Q::from_raw(100 * one); // lint:allow(no-bare-numeric) reason: test raw value; tracked: #256
        let b = Q::from_raw(1); // 1 ULP, a sub-unit divisor
        assert_eq!(a.div_euclid(b).to_raw(), i32::MAX);
    }
}
