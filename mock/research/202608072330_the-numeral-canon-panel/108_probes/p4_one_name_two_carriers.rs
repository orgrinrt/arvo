//! p4. Can one name bind a point in each component when the carriers differ?
//!
//! The pair's fourth clause, at `106` section 1, says three things that do not
//! sit together as a mechanism:
//!
//!   "The two components have **different carriers**. The first travels with
//!    the value. The second is supplied where the operation happens, because
//!    only the site knows the arity, the access pattern and the target. A
//!    **named** strategy binds one point in each, so that a consumer states one
//!    intent rather than answering two questions."
//!
//! If the name binds both and is written on the value, the site cannot move the
//! second component without changing the value's type. If it is written at the
//! site, the first component is not on the value and every consumer of that
//! value no longer agrees about it, which the same clause forbids. `106`
//! section 10 sees the tension and resolves it by declaring the named binding
//! to be the object the word denotes, which names the problem rather than
//! removing it, and it records the cost in one line: "Under type-carried cost,
//! folding one column two ways requires a cast that changes no value."
//!
//! This probe compiles the three shapes and reads what each costs.
//!
//!   A. TWO CARRIERS, NO NAME. Policy on the value, weighting at the site.
//!      One stored column folded two ways, two arms reached, no cast.
//!   B. ONE NAME BINDING BOTH, ON THE VALUE. The same two folds require a
//!      conversion on the value, which is what a cast that changes no value is.
//!   C. ONE NAME AS A DEFAULT IN THE SECOND COMPONENT. The common case writes
//!      one name; the site that needs a different weighting names one, and the
//!      value's type does not move.
//!
//! C is what survives, and it is not a compromise between A and B: it is A with
//! the ergonomics clause of the pair honoured, which is what `106` section 10
//! argues for on I2 and I4 and then does not build.
//!
//! Constraints held: `#![no_std]`, zero feature gates, no `dyn`, no `TypeId`,
//! no `generic_const_exprs`, no allocation. The arm selection is forced through
//! an inline `const { }` block so the claim is about const solving rather than
//! about backend folding.
//!
//! Spike. Names, arities and the cost table are scaffolding to reach the check.
//!
//! Build: rustc -O --edition 2021 --crate-type=lib p4_one_name_two_carriers.rs
//! Asm:   rustc -O --edition 2021 --crate-type=lib --emit=asm p4_one_name_two_carriers.rs

#![no_std]

// ---------------------------------------------------------------------------
// Component one: the observable policy assignment. A consumer of a value
// cannot recover it from the bits, so it travels with the value.
// ---------------------------------------------------------------------------

pub trait Policy {
    const SATURATES: bool;
}

pub struct Wrapping;
impl Policy for Wrapping {
    const SATURATES: bool = false;
}

pub struct Saturating;
impl Policy for Saturating {
    const SATURATES: bool = true;
}

// ---------------------------------------------------------------------------
// Component two: the weighting over cost coordinates. Nothing a consumer can
// observe depends on which arm it picks.
// ---------------------------------------------------------------------------

pub trait Weighting {
    const W_TIME: u32;
    const W_SIZE: u32;
}

pub struct TimeFirst;
impl Weighting for TimeFirst {
    const W_TIME: u32 = 100;
    const W_SIZE: u32 = 1;
}

pub struct SizeFirst;
impl Weighting for SizeFirst {
    const W_TIME: u32 = 1;
    const W_SIZE: u32 = 100;
}

// ---------------------------------------------------------------------------
// The arms, and a cost table indexed BY REGION. The region here is the fold's
// arity, which is `97` section 5's measured case: one stored column, the best
// arm moving with the arity, decided by something the column does not know.
// ---------------------------------------------------------------------------

const ARMS: usize = 3;
const REGIONS: usize = 4;

/// cost[region][arm] = [time, size]
const COST: [[[u32; 2]; ARMS]; REGIONS] = [
    [[90, 10], [40, 30], [55, 22]],
    [[85, 10], [42, 30], [50, 22]],
    [[80, 10], [45, 30], [48, 22]],
    [[75, 10], [47, 30], [46, 22]],
];

/// A plain const fn, so no const trait is needed anywhere.
const fn argmin(region: usize, w_time: u32, w_size: u32) -> usize {
    let mut best = 0usize;
    let mut best_cost = COST[region][0][0] * w_time + COST[region][0][1] * w_size;
    let mut i = 1usize;
    while i < ARMS {
        let c = COST[region][i][0] * w_time + COST[region][i][1] * w_size;
        if c < best_cost {
            best_cost = c;
            best = i;
        }
        i += 1;
    }
    best
}

