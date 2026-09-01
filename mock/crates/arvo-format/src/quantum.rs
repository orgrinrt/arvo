//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The quantum law: how the step between neighbouring values changes with magnitude.
//!
//! The canon says membership is one affine predicate, so the law is affine in the
//! magnitude and carries two coordinates rather than a function. A slope of zero
//! is the constant-quantum family, where integers, fixed point and scaled integers
//! live. A nonzero slope is the magnitude-indexed family, where floating point
//! lives and where subnormals fall out of the smallest magnitude rather than out
//! of a special case.
//!
//! Carrying it as coordinates instead of a method is what lets membership be a
//! free `const fn` over a format's associated items. A trait method would have to
//! be const to be reachable at stage zero, and that needs a feature this crate is
//! not permitted.
//!
//! The three coordinates below carry types of their own, and an index into the
//! magnitudes is not the same type as a count of them. That distinction is the one
//! this crate already refuses to leave to a reader's care in the slot range, and
//! carrying both as one host integer is the same hazard with nothing done about it.
//!
//! **What an implementor owes is checked rather than asked for**, on the same
//! reading the slot range carries: a law over no magnitudes and a law whose
//! exponent runs off the end of what an exponent carries are both refused, and the
//! conditions are written once and read twice, as an obligation and as a verdict.

use crate::width::Bool;

/// A signed power of the ambient domain's radix.
///
/// Both coordinates of the step law are one of these. The intercept is the
/// exponent at magnitude zero and the rate is how much that exponent moves per
/// magnitude, and they sit on the same additive axis, which is why the law adds
/// them rather than combining two kinds.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Exponent(i32);

impl Exponent {
    /// The zeroth power, which is the integers' step and a flat law's rate.
    pub const ZERO: Self = Self(0);

    /// One power, which is the rate the magnitude-indexed family runs at.
    pub const ONE: Self = Self(1);

    /// An exponent from a power.
    #[must_use]
    pub const fn of(power: i32) -> Self {
        Self(power)
    }

    /// The power, for the one place a host contract needs it back.
    ///
    /// The unwrap door, declared as one.
    #[must_use]
    pub const fn power(self) -> i32 {
        self.0
    }

    /// This exponent advanced by a rate over a magnitude.
    ///
    /// The step law's whole arithmetic, in one place, so a caller never
    /// reconstructs `base + slope * magnitude` out of three unwrapped host
    /// integers and gets the multiplication the wrong way round.
    ///
    /// **It computes in the exponent's own width and the index is cast down into
    /// it**, so both narrow, which is what `Quantum::ADMITTED` exists to make
    /// unreachable. A law admitted by that obligation cannot reach a magnitude
    /// where either narrowing bites, so there is no check here and nothing to
    /// catch at runtime.
    #[must_use]
    pub const fn advanced(self, rate: Self, magnitude: Magnitude) -> Self {
        Self(self.0 + rate.0 * (magnitude.0 as i32))
    }

    /// The smaller of two exponents.
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }
}

/// An index into a quantum law's magnitudes.
///
/// An index and not a count. The value it names is the position on the magnitude
/// axis the step law is being asked about, and a law over `n` magnitudes has
/// indices `0` through `n - 1`.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Magnitude(u32);

impl Magnitude {
    /// The smallest magnitude, which is where a constant law's only step sits and
    /// where the floating conventions' subnormals fall out.
    pub const SMALLEST: Self = Self(0);

    /// A magnitude at an index.
    #[must_use]
    pub const fn at(index: u32) -> Self {
        Self(index)
    }

    /// The index, for the one place a host contract needs it back.
    ///
    /// The unwrap door, declared as one.
    #[must_use]
    pub const fn index(self) -> u32 {
        self.0
    }

    /// Whether this index is one a law of that many magnitudes ranges over.
    ///
    /// The index-against-extent comparison, written once here rather than at every
    /// site that would otherwise have to remember which side the equality falls on.
    #[must_use]
    pub const fn is_within(self, count: MagnitudeCount) -> Bool {
        Bool::of(self.0 < count.0)
    }
}

