//! s3: one definition of a numeral, projected to both vocabularies.
//!
//! The claim under compile: a numeral is a GRID cut down to a REACH, and both the
//! concept vocabulary of `08` and the width vocabulary of `15` are namings of that one
//! pair. If that is true, a derivation written once against grid-and-reach must produce
//! the design's own quantities when instantiated at the constant-canonical-exponent case,
//! and must also produce sensible quantities at the cases the width pair cannot name.
//!
//! Nothing here is a proposed API. Names, arities and field orders are scaffolding to
//! reach the check, per the panel's spike rule.
//!
//! Feature gates: NONE. Uses only const generics with associated consts, no arithmetic in
//! any bound, so no `generic_const_exprs` and no `generic_const_args`. No `dyn`, no
//! `TypeId`, no `alloc`. Built with the repository's pinned nightly.
//!
//! Build:
//!   rustc --edition 2021 --crate-type lib s3_one_definition_two_projections.rs \
//!         -o /dev/null
//! Run the checks:
//!   rustc --edition 2021 s3_one_definition_two_projections.rs -o s3 && ./s3

#![no_std]
#![forbid(unsafe_code)]

// ---------------------------------------------------------------------------------
// The one definition.
// ---------------------------------------------------------------------------------

/// The grid: which magnitudes are denotable, expressed the way `08:190-192` expresses
/// it. Inside radix-binade `e` the step is `ADJ * RADIX^{f(e)}` at a fixed phase, where
/// `f` is the canonical exponent. Every canonical exponent this design has a name for is
/// affine in `e`, so `f` is carried as a slope and an intercept rather than as a table.
/// That is a restriction of `08`'s function space and it is stated as one: it covers
/// constant, slope one and slope two, and does not cover the knee.
pub trait Grid {
    const RADIX: i64;
    /// numerator and denominator of the slope of `f`, so slope one is `1/1` and a
    /// half-slope stays expressible without leaving the integers.
    const SLOPE_NUM: i64;
    const SLOPE_DEN: i64;
    const INTERCEPT: i64;
    /// the affine value map's `A` and `B`, from `seed/SETTLED_laws.md:274`, and the phase
    /// as a rational multiple of the step.
    const ADJ_NUM: i64;
    const ADJ_DEN: i64;
    const BIAS_NUM: i64;
    const BIAS_DEN: i64;
    const PHASE_NUM: i64;
    const PHASE_DEN: i64;

    /// A floor under the affine piece, which is what turns a slope into a knee. A grid
    /// with no knee sets this far below the reach so it never wins. See `s5`, which
    /// establishes that every canonical exponent the design names is `max(K, e + I)` and
    /// that the family is closed under the meet and not under the join.
    const FLOOR: i64;

    /// f(e). This is the whole of the difference between the families (`08:191`).
    fn canonical_exponent(e: i64) -> i64 {
        let affine = (Self::SLOPE_NUM * e) / Self::SLOPE_DEN + Self::INTERCEPT;
        if affine > Self::FLOOR {
            affine
        } else {
            Self::FLOOR
        }
    }
}

/// The reach: which binades the numeral covers. `EMIN` is the binade of the smallest
/// non-zero denotable magnitude; `EMAX` is the last binade covered, so the exclusive top
/// of the reach is `RADIX^(EMAX+1)`.
pub trait Reach {
    const EMIN: i64;
    const EMAX: i64;
    /// whether zero is denotable. Kept separate because it is a reach question and not a
    /// grid question, and because `08:583` holds the endpoints aside deliberately.
    const HAS_ZERO: bool;
}

/// A numeral is a grid and a reach. That is the whole definition.
pub trait Numeral {
    type G: Grid;
    type R: Reach;
}

// ---------------------------------------------------------------------------------
// The derivation, written once, against the definition and nothing else.
// ---------------------------------------------------------------------------------