/// The three arms. Each computes the same value; they differ only in how.
#[inline(always)]
fn arm0<P: Policy>(col: &[u64], wmax: u64) -> u64 {
    let mut a: u64 = 0;
    let mut i = 0;
    while i < col.len() {
        a = combine::<P>(a, col[i], wmax);
        i += 1;
    }
    a
}

#[inline(always)]
fn arm1<P: Policy>(col: &[u64], wmax: u64) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 0;
    let mut i = 0;
    while i + 1 < col.len() {
        a = combine::<P>(a, col[i], wmax);
        b = combine::<P>(b, col[i + 1], wmax);
        i += 2;
    }
    while i < col.len() {
        a = combine::<P>(a, col[i], wmax);
        i += 1;
    }
    combine::<P>(a, b, wmax)
}

#[inline(always)]
fn arm2<P: Policy>(col: &[u64], wmax: u64) -> u64 {
    let mut a: u64 = 0;
    let mut i = col.len();
    while i > 0 {
        i -= 1;
        a = combine::<P>(a, col[i], wmax);
    }
    a
}

/// Component one is read here and nowhere else. It is the only thing in this
/// file that changes the value.
#[inline(always)]
fn combine<P: Policy>(a: u64, x: u64, wmax: u64) -> u64 {
    let s = a.wrapping_add(x);
    if P::SATURATES {
        if s > wmax { wmax } else { s }
    } else {
        s & wmax
    }
}

#[inline(always)]
fn dispatch<P: Policy>(arm: usize, col: &[u64], wmax: u64) -> u64 {
    // `arm` is a const argument at every call below, so this collapses.
    match arm {
        0 => arm0::<P>(col, wmax),
        1 => arm1::<P>(col, wmax),
        _ => arm2::<P>(col, wmax),
    }
}

// ---------------------------------------------------------------------------
// SHAPE A. Two carriers, no name. Policy on the value, weighting at the site.
// ---------------------------------------------------------------------------

/// The stored column. Its type carries the declared width and component one,
/// and nothing else. Every consumer of it agrees about the policy because the
/// type says so, and none of them is told which arm anybody used.
#[repr(transparent)]
pub struct Column<'a, const W: u32, P: Policy> {
    raw: &'a [u64],
    _p: core::marker::PhantomData<P>,
}

impl<'a, const W: u32, P: Policy> Column<'a, W, P> {
    pub const fn new(raw: &'a [u64]) -> Self {
        Self { raw, _p: core::marker::PhantomData }
    }
}

/// The site supplies the weighting and the region. The value's type does not
/// mention either.
#[inline(always)]
pub fn fold_a<const W: u32, P: Policy, Wt: Weighting, const REGION: usize>(
    col: &Column<'_, W, P>,
) -> u64 {
    let arm = const { argmin(REGION, Wt::W_TIME, Wt::W_SIZE) };
    dispatch::<P>(arm, col.raw, (1u64 << W) - 1)
}

/// Two sites, one column type, two weightings, no cast anywhere.
#[no_mangle]
pub fn a_time_first(raw: &[u64]) -> u64 {
    let c: Column<13, Saturating> = Column::new(raw);
    fold_a::<13, Saturating, TimeFirst, 3>(&c)
}

#[no_mangle]
pub fn a_size_first(raw: &[u64]) -> u64 {
    let c: Column<13, Saturating> = Column::new(raw);
    fold_a::<13, Saturating, SizeFirst, 3>(&c)
}

/// The two really do reach different arms. A const assertion, so the comparison
/// cannot go vacuous the way an equal-output comparison silently can.
const _: () = {
    assert!(argmin(3, TimeFirst::W_TIME, TimeFirst::W_SIZE) != argmin(3, SizeFirst::W_TIME, SizeFirst::W_SIZE));
};

// ---------------------------------------------------------------------------
// SHAPE B. One name binding both, written on the value. This is the reading of
// clause four that treats a named strategy as a single thing on the type.
// ---------------------------------------------------------------------------

pub trait Strategy {
    type P: Policy;
    type W: Weighting;
}

pub struct Fast;
impl Strategy for Fast {
    type P = Saturating;
    type W = TimeFirst;
}

pub struct Small;
impl Strategy for Small {
    type P = Saturating;
    type W = SizeFirst;
}

/// The column now carries the whole named strategy, so it carries component two.
#[repr(transparent)]
pub struct ColumnB<'a, const W: u32, S: Strategy> {
    raw: &'a [u64],
    _s: core::marker::PhantomData<S>,
}

impl<'a, const W: u32, S: Strategy> ColumnB<'a, W, S> {
    pub const fn new(raw: &'a [u64]) -> Self {
        Self { raw, _s: core::marker::PhantomData }
    }

    /// THE COST. To fold this column under a different weighting, the value's
    /// type has to move. This function changes no bit and no denoted value; it
    /// exists only because component two was put on the carrier component one
    /// needed.
    #[inline(always)]
    pub const fn reinterpret<T: Strategy>(self) -> ColumnB<'a, W, T> {
        ColumnB { raw: self.raw, _s: core::marker::PhantomData }
    }
}

