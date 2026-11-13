//! Each rung's arithmetic against the oracle, exhaustively, independently of the
//! rung the map would pick.
//!
//! Widths 1 to 8 in every rung, both slot families, both policies; widths 9 and
//! 10 in their own rung. A mutant feature (`mutant-8` and so on) removes the
//! re-canonicalising step in one rung; the tests of that rung must then fail.

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::{Integer, UFixed};
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, DeclaredSignature, Policy, Signature, Width};
use common::oracle::{members, oracle_add};
use common::rung;

/// How many pairs of members a rung's `add` gets wrong against the oracle.
fn mismatches<S: DeclaredSignature>(
    width: u32,
    policy: Policy,
    add: impl Fn(i128, i128, Width, Policy) -> i128,
) -> usize {
    let mut wrong = 0;
    for a in members::<S>() {
        for b in members::<S>() {
            if add(a, b, Width::bits(width), policy) != oracle_add::<S>(a, b) {
                wrong += 1;
            }
        }
    }
    wrong
}

/// Every width listed, both families, both policies, in one rung.
macro_rules! sweep {
    ($test:ident, $s:ty, $u:ty, $add_s:path, $add_u:path, [$($w:literal)+]) => {
        #[test]
        fn $test() {
            let mut wrong = 0;
            $(
                wrong += mismatches::<Signature<Integer<$w>, Adapt<TowardZero, Wrap>>>(
                    $w, Policy::Wrap, |a, b, w, p| $add_s(a as $s, b as $s, w, p) as i128);
                wrong += mismatches::<Signature<Integer<$w>, Adapt<TowardZero, Saturate>>>(
                    $w, Policy::Saturate, |a, b, w, p| $add_s(a as $s, b as $s, w, p) as i128);
                wrong += mismatches::<Signature<UFixed<$w, 0>, Adapt<TowardZero, Wrap>>>(
                    $w, Policy::Wrap, |a, b, w, p| $add_u(a as $u, b as $u, w, p) as i128);
                wrong += mismatches::<Signature<UFixed<$w, 0>, Adapt<TowardZero, Saturate>>>(
                    $w, Policy::Saturate, |a, b, w, p| $add_u(a as $u, b as $u, w, p) as i128);
            )+
            assert_eq!(wrong, 0, "pairs of members the rung's add gets wrong");
        }
    };
}

sweep!(rung_8_widths_1_to_8, i8, u8, rung::add_s8, rung::add_u8, [1 2 3 4 5 6 7 8]);
sweep!(rung_16_widths_1_to_8, i16, u16, rung::add_s16, rung::add_u16, [1 2 3 4 5 6 7 8]);
sweep!(rung_32_widths_1_to_8, i32, u32, rung::add_s32, rung::add_u32, [1 2 3 4 5 6 7 8]);
sweep!(rung_64_widths_1_to_8, i64, u64, rung::add_s64, rung::add_u64, [1 2 3 4 5 6 7 8]);
sweep!(rung_16_widths_9_and_10, i16, u16, rung::add_s16, rung::add_u16, [9 10]);
