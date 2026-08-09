//! Probe 3b: both probe 1b's attack (Adjustment) and probe 2b's attack
//! (Bias), re-run verbatim against the sealed tower, in one file so a
//! partial fix cannot be mistaken for closing the perimeter. Both are
//! refused, and neither `Adjustment`, `Bias`, `Gcd`, `ExactDivOdd`,
//! `Strip2` nor `Reduce` was touched to get there; only `Pos`/`Nat`
//! themselves carry the seal, exactly as `36_probes/probe_5` already
//! showed in isolation.
//!
//! Both const blocks stay active at once, per file 41's own probe 5b
//! precedent, so a fix to only one route cannot be mistaken for closing
//! the perimeter. In practice the refusal lands once, at the single
//! `impl Pos for Fabricated` line both blocks depend on (E0277,
//! "`Fabricated: PosSealed` is not satisfied ... `Pos` is a sealed
//! trait"), which blocks both downstream uses at their common root
//! rather than reporting two separate diagnostics; the source of the
//! refusal is the same for both routes, and the build stopping at that
//! one line is itself confirmation of that, not a weaker result.
//!
//! Build (two steps, both against rustc 1.98.0-nightly (57d06900f
//! 2026-05-27)):
//!   rustc --edition 2021 --crate-type lib \
//!         probe_3_sealed_tower_refuses_both_lib.rs --out-dir <dir>
//!   rustc --edition 2021 --crate-type lib \
//!         --extern vu_sealed_tower=<dir>/libvu_sealed_tower.rlib \
//!         probe_3b_sealed_tower_refuses_both.rs --out-dir <dir>
//! Outcome: FAILS WITH E0277, verbatim in OUTCOMES.md, at the shared
//! `impl Pos for Fabricated` line both (a) and (b) depend on.

#![allow(dead_code)]
#![no_std]

use vu_sealed_tower::bias::nat::{Adjustment, Gcd, Pos, Ratio, H, O};
use vu_sealed_tower::bias::Bias;

pub struct Fabricated;
impl Pos for Fabricated {
    const VAL: u64 = 4;
}
impl<D: Pos> Gcd<D> for Fabricated {
    type Out = H;
}
pub type D4 = O<O<H>>;

// (a) Adjustment, via the fabricated-Pos route: refused.
const _: () = assert!(
    <Ratio<Fabricated, D4> as Adjustment>::NUM == 4
        && <Ratio<Fabricated, D4> as Adjustment>::DEN == 4
);

// (b) Bias, via the identical fabricated-Pos route: refused.
const _: () = assert!(
    <vu_sealed_tower::bias::BPos<Fabricated, D4> as Bias>::NUM == 4
        && <vu_sealed_tower::bias::BPos<Fabricated, D4> as Bias>::DEN == 4
);
