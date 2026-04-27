//! Set operations and bit scanning for `Mask64` / `Mask256`.
//!
//! Set ops and scans live here as inherent methods on the two
//! shipping flavours, not as a blanket impl on `Mask<W>`. Same
//! const-expr cycle pattern that drove arvo-bits' bridge-trait design
//! would otherwise bite: two separate predicates on one impl block
//! (containerness + bit-prim) trip `generic_const_exprs` evaluation.
//! Keep it flat on the concrete types.
//!
//! Ops route through arvo-bits contracts: `BitLogic` for whole-word
//! OR / AND / NOT / XOR; `BitSequence` for scan and popcount;
//! `BitAccess` for single-bit read/write. `Mask64` operates on its
//! single `QWord<Hot>`; `Mask256` unrolls across four.

use arvo::{Bool, USize};
use arvo::strategy::Hot;
use arvo::QWord;
use arvo_bits::{BitAccess, BitLogic, BitSequence};

use crate::mask::{Mask, Mask256, Mask64};

// --- Mask64 (== Mask<QWord<Hot>>) -----------------------------------------

impl Mask<QWord<Hot>> {
    /// Union (bitwise OR).
    #[inline(always)]
    pub fn union(self, other: Self) -> Self {
        Self::from_word(<QWord<Hot> as BitLogic>::bitor(self.word, other.word))
    }

    /// Intersection (bitwise AND).
    #[inline(always)]
    pub fn intersection(self, other: Self) -> Self {
        Self::from_word(<QWord<Hot> as BitLogic>::bitand(self.word, other.word))
    }

    /// Difference (`self & !other`).
    #[inline(always)]
    pub fn difference(self, other: Self) -> Self {
        let not_other = <QWord<Hot> as BitLogic>::bitnot(other.word);
        Self::from_word(<QWord<Hot> as BitLogic>::bitand(self.word, not_other))
    }

    /// Complement (bitwise NOT).
    #[inline(always)]
    pub fn complement(self) -> Self {
        Self::from_word(<QWord<Hot> as BitLogic>::bitnot(self.word))
    }

    /// `Bool::TRUE` when every bit is zero.
    #[inline(always)]
    pub fn is_empty(self) -> Bool {
        <QWord<Hot> as BitSequence>::is_zero(self.word)
    }

    /// `Bool::TRUE` when `self` and `other` share any bit.
    #[inline(always)]
    pub fn intersects(self, other: Self) -> Bool {
        let meet = <QWord<Hot> as BitLogic>::bitand(self.word, other.word);
        Bool(!<QWord<Hot> as BitSequence>::is_zero(meet).0)
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
        if <QWord<Hot> as BitSequence>::is_zero(self.word).0 {
            return USize(64);
        }
        let lz = <QWord<Hot> as BitSequence>::leading_zeros(self.word);
        USize(63 - lz.0)
    }

    /// Iterator over set bit indices, lowest-first.
    #[inline(always)]
    pub fn iter_set_bits(self) -> SetBitsIter64 {
        SetBitsIter64 { remaining: self.word }
    }
}

/// Nameable iterator over set bits of a `Mask64`.
///
/// Advance via `BitSequence::trailing_zeros` + `BitLogic::clear_lowest_set_bit`;
/// yields bit indices lowest-first.
#[derive(Copy, Clone)]
pub struct SetBitsIter64 {
    remaining: QWord<Hot>,
}

impl Iterator for SetBitsIter64 {
    type Item = USize;

