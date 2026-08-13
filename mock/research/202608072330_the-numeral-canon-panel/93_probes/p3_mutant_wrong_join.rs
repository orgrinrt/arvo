//! P3. Is the product-of-axes structure expressible under the pin.
//!
//! P1b concluded that resolution loses nothing exactly when the strategy set is
//! closed under the join of the demands its members carry, and that a product
//! of per-axis chains is closed by construction. That is a claim about
//! mathematics. This probe asks whether it is a claim about Rust on the pinned
//! nightly, which is a different question and the one that decides whether the
//! shape is available at all.
//!
//! Constraints honoured, all from INTENTS.md I14 and the workspace's
//! unstable-features.md:
//!   - no `generic_const_exprs`. The join therefore may NOT be a const
//!     expression in type position, which is where the obvious encoding wants
//!     to put it. It is a trait with an associated type instead, per
//!     `a-refused-bound-wants-a-trait-not-a-feature.md`.
//!   - no `dyn`, no `TypeId`, no `std::any`.
//!   - `#![no_std]`.
//!   - no runtime check anywhere on the resolution path.
//!
//! Two things are established, and they factor deliberately:
//!   A. the type-level join AGREES with a const-fn join on tags, for all pairs;
//!   B. the const-fn join is COMMUTATIVE, ASSOCIATIVE and IDEMPOTENT,
//!      exhaustively over the whole product.
//! Together those give the type-level laws without expanding a macro over
//! 19683 triples.
//!
//! Run: rustc --edition 2024 -O p3_product_lattice_in_the_type_system.rs -o /tmp/p3 && /tmp/p3

#![no_std]
#![feature(const_trait_impl)]

extern crate std;
use std::println;

// --------------------------------------------------------------------------
// Axis one: overflow policy. A chain, ordered by how much of the exact result
// survives. Illustrative points, not a proposal.
// --------------------------------------------------------------------------
pub struct Wrap;
pub struct Sat;
pub struct Widen;

// Axis two: rounding. A chain on the same principle.
pub struct Trunc;
pub struct Nearest;
pub struct Exact;

/// Every axis point carries a rank inside its own chain. The rank is what the
/// join reads; nothing else about a point is load-bearing here.
pub trait Point {
    const RANK: u32;
    const NAME: &'static str;
}

macro_rules! point {
    ($t:ty, $r:expr, $n:expr) => {
        impl Point for $t {
            const RANK: u32 = $r;
            const NAME: &'static str = $n;
        }
    };
}
point!(Wrap, 0, "wrap");
point!(Sat, 1, "sat");
point!(Widen, 2, "widen");
point!(Trunc, 0, "trunc");
point!(Nearest, 1, "nearest");
point!(Exact, 2, "exact");

/// Per-axis join. This is the shape the forbidden feature would otherwise be
/// used for: `join(A, B)` in type position needs arithmetic in a const
/// argument. As an associated type it needs nothing unstable beyond const
/// traits, and the solver resolves it.
pub trait JoinWith<R> {
    type Out;
}

macro_rules! join3 {
    ($lo:ty, $mid:ty, $hi:ty) => {
        impl JoinWith<$lo> for $lo { type Out = $lo; }
        impl JoinWith<$mid> for $lo { type Out = $mid; }
        impl JoinWith<$hi> for $lo { type Out = $hi; }
        impl JoinWith<$lo> for $mid { type Out = $mid; }
        impl JoinWith<$mid> for $mid { type Out = $mid; }
        impl JoinWith<$hi> for $mid { type Out = $mid; }
        impl JoinWith<$lo> for $hi { type Out = $hi; }
        impl JoinWith<$mid> for $hi { type Out = $hi; }
        impl JoinWith<$hi> for $hi { type Out = $hi; }
    };
}
join3!(Wrap, Sat, Widen);
join3!(Trunc, Nearest, Exact);

// --------------------------------------------------------------------------
// A strategy is a point in the product. The two roles are kept apart on
// purpose: `Store` is what a value costs at rest, `Compute` is what an
// operation costs in flight, and P1b's Hot-versus-Cold conflict is a conflict
// only when one parameter is asked to carry both.
// --------------------------------------------------------------------------
pub struct Strategy<Ov, Rd>(core::marker::PhantomData<(Ov, Rd)>);

impl<Ov: Point, Rd: Point> Strategy<Ov, Rd> {
    pub const TAG: u32 = Ov::RANK * 3 + Rd::RANK;
}

/// Componentwise join, blanket over the product. One impl, not nine, and it
/// stays one impl however many points each axis grows.
impl<A1, B1, A2, B2> JoinWith<Strategy<A2, B2>> for Strategy<A1, B1>
where
    A1: JoinWith<A2>,
    B1: JoinWith<B2>,
{
    type Out = Strategy<A1::Out, B1::Out>;
}

pub type Join<L, R> = <L as JoinWith<R>>::Out;

// --------------------------------------------------------------------------
// A. The type-level join agrees with a const-fn join on tags.
// --------------------------------------------------------------------------

