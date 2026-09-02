//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Parity against what MathWorks publishes, and the two gaps it turned up.
//!
//! Every expected number here comes from the `fi` reference page rather than
//! from arvo. An arm that computed both sides through arvo would assert that
//! arvo agrees with itself, which passes for the wrong reason and is the failure
//! this file would be most likely to ship.

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::apply::{adapt, Dither, Exact, Fraction};
use arvo_format::overflow::Wrap;
use arvo_format::points::Integer;
use arvo_format::quantum::Magnitude;
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, Stochastic, TowardZero};
use arvo_format::slots::{Slot, Slots};

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
/// scaled by the denominator, leaves `i64`; the two values handed back do not.
fn pi_at(f: u32) -> Exact {
    let scaled = PI_NUM * (1i128 << f);
    let slot = scaled / PI_DEN;
    let rem = scaled % PI_DEN;
    Exact::between(
        Slot::at(slot as i64),
        Fraction::of(rem as i64, PI_DEN as i64),
    )
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
    // position where the two disagree, and which way it should go is open in the
    // registry, so an arm reaching one would be closing that question quietly.
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
    let inside = |lo: Slot, hi: Slot, v: i64| v >= lo.index() && v <= hi.index();
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
    assert_eq!(
        exponents,
        [
            Exponent::of(-13),
            Exponent::of(-14),
            Exponent::of(-5),
            Exponent::of(-6),
            Exponent::of(-3),
        ]
    );
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
    use arvo_format::quantum::{is_constant_family, MagnitudeCount, Quantum};
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
            // length. **MathWorks prints a five-significant-figure display of it
            // and not the value**, so only the last of the five is exact and an
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

// --- what MATLAB needs and the vocabulary does not have ----------------------

/// Whether a mode is nearest with ties toward positive infinity.
///
/// Four positions decide it. Two off-grid ones say the mode is nearest rather
/// than directed, and the two ties say which way a midpoint goes. MathWorks
/// documents its Nearest as exactly this.
macro_rules! is_matlab_nearest {
    ($mode:ty) => {{
        type S = Signature<Integer<8>, Adapt<$mode, Wrap>>;
        let below = adapt::<S>(
            Exact::between(Slot::at(2), Fraction::of(1, 10)),
            Dither::UNUSED,
        ) == Slot::at(2);
        let above = adapt::<S>(
            Exact::between(Slot::at(2), Fraction::of(9, 10)),
            Dither::UNUSED,
        ) == Slot::at(3);
        let tie_up = adapt::<S>(
            Exact::between(Slot::at(2), Fraction::of(1, 2)),
            Dither::UNUSED,
        ) == Slot::at(3);
        let tie_down = adapt::<S>(
            Exact::between(Slot::at(-3), Fraction::of(1, 2)),
            Dither::UNUSED,
        ) == Slot::at(-2);
        below && above && tie_up && tie_down
    }};
}

#[test]
#[ignore = "catalogue: MATLAB's Nearest, ties toward positive infinity, has no mode in the ratified \
            vocabulary; closed by question::is_the_rounding_vocabulary_complete_at_six and \
            question::which_tie_direction_an_unqualified_nearest_names"]
fn some_shipped_mode_is_matlab_nearest() {
    // Red on purpose, and it is the finding rather than a defect in this crate.
    // MATLAB needs two nearest-with-ties operations and arvo names one, so one of
    // the two has nowhere to land whatever `half_up` turns out to mean. This goes
    // green when the vocabulary gains the name, not when anybody edits it.
    let any = is_matlab_nearest!(TowardZero)
        || is_matlab_nearest!(Floor)
        || is_matlab_nearest!(Ceil)
        || is_matlab_nearest!(HalfUp)
        || is_matlab_nearest!(HalfEven)
        || is_matlab_nearest!(Stochastic);
    assert!(
        any,
        "no shipped mode rounds to nearest with ties toward positive infinity"
    );
}

#[test]
fn the_gap_is_this_shape_rather_than_a_missing_re_export() {
    // The catalogued arm above is ignored by default, so on its own it says
    // nothing to a normal run. This one runs, and it pins why each candidate
    // fails, so the gap cannot be closed by somebody adding a re-export and
    // assuming it lines up.
    type Ceiling = Signature<Integer<8>, Adapt<Ceil, Wrap>>;
    type Away = Signature<Integer<8>, Adapt<HalfUp, Wrap>>;
    type Even = Signature<Integer<8>, Adapt<HalfEven, Wrap>>;

    // Ceiling agrees at both ties and is not nearest: it takes the upper
    // neighbour from a position nine tenths below it.
    assert_eq!(
        adapt::<Ceiling>(
            Exact::between(Slot::at(2), Fraction::of(1, 2)),
            Dither::UNUSED
        ),
        Slot::at(3)
    );
    assert_eq!(
        adapt::<Ceiling>(
            Exact::between(Slot::at(-3), Fraction::of(1, 2)),
            Dither::UNUSED
        ),
        Slot::at(-2)
    );
    assert_eq!(
        adapt::<Ceiling>(
            Exact::between(Slot::at(2), Fraction::of(1, 10)),
            Dither::UNUSED
        ),
        Slot::at(3)
    );

    // The shipped nearest-not-to-even mode is nearest and takes the negative tie
    // away from zero, which is MATLAB's Round rather than its Nearest.
    assert_eq!(
        adapt::<Away>(
            Exact::between(Slot::at(2), Fraction::of(1, 10)),
            Dither::UNUSED
        ),
        Slot::at(2)
    );
    assert_eq!(
        adapt::<Away>(
            Exact::between(Slot::at(2), Fraction::of(1, 2)),
            Dither::UNUSED
        ),
        Slot::at(3)
    );
    assert_eq!(
        adapt::<Away>(
            Exact::between(Slot::at(-3), Fraction::of(1, 2)),
            Dither::UNUSED
        ),
        Slot::at(-3)
    );

    // And the even mode is nearest and takes the positive tie down.
    assert_eq!(
        adapt::<Even>(
            Exact::between(Slot::at(2), Fraction::of(1, 2)),
            Dither::UNUSED
        ),
        Slot::at(2)
    );
    assert_eq!(
        adapt::<Even>(
            Exact::between(Slot::at(-3), Fraction::of(1, 2)),
            Dither::UNUSED
        ),
        Slot::at(-2)
    );
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
