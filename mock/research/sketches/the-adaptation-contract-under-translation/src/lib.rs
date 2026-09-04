//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Reaching the two regions of the shipped map separately, through the public
//! surface only.
//!
//! `round_slot` and `complete_slot` are private, so nothing here calls them.
//! Rounding is isolated by handing `adapt` a format wide enough that completion
//! never fires, and completion is isolated by handing it a position already on
//! the grid, where no mode does anything. Both isolations are measured rather
//! than asserted: `w1` and `w2` in the steps count how often the other region
//! fired, and both must be zero.
//!
//! Nothing here reimplements a rounding mode or a range policy. Every number a
//! step reports comes out of `arvo_format::apply::adapt`.

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::ambient::BinaryRationals;
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::format::{Format, Phase};
use arvo_format::overflow::{Clamp, Policy, Saturate, Wrap};
use arvo_format::points::{Integer, UFixed};
use arvo_format::quantum::Constant;
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, Mode, Stochastic, TowardZero};
use arvo_format::slots::{Slot, Slots};
use arvo_format::width::Width;

/// A slot range symmetric about zero, which no shipped format has.
///
/// The signed ranges are two's complement, so `MIN` is one below `-MAX` and the
/// low end has no positive twin. Negation equivariance of the completion region
/// is a question about a symmetric range, so the range has to be declared here.
/// The trait is open and this is what an outside implementor supplies.
pub struct SymmetricSlots<const HALF: i64, const BITS: u32>;

impl<const HALF: i64, const BITS: u32> Slots for SymmetricSlots<HALF, BITS> {
    const MAX: Slot = Slot::at(HALF);
    const MIN: Slot = Slot::at(-HALF);
    const WIDTH: Width = Width::bits(BITS);
}

/// A format over a symmetric slot range, otherwise an integer.
pub struct SymmetricInteger<const HALF: i64, const BITS: u32>;

impl<const HALF: i64, const BITS: u32> Format for SymmetricInteger<HALF, BITS> {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = SymmetricSlots<HALF, BITS>;

    const PHASE: Phase = Phase::ZERO;
}

/// Which slot range a step is measuring over.
///
/// Named rather than parameterised because the dispatch below has to reach a
/// concrete type per arm, and a name is what a report can print.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Range {
    /// `-2^39 .. 2^39 - 1`. Wide enough that the bands the steps sweep never
    /// leave it, which is what isolates the rounding region.
    WideSigned,
    /// `0 .. 2^40 - 1`, the same isolation on a domain with no negatives.
    WideUnsigned,
    /// `-16 .. 15`, the two's complement range of five bits.
    SmallSigned,
    /// `0 .. 31`, the unsigned range of five bits.
    SmallUnsigned,
    /// `-15 .. 15`, declared here because no shipped format is symmetric.
    SmallSymmetric,
}

impl Range {
    /// Every range a step may sweep, so a walk takes all of them rather than
    /// the ones somebody remembered.
    pub const ALL: [Range; 5] = [
        Range::WideSigned,
        Range::WideUnsigned,
        Range::SmallSigned,
        Range::SmallUnsigned,
        Range::SmallSymmetric,
    ];
    /// The ranges small enough to sweep a completion over exhaustively.
    pub const SMALL: [Range; 3] = [Range::SmallSigned, Range::SmallUnsigned, Range::SmallSymmetric];

    /// The lowest and highest admitted index, read off the format rather than
    /// written down twice.
    #[must_use]
    pub fn bounds(self) -> (i64, i64) {
        fn of<S: Slots>() -> (i64, i64) {
            (S::MIN.index(), S::MAX.index())
        }
        match self {
            Range::WideSigned => of::<<Integer<40> as Format>::Slots>(),
            Range::WideUnsigned => of::<<UFixed<40, 0> as Format>::Slots>(),
            Range::SmallSigned => of::<<Integer<5> as Format>::Slots>(),
            Range::SmallUnsigned => of::<<UFixed<5, 0> as Format>::Slots>(),
            Range::SmallSymmetric => of::<<SymmetricInteger<15, 5> as Format>::Slots>(),
        }
    }

    /// Whether the range admits a negative index.
    #[must_use]
    pub fn reaches_negatives(self) -> bool {
        self.bounds().0 < 0
    }

    /// Whether the range is symmetric about zero.
    #[must_use]
    pub fn is_symmetric(self) -> bool {
        let (lo, hi) = self.bounds();
        lo == -hi
    }
}

/// Every mode, so a walk takes all six rather than the ones an arm needed.
pub const MODES: [Mode; 6] = [
    Mode::TowardZero,
    Mode::Floor,
    Mode::Ceil,
    Mode::HalfUp,
    Mode::HalfEven,
    Mode::Stochastic,
];

/// Every policy this crate ships.
pub const POLICIES: [Policy; 3] = [Policy::Wrap, Policy::Saturate, Policy::Clamp];

