//! The identity laws, asserted over the whole matrix of shapes.
//!
//! `x + ZERO == x` and `x * ONE == x` are what an identity element is for.
//! Before this file the suite had 401 passing tests and asserted neither,
//! anywhere, and four constants were wrong: `UFixed<0, 8, Hot>::ONE` held
//! raw 0 so multiplying by it annihilated, `IFixed<0, 7, Hot>::ONE` held
//! the container minimum so it flipped sign, and the two `Precise`
//! counterparts held one ulp below one so every multiply shrank.
//!
//! The matrix is the point, not the assertions. Each of those is a line
//! anyone could have written; what was missing was writing it over every
//! shape rather than over the widths where it already held. A law asserted
//! only where it holds reports green and says nothing about the rest, and
//! that is precisely how four wrong constants survived a green suite.
//!
//! So this walks both signednesses, all four strategies, and integer and
//! fractional splits from zero bits upward, filtered only by what each
//! strategy's container projection actually reaches. Shapes with no
//! integer bit have no multiplicative identity at all; that refusal is a
//! compile error and cannot be written here, so it is pinned separately in
//! `tests/ui/no_multiplicative_identity.rs`.

#![no_std]

use arvo::ifixed::IFixed;
use arvo::strategy::{
    Additive, Cold, Hot, Identity, Multiplicative, Precise, SignedIdentity, Warm,
};
use arvo::ufixed::UFixed;
use arvo::{fbits, ibits};

/// Probe raws for an unsigned shape: zero, one, the identity, and a value with
/// bits set on both sides of the fractional point.
macro_rules! unsigned_probes {
    ($ty:ty, $one:ident, $i:expr) => {
        // Zero, one, the identity, a value straddling the fractional point,
        // and both logical extremes. `UFixed<I, F, S>` spans `[0, 2^I)`, so
        // the logical maximum raw is `(1 << (I + F)) - 1`.
        //
        // Written with `wrapping_sub` rather than `- 1` because the shift
        // reaches zero exactly when `I + F` equals the container width, which
        // is every `Hot` and `Cold` shape at a bucket boundary. Wrapping from
        // zero gives the all-ones pattern, which IS the logical maximum there.
        // Writing `- 1` fails to compile at those shapes, and dropping the
        // probe because it does not compile is choosing not to test the
        // extreme rather than expressing it.
        [0, 1, $one, $one | 1, ($one << $i).wrapping_sub(1)]
    };
}

/// The same, plus the mirror of each.
///
/// Every probe in this file was non-negative until a review pointed out that
/// neither law was asserted on a negative operand anywhere, which meant the
/// negative branch of the 256-bit multiply, the branch `wrapping_neg` was added
/// for, was never entered by the law matrix at all.
///
/// The LOGICAL minimum is here and the CONTAINER minimum is not, and the
/// difference is the point rather than a convenience.
///
/// The container minimum is inside the logical range only when the container is
/// exactly the logical width, true for `Hot` and `Cold` and false for `Warm`
/// and `Precise`. Feeding it to `Precise` asks the law to hold on a raw that is
/// not a value of the type, and `Precise` would rightly clamp it. That is a
/// reason to skip the container minimum; it is not a reason to skip the logical
/// one, which IS a value of the type at every strategy and is the extreme where
/// a sign-magnitude path has no symmetric answer.
///
/// It is reached by shifting `NEG_ONE`'s raw left by `I`, giving
/// `-(1 << (I + F))` without ever forming the positive intermediate, which is
/// exactly the encoding that does not fit and blocked writing it directly.
macro_rules! signed_probes {
    ($ty:ty, $one:ident, $i:expr) => {
        [
            0,
            1,
            $one,
            $one | 1,
            -$one,
            -($one | 1),
            // The logical minimum, `-(1 << (I + F))`. Reached by shifting
            // `NEG_ONE`'s raw left by `I` so the positive intermediate is
            // never formed: at `I + F` equal to the container width, that
            // intermediate is unrepresentable and the direct spelling does
            // not compile.
            <$ty as SignedIdentity>::NEG_ONE.to_raw() << $i,
            // The logical maximum, `(1 << (I + F)) - 1`, as the bitwise
            // complement of the minimum. In two's complement `!(-2^k)` is
            // `2^k - 1` exactly, so this needs no intermediate either.
            !(<$ty as SignedIdentity>::NEG_ONE.to_raw() << $i),
        ]
    };
}

