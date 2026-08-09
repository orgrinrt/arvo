//! PROBE 6: fallibility and fidelity are both grades, their coercions run in
//! OPPOSITE directions, and that alone forces opposite mixed-operand rules.
//!
//! The claim under test has three parts and each is checked separately:
//!
//!   (1) A fallibility grade coerces UPWARD (a total result can stand where a
//!       fallible one is expected). The witness is a function on VALUES.
//!   (2) A fidelity grade coerces DOWNWARD (a value carrying a permissive
//!       licence can be viewed as carrying a restrictive one, by declining the
//!       liberties). The witness is a permission, with no value to convert.
//!   (3) Therefore the mixed-operand grade is, in both cases, THE LEAST COMMON
//!       TARGET OF THE TWO COERCIONS, which is the join for (1) and the meet
//!       for (2). One rule, two instances, neither of them chosen.
//!
//! Part (3) is the design payload. arvo ships one uniform projection today,
//! `Resolve<S1, S2>::Out` = "the more conservative of two strategies"
//! (`arvo-strategy/src/cross_strategy.rs`, read via `12_lattner_fresh_read.md:29-31`),
//! and file 12 section 2 found it hand-waved at four presets and undefined at
//! ten axes. This probe argues it is worse than undefined: "more conservative"
//! is not a lattice operation at all, it is a human judgement that happens to
//! name the join on some axes and the meet on others, and a single projection
//! computing one of the two is silently wrong on the axes that want the other.
//!
//! The four builds:
//!
//!   (default)        the correct directions; compiles, runs, prints the table
//!   --cfg bad_lift   Fallible -> Total on values; MUST FAIL, and does
//!   --cfg bad_join   compute the fidelity mixed grade by the join, the way one
//!                    uniform `Resolve` would; MUST FAIL, and does, because the
//!                    join has no coercion witness from the strict operand
//!   --cfg bad_grant  Strict -> Relaxed on licences; SHOULD fail and DOES NOT.
//!                    This arm was written expecting a third compile error and
//!                    got a clean build and a wrong number instead, which is
//!                    the sharpest thing in the probe. See the note at the
//!                    coeffect section.
//!
//! Build:
//!   rustc -O 06_two_lattices_opposite_variance.rs -o p6 && ./p6
//!   rustc --cfg bad_lift  06_two_lattices_opposite_variance.rs   # expect error
//!   rustc --cfg bad_grant 06_two_lattices_opposite_variance.rs   # expect error
//!   rustc --cfg bad_join  06_two_lattices_opposite_variance.rs   # expect error

#![allow(dead_code)]

// ============================================================== effect side
// The fallibility grade. Two points, and the carrier is an INTERPRETATION of
// the grade rather than a member declared beside it. This is
// `07_spj_is_the_type_story_sound.md` section 2's shape, restated only as far
// as this probe needs it.

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Id<T>(pub T);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Or<T> {
    Val(T),
    Refused,
}

pub struct Total;
pub struct Fallible;

pub trait EGrade {
    type C<T: Copy + core::fmt::Debug>: Copy + core::fmt::Debug;
    fn pure<T: Copy + core::fmt::Debug>(t: T) -> Self::C<T>;
    const NAME: &'static str;
}

impl EGrade for Total {
    type C<T: Copy + core::fmt::Debug> = Id<T>;
    fn pure<T: Copy + core::fmt::Debug>(t: T) -> Id<T> {
        Id(t)
    }
    const NAME: &'static str = "Total";
}

impl EGrade for Fallible {
    type C<T: Copy + core::fmt::Debug> = Or<T>;
    fn pure<T: Copy + core::fmt::Debug>(t: T) -> Or<T> {
        Or::Val(t)
    }
    const NAME: &'static str = "Fallible";
}

/// Sub-effecting. `Self` may stand where `G` is expected, and the witness is a
/// total function on values. Every impl below is a real, writable function.
pub trait LiftE<G: EGrade>: EGrade {
    fn lift<T: Copy + core::fmt::Debug>(x: Self::C<T>) -> G::C<T>;
}

impl LiftE<Total> for Total {
    fn lift<T: Copy + core::fmt::Debug>(x: Id<T>) -> Id<T> {
        x
    }
}
impl LiftE<Fallible> for Fallible {
    fn lift<T: Copy + core::fmt::Debug>(x: Or<T>) -> Or<T> {
        x
    }
}
impl LiftE<Fallible> for Total {
    fn lift<T: Copy + core::fmt::Debug>(x: Id<T>) -> Or<T> {
        Or::Val(x.0)
    }
}

// There is deliberately no `impl LiftE<Total> for Fallible`, and the reason is
// not that nobody typed it. `Or::Refused` has no image in `Id<T>`: the function
// the impl would have to supply does not exist as a total function, at any
// payload type. The absence is a theorem, not an omission.

#[cfg(bad_lift)]
impl LiftE<Total> for Fallible {
    fn lift<T: Copy + core::fmt::Debug>(x: Or<T>) -> Id<T> {
        match x {
            Or::Val(v) => Id(v),
            // there is nothing to write here
        }
    }
}

