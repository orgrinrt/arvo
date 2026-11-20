//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The packed word: a word of a declared width divided into named fields.
//!
//! A consumer declares a layout and its fields as marker types of its own.
//! `Field` names the type a field reads as; `Layout` names the container the
//! word is held in and the fields as a `notko_hlist` list from the low bit up.
//! A field's position is the sum of the widths listed before it and is written
//! nowhere, so a field names no layout and sits at its own place in each layout
//! that lists it. `Word<L>` holds the container and nothing else, and reads and
//! replaces one field at a time, each a `const fn` a consumer with no feature
//! line calls in a constant.
//!
//! The layout is checked at build and nowhere else: every entry point forces a
//! `const` item asserting that the field widths sum to exactly the container's
//! width, each width one to sixty-four. A field the list does not hold, or
//! holds twice, is refused at type check. Nothing is checked at run time.
//!
//! What the check reads of the list comes from `Fields` and `Below`, sealed: a
//! consumer names them in a bound and cannot implement them, so no list claims
//! a width or a position its fields do not have.

use arvo_format::{Bool, Width};
use notko_hlist::{Cons, Empty, Here, Place, PlaceOf, There};

use crate::{BitWidth, Bits};

/// A value carried in the low bits of a `Bits<64>`: what a word is held in,
/// and what a field reads as.
///
/// `Bits<N>` and `Bool` implement it here; a consumer's own type implements it
/// in its own crate.
pub const trait Lane: Copy {
    /// How many low bits the value takes.
    const WIDTH: Width;

    /// The value from the low bits of `low`, every bit above them dropped.
    fn from_low(low: Bits<64>) -> Self;

    /// The value in the low bits, every bit above them clear.
    fn low(self) -> Bits<64>;
}

const impl<const N: u32> Lane for Bits<N> {
    // Read through `BitWidth`, so a width of zero or past sixty-four is refused
    // by the container's own admission.
    const WIDTH: Width = <Self as BitWidth>::WIDTH;

    fn from_low(low: Bits<64>) -> Self {
        low.cast::<N>()
    }

    fn low(self) -> Bits<64> {
        self.cast::<64>()
    }
}

/// One bit, read and written as the tier reads and writes bit zero.
const impl Lane for Bool {
    const WIDTH: Width = Width::bits(1);

    fn from_low(low: Bits<64>) -> Self {
        low.bit::<0>()
    }

    fn low(self) -> Bits<64> {
        Bits::<64>::masked(0).with_bit::<0>(self)
    }
}

/// A named field of a packed word.
///
/// Implemented by a consumer's marker type, passed by value to `Word::get` and
/// `Word::with`. A field says only what it reads as; where it sits is its
/// layout's list.
pub trait Field: Copy {
    /// What the field reads as, and so how wide it is.
    ///
    /// Bound `const` here, so the accessors need no const bound of their own.
    type Value: const Lane;
}

/// The layout of a packed word: what it is held in, and its fields in order.
///
/// Implemented by a consumer's marker type.
pub trait Layout {
    /// The container the word is held in, whose width the fields tile.
    type Container: const Lane;

    /// The fields from the low bit up, as `Cons<A, Cons<B, Cons<C, Empty>>>`.
    type Fields: Fields;
}

/// The low `width` bits set.
///
/// Callers reach it only past `Word::TILED`, which has refused every width the
/// shift cannot take.
const fn low_bits(width: Width) -> u64 {
    u64::MAX >> (64 - width.count())
}

/// The width of a field, refused at build where it has no bits or does not fit
/// the sixty-four-bit carrier.
const fn field_width<F: Field>() -> Width {
    let width = <F::Value as Lane>::WIDTH;
    assert!(
        width.count() >= 1 && width.count() <= 64,
        "a field of a packed word is zero bits wide or wider than sixty-four; its value's \
         `Lane::WIDTH` must be one to sixty-four"
    );
    width
}

/// The seal on the list traits: implemented for `Empty` and `Cons<H, T>` here
/// and, being unnameable outside this crate, nowhere else.
mod sealed {
    use notko_hlist::{Cons, Empty};

    use super::Field;

    /// A list of fields, as only this crate folds one.
    pub trait ListOfFields {}

    impl ListOfFields for Empty {}

    impl<H: Field, T: ListOfFields> ListOfFields for Cons<H, T> {}
}

