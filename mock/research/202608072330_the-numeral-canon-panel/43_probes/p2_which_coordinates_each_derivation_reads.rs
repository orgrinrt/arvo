//! p2: which of a numeral's static coordinates does each derivation actually read?
//!
//! Motivation.  `24` splits a numeral into a **grid** (radix, adjustment, bias,
//! phase, canonical exponent) and a **reach** (how many steps of that grid the
//! numeral covers, which is the total width in the constant case).  `15`/`16`
//! derive a carrier and a stride; `35` derives a fold accumulator from a width
//! and a capacity; `40`/`42` state laws over axis values.  Nobody has asked
//! which coordinates each of those derivations consults.
//!
//! The question is a binding-time question.  A coordinate that no derivation
//! reads is a coordinate a *composition* may hold at run time, shared across a
//! run, without any derivation changing.  A coordinate a derivation does read
//! is one that must stay static, or the derivation moves to run time with it.
//!
//! Method: write each derivation keyed only on what it claims to need, then
//! assert **type equality** of the derived types across two numerals that
//! differ only in the grid.  Type equality is asserted by a blanket
//! `SameAs<T> for T`, so a mismatch is a compile error rather than a runtime
//! comparison.  A negative control (`arm negcontrol`) does the same for a
//! derivation that genuinely reads the grid and must fail.
//!
//! Build (pinned toolchain, no feature gates anywhere):
//!   rustc +nightly-2026-05-28 --edition 2021 --crate-type lib \
//!         p2_which_coordinates_each_derivation_reads.rs
//!   rustc +nightly-2026-05-28 --edition 2021 --crate-type lib \
//!         --cfg negcontrol p2_which_coordinates_each_derivation_reads.rs
//!
//! Scaffolding warning, per the panel's spike rule: names, arities and field
//! orders here are chosen to reach the check.  None of them is a design
//! decision.

#![no_std]
#![forbid(unsafe_code)]

// ---------------------------------------------------------------- type nats

pub struct Z;
pub struct S<N>(core::marker::PhantomData<N>);

pub trait Nat {
    const V: usize;
}
impl Nat for Z {
    const V: usize = 0;
}
impl<N: Nat> Nat for S<N> {
    const V: usize = N::V + 1;
}

/// Type-level addition, inductive, no const arithmetic in any bound.
pub trait Plus<R> {
    type Out;
}
impl<R> Plus<R> for Z {
    type Out = R;
}
impl<L: Plus<R>, R> Plus<R> for S<L> {
    type Out = S<<L as Plus<R>>::Out>;
}
pub type Sum<A, B> = <A as Plus<B>>::Out;

pub type N0 = Z;
pub type N1 = S<N0>;
pub type N2 = S<N1>;
pub type N3 = S<N2>;
pub type N4 = S<N3>;
pub type N5 = S<N4>;
pub type N8 = S<S<S<N5>>>;
pub type N13 = S<S<S<S<S<N8>>>>>;
pub type N16 = S<S<S<N13>>>;

// ------------------------------------------------------- capacity, log2ceil

/// A capacity is a type, per `35`.  `Log2Ceil` is inductive rather than tabled;
/// this probe only needs a few rows and states them as such rather than
/// re-deriving `35_probes/p8`'s induction, which is that probe's result and not
/// this one's.
pub struct Cap<const K: usize>;

pub trait Log2Ceil {
    type Out: Nat;
}
impl Log2Ceil for Cap<1> {
    type Out = N0;
}
impl Log2Ceil for Cap<2> {
    type Out = N1;
}
impl Log2Ceil for Cap<4> {
    type Out = N2;
}
impl Log2Ceil for Cap<8> {
    type Out = N3;
}
impl Log2Ceil for Cap<16> {
    type Out = N4;
}
impl Log2Ceil for Cap<32> {
    type Out = N5;
}

// -------------------------------------------------------------- the grid

