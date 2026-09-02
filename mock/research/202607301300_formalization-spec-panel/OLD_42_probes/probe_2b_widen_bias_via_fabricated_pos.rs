//! Probe 2b: the headline finding. File 41's own probe 5/5b checked two
//! routes against `Bias` and found both refused: implementing the private
//! `bias_sealed::BiasSealed` supertrait directly (refused, E0603), and
//! feeding the exported `BPos` constructor an unreduced pair of `Bias`'s
//! own `H`/`O`/`I` numerals (refused, E0271). Neither route touches
//! `Pos`/`Gcd`, which sit one layer BELOW `Bias`'s own seal and are
//! completely unsealed in `vu_nat.rs`, the module `vu_bias.rs` actually
//! composes with (not the standalone, orphaned
//! `36_probes/probe_5_sealed_perimeter_lib.rs` the consolidation cites at
//! `40:446-448` as evidence that "Pos, Nat and Int are sealed").
//!
//! `Bias`'s own blanket impl (`impl<N: Pos + Gcd<D, Out = H>, D: Pos>
//! Bias for BPos<N, D>`) is a normal generic impl over the LOCAL type
//! `BPos<N, D>`, parameterised by whatever `N`, `D` a caller supplies. No
//! orphan-rule violation is possible here regardless of where `N`/`D`
//! come from, because `BPos` is defined upstream. So a downstream crate
//! never needs to implement `Bias` or `BiasSealed` at all: it only needs
//! an `N` satisfying `Pos + Gcd<D, Out = H>`, and probe 1b's exact
//! `Fabricated` type already is one, unconditionally, with a `Gcd` impl
//! that never runs a real coprimality check.
//!
//! `BPos<Fabricated, D4>` then satisfies `Bias` through the upstream
//! blanket impl, entirely through a foreign `N` whose magnitude was never
//! checked, and denotes the unreduced value 4/4, the same value
//! `BPos<H, H>` (1/1) does not, under a type nothing in the design
//! verified.
//!
//! Committed as a WORKS that should have been refused. Do not "fix" this
//! file by making it compile differently; probe 3 is the fix, sealing
//! `Pos`/`Nat` where they actually compose, and this file is the
//! regression test that should start refusing once it lands.
//!
//! Build (two steps, both against rustc 1.98.0-nightly (57d06900f
//! 2026-05-27)):
//!   rustc --edition 2021 --crate-type lib \
//!         probe_2_widen_bias_via_fabricated_pos_lib.rs --out-dir <dir>
//!   rustc --edition 2021 --crate-type lib \
//!         --extern vu_bias_unsealed=<dir>/libvu_bias_unsealed.rlib \
//!         probe_2b_widen_bias_via_fabricated_pos.rs --out-dir <dir>
//! Outcome: WORKS (the defect), verbatim in OUTCOMES.md.

#![allow(dead_code)]
#![no_std]

use vu_bias_unsealed::bias::nat::{Gcd, Pos, H, O};
use vu_bias_unsealed::bias::Bias;

/// The identical foreign type probe 1b uses against `Adjustment`,
/// unchanged, because the same hole underlies both attacks.
pub struct Fabricated;
impl Pos for Fabricated {
    const VAL: u64 = 4;
}
impl<D: Pos> Gcd<D> for Fabricated {
    type Out = H;
}

pub type D4 = O<O<H>>;

pub trait NumDen {
    fn num_den() -> (i64, u64);
}
impl<T: Bias> NumDen for T {
    fn num_den() -> (i64, u64) {
        (T::NUM, T::DEN)
    }
}

/// `BPos<Fabricated, D4>` satisfies `Bias` (through the upstream blanket
/// impl, never touching `Bias`'s own seal) and denotes the unreduced
/// value 4/4.
const _: () = assert!(
    <vu_bias_unsealed::bias::BPos<Fabricated, D4> as Bias>::NUM == 4
        && <vu_bias_unsealed::bias::BPos<Fabricated, D4> as Bias>::DEN == 4
);
