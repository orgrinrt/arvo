//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The quotient and the conversion onto another signature.
//!
//! Both place a member at a ratio and round it. The position is the format's
//! step over members, `Member::quotient` and `Member::converted`, which forms it
//! in the host's integer where the range leaves room and by the slot step
//! elsewhere, and the rounding is the format's applied map, `adapt`.

use arvo_format::overflow::Saturate;
use arvo_format::{
    Adaptation,
    Bool,
    Dither,
    Divisor,
    Exact,
    Policy,
    WithOverflow,
    WithRounding,
    adapt,
    convert as convert_slots,
    panic_on_overflow,
};

use crate::admit::Sole;
use crate::check::DebugCheck;
use crate::num::Num;

impl<S: Sole, C: DebugCheck> Num<S, C> {
    /// The quotient under the signature: the slot `arvo_format::div` gives for
    /// the slot and the divisor.
    ///
    /// Total: the divisor is not zero by its type, a quotient between two
    /// members is rounded by the declared mode, and one that leaves the range
    /// is completed by the declared overflow policy. Under the `Abort` check a
    /// debug build aborts on a quotient that leaves the range, which `fits_div`
    /// asks ahead; under `Quiet` no build does.
    ///
    /// Refused at build where the format's quotient is not admitted, with the
    /// format's own message.
    #[must_use]
    pub const fn div(self, by: Divisor<S::Format>) -> Self {
        if cfg!(debug_assertions) && const { C::ABORTS.get() } {
            assert!(
                self.fits_div(by).get(),
                "the quotient leaves the signature's range, and the `Abort` check aborts on that in a \
                 debug build. Name the `Quiet` check where the completion the overflow policy gives \
                 is what you mean"
            );
        }
        if cfg!(arvo_mutant_div) && !self.fits_div(by).get() {
            return self;
        }
        Self::at_index(adapt::<S>(self.member().quotient(by), Dither::UNUSED).index())
    }

    /// Whether the quotient needs no completion: the slot the declared mode
    /// rounds its exact position to is a member.
    #[must_use]
    pub const fn fits_div(self, by: Divisor<S::Format>) -> Bool {
        let position = self.member().quotient(by);
        Bool::of(!panic_on_overflow::<S>(position, Dither::UNUSED).get())
    }

    /// The member placed on the signature `T`: the slot `arvo_format::convert`
    /// gives from this format onto `T`.
    ///
    /// Total: a member between two slots of `T` is rounded by `T`'s mode, and
    /// one outside `T`'s range is completed by `T`'s overflow policy. Under the
    /// `Abort` check a debug build aborts on a member that leaves `T`'s range,
    /// which `fits_convert` asks ahead; under `Quiet` no build does. The check
    /// is kept.
    ///
    /// Where `T` is over this signature's format the result is what `under`
    /// gives, and `under` is the door for that, since it computes nothing.
    /// Refused at build where the format's conversion is not admitted for the
    /// pair, with the format's own message.
    #[must_use]
    pub const fn convert<T: Sole>(self) -> Num<T, C> {
        if cfg!(debug_assertions) && const { C::ABORTS.get() } {
            assert!(
                self.fits_convert::<T>().get(),
                "the member leaves the target signature's range, and the `Abort` check aborts on \
                 that in a debug build. Name the `Quiet` check where the completion the target's \
                 overflow policy gives is what you mean"
            );
        }
        if cfg!(arvo_mutant_convert) {
            return Num::at_index(adapt::<T>(Exact::on_grid(self.slot()), Dither::UNUSED).index());
        }
        if cfg!(arvo_mutant_convert_policy) && matches!(S::POLICY, Policy::Saturate) {
            let saturated =
                convert_slots::<S::Format, WithOverflow<T, Saturate>>(self.slot(), Dither::UNUSED);
            return Num::at_index(saturated.index());
        }
        if cfg!(arvo_mutant_convert_mode) {
            let rounded = convert_slots::<
                S::Format,
                WithRounding<T, <S::Adaptation as Adaptation>::Rounding>,
            >(self.slot(), Dither::UNUSED);
            return Num::at_index(rounded.index());
        }
        let position = self.member().converted::<T::Format>();
        Num::at_index(adapt::<T>(position, Dither::UNUSED).index())
    }

    /// Whether the conversion onto `T` needs no completion: the slot `T`'s mode
    /// rounds the member's position to is a member of `T`.
    #[must_use]
    pub const fn fits_convert<T: Sole>(self) -> Bool {
        let position = self.member().converted::<T::Format>();
        Bool::of(!panic_on_overflow::<T>(position, Dither::UNUSED).get())
    }
}