/// Both laws for a shape that has both identities (at least one integer bit).
///
/// The container type is never named. It is not a function of `I` and `F`
/// alone: each strategy has its own projection, so `UFixed<0, 8, Warm>`
/// and `UFixed<0, 8, Hot>` do not agree on it. Every literal below infers
/// to whatever the projection picked, which keeps the matrix honest about
/// a detail it is not testing.
macro_rules! both_identities {
    ($name:ident, $ty:ty, $i:expr, $f:expr) => {
        both_identities!($name, $ty, $i, $f, unsigned_probes);
    };
    ($name:ident, $ty:ty, $i:expr, $f:expr, $probes:ident) => {
        #[test]
        fn $name() {
            type T = $ty;

            let zero = <T as Identity<Additive>>::IDENTITY;
            let one = <T as Identity<Multiplicative>>::IDENTITY;

            // The constants themselves. `ONE` is raw `1 << F`; getting this
            // wrong is the whole defect, and every law below inherits it.
            assert_eq!(zero.to_raw(), 0, "additive identity is raw zero");
            assert_eq!(
                one.to_raw(),
                1 << $f,
                "multiplicative identity is raw 1 << F"
            );

            // The laws, over the probe set the shape's signedness selects.
            // See `unsigned_probes!` / `signed_probes!` for what each covers
            // and, for the signed one, which extreme it deliberately omits.
            let one_raw = one.to_raw();
            let probes = $probes!(T, one_raw, $i);
            for raw in probes {
                let x = T::from_raw(raw);
                // Both sides. An identity element is one from either
                // direction, and asserting only `x op e` would pass an
                // implementation that had got one side wrong.
                assert_eq!((x + zero).to_raw(), raw, "x + ZERO == x at raw {}", raw);
                assert_eq!((zero + x).to_raw(), raw, "ZERO + x == x at raw {}", raw);
                assert_eq!((x * one).to_raw(), raw, "x * ONE == x at raw {}", raw);
                assert_eq!((one * x).to_raw(), raw, "ONE * x == x at raw {}", raw);
            }
        }
    };
}

/// `NEG_ONE` on a signed shape, at every width including zero integer bits.
///
/// Its absence is what let four artifacts describe an impl that did not
/// exist: the trait's doc comment, the facade's design document, the locked
/// changelist justifying the dropped supertrait, and a comment in this very
/// file. Every one of them reasoned correctly about an asymmetry nothing
/// implemented, and none of them was a test.
///
/// Unconditional, unlike the multiplicative identity. `IFixed<I, F, S>` spans
/// `[-2^I, 2^I)`: minus one is inside at every `I`, and at `I == 0` it is
/// exactly the container minimum, while one is outside there.
macro_rules! neg_one_law {
    ($name:ident, $ty:ty, $f:expr) => {
        #[test]
        fn $name() {
            type T = $ty;

            let zero = <T as Identity<Additive>>::IDENTITY;
            let neg_one = <T as SignedIdentity>::NEG_ONE;

            // Asserted as laws rather than against `-(1 << F)`, and not for
            // convenience. At `I == 0` that literal does not compile: the
            // container is `1 + F` bits, so `+(1 << F)` is not representable
            // there and the negation overflows, while `-(1 << F)` is exactly
            // the container minimum and perfectly fine as a value. The
            // expected value has no expressible form precisely where the
            // asymmetry this trait exists for lives, so writing it down is
            // not available and the laws are what pin it.
            // Magnitude, not merely sign. `< zero` passes for raw -1, which
            // is one ulp rather than minus one, so it pinned almost nothing.
            //
            // The direct spelling `== -(1 << F)` does not compile at zero
            // integer bits: the positive intermediate is unrepresentable in a
            // `1 + F` bit container. Shifting right by `F - 1` lands on -2 for
            // the correct value and on -1 for a wrong one, which distinguishes
            // them without ever forming that intermediate.
            assert_eq!(
                neg_one.to_raw() >> ($f - 1),
                -2,
                "NEG_ONE is raw -(1 << F), not one ulp below zero",
            );
            assert!(neg_one.to_raw() < zero.to_raw(), "NEG_ONE is below zero");
            assert_eq!(
                (neg_one + zero).to_raw(),
                neg_one.to_raw(),
                "NEG_ONE + ZERO == NEG_ONE, the additive law on a negative operand",
            );
        }
    };
}

