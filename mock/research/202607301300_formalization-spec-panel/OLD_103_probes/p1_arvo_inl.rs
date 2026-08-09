//! Probe 1, crate 2 of 3. The platform crate (D27's `arvo-platform`).
//!
//! It declares `Bool`, the named wrapper over the host primitive, and implements
//! the foundation's contract for it. The coherence question: foreign trait,
//! local type, so this is orphan-legal. That is the whole of what branch B needs
//! from this crate, and it is what D5's own orphan argument could NOT get for
//! `Cardinal` (foreign trait AND foreign type), which is why the count contract
//! had to sit beside `Cons` and the truth contract does not.
#![no_std]
#![feature(const_trait_impl)]
#![allow(dead_code)]

use p1_foundation::{Truth, TruthHolds, TruthSelect};

/// One door, not six. The field is private, and the only exits are the ones the
/// contract names.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Bool(bool);

impl Bool {
    #[inline(always)]
    pub const fn new(b: bool) -> Self {
        Bool(b)
    }
}

const impl Truth for Bool {
    const TRUE: Self = Bool(true);
    const FALSE: Self = Bool(false);
    #[inline(always)]
    fn not(self) -> Self {
        Bool(!self.0)
    }
    #[inline(always)]
    fn and(self, other: Self) -> Self {
        Bool(self.0 & other.0)
    }
    #[inline(always)]
    fn or(self, other: Self) -> Self {
        Bool(self.0 | other.0)
    }
}

const impl TruthHolds for Bool {
    #[inline(always)]
    fn holds(self) -> bool {
        self.0
    }
}

impl TruthSelect for Bool {
    #[inline(always)]
    fn select<R, T: FnOnce() -> R, F: FnOnce() -> R>(self, on_true: T, on_false: F) -> R {
        if self.0 {
            on_true()
        } else {
            on_false()
        }
    }
}
