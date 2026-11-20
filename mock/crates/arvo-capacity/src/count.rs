//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! A capacity's count, read in a constant.

use arvo_format::{Signature, points};
use arvo_numeral::{DebugCheck, NamesAdaptation, Sole, USize};

use crate::capacity::Capacity;
use crate::capacity::seal::Sealed;

/// The count of the capacity `C`, as a `USize` under the consumer's adaptation.
///
/// It is the platform-sized unsigned integer under the adaptation `A` names and
/// the check `K`.
///
/// Reading it is a bit copy of a value every pointer width holds, so nothing
/// is adapted here; the adaptation is the consumer's, for whatever arithmetic
/// the count goes into next.
pub const fn count<C, A, K>() -> USize<A, K>
where
    C: Capacity,
    A: NamesAdaptation<points::USize>,
    K: DebugCheck,
    Signature<points::USize, <A as NamesAdaptation<points::USize>>::Adaptation>: Sole,
{
    USize::<A, K>::from_host(<C as Sealed>::LEN.get().host())
}