    #[inline(always)]
    fn next(&mut self) -> Option<USize> { // lint:allow(no-bare-option) reason: core::iter::Iterator::next trait-method signature returns Option<Self::Item>; tracked: #115
        if <QWord<Hot> as BitSequence>::is_zero(self.remaining).0 {
            return None;
        }
        let idx = <QWord<Hot> as BitSequence>::trailing_zeros(self.remaining);
        self.remaining = <QWord<Hot> as BitLogic>::clear_lowest_set_bit(self.remaining);
        Some(idx)
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
            <QWord<Hot> as BitLogic>::bitor(a[0], b[0]),
            <QWord<Hot> as BitLogic>::bitor(a[1], b[1]),
            <QWord<Hot> as BitLogic>::bitor(a[2], b[2]),
            <QWord<Hot> as BitLogic>::bitor(a[3], b[3]),
        ])
    }

    /// Intersection (bitwise AND across all four words).
    #[inline]
    pub fn intersection(self, other: Self) -> Self {
        let a = self.0;
        let b = other.0;
        Self([
            <QWord<Hot> as BitLogic>::bitand(a[0], b[0]),
            <QWord<Hot> as BitLogic>::bitand(a[1], b[1]),
            <QWord<Hot> as BitLogic>::bitand(a[2], b[2]),
            <QWord<Hot> as BitLogic>::bitand(a[3], b[3]),
        ])
    }

    /// Difference (`self & !other` across all four words).
    #[inline]
    pub fn difference(self, other: Self) -> Self {
        let a = self.0;
        let b = other.0;
        Self([
            <QWord<Hot> as BitLogic>::bitand(a[0], <QWord<Hot> as BitLogic>::bitnot(b[0])),
            <QWord<Hot> as BitLogic>::bitand(a[1], <QWord<Hot> as BitLogic>::bitnot(b[1])),
            <QWord<Hot> as BitLogic>::bitand(a[2], <QWord<Hot> as BitLogic>::bitnot(b[2])),
            <QWord<Hot> as BitLogic>::bitand(a[3], <QWord<Hot> as BitLogic>::bitnot(b[3])),
        ])
    }

    /// Complement (bitwise NOT across all four words).
    #[inline]
    pub fn complement(self) -> Self {
        let a = self.0;
        Self([
            <QWord<Hot> as BitLogic>::bitnot(a[0]),
            <QWord<Hot> as BitLogic>::bitnot(a[1]),
            <QWord<Hot> as BitLogic>::bitnot(a[2]),
            <QWord<Hot> as BitLogic>::bitnot(a[3]),
        ])
    }

    /// `Bool::TRUE` when every bit is zero.
    #[inline(always)]
    pub fn is_empty(self) -> Bool {
        let a = self.0;
        Bool(
            <QWord<Hot> as BitSequence>::is_zero(a[0]).0
                && <QWord<Hot> as BitSequence>::is_zero(a[1]).0
                && <QWord<Hot> as BitSequence>::is_zero(a[2]).0
                && <QWord<Hot> as BitSequence>::is_zero(a[3]).0,
        )
    }

    /// `Bool::TRUE` when `self` and `other` share any bit.
    #[inline(always)]
    pub fn intersects(self, other: Self) -> Bool {
        let a = self.0;
        let b = other.0;
        let m0 = <QWord<Hot> as BitLogic>::bitand(a[0], b[0]);
        let m1 = <QWord<Hot> as BitLogic>::bitand(a[1], b[1]);
        let m2 = <QWord<Hot> as BitLogic>::bitand(a[2], b[2]);
        let m3 = <QWord<Hot> as BitLogic>::bitand(a[3], b[3]);
        Bool(
            !<QWord<Hot> as BitSequence>::is_zero(m0).0
                || !<QWord<Hot> as BitSequence>::is_zero(m1).0
                || !<QWord<Hot> as BitSequence>::is_zero(m2).0
                || !<QWord<Hot> as BitSequence>::is_zero(m3).0,
        )
    }

    /// `Bool::TRUE` when bit at `pos` is set. `pos >= 256` returns
    /// `Bool::FALSE`.
    #[inline(always)]
    pub fn contains(self, pos: USize) -> Bool {
        if pos.0 >= 256 {
            return Bool::FALSE;
        }
        let word_idx = USize(pos.0 >> 6);
        let bit_idx = USize(pos.0 & 63);
        <QWord<Hot> as BitAccess>::bit(self.0[word_idx.0], bit_idx)
    }

    /// Set bit at `pos`. Leaves self unchanged for `pos >= 256`.
    #[inline(always)]
    pub fn insert(&mut self, pos: USize) {
        if pos.0 >= 256 {
            return;
        }
        let word_idx = USize(pos.0 >> 6);
        let bit_idx = USize(pos.0 & 63);
        self.0[word_idx.0] =
            <QWord<Hot> as BitAccess>::with_bit_set(self.0[word_idx.0], bit_idx);
    }

    /// Clear bit at `pos`. Leaves self unchanged for `pos >= 256`.
    #[inline(always)]
    pub fn remove(&mut self, pos: USize) {
        if pos.0 >= 256 {
            return;
        }
        let word_idx = USize(pos.0 >> 6);
        let bit_idx = USize(pos.0 & 63);
        self.0[word_idx.0] =
            <QWord<Hot> as BitAccess>::with_bit_cleared(self.0[word_idx.0], bit_idx);
    }

    /// Popcount across all four words.
    #[inline(always)]
    pub fn count(self) -> USize {
        let a = self.0;
        let n0 = <QWord<Hot> as BitSequence>::count_ones(a[0]);
        let n1 = <QWord<Hot> as BitSequence>::count_ones(a[1]);
        let n2 = <QWord<Hot> as BitSequence>::count_ones(a[2]);
        let n3 = <QWord<Hot> as BitSequence>::count_ones(a[3]);
        USize(n0.0 + n1.0 + n2.0 + n3.0)
    }

    /// Lowest set bit index, lowest-word-first. Returns 256 if the
    /// mask is empty.
    #[inline(always)]
    pub fn lowest_set(self) -> USize {
        let a = self.0;
        if !<QWord<Hot> as BitSequence>::is_zero(a[0]).0 {
            return <QWord<Hot> as BitSequence>::trailing_zeros(a[0]);
        }
        if !<QWord<Hot> as BitSequence>::is_zero(a[1]).0 {
            return USize(64 + <QWord<Hot> as BitSequence>::trailing_zeros(a[1]).0);
        }
        if !<QWord<Hot> as BitSequence>::is_zero(a[2]).0 {
            return USize(128 + <QWord<Hot> as BitSequence>::trailing_zeros(a[2]).0);
        }
        if !<QWord<Hot> as BitSequence>::is_zero(a[3]).0 {
            return USize(192 + <QWord<Hot> as BitSequence>::trailing_zeros(a[3]).0);
        }
        USize(256)
    }

    /// Highest set bit index, highest-word-first. Returns 256 if the
    /// mask is empty.
    #[inline(always)]
    pub fn highest_set(self) -> USize {
        let a = self.0;
        if !<QWord<Hot> as BitSequence>::is_zero(a[3]).0 {
            let lz = <QWord<Hot> as BitSequence>::leading_zeros(a[3]);
            return USize(192 + 63 - lz.0);
        }
        if !<QWord<Hot> as BitSequence>::is_zero(a[2]).0 {
            let lz = <QWord<Hot> as BitSequence>::leading_zeros(a[2]);
            return USize(128 + 63 - lz.0);
        }
        if !<QWord<Hot> as BitSequence>::is_zero(a[1]).0 {
            let lz = <QWord<Hot> as BitSequence>::leading_zeros(a[1]);
            return USize(64 + 63 - lz.0);
        }
        if !<QWord<Hot> as BitSequence>::is_zero(a[0]).0 {
            let lz = <QWord<Hot> as BitSequence>::leading_zeros(a[0]);
            return USize(63 - lz.0);
        }
        USize(256)
    }

    /// Iterator over set bit indices, lowest-first across all four
    /// words.
    #[inline(always)]
    pub fn iter_set_bits(self) -> SetBitsIter256 {
        SetBitsIter256 {
            words: self.0,
            word_idx: USize(0),
        }
    }
}

