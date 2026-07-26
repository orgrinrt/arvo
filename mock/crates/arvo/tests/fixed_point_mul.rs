//! Fixed-point multiply (`IArith::i_mul_fixed` / `UArith::u_mul_fixed`): the FRAC-rescaled multiply that
//! `IFixed` / `UFixed` `*` will route through. Verifies the rescale and, for Min-container strategies
//! (Hot/Cold), the overflow-safe widening (the raw product overflows the container but the widened product
//! is correct after the shift).

#![no_std]

use arvo::ifixed::IFixed;
use arvo::strategy::{
    Additive, Cold, Hot, IArith, Identity, Multiplicative, Precise, UArith, Warm,
};
use arvo::ufixed::UFixed;
use arvo::{fbits, ibits};

// Signed<31, 16, Hot>: ifixed_bits(31, 16) = 1 + 31 + 16 = 48 -> i64 container (Min), FRAC = 16.
const SQ: u16 = 48;

// NOTE: `i_mul_fixed` is the container-level primitive; its inputs/outputs are RAW values, and `FRAC` says
// how many low bits are fractional. So a raw `r` denotes the logical value `r / 2^FRAC`. The small raws
// below are chosen to exercise the `>>FRAC` rescale and the overflow-widening at the container level; the
// logical-level (`IFixed * IFixed`) behaviour is pinned by the identity/logical tests at the bottom.

#[test]
fn i_mul_fixed_rescales_fractional() {
    // raw 10<<16 = value 10.0, raw 1<<15 = value 0.5; 10.0 * 0.5 = 5.0 = raw 5<<16. A raw product without
    // the `>>FRAC` rescale would be off by 2^16.
    let a: i64 = 10 << 16;
    let b: i64 = 1 << 15;
    assert_eq!(<Hot as IArith<SQ>>::i_mul_fixed::<16>(a, b), 5 << 16);
}

#[test]
fn i_mul_fixed_widens_min_container() {
    // 65536.0 (raw 1<<32) squared = 2^32 (raw 1<<48). The raw product 1<<64 overflows the i64 container;
    // a non-widening `(a*b)>>16` would wrap to 0. The i128 widen gives the correct 1<<48.
    let big: i64 = 1 << 32;
    assert_eq!(<Hot as IArith<SQ>>::i_mul_fixed::<16>(big, big), 1 << 48);
}

#[test]
fn i_mul_fixed_frac_zero_is_integer_multiply() {
    // FRAC = 0 reduces to the integer multiply (no rescale).
    let a: i64 = 7;
    let b: i64 = 6;
    assert_eq!(<Hot as IArith<SQ>>::i_mul_fixed::<0>(a, b), 42);
}

#[test]
fn u_mul_fixed_doublelogical_no_extra_widen() {
    // Warm is DoubleLogical: UFixed<_, _, Warm> at logical width 16 uses a u32 container (2x), which
    // already holds the product, so u_mul_fixed = u_mul >> FRAC. 4.0 (raw 4<<8) * 2.0 (raw 2<<8) = 8.0
    // (raw 8<<8) with FRAC = 8.
    let a: u32 = 4 << 8;
    let b: u32 = 2 << 8;
    assert_eq!(<Warm as UArith<16>>::u_mul_fixed::<8>(a, b), 8 << 8);
}

#[test]
fn fixed_point_one_is_multiplicative_identity() {
    // The coupled ONE + Mul flip: with `*` rescaling by F, `Identity::<Multiplicative>::IDENTITY` must be the fixed-point one
    // (raw 1<<F) so that `x * ONE == x`. F = 16 here (a genuinely fractional width).
    type Q = IFixed<{ ibits(15) }, { fbits(16) }, Hot>;
    let one = <Q as Identity<Multiplicative>>::IDENTITY;
    // ONE is raw 1<<16, not raw 1.
    assert_eq!(one.to_raw(), 1 << 16);
    let x = Q::from_raw(3 << 16); // 3.0
    assert_eq!((x * one).to_raw(), x.to_raw(), "x * ONE must equal x");
    assert_eq!(
        (one * one).to_raw(),
        one.to_raw(),
        "ONE * ONE must equal ONE"
    );
    // a real fractional product: 2.5 * 1.5 = 3.75.
    let two_five = Q::from_raw((5 << 16) / 2);
    let one_five = Q::from_raw((3 << 16) / 2);
    assert_eq!(
        (two_five * one_five).to_raw(),
        (15 << 16) / 4,
        "2.5 * 1.5 = 3.75"
    );
}

