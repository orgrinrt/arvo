//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Parity against what MathWorks publishes, and where the vocabulary stops.
//!
//! Every expected number here comes from the `fi` reference page rather than
//! from arvo. An arm that computed both sides through arvo would assert that
//! arvo agrees with itself, which passes for the wrong reason and is the failure
//! this file would be most likely to ship.

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::overflow::Wrap;
use arvo_format::points::Integer;
use arvo_format::quantum::Magnitude;
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, Stochastic, TowardZero};
use arvo_format::slots::{Slot, Slots};
use arvo_format::standards::rounding_method::Nearest;
use arvo_format::standards::{Fi, FractionLength, Ufi};

/// Pi as an exact rational, to fifteen places.
///
/// The parity arms need a position on the grid rather than a float, and a float
/// would put the host's own rounding between MathWorks' number and arvo's. Fifteen
/// places is far more than enough: the nearest arm's closest call sits three
/// hundredths from the midpoint.
const PI_NUM: i128 = 3_141_592_653_589_793;
const PI_DEN: i128 = 1_000_000_000_000_000;

/// Where pi sits on the grid of a format at fraction length `f`.
///
/// Returns the slot below it and the remainder as a rational, which is what the
/// applied map takes. Computed in `i128` because pi times two to the fourteenth,
/// scaled by the denominator, leaves `i64`; the remainder handed back does not.
fn pi_at(f: u32) -> Exact {
    let scaled = PI_NUM * (1i128 << f);
    let slot = scaled / PI_DEN;
    let rem = scaled % PI_DEN;
    Exact::between(Slot::at(slot), Fraction::of(rem as i64, PI_DEN as i64))
}

/// Twice the remainder against the denominator, which is the tie test.
fn twice_remainder_against_den(f: u32) -> (i128, i128) {
    let scaled = PI_NUM * (1i128 << f);
    ((scaled % PI_DEN) * 2, PI_DEN)
}

// --- the controls ------------------------------------------------------------

#[test]
fn the_control_no_parity_arm_reaches_a_tie() {
    // The arms below are asserted under both nearest modes at once, and that is
    // only honest while none of them lands on a midpoint. A tie is the one
    // position where the two differ, `half_up` going toward positive infinity
    // and `half_even` to the even slot, so an arm reaching one would be
    // asserting two different answers as one.
    for f in [13u32, 14, 5, 6, 3] {
        let (twice, den) = twice_remainder_against_den(f);
        assert_ne!(
            twice, den,
            "pi at fraction length {f} lands on a tie, so the two nearest modes \
             disagree here and the arm is no longer mode-independent"
        );
    }
}

#[test]
fn the_control_no_parity_arm_reaches_an_overflow_policy() {
    // The same argument for the other axis. Every expected stored integer sits
    // inside its declared range, so the completion region is the identity and no
    // arm below is secretly asserting something about wrapping.
    let inside = |lo: Slot, hi: Slot, v: i128| v >= lo.index() && v <= hi.index();
    assert!(inside(
        <<Fi<16, 13> as arvo_format::format::Format>::Slots as Slots>::MIN,
        <<Fi<16, 13> as arvo_format::format::Format>::Slots as Slots>::MAX,
        25_736
    ));
    assert!(inside(
        <<Ufi<16, 14> as arvo_format::format::Format>::Slots as Slots>::MIN,
        <<Ufi<16, 14> as arvo_format::format::Format>::Slots as Slots>::MAX,
        51_472
    ));
    assert!(inside(
        <<Fi<8, 5> as arvo_format::format::Format>::Slots as Slots>::MIN,
        <<Fi<8, 5> as arvo_format::format::Format>::Slots as Slots>::MAX,
        101
    ));
    assert!(inside(
        <<Ufi<8, 6> as arvo_format::format::Format>::Slots as Slots>::MIN,
        <<Ufi<8, 6> as arvo_format::format::Format>::Slots as Slots>::MAX,
        201
    ));
    assert!(inside(
        <<Fi<8, 3> as arvo_format::format::Format>::Slots as Slots>::MIN,
        <<Fi<8, 3> as arvo_format::format::Format>::Slots as Slots>::MAX,
        25
    ));
}

