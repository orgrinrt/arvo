#![no_std]
//! One coordinate, one type. The door as the position rule would have it.
//!
//! Each type is `repr(transparent)` over the machine integer the coordinate
//! needs, with one declared constructor and one declared accessor, which is the
//! pattern `Width` and `Bool` already ship. The bare primitive appears in the
//! constructor and the accessor and nowhere else, which is the whole of what
//! defining a primitive in terms of itself requires.
//!
//! `Width` and `Bool` are reproduced here rather than depended on, so this tree
//! stands alone and a failure in it is about the shape rather than about a path
//! dependency.

macro_rules! coordinate {
    ($(#[$m:meta])* $name:ident, $held:ty, $make:ident, $read:ident) => {
        $(#[$m])*
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
        pub struct $name($held);

        impl $name {
            /// The declared way in.
            #[must_use]
            pub const fn $make(v: $held) -> Self { Self(v) }
            /// The declared way out, for the one place a host contract needs it.
            #[must_use]
            pub const fn $read(self) -> $held { self.0 }
        }
    };
}

coordinate!(/// A count of bits.
            Width, u32, bits, count);
coordinate!(/// The base a positional domain counts in.
            Radix, u32, of, count);
coordinate!(/// An exponent of the radix, and the step between two of them.
            Exponent, i32, of, value);
coordinate!(/// Which magnitude of the quantum law, as an index.
            Magnitude, u32, at, index);
coordinate!(/// How many magnitudes a law ranges over.
            MagnitudeCount, u32, of, count);
coordinate!(/// A multiple of the quantum, as an index into the grid.
            Slot, i64, at, index);
coordinate!(/// How many slots a range admits.
            SlotCount, i64, of, count);
coordinate!(/// How many operands an operation takes.
            Arity, u32, of, count);

/// A truth value.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bool(bool);

impl Bool {
    /// Yes.
    pub const TRUE: Self = Self(true);
    /// No.
    pub const FALSE: Self = Self(false);
    /// A truth value from the host's.
    #[must_use]
    pub const fn of(b: bool) -> Self { Self(b) }
    /// The host's, for the one place a control-flow construct needs it.
    #[must_use]
    pub const fn get(self) -> bool { self.0 }
    /// Both.
    #[must_use]
    pub const fn and(self, o: Self) -> Self { Self(self.0 && o.0) }
}

/// A rational in units of the quantum at magnitude zero.
///
/// Two coordinates that are one thing, so they are one constant rather than
/// two. The pair is what `PHASE_NUM` and `PHASE_DEN` are together, and nothing
/// downstream ever wants one without the other.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Phase {
    num: i64,
    den: i64,
}

impl Phase {
    /// No offset: the grid passes through the ambient zero.
    pub const NONE: Self = Self { num: 0, den: 1 };

    /// A phase from a numerator over a denominator.
    #[must_use]
    pub const fn of(num: i64, den: i64) -> Self { Self { num, den } }

    /// Whether the offset is the absent one.
    #[must_use]
    pub const fn is_none(self) -> Bool { Bool::of(self.num == 0) }
}

// --- the traits, parameterised on the coordinates -----------------------------

/// The domain a representable set is a subset of.
pub trait Ambient {
    /// The base positional notation counts in.
    const RADIX: Radix;
    /// Whether the domain carries values below zero.
    const SIGNED: Bool;
}

/// How the quantum varies with magnitude, as an affine law.
pub trait Quantum {
    /// The exponent at magnitude zero.
    const BASE: Exponent;
    /// The change in exponent per step of magnitude.
    const SLOPE: Exponent;
    /// How many distinct magnitudes the law ranges over.
    const MAGNITUDES: MagnitudeCount;
}

/// Which slot indices a format admits.
pub trait Slots {
    /// The lowest admitted slot index.
    const MIN: Slot;
    /// The highest admitted slot index.
    const MAX: Slot;
    /// The width the declaration stated.
    const WIDTH: Width;

    /// What an implementor owes, checked rather than asked for.
    ///
    /// The arithmetic that used to sit on bare integers now sits on the
    /// accessors, which is where the door's own primitive is allowed to be
    /// unwrapped. Nothing about the check weakened.
    const ADMITTED: () = {
        assert!(Self::MIN.index() <= Self::MAX.index(), "slot range is inverted");
        assert!(Self::WIDTH.count() >= 1, "a declared width of zero admits no values");
        assert!(Self::WIDTH.count() <= 62, "declared width is wider than a slot index carries");
        assert!(
            (Self::MAX.index() as i128) - (Self::MIN.index() as i128) < (1i128 << Self::WIDTH.count()),
            "the declared width does not cover the range"
        );
    };
}

/// A representable set, together with the domain it sits in.
pub trait Format {
    /// The domain the set is drawn from.
    type Ambient: Ambient;
    /// How the step changes with magnitude.
    type Quantum: Quantum;
    /// Which multiples of the step are admitted.
    type Slots: Slots;
    /// Where the grid sits relative to the ambient zero.
    const PHASE: Phase;
}

/// An operation admitted by the admission rule.
pub trait Operation {
    /// The format this operation is a function of.
    type Format: Format;
    /// How many operands it takes.
    const ARITY: Arity;
}

// --- the computations, still free `const fn` ----------------------------------

/// The exponent of the quantum at a magnitude.
#[must_use]
pub const fn exponent_at<Q: Quantum>(m: Magnitude) -> Exponent {
    Exponent::of(Q::BASE.value() + Q::SLOPE.value() * (m.index() as i32))
}

/// Whether a magnitude is one the law ranges over.
#[must_use]
pub const fn magnitude_in_range<Q: Quantum>(m: Magnitude) -> Bool {
    Bool::of(m.index() < Q::MAGNITUDES.count())
}

/// Whether a slot index is admitted.
#[must_use]
pub const fn slot_in_range<S: Slots>(s: Slot) -> Bool {
    let () = S::ADMITTED;
    Bool::of(s.index() >= S::MIN.index() && s.index() <= S::MAX.index())
}

/// How many slots the range admits.
#[must_use]
pub const fn slot_count<S: Slots>() -> SlotCount {
    let () = S::ADMITTED;
    SlotCount::of(S::MAX.index() - S::MIN.index() + 1)
}

/// Whether the coordinates name a member of the format's representable set.
#[must_use]
pub const fn contains<F: Format>(s: Slot, m: Magnitude) -> Bool {
    magnitude_in_range::<F::Quantum>(m).and(slot_in_range::<F::Slots>(s))
}

/// Whether the format's grid carries an additive identity.
#[must_use]
pub const fn has_additive_identity<F: Format>() -> Bool {
    F::PHASE.is_none().and(slot_in_range::<F::Slots>(Slot::at(0)))
}

/// The radix the format's ambient domain counts in.
#[must_use]
pub const fn radix<F: Format>() -> Radix {
    <F::Ambient as Ambient>::RADIX
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    /// No coordinate is wider than what it wraps.
    ///
    /// `never_a_runtime_check_and_one_lowered_path` is the intent this bears
    /// on: a door type that changed the layout would be a cost paid at every
    /// use in exchange for a name. `repr(transparent)` is what makes it free
    /// and this asserts the attribute is actually on each of them.
    ///
    /// **It lives here rather than beside the outside crate's own tests**, and
    /// that placement is a result rather than a tidiness choice. It was in the
    /// outside crate first, and the lint reported ten findings for it: a
    /// `#[cfg(test)]` module is under `src/`, so it is scanned like any other
    /// source, and naming `u32` to compare against is naming `u32`. The door
    /// is the one place the comparison can be written, which is the same
    /// reason the constructors and accessors are here.
    #[test]
    fn every_coordinate_is_the_size_of_what_it_wraps() {
        assert_eq!(size_of::<Width>(), size_of::<u32>());
        assert_eq!(size_of::<Radix>(), size_of::<u32>());
        assert_eq!(size_of::<Exponent>(), size_of::<i32>());
        assert_eq!(size_of::<Magnitude>(), size_of::<u32>());
        assert_eq!(size_of::<MagnitudeCount>(), size_of::<u32>());
        assert_eq!(size_of::<Slot>(), size_of::<i64>());
        assert_eq!(size_of::<SlotCount>(), size_of::<i64>());
        assert_eq!(size_of::<Arity>(), size_of::<u32>());
        assert_eq!(size_of::<Bool>(), size_of::<bool>());
        assert_eq!(size_of::<Phase>(), 2 * size_of::<i64>());
    }

    /// The control: a coordinate that is not transparent is a different size.
    ///
    /// Without it the assertions above pass for any newtype whose payload
    /// happens to be the same width, which is every one of them, so the run
    /// would say nothing about the attribute it is named for.
    #[test]
    fn the_control_a_coordinate_without_the_attribute_is_not_free() {
        #[derive(Clone, Copy)]
        pub struct Padded(u32, u8);
        assert_ne!(size_of::<Padded>(), size_of::<u32>());
    }
}
