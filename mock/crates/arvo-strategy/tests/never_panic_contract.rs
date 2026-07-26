//! `Hot` and `Cold` return a value for every input. They never panic.
//!
//! That is the whole reason to choose them: they wrap at the container
//! width instead of guarding, so the caller trades range for a branch-free
//! path that always produces something. A panic from either is a broken
//! contract, and under const evaluation it is worse than a broken contract,
//! because an overflow there is a hard error that fails the build rather
//! than the run.
//!
//! The contract was unguarded and one operation broke it. The signed
//! fixed-point multiply reached a negative result by negating an unsigned
//! magnitude, and at the container minimum that magnitude is `2^127`, which
//! casts to `i128::MIN`; negating that overflows. Nothing named the case, so
//! nothing caught it.
//!
//! The extremes are where a sign-magnitude path has no symmetric answer, so
//! that is what this walks: both container bounds, zero, and the values
//! adjacent to each, across every operation and both wrapping strategies.

#![no_std]
#![feature(macro_metavar_expr_concat)]

use arvo_strategy::{
    Additive, BitsContainerFor, Bounded, Cold, Hot, IArith, Identity, Multiplicative, Precise,
    Signed, SignedIdentity, UArith, Unsigned, Warm,
};

macro_rules! signed_never_panics {
    ($name:ident, $strategy:ty, $bits:literal) => {
        #[test]
        fn $name() {
            // The container type is never named. It is a function of the
            // strategy as well as the width: `Warm` at 8 logical bits projects
            // to `i16`, `Hot` to `i8`. Every probe comes from the container's
            // own constants instead, so the test cannot encode a guess about
            // a projection it is not testing.
            type T = <$strategy as BitsContainerFor<$bits, Signed>>::T;
            let min = <T as Bounded>::MIN;
            let max = <T as Bounded>::MAX;
            let zero = <T as Identity<Additive>>::IDENTITY;
            let one = <T as Identity<Multiplicative>>::IDENTITY;

            // MIN is the one with no positive counterpart: -MIN is not
            // representable, so a sign-magnitude path has no answer there.
            // Every other probe has a symmetric partner and would pass a test
            // that omitted it.
            let probes = [
                min,
                <$strategy as IArith<$bits>>::i_add(min, one),
                <T as SignedIdentity>::NEG_ONE,
                zero,
                one,
                <$strategy as IArith<$bits>>::i_sub(max, one),
                max,
            ];

            for a in probes {
                for b in probes {
                    // Each must return rather than panic. The value is the
                    // operation's business; that there IS one is this test's.
                    let _ = <$strategy as IArith<$bits>>::i_add(a, b);
                    let _ = <$strategy as IArith<$bits>>::i_sub(a, b);
                    let _ = <$strategy as IArith<$bits>>::i_mul(a, b);
                    let _ = <$strategy as IArith<$bits>>::i_div(a, b);

                    // The regressions. `i_mul_fixed::<0>(MIN, 1)` panicked on
                    // "attempt to negate with overflow"; `i_div_fixed(MIN, -1)`
                    // panicked on "attempt to divide with overflow".
                    let _ = <$strategy as IArith<$bits>>::i_mul_fixed::<0>(a, b);
                    let _ = <$strategy as IArith<$bits>>::i_mul_fixed::<1>(a, b);
                    let _ = <$strategy as IArith<$bits>>::i_mul_fixed::<4>(a, b);
                    let _ = <$strategy as IArith<$bits>>::i_div_fixed::<0>(a, b);
                    let _ = <$strategy as IArith<$bits>>::i_div_fixed::<1>(a, b);
                    let _ = <$strategy as IArith<$bits>>::i_div_fixed::<4>(a, b);
                }
            }
        }
    };
}

macro_rules! unsigned_never_panics {
    ($name:ident, $strategy:ty, $bits:literal) => {
        #[test]
        fn $name() {
            type T = <$strategy as BitsContainerFor<$bits, Unsigned>>::T;
            let max = <T as Bounded>::MAX;
            let zero = <T as Identity<Additive>>::IDENTITY;
            let one = <T as Identity<Multiplicative>>::IDENTITY;

            let probes = [
                zero,
                one,
                <$strategy as UArith<$bits>>::u_add(one, one),
                <$strategy as UArith<$bits>>::u_sub(max, one),
                max,
            ];

            for a in probes {
                for b in probes {
                    let _ = <$strategy as UArith<$bits>>::u_add(a, b);
                    let _ = <$strategy as UArith<$bits>>::u_sub(a, b);
                    let _ = <$strategy as UArith<$bits>>::u_mul(a, b);
                    let _ = <$strategy as UArith<$bits>>::u_div(a, b);

                    let _ = <$strategy as UArith<$bits>>::u_mul_fixed::<0>(a, b);
                    let _ = <$strategy as UArith<$bits>>::u_mul_fixed::<1>(a, b);
                    let _ = <$strategy as UArith<$bits>>::u_mul_fixed::<4>(a, b);
                    let _ = <$strategy as UArith<$bits>>::u_div_fixed::<0>(a, b);
                    let _ = <$strategy as UArith<$bits>>::u_div_fixed::<1>(a, b);
                    let _ = <$strategy as UArith<$bits>>::u_div_fixed::<4>(a, b);
                }
            }
        }
    };
}

