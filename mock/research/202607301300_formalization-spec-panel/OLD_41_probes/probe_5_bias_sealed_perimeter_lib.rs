//! Probe 5 (the library half): `Bias`'s perimeter, checked against exactly
//! the attack that succeeded against `Adjustment` in probe 4b.
//!
//! `Bias` is bounded directly on `N: Pos + Gcd<D, Out = H>, D: Pos`, never
//! on `Adjustment`, and it carries its own private `bias_sealed::
//! BiasSealed` supertrait on top of that, the same shape `Pos`/`Nat` use.
//! Probe 5b tries the identical attack probe 4b ran (a foreign type,
//! implementing the trait directly, with a fabricated or unreduced value)
//! and is refused, both for the sealed-supertrait reason (a downstream
//! crate cannot see `bias_sealed::BiasSealed` at all) and, independently,
//! for the coprimality reason (even inside this crate, an unreduced pair
//! does not satisfy the bound; probe 1b is that half).
//!
//! Build: rustc --edition 2021 --crate-type lib \
//!        probe_5_bias_sealed_perimeter_lib.rs --out-dir <dir>
//! Outcome: WORKS. rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]
#![crate_name = "vu_bias_sealed"]

#[path = "vu_bias.rs"]
pub mod bias;

pub type P1 = bias::nat::H;
pub type P2 = bias::nat::O<bias::nat::H>;
const _: () = assert!(<bias::BPos<P1, P2> as bias::Bias>::NUM == 1);