/// How many magnitudes a quantum law ranges over.
///
/// An extent and not an index, which is the whole reason it is not a `Magnitude`.
/// One for the constant family, and never zero: a law over no magnitudes describes
/// no values. That condition is an obligation on `Quantum` rather than a request
/// made here, so it is refused rather than written down and hoped for.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct MagnitudeCount(u32);

impl MagnitudeCount {
    /// One magnitude, which is the constant family's whole axis.
    pub const ONE: Self = Self(1);

    /// A count of magnitudes.
    #[must_use]
    pub const fn of(count: u32) -> Self {
        Self(count)
    }

    /// The count, for the one place a host contract needs it back.
    ///
    /// The unwrap door, declared as one.
    #[must_use]
    pub const fn count(self) -> u32 {
        self.0
    }

    /// The largest index this extent admits.
    ///
    /// Saturating at the smallest magnitude, so an extent of zero names an index
    /// rather than wrapping to the largest one there is. An extent of zero admits
    /// nothing and `is_within` says so, which is where that case is answered.
    #[must_use]
    pub const fn largest(self) -> Magnitude {
        if self.0 == 0 {
            Magnitude::SMALLEST
        } else {
            Magnitude(self.0 - 1)
        }
    }
}

/// Whether a law over this many magnitudes describes any values at all.
///
/// One of the two conditions `Quantum::ADMITTED` refuses and `is_admissible`
/// reports. Written over the coordinate rather than over the type, so the
/// obligation can call it without the trait needing a `Sized` bound it has no
/// other reason to carry.
const fn ranges_over_a_magnitude(magnitudes: MagnitudeCount) -> bool {
    magnitudes.count() >= 1
}

/// Whether the law's exponent at its largest admitted magnitude is one an
/// `Exponent` carries.
///
/// The other condition, and it is the slot range's span condition one contract
/// over: a rate and a count that between them run the exponent past what it holds
/// describe no quantum at the magnitudes they claim.
///
/// **Computed one domain wider than it has to be, on purpose.** At the current
/// widths a signed 64-bit integer would hold every product a caller can declare,
/// with `2^31` to spare at the worst corner, which
/// `the_reach_check_is_wider_than_the_widths_currently_need` pins. That margin is
/// an argument over the exact widths of three coordinate types rather than a
/// property of the check, so widening any one of them would break the narrower
/// form silently while every test still passed. The slot range's own count
/// condition is written the same way for the same reason.
const fn reach_is_representable(
    base: Exponent,
    slope: Exponent,
    magnitudes: MagnitudeCount,
) -> bool {
    let largest = (magnitudes.count() as i128) - 1;
    let reach = (base.power() as i128) + (slope.power() as i128) * largest;
    reach >= (i32::MIN as i128) && reach <= (i32::MAX as i128)
}

/// How the quantum varies with magnitude, as an affine law.
///
/// The quantum at magnitude `m` is `radix^(BASE + SLOPE * m)`. The radix lives on
/// the ambient domain because it is a property of the domain rather than of the
/// step law.
pub trait Quantum {
    /// The exponent at magnitude zero.
    const BASE: Exponent;

    /// The change in exponent per step of magnitude. Zero for the constant family.
    const SLOPE: Exponent;

    /// How many distinct magnitudes the law ranges over. One for the constant
    /// family, and never zero: a law over no magnitudes describes no values.
    const MAGNITUDES: MagnitudeCount;

