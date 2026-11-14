//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The value, its doors and its operations.

use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::mem::size_of;

use arvo_format::overflow::Overflow;
use arvo_format::rounding::Rounding;
use arvo_format::{
    Bool,
    DeclaredSignature,
    Dither,
    Exact,
    Format,
    Quantum,
    Slot,
    Slots,
    WithOverflow,
    WithRounding,
    adapt,
    mul as mul_slots,
    panic_on_overflow,
    product_position,
    slot_in_range,
};
use arvo_placement::SoleCarrier;

use crate::admit::Sole;
use crate::check::{Abort, DebugCheck};
use crate::machine::copy;
use crate::map::Held;
use crate::rung;

pub(crate) type SlotsOf<S> = <<S as DeclaredSignature>::Format as Format>::Slots;

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

pub(crate) use dispatch;

impl<S: Sole, C: DebugCheck> Num<S, C> {
    /// The value at a member's slot index. The index is a member's.
    pub(crate) const fn at_index(index: i128) -> Self {
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

    /// The same member held under `T`, a signature over this one's format.
    ///
    /// The slot is unchanged and every operation after the crossing is `T`'s.
    /// The carrier is handed over as it is: its type and its encoding are the
    /// format's, so the two values are the same bits. Nothing is read of the
    /// range, and the debug check is kept.
    ///
    /// The caller writes `T`'s admission, since this signature's says nothing
    /// of it. A `T` over another format is a type error here, because its slot
    /// would have to be rounded or completed onto the other grid.
    #[must_use]
    pub const fn under<T>(self) -> Num<T, C>
    where
        T: Sole + DeclaredSignature<Format = S::Format>,
    {
        if cfg!(arvo_mutant_under) && self.slot().index() == <SlotsOf<S> as Slots>::MAX.index() {
            return Num::at_index(<SlotsOf<S> as Slots>::MIN.index());
        }
        Num::from_carrier(self.0)
    }

    /// The same member under this signature with its overflow policy replaced
    /// by `O`: `under` at `WithOverflow<S, O>`.
    ///
    /// For a site that names its signature. Code generic over the signature
    /// takes the target as a parameter and calls `under`.
    #[must_use]
    pub const fn with_overflow<O: Overflow>(self) -> Num<WithOverflow<S, O>, C>
    where
        WithOverflow<S, O>: Sole + DeclaredSignature<Format = S::Format>,
    {
        self.under()
    }

    /// The same member under this signature with its rounding mode replaced by
    /// `R`: `under` at `WithRounding<S, R>`.
    ///
    /// For a site that names its signature, as `with_overflow` is.
    #[must_use]
    pub const fn with_rounding<R: Rounding>(self) -> Num<WithRounding<S, R>, C>
    where
        WithRounding<S, R>: Sole + DeclaredSignature<Format = S::Format>,
    {
        self.under()
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

    /// The difference under the signature: the slot `arvo_format::sub` gives
    /// for the two slots.
    ///
    /// Total: a difference that leaves the range is completed by the declared
    /// overflow policy. Under the `Abort` check a debug build aborts on that
    /// instead, which `fits_sub` asks ahead; under `Quiet` no build does.
    #[must_use]
    pub const fn sub(self, rhs: Self) -> Self {
        if cfg!(debug_assertions) && const { C::ABORTS.get() } {
            assert!(
                self.fits_sub(rhs).get(),
                "the difference leaves the signature's range, and the `Abort` check aborts on that in \
                 a debug build. Name the `Quiet` check where the completion the overflow policy gives \
                 is what you mean"
            );
        }
        Self::from_carrier(dispatch!(
            S,
            carrier,
            [
                rung::sub_s8,
                rung::sub_u8,
                rung::sub_s16,
                rung::sub_u16,
                rung::sub_s32,
                rung::sub_u32,
                rung::sub_s64,
                rung::sub_u64
            ],
            [self.0, rhs.0],
            [const { S::WIDTH.count() }, S::POLICY]
        ))
    }

    /// Whether the exact difference of the two members is a member, so `sub`
    /// needs no completion.
    #[must_use]
    pub const fn fits_sub(self, rhs: Self) -> Bool {
        slot_in_range::<SlotsOf<S>>(Slot::at(self.slot().index() - rhs.slot().index()))
    }

    /// The negation under the signature: the slot `arvo_format::neg` gives for
    /// the slot.
    ///
    /// Total: a negation that leaves the range is completed by the declared
    /// overflow policy, which over an unsigned range is every member's but
    /// zero's. Under the `Abort` check a debug build aborts on that instead,
    /// which `fits_neg` asks ahead; under `Quiet` no build does.
    #[must_use]
    pub const fn neg(self) -> Self {
        if cfg!(debug_assertions) && const { C::ABORTS.get() } {
            assert!(
                self.fits_neg().get(),
                "the negation leaves the signature's range, and the `Abort` check aborts on that in a \
                 debug build. Name the `Quiet` check where the completion the overflow policy gives \
                 is what you mean"
            );
        }
        Self::from_carrier(dispatch!(
            S,
            carrier,
            [
                rung::neg_s8,
                rung::neg_u8,
                rung::neg_s16,
                rung::neg_u16,
                rung::neg_s32,
                rung::neg_u32,
                rung::neg_s64,
                rung::neg_u64
            ],
            [self.0],
            [const { S::WIDTH.count() }, S::POLICY]
        ))
    }

    /// Whether the exact negation of the member is a member, so `neg` needs no
    /// completion.
    #[must_use]
    pub const fn fits_neg(self) -> Bool {
        slot_in_range::<SlotsOf<S>>(Slot::at(-self.slot().index()))
    }

    /// The product under the signature: the slot `arvo_format::mul` gives for
    /// the two slots.
    ///
    /// Total: a product that leaves the range is completed by the declared
    /// overflow policy, and at a fractional quantum one between two members is
    /// rounded by the declared mode first. Under the `Abort` check a debug
    /// build aborts on a product that leaves the range, which `fits_mul` asks
    /// ahead; under `Quiet` no build does.
    ///
    /// Refused at build where the format does not hold the product's exact
    /// position, with the format's own message. The unsigned 64-bit integer is
    /// among those today, although the format's `mul` admits it under wrapping.
    #[must_use]
    pub const fn mul(self, rhs: Self) -> Self {
        const {
            // Forces the format's multiplication obligation for this signature,
            // the one `fits_mul` reads the exact position under.
            // FIXME(value_s_product_admits_the_unsigned_64_bit_wrapping_product): the
            // format's `mul` admits a wrapping product past the slot index, and the
            // value takes it once the format holds the exact position there.
            let _ = product_position::<S::Format>(Slot::at(0), Slot::at(0));
        }
        if cfg!(debug_assertions) && const { C::ABORTS.get() } {
            assert!(
                self.fits_mul(rhs).get(),
                "the product leaves the signature's range, and the `Abort` check aborts on that in a \
                 debug build. Name the `Quiet` check where the completion the overflow policy gives \
                 is what you mean"
            );
        }
        // At a quantum exponent of zero the members' product sits at the slots'
        // product, on the grid, and only the completion acts.
        if const { <<S::Format as Format>::Quantum as Quantum>::BASE.power() == 0 } {
            Self::from_carrier(dispatch!(
                S,
                carrier,
                [
                    rung::mul_s8,
                    rung::mul_u8,
                    rung::mul_s16,
                    rung::mul_u16,
                    rung::mul_s32,
                    rung::mul_u32,
                    rung::mul_s64,
                    rung::mul_u64
                ],
                [self.0, rhs.0],
                [const { S::WIDTH.count() }, S::POLICY]
            ))
        } else {
            // FIXME(arvo_numeral_s_fixed_point_product_goes_through_the_128_bit): the
            // position is scaled by the radix and rounded by the mode, and the
            // rounding lives in the format's applied map alone. Its step is four
            // limbs wide. When the format ships the adaptation at the container's
            // width this becomes the machine's widening product and one call.
            Self::at_index(mul_slots::<S>(self.slot(), rhs.slot(), Dither::UNUSED).index())
        }
    }

    /// Whether the product of the two members needs no completion: the slot the
    /// declared mode rounds its exact position to is a member.
    #[must_use]
    pub const fn fits_mul(self, rhs: Self) -> Bool {
        let position = product_position::<S::Format>(self.slot(), rhs.slot());
        Bool::of(!panic_on_overflow::<S>(position, Dither::UNUSED).get())
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
