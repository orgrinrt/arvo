//! Arm B. No feature.
//!
//! The value holds the projected carrier. A `const fn` body cannot call a trait
//! method on it, so it moves the bits to the machine integer of the same size and
//! signedness by a copy inside this crate, computes there with the rung's own
//! function, and moves them back. The carrier set is closed: `Sole::Repr` comes
//! from the map, which only `common` writes and nothing outside can extend (the
//! orphan rule refuses an impl of `Carried` for `arvo_format`'s slot families
//! anywhere else).

#![no_std]

use core::mem::size_of;

use common::{Machine, Sole, rung};

pub mod wide;

pub use wide::{Count, Counted, Succ, Wide, Zero, max_terms};

/// A value of the declared signature `S`, held in the map's carrier.
pub struct Num<S: Sole>(S::Repr);

/// The bits of `x` read as a `U`.
///
/// The only `unsafe` in the arm. Every call is inside the size and signedness
/// dispatch below, which pairs `T` and `U` of the same size, both plain integers
/// with every bit pattern valid.
///
/// The guard cannot be an inline `const` block: every arm of the dispatch is
/// monomorphised, the dead ones included, and a `const` assertion fails in
/// those (`copy::<u64, u32>` at a 32-bit carrier). As an ordinary assertion it
/// is evaluated only on the arm taken, where it holds, and folds away.
const fn copy<T: Copy, U: Copy>(x: T) -> U {
    assert!(size_of::<T>() == size_of::<U>());
    // SAFETY: same size, both integers, every bit pattern valid; `x` lives
    // across the read.
    unsafe { core::ptr::read((&raw const x).cast::<U>()) }
}

/// One operation, dispatched on the carrier's size and signedness to the rung
/// function at that machine integer. The match is on constants of the
/// instantiation, so the backend sees one arm.
macro_rules! dispatch {
    (
        $r:ty,
        [$s8:path, $u8:path, $s16:path, $u16:path, $s32:path, $u32:path, $s64:path, $u64:path],
        [$($op:expr),*], [$($tail:expr),*]
    ) => {
        match (size_of::<$r>(), <$r as Machine>::SIGNED) {
            (1, true) => copy::<i8, $r>($s8($(copy::<$r, i8>($op),)* $($tail),*)),
            (1, false) => copy::<u8, $r>($u8($(copy::<$r, u8>($op),)* $($tail),*)),
            (2, true) => copy::<i16, $r>($s16($(copy::<$r, i16>($op),)* $($tail),*)),
            (2, false) => copy::<u16, $r>($u16($(copy::<$r, u16>($op),)* $($tail),*)),
            (4, true) => copy::<i32, $r>($s32($(copy::<$r, i32>($op),)* $($tail),*)),
            (4, false) => copy::<u32, $r>($u32($(copy::<$r, u32>($op),)* $($tail),*)),
            (_, true) => copy::<i64, $r>($s64($(copy::<$r, i64>($op),)* $($tail),*)),
            (_, false) => copy::<u64, $r>($u64($(copy::<$r, u64>($op),)* $($tail),*)),
        }
    };
}

impl<S: Sole> Num<S> {
    /// The member the bits name, adapted onto the format.
    #[must_use]
    pub const fn from_raw(raw: S::Repr) -> Self {
        let () = S::ADMITTED;
        Self(dispatch!(
            S::Repr,
            [
                rung::adapt_s8,
                rung::adapt_u8,
                rung::adapt_s16,
                rung::adapt_u16,
                rung::adapt_s32,
                rung::adapt_u32,
                rung::adapt_s64,
                rung::adapt_u64
            ],
            [raw],
            [S::WIDTH, S::POLICY]
        ))
    }

    /// The bits, an unwrap door.
    #[must_use]
    pub const fn raw(self) -> S::Repr {
        self.0
    }

    /// The sum under the signature. Total.
    #[must_use]
    pub const fn add(self, rhs: Self) -> Self {
        Self(dispatch!(
            S::Repr,
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
            [S::WIDTH, S::POLICY]
        ))
    }
}

common::value_traits!(Num);
common::value_lane!(Num);

#[cfg(test)]
mod tests {
    use super::Num;

    common::value_tests!(Num);
}