// --- Guard tests: the per-strategy paths and constraints this round established. If a future change
//     breaks the widening, the saturation, the integer fast path, or the identity, one of these fails. ---

#[test]
fn i_mul_fixed_cold_widens_like_hot() {
    // Cold is the other Min-container strategy: it must widen like Hot. Cold N=8 -> i8 container. raw 20 at
    // FRAC=4 = value 1.25; 1.25 * 1.25 = 1.5625 = raw 25. The container product 20*20=400 overflows i8
    // (max 127), so without the i128 widen the result would wrap; the widen gives the correct 400>>4 = 25.
    assert_eq!(<Cold as IArith<8>>::i_mul_fixed::<4>(20, 20), 25);
}

#[test]
fn u_mul_fixed_hot_widens_unsigned() {
    // Unsigned Min widening: Hot N=8 -> u8 container. raw 20 at FRAC=4 = value 1.25; 1.25*1.25 = 1.5625 =
    // raw 25. The container product 20*20=400 overflows u8 (would wrap to 144 -> 144>>4 = 9); the u128
    // widen gives the correct 400>>4 = 25.
    assert_eq!(<Hot as UArith<8>>::u_mul_fixed::<4>(20, 20), 25);
}

#[test]
fn i_mul_fixed_precise_in_range_rescales() {
    // Precise is DoubleLogical + saturating: N=8 -> i16 container (2x). For in-range 8-bit-logical operands
    // the 2x container holds the product, so no clamp fires and the result is the plain rescale. raw 8 at
    // FRAC=4 = value 0.5; 0.5 * 0.5 = 0.25 = raw 4. (Out-of-range Precise now clamps to the LOGICAL bound,
    // not the container bound; round 202606231229 fixed the arvo-wide bug. See the logical-saturation tests
    // below.)
    assert_eq!(<Precise as IArith<8>>::i_mul_fixed::<4>(8, 8), 4);
}

#[test]
fn fixed_point_one_identity_unsigned_and_doublelogical() {
    // The identity holds for unsigned types and for a DoubleLogical strategy too. UFixed<8, 8, Warm>:
    // logical 16 -> u32 container (2x), F = 8, so ONE = raw 1<<8.
    type U = UFixed<{ ibits(8) }, { fbits(8) }, Warm>;
    let one = <U as Identity<Multiplicative>>::IDENTITY;
    assert_eq!(one.to_raw(), 1 << 8);
    let x = U::from_raw(5 << 8); // 5.0
    assert_eq!(
        (x * one).to_raw(),
        x.to_raw(),
        "unsigned x * ONE must equal x"
    );
}

#[test]
fn fixed_point_identity_integer_width_is_raw_one() {
    // At F = 0 the fixed-point one collapses to the integer one (raw 1), and `*` is integer multiply.
    type I = IFixed<{ ibits(15) }, { fbits(0) }, Hot>;
    let one = <I as Identity<Multiplicative>>::IDENTITY;
    assert_eq!(one.to_raw(), 1);
    let x = I::from_raw(9);
    assert_eq!((x * one).to_raw(), 9, "integer x * ONE == x");
}

#[test]
fn fixed_point_mul_logical_level_requires_widen() {
    // Logical-level `IFixed * IFixed` through the full `Mul` path, with a product whose container form
    // overflows i64 before the shift, exercising the Hot widen end to end. Q = IFixed<31,16,Hot> -> i64.
    // a = 65536.0 (raw 1<<32); 65536.0 * 65536.0 = 2^32 (= raw 1<<48). The container product 1<<64
    // overflows i64; the i128 widen makes it correct (a non-widening `*` would wrap to 0).
    type Q = IFixed<{ ibits(31) }, { fbits(16) }, Hot>;
    let a = Q::from_raw(1 << 32);
    assert_eq!(
        (a * a).to_raw(),
        1 << 48,
        "65536.0^2 = 2^32 via the widen, through `*`"
    );
}