/// Nameable iterator over set bits of a `Mask256`.
///
/// Advances word-by-word, using `BitSequence::trailing_zeros` +
/// `BitLogic::clear_lowest_set_bit` within each non-zero word. Word
/// offsets add 64 * index.
#[derive(Copy, Clone)]
pub struct SetBitsIter256 {
    words: [QWord<Hot>; 4],
    word_idx: USize,
}

impl Iterator for SetBitsIter256 {
    type Item = USize;

    #[inline(always)]
    fn next(&mut self) -> Option<USize> { // lint:allow(no-bare-option) reason: core::iter::Iterator::next trait-method signature returns Option<Self::Item>; tracked: #115
        while self.word_idx.0 < 4 {
            let w = self.words[self.word_idx.0];
            if !<QWord<Hot> as BitSequence>::is_zero(w).0 {
                let bit = <QWord<Hot> as BitSequence>::trailing_zeros(w);
                self.words[self.word_idx.0] = <QWord<Hot> as BitLogic>::clear_lowest_set_bit(w);
                return Some(USize(self.word_idx.0 * 64 + bit.0));
            }
            self.word_idx = USize(self.word_idx.0 + 1);
        }
        None
    }
}

// `Mask64` alias re-use so the public surface name works from lib.
#[allow(dead_code)]
type _Mask64Alias = Mask64;
