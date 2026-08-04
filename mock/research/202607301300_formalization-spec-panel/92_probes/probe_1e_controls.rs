//! Two positive controls, so the refusals above are refusals of the attack
//! and not of the probe. First: the honest member reaches the audited entry.
//! Second: a foreign hand-laid `unsafe impl Crosses` still compiles, because
//! that is the trusted-base tier's own front door working as designed; the
//! seal guards the ONE audited niche entry, not the trusted base itself.
#![no_std]
extern crate tower;
use core::num::NonZeroU16;

fn require_crosses<T: tower::Crosses>() {}

pub fn honest() {
    require_crosses::<tower::ViaNiche<NonZeroU16>>();
}

pub struct HandLaidLowering;
// SAFETY: model of an ordinary audited trusted-base entry; asserts nothing
// here because the model Crosses carries no methods.
unsafe impl tower::Crosses for HandLaidLowering {}

pub fn hand_laid() {
    require_crosses::<HandLaidLowering>();
}
