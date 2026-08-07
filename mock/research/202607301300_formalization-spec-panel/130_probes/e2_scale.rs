#![no_std]
#![allow(dead_code)]
// The whole surface. No feature gates, no -Z flag, no macro at the surface.
use core::marker::PhantomData;

pub trait Policy {}
pub trait Lowering {}
pub struct Hot;
pub struct Warm;
pub struct Cold;
pub struct Precise;
impl Policy for Warm {}
impl Lowering for Warm {}
impl Policy for Hot {}
impl Lowering for Hot {}

pub trait Container: Copy {
    const BITS: u32;
    const ZERO: Self;
}
impl Container for u8 {
    const BITS: u32 = 8;
    const ZERO: u8 = 0;
}
impl Container for u16 {
    const BITS: u32 = 16;
    const ZERO: u16 = 0;
}
impl Container for u32 {
    const BITS: u32 = 32;
    const ZERO: u32 = 0;
}
impl Container for u64 {
    const BITS: u32 = 64;
    const ZERO: u64 = 0;
}
impl Container for u128 {
    const BITS: u32 = 8192;
    const ZERO: u128 = 0;
} // probe only: the ladder is widened so the four-digit widths fit
impl Container for i8 {
    const BITS: u32 = 8;
    const ZERO: i8 = 0;
}
impl Container for i16 {
    const BITS: u32 = 16;
    const ZERO: i16 = 0;
}
impl Container for i32 {
    const BITS: u32 = 32;
    const ZERO: i32 = 0;
}
impl Container for i64 {
    const BITS: u32 = 64;
    const ZERO: i64 = 0;
}
impl Container for i128 {
    const BITS: u32 = 128;
    const ZERO: i128 = 0;
}

pub struct Unsigned;
pub struct Signed;
pub trait Sign {}
impl Sign for Unsigned {}
impl Sign for Signed {}

// ---------------------------------------------------------------------------
// The numeral. Two coordinates the consumer writes, a carried container, a
// strategy, a sign. Nothing computes a const argument anywhere.
// ---------------------------------------------------------------------------
pub struct Fixed<const I: u32, const F: u32, C: Container, G: Sign, S: Policy + Lowering> {
    raw: C,
    _m: PhantomData<(G, S)>,
}
impl<const I: u32, const F: u32, C: Container, G: Sign, S: Policy + Lowering> Clone
    for Fixed<I, F, C, G, S>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, C: Container, G: Sign, S: Policy + Lowering> Copy
    for Fixed<I, F, C, G, S>
{
}

pub type UFixed<const I: u32, const F: u32, C, S> = Fixed<I, F, C, Unsigned, S>;
pub type IFixed<const I: u32, const F: u32, C, S> = Fixed<I, F, C, Signed, S>;

pub trait SignBits {
    const EXTRA: u32;
}
impl SignBits for Unsigned {
    const EXTRA: u32 = 0;
}
impl SignBits for Signed {
    const EXTRA: u32 = 1;
}

// ---------------------------------------------------------------------------
// The mathematical coordinates, as projections. Value position, always legal.
// ---------------------------------------------------------------------------
pub trait Format {
    const PRECISION: u32;
    const EXPONENT: i32;
    const INTEGER_DIGITS: u32;
    const FRACTION_DIGITS: u32;
    type Store: Container;
}
impl<const I: u32, const F: u32, C: Container, G: Sign + SignBits, S: Policy + Lowering> Format
    for Fixed<I, F, C, G, S>
{
    const PRECISION: u32 = G::EXTRA + I + F;
    const EXPONENT: i32 = -(F as i32);
    const INTEGER_DIGITS: u32 = I;
    const FRACTION_DIGITS: u32 = F;
    type Store = C;
}

impl<const I: u32, const F: u32, C: Container, G: Sign + SignBits, S: Policy + Lowering>
    Fixed<I, F, C, G, S>
{
    pub const FITS: () = assert!(
        G::EXTRA + I + F <= C::BITS,
        "arvo: the format does not fit its container.
  Fixed<I, F, C, ..> stores I + F significant digits, plus one sign digit when
  the sign axis is Signed, inside C. The instantiation printed above names I, F
  and C. Pick a wider C from the ladder, or write fewer digits."
    );
    pub const fn from_raw(raw: C) -> Self {
        let () = Self::FITS;
        Fixed {
            raw,
            _m: PhantomData,
        }
    }
    pub const fn to_raw(self) -> C {
        self.raw
    }
    pub const fn zero() -> Self {
        Self::from_raw(C::ZERO)
    }
}

