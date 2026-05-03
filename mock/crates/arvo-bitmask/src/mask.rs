//! Fixed-width bitmask types.
//!
//! `Mask<W>` is a generic bitmask over a single bit-bearing word type
//! `W` that implements the arvo-bits `BitSequence + BitAccess`
//! contracts. The bitmask width is derived from `W::WIDTH`.
//!
//! Two concrete shipping flavours:
//!
//! - `Mask64` = `Mask<QWord<Hot>>`. Backed by `u64`. Covers up to 64
//!   elements. Single-instruction set-ops.
//! - `Mask256` = a distinct struct wrapping `[QWord<Hot>; 4]`. Covers
//!   up to 256 elements. Four 64-bit words with unrolled loop-free
//!   set-ops. It is not a `Mask<[QWord<Hot>; 4]>` because Rust arrays
//!   do not implement the arvo-bits traits; unifying through a
//!   generic would require extra trait plumbing. Keep it flat.
//!
//! Set operations, predicates, and bit scanning live on the two
//! shipping flavours as inherent methods (`ops.rs`). `Mask<W>` is the
//! generic chassis; consumers that want a narrower width can
//! substitute any `W` meeting the trait bounds.

use arvo::USize;
use arvo::strategy::{Bounded, Hot, Identity};
use arvo_bits::QWord;
use arvo_bits_contracts::{BitAccess, BitSequence, HasBitWidth};

/// Generic fixed-width bitmask.
///
/// `W` is a single bit-bearing word. Width follows from `W::WIDTH`.
/// The shipping aliases are `Mask64` and `Mask256`; `Mask256` is a
/// distinct struct (see module docs) rather than `Mask<[_; 4]>`.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask<W>
where
    W: BitSequence + BitAccess + Copy + Default,
{
    /// Raw word storage.
    pub word: W,
}

impl<W> Mask<W>
where
    W: BitSequence + BitAccess + Copy + Default,
{
    /// Empty mask (all bits cleared).
    #[inline(always)]
    pub fn empty() -> Self {
        Self { word: W::default() }
    }

    /// Construct a mask from a raw word.
    #[inline(always)]
    pub const fn from_word(word: W) -> Self {
        Self { word }
    }

    /// Extract the raw word.
    #[inline(always)]
    pub const fn to_word(self) -> W {
        self.word
    }

    /// Logical bit width of the mask (from `W::WIDTH`).
    #[inline(always)]
    pub const fn width() -> USize {
        <W as HasBitWidth>::WIDTH
    }
}

impl<W> Default for Mask<W>
where
    W: BitSequence + BitAccess + Copy + Default,
{
    #[inline(always)]
    fn default() -> Self {
        Self::empty()
    }
}

/// 64-bit bitmask. Backed by `QWord<Hot>` (u64 container).
pub type Mask64 = Mask<QWord<Hot>>;

/// 256-bit bitmask.
///
/// Stored as four 64-bit words. Distinct from `Mask<W>` because Rust
/// arrays do not implement the arvo-bits traits.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask256(pub [QWord<Hot>; 4]);

impl Mask256 {
    /// Empty mask (all 256 bits cleared).
    #[inline]
    pub fn empty() -> Self {
        Self([QWord::<Hot>::default(); 4])
    }

    /// Construct from a fixed four-word array.
    #[inline(always)]
    pub const fn from_words(words: [QWord<Hot>; 4]) -> Self {
        Self(words)
    }

    /// Extract the backing word array.
    #[inline(always)]
    pub const fn to_words(self) -> [QWord<Hot>; 4] {
        self.0
    }

    /// Logical bit width of the mask (256).
    #[inline(always)]
    pub const fn width() -> USize {
        USize(256)
    }
}

impl Default for Mask256 {
    #[inline(always)]
    fn default() -> Self {
        Self::empty()
    }
}

// --- Generic Bounded / Identity blankets (round 202605021600) ----------
//
// `Mask<W>::MIN` (empty / all-zero) and `Mask<W>::MAX` (full / all-ones)
// flow through the underlying word's Bounded impl. The hand-coded
// `pub const FULL = ...u64::MAX...` constants from round 202605021400
// were leaky abstractions: the Mask<W> signature did not promise its
// container's primitive width, but the constants assumed u64. The
// blanket here removes that assumption: Mask<W>::MAX is always whatever
// `<W as Bounded>::MAX` is, which is correct for any future container.
//
// Identity::ZERO is the empty mask (no bits set). ONE corresponds to
// the underlying word's ONE (the lowest bit set), which is occasionally
// useful for shift-and-test patterns.

impl<W> const Bounded for Mask<W>
where
    W: BitSequence + BitAccess + Copy + Default + [const] Bounded,
{
    const MIN: Self = Self::from_word(<W as Bounded>::MIN);
    const MAX: Self = Self::from_word(<W as Bounded>::MAX);
}

impl<W> const Identity for Mask<W>
where
    W: BitSequence + BitAccess + Copy + Default + [const] Identity,
{
    const ZERO: Self = Self::from_word(<W as Identity>::ZERO);
    const ONE: Self = Self::from_word(<W as Identity>::ONE);
}

impl const Bounded for Mask256 {
    const MIN: Self = Self([<QWord<Hot> as Bounded>::MIN; 4]);
    const MAX: Self = Self([<QWord<Hot> as Bounded>::MAX; 4]);
}

impl const Identity for Mask256 {
    const ZERO: Self = Self([<QWord<Hot> as Identity>::ZERO; 4]);
    // ONE on Mask256 sets the lowest bit of the lowest word.
    const ONE: Self = Self([
        <QWord<Hot> as Identity>::ONE,
        <QWord<Hot> as Identity>::ZERO,
        <QWord<Hot> as Identity>::ZERO,
        <QWord<Hot> as Identity>::ZERO,
    ]);
}