/// The whole map, at one range, one mode and one policy.
///
/// Every arm is a real instantiation of `Signature`, so the value that comes
/// back is what a consumer declaring that signature would get.
#[must_use]
pub fn adapt_at(range: Range, mode: Mode, policy: Policy, exact: Exact, dither: Dither) -> i64 {
    macro_rules! at_policy {
        ($fmt:ty, $md:ty) => {
            match policy {
                Policy::Wrap => adapt::<Signature<$fmt, Adapt<$md, Wrap>>>(exact, dither),
                Policy::Saturate => adapt::<Signature<$fmt, Adapt<$md, Saturate>>>(exact, dither),
                Policy::Clamp => adapt::<Signature<$fmt, Adapt<$md, Clamp>>>(exact, dither),
            }
        };
    }
    macro_rules! at_mode {
        ($fmt:ty) => {
            match mode {
                Mode::TowardZero => at_policy!($fmt, TowardZero),
                Mode::Floor => at_policy!($fmt, Floor),
                Mode::Ceil => at_policy!($fmt, Ceil),
                Mode::HalfUp => at_policy!($fmt, HalfUp),
                Mode::HalfEven => at_policy!($fmt, HalfEven),
                Mode::Stochastic => at_policy!($fmt, Stochastic),
            }
        };
    }
    let slot = match range {
        Range::WideSigned => at_mode!(Integer<40>),
        Range::WideUnsigned => at_mode!(UFixed<40, 0>),
        Range::SmallSigned => at_mode!(Integer<5>),
        Range::SmallUnsigned => at_mode!(UFixed<5, 0>),
        Range::SmallSymmetric => at_mode!(SymmetricInteger<15, 5>),
    };
    slot.index()
}

/// A position, written the way the coordinate wants it.
#[must_use]
pub fn position(slot: i64, num: i64, den: i64) -> Exact {
    Exact::between(Slot::at(slot), Fraction::of(num, den))
}

/// A dither at `num/den`, for the one mode that reads one.
#[must_use]
pub fn dither(num: i64, den: i64) -> Dither {
    Dither::at(Fraction::of(num, den))
}

/// The rounding region alone: the slot a mode picks for a position, with the
/// completion known not to have fired.
///
/// Returns the slot and whether the answer stayed inside the range, so the
/// caller can count the isolation rather than assume it.
#[must_use]
pub fn round_only(range: Range, mode: Mode, exact: Exact, dither: Dither) -> (i64, bool) {
    debug_assert!(
        matches!(range, Range::WideSigned | Range::WideUnsigned),
        "rounding is isolated only by a range the sweep cannot leave"
    );
    let (lo, hi) = range.bounds();
    // Every policy agrees inside the range, so the answer is the rounding
    // region's whatever this arm passes. `Wrap` is the arm that would move a
    // value that left the range, so a completion that fired would be visible.
    let got = adapt_at(range, mode, Policy::Wrap, exact, dither);
    let saturated = adapt_at(range, mode, Policy::Saturate, exact, dither);
    // If the two policies agree and the answer is strictly inside the range,
    // no completion fired: wrapping and saturating differ on every value that
    // leaves it, which the shipped suite measures separately.
    (got, got == saturated && got > lo && got < hi)
}

/// The completion region alone: the slot a policy gives a position already on
/// the grid, where no mode has anything to decide.
///
/// The mode is passed through so a caller can check the answer does not move
/// with it, which is what says the rounding region is not firing.
#[must_use]
pub fn complete_only(range: Range, policy: Policy, slot: i64) -> i64 {
    adapt_at(
        range,
        Mode::Floor,
        policy,
        Exact::on_grid(Slot::at(slot)),
        Dither::UNUSED,
    )
}

/// Whether the completion region moved with the rounding mode at a position on
/// the grid, which it must not.
#[must_use]
pub fn completion_is_mode_blind(range: Range, policy: Policy, slot: i64) -> bool {
    let want = complete_only(range, policy, slot);
    MODES.iter().all(|&mode| {
        adapt_at(
            range,
            mode,
            policy,
            Exact::on_grid(Slot::at(slot)),
            Dither::UNUSED,
        ) == want
    })
}

/// A verdict line in the shape every step prints, so `out/` can be grepped for
/// `BROKEN` rather than read.
pub fn verdict(id: &str, claim: &str, held: bool) {
    println!("{id}: {claim} -> {}", if held { "HELD" } else { "BROKEN" });
}

/// The residues a sweep walks.
///
/// Denominators that reach a tie and denominators that cannot, so the tie-free
/// restriction is a restriction of a set that has ties in it rather than of a
/// set that never had any. A step that swept only odd denominators would report
/// every mode equivariant and every arm in it would look reasonable.
pub fn residues() -> impl Iterator<Item = (i64, i64)> {
    [2i64, 3, 4, 5, 8, 16]
        .into_iter()
        .flat_map(|den| (0 .. den).map(move |num| (num, den)))
}

/// Whether a residue is exactly half.
#[must_use]
pub fn is_tie(num: i64, den: i64) -> bool {
    num * 2 == den
}
