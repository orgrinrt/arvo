//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The laws the design owes addition, each stated over what the operation
//! computes.
//!
//! Three files along three seams: the exact step and what its obligation refuses,
//! the operation over the pairs the sweeps reach, and the verdict against brute
//! force with the counts the canon records. What they share is here, and every
//! piece of it is an instrument rather than a law.
//!
//! The rational arithmetic below is written independently of the step's. The
//! step never multiplies by the quantum, and the denotation test is only worth
//! anything if its oracle does, so the oracle carries the scale through value
//! space and back rather than reusing the step's euclidean split.
//!
//! The pair table is what makes the eight-bit count affordable in the ordinary
//! suite: every pair is added once, and a triple is then two lookups on each
//! side rather than four additions.

use notko::Maybe;

use crate::adapt::DeclaredSignature;
use crate::addition::add;
use crate::ambient::{Ambient, Radix};
use crate::apply::{Dither, Exact, Fraction};
use crate::format::{Format, Phase};
use crate::overflow::SHIPPED_POLICIES;
use crate::quantum::{Exponent, MagnitudeCount, Quantum};
use crate::rounding::ALL_MODES;
use crate::slots::{Slot, Slots};
use crate::tests::dispatch::{self, PerSignature};
use crate::width::{Bool, Width};

/// Every phase the sweeps cross, at one slot range, run through a walk.
///
/// Whole phases at zero, below and above it, and three fractional ones: a tie, a
/// third, and a third with the sign on the numerator. One list, so the files that
/// sweep phases cannot drift apart on which phases they mean.
macro_rules! every_phase {
    ($walk:ident; $slots:ty) => {
        $walk.run::<Grid<BinaryRationals, Constant<0>, $slots, 0, 1>>();
        $walk.run::<Grid<BinaryRationals, Constant<0>, $slots, -1, 1>>();
        $walk.run::<Grid<BinaryRationals, Constant<0>, $slots, 5, 1>>();
        $walk.run::<Grid<BinaryRationals, Constant<0>, $slots, 1, 2>>();
        $walk.run::<Grid<BinaryRationals, Constant<0>, $slots, 1, 3>>();
        $walk.run::<Grid<BinaryRationals, Constant<0>, $slots, -2, 3>>();
    };
}

mod the_operation;
mod the_step;
mod the_verdict;

/// How many phases `every_phase` walks.
const fn phases() -> usize {
    6
}

/// How many signatures one format is crossed under.
const fn signatures() -> usize {
    ALL_MODES.len() * SHIPPED_POLICIES.len()
}

/// The dithers the sweeps carry: none, below the midpoint, on it, and above it.
const DITHERS: [Dither; 4] = [
    Dither::UNUSED,
    Dither::at(Fraction::of(1, 4)),
    Dither::at(Fraction::of(1, 2)),
    Dither::at(Fraction::of(3, 4)),
];

/// A slot range with both ends free.
///
/// The shipped families are the two's complement range and the unsigned one,
/// each anchored at zero. The verdict is claimed over every range, including
/// ones that do not contain zero, so this is the family with both ends a
/// parameter.
struct Window<const LO: i64, const HI: i64>;

/// The narrowest width that addresses the slots from `lo` to `hi`.
///
/// What the slot range's obligation asks of the declared width, so every window
/// the sweeps name is admitted rather than merely declared.
const fn width_for(lo: i64, hi: i64) -> u32 {
    let span = (hi as i128) - (lo as i128);
    let mut bits = 1u32;
    while bits < 62 && span >= (1i128 << bits) {
        bits += 1;
    }
    bits
}

impl<const LO: i64, const HI: i64> Slots for Window<LO, HI> {
    const MAX: Slot = Slot::at(HI);
    const MIN: Slot = Slot::at(LO);
    const WIDTH: Width = Width::bits(width_for(LO, HI));
}

/// Every member of a format's slot range, lowest first.
fn members<F: Format>() -> impl Iterator<Item = Slot> {
    let min = <F::Slots as Slots>::MIN.index();
    let max = <F::Slots as Slots>::MAX.index();
    (min ..= max).map(Slot::at)
}

/// How many members a format's slot range has.
fn member_count<F: Format>() -> usize {
    (<F::Slots as Slots>::MAX.index() - <F::Slots as Slots>::MIN.index() + 1) as usize
}