// ---------------------------------------------------------------------------
// The laws. Every law is a named item, so rustc prints the law's name and its
// coordinates in the law's own order when it fails.
// ---------------------------------------------------------------------------
pub struct ProductFormat<
    const I: u32,
    const F: u32,
    const J: u32,
    const K: u32,
    const M: u32,
    const N: u32,
>;
impl<const I: u32, const F: u32, const J: u32, const K: u32, const M: u32, const N: u32>
    ProductFormat<I, F, J, K, M, N>
{
    pub const HOLDS: () = assert!(
        M == I + J && N == F + K,
        "arvo: the product's format does not follow from its inputs.
  The law: Fixed<I, F> times Fixed<J, K> has format Fixed<I + J, F + K>.
  The line above prints ProductFormat::<I, F, J, K, M, N> with the actual
  digit counts, in that order.
  If you wrote the call, name the output with those first four numbers added
  pairwise. If the call is inside a function you did not write, that function
  states a format relation that does not hold, and the note below names the
  function and the line. Search your own source for the two output numbers
  printed above to find which of your calls reached it."
    );
}

pub struct SumFormat<const I: u32, const J: u32, const M: u32>;
impl<const I: u32, const J: u32, const M: u32> SumFormat<I, J, M> {
    pub const HOLDS: () = assert!(
        M == (if I > J { I } else { J }) + 1,
        "arvo: the sum's integer digit count does not follow from its inputs.
  The law: adding two numerals of the same exponent widens the integer part
  to one digit above the wider input. The line above prints
  SumFormat::<I, J, M> with the actual digit counts.
  The exponents are already equal, because the signature requires it; only the
  integer width is a computed relation and only it can be got wrong here."
    );
}

pub struct WidenFormat<const I: u32, const A: u32, const M: u32>;
impl<const I: u32, const A: u32, const M: u32> WidenFormat<I, A, M> {
    pub const HOLDS: () = assert!(
        M == I + A,
        "arvo: the widened format does not follow from its input.
  The law: widening by A integer digits takes Fixed<I, F> to Fixed<I + A, F>.
  The line above prints WidenFormat::<I, A, M> with the actual digit counts."
    );
}

/// Alignment is an equality between coordinates, so it is a BOUND: the two
/// arguments share the parameter `F`. A misaligned call is refused where it is
/// written, before any monomorphisation.
pub fn add<
    const I: u32,
    const J: u32,
    const F: u32,
    const M: u32,
    C: Container,
    D: Container,
    G: Sign + SignBits,
    S: Policy + Lowering,
>(
    a: Fixed<I, F, C, G, S>,
    b: Fixed<J, F, C, G, S>,
) -> Fixed<M, F, D, G, S> {
    let () = SumFormat::<I, J, M>::HOLDS;
    let () = Fixed::<M, F, D, G, S>::FITS;
    let _ = (a, b);
    Fixed {
        raw: D::ZERO,
        _m: PhantomData,
    }
}

/// The product's coordinates are sums, so the relation is a check.
pub fn mul<
    const I: u32,
    const F: u32,
    const J: u32,
    const K: u32,
    const M: u32,
    const N: u32,
    C: Container,
    D: Container,
    G: Sign + SignBits,
    S: Policy + Lowering,
>(
    a: Fixed<I, F, C, G, S>,
    b: Fixed<J, K, C, G, S>,
) -> Fixed<M, N, D, G, S> {
    let () = ProductFormat::<I, F, J, K, M, N>::HOLDS;
    let () = Fixed::<M, N, D, G, S>::FITS;
    let _ = (a, b);
    Fixed {
        raw: D::ZERO,
        _m: PhantomData,
    }
}

pub fn widen_int<
    const I: u32,
    const F: u32,
    const A: u32,
    const M: u32,
    C: Container,
    D: Container,
    G: Sign + SignBits,
    S: Policy + Lowering,
>(
    x: Fixed<I, F, C, G, S>,
) -> Fixed<M, F, D, G, S> {
    let () = WidenFormat::<I, A, M>::HOLDS;
    let () = Fixed::<M, F, D, G, S>::FITS;
    let _ = x;
    Fixed {
        raw: D::ZERO,
        _m: PhantomData,
    }
}

