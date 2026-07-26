//! `UFixed<0, F, S>` spans `[0, 1)` and has no multiplicative identity.
//!
//! The absence of the impl is the statement, so naming `IDENTITY` for the
//! multiplicative operation is a trait-resolution error rather than a value.
//! This is the case that cannot be written as a runtime assertion, because
//! there is no correct expected value to compare against: the raw encoding
//! of one is `1 << F` and the container is `F` bits wide, so the encoding
//! does not fit and the expected value cannot be written down at all.
//!
//! Before the constraint existed this compiled and produced raw 0 under
//! `Hot`, so `x * ONE` annihilated every purely fractional value.

use arvo::strategy::{Hot, Identity, Multiplicative};
use arvo::ufixed::UFixed;
use arvo::{fbits, ibits};

fn main() {
    let _ = <UFixed<{ ibits(0) }, { fbits(8) }, Hot> as Identity<Multiplicative>>::IDENTITY;
}