/// `ONE + NEG_ONE == ZERO`, where the shape has both.
///
/// This is the law that ties the two traits together, and it is the reason
/// they can be separate without drifting apart: whatever `NEG_ONE` is, adding
/// it to the multiplicative identity must land on the additive one.
macro_rules! neg_one_cancels_one {
    ($name:ident, $ty:ty) => {
        #[test]
        fn $name() {
            type T = $ty;

            let zero = <T as Identity<Additive>>::IDENTITY;
            let one = <T as Identity<Multiplicative>>::IDENTITY;
            let neg_one = <T as SignedIdentity>::NEG_ONE;

            assert_eq!(
                (one + neg_one).to_raw(),
                zero.to_raw(),
                "ONE + NEG_ONE == ZERO"
            );
            // Where `ONE` exists the encoding IS expressible, so pin it: the
            // two are mirrors, and `-(one.to_raw())` cannot overflow when the
            // type has an integer bit to hold the positive side.
            assert_eq!(neg_one.to_raw(), -(one.to_raw()), "NEG_ONE is -ONE");
        }
    };
}

/// The additive law alone, for a shape with no integer bit.
///
/// `UFixed<0, F, S>` spans `[0, 1)` and `IFixed<0, F, S>` spans `[-1, 1)`.
/// Neither contains one, so neither has a multiplicative identity and
/// naming one does not compile. Zero is representable at every width, so
/// the additive identity is unaffected, and asserting it here is what stops
/// a future "fix" from removing the whole trait at these widths, as the
/// first attempt at this one did.
macro_rules! additive_only {
    ($name:ident, $ty:ty, $f:expr) => {
        #[test]
        fn $name() {
            type T = $ty;

            let zero = <T as Identity<Additive>>::IDENTITY;
            assert_eq!(zero.to_raw(), 0, "additive identity is raw zero");

            let probes = [0, 1, 1 << ($f - 1)];
            for raw in probes {
                let x = T::from_raw(raw);
                assert_eq!((x + zero).to_raw(), raw, "x + ZERO == x at raw {}", raw);
            }
        }
    };
}

// --- Unsigned. Container width is I + F. ------------------------------------
//
// Warm and Precise reach 64 logical bits; Hot and Cold reach 128. The rows
// below run on all four; the wide band that only Hot and Cold reach is a
// separate block, because 65..=128 is a different implementation family (the
// one holding the 256-bit widening multiply) and skipping it to keep the
// matrix square would be trading coverage for symmetry.

