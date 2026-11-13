//! The oracle, `arvo_format::add`, as the tests ask it. Nothing on an arm's
//! `add` path reads this module.

use arvo_format::slots::{Slot, Slots};
use arvo_format::{DeclaredSignature, Dither, Format, add};

/// The sum of the members at slot indices `a` and `b` under the signature.
#[must_use]
pub const fn oracle_add<S: DeclaredSignature>(a: i128, b: i128) -> i128 {
    add::<S>(Slot::at(a), Slot::at(b), Dither::UNUSED).index()
}

/// The slot indices of every member, low to high.
pub fn members<S: DeclaredSignature>() -> core::ops::RangeInclusive<i128> {
    let lo = <<S::Format as Format>::Slots as Slots>::MIN.index();
    let hi = <<S::Format as Format>::Slots as Slots>::MAX.index();
    lo ..= hi
}

/// How many pairs of members an arm's `add` gets wrong against the oracle.
///
/// Every pair where `exhaustive` (at most 1024 members), else every pair of the
/// edge set: both ends, one in from each, the middle and the three around zero,
/// those that are members.
pub fn wrong_pairs<S: crate::Sole>(
    exhaustive: bool,
    add: impl Fn(S::Repr, S::Repr) -> S::Repr,
) -> usize {
    use crate::Machine;
    let range = members::<S>();
    let (lo, hi) = (*range.start(), *range.end());
    let mut operands = [0i128; 1024];
    let mut n = 0;
    if exhaustive {
        assert!(hi - lo < 1024, "exhaustive only up to ten bits");
        for i in range {
            operands[n] = i;
            n += 1;
        }
    } else {
        for e in [lo, lo + 1, lo / 2, -1, 0, 1, hi / 2, hi - 1, hi] {
            if range.contains(&e) && !operands[.. n].contains(&e) {
                operands[n] = e;
                n += 1;
            }
        }
    }
    let mut wrong = 0;
    for &a in &operands[.. n] {
        for &b in &operands[.. n] {
            let got = add(S::Repr::from_index(a), S::Repr::from_index(b)).index();
            if got != oracle_add::<S>(a, b) {
                wrong += 1;
            }
        }
    }
    wrong
}

/// The tests every arm runs through its own value type: the four signatures at
/// widths 1 to 10 exhaustively, and at every admitted width over the edge set.
///
/// An arm's value type is named by `$num` and must offer `from_raw`, `add` and
/// `raw` on the concrete signature.
#[macro_export]
macro_rules! value_tests {
    ($num:ident) => {
        $crate::value_tests!(@test $num, value_add_widths_1_to_10_exhaustive, true,
            [1 2 3 4 5 6 7 8 9 10]);
        $crate::value_tests!(@test $num, value_add_every_width_over_the_edges, false,
            [1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
             33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61
             62 63 64]);
    };
    (@test $num:ident, $test:ident, $all:expr, [$($w:literal)+]) => {
        #[test]
        fn $test() {
            use $crate::__private::arvo_format::points::{Integer, UFixed};
            use $crate::__private::arvo_format::overflow::{Saturate, Wrap};
            let mut wrong = 0usize;
            $(
                wrong += $crate::value_tests!(@one $num, Integer<$w>, Wrap, $all);
                wrong += $crate::value_tests!(@one $num, Integer<$w>, Saturate, $all);
                wrong += $crate::value_tests!(@one $num, UFixed<$w, 0>, Wrap, $all);
                wrong += $crate::value_tests!(@one $num, UFixed<$w, 0>, Saturate, $all);
            )+
            assert_eq!(wrong, 0, "pairs of members the value type's add gets wrong");
        }
    };
    (@one $num:ident, $f:ty, $o:ty, $all:expr) => {{
        type S = $crate::__private::arvo_format::Signature<
            $f,
            $crate::__private::arvo_format::Adapt<
                $crate::__private::arvo_format::rounding::TowardZero,
                $o,
            >,
        >;
        $crate::oracle::wrong_pairs::<S>($all, |a, b| {
            $num::<S>::from_raw(a).add($num::<S>::from_raw(b)).raw()
        })
    }};
}

/// The adaptation of an exact sum, by the oracle: `adapt` of an on-grid position.
#[must_use]
pub const fn oracle_adapt<S: DeclaredSignature>(exact: i128) -> i128 {
    arvo_format::adapt::<S>(
        arvo_format::Exact::between(Slot::at(exact), arvo_format::Fraction::of(0, 1)),
        Dither::UNUSED,
    )
    .index()
}
