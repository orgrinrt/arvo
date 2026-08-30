//! UNION PROBE stage 1: resolutions, witnessed, including Refuse.
#![feature(const_trait_impl)]
#![allow(dead_code)]

// ===========================================================================
// 0. truth markers and the type-level boolean algebra
// ===========================================================================

pub trait TruthMarker {
    const VALUE: bool;
}
pub struct True;
pub struct False;
impl TruthMarker for True {
    const VALUE: bool = true;
}
impl TruthMarker for False {
    const VALUE: bool = false;
}

pub trait And<B> {
    type Out;
}
impl And<True> for True {
    type Out = True;
}
impl And<False> for True {
    type Out = False;
}
impl<B> And<B> for False {
    type Out = False;
}

pub trait Or<B> {
    type Out;
}
impl<B> Or<B> for True {
    type Out = True;
}
impl Or<True> for False {
    type Out = True;
}
impl Or<False> for False {
    type Out = False;
}

// ===========================================================================
// 1. the ONE semantic definition of a resolution: its recovery map.
//
// 07 probe A made phi total (`fn phi(..) -> i32`) and could therefore only
// classify ReduceModulo, Clamp and SubstituteZero. `Refuse` has no total
// recovery map. Made partial here, which is also what 01 finding 1's Kleene
// equality requires for the Refuse rows to be well-posed.
// ===========================================================================

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Rec {
    At(i32),
    Refused,
}

pub const trait Resolve {
    /// Recover an exact value that landed outside [min, max], or refuse.
    fn phi(x: i32, min: i32, max: i32) -> Rec;
}

/// 01 finding 3's translation-stability identity, under Kleene equality:
/// phi(phi(x) + c) == phi(x + c), where "both refuse" counts as equal.
pub const fn stable<R: [const] Resolve>(min: i32, max: i32, two_sided: bool) -> bool {
    let lo = if two_sided { min + min } else { 0 };
    let hi = max + max;
    let mut x = lo;
    while x <= hi {
        let clo = if two_sided { min } else { 0 };
        let mut c = clo;
        while c <= max {
            let lhs = match R::phi(x, min, max) {
                Rec::At(v) => R::phi(v + c, min, max),
                Rec::Refused => Rec::Refused,
            };
            let rhs = R::phi(x + c, min, max);
            let eq = match (lhs, rhs) {
                (Rec::At(a), Rec::At(b)) => a == b,
                (Rec::Refused, Rec::Refused) => true,
                _ => false,
            };
            if !eq {
                return false;
            }
            c += 1;
        }
        x += 1;
    }
    true
}

/// Does this recovery map ever refuse, over the bounded domain? This is the
/// GRADE, computed from the same phi, so the fallibility classification stops
/// being an assertion (01 finding 6) at the leaf, not only at the fold.
pub const fn ever_refuses<R: [const] Resolve>(min: i32, max: i32) -> bool {
    let mut x = min + min;
    let hi = max + max;
    while x <= hi {
        if let Rec::Refused = R::phi(x, min, max) {
            return true;
        }
        x += 1;
    }
    false
}

// ===========================================================================
// 2. the classification, witnessed.
// ===========================================================================

pub trait Resolution: const Resolve + Sized {
    type StableOneSided: TruthMarker;
    type StableTwoSided: TruthMarker;
    /// the grade: does this rule refuse
    type Refuses: TruthMarker;
    const WITNESS: () = {
        assert!(
            stable::<Self>(0, 7, false) == <Self::StableOneSided as TruthMarker>::VALUE,
            "declared one-sided stability disagrees with this rule's own recovery map"
        );
        assert!(
            stable::<Self>(-8, 7, true) == <Self::StableTwoSided as TruthMarker>::VALUE,
            "declared two-sided stability disagrees with this rule's own recovery map"
        );
        assert!(
            ever_refuses::<Self>(-8, 7) == <Self::Refuses as TruthMarker>::VALUE,
            "declared fallibility disagrees with this rule's own recovery map"
        );
    };
}

// --- the resolution constructors ------------------------------------------

