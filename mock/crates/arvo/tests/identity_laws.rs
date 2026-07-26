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
use arvo::strategy::{Additive, Cold, Hot, Identity, Multiplicative, Precise, Warm};
use arvo::ufixed::UFixed;
use arvo::{fbits, ibits};

/// Both laws for a shape that has both identities (at least one integer bit).
///
/// The container type is never named. It is not a function of `I` and `F`
/// alone: each strategy has its own projection, so `UFixed<0, 8, Warm>`
/// and `UFixed<0, 8, Hot>` do not agree on it. Every literal below infers
/// to whatever the projection picked, which keeps the matrix honest about
/// a detail it is not testing.
macro_rules! both_identities {
    ($name:ident, $ty:ty, $f:expr) => {
        #[test]
        fn $name() {
            type T = $ty;

            let zero = <T as Identity<Additive>>::IDENTITY;
            let one = <T as Identity<Multiplicative>>::IDENTITY;

            // The constants themselves. `ONE` is raw `1 << F`; getting this
            // wrong is the whole defect, and every law below inherits it.
            assert_eq!(zero.to_raw(), 0, "additive identity is raw zero");
            assert_eq!(
                one.to_raw() >> $f,
                1,
                "multiplicative identity is raw 1 << F"
            );

            // The laws, over a spread of operands rather than one: raw zero,
            // raw one, the identity itself, and a value with bits set on both
            // sides of the fractional point.
            let probes = [0, 1, one.to_raw(), one.to_raw() | 1];
            for raw in probes {
                let x = T::from_raw(raw);
                assert_eq!((x + zero).to_raw(), raw, "x + ZERO == x at raw {}", raw);
                assert_eq!((x * one).to_raw(), raw, "x * ONE == x at raw {}", raw);
            }
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
// Warm and Precise reach 64 logical bits; Hot and Cold reach 128. Every
// (I, F) pair below is inside all four, so the matrix is square and no
// strategy is quietly under-tested relative to another.

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
            both_identities!(u_1_0, UFixed<{ ibits(1) }, { fbits(0) }, $strategy>, 0);
            both_identities!(u_1_1, UFixed<{ ibits(1) }, { fbits(1) }, $strategy>, 1);
            both_identities!(u_1_7, UFixed<{ ibits(1) }, { fbits(7) }, $strategy>, 7);
            both_identities!(u_1_15, UFixed<{ ibits(1) }, { fbits(15) }, $strategy>, 15);
            both_identities!(u_1_31, UFixed<{ ibits(1) }, { fbits(31) }, $strategy>, 31);
            both_identities!(u_2_6, UFixed<{ ibits(2) }, { fbits(6) }, $strategy>, 6);
            both_identities!(u_3_5, UFixed<{ ibits(3) }, { fbits(5) }, $strategy>, 5);
            both_identities!(u_8_8, UFixed<{ ibits(8) }, { fbits(8) }, $strategy>, 8);
            both_identities!(u_13_3, UFixed<{ ibits(13) }, { fbits(3) }, $strategy>, 3);
            both_identities!(u_16_16, UFixed<{ ibits(16) }, { fbits(16) }, $strategy>, 16);
            both_identities!(u_23_9, UFixed<{ ibits(23) }, { fbits(9) }, $strategy>, 9);
            both_identities!(u_32_32, UFixed<{ ibits(32) }, { fbits(32) }, $strategy>, 32);
            both_identities!(u_47_17, UFixed<{ ibits(47) }, { fbits(17) }, $strategy>, 17);
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
            both_identities!(i_1_0, IFixed<{ ibits(1) }, { fbits(0) }, $strategy>, 0);
            both_identities!(i_1_1, IFixed<{ ibits(1) }, { fbits(1) }, $strategy>, 1);
            both_identities!(i_1_6, IFixed<{ ibits(1) }, { fbits(6) }, $strategy>, 6);
            both_identities!(i_1_14, IFixed<{ ibits(1) }, { fbits(14) }, $strategy>, 14);
            both_identities!(i_1_30, IFixed<{ ibits(1) }, { fbits(30) }, $strategy>, 30);
            both_identities!(i_2_5, IFixed<{ ibits(2) }, { fbits(5) }, $strategy>, 5);
            both_identities!(i_3_4, IFixed<{ ibits(3) }, { fbits(4) }, $strategy>, 4);
            both_identities!(i_7_8, IFixed<{ ibits(7) }, { fbits(8) }, $strategy>, 8);
            both_identities!(i_13_2, IFixed<{ ibits(13) }, { fbits(2) }, $strategy>, 2);
            both_identities!(i_15_16, IFixed<{ ibits(15) }, { fbits(16) }, $strategy>, 16);
            both_identities!(i_23_8, IFixed<{ ibits(23) }, { fbits(8) }, $strategy>, 8);
            both_identities!(i_31_32, IFixed<{ ibits(31) }, { fbits(32) }, $strategy>, 32);
            both_identities!(i_47_16, IFixed<{ ibits(47) }, { fbits(16) }, $strategy>, 16);
        }
    )+ };
}

i_matrix!(Hot => i_hot, Warm => i_warm, Cold => i_cold, Precise => i_precise);
