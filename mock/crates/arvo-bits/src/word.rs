//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The packed word: a word of a declared width divided into named fields.
//!
//! A consumer declares a layout and its fields as marker types of its own.
//! `Layout` names the container the word is held in and the fields as a list
//! `(A, (B, (C, ())))` in any order; `Field` names the word it belongs to, the
//! type it reads as and the count of the word's bits under it. `Word<L>` holds
//! the container and nothing else, and reads and replaces one field at a time,
//! each a `const fn`.
//!
//! The layout is checked at build and nowhere else: every entry point forces a
//! `const` item asserting that the fields share no bit, sum to the container's
//! width and cover it, and a read or a write forces one more asserting that the
//! layout lists the field. Nothing is checked at run time. A width of zero or
//! past sixty-four, and a field reaching past bit sixty-four, are refused
//! before any shift is worked out.
//!
//! What the checks read of the list comes from `Fields` and `Holds`, sealed:
//! a consumer names them in a bound and cannot implement them, so no list
//! claims a sum, a union or an overlap its fields do not have.

use core::marker::PhantomData;

use arvo_format::{Bool, Width};

use crate::Bits;

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
    const WIDTH: Width = Width::bits(N);

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
/// Implemented by a consumer's marker type. A field states where it sits; its
/// layout states that the word has it.
pub trait Field {
    /// The word this field belongs to.
    type Layout: Layout;

    /// What the field reads as, and so how wide it is.
    type Value: Lane;

    /// How many of the word's bits lie below the field.
    const BELOW: Width;
}

/// The layout of a packed word: what it is held in, and which fields it has.
///
/// Implemented by a consumer's marker type.
pub trait Layout {
    /// The container the word is held in, whose width the fields tile.
    type Container: Lane;

    /// The fields, as a list `(A, (B, (C, ())))` in any order.
    type Fields: Fields;
}

/// The bits a field takes, as a mask over the sixty-four-bit carrier.
///
/// Callers reach it only through `placed`, which has refused every width and
/// place the shifts below cannot take.
const fn span(below: Width, width: Width) -> Bits<64> {
    let low = u64::MAX >> (64 - width.count());
    Bits::<64>::masked(low << below.count())
}

/// The bits a field of `width` at `below` takes, refused at build where the
/// field has no bits or does not fit the sixty-four-bit carrier.
const fn placed(below: Width, width: Width) -> Bits<64> {
    assert!(
        width.count() >= 1 && width.count() <= 64,
        "a field of a packed word is zero bits wide or wider than sixty-four; its value's \
         `Lane::WIDTH` must be one to sixty-four"
    );
    assert!(
        below.count() <= 64 - width.count(),
        "a field of a packed word reaches past bit sixty-four; its `BELOW` and its width must \
         sum to sixty-four at most"
    );
    span(below, width)
}

/// The seal on the list traits: implemented for `()` and `(H, T)` here and,
/// being unnameable outside this crate, nowhere else.
mod sealed {
    use super::Field;

    /// A list of fields, as only this crate can build one.
    pub trait Sealed {}

    impl Sealed for () {}

    impl<H: Field, T: Sealed> Sealed for (H, T) {}
}

/// A list of fields, folded into what the tiling check reads.
///
/// Sealed: implemented for `()` and for `(H, T)` with `H` a field and `T` a
/// list, and by nothing outside this crate, so no list claims a sum, a union
/// or an overlap its fields do not have.
pub trait Fields: sealed::Sealed {
    /// The field widths summed.
    const SUM: Width;

    /// The union of every field's bits.
    const COVER: Bits<64>;

    /// Whether any two fields share a bit.
    const OVERLAP: Bool;
}

impl Fields for () {
    const SUM: Width = Width::bits(0);
    const COVER: Bits<64> = Bits::<64>::masked(0);
    const OVERLAP: Bool = Bool::FALSE;
}

impl<H: Field, T: Fields> Fields for (H, T) {
    const SUM: Width = <H::Value as Lane>::WIDTH.add(T::SUM);
    const COVER: Bits<64> =
        Bits::<64>::masked(placed(H::BELOW, <H::Value as Lane>::WIDTH).raw() | T::COVER.raw());
    const OVERLAP: Bool =
        Bool::of(placed(H::BELOW, <H::Value as Lane>::WIDTH).raw() & T::COVER.raw() != 0)
            .or(T::OVERLAP);
}

/// Whether a list holds a field at the place and width of `F`.
///
/// A marker at the place and width of a listed field is that field under a
/// second name, and reads and writes the same bits. Sealed like `Fields`, and
/// named in the bound on `Word::get` and `Word::with`, so code generic over a
/// field writes `L::Fields: Holds<F>`.
pub trait Holds<F: Field>: sealed::Sealed {
    /// Whether it does.
    const HOLDS: Bool;
}

impl<F: Field> Holds<F> for () {
    const HOLDS: Bool = Bool::FALSE;
}

impl<H: Field, T: Holds<F>, F: Field> Holds<F> for (H, T) {
    const HOLDS: Bool = H::BELOW
        .equals(F::BELOW)
        .and(<H::Value as Lane>::WIDTH.equals(<F::Value as Lane>::WIDTH))
        .or(T::HOLDS);
}

/// The proof that `F` is one of its layout's fields, refused at build otherwise.
struct Listed<F>(PhantomData<F>);

impl<F: Field> Listed<F>
where
    <F::Layout as Layout>::Fields: Holds<F>,
{
    const NEW: () = assert!(
        <<F::Layout as Layout>::Fields as Holds<F>>::HOLDS.get(),
        "a field read or written through a word whose layout does not list it; add the field's \
         marker to the layout's `Fields`, or read it through the word it was declared for"
    );
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
            !<L::Fields as Fields>::OVERLAP.get(),
            "two fields of a packed word share a bit; each field's `BELOW` and width must leave \
             every other field's bits alone"
        );
        assert!(
            <L::Fields as Fields>::SUM.equals(width).get(),
            "the field widths of a packed word do not sum to the width of its container; a bit \
             no field reads is still a field, so declare it as one"
        );
        assert!(
            <L::Fields as Fields>::COVER.raw() == span(Width::bits(0), width).raw(),
            "a field of a packed word reaches past its container"
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

    /// The field `F`.
    #[must_use]
    pub const fn get<F: Field<Layout = L>>(self) -> F::Value
    where
        L::Container: [const] Lane,
        F::Value: [const] Lane,
        L::Fields: Holds<F>,
    {
        let () = Self::TILED;
        let () = Listed::<F>::NEW;
        let down = self.0.low().raw() >> F::BELOW.count();
        // The mutant `arvo_mutant_bits_word_get` reads the field one bit up.
        let down = if cfg!(arvo_mutant_bits_word_get) { down >> 1 } else { down };
        F::Value::from_low(Bits::<64>::masked(down))
    }

    /// The word with the field `F` replaced by `to`, every other bit kept.
    #[must_use]
    pub const fn with<F: Field<Layout = L>>(self, to: F::Value) -> Self
    where
        L::Container: [const] Lane,
        F::Value: [const] Lane,
        L::Fields: Holds<F>,
    {
        let () = Self::TILED;
        let () = Listed::<F>::NEW;
        let field = span(F::BELOW, <F::Value as Lane>::WIDTH).raw();
        // The mutant `arvo_mutant_bits_word_with` keeps the field's old bits.
        let kept = if cfg!(arvo_mutant_bits_word_with) {
            self.0.low().raw()
        } else {
            self.0.low().raw() & !field
        };
        // Masked, so a value leaving bits set above its width reaches no other field.
        let put = (to.low().raw() << F::BELOW.count()) & field;
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
