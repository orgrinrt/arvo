//! Probe A: the finest-set reading of number-system membership, as a
//! derived projection plus ZST-marker dispatch, under the panel's feature
//! constraints (no specialization, no negative impls, monomorphisation
//! only, `no_std`).
//!
//! Hypothesis: a numeral carries ONE finest system as an associated type;
//! upward membership along the tower is a blanket impl over a chain-order
//! trait; consumer divergence on the system is written as one impl per
//! system MARKER (a closed set, so no coherence pressure), selected
//! through the projection. No specialization, no negative bounds, no
//! unstable feature beyond what stable trait resolution gives.
//!
//! Compile with: rustc --edition 2021 --crate-type lib
//! Outcome recorded in 27_probes/OUTCOMES.md.

#![no_std]

// The tower markers. A chain: N < Z < Z[1/2] < Q. The exotic rungs
// (R, C, H, O, surreal, hyperreal, p-adic) would be more ZSTs here;
// they add rows to `ContainedIn`, nothing else.
pub struct Nat;
pub struct Zint;
pub struct Dyadic;
pub struct Rat;
pub struct Real;

pub trait NumberSystem {
    /// tag for const-context checks; a real design would carry more
    const TAG: u8;
}
impl NumberSystem for Nat {
    const TAG: u8 = 0;
}
impl NumberSystem for Zint {
    const TAG: u8 = 1;
}
impl NumberSystem for Dyadic {
    const TAG: u8 = 2;
}
impl NumberSystem for Rat {
    const TAG: u8 = 3;
}
impl NumberSystem for Real {
    const TAG: u8 = 4;
}

/// The tower order, stated once per pair on the markers themselves.
/// A fixed chain of k rungs needs k(k+1)/2 rows; for ten rungs that is
/// 55 one-line impls, macro-expandable. Reflexive on purpose.
pub trait ContainedIn<Upper> {}
impl ContainedIn<Nat> for Nat {}
impl ContainedIn<Zint> for Nat {}
impl ContainedIn<Dyadic> for Nat {}
impl ContainedIn<Rat> for Nat {}
impl ContainedIn<Real> for Nat {}
impl ContainedIn<Zint> for Zint {}
impl ContainedIn<Dyadic> for Zint {}
impl ContainedIn<Rat> for Zint {}
impl ContainedIn<Real> for Zint {}
impl ContainedIn<Dyadic> for Dyadic {}
impl ContainedIn<Rat> for Dyadic {}
impl ContainedIn<Real> for Dyadic {}
impl ContainedIn<Rat> for Rat {}
impl ContainedIn<Real> for Rat {}
impl ContainedIn<Real> for Real {}

/// Stand-in for the spec's `Numeral`. The finest system is a projection,
/// in the real design derived from the axes by the same macro table that
/// feeds integrality (spec topic line 226), never free-assigned.
pub trait Numeral {
    type System: NumberSystem;
}

/// Upward membership, derived once, blanket, from the finest system.
/// This is the ONLY membership impl in the design; everything else is a
/// row in `ContainedIn`.
pub trait Inhabits<S> {}
impl<N: Numeral, S> Inhabits<S> for N where N::System: ContainedIn<S> {}

// Two model numerals: an integer shape and a fractional shape.
pub struct ModelU8_0; // UFixed<8, 0, S>: finest system N
pub struct ModelU5_3; // UFixed<5, 3, S>: finest system Z[1/2]
impl Numeral for ModelU8_0 {
    type System = Nat;
}
impl Numeral for ModelU5_3 {
    type System = Dyadic;
}

// 1. Upward membership discriminates DOWNWARD: a bound on Inhabits<Zint>
//    admits the integer shape and (see probe_a2) refuses the fractional.
pub fn needs_integers<N: Inhabits<Zint>>() {}
pub fn check_upward() {
    needs_integers::<ModelU8_0>(); // N contained in Z: admitted
}

// 2. Inhabits<Real> is satisfied by every numeral; it can never
//    discriminate. Both shapes pass, which is the point being proven.
pub fn needs_reals<N: Inhabits<Real>>() {}
pub fn check_real_is_vacuous() {
    needs_reals::<ModelU8_0>();
    needs_reals::<ModelU5_3>();
}

// 3. Divergence: behaviour lives on the system marker, one impl per
//    rung, selected through the projection. No specialization, no
//    overlap, because the impls are on a closed set of concrete ZSTs.
pub trait SumStrategy {
    /// which pipeline a consumer picks for this domain
    const PIPELINE: u8;
}
impl SumStrategy for Nat {
    const PIPELINE: u8 = 10; // integer pipeline
}
impl SumStrategy for Zint {
    const PIPELINE: u8 = 10;
}
impl SumStrategy for Dyadic {
    const PIPELINE: u8 = 20; // fractional pipeline
}
impl SumStrategy for Rat {
    const PIPELINE: u8 = 21;
}
impl SumStrategy for Real {
    const PIPELINE: u8 = 99;
}

pub const fn pipeline_of<N: Numeral>() -> u8
where
    N::System: SumStrategy,
{
    <N::System as SumStrategy>::PIPELINE
}

// checked at compile time: the divergence resolves per finest system
const _: () = assert!(pipeline_of::<ModelU8_0>() == 10);
const _: () = assert!(pipeline_of::<ModelU5_3>() == 20);