/// Changing the exponent is an operation with a name, not an assignment.
/// This is the thing a precision-keyed numeral cannot make the consumer write.
pub fn rescale<
    const I: u32,
    const F: u32,
    const J: u32,
    const K: u32,
    C: Container,
    G: Sign + SignBits,
    S: Policy + Lowering,
>(
    x: Fixed<I, F, C, G, S>,
) -> Fixed<J, K, C, G, S> {
    let () = Fixed::<J, K, C, G, S>::FITS;
    let _ = x;
    Fixed {
        raw: C::ZERO,
        _m: PhantomData,
    }
}

// ---------------------------------------------------------------------------
// The bit view, reached with an output parameter rather than a computed type.
// ---------------------------------------------------------------------------
pub struct Bits<const W: u32, C: Container, S: Policy + Lowering> {
    raw: C,
    _s: PhantomData<S>,
}
pub fn bits_of<
    const I: u32,
    const F: u32,
    const W: u32,
    C: Container,
    G: Sign + SignBits,
    S: Policy + Lowering,
>(
    x: Fixed<I, F, C, G, S>,
) -> Bits<W, C, S> {
    let () = BitWidth::<I, F, W>::HOLDS_UNSIGNED;
    Bits {
        raw: x.to_raw(),
        _s: PhantomData,
    }
}
pub struct BitWidth<const I: u32, const F: u32, const W: u32>;
impl<const I: u32, const F: u32, const W: u32> BitWidth<I, F, W> {
    pub const HOLDS_UNSIGNED: () = assert!(
        W == I + F,
        "arvo: the bit view's width must equal the format's precision."
    );
}

