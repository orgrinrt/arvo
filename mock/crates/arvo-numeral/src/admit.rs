//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Which declared signatures a value is held for.
//!
//! The admission is a check over what a signature already declares, not a
//! list of the instances that pass it. A slot family, an overflow policy or a
//! rounding mode from another crate is held exactly when it declares what the
//! carrier arithmetic computes, the same as the ones `arvo-format` ships.

use arvo_format::overflow::Overflow;
use arvo_format::quantum::is_constant_family;
use arvo_format::rounding::{Mode, Rounding};
use arvo_format::{Adaptation, Bool, DeclaredSignature, Format, Policy, Slot, Slots, Width};
use arvo_placement::SoleCarrier;

use crate::map::Held;

type SlotsOf<S> = <<S as DeclaredSignature>::Format as Format>::Slots;
type OverflowOf<S> = <<S as DeclaredSignature>::Adaptation as Adaptation>::Overflow;
type RoundingOf<S> = <<S as DeclaredSignature>::Adaptation as Adaptation>::Rounding;

/// A declared signature a value can be held for.
///
/// Implemented once, over every signature whose slot family has a container at
/// sole occupancy, so nothing outside this crate implements it and every
/// signature that has one is a `Sole`. What the carrier arithmetic needs beyond
/// that is `ADMITTED`, which every door forces.
pub trait Sole: DeclaredSignature<Format: Format<Slots: SoleCarrier<Carrier: Held>>> {
    /// The declared width.
    const WIDTH: Width;

    /// The overflow policy.
    const POLICY: Policy;

    /// Whether the slot range is the signed one of its width.
    const SIGNED: Bool;

    /// What the carrier arithmetic computes, checked rather than asked for.
    ///
    /// The slot range is the whole two's complement or unsigned range of its
    /// width, and the placement's container for it is the one the derivation
    /// picks. The overflow policy is wrapping or saturation, and the rounding
    /// mode reads no dither. The format's quantum is constant and its phase is
    /// zero, which is where the sum of two members is the member at the sum of
    /// their slots. Every door forces it, so no value of another signature is
    /// built.
    const ADMITTED: ();
}

/// Whether a slot range is the whole signed or unsigned range of its width,
/// which is what the carrier arithmetic reduces onto.
const fn whole_range<Sl: Slots>() -> bool {
    let w = Sl::WIDTH.count();
    if w == 0 || w > 64 {
        return false;
    }
    let top = (1i128 << w) - 1;
    let half = 1i128 << (w - 1);
    let (min, max) = (Sl::MIN.index(), Sl::MAX.index());
    (min == 0 && max == top) || (min == -half && max == half - 1)
}

impl<S> Sole for S
where
    S: DeclaredSignature,
    SlotsOf<S>: SoleCarrier,
    <SlotsOf<S> as SoleCarrier>::Carrier: Held,
{
    const ADMITTED: () = {
        let () = <SlotsOf<S> as Slots>::ADMITTED;
        let () = <SlotsOf<S> as SoleCarrier>::ADMITTED;
        assert!(
            whole_range::<SlotsOf<S>>(),
            "a value is held only for a slot range that is the whole signed or unsigned range of its \
             width, which is what the carrier arithmetic reduces onto. This range is a part of one, \
             or offset; compute on it with `arvo_format::add` on slots instead"
        );
        assert!(
            matches!(
                <OverflowOf<S> as Overflow>::POLICY,
                Policy::Wrap | Policy::Saturate
            ),
            "a value completes a result by wrapping or by saturation. This overflow policy clamps to \
             a bound, and a value has no bound to carry, so with none it is saturation under another \
             name: spell it `Saturate`"
        );
        assert!(
            !matches!(<RoundingOf<S> as Rounding>::MODE, Mode::Stochastic),
            "a value rounds by a deterministic mode. This rounding mode reads a dither, and a value \
             has nowhere to carry one"
        );
        assert!(
            <S::Format as Format>::PHASE.is_zero().get()
                && is_constant_family::<<S::Format as Format>::Quantum>().get(),
            "a value is held only for a format whose sum of two members is on its grid: a zero phase \
             and a constant quantum. This format has a phase or an indexed quantum, so its sums are \
             not slot sums; compute them with `arvo_format::add` on slots instead"
        );
    };
    const POLICY: Policy = <OverflowOf<S> as Overflow>::POLICY;
    const SIGNED: Bool = Bool::of(<SlotsOf<S> as Slots>::MIN.index() < Slot::ZERO.index());
    const WIDTH: Width = <SlotsOf<S> as Slots>::WIDTH;
}