#[test]
fn the_control_the_five_declarations_are_genuinely_different() {
    // If the fraction lengths collapsed, five arms would be one arm reported five
    // times. The step exponent is the coordinate the fraction length sets, so it
    // is what has to differ.
    use arvo_format::format::step_exponent;
    use arvo_format::quantum::Exponent;
    let exponents = [
        step_exponent::<Fi<16, 13>>(Magnitude::SMALLEST),
        step_exponent::<Ufi<16, 14>>(Magnitude::SMALLEST),
        step_exponent::<Fi<8, 5>>(Magnitude::SMALLEST),
        step_exponent::<Ufi<8, 6>>(Magnitude::SMALLEST),
        step_exponent::<Fi<8, 3>>(Magnitude::SMALLEST),
    ];
    assert_eq!(exponents, [
        Exponent::of(-13),
        Exponent::of(-14),
        Exponent::of(-5),
        Exponent::of(-6),
        Exponent::of(-3),
    ]);
}

// --- the fraction length is the negation, including below zero ---------------

#[test]
fn the_fraction_length_is_the_negated_exponent_at_every_sign() {
    use arvo_format::format::step_exponent;
    use arvo_format::quantum::Exponent;
    // Positive, which is every fixed-point declaration anybody writes.
    assert_eq!(
        step_exponent::<Fi<16, 13>>(Magnitude::SMALLEST),
        Exponent::of(-13)
    );
    // Zero, which is the integers and is where MATLAB's `fi` meets `Integer`.
    assert_eq!(
        step_exponent::<Fi<8, 0>>(Magnitude::SMALLEST),
        Exponent::ZERO
    );
    // Negative, which MATLAB admits and which scales the step up rather than
    // down. No case of its own here, which is the point of asserting it.
    assert_eq!(
        step_exponent::<Fi<8, -4>>(Magnitude::SMALLEST),
        Exponent::of(4)
    );
}

#[test]
fn the_fraction_length_is_the_constant_family_rather_than_the_indexed_one() {
    // A fixed-point convention has one step. If this were the indexed family the
    // grid would have a step per magnitude, which is a float and not a `fi`.
    use arvo_format::quantum::{MagnitudeCount, Quantum, is_constant_family};
    assert!(is_constant_family::<FractionLength<13>>().get());
    assert_eq!(
        <FractionLength<13> as Quantum>::MAGNITUDES,
        MagnitudeCount::ONE
    );
}

// --- the five MathWorks publishes --------------------------------------------

/// One published example, run under both nearest modes.
macro_rules! parity {
    ($name:ident, $fmt:ty, $f:literal, $stored:literal, $printed_num:literal, $printed_den:literal) => {
        #[test]
        fn $name() {
            let position = pi_at($f);
            for got in [
                adapt::<Signature<$fmt, Adapt<HalfUp, Wrap>>>(position, Dither::UNUSED),
                adapt::<Signature<$fmt, Adapt<HalfEven, Wrap>>>(position, Dither::UNUSED),
            ] {
                assert_eq!(
                    got,
                    Slot::at($stored),
                    "MathWorks prints a stored integer of {} for this declaration",
                    $stored
                );
            }
            // The real-world value is the stored integer over two to the fraction
            // length. MathWorks prints a five-significant-figure display of it
            // and not the value, so only the last of the five is exact and an
            // arm asserting equality against the printed number is wrong. What
            // holds is that the value rounds to what is printed, which is the
            // difference being at most half a unit in the last printed place.
            //
            // In exact integers, with no float between the two sides: the value
            // scaled by the display denominator is `stored * den`, the printed
            // number scaled the same way is `printed * 2^f`, and half a unit in
            // the last place is `2^f / 2`.
            let value = ($stored as i128) * ($printed_den as i128);
            let printed = ($printed_num as i128) * (1i128 << $f);
            // A whole unit in the last printed place, and the halving is the
            // `* 2` on the other side of the comparison rather than a division
            // here, so the arithmetic stays in exact integers. The name used to
            // say half, which is worth more than a naming quibble because the
            // bound is attained exactly on one arm and the `<=` is therefore
            // load-bearing: `the_bound_is_attained_and_not_merely_respected`
            // pins that, so nobody reads slack into it and relaxes the
            // comparison.
            let a_whole_display_unit = 1i128 << $f;
            assert!(
                (value - printed).abs() * 2 <= a_whole_display_unit,
                "the stored integer carries a value that does not round to the {} \
                 MathWorks prints",
                $printed_num
            );
        }
    };
}

// `a = fi(pi)` prints 3.1416 at word length 16, fraction length 13.
parity!(fi_pi, Fi<16, 13>, 13, 25_736, 31_416, 10_000);

// `a = fi(pi,0)` prints 3.1416 at word length 16, fraction length 14.
parity!(fi_pi_unsigned, Ufi<16, 14>, 14, 51_472, 31_416, 10_000);

