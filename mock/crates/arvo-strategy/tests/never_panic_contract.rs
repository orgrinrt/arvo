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

use arvo_strategy::{Cold, Hot, IArith, UArith};

/// The signed container extremes, plus their neighbours and zero.
///
/// `MIN` is the one with no positive counterpart: `-MIN` is not
/// representable, so any implementation that reaches a negative result by
/// negating a magnitude has no answer here. Every other value in this list
/// has a symmetric partner and would pass a test that omitted `MIN`.
const I_PROBES: [i128; 7] = [i128::MIN, i128::MIN + 1, -1, 0, 1, i128::MAX - 1, i128::MAX];

const U_PROBES: [u128; 5] = [0, 1, 2, u128::MAX - 1, u128::MAX];

// The fractional-bit count is a const generic, so the rescale's branches are
// instantiated rather than looped: none, inside the low limb, exactly at the
// limb boundary, and above it.

macro_rules! signed_never_panics {
    ($name:ident, $strategy:ty) => {
        #[test]
        fn $name() {
            for a in I_PROBES {
                for b in I_PROBES {
                    // Each of these must return rather than panic. The value
                    // is the operation's business; that there IS one is this
                    // test's business.
                    let _ = <$strategy as IArith<128>>::i_add(a, b);
                    let _ = <$strategy as IArith<128>>::i_sub(a, b);
                    let _ = <$strategy as IArith<128>>::i_mul(a, b);
                    let _ = <$strategy as IArith<128>>::i_div(a, b);

                    // The regression. `i_mul_fixed::<0>(i128::MIN, 1)`
                    // panicked with "attempt to negate with overflow".
                    let _ = <$strategy as IArith<128>>::i_mul_fixed::<0>(a, b);
                    let _ = <$strategy as IArith<128>>::i_mul_fixed::<1>(a, b);
                    let _ = <$strategy as IArith<128>>::i_mul_fixed::<64>(a, b);
                    let _ = <$strategy as IArith<128>>::i_mul_fixed::<127>(a, b);
                    let _ = <$strategy as IArith<128>>::i_div_fixed::<0>(a, b);
                    let _ = <$strategy as IArith<128>>::i_div_fixed::<1>(a, b);
                    let _ = <$strategy as IArith<128>>::i_div_fixed::<64>(a, b);
                }
            }
        }
    };
}

macro_rules! unsigned_never_panics {
    ($name:ident, $strategy:ty) => {
        #[test]
        fn $name() {
            for a in U_PROBES {
                for b in U_PROBES {
                    let _ = <$strategy as UArith<128>>::u_add(a, b);
                    let _ = <$strategy as UArith<128>>::u_sub(a, b);
                    let _ = <$strategy as UArith<128>>::u_mul(a, b);
                    let _ = <$strategy as UArith<128>>::u_div(a, b);

                    let _ = <$strategy as UArith<128>>::u_mul_fixed::<0>(a, b);
                    let _ = <$strategy as UArith<128>>::u_mul_fixed::<1>(a, b);
                    let _ = <$strategy as UArith<128>>::u_mul_fixed::<64>(a, b);
                    let _ = <$strategy as UArith<128>>::u_mul_fixed::<127>(a, b);
                    let _ = <$strategy as UArith<128>>::u_div_fixed::<0>(a, b);
                    let _ = <$strategy as UArith<128>>::u_div_fixed::<1>(a, b);
                    let _ = <$strategy as UArith<128>>::u_div_fixed::<64>(a, b);
                }
            }
        }
    };
}

signed_never_panics!(hot_signed_returns_for_every_input, Hot);
signed_never_panics!(cold_signed_returns_for_every_input, Cold);
unsigned_never_panics!(hot_unsigned_returns_for_every_input, Hot);
unsigned_never_panics!(cold_unsigned_returns_for_every_input, Cold);

/// The exact input that broke, named on its own so a failure reads as the
/// regression rather than as one cell of a loop.
#[test]
fn signed_fixed_multiply_at_the_container_minimum() {
    // Multiplying the minimum by raw 1 at FRAC 0 is the identity on raws, so
    // the answer is the minimum itself. Reaching it by negating the magnitude
    // overflows, because the magnitude of the minimum is not representable.
    assert_eq!(
        <Hot as IArith<128>>::i_mul_fixed::<0>(i128::MIN, 1),
        i128::MIN,
    );
    assert_eq!(
        <Cold as IArith<128>>::i_mul_fixed::<0>(i128::MIN, 1),
        i128::MIN,
    );
}