/// How many magnitudes binade `e` denotes under this grid. Binade `e` spans
/// `[r^e, r^{e+1})`, a span of `r^e * (r-1)`, and the step there is `r^{f(e)}` scaled by
/// the adjustment. Integer arithmetic only; both exponents are small in every case here.
pub fn magnitudes_in_binade<G: Grid>(e: i64) -> i64 {
    let fe = G::canonical_exponent(e);
    // span / step, in units of r^0, computed as r^(e - f(e)) * (r - 1) / adj
    let k = e - fe;
    if k < 0 {
        return 0;
    }
    let mut span = 1i64;
    let mut i = 0;
    while i < k {
        span = span.saturating_mul(G::RADIX);
        i += 1;
    }
    span.saturating_mul(G::RADIX - 1) * G::ADJ_DEN / G::ADJ_NUM
}

/// How many magnitudes the whole numeral denotes. This is the quantity the width
/// vocabulary calls the value count, and it is where the two vocabularies meet.
pub fn magnitude_count<N: Numeral>() -> i64 {
    let mut total: i64 = if <N::R as Reach>::HAS_ZERO { 1 } else { 0 };
    let mut e = <N::R as Reach>::EMIN;
    while e <= <N::R as Reach>::EMAX {
        total = total.saturating_add(magnitudes_in_binade::<N::G>(e));
        e += 1;
    }
    total
}

/// The step in the numeral's finest binade, as the exponent of the radix. For a constant
/// canonical exponent this is the same in every binade and it is what the design calls
/// the fraction width, negated.
pub fn finest_step_exponent<N: Numeral>() -> i64 {
    <N::G as Grid>::canonical_exponent(<N::R as Reach>::EMIN)
}

/// The binade the reach stops at, exclusive. For a constant canonical exponent this is
/// what the design calls the integer width.
pub fn reach_top_binade<N: Numeral>() -> i64 {
    <N::R as Reach>::EMAX + 1
}

// ---------------------------------------------------------------------------------
// Projection one: the width vocabulary. `15:105-124`.
// ---------------------------------------------------------------------------------

/// The constant-canonical-exponent grid at radix two, unit adjustment, zero bias, zero
/// phase. `F` is the negated canonical exponent, which is the whole of the projection.
pub struct BinaryFixedGrid<const F: u32>;

impl<const F: u32> Grid for BinaryFixedGrid<F> {
    const RADIX: i64 = 2;
    const SLOPE_NUM: i64 = 0;
    const SLOPE_DEN: i64 = 1;
    const INTERCEPT: i64 = -(F as i64);
    const ADJ_NUM: i64 = 1;
    const ADJ_DEN: i64 = 1;
    const BIAS_NUM: i64 = 0;
    const BIAS_DEN: i64 = 1;
    const PHASE_NUM: i64 = 0;
    const PHASE_DEN: i64 = 1;
    const FLOOR: i64 = -1_000_000;
}

/// The reach a total width names: `2^W` steps of size `2^-F`, anchored at zero, so the
/// magnitudes run over binades `-F` through `W - F - 1`.
pub struct WidthReach<const W: u32, const F: u32>;

impl<const W: u32, const F: u32> Reach for WidthReach<W, F> {
    const EMIN: i64 = -(F as i64);
    const EMAX: i64 = W as i64 - F as i64 - 1;
    const HAS_ZERO: bool = true;
}

/// A numeral named by a width pair. `I = W - F` is deliberately absent: it is a view.
pub struct WF<const W: u32, const F: u32>;

impl<const W: u32, const F: u32> Numeral for WF<W, F> {
    type G = BinaryFixedGrid<F>;
    type R = WidthReach<W, F>;
}

// ---------------------------------------------------------------------------------
// Projection two: a family the width vocabulary cannot name. `08:308`.
// ---------------------------------------------------------------------------------

/// Canonical exponent of slope one: `f(e) = e - p + 1`. The precision `P` is the grid
/// coordinate; there is no fraction width, because the step is different in every binade.
pub struct BinaryFloatGrid<const P: u32>;

impl<const P: u32> Grid for BinaryFloatGrid<P> {
    const RADIX: i64 = 2;
    const SLOPE_NUM: i64 = 1;
    const SLOPE_DEN: i64 = 1;
    const INTERCEPT: i64 = 1 - P as i64;
    const ADJ_NUM: i64 = 1;
    const ADJ_DEN: i64 = 1;
    const BIAS_NUM: i64 = 0;
    const BIAS_DEN: i64 = 1;
    const PHASE_NUM: i64 = 0;
    const PHASE_DEN: i64 = 1;
    const FLOOR: i64 = -1_000_000;
}

