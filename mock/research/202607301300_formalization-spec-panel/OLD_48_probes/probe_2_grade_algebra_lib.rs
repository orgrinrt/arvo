//! Probe 2: the grade algebra file 47's two proposals jointly need, built,
//! sealed, and checked over the whole matrix.
//!
//! File 47's section 1.1 recommends the strict sentence: "an operation's grade
//! is the JOIN of its operands' grades." File 47's section 3.2 moves the grade
//! from a const bitmask to a type. Nobody put the two together: if both are
//! adopted, the join is a type-level operation, and file 47 compiled neither a
//! `Join` nor any law over it. Its `.weaken` bound (`WeakerThan`, `47_probes/
//! probe_3:223-232`) is the order half of the lattice; the join half is absent.
//!
//! CLAIM A: the join exists as pure impl selection over the four constructor
//! heads, sixteen impls, no blanket, no recursion, so it sits on the safe side
//! of probe 1's wall by construction.
//!
//! CLAIM B: the algebra is checked over the WHOLE matrix, not a sample:
//! join-BITS agreement (16 cases), commutativity as type equality (16),
//! associativity as type equality (64), identity and absorption (4 + 4), and
//! order-join compatibility, `a WeakerThan b` iff `join(a, b) = b`, positive
//! half instantiated for all nine pairs the order declares. The seven negative
//! order pairs are compile-fail cases and are listed for the owed suite rather
//! than faked here.
//!
//! CLAIM C: the grade carrier is itself a carrier in file 46's sense, so it
//! gets the two-line seal (`46:section 1`): sealed at the trait,
//! constructor-headed impls. File 47 left `Grade` unsealed; probe 2b is the
//! downstream attack that this closes.
//!
//! CLAIM D: file 47's two open sentences meet in one signature: a binary
//! operation over graded operands returns the joined grade by projection,
//! which is section 1.1's strict semantics carried in section 3.2's mechanism.
//!
//! EXPECTED: COMPILES CLEAN.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --crate-name grade_lib
//!   probe_2_grade_algebra_lib.rs

#![allow(dead_code)]

use core::marker::PhantomData;

mod sealed {
    pub trait GradeSealed {}
}

/// The four grade points, file 37's bitmask kept as the value the types are
/// unique over, exactly the discipline the numeral tower uses.
pub trait Grade: sealed::GradeSealed {
    const BITS: u8;
}

pub struct Faithful;
pub struct RefusalsTransferred;
pub struct EventsTransferred;
pub struct BothTransferred;

impl sealed::GradeSealed for Faithful {}
impl sealed::GradeSealed for RefusalsTransferred {}
impl sealed::GradeSealed for EventsTransferred {}
impl sealed::GradeSealed for BothTransferred {}

impl Grade for Faithful {
    const BITS: u8 = 0;
}
impl Grade for RefusalsTransferred {
    const BITS: u8 = 1;
}
impl Grade for EventsTransferred {
    const BITS: u8 = 2;
}
impl Grade for BothTransferred {
    const BITS: u8 = 3;
}

// ---------------------------------------------------------------------------
// The join: sixteen constructor-headed impls, no blanket anywhere, which is
// the structural property probe 1 shows is load-bearing.
// ---------------------------------------------------------------------------

pub trait Join<Rhs: Grade>: Grade {
    type Out: Grade;
}

macro_rules! join_impl {
    ($a:ty, $b:ty, $out:ty) => {
        impl Join<$b> for $a {
            type Out = $out;
        }
    };
}

join_impl!(Faithful, Faithful, Faithful);
join_impl!(Faithful, RefusalsTransferred, RefusalsTransferred);
join_impl!(Faithful, EventsTransferred, EventsTransferred);
join_impl!(Faithful, BothTransferred, BothTransferred);
join_impl!(RefusalsTransferred, Faithful, RefusalsTransferred);
join_impl!(
    RefusalsTransferred,
    RefusalsTransferred,
    RefusalsTransferred
);
join_impl!(RefusalsTransferred, EventsTransferred, BothTransferred);
join_impl!(RefusalsTransferred, BothTransferred, BothTransferred);
join_impl!(EventsTransferred, Faithful, EventsTransferred);
join_impl!(EventsTransferred, RefusalsTransferred, BothTransferred);
join_impl!(EventsTransferred, EventsTransferred, EventsTransferred);
join_impl!(EventsTransferred, BothTransferred, BothTransferred);
join_impl!(BothTransferred, Faithful, BothTransferred);
join_impl!(BothTransferred, RefusalsTransferred, BothTransferred);
join_impl!(BothTransferred, EventsTransferred, BothTransferred);
join_impl!(BothTransferred, BothTransferred, BothTransferred);

/// The order, file 47's `WeakerThan` carried unchanged: `Self`'s published
/// classes are a subset of `To`'s.
pub trait WeakerThan<To: Grade>: Grade {}
impl WeakerThan<Faithful> for Faithful {}
impl WeakerThan<RefusalsTransferred> for Faithful {}
impl WeakerThan<EventsTransferred> for Faithful {}
impl WeakerThan<BothTransferred> for Faithful {}
impl WeakerThan<RefusalsTransferred> for RefusalsTransferred {}
impl WeakerThan<BothTransferred> for RefusalsTransferred {}
impl WeakerThan<EventsTransferred> for EventsTransferred {}
impl WeakerThan<BothTransferred> for EventsTransferred {}
impl WeakerThan<BothTransferred> for BothTransferred {}