pub struct ReduceModulo;
const impl Resolve for ReduceModulo {
    fn phi(x: i32, min: i32, max: i32) -> Rec {
        let span = max - min + 1;
        Rec::At((x - min).rem_euclid(span) + min)
    }
}
impl Resolution for ReduceModulo {
    type StableOneSided = True;
    type StableTwoSided = True;
    type Refuses = False;
}

pub struct TowardNegative;
const impl Resolve for TowardNegative {
    fn phi(x: i32, min: i32, max: i32) -> Rec {
        Rec::At(if x > max {
            max
        } else if x < min {
            min
        } else {
            x
        })
    }
}
impl Resolution for TowardNegative {
    type StableOneSided = True;
    type StableTwoSided = False;
    type Refuses = False;
}

pub struct TowardPositive;
const impl Resolve for TowardPositive {
    fn phi(x: i32, min: i32, max: i32) -> Rec {
        Rec::At(if x > max {
            max
        } else if x < min {
            min
        } else {
            x
        })
    }
}
impl Resolution for TowardPositive {
    type StableOneSided = True;
    type StableTwoSided = False;
    type Refuses = False;
}

pub struct SubstituteZero;
const impl Resolve for SubstituteZero {
    fn phi(x: i32, min: i32, max: i32) -> Rec {
        Rec::At(if x > max || x < min { 0 } else { x })
    }
}
impl Resolution for SubstituteZero {
    type StableOneSided = False;
    type StableTwoSided = False;
    type Refuses = False;
}

pub struct Refuse;
const impl Resolve for Refuse {
    fn phi(x: i32, min: i32, max: i32) -> Rec {
        if x > max || x < min {
            Rec::Refused
        } else {
            Rec::At(x)
        }
    }
}
impl Resolution for Refuse {
    type StableOneSided = True; // 01 finding 1's Refuse/unsigned row, Kleene
    type StableTwoSided = False; // 01's (127+1)+(-1) counterexample
    type Refuses = True;
}

// --- eager forcing sites in the declaring crate (07's route 1) -------------

const _: () = <ReduceModulo as Resolution>::WITNESS;
const _: () = <TowardNegative as Resolution>::WITNESS;
const _: () = <TowardPositive as Resolution>::WITNESS;
const _: () = <SubstituteZero as Resolution>::WITNESS;
const _: () = <Refuse as Resolution>::WITNESS;

// ===========================================================================
// STAGE 2: the ten axes, nominal, split parameters, delivery on Lowering,
// graded fallibility, computed law. All five standing proposals at once.
// ===========================================================================

use core::marker::PhantomData;

// --- 3. carriers and deliveries (05 sec 2: delivery is a Lowering member) --

pub trait Carrier<T: Copy> {
    fn from_output(v: T) -> Self;
}
#[derive(Debug, Copy, Clone)]
pub struct Total<T>(pub T);
impl<T: Copy> Carrier<T> for Total<T> {
    fn from_output(v: T) -> Self {
        Total(v)
    }
}
#[derive(Debug, Copy, Clone)]
pub enum Fallible<T> {
    Ok(T),
    Refused,
}
impl<T: Copy> Carrier<T> for Fallible<T> {
    fn from_output(v: T) -> Self {
        Fallible::Ok(v)
    }
}
/// 05 probe E's absorbing bottom: same refusal, delivered as data.
#[derive(Debug, Copy, Clone)]
pub struct Poison<T> {
    pub v: T,
    pub bottom: bool,
}
impl<T: Copy> Carrier<T> for Poison<T> {
    fn from_output(v: T) -> Self {
        Poison { v, bottom: false }
    }
}

/// A delivery interprets a GRADE (True = can refuse) as a carrier.
pub trait Deliver<G> {
    type C<T: Copy>: Carrier<T>;
    fn refuse<T: Copy>(nearest: T) -> Self::C<T>;
}
/// Grade subsumption, once per (delivery, grade pair).
pub trait LiftD<G1, G2>: Deliver<G1> + Deliver<G2> {
    fn lift<T: Copy>(x: <Self as Deliver<G1>>::C<T>) -> <Self as Deliver<G2>>::C<T>;
}

