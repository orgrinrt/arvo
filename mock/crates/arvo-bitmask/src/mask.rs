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

use arvo::newtype::USize;
use arvo::strategy::Hot;
use arvo_bits::{BitAccess, BitSequence, BitWidth, QWord};

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
        <W as BitWidth>::WIDTH
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