// `a = fi(pi,1,8)` prints 3.1562 at word length 8, fraction length 5.
parity!(fi_pi_signed_eight, Fi<8, 5>, 5, 101, 31_562, 10_000);

// `b = fi(pi,0,8)` prints 3.1406 at word length 8, fraction length 6.
parity!(fi_pi_unsigned_eight, Ufi<8, 6>, 6, 201, 31_406, 10_000);

// `a = fi(pi,1,8,3)` prints 3.1250 at word length 8, fraction length 3.
parity!(fi_pi_fraction_three, Fi<8, 3>, 3, 25, 31_250, 10_000);

#[test]
fn the_control_a_wrong_stored_integer_would_be_caught() {
    // The arms above all pass, so on their own they do not establish that the
    // comparison can fail. One position asserted against the neighbour it does
    // not round to, reported rather than asserted, is what says the instrument
    // works.
    let got = adapt::<Signature<Fi<8, 3>, Adapt<HalfEven, Wrap>>>(pi_at(3), Dither::UNUSED);
    assert_ne!(
        got,
        Slot::at(26),
        "the map returned the neighbour it should not have"
    );
    assert_ne!(got, Slot::at(24));
}

// --- MATLAB's two nearest rules ------------------------------------------------
//
// MathWorks documents `Nearest` as nearest with a tie toward positive infinity
// and `Round` as nearest with a tie away from zero. Each is checked at four
// positions: two off the grid say the mode is nearest rather than directed, and
// a positive and a negative tie say where a midpoint goes. Every expected slot
// is MathWorks' number, not arvo's.

/// Whether a mode answers the four positions as `rule` says, where `rule` is
/// `(2.1, 2.9, 2.5, -2.5)` to the slots it names.
macro_rules! answers_like {
    ($mode:ty, $rule:expr) => {{
        type S = Signature<Integer<8>, Adapt<$mode, Wrap>>;
        let at = |slot: i128, num: i64, den: i64| {
            adapt::<S>(
                Exact::between(Slot::at(slot), Fraction::of(num, den)),
                Dither::UNUSED,
            )
            .index()
        };
        let got = [at(2, 1, 10), at(2, 9, 10), at(2, 1, 2), at(-3, 1, 2)];
        got == $rule
    }};
}

/// MATLAB's `Nearest`: `2`, `3`, `3`, `-2`.
const MATLAB_NEAREST: [i128; 4] = [2, 3, 3, -2];

/// MATLAB's `Round`: `2`, `3`, `3`, `-3`.
const MATLAB_ROUND: [i128; 4] = [2, 3, 3, -3];

#[test]
fn half_up_is_matlab_nearest_and_no_other_shipped_mode_is() {
    let answers = [
        answers_like!(TowardZero, MATLAB_NEAREST),
        answers_like!(Floor, MATLAB_NEAREST),
        answers_like!(Ceil, MATLAB_NEAREST),
        answers_like!(HalfUp, MATLAB_NEAREST),
        answers_like!(HalfEven, MATLAB_NEAREST),
        answers_like!(Stochastic, MATLAB_NEAREST),
    ];
    assert_eq!(answers, [false, false, false, true, false, false]);
    // And the standards module names it by MATLAB's word.
    assert!(answers_like!(Nearest, MATLAB_NEAREST));
}

#[test]
fn no_shipped_mode_is_matlab_round() {
    // A tie away from zero is not a mode of this crate. The ruling reaches it as
    // `toward_zero(x + sign(x) q/2)`, which the next arm writes out, so this is
    // pinned as a fact about the vocabulary rather than catalogued as a hole.
    let answers = [
        answers_like!(TowardZero, MATLAB_ROUND),
        answers_like!(Floor, MATLAB_ROUND),
        answers_like!(Ceil, MATLAB_ROUND),
        answers_like!(HalfUp, MATLAB_ROUND),
        answers_like!(HalfEven, MATLAB_ROUND),
        answers_like!(Stochastic, MATLAB_ROUND),
    ];
    assert_eq!(answers, [false; 6]);
    // The control: the two rules differ, so the arm above is not the arm before
    // it with its answer inverted.
    assert_ne!(MATLAB_NEAREST, MATLAB_ROUND);
}

