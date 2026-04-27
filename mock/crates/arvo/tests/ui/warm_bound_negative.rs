//! Negative compile fixture: Warm at BITS > 32 must not resolve.

use arvo::{FBits, IBits};
use arvo::strategy::Warm;
use arvo::ufixed::UFixed;

fn main() {
    // 17 + 16 = 33 total logical bits — beyond Warm's container table.
    let _: UFixed<{ IBits(17) }, { FBits(16) }, Warm> = UFixed::from_raw(0);
}
