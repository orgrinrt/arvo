//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The machine integers a carrier can be, and the one bit copy between them.

use core::fmt::Debug;
use core::hash::Hash;
use core::mem::size_of;

mod sealed {
    pub trait Sealed {}
}

/// A plain machine integer: every bit pattern of it is a valid value.
///
/// Sealed to the signed and unsigned integers of 8 to 64 bits and the two
/// pointer-width ones, which is what makes the bit copy sound.
pub trait Machine: sealed::Sealed + Copy + Eq + Ord + Hash + Debug + Send + Sync + 'static {
    /// Whether the integer is a signed one. The host door dispatches on it
    /// beside the size.
    #[doc(hidden)]
    const SIGNED: bool;
}

macro_rules! machine {
    ($signed:literal: $($t:ty),+ $(,)?) => {
        $(
            impl sealed::Sealed for $t {}
            impl Machine for $t {
                const SIGNED: bool = $signed;
            }
        )+
    };
}

machine! { true: i8, i16, i32, i64, isize }
machine! { false: u8, u16, u32, u64, usize }

/// The bits of `x` read as a `U`.
///
/// Every operation that dispatches to a rung passes through here twice, and it
/// is the crate's one `unsafe`. Both sides are `Machine`, sealed to plain
/// integers with every bit pattern valid, so a copy between two of equal size
/// is sound, and the size is asserted.
///
/// The guard is an ordinary assertion rather than an inline `const` block: every
/// arm of a dispatch is monomorphised, the dead ones included, and a `const`
/// assertion would fail in those. As an ordinary one it is evaluated only on the
/// arm taken, where it holds, and it folds away.
#[inline]
pub(crate) const fn copy<T: Machine, U: Machine>(x: T) -> U {
    assert!(
        size_of::<T>() == size_of::<U>(),
        "a bit copy between integers of two sizes"
    );
    // SAFETY: `T` and `U` are both `Machine`, plain integers with every bit
    // pattern valid, and of the same size by the assertion above; `x` lives
    // across the read.
    unsafe { core::ptr::read((&raw const x).cast::<U>()) }
}
