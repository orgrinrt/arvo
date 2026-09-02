//! p4. Can a chain's depth reach a const predicate, without a forbidden feature?
//!
//! p3 and `101` section 6.1 both land on the same requirement: an arm that serves
//! op's accuracy intent (I7, "especially within chains and ops, not only alone")
//! has to be selected per chain depth. `101` puts depth in the region; p3 shows
//! the arm family that indexing exposes.
//!
//! Both rest on depth being available where the arm is chosen. I13 as op specified
//! it at `83` says the admissible category is "whatever is available at const
//! time", and the typestate is one source of that. Nobody in this unit has checked
//! whether a chain's depth can be in the typestate at all.
//!
//! It is not obvious that it can. The natural spelling increments a const generic,
//! `Fx<D>` to `Fx<{D + 1}>`, and arithmetic in a const argument position needs
//! `generic_const_exprs`, which is forbidden. So the question is whether the
//! workspace's standing reflex applies: a refused bound wants a trait, not a
//! feature.
//!
//! What this checks, in order:
//!
//!   1. a depth that increments through a chain, with no arithmetic in type
//!      position, no `generic_const_exprs`, no `dyn`, no `TypeId`, `#![no_std]`;
//!   2. the depth reaching a `const` predicate that selects a rounding policy;
//!   3. whether the selection survives to one lowered path, which is what I15
//!      asks for, checked in the emitted assembly rather than asserted.
//!
//! This is a spike. The names, the widths, the grid and the arity are scaffolding
//! to reach the check. It is not a proposed API and its shape is not a decision.
//!
//! Build:  rustc --crate-type=lib -O --edition 2021 p4_is_chain_depth_const_available.rs
//! Asm:    rustc --crate-type=lib -O --edition 2021 --emit=asm p4_is_chain_depth_const_available.rs

#![no_std]
// Deliberately no `#![feature(...)]` of any kind. If this file compiles, it
// compiles on the pinned stable-shaped surface of the nightly, and the absence of
// a gate is the evidence, in the same form arvo-comb used for its own escape.

use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// 1. Depth as a type, not as a const-generic expression.
// ---------------------------------------------------------------------------

/// A depth. `VALUE` is an associated const, so `D::VALUE + 1` is arithmetic in a
/// VALUE position, which is ordinary const evaluation, rather than arithmetic in a
/// TYPE position, which is the thing that needs the forbidden feature.
pub trait Depth {
    const VALUE: usize;
}

/// Depth zero: a leaf, a value that has been through no inexact step.
pub struct Zero;

/// One deeper than `D`.
pub struct Succ<D>(PhantomData<D>);

impl Depth for Zero {
    const VALUE: usize = 0;
}

impl<D: Depth> Depth for Succ<D> {
    const VALUE: usize = D::VALUE + 1;
}

// ---------------------------------------------------------------------------
// 2. A numeral carrying its depth, and an operation that increments it.
// ---------------------------------------------------------------------------

/// A fixed-point value on a grid of `2^-FRAC`, carrying the depth of the chain
/// that produced it. `SWITCH` is the arm's const predicate: the depth at which the
/// rounding policy changes, which p3 measures as the thing a weighting selects.
#[repr(transparent)]
pub struct Fx<const FRAC: u32, const SWITCH: usize, D: Depth> {
    raw: i64,
    _d: PhantomData<D>,
}