// 64 distinct four-digit compositions, each with its own law check.
pub fn c0(
    a: UFixed<1000, 1000, u128, Warm>,
    b: UFixed<1200, 1300, u128, Warm>,
) -> UFixed<2200, 2300, u128, Warm> {
    mul(a, b)
}
pub fn c1(
    a: UFixed<1011, 1151, u128, Warm>,
    b: UFixed<1200, 1389, u128, Warm>,
) -> UFixed<2211, 2540, u128, Warm> {
    mul(a, b)
}
pub fn c2(
    a: UFixed<1022, 1302, u128, Warm>,
    b: UFixed<1200, 1478, u128, Warm>,
) -> UFixed<2222, 2780, u128, Warm> {
    mul(a, b)
}
pub fn c3(
    a: UFixed<1033, 1453, u128, Warm>,
    b: UFixed<1200, 1567, u128, Warm>,
) -> UFixed<2233, 3020, u128, Warm> {
    mul(a, b)
}
pub fn c4(
    a: UFixed<1044, 1604, u128, Warm>,
    b: UFixed<1200, 1656, u128, Warm>,
) -> UFixed<2244, 3260, u128, Warm> {
    mul(a, b)
}
pub fn c5(
    a: UFixed<1055, 1755, u128, Warm>,
    b: UFixed<1200, 1745, u128, Warm>,
) -> UFixed<2255, 3500, u128, Warm> {
    mul(a, b)
}
pub fn c6(
    a: UFixed<1066, 1906, u128, Warm>,
    b: UFixed<1200, 1834, u128, Warm>,
) -> UFixed<2266, 3740, u128, Warm> {
    mul(a, b)
}
pub fn c7(
    a: UFixed<1077, 2057, u128, Warm>,
    b: UFixed<1200, 1923, u128, Warm>,
) -> UFixed<2277, 3980, u128, Warm> {
    mul(a, b)
}
pub fn c8(
    a: UFixed<1137, 1007, u128, Warm>,
    b: UFixed<1297, 1300, u128, Warm>,
) -> UFixed<2434, 2307, u128, Warm> {
    mul(a, b)
}
pub fn c9(
    a: UFixed<1148, 1158, u128, Warm>,
    b: UFixed<1297, 1389, u128, Warm>,
) -> UFixed<2445, 2547, u128, Warm> {
    mul(a, b)
}
pub fn c10(
    a: UFixed<1159, 1309, u128, Warm>,
    b: UFixed<1297, 1478, u128, Warm>,
) -> UFixed<2456, 2787, u128, Warm> {
    mul(a, b)
}
pub fn c11(
    a: UFixed<1170, 1460, u128, Warm>,
    b: UFixed<1297, 1567, u128, Warm>,
) -> UFixed<2467, 3027, u128, Warm> {
    mul(a, b)
}
pub fn c12(
    a: UFixed<1181, 1611, u128, Warm>,
    b: UFixed<1297, 1656, u128, Warm>,
) -> UFixed<2478, 3267, u128, Warm> {
    mul(a, b)
}
pub fn c13(
    a: UFixed<1192, 1762, u128, Warm>,
    b: UFixed<1297, 1745, u128, Warm>,
) -> UFixed<2489, 3507, u128, Warm> {
    mul(a, b)
}
pub fn c14(
    a: UFixed<1203, 1913, u128, Warm>,
    b: UFixed<1297, 1834, u128, Warm>,
) -> UFixed<2500, 3747, u128, Warm> {
    mul(a, b)
}
pub fn c15(
    a: UFixed<1214, 2064, u128, Warm>,
    b: UFixed<1297, 1923, u128, Warm>,
) -> UFixed<2511, 3987, u128, Warm> {
    mul(a, b)
}
pub fn c16(
    a: UFixed<1274, 1014, u128, Warm>,
    b: UFixed<1394, 1300, u128, Warm>,
) -> UFixed<2668, 2314, u128, Warm> {
    mul(a, b)
}
pub fn c17(
    a: UFixed<1285, 1165, u128, Warm>,
    b: UFixed<1394, 1389, u128, Warm>,
) -> UFixed<2679, 2554, u128, Warm> {
    mul(a, b)
}
pub fn c18(
    a: UFixed<1296, 1316, u128, Warm>,
    b: UFixed<1394, 1478, u128, Warm>,
) -> UFixed<2690, 2794, u128, Warm> {
    mul(a, b)
}
pub fn c19(
    a: UFixed<1307, 1467, u128, Warm>,
    b: UFixed<1394, 1567, u128, Warm>,
) -> UFixed<2701, 3034, u128, Warm> {
    mul(a, b)
}
pub fn c20(
    a: UFixed<1318, 1618, u128, Warm>,
    b: UFixed<1394, 1656, u128, Warm>,
) -> UFixed<2712, 3274, u128, Warm> {
    mul(a, b)
}
pub fn c21(
    a: UFixed<1329, 1769, u128, Warm>,
    b: UFixed<1394, 1745, u128, Warm>,
) -> UFixed<2723, 3514, u128, Warm> {
    mul(a, b)
}
pub fn c22(
    a: UFixed<1340, 1920, u128, Warm>,
    b: UFixed<1394, 1834, u128, Warm>,
) -> UFixed<2734, 3754, u128, Warm> {
    mul(a, b)
}
pub fn c23(
    a: UFixed<1351, 2071, u128, Warm>,
    b: UFixed<1394, 1923, u128, Warm>,
) -> UFixed<2745, 3994, u128, Warm> {
    mul(a, b)
}
pub fn c24(
    a: UFixed<1411, 1021, u128, Warm>,
    b: UFixed<1491, 1300, u128, Warm>,
) -> UFixed<2902, 2321, u128, Warm> {
    mul(a, b)
}
pub fn c25(
    a: UFixed<1422, 1172, u128, Warm>,
    b: UFixed<1491, 1389, u128, Warm>,
) -> UFixed<2913, 2561, u128, Warm> {
    mul(a, b)
}
pub fn c26(
    a: UFixed<1433, 1323, u128, Warm>,
    b: UFixed<1491, 1478, u128, Warm>,
) -> UFixed<2924, 2801, u128, Warm> {
    mul(a, b)
}
pub fn c27(
    a: UFixed<1444, 1474, u128, Warm>,
    b: UFixed<1491, 1567, u128, Warm>,
) -> UFixed<2935, 3041, u128, Warm> {
    mul(a, b)
}
pub fn c28(
    a: UFixed<1455, 1625, u128, Warm>,
    b: UFixed<1491, 1656, u128, Warm>,
) -> UFixed<2946, 3281, u128, Warm> {
    mul(a, b)
}
pub fn c29(
    a: UFixed<1466, 1776, u128, Warm>,
    b: UFixed<1491, 1745, u128, Warm>,
) -> UFixed<2957, 3521, u128, Warm> {
    mul(a, b)
}
pub fn c30(
    a: UFixed<1477, 1927, u128, Warm>,
    b: UFixed<1491, 1834, u128, Warm>,
) -> UFixed<2968, 3761, u128, Warm> {
    mul(a, b)
}
pub fn c31(
    a: UFixed<1488, 2078, u128, Warm>,
    b: UFixed<1491, 1923, u128, Warm>,
) -> UFixed<2979, 4001, u128, Warm> {
    mul(a, b)
}
pub fn c32(
    a: UFixed<1548, 1028, u128, Warm>,
    b: UFixed<1588, 1300, u128, Warm>,
) -> UFixed<3136, 2328, u128, Warm> {
    mul(a, b)
}
pub fn c33(
    a: UFixed<1559, 1179, u128, Warm>,
    b: UFixed<1588, 1389, u128, Warm>,
) -> UFixed<3147, 2568, u128, Warm> {
    mul(a, b)
}
pub fn c34(
    a: UFixed<1570, 1330, u128, Warm>,
    b: UFixed<1588, 1478, u128, Warm>,
) -> UFixed<3158, 2808, u128, Warm> {
    mul(a, b)
}
pub fn c35(
    a: UFixed<1581, 1481, u128, Warm>,
    b: UFixed<1588, 1567, u128, Warm>,
) -> UFixed<3169, 3048, u128, Warm> {
    mul(a, b)
}
pub fn c36(
    a: UFixed<1592, 1632, u128, Warm>,
    b: UFixed<1588, 1656, u128, Warm>,
) -> UFixed<3180, 3288, u128, Warm> {
    mul(a, b)
}
pub fn c37(
    a: UFixed<1603, 1783, u128, Warm>,
    b: UFixed<1588, 1745, u128, Warm>,
) -> UFixed<3191, 3528, u128, Warm> {
    mul(a, b)
}
pub fn c38(
    a: UFixed<1614, 1934, u128, Warm>,
    b: UFixed<1588, 1834, u128, Warm>,
) -> UFixed<3202, 3768, u128, Warm> {
    mul(a, b)
}
pub fn c39(
    a: UFixed<1625, 2085, u128, Warm>,
    b: UFixed<1588, 1923, u128, Warm>,
) -> UFixed<3213, 4008, u128, Warm> {
    mul(a, b)
}
pub fn c40(
    a: UFixed<1685, 1035, u128, Warm>,
    b: UFixed<1685, 1300, u128, Warm>,
) -> UFixed<3370, 2335, u128, Warm> {
    mul(a, b)
}
pub fn c41(
    a: UFixed<1696, 1186, u128, Warm>,
    b: UFixed<1685, 1389, u128, Warm>,
) -> UFixed<3381, 2575, u128, Warm> {
    mul(a, b)
}
pub fn c42(
    a: UFixed<1707, 1337, u128, Warm>,
    b: UFixed<1685, 1478, u128, Warm>,
) -> UFixed<3392, 2815, u128, Warm> {
    mul(a, b)
}
pub fn c43(
    a: UFixed<1718, 1488, u128, Warm>,
    b: UFixed<1685, 1567, u128, Warm>,
) -> UFixed<3403, 3055, u128, Warm> {
    mul(a, b)
}
pub fn c44(
    a: UFixed<1729, 1639, u128, Warm>,
    b: UFixed<1685, 1656, u128, Warm>,
) -> UFixed<3414, 3295, u128, Warm> {
    mul(a, b)
}
pub fn c45(
    a: UFixed<1740, 1790, u128, Warm>,
    b: UFixed<1685, 1745, u128, Warm>,
) -> UFixed<3425, 3535, u128, Warm> {
    mul(a, b)
}
pub fn c46(
    a: UFixed<1751, 1941, u128, Warm>,
    b: UFixed<1685, 1834, u128, Warm>,
) -> UFixed<3436, 3775, u128, Warm> {
    mul(a, b)
}
pub fn c47(
    a: UFixed<1762, 2092, u128, Warm>,
    b: UFixed<1685, 1923, u128, Warm>,
) -> UFixed<3447, 4015, u128, Warm> {
    mul(a, b)
}
pub fn c48(
    a: UFixed<1822, 1042, u128, Warm>,
    b: UFixed<1782, 1300, u128, Warm>,
) -> UFixed<3604, 2342, u128, Warm> {
    mul(a, b)
}
pub fn c49(
    a: UFixed<1833, 1193, u128, Warm>,
    b: UFixed<1782, 1389, u128, Warm>,
) -> UFixed<3615, 2582, u128, Warm> {
    mul(a, b)
}
pub fn c50(
    a: UFixed<1844, 1344, u128, Warm>,
    b: UFixed<1782, 1478, u128, Warm>,
) -> UFixed<3626, 2822, u128, Warm> {
    mul(a, b)
}
pub fn c51(
    a: UFixed<1855, 1495, u128, Warm>,
    b: UFixed<1782, 1567, u128, Warm>,
) -> UFixed<3637, 3062, u128, Warm> {
    mul(a, b)
}
pub fn c52(
    a: UFixed<1866, 1646, u128, Warm>,
    b: UFixed<1782, 1656, u128, Warm>,
) -> UFixed<3648, 3302, u128, Warm> {
    mul(a, b)
}
pub fn c53(
    a: UFixed<1877, 1797, u128, Warm>,
    b: UFixed<1782, 1745, u128, Warm>,
) -> UFixed<3659, 3542, u128, Warm> {
    mul(a, b)
}
pub fn c54(
    a: UFixed<1888, 1948, u128, Warm>,
    b: UFixed<1782, 1834, u128, Warm>,
) -> UFixed<3670, 3782, u128, Warm> {
    mul(a, b)
}
pub fn c55(
    a: UFixed<1899, 2099, u128, Warm>,
    b: UFixed<1782, 1923, u128, Warm>,
) -> UFixed<3681, 4022, u128, Warm> {
    mul(a, b)
}
pub fn c56(
    a: UFixed<1959, 1049, u128, Warm>,
    b: UFixed<1879, 1300, u128, Warm>,
) -> UFixed<3838, 2349, u128, Warm> {
    mul(a, b)
}
pub fn c57(
    a: UFixed<1970, 1200, u128, Warm>,
    b: UFixed<1879, 1389, u128, Warm>,
) -> UFixed<3849, 2589, u128, Warm> {
    mul(a, b)
}
pub fn c58(
    a: UFixed<1981, 1351, u128, Warm>,
    b: UFixed<1879, 1478, u128, Warm>,
) -> UFixed<3860, 2829, u128, Warm> {
    mul(a, b)
}
pub fn c59(
    a: UFixed<1992, 1502, u128, Warm>,
    b: UFixed<1879, 1567, u128, Warm>,
) -> UFixed<3871, 3069, u128, Warm> {
    mul(a, b)
}
pub fn c60(
    a: UFixed<2003, 1653, u128, Warm>,
    b: UFixed<1879, 1656, u128, Warm>,
) -> UFixed<3882, 3309, u128, Warm> {
    mul(a, b)
}
pub fn c61(
    a: UFixed<2014, 1804, u128, Warm>,
    b: UFixed<1879, 1745, u128, Warm>,
) -> UFixed<3893, 3549, u128, Warm> {
    mul(a, b)
}
pub fn c62(
    a: UFixed<2025, 1955, u128, Warm>,
    b: UFixed<1879, 1834, u128, Warm>,
) -> UFixed<3904, 3789, u128, Warm> {
    mul(a, b)
}
pub fn c63(
    a: UFixed<2036, 2106, u128, Warm>,
    b: UFixed<1879, 1923, u128, Warm>,
) -> UFixed<3915, 4029, u128, Warm> {
    mul(a, b)
}

// the projections agree at every one of them
const _: () = assert!(<UFixed<1000, 1000, u128, Warm> as Format>::PRECISION == 2000);
const _: () = assert!(<UFixed<1137, 1151, u128, Warm> as Format>::PRECISION == 2288);
const _: () = assert!(<UFixed<1274, 1302, u128, Warm> as Format>::PRECISION == 2576);
const _: () = assert!(<UFixed<1411, 1453, u128, Warm> as Format>::PRECISION == 2864);
const _: () = assert!(<UFixed<1548, 1604, u128, Warm> as Format>::PRECISION == 3152);
const _: () = assert!(<UFixed<1685, 1755, u128, Warm> as Format>::PRECISION == 3440);
const _: () = assert!(<UFixed<1822, 1906, u128, Warm> as Format>::PRECISION == 3728);
const _: () = assert!(<UFixed<1959, 2057, u128, Warm> as Format>::PRECISION == 4016);
