//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

//! The declared signature of a number: which values exist, and what happens to a
//! result that is not one of them.
//!
//! A format here is not a container. It is a representable set together with the
//! domain that set sits in, and membership in it is one affine predicate rather
//! than a family of them: a phase, a quantum per magnitude, and a slot range.
//! Integers, fixed point, scaled integers and floating point are points of that
//! one predicate, and subnormals fall out of the smallest magnitude rather than
//! out of a case naming them.
//!
//! There is no machine carrier in this crate and none is reachable from it. Every
//! operation is a function of the declared signature and never of what the value
//! happens to sit in, and the dependency edge is what keeps that true rather than
//! a comment asking for it. Where the bits go is `arvo-placement`.
//!
//! Every derived quantity is an associated item resolved by an impl, and every
//! computation over one is a free `const fn`. That is the staging boundary rather
//! than a restriction worked around: the trait is what is known at
//! monomorphisation, the `const fn` is the computation over it, and what the
//! backend sees has no branch left to erase.
//!
//! This crate introduces the numeric category, so the bare-primitive lints skip
//! it. That is the door for the one place the stack's own primitives cannot be
//! used to define themselves, and `width` is the narrow thing it exists for.

pub mod adapt;
pub mod ambient;
pub mod apply;
pub mod format;
pub mod overflow;
pub mod quantum;
pub mod rounding;
pub mod slots;
pub mod standards;
pub mod width;

pub use adapt::{
    operation_overflow, operation_rounding, overflow_of, rounding_of, Adapt, Adaptation,
    DeclaredSignature, Operation, Signature,
};
pub use ambient::{Ambient, BinaryRationals, DecimalRationals, UnsignedBinaryRationals};
pub use format::{
    contains, has_additive_identity, radix, smallest_step_exponent, step_exponent, Format,
};
pub use overflow::{Overflow, Policy, SHIPPED_POLICIES};
pub use quantum::{exponent_at, is_constant_family, magnitude_in_range, Quantum};
pub use rounding::{Mode, Rounding, ALL_MODES};
pub use slots::{slot_count, slot_in_range, Slots};
pub use width::{Bool, Width};

/// The four points of the parameterisation the canon names, as formats.
///
/// Shipped as worked instances of the open inventory rather than as the inventory
/// itself. A new numeral joins by implementing `Format`, and none of these is
/// privileged by being here.
pub mod points {
    use crate::ambient::{BinaryRationals, UnsignedBinaryRationals};
    use crate::format::Format;
    use crate::quantum::{Constant, Indexed};
    use crate::slots::{Signed, Slots, Unsigned};

    /// Signed integers of `BITS` bits: constant quantum at exponent zero, no phase.
    pub struct Integer<const BITS: u32>;

    impl<const BITS: u32> Format for Integer<BITS>
    where
        Signed<BITS>: Slots,
    {
        type Ambient = BinaryRationals;
        type Quantum = Constant<0>;
        type Slots = Signed<BITS>;
        const PHASE_NUM: i64 = 0;
        const PHASE_DEN: i64 = 1;
    }

    /// Unsigned fixed point of `BITS` bits with the quantum at exponent `FRAC`.
    ///
    /// The constant-quantum family at a negative exponent, which is the whole of
    /// what makes it fixed point rather than integral.
    pub struct UFixed<const BITS: u32, const FRAC: i32>;

    impl<const BITS: u32, const FRAC: i32> Format for UFixed<BITS, FRAC>
    where
        Unsigned<BITS>: Slots,
    {
        type Ambient = UnsignedBinaryRationals;
        type Quantum = Constant<FRAC>;
        type Slots = Unsigned<BITS>;
        const PHASE_NUM: i64 = 0;
        const PHASE_DEN: i64 = 1;
    }

    /// A scaled integer: constant quantum at a declared exponent, with a phase.
    ///
    /// The point that exercises the phase coordinate, which the other three leave
    /// at zero. A nonzero phase takes the additive identity off the grid, and the
    /// law asserting that is what keeps the coordinate honest.
    pub struct Biased<const BITS: u32, const EXP: i32, const PHASE: i64>;

    impl<const BITS: u32, const EXP: i32, const PHASE: i64> Format for Biased<BITS, EXP, PHASE>
    where
        Signed<BITS>: Slots,
    {
        type Ambient = BinaryRationals;
        type Quantum = Constant<EXP>;
        type Slots = Signed<BITS>;
        const PHASE_NUM: i64 = PHASE;
        const PHASE_DEN: i64 = 2;
    }

    /// A floating point: the magnitude-indexed family.
    ///
    /// `MANTISSA` gives the slot range within one magnitude, `MIN_EXP` the
    /// exponent at the smallest magnitude, and `EXPONENTS` how many magnitudes
    /// there are. Nothing here names a subnormal.
    pub struct Floating<const MANTISSA: u32, const MIN_EXP: i32, const EXPONENTS: u32>;

    impl<const MANTISSA: u32, const MIN_EXP: i32, const EXPONENTS: u32> Format
        for Floating<MANTISSA, MIN_EXP, EXPONENTS>
    where
        Signed<MANTISSA>: Slots,
    {
        type Ambient = BinaryRationals;
        type Quantum = Indexed<MIN_EXP, EXPONENTS>;
        type Slots = Signed<MANTISSA>;
        const PHASE_NUM: i64 = 0;
        const PHASE_DEN: i64 = 1;
    }
}

#[cfg(test)]
mod tests;