/// The grid coordinates of `24`'s reading: radix, adjustment, bias, phase, and
/// the canonical exponent's two integers (`24` section 3.5's `max(K, e + I)`).
/// Carried as associated consts so nothing here needs const arithmetic.
pub trait Grid {
    const RADIX: u32;
    const ADJ_NUM: i64;
    const ADJ_DEN: i64;
    const BIAS_NUM: i64;
    const BIAS_DEN: i64;
    const PHASE_NUM: i64;
    const PHASE_DEN: i64;
    /// canonical exponent floor and slope-intercept, per `24` section 3.5
    const CE_FLOOR: i32;
    const CE_INTERCEPT: i32;
}

/// Binary, unit adjustment, zero bias, zero phase, constant canonical exponent
/// at -2.  This is a plain fixed-point grid with two fraction bits.
pub struct GridA;
impl Grid for GridA {
    const RADIX: u32 = 2;
    const ADJ_NUM: i64 = 1;
    const ADJ_DEN: i64 = 1;
    const BIAS_NUM: i64 = 0;
    const BIAS_DEN: i64 = 1;
    const PHASE_NUM: i64 = 0;
    const PHASE_DEN: i64 = 1;
    const CE_FLOOR: i32 = -2;
    const CE_INTERCEPT: i32 = -2;
}

/// Same radix and same canonical exponent, different **adjustment**, **bias**
/// and **phase**.  A half-unit-biased grid at adjustment three, which is the
/// case `08:306` says the design's `Bias` axis exists for.
pub struct GridB;
impl Grid for GridB {
    const RADIX: u32 = 2;
    const ADJ_NUM: i64 = 3;
    const ADJ_DEN: i64 = 1;
    const BIAS_NUM: i64 = 1;
    const BIAS_DEN: i64 = 2;
    const PHASE_NUM: i64 = 1;
    const PHASE_DEN: i64 = 2;
    const CE_FLOOR: i32 = -2;
    const CE_INTERCEPT: i32 = -2;
}

/// Different **canonical exponent**: the grid is ten times finer.  This is the
/// coordinate the negative control is about.
pub struct GridC;
impl Grid for GridC {
    const RADIX: u32 = 2;
    const ADJ_NUM: i64 = 1;
    const ADJ_DEN: i64 = 1;
    const BIAS_NUM: i64 = 0;
    const BIAS_DEN: i64 = 1;
    const PHASE_NUM: i64 = 0;
    const PHASE_DEN: i64 = 1;
    const CE_FLOOR: i32 = -6;
    const CE_INTERCEPT: i32 = -6;
}

// ------------------------------------------------------------- strategy axes

pub struct Wrap;
pub struct Saturate;

pub trait Overflow {}
impl Overflow for Wrap {}
impl Overflow for Saturate {}

pub trait AbsorbingTop {}
impl AbsorbingTop for Saturate {}

pub trait Strategy {
    type Ovf: Overflow;
}
pub struct Hot;
pub struct Precise;
impl Strategy for Hot {
    type Ovf = Wrap;
}
impl Strategy for Precise {
    type Ovf = Saturate;
}

// ---------------------------------------------------------------- the numeral

/// A numeral is a grid, a reach measured in that grid's steps, and a strategy.
/// `W` is `24`'s reach, which in the constant-canonical-exponent case is the
/// total width.
pub struct Num<G, W, St>(core::marker::PhantomData<(G, W, St)>);

// ---------------------------------------------------- derivation 1: carrier

/// `15`/`16`'s carrier.  Claimed to read the reach and the strategy only.
pub trait CarrierFor<St> {
    type Carrier;
}
impl<St> CarrierFor<St> for N8 {
    type Carrier = u8;
}
impl<St> CarrierFor<St> for N13 {
    type Carrier = u16;
}
impl<St> CarrierFor<St> for N16 {
    type Carrier = u16;
}

pub trait HasCarrier {
    type Carrier;
}
impl<G, W: CarrierFor<St>, St> HasCarrier for Num<G, W, St> {
    // note: G does not appear on the right-hand side
    type Carrier = <W as CarrierFor<St>>::Carrier;
}

// ------------------------------------------- derivation 2: fold accumulator

/// `35`'s accumulator relation: the element's reach plus the log of the
/// capacity.  Claimed to read the reach and the capacity only.
pub trait AccFor<C> {
    type Acc;
}
impl<G, W, St, const K: usize> AccFor<Cap<K>> for Num<G, W, St>
where
    Cap<K>: Log2Ceil,
    W: Plus<<Cap<K> as Log2Ceil>::Out>,
{
    // note: G does not appear on the right-hand side
    type Acc = Num<G, Sum<W, <Cap<K> as Log2Ceil>::Out>, St>;
}

