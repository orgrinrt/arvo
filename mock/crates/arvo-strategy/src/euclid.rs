//! The scalar euclidean raw contract.
//!
//! Split out of `arith.rs`. Distinct from the `UArith` / `IArith` surface
//! above it: those are the fixed-point operations at a logical width,
//! these divide a scalar into even shares and report the remainder.

use crate::identity::{Additive, Identity};
use crate::{BitsContainerFor, Cold, Hot, Precise, Signed, Unsigned, Warm};

// --- Scalar euclidean raw contract (round 202606232230) ----------------
//
// Inner per-width capability behind the fixed-point `ScalarEuclid` /
// `EuclidDiv` / `EvenSplittable` traits in `arvo`. Separate from
// `IArith` / `UArith` (composed by bound, not folded in), the array-
// `Capacity`-contract pattern: a contract for the specific capability,
// concrete impls per width, outer traits blanket-bound on it. Bodies are
// strategy-independent (euclidean division by a count >= 1 shrinks
// magnitude, so the quotient stays inside the logical bound the numerator
// satisfied; no Precise-style clamp needed). Div-by-zero returns the
// numerator (the `i_div` convention). `usize` count carriers use
// `lint:allow` because arvo-strategy sits below arvo-storage's `USize`,
// exactly as `IArith` carries bare const carriers.

/// Inner per-width signed euclidean-division contract.
///
/// Macro-impl'd per strategy x native container (widths 1..=128). The
/// outer `ScalarEuclid` / `EuclidDiv` / `EvenSplittable` traits on
/// `IFixed` blanket-delegate to it.
pub const trait ScalarEuclidRaw<const N: u16>: [const] BitsContainerFor<N, Signed> {
    // lint:allow(no-bare-numeric) reason: const-generic width carrier mirrors IArith<const N: u16>; tracked: #256
    /// Euclidean quotient of the raw container by an integer count (ULP granularity).
    fn div_euclid_scalar(
        a: <Self as BitsContainerFor<N, Signed>>::T,
        n: usize,
    ) -> <Self as BitsContainerFor<N, Signed>>::T; // lint:allow(no-bare-numeric) reason: divisor count carrier; tracked: #256
    /// Euclidean remainder of the raw container by an integer count (non-negative, < n).
    fn rem_euclid_scalar(
        a: <Self as BitsContainerFor<N, Signed>>::T,
        n: usize,
    ) -> <Self as BitsContainerFor<N, Signed>>::T; // lint:allow(no-bare-numeric) reason: divisor count carrier; tracked: #256
    /// `(base, base + 1 ULP, remainder as a share count)` for an even split into `n` parts.
    fn even_split_parts(
        a: <Self as BitsContainerFor<N, Signed>>::T,
        n: usize,
    ) -> (
        <Self as BitsContainerFor<N, Signed>>::T,
        <Self as BitsContainerFor<N, Signed>>::T,
        usize,
    ); // lint:allow(no-bare-numeric) reason: divisor count + remainder share-count carriers; tracked: #256
    /// Whole-unit euclidean quotient by a fixed-point divisor (`FRAC` fractional bits), floored to units.
    fn div_euclid_whole<const FRAC: u16>(
        a: <Self as BitsContainerFor<N, Signed>>::T,
        b: <Self as BitsContainerFor<N, Signed>>::T,
    ) -> <Self as BitsContainerFor<N, Signed>>::T; // lint:allow(no-bare-numeric) reason: const-generic FRAC carrier mirrors i_div_fixed; tracked: #256
    /// Whole-unit euclidean remainder by a fixed-point divisor (the fixed-point leftover).
    fn rem_euclid_whole<const FRAC: u16>(
        a: <Self as BitsContainerFor<N, Signed>>::T,
        b: <Self as BitsContainerFor<N, Signed>>::T,
    ) -> <Self as BitsContainerFor<N, Signed>>::T; // lint:allow(no-bare-numeric) reason: const-generic FRAC carrier; tracked: #256
}

