//! The proposed packed word, written beside `arvo-bits` rather than in it, so
//! the probe builds on the crate as it stands. Each `examples/` file is a
//! consumer of what this library makes public.

#![no_std]
#![feature(const_trait_impl)]

use arvo_bits::Bits;
use arvo_format::{Bool, Width};

/// A value carried in the low bits of the sixty-four-bit container: the
/// container a word is held in, and the value a field reads as.
pub const trait Lane: Copy {
    /// How many low bits the value takes.
    const WIDTH: Width;

    /// The value from the low bits of `low`, the rest dropped.
    fn from_low(low: Bits<64>) -> Self;

    /// The value in the low bits, the rest clear.
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

/// One bit, read as the tier reads one bit.
const impl Lane for Bool {
    const WIDTH: Width = Width::bits(1);

    fn from_low(low: Bits<64>) -> Self {
        low.bit::<0>()
    }

    fn low(self) -> Bits<64> {
        Bits::<64>::masked(0).with_bit::<0>(self)
    }
}

/// A named field: the word it belongs to, what it reads as, and how many bits
/// of the word lie below it.
pub trait Field {
    type Layout: Layout;
    type Value: Lane;
    const BELOW: Width;
}

/// A word's layout: the container it is held in and its fields, as a list
/// `(A, (B, (C, ())))` in any order.
pub trait Layout {
    type Container: Lane;
    type Fields: Fields;
}

/// The bits a field takes, as a mask over the sixty-four-bit container.
const fn span(below: Width, width: Width) -> u64 {
    let w = width.count();
    let low = if w >= 64 { u64::MAX } else { (1u64 << w) - 1 };
    low << below.count()
}

/// The fields of a layout, folded into what the tiling check reads.
pub trait Fields {
    /// The widths summed.
    const SUM: u32;
    /// The union of every field's bits.
    const COVER: u64;
    /// Whether two fields share a bit.
    const OVERLAP: bool;
}

impl Fields for () {
    const SUM: u32 = 0;
    const COVER: u64 = 0;
    const OVERLAP: bool = false;
}

impl<H: Field, T: Fields> Fields for (H, T) {
    const SUM: u32 = <H::Value as Lane>::WIDTH.count() + T::SUM;
    const COVER: u64 = span(H::BELOW, <H::Value as Lane>::WIDTH) | T::COVER;
    const OVERLAP: bool =
        span(H::BELOW, <H::Value as Lane>::WIDTH) & T::COVER != 0 || T::OVERLAP;
}

/// Whether a list holds a field at `F`'s place and width.
pub trait Holds<F: Field> {
    const HOLDS: bool;
}

impl<F: Field> Holds<F> for () {
    const HOLDS: bool = false;
}

impl<H: Field, T: Holds<F>, F: Field> Holds<F> for (H, T) {
    const HOLDS: bool = (H::BELOW.count() == F::BELOW.count()
        && <H::Value as Lane>::WIDTH.count() == <F::Value as Lane>::WIDTH.count())
        || T::HOLDS;
}

/// The proof that `F` is one of its layout's fields, refused at build otherwise.
struct Listed<F>(core::marker::PhantomData<F>);

impl<F: Field> Listed<F>
where
    <F::Layout as Layout>::Fields: Holds<F>,
{
    const NEW: () = assert!(
        <<F::Layout as Layout>::Fields as Holds<F>>::HOLDS,
        "a field read or written through a word whose layout does not list it"
    );
}

/// A word of a declared layout, held in its container and nothing else.
///
/// Equality, copying and the debug view are bound on the container and never
/// on the layout marker, which a consumer has no reason to derive anything on.
#[repr(transparent)]
pub struct Word<L: Layout>(L::Container);

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

impl<L: Layout> Word<L> {
    /// Refused at build where the fields do not tile the container exactly.
    const TILED: () = {
        let width = <L::Container as Lane>::WIDTH;
        assert!(
            !<L::Fields as Fields>::OVERLAP,
            "two fields of a packed word share a bit"
        );
        assert!(
            <L::Fields as Fields>::SUM == width.count(),
            "the field widths of a packed word do not sum to the width of its container"
        );
        assert!(
            <L::Fields as Fields>::COVER == span(Width::bits(0), width),
            "a field of a packed word reaches past its container"
        );
    };
}

impl<L: Layout> Word<L> {
    /// The word over a container, every field read from its bits.
    pub const fn of(bits: L::Container) -> Self {
        let () = Self::TILED;
        Self(bits)
    }

    /// The container back.
    pub const fn bits(self) -> L::Container {
        self.0
    }

    /// The field `F`.
    pub const fn get<F: Field<Layout = L>>(self) -> F::Value
    where
        L::Container: [const] Lane,
        F::Value: [const] Lane,
        L::Fields: Holds<F>,
    {
        let () = Self::TILED;
        let () = Listed::<F>::NEW;
        F::Value::from_low(Bits::<64>::masked(self.0.low().raw() >> F::BELOW.count()))
    }

    /// The word with the field `F` replaced by `to`, every other field kept.
    pub const fn with<F: Field<Layout = L>>(self, to: F::Value) -> Self
    where
        L::Container: [const] Lane,
        F::Value: [const] Lane,
        L::Fields: Holds<F>,
    {
        let () = Self::TILED;
        let () = Listed::<F>::NEW;
        let kept = self.0.low().raw() & !span(F::BELOW, <F::Value as Lane>::WIDTH);
        let put = to.low().raw() << F::BELOW.count();
        Self(L::Container::from_low(Bits::<64>::masked(kept | put)))
    }
}