/// Both ends of the coordinate and one in from each, and each end of the range
/// with the slot either side of it.
///
/// Shared rather than local to one file: the totality sweep draws its fed pairs
/// from it and the verdict sweep draws its no-divergence sample from the same
/// ten, so a width neither can brute force still gets one instrument's worth of
/// coverage from both laws.
pub(super) fn edges(min: Slot, max: Slot) -> [Slot; 10] {
    let (lo, hi) = (min.index(), max.index());
    [
        i64::MIN,
        i64::MIN + 1,
        lo - 1,
        lo,
        lo + 1,
        hi - 1,
        hi,
        hi + 1,
        i64::MAX - 1,
        i64::MAX,
    ]
    .map(Slot::at)
}

/// Four members at the ends of the range: the two lowest and the two highest.
///
/// Members rather than `edges`' wider set on purpose. The associativity
/// verdict is quantified over stored operands rather than over the ambient
/// domain, `quantifying_over_the_ambient_domain_refuses_a_format_that_is_associative`
/// pins the distinction with a format the law holds over until an ambient
/// translation is asked about, so a sample checking the licensed half of the
/// claim draws only from what the claim is actually stated over. Where a
/// range holds fewer than four members the ends clamp together, which asks
/// the same member twice rather than a wrong one.
pub(super) fn boundary_members(min: Slot, max: Slot) -> [Slot; 4] {
    let (lo, hi) = (min.index(), max.index());
    [lo, (lo + 1).min(hi), (hi - 1).max(lo), hi].map(Slot::at)
}

/// A pair of members summing to `sum`, where one exists.
///
/// `a` and `b` range independently over `[min, max]`, so their sum covers every
/// integer in `[2 * min, 2 * max]` and nothing outside it. Where `sum` is in
/// range the construction is the one the associativity witness wants: pin one
/// operand at whichever end `sum` sits closer to and let the other carry the
/// rest, so the pair sits at the end of the range the witness needs rather than
/// in its middle.
pub(super) fn pair_for_sum(min: i64, max: i64, sum: i128) -> Maybe<(Slot, Slot)> {
    let (min, max) = (min as i128, max as i128);
    if sum < 2 * min || sum > 2 * max {
        return Maybe::Isnt;
    }
    let (a, b) = if sum <= min + max { (min, sum - min) } else { (max, sum - max) };
    Maybe::Is((Slot::at(a as i64), Slot::at(b as i64)))
}

/// The declared coordinates of a format, for a failure message.
///
/// The crate asks the compiler for no type names at run time, so a message names the
/// declaration rather than the type: the ambient's radix and sign, the quantum
/// law's base, slope and magnitude count, the phase, and the ends of the range.
fn coordinates<F: Format>() -> (
    Radix,
    Bool,
    Exponent,
    Exponent,
    MagnitudeCount,
    Phase,
    Slot,
    Slot,
) {
    (
        <F::Ambient as Ambient>::RADIX,
        <F::Ambient as Ambient>::SIGNED,
        <F::Quantum as Quantum>::BASE,
        <F::Quantum as Quantum>::SLOPE,
        <F::Quantum as Quantum>::MAGNITUDES,
        F::PHASE,
        <F::Slots as Slots>::MIN,
        <F::Slots as Slots>::MAX,
    )
}

/// `what` at every mode and every shipped policy over `F`, summed.
fn at_every_signature<F: Format, P: PerSignature<Out = u64>>(what: &P) -> u64 {
    let mut total = 0u64;
    for mode in ALL_MODES {
        for policy in SHIPPED_POLICIES {
            total += dispatch::at::<F, P>(mode, policy, what);
        }
    }
    total
}

/// A ratio in the wide carrier, in lowest terms with a positive denominator.
///
/// The oracle's number type. Nothing in it comes from the step, so a law checked
/// against it is checked against arithmetic the step did not do.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Ratio {
    num: i128,
    den: i128,
}

/// Euclid's algorithm, the oracle's own copy.
///
/// Kept apart from the step's on purpose: an oracle sharing the implementation's
/// reduction would agree with it about any mistake the reduction makes.
fn common_factor(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    if a == 0 { 1 } else { a }
}

