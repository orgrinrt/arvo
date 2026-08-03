//! Probe 4 (the library half): `Adjustment`, as shipped by file 36, has no
//! seal, and this crate is the one a downstream crate widens against.
//!
//! Op's own framing for this dispatch: "the seal that prevents a downstream
//! crate from reinstating a non-normal form has to survive the
//! composition." `Bias`'s magnitude is `Adjustment`'s own reduced pair, so
//! checking whether the seal survives means checking `Adjustment` itself,
//! not assuming it because `Pos`/`Nat` are sealed underneath it.
//!
//! `vu_nat.rs`'s `Adjustment` is `pub trait Adjustment { const NUM: u64;
//! const DEN: u64; } impl<N: Pos + Gcd<D, Out = H>, D: Pos> Adjustment for
//! Ratio<N, D> { ... }`. `Pos` and `Nat` carry a private `sealed::PosSealed`
//! / `NatSealed` supertrait (`36_probes/probe_5`); `Adjustment` does not.
//! Nothing stops a foreign crate implementing `Adjustment` directly on a
//! foreign type, with any `(NUM, DEN)` pair it likes, coprime or not.
//! Probe 4b is that foreign crate, and it is accepted, not refused. File
//! 36's own section 6 predicted this weakly ("I have not tested whether the
//! sealed encoding survives a downstream crate defining a foreign
//! numeral... predicted weakly to fail"); this is that test, and it is not
//! about the sealed encoding surviving, it is about `Adjustment` never
//! having been inside the seal at all.
//!
//! `vu_bias.rs`'s `Bias` does not inherit this hole: `BPos`/`BNeg` are
//! bounded directly on `N: Pos + Gcd<D, Out = H>, D: Pos`, never on
//! `Adjustment`, precisely so that `Bias`'s guarantee rests only on
//! `Pos`/`Nat`'s own seal. Probe 5/5b confirm that bound is closed.
//!
//! Build: rustc --edition 2021 --crate-type lib \
//!        probe_4_adjustment_is_not_sealed_lib.rs --out-dir <dir>
//! Outcome: WORKS. rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]
#![crate_name = "vu_adjustment_unsealed"]

#[path = "vu_nat.rs"]
pub mod nat;
