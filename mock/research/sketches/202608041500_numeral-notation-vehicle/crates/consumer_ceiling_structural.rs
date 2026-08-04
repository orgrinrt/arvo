//! FAILS test, on purpose, and it fails one wall earlier than its own name
//! suggests. This was meant to test the ~128-bit STRUCTURAL ceiling
//! directly through the macro; it does not reach that branch at all, and
//! that is itself the finding this file records.
//!
//! `check_ceiling`'s `STRUCTURAL_CEILING_BITS` branch is real, correct
//! design intent, and is dead code under this sketch's own host
//! arithmetic: `Parsed::num`/`den` are `u128`, and `u128::MAX` needs
//! exactly 128 bits, so no value `.parse::<u128>()` can ever produce
//! requires more than 128 bits. The literal below (60 digits, ~200 bits)
//! is refused by `str::parse::<u128>()` failing BEFORE `check_ceiling`
//! ever runs, with `parse_expr`'s own honest "does not fit u128" message,
//! not the structural-ceiling message. A vehicle wanting to exercise the
//! structural wall by ITS OWN mechanism (rather than by accident of a
//! narrower host integer) needs host arithmetic wider than u128 (a small
//! bignum, or u256): out of scope for this sketch, recorded as a real,
//! priced gap rather than silently worked around.
//!
//! The structural wall itself is not in doubt: `61_probes/probe_5`
//! compiles it directly against hand-built types (depth 128 succeeds,
//! depth 129 fails), independent of this macro or its host-arithmetic
//! width.

#[path = "tower.rs"]
mod tower;

extern crate numeral_pm;
use numeral_pm::raw_bias;

type _T = raw_bias!(123456789012345678901234567890123456789012345678901234567890);

fn main() {}