pub struct AsSum;
impl Deliver<False> for AsSum {
    type C<T: Copy> = Total<T>;
    fn refuse<T: Copy>(nearest: T) -> Total<T> {
        Total(nearest)
    }
}
impl Deliver<True> for AsSum {
    type C<T: Copy> = Fallible<T>;
    fn refuse<T: Copy>(_n: T) -> Fallible<T> {
        Fallible::Refused
    }
}
impl LiftD<False, False> for AsSum {
    fn lift<T: Copy>(x: Total<T>) -> Total<T> {
        x
    }
}
impl LiftD<False, True> for AsSum {
    fn lift<T: Copy>(x: Total<T>) -> Fallible<T> {
        Fallible::Ok(x.0)
    }
}
impl LiftD<True, True> for AsSum {
    fn lift<T: Copy>(x: Fallible<T>) -> Fallible<T> {
        x
    }
}

pub struct AsBottom;
impl Deliver<False> for AsBottom {
    type C<T: Copy> = Total<T>;
    fn refuse<T: Copy>(nearest: T) -> Total<T> {
        Total(nearest)
    }
}
impl Deliver<True> for AsBottom {
    type C<T: Copy> = Poison<T>;
    fn refuse<T: Copy>(nearest: T) -> Poison<T> {
        Poison {
            v: nearest,
            bottom: true,
        }
    }
}
impl LiftD<False, False> for AsBottom {
    fn lift<T: Copy>(x: Total<T>) -> Total<T> {
        x
    }
}
impl LiftD<False, True> for AsBottom {
    fn lift<T: Copy>(x: Total<T>) -> Poison<T> {
        Poison {
            v: x.0,
            bottom: false,
        }
    }
}
impl LiftD<True, True> for AsBottom {
    fn lift<T: Copy>(x: Poison<T>) -> Poison<T> {
        x
    }
}

// --- 4. the ten axes -------------------------------------------------------

// identity markers
pub struct Unsigned;
pub struct Signed;
pub struct Unit;
pub struct FullRange<const F: u16>;
pub struct Zero;
pub struct AtOrigin<const B: i32>;
pub struct Gradual;
pub struct Flushed;
pub struct Unbounded;
// policy markers
pub struct ToEven;
pub struct TowardZero;
pub struct Exact;
pub struct NarrowedToOperand;
// lowering markers
pub struct Minimum;
pub struct DoubleLogical;
pub struct NoWidening;
pub struct InContainer;
pub struct PerOperation;
pub struct Dense;
pub struct Bitpacked;

pub trait Signedness {
    type TwoSided: TruthMarker;
}
impl Signedness for Unsigned {
    type TwoSided = False;
}
impl Signedness for Signed {
    type TwoSided = True;
}

pub trait Numeral {
    type ExponentForm;
    type Adjustment;
    type Bias;
    type Sign: Signedness;
    const LOGICAL_WIDTH: u16;
    // 06 sec 7: the significand derivation needs no type position.
    const EXPONENT_FIELD: u16;
    const SIGN_BITS: u16;
    const SIGNIFICAND: u16 = Self::LOGICAL_WIDTH - Self::EXPONENT_FIELD - Self::SIGN_BITS;
}

pub trait Quantisation {
    type UnderMidpoint;
    type OnMidpoint;
    type OverMidpoint;
    type OverRange: Resolution;
    type UnderRange: Resolution;
    // NOTE: no `type Fallibility<T>`. Deleted per 07 sec 2 / 01 finding 6.
}
pub trait Policy {
    type Quantisation: Quantisation;
    type Growth;
}
pub trait Lowering {
    type StoredWidth;
    type Widening;
    type Layout;
    /// 05 sec 2's proposed axis. Nothing above may condition on it.
    type Delivery;
}

// --- nominal numerals (06 sec 4) ------------------------------------------

