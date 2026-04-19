//! Set operations and bit scanning for `Mask64` / `Mask256`.
//!
//! Set ops and scans live here as inherent methods on the two
//! shipping flavours — not as a blanket impl on `Mask<W>`. Same
//! const-expr cycle pattern that drove arvo-bits' bridge-trait design
//! would otherwise bite: two separate predicates on one impl block
//! (containerness + bit-prim) trip `generic_const_exprs` evaluation.
//! Keep it flat on the concrete types.
//!
//! `Mask64` reaches the backing `u64` through `UFixed::to_raw` /
//! `from_raw` for whole-word bitwise ops the arvo-bits trait surface
//! doesn't expose. `Mask256` unrolls the same pattern across four
//! 64-bit words.

use arvo::newtype::{Bool, FBits, IBits, USize};
use arvo::strategy::Hot;
use arvo::ufixed::UFixed;
use arvo_bits::{BitAccess, BitSequence, QWord};

use crate::mask::{Mask, Mask256, Mask64};

// Named const for the `FBits::ZERO` / literal-IBits parameters on
// `QWord<Hot>`. Const generics do not accept `IBits(64)` inline on
// current nightly (see arvo-bits alias module docs).
const IBITS_SIXTYFOUR: IBits = IBits(64);

// --- Helpers ---------------------------------------------------------------

/// Convert a `QWord<Hot>` to its raw `u64` container.
#[inline(always)]
fn qword_to_u64(w: QWord<Hot>) -> u64 {
    w.to_raw()
}

/// Wrap a `u64` back into a `QWord<Hot>`.
#[inline(always)]
const fn u64_to_qword(b: u64) -> QWord<Hot> {
    UFixed::<IBITS_SIXTYFOUR, { FBits::ZERO }, Hot>::from_raw(b)
}

// --- Mask64 (== Mask<QWord<Hot>>) -----------------------------------------

impl Mask<QWord<Hot>> {
    /// Union (bitwise OR).
    #[inline(always)]
    pub fn union(self, other: Self) -> Self {
        let a = qword_to_u64(self.word);
        let b = qword_to_u64(other.word);
        Self::from_word(u64_to_qword(a | b))
    }

    /// Intersection (bitwise AND).
    #[inline(always)]
    pub fn intersection(self, other: Self) -> Self {
        let a = qword_to_u64(self.word);
        let b = qword_to_u64(other.word);
        Self::from_word(u64_to_qword(a & b))
    }

    /// Difference (`self & !other`).
    #[inline(always)]
    pub fn difference(self, other: Self) -> Self {
        let a = qword_to_u64(self.word);
        let b = qword_to_u64(other.word);
        Self::from_word(u64_to_qword(a & !b))
    }

    /// Complement (bitwise NOT).
    #[inline(always)]
    pub fn complement(self) -> Self {
        let a = qword_to_u64(self.word);
        Self::from_word(u64_to_qword(!a))
    }

    /// `Bool::TRUE` when every bit is zero.
    #[inline(always)]
    pub fn is_empty(self) -> Bool {
        <QWord<Hot> as BitSequence>::is_zero(self.word)
    }

    /// `Bool::TRUE` when `self` and `other` share any bit.
    #[inline(always)]
    pub fn intersects(self, other: Self) -> Bool {
        let a = qword_to_u64(self.word);
        let b = qword_to_u64(other.word);
        Bool((a & b) != 0)
    }

    /// `Bool::TRUE` when bit at `pos` is set.
    #[inline(always)]
    pub fn contains(self, pos: USize) -> Bool {
        <QWord<Hot> as BitAccess>::bit(self.word, pos)
    }

    /// Set bit at `pos`.
    #[inline(always)]
    pub fn insert(&mut self, pos: USize) {
        self.word = <QWord<Hot> as BitAccess>::with_bit_set(self.word, pos);
    }

    /// Clear bit at `pos`.
    #[inline(always)]
    pub fn remove(&mut self, pos: USize) {
        self.word = <QWord<Hot> as BitAccess>::with_bit_cleared(self.word, pos);
    }

    /// Popcount.
    #[inline(always)]
    pub fn count(self) -> USize {
        <QWord<Hot> as BitSequence>::count_ones(self.word)
    }

    /// Lowest set bit index. Returns 64 if the mask is empty
    /// (matches `trailing_zeros` semantics on a zero word).
    #[inline(always)]
    pub fn lowest_set(self) -> USize {
        <QWord<Hot> as BitSequence>::trailing_zeros(self.word)
    }

