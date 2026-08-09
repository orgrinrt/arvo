//! E2a: the forbidden route, recorded so the refusal is on the record.
//! Adding two GENERIC consts in type position.
#![no_std]
pub struct W<const N: u16>;

pub type PrecisionOf<const I: u16, const F: u16> = W<{ I + F }>;