pub struct Fix<const I: u16, const F: u16, S>(PhantomData<S>);
impl<const I: u16, const F: u16, S: Signedness> Numeral for Fix<I, F, S> {
    type ExponentForm = Implicit<F>;
    type Adjustment = Unit;
    type Bias = Zero;
    type Sign = S;
    const LOGICAL_WIDTH: u16 = I + F;
    const EXPONENT_FIELD: u16 = 0;
    const SIGN_BITS: u16 = 0;
}
pub struct Implicit<const E: u16>;
pub struct Stored<const B: u16, U>(PhantomData<U>);

pub struct Flt<const E: u16, const M: u16, U>(PhantomData<U>);
impl<const E: u16, const M: u16, U> Numeral for Flt<E, M, U> {
    type ExponentForm = Stored<E, U>;
    type Adjustment = Unit;
    type Bias = Zero;
    type Sign = Signed;
    const LOGICAL_WIDTH: u16 = 1 + E + M;
    const EXPONENT_FIELD: u16 = E;
    const SIGN_BITS: u16 = 1;
}

pub struct Unorm<const F: u16>;
impl<const F: u16> Numeral for Unorm<F> {
    type ExponentForm = Implicit<F>;
    type Adjustment = FullRange<F>;
    type Bias = Zero;
    type Sign = Unsigned;
    const LOGICAL_WIDTH: u16 = F;
    const EXPONENT_FIELD: u16 = 0;
    const SIGN_BITS: u16 = 0;
}

// --- nominal presets (06 sec 4) -------------------------------------------

pub struct HotQ;
impl Quantisation for HotQ {
    type UnderMidpoint = TowardZero;
    type OnMidpoint = TowardZero;
    type OverMidpoint = TowardZero;
    type OverRange = ReduceModulo;
    type UnderRange = ReduceModulo;
}
pub struct Hot;
impl Policy for Hot {
    type Quantisation = HotQ;
    type Growth = NarrowedToOperand;
}
impl Lowering for Hot {
    type StoredWidth = Minimum;
    type Widening = NoWidening;
    type Layout = Dense;
    type Delivery = AsSum;
}

pub struct WarmQ;
impl Quantisation for WarmQ {
    type UnderMidpoint = ToEven;
    type OnMidpoint = ToEven;
    type OverMidpoint = ToEven;
    type OverRange = TowardNegative;
    type UnderRange = TowardPositive;
}
pub struct Warm;
impl Policy for Warm {
    type Quantisation = WarmQ;
    type Growth = Exact;
}
impl Lowering for Warm {
    type StoredWidth = DoubleLogical;
    type Widening = InContainer;
    type Layout = Dense;
    type Delivery = AsSum;
}

pub struct ColdQ;
impl Quantisation for ColdQ {
    type UnderMidpoint = ToEven;
    type OnMidpoint = ToEven;
    type OverMidpoint = ToEven;
    type OverRange = TowardNegative;
    type UnderRange = TowardPositive;
}
pub struct Cold;
impl Policy for Cold {
    type Quantisation = ColdQ;
    type Growth = Exact;
}
impl Lowering for Cold {
    type StoredWidth = Minimum;
    type Widening = PerOperation;
    type Layout = Bitpacked;
    type Delivery = AsSum;
}

pub struct PreciseQ;
impl Quantisation for PreciseQ {
    type UnderMidpoint = ToEven;
    type OnMidpoint = ToEven;
    type OverMidpoint = ToEven;
    type OverRange = Refuse;
    type UnderRange = Refuse;
}
pub struct Precise;
impl Policy for Precise {
    type Quantisation = PreciseQ;
    type Growth = Exact;
}
impl Lowering for Precise {
    type StoredWidth = DoubleLogical;
    type Widening = PerOperation;
    type Layout = Dense;
    type Delivery = AsSum;
}

// --- 5. modifiers (06 sec 5), one per axis, delegating ---------------------

pub struct OverRangeOf<S, R>(PhantomData<(S, R)>);
pub struct OverRangeQ<Q, R>(PhantomData<(Q, R)>);
impl<Q: Quantisation, R: Resolution> Quantisation for OverRangeQ<Q, R> {
    type UnderMidpoint = Q::UnderMidpoint;
    type OnMidpoint = Q::OnMidpoint;
    type OverMidpoint = Q::OverMidpoint;
    type OverRange = R;
    type UnderRange = Q::UnderRange;
}
impl<S: Policy, R: Resolution> Policy for OverRangeOf<S, R> {
    type Quantisation = OverRangeQ<S::Quantisation, R>;
    type Growth = S::Growth;
}
impl<S: Lowering, R> Lowering for OverRangeOf<S, R> {
    type StoredWidth = S::StoredWidth;
    type Widening = S::Widening;
    type Layout = S::Layout;
    type Delivery = S::Delivery;
}

