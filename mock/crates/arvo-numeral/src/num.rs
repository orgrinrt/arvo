//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The value, its doors and its operations.

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::mem::size_of;

use arvo_format::{
    Bool,
    DeclaredSignature,
    Dither,
    Exact,
    Format,
    Slot,
    adapt,
    panic_on_overflow,
    slot_in_range,
};
use arvo_placement::SoleCarrier;

use crate::admit::Sole;
use crate::check::{Abort, DebugCheck};
use crate::machine::copy;
use crate::map::Held;
use crate::rung;

type SlotsOf<S> = <<S as DeclaredSignature>::Format as Format>::Slots;

/// The machine integer of the container the placement names for the signature's
/// slot family. Nameable inside this crate only.
pub(crate) type ReprOf<S> = <<SlotsOf<S> as SoleCarrier>::Carrier as Held>::Repr;

/// A value of the declared signature `S`: one member of its format, held alone
/// in the carrier the placement derives at sole occupancy.
///
/// The carrier is canonical, one member to one bit pattern, so two values are
/// equal exactly when their slots are. Every operation is the format's own on
/// the slot, computed in the carrier.
///
/// `C` is the debug check, `Abort` unless named: whether a debug build aborts
/// when a result leaves the range. It is no part of the signature, so two values
/// of one signature under different checks hold the same member the same way.
#[repr(transparent)]
pub struct Num<S: Sole, C: DebugCheck = Abort>(ReprOf<S>, PhantomData<C>);

/// One operation, dispatched on the carrier's size and the slot range's sign to
/// the rung function at that machine integer. The match is over constants of
/// the instantiation, so one arm survives monomorphisation.
///
/// `carrier` moves the rung's result back into the carrier; `plain` returns it
/// as it is.
macro_rules! dispatch {
    (
        $s:ty, $out:ident,
        [$s8:path, $u8:path, $s16:path, $u16:path, $s32:path, $u32:path, $s64:path, $u64:path],
        [$($op:expr),*], [$($tail:expr),*]
    ) => {
        match const { (size_of::<ReprOf<$s>>(), <$s as Sole>::SIGNED.get()) } {
            (1, true) => dispatch!(@$out i8, ReprOf<$s>, $s8($(copy::<ReprOf<$s>, i8>($op),)* $($tail),*)),
            (1, false) => dispatch!(@$out u8, ReprOf<$s>, $u8($(copy::<ReprOf<$s>, u8>($op),)* $($tail),*)),
            (2, true) => dispatch!(@$out i16, ReprOf<$s>, $s16($(copy::<ReprOf<$s>, i16>($op),)* $($tail),*)),
            (2, false) => dispatch!(@$out u16, ReprOf<$s>, $u16($(copy::<ReprOf<$s>, u16>($op),)* $($tail),*)),
            (4, true) => dispatch!(@$out i32, ReprOf<$s>, $s32($(copy::<ReprOf<$s>, i32>($op),)* $($tail),*)),
            (4, false) => dispatch!(@$out u32, ReprOf<$s>, $u32($(copy::<ReprOf<$s>, u32>($op),)* $($tail),*)),
            (8, true) => dispatch!(@$out i64, ReprOf<$s>, $s64($(copy::<ReprOf<$s>, i64>($op),)* $($tail),*)),
            (8, false) => dispatch!(@$out u64, ReprOf<$s>, $u64($(copy::<ReprOf<$s>, u64>($op),)* $($tail),*)),
            _ => unreachable!(),
        }
    };
    (@carrier $m:ty, $r:ty, $e:expr) => {
        copy::<$m, $r>($e)
    };
    (@plain $m:ty, $r:ty, $e:expr) => {
        $e
    };
}

impl<S: Sole, C: DebugCheck> Num<S, C> {
    /// The value at a member's slot index. The index is a member's.
    const fn at_index(index: i128) -> Self {
        let () = S::ADMITTED;
        Self::from_carrier(dispatch!(
            S,
            carrier,
            [
                rung::from_s8,
                rung::from_u8,
                rung::from_s16,
                rung::from_u16,
                rung::from_s32,
                rung::from_u32,
                rung::from_s64,
                rung::from_u64
            ],
            [],
            [index]
        ))
    }

    /// The member at slot index `V`, refused at build where `V` is not one.
    #[must_use]
    pub const fn constant<const V: i128>() -> Self {
        const {
            let () = S::ADMITTED;
            assert!(
                slot_in_range::<SlotsOf<S>>(Slot::at(V)).get(),
                "`constant::<V>()` names a slot index outside the signature's range. Pick a member, \
                 or ingest the position with `adapted`, which completes it by the declared policy"
            );
        }
        Self::at_index(V)
    }

    /// An exact position put onto the format by the signature's own adaptation.
    ///
    /// Total: a position whose rounded slot leaves the range is completed by the
    /// declared overflow policy. Under the `Abort` check a debug build aborts on
    /// that instead; under `Quiet` no build does.
    #[must_use]
    pub const fn adapted(exact: Exact) -> Self {
        if cfg!(debug_assertions) && const { C::ABORTS.get() } {
            assert!(
                !panic_on_overflow::<S>(exact, Dither::UNUSED).get(),
                "the position leaves the signature's range, and the `Abort` check aborts on that in a \
                 debug build. Name the `Quiet` check where the completion the overflow policy gives \
                 is what you mean"
            );
        }
        Self::at_index(adapt::<S>(exact, Dither::UNUSED).index())
    }

