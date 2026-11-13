//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The value, its doors and its operations.

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
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

use crate::admit::Sole;
use crate::machine::{Machine, copy};
use crate::rung;

type SlotsOf<S> = <<S as DeclaredSignature>::Format as Format>::Slots;

/// A value of the declared signature `S`: one member of its format, held alone
/// in the carrier the placement derives at sole occupancy.
///
/// The carrier is canonical, one member to one bit pattern, so two values are
/// equal exactly when their slots are. Every operation is the format's own on
/// the slot, computed in the carrier.
#[repr(transparent)]
pub struct Num<S: Sole>(S::Repr);

/// One operation, dispatched on the carrier's size and signedness to the rung
/// function at that machine integer. The match is over constants of the
/// instantiation, so one arm survives monomorphisation.
///
/// `carrier` moves the rung's result back into the carrier; `plain` returns it
/// as it is.
macro_rules! dispatch {
    (
        $r:ty, $out:ident,
        [$s8:path, $u8:path, $s16:path, $u16:path, $s32:path, $u32:path, $s64:path, $u64:path],
        [$($op:expr),*], [$($tail:expr),*]
    ) => {
        match const { (size_of::<$r>(), <$r as Machine>::SIGNED.get()) } {
            (1, true) => dispatch!(@$out i8, $r, $s8($(copy::<$r, i8>($op),)* $($tail),*)),
            (1, false) => dispatch!(@$out u8, $r, $u8($(copy::<$r, u8>($op),)* $($tail),*)),
            (2, true) => dispatch!(@$out i16, $r, $s16($(copy::<$r, i16>($op),)* $($tail),*)),
            (2, false) => dispatch!(@$out u16, $r, $u16($(copy::<$r, u16>($op),)* $($tail),*)),
            (4, true) => dispatch!(@$out i32, $r, $s32($(copy::<$r, i32>($op),)* $($tail),*)),
            (4, false) => dispatch!(@$out u32, $r, $u32($(copy::<$r, u32>($op),)* $($tail),*)),
            (8, true) => dispatch!(@$out i64, $r, $s64($(copy::<$r, i64>($op),)* $($tail),*)),
            (8, false) => dispatch!(@$out u64, $r, $u64($(copy::<$r, u64>($op),)* $($tail),*)),
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

impl<S: Sole> Num<S> {
    /// The value at a member's slot index. The index is a member's.
    const fn at_index(index: i128) -> Self {
        let () = S::ADMITTED;
        Self(dispatch!(
            S::Repr,
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
    /// Total. Under `Wrap` or `Saturate` in a debug build a position whose
    /// rounded slot leaves the range asserts; under `Wrapping` or `Saturating`
    /// it is completed and nothing asserts.
    #[must_use]
    pub const fn adapted(exact: Exact) -> Self {
        if cfg!(debug_assertions) && const { S::ASSERTS.get() } {
            assert!(
                !panic_on_overflow::<S>(exact, Dither::UNUSED).get(),
                "the position leaves the signature's range, and the plain overflow marker asserts on \
                 that in a debug build. Spell the policy `Wrapping` or `Saturating` where the \
                 completion is what you mean"
            );
        }
        Self::at_index(adapt::<S>(exact, Dither::UNUSED).index())
    }

    /// The member the carrier bits name, after the adaptation onto the declared
    /// width.
    ///
    /// Total over every bit pattern; `from_raw(v.raw())` is `v`. Under `Wrap` or
    /// `Saturate` in a debug build, bits outside the range assert.
    #[must_use]
    pub const fn from_raw(raw: S::Repr) -> Self {
        let () = S::ADMITTED;
        let member = Self(dispatch!(
            S::Repr,
            carrier,
            [
                rung::adapt_s8,
                rung::adapt_u8,
                rung::adapt_s16,
                rung::adapt_u16,
                rung::adapt_s32,
                rung::adapt_u32,
                rung::adapt_s64,
                rung::adapt_u64
            ],
            [raw],
            [const { S::WIDTH.count() }, S::POLICY]
        ));
        if cfg!(debug_assertions) && const { S::ASSERTS.get() } {
            assert!(
                member.eq(Self(raw)).get(),
                "the bits name a slot outside the signature's range, and the plain overflow marker \
                 asserts on that in a debug build. Spell the policy `Wrapping` or `Saturating` \
                 where the completion is what you mean"
            );
        }
        member
    }

    /// The member's slot. An unwrap door.
    #[must_use]
    pub const fn slot(self) -> Slot {
        Slot::at(dispatch!(
            S::Repr,
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

    /// The carrier bits. An unwrap door, and the one place a machine integer
    /// shows.
    #[must_use]
    pub const fn raw(self) -> S::Repr {
        self.0
    }

    /// The sum under the signature: the slot `arvo_format::add` gives for the
    /// two slots.
    ///
    /// Total. Under `Wrap` or `Saturate` in a debug build a sum that leaves the
    /// range asserts, asking `fits_add`; under `Wrapping` or `Saturating` it is
    /// completed and nothing asserts.
    #[must_use]
    pub const fn add(self, rhs: Self) -> Self {
        if cfg!(debug_assertions) && const { S::ASSERTS.get() } {
            assert!(
                self.fits_add(rhs).get(),
                "the sum leaves the signature's range, and the plain overflow marker asserts on that \
                 in a debug build. Spell the policy `Wrapping` or `Saturating` where the completion \
                 is what you mean"
            );
        }
        Self(dispatch!(
            S::Repr,
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
            S::Repr,
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

// Written by hand so that `S` itself is not bounded, and through the carrier,
// which is canonical.

impl<S: Sole> Clone for Num<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S: Sole> Copy for Num<S> {}

impl<S: Sole> PartialEq for Num<S> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<S: Sole> Eq for Num<S> {}

impl<S: Sole> Hash for Num<S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<S: Sole> fmt::Debug for Num<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Num").field(&self.0).finish()
    }
}
