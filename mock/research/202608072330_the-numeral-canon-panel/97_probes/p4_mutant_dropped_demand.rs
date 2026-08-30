// P4. The closure `93` priced as a cost is the free join-semilattice on its own
// generators, so it is generated rather than enumerated and costs no names.
//
// `93`'s P1b Part B reports that four markers carrying one demand each leave 12 of 16
// ordered pairs unresolvable, and that the smallest set closed under their own
// resolution has 15 elements. `93` section 4 then lists "carry the closure" as
// response (a) and prices it: "the space is larger than four, so every 'what does
// this strategy do' question needs an answer parameterised by axis rather than looked
// up per marker."
//
// 15 is 2^4 - 1. That is not a coincidence and it is the whole point: the closure of
// d one-demand generators under union IS the free join-semilattice on d generators,
// whose carrier is the non-empty subsets. Nobody names 15 things. Four generators are
// named and every other element is a formal join of them, exactly the way a formal
// union type is not a new named type.
//
// This file checks the encoding is available on the pin and that the join is total,
// lawful, and free of the escalation pathology `93`'s F4 reports for every admissible
// flat table.
//
// What is being demonstrated, and each is a compile-time assertion below:
//
//   1. A strategy that is SILENT on a demand is a first-class element, not a hole.
//      Under a total-assignment product every strategy must name a value on every
//      axis, which is why `40`'s p2 finds that none of the four named strategies pins
//      a point (reachable-determinate 0 of 16). Here silence is the bottom on that
//      coordinate and joins as an identity.
//   2. The join is total on the whole lattice, in ONE impl, and its result's demand
//      set is the union. All 256 ordered pairs are asserted.
//   3. `Hot` joined with `Cold` is a defined element that is NOT the accuracy-first
//      preset. `93`'s F4 reports that all four admissible tables on the flat set
//      escalate every mixed expression to the accuracy-first preset, and calls the
//      cost "nobody asked and everybody pays". Here nobody asks and nobody pays.
//   4. No forbidden feature. No `generic_const_exprs`: the join is an associated type
//      on a trait rather than an expression in type position, which is
//      `a-refused-bound-wants-a-trait-not-a-feature.md` applied before the wall.
//
// This is a spike. Its names, its arity of four, and its use of a byte for the mask
// are scaffolding chosen to reach the check and are not proposals.
//
// Build: rustc --edition 2024 -O p4_demand_lattice.rs --crate-type lib

#![no_std]

// ---------------------------------------------------------------- presence

pub struct Absent;
pub struct Present;

pub trait Presence {
    const BIT: u8;
}
impl Presence for Absent {
    const BIT: u8 = 0;
}
impl Presence for Present {
    const BIT: u8 = 1;
}

/// Union on one coordinate. Two elements, so four impls and no more, ever.
pub trait Or<Rhs> {
    type Out: Presence;
}
impl Or<Absent> for Absent {
    type Out = Absent;
}
impl Or<Present> for Absent {
    // MUTANT: one coordinate of the union is dropped. If the assertions are real,
    // this must not compile.
    type Out = Absent;
}
impl Or<Absent> for Present {
    type Out = Present;
}
impl Or<Present> for Present {
    type Out = Present;
}

// ---------------------------------------------------------------- the lattice

/// A strategy is the set of demands it makes. Absence on a coordinate means the
/// consumer made no demand there and the resolver is free, which is a statement
/// rather than a gap.
pub struct Demands<S, R, A, I>(core::marker::PhantomData<(S, R, A, I)>);

pub trait Mask {
    const M: u8;
}
impl<S: Presence, R: Presence, A: Presence, I: Presence> Mask for Demands<S, R, A, I> {
    const M: u8 = S::BIT | (R::BIT << 1) | (A::BIT << 2) | (I::BIT << 3);
}

/// The join. One impl, and it stays one impl however many coordinates are added.
pub trait Join<Rhs> {
    type Out: Mask;
}
impl<S1, R1, A1, I1, S2, R2, A2, I2> Join<Demands<S2, R2, A2, I2>> for Demands<S1, R1, A1, I1>
where
    S1: Or<S2>,
    R1: Or<R2>,
    A1: Or<A2>,
    I1: Or<I2>,
{
    type Out =
        Demands<<S1 as Or<S2>>::Out, <R1 as Or<R2>>::Out, <A1 as Or<A2>>::Out, <I1 as Or<I2>>::Out>;
}

// ---------------------------------------------------------------- generators

// The four named presets are generators, each carrying exactly one demand. The
// names are the panel's current ones and are open per I1; what matters here is that
// each is one generator rather than a member of a closed set.
pub type Nothing = Demands<Absent, Absent, Absent, Absent>;
pub type Speed = Demands<Present, Absent, Absent, Absent>;
pub type Residency = Demands<Absent, Present, Absent, Absent>;
pub type Accuracy = Demands<Absent, Absent, Present, Absent>;
pub type Familiarity = Demands<Absent, Absent, Absent, Present>;

pub type JoinOf<X, Y> = <X as Join<Y>>::Out;

// ---------------------------------------------------------------- checks

