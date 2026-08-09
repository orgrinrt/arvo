//! P12. If the ambient strategy is supplied by a scope selection at the top
//! of an item's body, does it reach that item's signature?
//!
//! It decides whether the attribute's arvo half can be one injected scope
//! selection, or whether the attribute must also rewrite the signature the
//! way notko's shipped `#[profile]` rewrites its return type.
//!
//! Expected: it does not reach. The signature resolves at the enclosing
//! scope, so a body selection leaves the caller-visible contract alone. That
//! is a limit, and it may be the correct behaviour rather than a gap.

#![no_std]

pub struct Hot;
pub struct Warm;

pub mod base {
    pub struct UInt<const N: u32, S>(pub u32, core::marker::PhantomData<S>);
    impl<const N: u32, S> UInt<N, S> {
        pub const fn new(v: u32) -> Self {
            Self(v, core::marker::PhantomData)
        }
    }
}

pub mod warm {
    pub type UInt<const N: u32> = crate::base::UInt<N, crate::Warm>;
}
pub mod hot {
    pub type UInt<const N: u32> = crate::base::UInt<N, crate::Hot>;
}

use warm::UInt;

/// The signature uses the elided spelling. A body selection follows.
/// If the body selection reached the signature, the declared return type
/// would be the Hot one and this would compile.
pub fn signature_elided(v: u32) -> UInt<5> {
    use hot::UInt;
    UInt::<5>::new(v)
}
