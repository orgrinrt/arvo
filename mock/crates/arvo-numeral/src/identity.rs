//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Identities, computed from the format's own predicate.

use arvo_format::has_additive_identity;

use crate::admit::Sole;
use crate::check::DebugCheck;
use crate::num::Num;

/// A type with an identity element for the operation `M`.
///
/// The call shape is `<T as Identity<M>>::IDENTITY`.
pub trait Identity<M> {
    /// The identity element.
    const IDENTITY: Self;
}

/// Addition, as the operation an identity is named for.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Additive;

impl<S: Sole, C: DebugCheck> Identity<Additive> for Num<S, C> {
    /// Zero, which at a zero phase is slot zero.
    ///
    /// The format's own predicate is asked first, so a format without an
    /// additive identity is refused here as a failed constant naming that,
    /// rather than as a missing impl.
    const IDENTITY: Self = {
        assert!(
            has_additive_identity::<S::Format>().get(),
            "the format has no additive identity: no member adds to every member without moving \
             it, so `Identity<Additive>` has no element to give. A format with a phase has none"
        );
        Num::constant::<0>()
    };
}