    /// The member's slot. An unwrap door.
    #[must_use]
    pub const fn slot(self) -> Slot {
        Slot::at(dispatch!(
            S,
            plain,
            [
                rung::index_s8,
                rung::index_u8,
                rung::index_s16,
                rung::index_u16,
                rung::index_s32,
                rung::index_u32,
                rung::index_s64,
                rung::index_u64
            ],
            [self.0],
            []
        ))
    }

    /// The value whose carrier is `carrier`. The host doors build on it, where
    /// every carrier bit pattern is a member.
    pub(crate) const fn from_carrier(carrier: ReprOf<S>) -> Self {
        let () = S::ADMITTED;
        Self(carrier, PhantomData)
    }

    /// The carrier. The host doors read it.
    pub(crate) const fn carrier(self) -> ReprOf<S> {
        self.0
    }

    /// The sum under the signature: the slot `arvo_format::add` gives for the
    /// two slots.
    ///
    /// Total: a sum that leaves the range is completed by the declared overflow
    /// policy. Under the `Abort` check a debug build aborts on that instead,
    /// which `fits_add` asks ahead; under `Quiet` no build does.
    #[must_use]
    pub const fn add(self, rhs: Self) -> Self {
        if cfg!(debug_assertions) && const { C::ABORTS.get() } {
            assert!(
                self.fits_add(rhs).get(),
                "the sum leaves the signature's range, and the `Abort` check aborts on that in a debug \
                 build. Name the `Quiet` check where the completion the overflow policy gives is what \
                 you mean"
            );
        }
        Self::from_carrier(dispatch!(
            S,
            carrier,
            [
                rung::add_s8,
                rung::add_u8,
                rung::add_s16,
                rung::add_u16,
                rung::add_s32,
                rung::add_u32,
                rung::add_s64,
                rung::add_u64
            ],
            [self.0, rhs.0],
            [const { S::WIDTH.count() }, S::POLICY]
        ))
    }

    /// Whether the exact sum of the two members is a member, so `add` needs no
    /// completion.
    #[must_use]
    pub const fn fits_add(self, rhs: Self) -> Bool {
        slot_in_range::<SlotsOf<S>>(Slot::at(self.slot().index() + rhs.slot().index()))
    }

    /// The order of the two members.
    #[must_use]
    pub const fn cmp(self, rhs: Self) -> Ordering {
        dispatch!(
            S,
            plain,
            [
                rung::cmp_s8,
                rung::cmp_u8,
                rung::cmp_s16,
                rung::cmp_u16,
                rung::cmp_s32,
                rung::cmp_u32,
                rung::cmp_s64,
                rung::cmp_u64
            ],
            [self.0, rhs.0],
            []
        )
    }

    /// Whether the two are the same member.
    #[must_use]
    pub const fn eq(self, rhs: Self) -> Bool {
        Bool::of(matches!(self.cmp(rhs), Ordering::Equal))
    }

    /// Whether this member is below the other.
    #[must_use]
    pub const fn lt(self, rhs: Self) -> Bool {
        Bool::of(matches!(self.cmp(rhs), Ordering::Less))
    }

    /// Whether this member is below or at the other.
    #[must_use]
    pub const fn le(self, rhs: Self) -> Bool {
        Bool::of(!matches!(self.cmp(rhs), Ordering::Greater))
    }

    /// Whether this member is above the other.
    #[must_use]
    pub const fn gt(self, rhs: Self) -> Bool {
        Bool::of(matches!(self.cmp(rhs), Ordering::Greater))
    }

    /// Whether this member is above or at the other.
    #[must_use]
    pub const fn ge(self, rhs: Self) -> Bool {
        Bool::of(!matches!(self.cmp(rhs), Ordering::Less))
    }

    /// The lower of the two members.
    #[must_use]
    pub const fn min(self, rhs: Self) -> Self {
        if self.le(rhs).get() { self } else { rhs }
    }

    /// The higher of the two members.
    #[must_use]
    pub const fn max(self, rhs: Self) -> Self {
        if self.ge(rhs).get() { self } else { rhs }
    }
}

// Written by hand so that neither `S` nor `C` is bounded beyond what the value
// needs. Equality reads the carrier, which is canonical; `Hash` and `Debug` read
// the slot, so neither depends on which carrier the placement derived.

impl<S: Sole, C: DebugCheck> Clone for Num<S, C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S: Sole, C: DebugCheck> Copy for Num<S, C> {}

impl<S: Sole, C: DebugCheck> PartialEq for Num<S, C> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<S: Sole, C: DebugCheck> Eq for Num<S, C> {}

impl<S: Sole, C: DebugCheck> Hash for Num<S, C> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.slot().index().hash(state);
    }
}

impl<S: Sole, C: DebugCheck> fmt::Debug for Num<S, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Num").field(&self.slot()).finish()
    }
}