pub struct UnderRangeOf<S, R>(PhantomData<(S, R)>);
pub struct UnderRangeQ<Q, R>(PhantomData<(Q, R)>);
impl<Q: Quantisation, R: Resolution> Quantisation for UnderRangeQ<Q, R> {
    type UnderMidpoint = Q::UnderMidpoint;
    type OnMidpoint = Q::OnMidpoint;
    type OverMidpoint = Q::OverMidpoint;
    type OverRange = Q::OverRange;
    type UnderRange = R;
}
impl<S: Policy, R: Resolution> Policy for UnderRangeOf<S, R> {
    type Quantisation = UnderRangeQ<S::Quantisation, R>;
    type Growth = S::Growth;
}
impl<S: Lowering, R> Lowering for UnderRangeOf<S, R> {
    type StoredWidth = S::StoredWidth;
    type Widening = S::Widening;
    type Layout = S::Layout;
    type Delivery = S::Delivery;
}

pub struct LayoutOf<S, X>(PhantomData<(S, X)>);
impl<S: Policy, X> Policy for LayoutOf<S, X> {
    type Quantisation = S::Quantisation;
    type Growth = S::Growth;
}
impl<S: Lowering, X> Lowering for LayoutOf<S, X> {
    type StoredWidth = S::StoredWidth;
    type Widening = S::Widening;
    type Layout = X;
    type Delivery = S::Delivery;
}

/// 06 sec 13's suggestion: delivery spelled as a named modifier.
pub struct DeliveredAs<S, D>(PhantomData<(S, D)>);
impl<S: Policy, D> Policy for DeliveredAs<S, D> {
    type Quantisation = S::Quantisation;
    type Growth = S::Growth;
}
impl<S: Lowering, D> Lowering for DeliveredAs<S, D> {
    type StoredWidth = S::StoredWidth;
    type Widening = S::Widening;
    type Layout = S::Layout;
    type Delivery = D;
}

// --- 6. the composition ----------------------------------------------------

pub struct Number<N, S>(PhantomData<(N, S)>);

pub type OverRes<P> = <<P as Policy>::Quantisation as Quantisation>::OverRange;
pub type UnderRes<P> = <<P as Policy>::Quantisation as Quantisation>::UnderRange;
pub type OverG<P> = <OverRes<P> as Resolution>::Refuses;
pub type UnderG<P> = <UnderRes<P> as Resolution>::Refuses;
pub type JoinG<P> = <OverG<P> as Or<UnderG<P>>>::Out;

// ===========================================================================
// STAGE 3: the law derivation and the graded aggregate, together.
// ===========================================================================

/// project a resolution's stability at the domain's sidedness
pub trait StableFor<D> {
    type Out: TruthMarker;
}
impl<R: Resolution> StableFor<False> for R {
    type Out = R::StableOneSided;
}
impl<R: Resolution> StableFor<True> for R {
    type Out = R::StableTwoSided;
}

/// 06 sec 6's repair: the verdict marker carries the composition so the
/// attribute has the consumer's type in scope.
#[diagnostic::on_unimplemented(
    message = "`{C}` has no associative addition",
    label = "this composition cannot be folded",
    note = "Its out-of-range rule is not translation-stable on this domain. Fold under a wrapping out-of-range rule, or accumulate in a wider numeral and quantise once at the end."
)]
pub trait Proves<C> {}
impl<C> Proves<C> for True {}

pub trait AddAssoc {}

