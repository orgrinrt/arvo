//! The current shape, reduced from `55_probes/probe_2b_the_arity_of_an_unbounded_loop.rs`
//! to the minimum needed to demonstrate the seal question, unaltered in the part
//! that matters: `InteriorSafety<A>`'s `A` carries NO bound.
//!
//! `Pos` and `Cmp` are stood in by a closed two-value tower (`Small`, `Big`) with
//! a hand-written three-way compare, because the point under test is the OPEN
//! position (`A` in `InteriorSafety<A>`), not the tower's own arithmetic, which
//! files 47/48/50/54/55/62 have already exhaustively covered elsewhere in this
//! review.

pub struct Safe;
pub struct Unsafe;
pub trait Safety {}
impl Safety for Safe {}
impl Safety for Unsafe {}

// Reduced `Pos`/`Cmp` stand-in: two inhabitants, enough to carry a real
// `InteriorSafety` blanket without needing the full tower.
pub struct Small;
pub struct Big;
pub trait Pos {}
impl Pos for Small {}
impl Pos for Big {}

pub trait Cmp<Rhs> {
    type Out: SafetyOf;
}
impl Cmp<Small> for Small {
    type Out = AtOrAbove;
}
impl Cmp<Small> for Big {
    type Out = AtOrAbove;
}
impl Cmp<Big> for Small {
    type Out = Below;
}
impl Cmp<Big> for Big {
    type Out = AtOrAbove;
}

pub struct AtOrAbove;
pub struct Below;
pub trait SafetyOf {
    type Out: Safety;
}
impl SafetyOf for AtOrAbove {
    type Out = Safe;
}
impl SafetyOf for Below {
    type Out = Unsafe;
}

/// UNCHANGED from the shipped shape: `A` carries no bound at all. Anyone,
/// anywhere, can name any type here.
pub trait InteriorSafety<A> {
    type Out: Safety;
}

impl<Hd: Pos + Cmp<A>, A: Pos> InteriorSafety<A> for Hd {
    type Out = <<Hd as Cmp<A>>::Out as SafetyOf>::Out;
}

/// The design's own `Unbounded` marker: not a `Pos`, disjoint from the blanket
/// above by parameter, no specialisation needed. This is the mechanism probe
/// 2b compiled and it is not in question.
pub struct Unbounded;

impl<Hd: Pos> InteriorSafety<Unbounded> for Hd {
    type Out = Unsafe;
}

/// The guarantee this whole mechanism exists to make: an unbounded-arity loop
/// is always classified `Unsafe`, so a consumer generic over any `Hd: Pos`
/// cannot accidentally read a `Safe` grade for one.
pub fn assert_unbounded_is_always_unsafe<Hd: Pos + InteriorSafety<Unbounded, Out = Unsafe>>() {}

pub fn witness() {
    assert_unbounded_is_always_unsafe::<Small>();
    assert_unbounded_is_always_unsafe::<Big>();
}