// Every width, because the width is what selects the implementation family.
// The previous version ran 128 bits alone, which dispatches solely through the
// 256-bit widening macros, so the plain wrapping and native-widening families
// were never entered and four of the six division fixes sat in code the test
// could not reach.
//
// `Warm` and `Precise` reach 64 logical bits; `Hot` and `Cold` reach 128.

macro_rules! across_widths {
    ($prefix:ident, $strategy:ty, wide) => {
        across_widths!($prefix, $strategy, narrow);
        signed_never_panics!(${concat($prefix, _signed_128)}, $strategy, 128);
        unsigned_never_panics!(${concat($prefix, _unsigned_128)}, $strategy, 128);
    };
    ($prefix:ident, $strategy:ty, narrow) => {
        signed_never_panics!(${concat($prefix, _signed_8)}, $strategy, 8);
        signed_never_panics!(${concat($prefix, _signed_16)}, $strategy, 16);
        signed_never_panics!(${concat($prefix, _signed_32)}, $strategy, 32);
        signed_never_panics!(${concat($prefix, _signed_64)}, $strategy, 64);
        unsigned_never_panics!(${concat($prefix, _unsigned_8)}, $strategy, 8);
        unsigned_never_panics!(${concat($prefix, _unsigned_16)}, $strategy, 16);
        unsigned_never_panics!(${concat($prefix, _unsigned_32)}, $strategy, 32);
        unsigned_never_panics!(${concat($prefix, _unsigned_64)}, $strategy, 64);
    };
}

across_widths!(hot, Hot, wide);
across_widths!(cold, Cold, wide);
across_widths!(warm, Warm, narrow);
across_widths!(precise, Precise, narrow);

/// The exact inputs that broke, named on their own so a failure reads as the
/// regression rather than as one cell of a loop.
#[test]
fn signed_fixed_multiply_at_the_container_minimum() {
    // Multiplying the minimum by raw 1 at FRAC 0 is the identity on raws, so
    // the answer is the minimum itself. Reaching it by negating the magnitude
    // overflows, because the magnitude of the minimum is not representable.
    assert_eq!(
        <Hot as IArith<128>>::i_mul_fixed::<0>(i128::MIN, 1),
        i128::MIN
    );
    assert_eq!(
        <Cold as IArith<128>>::i_mul_fixed::<0>(i128::MIN, 1),
        i128::MIN
    );
}

/// `Precise` owes more than not panicking: it owes the clamped bound.
///
/// Returning a value is what the wrapping strategies promise. A saturating one
/// promises the *right* value at an extreme, and `MIN / -1` is where the two
/// answers differ: mathematically it is `+2^(N-1)`, which saturates upward to
/// the maximum, while `wrapping_div` answers with the minimum.
///
/// This is the assertion the previous round did not have. It replaced a
/// panicking `/` with `wrapping_div` at every site uniformly, which is right
/// for `Hot` and `Cold` and inverts the sign for `Precise`, and no test ran
/// `Precise` at all.
#[test]
fn precise_division_saturates_upward_at_the_container_minimum() {
    type T = <Precise as BitsContainerFor<64, Signed>>::T;
    let min = <T as Bounded>::MIN;
    let zero = <T as Identity<Additive>>::IDENTITY;
    let neg_one = <T as SignedIdentity>::NEG_ONE;

    // Asserted as a relationship rather than against a literal, because the
    // bound `Precise` clamps to is the LOGICAL one and `Bounded::MAX` is the
    // CONTAINER one: at 64 logical bits `Precise` projects to an i128, so the
    // two differ by 64 bits and comparing against the container max would be
    // asserting the wrong number.
    let via_div = <Precise as IArith<64>>::i_div(min, neg_one);
    let via_div_fixed = <Precise as IArith<64>>::i_div_fixed::<0>(min, neg_one);

    assert_eq!(
        via_div_fixed, via_div,
        "i_div_fixed must agree with its sibling i_div on MIN / -1, not invert its sign",
    );
    assert!(
        via_div > zero,
        "MIN / -1 is mathematically +2^(N-1), so a saturating strategy clamps upward",
    );
}

/// The wrapping strategies answer the same input with the wrap, and that is
/// the correct disagreement rather than an inconsistency: the two strategies
/// exist to differ here.
#[test]
fn wrapping_division_wraps_where_precise_saturates() {
    type T = <Hot as BitsContainerFor<64, Signed>>::T;
    let min = <T as Bounded>::MIN;
    let neg_one = <T as SignedIdentity>::NEG_ONE;

    assert_eq!(<Hot as IArith<64>>::i_div(min, neg_one), min);
    assert_eq!(<Cold as IArith<64>>::i_div_fixed::<0>(min, neg_one), min);
}