macro_rules! d {
    (0) => {
        Absent
    };
    (1) => {
        Present
    };
}

/// One row of the 16 x 16 table: the join's demand set is the union of the two.
macro_rules! union_is_the_join {
    ($($a:tt $b:tt $c:tt $e:tt | $p:tt $q:tt $r:tt $s:tt),* $(,)?) => {
        $(
            const _: () = {
                type L = Demands<d!($a), d!($b), d!($c), d!($e)>;
                type R = Demands<d!($p), d!($q), d!($r), d!($s)>;
                assert!(
                    <JoinOf<L, R> as Mask>::M == (<L as Mask>::M | <R as Mask>::M),
                    "the join of two demand sets is not their union"
                );
                // commutativity, at the level of the demand set the join produces
                assert!(
                    <JoinOf<L, R> as Mask>::M == <JoinOf<R, L> as Mask>::M,
                    "the join is not commutative"
                );
            };
        )*
    };
}

// All 256 ordered pairs, written out rather than sampled. Generated by
// p4_generate_pairs.py, committed beside this file.
include!("p4_pairs.inc");

// Idempotence over all sixteen elements, and bottom as the identity.
macro_rules! idem_and_bottom {
    ($($a:tt $b:tt $c:tt $e:tt),* $(,)?) => {
        $(
            const _: () = {
                type X = Demands<d!($a), d!($b), d!($c), d!($e)>;
                assert!(
                    <JoinOf<X, X> as Mask>::M == <X as Mask>::M,
                    "the join is not idempotent"
                );
                assert!(
                    <JoinOf<X, Nothing> as Mask>::M == <X as Mask>::M,
                    "silence is not the identity of the join"
                );
                assert!(
                    <JoinOf<Nothing, X> as Mask>::M == <X as Mask>::M,
                    "silence is not the identity of the join"
                );
            };
        )*
    };
}

idem_and_bottom!(
    0 0 0 0, 0 0 0 1, 0 0 1 0, 0 0 1 1,
    0 1 0 0, 0 1 0 1, 0 1 1 0, 0 1 1 1,
    1 0 0 0, 1 0 0 1, 1 0 1 0, 1 0 1 1,
    1 1 0 0, 1 1 0 1, 1 1 1 0, 1 1 1 1,
);

// Associativity over the whole 16 x 16 x 16 table. An earlier revision of this file
// asserted a sample of five triples and reasoned that the mask identity above implied
// the rest. The reasoning is sound and the sample is still the thing `the-test-gate.md`
// names: a law asserted over a subset of the shapes it is claimed for. Choosing which
// triples to include is choosing what not to find out, so all 4096 are written out.
macro_rules! join_is_associative {
    ($($a:tt $b:tt $c:tt $e:tt | $p:tt $q:tt $r:tt $s:tt | $w:tt $x:tt $y:tt $z:tt),* $(,)?) => {
        $(
            const _: () = {
                type A = Demands<d!($a), d!($b), d!($c), d!($e)>;
                type B = Demands<d!($p), d!($q), d!($r), d!($s)>;
                type C = Demands<d!($w), d!($x), d!($y), d!($z)>;
                assert!(
                    <JoinOf<JoinOf<A, B>, C> as Mask>::M
                        == <JoinOf<A, JoinOf<B, C>> as Mask>::M,
                    "the join is not associative"
                );
            };
        )*
    };
}

include!("p4_triples.inc");

// ---------------------------------------------------------------- the payoff

// `93`'s F4: every admissible resolution table on the flat four-element set sends a
// mixed expression to the accuracy-first preset, so two operands neither of which
// asked for accuracy produce the most expensive policy in the set. Here the join of
// the speed demand and the residency demand is the element that demands both, and it
// is not the accuracy demand.
const _: () = {
    type SpeedAndResidency = JoinOf<Speed, Residency>;
    assert!(
        <SpeedAndResidency as Mask>::M == (<Speed as Mask>::M | <Residency as Mask>::M),
        "the mixed demand is not the union"
    );
    assert!(
        <SpeedAndResidency as Mask>::M != <Accuracy as Mask>::M,
        "the mixed expression escalated to the accuracy demand"
    );
    assert!(
        <SpeedAndResidency as Mask>::M != <Speed as Mask>::M,
        "the mixed expression lost the residency demand"
    );
    assert!(
        <SpeedAndResidency as Mask>::M != <Residency as Mask>::M,
        "the mixed expression lost the speed demand"
    );
};

// And the element that has no name under a closed set of four exists here without
// being named: everything that demands residency and accuracy at once.
const _: () = {
    type ColdExact = JoinOf<Residency, Accuracy>;
    assert!(
        <ColdExact as Mask>::M == 0b0110,
        "the unnamed point is not reachable"
    );
};

// A public function taking a strategy by its DEMANDS rather than by a name. The
// bound names what the caller must supply, which is a contract, and nothing about
// the mechanism the resolver picks appears in it.
pub fn resolved_mask<X: Mask, Y: Mask>() -> u8
where
    X: Join<Y>,
{
    <JoinOf<X, Y> as Mask>::M
}