    /// Highest set bit index. Returns 64 if the mask is empty.
    #[inline(always)]
    pub fn highest_set(self) -> USize {
        let a = qword_to_u64(self.word);
        if a == 0 {
            return USize(64);
        }
        USize(63 - a.leading_zeros() as usize)
    }

    /// Iterator over set bit indices, lowest-first.
    #[inline(always)]
    pub fn iter_set_bits(self) -> SetBitsIter64 {
        SetBitsIter64 { remaining: qword_to_u64(self.word) }
    }
}

/// Nameable iterator over set bits of a `Mask64`.
///
/// Advance via `trailing_zeros` + clear lowest bit; yields bit
/// indices lowest-first.
#[derive(Copy, Clone)]
pub struct SetBitsIter64 {
    remaining: u64,
}

impl Iterator for SetBitsIter64 {
    type Item = USize;

    #[inline(always)]
    fn next(&mut self) -> Option<USize> {
        if self.remaining == 0 {
            return None;
        }
        let idx = self.remaining.trailing_zeros() as usize;
        // Clear the lowest set bit.
        self.remaining &= self.remaining - 1;
        Some(USize(idx))
    }
}

// --- Mask256 ---------------------------------------------------------------

impl Mask256 {
    /// Union (bitwise OR across all four words).
    #[inline]
    pub fn union(self, other: Self) -> Self {
        let a = self.0;
        let b = other.0;
        Self([
            u64_to_qword(qword_to_u64(a[0]) | qword_to_u64(b[0])),
            u64_to_qword(qword_to_u64(a[1]) | qword_to_u64(b[1])),
            u64_to_qword(qword_to_u64(a[2]) | qword_to_u64(b[2])),
            u64_to_qword(qword_to_u64(a[3]) | qword_to_u64(b[3])),
        ])
    }

    /// Intersection (bitwise AND across all four words).
    #[inline]
    pub fn intersection(self, other: Self) -> Self {
        let a = self.0;
        let b = other.0;
        Self([
            u64_to_qword(qword_to_u64(a[0]) & qword_to_u64(b[0])),
            u64_to_qword(qword_to_u64(a[1]) & qword_to_u64(b[1])),
            u64_to_qword(qword_to_u64(a[2]) & qword_to_u64(b[2])),
            u64_to_qword(qword_to_u64(a[3]) & qword_to_u64(b[3])),
        ])
    }

    /// Difference (`self & !other` across all four words).
    #[inline]
    pub fn difference(self, other: Self) -> Self {
        let a = self.0;
        let b = other.0;
        Self([
            u64_to_qword(qword_to_u64(a[0]) & !qword_to_u64(b[0])),
            u64_to_qword(qword_to_u64(a[1]) & !qword_to_u64(b[1])),
            u64_to_qword(qword_to_u64(a[2]) & !qword_to_u64(b[2])),
            u64_to_qword(qword_to_u64(a[3]) & !qword_to_u64(b[3])),
        ])
    }

    /// Complement (bitwise NOT across all four words).
    #[inline]
    pub fn complement(self) -> Self {
        let a = self.0;
        Self([
            u64_to_qword(!qword_to_u64(a[0])),
            u64_to_qword(!qword_to_u64(a[1])),
            u64_to_qword(!qword_to_u64(a[2])),
            u64_to_qword(!qword_to_u64(a[3])),
        ])
    }

    /// `Bool::TRUE` when every bit is zero.
    #[inline(always)]
    pub fn is_empty(self) -> Bool {
        let a = self.0;
        Bool(
            qword_to_u64(a[0]) == 0
                && qword_to_u64(a[1]) == 0
                && qword_to_u64(a[2]) == 0
                && qword_to_u64(a[3]) == 0,
        )
    }

    /// `Bool::TRUE` when `self` and `other` share any bit.
    #[inline(always)]
    pub fn intersects(self, other: Self) -> Bool {
        let a = self.0;
        let b = other.0;
        let any = (qword_to_u64(a[0]) & qword_to_u64(b[0]))
            | (qword_to_u64(a[1]) & qword_to_u64(b[1]))
            | (qword_to_u64(a[2]) & qword_to_u64(b[2]))
            | (qword_to_u64(a[3]) & qword_to_u64(b[3]));
        Bool(any != 0)
    }

