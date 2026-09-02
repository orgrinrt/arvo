// The document's own declarations, transcribed from 110_consolidation_eleven.md
// sections 1.2 (110:874-887) and 1.23 (110:3140-3210, 3262-3286).
// Three additions the document does not state, per 110:6415-6422.
#![no_std]
#![feature(const_trait_impl)]
#![allow(dead_code)]

// stand-in for notko::ConstTry (110:6418-6419)
pub trait ConstTry {
    type Output;
}

mod sealed {
    pub trait Sealed {}
}
use sealed::Sealed;

pub const trait Pos: Sealed {
    const VAL: u64;
}
pub struct H;
pub struct O<P: Pos>(PhantomData<P>);
pub struct I<P: Pos>(PhantomData<P>);
impl Sealed for H {}
impl<P: Pos> Sealed for O<P> {}
impl<P: Pos> Sealed for I<P> {}
const impl Pos for H {
    const VAL: u64 = 1;
}
const impl<P: [const] Pos> Pos for O<P> {
    const VAL: u64 = 2 * P::VAL;
}
const impl<P: [const] Pos> Pos for I<P> {
    const VAL: u64 = 2 * P::VAL + 1;
}

pub const trait Nat: Sealed {
    const VAL: u64;
}
pub struct Z;
pub struct Pz<P: Pos>(PhantomData<P>);
impl Sealed for Z {}
impl<P: Pos> Sealed for Pz<P> {}
const impl Nat for Z {
    const VAL: u64 = 0;
}
const impl<P: [const] Pos> Nat for Pz<P> {
    const VAL: u64 = P::VAL;
}

pub const trait AtLeastTwo: [const] Pos {}
const impl<P: [const] Pos> AtLeastTwo for O<P> {}
const impl<P: [const] Pos> AtLeastTwo for I<P> {}

pub const trait Exponent: Sealed {}
pub struct EZero;
pub struct EPos<P: Pos>(PhantomData<P>);
pub struct ENeg<P: Pos>(PhantomData<P>);
impl Sealed for EZero {}
impl<P: Pos> Sealed for EPos<P> {}
impl<P: Pos> Sealed for ENeg<P> {}
const impl Exponent for EZero {}
const impl<P: Pos> Exponent for EPos<P> {}
const impl<P: Pos> Exponent for ENeg<P> {}

pub const trait Radix: Sealed {}
pub struct Rad<P: AtLeastTwo>(PhantomData<P>);
impl<P: AtLeastTwo> Sealed for Rad<P> {}
const impl<P: AtLeastTwo> Radix for Rad<P> {}

pub const trait Bias: Sealed {}
pub struct BZero;
impl Sealed for BZero {}
const impl Bias for BZero {}

pub trait Precision: Nat {} // note: nullary MARKER trait, 110:3176
impl<T: Nat> Precision for T {}
pub trait Adjustment: Bias {}
impl<T: Bias> Adjustment for T {}

pub const trait ExponentForm: Sealed {}
pub const trait SignDomain: Sealed {}
pub struct NonNegative;
pub struct Symmetric;
pub struct AsymmetricLow;
impl Sealed for NonNegative {}
impl Sealed for Symmetric {}
impl Sealed for AsymmetricLow {}
const impl SignDomain for NonNegative {}
const impl SignDomain for Symmetric {}
const impl SignDomain for AsymmetricLow {}

pub const trait Underflow: Sealed {}
pub struct Gradual;
pub struct Abrupt;
impl Sealed for Gradual {}
impl Sealed for Abrupt {}
const impl Underflow for Gradual {}
const impl Underflow for Abrupt {}

pub const trait Specials: Sealed {}
pub struct NoSpecials;
pub struct IeeeSpecials;
impl Sealed for NoSpecials {}
impl Sealed for IeeeSpecials {}
const impl Specials for NoSpecials {}
const impl Specials for IeeeSpecials {}

