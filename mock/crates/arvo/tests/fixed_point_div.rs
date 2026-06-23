//! Fixed-point divide (`IArith::i_div_fixed` / `UArith::u_div_fixed`): the FRAC-rescaled divide that
//! `IFixed` / `UFixed` `/` routes through. The result raw is `(a << FRAC) / b`; the numerator is widened so
//! `a << FRAC` does not overflow before the divide. Truncates toward zero (the natural integer-`/` rounding,
//! a documented asymmetry with the multiply's floor toward minus infinity).

#![no_std]

use arvo::ifixed::IFixed;
use arvo::strategy::{Cold, Hot, IArith, Precise, UArith, Warm};
use arvo::{fbits, ibits};

// Signed<31, 16, Hot>: ifixed_bits(31, 16) = 48 -> i64 container (Min), FRAC = 16.
const SQ: u16 = 48;

#[test]
fn i_div_fixed_rescales_fractional() {
    // 1.0 / 2.0 = 0.5. raw (1<<16) / (2<<16): (a<<16)/b = (1<<32)/(2<<16) = 1<<15 = raw 0.5.
    assert_eq!(<Hot as IArith<SQ>>::i_div_fixed::<16>(1 << 16, 2 << 16), 1 << 15);
    // 3.0 / 4.0 = 0.75 = raw 3<<14.
    assert_eq!(<Hot as IArith<SQ>>::i_div_fixed::<16>(3 << 16, 4 << 16), 3 << 14);
}

#[test]
fn i_div_fixed_widens_numerator() {
    // a = raw 1<<47 (a large in-range 48-bit value), b = 1.0 (raw 1<<16): result = a = 1<<47. The numerator
    // `a << 16` = 1<<63 overflows the i64 container; a non-widening `(a<<16)/b` would wrap. The i128 widen
    // gives the correct 1<<47.
    assert_eq!(<Hot as IArith<SQ>>::i_div_fixed::<16>(1 << 47, 1 << 16), 1 << 47);
}

#[test]
fn i_div_fixed_frac_zero_is_integer_divide() {
    // FRAC = 0 reduces to integer divide, truncating toward zero.
    assert_eq!(<Hot as IArith<SQ>>::i_div_fixed::<0>(7, 2), 3);
}

#[test]
fn i_div_fixed_truncates_toward_zero_on_negatives() {
    // Divide truncates toward zero (NOT floor toward minus infinity): -7 / 2 = -3, not -4. This pins the
    // documented rounding asymmetry with the multiply (which floors).
    assert_eq!(<Hot as IArith<SQ>>::i_div_fixed::<0>(-7, 2), -3);
    // -1.0 / 2.0 = -0.5 = raw -(1<<15).
    assert_eq!(<Hot as IArith<SQ>>::i_div_fixed::<16>(-(1 << 16), 2 << 16), -(1 << 15));
}

#[test]
fn i_div_fixed_cold_widens_like_hot() {
    // Cold N=8 -> i8 container. raw 16 (=1.0 at FRAC=4) / raw 8 (=0.5) = 2.0 = raw 32. The numerator 16<<4 =
    // 256 overflows i8; the i128 widen gives the correct 256/8 = 32.
    assert_eq!(<Cold as IArith<8>>::i_div_fixed::<4>(16, 8), 32);
}

#[test]
fn u_div_fixed_hot_widens_unsigned() {
    // Hot N=8 -> u8 container. raw 16 (1.0) / raw 8 (0.5) = 2.0 = raw 32. Numerator 16<<4 = 256 overflows
    // u8; the u128 widen gives 256/8 = 32.
    assert_eq!(<Hot as UArith<8>>::u_div_fixed::<4>(16, 8), 32);
}

#[test]
fn u_div_fixed_warm_doublelogical() {
    // Warm is DoubleLogical: UFixed<_, _, Warm> at logical 16 uses a u32 container (2x), which holds the
    // shifted numerator. 8.0 (raw 8<<8) / 2.0 (raw 2<<8) = 4.0 (raw 4<<8) with FRAC = 8.
    assert_eq!(<Warm as UArith<16>>::u_div_fixed::<8>(8 << 8, 2 << 8), 4 << 8);
}

#[test]
fn i_div_fixed_by_zero_returns_numerator_for_wrapping() {
    // Wrapping strategies (Hot) never panic on div-by-zero: return the numerator (the existing convention).
    assert_eq!(<Hot as IArith<SQ>>::i_div_fixed::<16>(5 << 16, 0), 5 << 16);
}

#[test]
fn i_div_fixed_precise_clamps_to_logical_bound() {
    // Precise N=8 -> i16 container (2x). raw 100 / raw 1 at FRAC=4: (100<<4)/1 = 1600, far above the logical
    // i8 max 127, so it clamps to 127.
    assert_eq!(<Precise as IArith<8>>::i_div_fixed::<4>(100, 1), 127);
}

#[test]
fn i_div_fixed_precise_by_zero_clamps_to_logical_max() {
    // Precise div-by-zero clamps to the logical MAX (127), not the container MAX.
    assert_eq!(<Precise as IArith<8>>::i_div_fixed::<4>(50, 0), 127);
}

#[test]
fn fixed_point_div_logical_level() {
    // Logical-level `IFixed / IFixed` through the `Div` operator end to end. Q = IFixed<31,16,Hot> -> i64.
    // 3.0 / 4.0 = 0.75.
    type Q = IFixed<{ ibits(31) }, { fbits(16) }, Hot>;
    let a = Q::from_raw(3 << 16);
    let b = Q::from_raw(4 << 16);
    assert_eq!((a / b).to_raw(), 3 << 14, "3.0 / 4.0 = 0.75 through `/`");
}

#[test]
fn fixed_point_div_one_is_right_identity() {
    // x / 1.0 == x through the operator (ONE = raw 1<<16).
    type Q = IFixed<{ ibits(31) }, { fbits(16) }, Hot>;
    let x = Q::from_raw(7 << 16);
    let one = Q::from_raw(1 << 16);
    assert_eq!((x / one).to_raw(), x.to_raw(), "x / 1.0 == x");
}

// Catalogue red (tracked task #5): the >64-bit-logical fixed-point divide needs a 256/128 long division
// (the numerator `a << FRAC` exceeds the 128-bit container, and there is no `carrying_div` intrinsic the way
// `carrying_mul` served the multiply). Hot/Cold 65..128 use the naive in-container `(a << FRAC) / b`, which
// is correct only when the numerator does not overflow. This pins the intended widened result; it fails
// today and goes green when the 256/128 divide lands.
#[test]
#[ignore = "catalogue: >64-bit-logical fixed-point divide needs 256/128 long division; tracked #5"]
fn i_div_fixed_above_64_bits_catalogue() {
    // Hot N=128 -> i128 container. raw 1<<100 / 1.0 (raw 1<<30) at FRAC=30 = raw 1<<100. The numerator
    // (1<<100) << 30 = 1<<130 overflows i128 and wraps in the naive body, so the result is wrong; the
    // 256/128 widen gives (1<<130) / (1<<30) = 1<<100.
    assert_eq!(<Hot as IArith<128>>::i_div_fixed::<30>(1 << 100, 1 << 30), 1 << 100);
}
