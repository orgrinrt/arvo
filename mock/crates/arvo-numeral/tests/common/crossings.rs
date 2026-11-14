//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! One conversion between two signatures, held against the tests' own
//! arithmetic.
//!
//! A sweep over pairs of shapes instantiates one conversion per pair, so what
//! is generic over the pair is kept to the two calls and what the two
//! signatures declare. The members a pair is run at, and the comparison, are
//! one function for every pair.

use arvo_format::{Format, Quantum, radix};
use arvo_numeral::Sole;

use super::oracle::{self, Landed};
use super::shapes::{Shapes, members};
use super::{range, value};

/// The pair of a source signature and a target signature, as the sweep reads it.
pub struct Crossing {
    /// The source's range.
    from:  (i128, i128),
    /// The target's range.
    onto:  (i128, i128),
    /// A source slot on the target's grid, as a ratio of two integers: the
    /// source's quantum over the target's. Nothing where a term of it leaves
    /// the host's integer, and then no slot is added around the target's ends.
    ratio: Option<(i128, i128)>,
    /// The value type's answer.
    got:   fn(i128) -> Landed,
    /// The tests' own.
    want:  fn(i128) -> Landed,
}

fn got<S: Sole, T: Sole>(a: i128) -> Landed {
    let v = value::<S>(a);
    Landed {
        slot: v.convert::<T>().slot().index(),
        fits: v.fits_convert::<T>().get(),
    }
}

/// The quantum of `F` as a ratio of two integers.
fn quantum<F: Format>() -> Option<(i128, i128)> {
    let power = <F::Quantum as Quantum>::BASE.power();
    let scale = i128::from(radix::<F>().base()).checked_pow(power.unsigned_abs())?;
    Some(if power >= 0 { (scale, 1) } else { (1, scale) })
}

impl Crossing {
    /// The crossing from `S` onto `T`.
    pub fn of<S: Sole, T: Sole>() -> Self {
        let ratio = quantum::<S::Format>().and_then(|(n, d)| {
            let (tn, td) = quantum::<T::Format>()?;
            Some((n.checked_mul(td)?, d.checked_mul(tn)?))
        });
        Self {
            from: range::<S>(),
            onto: range::<T>(),
            ratio,
            got: got::<S, T>,
            want: oracle::converted::<S, T>,
        }
    }

    /// The source slots that land around a target slot `t`: on it, a slot and
    /// half a slot either side of it, and one source slot either side of each
    /// of those, which is where a mode decides and where a policy takes over.
    fn around(&self, t: i128, out: &mut Vec<i128>) {
        let Some((num, den)) = self.ratio else {
            return;
        };
        let Some(scaled) = t.checked_mul(den) else {
            return;
        };
        // The source slot at or below the target slot, and how many source
        // slots a target slot spans, none where the source's grid is coarser.
        let at = scaled.div_euclid(num);
        let reach = den / num;
        let centres = [
            at.saturating_sub(reach),
            at.saturating_sub(reach / 2),
            at,
            at.saturating_add(reach / 2),
            at.saturating_add(reach),
        ];
        for centre in centres {
            for a in [centre.saturating_sub(1), centre, centre.saturating_add(1)] {
                if a >= self.from.0 && a <= self.from.1 {
                    out.push(a);
                }
            }
        }
    }

    /// The crossing at every member given and at the source slots around zero
    /// and around each end of the target's range.
    pub fn hold(&self, members: &[i128], shapes: &mut Shapes) {
        shapes.signatures += 1;
        let mut edges = Vec::new();
        for t in [self.onto.0, -1, 0, 1, self.onto.1] {
            self.around(t, &mut edges);
        }
        for &a in members.iter().chain(&edges) {
            shapes.checked += 1;
            let want = (self.want)(a);
            if !want.fits {
                shapes.unfit += 1;
            }
            if (self.got)(a) != want {
                shapes.wrong += 1;
            }
        }
    }

    /// The planted arm: this crossing's value held against the expectation of
    /// its sibling under the other policy, at the members and edges `hold`
    /// takes. Where some member does not fit the target the two policies part,
    /// so the wrong expectation has to disagree on this very pair, or the
    /// sweep does not see the policy.
    pub fn against(&self, other: &Crossing, members: &[i128], shapes: &mut Shapes) {
        let mut edges = Vec::new();
        for t in [self.onto.0, -1, 0, 1, self.onto.1] {
            self.around(t, &mut edges);
        }
        let (mut unfit, mut caught) = (false, 0u64);
        for &a in members.iter().chain(&edges) {
            let wrong = (other.want)(a);
            unfit |= !wrong.fits;
            if (self.got)(a) != wrong {
                caught += 1;
            }
        }
        assert!(
            caught > 0 || !unfit,
            "the other policy's expectation passed on a pair that leaves the range"
        );
        shapes.planted += caught;
    }
}

/// The members a sweep takes of a source format, which no adaptation moves.
pub fn source<S: Sole>() -> Vec<i128> {
    members::<S>()
}

/// A source signature onto one target format under both policies and the five
/// modes.
#[macro_export]
macro_rules! onto_every_adaptation {
    ($source:ty, $target:ty, $members:expr, $shapes:expr) => {{
        use arvo_format::overflow::{Saturate, Wrap};
        use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
        use arvo_format::{Adapt, Signature};
        use $crate::common::crossings::Crossing;
        macro_rules! onto {
            ($mode:ty, $policy:ty) => {
                Crossing::of::<$source, Signature<$target, Adapt<$mode, $policy>>>()
                    .hold($members, $shapes)
            };
        }
        onto!(TowardZero, Wrap);
        onto!(Floor, Wrap);
        onto!(Ceil, Wrap);
        onto!(HalfUp, Wrap);
        onto!(HalfEven, Wrap);
        onto!(TowardZero, Saturate);
        onto!(Floor, Saturate);
        onto!(Ceil, Saturate);
        onto!(HalfUp, Saturate);
        onto!(HalfEven, Saturate);
    }};
}

/// A source signature onto one target format under both policies, at one mode,
/// and the wrapping value against the saturating expectation as the planted
/// arm.
#[macro_export]
macro_rules! onto_both_policies {
    ($source:ty, $target:ty, $mode:ident, $members:expr, $shapes:expr) => {{
        use arvo_format::overflow::{Saturate, Wrap};
        use arvo_format::rounding::$mode;
        use arvo_format::{Adapt, Signature};
        use $crate::common::crossings::Crossing;
        let wrapping = Crossing::of::<$source, Signature<$target, Adapt<$mode, Wrap>>>();
        let saturating = Crossing::of::<$source, Signature<$target, Adapt<$mode, Saturate>>>();
        wrapping.hold($members, $shapes);
        saturating.hold($members, $shapes);
        wrapping.against(&saturating, $members, $shapes);
    }};
}