// ============================================================ coeffect side
// The fidelity grade. Also two points, also ordered, and the order runs the
// other way: `Relaxed` GRANTS more than `Strict` grants.

pub struct Strict;
pub struct Relaxed;

pub trait CGrade {
    /// The liberty set this licence grants. Order is by inclusion.
    const LIBERTIES: &'static [&'static str];
    const NAME: &'static str;
}

impl CGrade for Strict {
    const LIBERTIES: &'static [&'static str] = &[];
    const NAME: &'static str = "Strict";
}
impl CGrade for Relaxed {
    const LIBERTIES: &'static [&'static str] = &["reassoc", "contract", "arcp"];
    const NAME: &'static str = "Relaxed";
}

/// Sub-coeffecting. A value under licence `Self` may be viewed under licence
/// `G` when `G` grants no more than `Self` does, i.e. when the view DECLINES
/// liberties rather than acquiring them.
///
/// Note what the witness is, because it is the whole point of the section: not
/// a function. There is no representation change and nothing to convert. The
/// impl exists or it does not, and its existence is the permission.
pub trait ViewC<G: CGrade>: CGrade {}

impl ViewC<Strict> for Strict {}
impl ViewC<Relaxed> for Relaxed {}
impl ViewC<Strict> for Relaxed {}

// There is deliberately no `impl ViewC<Relaxed> for Strict`. Granting a licence
// the consumer did not ask for is exactly the failure the axis exists to
// prevent: a `StrictFloat` operand entering an expression that then
// reassociates is a silently different answer with nothing in the type saying
// so.
//
// AND HERE IS THE ASYMMETRY THIS PROBE DID NOT SET OUT TO FIND. On the effect
// side the missing coercion is missing because it CANNOT BE WRITTEN: `bad_lift`
// fails with E0004, the compiler stating that `Or::Refused` has no case. On the
// coeffect side the missing coercion is missing only because nobody typed it.
// The line below compiles clean. There is no proof obligation attached to it,
// because the witness carries no data, so there is nothing for the compiler to
// check and nothing for a reviewer to notice.
//
// So the two grades need different enforcement. Fallibility's order is policed
// by the type system for free. Fidelity's order is a hand-typed leaf fact of
// exactly the kind Thread C exists to kill, and it needs the same witness
// treatment: the liberty sets are data (`LIBERTIES` below), inclusion between
// them is decidable, and a const check can refuse an impl whose declared
// direction disagrees with the sets. Nothing in the design currently says this.

#[cfg(bad_grant)]
impl ViewC<Relaxed> for Strict {}

/// A licence-gated body. The grade decides which of two source-level shapes
/// compiles, which is the whole mechanism (`16_fallin...md:208-233`): the fact
/// gates a monomorphised body and never leaves the crate.
fn sum4<L: CGrade + ViewC<L>>(xs: [f64; 4]) -> f64 {
    if L::LIBERTIES.contains(&"reassoc") {
        // two independent accumulators, then combine: a REGROUPING
        (xs[0] + xs[2]) + (xs[1] + xs[3])
    } else {
        ((xs[0] + xs[1]) + xs[2]) + xs[3]
    }
}

/// The call an operand of licence `A` makes when it lands in a context of
/// licence `L`. The `ViewC` bound is the only thing standing between a strict
/// operand and a reassociated answer.
fn sum4_in_context<A: CGrade + ViewC<L>, L: CGrade + ViewC<L>>(xs: [f64; 4]) -> f64 {
    sum4::<L>(xs)
}

// ================================================ the one mixed-operand rule
// Stated once, instantiated twice. "The result grade is the least grade both
// operands can be coerced to." Nothing here says join or meet; the direction of
// the coercion witness decides, per axis.

/// Mixed-operand fallibility: the least `Out` such that both `A` and `B` lift
/// to it.
pub trait MixE<B: EGrade>: EGrade {
    type Out: EGrade;
}
impl MixE<Total> for Total {
    type Out = Total;
}
impl MixE<Fallible> for Total {
    type Out = Fallible;
}
impl MixE<Total> for Fallible {
    type Out = Fallible;
}
impl MixE<Fallible> for Fallible {
    type Out = Fallible;
}

/// Mixed-operand fidelity: the least `Out` such that both `A` and `B` can be
/// VIEWED at it.
pub trait MixC<B: CGrade>: CGrade {
    type Out: CGrade;
}
#[cfg(not(bad_join))]
mod correct_mix {
    use super::*;
    impl MixC<Strict> for Strict {
        type Out = Strict;
    }
    impl MixC<Relaxed> for Strict {
        type Out = Strict;
    }
    impl MixC<Strict> for Relaxed {
        type Out = Strict;
    }
    impl MixC<Relaxed> for Relaxed {
        type Out = Relaxed;
    }
}

/// The wrong shape: one uniform "more conservative" projection that takes the
/// join on every axis, which is what a single `Resolve<S1, S2>` over a fused
/// strategy marker does.
#[cfg(bad_join)]
mod join_mix {
    use super::*;
    impl MixC<Strict> for Strict {
        type Out = Strict;
    }
    impl MixC<Relaxed> for Strict {
        type Out = Relaxed; // join
    }
    impl MixC<Strict> for Relaxed {
        type Out = Relaxed; // join
    }
    impl MixC<Relaxed> for Relaxed {
        type Out = Relaxed;
    }
}

