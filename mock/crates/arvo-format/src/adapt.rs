//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The adaptation, and the declared signature it completes.
//!
//! Arithmetic on a format is an exact operation in the ambient domain composed
//! with a named total adaptation back onto the representable set. The adaptation
//! is a first-class object with its own laws, so it is a thing named here rather
//! than a flag hanging off an operation. An operation that fuses one invisibly is
//! an operation and an adaptation point wearing one name.
//!
//! The realisation map is one map with two regions rather than two mechanisms:
//! rounding between grid points, completion outside the range. A magnitude bound
//! switches off the completion and not the rounding, and a grid bound does the
//! reverse, which is what two regions of one map predict.

use crate::format::Format;
use crate::overflow::{Overflow, Policy};
use crate::rounding::{Mode, Rounding};

/// A total map from the ambient domain onto a representable set.
///
/// Two independent coordinates, because they answer different questions and
/// because switching one off does not switch the other off.
pub trait Adaptation {
    /// What happens between grid points.
    type Rounding: Rounding;

    /// What happens outside the range.
    type Overflow: Overflow;
}

/// An adaptation named by its two coordinates.
///
/// The only implementor this crate ships, because an adaptation is its
/// coordinates and there is nothing else for a member to vary.
pub struct Adapt<R: Rounding, O: Overflow>(core::marker::PhantomData<(R, O)>);

impl<R: Rounding, O: Overflow> Adaptation for Adapt<R, O> {
    type Rounding = R;
    type Overflow = O;
}

/// The rounding mode an adaptation selects.
#[must_use]
pub const fn rounding_of<A: Adaptation>() -> Mode {
    <A::Rounding as Rounding>::MODE
}

/// The overflow policy an adaptation selects.
#[must_use]
pub const fn overflow_of<A: Adaptation>() -> Policy {
    <A::Overflow as Overflow>::POLICY
}

/// A format paired with the adaptation onto it.
///
/// This is what behaviour is stated over, and it is the whole of what an operation
/// may read. There is no carrier here and none can be reached from here, because
/// this crate does not depend on the crate that has one.
pub trait DeclaredSignature {
    /// The representable set and its domain.
    type Format: Format;

    /// The total map back onto that set.
    type Adaptation: Adaptation;
}

/// A declared signature named by its two halves.
pub struct Signature<F: Format, A: Adaptation>(core::marker::PhantomData<(F, A)>);

impl<F: Format, A: Adaptation> DeclaredSignature for Signature<F, A> {
    type Format = F;
    type Adaptation = A;
}

/// How many operands an operation takes.
///
/// A count, and the only coordinate an operation carries beside the signature it
/// is a function of. It is here rather than with the other counts because it
/// belongs to the contract that reads it.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Arity(u32);

impl Arity {
    /// One operand.
    pub const UNARY: Self = Self(1);

    /// Two operands.
    pub const BINARY: Self = Self(2);

    /// An arity from a count of operands.
    #[must_use]
    pub const fn of(operands: u32) -> Self {
        Self(operands)
    }

    /// The count, for the one place a host contract needs it back.
    ///
    /// The unwrap door, declared as one.
    #[must_use]
    pub const fn count(self) -> u32 {
        self.0
    }
}

/// An operation admitted by the admission rule.
///
/// The rule is that an operation is admitted exactly when it is a function of the
/// declared signature. This trait is that rule expressed so the compiler holds it:
/// an implementor names a signature and nothing else, so an operation that wanted
/// to read a carrier would have nowhere to put it.
///
/// Where two realisations of one name disagree, the signature is missing a
/// coordinate. That is the second half of the rule and it is a diagnostic rather
/// than a failure: the repair is to declare the coordinate, not to refuse the
/// operation.
pub trait Operation {
    /// The signature this operation is a function of.
    type Signature: DeclaredSignature;

    /// How many operands it takes.
    const ARITY: Arity;
}

/// The rounding mode an operation's signature selects, reached through the
/// signature rather than around it.
#[must_use]
pub const fn operation_rounding<Op: Operation>() -> Mode {
    rounding_of::<<Op::Signature as DeclaredSignature>::Adaptation>()
}

/// The overflow policy an operation's signature selects.
#[must_use]
pub const fn operation_overflow<Op: Operation>() -> Policy {
    overflow_of::<<Op::Signature as DeclaredSignature>::Adaptation>()
}