/// The law. Note what it does NOT mention: `L`.
impl<N: Numeral, P: Policy> AddAssoc for Number<N, P>
where
    OverRes<P>: StableFor<<N::Sign as Signedness>::TwoSided>,
    UnderRes<P>: StableFor<<N::Sign as Signedness>::TwoSided>,
    <OverRes<P> as StableFor<<N::Sign as Signedness>::TwoSided>>::Out:
        And<<UnderRes<P> as StableFor<<N::Sign as Signedness>::TwoSided>>::Out>,
    <<OverRes<P> as StableFor<<N::Sign as Signedness>::TwoSided>>::Out as And<
        <UnderRes<P> as StableFor<<N::Sign as Signedness>::TwoSided>>::Out,
    >>::Out: Proves<Number<N, P>>,
{
}

pub fn fold<T: AddAssoc>() {}

// --- the graded aggregate, with the interpretation on Lowering ------------

pub trait Arith {
    type Answer<T: Copy>: Carrier<T>;
    fn ok<T: Copy>(v: T) -> Self::Answer<T>;
    fn over<T: Copy>(nearest: T) -> Self::Answer<T>;
    fn under<T: Copy>(nearest: T) -> Self::Answer<T>;
}

impl<N: Numeral, P: Policy + Lowering> Arith for Number<N, P>
where
    OverG<P>: Or<UnderG<P>>,
    <P as Lowering>::Delivery: Deliver<OverG<P>>
        + Deliver<UnderG<P>>
        + Deliver<JoinG<P>>
        + LiftD<OverG<P>, JoinG<P>>
        + LiftD<UnderG<P>, JoinG<P>>,
{
    type Answer<T: Copy> = <<P as Lowering>::Delivery as Deliver<JoinG<P>>>::C<T>;
    fn ok<T: Copy>(v: T) -> Self::Answer<T> {
        <Self::Answer<T> as Carrier<T>>::from_output(v)
    }
    fn over<T: Copy>(nearest: T) -> Self::Answer<T> {
        // THE DOOR: check the declared classification against this rule's own
        // recovery map, directly, so an impl-side override cannot disarm it
        // (07 probe a7).
        const {
            assert!(
                stable::<OverRes<P>>(0, 7, false)
                    == <<OverRes<P> as Resolution>::StableOneSided as TruthMarker>::VALUE
                    && ever_refuses::<OverRes<P>>(-8, 7)
                        == <<OverRes<P> as Resolution>::Refuses as TruthMarker>::VALUE,
                "this resolution's declared classification disagrees with its own recovery map"
            )
        };
        <<P as Lowering>::Delivery as LiftD<OverG<P>, JoinG<P>>>::lift(
            <<P as Lowering>::Delivery as Deliver<OverG<P>>>::refuse(nearest),
        )
    }
    fn under<T: Copy>(nearest: T) -> Self::Answer<T> {
        <<P as Lowering>::Delivery as LiftD<UnderG<P>, JoinG<P>>>::lift(
            <<P as Lowering>::Delivery as Deliver<UnderG<P>>>::refuse(nearest),
        )
    }
}

/// The arithmetic. ONE bound.
pub fn add<C: Arith>(a: u16, b: u16, min: u16, max: u16) -> C::Answer<u16> {
    match a.checked_add(b) {
        Some(v) if v <= max && v >= min => C::ok(v),
        Some(_) => C::over(max),
        None => C::over(max),
    }
}

// --- the ten compositions --------------------------------------------------

pub type UFixed<const I: u16, const F: u16, S> = Number<Fix<I, F, Unsigned>, S>;
pub type IFixed<const I: u16, const F: u16, S> = Number<Fix<I, F, Signed>, S>;

pub fn c1() {
    fold::<IFixed<13, 3, Hot>>()
} // signed wrap folds
pub fn c2() {
    fold::<UFixed<13, 3, Warm>>()
} // unsigned clamp folds
pub fn c3() {
    fold::<UFixed<13, 3, Precise>>()
} // unsigned refuse folds (Kleene)
pub fn c4() {
    fold::<IFixed<13, 3, UnderRangeOf<OverRangeOf<Warm, ReduceModulo>, ReduceModulo>>>()
} // both range rules swapped through two stacked modifiers
pub fn c5() {
    fold::<Number<Unorm<8>, Hot>>()
}
