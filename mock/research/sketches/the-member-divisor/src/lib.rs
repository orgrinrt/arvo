//! Can a member type with a const door and a divisor door over a second format
//! be refused at build on the pinned toolchain, with no runtime check?
//!
//! The formats are toy traits carrying what arvo-format's verdicts read: radix,
//! a constant exponent, the reduced phase and the slot range. `Member<F>` has a
//! private slot and, here, only its const door, since the adapted door is `adapt`
//! and asks nothing new of the language. The divisor door takes a member of `G`
//! and is admitted where `G` sits on `F`'s grid inside `F`'s range and no member
//! of `G` names the zero.

#![no_std]

use core::marker::PhantomData;

/// What the verdicts read off a format.
pub trait Format {
    const RADIX: u32;
    const EXPONENT: i32;
    /// The phase `n / d` in lowest terms, `d` positive.
    const PHASE: (i128, i128);
    const MIN: i128;
    const MAX: i128;
}

/// A slot of `F`'s range.
pub struct Member<F: Format> {
    slot:   i128,
    format: PhantomData<F>,
}

struct InRange<F, const B: i128>(PhantomData<F>);

impl<F: Format, const B: i128> InRange<F, B> {
    const ADMITTED: () = assert!(
        B >= F::MIN && B <= F::MAX,
        "a member constant outside the range"
    );
}

impl<F: Format> Member<F> {
    pub const fn constant<const B: i128>() -> Self {
        let () = InRange::<F, B>::ADMITTED;
        Self {
            slot:   B,
            format: PhantomData,
        }
    }

    pub const fn slot(&self) -> i128 {
        self.slot
    }
}

/// A slot of `F` that is not the zero.
pub struct Divisor<F: Format> {
    slot:   i128,
    format: PhantomData<F>,
}

/// Whether a slot of `G` names the zero: the phase is whole, `n / 1`, and the
/// slot `-n` is in `G`'s range.
const fn names_the_zero<G: Format>() -> bool {
    let (n, d) = G::PHASE;
    d == 1 && -n >= G::MIN && -n <= G::MAX
}

struct OnTheGrid<F, G>(PhantomData<(F, G)>);

impl<F: Format, G: Format> OnTheGrid<F, G> {
    const ADMITTED: () = {
        assert!(F::RADIX == G::RADIX, "a member divisor from another radix");
        assert!(
            F::EXPONENT == G::EXPONENT,
            "a member divisor from another exponent"
        );
        assert!(
            F::PHASE.0 == G::PHASE.0 && F::PHASE.1 == G::PHASE.1,
            "a member divisor from another phase"
        );
        assert!(
            G::MIN >= F::MIN && G::MAX <= F::MAX,
            "a member divisor from a range wider than the divisor's format"
        );
        assert!(
            !names_the_zero::<G>(),
            "a member divisor over a range that holds the zero"
        );
    };
}

impl<F: Format> Divisor<F> {
    pub const fn member<G: Format>(m: Member<G>) -> Self {
        let () = OnTheGrid::<F, G>::ADMITTED;
        Self {
            slot:   m.slot(),
            format: PhantomData,
        }
    }

    pub const fn slot(&self) -> i128 {
        self.slot
    }
}

/// Byte slots, `Unsigned<8>` at exponent zero.
pub struct Byte;
impl Format for Byte {
    const EXPONENT: i32 = 0;
    const MAX: i128 = 255;
    const MIN: i128 = 0;
    const PHASE: (i128, i128) = (0, 1);
    const RADIX: u32 = 2;
}

/// Slots 1 to 255 on the same grid, in which no member names the zero.
pub struct Positive;
impl Format for Positive {
    const EXPONENT: i32 = 0;
    const MAX: i128 = 255;
    const MIN: i128 = 1;
    const PHASE: (i128, i128) = (0, 1);
    const RADIX: u32 = 2;
}

/// The same range at another exponent.
pub struct Halves;
impl Format for Halves {
    const EXPONENT: i32 = -1;
    const MAX: i128 = 255;
    const MIN: i128 = 1;
    const PHASE: (i128, i128) = (0, 1);
    const RADIX: u32 = 2;
}

/// The byte range at a phase of a half, where no slot names the zero.
pub struct Biased;
impl Format for Biased {
    const EXPONENT: i32 = 0;
    const MAX: i128 = 255;
    const MIN: i128 = 0;
    const PHASE: (i128, i128) = (1, 2);
    const RADIX: u32 = 2;
}

/// Slots 1 to 65535 on the byte's grid, a range past the byte's.
pub struct Wider;
impl Format for Wider {
    const EXPONENT: i32 = 0;
    const MAX: i128 = 65535;
    const MIN: i128 = 1;
    const PHASE: (i128, i128) = (0, 1);
    const RADIX: u32 = 2;
}

/// Slots 1 to 255 at radix ten.
pub struct Decimal;
impl Format for Decimal {
    const EXPONENT: i32 = 0;
    const MAX: i128 = 255;
    const MIN: i128 = 1;
    const PHASE: (i128, i128) = (0, 1);
    const RADIX: u32 = 10;
}

/// The admitted cases, all evaluated at build.
pub const ADMITTED: [i128; 4] = [
    Member::<Byte>::constant::<255>().slot(),
    Divisor::<Byte>::member(Member::<Positive>::constant::<7>()).slot(),
    Divisor::<Positive>::member(Member::<Positive>::constant::<1>()).slot(),
    Divisor::<Biased>::member(Member::<Biased>::constant::<0>()).slot(),
];

#[cfg(feature = "past-the-top")]
pub const PAST_THE_TOP: i128 = Member::<Byte>::constant::<256>().slot();

#[cfg(feature = "zero-in-range")]
pub const ZERO_IN_RANGE: i128 = Divisor::<Byte>::member(Member::<Byte>::constant::<7>()).slot();

#[cfg(feature = "other-exponent")]
pub const OTHER_EXPONENT: i128 = Divisor::<Byte>::member(Member::<Halves>::constant::<7>()).slot();

#[cfg(feature = "other-phase")]
pub const OTHER_PHASE: i128 = Divisor::<Byte>::member(Member::<Biased>::constant::<7>()).slot();

#[cfg(feature = "wider-range")]
pub const WIDER_RANGE: i128 = Divisor::<Byte>::member(Member::<Wider>::constant::<7>()).slot();

#[cfg(feature = "other-radix")]
pub const OTHER_RADIX: i128 = Divisor::<Byte>::member(Member::<Decimal>::constant::<7>()).slot();