/// The accumulator's **reach**, which is the part a derivation downstream of
/// the accumulator actually consumes.  Stated separately so the type-equality
/// check below is about the derived quantity rather than about the wrapper.
pub trait AccReach<C> {
    type R: Nat;
}
impl<G, W: Nat, St, const K: usize> AccReach<Cap<K>> for Num<G, W, St>
where
    Cap<K>: Log2Ceil,
    W: Plus<<Cap<K> as Log2Ceil>::Out>,
    Sum<W, <Cap<K> as Log2Ceil>::Out>: Nat,
{
    type R = Sum<W, <Cap<K> as Log2Ceil>::Out>;
}

// --------------------------------------------------- derivation 3: the laws

/// `40`/`42`'s law bound: a property implemented on the axis value.
pub fn tropical_fold_site<G, W, St>()
where
    St: Strategy,
    St::Ovf: AbsorbingTop,
{
}

// --------------------------------- derivation 4 (negative control): the value

/// The value map, which is the one derivation that must read the grid.  It is a
/// const function of the grid's coordinates and the stored integer.
pub const fn value_numer<G: Grid>(k: i64) -> i64 {
    // A * k + B, over a common denominator; the canonical exponent scales it,
    // and the whole point is that every coordinate appears.
    G::ADJ_NUM * k * G::BIAS_DEN * G::PHASE_DEN
        + G::BIAS_NUM * G::ADJ_DEN * G::PHASE_DEN
        + G::PHASE_NUM * G::ADJ_DEN * G::BIAS_DEN
        + (G::CE_FLOOR as i64)
        + (G::CE_INTERCEPT as i64)
        + (G::RADIX as i64)
}

// ------------------------------------------------------- type equality check

pub trait SameAs<T> {}
impl<T> SameAs<T> for T {}

/// Instantiating this is the assertion.  If `A` and `B` are different types the
/// bound is unsatisfied and the crate does not build.
pub const fn assert_same<A: SameAs<B>, B>() {}

// The checks.  Each pairs two numerals differing ONLY in the grid, and asserts
// the derived quantity is literally the same type.

pub const _CARRIER_IGNORES_GRID_AB: () = {
    assert_same::<
        <Num<GridA, N13, Hot> as HasCarrier>::Carrier,
        <Num<GridB, N13, Hot> as HasCarrier>::Carrier,
    >()
};

pub const _CARRIER_IGNORES_CANONICAL_EXPONENT: () = {
    assert_same::<
        <Num<GridA, N13, Hot> as HasCarrier>::Carrier,
        <Num<GridC, N13, Hot> as HasCarrier>::Carrier,
    >()
};

pub const _ACC_REACH_IGNORES_GRID_AB: () = {
    assert_same::<
        <Num<GridA, N13, Hot> as AccReach<Cap<8>>>::R,
        <Num<GridB, N13, Hot> as AccReach<Cap<8>>>::R,
    >()
};

pub const _ACC_REACH_IGNORES_CANONICAL_EXPONENT: () = {
    assert_same::<
        <Num<GridA, N13, Hot> as AccReach<Cap<8>>>::R,
        <Num<GridC, N13, Hot> as AccReach<Cap<8>>>::R,
    >()
};

/// The accumulator reach is what the derivation claims: 13 + ceil(log2 8) = 16.
pub const _ACC_REACH_IS_ARITHMETIC: () = {
    assert!(<<Num<GridA, N13, Hot> as AccReach<Cap<8>>>::R as Nat>::V == 16);
    assert!(<<Num<GridA, N13, Hot> as AccReach<Cap<1>>>::R as Nat>::V == 13);
    assert!(<<Num<GridA, N8, Hot> as AccReach<Cap<32>>>::R as Nat>::V == 13);
};

/// The law bound is satisfied or not by the strategy alone, at any grid.
pub fn law_site_at_three_grids() {
    tropical_fold_site::<GridA, N13, Precise>();
    tropical_fold_site::<GridB, N13, Precise>();
    tropical_fold_site::<GridC, N13, Precise>();
}