impl<const FRAC: u32, const SWITCH: usize, D: Depth> Fx<FRAC, SWITCH, D> {
    #[inline(always)]
    pub const fn from_raw(raw: i64) -> Self {
        Self {
            raw,
            _d: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn raw(&self) -> i64 {
        self.raw
    }

    /// The depth of this value, as a constant. This is the whole point: it is
    /// readable in a const context without the value existing.
    pub const DEPTH: usize = D::VALUE;

    /// Whether an operation producing a value at THIS depth rounds to nearest.
    /// A const, per instantiation, so the branch below is decided before codegen.
    pub const ROUNDS_TO_NEAREST: bool = D::VALUE >= SWITCH;
}

/// Multiply-accumulate: `a * b + c`, quantising the product back to the grid.
/// The result is one deeper than its accumulator operand.
///
/// The rounding policy is chosen by a const predicate over the RESULT's depth.
#[inline(always)]
pub fn mac<const FRAC: u32, const SWITCH: usize, D: Depth>(
    a: Fx<FRAC, SWITCH, D>,
    b: i64,
    c: i64,
) -> Fx<FRAC, SWITCH, Succ<D>>
where
    Succ<D>: Depth,
{
    let wide = a.raw() * b;
    // The one const-time branch. `ROUNDS_TO_NEAREST` is a constant per
    // instantiation, so this is the const-time `if` I15 asks for rather than a
    // runtime check.
    let q = if Fx::<FRAC, SWITCH, Succ<D>>::ROUNDS_TO_NEAREST {
        let half = 1i64 << (FRAC - 1);
        (wide + half) >> FRAC
    } else {
        wide >> FRAC
    };
    Fx::from_raw(q + c)
}

// ---------------------------------------------------------------------------
// 3. A chain, so the depth actually moves, and two instantiations that must
//    lower differently.
// ---------------------------------------------------------------------------

type D0 = Zero;
type D1 = Succ<D0>;
type D2 = Succ<D1>;
type D3 = Succ<D2>;

/// A four-step chain at switch depth 2: steps at result-depth 1 truncate, steps at
/// result-depth 2, 3 and 4 round to nearest.
#[unsafe(no_mangle)]
pub extern "C" fn chain_switch_at_2(x: i64, b: i64, c: i64) -> i64 {
    let v0: Fx<8, 2, D0> = Fx::from_raw(x);
    let v1 = mac(v0, b, c);
    let v2 = mac(v1, b, c);
    let v3 = mac(v2, b, c);
    let v4 = mac(v3, b, c);
    v4.raw()
}

/// The same chain with the switch past its end: every step truncates.
#[unsafe(no_mangle)]
pub extern "C" fn chain_truncate_everywhere(x: i64, b: i64, c: i64) -> i64 {
    let v0: Fx<8, 99, D0> = Fx::from_raw(x);
    let v1 = mac(v0, b, c);
    let v2 = mac(v1, b, c);
    let v3 = mac(v2, b, c);
    let v4 = mac(v3, b, c);
    v4.raw()
}

/// And with the switch at zero: every step rounds.
#[unsafe(no_mangle)]
pub extern "C" fn chain_round_everywhere(x: i64, b: i64, c: i64) -> i64 {
    let v0: Fx<8, 0, D0> = Fx::from_raw(x);
    let v1 = mac(v0, b, c);
    let v2 = mac(v1, b, c);
    let v3 = mac(v2, b, c);
    let v4 = mac(v3, b, c);
    v4.raw()
}

// ---------------------------------------------------------------------------
// 4. The depths are what the type says, checked at compile time.
//    A const assertion, so a wrong depth is a build failure rather than a test.
// ---------------------------------------------------------------------------

const _: () = {
    assert!(<D0 as Depth>::VALUE == 0);
    assert!(<D1 as Depth>::VALUE == 1);
    assert!(<D2 as Depth>::VALUE == 2);
    assert!(<D3 as Depth>::VALUE == 3);
    // and the predicate reads them
    assert!(!Fx::<8, 2, D1>::ROUNDS_TO_NEAREST);
    assert!(Fx::<8, 2, D2>::ROUNDS_TO_NEAREST);
    assert!(Fx::<8, 2, D3>::ROUNDS_TO_NEAREST);
    assert!(!Fx::<8, 99, D3>::ROUNDS_TO_NEAREST);
    assert!(Fx::<8, 0, D0>::ROUNDS_TO_NEAREST);
};

// A negative control: this must NOT compile if uncommented, because it asserts a
// depth the type does not have. Kept as a comment because a compile-fail case in a
// spike has nowhere to live; it is named so a later reader can run it.
//
// const _: () = { assert!(<D2 as Depth>::VALUE == 5); };