#[test]
fn i_mul_fixed_signed_negatives() {
    // Sign coverage on the Min widen path (Hot N=8 -> i8, widen to i128). raws at FRAC=4: -16 = -1.0,
    // 16 = 1.0, -24 = -1.5.
    assert_eq!(
        <Hot as IArith<8>>::i_mul_fixed::<4>(-16, 16),
        -16,
        "-1.0 * 1.0 = -1.0"
    );
    assert_eq!(
        <Hot as IArith<8>>::i_mul_fixed::<4>(-16, -16),
        16,
        "-1.0 * -1.0 = 1.0"
    );
    assert_eq!(
        <Hot as IArith<8>>::i_mul_fixed::<4>(-24, 16),
        -24,
        "-1.5 * 1.0 = -1.5"
    );
}

#[test]
fn i_mul_fixed_negative_floor_rounding() {
    // The rescale is an arithmetic right shift, so negative results floor toward minus infinity (not toward
    // zero). raw -1 (=-1/16) * raw 8 (=0.5) = -1/32, which at FRAC=4 lands between raw -1 and raw 0 and
    // floors to raw -1. This pins the rounding policy; whether floor (vs toward-zero / nearest) is the
    // intended fixed-point rounding is a design question tracked in task #3.
    assert_eq!(
        <Hot as IArith<8>>::i_mul_fixed::<4>(-1, 8),
        -1,
        "arithmetic-shift floor on negatives"
    );
}

#[test]
fn zero_integer_width_has_no_multiplicative_identity() {
    // `IFixed<0, F, S>` spans [-1, 1), which does not contain one, so it has no
    // multiplicative identity and its `Identity<Multiplicative>` impl does not
    // exist. The additive one is unaffected: zero is representable at every width.
    //
    // This replaces a test that asserted the opposite. That one named
    // `IFixed<0, 16, Hot>`, read `1 << 16` back out of it, and concluded in a
    // comment that the edge "is not an overflow bug". It passed for a reason that
    // has nothing to do with the type being correct: 17 logical bits project to an
    // i32 container under Hot, so the encoding had room. At `IFixed<0, 7, Hot>` the
    // container is i8 and the same encoding lands on -128, so multiplying by the
    // identity flipped sign. Container slack is strategy-dependent and is not what
    // makes a value a member of the type.
    //
    // The refusal itself is a compile-fail case and cannot be written here or
    // in `identity_laws.rs`, which says so explicitly. It is pinned under
    // `tests/ui/`, one case per impl and per strategy.
    type Q = IFixed<{ ibits(0) }, { fbits(16) }, Hot>;
    let zero = <Q as Identity<Additive>>::IDENTITY;
    assert_eq!(zero.to_raw(), 0, "Q0.F still has an additive identity");
    let x = Q::from_raw(1 << 15); // 0.5
    assert_eq!((x + zero).to_raw(), x.to_raw(), "Q0.F: x + ZERO == x");
}

// --- The >64-bit-logical fixed-point multiply (256-bit widen, round 202606231229). Hot/Cold 65..128 use a
//     u128/i128 container; the 2x product is 256 bits, formed via `carrying_mul`, shifted, narrowed. ---

#[test]
fn i_mul_fixed_widen_above_64_bits() {
    // Hot N=128 -> i128 container. raw 1<<64 at FRAC=30; (1<<64)^2 = 1<<128 overflows i128 and would wrap to
    // 0 in a non-widening body, so >>30 = 0. The 256-bit widen gives the correct (1<<128)>>30 = 1<<98.
    assert_eq!(
        <Hot as IArith<128>>::i_mul_fixed::<30>(1 << 64, 1 << 64),
        1 << 98
    );
}