macro_rules! impl_scalar_euclid_i {
    ($strategy:ty, $container:ty, $($bits:literal),+) => {
        $(
            impl const ScalarEuclidRaw<$bits> for $strategy {
                #[inline(always)]
                fn div_euclid_scalar(a: <Self as BitsContainerFor<$bits, Signed>>::T, n: usize) -> <Self as BitsContainerFor<$bits, Signed>>::T { // lint:allow(no-bare-numeric) reason: divisor count carrier; tracked: #256
                    if n == 0 { a } else { a.div_euclid(n as $container) }
                }
                #[inline(always)]
                fn rem_euclid_scalar(a: <Self as BitsContainerFor<$bits, Signed>>::T, n: usize) -> <Self as BitsContainerFor<$bits, Signed>>::T { // lint:allow(no-bare-numeric) reason: divisor count carrier; tracked: #256
                    if n == 0 { <<Self as BitsContainerFor<$bits, Signed>>::T as Identity<Additive>>::IDENTITY } else { a.rem_euclid(n as $container) }
                }
                #[inline(always)]
                fn even_split_parts(a: <Self as BitsContainerFor<$bits, Signed>>::T, n: usize) -> (<Self as BitsContainerFor<$bits, Signed>>::T, <Self as BitsContainerFor<$bits, Signed>>::T, usize) { // lint:allow(no-bare-numeric) reason: divisor count + remainder share-count carriers; tracked: #256
                    if n == 0 { (a, a, 0) } else { let base = a.div_euclid(n as $container); let rc = a.rem_euclid(n as $container) as usize; let base_plus = if rc == 0 { base } else { base + 1 }; (base, base_plus, rc) } // lint:allow(no-bare-numeric) reason: remainder share-count cast; tracked: #256
                }
                #[inline(always)]
                fn div_euclid_whole<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) -> <Self as BitsContainerFor<$bits, Signed>>::T { // lint:allow(no-bare-numeric) reason: const-generic FRAC carrier mirrors i_div_fixed; tracked: #256
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity<Additive>>::IDENTITY { a } else { a.div_euclid(b).saturating_mul((1 as $container) << FRAC) }
                }
                #[inline(always)]
                fn rem_euclid_whole<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) -> <Self as BitsContainerFor<$bits, Signed>>::T { // lint:allow(no-bare-numeric) reason: const-generic FRAC carrier; tracked: #256
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity<Additive>>::IDENTITY { <<Self as BitsContainerFor<$bits, Signed>>::T as Identity<Additive>>::IDENTITY } else { a.rem_euclid(b) }
                }
            }
        )+
    };
}

