//! Arm C. `#![feature(const_trait_impl)]` in this crate only.
//!
//! A const carrier trait, implemented for the eight ladder carriers, and the
//! value type's `const fn` bodies call its methods on `S::Repr`. The syntax is
//! the one kaski uses on this nightly (`kaski-engine-contracts/src/lane.rs:17`,
//! `:42`).

#![no_std]
#![feature(const_trait_impl)]

use common::__private::arvo_format::{Policy, Width};
use common::{Sole, rung};

/// A ladder carrier's arithmetic, callable in a const context.
pub const trait Carrier: Copy {
    /// `rung::add_*` at this carrier.
    fn add_in(a: Self, b: Self, width: Width, policy: Policy) -> Self;
    /// `rung::adapt_*` at this carrier.
    fn adapt_in(x: Self, width: Width, policy: Policy) -> Self;
}

macro_rules! carrier {
    ($($t:ty, $add:path, $adapt:path;)+) => {$(
        const impl Carrier for $t {
            fn add_in(a: Self, b: Self, width: Width, policy: Policy) -> Self {
                $add(a, b, width, policy)
            }
            fn adapt_in(x: Self, width: Width, policy: Policy) -> Self {
                $adapt(x, width, policy)
            }
        }
    )+};
}

carrier! {
    i8, rung::add_s8, rung::adapt_s8;
    u8, rung::add_u8, rung::adapt_u8;
    i16, rung::add_s16, rung::adapt_s16;
    u16, rung::add_u16, rung::adapt_u16;
    i32, rung::add_s32, rung::adapt_s32;
    u32, rung::add_u32, rung::adapt_u32;
    i64, rung::add_s64, rung::adapt_s64;
    u64, rung::add_u64, rung::adapt_u64;
}

/// A value of the declared signature `S`, held in the map's carrier.
pub struct Num<S: Sole>(S::Repr);

impl<S: Sole> Num<S> {
    /// The bits, an unwrap door.
    #[must_use]
    pub const fn raw(self) -> S::Repr {
        self.0
    }
}

/// The operations, conditionally const: `[const] Carrier`. The shape the
/// feature's documentation leads with.
#[cfg(not(feature = "always-const"))]
impl<S: Sole> Num<S> {
    /// The member the bits name, adapted onto the format.
    #[must_use]
    pub const fn from_raw(raw: S::Repr) -> Self
    where
        S::Repr: [const] Carrier,
    {
        let () = S::ADMITTED;
        Self(<S::Repr as Carrier>::adapt_in(raw, S::WIDTH, S::POLICY))
    }

    /// The sum under the signature. Total.
    #[must_use]
    pub const fn add(self, rhs: Self) -> Self
    where
        S::Repr: [const] Carrier,
    {
        Self(<S::Repr as Carrier>::add_in(
            self.0,
            rhs.0,
            S::WIDTH,
            S::POLICY,
        ))
    }
}

/// The operations, always const: `const Carrier`. The variant tried after the
/// conditionally-const one was refused in the consumer's `const` item.
#[cfg(feature = "always-const")]
impl<S: Sole> Num<S> {
    /// The member the bits name, adapted onto the format.
    #[must_use]
    pub const fn from_raw(raw: S::Repr) -> Self
    where
        S::Repr: const Carrier,
    {
        let () = S::ADMITTED;
        Self(<S::Repr as Carrier>::adapt_in(raw, S::WIDTH, S::POLICY))
    }

    /// The sum under the signature. Total.
    #[must_use]
    pub const fn add(self, rhs: Self) -> Self
    where
        S::Repr: const Carrier,
    {
        Self(<S::Repr as Carrier>::add_in(
            self.0,
            rhs.0,
            S::WIDTH,
            S::POLICY,
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