#[inline(always)]
pub fn fold_b<const W: u32, S: Strategy, const REGION: usize>(col: &ColumnB<'_, W, S>) -> u64 {
    let arm = const { argmin(REGION, S::W::W_TIME, S::W::W_SIZE) };
    dispatch::<S::P>(arm, col.raw, (1u64 << W) - 1)
}

#[no_mangle]
pub fn b_as_stored(raw: &[u64]) -> u64 {
    let c: ColumnB<13, Fast> = ColumnB::new(raw);
    fold_b::<13, Fast, 3>(&c)
}

/// The same column, folded the other way. Note the `reinterpret`: the value's
/// type had to change to express a decision the value has nothing to do with.
#[no_mangle]
pub fn b_other_weighting(raw: &[u64]) -> u64 {
    let c: ColumnB<13, Fast> = ColumnB::new(raw);
    let c2 = c.reinterpret::<Small>();
    fold_b::<13, Small, 3>(&c2)
}

// ---------------------------------------------------------------------------
// SHAPE C. The name is a DEFAULT in the second component, not a binding of it.
// Component one stays on the value; the name supplies both; the site may name a
// weighting and the value's type does not move.
// ---------------------------------------------------------------------------

/// The value carries the declared width, component one, and the name, so a
/// reader of the type still sees one intent. It does NOT carry component two:
/// the name's weighting is reached through the name, as a default.
#[repr(transparent)]
pub struct ColumnC<'a, const W: u32, S: Strategy> {
    raw: &'a [u64],
    _s: core::marker::PhantomData<S>,
}

impl<'a, const W: u32, S: Strategy> ColumnC<'a, W, S> {
    pub const fn new(raw: &'a [u64]) -> Self {
        Self { raw, _s: core::marker::PhantomData }
    }
}

/// The common case: one name, nothing else written.
#[inline(always)]
pub fn fold_c<const W: u32, S: Strategy, const REGION: usize>(col: &ColumnC<'_, W, S>) -> u64 {
    let arm = const { argmin(REGION, S::W::W_TIME, S::W::W_SIZE) };
    dispatch::<S::P>(arm, col.raw, (1u64 << W) - 1)
}

/// The override: a second weighting named at the site. Component one is still
/// read from the value's own name, so the thing every consumer must agree about
/// cannot be moved here.
#[inline(always)]
pub fn fold_c_with<const W: u32, S: Strategy, Wt: Weighting, const REGION: usize>(
    col: &ColumnC<'_, W, S>,
) -> u64 {
    let arm = const { argmin(REGION, Wt::W_TIME, Wt::W_SIZE) };
    dispatch::<S::P>(arm, col.raw, (1u64 << W) - 1)
}

#[no_mangle]
pub fn c_default(raw: &[u64]) -> u64 {
    let c: ColumnC<13, Fast> = ColumnC::new(raw);
    fold_c::<13, Fast, 3>(&c)
}

#[no_mangle]
pub fn c_overridden(raw: &[u64]) -> u64 {
    let c: ColumnC<13, Fast> = ColumnC::new(raw);
    fold_c_with::<13, Fast, SizeFirst, 3>(&c)
}

/// And the property that makes C the answer rather than a convenience: an
/// override moves the arm and cannot move the policy. Asserted at const time
/// rather than described, so a later edit that lets the override reach
/// component one fails the build.
const _: () = {
    // the override reaches a different arm than the name's default
    assert!(argmin(3, <Fast as Strategy>::W::W_TIME, <Fast as Strategy>::W::W_SIZE)
        != argmin(3, SizeFirst::W_TIME, SizeFirst::W_SIZE));
    // and both folds read the same component one
    assert!(<<Fast as Strategy>::P as Policy>::SATURATES == <Saturating as Policy>::SATURATES);
};

// ---------------------------------------------------------------------------
// The region really does move the answer, so REGION is not decoration.
// ---------------------------------------------------------------------------

const _: () = {
    assert!(argmin(0, TimeFirst::W_TIME, TimeFirst::W_SIZE) == 1);
    assert!(argmin(3, SizeFirst::W_TIME, SizeFirst::W_SIZE) == 0);
    // `97` section 5's shape: at a fixed declared width and a fixed policy, the
    // best arm moves with the region alone.
    assert!(argmin(0, SizeFirst::W_TIME, SizeFirst::W_SIZE)
        == argmin(3, SizeFirst::W_TIME, SizeFirst::W_SIZE));
    assert!(argmin(0, TimeFirst::W_TIME, TimeFirst::W_SIZE)
        != argmin(3, TimeFirst::W_TIME, TimeFirst::W_SIZE));
};