#[rustfmt::skip]
impl_scalar_euclid_i!(Hot, i8, 1, 2, 3, 4, 5, 6, 7, 8); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers, mirrors impl_i_arith invocations; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Hot, i16, 9, 10, 11, 12, 13, 14, 15, 16); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Hot, i32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Hot, i64, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Hot, i128, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Cold, i8, 1, 2, 3, 4, 5, 6, 7, 8); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Cold, i16, 9, 10, 11, 12, 13, 14, 15, 16); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Cold, i32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Cold, i64, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Cold, i128, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Warm, i16, 1, 2, 3, 4, 5, 6, 7, 8); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Warm, i32, 9, 10, 11, 12, 13, 14, 15, 16); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Warm, i64, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Warm, i128, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Precise, i16, 1, 2, 3, 4, 5, 6, 7, 8); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Precise, i32, 9, 10, 11, 12, 13, 14, 15, 16); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Precise, i64, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_i!(Precise, i128, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256

/// Inner per-width unsigned euclidean-division contract.
///
/// Unsigned counterpart of `ScalarEuclidRaw`. For unsigned containers
/// euclidean division coincides with truncating `/` and `%`; the surface
/// is symmetric so `UFixed` gets the same outer traits as `IFixed`.
pub const trait UScalarEuclidRaw<const N: u16>:
    [const] BitsContainerFor<N, Unsigned>
{
    // lint:allow(no-bare-numeric) reason: const-generic width carrier mirrors UArith<const N: u16>; tracked: #256
    /// Euclidean quotient of the raw container by an integer count (ULP granularity).
    fn div_euclid_scalar(
        a: <Self as BitsContainerFor<N, Unsigned>>::T,
        n: usize,
    ) -> <Self as BitsContainerFor<N, Unsigned>>::T; // lint:allow(no-bare-numeric) reason: divisor count carrier; tracked: #256
    /// Euclidean remainder of the raw container by an integer count.
    fn rem_euclid_scalar(
        a: <Self as BitsContainerFor<N, Unsigned>>::T,
        n: usize,
    ) -> <Self as BitsContainerFor<N, Unsigned>>::T; // lint:allow(no-bare-numeric) reason: divisor count carrier; tracked: #256
    /// `(base, base + 1 ULP, remainder as a share count)` for an even split into `n` parts.
    fn even_split_parts(
        a: <Self as BitsContainerFor<N, Unsigned>>::T,
        n: usize,
    ) -> (
        <Self as BitsContainerFor<N, Unsigned>>::T,
        <Self as BitsContainerFor<N, Unsigned>>::T,
        usize,
    ); // lint:allow(no-bare-numeric) reason: divisor count + remainder share-count carriers; tracked: #256
    /// Whole-unit euclidean quotient by a fixed-point divisor (`FRAC` fractional bits), floored to units.
    fn div_euclid_whole<const FRAC: u16>(
        a: <Self as BitsContainerFor<N, Unsigned>>::T,
        b: <Self as BitsContainerFor<N, Unsigned>>::T,
    ) -> <Self as BitsContainerFor<N, Unsigned>>::T; // lint:allow(no-bare-numeric) reason: const-generic FRAC carrier mirrors u_div_fixed; tracked: #256
    /// Whole-unit euclidean remainder by a fixed-point divisor (the fixed-point leftover).
    fn rem_euclid_whole<const FRAC: u16>(
        a: <Self as BitsContainerFor<N, Unsigned>>::T,
        b: <Self as BitsContainerFor<N, Unsigned>>::T,
    ) -> <Self as BitsContainerFor<N, Unsigned>>::T; // lint:allow(no-bare-numeric) reason: const-generic FRAC carrier; tracked: #256
}

macro_rules! impl_scalar_euclid_u {
    ($strategy:ty, $container:ty, $($bits:literal),+) => {
        $(
            impl const UScalarEuclidRaw<$bits> for $strategy {
                #[inline(always)]
                fn div_euclid_scalar(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, n: usize) -> <Self as BitsContainerFor<$bits, Unsigned>>::T { // lint:allow(no-bare-numeric) reason: divisor count carrier; tracked: #256
                    if n == 0 { a } else { a.div_euclid(n as $container) }
                }
                #[inline(always)]
                fn rem_euclid_scalar(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, n: usize) -> <Self as BitsContainerFor<$bits, Unsigned>>::T { // lint:allow(no-bare-numeric) reason: divisor count carrier; tracked: #256
                    if n == 0 { <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity<Additive>>::IDENTITY } else { a.rem_euclid(n as $container) }
                }
                #[inline(always)]
                fn even_split_parts(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, n: usize) -> (<Self as BitsContainerFor<$bits, Unsigned>>::T, <Self as BitsContainerFor<$bits, Unsigned>>::T, usize) { // lint:allow(no-bare-numeric) reason: divisor count + remainder share-count carriers; tracked: #256
                    if n == 0 { (a, a, 0) } else { let base = a.div_euclid(n as $container); let rc = a.rem_euclid(n as $container) as usize; let base_plus = if rc == 0 { base } else { base + 1 }; (base, base_plus, rc) } // lint:allow(no-bare-numeric) reason: remainder share-count cast; tracked: #256
                }
                #[inline(always)]
                fn div_euclid_whole<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) -> <Self as BitsContainerFor<$bits, Unsigned>>::T { // lint:allow(no-bare-numeric) reason: const-generic FRAC carrier mirrors u_div_fixed; tracked: #256
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity<Additive>>::IDENTITY { a } else { a.div_euclid(b).saturating_mul((1 as $container) << FRAC) }
                }
                #[inline(always)]
                fn rem_euclid_whole<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) -> <Self as BitsContainerFor<$bits, Unsigned>>::T { // lint:allow(no-bare-numeric) reason: const-generic FRAC carrier; tracked: #256
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity<Additive>>::IDENTITY { <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity<Additive>>::IDENTITY } else { a.rem_euclid(b) }
                }
            }
        )+
    };
}

#[rustfmt::skip]
impl_scalar_euclid_u!(Hot, u8, 1, 2, 3, 4, 5, 6, 7, 8); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Hot, u16, 9, 10, 11, 12, 13, 14, 15, 16); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Hot, u32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Hot, u64, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Hot, u128, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Cold, u8, 1, 2, 3, 4, 5, 6, 7, 8); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Cold, u16, 9, 10, 11, 12, 13, 14, 15, 16); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Cold, u32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Cold, u64, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Cold, u128, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Warm, u16, 1, 2, 3, 4, 5, 6, 7, 8); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Warm, u32, 9, 10, 11, 12, 13, 14, 15, 16); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Warm, u64, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Warm, u128, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Precise, u16, 1, 2, 3, 4, 5, 6, 7, 8); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Precise, u32, 9, 10, 11, 12, 13, 14, 15, 16); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Precise, u64, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
#[rustfmt::skip]
impl_scalar_euclid_u!(Precise, u128, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64); // lint:allow(no-bare-numeric) reason: macro container-type + width carriers; tracked: #256