/// The reference join, on tags, at const time.
const fn tag_join(a: u32, b: u32) -> u32 {
    let (ao, ar) = (a / 3, a % 3);
    let (bo, br) = (b / 3, b % 3);
    let o = if ao > bo { ao } else { bo };
    let r = if ar > br { ar } else { br };
    o * 3 + r
}

/// Every check below is a `const` item, so a mismatch is a COMPILE error and
/// this probe cannot report success while being wrong.
macro_rules! agree {
    ($($a:ty, $b:ty);* $(;)?) => {
        $(
            const _: () = {
                assert!(
                    <Join<$a, $b>>::TAG
                        == tag_join(<$a>::TAG, <$b>::TAG)
                );
            };
        )*
    };
}

type S00 = Strategy<Wrap, Trunc>;
type S01 = Strategy<Wrap, Nearest>;
type S02 = Strategy<Wrap, Exact>;
type S10 = Strategy<Sat, Trunc>;
type S11 = Strategy<Sat, Nearest>;
type S12 = Strategy<Sat, Exact>;
type S20 = Strategy<Widen, Trunc>;
type S21 = Strategy<Widen, Nearest>;
type S22 = Strategy<Widen, Exact>;

// All 81 ordered pairs. Written out rather than sampled: a law asserted over a
// subset of the shapes it supports is a choice about what not to find out.
agree! {
    S00,S00; S00,S01; S00,S02; S00,S10; S00,S11; S00,S12; S00,S20; S00,S21; S00,S22;
    S01,S00; S01,S01; S01,S02; S01,S10; S01,S11; S01,S12; S01,S20; S01,S21; S01,S22;
    S02,S00; S02,S01; S02,S02; S02,S10; S02,S11; S02,S12; S02,S20; S02,S21; S02,S22;
    S10,S00; S10,S01; S10,S02; S10,S10; S10,S11; S10,S12; S10,S20; S10,S21; S10,S22;
    S11,S00; S11,S01; S11,S02; S11,S10; S11,S11; S11,S12; S11,S20; S11,S21; S11,S22;
    S12,S00; S12,S01; S12,S02; S12,S10; S12,S11; S12,S12; S12,S20; S12,S21; S12,S22;
    S20,S00; S20,S01; S20,S02; S20,S10; S20,S11; S20,S12; S20,S20; S20,S21; S20,S22;
    S21,S00; S21,S01; S21,S02; S21,S10; S21,S11; S21,S12; S21,S20; S21,S21; S21,S22;
    S22,S00; S22,S01; S22,S02; S22,S10; S22,S11; S22,S12; S22,S20; S22,S21; S22,S22;
}

// A named preset is a point in the product, and naming one costs a type alias.
// That is what I1 asks for structurally: the set is not closed because a
// product has no list to close.
pub type PresetSpeed = S00; // wrap, truncate
pub type PresetNative = S01; // wrap, nearest
pub type PresetAccurate = S22; // widen, exact

// The two presets that P1b could not join on a flat set do join here, and the
// result is a named element rather than an escalation to the top.
const _: () = {
    assert!(<Join<PresetSpeed, PresetAccurate>>::TAG == S22::TAG);
    assert!(<Join<PresetSpeed, PresetNative>>::TAG == S01::TAG);
};

fn main() {
    println!("P3. Product lattice in the type system");
    println!("======================================");
    println!();
    println!("Compiled on the pinned nightly with:");
    println!("  no generic_const_exprs, no dyn, no TypeId, no std::any, no_std core.");
    println!("The only gate is const_trait_impl, which is on the allowed list.");
    println!();
    println!("A. 81 of 81 ordered pairs: the type-level join equals the const-fn");
    println!("   join on tags. Each is a `const _: () = assert!(..)`, so this line");
    println!("   printing at all means every one of them passed at compile time.");
    println!();
    println!("B. exhaustive laws on the const-fn join, computed now:");
    let mut bad_c = 0u32;
    let mut bad_a = 0u32;
    let mut bad_i = 0u32;
    for a in 0..9u32 {
        if tag_join(a, a) != a {
            bad_i += 1;
        }
        for b in 0..9u32 {
            if tag_join(a, b) != tag_join(b, a) {
                bad_c += 1;
            }
            for c in 0..9u32 {
                if tag_join(tag_join(a, b), c) != tag_join(a, tag_join(b, c)) {
                    bad_a += 1;
                }
            }
        }
    }
    println!("   idempotence  : {} failures of 9", bad_i);
    println!("   commutativity: {} failures of 81", bad_c);
    println!("   associativity: {} failures of 729", bad_a);
    println!();
    println!("A and B compose: the type-level join is the const-fn join, and the");
    println!("const-fn join is a semilattice, so the type-level join is one, without");
    println!("expanding a macro over every triple.");
    println!();
    println!("What this does NOT establish: that these are the right axes, that");
    println!("three points per axis is the right granularity, or that the two roles");
    println!("named here are the only ones. It establishes that the SHAPE compiles");
    println!("and is lawful, which is the question a canon has to answer before it");
    println!("can write the intent down.");
}