/// A list of fields, folded into the width the tiling check reads.
///
/// Sealed: implemented for `Empty` and for `Cons<H, T>` with `H` a field and
/// `T` a list, and by nothing outside this crate.
pub trait Fields: sealed::ListOfFields {
    /// The field widths summed, each refused at build outside one to sixty-four.
    const SUM: Width;
}

impl Fields for Empty {
    const SUM: Width = Width::bits(0);
}

impl<H: Field, T: Fields> Fields for Cons<H, T> {
    const SUM: Width = field_width::<H>().add(T::SUM);
}

/// The position of the member at the place `P` of a list: the widths listed
/// below it, summed.
///
/// Sealed through `Fields`, and named in the bound on `Word::get` and
/// `Word::with`, so code generic over a field writes `L::Fields: Below<P>`.
pub trait Below<P: Place>: Fields {
    /// How many of the word's bits lie below the member.
    const BELOW: Width;
}

impl<H: Field, T: Fields> Below<Here> for Cons<H, T> {
    const BELOW: Width = Width::bits(0);
}

impl<H: Field, P: Place, T: Below<P>> Below<There<P>> for Cons<H, T> {
    const BELOW: Width = field_width::<H>().add(T::BELOW);
}

/// A word of the layout `L`, held in its container and nothing else.
///
/// Every bit pattern of the container is a word, since the fields tile it, so
/// `of` takes every value and validates nothing. Equality, copying and the debug
/// view are written by hand and bound on the container, never on the layout
/// marker, which a consumer has no reason to derive anything on.
#[repr(transparent)]
pub struct Word<L: Layout>(L::Container);

impl<L: Layout> Word<L> {
    /// Refused at build where the fields do not tile the container exactly.
    const TILED: () = {
        let width = <L::Container as Lane>::WIDTH;
        assert!(
            width.count() >= 1 && width.count() <= 64,
            "the container of a packed word is zero bits wide or wider than sixty-four; its \
             `Lane::WIDTH` must be one to sixty-four"
        );
        assert!(
            <L::Fields as Fields>::SUM.equals(width).get(),
            "the field widths of a packed word do not sum to the width of its container; a bit \
             no field reads is still a field, so declare it as one"
        );
    };

    /// The word over a container, every field read from its bits.
    #[must_use]
    pub const fn of(bits: L::Container) -> Self {
        let () = Self::TILED;
        Self(bits)
    }

    /// The container back.
    #[must_use]
    pub const fn bits(self) -> L::Container {
        self.0
    }

    /// The field `F`, its place `P` in the layout's list inferred.
    #[must_use]
    pub const fn get<F: Field, P: Place>(self, _field: F) -> F::Value
    where
        L::Fields: PlaceOf<F, P> + Below<P>,
    {
        let () = Self::TILED;
        let down = self.0.low().raw() >> <L::Fields as Below<P>>::BELOW.count();
        // The mutant `arvo_mutant_bits_word_get` reads the field one bit up.
        let down = if cfg!(arvo_mutant_bits_word_get) { down >> 1 } else { down };
        // Masked, so a value reading bits above its width sees none of another field's.
        let down = down & low_bits(<F::Value as Lane>::WIDTH);
        F::Value::from_low(Bits::<64>::masked(down))
    }

    /// The word with the field `F` replaced by `to`, every other bit kept.
    #[must_use]
    pub const fn with<F: Field, P: Place>(self, _field: F, to: F::Value) -> Self
    where
        L::Fields: PlaceOf<F, P> + Below<P>,
    {
        let () = Self::TILED;
        let below = <L::Fields as Below<P>>::BELOW.count();
        let field = low_bits(<F::Value as Lane>::WIDTH) << below;
        // The mutant `arvo_mutant_bits_word_with` keeps the field's old bits.
        let kept = if cfg!(arvo_mutant_bits_word_with) {
            self.0.low().raw()
        } else {
            self.0.low().raw() & !field
        };
        // Masked, so a value leaving bits set above its width reaches no other field.
        let put = (to.low().raw() << below) & field;
        Self(L::Container::from_low(Bits::<64>::masked(kept | put)))
    }
}

impl<L: Layout> Clone for Word<L> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<L: Layout> Copy for Word<L> {}

impl<L: Layout> PartialEq for Word<L>
where
    L::Container: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<L: Layout> Eq for Word<L> where L::Container: Eq {}

impl<L: Layout> core::fmt::Debug for Word<L>
where
    L::Container: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}