// The check that separates the two. A mixed operation may only be emitted when
// BOTH operands can actually be viewed at the result grade. Under the correct
// (meet) mix this bound is satisfiable for all four pairs; under the join mix
// the Strict operand cannot be viewed at `Relaxed` and the bound has no
// witness.
pub fn mixed_fidelity_op<A, B>() -> &'static str
where
    A: CGrade + MixC<B> + ViewC<<A as MixC<B>>::Out>,
    B: CGrade + ViewC<<A as MixC<B>>::Out>,
    <A as MixC<B>>::Out: CGrade,
{
    <<A as MixC<B>>::Out as CGrade>::NAME
}

pub fn mixed_fallibility_op<A, B, T>(a: A::C<T>, b: B::C<T>) -> (<A as MixE<B>>::Out, &'static str)
where
    T: Copy + core::fmt::Debug,
    A: EGrade + MixE<B> + LiftE<<A as MixE<B>>::Out>,
    B: EGrade + LiftE<<A as MixE<B>>::Out>,
    <A as MixE<B>>::Out: EGrade + Default,
{
    let _la = <A as LiftE<<A as MixE<B>>::Out>>::lift(a);
    let _lb = <B as LiftE<<A as MixE<B>>::Out>>::lift(b);
    (Default::default(), <<A as MixE<B>>::Out as EGrade>::NAME)
}

impl Default for Total {
    fn default() -> Self {
        Total
    }
}
impl Default for Fallible {
    fn default() -> Self {
        Fallible
    }
}

fn main() {
    println!("effect grade (fallibility): coercion is a FUNCTION ON VALUES, upward only");
    println!(
        "  Total    -> Fallible   {:?}",
        <Total as LiftE<Fallible>>::lift(Id(7i32))
    );
    println!("  Fallible -> Total      does not exist: Or::Refused has no image in Id<T>");

    println!();
    println!("coeffect grade (fidelity): coercion is a PERMISSION, downward only");
    println!(
        "  Relaxed  -> Strict     exists   (liberties {:?} declined)",
        <Relaxed as CGrade>::LIBERTIES
    );
    println!("  Strict   -> Relaxed    does not exist: a licence cannot be acquired by a view");

    println!();
    println!("the ONE rule: the mixed grade is the least common coercion target");
    println!(
        "{:<22}{:<26}{:<26}",
        "operands", "fallibility (join)", "fidelity (meet)"
    );
    println!(
        "{:<22}{:<26}{:<26}",
        "same, permissive",
        mixed_fallibility_op::<Total, Total, i32>(Id(1), Id(2)).1,
        mixed_fidelity_op::<Relaxed, Relaxed>()
    );
    println!(
        "{:<22}{:<26}{:<26}",
        "mixed (a first)",
        mixed_fallibility_op::<Total, Fallible, i32>(Id(1), Or::Val(2)).1,
        mixed_fidelity_op::<Relaxed, Strict>()
    );
    println!(
        "{:<22}{:<26}{:<26}",
        "mixed (b first)",
        mixed_fallibility_op::<Fallible, Total, i32>(Or::Val(1), Id(2)).1,
        mixed_fidelity_op::<Strict, Relaxed>()
    );
    println!(
        "{:<22}{:<26}{:<26}",
        "same, restrictive",
        mixed_fallibility_op::<Fallible, Fallible, i32>(Or::Val(1), Or::Refused).1,
        mixed_fidelity_op::<Strict, Strict>()
    );

    // The catastrophic-cancellation quartet: regrouping changes the answer by
    // 100% of it. Nothing exotic, just two large opposite values and two small
    // ones.
    let xs = [1.0e16f64, -1.0e16, 1.0, 1.0];
    println!();
    println!("a licence-gated body, on [1e16, -1e16, 1, 1]:");
    println!("  under Strict   {:?}", sum4::<Strict>(xs));
    println!("  under Relaxed  {:?}", sum4::<Relaxed>(xs));
    #[cfg(bad_grant)]
    println!(
        "  a Strict operand in a Relaxed context: {:?}   <-- WRONG, and it COMPILED",
        sum4_in_context::<Strict, Relaxed>(xs)
    );
    #[cfg(not(bad_grant))]
    println!("  a Strict operand in a Relaxed context: refused, no ViewC witness");

    println!();
    println!("reading: both columns land on the conservative answer, and they get");
    println!("there by opposite lattice operations, because the conservative end is");
    println!("the TOP of the effect lattice and the BOTTOM of the coeffect lattice.");
    println!("A single projection computing 'the more conservative of two strategies'");
    println!("is therefore not one operation. Build with --cfg bad_join to watch the");
    println!("uniform-join version fail to produce a coercion witness.");
    println!();
    println!("and the asymmetry: --cfg bad_lift is refused by the compiler (E0004),");
    println!("--cfg bad_grant is not refused by anything. The effect order polices");
    println!("itself; the coeffect order is a hand-typed claim needing a witness.");
}