macro_rules! u_matrix {
    ($($strategy:ident => $prefix:ident),+ $(,)?) => { $(
        mod $prefix {
            use super::*;

            // No integer bit: [0, 1), so no one to be an identity. Additive only.
            additive_only!(u_0_1, UFixed<{ ibits(0) }, { fbits(1) }, $strategy>, 1);
            additive_only!(u_0_7, UFixed<{ ibits(0) }, { fbits(7) }, $strategy>, 7);
            additive_only!(u_0_8, UFixed<{ ibits(0) }, { fbits(8) }, $strategy>, 8);
            additive_only!(u_0_16, UFixed<{ ibits(0) }, { fbits(16) }, $strategy>, 16);
            additive_only!(u_0_31, UFixed<{ ibits(0) }, { fbits(31) }, $strategy>, 31);

            // At least one integer bit, across every container bucket boundary.
            both_identities!(u_1_0, UFixed<{ ibits(1) }, { fbits(0) }, $strategy>, 1, 0);
            both_identities!(u_1_1, UFixed<{ ibits(1) }, { fbits(1) }, $strategy>, 1, 1);
            both_identities!(u_1_7, UFixed<{ ibits(1) }, { fbits(7) }, $strategy>, 1, 7);
            both_identities!(u_1_15, UFixed<{ ibits(1) }, { fbits(15) }, $strategy>, 1, 15);
            both_identities!(u_1_31, UFixed<{ ibits(1) }, { fbits(31) }, $strategy>, 1, 31);
            both_identities!(u_2_6, UFixed<{ ibits(2) }, { fbits(6) }, $strategy>, 2, 6);
            both_identities!(u_3_5, UFixed<{ ibits(3) }, { fbits(5) }, $strategy>, 3, 5);
            both_identities!(u_8_8, UFixed<{ ibits(8) }, { fbits(8) }, $strategy>, 8, 8);
            both_identities!(u_13_3, UFixed<{ ibits(13) }, { fbits(3) }, $strategy>, 13, 3);
            both_identities!(u_16_16, UFixed<{ ibits(16) }, { fbits(16) }, $strategy>, 16, 16);
            both_identities!(u_23_9, UFixed<{ ibits(23) }, { fbits(9) }, $strategy>, 23, 9);
            both_identities!(u_32_32, UFixed<{ ibits(32) }, { fbits(32) }, $strategy>, 32, 32);
            both_identities!(u_47_17, UFixed<{ ibits(47) }, { fbits(17) }, $strategy>, 47, 17);
        }
    )+ };
}

u_matrix!(Hot => u_hot, Warm => u_warm, Cold => u_cold, Precise => u_precise);

// --- Signed. Container width is 1 + I + F. ----------------------------------