/// The ties-away alias, written from the public surface alone.
///
/// A position `slot + n/d`, with `0 <= n < d`, is negative exactly when `slot`
/// is. Adding half a quantum toward its sign is a position the constructor
/// takes directly, `slot + (2n + d)/(2d)` at or above zero and
/// `slot + (2n - d)/(2d)` below it, and `toward_zero` of that is the alias. The
/// doubled denominator has to fit the fraction, so this returns nothing where
/// `2d` does not.
fn ties_away(slot: i128, n: i64, d: i64) -> Option<i128> {
    type Zero = Signature<Integer<16>, Adapt<TowardZero, Wrap>>;
    let twice_d = d.checked_mul(2)?;
    let twice_n = n.checked_mul(2)?;
    let num = if slot < 0 { twice_n.checked_sub(d)? } else { twice_n.checked_add(d)? };
    let shifted = Exact::between(Slot::at(slot), Fraction::of(num, twice_d));
    Some(adapt::<Zero>(shifted, Dither::UNUSED).index())
}

#[test]
fn a_consumer_can_write_the_ties_away_alias_and_it_is_matlab_round() {
    // MathWorks' four positions.
    let round = [
        ties_away(2, 1, 10),
        ties_away(2, 9, 10),
        ties_away(2, 1, 2),
        ties_away(-3, 1, 2),
    ];
    assert_eq!(round, MATLAB_ROUND.map(Some));

    // Every quarter from `-8` to `8`, against the integer rule for a tie away
    // from zero, `sign(k) floor((|k| + 2) / 4)` for `x = k/4`, which reads no
    // arvo code.
    let mut ties = 0;
    for k in -32i128 ..= 32 {
        let (slot, n) = (k.div_euclid(4), k.rem_euclid(4) as i64);
        let want = k.signum() * ((k.abs() + 2) / 4);
        assert_eq!(ties_away(slot, n, 4), Some(want), "at {k}/4");
        ties += usize::from(n == 2);
    }
    assert_eq!(ties, 16, "the walk reached every tie from -7.5 to 7.5");

    // The control: the alias differs from `half_up` exactly at a negative tie.
    type Up = Signature<Integer<16>, Adapt<HalfUp, Wrap>>;
    let up = |slot: i128, n: i64, d: i64| {
        adapt::<Up>(
            Exact::between(Slot::at(slot), Fraction::of(n, d)),
            Dither::UNUSED,
        )
        .index()
    };
    assert_eq!(ties_away(-3, 1, 2), Some(-3));
    assert_eq!(up(-3, 1, 2), -2);
    assert_eq!(ties_away(2, 1, 2), Some(up(2, 1, 2)));
    assert_eq!(ties_away(-3, 1, 4), Some(up(-3, 1, 4)));
}

#[test]
fn the_alias_cannot_be_written_at_a_denominator_past_half_the_fraction() {
    // The gap the ruling leaves open, pinned as it stands: the alias doubles the
    // denominator, so a position whose remainder is over a denominator past
    // `i64::MAX / 2` has no exact half-quantum shift in the public surface, and
    // no shipped mode takes the tie away from zero directly.
    // FIXME: an exact ties-away over every denominator needs either the shift on
    // the position type or the rule as a mode; the ruling names neither, so the
    // call is the design's rather than this file's.
    assert_eq!(ties_away(0, 1, i64::MAX / 2), Some(0));
    assert_eq!(ties_away(0, 1, i64::MAX / 2 + 1), None);
    assert_eq!(ties_away(-1, 1, i64::MAX), None);
}

/// The bound above is reached, so the comparison may not become strict.
///
/// `fi(pi)` at word length 8 and fraction length 5 stores 101, and MathWorks
/// prints 3.1562. Scaled into exact integers that is `101 * 10000 = 1010000`
/// against `31562 * 2^5 = 1009984`, a difference of 16, doubled to 32, against a
/// whole display unit of `2^5 = 32`. Equal, on the nose.
///
/// Computed by hand while checking a name, which is exactly the check that
/// evaporates and leaves the next reader to redo it. Turning `<=` into `<`
/// fails here and nowhere else in the file.
#[test]
fn the_bound_is_attained_and_not_merely_respected() {
    const F: u32 = 5;
    const STORED: i128 = 101;
    const PRINTED_NUM: i128 = 31_562;
    const PRINTED_DEN: i128 = 10_000;

    let value = STORED * PRINTED_DEN;
    let printed = PRINTED_NUM * (1i128 << F);
    let whole = 1i128 << F;

    assert_eq!(
        (value - printed).abs() * 2,
        whole,
        "the tightest arm sits on the bound rather than inside it"
    );
    assert!(
        (value - printed).abs() * 2 >= whole,
        "and a strict comparison would reject a value MathWorks does print"
    );
}
