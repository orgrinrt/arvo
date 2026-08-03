//! Probe 1b: a genuinely separate crate defeats `Adjustment`'s blanket
//! impl (`impl<N: Pos + Gcd<D, Out = H>, D: Pos> Adjustment for Ratio<N,
//! D>`) not by implementing `Adjustment` directly (file 41's probe 4b, a
//! route the recommended fix in file 41 section 2 closes), but by
//! fabricating a foreign `Pos` type whose `Gcd` impl claims unconditional
//! coprimality with no real computation, then naming `Ratio<Fabricated,
//! D4>` at the ALREADY-EXPORTED, never-touched `Adjustment` blanket impl.
//! Sealing `Adjustment` itself does nothing against this route: the attack
//! never implements `Adjustment`, it supplies an `N` the blanket impl's
//! own bound accepts because `Pos` and `Gcd` are unsealed in the module
//! everything actually composes with.
//!
//! `Ratio<N, D>` is `PhantomData`-only, so no value construction is
//! needed anywhere: the attack works entirely at the const-projection
//! level, `<Ratio<Fabricated, D4> as Adjustment>::NUM`.
//!
//! Committed as a WORKS that should have been refused. Do not "fix" this
//! file by making it compile differently; the finding is that it
//! compiles at all, and that it survives the fix file 41 proposed.
//!
//! Build (two steps, both against rustc 1.98.0-nightly (57d06900f
//! 2026-05-27)):
//!   rustc --edition 2021 --crate-type lib \
//!         probe_1_widen_adjustment_via_fabricated_pos_lib.rs --out-dir <dir>
//!   rustc --edition 2021 --crate-type lib \
//!         --extern vu_nat_unsealed=<dir>/libvu_nat_unsealed.rlib \
//!         probe_1b_widen_adjustment_via_fabricated_pos.rs --out-dir <dir>
//! Outcome: WORKS (the defect), verbatim in OUTCOMES.md.

#![allow(dead_code)]
#![no_std]

use vu_nat_unsealed::nat::{Adjustment, Gcd, Pos, Ratio, H, O};

/// A foreign type, nothing to do with `H`/`O`/`I`. Claims `Pos::VAL = 4`
/// with no construction behind the claim: nobody checked it, because
/// `Pos` has no seal.
pub struct Fabricated;
impl Pos for Fabricated {
    const VAL: u64 = 4;
}

/// Claims unconditional coprimality with anything, `Out = H` regardless
/// of the actual gcd. No Stein computation performed. `Gcd` has no seal
/// either, so nothing stops this.
impl<D: Pos> Gcd<D> for Fabricated {
    type Out = H;
}

/// A real Pos value, 4 = O<O<H>>.
pub type D4 = O<O<H>>;

/// `Ratio<Fabricated, D4>` denotes the unreduced fraction 4/4, the same
/// value `Ratio<H, H>` (via `Adjustment`) already denotes, one, under a
/// second, wholly foreign type Adjustment's own bound never rules out.
const _: () = assert!(
    <Ratio<Fabricated, D4> as Adjustment>::NUM == 4
        && <Ratio<Fabricated, D4> as Adjustment>::DEN == 4
);
