//! Probe: `MaskOps` reshape drops the const-Width parameter.
//!
//! Standalone probe that mirrors the chassis layout from arvo-bitmask
//! and the trait from arvo-mask-contracts. Demonstrates that path (a)
//! (drop the `const W: Width` parameter, blanket impl on `Mask<W>` for
//! any `W: [const] BitTraits + Copy + Default`) compiles under rustc
//! 1.96.0-nightly.
//!
//! Run:
//!   rustc +nightly --edition 2024 --crate-type rlib \
//!       01_reshape_no_param.rs -o /tmp/maskops_probe.rlib
//!
//! Outcome: compiles cleanly. Path (a) is feasible.

#![no_std]
#![feature(const_trait_impl)]
#![allow(dead_code)]

// --- stubs mirroring the substrate surface --------------------------

#[derive(Copy, Clone, Default)]
pub struct USize(pub usize);

#[derive(Copy, Clone, Default)]
pub struct Bool(pub bool);

pub const trait BitOps: Copy + Default {
    fn empty() -> Self;
    fn full() -> Self;
    fn set_bit(self, idx: USize) -> Self;
    fn clear_bit(self, idx: USize) -> Self;
    fn get_bit(self, idx: USize) -> Bool;
    fn count(self) -> USize;
    fn or(self, other: Self) -> Self;
    fn and(self, other: Self) -> Self;
    fn xor(self, other: Self) -> Self;
    fn not(self) -> Self;
    fn mask_low(n: USize) -> Self;
}

// --- chassis stub ---------------------------------------------------

#[derive(Copy, Clone, Default)]
pub struct Mask<W: Copy + Default> {
    word: W,
}

impl<W: Copy + Default> Mask<W> {
    #[inline(always)]
    pub const fn from_word(word: W) -> Self {
        Self { word }
    }
}

// --- reshaped MaskOps (path a: no const-Width parameter) -----------

pub const trait MaskOps: Sized + Copy {
    fn empty() -> Self;
    fn full() -> Self;
    fn set(self, idx: USize) -> Self;
    fn clear(self, idx: USize) -> Self;
    fn test(self, idx: USize) -> Bool;
    fn count(self) -> USize;
    fn union(self, other: Self) -> Self;
    fn intersection(self, other: Self) -> Self;
    fn difference(self, other: Self) -> Self;
    fn complement(self) -> Self;
    fn mask_for_width(n: USize) -> Self;
}

// --- blanket impl on Mask<W> for any W: [const] BitOps -------------

impl<W: Copy + Default + [const] BitOps> const MaskOps for Mask<W> {
    #[inline(always)]
    fn empty() -> Self {
        Self::from_word(W::empty())
    }
    #[inline(always)]
    fn full() -> Self {
        Self::from_word(W::full())
    }
    #[inline(always)]
    fn set(self, idx: USize) -> Self {
        Self::from_word(self.word.set_bit(idx))
    }
    #[inline(always)]
    fn clear(self, idx: USize) -> Self {
        Self::from_word(self.word.clear_bit(idx))
    }
    #[inline(always)]
    fn test(self, idx: USize) -> Bool {
        self.word.get_bit(idx)
    }
    #[inline(always)]
    fn count(self) -> USize {
        self.word.count()
    }
    #[inline(always)]
    fn union(self, other: Self) -> Self {
        Self::from_word(self.word.or(other.word))
    }
    #[inline(always)]
    fn intersection(self, other: Self) -> Self {
        Self::from_word(self.word.and(other.word))
    }
    #[inline(always)]
    fn difference(self, other: Self) -> Self {
        Self::from_word(self.word.and(other.word.not()))
    }
    #[inline(always)]
    fn complement(self) -> Self {
        Self::from_word(self.word.not())
    }
    #[inline(always)]
    fn mask_for_width(n: USize) -> Self {
        Self::from_word(W::mask_low(n))
    }
}

// --- canary impl on a concrete word so the blanket actually links --

#[derive(Copy, Clone, Default)]
struct StubWord(u64);

impl const BitOps for StubWord {
    fn empty() -> Self { StubWord(0) }
    fn full() -> Self { StubWord(u64::MAX) }
    fn set_bit(self, idx: USize) -> Self { StubWord(self.0 | (1u64 << idx.0)) }
    fn clear_bit(self, idx: USize) -> Self { StubWord(self.0 & !(1u64 << idx.0)) }
    fn get_bit(self, idx: USize) -> Bool { Bool((self.0 >> idx.0) & 1 == 1) }
    fn count(self) -> USize { USize(self.0.count_ones() as usize) }
    fn or(self, other: Self) -> Self { StubWord(self.0 | other.0) }
    fn and(self, other: Self) -> Self { StubWord(self.0 & other.0) }
    fn xor(self, other: Self) -> Self { StubWord(self.0 ^ other.0) }
    fn not(self) -> Self { StubWord(!self.0) }
    fn mask_low(n: USize) -> Self {
        if n.0 >= 64 { StubWord(u64::MAX) }
        else if n.0 == 0 { StubWord(0) }
        else { StubWord((1u64 << n.0) - 1) }
    }
}

// --- const-callable canary; if the blanket is wired right, this -----
// --- evaluates at compile time. ------------------------------------

const _CANARY: () = {
    let m: Mask<StubWord> = <Mask<StubWord> as MaskOps>::empty();
    let m2 = m.set(USize(3));
    assert!(m2.count().0 == 1);
    let m3 = m2.clear(USize(3));
    assert!(m3.count().0 == 0);
    let f = <Mask<StubWord> as MaskOps>::mask_for_width(USize(8));
    assert!(f.count().0 == 8);
};