impl Ratio {
    /// `num / den`, normalised.
    fn of(num: i128, den: i128) -> Self {
        assert!(den != 0, "the oracle was handed a ratio over zero");
        let (num, den) = if den < 0 { (-num, -den) } else { (num, den) };
        let g = common_factor(num, den);
        Self {
            num: num / g,
            den: den / g,
        }
    }

    /// A whole number.
    fn whole(n: i128) -> Self {
        Self::of(n, 1)
    }

    /// The phase a format declares, as the ratio it names.
    fn phase_of<F: Format>() -> Self {
        Self::of(F::PHASE.numerator() as i128, F::PHASE.denominator() as i128)
    }

    /// The sum, over the least common denominator so no product runs away.
    fn plus(self, other: Self) -> Self {
        let g = common_factor(self.den, other.den);
        let den = self.den / g * other.den;
        Self::of(
            self.num * (den / self.den) + other.num * (den / other.den),
            den,
        )
    }

    /// The difference.
    fn minus(self, other: Self) -> Self {
        self.plus(Self {
            num: -other.num,
            den: other.den,
        })
    }

    /// The product, with the cross factors taken out first.
    fn times(self, other: Self) -> Self {
        let g1 = common_factor(self.num, other.den);
        let g2 = common_factor(other.num, self.den);
        Self::of(
            (self.num / g1) * (other.num / g2),
            (self.den / g2) * (other.den / g1),
        )
    }

    /// The quotient.
    fn over(self, other: Self) -> Self {
        self.times(Self::of(other.den, other.num))
    }

    /// The greatest whole number not above it.
    fn floor(self) -> i128 {
        self.num.div_euclid(self.den)
    }

    /// Whether it is a whole number.
    fn is_whole(self) -> bool {
        self.den == 1
    }

    /// The position a ratio of slots names: the slot below it and the remainder.
    fn position(self) -> Exact {
        let whole = self.floor();
        let rest = Self::of(self.num.rem_euclid(self.den), self.den);
        Exact::between(
            Slot::at(whole as i64),
            Fraction::of(rest.num as i64, rest.den as i64),
        )
    }
}

/// The adapted sum of every pair of members under one signature and dither, as
/// offsets from the lowest slot.
///
/// Sized for the widest format the sweeps run, eight bits.
#[derive(PartialEq, Eq)]
struct Table {
    members: usize,
    sums:    [[u8; 256]; 256],
}

impl Table {
    /// Every pair of members added under `S` at `dither`.
    ///
    /// Requires every answer to be a member, so a table is also a totality check
    /// over the pairs it holds.
    fn of<S: DeclaredSignature>(dither: Dither) -> Self {
        let min = <<S::Format as Format>::Slots as Slots>::MIN;
        let max = <<S::Format as Format>::Slots as Slots>::MAX;
        let members = member_count::<S::Format>();
        assert!(
            members <= 256,
            "the table holds eight-bit ranges and narrower"
        );
        let mut sums = [[0u8; 256]; 256];
        for (i, row) in sums.iter_mut().enumerate().take(members) {
            for (j, cell) in row.iter_mut().enumerate().take(members) {
                let a = Slot::at(min.index() + i as i64);
                let b = Slot::at(min.index() + j as i64);
                let got = add::<S>(a, b, dither);
                assert!(
                    got.is_within(min, max).get(),
                    "{a:?} + {b:?} adapted to {got:?}, outside [{min:?}, {max:?}]"
                );
                *cell = (got.index() - min.index()) as u8;
            }
        }
        Self {
            members,
            sums,
        }
    }

    /// The adapted sum of the members at offsets `i` and `j`.
    fn at(&self, i: usize, j: usize) -> usize {
        self.sums[i][j] as usize
    }

    /// How many triples `(a + b) + c` and `a + (b + c)` disagree on.
    fn divergent(&self) -> u64 {
        let mut count = 0u64;
        for a in 0 .. self.members {
            for b in 0 .. self.members {
                let ab = self.at(a, b);
                for c in 0 .. self.members {
                    if self.at(ab, c) != self.at(a, self.at(b, c)) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

/// The table at one signature, for a dispatch.
struct Sums(Dither);

impl PerSignature for Sums {
    type Out = Table;

    fn run<S: DeclaredSignature>(&self) -> Table {
        Table::of::<S>(self.0)
    }
}
