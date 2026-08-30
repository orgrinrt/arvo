//! Probe 5b (the refusing half of probe 5): a genuinely separate crate,
//! compiled against probe 5's rlib, tries to add a fourth `Pos` and is
//! refused. Two crates rather than two modules, because a private supertrait
//! is visible inside its own crate and the claim under test is specifically
//! about the outside.
//!
//! Committed refusing, on purpose. Do not "fix" this file.
//!
//! Build (two steps, both against rustc 1.98.0-nightly (57d06900f 2026-05-27)):
//!   rustc --edition 2021 --crate-type lib probe_5_sealed_perimeter_lib.rs --out-dir <dir>
//!   rustc --edition 2021 --crate-type lib \
//!         --extern vu_sealed=<dir>/libvu_sealed.rlib \
//!         probe_5b_downstream_cannot_widen_the_perimeter.rs --out-dir <dir>
//! Outcome: FAILS WITH E0277, verbatim in OUTCOMES.md.

#![allow(dead_code)]
#![no_std]

use vu_sealed::{Nat, Pos};

/// A second inhabitant denoting six, which `probe_5.P6` already denotes.
pub struct MySix;

impl Pos for MySix {
    const VAL: u64 = 6;
}

/// And a second inhabitant of `Nat`, likewise.
pub struct MyZero;

impl Nat for MyZero {
    const VAL: u64 = 0;
}