    /// What an implementor owes, checked rather than asked for.
    ///
    /// The three coordinates above can describe something that is not a step law,
    /// and an implementor supplying one has not supplied the obligations.
    /// `proposal::the_concept_is_closed_and_the_inventory_is_open` says a new
    /// instance earns admission by supplying the concept's obligations, and that
    /// closing the concept while opening the inventory is what makes admission a
    /// check rather than a negotiation. This is that check, and until it existed
    /// the first condition below was a sentence in a doc comment and the second
    /// was written nowhere.
    ///
    /// Every function in this file that reads the law forces this const, so an
    /// implementor that does not meet it stops the build at the use site with a
    /// named message. `contains` conjoins `magnitude_in_range`, so it reaches a
    /// format's law too, and nothing arrives at a lowered path, which is what
    /// `ruling::never_a_runtime_check_and_one_lowered_path` asks for.
    ///
    /// **It fires at codegen, not at `cargo check`**, on the same reading and for
    /// the same reason the slot range's obligation states. The guarantee is that
    /// an inadmissible law cannot reach a produced binary; it can reach a passing
    /// check.
    ///
    /// `is_admissible` below is the same question asked without forcing the const,
    /// which is what a test can use on a construction that must keep compiling.
    /// Both read the same two predicates rather than restating them, so the
    /// verdict and the refusal cannot come apart.
    const ADMITTED: () = {
        assert!(
            ranges_over_a_magnitude(Self::MAGNITUDES),
            "a quantum law over no magnitudes describes no values, so it is not a step law"
        );
        assert!(
            reach_is_representable(Self::BASE, Self::SLOPE, Self::MAGNITUDES),
            "the step law runs its exponent past what an exponent carries before reaching its \
             largest magnitude, so it names no quantum there"
        );
    };
}

/// Whether a quantum law meets what the contract asks of it.
///
/// The law, returning a verdict rather than asserting one, so a construction that
/// compiles and is wrong can be reported on without forcing the const that would
/// refuse it. That is what lets the wrong construction live permanently in a test
/// rather than in a scratch file somebody deletes.
#[must_use]
pub const fn is_admissible<Q: Quantum>() -> Bool {
    Bool::of(
        ranges_over_a_magnitude(Q::MAGNITUDES)
            && reach_is_representable(Q::BASE, Q::SLOPE, Q::MAGNITUDES),
    )
}

/// A step that does not change with magnitude.
///
/// `EXP` is the exponent, so `EXP = 0` is the integers and `EXP = -F` is fixed
/// point at fraction width `F`.
pub struct Constant<const EXP: i32>;

impl<const EXP: i32> Quantum for Constant<EXP> {
    const BASE: Exponent = Exponent::of(EXP);
    const SLOPE: Exponent = Exponent::ZERO;
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::ONE;
}

/// A step that grows by one exponent per magnitude, which is the floating shape.
///
/// `MIN_EXP` is the exponent at the smallest magnitude and `COUNT` is how many
/// magnitudes there are.
pub struct Indexed<const MIN_EXP: i32, const COUNT: u32>;

impl<const MIN_EXP: i32, const COUNT: u32> Quantum for Indexed<MIN_EXP, COUNT> {
    const BASE: Exponent = Exponent::of(MIN_EXP);
    const SLOPE: Exponent = Exponent::ONE;
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(COUNT);
}

/// The exponent of the quantum at a magnitude.
///
/// Free rather than a trait method so it is callable at stage zero without the
/// trait having to be const.
#[must_use]
pub const fn exponent_at<Q: Quantum>(magnitude: Magnitude) -> Exponent {
    let () = Q::ADMITTED;
    Q::BASE.advanced(Q::SLOPE, magnitude)
}

/// Whether a magnitude is one the law ranges over.
#[must_use]
pub const fn magnitude_in_range<Q: Quantum>(magnitude: Magnitude) -> Bool {
    let () = Q::ADMITTED;
    magnitude.is_within(Q::MAGNITUDES)
}

/// Whether the law is the constant-quantum family.
///
/// This is the axis the two families differ on and the only one, so a claim about
/// one family is a claim predicated on this.
#[must_use]
pub const fn is_constant_family<Q: Quantum>() -> Bool {
    let () = Q::ADMITTED;
    Bool::of(Q::SLOPE.power() == Exponent::ZERO.power())
}