    /// `Bool::TRUE` when bit at `pos` is set. `pos >= 256` returns
    /// `Bool::FALSE`.
    #[inline(always)]
    pub fn contains(self, pos: USize) -> Bool {
        let p = pos.0;
        if p >= 256 {
            return Bool::FALSE;
        }
        let (word_idx, bit_idx) = (p >> 6, p & 63);
        <QWord<Hot> as BitAccess>::bit(self.0[word_idx], USize(bit_idx))
    }

    /// Set bit at `pos`. Leaves self unchanged for `pos >= 256`.
    #[inline(always)]
    pub fn insert(&mut self, pos: USize) {
        let p = pos.0;
        if p >= 256 {
            return;
        }
        let (word_idx, bit_idx) = (p >> 6, p & 63);
        self.0[word_idx] =
            <QWord<Hot> as BitAccess>::with_bit_set(self.0[word_idx], USize(bit_idx));
    }

    /// Clear bit at `pos`. Leaves self unchanged for `pos >= 256`.
    #[inline(always)]
    pub fn remove(&mut self, pos: USize) {
        let p = pos.0;
        if p >= 256 {
            return;
        }
        let (word_idx, bit_idx) = (p >> 6, p & 63);
        self.0[word_idx] =
            <QWord<Hot> as BitAccess>::with_bit_cleared(self.0[word_idx], USize(bit_idx));
    }

    /// Popcount across all four words.
    #[inline(always)]
    pub fn count(self) -> USize {
        let a = self.0;
        let n = qword_to_u64(a[0]).count_ones()
            + qword_to_u64(a[1]).count_ones()
            + qword_to_u64(a[2]).count_ones()
            + qword_to_u64(a[3]).count_ones();
        USize(n as usize)
    }

    /// Lowest set bit index, lowest-word-first. Returns 256 if the
    /// mask is empty.
    #[inline(always)]
    pub fn lowest_set(self) -> USize {
        let a = self.0;
        let w0 = qword_to_u64(a[0]);
        if w0 != 0 {
            return USize(w0.trailing_zeros() as usize);
        }
        let w1 = qword_to_u64(a[1]);
        if w1 != 0 {
            return USize(64 + w1.trailing_zeros() as usize);
        }
        let w2 = qword_to_u64(a[2]);
        if w2 != 0 {
            return USize(128 + w2.trailing_zeros() as usize);
        }
        let w3 = qword_to_u64(a[3]);
        if w3 != 0 {
            return USize(192 + w3.trailing_zeros() as usize);
        }
        USize(256)
    }

    /// Highest set bit index, highest-word-first. Returns 256 if the
    /// mask is empty.
    #[inline(always)]
    pub fn highest_set(self) -> USize {
        let a = self.0;
        let w3 = qword_to_u64(a[3]);
        if w3 != 0 {
            return USize(192 + 63 - w3.leading_zeros() as usize);
        }
        let w2 = qword_to_u64(a[2]);
        if w2 != 0 {
            return USize(128 + 63 - w2.leading_zeros() as usize);
        }
        let w1 = qword_to_u64(a[1]);
        if w1 != 0 {
            return USize(64 + 63 - w1.leading_zeros() as usize);
        }
        let w0 = qword_to_u64(a[0]);
        if w0 != 0 {
            return USize(63 - w0.leading_zeros() as usize);
        }
        USize(256)
    }

    /// Iterator over set bit indices, lowest-first across all four
    /// words.
    #[inline(always)]
    pub fn iter_set_bits(self) -> SetBitsIter256 {
        let a = self.0;
        SetBitsIter256 {
            words: [
                qword_to_u64(a[0]),
                qword_to_u64(a[1]),
                qword_to_u64(a[2]),
                qword_to_u64(a[3]),
            ],
            word_idx: 0,
        }
    }
}

/// Nameable iterator over set bits of a `Mask256`.
///
/// Advances word-by-word, using the `trailing_zeros` + clear-lowest
/// pattern within each non-zero word. Word offsets add 64 * index.
#[derive(Copy, Clone)]
pub struct SetBitsIter256 {
    words: [u64; 4],
    word_idx: usize,
}

impl Iterator for SetBitsIter256 {
    type Item = USize;

    #[inline(always)]
    fn next(&mut self) -> Option<USize> {
        while self.word_idx < 4 {
            let w = self.words[self.word_idx];
            if w != 0 {
                let bit = w.trailing_zeros() as usize;
                self.words[self.word_idx] = w & (w - 1);
                return Some(USize(self.word_idx * 64 + bit));
            }
            self.word_idx += 1;
        }
        None
    }
}

// `Mask64` alias re-use so the public surface name works from lib.
#[allow(dead_code)]
type _Mask64Alias = Mask64;
