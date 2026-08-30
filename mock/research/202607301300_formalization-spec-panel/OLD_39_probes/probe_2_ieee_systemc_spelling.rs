//! Probe 2: the post-collapse vocabulary spells the IEEE and SystemC constructions.
//!
//! Hypothesis: after file 35 (no `Widening` on `Lowering`, no `Growth` on `Policy`)
//! the trait shapes in 38:318-356 still carry, as types:
//!
//! (a) IEEE 754 decimal32 as ONE `Numeral` (radix ten, precision 7, exponent
//!     range -95..=96, gradual underflow, specials) with TWO `Lowering`s whose
//!     `Encoding::Fields` differ (BID and DPD), the standard's own
//!     format-versus-encoding separation. The check is a function that demands
//!     two compositions share one `Numeral` while their encodings are distinct
//!     types; it type-checks for (BID, DPD) over decimal32.
//!
//! (b) SystemC `sc_fixed<8, 3, SC_RND, SC_SAT>` as a `Numeral` (precision 8,
//!     implicit exponent -5) plus a `Quantisation` whose midpoint takes a
//!     Direction (SC_RND = toward positive on tie) and whose range ends take a
//!     Resolution (SC_SAT = clamp). SC_WRAP maps to ReduceModulo; SC_SAT_ZERO
//!     to SubstituteZero. `ReduceModulo` at the midpoint slot refuses (the
//!     consolidation's own compile gate, 26:47-48), pinned here as a trait-bound
//!     refusal in a comment rather than a live line so the file compiles; the
//!     refusal itself is exercised by flipping the marked line.
//!
//! (c) SystemC's deferred (per-assignment) quantisation and MATLAB's
//!     per-operation modes as the two call patterns of one mul_full/quantize
//!     pair, with NO Widening axis anywhere: the product numeral is a named
//!     type, the quantise is a named map. This is the shape 26:236-243 verified
//!     for values; here it is shown to need no axis to spell.
//!
//! This is a spelling probe: it establishes that the vocabulary carries the
//! constructions as types. The arithmetic behind them was verified by files
//! 25, 33, 35 and is not re-verified here.

#![allow(dead_code)]

use core::marker::PhantomData;

// ---- the post-collapse contracts, per 38:318-356 --------------------------

trait Radix {}
struct Two;
struct Ten;
impl Radix for Two {}
impl Radix for Ten {}

trait Precision {}
struct P7;
struct P8;
struct P24;
impl Precision for P7 {}
impl Precision for P8 {}
impl Precision for P24 {}

trait ExponentForm {}
trait Underflow {}
struct Gradual;
impl Underflow for Gradual {}
trait Specials {}
struct Ieee754Specials; // qNaN, sNaN, +-inf
impl Specials for Ieee754Specials {}
struct NoSpecials;
impl Specials for NoSpecials {}

struct Ranged<const EMIN: i32, const EMAX: i32, U: Underflow, S: Specials>(PhantomData<(U, S)>);
impl<const EMIN: i32, const EMAX: i32, U: Underflow, S: Specials> ExponentForm
    for Ranged<EMIN, EMAX, U, S>
{
}

trait Adjustment {}
struct Unit;
impl Adjustment for Unit {}
trait Bias {}
struct Zero;
impl Bias for Zero {}
struct Implicit<const E: i32, A: Adjustment, B: Bias>(PhantomData<(A, B)>);
impl<const E: i32, A: Adjustment, B: Bias> ExponentForm for Implicit<E, A, B> {}

trait SignDomain {}
struct NonNegative;
struct Symmetric;
struct AsymmetricLow;
impl SignDomain for NonNegative {}
impl SignDomain for Symmetric {}
impl SignDomain for AsymmetricLow {}

trait Numeral {
    type Radix: Radix;
    type Precision: Precision;
    type Exponent: ExponentForm;
    type Domain: SignDomain;
}

// Policy, post-35: Quantisation only. No Growth.
trait Resolution {}
trait Direction: Resolution {}
struct TowardPositive;
impl Resolution for TowardPositive {}
impl Direction for TowardPositive {}
struct ToEven;
impl Resolution for ToEven {}
impl Direction for ToEven {}
struct Clamp<D: Direction>(PhantomData<D>); // clamping is a direction at a range end
impl<D: Direction> Resolution for Clamp<D> {}
struct TowardZero;
impl Resolution for TowardZero {}
impl Direction for TowardZero {}
struct ReduceModulo;
impl Resolution for ReduceModulo {} // NOT a Direction: the consolidation's gate
struct SubstituteZero;
impl Resolution for SubstituteZero {}

// midpoint slot demands Direction; range-end slots take any Resolution
struct Quant<Mid: Direction, Top: Resolution, Bot: Resolution>(PhantomData<(Mid, Top, Bot)>);
trait Policy {
    type Quantisation;
}

// Lowering, post-35: no Widening.
trait SignIndexing {}
struct TwosComplement;
impl SignIndexing for TwosComplement {}
struct SignMagnitude;
impl SignIndexing for SignMagnitude {}
trait FieldLayout {}
struct BidFields; // binary integer significand encoding of decimal32
struct DpdFields; // densely packed decimal encoding of decimal32
impl FieldLayout for BidFields {}
impl FieldLayout for DpdFields {}
trait Canonicalisation {}
struct PreferredExponent; // decimal cohort selection lives here
impl Canonicalisation for PreferredExponent {}

trait Encoding {
    type SignIndexing: SignIndexing;
    type Fields: FieldLayout;
    type Canonical: Canonicalisation;
}
trait Lowering {
    type Encoding: Encoding;
    // StoredWidth, Layout elided: not load-bearing for the spelling question
}