/// An exponent-range reach, which is what a float's two remaining coordinates name.
pub struct BinadeReach<const NEG_EMIN: u32, const EMAX: u32>;

impl<const NEG_EMIN: u32, const EMAX: u32> Reach for BinadeReach<NEG_EMIN, EMAX> {
    const EMIN: i64 = -(NEG_EMIN as i64);
    const EMAX: i64 = EMAX as i64;
    const HAS_ZERO: bool = true;
}

pub struct Flt<const P: u32, const NEG_EMIN: u32, const EMAX: u32>;

impl<const P: u32, const NEG_EMIN: u32, const EMAX: u32> Numeral for Flt<P, NEG_EMIN, EMAX> {
    type G = BinaryFloatGrid<P>;
    type R = BinadeReach<NEG_EMIN, EMAX>;
}

// ---------------------------------------------------------------------------------
// Projection three: slope two, which is outside the design entirely. `08:311`.
// ---------------------------------------------------------------------------------

pub struct TaperedGrid<const P: u32>;

impl<const P: u32> Grid for TaperedGrid<P> {
    const RADIX: i64 = 2;
    const SLOPE_NUM: i64 = 2;
    const SLOPE_DEN: i64 = 1;
    const INTERCEPT: i64 = 1 - P as i64;
    const ADJ_NUM: i64 = 1;
    const ADJ_DEN: i64 = 1;
    const BIAS_NUM: i64 = 0;
    const BIAS_DEN: i64 = 1;
    const PHASE_NUM: i64 = 0;
    const PHASE_DEN: i64 = 1;
    const FLOOR: i64 = -1_000_000;
}

pub struct Tapered<const P: u32, const NEG_EMIN: u32, const EMAX: u32>;

impl<const P: u32, const NEG_EMIN: u32, const EMAX: u32> Numeral for Tapered<P, NEG_EMIN, EMAX> {
    type G = TaperedGrid<P>;
    type R = BinadeReach<NEG_EMIN, EMAX>;
}

// ---------------------------------------------------------------------------------
// Projection four: the knee, which is where s5 lands. `08:309`, `08:420-431`.
// ---------------------------------------------------------------------------------

/// Gradual underflow: constant below a knee, slope one above. Two integers, no list, no
/// table, no enumeration. `KNEE_AT` is the binade the taper stops at.
pub struct KneeGrid<const P: u32, const NEG_KNEE_AT: u32>;

impl<const P: u32, const NEG_KNEE_AT: u32> Grid for KneeGrid<P, NEG_KNEE_AT> {
    const RADIX: i64 = 2;
    const SLOPE_NUM: i64 = 1;
    const SLOPE_DEN: i64 = 1;
    const INTERCEPT: i64 = 1 - P as i64;
    const ADJ_NUM: i64 = 1;
    const ADJ_DEN: i64 = 1;
    const BIAS_NUM: i64 = 0;
    const BIAS_DEN: i64 = 1;
    const PHASE_NUM: i64 = 0;
    const PHASE_DEN: i64 = 1;
    // f(e) = max(KNEE, e - P + 1), with the floor at the knee binade's own exponent.
    const FLOOR: i64 = -(NEG_KNEE_AT as i64) - P as i64 + 1;
}

pub struct Knee<const P: u32, const NEG_KNEE_AT: u32, const NEG_EMIN: u32, const EMAX: u32>;

impl<const P: u32, const NEG_KNEE_AT: u32, const NEG_EMIN: u32, const EMAX: u32> Numeral
    for Knee<P, NEG_KNEE_AT, NEG_EMIN, EMAX>
{
    type G = KneeGrid<P, NEG_KNEE_AT>;
    type R = BinadeReach<NEG_EMIN, EMAX>;
}

// ---------------------------------------------------------------------------------
// The checks.
// ---------------------------------------------------------------------------------