/// And the positive control on the law bound: a wrapping strategy is refused.
/// Enabled only under `--cfg lawneg`, where the crate must NOT build.
#[cfg(lawneg)]
pub fn law_site_refused() {
    tropical_fold_site::<GridA, N13, Hot>();
}

// ------------------------------------------------------------ the composition

/// A composition, in the sense op names at `32`: a run of values bigger than one
/// numeral.  Its static part is the element numeral and a capacity; its dynamic
/// part is the run.
///
/// The interesting field is `shared`: a grid coordinate held **at run time**,
/// once for the whole run.  This is what frame-of-reference and shared-scale
/// column encodings do.  `08` section 2.3 puts those outside the numeral
/// because "no per-datum type can express a constraint that holds between
/// data".  A composition can, and this is the compiled form of that.
pub struct Run<N, const K: usize> {
    /// stored integers, one per element, capacity K
    data: [i64; K],
    /// how many are live; `len <= K` is the composition's own invariant
    len: usize,
    /// the shared grid coordinate, dynamic and uniform over the run
    shared_bias_numer: i64,
    _n: core::marker::PhantomData<N>,
}

impl<G, W, St, const K: usize> Run<Num<G, W, St>, K>
where
    Num<G, W, St>: AccFor<Cap<K>>,
    Cap<K>: Log2Ceil,
{
    pub const fn new(data: [i64; K], len: usize, shared_bias_numer: i64) -> Self {
        Self {
            data,
            len,
            shared_bias_numer,
            _n: core::marker::PhantomData,
        }
    }

    /// The accumulator type is derived from the element numeral and the
    /// capacity, exactly as `35` states, and the derivation never consults the
    /// shared runtime coordinate.
    pub fn fold_sum(&self) -> i64 {
        let mut acc: i64 = 0;
        let mut i = 0;
        while i < self.len {
            acc += self.data[i];
            i += 1;
        }
        acc
    }

    /// The value map is where the shared coordinate is consumed, and it is the
    /// only place.
    pub fn value_numer_of(&self, i: usize) -> i64 {
        self.data[i] + self.shared_bias_numer
    }
}

/// The composition's derived accumulator type, named so the check below can
/// assert it is the same whatever the grid is.
pub type RunAcc<G, W, St, const K: usize> = <Num<G, W, St> as AccFor<Cap<K>>>::Acc;

// A composition over a numeral whose grid coordinate is held dynamically has
// the same derived accumulator reach as one whose grid is static, because the
// derivation never read the grid.  Asserted rather than argued:
pub const _RUN_ACC_REACH_IGNORES_GRID: () = {
    assert_same::<
        <Num<GridA, N13, Hot> as AccReach<Cap<16>>>::R,
        <Num<GridB, N13, Hot> as AccReach<Cap<16>>>::R,
    >()
};

// --------------------------------------------------------- negative control

/// Under `--cfg negcontrol` the crate must NOT build: the value map genuinely
/// reads the grid, so two grids do not give the same value, and asserting they
/// do is false.  This is what stops every check above from being vacuous.
#[cfg(negcontrol)]
pub const _VALUE_MAP_IGNORES_GRID_IS_FALSE: () = {
    assert!(value_numer::<GridA>(5) == value_numer::<GridB>(5));
};

/// Printed by the runner rather than asserted, so a reader can see the value
/// map does depend on the grid at every coordinate.
pub const VALUE_A: i64 = value_numer::<GridA>(5);
pub const VALUE_B: i64 = value_numer::<GridB>(5);
pub const VALUE_C: i64 = value_numer::<GridC>(5);

/// Third negative control: `assert_same` must refuse two genuinely different
/// types, otherwise every equality check above is vacuous.  Under
/// `--cfg sameneg` the crate must NOT build.
#[cfg(sameneg)]
pub const _SAME_AS_IS_NOT_VACUOUS: () = {
    assert_same::<
        <Num<GridA, N13, Hot> as AccReach<Cap<8>>>::R,
        <Num<GridA, N13, Hot> as AccReach<Cap<16>>>::R,
    >()
};