// ---------------------------------------------------------------------------
// The laws, whole matrix. Type equality through the one-impl gadget, so a
// wrong `Out` fails to compile at the exact cell rather than averaging away.
// ---------------------------------------------------------------------------

pub trait Same<T> {}
impl<T> Same<T> for T {}

const fn same<A: Same<B>, B>() {}

type J<A, B> = <A as Join<B>>::Out;

macro_rules! for_all_grades {
    ($m:ident) => {
        $m!(Faithful);
        $m!(RefusalsTransferred);
        $m!(EventsTransferred);
        $m!(BothTransferred);
    };
}

// join agrees with the bitmask, all sixteen cells.
macro_rules! bits_row {
    ($a:ty) => {
        const _: () = {
            assert!(<J<$a, Faithful> as Grade>::BITS == <$a as Grade>::BITS | 0);
            assert!(<J<$a, RefusalsTransferred> as Grade>::BITS == <$a as Grade>::BITS | 1);
            assert!(<J<$a, EventsTransferred> as Grade>::BITS == <$a as Grade>::BITS | 2);
            assert!(<J<$a, BothTransferred> as Grade>::BITS == <$a as Grade>::BITS | 3);
        };
    };
}
for_all_grades!(bits_row);

// commutativity, all sixteen cells, as type equality.
macro_rules! comm_row {
    ($a:ty) => {
        const _: () = {
            same::<J<$a, Faithful>, J<Faithful, $a>>();
            same::<J<$a, RefusalsTransferred>, J<RefusalsTransferred, $a>>();
            same::<J<$a, EventsTransferred>, J<EventsTransferred, $a>>();
            same::<J<$a, BothTransferred>, J<BothTransferred, $a>>();
        };
    };
}
for_all_grades!(comm_row);

// associativity, all sixty-four cells, as type equality.
macro_rules! assoc_cell {
    ($a:ty, $b:ty, $c:ty) => {
        const _: () = same::<J<J<$a, $b>, $c>, J<$a, J<$b, $c>>>();
    };
}
macro_rules! assoc_bc {
    ($a:ty, $b:ty) => {
        assoc_cell!($a, $b, Faithful);
        assoc_cell!($a, $b, RefusalsTransferred);
        assoc_cell!($a, $b, EventsTransferred);
        assoc_cell!($a, $b, BothTransferred);
    };
}
macro_rules! assoc_b {
    ($a:ty) => {
        assoc_bc!($a, Faithful);
        assoc_bc!($a, RefusalsTransferred);
        assoc_bc!($a, EventsTransferred);
        assoc_bc!($a, BothTransferred);
    };
}
for_all_grades!(assoc_b);

// identity and absorption, all four each.
macro_rules! bounds_row {
    ($a:ty) => {
        const _: () = {
            same::<J<Faithful, $a>, $a>();
            same::<J<BothTransferred, $a>, BothTransferred>();
        };
    };
}
for_all_grades!(bounds_row);

// order-join compatibility, positive half: for every declared `A WeakerThan B`,
// `join(A, B) = B`. Nine instantiations, one per declared pair.
const fn weaker_joins_to<A: WeakerThan<B> + Join<B, Out = B>, B: Grade>() {}
const _: () = {
    weaker_joins_to::<Faithful, Faithful>();
    weaker_joins_to::<Faithful, RefusalsTransferred>();
    weaker_joins_to::<Faithful, EventsTransferred>();
    weaker_joins_to::<Faithful, BothTransferred>();
    weaker_joins_to::<RefusalsTransferred, RefusalsTransferred>();
    weaker_joins_to::<RefusalsTransferred, BothTransferred>();
    weaker_joins_to::<EventsTransferred, EventsTransferred>();
    weaker_joins_to::<EventsTransferred, BothTransferred>();
    weaker_joins_to::<BothTransferred, BothTransferred>();
};
// The seven undeclared pairs (each `join(A, B) != B`) are the compile-fail
// half: `weaker_joins_to::<RefusalsTransferred, Faithful>()` must refuse on
// BOTH bounds. Owed to the compile-fail suite beside the seal's adversary,
// forced through a call per file 46's section 6.1 lesson.

// ---------------------------------------------------------------------------
// CLAIM D: the two open sentences in one signature. Strict semantics (the
// grade is the join of the operands' grades) carried in the projected
// mechanism (the grade is a type; the caller declares nothing).
// ---------------------------------------------------------------------------

pub struct Graded<G: Grade>(i32, PhantomData<G>);

impl Graded<Faithful> {
    pub const fn pure(v: i32) -> Self {
        Graded(v, PhantomData)
    }
}

pub fn combine<G1, G2>(a: Graded<G1>, b: Graded<G2>) -> Graded<<G1 as Join<G2>>::Out>
where
    G1: Grade + Join<G2>,
    G2: Grade,
{
    Graded(a.0 + b.0, PhantomData)
}

/// A consumer's term: the published grade of the whole term is the join over
/// its leaves, by construction, whatever the grouping, which is exactly the
/// strict sentence's corollary in file 47's section 1.1.
pub fn a_term(
    x: Graded<RefusalsTransferred>,
    y: Graded<EventsTransferred>,
    z: Graded<Faithful>,
) -> Graded<BothTransferred> {
    combine(combine(x, y), z)
}