/// The load-bearing one. The general derivation, instantiated at the constant case,
/// must produce `2^W`. If it does, the design's total width IS the reach measured in
/// steps, and that is the sentence the seam needs.
pub fn check_width_pair_recovers_two_to_the_w() -> bool {
    // W and F are const parameters, so the box is written out rather than looped.
    macro_rules! row {
        ($w:literal, $f:literal) => {{
            let n = magnitude_count::<WF<$w, $f>>();
            let want = 1i64 << $w;
            let fse = finest_step_exponent::<WF<$w, $f>>();
            let top = reach_top_binade::<WF<$w, $f>>();
            (n == want) && (fse == -($f as i64)) && (top == ($w as i64) - ($f as i64))
        }};
    }
    row!(0, 0)
        && row!(1, 0)
        && row!(1, 1)
        && row!(1, 4)
        && row!(1, 8)
        && row!(4, 0)
        && row!(4, 2)
        && row!(4, 4)
        && row!(8, 3)
        && row!(8, 8)
        && row!(10, 5)
        && row!(12, 12)
}

/// A compile-time version of the same check, so the claim is not merely a runtime pass.
const _: () = {
    // 2^4 = 16 magnitudes for W=4 at any F.
    assert!(WIDTH_COUNT_4_2 == 16);
    assert!(WIDTH_COUNT_4_0 == 16);
    assert!(WIDTH_COUNT_1_32 == 2);
};

pub const WIDTH_COUNT_4_2: i64 = width_count(4, 2);
pub const WIDTH_COUNT_4_0: i64 = width_count(4, 0);
pub const WIDTH_COUNT_1_32: i64 = width_count(1, 32);

/// The same walk as `magnitude_count`, written as a free const fn over the projection's
/// own numbers. Kept separate because a generic const fn calling trait items would need
/// const traits, which is a gate this probe does not take.
const fn width_count(w: i64, f: i64) -> i64 {
    let mut total = 1; // zero
    let mut e = -f;
    while e <= w - f - 1 {
        // binade e under a constant canonical exponent -f: 2^(e+f) magnitudes
        let k = e + f;
        let mut span = 1i64;
        let mut i = 0;
        while i < k {
            span *= 2;
            i += 1;
        }
        total += span; // times (radix - 1) = 1
        e += 1;
    }
    total
}

// The checks are driven from `s3_run.rs`, which is std-linked scaffolding. Everything
// above this line is `no_std` and compiles as a library, which is the constraint that
// matters.

/// Reported so the driver can print them without re-deriving anything.
pub fn float_counts() -> (i64, i64, i64) {
    (
        magnitude_count::<Flt<4, 3, 3>>(),
        magnitude_count::<Flt<3, 2, 2>>(),
        magnitude_count::<Tapered<4, 3, 3>>(),
    )
}

/// The discriminating fact: a constant canonical exponent has one step exponent across
/// every binade, so a fraction width names it. A slope-one one does not.
pub fn step_exponent_spread() -> ((i64, i64), (i64, i64)) {
    (
        (
            <BinaryFixedGrid<2> as Grid>::canonical_exponent(-2),
            <BinaryFixedGrid<2> as Grid>::canonical_exponent(5),
        ),
        (
            <BinaryFloatGrid<4> as Grid>::canonical_exponent(-3),
            <BinaryFloatGrid<4> as Grid>::canonical_exponent(3),
        ),
    )
}

/// The knee's step exponent at three binades, so the driver can show it is constant
/// below the knee and sloped above without re-deriving anything.
pub fn knee_profile() -> (i64, i64, i64, i64, i64) {
    type G = KneeGrid<4, 6>; // p = 4, knee at binade -6
    (
        <G as Grid>::canonical_exponent(-9),
        <G as Grid>::canonical_exponent(-7),
        <G as Grid>::canonical_exponent(-6),
        <G as Grid>::canonical_exponent(-4),
        <G as Grid>::canonical_exponent(0),
    )
}

/// And the whole numeral's magnitude count through the SAME derivation, which is the
/// point: no new machinery was needed for the knee.
pub fn knee_count() -> i64 {
    magnitude_count::<Knee<4, 6, 8, 3>>()
}
