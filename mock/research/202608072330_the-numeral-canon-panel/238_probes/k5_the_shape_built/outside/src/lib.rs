#![no_std]
//! A format this crate's dependency does not know about, declared with no bare
//! primitive on any line.
//!
//! Every constant is a coordinate type of the door's, every literal is a
//! literal, and nothing here names `u32`, `i32`, `i64` or `bool`. That is the
//! whole claim: an outside implementor can supply the concept's obligations
//! without falling back on the machine's own types.
//!
//! The const items at the bottom are what makes the claim about **const time**
//! rather than about runtime. A `const` item is evaluated during compilation or
//! the build fails, so a membership answer sitting in one is an answer the
//! backend has no branch left to erase.

use door::{
    contains, has_additive_identity, radix, slot_count, Ambient, Arity, Bool, Exponent, Format,
    Magnitude, MagnitudeCount, Operation, Phase, Quantum, Radix, Slot, SlotCount, Slots, Width,
};

/// A domain counting in three, which no crate in the stack ships.
pub struct TernaryRationals;

impl Ambient for TernaryRationals {
    const RADIX: Radix = Radix::of(3);
    const SIGNED: Bool = Bool::TRUE;
}

/// A constant quantum at a negative exponent, which is the fixed-point shape.
pub struct ThreeToTheMinusFour;

impl Quantum for ThreeToTheMinusFour {
    const BASE: Exponent = Exponent::of(-4);
    const SLOPE: Exponent = Exponent::of(0);
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(1);
}

/// A signed range of eight bits.
pub struct EightBitSigned;

impl Slots for EightBitSigned {
    const MIN: Slot = Slot::at(-128);
    const MAX: Slot = Slot::at(127);
    const WIDTH: Width = Width::bits(8);
}

/// The format, biased half a step so the phase coordinate is exercised.
pub struct BiasedTernary;

impl Format for BiasedTernary {
    type Ambient = TernaryRationals;
    type Quantum = ThreeToTheMinusFour;
    type Slots = EightBitSigned;
    const PHASE: Phase = Phase::of(1, 2);
}

/// A binary operation over it.
pub struct Add;

impl Operation for Add {
    type Format = BiasedTernary;
    const ARITY: Arity = Arity::of(2);
}

// --- what the compiler has to answer before this crate exists ----------------

/// A slot inside the range is a member.
pub const INSIDE: Bool = contains::<BiasedTernary>(Slot::at(0), Magnitude::at(0));
/// A slot outside it is not.
pub const OUTSIDE: Bool = contains::<BiasedTernary>(Slot::at(200), Magnitude::at(0));
/// A magnitude the constant law does not range over is not.
pub const NO_SUCH_MAGNITUDE: Bool = contains::<BiasedTernary>(Slot::at(0), Magnitude::at(1));
/// The half-step bias takes the additive identity off the grid.
pub const IDENTITY: Bool = has_additive_identity::<BiasedTernary>();
/// The domain's radix, read through the format.
pub const RADIX: Radix = radix::<BiasedTernary>();
/// The cardinality of the slot range.
pub const SLOTS: SlotCount = slot_count::<EightBitSigned>();
/// The operation's arity.
pub const ARITY: Arity = <Add as Operation>::ARITY;

#[cfg(test)]
mod tests {
    use super::*;

    /// The membership answers are the right ones, not merely constant.
    ///
    /// A const item that evaluates proves the staging. It does not prove the
    /// arithmetic survived being retyped, and retyping arithmetic is exactly
    /// where a sign or a width gets dropped. These are the values the shipped
    /// crate's own laws assert, read back through the coordinate types.
    #[test]
    fn the_answers_are_the_ones_the_shipped_laws_assert() {
        assert!(
            INSIDE.get(),
            "slot 0 at magnitude 0 is in an eight-bit signed range"
        );
        assert!(
            !OUTSIDE.get(),
            "slot 200 is past the range's maximum of 127"
        );
        assert!(
            !NO_SUCH_MAGNITUDE.get(),
            "a constant-quantum law ranges over one magnitude, so magnitude 1 is not one"
        );
        assert!(
            !IDENTITY.get(),
            "a half-step phase takes the additive identity off the grid"
        );
        assert_eq!(RADIX.count(), 3);
        assert_eq!(SLOTS.count(), 256);
        assert_eq!(ARITY.count(), 2);
    }

    /// The control the retyping did not simply make everything false.
    ///
    /// Four of the seven assertions above are negative, and a coordinate type
    /// whose accessor returned a fixed value would satisfy all four. So a
    /// second format with the opposite answers, differing from the first only
    /// in phase and in the width of its range.
    #[test]
    fn the_control_a_second_format_answers_the_other_way() {
        pub struct Unbiased;
        impl Format for Unbiased {
            type Ambient = TernaryRationals;
            type Quantum = ThreeToTheMinusFour;
            type Slots = EightBitSigned;
            const PHASE: Phase = Phase::NONE;
        }
        assert!(
            has_additive_identity::<Unbiased>().get(),
            "a zero phase puts the identity on the grid, so the negative above is about the phase"
        );
        assert!(contains::<Unbiased>(Slot::at(127), Magnitude::at(0)).get());
        assert!(!contains::<Unbiased>(Slot::at(128), Magnitude::at(0)).get());
    }
}
