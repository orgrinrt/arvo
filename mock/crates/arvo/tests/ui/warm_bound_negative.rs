//! Negative compile fixture: Warm at BITS > 64 must not resolve.
//!
//! Round 202604280500 extended Warm's container table from 32 to 64
//! via a u128 carrier (the 2x-logical primitive at 33..=64). Wider
//! logical widths (65+) still drop off Warm's table; Hot or Cold are
//! the only options past 64 because no native u256 exists for the
//! 2x-logical contract.

use arvo::{FBits, IBits};
use arvo::strategy::Warm;
use arvo::ufixed::UFixed;

fn main() {
    // 33 + 32 = 65 total logical bits, beyond Warm's post-Round-B
    // container table (caps at 64).
    let _: UFixed<{ IBits(33) }, { FBits(32) }, Warm> = UFixed::from_raw(0);
}