// 110:884-887, with the file-118 PhantomData repair.
pub struct Implicit<E: Exponent, A: Adjustment, B: Bias>(PhantomData<(E, A, B)>);
pub struct Ranged<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials>(
    PhantomData<(EMIN, EMAX, U, S)>,
);
impl<E: Exponent, A: Adjustment, B: Bias> Sealed for Implicit<E, A, B> {}
impl<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials> Sealed
    for Ranged<EMIN, EMAX, U, S>
{
}
const impl<E: Exponent, A: Adjustment, B: Bias> ExponentForm for Implicit<E, A, B> {}
const impl<EMIN: Exponent, EMAX: Exponent, U: Underflow, S: Specials> ExponentForm
    for Ranged<EMIN, EMAX, U, S>
{
}

// 110:3083-3088
pub const trait Numeral {
    type Radix: Radix;
    type Precision: Precision;
    type Exponent: ExponentForm;
    type Domain: SignDomain;
}

// The quantiser vocabulary, 110:3262-3286.
pub const trait Resolution {}
pub const trait Direction: [const] Resolution {}
pub struct TowardNegative;
pub struct TowardPositive;
pub struct ToEven;
pub struct ToOdd;
const impl Resolution for TowardNegative {}
const impl Direction for TowardNegative {}
const impl Resolution for TowardPositive {}
const impl Direction for TowardPositive {}
const impl Resolution for ToEven {}
const impl Direction for ToEven {}
const impl Resolution for ToOdd {}
const impl Direction for ToOdd {}
pub struct ReduceModulo;
pub struct Refuse;
pub struct FarPoint;
const impl Resolution for ReduceModulo {}
const impl Resolution for Refuse {}
const impl Resolution for FarPoint {}

pub const trait Quantisation {
    type UnderMidpoint: Direction;
    type OnMidpoint: Direction;
    type OverMidpoint: Direction;
    type OverRange: Resolution;
    type UnderRange: Resolution;
}

pub const trait Encoding {}
pub const trait StoredWidth: Sealed {}
pub struct Minimum;
pub struct DoubleLogical;
impl Sealed for Minimum {}
impl Sealed for DoubleLogical {}
const impl StoredWidth for Minimum {}
const impl StoredWidth for DoubleLogical {}

pub const trait StorageLayout: Sealed {}
pub struct Dense;
pub struct Bitpacked;
impl Sealed for Dense {}
impl Sealed for Bitpacked {}
const impl StorageLayout for Dense {}
const impl StorageLayout for Bitpacked {}

pub const trait LoweringDoor: Sealed {}
pub struct Inert;
pub struct Quantised;
pub struct HostFloat<E>(PhantomData<E>);
impl Sealed for Inert {}
impl Sealed for Quantised {}
impl<E> Sealed for HostFloat<E> {}
const impl LoweringDoor for Inert {}
const impl LoweringDoor for Quantised {}
const impl<E> LoweringDoor for HostFloat<E> {}
pub struct DefaultEnv;

// The two concrete numerals the probes need: one Implicit (fixed-point), one Ranged (float).
pub struct U13F3;
const impl Numeral for U13F3 {
    type Radix = Rad<O<H>>;
    type Precision = Pz<O<O<O<O<H>>>>>; // 16
    type Exponent = Implicit<ENeg<I<H>>, BZero, BZero>;
    type Domain = NonNegative;
}
pub struct U14F2;
const impl Numeral for U14F2 {
    type Radix = Rad<O<H>>;
    type Precision = Pz<O<O<O<O<H>>>>>; // 16
    type Exponent = Implicit<ENeg<O<H>>, BZero, BZero>;
    type Domain = NonNegative;
}
pub struct Binary32;
const impl Numeral for Binary32 {
    type Radix = Rad<O<H>>;
    type Precision = Pz<I<I<O<H>>>>; // 24-ish, value irrelevant here
    type Exponent = Ranged<ENeg<I<H>>, EPos<I<H>>, Gradual, IeeeSpecials>;
    type Domain = Symmetric;
}