#[test]
fn u_mul_fixed_widen_above_64_bits_unsigned() {
    // Unsigned 256-bit widen: Hot N=128 -> u128 container. Same product as the signed case, via umul256.
    assert_eq!(
        <Hot as UArith<128>>::u_mul_fixed::<30>(1 << 64, 1 << 64),
        1 << 98
    );
}

#[test]
fn i_mul_fixed_widen_above_64_bits_cold() {
    // Cold is the other Min-container strategy at N=128 -> i128; it must take the same 256-bit widen.
    assert_eq!(
        <Cold as IArith<128>>::i_mul_fixed::<30>(1 << 64, 1 << 64),
        1 << 98
    );
}

#[test]
fn i_mul_fixed_widen_above_64_bits_negative_exact() {
    // Sign on the 256-bit path: -(1<<64) * (1<<64) = -(1<<128); >>30 with no dropped bits = -(1<<98).
    assert_eq!(
        <Hot as IArith<128>>::i_mul_fixed::<30>(-(1 << 64), 1 << 64),
        -(1 << 98)
    );
}

#[test]
fn i_mul_fixed_widen_above_64_bits_negative_floor() {
    // The floor-toward-minus-infinity correction on the 256-bit path (the silent-regression trap). N=128.
    // raw -1 * raw 1 >> 30 = -1/2^30, which floors to raw -1 (NOT 0, which a magnitude/sign multiply
    // without the floor correction would give). The dropped low bit triggers the `-m - 1` branch.
    assert_eq!(<Hot as IArith<128>>::i_mul_fixed::<30>(-1, 1), -1);
}

// --- Precise saturates at the LOGICAL bound, not the container bound (round 202606231229 fixed this
//     arvo-wide; it affects every Precise op, not just the multiply). ---

#[test]
fn i_mul_fixed_precise_saturates_to_logical_bound() {
    // Precise N=8 -> i16 container. Logical range is 8-bit signed: max raw 127. raw 100 (6.25) * raw 100 at
    // FRAC=4 = 625 (10000>>4), above the logical max, so it clamps to raw 127. Before the fix it returned
    // 625 unclamped (the i16 container did not overflow, so the old container-bound saturation never fired).
    assert_eq!(<Precise as IArith<8>>::i_mul_fixed::<4>(100, 100), 127);
}

#[test]
fn i_add_precise_saturates_to_logical_bound() {
    // Precise add clamps to the logical bound: i8 logical max 127. 100 + 100 = 200 -> 127.
    assert_eq!(<Precise as IArith<8>>::i_add(100, 100), 127);
}

#[test]
fn i_sub_precise_saturates_to_logical_min() {
    // Precise sub clamps to the logical MIN: i8 logical min -128. -100 - 100 = -200 -> -128.
    assert_eq!(<Precise as IArith<8>>::i_sub(-100, 100), -128);
}

#[test]
fn i_div_precise_by_zero_clamps_to_logical_max() {
    // Precise div-by-zero never panics; it clamps to the logical MAX (127), not the container MAX.
    assert_eq!(<Precise as IArith<8>>::i_div(50, 0), 127);
}

#[test]
fn u_add_precise_saturates_to_logical_bound() {
    // Unsigned Precise add clamps to the logical MAX: u8 logical max 255. 200 + 100 = 300 -> 255.
    assert_eq!(<Precise as UArith<8>>::u_add(200, 100), 255);
}

#[test]
fn i_mul_fixed_precise_doublelogical_33_64_clamps() {
    // Where Track 1 (logical clamp) meets the DoubleLogical 33..64 band: Precise N=40 -> i128 container
    // (2N=80) holds the product, then the result clamps to the logical 40-bit bound. raw 1<<25 * raw 1<<25
    // at FRAC=0 = 1<<50, above the logical i40 max (1<<39)-1, so it clamps there.
    assert_eq!(
        <Precise as IArith<40>>::i_mul_fixed::<0>(1 << 25, 1 << 25),
        (1 << 39) - 1
    );
}
