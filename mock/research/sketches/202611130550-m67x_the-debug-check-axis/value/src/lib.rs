//! A four-bit unsigned value whose sum wraps, with the debug abort on a sum
//! that leaves the range chosen two ways.

#![no_std]

use core::marker::PhantomData;

/// Whether a debug build aborts when a result leaves the range.
pub trait DebugCheck {
    /// True where it aborts.
    const ABORTS: bool;
}

/// Aborts in a debug build. The default.
pub struct Abort;

/// Never aborts; the completion is what the operation means.
pub struct Quiet;

impl DebugCheck for Abort {
    const ABORTS: bool = true;
}

impl DebugCheck for Quiet {
    const ABORTS: bool = false;
}

/// Arm one: the choice is a cargo feature of this crate.
pub mod by_feature {
    /// Four bits in a byte.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct U4(u8);

    impl U4 {
        /// The member at `x`, reduced to four bits.
        pub const fn new(x: u8) -> Self {
            Self(x & 0xF)
        }

        /// The member.
        pub const fn get(self) -> u8 {
            self.0
        }

        /// The wrapping sum, aborting in a debug build unless the feature is on.
        pub const fn add(self, rhs: Self) -> Self {
            let sum = self.0 + rhs.0;
            if cfg!(debug_assertions) && !cfg!(feature = "quiet") {
                assert!(sum <= 0xF, "the sum leaves the range");
            }
            Self(sum & 0xF)
        }
    }
}

/// Arm two: the choice is a type parameter with a default.
pub mod by_parameter {
    use super::{Abort, DebugCheck, PhantomData};

    /// Four bits in a byte, with the debug check in the type.
    pub struct U4<C: DebugCheck = Abort>(u8, PhantomData<C>);

    impl<C: DebugCheck> Clone for U4<C> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<C: DebugCheck> Copy for U4<C> {}

    impl<C: DebugCheck> U4<C> {
        /// The member at `x`, reduced to four bits.
        pub const fn new(x: u8) -> Self {
            Self(x & 0xF, PhantomData)
        }

        /// The member.
        pub const fn get(self) -> u8 {
            self.0
        }

        /// The wrapping sum, aborting in a debug build where `C` says so.
        pub const fn add(self, rhs: Self) -> Self {
            let sum = self.0 + rhs.0;
            if cfg!(debug_assertions) && C::ABORTS {
                assert!(sum <= 0xF, "the sum leaves the range");
            }
            Self(sum & 0xF, PhantomData)
        }
    }
}