macro_rules! i_matrix {
    ($($strategy:ident => $prefix:ident),+ $(,)?) => { $(
        mod $prefix {
            use super::*;

            // No integer bit: [-1, 1). Minus one IS representable here, as the
            // container minimum, which is why `SignedIdentity` carries no such
            // condition. One is not, so the multiplicative identity is absent.
            additive_only!(i_0_1, IFixed<{ ibits(0) }, { fbits(1) }, $strategy>, 1);
            additive_only!(i_0_7, IFixed<{ ibits(0) }, { fbits(7) }, $strategy>, 7);
            additive_only!(i_0_15, IFixed<{ ibits(0) }, { fbits(15) }, $strategy>, 15);
            additive_only!(i_0_16, IFixed<{ ibits(0) }, { fbits(16) }, $strategy>, 16);
            additive_only!(i_0_31, IFixed<{ ibits(0) }, { fbits(31) }, $strategy>, 31);

            // At least one integer bit, across every container bucket boundary.
            both_identities!(i_1_0, IFixed<{ ibits(1) }, { fbits(0) }, $strategy>, 1, 0, signed_probes);
            both_identities!(i_1_1, IFixed<{ ibits(1) }, { fbits(1) }, $strategy>, 1, 1, signed_probes);
            both_identities!(i_1_6, IFixed<{ ibits(1) }, { fbits(6) }, $strategy>, 1, 6, signed_probes);
            both_identities!(i_1_14, IFixed<{ ibits(1) }, { fbits(14) }, $strategy>, 1, 14, signed_probes);
            both_identities!(i_1_30, IFixed<{ ibits(1) }, { fbits(30) }, $strategy>, 1, 30, signed_probes);
            both_identities!(i_2_5, IFixed<{ ibits(2) }, { fbits(5) }, $strategy>, 2, 5, signed_probes);
            both_identities!(i_3_4, IFixed<{ ibits(3) }, { fbits(4) }, $strategy>, 3, 4, signed_probes);
            both_identities!(i_7_8, IFixed<{ ibits(7) }, { fbits(8) }, $strategy>, 7, 8, signed_probes);
            both_identities!(i_13_2, IFixed<{ ibits(13) }, { fbits(2) }, $strategy>, 13, 2, signed_probes);
            both_identities!(i_15_16, IFixed<{ ibits(15) }, { fbits(16) }, $strategy>, 15, 16, signed_probes);
            both_identities!(i_23_8, IFixed<{ ibits(23) }, { fbits(8) }, $strategy>, 23, 8, signed_probes);
            both_identities!(i_31_32, IFixed<{ ibits(31) }, { fbits(32) }, $strategy>, 31, 32, signed_probes);
            both_identities!(i_47_16, IFixed<{ ibits(47) }, { fbits(16) }, $strategy>, 47, 16, signed_probes);

            // NEG_ONE at every width, including the zero-integer-bit shapes
            // where it is the container minimum and where `ONE` does not exist.
            neg_one_law!(neg_one_0_1, IFixed<{ ibits(0) }, { fbits(1) }, $strategy>, 1);
            neg_one_law!(neg_one_0_7, IFixed<{ ibits(0) }, { fbits(7) }, $strategy>, 7);
            neg_one_law!(neg_one_0_15, IFixed<{ ibits(0) }, { fbits(15) }, $strategy>, 15);
            neg_one_law!(neg_one_0_31, IFixed<{ ibits(0) }, { fbits(31) }, $strategy>, 31);
            neg_one_law!(neg_one_1_6, IFixed<{ ibits(1) }, { fbits(6) }, $strategy>, 6);
            neg_one_law!(neg_one_7_8, IFixed<{ ibits(7) }, { fbits(8) }, $strategy>, 8);
            neg_one_law!(neg_one_15_16, IFixed<{ ibits(15) }, { fbits(16) }, $strategy>, 16);
            neg_one_law!(neg_one_31_32, IFixed<{ ibits(31) }, { fbits(32) }, $strategy>, 32);

            neg_one_cancels_one!(cancels_1_6, IFixed<{ ibits(1) }, { fbits(6) }, $strategy>);
            neg_one_cancels_one!(cancels_7_8, IFixed<{ ibits(7) }, { fbits(8) }, $strategy>);
            neg_one_cancels_one!(cancels_15_16, IFixed<{ ibits(15) }, { fbits(16) }, $strategy>);
            neg_one_cancels_one!(cancels_31_32, IFixed<{ ibits(31) }, { fbits(32) }, $strategy>);
        }
    )+ };
}

i_matrix!(Hot => i_hot, Warm => i_warm, Cold => i_cold, Precise => i_precise);

// --- The wide band: 65..=128 logical bits, which only Hot and Cold reach. ---
//
// A separate implementation family, the one carrying the 256-bit widening
// multiply that this arc patched. The square matrix above stops at 64 because
// Warm and Precise stop there; stopping the whole file at 64 for symmetry
// would have left the patched family untested.

macro_rules! wide_matrix {
    ($($strategy:ident => $prefix:ident),+ $(,)?) => { $(
        mod $prefix {
            use super::*;

            additive_only!(u_0_100, UFixed<{ ibits(0) }, { fbits(100) }, $strategy>, 100);
            both_identities!(u_1_99, UFixed<{ ibits(1) }, { fbits(99) }, $strategy>, 1, 99);
            both_identities!(u_65_35, UFixed<{ ibits(65) }, { fbits(35) }, $strategy>, 65, 35);
            both_identities!(u_100_28, UFixed<{ ibits(100) }, { fbits(28) }, $strategy>, 100, 28);

            additive_only!(i_0_100, IFixed<{ ibits(0) }, { fbits(100) }, $strategy>, 100);
            both_identities!(i_1_98, IFixed<{ ibits(1) }, { fbits(98) }, $strategy>, 1, 98, signed_probes);
            both_identities!(i_65_34, IFixed<{ ibits(65) }, { fbits(34) }, $strategy>, 65, 34, signed_probes);
            both_identities!(i_100_27, IFixed<{ ibits(100) }, { fbits(27) }, $strategy>, 100, 27, signed_probes);
        }
    )+ };
}

wide_matrix!(Hot => wide_hot, Cold => wide_cold);
