//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The member door's obligation: what a divisor taken from a member of a second
//! format owes, and the four verdicts it is stated as.
//!
//! The door itself, `Divisor::member`, sits with the other two constructors and
//! forces the obligation here.

use core::cmp::Ordering;
use core::marker::PhantomData;

use notko::Maybe;

use crate::format::Format;
use crate::placing::{powers, reduced_phase, times};
use crate::quantum::is_constant_family;
use crate::slots::{Slot, Slots, slot_in_range};
use crate::width::Bool;

/// What a divisor taken from a member of `G` owes: `G` sits on `F`'s grid inside
/// `F`'s range, and no member of `G` names the zero.
///
/// The grid is one constant quantum and one reduced phase, so slot `b` of `G`
/// denotes the value `(phase + b) q` slot `b` of `F` does, whichever the radixes.
/// Inside the range, the divisor is a member of `F` too, so the quotient is exact.
pub(super) struct MemberDivisor<F, G>(PhantomData<(F, G)>);

impl<F: Format, G: Format> MemberDivisor<F, G> {
    pub(super) const ADMITTED: () = {
        let [quantum, phase, inside, zero_free] = member_conditions::<F, G>();
        assert!(
            quantum.get(),
            "Divisor::member is refused from a format whose quantum is not constant or is another \
             value: slot b names (phase + b) q, so another q is another value than the same slot of \
             the divisor format. The radixes may differ where r^e is one value on both sides, as at \
             exponent zero. A pair whose powers leave the four limbs is not formed, and is refused \
             as well"
        );
        assert!(
            phase.get(),
            "Divisor::member is refused from a format of another reduced phase, or one whose phase \
             does not denote: its slot names a value off the divisor format's grid"
        );
        assert!(
            inside.get(),
            "Divisor::member is refused from a range wider than the divisor format's: a divisor past \
             that range takes the quotient past the exact step, which saturates there"
        );
        assert!(
            zero_free.get(),
            "Divisor::member is refused from a range that holds its zero slot: a member read at run \
             time could be that slot. Declare a range that leaves the slot -n out, n / 1 being the \
             reduced phase, or name the divisor with Divisor::constant"
        );
    };
}

/// The member door's four verdicts, in order: one quantum, the reduced phase,
/// `G`'s range inside `F`'s, and no member of `G` naming the zero.
///
/// The one place they are stated, read by the obligation and by the door's
/// suite, which reads each without forcing the const that refuses it.
#[inline]
pub(super) const fn member_conditions<F: Format, G: Format>() -> [Bool; 4] {
    [
        one_quantum::<F, G>(),
        same_phase::<F, G>(),
        range_inside::<F, G>(),
        no_member_names_the_zero::<G>(),
    ]
}

/// Whether both quanta are constant and one value.
///
/// `r^e` is `U / V` with `U = r^max(e, 0)` and `V = r^max(-e, 0)`, so the two are
/// one value where `U_F V_G = U_G V_F`, whichever the radixes. A side whose powers
/// or product leave the four limbs answers false, as a pair not formed.
#[inline]
const fn one_quantum<F: Format, G: Format>() -> Bool {
    if !(is_constant_family::<F::Quantum>().get() && is_constant_family::<G::Quantum>().get()) {
        return Bool::FALSE;
    }
    let (f, g) = (powers::<F>(), powers::<G>());
    Bool::of(matches!(
        (times(f.up, g.down), times(g.up, f.down)),
        (Maybe::Is(left), Maybe::Is(right)) if matches!(left.ordering(right), Ordering::Equal)
    ))
}

/// Whether both phases denote and are one ratio in lowest terms.
#[inline]
const fn same_phase<F: Format, G: Format>() -> Bool {
    Bool::of(matches!(
        (reduced_phase::<F>(), reduced_phase::<G>()),
        (Maybe::Is(f), Maybe::Is(g))
            if f.numerator == g.numerator && f.denominator == g.denominator
    ))
}

/// Whether `G`'s slot range lies inside `F`'s.
#[inline]
const fn range_inside<F: Format, G: Format>() -> Bool {
    let (f_min, f_max) = (
        <F::Slots as Slots>::MIN.index(),
        <F::Slots as Slots>::MAX.index(),
    );
    let (g_min, g_max) = (
        <G::Slots as Slots>::MIN.index(),
        <G::Slots as Slots>::MAX.index(),
    );
    Bool::of(g_min >= f_min && g_max <= f_max)
}

/// Whether no member of `G` names the zero: its phase denotes, and either its
/// reduced denominator is past one or, at a whole phase `n / 1`, the slot `-n` is
/// outside its range.
#[inline]
pub(super) const fn no_member_names_the_zero<G: Format>() -> Bool {
    Bool::of(match reduced_phase::<G>() {
        Maybe::Is(phase) => {
            phase.denominator > 1 || !slot_in_range::<G::Slots>(Slot::at(-phase.numerator)).get()
        },
        Maybe::Isnt => false,
    })
}
