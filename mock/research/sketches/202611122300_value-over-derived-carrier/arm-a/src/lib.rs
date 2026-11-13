//! Arm A. No feature, no `unsafe`.
//!
//! The carrier is a second type parameter of the value type, defaulted to the
//! map's projection, and the bound on the type requires the two to agree. The
//! operations are inherent `const fn` in one impl block per ladder carrier, all
//! expanded from one macro body, so a `const fn` body names a concrete machine
//! integer and calls no trait method.

#![no_std]
#![forbid(unsafe_code)]

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;

use common::{Lane, Machine, Sole, rung};

/// A value of the declared signature `S`, held in the ladder carrier `C`.
///
/// `C` is the map's carrier for `S` and nothing else: the bound refuses any other.
pub struct Num<S: Sole<Repr = C>, C = <S as Sole>::Repr>(C, PhantomData<fn() -> S>);

macro_rules! ops {
    ($($c:ty, $add:path, $adapt:path;)+) => {$(
        impl<S: Sole<Repr = $c>> Num<S, $c> {
            /// The member the bits name, adapted onto the format.
            #[must_use]
            pub const fn from_raw(raw: $c) -> Self {
                let () = S::ADMITTED;
                Self($adapt(raw, S::WIDTH, S::POLICY), PhantomData)
            }

            /// The bits, an unwrap door.
            #[must_use]
            pub const fn raw(self) -> $c {
                self.0
            }

            /// The sum under the signature. Total.
            #[must_use]
            pub const fn add(self, rhs: Self) -> Self {
                Self($add(self.0, rhs.0, S::WIDTH, S::POLICY), PhantomData)
            }
        }
    )+};
}

ops! {
    i8, rung::add_s8, rung::adapt_s8;
    u8, rung::add_u8, rung::adapt_u8;
    i16, rung::add_s16, rung::adapt_s16;
    u16, rung::add_u16, rung::adapt_u16;
    i32, rung::add_s32, rung::adapt_s32;
    u32, rung::add_u32, rung::adapt_u32;
    i64, rung::add_s64, rung::adapt_s64;
    u64, rung::add_u64, rung::adapt_u64;
}

impl<S: Sole<Repr = C>, C: Copy> Clone for Num<S, C> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<S: Sole<Repr = C>, C: Copy> Copy for Num<S, C> {}
impl<S: Sole<Repr = C>, C: PartialEq> PartialEq for Num<S, C> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<S: Sole<Repr = C>, C: Eq> Eq for Num<S, C> {}
impl<S: Sole<Repr = C>, C: PartialOrd> PartialOrd for Num<S, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<S: Sole<Repr = C>, C: Ord> Ord for Num<S, C> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
impl<S: Sole<Repr = C>, C: Hash> Hash for Num<S, C> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl<S: Sole<Repr = C>, C: fmt::Debug> fmt::Debug for Num<S, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<S, C> Lane for Num<S, C>
where
    S: Sole<Repr = C> + 'static,
    C: Machine<Wide = S::Wide> + Send + Sync + 'static,
{
    type Wide = S::Wide;

    const WIDE_ZERO: S::Wide = C::WIDE_ZERO;

    fn widen(self) -> S::Wide {
        self.0.widen()
    }

    fn wide_add(a: S::Wide, b: S::Wide) -> S::Wide {
        C::wide_add(a, b)
    }

    fn wide_sub(a: S::Wide, b: S::Wide) -> S::Wide {
        C::wide_sub(a, b)
    }

    fn narrow(wide: S::Wide) -> Self {
        Self(C::narrow(wide, S::WIDTH, S::POLICY), PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::Num;

    common::value_tests!(Num);
}