// ---- (a) decimal32: one format, two encodings -----------------------------

struct Decimal32;
impl Numeral for Decimal32 {
    type Radix = Ten;
    type Precision = P7;
    type Exponent = Ranged<-95, 96, Gradual, Ieee754Specials>;
    type Domain = Symmetric;
}

struct Bid;
impl Encoding for Bid {
    type SignIndexing = SignMagnitude;
    type Fields = BidFields;
    type Canonical = PreferredExponent;
}
struct Dpd;
impl Encoding for Dpd {
    type SignIndexing = SignMagnitude;
    type Fields = DpdFields;
    type Canonical = PreferredExponent;
}

struct LowerVia<E: Encoding>(PhantomData<E>);
impl<E: Encoding> Lowering for LowerVia<E> {
    type Encoding = E;
}

struct Number<N: Numeral, L: Lowering>(PhantomData<(N, L)>);

// demands: same Numeral, distinct Encoding types
fn one_format_two_encodings<N, EA, EB>(
    _a: PhantomData<Number<N, LowerVia<EA>>>,
    _b: PhantomData<Number<N, LowerVia<EB>>>,
) where
    N: Numeral,
    EA: Encoding,
    EB: Encoding,
    (EA, EB): DistinctPair,
{
}
trait DistinctPair {}
impl DistinctPair for (Bid, Dpd) {} // Bid != Dpd witnessed by which impl exists

const _: () = {
    // the call type-checks: decimal32 is one Numeral under two Lowerings
    let _ = one_format_two_encodings::<Decimal32, Bid, Dpd>;
};

// ---- (b) sc_fixed<8, 3, SC_RND, SC_SAT> -----------------------------------

// W = 8 total digits, IW = 3 integer digits => quantum 2^-5, precision 8.
struct ScFixed8x3;
impl Numeral for ScFixed8x3 {
    type Radix = Two;
    type Precision = P8;
    type Exponent = Implicit<-5, Unit, Zero>;
    type Domain = AsymmetricLow; // two's complement asymmetric range
}

// SC_RND: round to nearest, tie toward plus infinity. SC_SAT: clamp both ends.
type ScRndSat = Quant<TowardPositive, Clamp<TowardZero>, Clamp<TowardZero>>;
// SC_TRN + SC_WRAP (the default sc_fixed mode): truncate + wrap.
type ScTrnWrap = Quant<TowardZero, ReduceModulo, ReduceModulo>;
// SC_SAT_ZERO: substitute zero past either end.
type ScSatZero = Quant<ToEven, SubstituteZero, SubstituteZero>;
// SC_SAT_SYM is NOT a Quantisation: it is a Domain (Symmetric numeral), per
// 31 section 1.2, and is spelled by swapping AsymmetricLow for Symmetric above.

// The refusal gate: ReduceModulo at the midpoint slot refuses with E0277
// (`the trait bound ReduceModulo: Direction is not satisfied`), verified in a
// scratch copy during authoring, IN A CHECKED POSITION (a fn parameter):
//   fn illegal(_q: Quant<ReduceModulo, ReduceModulo, ReduceModulo>) {}
// A first attempt used a bare `type Illegal = Quant<...>` alias and COMPILED
// CLEAN, because Rust does not enforce a struct's bounds at a type-alias
// declaration, only at use sites. So the consolidation's compile gate
// (26:47-48) holds at every position where the type is used and NOT at an
// alias that merely names it; a preset table written as bare aliases would
// not exercise the gate until something consumed the alias. Kept per the
// panel's practice of recording the probe the compiler killed.

// ---- (c) deferred vs per-operation quantisation, no axis ------------------

// the exact product of Q(I1,F1) x Q(I2,F2) is a NAMED numeral (P8 stands in
// for the width-summed precision; the real adder is 25/33/36 machinery)
struct Prod;
impl Numeral for Prod {
    type Radix = Two;
    type Precision = P24;
    type Exponent = Implicit<-10, Unit, Zero>;
    type Domain = AsymmetricLow;
}

fn mul_full<L: Lowering>(_a: Number<ScFixed8x3, L>, _b: Number<ScFixed8x3, L>) -> Number<Prod, L> {
    Number(PhantomData)
}
fn quantize<Src: Numeral, Dst: Numeral, Q, L: Lowering>(_x: Number<Src, L>) -> Number<Dst, L> {
    Number(PhantomData)
}

// SystemC shape: exact interior, one quantise at the assignment
fn systemc_assignment<L: Lowering>(
    a: Number<ScFixed8x3, L>,
    b: Number<ScFixed8x3, L>,
) -> Number<ScFixed8x3, L> {
    let wide = mul_full(a, b); // named product numeral, no Widening axis
    quantize::<Prod, ScFixed8x3, ScRndSat, L>(wide) // fires at the store
}

// MATLAB per-operation shape: identical body, quantise placed by the caller
// immediately; SpecifyPrecision is quantize into a THIRD named numeral,
// which the old Widening axis (None | InContainer | PerOperation) could not
// name at all.
struct Spec12;
impl Numeral for Spec12 {
    type Radix = Two;
    type Precision = P7;
    type Exponent = Implicit<-3, Unit, Zero>;
    type Domain = AsymmetricLow;
}
fn matlab_specify_precision<L: Lowering>(
    a: Number<ScFixed8x3, L>,
    b: Number<ScFixed8x3, L>,
) -> Number<Spec12, L> {
    quantize::<Prod, Spec12, ScRndSat, L>(mul_full(a, b))
}

fn main() {
    println!("probe 2: the constructions spell");
}
