//! What every arm shares: the map from a slot family to its ladder carrier, the
//! arithmetic written once per rung, and the `Lane` trait.
//!
//! The map is the easy half of the sketch. One list of admitted widths writes the
//! carrier for `Signed<W>` and `Unsigned<W>`, and each row asserts at check time
//! that the carrier's size is what `arvo_placement::derive_sole` computes for the
//! canonical point over that slot family.
//!
//! Nothing here calls the oracle outside `oracle`, which only tests read.

#![no_std]

pub mod consumer;
pub mod lane;
pub mod oracle;
pub mod rung;

use core::fmt::Debug;
use core::hash::Hash;

use arvo_format::overflow::{Overflow, Saturate, Wrap};
use arvo_format::points::{Integer, UFixed};
use arvo_format::quantum::is_constant_family;
use arvo_format::rounding::TowardZero;
use arvo_format::slots::{Signed, Unsigned};
use arvo_format::{
    Adapt,
    Adaptation,
    DeclaredSignature,
    Format,
    Policy,
    Signature,
    Slots,
    Width,
    overflow_of,
};
use arvo_placement::derive_sole;
use arvo_placement::objective::Access;
pub use lane::{Bits, Lane, Machine};

#[doc(hidden)]
pub mod __private {
    pub use arvo_format;
}

/// The trait set a value type `$num<S: Sole>(S::Repr)` implements, read through
/// its carrier. Written by hand because a derive would bound `S` itself.
#[macro_export]
macro_rules! value_traits {
    ($num:ident) => {
        impl<S: $crate::Sole> ::core::clone::Clone for $num<S> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<S: $crate::Sole> ::core::marker::Copy for $num<S> {}
        impl<S: $crate::Sole> ::core::cmp::PartialEq for $num<S> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<S: $crate::Sole> ::core::cmp::Eq for $num<S> {}
        impl<S: $crate::Sole> ::core::cmp::PartialOrd for $num<S> {
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl<S: $crate::Sole> ::core::cmp::Ord for $num<S> {
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }
        impl<S: $crate::Sole> ::core::hash::Hash for $num<S> {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
        impl<S: $crate::Sole> ::core::fmt::Debug for $num<S> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

/// `Lane` for a value type `$num<S: Sole>(S::Repr)`, through the ordinary
/// carrier operations: a `Lane` body is not const, so no arm needs anything of
/// its own here.
#[macro_export]
macro_rules! value_lane {
    ($num:ident) => {
        impl<S: $crate::Sole + 'static> $crate::Lane for $num<S> {
            type Wide = S::Wide;

            const WIDE_ZERO: S::Wide = <S::Repr as $crate::Machine>::WIDE_ZERO;

            fn widen(self) -> S::Wide {
                $crate::Machine::widen(self.0)
            }

            fn wide_add(a: S::Wide, b: S::Wide) -> S::Wide {
                <S::Repr as $crate::Machine>::wide_add(a, b)
            }

            fn wide_sub(a: S::Wide, b: S::Wide) -> S::Wide {
                <S::Repr as $crate::Machine>::wide_sub(a, b)
            }

            fn narrow(wide: S::Wide) -> Self {
                Self(<S::Repr as $crate::Machine>::narrow(
                    wide,
                    S::WIDTH,
                    S::POLICY,
                ))
            }
        }
    };
}

/// A slot family's ladder carrier, and the signed machine integer twice as wide.
pub trait Carried: Slots {
    /// The narrowest ladder carrier that holds the family's declared width.
    type Repr: Machine<Wide = Self::Wide> + Eq + Ord + Hash + Debug + Send + Sync + 'static;

    /// The signed machine integer twice the carrier's width.
    type Wide: Bits + Eq + Ord + Hash + Debug + Send + Sync + 'static;
}

/// The canonical point a slot family is the slot range of.
///
/// Only so the map can ask the placement crate what it derives: `derive_sole`
/// reads a declared signature, and the slot family alone is not one.
pub trait PointOf: Slots {
    type Point: Format;
}

impl<const W: u32> PointOf for Signed<W>
where
    Signed<W>: Slots,
{
    type Point = Integer<W>;
}

impl<const W: u32> PointOf for Unsigned<W>
where
    Unsigned<W>: Slots,
{
    type Point = UFixed<W, 0>;
}

/// How many bits `derive_sole` puts a slot family's canonical point in.
#[must_use]
pub const fn derived_carrier_bits<Sl: PointOf>() -> u32 {
    derive_sole::<Signature<Sl::Point, Adapt<TowardZero, Wrap>>, Access>()
        .carrier
        .count()
}

/// The assertion every row of the map carries: the carrier the row names is the
/// one the placement derives.
#[macro_export]
macro_rules! assert_carrier {
    ($slots:ty, $repr:ty) => {
        const _: () = ::core::assert!(
            ::core::mem::size_of::<$repr>() as u32 * 8 == $crate::derived_carrier_bits::<$slots>(),
            "the carrier the map names is not the one the placement derives"
        );
    };
}

/// The map, from one list per rung.
macro_rules! map {
    ($($s:ty, $u:ty, $wide:ty => [$($w:literal)+];)+) => {
        $($(
            impl Carried for Signed<$w> {
                type Repr = $s;
                type Wide = $wide;
            }
            impl Carried for Unsigned<$w> {
                type Repr = $u;
                type Wide = $wide;
            }
            assert_carrier!(Signed<$w>, $s);
            assert_carrier!(Unsigned<$w>, $u);
        )+)+
    };
}

map! {
    i8, u8, i16 => [1 2 3 4 5 6 7 8];
    i16, u16, i32 => [9 10 11 12 13 14 15 16];
    i32, u32, i64 => [17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32];
    i64, u64, i128 => [
        33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48
        49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64
    ];
}

/// The overflow policies a value signature admits. `Clamp` is not one.
pub trait ValuePolicy: Overflow {}
impl ValuePolicy for Wrap {}
impl ValuePolicy for Saturate {}

/// A declared signature a value can be held for: its slot family is on the map
/// and its overflow policy is one a value admits.
///
/// Implemented once, over every such signature, so nothing outside can add one.
pub trait Sole: DeclaredSignature {
    /// The ladder carrier, from the map.
    type Repr: Machine<Wide = Self::Wide> + Eq + Ord + Hash + Debug + Send + Sync + 'static;
    /// The double-width accumulator.
    type Wide: Bits + Eq + Ord + Hash + Debug + Send + Sync + 'static;
    /// The declared width.
    const WIDTH: Width;
    /// The overflow policy.
    const POLICY: Policy;
    /// The carrier arithmetic equals the format's own addition only where the sum
    /// of two members is on the grid: a constant quantum and no phase. Forced by
    /// every constructor, so no value of another format exists.
    const ADMITTED: ();
}

type SlotsOf<S> = <<S as DeclaredSignature>::Format as Format>::Slots;

impl<S> Sole for S
where
    S: DeclaredSignature,
    SlotsOf<S>: Carried,
    <S::Adaptation as Adaptation>::Overflow: ValuePolicy,
{
    type Repr = <SlotsOf<S> as Carried>::Repr;
    type Wide = <SlotsOf<S> as Carried>::Wide;

    const ADMITTED: () = assert!(
        <S::Format as Format>::PHASE.is_zero().get()
            && is_constant_family::<<S::Format as Format>::Quantum>().get(),
        "a value is held only for a format whose sums of two members are on its grid"
    );
    const POLICY: Policy = overflow_of::<S::Adaptation>();
    const WIDTH: Width = <SlotsOf<S> as Slots>::WIDTH;
}
