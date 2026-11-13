//! Arm D. No feature.
//!
//! One concrete value type per ladder carrier, each holding its machine integer,
//! and `Num<S>` an alias through the map's projection to the one the signature
//! lands on. The fall-back the brief names: the consumer naming the rung.

#![no_std]

use core::marker::PhantomData;

use common::{Sole, rung};

/// The value type a ladder carrier holds a value in.
pub trait ValueOf {
    /// The concrete value type, at a signature carried in `Self`.
    type Value<S: Sole<Repr = Self>>;
}

/// A value of the declared signature `S`, through the projection.
pub type Num<S> = <<S as Sole>::Repr as ValueOf>::Value<S>;

macro_rules! values {
    ($($name:ident, $c:ty, $add:path, $adapt:path;)+) => {$(
        /// A value whose signature the map puts in this carrier.
        pub struct $name<S: Sole<Repr = $c>>($c, PhantomData<fn() -> S>);

        impl ValueOf for $c {
            type Value<S: Sole<Repr = $c>> = $name<S>;
        }

        impl<S: Sole<Repr = $c>> $name<S> {
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

        impl<S: Sole<Repr = $c>> Clone for $name<S> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<S: Sole<Repr = $c>> Copy for $name<S> {}
        impl<S: Sole<Repr = $c>> PartialEq for $name<S> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<S: Sole<Repr = $c>> Eq for $name<S> {}
        impl<S: Sole<Repr = $c>> core::fmt::Debug for $name<S> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        // `S::Wide` is not used: rustc does not normalise it to the carrier's
        // wide type through `Repr: Machine<Wide = Self::Wide>` with `Repr`
        // fixed (E0308, "expected `i128`, found associated type").
        impl<S: Sole<Repr = $c> + 'static> common::Lane for $name<S> {
            type Wide = <$c as common::Machine>::Wide;

            const WIDE_ZERO: Self::Wide = <$c as common::Machine>::WIDE_ZERO;

            fn widen(self) -> Self::Wide {
                common::Machine::widen(self.0)
            }

            fn wide_add(a: Self::Wide, b: Self::Wide) -> Self::Wide {
                <$c as common::Machine>::wide_add(a, b)
            }

            fn wide_sub(a: Self::Wide, b: Self::Wide) -> Self::Wide {
                <$c as common::Machine>::wide_sub(a, b)
            }

            fn narrow(wide: Self::Wide) -> Self {
                Self(<$c as common::Machine>::narrow(wide, S::WIDTH, S::POLICY), PhantomData)
            }
        }
    )+};
}

values! {
    NumI8, i8, rung::add_s8, rung::adapt_s8;
    NumU8, u8, rung::add_u8, rung::adapt_u8;
    NumI16, i16, rung::add_s16, rung::adapt_s16;
    NumU16, u16, rung::add_u16, rung::adapt_u16;
    NumI32, i32, rung::add_s32, rung::adapt_s32;
    NumU32, u32, rung::add_u32, rung::adapt_u32;
    NumI64, i64, rung::add_s64, rung::adapt_s64;
    NumU64, u64, rung::add_u64, rung::adapt_u64;
}

#[cfg(test)]
mod tests {
    use super::Num;

    common::value_tests!(Num);
}
