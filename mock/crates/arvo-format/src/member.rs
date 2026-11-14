//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A slot the type knows is in its format's range.
//!
//! A `Slot` is an index shared by every format and says nothing about any range.
//! `Member<F>` is a slot of `F`'s range, with a private field and two doors, and
//! neither door reads a value at run time to decide anything. Both refuse a format
//! whose phase does not denote, and neither admits one whose quantum is not
//! constant yet. `constant`
//! refuses a slot outside the range at build time; `adapted` is the applied map,
//! which lands in range by its totality at every mode, policy and admitted shape. No
//! door classifies a slot read at run time, since that would be a runtime test
//! of membership.
//!
//! The guarantee is over this perimeter: the field is not written outside the
//! crate, both doors yield a slot in range, and `slot` hands out a copy.

use core::marker::PhantomData;

use crate::adapt::DeclaredSignature;
use crate::apply::{Dither, Exact, adapt};
use crate::format::Format;
use crate::quantum::is_constant_family;
use crate::slots::{Slot, slot_in_range};

/// A slot of `F`'s range.
///
/// Built by `constant`, refused at build time outside the range, or by
/// `adapted`, which is `adapt` and is in range by its totality. Every value of
/// it that can be observed holds a slot of the range.
pub struct Member<F: Format> {
    slot:   Slot,
    format: PhantomData<F>,
}

impl<F: Format> Clone for Member<F> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<F: Format> Copy for Member<F> {}

/// What a format owes before either door builds a member of it.
///
/// The format's own obligation, so its phase denotes, and a constant quantum:
/// membership over a magnitude-indexed one is a slot and a magnitude, which is
/// not yet admitted.
struct MemberOf<F>(PhantomData<F>);

impl<F: Format> MemberOf<F> {
    // FIXME(arvo_member_over_a_magnitude_indexed_quantum): a member of a floating
    // format is a slot and a magnitude, and the applied map has no magnitude
    // coordinate to land one at, so the member holds a slot alone for now.
    const ADMITTED: () = {
        let () = <F as Format>::ADMITTED;
        assert!(
            is_constant_family::<F::Quantum>().get(),
            "a member over a magnitude-indexed quantum is not yet admitted: membership there is a slot \
             and a magnitude, and the applied map has no magnitude coordinate to land a member at \
             yet. Declare the format over a constant quantum for now"
        );
    };
}

/// What a constant member owes, checked rather than asked for.
struct ConstantMember<F, const B: i128>(PhantomData<F>);

impl<F: Format, const B: i128> ConstantMember<F, B> {
    const ADMITTED: () = assert!(
        slot_in_range::<F::Slots>(Slot::at(B)).get(),
        "a constant member is refused outside the format's slot range: such a slot is not a member \
         of this format. Name a slot from MIN to MAX, or reach a position outside the range through \
         Member::adapted, which places it by the signature's policy"
    );
}

impl<F: Format> Member<F> {
    /// The member at the constant slot `B`.
    ///
    /// Refused at build time when `B` is outside the range or where the format's
    /// phase does not denote, and not yet admitted where its quantum is not
    /// constant.
    #[must_use]
    #[inline]
    pub const fn constant<const B: i128>() -> Self {
        let () = MemberOf::<F>::ADMITTED;
        let () = ConstantMember::<F, B>::ADMITTED;
        Self {
            slot:   Slot::at(B),
            format: PhantomData,
        }
    }

    /// The member `adapt` places `exact` at under `S`.
    ///
    /// The dither is passed through for the one mode that reads it. In range by
    /// the map's totality, so nothing is checked. A slot already on the grid and
    /// in range reaches its member at `Exact::on_grid`, which every mode and policy
    /// places on itself. Refused at build time where the format's phase does not
    /// denote, and not yet admitted where its quantum is not constant.
    #[must_use]
    #[inline]
    pub const fn adapted<S: DeclaredSignature<Format = F>>(exact: Exact, dither: Dither) -> Self {
        let () = MemberOf::<F>::ADMITTED;
        Self {
            slot:   adapt::<S>(exact, dither),
            format: PhantomData,
        }
    }

    /// The slot the member sits at.
    #[must_use]
    #[inline]
    pub const fn slot(self) -> Slot {
        self.slot
    }
}

#[cfg(test)]
mod tests;
